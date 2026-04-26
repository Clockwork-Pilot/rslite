pub use crate::__stdarg_GnucVaList_h::GnucVaList;
pub use crate::__stddef_size_t_h::SizeT;

pub use crate::fts3Int_h::FTS_CORRUPT_VTAB;
pub use crate::fts3Int_h::FTS3_BUFFER_PADDING;
pub use crate::fts3Int_h::FTS3_DOCID_SEARCH;
pub use crate::fts3Int_h::FTS3_FULLSCAN_SEARCH;
pub use crate::fts3Int_h::FTS3_FULLTEXT_SEARCH;
pub use crate::fts3Int_h::FTS3_HAVE_DOCID_GE;
pub use crate::fts3Int_h::FTS3_HAVE_DOCID_LE;
pub use crate::fts3Int_h::FTS3_HAVE_LANGID;
pub use crate::fts3Int_h::FTS3_MAX_PENDING_DATA;
pub use crate::fts3Int_h::FTS3_MERGE_COUNT;
pub use crate::fts3Int_h::FTS3_SEGCURSOR_ALL;
pub use crate::fts3Int_h::FTS3_SEGCURSOR_PENDING;
pub use crate::fts3Int_h::FTS3_SEGMENT_COLUMN_FILTER;
pub use crate::fts3Int_h::FTS3_SEGMENT_FIRST;
pub use crate::fts3Int_h::FTS3_SEGMENT_IGNORE_EMPTY;
pub use crate::fts3Int_h::FTS3_SEGMENT_PREFIX;
pub use crate::fts3Int_h::FTS3_SEGMENT_REQUIRE_POS;
pub use crate::fts3Int_h::FTS3_VARINT_MAX;
pub use crate::fts3Int_h::FTSQUERY_AND;
pub use crate::fts3Int_h::FTSQUERY_NEAR;
pub use crate::fts3Int_h::FTSQUERY_NOT;
pub use crate::fts3Int_h::FTSQUERY_OR;
pub use crate::fts3Int_h::FTSQUERY_PHRASE;
pub use crate::fts3Int_h::Fts3Cursor;
pub use crate::fts3Int_h::Fts3DeferredToken;
pub use crate::fts3Int_h::Fts3Doclist;
pub use crate::fts3Int_h::Fts3Expr;
pub use crate::fts3Int_h::Fts3Index;
pub use crate::fts3Int_h::Fts3MultiSegReader;
pub use crate::fts3Int_h::Fts3Phrase;
pub use crate::fts3Int_h::Fts3PhraseToken;
pub use crate::fts3Int_h::Fts3SegFilter;
pub use crate::fts3Int_h::Fts3SegReader;
pub use crate::fts3Int_h::Fts3Table;
pub use crate::fts3Int_h::LARGEST_INT64;
pub use crate::fts3Int_h::MatchinfoBuffer;
pub use crate::fts3Int_h::POS_COLUMN;
pub use crate::fts3Int_h::POS_END;
pub use crate::fts3Int_h::SMALLEST_INT64;
#[cfg(feature = "test")]
pub use crate::fts3Int_h::sqlite3Fts3InitTerm;
pub use crate::internal::BuiltinVaList;
pub use crate::internal::VaListTag;
pub use crate::src::ext::fts3::fts3_aux::sqlite3Fts3InitAux;
pub use crate::src::ext::fts3::fts3_expr::sqlite3Fts3ExprFree;
pub use crate::src::ext::fts3::fts3_expr::sqlite3Fts3ExprInitTestInterface;
pub use crate::src::ext::fts3::fts3_expr::sqlite3Fts3ExprParse;
pub use crate::src::ext::fts3::fts3_expr::sqlite3Fts3MallocZero;
pub use crate::src::ext::fts3::fts3_hash::_fts3ht;
pub use crate::src::ext::fts3::fts3_hash::FTS3_HASH_STRING;
pub use crate::src::ext::fts3::fts3_hash::Fts3Hash;
pub use crate::src::ext::fts3::fts3_hash::Fts3HashElem;
pub use crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashClear;
pub use crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashInit;
pub use crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashInsert;
pub use crate::src::ext::fts3::fts3_snippet::sqlite3Fts3ExprIterate;
pub use crate::src::ext::fts3::fts3_snippet::sqlite3Fts3MIBufferFree;
pub use crate::src::ext::fts3::fts3_snippet::sqlite3Fts3Matchinfo;
pub use crate::src::ext::fts3::fts3_snippet::sqlite3Fts3Offsets;
pub use crate::src::ext::fts3::fts3_snippet::sqlite3Fts3Snippet;
pub use crate::src::ext::fts3::fts3_tokenize_vtab::sqlite3Fts3InitTok;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3Fts3InitHashTable;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3Fts3InitTokenizer;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3Fts3IsIdChar;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3Fts3NextToken;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3AllSegdirs;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3CacheDeferredDoclists;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3DeferToken;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3DeferredTokenList;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3FreeDeferredDoclists;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3FreeDeferredTokens;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3Incrmerge;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3IntegrityCheck;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3MaxLevel;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3MsrIncrNext;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3MsrIncrRestart;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3MsrIncrStart;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3MsrOvfl;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3Optimize;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3PendingTermsClear;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3PendingTermsFlush;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3ReadBlock;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderFinish;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderFree;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderNew;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderPending;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderStart;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderStep;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3SegmentsClose;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3SelectDoctotal;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3UpdateMethod;
pub use crate::src::ext::rtree::rtree::I64_0;
pub use crate::src::ext::rtree::rtree::U8_0;
pub use crate::src::ext::rtree::rtree::U32_0;
pub use crate::src::ext::rtree::rtree::U64_0;
pub use crate::src::fts5::I16_0;
pub use crate::src::headers::sqlite3_h::SQLITE_AUTH;
pub use crate::src::headers::sqlite3_h::SQLITE_CORRUPT;
pub use crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
pub use crate::src::headers::sqlite3_h::SQLITE_DONE;
pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;
pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_EQ;
pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_GE;
pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_GT;
pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_LE;
pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_LT;
pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_MATCH;
pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_SCAN_UNIQUE;
pub use crate::src::headers::sqlite3_h::SQLITE_INTEGER;
pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_NULL;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SQLITE_PREPARE_PERSISTENT;
pub use crate::src::headers::sqlite3_h::SQLITE_ROW;
pub use crate::src::headers::sqlite3_h::SQLITE_STATIC;
pub use crate::src::headers::sqlite3_h::SQLITE_VTAB_CONSTRAINT_SUPPORT;
pub use crate::src::headers::sqlite3_h::SQLITE_VTAB_INNOCUOUS;
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
pub use crate::src::headers::stdlib::VaList;

