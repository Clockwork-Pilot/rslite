#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(static_mut_refs)]
#![allow(unsafe_op_in_unsafe_fn)]
#![feature(
    c_variadic
)]


// Re-export sqlite_printf!, sqlite_snprintf! and json_printf! macros from proc-macro crate
pub use sqlite_printf_macros::sqlite_printf;
pub use sqlite_printf_macros::sqlite_snprintf;
pub use sqlite_printf_macros::json_printf;
pub use sqlite_printf_macros::sqlite_vmprintf;
pub use sqlite_printf_macros::sqlite_vsnprintf;

#[path = "src/vdbe/mod.rs"]
pub mod vdbe;
#[path = "src/sql/mod.rs"]
pub mod sql;
#[path = "src/schema.rs"]
pub mod schema;
#[path = "src/wip_db.rs"]
pub mod wip_db;
#[path = "src/db.rs"]
pub mod db;
pub mod pcache_h {
    pub use crate::src::src::pcache::PCache;
}
pub mod __stddef_ptrdiff_t_h {
    pub type ptrdiff_t = isize;
}
pub mod __stdarg___gnuc_va_list_h {
    pub type __gnuc_va_list = crate::internal::__builtin_va_list;
}
pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod keywordhash_h {
    pub const SQLITE_N_KEYWORD: ::core::ffi::c_int = 147 as ::core::ffi::c_int;
}
pub mod vxworks_h {
    pub const OS_VXWORKS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub mod sqliteLimit_h {
    pub const SQLITE_MAX_LENGTH: ::core::ffi::c_int = 1000000000 as ::core::ffi::c_int;

    pub const SQLITE_MIN_LENGTH: ::core::ffi::c_int = 30 as ::core::ffi::c_int;

    pub const SQLITE_MAX_COLUMN: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;

    pub const SQLITE_MAX_SQL_LENGTH: ::core::ffi::c_int = 1000000000 as ::core::ffi::c_int;

    pub const SQLITE_MAX_EXPR_DEPTH: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;

    pub const SQLITE_MAX_COMPOUND_SELECT: ::core::ffi::c_int = 500 as ::core::ffi::c_int;

    pub const SQLITE_MAX_VDBE_OP: ::core::ffi::c_int = 250000000 as ::core::ffi::c_int;

    pub const SQLITE_MAX_FUNCTION_ARG: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;

    pub const SQLITE_DEFAULT_CACHE_SIZE: ::core::ffi::c_int = -(2000 as ::core::ffi::c_int);

    pub const SQLITE_DEFAULT_WAL_AUTOCHECKPOINT: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;

    pub const SQLITE_MAX_ATTACHED: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

    pub const SQLITE_MAX_VARIABLE_NUMBER: ::core::ffi::c_int = 32766 as ::core::ffi::c_int;

    pub const SQLITE_MAX_PAGE_SIZE: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;

    pub const SQLITE_MAX_DEFAULT_PAGE_SIZE: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;

    pub const SQLITE_MAX_PAGE_COUNT: ::core::ffi::c_uint = 0xfffffffe as ::core::ffi::c_uint;

    pub const SQLITE_MAX_LIKE_PATTERN_LENGTH: ::core::ffi::c_int = 50000 as ::core::ffi::c_int;

    pub const SQLITE_MAX_TRIGGER_DEPTH: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
}
pub mod internal {
    pub type __builtin_va_list = [crate::internal::__va_list_tag; 1];

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }

    pub const SQLITE_DEFAULT_PAGE_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

    pub const __ATOMIC_RELAXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const SQLITE_THREADSAFE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const __SIZEOF_POINTER__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
}

pub mod geopoly_c {
    pub type GeoCoord = ::core::ffi::c_float;
    use crate::src::ext::rtree::rtree::RtreeCoord;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct GeoPoly {
        pub nVertex: ::core::ffi::c_int,
        pub hdr: [::core::ffi::c_uchar; 4],
        pub a: [crate::geopoly_c::GeoCoord; 8],
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct GeoParse {
        pub z: *const ::core::ffi::c_uchar,
        pub nVertex: ::core::ffi::c_int,
        pub nAlloc: ::core::ffi::c_int,
        pub nErr: ::core::ffi::c_int,
        pub a: *mut crate::geopoly_c::GeoCoord,
    }

    pub const GEOPOLY_PI: ::core::ffi::c_double = 3.1415926535897932385f64;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct GeoBBox {
        pub isInit: ::core::ffi::c_int,
        pub a: [RtreeCoord; 4],
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct GeoEvent {
        pub x: ::core::ffi::c_double,
        pub eType: ::core::ffi::c_int,
        pub pSeg: *mut crate::geopoly_c::GeoSegment,
        pub pNext: *mut crate::geopoly_c::GeoEvent,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct GeoSegment {
        pub C: ::core::ffi::c_double,
        pub B: ::core::ffi::c_double,
        pub y: ::core::ffi::c_double,
        pub y0: ::core::ffi::c_float,
        pub side: ::core::ffi::c_uchar,
        pub idx: ::core::ffi::c_uint,
        pub pNext: *mut crate::geopoly_c::GeoSegment,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct GeoOverlap {
        pub aEvent: *mut crate::geopoly_c::GeoEvent,
        pub aSegment: *mut crate::geopoly_c::GeoSegment,
        pub nEvent: ::core::ffi::c_int,
        pub nSegment: ::core::ffi::c_int,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct C2RustUnnamed_2 {
        pub xFunc: Option<
            unsafe extern "C" fn(
                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            ) -> (),
        >,
        pub nArg: ::core::ffi::c_schar,
        pub bPure: ::core::ffi::c_uchar,
        pub zName: *const ::core::ffi::c_char,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct C2RustUnnamed_1 {
        pub xStep: Option<
            unsafe extern "C" fn(
                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            ) -> (),
        >,
        pub xFinal: Option<
            unsafe extern "C" fn(*mut crate::src::headers::vdbeInt_h::sqlite3_context) -> (),
        >,
        pub zName: *const ::core::ffi::c_char,
    }
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

    pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
}
pub mod fts3Int_h {
    #[cfg(feature = "fts3")]
    #[cfg(feature = "test")]
    unsafe extern "C" {
        pub fn sqlite3Fts3InitTerm(
            db: *mut crate::src::headers::sqliteInt_h::sqlite3,
        ) -> ::core::ffi::c_int;
    }
    #[cfg(feature = "fts3")]
    pub use crate::src::ext::fts3::fts3_snippet::MatchinfoBuffer;
    #[cfg(feature = "fts3")]
    pub use crate::src::ext::fts3::fts3_write::Fts3DeferredToken;
    #[cfg(feature = "fts3")]
    pub use crate::src::ext::fts3::fts3_write::Fts3SegReader;

    #[cfg(feature = "fts3")]
    pub const SQLITE_FTS3_MAX_EXPR_DEPTH: ::core::ffi::c_int = 12 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_MERGE_COUNT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_MAX_PENDING_DATA: ::core::ffi::c_int =
        1 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_VARINT_MAX: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_BUFFER_PADDING: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_SEGDIR_MAXLEVEL: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const POS_COLUMN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const POS_END: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    // LARGEST_INT64 / SMALLEST_INT64 are used throughout core SQLite (not FTS3-specific)
    pub const LARGEST_INT64: crate::src::ext::rtree::rtree::i64_0 = 0xffffffff
        as crate::src::ext::rtree::rtree::i64_0
        | (0x7fffffff as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0)
            << 32 as ::core::ffi::c_int;

    pub const SMALLEST_INT64: crate::src::ext::rtree::rtree::i64_0 = -(1 as ::core::ffi::c_int)
        as crate::src::ext::rtree::rtree::i64_0
        - crate::fts3Int_h::LARGEST_INT64;

    #[cfg(feature = "fts3")]
    pub const FTS_CORRUPT_VTAB: ::core::ffi::c_int =
        crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;

    #[cfg(feature = "fts3")]
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct Fts3Table {
        pub base: crate::src::headers::sqlite3_h::sqlite3_vtab,
        pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
        pub zDb: *const ::core::ffi::c_char,
        pub zName: *const ::core::ffi::c_char,
        pub nColumn: ::core::ffi::c_int,
        pub azColumn: *mut *mut ::core::ffi::c_char,
        pub abNotindexed: *mut crate::src::ext::rtree::rtree::u8_0,
        pub pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
        pub zContentTbl: *mut ::core::ffi::c_char,
        pub zLanguageid: *mut ::core::ffi::c_char,
        pub nAutoincrmerge: ::core::ffi::c_int,
        pub nLeafAdd: crate::src::ext::rtree::rtree::u32_0,
        pub bLock: ::core::ffi::c_int,
        pub aStmt: [*mut crate::src::headers::sqlite3_h::sqlite3_stmt; 40],
        pub pSeekStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt,
        pub zReadExprlist: *mut ::core::ffi::c_char,
        pub zWriteExprlist: *mut ::core::ffi::c_char,
        pub nNodeSize: ::core::ffi::c_int,
        pub bFts4: crate::src::ext::rtree::rtree::u8_0,
        pub bHasStat: crate::src::ext::rtree::rtree::u8_0,
        pub bHasDocsize: crate::src::ext::rtree::rtree::u8_0,
        pub bDescIdx: crate::src::ext::rtree::rtree::u8_0,
        pub bIgnoreSavepoint: crate::src::ext::rtree::rtree::u8_0,
        pub nPgsz: ::core::ffi::c_int,
        pub zSegmentsTbl: *mut ::core::ffi::c_char,
        pub pSegments: *mut crate::src::headers::sqlite3_h::sqlite3_blob,
        pub iSavepoint: ::core::ffi::c_int,
        pub nIndex: ::core::ffi::c_int,
        pub aIndex: *mut crate::fts3Int_h::Fts3Index,
        pub nMaxPendingData: ::core::ffi::c_int,
        pub nPendingData: ::core::ffi::c_int,
        pub iPrevDocid: crate::src::headers::sqlite3_h::sqlite_int64,
        pub iPrevLangid: ::core::ffi::c_int,
        pub bPrevDelete: ::core::ffi::c_int,
        pub bNoIncrDoclist: ::core::ffi::c_int,
        pub nMergeCount: ::core::ffi::c_int,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    #[cfg(feature = "fts3")]
    pub struct Fts3Index {
        pub nPrefix: ::core::ffi::c_int,
        pub hPending: crate::src::ext::fts3::fts3_hash::Fts3Hash,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    #[cfg(feature = "fts3")]
    pub struct Fts3Cursor {
        pub base: crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
        pub eSearch: crate::src::fts5::i16_0,
        pub isEof: crate::src::ext::rtree::rtree::u8_0,
        pub isRequireSeek: crate::src::ext::rtree::rtree::u8_0,
        pub bSeekStmt: crate::src::ext::rtree::rtree::u8_0,
        pub pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt,
        pub pExpr: *mut crate::fts3Int_h::Fts3Expr,
        pub iLangid: ::core::ffi::c_int,
        pub nPhrase: ::core::ffi::c_int,
        pub pDeferred: *mut crate::fts3Int_h::Fts3DeferredToken,
        pub iPrevId: crate::src::headers::sqlite3_h::sqlite3_int64,
        pub pNextId: *mut ::core::ffi::c_char,
        pub aDoclist: *mut ::core::ffi::c_char,
        pub nDoclist: ::core::ffi::c_int,
        pub bDesc: crate::src::ext::rtree::rtree::u8_0,
        pub eEvalmode: ::core::ffi::c_int,
        pub nRowAvg: ::core::ffi::c_int,
        pub nDoc: crate::src::headers::sqlite3_h::sqlite3_int64,
        pub iMinDocid: crate::src::ext::rtree::rtree::i64_0,
        pub iMaxDocid: crate::src::ext::rtree::rtree::i64_0,
        pub isMatchinfoNeeded: ::core::ffi::c_int,
        pub pMIBuffer: *mut crate::fts3Int_h::MatchinfoBuffer,
    }

    #[cfg(feature = "fts3")]
    pub const FTS3_FULLSCAN_SEARCH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_DOCID_SEARCH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_FULLTEXT_SEARCH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_HAVE_LANGID: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_HAVE_DOCID_GE: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_HAVE_DOCID_LE: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;

    #[derive(Copy, Clone)]
    #[repr(C)]

    #[cfg(feature = "fts3")]
    pub struct Fts3Doclist {
        pub aAll: *mut ::core::ffi::c_char,
        pub nAll: ::core::ffi::c_int,
        pub pNextDocid: *mut ::core::ffi::c_char,
        pub iDocid: crate::src::headers::sqlite3_h::sqlite3_int64,
        pub bFreeList: ::core::ffi::c_int,
        pub pList: *mut ::core::ffi::c_char,
        pub nList: ::core::ffi::c_int,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    #[cfg(feature = "fts3")]
    pub struct Fts3PhraseToken {
        pub z: *mut ::core::ffi::c_char,
        pub n: ::core::ffi::c_int,
        pub isPrefix: ::core::ffi::c_int,
        pub bFirst: ::core::ffi::c_int,
        pub pDeferred: *mut crate::fts3Int_h::Fts3DeferredToken,
        pub pSegcsr: *mut crate::fts3Int_h::Fts3MultiSegReader,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    #[cfg(feature = "fts3")]
    pub struct Fts3Phrase {
        pub doclist: crate::fts3Int_h::Fts3Doclist,
        pub bIncr: ::core::ffi::c_int,
        pub iDoclistToken: ::core::ffi::c_int,
        pub pOrPoslist: *mut ::core::ffi::c_char,
        pub iOrDocid: crate::src::ext::rtree::rtree::i64_0,
        pub nToken: ::core::ffi::c_int,
        pub iColumn: ::core::ffi::c_int,
        pub aToken: [crate::fts3Int_h::Fts3PhraseToken; 0],
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    #[cfg(feature = "fts3")]
    pub struct Fts3Expr {
        pub eType: ::core::ffi::c_int,
        pub nNear: ::core::ffi::c_int,
        pub pParent: *mut crate::fts3Int_h::Fts3Expr,
        pub pLeft: *mut crate::fts3Int_h::Fts3Expr,
        pub pRight: *mut crate::fts3Int_h::Fts3Expr,
        pub pPhrase: *mut crate::fts3Int_h::Fts3Phrase,
        pub iDocid: crate::src::headers::sqlite3_h::sqlite3_int64,
        pub bEof: crate::src::ext::rtree::rtree::u8_0,
        pub bStart: crate::src::ext::rtree::rtree::u8_0,
        pub bDeferred: crate::src::ext::rtree::rtree::u8_0,
        pub iPhrase: ::core::ffi::c_int,
        pub aMI: *mut crate::src::ext::rtree::rtree::u32_0,
    }

    #[cfg(feature = "fts3")]
    pub const FTSQUERY_NEAR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTSQUERY_NOT: ::core::ffi::c_int = 2;

    #[cfg(feature = "fts3")]
    pub const FTSQUERY_NOT_1: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTSQUERY_AND: ::core::ffi::c_int = 3;

    #[cfg(feature = "fts3")]
    pub const FTSQUERY_AND_1: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTSQUERY_OR: ::core::ffi::c_int = 4;

    #[cfg(feature = "fts3")]
    pub const FTSQUERY_OR_1: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTSQUERY_PHRASE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_SEGCURSOR_PENDING: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);

    #[cfg(feature = "fts3")]
    pub const FTS3_SEGCURSOR_ALL: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);

    #[cfg(feature = "fts3")]
    pub const FTS3_SEGMENT_REQUIRE_POS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_SEGMENT_IGNORE_EMPTY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_SEGMENT_COLUMN_FILTER: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_SEGMENT_PREFIX: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_SEGMENT_SCAN: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

    #[cfg(feature = "fts3")]
    pub const FTS3_SEGMENT_FIRST: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

    #[derive(Copy, Clone)]
    #[repr(C)]

    #[cfg(feature = "fts3")]
    pub struct Fts3SegFilter {
        pub zTerm: *const ::core::ffi::c_char,
        pub nTerm: ::core::ffi::c_int,
        pub iCol: ::core::ffi::c_int,
        pub flags: ::core::ffi::c_int,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    #[cfg(feature = "fts3")]
    pub struct Fts3MultiSegReader {
        pub apSegment: *mut *mut crate::fts3Int_h::Fts3SegReader,
        pub nSegment: ::core::ffi::c_int,
        pub nAdvance: ::core::ffi::c_int,
        pub pFilter: *mut crate::fts3Int_h::Fts3SegFilter,
        pub aBuffer: *mut ::core::ffi::c_char,
        pub nBuffer: crate::src::ext::rtree::rtree::i64_0,
        pub iColFilter: ::core::ffi::c_int,
        pub bRestart: ::core::ffi::c_int,
        pub nCost: ::core::ffi::c_int,
        pub bLookup: ::core::ffi::c_int,
        pub zTerm: *mut ::core::ffi::c_char,
        pub nTerm: ::core::ffi::c_int,
        pub aDoclist: *mut ::core::ffi::c_char,
        pub nDoclist: ::core::ffi::c_int,
    }
}


#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod src {
    pub mod ctime;
    pub mod headers;
    pub mod ext {
        #[cfg(feature = "fts3")]
        pub mod fts3 {
            pub mod fts3;
            pub mod fts3_aux;
            pub mod fts3_expr;
            pub mod fts3_hash;
            pub mod fts3_icu;
            pub mod fts3_porter;
            pub mod fts3_snippet;
            pub mod fts3_tokenize_vtab;
            pub mod fts3_tokenizer;
            pub mod fts3_tokenizer1;
            pub mod fts3_unicode;
            pub mod fts3_unicode2;
            pub mod fts3_write;
        } // mod fts3
        pub mod icu {
            pub mod icu;
        } // mod icu
        pub mod misc {
            pub mod stmt;
        } // mod misc
        pub mod rbu {
            pub mod sqlite3rbu;
        } // mod rbu
        pub mod rtree {
            pub mod rtree;
        } // mod rtree
        pub mod session {
            pub mod sqlite3session;
        } // mod session
    } // mod ext
    pub mod fts5;
    pub mod mutex_pl;
    pub mod opcodes;
    pub mod parse;
    pub mod printf_c_variadic;
    pub mod src {
        pub mod alter;
        pub mod analyze;
        pub mod attach;
        pub mod auth;
        pub mod backup;
        pub mod bitvec;
        pub mod btmutex;
        pub mod btree;
        pub mod build;
        pub mod callback;
        pub mod carray;
        pub mod complete;
        pub mod date;
        pub mod dbpage;
        pub mod dbstat;
        pub mod delete;
        pub mod expr;
        pub mod fault;
        pub mod fkey;
        pub mod func;
        pub mod global;
        pub mod hash;
        pub mod insert;
        pub mod json;
        pub mod legacy;
        pub mod loadext;
        pub mod malloc;
        pub mod mem0;
        pub mod mem1;
        pub mod mem2;
        pub mod mem3;
        pub mod mem5;
        pub mod memdb;
        pub mod memjournal;
        pub mod mutex;
        pub mod mutex_noop;
        pub mod mutex_unix;
        pub mod mutex_w32;
        pub mod notify;
        pub mod os;
        pub mod os_kv;
        pub mod os_unix;
        pub mod os_win;
        pub mod pager;
        pub mod pcache;
        pub mod pcache1;
        pub mod pragma;
        pub mod prepare;
        pub mod printf;
        pub mod random;
        pub mod resolve;
        pub mod rowset;
        pub mod select;
        pub mod main;
        pub mod r#where;
        pub mod status;
        pub mod table;
        pub mod threads;
        pub mod tokenize;
        pub mod treeview;
        pub mod trigger;
        pub mod update;
        pub mod upsert;
        pub mod utf;
        pub mod util;
        pub mod vacuum;
        pub mod vdbe;
        pub mod vdbeapi;
        pub mod vdbeaux;
        pub mod vdbeblob;
        pub mod vdbemem;
        pub mod vdbesort;
        pub mod vdbetrace;
        pub mod vdbevtab;
        pub mod vtab;
        pub mod wal;
        pub mod walker;
        pub mod wherecode;
        pub mod whereexpr;
        pub mod window;
    } // mod src
} // mod src