pub use crate::src::headers::vdbeInt_h::sqlite3_context;
pub use crate::src::headers::vdbeInt_h::sqlite3_value;
pub use crate::src::src::legacy::sqlite3_exec;
pub use crate::src::src::main::sqlite3_errmsg;
pub use crate::src::src::main::sqlite3_errstr;
pub use crate::src::src::main::sqlite3_last_insert_rowid;
pub use crate::src::src::main::sqlite3_libversion_number;
pub use crate::src::src::main::sqlite3_overload_function;
pub use crate::src::src::main::sqlite3_set_last_insert_rowid;
pub use crate::src::src::main::sqlite3_table_column_metadata;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3_malloc;
pub use crate::src::src::malloc::sqlite3_malloc64;
pub use crate::src::src::malloc::sqlite3_realloc64;
pub use crate::src::src::prepare::sqlite3_prepare;
pub use crate::src::src::prepare::sqlite3_prepare_v3;
pub use crate::src::src::util::sqlite3_stricmp;
pub use crate::src::src::util::sqlite3_strnicmp;
pub use crate::src::src::vdbe::sqlite3_value_numeric_type;
pub use crate::src::src::vdbeapi::sqlite3_bind_int64;
pub use crate::src::src::vdbeapi::sqlite3_bind_value;
pub use crate::src::src::vdbeapi::sqlite3_column_blob;
pub use crate::src::src::vdbeapi::sqlite3_column_bytes;
pub use crate::src::src::vdbeapi::sqlite3_column_count;
pub use crate::src::src::vdbeapi::sqlite3_column_int;
pub use crate::src::src::vdbeapi::sqlite3_column_int64;
pub use crate::src::src::vdbeapi::sqlite3_column_name;
pub use crate::src::src::vdbeapi::sqlite3_column_value;
pub use crate::src::src::vdbeapi::sqlite3_data_count;
pub use crate::src::src::vdbeapi::sqlite3_finalize;
pub use crate::src::src::vdbeapi::sqlite3_reset;
pub use crate::src::src::vdbeapi::sqlite3_result_error;
pub use crate::src::src::vdbeapi::sqlite3_result_error_code;
pub use crate::src::src::vdbeapi::sqlite3_result_error_nomem;
pub use crate::src::src::vdbeapi::sqlite3_result_int;
pub use crate::src::src::vdbeapi::sqlite3_result_int64;
pub use crate::src::src::vdbeapi::sqlite3_result_pointer;
pub use crate::src::src::vdbeapi::sqlite3_result_text;
pub use crate::src::src::vdbeapi::sqlite3_result_value;
pub use crate::src::src::vdbeapi::sqlite3_step;
pub use crate::src::src::vdbeapi::sqlite3_value_int;
pub use crate::src::src::vdbeapi::sqlite3_value_int64;
pub use crate::src::src::vdbeapi::sqlite3_value_pointer;
pub use crate::src::src::vdbeapi::sqlite3_value_text;
pub use crate::src::src::vdbeapi::sqlite3_value_type;
pub use crate::src::src::vtab::sqlite3_create_module_v2;
pub use crate::src::src::vtab::sqlite3_declare_vtab;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3Fts3SimpleTokenizerModule;
pub use crate::src::ext::fts3::fts3_porter::sqlite3Fts3PorterTokenizerModule;
pub use crate::src::ext::fts3::fts3_unicode::sqlite3Fts3UnicodeTokenizer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TokenDoclist {
    pub bIgnore: ::core::ffi::c_int,
    pub iDocid: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub pList: *mut ::core::ffi::c_char,
    pub nList: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TermSelect {
    pub aaOutput: [*mut ::core::ffi::c_char; 16],
    pub anOutput: [::core::ffi::c_int; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3HashWrapper {
    pub hash: crate::src::ext::fts3::fts3_hash::Fts3Hash,
    pub nRef: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Overloaded {
    pub zName: *const ::core::ffi::c_char,
    pub xFunc: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3TokenAndCost {
    pub pPhrase: *mut crate::fts3Int_h::Fts3Phrase,
    pub iToken: ::core::ffi::c_int,
    pub pToken: *mut crate::fts3Int_h::Fts3PhraseToken,
    pub pRoot: *mut crate::fts3Int_h::Fts3Expr,
    pub nOvfl: ::core::ffi::c_int,
    pub iCol: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts4Option {
    pub zOpt: *const ::core::ffi::c_char,
    pub nOpt: ::core::ffi::c_int,
}

// Helper: execute a pre-formatted SQL string, freeing it. Replaces fts3DbExec.
unsafe fn fts3_db_exec_sql(
    pRc: *mut ::core::ffi::c_int,
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zSql: *mut ::core::ffi::c_char,
) {
    if *pRc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
        return;
    }
    if zSql.is_null() {
        *pRc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        return;
    }
    *pRc = crate::src::src::legacy::sqlite3_exec(
        db,
        zSql,
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
}

// Helper: append a pre-formatted string to *pz. Replaces fts3Appendf.
unsafe fn fts3_appendf_raw(
    pRc: *mut ::core::ffi::c_int,
    pz: *mut *mut ::core::ffi::c_char,
    zNew: *mut ::core::ffi::c_char,
) {
    if *pRc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::malloc::sqlite3_free(zNew as *mut ::core::ffi::c_void);
        return;
    }
    let z = if !zNew.is_null() && !(*pz).is_null() {
        let z2 = crate::sqlite_printf!("%s%s", *pz, zNew);
        crate::src::src::malloc::sqlite3_free(zNew as *mut ::core::ffi::c_void);
        z2
    } else {
        zNew
    };
    if z.is_null() {
        *pRc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    crate::src::src::malloc::sqlite3_free(*pz as *mut ::core::ffi::c_void);
    *pz = z;
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe fn sqlite3Fts3ErrMsg(
    pzErr: *mut *mut ::core::ffi::c_char,
    zFormat: *const ::core::ffi::c_char,
    args: &[crate::src::src::printf::PrintfArg],
) {
    crate::src::src::malloc::sqlite3_free(*pzErr as *mut ::core::ffi::c_void);
    *pzErr = crate::src::src::printf::sqlite3_vmprintf_args(zFormat, args);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3PutVarint(
    p: *mut ::core::ffi::c_char,
    v: crate::src::headers::sqlite3_h::SqliteInt64,
) -> ::core::ffi::c_int {
    let mut q: *mut ::core::ffi::c_uchar = p as *mut ::core::ffi::c_uchar;
    let mut vu: crate::src::headers::sqlite3_h::SqliteUint64 =
        v as crate::src::headers::sqlite3_h::SqliteUint64;
    loop {
        let fresh11 = q;
        q = q.offset(1);
        *fresh11 = (vu & 0x7f as crate::src::headers::sqlite3_h::SqliteUint64
            | 0x80 as crate::src::headers::sqlite3_h::SqliteUint64)
            as ::core::ffi::c_uchar;
        vu >>= 7 as ::core::ffi::c_int;
        if (vu == 0 as crate::src::headers::sqlite3_h::SqliteUint64) {
            break;
        }
    }
    let fresh12 = &mut *q.offset(-(1 as ::core::ffi::c_int) as isize);
    *fresh12 =
        (*fresh12 as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    q.offset_from(p as *mut ::core::ffi::c_uchar) as ::core::ffi::c_long as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3GetVarintU(
    pBuf: *const ::core::ffi::c_char,
    v: *mut crate::src::headers::sqlite3_h::SqliteUint64,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_uchar = pBuf as *const ::core::ffi::c_uchar;
    let pStart: *const ::core::ffi::c_uchar = p;
    let mut a: crate::src::ext::rtree::rtree::U32_0;
    let mut b: crate::src::ext::rtree::rtree::U64_0;
    let mut shift: ::core::ffi::c_int;
    let fresh6 = p;
    p = p.offset(1);
    a = *fresh6 as crate::src::ext::rtree::rtree::U32_0;
    if a & 0x80 as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0
    {
        *v = a as crate::src::headers::sqlite3_h::SqliteUint64;
        return 1 as ::core::ffi::c_int;
    }
    let fresh7 = p;
    p = p.offset(1);
    a = a & 0x7f as crate::src::ext::rtree::rtree::U32_0
        | ((*fresh7 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0;
    if a & 0x4000 as crate::src::ext::rtree::rtree::U32_0
        == 0 as crate::src::ext::rtree::rtree::U32_0
    {
        *v = a as crate::src::headers::sqlite3_h::SqliteUint64;
        return 2 as ::core::ffi::c_int;
    }
    let fresh8 = p;
    p = p.offset(1);
    a = a & 0x3fff as crate::src::ext::rtree::rtree::U32_0
        | ((*fresh8 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0;
    if a & 0x200000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        == 0 as crate::src::ext::rtree::rtree::U32_0
    {
        *v = a as crate::src::headers::sqlite3_h::SqliteUint64;
        return 3 as ::core::ffi::c_int;
    }
    let fresh9 = p;
    p = p.offset(1);
    a = a & 0x1fffff as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        | ((*fresh9 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0;
    if a & 0x10000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        == 0 as crate::src::ext::rtree::rtree::U32_0
    {
        *v = a as crate::src::headers::sqlite3_h::SqliteUint64;
        return 4 as ::core::ffi::c_int;
    }
    b = (a & 0xfffffff as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0)
        as crate::src::ext::rtree::rtree::U64_0;
    shift = 28 as ::core::ffi::c_int;
    while shift <= 63 as ::core::ffi::c_int {
        let fresh10 = p;
        p = p.offset(1);
        let c: crate::src::ext::rtree::rtree::U64_0 =
            *fresh10 as crate::src::ext::rtree::rtree::U64_0;
        b = b.wrapping_add((c & 0x7f as crate::src::ext::rtree::rtree::U64_0) << shift);
        if c & 0x80 as crate::src::ext::rtree::rtree::U64_0
            == 0 as crate::src::ext::rtree::rtree::U64_0
        {
            break;
        }
        shift += 7 as ::core::ffi::c_int;
    }
    *v = b as crate::src::headers::sqlite3_h::SqliteUint64;
    p.offset_from(pStart) as ::core::ffi::c_long as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3GetVarint(
    pBuf: *const ::core::ffi::c_char,
    v: *mut crate::src::headers::sqlite3_h::SqliteInt64,
) -> ::core::ffi::c_int {
    sqlite3Fts3GetVarintU(
        pBuf,
        v as *mut crate::src::headers::sqlite3_h::SqliteUint64,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3GetVarintBounded(
    pBuf: *const ::core::ffi::c_char,
    pEnd: *const ::core::ffi::c_char,
    v: *mut crate::src::headers::sqlite3_h::SqliteInt64,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_uchar = pBuf as *const ::core::ffi::c_uchar;
    let pStart: *const ::core::ffi::c_uchar = p;
    let pX: *const ::core::ffi::c_uchar = pEnd as *const ::core::ffi::c_uchar;
    let mut b: crate::src::ext::rtree::rtree::U64_0 = 0 as crate::src::ext::rtree::rtree::U64_0;
    let mut shift: ::core::ffi::c_int;
    shift = 0 as ::core::ffi::c_int;
    while shift <= 63 as ::core::ffi::c_int {
        let c: crate::src::ext::rtree::rtree::U64_0 = (if p < pX {
            *p as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })
            as crate::src::ext::rtree::rtree::U64_0;
        p = p.offset(1);
        b = b.wrapping_add((c & 0x7f as crate::src::ext::rtree::rtree::U64_0) << shift);
        if c & 0x80 as crate::src::ext::rtree::rtree::U64_0
            == 0 as crate::src::ext::rtree::rtree::U64_0
        {
            break;
        }
        shift += 7 as ::core::ffi::c_int;
    }
    *v = b as crate::src::headers::sqlite3_h::SqliteInt64;
    p.offset_from(pStart) as ::core::ffi::c_long as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3GetVarint32(
    p: *const ::core::ffi::c_char,
    pi: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ptr: *const ::core::ffi::c_uchar = p as *const ::core::ffi::c_uchar;
    let mut a: crate::src::ext::rtree::rtree::U32_0;
    let fresh2 = ptr;
    ptr = ptr.offset(1);
    a = *fresh2 as crate::src::ext::rtree::rtree::U32_0;
    let fresh3 = ptr;
    ptr = ptr.offset(1);
    a = a & 0x7f as crate::src::ext::rtree::rtree::U32_0
        | ((*fresh3 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0;
    if a & 0x4000 as crate::src::ext::rtree::rtree::U32_0
        == 0 as crate::src::ext::rtree::rtree::U32_0
    {
        *pi = a as ::core::ffi::c_int;
        return 2 as ::core::ffi::c_int;
    }
    let fresh4 = ptr;
    ptr = ptr.offset(1);
    a = a & 0x3fff as crate::src::ext::rtree::rtree::U32_0
        | ((*fresh4 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0;
    if a & 0x200000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        == 0 as crate::src::ext::rtree::rtree::U32_0
    {
        *pi = a as ::core::ffi::c_int;
        return 3 as ::core::ffi::c_int;
    }
    let fresh5 = ptr;
    ptr = ptr.offset(1);
    a = a & 0x1fffff as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        | ((*fresh5 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0;
    if a & 0x10000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        == 0 as crate::src::ext::rtree::rtree::U32_0
    {
        *pi = a as ::core::ffi::c_int;
        return 4 as ::core::ffi::c_int;
    }
    a &= 0xfffffff as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
    *pi = (a
        | ((*ptr as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0)
            << 28 as ::core::ffi::c_int) as ::core::ffi::c_int;
    5 as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3VarintLen(
    mut v: crate::src::headers::sqlite3_h::Sqlite3Uint64,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        i += 1;
        v >>= 7 as ::core::ffi::c_int;
        if (v == 0 as crate::src::headers::sqlite3_h::Sqlite3Uint64) {
            break;
        }
    }
    i
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3Dequote(z: *mut ::core::ffi::c_char) {
    let mut quote: ::core::ffi::c_char;
    quote = *z.offset(0_isize);
    if quote as ::core::ffi::c_int == '[' as i32
        || quote as ::core::ffi::c_int == '\'' as i32
        || quote as ::core::ffi::c_int == '"' as i32
        || quote as ::core::ffi::c_int == '`' as i32
    {
        let mut iIn: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut iOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if quote as ::core::ffi::c_int == '[' as i32 {
            quote = ']' as i32 as ::core::ffi::c_char;
        }
        while *z.offset(iIn as isize) != 0 {
            if *z.offset(iIn as isize) as ::core::ffi::c_int == quote as ::core::ffi::c_int {
                if *z.offset((iIn + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    != quote as ::core::ffi::c_int
                {
                    break;
                }
                let fresh13 = iOut;
                iOut += 1;
                *z.offset(fresh13 as isize) = quote;
                iIn += 2 as ::core::ffi::c_int;
            } else {
                let fresh14 = iIn;
                iIn += 1;
                let fresh15 = iOut;
                iOut += 1;
                *z.offset(fresh15 as isize) = *z.offset(fresh14 as isize);
            }
        }
        *z.offset(iOut as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
}

unsafe extern "C" fn fts3GetDeltaVarint(
    pp: *mut *mut ::core::ffi::c_char,
    pVal: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
) {
    let mut iVal: crate::src::headers::sqlite3_h::Sqlite3Int64 = 0;
    *pp = (*pp).offset(sqlite3Fts3GetVarint(*pp, &raw mut iVal) as isize);
    *pVal += iVal;
}

unsafe extern "C" fn fts3GetReverseVarint(
    pp: *mut *mut ::core::ffi::c_char,
    pStart: *mut ::core::ffi::c_char,
    pVal: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
) {
    let mut iVal: crate::src::headers::sqlite3_h::Sqlite3Int64 = 0;
    let mut p: *mut ::core::ffi::c_char;
    p = (*pp).offset(-(2 as ::core::ffi::c_int as isize));
    while p >= pStart && *p as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
        p = p.offset(-1);
    }
    p = p.offset(1);
    *pp = p;
    sqlite3Fts3GetVarint(p, &raw mut iVal);
    *pVal = iVal;
}

unsafe extern "C" fn fts3DisconnectMethod(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
) -> ::core::ffi::c_int {
    let p: *mut crate::fts3Int_h::Fts3Table = pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut i: ::core::ffi::c_int;
    let __p_ref = unsafe { &mut *p };
    crate::src::src::vdbeapi::sqlite3_finalize(__p_ref.pSeekStmt);
    i = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[*mut crate::src::headers::sqlite3_h::Sqlite3Stmt; 40]>()
            as usize)
            .wrapping_div(
                ::core::mem::size_of::<*mut crate::src::headers::sqlite3_h::Sqlite3Stmt>()
                    as usize,
            ) as ::core::ffi::c_int
    {
        crate::src::src::vdbeapi::sqlite3_finalize(__p_ref.aStmt[i as usize]);
        i += 1;
    }
    crate::src::src::malloc::sqlite3_free(__p_ref.zSegmentsTbl as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(__p_ref.zReadExprlist as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(__p_ref.zWriteExprlist as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(__p_ref.zContentTbl as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(__p_ref.zLanguageid as *mut ::core::ffi::c_void);
    (*(*__p_ref.pTokenizer).pModule)
        .xDestroy
        .expect("non-null function pointer")(__p_ref.pTokenizer);
    crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3DestroyMethod(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
) -> ::core::ffi::c_int {
    let p: *mut crate::fts3Int_h::Fts3Table = pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __p_ref = unsafe { &mut *p };
    let zDb: *const ::core::ffi::c_char = __p_ref.zDb;
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __p_ref.db;
    fts3_db_exec_sql(
        &raw mut rc,
        db,
        crate::sqlite_printf!(
            "DROP TABLE IF EXISTS %Q.'%q_segments';DROP TABLE IF EXISTS %Q.'%q_segdir';DROP TABLE IF EXISTS %Q.'%q_docsize';DROP TABLE IF EXISTS %Q.'%q_stat';%s DROP TABLE IF EXISTS %Q.'%q_content';",
            zDb,
            __p_ref.zName,
            zDb,
            __p_ref.zName,
            zDb,
            __p_ref.zName,
            zDb,
            __p_ref.zName,
            if !__p_ref.zContentTbl.is_null() {
                b"--\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            zDb,
            __p_ref.zName,
        ),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        fts3DisconnectMethod(pVtab)
    } else {
        rc
    }
}

unsafe extern "C" fn fts3DeclareVtab(
    pRc: *mut ::core::ffi::c_int,
    p: *mut crate::fts3Int_h::Fts3Table,
) {
    if *pRc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut i: ::core::ffi::c_int;
        let rc: ::core::ffi::c_int;
        
        let mut zCols: *mut ::core::ffi::c_char;
        
        let __p_ref = unsafe { &mut *p };
        let zLanguageid: *const ::core::ffi::c_char = if !__p_ref.zLanguageid.is_null() {
            __p_ref.zLanguageid as *const ::core::ffi::c_char
        } else {
            b"__langid\0" as *const u8 as *const ::core::ffi::c_char
        };
        {
            let _vtab_args: [u64; 1] = [1u64];
            crate::src::src::vtab::sqlite3_vtab_config_args(
                __p_ref.db,
                crate::src::headers::sqlite3_h::SQLITE_VTAB_CONSTRAINT_SUPPORT,
                _vtab_args.as_ptr(),
            );
        }
        {
            let _vtab_args: [u64; 1] = [0];
            crate::src::src::vtab::sqlite3_vtab_config_args(
                __p_ref.db,
                crate::src::headers::sqlite3_h::SQLITE_VTAB_INNOCUOUS,
                _vtab_args.as_ptr(),
            );
        }
        zCols = crate::sqlite_printf!("%Q, ", *__p_ref.azColumn.offset(0_isize));
        i = 1 as ::core::ffi::c_int;
        while !zCols.is_null() && i < __p_ref.nColumn {
            zCols = crate::sqlite_printf!("%z%Q, ", zCols, *__p_ref.azColumn.offset(i as isize));
            i += 1;
        }
        let zSql: *mut ::core::ffi::c_char = crate::sqlite_printf!(
            "CREATE TABLE x(%s %Q HIDDEN, docid HIDDEN, %Q HIDDEN)",
            zCols,
            __p_ref.zName,
            zLanguageid,
        );
        if zCols.is_null() || zSql.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            rc = crate::src::src::vtab::sqlite3_declare_vtab(__p_ref.db, zSql);
        }
        crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(zCols as *mut ::core::ffi::c_void);
        *pRc = rc;
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3CreateStatTable(
    pRc: *mut ::core::ffi::c_int,
    p: *mut crate::fts3Int_h::Fts3Table,
) {
    let __p_ref = unsafe { &mut *p };
    fts3_db_exec_sql(
        pRc,
        __p_ref.db,
        crate::sqlite_printf!(
            "CREATE TABLE IF NOT EXISTS %Q.'%q_stat'(id INTEGER PRIMARY KEY, value BLOB);",
            __p_ref.zDb,
            __p_ref.zName
        ),
    );
    if *pRc == crate::src::headers::sqlite3_h::SQLITE_OK {
        __p_ref.bHasStat = 1 as crate::src::ext::rtree::rtree::U8_0;
    }
}

unsafe extern "C" fn fts3CreateTables(
    p: *mut crate::fts3Int_h::Fts3Table,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut i: ::core::ffi::c_int;
    let __p_ref = unsafe { &mut *p };
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __p_ref.db;
    if __p_ref.zContentTbl.is_null() {
        let zLanguageid: *const ::core::ffi::c_char = __p_ref.zLanguageid;
        let mut zContentCols: *mut ::core::ffi::c_char;
        zContentCols = crate::sqlite_printf!("docid INTEGER PRIMARY KEY");
        i = 0 as ::core::ffi::c_int;
        while !zContentCols.is_null() && i < __p_ref.nColumn {
            let z: *mut ::core::ffi::c_char = *__p_ref.azColumn.offset(i as isize);
            zContentCols = crate::sqlite_printf!("%z, 'c%d%q'", zContentCols, i, z);
            i += 1;
        }
        if !zLanguageid.is_null() && !zContentCols.is_null() {
            zContentCols = crate::sqlite_printf!("%z, langid", zContentCols);
        }
        if zContentCols.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        fts3_db_exec_sql(
            &raw mut rc,
            db,
            crate::sqlite_printf!(
                "CREATE TABLE %Q.'%q_content'(%s)",
                __p_ref.zDb,
                __p_ref.zName,
                zContentCols
            ),
        );
        crate::src::src::malloc::sqlite3_free(zContentCols as *mut ::core::ffi::c_void);
    }
    fts3_db_exec_sql(
        &raw mut rc,
        db,
        crate::sqlite_printf!(
            "CREATE TABLE %Q.'%q_segments'(blockid INTEGER PRIMARY KEY, block BLOB);",
            __p_ref.zDb,
            __p_ref.zName
        ),
    );
    fts3_db_exec_sql(
        &raw mut rc,
        db,
        crate::sqlite_printf!(
            "CREATE TABLE %Q.'%q_segdir'(level INTEGER,idx INTEGER,start_block INTEGER,leaves_end_block INTEGER,end_block INTEGER,root BLOB,PRIMARY KEY(level, idx));",
            __p_ref.zDb,
            __p_ref.zName
        ),
    );
    if __p_ref.bHasDocsize != 0 {
        fts3_db_exec_sql(
            &raw mut rc,
            db,
            crate::sqlite_printf!(
                "CREATE TABLE %Q.'%q_docsize'(docid INTEGER PRIMARY KEY, size BLOB);",
                __p_ref.zDb,
                __p_ref.zName
            ),
        );
    }
    if __p_ref.bHasStat != 0 {
        sqlite3Fts3CreateStatTable(&raw mut rc, p);
    }
    rc
}

unsafe extern "C" fn fts3DatabasePageSize(
    pRc: *mut ::core::ffi::c_int,
    p: *mut crate::fts3Int_h::Fts3Table,
) {
    if *pRc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut rc: ::core::ffi::c_int;
        
        let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        let zSql: *mut ::core::ffi::c_char = crate::sqlite_printf!("PRAGMA %Q.page_size", (*p).zDb);
        if zSql.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            rc = crate::src::src::prepare::sqlite3_prepare(
                (*p).db,
                zSql,
                -(1 as ::core::ffi::c_int),
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                crate::src::src::vdbeapi::sqlite3_step(pStmt);
                (*p).nPgsz =
                    crate::src::src::vdbeapi::sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
                rc = crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
            } else if rc == crate::src::headers::sqlite3_h::SQLITE_AUTH {
                (*p).nPgsz = 1024 as ::core::ffi::c_int;
                rc = crate::src::headers::sqlite3_h::SQLITE_OK;
            }
        }
        crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
        *pRc = rc;
    }
}

unsafe extern "C" fn fts3IsSpecialColumn(
    z: *const ::core::ffi::c_char,
    pnKey: *mut ::core::ffi::c_int,
    pzValue: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    
    let mut zCsr: *const ::core::ffi::c_char = z;
    while *zCsr as ::core::ffi::c_int != '=' as i32 {
        if *zCsr as ::core::ffi::c_int == '\0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
        zCsr = zCsr.offset(1);
    }
    *pnKey = zCsr.offset_from(z) as ::core::ffi::c_long as ::core::ffi::c_int;
    let zValue: *mut ::core::ffi::c_char = crate::sqlite_printf!("%s", zCsr.offset(1_isize) as *const ::core::ffi::c_char);
    if !zValue.is_null() {
        sqlite3Fts3Dequote(zValue);
    }
    *pzValue = zValue;
    1 as ::core::ffi::c_int
}

unsafe extern "C" fn fts3QuoteId(
    zInput: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    
    
    let nRet: crate::src::headers::sqlite3_h::Sqlite3Int64 = (2 as ::core::ffi::c_int
        + ::libc::strlen(zInput) as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let zRet: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_malloc64(
        nRet as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut ::core::ffi::c_char;
    if !zRet.is_null() {
        let mut i: ::core::ffi::c_int;
        let mut z: *mut ::core::ffi::c_char = zRet;
        let fresh50 = z;
        z = z.offset(1);
        *fresh50 = '"' as i32 as ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while *zInput.offset(i as isize) != 0 {
            if *zInput.offset(i as isize) as ::core::ffi::c_int == '"' as i32 {
                let fresh51 = z;
                z = z.offset(1);
                *fresh51 = '"' as i32 as ::core::ffi::c_char;
            }
            let fresh52 = z;
            z = z.offset(1);
            *fresh52 = *zInput.offset(i as isize);
            i += 1;
        }
        let fresh53 = z;
        z = z.offset(1);
        *fresh53 = '"' as i32 as ::core::ffi::c_char;
        let fresh54 = z;
        *fresh54 = '\0' as i32 as ::core::ffi::c_char;
    }
    zRet
}

unsafe extern "C" fn fts3ReadExprList(
    p: *mut crate::fts3Int_h::Fts3Table,
    zFunc: *const ::core::ffi::c_char,
    pRc: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut zRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zFree: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let zFunction: *mut ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int;
    let __p_ref = unsafe { &mut *p };
    if __p_ref.zContentTbl.is_null() {
        if zFunc.is_null() {
            zFunction =
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        } else {
            zFunction = fts3QuoteId(zFunc);
            zFree = zFunction;
        }
        fts3_appendf_raw(pRc, &raw mut zRet, crate::sqlite_printf!("docid"));
        i = 0 as ::core::ffi::c_int;
        while i < __p_ref.nColumn {
            fts3_appendf_raw(
                pRc,
                &raw mut zRet,
                crate::sqlite_printf!(
                    ",%s(x.'c%d%q')",
                    zFunction,
                    i,
                    *__p_ref.azColumn.offset(i as isize)
                ),
            );
            i += 1;
        }
        if !__p_ref.zLanguageid.is_null() {
            fts3_appendf_raw(
                pRc,
                &raw mut zRet,
                crate::sqlite_printf!(
                    ", x.%Q",
                    b"langid\0" as *const u8 as *const ::core::ffi::c_char
                ),
            );
        }
        crate::src::src::malloc::sqlite3_free(zFree as *mut ::core::ffi::c_void);
    } else {
        fts3_appendf_raw(pRc, &raw mut zRet, crate::sqlite_printf!("rowid"));
        i = 0 as ::core::ffi::c_int;
        while i < __p_ref.nColumn {
            fts3_appendf_raw(
                pRc,
                &raw mut zRet,
                crate::sqlite_printf!(", x.'%q'", *__p_ref.azColumn.offset(i as isize)),
            );
            i += 1;
        }
        if !__p_ref.zLanguageid.is_null() {
            fts3_appendf_raw(
                pRc,
                &raw mut zRet,
                crate::sqlite_printf!(", x.%Q", __p_ref.zLanguageid),
            );
        }
    }
    fts3_appendf_raw(
        pRc,
        &raw mut zRet,
        crate::sqlite_printf!(
            " FROM '%q'.'%q%s' AS x",
            __p_ref.zDb,
            if !__p_ref.zContentTbl.is_null() {
                __p_ref.zContentTbl as *const ::core::ffi::c_char
            } else {
                __p_ref.zName
            },
            if !__p_ref.zContentTbl.is_null() {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"_content\0" as *const u8 as *const ::core::ffi::c_char
            },
        ),
    );
    zRet
}

unsafe extern "C" fn fts3WriteExprList(
    p: *mut crate::fts3Int_h::Fts3Table,
    zFunc: *const ::core::ffi::c_char,
    pRc: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut zRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zFree: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let zFunction: *mut ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int;
    if zFunc.is_null() {
        zFunction = b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    } else {
        zFunction = fts3QuoteId(zFunc);
        zFree = zFunction;
    }
    fts3_appendf_raw(pRc, &raw mut zRet, crate::sqlite_printf!("?"));
    i = 0 as ::core::ffi::c_int;
    while i < (*p).nColumn {
        fts3_appendf_raw(
            pRc,
            &raw mut zRet,
            crate::sqlite_printf!(",%s(?)", zFunction),
        );
        i += 1;
    }
    if !(*p).zLanguageid.is_null() {
        fts3_appendf_raw(pRc, &raw mut zRet, crate::sqlite_printf!(", ?"));
    }
    crate::src::src::malloc::sqlite3_free(zFree as *mut ::core::ffi::c_void);
    zRet
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3ReadInt(
    z: *const ::core::ffi::c_char,
    pnOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut iVal: crate::src::ext::rtree::rtree::U64_0 = 0 as crate::src::ext::rtree::rtree::U64_0;
    let mut i: ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while *z.offset(i as isize) as ::core::ffi::c_int >= '0' as i32
        && *z.offset(i as isize) as ::core::ffi::c_int <= '9' as i32
    {
        iVal = iVal
            .wrapping_mul(10 as crate::src::ext::rtree::rtree::U64_0)
            .wrapping_add(
                (*z.offset(i as isize) as ::core::ffi::c_int - '0' as i32)
                    as crate::src::ext::rtree::rtree::U64_0,
            );
        if iVal > 0x7fffffff as crate::src::ext::rtree::rtree::U64_0 {
            return -(1 as ::core::ffi::c_int);
        }
        i += 1;
    }
    *pnOut = iVal as ::core::ffi::c_int;
    i
}

unsafe extern "C" fn fts3GobbleInt(
    pp: *mut *const ::core::ffi::c_char,
    pnOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let MAX_NPREFIX: ::core::ffi::c_int = 10000000 as ::core::ffi::c_int;
    let mut nInt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    let nByte: ::core::ffi::c_int = sqlite3Fts3ReadInt(*pp, &raw mut nInt);
    if nInt > MAX_NPREFIX {
        nInt = 0 as ::core::ffi::c_int;
    }
    if nByte == 0 as ::core::ffi::c_int {
        return crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    *pnOut = nInt;
    *pp = (*pp).offset(nByte as isize);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3PrefixParameter(
    zParam: *const ::core::ffi::c_char,
    pnIndex: *mut ::core::ffi::c_int,
    apIndex: *mut *mut crate::fts3Int_h::Fts3Index,
) -> ::core::ffi::c_int {
    
    let mut nIndex: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if !zParam.is_null() && *zParam.offset(0_isize) as ::core::ffi::c_int != 0 {
        let mut p: *const ::core::ffi::c_char;
        nIndex += 1;
        p = zParam;
        while *p != 0 {
            if *p as ::core::ffi::c_int == ',' as i32 {
                nIndex += 1;
            }
            p = p.offset(1);
        }
    }
    let aIndex: *mut crate::fts3Int_h::Fts3Index = crate::src::src::malloc::sqlite3_malloc64(
        (::core::mem::size_of::<crate::fts3Int_h::Fts3Index>() as usize)
            .wrapping_mul(nIndex as usize)
            as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut crate::fts3Int_h::Fts3Index;
    *apIndex = aIndex;
    if aIndex.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        aIndex as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<crate::fts3Int_h::Fts3Index>() as crate::__stddef_size_t_h::SizeT)
            .wrapping_mul(nIndex as crate::__stddef_size_t_h::SizeT),
    );
    if !zParam.is_null() {
        let mut p_0: *const ::core::ffi::c_char = zParam;
        let mut i: ::core::ffi::c_int;
        i = 1 as ::core::ffi::c_int;
        while i < nIndex {
            let mut nPrefix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if fts3GobbleInt(&raw mut p_0, &raw mut nPrefix) != 0 {
                return crate::src::headers::sqlite3_h::SQLITE_ERROR;
            }
            if nPrefix == 0 as ::core::ffi::c_int {
                nIndex -= 1;
                i -= 1;
            } else {
                (*aIndex.offset(i as isize)).nPrefix = nPrefix;
            }
            p_0 = p_0.offset(1);
            i += 1;
        }
    }
    *pnIndex = nIndex;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3ContentColumns(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zDb: *const ::core::ffi::c_char,
    zTbl: *const ::core::ffi::c_char,
    pazCol: *mut *mut *const ::core::ffi::c_char,
    pnCol: *mut ::core::ffi::c_int,
    pnStr: *mut ::core::ffi::c_int,
    pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let zSql: *mut ::core::ffi::c_char = crate::sqlite_printf!("SELECT * FROM %Q.%Q", zDb, zTbl);
    if zSql.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    } else {
        rc = crate::src::src::prepare::sqlite3_prepare(
            db,
            zSql,
            -(1 as ::core::ffi::c_int),
            &raw mut pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::malloc::sqlite3_free(*pzErr as *mut ::core::ffi::c_void);
            *pzErr = crate::sqlite_printf!("%s", crate::src::src::main::sqlite3_errmsg(db));
        }
    }
    crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        
        let mut nStr: crate::src::headers::sqlite3_h::Sqlite3Int64 =
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        
        let mut i: ::core::ffi::c_int;
        let nCol: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_column_count(pStmt);
        i = 0 as ::core::ffi::c_int;
        while i < nCol {
            let zCol: *const ::core::ffi::c_char =
                crate::src::src::vdbeapi::sqlite3_column_name(pStmt, i);
            nStr = (nStr as ::core::ffi::c_ulonglong).wrapping_add(
                ::libc::strlen(zCol).wrapping_add(1 as crate::__stddef_size_t_h::SizeT)
                    as ::core::ffi::c_ulonglong,
            ) as crate::src::headers::sqlite3_h::Sqlite3Int64
                as crate::src::headers::sqlite3_h::Sqlite3Int64;
            i += 1;
        }
        let azCol: *mut *const ::core::ffi::c_char = crate::src::src::malloc::sqlite3_malloc64(
            ((::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                .wrapping_mul(nCol as usize)
                as crate::src::headers::sqlite3_h::Sqlite3Uint64)
                .wrapping_add(nStr as crate::src::headers::sqlite3_h::Sqlite3Uint64),
        ) as *mut *const ::core::ffi::c_char;
        if azCol.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            let mut p: *mut ::core::ffi::c_char = azCol.offset(nCol as isize)
                as *mut *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            i = 0 as ::core::ffi::c_int;
            while i < nCol {
                let zCol_0: *const ::core::ffi::c_char =
                    crate::src::src::vdbeapi::sqlite3_column_name(pStmt, i);
                let n: ::core::ffi::c_int =
                    ::libc::strlen(zCol_0) as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                ::core::ptr::copy_nonoverlapping(zCol_0 as *const u8, p as *mut u8, n as usize);
                let fresh55 = &mut *azCol.offset(i as isize);
                *fresh55 = p;
                p = p.offset(n as isize);
                i += 1;
            }
        }
        crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
        *pnCol = nCol;
        *pnStr = nStr as ::core::ffi::c_int;
        *pazCol = azCol;
    }
    rc
}

unsafe extern "C" fn fts3InitVtab(
    isCreate: ::core::ffi::c_int,
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pAux: *mut ::core::ffi::c_void,
    argc: ::core::ffi::c_int,
    argv: *const *const ::core::ffi::c_char,
    ppVTab: *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let current_block: u64;
    let pHash: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash =
        &raw mut (*(pAux as *mut Fts3HashWrapper)).hash;
    let mut p: *mut crate::fts3Int_h::Fts3Table =
        ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Table>();
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut i: ::core::ffi::c_int;
    let mut nByte: crate::src::headers::sqlite3_h::Sqlite3Int64;
    let mut iCol: ::core::ffi::c_int;
    let mut nString: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut zCsr: *mut ::core::ffi::c_char;
    
    
    let isFts4: ::core::ffi::c_int = (*(*argv.offset(0_isize)).offset(3_isize)
        as ::core::ffi::c_int
        == '4' as i32) as ::core::ffi::c_int;
    let mut aCol: *mut *const ::core::ffi::c_char;
    let mut pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer =
        ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer>();
    let mut nIndex: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut aIndex: *mut crate::fts3Int_h::Fts3Index =
        ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Index>();
    let mut bNoDocsize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bDescIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut zPrefix: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zCompress: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zUncompress: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zContent: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zLanguageid: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut azNotindexed: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut nNotindexed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let nDb: ::core::ffi::c_int = ::libc::strlen(*argv.offset(1_isize)) as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    let nName: ::core::ffi::c_int = ::libc::strlen(*argv.offset(2_isize)) as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    nByte = (::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
        .wrapping_mul((argc - 2 as ::core::ffi::c_int) as usize)
        as crate::src::headers::sqlite3_h::Sqlite3Int64;
    aCol = crate::src::src::malloc::sqlite3_malloc64(
        nByte as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut *const ::core::ffi::c_char;
    if !aCol.is_null() {
        ::libc::memset(
            aCol as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            nByte as crate::__stddef_size_t_h::SizeT,
        );
        azNotindexed = crate::src::src::malloc::sqlite3_malloc64(
            nByte as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut *mut ::core::ffi::c_char;
    }
    if !azNotindexed.is_null() {
        ::libc::memset(
            azNotindexed as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            nByte as crate::__stddef_size_t_h::SizeT,
        );
    }
    if aCol.is_null() || azNotindexed.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    } else {
        i = 3 as ::core::ffi::c_int;
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < argc {
            let z: *const ::core::ffi::c_char = *argv.offset(i as isize);
            let mut nKey: ::core::ffi::c_int = 0;
            let mut zVal: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if pTokenizer.is_null()
                && ::libc::strlen(z) > 8 as crate::__stddef_size_t_h::SizeT
                && 0 as ::core::ffi::c_int
                    == crate::src::src::util::sqlite3_strnicmp(
                        z,
                        b"tokenize\0" as *const u8 as *const ::core::ffi::c_char,
                        8 as ::core::ffi::c_int,
                    )
                && 0 as ::core::ffi::c_int
                    == crate::src::ext::fts3::fts3_tokenizer::sqlite3Fts3IsIdChar(
                        *z.offset(8_isize),
                    )
            {
                rc = crate::src::ext::fts3::fts3_tokenizer::sqlite3Fts3InitTokenizer(
                    pHash as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
                    z.offset(9_isize) as *const ::core::ffi::c_char,
                    &raw mut pTokenizer as *mut _
                        as *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
                    pzErr,
                );
            } else if isFts4 != 0 && fts3IsSpecialColumn(z, &raw mut nKey, &raw mut zVal) != 0 {
                let mut aFts4Opt: [Fts4Option; 8] = [
                    Fts4Option {
                        zOpt: b"matchinfo\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 9 as ::core::ffi::c_int,
                    },
                    Fts4Option {
                        zOpt: b"prefix\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 6 as ::core::ffi::c_int,
                    },
                    Fts4Option {
                        zOpt: b"compress\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 8 as ::core::ffi::c_int,
                    },
                    Fts4Option {
                        zOpt: b"uncompress\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 10 as ::core::ffi::c_int,
                    },
                    Fts4Option {
                        zOpt: b"order\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 5 as ::core::ffi::c_int,
                    },
                    Fts4Option {
                        zOpt: b"content\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 7 as ::core::ffi::c_int,
                    },
                    Fts4Option {
                        zOpt: b"languageid\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 10 as ::core::ffi::c_int,
                    },
                    Fts4Option {
                        zOpt: b"notindexed\0" as *const u8 as *const ::core::ffi::c_char,
                        nOpt: 10 as ::core::ffi::c_int,
                    },
                ];
                let mut iOpt: ::core::ffi::c_int;
                if zVal.is_null() {
                    rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                } else {
                    iOpt = 0 as ::core::ffi::c_int;
                    while iOpt
                        < (::core::mem::size_of::<[Fts4Option; 8]>() as usize)
                            .wrapping_div(::core::mem::size_of::<Fts4Option>() as usize)
                            as ::core::ffi::c_int
                    {
                        let pOp: *mut Fts4Option = (&raw mut aFts4Opt as *mut Fts4Option)
                            .offset(iOpt as isize)
                            as *mut Fts4Option;
                        let __pOp_ref = unsafe { &*pOp };
                        if nKey == __pOp_ref.nOpt
                            && crate::src::src::util::sqlite3_strnicmp(
                                z,
                                __pOp_ref.zOpt,
                                __pOp_ref.nOpt,
                            ) == 0
                        {
                            break;
                        }
                        iOpt += 1;
                    }
                    match iOpt {
                        0 => {
                            if ::libc::strlen(zVal) != 4 as crate::__stddef_size_t_h::SizeT
                                || crate::src::src::util::sqlite3_strnicmp(
                                    zVal,
                                    b"fts3\0" as *const u8 as *const ::core::ffi::c_char,
                                    4 as ::core::ffi::c_int,
                                ) != 0
                            {
                                crate::src::src::malloc::sqlite3_free(
                                    *pzErr as *mut ::core::ffi::c_void,
                                );
                                *pzErr = crate::sqlite_printf!("unrecognized matchinfo: %s", zVal);
                                rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                            }
                            bNoDocsize = 1 as ::core::ffi::c_int;
                        }
                        1 => {
                            crate::src::src::malloc::sqlite3_free(
                                zPrefix as *mut ::core::ffi::c_void,
                            );
                            zPrefix = zVal;
                            zVal = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                        2 => {
                            crate::src::src::malloc::sqlite3_free(
                                zCompress as *mut ::core::ffi::c_void,
                            );
                            zCompress = zVal;
                            zVal = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                        3 => {
                            crate::src::src::malloc::sqlite3_free(
                                zUncompress as *mut ::core::ffi::c_void,
                            );
                            zUncompress = zVal;
                            zVal = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                        4 => {
                            if (::libc::strlen(zVal) != 3 as crate::__stddef_size_t_h::SizeT
                                || crate::src::src::util::sqlite3_strnicmp(
                                    zVal,
                                    b"asc\0" as *const u8 as *const ::core::ffi::c_char,
                                    3 as ::core::ffi::c_int,
                                ) != 0)
                                && (::libc::strlen(zVal) != 4 as crate::__stddef_size_t_h::SizeT
                                    || crate::src::src::util::sqlite3_strnicmp(
                                        zVal,
                                        b"desc\0" as *const u8 as *const ::core::ffi::c_char,
                                        4 as ::core::ffi::c_int,
                                    ) != 0)
                            {
                                crate::src::src::malloc::sqlite3_free(
                                    *pzErr as *mut ::core::ffi::c_void,
                                );
                                *pzErr = crate::sqlite_printf!("unrecognized order: %s", zVal);
                                rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                            }
                            bDescIdx = (*zVal.offset(0_isize) as ::core::ffi::c_int
                                == 'd' as i32
                                || *zVal.offset(0_isize) as ::core::ffi::c_int == 'D' as i32)
                                as ::core::ffi::c_int;
                        }
                        5 => {
                            crate::src::src::malloc::sqlite3_free(
                                zContent as *mut ::core::ffi::c_void,
                            );
                            zContent = zVal;
                            zVal = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                        6 => {
                            crate::src::src::malloc::sqlite3_free(
                                zLanguageid as *mut ::core::ffi::c_void,
                            );
                            zLanguageid = zVal;
                            zVal = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                        7 => {
                            let fresh42 = nNotindexed;
                            nNotindexed += 1;
                            let fresh43 = &mut *azNotindexed.offset(fresh42 as isize);
                            *fresh43 = zVal;
                            zVal = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                        _ => {
                            crate::src::src::malloc::sqlite3_free(
                                *pzErr as *mut ::core::ffi::c_void,
                            );
                            *pzErr = crate::sqlite_printf!("unrecognized parameter: %s", z);
                            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                        }
                    }
                    crate::src::src::malloc::sqlite3_free(zVal as *mut ::core::ffi::c_void);
                }
            } else {
                nString += ::libc::strlen(z).wrapping_add(1 as crate::__stddef_size_t_h::SizeT)
                    as ::core::ffi::c_int;
                let fresh44 = nCol;
                nCol += 1;
                let fresh45 = &mut *aCol.offset(fresh44 as isize);
                *fresh45 = z;
            }
            i += 1;
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !zContent.is_null() {
            crate::src::src::malloc::sqlite3_free(zCompress as *mut ::core::ffi::c_void);
            crate::src::src::malloc::sqlite3_free(zUncompress as *mut ::core::ffi::c_void);
            zCompress = ::core::ptr::null_mut::<::core::ffi::c_char>();
            zUncompress = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if nCol == 0 as ::core::ffi::c_int {
                crate::src::src::malloc::sqlite3_free(aCol as *mut ::core::ffi::c_void);
                aCol = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
                rc = fts3ContentColumns(
                    db,
                    *argv.offset(1_isize),
                    zContent,
                    &raw mut aCol,
                    &raw mut nCol,
                    &raw mut nString,
                    pzErr,
                );
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !zLanguageid.is_null() {
                    let mut j: ::core::ffi::c_int;
                    j = 0 as ::core::ffi::c_int;
                    while j < nCol {
                        if crate::src::src::util::sqlite3_stricmp(
                            zLanguageid,
                            *aCol.offset(j as isize),
                        ) == 0 as ::core::ffi::c_int
                        {
                            let mut k: ::core::ffi::c_int;
                            k = j;
                            while k < nCol {
                                let fresh46 = &mut *aCol.offset(k as isize);
                                *fresh46 = *aCol.offset((k + 1 as ::core::ffi::c_int) as isize);
                                k += 1;
                            }
                            nCol -= 1;
                            break;
                        } else {
                            j += 1;
                        }
                    }
                }
            }
        }
        if (rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
            if nCol == 0 as ::core::ffi::c_int {
                let fresh47 = &mut *aCol.offset(0_isize);
                *fresh47 = b"content\0" as *const u8 as *const ::core::ffi::c_char;
                nString = 8 as ::core::ffi::c_int;
                nCol = 1 as ::core::ffi::c_int;
            }
            if pTokenizer.is_null() {
                rc = crate::src::ext::fts3::fts3_tokenizer::sqlite3Fts3InitTokenizer(
                    pHash as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
                    b"simple\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut pTokenizer as *mut _
                        as *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
                    pzErr,
                );
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    current_block = 4597764963221154413;
                } else {
                    current_block = 18425699056680496821;
                }
            } else {
                current_block = 18425699056680496821;
            }
            match current_block {
                4597764963221154413 => {}
                _ => {
                    rc = fts3PrefixParameter(zPrefix, &raw mut nIndex, &raw mut aIndex);
                    if rc == crate::src::headers::sqlite3_h::SQLITE_ERROR {
                        crate::src::src::malloc::sqlite3_free(*pzErr as *mut ::core::ffi::c_void);
                        *pzErr =
                            crate::sqlite_printf!("error parsing prefix parameter: %s", zPrefix);
                    }
                    if (rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
                        nByte =
                            (::core::mem::size_of::<crate::fts3Int_h::Fts3Table>() as usize)
                                .wrapping_add((nCol as usize).wrapping_mul(::core::mem::size_of::<
                                    *mut ::core::ffi::c_char,
                                >(
                                )
                                    as usize))
                                .wrapping_add((nIndex as usize).wrapping_mul(
                                    ::core::mem::size_of::<crate::fts3Int_h::Fts3Index>() as usize,
                                ))
                                .wrapping_add((nCol as usize).wrapping_mul(::core::mem::size_of::<
                                    crate::src::ext::rtree::rtree::U8_0,
                                >(
                                )
                                    as usize))
                                .wrapping_add(nName as usize)
                                .wrapping_add(nDb as usize)
                                .wrapping_add(nString as usize)
                                as crate::src::headers::sqlite3_h::Sqlite3Int64;
                        p = crate::src::src::malloc::sqlite3_malloc64(
                            nByte as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                        ) as *mut crate::fts3Int_h::Fts3Table;
                        if p.is_null() {
                            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                        } else {
                            ::libc::memset(
                                p as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                nByte as crate::__stddef_size_t_h::SizeT,
                            );
                            let __p_ref = unsafe { &mut *p };
                            __p_ref.db = db;
                            __p_ref.nColumn = nCol;
                            __p_ref.nPendingData = 0 as ::core::ffi::c_int;
                            __p_ref.azColumn = p.offset(1_isize)
                                as *mut crate::fts3Int_h::Fts3Table
                                as *mut *mut ::core::ffi::c_char;
                            __p_ref.pTokenizer = pTokenizer;
                            __p_ref.nMaxPendingData = crate::fts3Int_h::FTS3_MAX_PENDING_DATA;
                            __p_ref.bHasDocsize = (isFts4 != 0
                                && bNoDocsize == 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                as crate::src::ext::rtree::rtree::U8_0;
                            __p_ref.bHasStat = isFts4 as crate::src::ext::rtree::rtree::U8_0;
                            __p_ref.bFts4 = isFts4 as crate::src::ext::rtree::rtree::U8_0;
                            __p_ref.bDescIdx = bDescIdx as crate::src::ext::rtree::rtree::U8_0;
                            __p_ref.nAutoincrmerge = 0xff as ::core::ffi::c_int;
                            __p_ref.zContentTbl = zContent;
                            __p_ref.zLanguageid = zLanguageid;
                            zContent = ::core::ptr::null_mut::<::core::ffi::c_char>();
                            zLanguageid = ::core::ptr::null_mut::<::core::ffi::c_char>();
                            __p_ref.aIndex = __p_ref.azColumn.offset(nCol as isize)
                                as *mut *mut ::core::ffi::c_char
                                as *mut crate::fts3Int_h::Fts3Index
                                as *mut crate::fts3Int_h::Fts3Index;
                            ::core::ptr::copy_nonoverlapping(
                                aIndex as *const u8,
                                __p_ref.aIndex as *mut u8,
                                ((::core::mem::size_of::<crate::fts3Int_h::Fts3Index>()
                                    as crate::__stddef_size_t_h::SizeT)
                                    .wrapping_mul(nIndex as crate::__stddef_size_t_h::SizeT))
                                    as usize,
                            );
                            __p_ref.nIndex = nIndex;
                            i = 0 as ::core::ffi::c_int;
                            while i < nIndex {
                                crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashInit(
                                    &raw mut (*__p_ref.aIndex.offset(i as isize)).hPending as *mut _
                                        as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
                                    crate::src::ext::fts3::fts3_hash::FTS3_HASH_STRING
                                        as ::core::ffi::c_char,
                                    1 as ::core::ffi::c_char,
                                );
                                i += 1;
                            }
                            __p_ref.abNotindexed = __p_ref.aIndex.offset(nIndex as isize)
                                as *mut crate::fts3Int_h::Fts3Index
                                as *mut crate::src::ext::rtree::rtree::U8_0;
                            zCsr = __p_ref.abNotindexed.offset(nCol as isize)
                                as *mut crate::src::ext::rtree::rtree::U8_0
                                as *mut ::core::ffi::c_char;
                            __p_ref.zName = zCsr;
                            ::core::ptr::copy_nonoverlapping(
                                *argv.offset(2_isize) as *const u8,
                                zCsr as *mut u8,
                                nName as usize,
                            );
                            zCsr = zCsr.offset(nName as isize);
                            __p_ref.zDb = zCsr;
                            ::core::ptr::copy_nonoverlapping(
                                *argv.offset(1_isize) as *const u8,
                                zCsr as *mut u8,
                                nDb as usize,
                            );
                            zCsr = zCsr.offset(nDb as isize);
                            iCol = 0 as ::core::ffi::c_int;
                            while iCol < nCol {
                                
                                let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                let z_0: *mut ::core::ffi::c_char = crate::src::ext::fts3::fts3_tokenizer::sqlite3Fts3NextToken(
                                    *aCol.offset(iCol as isize),
                                    &raw mut n,
                                ) as *mut ::core::ffi::c_char;
                                if n > 0 as ::core::ffi::c_int {
                                    ::core::ptr::copy_nonoverlapping(
                                        z_0 as *const u8,
                                        zCsr as *mut u8,
                                        n as usize,
                                    );
                                }
                                *zCsr.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
                                sqlite3Fts3Dequote(zCsr);
                                let fresh48 = &mut *__p_ref.azColumn.offset(iCol as isize);
                                *fresh48 = zCsr;
                                zCsr = zCsr.offset((n + 1 as ::core::ffi::c_int) as isize);
                                iCol += 1;
                            }
                            iCol = 0 as ::core::ffi::c_int;
                            while iCol < nCol {
                                let n_0: ::core::ffi::c_int =
                                    ::libc::strlen(*__p_ref.azColumn.offset(iCol as isize))
                                        as ::core::ffi::c_int;
                                i = 0 as ::core::ffi::c_int;
                                while i < nNotindexed {
                                    let zNot: *mut ::core::ffi::c_char =
                                        *azNotindexed.offset(i as isize);
                                    if !zNot.is_null()
                                        && n_0 == ::libc::strlen(zNot) as ::core::ffi::c_int
                                        && 0 as ::core::ffi::c_int
                                            == crate::src::src::util::sqlite3_strnicmp(
                                                *__p_ref.azColumn.offset(iCol as isize),
                                                zNot,
                                                n_0,
                                            )
                                    {
                                        *__p_ref.abNotindexed.offset(iCol as isize) =
                                            1 as crate::src::ext::rtree::rtree::U8_0;
                                        crate::src::src::malloc::sqlite3_free(
                                            zNot as *mut ::core::ffi::c_void,
                                        );
                                        let fresh49 = &mut *azNotindexed.offset(i as isize);
                                        *fresh49 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                                    }
                                    i += 1;
                                }
                                iCol += 1;
                            }
                            i = 0 as ::core::ffi::c_int;
                            while i < nNotindexed {
                                if !(*azNotindexed.offset(i as isize)).is_null() {
                                    crate::src::src::malloc::sqlite3_free(
                                        *pzErr as *mut ::core::ffi::c_void,
                                    );
                                    *pzErr = crate::sqlite_printf!(
                                        "no such column: %s",
                                        *azNotindexed.offset(i as isize)
                                    );
                                    rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                                }
                                i += 1;
                            }
                            if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                                && zCompress.is_null()
                                    as ::core::ffi::c_int
                                    != zUncompress.is_null()
                                        as ::core::ffi::c_int
                            {
                                let zMiss: *const ::core::ffi::c_char = if zCompress.is_null() {
                                    b"compress\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"uncompress\0" as *const u8 as *const ::core::ffi::c_char
                                };
                                rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                                crate::src::src::malloc::sqlite3_free(
                                    *pzErr as *mut ::core::ffi::c_void,
                                );
                                *pzErr = crate::sqlite_printf!(
                                    "missing %s parameter in fts4 constructor",
                                    zMiss
                                );
                            }
                            __p_ref.zReadExprlist = fts3ReadExprList(p, zUncompress, &raw mut rc);
                            __p_ref.zWriteExprlist = fts3WriteExprList(p, zCompress, &raw mut rc);
                            if (rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
                                if isCreate != 0 {
                                    rc = fts3CreateTables(p);
                                }
                                if isFts4 == 0 && isCreate == 0 {
                                    __p_ref.bHasStat = 2 as crate::src::ext::rtree::rtree::U8_0;
                                }
                                fts3DatabasePageSize(&raw mut rc, p);
                                __p_ref.nNodeSize = __p_ref.nPgsz - 35 as ::core::ffi::c_int;
                                __p_ref.nMergeCount = crate::fts3Int_h::FTS3_MERGE_COUNT;
                                fts3DeclareVtab(&raw mut rc, p);
                            }
                        }
                    }
                }
            }
        }
    }
    crate::src::src::malloc::sqlite3_free(zPrefix as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(aIndex as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(zCompress as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(zUncompress as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(zContent as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(zLanguageid as *mut ::core::ffi::c_void);
    i = 0 as ::core::ffi::c_int;
    while i < nNotindexed {
        crate::src::src::malloc::sqlite3_free(
            *azNotindexed.offset(i as isize) as *mut ::core::ffi::c_void
        );
        i += 1;
    }
    crate::src::src::malloc::sqlite3_free(aCol as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(azNotindexed as *mut ::core::ffi::c_void);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        if !p.is_null() {
            fts3DisconnectMethod(p as *mut crate::src::headers::sqlite3_h::sqlite3_vtab);
        } else if !pTokenizer.is_null() {
            (*(*pTokenizer).pModule)
                .xDestroy
                .expect("non-null function pointer")(pTokenizer);
        }
    } else {
        *ppVTab = &raw mut (*p).base;
    }
    rc
}

unsafe extern "C" fn fts3ConnectMethod(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pAux: *mut ::core::ffi::c_void,
    argc: ::core::ffi::c_int,
    argv: *const *const ::core::ffi::c_char,
    ppVtab: *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    fts3InitVtab(0 as ::core::ffi::c_int, db, pAux, argc, argv, ppVtab, pzErr)
}

unsafe extern "C" fn fts3CreateMethod(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pAux: *mut ::core::ffi::c_void,
    argc: ::core::ffi::c_int,
    argv: *const *const ::core::ffi::c_char,
    ppVtab: *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    fts3InitVtab(1 as ::core::ffi::c_int, db, pAux, argc, argv, ppVtab, pzErr)
}

unsafe extern "C" fn fts3SetEstimatedRows(
    pIdxInfo: *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
    nRow: crate::src::ext::rtree::rtree::I64_0,
) {
    if crate::src::src::main::sqlite3_libversion_number() >= 3008002 as ::core::ffi::c_int {
        (*pIdxInfo).estimatedRows = nRow as crate::src::headers::sqlite3_h::Sqlite3Int64;
    }
}

unsafe extern "C" fn fts3SetUniqueFlag(
    pIdxInfo: *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
) {
    if crate::src::src::main::sqlite3_libversion_number() >= 3008012 as ::core::ffi::c_int {
        (*pIdxInfo).idxFlags |= crate::src::headers::sqlite3_h::SQLITE_INDEX_SCAN_UNIQUE;
    }
}

unsafe extern "C" fn fts3BestIndexMethod(
    pVTab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    pInfo: *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
) -> ::core::ffi::c_int {
    let p: *mut crate::fts3Int_h::Fts3Table = pVTab as *mut crate::fts3Int_h::Fts3Table;
    let mut i: ::core::ffi::c_int;
    let mut iCons: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iLangidCons: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iDocidGe: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iDocidLe: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iIdx: ::core::ffi::c_int;
    if (*p).bLock != 0 {
        return crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    let __pInfo_ref = unsafe { &mut *pInfo };
    __pInfo_ref.idxNum = crate::fts3Int_h::FTS3_FULLSCAN_SEARCH;
    __pInfo_ref.estimatedCost = 5000000 as ::core::ffi::c_int as ::core::ffi::c_double;
    i = 0 as ::core::ffi::c_int;
    while i < __pInfo_ref.nConstraint {
        let bDocid: ::core::ffi::c_int;
        let pCons: *mut crate::src::headers::sqlite3_h::sqlite3_index_constraint =
            __pInfo_ref.aConstraint.offset(i as isize)
                as *mut crate::src::headers::sqlite3_h::sqlite3_index_constraint;
        if (*pCons).usable as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if (*pCons).op as ::core::ffi::c_int
                == crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_MATCH
            {
                __pInfo_ref.idxNum = crate::fts3Int_h::FTS3_FULLSCAN_SEARCH;
                __pInfo_ref.estimatedCost = 1e50f64;
                fts3SetEstimatedRows(
                    pInfo,
                    (1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::I64_0)
                        << 50 as ::core::ffi::c_int,
                );
                return crate::src::headers::sqlite3_h::SQLITE_OK;
            }
        } else {
            let __p_ref = unsafe { &*p };
            let __pCons_ref = unsafe { &*pCons };
            bDocid = (__pCons_ref.iColumn < 0 as ::core::ffi::c_int
                || __pCons_ref.iColumn == __p_ref.nColumn + 1 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if iCons < 0 as ::core::ffi::c_int
                && __pCons_ref.op as ::core::ffi::c_int
                    == crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_EQ
                && bDocid != 0
            {
                __pInfo_ref.idxNum = crate::fts3Int_h::FTS3_DOCID_SEARCH;
                __pInfo_ref.estimatedCost = 1.0f64;
                iCons = i;
            }
            if __pCons_ref.op as ::core::ffi::c_int
                == crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_MATCH
                && __pCons_ref.iColumn >= 0 as ::core::ffi::c_int
                && __pCons_ref.iColumn <= __p_ref.nColumn
            {
                __pInfo_ref.idxNum = crate::fts3Int_h::FTS3_FULLTEXT_SEARCH + __pCons_ref.iColumn;
                __pInfo_ref.estimatedCost = 2.0f64;
                iCons = i;
            }
            if __pCons_ref.op as ::core::ffi::c_int
                == crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_EQ
                && __pCons_ref.iColumn == __p_ref.nColumn + 2 as ::core::ffi::c_int
            {
                iLangidCons = i;
            }
            if bDocid != 0 {
                match __pCons_ref.op as ::core::ffi::c_int {
                    crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_GE
                    | crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_GT => {
                        iDocidGe = i;
                    }
                    crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_LE
                    | crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_LT => {
                        iDocidLe = i;
                    }
                    _ => {}
                }
            }
        }
        i += 1;
    }
    if __pInfo_ref.idxNum == crate::fts3Int_h::FTS3_DOCID_SEARCH {
        fts3SetUniqueFlag(pInfo);
    }
    iIdx = 1 as ::core::ffi::c_int;
    if iCons >= 0 as ::core::ffi::c_int {
        let fresh38 = iIdx;
        iIdx += 1;
        (*__pInfo_ref.aConstraintUsage.offset(iCons as isize)).argvIndex = fresh38;
        (*__pInfo_ref.aConstraintUsage.offset(iCons as isize)).omit = 1 as ::core::ffi::c_uchar;
    }
    if iLangidCons >= 0 as ::core::ffi::c_int {
        __pInfo_ref.idxNum |= crate::fts3Int_h::FTS3_HAVE_LANGID;
        let fresh39 = iIdx;
        iIdx += 1;
        (*__pInfo_ref.aConstraintUsage.offset(iLangidCons as isize)).argvIndex = fresh39;
    }
    if iDocidGe >= 0 as ::core::ffi::c_int {
        __pInfo_ref.idxNum |= crate::fts3Int_h::FTS3_HAVE_DOCID_GE;
        let fresh40 = iIdx;
        iIdx += 1;
        (*__pInfo_ref.aConstraintUsage.offset(iDocidGe as isize)).argvIndex = fresh40;
    }
    if iDocidLe >= 0 as ::core::ffi::c_int {
        __pInfo_ref.idxNum |= crate::fts3Int_h::FTS3_HAVE_DOCID_LE;
        let fresh41 = iIdx;
        (*__pInfo_ref.aConstraintUsage.offset(iDocidLe as isize)).argvIndex = fresh41;
    }
    if __pInfo_ref.nOrderBy == 1 as ::core::ffi::c_int {
        let pOrder = &*(__pInfo_ref.aOrderBy.offset(0_isize)
            as *mut crate::src::headers::sqlite3_h::sqlite3_index_orderby);

        if pOrder.iColumn < 0 as ::core::ffi::c_int
            || pOrder.iColumn == (*p).nColumn + 1 as ::core::ffi::c_int
        {
            if pOrder.desc != 0 {
                __pInfo_ref.idxStr = b"DESC\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            } else {
                __pInfo_ref.idxStr =
                    b"ASC\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
            __pInfo_ref.orderByConsumed = 1 as ::core::ffi::c_int;
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3OpenMethod(
    mut _pVTab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    ppCsr: *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    
    let pCsr: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor = crate::src::src::malloc::sqlite3_malloc(::core::mem::size_of::<
        crate::fts3Int_h::Fts3Cursor,
    >() as ::core::ffi::c_int)
        as *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;
    *ppCsr = pCsr;
    if pCsr.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        pCsr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<crate::fts3Int_h::Fts3Cursor>() as crate::__stddef_size_t_h::SizeT,
    );
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3CursorFinalizeStmt(pCsr: *mut crate::fts3Int_h::Fts3Cursor) {
    if (*pCsr).bSeekStmt != 0 {
        let p: *mut crate::fts3Int_h::Fts3Table =
            (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
        if (*p).pSeekStmt.is_null() {
            let __pCsr_ref = unsafe { &mut *pCsr };
            (*p).pSeekStmt = __pCsr_ref.pStmt;
            crate::src::src::vdbeapi::sqlite3_reset(__pCsr_ref.pStmt);
            __pCsr_ref.pStmt =
                ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        }
        (*pCsr).bSeekStmt = 0 as crate::src::ext::rtree::rtree::U8_0;
    }
    crate::src::src::vdbeapi::sqlite3_finalize((*pCsr).pStmt);
}

unsafe extern "C" fn fts3ClearCursor(pCsr: *mut crate::fts3Int_h::Fts3Cursor) {
    fts3CursorFinalizeStmt(pCsr);
    crate::src::ext::fts3::fts3_write::sqlite3Fts3FreeDeferredTokens(
        pCsr as *mut crate::fts3Int_h::Fts3Cursor,
    );
    let __pCsr_ref = unsafe { &mut *pCsr };
    crate::src::src::malloc::sqlite3_free(__pCsr_ref.aDoclist as *mut ::core::ffi::c_void);
    crate::src::ext::fts3::fts3_snippet::sqlite3Fts3MIBufferFree(__pCsr_ref.pMIBuffer);
    crate::src::ext::fts3::fts3_expr::sqlite3Fts3ExprFree(
        __pCsr_ref.pExpr as *mut crate::fts3Int_h::Fts3Expr,
    );
    ::libc::memset(
        (&raw mut __pCsr_ref.base).offset(1_isize)
            as *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<crate::fts3Int_h::Fts3Cursor>()
            as crate::__stddef_size_t_h::SizeT)
            .wrapping_sub(
                ::core::mem::size_of::<crate::src::headers::sqlite3_h::sqlite3_vtab_cursor>()
                    as crate::__stddef_size_t_h::SizeT,
            ),
    );
}

unsafe extern "C" fn fts3CloseMethod(
    pCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let pCsr: *mut crate::fts3Int_h::Fts3Cursor = pCursor as *mut crate::fts3Int_h::Fts3Cursor;
    fts3ClearCursor(pCsr);
    crate::src::src::malloc::sqlite3_free(pCsr as *mut ::core::ffi::c_void);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3CursorSeekStmt(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pCsr).pStmt.is_null() {
        let p: *mut crate::fts3Int_h::Fts3Table =
            (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
        let zSql: *mut ::core::ffi::c_char;
        if !(*p).pSeekStmt.is_null() {
            (*pCsr).pStmt = (*p).pSeekStmt;
            (*p).pSeekStmt =
                ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        } else {
            let __p_ref = unsafe { &mut *p };
            zSql = crate::sqlite_printf!("SELECT %s WHERE rowid = ?", __p_ref.zReadExprlist);
            if zSql.is_null() {
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
            __p_ref.bLock += 1;
            rc = crate::src::src::prepare::sqlite3_prepare_v3(
                __p_ref.db,
                zSql,
                -(1 as ::core::ffi::c_int),
                crate::src::headers::sqlite3_h::SQLITE_PREPARE_PERSISTENT as ::core::ffi::c_uint,
                &raw mut (*pCsr).pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            __p_ref.bLock -= 1;
            crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            (*pCsr).bSeekStmt = 1 as crate::src::ext::rtree::rtree::U8_0;
        }
    }
    rc
}

unsafe extern "C" fn fts3CursorSeek(
    pContext: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pCsr).isRequireSeek != 0 {
        rc = fts3CursorSeekStmt(pCsr);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let __pCsr_ref = unsafe { &mut *pCsr };
            let pTab: *mut crate::fts3Int_h::Fts3Table =
                __pCsr_ref.base.pVtab as *mut crate::fts3Int_h::Fts3Table;
            (*pTab).bLock += 1;
            crate::src::src::vdbeapi::sqlite3_bind_int64(
                __pCsr_ref.pStmt,
                1 as ::core::ffi::c_int,
                __pCsr_ref.iPrevId,
            );
            __pCsr_ref.isRequireSeek = 0 as crate::src::ext::rtree::rtree::U8_0;
            if crate::src::headers::sqlite3_h::SQLITE_ROW
                == crate::src::src::vdbeapi::sqlite3_step(__pCsr_ref.pStmt)
            {
                (*pTab).bLock -= 1;
                return crate::src::headers::sqlite3_h::SQLITE_OK;
            } else {
                (*pTab).bLock -= 1;
                rc = crate::src::src::vdbeapi::sqlite3_reset(__pCsr_ref.pStmt);
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                    && (*(__pCsr_ref.base.pVtab as *mut crate::fts3Int_h::Fts3Table))
                        .zContentTbl
                        .is_null()
                {
                    rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
                    __pCsr_ref.isEof = 1 as crate::src::ext::rtree::rtree::U8_0;
                }
            }
        }
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK && !pContext.is_null() {
        crate::src::src::vdbeapi::sqlite3_result_error_code(pContext, rc);
    }
    rc
}

unsafe extern "C" fn fts3ScanInteriorNode(
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
    zNode: *const ::core::ffi::c_char,
    nNode: ::core::ffi::c_int,
    mut piFirst: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
    mut piLast: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    let current_block: u64;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut zCsr: *const ::core::ffi::c_char = zNode;
    let zEnd: *const ::core::ffi::c_char =
        zCsr.offset(nNode as isize) as *const ::core::ffi::c_char;
    let mut zBuffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nAlloc: crate::src::ext::rtree::rtree::I64_0 =
        0 as crate::src::ext::rtree::rtree::I64_0;
    let mut isFirstTerm: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut iChild: crate::src::ext::rtree::rtree::U64_0 = 0;
    let mut nBuffer: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    zCsr = zCsr.offset(sqlite3Fts3GetVarintU(zCsr, &raw mut iChild) as isize);
    zCsr = zCsr.offset(sqlite3Fts3GetVarintU(zCsr, &raw mut iChild) as isize);
    if zCsr > zEnd {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    loop {
        if !(zCsr < zEnd && (!piFirst.is_null() || !piLast.is_null())) {
            current_block = 18377268871191777778;
            break;
        }
        let cmp: ::core::ffi::c_int;
        let mut nSuffix: ::core::ffi::c_int = 0;
        let mut nPrefix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if isFirstTerm == 0 {
            zCsr = zCsr.offset(
                (if *(zCsr as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    != 0
                {
                    sqlite3Fts3GetVarint32(zCsr, &raw mut nPrefix)
                } else {
                    nPrefix =
                        *(zCsr as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                }) as isize,
            );
            if nPrefix > nBuffer {
                rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
                current_block = 1916983269954443888;
                break;
            }
        }
        isFirstTerm = 0 as ::core::ffi::c_int;
        zCsr = zCsr.offset(
            (if *(zCsr as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                != 0
            {
                sqlite3Fts3GetVarint32(zCsr, &raw mut nSuffix)
            } else {
                nSuffix = *(zCsr as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            }) as isize,
        );
        if nPrefix as ::core::ffi::c_long > zCsr.offset_from(zNode) as ::core::ffi::c_long
            || nSuffix as ::core::ffi::c_long > zEnd.offset_from(zCsr) as ::core::ffi::c_long
            || nSuffix == 0 as ::core::ffi::c_int
        {
            rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
            current_block = 1916983269954443888;
            break;
        } else {
            if nPrefix as crate::src::ext::rtree::rtree::I64_0
                + nSuffix as crate::src::ext::rtree::rtree::I64_0
                > nAlloc
            {
                
                nAlloc = (nPrefix as crate::src::ext::rtree::rtree::I64_0
                    + nSuffix as crate::src::ext::rtree::rtree::I64_0)
                    * 2 as crate::src::ext::rtree::rtree::I64_0;
                let zNew: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_realloc64(
                    zBuffer as *mut ::core::ffi::c_void,
                    nAlloc as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                ) as *mut ::core::ffi::c_char;
                if zNew.is_null() {
                    rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                    current_block = 1916983269954443888;
                    break;
                } else {
                    zBuffer = zNew;
                }
            }
            ::core::ptr::copy_nonoverlapping(
                zCsr as *const u8,
                zBuffer.offset(nPrefix as isize) as *mut ::core::ffi::c_char as *mut u8,
                nSuffix as usize,
            );
            nBuffer = nPrefix + nSuffix;
            zCsr = zCsr.offset(nSuffix as isize);
            cmp = ::libc::memcmp(
                zTerm as *const ::core::ffi::c_void,
                zBuffer as *const ::core::ffi::c_void,
                (if nBuffer > nTerm { nTerm } else { nBuffer }) as crate::__stddef_size_t_h::SizeT,
            );
            if !piFirst.is_null()
                && (cmp < 0 as ::core::ffi::c_int
                    || cmp == 0 as ::core::ffi::c_int && nBuffer > nTerm)
            {
                *piFirst = iChild as crate::src::ext::rtree::rtree::I64_0
                    as crate::src::headers::sqlite3_h::Sqlite3Int64;
                piFirst = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Int64>();
            }
            if !piLast.is_null() && cmp < 0 as ::core::ffi::c_int {
                *piLast = iChild as crate::src::ext::rtree::rtree::I64_0
                    as crate::src::headers::sqlite3_h::Sqlite3Int64;
                piLast = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Int64>();
            }
            iChild = iChild.wrapping_add(1);
        }
    }
    if current_block == 18377268871191777778 {
        if !piFirst.is_null() {
            *piFirst = iChild as crate::src::ext::rtree::rtree::I64_0
                as crate::src::headers::sqlite3_h::Sqlite3Int64;
        }
        if !piLast.is_null() {
            *piLast = iChild as crate::src::ext::rtree::rtree::I64_0
                as crate::src::headers::sqlite3_h::Sqlite3Int64;
        }
    }
    crate::src::src::malloc::sqlite3_free(zBuffer as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn fts3SelectLeaf(
    p: *mut crate::fts3Int_h::Fts3Table,
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
    zNode: *const ::core::ffi::c_char,
    nNode: ::core::ffi::c_int,
    mut piLeaf: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
    piLeaf2: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut iHeight: ::core::ffi::c_int = 0;
    if *(zNode as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
        & 0x80 as ::core::ffi::c_int
        != 0
    {
        sqlite3Fts3GetVarint32(zNode, &raw mut iHeight);
    } else {
        iHeight = *(zNode as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
    };
    rc = fts3ScanInteriorNode(zTerm, nTerm, zNode, nNode, piLeaf, piLeaf2);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && iHeight > 1 as ::core::ffi::c_int {
        let mut zBlob: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut nBlob: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !piLeaf.is_null() && !piLeaf2.is_null() && *piLeaf != *piLeaf2 {
            rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3ReadBlock(
                p as *mut crate::fts3Int_h::Fts3Table,
                *piLeaf,
                &raw mut zBlob,
                &raw mut nBlob,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = fts3SelectLeaf(
                    p,
                    zTerm,
                    nTerm,
                    zBlob,
                    nBlob,
                    piLeaf,
                    ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Int64>(),
                );
            }
            crate::src::src::malloc::sqlite3_free(zBlob as *mut ::core::ffi::c_void);
            piLeaf = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Int64>();
            zBlob = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3ReadBlock(
                p as *mut crate::fts3Int_h::Fts3Table,
                if !piLeaf.is_null() { *piLeaf } else { *piLeaf2 },
                &raw mut zBlob,
                &raw mut nBlob,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let mut iNewHeight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if *(zBlob as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                != 0
            {
                sqlite3Fts3GetVarint32(zBlob, &raw mut iNewHeight);
            } else {
                iNewHeight =
                    *(zBlob as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
            };
            if iNewHeight >= iHeight {
                rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
            } else {
                rc = fts3SelectLeaf(p, zTerm, nTerm, zBlob, nBlob, piLeaf, piLeaf2);
            }
        }
        crate::src::src::malloc::sqlite3_free(zBlob as *mut ::core::ffi::c_void);
    }
    rc
}

unsafe extern "C" fn fts3PutDeltaVarint(
    pp: *mut *mut ::core::ffi::c_char,
    piPrev: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
    iVal: crate::src::headers::sqlite3_h::Sqlite3Int64,
) {
    *pp = (*pp).offset(sqlite3Fts3PutVarint(
        *pp,
        iVal as crate::src::headers::sqlite3_h::SqliteInt64 - *piPrev,
    ) as isize);
    *piPrev = iVal;
}

unsafe extern "C" fn fts3PoslistCopy(
    pp: *mut *mut ::core::ffi::c_char,
    ppPoslist: *mut *mut ::core::ffi::c_char,
) {
    let mut pEnd: *mut ::core::ffi::c_char = *ppPoslist;
    let mut c: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    while *pEnd as ::core::ffi::c_int | c as ::core::ffi::c_int != 0 {
        let fresh19 = pEnd;
        pEnd = pEnd.offset(1);
        c = (*fresh19 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
    }
    pEnd = pEnd.offset(1);
    if !pp.is_null() {
        let n: ::core::ffi::c_int =
            pEnd.offset_from(*ppPoslist) as ::core::ffi::c_long as ::core::ffi::c_int;
        let mut p: *mut ::core::ffi::c_char = *pp;
        ::core::ptr::copy_nonoverlapping(*ppPoslist as *const u8, p as *mut u8, n as usize);
        p = p.offset(n as isize);
        *pp = p;
    }
    *ppPoslist = pEnd;
}

unsafe extern "C" fn fts3ColumnlistCopy(
    pp: *mut *mut ::core::ffi::c_char,
    ppPoslist: *mut *mut ::core::ffi::c_char,
) {
    let mut pEnd: *mut ::core::ffi::c_char = *ppPoslist;
    let mut c: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    while 0xfe as ::core::ffi::c_int & (*pEnd as ::core::ffi::c_int | c as ::core::ffi::c_int) != 0
    {
        let fresh22 = pEnd;
        pEnd = pEnd.offset(1);
        c = (*fresh22 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
    }
    if !pp.is_null() {
        let n: ::core::ffi::c_int =
            pEnd.offset_from(*ppPoslist) as ::core::ffi::c_long as ::core::ffi::c_int;
        let mut p: *mut ::core::ffi::c_char = *pp;
        ::core::ptr::copy_nonoverlapping(*ppPoslist as *const u8, p as *mut u8, n as usize);
        p = p.offset(n as isize);
        *pp = p;
    }
    *ppPoslist = pEnd;
}

pub const POSITION_LIST_END: crate::src::ext::rtree::rtree::I64_0 = crate::fts3Int_h::LARGEST_INT64;

unsafe extern "C" fn fts3ReadNextPos(
    pp: *mut *mut ::core::ffi::c_char,
    pi: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
) {
    if **pp as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int != 0 {
        let mut iVal: ::core::ffi::c_int = 0;
        *pp = (*pp).offset(
            (if *(*pp as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                != 0
            {
                sqlite3Fts3GetVarint32(*pp, &raw mut iVal)
            } else {
                iVal = *(*pp as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            }) as isize,
        );
        *pi += iVal as crate::src::headers::sqlite3_h::Sqlite3Int64;
        *pi -= 2 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    } else {
        *pi = POSITION_LIST_END as crate::src::headers::sqlite3_h::Sqlite3Int64;
    };
}

unsafe extern "C" fn fts3PutColNumber(
    pp: *mut *mut ::core::ffi::c_char,
    iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if iCol != 0 {
        let p: *mut ::core::ffi::c_char = *pp;
        n = 1 as ::core::ffi::c_int
            + sqlite3Fts3PutVarint(
                p.offset(1_isize) as *mut ::core::ffi::c_char,
                iCol as crate::src::headers::sqlite3_h::SqliteInt64,
            );
        *p = 0x1 as ::core::ffi::c_char;
        *pp = p.offset(n as isize) as *mut ::core::ffi::c_char;
    }
    n
}

unsafe extern "C" fn fts3PoslistMerge(
    pp: *mut *mut ::core::ffi::c_char,
    pp1: *mut *mut ::core::ffi::c_char,
    pp2: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = *pp;
    let mut p1: *mut ::core::ffi::c_char = *pp1;
    let mut p2: *mut ::core::ffi::c_char = *pp2;
    while *p1 as ::core::ffi::c_int != 0 || *p2 as ::core::ffi::c_int != 0 {
        let mut iCol1: ::core::ffi::c_int = 0;
        let mut iCol2: ::core::ffi::c_int = 0;
        if *p1 as ::core::ffi::c_int == crate::fts3Int_h::POS_COLUMN {
            if *(p1.offset(1_isize) as *mut ::core::ffi::c_char
                as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                != 0
            {
                sqlite3Fts3GetVarint32(
                    p1.offset(1_isize) as *mut ::core::ffi::c_char,
                    &raw mut iCol1,
                );
            } else {
                iCol1 = *(p1.offset(1_isize) as *mut ::core::ffi::c_char
                    as *mut crate::src::ext::rtree::rtree::U8_0)
                    as ::core::ffi::c_int;
            };
            if iCol1 == 0 as ::core::ffi::c_int {
                return crate::fts3Int_h::FTS_CORRUPT_VTAB;
            }
        } else if *p1 as ::core::ffi::c_int == crate::fts3Int_h::POS_END {
            iCol1 = 0x7fffffff as ::core::ffi::c_int;
        } else {
            iCol1 = 0 as ::core::ffi::c_int;
        }
        if *p2 as ::core::ffi::c_int == crate::fts3Int_h::POS_COLUMN {
            if *(p2.offset(1_isize) as *mut ::core::ffi::c_char
                as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                != 0
            {
                sqlite3Fts3GetVarint32(
                    p2.offset(1_isize) as *mut ::core::ffi::c_char,
                    &raw mut iCol2,
                );
            } else {
                iCol2 = *(p2.offset(1_isize) as *mut ::core::ffi::c_char
                    as *mut crate::src::ext::rtree::rtree::U8_0)
                    as ::core::ffi::c_int;
            };
            if iCol2 == 0 as ::core::ffi::c_int {
                return crate::fts3Int_h::FTS_CORRUPT_VTAB;
            }
        } else if *p2 as ::core::ffi::c_int == crate::fts3Int_h::POS_END {
            iCol2 = 0x7fffffff as ::core::ffi::c_int;
        } else {
            iCol2 = 0 as ::core::ffi::c_int;
        }
        if iCol1 == iCol2 {
            let mut i1: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            let mut i2: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            let mut iPrev: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            let n: ::core::ffi::c_int = fts3PutColNumber(&raw mut p, iCol1);
            p1 = p1.offset(n as isize);
            p2 = p2.offset(n as isize);
            fts3GetDeltaVarint(&raw mut p1, &raw mut i1);
            fts3GetDeltaVarint(&raw mut p2, &raw mut i2);
            if i1 < 2 as crate::src::headers::sqlite3_h::Sqlite3Int64
                || i2 < 2 as crate::src::headers::sqlite3_h::Sqlite3Int64
            {
                break;
            }
            loop {
                fts3PutDeltaVarint(&raw mut p, &raw mut iPrev, if i1 < i2 { i1 } else { i2 });
                iPrev -= 2 as crate::src::headers::sqlite3_h::Sqlite3Int64;
                if i1 == i2 {
                    fts3ReadNextPos(&raw mut p1, &raw mut i1);
                    fts3ReadNextPos(&raw mut p2, &raw mut i2);
                } else if i1 < i2 {
                    fts3ReadNextPos(&raw mut p1, &raw mut i1);
                } else {
                    fts3ReadNextPos(&raw mut p2, &raw mut i2);
                }
                if !(i1 != POSITION_LIST_END || i2 != POSITION_LIST_END) {
                    break;
                }
            }
        } else if iCol1 < iCol2 {
            p1 = p1.offset(fts3PutColNumber(&raw mut p, iCol1) as isize);
            fts3ColumnlistCopy(&raw mut p, &raw mut p1);
        } else {
            p2 = p2.offset(fts3PutColNumber(&raw mut p, iCol2) as isize);
            fts3ColumnlistCopy(&raw mut p, &raw mut p2);
        }
    }
    let fresh23 = p;
    p = p.offset(1);
    *fresh23 = crate::fts3Int_h::POS_END as ::core::ffi::c_char;
    *pp = p;
    *pp1 = p1.offset(1_isize);
    *pp2 = p2.offset(1_isize);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3PoslistPhraseMerge(
    pp: *mut *mut ::core::ffi::c_char,
    nToken: ::core::ffi::c_int,
    isSaveLeft: ::core::ffi::c_int,
    isExact: ::core::ffi::c_int,
    pp1: *mut *mut ::core::ffi::c_char,
    pp2: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = *pp;
    let mut p1: *mut ::core::ffi::c_char = *pp1;
    let mut p2: *mut ::core::ffi::c_char = *pp2;
    let mut iCol1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iCol2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *p1 as ::core::ffi::c_int == crate::fts3Int_h::POS_COLUMN {
        p1 = p1.offset(1);
        p1 = p1.offset(
            (if *(p1 as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                != 0
            {
                sqlite3Fts3GetVarint32(p1, &raw mut iCol1)
            } else {
                iCol1 = *(p1 as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            }) as isize,
        );
        if iCol1 == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *p2 as ::core::ffi::c_int == crate::fts3Int_h::POS_COLUMN {
        p2 = p2.offset(1);
        p2 = p2.offset(
            (if *(p2 as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                != 0
            {
                sqlite3Fts3GetVarint32(p2, &raw mut iCol2)
            } else {
                iCol2 = *(p2 as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            }) as isize,
        );
        if iCol2 == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    loop {
        if iCol1 == iCol2 {
            let mut pSave: *mut ::core::ffi::c_char = p;
            let mut iPrev: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            let mut iPos1: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            let mut iPos2: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            if iCol1 != 0 {
                let fresh20 = p;
                p = p.offset(1);
                *fresh20 = crate::fts3Int_h::POS_COLUMN as ::core::ffi::c_char;
                p = p.offset(sqlite3Fts3PutVarint(
                    p,
                    iCol1 as crate::src::headers::sqlite3_h::SqliteInt64,
                ) as isize);
            }
            fts3GetDeltaVarint(&raw mut p1, &raw mut iPos1);
            iPos1 -= 2 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            fts3GetDeltaVarint(&raw mut p2, &raw mut iPos2);
            iPos2 -= 2 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            if iPos1 < 0 as crate::src::headers::sqlite3_h::Sqlite3Int64
                || iPos2 < 0 as crate::src::headers::sqlite3_h::Sqlite3Int64
            {
                break;
            }
            loop {
                if iPos2 == iPos1 + nToken as crate::src::headers::sqlite3_h::Sqlite3Int64
                    || isExact == 0 as ::core::ffi::c_int
                        && iPos2 > iPos1
                        && iPos2 <= iPos1 + nToken as crate::src::headers::sqlite3_h::Sqlite3Int64
                {
                    
                    let iSave: crate::src::headers::sqlite3_h::Sqlite3Int64 = if isSaveLeft != 0 { iPos1 } else { iPos2 };
                    fts3PutDeltaVarint(
                        &raw mut p,
                        &raw mut iPrev,
                        iSave + 2 as crate::src::headers::sqlite3_h::Sqlite3Int64,
                    );
                    iPrev -= 2 as crate::src::headers::sqlite3_h::Sqlite3Int64;
                    pSave = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                if isSaveLeft == 0
                    && iPos2 <= iPos1 + nToken as crate::src::headers::sqlite3_h::Sqlite3Int64
                    || iPos2 <= iPos1
                {
                    if *p2 as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                    fts3GetDeltaVarint(&raw mut p2, &raw mut iPos2);
                    iPos2 -= 2 as crate::src::headers::sqlite3_h::Sqlite3Int64;
                } else {
                    if *p1 as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                    fts3GetDeltaVarint(&raw mut p1, &raw mut iPos1);
                    iPos1 -= 2 as crate::src::headers::sqlite3_h::Sqlite3Int64;
                }
            }
            if !pSave.is_null() {
                p = pSave;
            }
            fts3ColumnlistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut p1,
            );
            fts3ColumnlistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut p2,
            );
            if 0 as ::core::ffi::c_int == *p1 as ::core::ffi::c_int
                || 0 as ::core::ffi::c_int == *p2 as ::core::ffi::c_int
            {
                break;
            }
            p1 = p1.offset(1);
            p1 = p1.offset(
                (if *(p1 as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    != 0
                {
                    sqlite3Fts3GetVarint32(p1, &raw mut iCol1)
                } else {
                    iCol1 = *(p1 as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                }) as isize,
            );
            p2 = p2.offset(1);
            p2 = p2.offset(
                (if *(p2 as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    != 0
                {
                    sqlite3Fts3GetVarint32(p2, &raw mut iCol2)
                } else {
                    iCol2 = *(p2 as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                }) as isize,
            );
        } else if iCol1 < iCol2 {
            fts3ColumnlistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut p1,
            );
            if 0 as ::core::ffi::c_int == *p1 as ::core::ffi::c_int {
                break;
            }
            p1 = p1.offset(1);
            p1 = p1.offset(
                (if *(p1 as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    != 0
                {
                    sqlite3Fts3GetVarint32(p1, &raw mut iCol1)
                } else {
                    iCol1 = *(p1 as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                }) as isize,
            );
        } else {
            fts3ColumnlistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut p2,
            );
            if 0 as ::core::ffi::c_int == *p2 as ::core::ffi::c_int {
                break;
            }
            p2 = p2.offset(1);
            p2 = p2.offset(
                (if *(p2 as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    != 0
                {
                    sqlite3Fts3GetVarint32(p2, &raw mut iCol2)
                } else {
                    iCol2 = *(p2 as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                }) as isize,
            );
        }
    }
    fts3PoslistCopy(
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        &raw mut p2,
    );
    fts3PoslistCopy(
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        &raw mut p1,
    );
    *pp1 = p1;
    *pp2 = p2;
    if *pp == p {
        return 0 as ::core::ffi::c_int;
    }
    let fresh21 = p;
    p = p.offset(1);
    *fresh21 = 0 as ::core::ffi::c_char;
    *pp = p;
    1 as ::core::ffi::c_int
}

unsafe extern "C" fn fts3PoslistNearMerge(
    pp: *mut *mut ::core::ffi::c_char,
    mut aTmp: *mut ::core::ffi::c_char,
    nRight: ::core::ffi::c_int,
    nLeft: ::core::ffi::c_int,
    pp1: *mut *mut ::core::ffi::c_char,
    pp2: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let p1: *mut ::core::ffi::c_char = *pp1;
    let p2: *mut ::core::ffi::c_char = *pp2;
    let mut pTmp1: *mut ::core::ffi::c_char = aTmp;
    let mut pTmp2: *mut ::core::ffi::c_char;
    let mut aTmp2: *mut ::core::ffi::c_char;
    let mut res: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    fts3PoslistPhraseMerge(
        &raw mut pTmp1,
        nRight,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        pp1,
        pp2,
    );
    pTmp2 = pTmp1;
    aTmp2 = pTmp2;
    *pp1 = p1;
    *pp2 = p2;
    fts3PoslistPhraseMerge(
        &raw mut pTmp2,
        nLeft,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        pp2,
        pp1,
    );
    if pTmp1 != aTmp && pTmp2 != aTmp2 {
        fts3PoslistMerge(pp, &raw mut aTmp, &raw mut aTmp2);
    } else if pTmp1 != aTmp {
        fts3PoslistCopy(pp, &raw mut aTmp);
    } else if pTmp2 != aTmp2 {
        fts3PoslistCopy(pp, &raw mut aTmp2);
    } else {
        res = 0 as ::core::ffi::c_int;
    }
    res
}

unsafe extern "C" fn fts3GetDeltaVarint3(
    pp: *mut *mut ::core::ffi::c_char,
    pEnd: *mut ::core::ffi::c_char,
    bDescIdx: ::core::ffi::c_int,
    pVal: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
) {
    if *pp >= pEnd {
        *pp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        let mut iVal: crate::src::ext::rtree::rtree::U64_0 = 0;
        *pp = (*pp).offset(sqlite3Fts3GetVarintU(*pp, &raw mut iVal) as isize);
        if bDescIdx != 0 {
            *pVal = (*pVal as crate::src::ext::rtree::rtree::U64_0).wrapping_sub(iVal)
                as crate::src::ext::rtree::rtree::I64_0
                as crate::src::headers::sqlite3_h::Sqlite3Int64;
        } else {
            *pVal = (*pVal as crate::src::ext::rtree::rtree::U64_0).wrapping_add(iVal)
                as crate::src::ext::rtree::rtree::I64_0
                as crate::src::headers::sqlite3_h::Sqlite3Int64;
        }
    };
}

unsafe extern "C" fn fts3PutDeltaVarint3(
    pp: *mut *mut ::core::ffi::c_char,
    bDescIdx: ::core::ffi::c_int,
    piPrev: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
    pbFirst: *mut ::core::ffi::c_int,
    iVal: crate::src::headers::sqlite3_h::Sqlite3Int64,
) {
    let iWrite: crate::src::headers::sqlite3_h::Sqlite3Uint64;
    if bDescIdx == 0 as ::core::ffi::c_int || *pbFirst == 0 as ::core::ffi::c_int {
        iWrite = (iVal as crate::src::ext::rtree::rtree::U64_0)
            .wrapping_sub(*piPrev as crate::src::ext::rtree::rtree::U64_0)
            as crate::src::headers::sqlite3_h::Sqlite3Uint64;
    } else {
        iWrite = (*piPrev as crate::src::ext::rtree::rtree::U64_0)
            .wrapping_sub(iVal as crate::src::ext::rtree::rtree::U64_0)
            as crate::src::headers::sqlite3_h::Sqlite3Uint64;
    }
    *pp = (*pp).offset(sqlite3Fts3PutVarint(
        *pp,
        iWrite as crate::src::headers::sqlite3_h::SqliteInt64,
    ) as isize);
    *piPrev = iVal;
    *pbFirst = 1 as ::core::ffi::c_int;
}

unsafe extern "C" fn fts3DoclistOrMerge(
    bDescDoclist: ::core::ffi::c_int,
    a1: *mut ::core::ffi::c_char,
    n1: ::core::ffi::c_int,
    a2: *mut ::core::ffi::c_char,
    n2: ::core::ffi::c_int,
    paOut: *mut *mut ::core::ffi::c_char,
    pnOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut i1: crate::src::headers::sqlite3_h::Sqlite3Int64 =
        0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let mut i2: crate::src::headers::sqlite3_h::Sqlite3Int64 =
        0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let mut iPrev: crate::src::headers::sqlite3_h::Sqlite3Int64 =
        0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let pEnd1: *mut ::core::ffi::c_char = a1.offset(n1 as isize) as *mut ::core::ffi::c_char;
    let pEnd2: *mut ::core::ffi::c_char = a2.offset(n2 as isize) as *mut ::core::ffi::c_char;
    let mut p1: *mut ::core::ffi::c_char = a1;
    let mut p2: *mut ::core::ffi::c_char = a2;
    let mut p: *mut ::core::ffi::c_char;
    let mut aOut: *mut ::core::ffi::c_char;
    let mut bFirstOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    *paOut = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *pnOut = 0 as ::core::ffi::c_int;
    aOut = crate::src::src::malloc::sqlite3_malloc64(
        (n1 as crate::src::ext::rtree::rtree::I64_0
            + n2 as crate::src::ext::rtree::rtree::I64_0
            + crate::fts3Int_h::FTS3_VARINT_MAX as crate::src::ext::rtree::rtree::I64_0
            - 1 as crate::src::ext::rtree::rtree::I64_0
            + crate::fts3Int_h::FTS3_BUFFER_PADDING as crate::src::ext::rtree::rtree::I64_0)
            as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut ::core::ffi::c_char;
    if aOut.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    p = aOut;
    fts3GetDeltaVarint3(&raw mut p1, pEnd1, 0 as ::core::ffi::c_int, &raw mut i1);
    fts3GetDeltaVarint3(&raw mut p2, pEnd2, 0 as ::core::ffi::c_int, &raw mut i2);
    while !p1.is_null() || !p2.is_null() {
        let iDiff: crate::src::headers::sqlite3_h::Sqlite3Int64 = ((if bDescDoclist != 0 {
            -(1 as ::core::ffi::c_int)
        } else {
            1 as ::core::ffi::c_int
        }) * (if i1 > i2 {
            1 as ::core::ffi::c_int
        } else if i1 == i2 {
            0 as ::core::ffi::c_int
        } else {
            -(1 as ::core::ffi::c_int)
        }))
            as crate::src::headers::sqlite3_h::Sqlite3Int64;
        if !p2.is_null()
            && !p1.is_null()
            && iDiff == 0 as crate::src::headers::sqlite3_h::Sqlite3Int64
        {
            fts3PutDeltaVarint3(
                &raw mut p,
                bDescDoclist,
                &raw mut iPrev,
                &raw mut bFirstOut,
                i1,
            );
            rc = fts3PoslistMerge(&raw mut p, &raw mut p1, &raw mut p2);
            if rc != 0 {
                break;
            }
            fts3GetDeltaVarint3(&raw mut p1, pEnd1, bDescDoclist, &raw mut i1);
            fts3GetDeltaVarint3(&raw mut p2, pEnd2, bDescDoclist, &raw mut i2);
        } else if p2.is_null()
            || !p1.is_null() && iDiff < 0 as crate::src::headers::sqlite3_h::Sqlite3Int64
        {
            fts3PutDeltaVarint3(
                &raw mut p,
                bDescDoclist,
                &raw mut iPrev,
                &raw mut bFirstOut,
                i1,
            );
            fts3PoslistCopy(&raw mut p, &raw mut p1);
            fts3GetDeltaVarint3(&raw mut p1, pEnd1, bDescDoclist, &raw mut i1);
        } else {
            fts3PutDeltaVarint3(
                &raw mut p,
                bDescDoclist,
                &raw mut iPrev,
                &raw mut bFirstOut,
                i2,
            );
            fts3PoslistCopy(&raw mut p, &raw mut p2);
            fts3GetDeltaVarint3(&raw mut p2, pEnd2, bDescDoclist, &raw mut i2);
        }
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::malloc::sqlite3_free(aOut as *mut ::core::ffi::c_void);
        aOut = ::core::ptr::null_mut::<::core::ffi::c_char>();
        p = aOut;
    } else {
        ::libc::memset(
            aOut.offset(p.offset_from(aOut) as ::core::ffi::c_long as isize)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            crate::fts3Int_h::FTS3_BUFFER_PADDING as crate::__stddef_size_t_h::SizeT,
        );
    }
    *paOut = aOut;
    *pnOut = p.offset_from(aOut) as ::core::ffi::c_long as ::core::ffi::c_int;
    rc
}

unsafe extern "C" fn fts3DoclistPhraseMerge(
    bDescDoclist: ::core::ffi::c_int,
    nDist: ::core::ffi::c_int,
    aLeft: *mut ::core::ffi::c_char,
    nLeft: ::core::ffi::c_int,
    paRight: *mut *mut ::core::ffi::c_char,
    pnRight: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i1: crate::src::headers::sqlite3_h::Sqlite3Int64 =
        0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let mut i2: crate::src::headers::sqlite3_h::Sqlite3Int64 =
        0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let mut iPrev: crate::src::headers::sqlite3_h::Sqlite3Int64 =
        0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let aRight: *mut ::core::ffi::c_char = *paRight;
    let pEnd1: *mut ::core::ffi::c_char =
        aLeft.offset(nLeft as isize) as *mut ::core::ffi::c_char;
    let pEnd2: *mut ::core::ffi::c_char =
        aRight.offset(*pnRight as isize) as *mut ::core::ffi::c_char;
    let mut p1: *mut ::core::ffi::c_char = aLeft;
    let mut p2: *mut ::core::ffi::c_char = aRight;
    let mut p: *mut ::core::ffi::c_char;
    let mut bFirstOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let aOut: *mut ::core::ffi::c_char;
    if bDescDoclist != 0 {
        aOut = crate::src::src::malloc::sqlite3_malloc64(
            (*pnRight as crate::src::headers::sqlite3_h::Sqlite3Int64
                + crate::fts3Int_h::FTS3_VARINT_MAX
                    as crate::src::headers::sqlite3_h::Sqlite3Int64)
                as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut ::core::ffi::c_char;
        if aOut.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    } else {
        aOut = aRight;
    }
    p = aOut;
    fts3GetDeltaVarint3(&raw mut p1, pEnd1, 0 as ::core::ffi::c_int, &raw mut i1);
    fts3GetDeltaVarint3(&raw mut p2, pEnd2, 0 as ::core::ffi::c_int, &raw mut i2);
    while !p1.is_null() && !p2.is_null() {
        let iDiff: crate::src::headers::sqlite3_h::Sqlite3Int64 = ((if bDescDoclist != 0 {
            -(1 as ::core::ffi::c_int)
        } else {
            1 as ::core::ffi::c_int
        }) * (if i1 > i2 {
            1 as ::core::ffi::c_int
        } else if i1 == i2 {
            0 as ::core::ffi::c_int
        } else {
            -(1 as ::core::ffi::c_int)
        }))
            as crate::src::headers::sqlite3_h::Sqlite3Int64;
        if iDiff == 0 as crate::src::headers::sqlite3_h::Sqlite3Int64 {
            let pSave: *mut ::core::ffi::c_char = p;
            let iPrevSave: crate::src::headers::sqlite3_h::Sqlite3Int64 = iPrev;
            let bFirstOutSave: ::core::ffi::c_int = bFirstOut;
            fts3PutDeltaVarint3(
                &raw mut p,
                bDescDoclist,
                &raw mut iPrev,
                &raw mut bFirstOut,
                i1,
            );
            if 0 as ::core::ffi::c_int
                == fts3PoslistPhraseMerge(
                    &raw mut p,
                    nDist,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    &raw mut p1,
                    &raw mut p2,
                )
            {
                p = pSave;
                iPrev = iPrevSave;
                bFirstOut = bFirstOutSave;
            }
            fts3GetDeltaVarint3(&raw mut p1, pEnd1, bDescDoclist, &raw mut i1);
            fts3GetDeltaVarint3(&raw mut p2, pEnd2, bDescDoclist, &raw mut i2);
        } else if iDiff < 0 as crate::src::headers::sqlite3_h::Sqlite3Int64 {
            fts3PoslistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut p1,
            );
            fts3GetDeltaVarint3(&raw mut p1, pEnd1, bDescDoclist, &raw mut i1);
        } else {
            fts3PoslistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut p2,
            );
            fts3GetDeltaVarint3(&raw mut p2, pEnd2, bDescDoclist, &raw mut i2);
        }
    }
    *pnRight = p.offset_from(aOut) as ::core::ffi::c_long as ::core::ffi::c_int;
    if bDescDoclist != 0 {
        crate::src::src::malloc::sqlite3_free(aRight as *mut ::core::ffi::c_void);
        *paRight = aOut;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3FirstFilter(
    iDelta: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pList: *mut ::core::ffi::c_char,
    nList: ::core::ffi::c_int,
    pOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut nOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bWritten: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *mut ::core::ffi::c_char = pList;
    let pEnd: *mut ::core::ffi::c_char =
        pList.offset(nList as isize) as *mut ::core::ffi::c_char;
    if *p as ::core::ffi::c_int != 0x1 as ::core::ffi::c_int {
        if *p as ::core::ffi::c_int == 0x2 as ::core::ffi::c_int {
            nOut += sqlite3Fts3PutVarint(
                pOut.offset(nOut as isize) as *mut ::core::ffi::c_char,
                iDelta as crate::src::headers::sqlite3_h::SqliteInt64,
            );
            let fresh27 = nOut;
            nOut += 1;
            *pOut.offset(fresh27 as isize) = 0x2 as ::core::ffi::c_char;
            bWritten = 1 as ::core::ffi::c_int;
        }
        fts3ColumnlistCopy(
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            &raw mut p,
        );
    }
    while p < pEnd {
        let mut iCol: crate::src::headers::sqlite3_h::Sqlite3Int64 = 0;
        p = p.offset(1);
        p = p.offset(sqlite3Fts3GetVarint(p, &raw mut iCol) as isize);
        if *p as ::core::ffi::c_int == 0x2 as ::core::ffi::c_int {
            if bWritten == 0 as ::core::ffi::c_int {
                nOut += sqlite3Fts3PutVarint(
                    pOut.offset(nOut as isize) as *mut ::core::ffi::c_char,
                    iDelta as crate::src::headers::sqlite3_h::SqliteInt64,
                );
                bWritten = 1 as ::core::ffi::c_int;
            }
            let fresh28 = nOut;
            nOut += 1;
            *pOut.offset(fresh28 as isize) = 0x1 as ::core::ffi::c_char;
            nOut += sqlite3Fts3PutVarint(
                pOut.offset(nOut as isize) as *mut ::core::ffi::c_char,
                iCol as crate::src::headers::sqlite3_h::SqliteInt64,
            );
            let fresh29 = nOut;
            nOut += 1;
            *pOut.offset(fresh29 as isize) = 0x2 as ::core::ffi::c_char;
        }
        fts3ColumnlistCopy(
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            &raw mut p,
        );
    }
    if bWritten != 0 {
        let fresh30 = nOut;
        nOut += 1;
        *pOut.offset(fresh30 as isize) = 0 as ::core::ffi::c_char;
    }
    nOut
}

unsafe extern "C" fn fts3TermSelectFinishMerge(
    p: *mut crate::fts3Int_h::Fts3Table,
    pTS: *mut TermSelect,
) -> ::core::ffi::c_int {
    let mut aOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[*mut ::core::ffi::c_char; 16]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            as ::core::ffi::c_int
    {
        if !(*pTS).aaOutput[i as usize].is_null() {
            if aOut.is_null() {
                let __pTS_ref = unsafe { &mut *pTS };
                aOut = __pTS_ref.aaOutput[i as usize];
                nOut = __pTS_ref.anOutput[i as usize];
                __pTS_ref.aaOutput[i as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
            } else {
                let mut nNew: ::core::ffi::c_int = 0;
                let mut aNew: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let __pTS_ref = unsafe { &mut *pTS };
                let rc: ::core::ffi::c_int = fts3DoclistOrMerge(
                    (*p).bDescIdx as ::core::ffi::c_int,
                    __pTS_ref.aaOutput[i as usize],
                    __pTS_ref.anOutput[i as usize],
                    aOut,
                    nOut,
                    &raw mut aNew,
                    &raw mut nNew,
                );
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    crate::src::src::malloc::sqlite3_free(aOut as *mut ::core::ffi::c_void);
                    return rc;
                }
                crate::src::src::malloc::sqlite3_free(
                    __pTS_ref.aaOutput[i as usize] as *mut ::core::ffi::c_void,
                );
                crate::src::src::malloc::sqlite3_free(aOut as *mut ::core::ffi::c_void);
                __pTS_ref.aaOutput[i as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
                aOut = aNew;
                nOut = nNew;
            }
        }
        i += 1;
    }
    (*pTS).aaOutput[0 as ::core::ffi::c_int as usize] = aOut;
    (*pTS).anOutput[0 as ::core::ffi::c_int as usize] = nOut;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3TermSelectMerge(
    p: *mut crate::fts3Int_h::Fts3Table,
    pTS: *mut TermSelect,
    aDoclist: *mut ::core::ffi::c_char,
    nDoclist: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*pTS).aaOutput[0 as ::core::ffi::c_int as usize].is_null() {
        let __pTS_ref = unsafe { &mut *pTS };
        __pTS_ref.aaOutput[0 as ::core::ffi::c_int as usize] =
            crate::src::src::malloc::sqlite3_malloc64(
                (nDoclist as crate::src::ext::rtree::rtree::I64_0
                    + crate::fts3Int_h::FTS3_VARINT_MAX as crate::src::ext::rtree::rtree::I64_0
                    + 1 as crate::src::ext::rtree::rtree::I64_0)
                    as crate::src::headers::sqlite3_h::Sqlite3Uint64,
            ) as *mut ::core::ffi::c_char;
        __pTS_ref.anOutput[0 as ::core::ffi::c_int as usize] = nDoclist;
        if !__pTS_ref.aaOutput[0 as ::core::ffi::c_int as usize].is_null() {
            ::core::ptr::copy_nonoverlapping(
                aDoclist as *const u8,
                __pTS_ref.aaOutput[0 as ::core::ffi::c_int as usize] as *mut u8,
                nDoclist as usize,
            );
            ::libc::memset(
                (*(&raw mut __pTS_ref.aaOutput as *mut *mut ::core::ffi::c_char).offset(0_isize))
                    .offset(nDoclist as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                crate::fts3Int_h::FTS3_VARINT_MAX as crate::__stddef_size_t_h::SizeT,
            );
        } else {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    } else {
        let mut aMerge: *mut ::core::ffi::c_char = aDoclist;
        let mut nMerge: ::core::ffi::c_int = nDoclist;
        let mut iOut: ::core::ffi::c_int;
        iOut = 0 as ::core::ffi::c_int;
        while iOut
            < (::core::mem::size_of::<[*mut ::core::ffi::c_char; 16]>() as usize)
                .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                as ::core::ffi::c_int
        {
            if (*pTS).aaOutput[iOut as usize].is_null() {
                (*pTS).aaOutput[iOut as usize] = aMerge;
                (*pTS).anOutput[iOut as usize] = nMerge;
                break;
            } else {
                let mut aNew: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut nNew: ::core::ffi::c_int = 0;
                let __pTS_ref = unsafe { &mut *pTS };
                let rc: ::core::ffi::c_int = fts3DoclistOrMerge(
                    (*p).bDescIdx as ::core::ffi::c_int,
                    aMerge,
                    nMerge,
                    __pTS_ref.aaOutput[iOut as usize],
                    __pTS_ref.anOutput[iOut as usize],
                    &raw mut aNew,
                    &raw mut nNew,
                );
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    if aMerge != aDoclist {
                        crate::src::src::malloc::sqlite3_free(aMerge as *mut ::core::ffi::c_void);
                    }
                    return rc;
                }
                if aMerge != aDoclist {
                    crate::src::src::malloc::sqlite3_free(aMerge as *mut ::core::ffi::c_void);
                }
                crate::src::src::malloc::sqlite3_free(
                    __pTS_ref.aaOutput[iOut as usize] as *mut ::core::ffi::c_void,
                );
                __pTS_ref.aaOutput[iOut as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
                aMerge = aNew;
                nMerge = nNew;
                if iOut + 1 as ::core::ffi::c_int
                    == (::core::mem::size_of::<[*mut ::core::ffi::c_char; 16]>() as usize)
                        .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                        as ::core::ffi::c_int
                {
                    __pTS_ref.aaOutput[iOut as usize] = aMerge;
                    __pTS_ref.anOutput[iOut as usize] = nMerge;
                }
                iOut += 1;
            }
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3SegReaderCursorAppend(
    pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    pNew: *mut crate::fts3Int_h::Fts3SegReader,
) -> ::core::ffi::c_int {
    let __pCsr_ref = unsafe { &mut *pCsr };
    if __pCsr_ref.nSegment % 16 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        
        let nByte: crate::src::headers::sqlite3_h::Sqlite3Int64 =
            ((__pCsr_ref.nSegment + 16 as ::core::ffi::c_int) as usize).wrapping_mul(
                ::core::mem::size_of::<*mut crate::fts3Int_h::Fts3SegReader>() as usize,
            ) as crate::src::headers::sqlite3_h::Sqlite3Int64;
        let apNew: *mut *mut crate::fts3Int_h::Fts3SegReader = crate::src::src::malloc::sqlite3_realloc64(
            __pCsr_ref.apSegment as *mut ::core::ffi::c_void,
            nByte as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut *mut crate::fts3Int_h::Fts3SegReader;
        if apNew.is_null() {
            crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderFree(pNew);
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        __pCsr_ref.apSegment = apNew;
    }
    let fresh0 = __pCsr_ref.nSegment;
    __pCsr_ref.nSegment += 1;
    let fresh1 = &mut *__pCsr_ref.apSegment.offset(fresh0 as isize);
    *fresh1 = pNew;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3SegReaderCursor(
    p: *mut crate::fts3Int_h::Fts3Table,
    iLangid: ::core::ffi::c_int,
    iIndex: ::core::ffi::c_int,
    iLevel: ::core::ffi::c_int,
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
    isPrefix: ::core::ffi::c_int,
    isScan: ::core::ffi::c_int,
    pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    
    if iLevel < 0 as ::core::ffi::c_int && !(*p).aIndex.is_null() && (*p).iPrevLangid == iLangid {
        let mut pSeg: *mut crate::fts3Int_h::Fts3SegReader =
            ::core::ptr::null_mut::<crate::fts3Int_h::Fts3SegReader>();
        rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderPending(
            p as *mut crate::fts3Int_h::Fts3Table,
            iIndex,
            zTerm,
            nTerm,
            (isPrefix != 0 || isScan != 0) as ::core::ffi::c_int,
            &raw mut pSeg,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !pSeg.is_null() {
            rc = fts3SegReaderCursorAppend(pCsr, pSeg);
        }
    }
    if iLevel != crate::fts3Int_h::FTS3_SEGCURSOR_PENDING {
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3AllSegdirs(
                p as *mut crate::fts3Int_h::Fts3Table,
                iLangid,
                iIndex,
                iLevel,
                &raw mut pStmt,
            );
        }
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && {
            rc = crate::src::src::vdbeapi::sqlite3_step(pStmt);
            crate::src::headers::sqlite3_h::SQLITE_ROW == rc
        } {
            let mut pSeg_0: *mut crate::fts3Int_h::Fts3SegReader =
                ::core::ptr::null_mut::<crate::fts3Int_h::Fts3SegReader>();
            let mut iStartBlock: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 1 as ::core::ffi::c_int);
            let mut iLeavesEndBlock: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 2 as ::core::ffi::c_int);
            let iEndBlock: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 3 as ::core::ffi::c_int);
            let nRoot: ::core::ffi::c_int =
                crate::src::src::vdbeapi::sqlite3_column_bytes(pStmt, 4 as ::core::ffi::c_int);
            let zRoot: *const ::core::ffi::c_char =
                crate::src::src::vdbeapi::sqlite3_column_blob(pStmt, 4 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char;
            if iStartBlock != 0 && !zTerm.is_null() && !zRoot.is_null() {
                let pi: *mut crate::src::headers::sqlite3_h::Sqlite3Int64 = if isPrefix != 0 {
                    &raw mut iLeavesEndBlock
                } else {
                    ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Int64>()
                };
                rc = fts3SelectLeaf(p, zTerm, nTerm, zRoot, nRoot, &raw mut iStartBlock, pi);
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    break;
                }
                if isPrefix == 0 as ::core::ffi::c_int && isScan == 0 as ::core::ffi::c_int {
                    iLeavesEndBlock = iStartBlock;
                }
            }
            rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderNew(
                (*pCsr).nSegment + 1 as ::core::ffi::c_int,
                (isPrefix == 0 as ::core::ffi::c_int && isScan == 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int,
                iStartBlock,
                iLeavesEndBlock,
                iEndBlock,
                zRoot,
                nRoot,
                &raw mut pSeg_0,
            );
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                break;
            }
            rc = fts3SegReaderCursorAppend(pCsr, pSeg_0);
        }
    }
    let rc2: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
    if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
        rc = rc2;
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3SegReaderCursor(
    p: *mut crate::fts3Int_h::Fts3Table,
    iLangid: ::core::ffi::c_int,
    iIndex: ::core::ffi::c_int,
    iLevel: ::core::ffi::c_int,
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
    isPrefix: ::core::ffi::c_int,
    isScan: ::core::ffi::c_int,
    pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    ::libc::memset(
        pCsr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<crate::fts3Int_h::Fts3MultiSegReader>()
            as crate::__stddef_size_t_h::SizeT,
    );
    fts3SegReaderCursor(
        p, iLangid, iIndex, iLevel, zTerm, nTerm, isPrefix, isScan, pCsr,
    )
}

unsafe extern "C" fn fts3SegReaderCursorAddZero(
    p: *mut crate::fts3Int_h::Fts3Table,
    iLangid: ::core::ffi::c_int,
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
    pCsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    fts3SegReaderCursor(
        p,
        iLangid,
        0 as ::core::ffi::c_int,
        crate::fts3Int_h::FTS3_SEGCURSOR_ALL,
        zTerm,
        nTerm,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        pCsr,
    )
}

unsafe extern "C" fn fts3TermSegReaderCursor(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    zTerm: *const ::core::ffi::c_char,
    nTerm: ::core::ffi::c_int,
    isPrefix: ::core::ffi::c_int,
    ppSegcsr: *mut *mut crate::fts3Int_h::Fts3MultiSegReader,
) -> ::core::ffi::c_int {
    
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    let pSegcsr: *mut crate::fts3Int_h::Fts3MultiSegReader = crate::src::src::malloc::sqlite3_malloc(::core::mem::size_of::<
        crate::fts3Int_h::Fts3MultiSegReader,
    >() as ::core::ffi::c_int) as *mut crate::fts3Int_h::Fts3MultiSegReader;
    if !pSegcsr.is_null() {
        let mut i: ::core::ffi::c_int;
        let mut bFound: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let p: *mut crate::fts3Int_h::Fts3Table =
            (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
        if isPrefix != 0 {
            i = 1 as ::core::ffi::c_int;
            while bFound == 0 as ::core::ffi::c_int && i < (*p).nIndex {
                if (*(*p).aIndex.offset(i as isize)).nPrefix == nTerm {
                    bFound = 1 as ::core::ffi::c_int;
                    rc = sqlite3Fts3SegReaderCursor(
                        p,
                        (*pCsr).iLangid,
                        i,
                        crate::fts3Int_h::FTS3_SEGCURSOR_ALL,
                        zTerm,
                        nTerm,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        pSegcsr,
                    );
                    (*pSegcsr).bLookup = 1 as ::core::ffi::c_int;
                }
                i += 1;
            }
            i = 1 as ::core::ffi::c_int;
            while bFound == 0 as ::core::ffi::c_int && i < (*p).nIndex {
                if (*(*p).aIndex.offset(i as isize)).nPrefix == nTerm + 1 as ::core::ffi::c_int {
                    bFound = 1 as ::core::ffi::c_int;
                    rc = sqlite3Fts3SegReaderCursor(
                        p,
                        (*pCsr).iLangid,
                        i,
                        crate::fts3Int_h::FTS3_SEGCURSOR_ALL,
                        zTerm,
                        nTerm,
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        pSegcsr,
                    );
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        rc = fts3SegReaderCursorAddZero(p, (*pCsr).iLangid, zTerm, nTerm, pSegcsr);
                    }
                }
                i += 1;
            }
        }
        if bFound == 0 as ::core::ffi::c_int {
            rc = sqlite3Fts3SegReaderCursor(
                p,
                (*pCsr).iLangid,
                0 as ::core::ffi::c_int,
                crate::fts3Int_h::FTS3_SEGCURSOR_ALL,
                zTerm,
                nTerm,
                isPrefix,
                0 as ::core::ffi::c_int,
                pSegcsr,
            );
            (*pSegcsr).bLookup = (isPrefix == 0) as ::core::ffi::c_int;
        }
    }
    *ppSegcsr = pSegcsr;
    rc
}

unsafe extern "C" fn fts3SegReaderCursorFree(
    pSegcsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
) {
    crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderFinish(
        pSegcsr as *mut crate::fts3Int_h::Fts3MultiSegReader,
    );
    crate::src::src::malloc::sqlite3_free(pSegcsr as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn fts3TermSelect(
    p: *mut crate::fts3Int_h::Fts3Table,
    pTok: *mut crate::fts3Int_h::Fts3PhraseToken,
    iColumn: ::core::ffi::c_int,
    pnOut: *mut ::core::ffi::c_int,
    ppOut: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    
    let mut tsc: TermSelect = unsafe { ::core::mem::zeroed() };
    let mut filter: crate::fts3Int_h::Fts3SegFilter = unsafe { ::core::mem::zeroed() };
    let __pTok_ref = unsafe { &mut *pTok };
    let pSegcsr: *mut crate::fts3Int_h::Fts3MultiSegReader = __pTok_ref.pSegcsr;
    filter.flags = crate::fts3Int_h::FTS3_SEGMENT_IGNORE_EMPTY
        | crate::fts3Int_h::FTS3_SEGMENT_REQUIRE_POS
        | (if __pTok_ref.isPrefix != 0 {
            crate::fts3Int_h::FTS3_SEGMENT_PREFIX
        } else {
            0 as ::core::ffi::c_int
        })
        | (if __pTok_ref.bFirst != 0 {
            crate::fts3Int_h::FTS3_SEGMENT_FIRST
        } else {
            0 as ::core::ffi::c_int
        })
        | (if iColumn < (*p).nColumn {
            crate::fts3Int_h::FTS3_SEGMENT_COLUMN_FILTER
        } else {
            0 as ::core::ffi::c_int
        });
    filter.iCol = iColumn;
    filter.zTerm = __pTok_ref.z;
    filter.nTerm = __pTok_ref.n;
    rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderStart(
        p as *mut crate::fts3Int_h::Fts3Table,
        pSegcsr as *mut crate::fts3Int_h::Fts3MultiSegReader,
        &raw mut filter as *mut _ as *mut crate::fts3Int_h::Fts3SegFilter,
    );
    while crate::src::headers::sqlite3_h::SQLITE_OK == rc && {
        rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderStep(
            p as *mut crate::fts3Int_h::Fts3Table,
            pSegcsr as *mut crate::fts3Int_h::Fts3MultiSegReader,
        );
        crate::src::headers::sqlite3_h::SQLITE_ROW == rc
    } {
        rc = fts3TermSelectMerge(p, &raw mut tsc, (*pSegcsr).aDoclist, (*pSegcsr).nDoclist);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = fts3TermSelectFinishMerge(p, &raw mut tsc);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        *ppOut = tsc.aaOutput[0 as ::core::ffi::c_int as usize];
        *pnOut = tsc.anOutput[0 as ::core::ffi::c_int as usize];
    } else {
        let mut i: ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i
            < (::core::mem::size_of::<[*mut ::core::ffi::c_char; 16]>() as usize)
                .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                as ::core::ffi::c_int
        {
            crate::src::src::malloc::sqlite3_free(
                tsc.aaOutput[i as usize] as *mut ::core::ffi::c_void,
            );
            i += 1;
        }
    }
    fts3SegReaderCursorFree(pSegcsr);
    __pTok_ref.pSegcsr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3MultiSegReader>();
    rc
}

unsafe extern "C" fn fts3DoclistCountDocids(
    aList: *mut ::core::ffi::c_char,
    nList: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nDoc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !aList.is_null() {
        let aEnd: *mut ::core::ffi::c_char =
            aList.offset(nList as isize) as *mut ::core::ffi::c_char;
        let mut p: *mut ::core::ffi::c_char = aList;
        while p < aEnd {
            nDoc += 1;
            loop {
                let fresh36 = p;
                p = p.offset(1);
                if (*fresh36 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0) {
                    break;
                }
            }
            fts3PoslistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut p,
            );
        }
    }
    nDoc
}

unsafe extern "C" fn fts3NextMethod(
    pCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int;
    let pCsr: *mut crate::fts3Int_h::Fts3Cursor = pCursor as *mut crate::fts3Int_h::Fts3Cursor;
    if (*pCsr).eSearch as ::core::ffi::c_int == crate::fts3Int_h::FTS3_DOCID_SEARCH
        || (*pCsr).eSearch as ::core::ffi::c_int == crate::fts3Int_h::FTS3_FULLSCAN_SEARCH
    {
        let pTab: *mut crate::fts3Int_h::Fts3Table =
            (*pCursor).pVtab as *mut crate::fts3Int_h::Fts3Table;
        (*pTab).bLock += 1;
        if crate::src::headers::sqlite3_h::SQLITE_ROW
            != crate::src::src::vdbeapi::sqlite3_step((*pCsr).pStmt)
        {
            (*pCsr).isEof = 1 as crate::src::ext::rtree::rtree::U8_0;
            rc = crate::src::src::vdbeapi::sqlite3_reset((*pCsr).pStmt);
        } else {
            (*pCsr).iPrevId = crate::src::src::vdbeapi::sqlite3_column_int64(
                (*pCsr).pStmt,
                0 as ::core::ffi::c_int,
            );
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        (*pTab).bLock -= 1;
    } else {
        rc = fts3EvalNext(pCursor as *mut crate::fts3Int_h::Fts3Cursor);
    }
    rc
}

unsafe extern "C" fn fts3DocidRange(
    pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    iDefault: crate::src::ext::rtree::rtree::I64_0,
) -> crate::src::headers::sqlite3_h::Sqlite3Int64 {
    if !pVal.is_null() {
        let eType: ::core::ffi::c_int = crate::src::src::vdbe::sqlite3_value_numeric_type(pVal);
        if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER {
            return crate::src::src::vdbeapi::sqlite3_value_int64(pVal);
        }
    }
    iDefault as crate::src::headers::sqlite3_h::Sqlite3Int64
}

unsafe extern "C" fn fts3FilterMethod(
    pCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    idxNum: ::core::ffi::c_int,
    idxStr: *const ::core::ffi::c_char,
    mut _nVal: ::core::ffi::c_int,
    apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let zSql: *mut ::core::ffi::c_char;
    
    let p: *mut crate::fts3Int_h::Fts3Table =
        (*pCursor).pVtab as *mut crate::fts3Int_h::Fts3Table;
    let pCsr: *mut crate::fts3Int_h::Fts3Cursor = pCursor as *mut crate::fts3Int_h::Fts3Cursor;
    let mut pCons: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
        ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
    let mut pLangid: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
        ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
    let mut pDocidGe: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
        ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
    let mut pDocidLe: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
        ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
    let mut iIdx: ::core::ffi::c_int;
    if (*p).bLock != 0 {
        return crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    let eSearch: ::core::ffi::c_int = idxNum & 0xffff as ::core::ffi::c_int;
    iIdx = 0 as ::core::ffi::c_int;
    if eSearch != crate::fts3Int_h::FTS3_FULLSCAN_SEARCH {
        let fresh32 = iIdx;
        iIdx += 1;
        pCons = *apVal.offset(fresh32 as isize);
    }
    if idxNum & crate::fts3Int_h::FTS3_HAVE_LANGID != 0 {
        let fresh33 = iIdx;
        iIdx += 1;
        pLangid = *apVal.offset(fresh33 as isize);
    }
    if idxNum & crate::fts3Int_h::FTS3_HAVE_DOCID_GE != 0 {
        let fresh34 = iIdx;
        iIdx += 1;
        pDocidGe = *apVal.offset(fresh34 as isize);
    }
    if idxNum & crate::fts3Int_h::FTS3_HAVE_DOCID_LE != 0 {
        let fresh35 = iIdx;
        pDocidLe = *apVal.offset(fresh35 as isize);
    }
    fts3ClearCursor(pCsr);
    let __pCsr_ref = unsafe { &mut *pCsr };
    __pCsr_ref.iMinDocid = fts3DocidRange(pDocidGe, crate::fts3Int_h::SMALLEST_INT64)
        as crate::src::ext::rtree::rtree::I64_0;
    __pCsr_ref.iMaxDocid = fts3DocidRange(pDocidLe, crate::fts3Int_h::LARGEST_INT64)
        as crate::src::ext::rtree::rtree::I64_0;
    if !idxStr.is_null() {
        __pCsr_ref.bDesc = (*idxStr.offset(0_isize) as ::core::ffi::c_int == 'D' as i32)
            as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U8_0;
    } else {
        __pCsr_ref.bDesc = (*p).bDescIdx;
    }
    __pCsr_ref.eSearch = eSearch as crate::src::fts5::I16_0;
    if eSearch != crate::fts3Int_h::FTS3_DOCID_SEARCH
        && eSearch != crate::fts3Int_h::FTS3_FULLSCAN_SEARCH
    {
        let iCol: ::core::ffi::c_int = eSearch - crate::fts3Int_h::FTS3_FULLTEXT_SEARCH;
        let zQuery: *const ::core::ffi::c_char =
            crate::src::src::vdbeapi::sqlite3_value_text(pCons) as *const ::core::ffi::c_char;
        if zQuery.is_null()
            && crate::src::src::vdbeapi::sqlite3_value_type(pCons)
                != crate::src::headers::sqlite3_h::SQLITE_NULL
        {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        __pCsr_ref.iLangid = 0 as ::core::ffi::c_int;
        if !pLangid.is_null() {
            __pCsr_ref.iLangid = crate::src::src::vdbeapi::sqlite3_value_int(pLangid);
        }
        let __p_ref = unsafe { &mut *p };
        rc = crate::src::ext::fts3::fts3_expr::sqlite3Fts3ExprParse(
            __p_ref.pTokenizer as *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
            __pCsr_ref.iLangid,
            __p_ref.azColumn,
            __p_ref.bFts4 as ::core::ffi::c_int,
            __p_ref.nColumn,
            iCol,
            zQuery,
            -(1 as ::core::ffi::c_int),
            &raw mut __pCsr_ref.pExpr as *mut _ as *mut *mut crate::fts3Int_h::Fts3Expr,
            &raw mut __p_ref.base.zErrMsg,
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        rc = fts3EvalStart(pCsr);
        crate::src::ext::fts3::fts3_write::sqlite3Fts3SegmentsClose(
            p as *mut crate::fts3Int_h::Fts3Table,
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        __pCsr_ref.pNextId = __pCsr_ref.aDoclist;
        __pCsr_ref.iPrevId = 0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    }
    if eSearch == crate::fts3Int_h::FTS3_FULLSCAN_SEARCH {
        if !pDocidGe.is_null() || !pDocidLe.is_null() {
            zSql = crate::sqlite_printf!(
                "SELECT %s WHERE rowid BETWEEN %lld AND %lld ORDER BY rowid %s",
                (*p).zReadExprlist,
                __pCsr_ref.iMinDocid,
                __pCsr_ref.iMaxDocid,
                if __pCsr_ref.bDesc as ::core::ffi::c_int != 0 {
                    b"DESC\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"ASC\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
        } else {
            zSql = crate::sqlite_printf!(
                "SELECT %s ORDER BY rowid %s",
                (*p).zReadExprlist,
                if __pCsr_ref.bDesc as ::core::ffi::c_int != 0 {
                    b"DESC\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"ASC\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
        }
        if !zSql.is_null() {
            let __p_ref = unsafe { &mut *p };
            __p_ref.bLock += 1;
            rc = crate::src::src::prepare::sqlite3_prepare_v3(
                __p_ref.db,
                zSql,
                -(1 as ::core::ffi::c_int),
                crate::src::headers::sqlite3_h::SQLITE_PREPARE_PERSISTENT as ::core::ffi::c_uint,
                &raw mut __pCsr_ref.pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            __p_ref.bLock -= 1;
            crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
        } else {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    } else if eSearch == crate::fts3Int_h::FTS3_DOCID_SEARCH {
        rc = fts3CursorSeekStmt(pCsr);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = crate::src::src::vdbeapi::sqlite3_bind_value(
                __pCsr_ref.pStmt,
                1 as ::core::ffi::c_int,
                pCons,
            );
        }
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    fts3NextMethod(pCursor)
}

unsafe extern "C" fn fts3EofMethod(
    pCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let pCsr: *mut crate::fts3Int_h::Fts3Cursor = pCursor as *mut crate::fts3Int_h::Fts3Cursor;
    if (*pCsr).isEof != 0 {
        fts3ClearCursor(pCsr);
        (*pCsr).isEof = 1 as crate::src::ext::rtree::rtree::U8_0;
    }
    (*pCsr).isEof as ::core::ffi::c_int
}

unsafe extern "C" fn fts3RowidMethod(
    pCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    pRowid: *mut crate::src::headers::sqlite3_h::SqliteInt64,
) -> ::core::ffi::c_int {
    let pCsr = &*(pCursor as *mut crate::fts3Int_h::Fts3Cursor);
    *pRowid = pCsr.iPrevId as crate::src::headers::sqlite3_h::SqliteInt64;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3ColumnMethod(
    pCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    pCtx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let pCsr: *mut crate::fts3Int_h::Fts3Cursor = pCursor as *mut crate::fts3Int_h::Fts3Cursor;
    let p: *mut crate::fts3Int_h::Fts3Table =
        (*pCursor).pVtab as *mut crate::fts3Int_h::Fts3Table;
    let current_block_10: u64;
    match iCol - (*p).nColumn {
        0 => {
            crate::src::src::vdbeapi::sqlite3_result_pointer(
                pCtx,
                pCsr as *mut ::core::ffi::c_void,
                b"fts3cursor\0" as *const u8 as *const ::core::ffi::c_char,
                None,
            );
            current_block_10 = 7976072742316086414;
        }
        1 => {
            crate::src::src::vdbeapi::sqlite3_result_int64(pCtx, (*pCsr).iPrevId);
            current_block_10 = 7976072742316086414;
        }
        2 => {
            if !(*pCsr).pExpr.is_null() {
                crate::src::src::vdbeapi::sqlite3_result_int64(
                    pCtx,
                    (*pCsr).iLangid as crate::src::headers::sqlite3_h::Sqlite3Int64,
                );
                current_block_10 = 7976072742316086414;
            } else if (*p).zLanguageid.is_null() {
                crate::src::src::vdbeapi::sqlite3_result_int(pCtx, 0 as ::core::ffi::c_int);
                current_block_10 = 7976072742316086414;
            } else {
                iCol = (*p).nColumn;
                current_block_10 = 9242256417102454865;
            }
        }
        _ => {
            current_block_10 = 9242256417102454865;
        }
    }
    if current_block_10 == 9242256417102454865 {
        rc = fts3CursorSeek(
            ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_context>(),
            pCsr,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK
            && crate::src::src::vdbeapi::sqlite3_data_count((*pCsr).pStmt)
                - 1 as ::core::ffi::c_int
                > iCol
        {
            crate::src::src::vdbeapi::sqlite3_result_value(
                pCtx,
                crate::src::src::vdbeapi::sqlite3_column_value(
                    (*pCsr).pStmt,
                    iCol + 1 as ::core::ffi::c_int,
                ),
            );
        }
    }
    rc
}

unsafe extern "C" fn fts3UpdateMethod(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    nArg: ::core::ffi::c_int,
    apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pRowid: *mut crate::src::headers::sqlite3_h::SqliteInt64,
) -> ::core::ffi::c_int {
    crate::src::ext::fts3::fts3_write::sqlite3Fts3UpdateMethod(
        pVtab as *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
        nArg,
        apVal,
        pRowid as *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
    )
}

unsafe extern "C" fn fts3SyncMethod(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
) -> ::core::ffi::c_int {
    let nMinMerge: crate::src::ext::rtree::rtree::U32_0 =
        64 as crate::src::ext::rtree::rtree::U32_0;
    let p: *mut crate::fts3Int_h::Fts3Table = pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut rc: ::core::ffi::c_int;
    let __p_ref = unsafe { &mut *p };
    let iLastRowid: crate::src::ext::rtree::rtree::I64_0 =
        crate::src::src::main::sqlite3_last_insert_rowid(__p_ref.db)
            as crate::src::ext::rtree::rtree::I64_0;
    rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3PendingTermsFlush(
        p as *mut crate::fts3Int_h::Fts3Table,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && __p_ref.nLeafAdd > nMinMerge.wrapping_div(16 as crate::src::ext::rtree::rtree::U32_0)
        && __p_ref.nAutoincrmerge != 0
        && __p_ref.nAutoincrmerge != 0xff as ::core::ffi::c_int
    {
        let mut mxLevel: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut A: ::core::ffi::c_int;
        rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3MaxLevel(
            p as *mut crate::fts3Int_h::Fts3Table,
            &raw mut mxLevel,
        );
        A = __p_ref
            .nLeafAdd
            .wrapping_mul(mxLevel as crate::src::ext::rtree::rtree::U32_0)
            as ::core::ffi::c_int;
        A += A / 2 as ::core::ffi::c_int;
        if A > nMinMerge as ::core::ffi::c_int {
            rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3Incrmerge(
                p as *mut crate::fts3Int_h::Fts3Table,
                A,
                __p_ref.nAutoincrmerge,
            );
        }
    }
    crate::src::ext::fts3::fts3_write::sqlite3Fts3SegmentsClose(
        p as *mut crate::fts3Int_h::Fts3Table,
    );
    crate::src::src::main::sqlite3_set_last_insert_rowid(
        __p_ref.db,
        iLastRowid as crate::src::headers::sqlite3_h::Sqlite3Int64,
    );
    rc
}

unsafe extern "C" fn fts3SetHasStat(p: *mut crate::fts3Int_h::Fts3Table) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*p).bHasStat as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        let zTbl: *mut ::core::ffi::c_char = crate::sqlite_printf!("%s_stat", (*p).zName,);
        if !zTbl.is_null() {
            let __p_ref = unsafe { &mut *p };
            let res: ::core::ffi::c_int = crate::src::src::main::sqlite3_table_column_metadata(
                __p_ref.db,
                __p_ref.zDb,
                zTbl,
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            crate::src::src::malloc::sqlite3_free(zTbl as *mut ::core::ffi::c_void);
            __p_ref.bHasStat = (res == crate::src::headers::sqlite3_h::SQLITE_OK)
                as ::core::ffi::c_int
                as crate::src::ext::rtree::rtree::U8_0;
        } else {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    }
    rc
}

unsafe extern "C" fn fts3BeginMethod(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
) -> ::core::ffi::c_int {
    let p: *mut crate::fts3Int_h::Fts3Table = pVtab as *mut crate::fts3Int_h::Fts3Table;
    
    (*p).nLeafAdd = 0 as crate::src::ext::rtree::rtree::U32_0;
    let rc: ::core::ffi::c_int = fts3SetHasStat(p);
    rc
}

unsafe extern "C" fn fts3CommitMethod(
    mut _pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
) -> ::core::ffi::c_int {
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3RollbackMethod(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
) -> ::core::ffi::c_int {
    let p: *mut crate::fts3Int_h::Fts3Table = pVtab as *mut crate::fts3Int_h::Fts3Table;
    crate::src::ext::fts3::fts3_write::sqlite3Fts3PendingTermsClear(
        p as *mut crate::fts3Int_h::Fts3Table,
    );
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3ReversePoslist(
    pStart: *mut ::core::ffi::c_char,
    ppPoslist: *mut *mut ::core::ffi::c_char,
) {
    let mut p: *mut ::core::ffi::c_char =
        (*ppPoslist).offset(-(2 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_char;
    let mut c: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    while p > pStart && {
        let fresh16 = p;
        p = p.offset(-1);
        c = *fresh16;
        c as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    } {}
    while p > pStart
        && *p as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int | c as ::core::ffi::c_int != 0
    {
        let fresh17 = p;
        p = p.offset(-1);
        c = *fresh17;
    }
    if p > pStart
        || c as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && *ppPoslist > p.offset(2_isize) as *mut ::core::ffi::c_char
    {
        p = p.offset(2_isize) as *mut ::core::ffi::c_char;
    }
    loop {
        let fresh18 = p;
        p = p.offset(1);
        if (*fresh18 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0) {
            break;
        }
    }
    *ppPoslist = p;
}

unsafe extern "C" fn fts3FunctionArg(
    pContext: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    zFunc: *const ::core::ffi::c_char,
    pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    ppCsr: *mut *mut crate::fts3Int_h::Fts3Cursor,
) -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int;
    *ppCsr = crate::src::src::vdbeapi::sqlite3_value_pointer(
        pVal,
        b"fts3cursor\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut crate::fts3Int_h::Fts3Cursor;
    if !(*ppCsr).is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    } else {
        let zErr: *mut ::core::ffi::c_char =
            crate::sqlite_printf!("illegal first argument to %s", zFunc,);
        crate::src::src::vdbeapi::sqlite3_result_error(pContext, zErr, -(1 as ::core::ffi::c_int));
        crate::src::src::malloc::sqlite3_free(zErr as *mut ::core::ffi::c_void);
        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    rc
}

unsafe extern "C" fn fts3SnippetFunc(
    pContext: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    nVal: ::core::ffi::c_int,
    apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let mut pCsr: *mut crate::fts3Int_h::Fts3Cursor =
        ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Cursor>();
    let mut zStart: *const ::core::ffi::c_char =
        b"<b>\0" as *const u8 as *const ::core::ffi::c_char;
    let mut zEnd: *const ::core::ffi::c_char = b"</b>\0" as *const u8 as *const ::core::ffi::c_char;
    let mut zEllipsis: *const ::core::ffi::c_char =
        b"<b>...</b>\0" as *const u8 as *const ::core::ffi::c_char;
    let mut iCol: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut nToken: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
    if nVal > 6 as ::core::ffi::c_int {
        crate::src::src::vdbeapi::sqlite3_result_error(
            pContext,
            b"wrong number of arguments to function snippet()\0" as *const u8
                as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int),
        );
        return;
    }
    if fts3FunctionArg(
        pContext,
        b"snippet\0" as *const u8 as *const ::core::ffi::c_char,
        *apVal.offset(0_isize),
        &raw mut pCsr,
    ) != 0
    {
        return;
    }
    let mut current_block_11: u64;
    match nVal {
        6 => {
            nToken = crate::src::src::vdbeapi::sqlite3_value_int(*apVal.offset(5_isize));
            current_block_11 = 10414022648205370886;
        }
        5 => {
            current_block_11 = 10414022648205370886;
        }
        4 => {
            current_block_11 = 11896608253015855976;
        }
        3 => {
            current_block_11 = 5542415300624813314;
        }
        2 => {
            current_block_11 = 6920105402610931133;
        }
        _ => {
            current_block_11 = 11050875288958768710;
        }
    }
    if current_block_11 == 10414022648205370886 {
        iCol = crate::src::src::vdbeapi::sqlite3_value_int(*apVal.offset(4_isize));
        current_block_11 = 11896608253015855976;
    }
    if current_block_11 == 11896608253015855976 {
        zEllipsis = crate::src::src::vdbeapi::sqlite3_value_text(*apVal.offset(3_isize))
            as *const ::core::ffi::c_char;
        current_block_11 = 5542415300624813314;
    }
    if current_block_11 == 5542415300624813314 {
        zEnd = crate::src::src::vdbeapi::sqlite3_value_text(*apVal.offset(2_isize))
            as *const ::core::ffi::c_char;
        current_block_11 = 6920105402610931133;
    }
    if current_block_11 == 6920105402610931133 {
        zStart = crate::src::src::vdbeapi::sqlite3_value_text(*apVal.offset(1_isize))
            as *const ::core::ffi::c_char;
    }
    if zEllipsis.is_null() || zEnd.is_null() || zStart.is_null() {
        crate::src::src::vdbeapi::sqlite3_result_error_nomem(pContext);
    } else if nToken == 0 as ::core::ffi::c_int {
        crate::src::src::vdbeapi::sqlite3_result_text(
            pContext,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int),
            crate::src::headers::sqlite3_h::SQLITE_STATIC,
        );
    } else if crate::src::headers::sqlite3_h::SQLITE_OK == fts3CursorSeek(pContext, pCsr) {
        crate::src::ext::fts3::fts3_snippet::sqlite3Fts3Snippet(
            pContext,
            pCsr as *mut crate::fts3Int_h::Fts3Cursor,
            zStart,
            zEnd,
            zEllipsis,
            iCol,
            nToken,
        );
    }
}

unsafe extern "C" fn fts3OffsetsFunc(
    pContext: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut _nVal: ::core::ffi::c_int,
    apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let mut pCsr: *mut crate::fts3Int_h::Fts3Cursor =
        ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Cursor>();
    if fts3FunctionArg(
        pContext,
        b"offsets\0" as *const u8 as *const ::core::ffi::c_char,
        *apVal.offset(0_isize),
        &raw mut pCsr,
    ) != 0
    {
        return;
    }
    if crate::src::headers::sqlite3_h::SQLITE_OK == fts3CursorSeek(pContext, pCsr) {
        crate::src::ext::fts3::fts3_snippet::sqlite3Fts3Offsets(
            pContext,
            pCsr as *mut crate::fts3Int_h::Fts3Cursor,
        );
    }
}

unsafe extern "C" fn fts3OptimizeFunc(
    pContext: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut _nVal: ::core::ffi::c_int,
    apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    
    
    let mut pCursor: *mut crate::fts3Int_h::Fts3Cursor =
        ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Cursor>();
    if fts3FunctionArg(
        pContext,
        b"optimize\0" as *const u8 as *const ::core::ffi::c_char,
        *apVal.offset(0_isize),
        &raw mut pCursor,
    ) != 0
    {
        return;
    }
    let p: *mut crate::fts3Int_h::Fts3Table = (*pCursor).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let rc: ::core::ffi::c_int = crate::src::ext::fts3::fts3_write::sqlite3Fts3Optimize(
        p as *mut crate::fts3Int_h::Fts3Table,
    );
    match rc {
        crate::src::headers::sqlite3_h::SQLITE_OK => {
            crate::src::src::vdbeapi::sqlite3_result_text(
                pContext,
                b"Index optimized\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int),
                crate::src::headers::sqlite3_h::SQLITE_STATIC,
            );
        }
        crate::src::headers::sqlite3_h::SQLITE_DONE => {
            crate::src::src::vdbeapi::sqlite3_result_text(
                pContext,
                b"Index already optimal\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int),
                crate::src::headers::sqlite3_h::SQLITE_STATIC,
            );
        }
        _ => {
            crate::src::src::vdbeapi::sqlite3_result_error_code(pContext, rc);
        }
    };
}

unsafe extern "C" fn fts3MatchinfoFunc(
    pContext: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    nVal: ::core::ffi::c_int,
    apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let mut pCsr: *mut crate::fts3Int_h::Fts3Cursor =
        ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Cursor>();
    if crate::src::headers::sqlite3_h::SQLITE_OK
        == fts3FunctionArg(
            pContext,
            b"matchinfo\0" as *const u8 as *const ::core::ffi::c_char,
            *apVal.offset(0_isize),
            &raw mut pCsr,
        )
    {
        let mut zArg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if nVal > 1 as ::core::ffi::c_int {
            zArg = crate::src::src::vdbeapi::sqlite3_value_text(*apVal.offset(1_isize))
                as *const ::core::ffi::c_char;
        }
        crate::src::ext::fts3::fts3_snippet::sqlite3Fts3Matchinfo(
            pContext,
            pCsr as *mut crate::fts3Int_h::Fts3Cursor,
            zArg,
        );
    }
}

unsafe extern "C" fn fts3FindFunctionMethod(
    mut _pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    mut _nArg: ::core::ffi::c_int,
    zName: *const ::core::ffi::c_char,
    pxFunc: *mut Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> (),
    >,
    mut _ppArg: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let aOverload: [Overloaded; 4] = [
        Overloaded {
            zName: b"snippet\0" as *const u8 as *const ::core::ffi::c_char,
            xFunc: Some(
                fts3SnippetFunc
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> (),
            ),
        },
        Overloaded {
            zName: b"offsets\0" as *const u8 as *const ::core::ffi::c_char,
            xFunc: Some(
                fts3OffsetsFunc
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> (),
            ),
        },
        Overloaded {
            zName: b"optimize\0" as *const u8 as *const ::core::ffi::c_char,
            xFunc: Some(
                fts3OptimizeFunc
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> (),
            ),
        },
        Overloaded {
            zName: b"matchinfo\0" as *const u8 as *const ::core::ffi::c_char,
            xFunc: Some(
                fts3MatchinfoFunc
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> (),
            ),
        },
    ];
    let mut i: ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[Overloaded; 4]>() as usize)
            .wrapping_div(::core::mem::size_of::<Overloaded>() as usize)
            as ::core::ffi::c_int
    {
        if ::libc::strcmp(zName, aOverload[i as usize].zName) == 0 as ::core::ffi::c_int {
            *pxFunc = aOverload[i as usize].xFunc;
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn fts3RenameMethod(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    zName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let p: *mut crate::fts3Int_h::Fts3Table = pVtab as *mut crate::fts3Int_h::Fts3Table;
    let __p_ref = unsafe { &mut *p };
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __p_ref.db;
    let mut rc: ::core::ffi::c_int;
    rc = fts3SetHasStat(p);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3PendingTermsFlush(
            p as *mut crate::fts3Int_h::Fts3Table,
        );
    }
    __p_ref.bIgnoreSavepoint = 1 as crate::src::ext::rtree::rtree::U8_0;
    if __p_ref.zContentTbl.is_null() {
        fts3_db_exec_sql(
            &raw mut rc,
            db,
            crate::sqlite_printf!(
                "ALTER TABLE %Q.'%q_content'  RENAME TO '%q_content';",
                __p_ref.zDb,
                __p_ref.zName,
                zName,
            ),
        );
    }
    if __p_ref.bHasDocsize != 0 {
        fts3_db_exec_sql(
            &raw mut rc,
            db,
            crate::sqlite_printf!(
                "ALTER TABLE %Q.'%q_docsize'  RENAME TO '%q_docsize';",
                __p_ref.zDb,
                __p_ref.zName,
                zName,
            ),
        );
    }
    if __p_ref.bHasStat != 0 {
        fts3_db_exec_sql(
            &raw mut rc,
            db,
            crate::sqlite_printf!(
                "ALTER TABLE %Q.'%q_stat'  RENAME TO '%q_stat';",
                __p_ref.zDb,
                __p_ref.zName,
                zName,
            ),
        );
    }
    fts3_db_exec_sql(
        &raw mut rc,
        db,
        crate::sqlite_printf!(
            "ALTER TABLE %Q.'%q_segments' RENAME TO '%q_segments';",
            __p_ref.zDb,
            __p_ref.zName,
            zName,
        ),
    );
    fts3_db_exec_sql(
        &raw mut rc,
        db,
        crate::sqlite_printf!(
            "ALTER TABLE %Q.'%q_segdir'   RENAME TO '%q_segdir';",
            __p_ref.zDb,
            __p_ref.zName,
            zName,
        ),
    );
    __p_ref.bIgnoreSavepoint = 0 as crate::src::ext::rtree::rtree::U8_0;
    rc
}

unsafe extern "C" fn fts3SavepointMethod(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let pTab: *mut crate::fts3Int_h::Fts3Table = pVtab as *mut crate::fts3Int_h::Fts3Table;
    if (*pTab).bIgnoreSavepoint as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if (*(*pTab).aIndex.offset(0_isize)).hPending.count > 0 as ::core::ffi::c_int {
            let __pTab_ref = unsafe { &mut *pTab };
            let zSql: *mut ::core::ffi::c_char = crate::sqlite_printf!(
                "INSERT INTO %Q.%Q(%Q) VALUES('flush')",
                __pTab_ref.zDb,
                __pTab_ref.zName,
                __pTab_ref.zName,
            );
            if !zSql.is_null() {
                __pTab_ref.bIgnoreSavepoint = 1 as crate::src::ext::rtree::rtree::U8_0;
                rc = crate::src::src::legacy::sqlite3_exec(
                    __pTab_ref.db,
                    zSql,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
                __pTab_ref.bIgnoreSavepoint = 0 as crate::src::ext::rtree::rtree::U8_0;
                crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
            } else {
                rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            (*pTab).iSavepoint = iSavepoint + 1 as ::core::ffi::c_int;
        }
    }
    rc
}

unsafe extern "C" fn fts3ReleaseMethod(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pTab = &mut *(pVtab as *mut crate::fts3Int_h::Fts3Table);
    pTab.iSavepoint = iSavepoint;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3RollbackToMethod(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pTab: *mut crate::fts3Int_h::Fts3Table = pVtab as *mut crate::fts3Int_h::Fts3Table;
    if iSavepoint + 1 as ::core::ffi::c_int <= (*pTab).iSavepoint {
        crate::src::ext::fts3::fts3_write::sqlite3Fts3PendingTermsClear(
            pTab as *mut crate::fts3Int_h::Fts3Table,
        );
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3ShadowName(zName: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    static mut azName: [*const ::core::ffi::c_char; 5] = [
        b"content\0" as *const u8 as *const ::core::ffi::c_char,
        b"docsize\0" as *const u8 as *const ::core::ffi::c_char,
        b"segdir\0" as *const u8 as *const ::core::ffi::c_char,
        b"segments\0" as *const u8 as *const ::core::ffi::c_char,
        b"stat\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut i: ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_uint;
    while (i as usize)
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 5]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if crate::src::src::util::sqlite3_stricmp(zName, azName[i as usize])
            == 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn fts3IntegrityMethod(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    zSchema: *const ::core::ffi::c_char,
    zTabname: *const ::core::ffi::c_char,
    mut _isQuick: ::core::ffi::c_int,
    pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let p: *mut crate::fts3Int_h::Fts3Table = pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut rc: ::core::ffi::c_int;
    let mut bOk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3IntegrityCheck(
        p as *mut crate::fts3Int_h::Fts3Table,
        &raw mut bOk,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_ERROR
        || rc & 0xff as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_CORRUPT
    {
        *pzErr = crate::sqlite_printf!(
            "unable to validate the inverted index for FTS%d table %s.%s: %s",
            if (*p).bFts4 as ::core::ffi::c_int != 0 {
                4 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            },
            zSchema,
            zTabname,
            crate::src::src::main::sqlite3_errstr(rc),
        );
        if !(*pzErr).is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    } else if rc == crate::src::headers::sqlite3_h::SQLITE_OK && bOk == 0 as ::core::ffi::c_int {
        *pzErr = crate::sqlite_printf!(
            "malformed inverted index for FTS%d table %s.%s",
            if (*p).bFts4 as ::core::ffi::c_int != 0 {
                4 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            },
            zSchema,
            zTabname,
        );
        if (*pzErr).is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    }
    crate::src::ext::fts3::fts3_write::sqlite3Fts3SegmentsClose(
        p as *mut crate::fts3Int_h::Fts3Table,
    );
    rc
}

static mut fts3Module: crate::src::headers::sqlite3_h::sqlite3_module =
    crate::src::headers::sqlite3_h::sqlite3_module {
        iVersion: 4 as ::core::ffi::c_int,
        xCreate: Some(
            fts3CreateMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqliteInt_h::sqlite3,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const *const ::core::ffi::c_char,
                    *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xConnect: Some(
            fts3ConnectMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqliteInt_h::sqlite3,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const *const ::core::ffi::c_char,
                    *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xBestIndex: Some(
            fts3BestIndexMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            fts3DisconnectMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                ) -> ::core::ffi::c_int,
        ),
        xDestroy: Some(
            fts3DestroyMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                ) -> ::core::ffi::c_int,
        ),
        xOpen: Some(
            fts3OpenMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            fts3CloseMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            fts3FilterMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            fts3NextMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            fts3EofMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            fts3ColumnMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                    *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            fts3RowidMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                    *mut crate::src::headers::sqlite3_h::SqliteInt64,
                ) -> ::core::ffi::c_int,
        ),
        xUpdate: Some(
            fts3UpdateMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    ::core::ffi::c_int,
                    *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    *mut crate::src::headers::sqlite3_h::SqliteInt64,
                ) -> ::core::ffi::c_int,
        ),
        xBegin: Some(
            fts3BeginMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                ) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            fts3SyncMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                ) -> ::core::ffi::c_int,
        ),
        xCommit: Some(
            fts3CommitMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                ) -> ::core::ffi::c_int,
        ),
        xRollback: Some(
            fts3RollbackMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                ) -> ::core::ffi::c_int,
        ),
        xFindFunction: Some(
            fts3FindFunctionMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    *mut Option<
                        unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                    >,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xRename: Some(
            fts3RenameMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xSavepoint: Some(
            fts3SavepointMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRelease: Some(
            fts3ReleaseMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRollbackTo: Some(
            fts3RollbackToMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xShadowName: Some(
            fts3ShadowName
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
        ),
        xIntegrity: Some(
            fts3IntegrityMethod
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    };

unsafe extern "C" fn hashDestroy(p: *mut ::core::ffi::c_void) {
    let pHash: *mut Fts3HashWrapper = p as *mut Fts3HashWrapper;
    (*pHash).nRef -= 1;
    if (*pHash).nRef <= 0 as ::core::ffi::c_int {
        crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashClear(
            &raw mut (*pHash).hash as *mut _ as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
        );
        crate::src::src::malloc::sqlite3_free(pHash as *mut ::core::ffi::c_void);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3Init(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    
    let mut pSimple: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module =
        ::core::ptr::null::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module>();
    let mut pPorter: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module =
        ::core::ptr::null::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module>();
    let mut pUnicode: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module =
        ::core::ptr::null::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module>();
    sqlite3Fts3UnicodeTokenizer(&raw mut pUnicode);
    #[cfg(feature = "test")]
    {
        rc = crate::fts3Int_h::sqlite3Fts3InitTerm(db);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
    }
    rc = crate::src::ext::fts3::fts3_aux::sqlite3Fts3InitAux(db);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    sqlite3Fts3SimpleTokenizerModule(&raw mut pSimple);
    sqlite3Fts3PorterTokenizerModule(&raw mut pPorter);
    let pHash: *mut Fts3HashWrapper = crate::src::src::malloc::sqlite3_malloc(
        ::core::mem::size_of::<Fts3HashWrapper>() as ::core::ffi::c_int
    ) as *mut Fts3HashWrapper;
    if pHash.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    } else {
        crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashInit(
            &raw mut (*pHash).hash as *mut _ as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
            crate::src::ext::fts3::fts3_hash::FTS3_HASH_STRING as ::core::ffi::c_char,
            1 as ::core::ffi::c_char,
        );
        (*pHash).nRef = 0 as ::core::ffi::c_int;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let __pHash_ref = unsafe { &mut *pHash };
        if !crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashInsert(
            &raw mut __pHash_ref.hash as *mut _ as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
            b"simple\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            7 as ::core::ffi::c_int,
            pSimple as *mut ::core::ffi::c_void,
        )
        .is_null()
            || !crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashInsert(
                &raw mut __pHash_ref.hash as *mut _
                    as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
                b"porter\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                7 as ::core::ffi::c_int,
                pPorter as *mut ::core::ffi::c_void,
            )
            .is_null()
            || !crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashInsert(
                &raw mut __pHash_ref.hash as *mut _
                    as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
                b"unicode61\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                10 as ::core::ffi::c_int,
                pUnicode as *mut ::core::ffi::c_void,
            )
            .is_null()
        {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = crate::src::ext::fts3::fts3_expr::sqlite3Fts3ExprInitTestInterface(
            db,
            &raw mut (*pHash).hash as *mut _ as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
        );
    }
    if crate::src::headers::sqlite3_h::SQLITE_OK == rc
        && {
            rc = crate::src::ext::fts3::fts3_tokenizer::sqlite3Fts3InitHashTable(
                db,
                &raw mut (*pHash).hash as *mut _ as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
                b"fts3_tokenizer\0" as *const u8 as *const ::core::ffi::c_char,
            );
            crate::src::headers::sqlite3_h::SQLITE_OK == rc
        }
        && {
            rc = crate::src::src::main::sqlite3_overload_function(
                db,
                b"snippet\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int),
            );
            crate::src::headers::sqlite3_h::SQLITE_OK == rc
        }
        && {
            rc = crate::src::src::main::sqlite3_overload_function(
                db,
                b"offsets\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
            crate::src::headers::sqlite3_h::SQLITE_OK == rc
        }
        && {
            rc = crate::src::src::main::sqlite3_overload_function(
                db,
                b"matchinfo\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
            crate::src::headers::sqlite3_h::SQLITE_OK == rc
        }
        && {
            rc = crate::src::src::main::sqlite3_overload_function(
                db,
                b"matchinfo\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            );
            crate::src::headers::sqlite3_h::SQLITE_OK == rc
        }
        && {
            rc = crate::src::src::main::sqlite3_overload_function(
                db,
                b"optimize\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
            crate::src::headers::sqlite3_h::SQLITE_OK == rc
        }
    {
        (*pHash).nRef += 1;
        rc = crate::src::src::vtab::sqlite3_create_module_v2(
            db,
            b"fts3\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const fts3Module as *const _
                as *const crate::src::headers::sqlite3_h::sqlite3_module,
            pHash as *mut ::core::ffi::c_void,
            Some(hashDestroy as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        #[cfg(feature = "fts4")]
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            (*pHash).nRef += 1;
            rc = crate::src::src::vtab::sqlite3_create_module_v2(
                db,
                b"fts4\0" as *const u8 as *const ::core::ffi::c_char,
                &raw const fts3Module as *const _
                    as *const crate::src::headers::sqlite3_h::sqlite3_module,
                pHash as *mut ::core::ffi::c_void,
                Some(hashDestroy as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            );
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            (*pHash).nRef += 1;
            rc = crate::src::ext::fts3::fts3_tokenize_vtab::sqlite3Fts3InitTok(
                db,
                pHash as *mut ::core::ffi::c_void as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash
                    as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
                Some(hashDestroy as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            );
        }
        return rc;
    }
    if !pHash.is_null() {
        crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashClear(
            &raw mut (*pHash).hash as *mut _ as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
        );
        crate::src::src::malloc::sqlite3_free(pHash as *mut ::core::ffi::c_void);
    }
    rc
}

unsafe extern "C" fn fts3EvalAllocateReaders(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    pnToken: *mut ::core::ffi::c_int,
    pnOr: *mut ::core::ffi::c_int,
    pRc: *mut ::core::ffi::c_int,
) {
    if !pExpr.is_null() && crate::src::headers::sqlite3_h::SQLITE_OK == *pRc {
        if (*pExpr).eType == crate::fts3Int_h::FTSQUERY_PHRASE {
            let mut i: ::core::ffi::c_int;
            let __pPhrase_ref = &mut *(*pExpr).pPhrase;
            let nToken: ::core::ffi::c_int = __pPhrase_ref.nToken;
            *pnToken += nToken;
            i = 0 as ::core::ffi::c_int;
            while i < nToken {
                let pToken: *mut crate::fts3Int_h::Fts3PhraseToken =
                    (&raw mut __pPhrase_ref.aToken as *mut crate::fts3Int_h::Fts3PhraseToken)
                        .offset(i as isize)
                        as *mut crate::fts3Int_h::Fts3PhraseToken;
                let __pToken_ref = unsafe { &mut *pToken };
                let rc: ::core::ffi::c_int = fts3TermSegReaderCursor(
                    pCsr,
                    __pToken_ref.z,
                    __pToken_ref.n,
                    __pToken_ref.isPrefix,
                    &raw mut __pToken_ref.pSegcsr,
                );
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    *pRc = rc;
                    return;
                }
                i += 1;
            }
            __pPhrase_ref.iDoclistToken = -(1 as ::core::ffi::c_int);
        } else {
            let __pExpr_ref = unsafe { &*pExpr };
            *pnOr += (__pExpr_ref.eType == crate::fts3Int_h::FTSQUERY_OR) as ::core::ffi::c_int;
            fts3EvalAllocateReaders(pCsr, __pExpr_ref.pLeft, pnToken, pnOr, pRc);
            fts3EvalAllocateReaders(pCsr, __pExpr_ref.pRight, pnToken, pnOr, pRc);
        }
    }
}

unsafe extern "C" fn fts3EvalPhraseMergeToken(
    pTab: *mut crate::fts3Int_h::Fts3Table,
    p: *mut crate::fts3Int_h::Fts3Phrase,
    iToken: ::core::ffi::c_int,
    pList: *mut ::core::ffi::c_char,
    nList: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __p_ref = unsafe { &mut *p };
    if pList.is_null() {
        crate::src::src::malloc::sqlite3_free(__p_ref.doclist.aAll as *mut ::core::ffi::c_void);
        __p_ref.doclist.aAll = ::core::ptr::null_mut::<::core::ffi::c_char>();
        __p_ref.doclist.nAll = 0 as ::core::ffi::c_int;
    } else if __p_ref.iDoclistToken < 0 as ::core::ffi::c_int {
        __p_ref.doclist.aAll = pList;
        __p_ref.doclist.nAll = nList;
    } else if __p_ref.doclist.aAll.is_null() {
        crate::src::src::malloc::sqlite3_free(pList as *mut ::core::ffi::c_void);
    } else {
        let pLeft: *mut ::core::ffi::c_char;
        let mut pRight: *mut ::core::ffi::c_char;
        let nLeft: ::core::ffi::c_int;
        let mut nRight: ::core::ffi::c_int;
        let nDiff: ::core::ffi::c_int;
        if __p_ref.iDoclistToken < iToken {
            pLeft = __p_ref.doclist.aAll;
            nLeft = __p_ref.doclist.nAll;
            pRight = pList;
            nRight = nList;
            nDiff = iToken - __p_ref.iDoclistToken;
        } else {
            pRight = __p_ref.doclist.aAll;
            nRight = __p_ref.doclist.nAll;
            pLeft = pList;
            nLeft = nList;
            nDiff = __p_ref.iDoclistToken - iToken;
        }
        rc = fts3DoclistPhraseMerge(
            (*pTab).bDescIdx as ::core::ffi::c_int,
            nDiff,
            pLeft,
            nLeft,
            &raw mut pRight,
            &raw mut nRight,
        );
        crate::src::src::malloc::sqlite3_free(pLeft as *mut ::core::ffi::c_void);
        __p_ref.doclist.aAll = pRight;
        __p_ref.doclist.nAll = nRight;
    }
    if iToken > __p_ref.iDoclistToken {
        __p_ref.iDoclistToken = iToken;
    }
    rc
}

unsafe extern "C" fn fts3EvalPhraseLoad(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    p: *mut crate::fts3Int_h::Fts3Phrase,
) -> ::core::ffi::c_int {
    let pTab: *mut crate::fts3Int_h::Fts3Table =
        (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut iToken: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    iToken = 0 as ::core::ffi::c_int;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && iToken < (*p).nToken {
        let pToken: *mut crate::fts3Int_h::Fts3PhraseToken = (&raw mut (*p).aToken
            as *mut crate::fts3Int_h::Fts3PhraseToken)
            .offset(iToken as isize)
            as *mut crate::fts3Int_h::Fts3PhraseToken;
        if !(*pToken).pSegcsr.is_null() {
            let mut nThis: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pThis: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            rc = fts3TermSelect(pTab, pToken, (*p).iColumn, &raw mut nThis, &raw mut pThis);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = fts3EvalPhraseMergeToken(pTab, p, iToken, pThis, nThis);
            }
        }
        iToken += 1;
    }
    rc
}

unsafe extern "C" fn fts3EvalDeferredPhrase(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pPhrase: *mut crate::fts3Int_h::Fts3Phrase,
) -> ::core::ffi::c_int {
    let mut iToken: ::core::ffi::c_int;
    let mut aPoslist: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nPoslist: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iPrev: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let __pPhrase_ref = unsafe { &mut *pPhrase };
    let aFree: *mut ::core::ffi::c_char = if __pPhrase_ref.doclist.bFreeList != 0 {
        __pPhrase_ref.doclist.pList
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    iToken = 0 as ::core::ffi::c_int;
    while iToken < __pPhrase_ref.nToken {
        let pToken = &*((&raw mut __pPhrase_ref.aToken as *mut crate::fts3Int_h::Fts3PhraseToken)
            .offset(iToken as isize)
            as *mut crate::fts3Int_h::Fts3PhraseToken);

        let pDeferred: *mut crate::fts3Int_h::Fts3DeferredToken = pToken.pDeferred;
        if !pDeferred.is_null() {
            let mut pList: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut nList: ::core::ffi::c_int = 0;
            let rc: ::core::ffi::c_int =
                crate::src::ext::fts3::fts3_write::sqlite3Fts3DeferredTokenList(
                    pDeferred,
                    &raw mut pList,
                    &raw mut nList,
                );
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            if pList.is_null() {
                crate::src::src::malloc::sqlite3_free(aPoslist as *mut ::core::ffi::c_void);
                crate::src::src::malloc::sqlite3_free(aFree as *mut ::core::ffi::c_void);
                __pPhrase_ref.doclist.pList = ::core::ptr::null_mut::<::core::ffi::c_char>();
                __pPhrase_ref.doclist.nList = 0 as ::core::ffi::c_int;
                return crate::src::headers::sqlite3_h::SQLITE_OK;
            } else if aPoslist.is_null() {
                aPoslist = pList;
                nPoslist = nList;
            } else {
                let mut aOut: *mut ::core::ffi::c_char = pList;
                let mut p1: *mut ::core::ffi::c_char = aPoslist;
                let mut p2: *mut ::core::ffi::c_char = aOut;
                fts3PoslistPhraseMerge(
                    &raw mut aOut,
                    iToken - iPrev,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    &raw mut p1,
                    &raw mut p2,
                );
                crate::src::src::malloc::sqlite3_free(aPoslist as *mut ::core::ffi::c_void);
                aPoslist = pList;
                nPoslist = aOut.offset_from(aPoslist) as ::core::ffi::c_long as ::core::ffi::c_int;
                if nPoslist == 0 as ::core::ffi::c_int {
                    crate::src::src::malloc::sqlite3_free(aPoslist as *mut ::core::ffi::c_void);
                    crate::src::src::malloc::sqlite3_free(aFree as *mut ::core::ffi::c_void);
                    __pPhrase_ref.doclist.pList = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    __pPhrase_ref.doclist.nList = 0 as ::core::ffi::c_int;
                    return crate::src::headers::sqlite3_h::SQLITE_OK;
                }
            }
            iPrev = iToken;
        }
        iToken += 1;
    }
    if iPrev >= 0 as ::core::ffi::c_int {
        let nMaxUndeferred: ::core::ffi::c_int = __pPhrase_ref.iDoclistToken;
        if nMaxUndeferred < 0 as ::core::ffi::c_int {
            __pPhrase_ref.doclist.pList = aPoslist;
            __pPhrase_ref.doclist.nList = nPoslist;
            __pPhrase_ref.doclist.iDocid = (*pCsr).iPrevId;
            __pPhrase_ref.doclist.bFreeList = 1 as ::core::ffi::c_int;
        } else {
            let nDistance: ::core::ffi::c_int;
            let mut p1_0: *mut ::core::ffi::c_char;
            let mut p2_0: *mut ::core::ffi::c_char;
            let mut aOut_0: *mut ::core::ffi::c_char;
            if nMaxUndeferred > iPrev {
                p1_0 = aPoslist;
                p2_0 = __pPhrase_ref.doclist.pList;
                nDistance = nMaxUndeferred - iPrev;
            } else {
                p1_0 = __pPhrase_ref.doclist.pList;
                p2_0 = aPoslist;
                nDistance = iPrev - nMaxUndeferred;
            }
            aOut_0 = crate::src::ext::fts3::fts3_expr::sqlite3Fts3MallocZero(
                nPoslist as crate::src::ext::rtree::rtree::I64_0
                    + crate::fts3Int_h::FTS3_BUFFER_PADDING as crate::src::ext::rtree::rtree::I64_0,
            ) as *mut ::core::ffi::c_char;
            if aOut_0.is_null() {
                crate::src::src::malloc::sqlite3_free(aPoslist as *mut ::core::ffi::c_void);
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
            __pPhrase_ref.doclist.pList = aOut_0;
            if fts3PoslistPhraseMerge(
                &raw mut aOut_0,
                nDistance,
                0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                &raw mut p1_0,
                &raw mut p2_0,
            ) != 0
            {
                __pPhrase_ref.doclist.bFreeList = 1 as ::core::ffi::c_int;
                __pPhrase_ref.doclist.nList = aOut_0.offset_from(__pPhrase_ref.doclist.pList)
                    as ::core::ffi::c_long
                    as ::core::ffi::c_int;
            } else {
                crate::src::src::malloc::sqlite3_free(aOut_0 as *mut ::core::ffi::c_void);
                __pPhrase_ref.doclist.pList = ::core::ptr::null_mut::<::core::ffi::c_char>();
                __pPhrase_ref.doclist.nList = 0 as ::core::ffi::c_int;
            }
            crate::src::src::malloc::sqlite3_free(aPoslist as *mut ::core::ffi::c_void);
        }
    }
    if __pPhrase_ref.doclist.pList != aFree {
        crate::src::src::malloc::sqlite3_free(aFree as *mut ::core::ffi::c_void);
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

pub const MAX_INCR_PHRASE_TOKENS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

unsafe extern "C" fn fts3EvalPhraseStart(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    bOptOk: ::core::ffi::c_int,
    p: *mut crate::fts3Int_h::Fts3Phrase,
) -> ::core::ffi::c_int {
    let pTab: *mut crate::fts3Int_h::Fts3Table =
        (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut i: ::core::ffi::c_int;
    let mut bHaveIncr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let __p_ref = unsafe { &mut *p };
    let mut bIncrOk: ::core::ffi::c_int = (bOptOk != 0
        && (*pCsr).bDesc as ::core::ffi::c_int == (*pTab).bDescIdx as ::core::ffi::c_int
        && __p_ref.nToken <= MAX_INCR_PHRASE_TOKENS
        && __p_ref.nToken > 0 as ::core::ffi::c_int
        && (*pTab).bNoIncrDoclist == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while bIncrOk == 1 as ::core::ffi::c_int && i < __p_ref.nToken {
        let pToken: *mut crate::fts3Int_h::Fts3PhraseToken =
            (&raw mut __p_ref.aToken as *mut crate::fts3Int_h::Fts3PhraseToken).offset(i as isize)
                as *mut crate::fts3Int_h::Fts3PhraseToken;
        let __pToken_ref = unsafe { &mut *pToken };
        if __pToken_ref.bFirst != 0
            || !__pToken_ref.pSegcsr.is_null() && (*__pToken_ref.pSegcsr).bLookup == 0
        {
            bIncrOk = 0 as ::core::ffi::c_int;
        }
        if !__pToken_ref.pSegcsr.is_null() {
            bHaveIncr = 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if bIncrOk != 0 && bHaveIncr != 0 {
        let iCol: ::core::ffi::c_int = if __p_ref.iColumn >= (*pTab).nColumn {
            -(1 as ::core::ffi::c_int)
        } else {
            __p_ref.iColumn
        };
        i = 0 as ::core::ffi::c_int;
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < __p_ref.nToken {
            let pToken_0: *mut crate::fts3Int_h::Fts3PhraseToken =
                (&raw mut __p_ref.aToken as *mut crate::fts3Int_h::Fts3PhraseToken)
                    .offset(i as isize) as *mut crate::fts3Int_h::Fts3PhraseToken;
            let pSegcsr: *mut crate::fts3Int_h::Fts3MultiSegReader = (*pToken_0).pSegcsr;
            if !pSegcsr.is_null() {
                rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3MsrIncrStart(
                    pTab as *mut crate::fts3Int_h::Fts3Table,
                    pSegcsr as *mut crate::fts3Int_h::Fts3MultiSegReader,
                    iCol,
                    (*pToken_0).z,
                    (*pToken_0).n,
                );
            }
            i += 1;
        }
        __p_ref.bIncr = 1 as ::core::ffi::c_int;
    } else {
        rc = fts3EvalPhraseLoad(pCsr, p);
        __p_ref.bIncr = 0 as ::core::ffi::c_int;
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3DoclistPrev(
    bDescIdx: ::core::ffi::c_int,
    aDoclist: *mut ::core::ffi::c_char,
    nDoclist: ::core::ffi::c_int,
    ppIter: *mut *mut ::core::ffi::c_char,
    piDocid: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
    pnList: *mut ::core::ffi::c_int,
    pbEof: *mut crate::src::ext::rtree::rtree::U8_0,
) {
    let mut p: *mut ::core::ffi::c_char = *ppIter;
    if p.is_null() {
        let mut iDocid: crate::src::headers::sqlite3_h::Sqlite3Int64 =
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        let mut pNext: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut pDocid: *mut ::core::ffi::c_char = aDoclist;
        let pEnd: *mut ::core::ffi::c_char =
            aDoclist.offset(nDoclist as isize) as *mut ::core::ffi::c_char;
        let mut iMul: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while pDocid < pEnd {
            let mut iDelta: crate::src::headers::sqlite3_h::Sqlite3Int64 = 0;
            pDocid = pDocid.offset(sqlite3Fts3GetVarint(pDocid, &raw mut iDelta) as isize);
            iDocid += iMul as crate::src::headers::sqlite3_h::Sqlite3Int64 * iDelta;
            pNext = pDocid;
            fts3PoslistCopy(
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                &raw mut pDocid,
            );
            while pDocid < pEnd && *pDocid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                pDocid = pDocid.offset(1);
            }
            iMul = if bDescIdx != 0 {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            };
        }
        *pnList = pEnd.offset_from(pNext) as ::core::ffi::c_long as ::core::ffi::c_int;
        *ppIter = pNext;
        *piDocid = iDocid;
    } else {
        let iMul_0: ::core::ffi::c_int = if bDescIdx != 0 {
            -(1 as ::core::ffi::c_int)
        } else {
            1 as ::core::ffi::c_int
        };
        let mut iDelta_0: crate::src::headers::sqlite3_h::Sqlite3Int64 = 0;
        fts3GetReverseVarint(&raw mut p, aDoclist, &raw mut iDelta_0);
        *piDocid -= iMul_0 as crate::src::headers::sqlite3_h::Sqlite3Int64 * iDelta_0;
        if p == aDoclist {
            *pbEof = 1 as crate::src::ext::rtree::rtree::U8_0;
        } else {
            let pSave: *mut ::core::ffi::c_char = p;
            fts3ReversePoslist(aDoclist, &raw mut p);
            *pnList = pSave.offset_from(p) as ::core::ffi::c_long as ::core::ffi::c_int;
        }
        *ppIter = p;
    };
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3DoclistNext(
    bDescIdx: ::core::ffi::c_int,
    aDoclist: *mut ::core::ffi::c_char,
    nDoclist: ::core::ffi::c_int,
    ppIter: *mut *mut ::core::ffi::c_char,
    piDocid: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
    pbEof: *mut crate::src::ext::rtree::rtree::U8_0,
) {
    let mut p: *mut ::core::ffi::c_char = *ppIter;
    if p.is_null() {
        p = aDoclist;
        p = p.offset(sqlite3Fts3GetVarint(
            p,
            piDocid as *mut crate::src::headers::sqlite3_h::SqliteInt64,
        ) as isize);
    } else {
        fts3PoslistCopy(
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            &raw mut p,
        );
        while p < aDoclist.offset(nDoclist as isize) as *mut ::core::ffi::c_char
            && *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        if p >= aDoclist.offset(nDoclist as isize) as *mut ::core::ffi::c_char {
            *pbEof = 1 as crate::src::ext::rtree::rtree::U8_0;
        } else {
            let mut iVar: crate::src::headers::sqlite3_h::Sqlite3Int64 = 0;
            p = p.offset(sqlite3Fts3GetVarint(p, &raw mut iVar) as isize);
            *piDocid += (if bDescIdx != 0 {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            }) as crate::src::headers::sqlite3_h::Sqlite3Int64
                * iVar;
        }
    }
    *ppIter = p;
}

unsafe extern "C" fn fts3EvalDlPhraseNext(
    pTab: *mut crate::fts3Int_h::Fts3Table,
    pDL: *mut crate::fts3Int_h::Fts3Doclist,
    pbEof: *mut crate::src::ext::rtree::rtree::U8_0,
) {
    let mut pIter: *mut ::core::ffi::c_char;
    let pEnd: *mut ::core::ffi::c_char;
    if !(*pDL).pNextDocid.is_null() {
        pIter = (*pDL).pNextDocid;
    } else {
        pIter = (*pDL).aAll;
    }
    if pIter.is_null() || {
        pEnd = (*pDL).aAll.offset((*pDL).nAll as isize);
        pIter >= pEnd
    } {
        *pbEof = 1 as crate::src::ext::rtree::rtree::U8_0;
    } else {
        let mut iDelta: crate::src::headers::sqlite3_h::Sqlite3Int64 = 0;
        pIter = pIter.offset(sqlite3Fts3GetVarint(pIter, &raw mut iDelta) as isize);
        let __pDL_ref = unsafe { &mut *pDL };
        if (*pTab).bDescIdx as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || __pDL_ref.pNextDocid.is_null()
        {
            __pDL_ref.iDocid += iDelta;
        } else {
            __pDL_ref.iDocid -= iDelta;
        }
        __pDL_ref.pList = pIter;
        fts3PoslistCopy(
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            &raw mut pIter,
        );
        __pDL_ref.nList =
            pIter.offset_from(__pDL_ref.pList) as ::core::ffi::c_long as ::core::ffi::c_int;
        while pIter < pEnd && *pIter as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            pIter = pIter.offset(1);
        }
        __pDL_ref.pNextDocid = pIter;
        *pbEof = 0 as crate::src::ext::rtree::rtree::U8_0;
    };
}

unsafe extern "C" fn incrPhraseTokenNext(
    pTab: *mut crate::fts3Int_h::Fts3Table,
    pPhrase: *mut crate::fts3Int_h::Fts3Phrase,
    iToken: ::core::ffi::c_int,
    p: *mut TokenDoclist,
    pbEof: *mut crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pPhrase).iDoclistToken == iToken {
        let __pPhrase_ref = unsafe { &mut *pPhrase };
        fts3EvalDlPhraseNext(pTab, &raw mut __pPhrase_ref.doclist, pbEof);
        let __p_ref = unsafe { &mut *p };
        __p_ref.pList = __pPhrase_ref.doclist.pList;
        __p_ref.nList = __pPhrase_ref.doclist.nList;
        __p_ref.iDocid = __pPhrase_ref.doclist.iDocid;
    } else {
        let pToken: *mut crate::fts3Int_h::Fts3PhraseToken =
            (&raw mut (*pPhrase).aToken as *mut crate::fts3Int_h::Fts3PhraseToken)
                .offset(iToken as isize) as *mut crate::fts3Int_h::Fts3PhraseToken;
        if !(*pToken).pSegcsr.is_null() {
            let __p_ref = unsafe { &mut *p };
            rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3MsrIncrNext(
                pTab as *mut crate::fts3Int_h::Fts3Table,
                (*pToken).pSegcsr as *mut crate::fts3Int_h::Fts3MultiSegReader,
                &raw mut __p_ref.iDocid,
                &raw mut __p_ref.pList,
                &raw mut __p_ref.nList,
            );
            if __p_ref.pList.is_null() {
                *pbEof = 1 as crate::src::ext::rtree::rtree::U8_0;
            }
        } else {
            (*p).bIgnore = 1 as ::core::ffi::c_int;
        }
    }
    rc
}

unsafe extern "C" fn fts3EvalIncrPhraseNext(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    p: *mut crate::fts3Int_h::Fts3Phrase,
    pbEof: *mut crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let pDL: *mut crate::fts3Int_h::Fts3Doclist = &raw mut (*p).doclist;
    let pTab: *mut crate::fts3Int_h::Fts3Table =
        (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut bEof: crate::src::ext::rtree::rtree::U8_0 = 0 as crate::src::ext::rtree::rtree::U8_0;
    if (*p).nToken == 1 as ::core::ffi::c_int {
        let __pDL_ref = unsafe { &mut *pDL };
        rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3MsrIncrNext(
            pTab as *mut crate::fts3Int_h::Fts3Table,
            (*(&raw mut (*p).aToken as *mut crate::fts3Int_h::Fts3PhraseToken).offset(0_isize))
                .pSegcsr as *mut crate::fts3Int_h::Fts3MultiSegReader,
            &raw mut __pDL_ref.iDocid,
            &raw mut __pDL_ref.pList,
            &raw mut __pDL_ref.nList,
        );
        if __pDL_ref.pList.is_null() {
            bEof = 1 as crate::src::ext::rtree::rtree::U8_0;
        }
    } else {
        let bDescDoclist: ::core::ffi::c_int = (*pCsr).bDesc as ::core::ffi::c_int;
        let mut a: [TokenDoclist; 4] = unsafe { ::core::mem::zeroed() };
        while bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut bMaxSet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut iMax: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            let mut i: ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            let __p_ref = unsafe { &*p };
            while rc == crate::src::headers::sqlite3_h::SQLITE_OK
                && i < __p_ref.nToken
                && bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                rc = incrPhraseTokenNext(
                    pTab,
                    p,
                    i,
                    (&raw mut a as *mut TokenDoclist).offset(i as isize) as *mut TokenDoclist,
                    &raw mut bEof,
                );
                if a[i as usize].bIgnore == 0 as ::core::ffi::c_int
                    && (bMaxSet == 0 as ::core::ffi::c_int
                        || (if bDescDoclist != 0 {
                            -(1 as ::core::ffi::c_int)
                        } else {
                            1 as ::core::ffi::c_int
                        }) * (if iMax > a[i as usize].iDocid {
                            1 as ::core::ffi::c_int
                        } else if iMax == a[i as usize].iDocid {
                            0 as ::core::ffi::c_int
                        } else {
                            -(1 as ::core::ffi::c_int)
                        }) < 0 as ::core::ffi::c_int)
                {
                    iMax = a[i as usize].iDocid;
                    bMaxSet = 1 as ::core::ffi::c_int;
                }
                i += 1;
            }
            i = 0 as ::core::ffi::c_int;
            while i < __p_ref.nToken {
                while rc == crate::src::headers::sqlite3_h::SQLITE_OK
                    && bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && a[i as usize].bIgnore == 0 as ::core::ffi::c_int
                    && (if bDescDoclist != 0 {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        1 as ::core::ffi::c_int
                    }) * (if a[i as usize].iDocid > iMax {
                        1 as ::core::ffi::c_int
                    } else if a[i as usize].iDocid == iMax {
                        0 as ::core::ffi::c_int
                    } else {
                        -(1 as ::core::ffi::c_int)
                    }) < 0 as ::core::ffi::c_int
                {
                    rc = incrPhraseTokenNext(
                        pTab,
                        p,
                        i,
                        (&raw mut a as *mut TokenDoclist).offset(i as isize) as *mut TokenDoclist,
                        &raw mut bEof,
                    );
                    if (if bDescDoclist != 0 {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        1 as ::core::ffi::c_int
                    }) * (if a[i as usize].iDocid > iMax {
                        1 as ::core::ffi::c_int
                    } else if a[i as usize].iDocid == iMax {
                        0 as ::core::ffi::c_int
                    } else {
                        -(1 as ::core::ffi::c_int)
                    }) > 0 as ::core::ffi::c_int
                    {
                        iMax = a[i as usize].iDocid;
                        i = 0 as ::core::ffi::c_int;
                    }
                }
                i += 1;
            }
            if (bEof as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                continue;
            }
            let mut nList: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let nByte: ::core::ffi::c_int =
                a[(__p_ref.nToken - 1 as ::core::ffi::c_int) as usize].nList;
            let aDoclist: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_malloc64(
                (nByte as crate::src::ext::rtree::rtree::I64_0
                    + crate::fts3Int_h::FTS3_BUFFER_PADDING as crate::src::ext::rtree::rtree::I64_0)
                    as crate::src::headers::sqlite3_h::Sqlite3Uint64,
            ) as *mut ::core::ffi::c_char;
            if aDoclist.is_null() {
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
            ::core::ptr::copy_nonoverlapping(
                a[(__p_ref.nToken - 1 as ::core::ffi::c_int) as usize].pList as *const u8,
                aDoclist as *mut u8,
                (nByte + 1 as ::core::ffi::c_int) as usize,
            );
            ::libc::memset(
                aDoclist.offset(nByte as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                crate::fts3Int_h::FTS3_BUFFER_PADDING as crate::__stddef_size_t_h::SizeT,
            );
            i = 0 as ::core::ffi::c_int;
            while i < __p_ref.nToken - 1 as ::core::ffi::c_int {
                if a[i as usize].bIgnore == 0 as ::core::ffi::c_int {
                    let mut pL: *mut ::core::ffi::c_char = a[i as usize].pList;
                    let mut pR: *mut ::core::ffi::c_char = aDoclist;
                    let mut pOut: *mut ::core::ffi::c_char = aDoclist;
                    let nDist: ::core::ffi::c_int =
                        __p_ref.nToken - 1 as ::core::ffi::c_int - i;
                    let res: ::core::ffi::c_int = fts3PoslistPhraseMerge(
                        &raw mut pOut,
                        nDist,
                        0 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        &raw mut pL,
                        &raw mut pR,
                    );
                    if res == 0 as ::core::ffi::c_int {
                        break;
                    }
                    nList = pOut.offset_from(aDoclist) as ::core::ffi::c_long as ::core::ffi::c_int;
                }
                i += 1;
            }
            if i == __p_ref.nToken - 1 as ::core::ffi::c_int {
                let __pDL_ref = unsafe { &mut *pDL };
                __pDL_ref.iDocid = iMax;
                __pDL_ref.pList = aDoclist;
                __pDL_ref.nList = nList;
                __pDL_ref.bFreeList = 1 as ::core::ffi::c_int;
                break;
            } else {
                crate::src::src::malloc::sqlite3_free(aDoclist as *mut ::core::ffi::c_void);
            }
        }
    }
    *pbEof = bEof;
    rc
}

unsafe extern "C" fn fts3EvalPhraseNext(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    p: *mut crate::fts3Int_h::Fts3Phrase,
    pbEof: *mut crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let pDL: *mut crate::fts3Int_h::Fts3Doclist = &raw mut (*p).doclist;
    let pTab: *mut crate::fts3Int_h::Fts3Table =
        (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    if (*p).bIncr != 0 {
        rc = fts3EvalIncrPhraseNext(pCsr, p, pbEof);
    } else if (*pCsr).bDesc as ::core::ffi::c_int != (*pTab).bDescIdx as ::core::ffi::c_int
        && (*pDL).nAll != 0
    {
        let __pDL_ref = unsafe { &mut *pDL };
        sqlite3Fts3DoclistPrev(
            (*pTab).bDescIdx as ::core::ffi::c_int,
            __pDL_ref.aAll,
            __pDL_ref.nAll,
            &raw mut __pDL_ref.pNextDocid,
            &raw mut __pDL_ref.iDocid,
            &raw mut __pDL_ref.nList,
            pbEof,
        );
        __pDL_ref.pList = __pDL_ref.pNextDocid;
    } else {
        fts3EvalDlPhraseNext(pTab, pDL, pbEof);
    }
    rc
}

unsafe extern "C" fn fts3EvalStartReaders(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    pRc: *mut ::core::ffi::c_int,
) {
    if !pExpr.is_null() && crate::src::headers::sqlite3_h::SQLITE_OK == *pRc {
        if (*pExpr).eType == crate::fts3Int_h::FTSQUERY_PHRASE {
            let nToken: ::core::ffi::c_int = (*(*pExpr).pPhrase).nToken;
            if nToken != 0 {
                let mut i: ::core::ffi::c_int;
                i = 0 as ::core::ffi::c_int;
                while i < nToken {
                    if (*(&raw mut (*(*pExpr).pPhrase).aToken
                        as *mut crate::fts3Int_h::Fts3PhraseToken)
                        .offset(i as isize))
                    .pDeferred
                    .is_null()
                    {
                        break;
                    }
                    i += 1;
                }
                (*pExpr).bDeferred =
                    (i == nToken) as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U8_0;
            }
            *pRc = fts3EvalPhraseStart(pCsr, 1 as ::core::ffi::c_int, (*pExpr).pPhrase);
        } else {
            let __pExpr_ref = unsafe { &mut *pExpr };
            fts3EvalStartReaders(pCsr, __pExpr_ref.pLeft, pRc);
            fts3EvalStartReaders(pCsr, __pExpr_ref.pRight, pRc);
            __pExpr_ref.bDeferred = ((*__pExpr_ref.pLeft).bDeferred as ::core::ffi::c_int != 0
                && (*__pExpr_ref.pRight).bDeferred as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int
                as crate::src::ext::rtree::rtree::U8_0;
        }
    }
}

unsafe extern "C" fn fts3EvalTokenCosts(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    mut pRoot: *mut crate::fts3Int_h::Fts3Expr,
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    ppTC: *mut *mut Fts3TokenAndCost,
    ppOr: *mut *mut *mut crate::fts3Int_h::Fts3Expr,
    pRc: *mut ::core::ffi::c_int,
) {
    if *pRc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if (*pExpr).eType == crate::fts3Int_h::FTSQUERY_PHRASE {
            let pPhrase: *mut crate::fts3Int_h::Fts3Phrase = (*pExpr).pPhrase;
            let mut i: ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while *pRc == crate::src::headers::sqlite3_h::SQLITE_OK && i < (*pPhrase).nToken {
                let fresh37 = *ppTC;
                *ppTC = (*ppTC).offset(1);
                let pTC: *mut Fts3TokenAndCost = fresh37;
                let __pTC_ref = unsafe { &mut *pTC };
                __pTC_ref.pPhrase = pPhrase;
                __pTC_ref.iToken = i;
                __pTC_ref.pRoot = pRoot;
                __pTC_ref.pToken = (&raw mut (*pPhrase).aToken
                    as *mut crate::fts3Int_h::Fts3PhraseToken)
                    .offset(i as isize)
                    as *mut crate::fts3Int_h::Fts3PhraseToken;
                __pTC_ref.iCol = (*pPhrase).iColumn;
                *pRc = crate::src::ext::fts3::fts3_write::sqlite3Fts3MsrOvfl(
                    pCsr as *mut crate::fts3Int_h::Fts3Cursor,
                    (*__pTC_ref.pToken).pSegcsr as *mut crate::fts3Int_h::Fts3MultiSegReader,
                    &raw mut __pTC_ref.nOvfl,
                );
                i += 1;
            }
        } else if (*pExpr).eType != crate::fts3Int_h::FTSQUERY_NOT {
            let __pExpr_ref = unsafe { &*pExpr };
            if __pExpr_ref.eType == crate::fts3Int_h::FTSQUERY_OR {
                pRoot = __pExpr_ref.pLeft;
                **ppOr = pRoot;
                *ppOr = (*ppOr).offset(1);
            }
            fts3EvalTokenCosts(pCsr, pRoot, __pExpr_ref.pLeft, ppTC, ppOr, pRc);
            if __pExpr_ref.eType == crate::fts3Int_h::FTSQUERY_OR {
                pRoot = __pExpr_ref.pRight;
                **ppOr = pRoot;
                *ppOr = (*ppOr).offset(1);
            }
            fts3EvalTokenCosts(pCsr, pRoot, __pExpr_ref.pRight, ppTC, ppOr, pRc);
        }
    }
}

unsafe extern "C" fn fts3EvalAverageDocsize(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pnPage: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pCsr).nRowAvg == 0 as ::core::ffi::c_int {
        let __pCsr_ref = unsafe { &mut *pCsr };
        let p: *mut crate::fts3Int_h::Fts3Table =
            __pCsr_ref.base.pVtab as *mut crate::fts3Int_h::Fts3Table;
        let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        let mut nDoc: crate::src::headers::sqlite3_h::Sqlite3Int64 =
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        let mut nByte: crate::src::headers::sqlite3_h::Sqlite3Int64 =
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        let pEnd: *const ::core::ffi::c_char;
        let mut a: *const ::core::ffi::c_char;
        rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3SelectDoctotal(
            p as *mut crate::fts3Int_h::Fts3Table,
            &raw mut pStmt,
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        a = crate::src::src::vdbeapi::sqlite3_column_blob(pStmt, 0 as ::core::ffi::c_int)
            as *const ::core::ffi::c_char;
        if !a.is_null() {
            pEnd = a.offset((crate::src::src::vdbeapi::sqlite3_column_bytes
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int)(pStmt, 0 as ::core::ffi::c_int)
                as isize) as *const ::core::ffi::c_char;
            a = a.offset(sqlite3Fts3GetVarintBounded(a, pEnd, &raw mut nDoc) as isize);
            while a < pEnd {
                a = a.offset(sqlite3Fts3GetVarintBounded(a, pEnd, &raw mut nByte) as isize);
            }
        }
        if nDoc == 0 as crate::src::headers::sqlite3_h::Sqlite3Int64
            || nByte == 0 as crate::src::headers::sqlite3_h::Sqlite3Int64
        {
            crate::src::src::vdbeapi::sqlite3_reset(pStmt);
            return crate::fts3Int_h::FTS_CORRUPT_VTAB;
        }
        __pCsr_ref.nDoc = nDoc;
        __pCsr_ref.nRowAvg = ((nByte / nDoc
            + (*p).nPgsz as crate::src::headers::sqlite3_h::Sqlite3Int64)
            / (*p).nPgsz as crate::src::headers::sqlite3_h::Sqlite3Int64)
            as ::core::ffi::c_int;
        rc = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
    }
    *pnPage = (*pCsr).nRowAvg;
    rc
}

unsafe extern "C" fn fts3EvalSelectDeferred(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pRoot: *mut crate::fts3Int_h::Fts3Expr,
    aTC: *mut Fts3TokenAndCost,
    nTC: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pTab: *mut crate::fts3Int_h::Fts3Table =
        (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut nDocSize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int;
    let mut ii: ::core::ffi::c_int;
    let mut nOvfl: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nMinEst: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nLoad4: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if !(*pTab).zContentTbl.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    ii = 0 as ::core::ffi::c_int;
    while ii < nTC {
        if (*aTC.offset(ii as isize)).pRoot == pRoot {
            nOvfl += (*aTC.offset(ii as isize)).nOvfl;
            nToken += 1;
        }
        ii += 1;
    }
    if nOvfl == 0 as ::core::ffi::c_int || nToken < 2 as ::core::ffi::c_int {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    rc = fts3EvalAverageDocsize(pCsr, &raw mut nDocSize);
    ii = 0 as ::core::ffi::c_int;
    while ii < nToken && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut iTC: ::core::ffi::c_int;
        let mut pTC: *mut Fts3TokenAndCost = ::core::ptr::null_mut::<Fts3TokenAndCost>();
        iTC = 0 as ::core::ffi::c_int;
        while iTC < nTC {
            if !(*aTC.offset(iTC as isize)).pToken.is_null()
                && (*aTC.offset(iTC as isize)).pRoot == pRoot
                && (pTC.is_null() || (*aTC.offset(iTC as isize)).nOvfl < (*pTC).nOvfl)
            {
                pTC = aTC.offset(iTC as isize) as *mut Fts3TokenAndCost;
            }
            iTC += 1;
        }
        if ii != 0
            && (*pTC).nOvfl
                >= (nMinEst + nLoad4 / 4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    / (nLoad4 / 4 as ::core::ffi::c_int)
                    * nDocSize
        {
            let pToken: *mut crate::fts3Int_h::Fts3PhraseToken = (*pTC).pToken;
            rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3DeferToken(
                pCsr as *mut crate::fts3Int_h::Fts3Cursor,
                pToken as *mut crate::fts3Int_h::Fts3PhraseToken,
                (*pTC).iCol,
            );
            fts3SegReaderCursorFree((*pToken).pSegcsr);
            (*pToken).pSegcsr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3MultiSegReader>();
        } else {
            if ii < 12 as ::core::ffi::c_int {
                nLoad4 *= 4 as ::core::ffi::c_int;
            }
            if ii == 0 as ::core::ffi::c_int
                || (*(*pTC).pPhrase).nToken > 1 as ::core::ffi::c_int
                    && ii != nToken - 1 as ::core::ffi::c_int
            {
                let pToken_0: *mut crate::fts3Int_h::Fts3PhraseToken = (*pTC).pToken;
                let mut nList: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut pList: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                rc = fts3TermSelect(pTab, pToken_0, (*pTC).iCol, &raw mut nList, &raw mut pList);
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    rc =
                        fts3EvalPhraseMergeToken(pTab, (*pTC).pPhrase, (*pTC).iToken, pList, nList);
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    
                    let __pPhrase_ref = &*(*pTC).pPhrase;
                    let nCount: ::core::ffi::c_int = fts3DoclistCountDocids(
                        __pPhrase_ref.doclist.aAll,
                        __pPhrase_ref.doclist.nAll,
                    );
                    if ii == 0 as ::core::ffi::c_int || nCount < nMinEst {
                        nMinEst = nCount;
                    }
                }
            }
        }
        (*pTC).pToken = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3PhraseToken>();
        ii += 1;
    }
    rc
}

unsafe extern "C" fn fts3EvalStart(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
) -> ::core::ffi::c_int {
    let __pCsr_ref = unsafe { &*pCsr };
    let pTab = &*(__pCsr_ref.base.pVtab as *mut crate::fts3Int_h::Fts3Table);
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nOr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    fts3EvalAllocateReaders(
        pCsr,
        __pCsr_ref.pExpr,
        &raw mut nToken,
        &raw mut nOr,
        &raw mut rc,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && nToken > 1 as ::core::ffi::c_int
        && pTab.bFts4 as ::core::ffi::c_int != 0
    {
        
        let aTC: *mut Fts3TokenAndCost = crate::src::src::malloc::sqlite3_malloc64(
            (::core::mem::size_of::<Fts3TokenAndCost>() as usize)
                .wrapping_mul(nToken as usize)
                .wrapping_add(
                    (::core::mem::size_of::<*mut crate::fts3Int_h::Fts3Expr>() as usize)
                        .wrapping_mul(nOr as usize)
                        .wrapping_mul(2_usize),
                ) as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut Fts3TokenAndCost;
        if aTC.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            let apOr: *mut *mut crate::fts3Int_h::Fts3Expr = aTC.offset(nToken as isize)
                as *mut Fts3TokenAndCost
                as *mut *mut crate::fts3Int_h::Fts3Expr;
            let mut ii: ::core::ffi::c_int;
            let mut pTC: *mut Fts3TokenAndCost = aTC;
            let mut ppOr: *mut *mut crate::fts3Int_h::Fts3Expr = apOr;
            fts3EvalTokenCosts(
                pCsr,
                ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>(),
                __pCsr_ref.pExpr,
                &raw mut pTC,
                &raw mut ppOr,
                &raw mut rc,
            );
            nToken = pTC.offset_from(aTC) as ::core::ffi::c_long as ::core::ffi::c_int;
            nOr = ppOr.offset_from(apOr) as ::core::ffi::c_long as ::core::ffi::c_int;
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = fts3EvalSelectDeferred(
                    pCsr,
                    ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>(),
                    aTC,
                    nToken,
                );
                ii = 0 as ::core::ffi::c_int;
                while rc == crate::src::headers::sqlite3_h::SQLITE_OK && ii < nOr {
                    rc = fts3EvalSelectDeferred(pCsr, *apOr.offset(ii as isize), aTC, nToken);
                    ii += 1;
                }
            }
            crate::src::src::malloc::sqlite3_free(aTC as *mut ::core::ffi::c_void);
        }
    }
    fts3EvalStartReaders(pCsr, __pCsr_ref.pExpr, &raw mut rc);
    rc
}

unsafe extern "C" fn fts3EvalInvalidatePoslist(pPhrase: *mut crate::fts3Int_h::Fts3Phrase) {
    let __pPhrase_ref = unsafe { &mut *pPhrase };
    if __pPhrase_ref.doclist.bFreeList != 0 {
        crate::src::src::malloc::sqlite3_free(
            __pPhrase_ref.doclist.pList as *mut ::core::ffi::c_void,
        );
    }
    __pPhrase_ref.doclist.pList = ::core::ptr::null_mut::<::core::ffi::c_char>();
    __pPhrase_ref.doclist.nList = 0 as ::core::ffi::c_int;
    __pPhrase_ref.doclist.bFreeList = 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn fts3EvalNearTrim(
    nNear: ::core::ffi::c_int,
    aTmp: *mut ::core::ffi::c_char,
    paPoslist: *mut *mut ::core::ffi::c_char,
    pnToken: *mut ::core::ffi::c_int,
    pPhrase: *mut crate::fts3Int_h::Fts3Phrase,
) -> ::core::ffi::c_int {
    let nParam1: ::core::ffi::c_int = nNear + (*pPhrase).nToken;
    let nParam2: ::core::ffi::c_int = nNear + *pnToken;
    let nNew: ::core::ffi::c_int;
    let mut p2: *mut ::core::ffi::c_char;
    let mut pOut: *mut ::core::ffi::c_char;
    
    pOut = (*pPhrase).doclist.pList;
    p2 = pOut;
    let res: ::core::ffi::c_int = fts3PoslistNearMerge(
        &raw mut pOut,
        aTmp,
        nParam1,
        nParam2,
        paPoslist,
        &raw mut p2,
    );
    if res != 0 {
        let __pPhrase_ref = unsafe { &mut *pPhrase };
        nNew = pOut.offset_from(__pPhrase_ref.doclist.pList) as ::core::ffi::c_long
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
        if nNew >= 0 as ::core::ffi::c_int && nNew <= __pPhrase_ref.doclist.nList {
            ::libc::memset(
                __pPhrase_ref.doclist.pList.offset(nNew as isize) as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (__pPhrase_ref.doclist.nList - nNew) as crate::__stddef_size_t_h::SizeT,
            );
            __pPhrase_ref.doclist.nList = nNew;
        }
        *paPoslist = __pPhrase_ref.doclist.pList;
        *pnToken = __pPhrase_ref.nToken;
    }
    res
}

unsafe extern "C" fn fts3EvalNextRow(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    pRc: *mut ::core::ffi::c_int,
) {
    if *pRc == crate::src::headers::sqlite3_h::SQLITE_OK
        && (*pExpr).bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        let bDescDoclist: ::core::ffi::c_int = (*pCsr).bDesc as ::core::ffi::c_int;
        (*pExpr).bStart = 1 as crate::src::ext::rtree::rtree::U8_0;
        match (*pExpr).eType {
            crate::fts3Int_h::FTSQUERY_NEAR | crate::fts3Int_h::FTSQUERY_AND => {
                let pLeft: *mut crate::fts3Int_h::Fts3Expr = (*pExpr).pLeft;
                let pRight: *mut crate::fts3Int_h::Fts3Expr = (*pExpr).pRight;
                if (*pLeft).bDeferred != 0 {
                    fts3EvalNextRow(pCsr, pRight, pRc);
                    (*pExpr).iDocid = (*pRight).iDocid;
                    (*pExpr).bEof = (*pRight).bEof;
                } else if (*pRight).bDeferred != 0 {
                    fts3EvalNextRow(pCsr, pLeft, pRc);
                    (*pExpr).iDocid = (*pLeft).iDocid;
                    (*pExpr).bEof = (*pLeft).bEof;
                } else {
                    fts3EvalNextRow(pCsr, pLeft, pRc);
                    fts3EvalNextRow(pCsr, pRight, pRc);
                    loop {
                        if !((*pLeft).bEof == 0
                            && (*pRight).bEof == 0
                            && *pRc == crate::src::headers::sqlite3_h::SQLITE_OK)
                        {
                            break;
                        }
                        let iDiff: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                            ((if bDescDoclist != 0 {
                                -(1 as ::core::ffi::c_int)
                            } else {
                                1 as ::core::ffi::c_int
                            }) * (if (*pLeft).iDocid > (*pRight).iDocid {
                                1 as ::core::ffi::c_int
                            } else if (*pLeft).iDocid == (*pRight).iDocid {
                                0 as ::core::ffi::c_int
                            } else {
                                -(1 as ::core::ffi::c_int)
                            }))
                                as crate::src::headers::sqlite3_h::Sqlite3Int64;
                        if iDiff == 0 as crate::src::headers::sqlite3_h::Sqlite3Int64 {
                            break;
                        }
                        if iDiff < 0 as crate::src::headers::sqlite3_h::Sqlite3Int64 {
                            fts3EvalNextRow(pCsr, pLeft, pRc);
                        } else {
                            fts3EvalNextRow(pCsr, pRight, pRc);
                        }
                    }
                    let __pExpr_ref = unsafe { &mut *pExpr };
                    __pExpr_ref.iDocid = (*pLeft).iDocid;
                    __pExpr_ref.bEof = ((*pLeft).bEof as ::core::ffi::c_int != 0
                        || (*pRight).bEof as ::core::ffi::c_int != 0)
                        as ::core::ffi::c_int
                        as crate::src::ext::rtree::rtree::U8_0;
                    if __pExpr_ref.eType == crate::fts3Int_h::FTSQUERY_NEAR
                        && __pExpr_ref.bEof as ::core::ffi::c_int != 0
                    {
                        if !(*(*pRight).pPhrase).doclist.aAll.is_null() {
                            let pDl: *mut crate::fts3Int_h::Fts3Doclist =
                                &raw mut (*(*pRight).pPhrase).doclist;
                            loop {
                                if !(*pRc == crate::src::headers::sqlite3_h::SQLITE_OK
                                    && (*pRight).bEof as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int)
                                {
                                    break;
                                }
                                ::libc::memset(
                                    (*pDl).pList as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    (*pDl).nList as crate::__stddef_size_t_h::SizeT,
                                );
                                fts3EvalNextRow(pCsr, pRight, pRc);
                            }
                        }
                        if !(*pLeft).pPhrase.is_null()
                            && !(*(*pLeft).pPhrase).doclist.aAll.is_null()
                        {
                            let pDl_0: *mut crate::fts3Int_h::Fts3Doclist =
                                &raw mut (*(*pLeft).pPhrase).doclist;
                            loop {
                                if !(*pRc == crate::src::headers::sqlite3_h::SQLITE_OK
                                    && (*pLeft).bEof as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int)
                                {
                                    break;
                                }
                                ::libc::memset(
                                    (*pDl_0).pList as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    (*pDl_0).nList as crate::__stddef_size_t_h::SizeT,
                                );
                                fts3EvalNextRow(pCsr, pLeft, pRc);
                            }
                        }
                        (*pLeft).bEof = 1 as crate::src::ext::rtree::rtree::U8_0;
                        (*pRight).bEof = (*pLeft).bEof;
                    }
                }
            }
            crate::fts3Int_h::FTSQUERY_OR => {
                let __pExpr_ref = unsafe { &mut *pExpr };
                let pLeft_0: *mut crate::fts3Int_h::Fts3Expr = __pExpr_ref.pLeft;
                let pRight_0: *mut crate::fts3Int_h::Fts3Expr = __pExpr_ref.pRight;
                let mut iCmp: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                    ((if bDescDoclist != 0 {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        1 as ::core::ffi::c_int
                    }) * (if (*pLeft_0).iDocid > (*pRight_0).iDocid {
                        1 as ::core::ffi::c_int
                    } else if (*pLeft_0).iDocid == (*pRight_0).iDocid {
                        0 as ::core::ffi::c_int
                    } else {
                        -(1 as ::core::ffi::c_int)
                    })) as crate::src::headers::sqlite3_h::Sqlite3Int64;
                if (*pRight_0).bEof as ::core::ffi::c_int != 0
                    || (*pLeft_0).bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && iCmp < 0 as crate::src::headers::sqlite3_h::Sqlite3Int64
                {
                    fts3EvalNextRow(pCsr, pLeft_0, pRc);
                } else if (*pLeft_0).bEof as ::core::ffi::c_int != 0
                    || iCmp > 0 as crate::src::headers::sqlite3_h::Sqlite3Int64
                {
                    fts3EvalNextRow(pCsr, pRight_0, pRc);
                } else {
                    fts3EvalNextRow(pCsr, pLeft_0, pRc);
                    fts3EvalNextRow(pCsr, pRight_0, pRc);
                }
                __pExpr_ref.bEof = ((*pLeft_0).bEof as ::core::ffi::c_int != 0
                    && (*pRight_0).bEof as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int
                    as crate::src::ext::rtree::rtree::U8_0;
                iCmp = ((if bDescDoclist != 0 {
                    -(1 as ::core::ffi::c_int)
                } else {
                    1 as ::core::ffi::c_int
                }) * (if (*pLeft_0).iDocid > (*pRight_0).iDocid {
                    1 as ::core::ffi::c_int
                } else if (*pLeft_0).iDocid == (*pRight_0).iDocid {
                    0 as ::core::ffi::c_int
                } else {
                    -(1 as ::core::ffi::c_int)
                })) as crate::src::headers::sqlite3_h::Sqlite3Int64;
                if (*pRight_0).bEof as ::core::ffi::c_int != 0
                    || (*pLeft_0).bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && iCmp < 0 as crate::src::headers::sqlite3_h::Sqlite3Int64
                {
                    __pExpr_ref.iDocid = (*pLeft_0).iDocid;
                } else {
                    __pExpr_ref.iDocid = (*pRight_0).iDocid;
                }
            }
            crate::fts3Int_h::FTSQUERY_NOT => {
                let __pExpr_ref = unsafe { &mut *pExpr };
                let pLeft_1: *mut crate::fts3Int_h::Fts3Expr = __pExpr_ref.pLeft;
                let pRight_1: *mut crate::fts3Int_h::Fts3Expr = __pExpr_ref.pRight;
                if (*pRight_1).bStart as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    fts3EvalNextRow(pCsr, pRight_1, pRc);
                }
                fts3EvalNextRow(pCsr, pLeft_1, pRc);
                if (*pLeft_1).bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    loop {
                        if !(*pRc == 0
                            && (*pRight_1).bEof == 0
                            && (if bDescDoclist != 0 {
                                -(1 as ::core::ffi::c_int)
                            } else {
                                1 as ::core::ffi::c_int
                            }) * (if (*pLeft_1).iDocid > (*pRight_1).iDocid {
                                1 as ::core::ffi::c_int
                            } else if (*pLeft_1).iDocid == (*pRight_1).iDocid {
                                0 as ::core::ffi::c_int
                            } else {
                                -(1 as ::core::ffi::c_int)
                            }) > 0 as ::core::ffi::c_int)
                        {
                            break;
                        }
                        fts3EvalNextRow(pCsr, pRight_1, pRc);
                    }
                }
                __pExpr_ref.iDocid = (*pLeft_1).iDocid;
                __pExpr_ref.bEof = (*pLeft_1).bEof;
            }
            _ => {
                let __pExpr_ref = unsafe { &mut *pExpr };
                let pPhrase: *mut crate::fts3Int_h::Fts3Phrase = __pExpr_ref.pPhrase;
                fts3EvalInvalidatePoslist(pPhrase);
                *pRc = fts3EvalPhraseNext(pCsr, pPhrase, &raw mut __pExpr_ref.bEof);
                __pExpr_ref.iDocid = (*pPhrase).doclist.iDocid;
            }
        }
    }
}

unsafe extern "C" fn fts3EvalNearTest(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    pRc: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let __pExpr_ref = unsafe { &mut *pExpr };
    if *pRc == crate::src::headers::sqlite3_h::SQLITE_OK
        && __pExpr_ref.eType == crate::fts3Int_h::FTSQUERY_NEAR
        && (__pExpr_ref.pParent.is_null()
            || (*__pExpr_ref.pParent).eType != crate::fts3Int_h::FTSQUERY_NEAR)
    {
        let mut p: *mut crate::fts3Int_h::Fts3Expr;
        let mut nTmp: crate::src::headers::sqlite3_h::Sqlite3Int64 =
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        
        p = pExpr;
        while !(*p).pLeft.is_null() {
            nTmp += (*(*(*p).pRight).pPhrase).doclist.nList
                as crate::src::headers::sqlite3_h::Sqlite3Int64;
            p = (*p).pLeft;
        }
        nTmp += (*(*p).pPhrase).doclist.nList as crate::src::headers::sqlite3_h::Sqlite3Int64;
        let aTmp: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_malloc64(
            (nTmp * 2 as crate::src::headers::sqlite3_h::Sqlite3Int64
                + crate::fts3Int_h::FTS3_VARINT_MAX
                    as crate::src::headers::sqlite3_h::Sqlite3Int64)
                as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut ::core::ffi::c_char;
        if aTmp.is_null() {
            *pRc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            res = 0 as ::core::ffi::c_int;
        } else {
            let mut aPoslist: *mut ::core::ffi::c_char = (*(*p).pPhrase).doclist.pList;
            let mut nToken: ::core::ffi::c_int = (*(*p).pPhrase).nToken;
            p = (*p).pParent;
            while res != 0 && !p.is_null() && (*p).eType == crate::fts3Int_h::FTSQUERY_NEAR {
                let pPhrase: *mut crate::fts3Int_h::Fts3Phrase = (*(*p).pRight).pPhrase;
                let nNear: ::core::ffi::c_int = (*p).nNear;
                res = fts3EvalNearTrim(nNear, aTmp, &raw mut aPoslist, &raw mut nToken, pPhrase);
                p = (*p).pParent;
            }
            aPoslist = (*(*__pExpr_ref.pRight).pPhrase).doclist.pList;
            nToken = (*(*__pExpr_ref.pRight).pPhrase).nToken;
            p = __pExpr_ref.pLeft;
            while !p.is_null() && res != 0 {
                
                
                let nNear_0: ::core::ffi::c_int = (*(*p).pParent).nNear;
                let pPhrase_0: *mut crate::fts3Int_h::Fts3Phrase = if (*p).eType == crate::fts3Int_h::FTSQUERY_NEAR {
                    (*(*p).pRight).pPhrase
                } else {
                    (*p).pPhrase
                };
                res =
                    fts3EvalNearTrim(nNear_0, aTmp, &raw mut aPoslist, &raw mut nToken, pPhrase_0);
                p = (*p).pLeft;
            }
        }
        crate::src::src::malloc::sqlite3_free(aTmp as *mut ::core::ffi::c_void);
    }
    res
}

unsafe extern "C" fn fts3EvalTestExpr(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    pRc: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bHit: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if *pRc == crate::src::headers::sqlite3_h::SQLITE_OK {
        match (*pExpr).eType {
            crate::fts3Int_h::FTSQUERY_NEAR | crate::fts3Int_h::FTSQUERY_AND => {
                let __pExpr_ref = unsafe { &mut *pExpr };
                bHit = (fts3EvalTestExpr(pCsr, __pExpr_ref.pLeft, pRc) != 0
                    && fts3EvalTestExpr(pCsr, __pExpr_ref.pRight, pRc) != 0
                    && fts3EvalNearTest(pExpr, pRc) != 0)
                    as ::core::ffi::c_int;
                if bHit == 0 as ::core::ffi::c_int
                    && __pExpr_ref.eType == crate::fts3Int_h::FTSQUERY_NEAR
                    && (__pExpr_ref.pParent.is_null()
                        || (*__pExpr_ref.pParent).eType != crate::fts3Int_h::FTSQUERY_NEAR)
                {
                    let mut p: *mut crate::fts3Int_h::Fts3Expr;
                    p = pExpr;
                    while (*p).pPhrase.is_null() {
                        if (*(*p).pRight).iDocid == (*pCsr).iPrevId {
                            fts3EvalInvalidatePoslist((*(*p).pRight).pPhrase);
                        }
                        p = (*p).pLeft;
                    }
                    if (*p).iDocid == (*pCsr).iPrevId {
                        fts3EvalInvalidatePoslist((*p).pPhrase);
                    }
                }
            }
            crate::fts3Int_h::FTSQUERY_OR => {
                let bHit1: ::core::ffi::c_int = fts3EvalTestExpr(pCsr, (*pExpr).pLeft, pRc);
                let bHit2: ::core::ffi::c_int = fts3EvalTestExpr(pCsr, (*pExpr).pRight, pRc);
                bHit = (bHit1 != 0 || bHit2 != 0) as ::core::ffi::c_int;
            }
            crate::fts3Int_h::FTSQUERY_NOT => {
                bHit = (fts3EvalTestExpr(pCsr, (*pExpr).pLeft, pRc) != 0
                    && fts3EvalTestExpr(pCsr, (*pExpr).pRight, pRc) == 0)
                    as ::core::ffi::c_int;
            }
            _ => {
                let __pExpr_ref = unsafe { &mut *pExpr };
                if !(*pCsr).pDeferred.is_null()
                    && (__pExpr_ref.bDeferred as ::core::ffi::c_int != 0
                        || __pExpr_ref.iDocid == (*pCsr).iPrevId
                            && !(*__pExpr_ref.pPhrase).doclist.pList.is_null())
                {
                    let pPhrase: *mut crate::fts3Int_h::Fts3Phrase = __pExpr_ref.pPhrase;
                    if __pExpr_ref.bDeferred != 0 {
                        fts3EvalInvalidatePoslist(pPhrase);
                    }
                    *pRc = fts3EvalDeferredPhrase(pCsr, pPhrase);
                    bHit = !(*pPhrase).doclist.pList.is_null()
                        as ::core::ffi::c_int;
                    __pExpr_ref.iDocid = (*pCsr).iPrevId;
                } else {
                    bHit = (__pExpr_ref.bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && __pExpr_ref.iDocid == (*pCsr).iPrevId
                        && (*__pExpr_ref.pPhrase).doclist.nList > 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                }
            }
        }
    }
    bHit
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3EvalTestDeferred(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pRc: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = *pRc;
    let mut bMiss: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if !(*pCsr).pDeferred.is_null() {
            rc = fts3CursorSeek(
                ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_context>(),
                pCsr,
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3CacheDeferredDoclists(
                    pCsr as *mut crate::fts3Int_h::Fts3Cursor,
                );
            }
        }
        bMiss = (0 as ::core::ffi::c_int == fts3EvalTestExpr(pCsr, (*pCsr).pExpr, &raw mut rc))
            as ::core::ffi::c_int;
        crate::src::ext::fts3::fts3_write::sqlite3Fts3FreeDeferredDoclists(
            pCsr as *mut crate::fts3Int_h::Fts3Cursor,
        );
        *pRc = rc;
    }
    (rc == crate::src::headers::sqlite3_h::SQLITE_OK && bMiss != 0) as ::core::ffi::c_int
}

unsafe extern "C" fn fts3EvalNext(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pCsr_ref = unsafe { &mut *pCsr };
    let pExpr: *mut crate::fts3Int_h::Fts3Expr = __pCsr_ref.pExpr;
    if pExpr.is_null() {
        __pCsr_ref.isEof = 1 as crate::src::ext::rtree::rtree::U8_0;
    } else {
        loop {
            if __pCsr_ref.isRequireSeek as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                crate::src::src::vdbeapi::sqlite3_reset(__pCsr_ref.pStmt);
            }
            fts3EvalNextRow(pCsr, pExpr, &raw mut rc);
            __pCsr_ref.isEof = (*pExpr).bEof;
            __pCsr_ref.isRequireSeek = 1 as crate::src::ext::rtree::rtree::U8_0;
            __pCsr_ref.isMatchinfoNeeded = 1 as ::core::ffi::c_int;
            __pCsr_ref.iPrevId = (*pExpr).iDocid;
            if !(__pCsr_ref.isEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && sqlite3Fts3EvalTestDeferred(pCsr, &raw mut rc) != 0)
            {
                break;
            }
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && (__pCsr_ref.bDesc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && __pCsr_ref.iPrevId > __pCsr_ref.iMaxDocid
            || __pCsr_ref.bDesc as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && __pCsr_ref.iPrevId < __pCsr_ref.iMinDocid)
    {
        __pCsr_ref.isEof = 1 as crate::src::ext::rtree::rtree::U8_0;
    }
    rc
}

unsafe extern "C" fn fts3EvalRestart(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    pRc: *mut ::core::ffi::c_int,
) {
    if !pExpr.is_null() && *pRc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let __pExpr_ref = unsafe { &mut *pExpr };
        let pPhrase: *mut crate::fts3Int_h::Fts3Phrase = __pExpr_ref.pPhrase;
        if !pPhrase.is_null() {
            fts3EvalInvalidatePoslist(pPhrase);
            let __pPhrase_ref = unsafe { &mut *pPhrase };
            if __pPhrase_ref.bIncr != 0 {
                let mut i: ::core::ffi::c_int;
                i = 0 as ::core::ffi::c_int;
                while i < __pPhrase_ref.nToken {
                    let pToken: *mut crate::fts3Int_h::Fts3PhraseToken =
                        (&raw mut __pPhrase_ref.aToken as *mut crate::fts3Int_h::Fts3PhraseToken)
                            .offset(i as isize)
                            as *mut crate::fts3Int_h::Fts3PhraseToken;
                    if !(*pToken).pSegcsr.is_null() {
                        crate::src::ext::fts3::fts3_write::sqlite3Fts3MsrIncrRestart(
                            (*pToken).pSegcsr as *mut crate::fts3Int_h::Fts3MultiSegReader,
                        );
                    }
                    i += 1;
                }
                *pRc = fts3EvalPhraseStart(pCsr, 0 as ::core::ffi::c_int, pPhrase);
            }
            __pPhrase_ref.doclist.pNextDocid = ::core::ptr::null_mut::<::core::ffi::c_char>();
            __pPhrase_ref.doclist.iDocid = 0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            __pPhrase_ref.pOrPoslist = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        __pExpr_ref.iDocid = 0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        __pExpr_ref.bEof = 0 as crate::src::ext::rtree::rtree::U8_0;
        __pExpr_ref.bStart = 0 as crate::src::ext::rtree::rtree::U8_0;
        fts3EvalRestart(pCsr, __pExpr_ref.pLeft, pRc);
        fts3EvalRestart(pCsr, __pExpr_ref.pRight, pRc);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3MsrCancel(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pExpr).bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let iDocid: crate::src::ext::rtree::rtree::I64_0 =
            (*pExpr).iDocid as crate::src::ext::rtree::rtree::I64_0;
        fts3EvalRestart(pCsr, pExpr, &raw mut rc);
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && (*pExpr).iDocid != iDocid {
            fts3EvalNextRow(pCsr, pExpr, &raw mut rc);
            if (*pExpr).bEof != 0 {
                rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
            }
        }
    }
    rc
}

unsafe extern "C" fn fts3EvalUpdateCounts(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    nCol: ::core::ffi::c_int,
) {
    if !pExpr.is_null() {
        let __pExpr_ref = unsafe { &mut *pExpr };
        let pPhrase: *mut crate::fts3Int_h::Fts3Phrase = __pExpr_ref.pPhrase;
        if !pPhrase.is_null() && !(*pPhrase).doclist.pList.is_null() {
            let mut iCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut p: *mut ::core::ffi::c_char = (*pPhrase).doclist.pList;
            loop {
                let mut c: crate::src::ext::rtree::rtree::U8_0 =
                    0 as crate::src::ext::rtree::rtree::U8_0;
                let mut iCnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while 0xfe as ::core::ffi::c_int
                    & (*p as ::core::ffi::c_int | c as ::core::ffi::c_int)
                    != 0
                {
                    if c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        iCnt += 1;
                    }
                    let fresh24 = p;
                    p = p.offset(1);
                    c = (*fresh24 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int)
                        as crate::src::ext::rtree::rtree::U8_0;
                }
                let fresh25 = &mut *(*pExpr)
                    .aMI
                    .offset((iCol * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
                *fresh25 = (*fresh25).wrapping_add(iCnt as crate::src::ext::rtree::rtree::U32_0);
                let fresh26 = &mut *(*pExpr)
                    .aMI
                    .offset((iCol * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize);
                *fresh26 = (*fresh26).wrapping_add(
                    (iCnt > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as crate::src::ext::rtree::rtree::U32_0,
                );
                if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    break;
                }
                p = p.offset(1);
                p = p.offset(
                    (if *(p as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        != 0
                    {
                        sqlite3Fts3GetVarint32(p, &raw mut iCol)
                    } else {
                        iCol =
                            *(p as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
                        1 as ::core::ffi::c_int
                    }) as isize,
                );
                if (iCol >= nCol) {
                    break;
                }
            }
        }
        fts3EvalUpdateCounts(__pExpr_ref.pLeft, nCol);
        fts3EvalUpdateCounts(__pExpr_ref.pRight, nCol);
    }
}

unsafe extern "C" fn fts3AllocateMSI(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    mut _iPhrase: ::core::ffi::c_int,
    pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let pTab: *mut crate::fts3Int_h::Fts3Table = pCtx as *mut crate::fts3Int_h::Fts3Table;
    if (*pExpr).aMI.is_null() {
        (*pExpr).aMI = crate::src::src::malloc::sqlite3_malloc64(
            (((*pTab).nColumn * 3 as ::core::ffi::c_int) as usize).wrapping_mul(
                ::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>() as usize,
            ) as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut crate::src::ext::rtree::rtree::U32_0;
        if (*pExpr).aMI.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    }
    ::libc::memset(
        (*pExpr).aMI as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (((*pTab).nColumn * 3 as ::core::ffi::c_int) as crate::__stddef_size_t_h::SizeT)
            .wrapping_mul(
                ::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>()
                    as crate::__stddef_size_t_h::SizeT,
            ),
    );
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3EvalGatherStats(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pExpr).aMI.is_null() {
        let __pCsr_ref = unsafe { &mut *pCsr };
        let pTab: *mut crate::fts3Int_h::Fts3Table =
            __pCsr_ref.base.pVtab as *mut crate::fts3Int_h::Fts3Table;
        let mut pRoot: *mut crate::fts3Int_h::Fts3Expr;
        let iPrevId: crate::src::headers::sqlite3_h::Sqlite3Int64 = __pCsr_ref.iPrevId;
        
        
        pRoot = pExpr;
        while !(*pRoot).pParent.is_null()
            && ((*(*pRoot).pParent).eType == crate::fts3Int_h::FTSQUERY_NEAR
                || (*pRoot).bDeferred as ::core::ffi::c_int != 0)
        {
            pRoot = (*pRoot).pParent;
        }
        let iDocid: crate::src::headers::sqlite3_h::Sqlite3Int64 = (*pRoot).iDocid;
        let bEof: crate::src::ext::rtree::rtree::U8_0 = (*pRoot).bEof;
        rc = crate::src::ext::fts3::fts3_snippet::sqlite3Fts3ExprIterate(
            pRoot as *mut crate::fts3Int_h::Fts3Expr,
            Some(
                fts3AllocateMSI
                    as unsafe extern "C" fn(
                        *mut crate::fts3Int_h::Fts3Expr,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            pTab as *mut ::core::ffi::c_void,
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        fts3EvalRestart(pCsr, pRoot, &raw mut rc);
        while __pCsr_ref.isEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && rc == crate::src::headers::sqlite3_h::SQLITE_OK
        {
            loop {
                if __pCsr_ref.isRequireSeek as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    crate::src::src::vdbeapi::sqlite3_reset(__pCsr_ref.pStmt);
                }
                fts3EvalNextRow(pCsr, pRoot, &raw mut rc);
                let __pRoot_ref = unsafe { &*pRoot };
                __pCsr_ref.isEof = __pRoot_ref.bEof;
                __pCsr_ref.isRequireSeek = 1 as crate::src::ext::rtree::rtree::U8_0;
                __pCsr_ref.isMatchinfoNeeded = 1 as ::core::ffi::c_int;
                __pCsr_ref.iPrevId = __pRoot_ref.iDocid;
                if !(__pCsr_ref.isEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && __pRoot_ref.eType == crate::fts3Int_h::FTSQUERY_NEAR
                    && sqlite3Fts3EvalTestDeferred(pCsr, &raw mut rc) != 0)
                {
                    break;
                }
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                && __pCsr_ref.isEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                fts3EvalUpdateCounts(pRoot, (*pTab).nColumn);
            }
        }
        __pCsr_ref.isEof = 0 as crate::src::ext::rtree::rtree::U8_0;
        __pCsr_ref.iPrevId = iPrevId;
        if bEof != 0 {
            (*pRoot).bEof = bEof;
        } else {
            fts3EvalRestart(pCsr, pRoot, &raw mut rc);
            loop {
                fts3EvalNextRow(pCsr, pRoot, &raw mut rc);
                if (*pRoot).bEof != 0 {
                    rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
                }
                if !((*pRoot).iDocid != iDocid && rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
                    break;
                }
            }
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3EvalPhraseStats(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    aiOut: *mut crate::src::ext::rtree::rtree::U32_0,
) -> ::core::ffi::c_int {
    let pTab: *mut crate::fts3Int_h::Fts3Table =
        (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut iCol: ::core::ffi::c_int;
    if (*pExpr).bDeferred as ::core::ffi::c_int != 0
        && (*(*pExpr).pParent).eType != crate::fts3Int_h::FTSQUERY_NEAR
    {
        iCol = 0 as ::core::ffi::c_int;
        while iCol < (*pTab).nColumn {
            *aiOut.offset((iCol * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                (*pCsr).nDoc as crate::src::ext::rtree::rtree::U32_0;
            *aiOut.offset((iCol * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                (*pCsr).nDoc as crate::src::ext::rtree::rtree::U32_0;
            iCol += 1;
        }
    } else {
        rc = fts3EvalGatherStats(pCsr, pExpr);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            iCol = 0 as ::core::ffi::c_int;
            while iCol < (*pTab).nColumn {
                *aiOut
                    .offset((iCol * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                    *(*pExpr).aMI.offset(
                        (iCol * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                    );
                *aiOut
                    .offset((iCol * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                    *(*pExpr).aMI.offset(
                        (iCol * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                    );
                iCol += 1;
            }
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3EvalPhrasePoslist(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    iCol: ::core::ffi::c_int,
    ppOut: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let __pExpr_ref = unsafe { &*pExpr };
    let pPhrase: *mut crate::fts3Int_h::Fts3Phrase = __pExpr_ref.pPhrase;
    let pTab: *mut crate::fts3Int_h::Fts3Table =
        (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut pIter: *mut ::core::ffi::c_char;
    let mut iThis: ::core::ffi::c_int = 0;
    let mut iDocid: crate::src::headers::sqlite3_h::Sqlite3Int64;
    *ppOut = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let __pPhrase_ref = unsafe { &*pPhrase };
    if __pPhrase_ref.iColumn < (*pTab).nColumn && __pPhrase_ref.iColumn != iCol {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    iDocid = __pExpr_ref.iDocid;
    pIter = __pPhrase_ref.doclist.pList;
    if iDocid != (*pCsr).iPrevId || __pExpr_ref.bEof as ::core::ffi::c_int != 0 {
        let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
        let bDescDoclist: ::core::ffi::c_int = (*pTab).bDescIdx as ::core::ffi::c_int;
        let mut bOr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut bTreeEof: crate::src::ext::rtree::rtree::U8_0 =
            0 as crate::src::ext::rtree::rtree::U8_0;
        let mut p: *mut crate::fts3Int_h::Fts3Expr;
        let mut pNear: *mut crate::fts3Int_h::Fts3Expr;
        let mut pRun: *mut crate::fts3Int_h::Fts3Expr;
        let mut bMatch: ::core::ffi::c_int;
        pNear = pExpr;
        p = __pExpr_ref.pParent;
        while !p.is_null() {
            if (*p).eType == crate::fts3Int_h::FTSQUERY_OR {
                bOr = 1 as ::core::ffi::c_int;
            }
            if (*p).eType == crate::fts3Int_h::FTSQUERY_NEAR {
                pNear = p;
            }
            if (*p).bEof != 0 {
                bTreeEof = 1 as crate::src::ext::rtree::rtree::U8_0;
            }
            p = (*p).pParent;
        }
        if bOr == 0 as ::core::ffi::c_int {
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        pRun = pNear;
        while (*pRun).bDeferred != 0 {
            pRun = (*pRun).pParent;
        }
        if __pPhrase_ref.bIncr != 0 {
            let bEofSave: ::core::ffi::c_int = (*pRun).bEof as ::core::ffi::c_int;
            fts3EvalRestart(pCsr, pRun, &raw mut rc);
            while rc == crate::src::headers::sqlite3_h::SQLITE_OK && (*pRun).bEof == 0 {
                fts3EvalNextRow(pCsr, pRun, &raw mut rc);
                if bEofSave == 0 as ::core::ffi::c_int && (*pRun).iDocid == iDocid {
                    break;
                }
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                && (*pRun).bEof as ::core::ffi::c_int != bEofSave
            {
                rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
            }
        }
        if bTreeEof != 0 {
            while rc == crate::src::headers::sqlite3_h::SQLITE_OK && (*pRun).bEof == 0 {
                fts3EvalNextRow(pCsr, pRun, &raw mut rc);
            }
        }
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        bMatch = 1 as ::core::ffi::c_int;
        p = pNear;
        while !p.is_null() {
            let mut bEof: crate::src::ext::rtree::rtree::U8_0;
            let mut pTest: *mut crate::fts3Int_h::Fts3Expr = p;
            
            if (*pTest).eType == crate::fts3Int_h::FTSQUERY_NEAR {
                pTest = (*pTest).pRight;
            }
            let pPh: *mut crate::fts3Int_h::Fts3Phrase = (*pTest).pPhrase;
            pIter = (*pPh).pOrPoslist;
            iDocid = (*pPh).iOrDocid as crate::src::headers::sqlite3_h::Sqlite3Int64;
            if (*pCsr).bDesc as ::core::ffi::c_int == bDescDoclist {
                let __pPh_ref = unsafe { &mut *pPh };
                bEof = (__pPh_ref.doclist.nAll == 0
                    || pIter
                        >= __pPh_ref
                            .doclist
                            .aAll
                            .offset(__pPh_ref.doclist.nAll as isize))
                    as ::core::ffi::c_int
                    as crate::src::ext::rtree::rtree::U8_0;
                while (pIter.is_null()
                    || (if bDescDoclist != 0 {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        1 as ::core::ffi::c_int
                    }) * (if iDocid > (*pCsr).iPrevId {
                        1 as ::core::ffi::c_int
                    } else if iDocid == (*pCsr).iPrevId {
                        0 as ::core::ffi::c_int
                    } else {
                        -(1 as ::core::ffi::c_int)
                    }) < 0 as ::core::ffi::c_int)
                    && bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    sqlite3Fts3DoclistNext(
                        bDescDoclist,
                        __pPh_ref.doclist.aAll,
                        __pPh_ref.doclist.nAll,
                        &raw mut pIter,
                        &raw mut iDocid,
                        &raw mut bEof,
                    );
                }
            } else {
                bEof = ((*pPh).doclist.nAll == 0
                    || !pIter.is_null() && pIter <= (*pPh).doclist.aAll)
                    as ::core::ffi::c_int
                    as crate::src::ext::rtree::rtree::U8_0;
                while (pIter.is_null()
                    || (if bDescDoclist != 0 {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        1 as ::core::ffi::c_int
                    }) * (if iDocid > (*pCsr).iPrevId {
                        1 as ::core::ffi::c_int
                    } else if iDocid == (*pCsr).iPrevId {
                        0 as ::core::ffi::c_int
                    } else {
                        -(1 as ::core::ffi::c_int)
                    }) > 0 as ::core::ffi::c_int)
                    && bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    let mut dummy: ::core::ffi::c_int = 0;
                    sqlite3Fts3DoclistPrev(
                        bDescDoclist,
                        (*pPh).doclist.aAll,
                        (*pPh).doclist.nAll,
                        &raw mut pIter,
                        &raw mut iDocid,
                        &raw mut dummy,
                        &raw mut bEof,
                    );
                }
            }
            (*pPh).pOrPoslist = pIter;
            (*pPh).iOrDocid = iDocid as crate::src::ext::rtree::rtree::I64_0;
            if bEof as ::core::ffi::c_int != 0 || iDocid != (*pCsr).iPrevId {
                bMatch = 0 as ::core::ffi::c_int;
            }
            p = (*p).pLeft;
        }
        if bMatch != 0 {
            pIter = __pPhrase_ref.pOrPoslist;
        } else {
            pIter = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    if pIter.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    if *pIter as ::core::ffi::c_int == 0x1 as ::core::ffi::c_int {
        pIter = pIter.offset(1);
        pIter = pIter.offset(
            (if *(pIter as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                != 0
            {
                sqlite3Fts3GetVarint32(pIter, &raw mut iThis)
            } else {
                iThis = *(pIter as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            }) as isize,
        );
    } else {
        iThis = 0 as ::core::ffi::c_int;
    }
    while iThis < iCol {
        fts3ColumnlistCopy(
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            &raw mut pIter,
        );
        if *pIter as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        pIter = pIter.offset(1);
        pIter = pIter.offset(
            (if *(pIter as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                != 0
            {
                sqlite3Fts3GetVarint32(pIter, &raw mut iThis)
            } else {
                iThis = *(pIter as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            }) as isize,
        );
    }
    if *pIter as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        pIter = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *ppOut = if iCol == iThis {
        pIter
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Fts3EvalPhraseCleanup(
    pPhrase: *mut crate::fts3Int_h::Fts3Phrase,
) {
    if !pPhrase.is_null() {
        let mut i: ::core::ffi::c_int;
        let __pPhrase_ref = unsafe { &mut *pPhrase };
        crate::src::src::malloc::sqlite3_free(
            __pPhrase_ref.doclist.aAll as *mut ::core::ffi::c_void,
        );
        fts3EvalInvalidatePoslist(pPhrase);
        ::libc::memset(
            &raw mut __pPhrase_ref.doclist as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<crate::fts3Int_h::Fts3Doclist>()
                as crate::__stddef_size_t_h::SizeT,
        );
        i = 0 as ::core::ffi::c_int;
        while i < __pPhrase_ref.nToken {
            fts3SegReaderCursorFree(
                (*(&raw mut __pPhrase_ref.aToken as *mut crate::fts3Int_h::Fts3PhraseToken)
                    .offset(i as isize))
                .pSegcsr,
            );
            let fresh31 = &mut (*(&raw mut __pPhrase_ref.aToken
                as *mut crate::fts3Int_h::Fts3PhraseToken)
                .offset(i as isize))
            .pSegcsr;
            *fresh31 = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3MultiSegReader>();
            i += 1;
        }
    }
}
