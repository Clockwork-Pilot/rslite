#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(static_mut_refs)]
#![feature(
    c_variadic
)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![feature(atomic_from_ptr)]
#![feature(let_chains)]
#![feature(result_option_inspect)]

// Re-export sqlite_printf! macro from proc-macro crate
pub use sqlite_printf_macros::sqlite_printf;

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
#[path = "src/safe_format.rs"]
pub mod safe_format;
#[path = "src/format_utils.rs"]
pub mod format_utils;
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
    #[cfg(feature = "test")]
    unsafe extern "C" {
        pub fn sqlite3Fts3InitTerm(
            db: *mut crate::src::headers::sqliteInt_h::sqlite3,
        ) -> ::core::ffi::c_int;
    }
    pub use crate::src::ext::fts3::fts3_snippet::MatchinfoBuffer;
    pub use crate::src::ext::fts3::fts3_write::Fts3DeferredToken;
    pub use crate::src::ext::fts3::fts3_write::Fts3SegReader;

    pub const SQLITE_FTS3_MAX_EXPR_DEPTH: ::core::ffi::c_int = 12 as ::core::ffi::c_int;

    pub const FTS3_MERGE_COUNT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;

    pub const FTS3_MAX_PENDING_DATA: ::core::ffi::c_int =
        1 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;

    pub const FTS3_VARINT_MAX: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

    pub const FTS3_BUFFER_PADDING: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

    pub const FTS3_SEGDIR_MAXLEVEL: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

    pub const POS_COLUMN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const POS_END: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const LARGEST_INT64: crate::src::ext::rtree::rtree::i64_0 = 0xffffffff
        as crate::src::ext::rtree::rtree::i64_0
        | (0x7fffffff as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0)
            << 32 as ::core::ffi::c_int;

    pub const SMALLEST_INT64: crate::src::ext::rtree::rtree::i64_0 = -(1 as ::core::ffi::c_int)
        as crate::src::ext::rtree::rtree::i64_0
        - crate::fts3Int_h::LARGEST_INT64;

    pub const FTS_CORRUPT_VTAB: ::core::ffi::c_int =
        crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;

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

    pub struct Fts3Index {
        pub nPrefix: ::core::ffi::c_int,
        pub hPending: crate::src::ext::fts3::fts3_hash::Fts3Hash,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

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

    pub const FTS3_FULLSCAN_SEARCH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const FTS3_DOCID_SEARCH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const FTS3_FULLTEXT_SEARCH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    pub const FTS3_HAVE_LANGID: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;

    pub const FTS3_HAVE_DOCID_GE: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;

    pub const FTS3_HAVE_DOCID_LE: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;

    #[derive(Copy, Clone)]
    #[repr(C)]

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

    pub const FTSQUERY_NEAR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

    pub const FTSQUERY_NOT: ::core::ffi::c_int = 2;

    pub const FTSQUERY_NOT_1: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

    pub const FTSQUERY_AND: ::core::ffi::c_int = 3;

    pub const FTSQUERY_AND_1: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

    pub const FTSQUERY_OR: ::core::ffi::c_int = 4;

    pub const FTSQUERY_OR_1: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

    pub const FTSQUERY_PHRASE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

    pub const FTS3_SEGCURSOR_PENDING: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);

    pub const FTS3_SEGCURSOR_ALL: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);

    pub const FTS3_SEGMENT_REQUIRE_POS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

    pub const FTS3_SEGMENT_IGNORE_EMPTY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

    pub const FTS3_SEGMENT_COLUMN_FILTER: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

    pub const FTS3_SEGMENT_PREFIX: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

    pub const FTS3_SEGMENT_SCAN: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

    pub const FTS3_SEGMENT_FIRST: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct Fts3SegFilter {
        pub zTerm: *const ::core::ffi::c_char,
        pub nTerm: ::core::ffi::c_int,
        pub iCol: ::core::ffi::c_int,
        pub flags: ::core::ffi::c_int,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

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

pub mod stdlib {
    unsafe extern "C" {
        pub fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
        pub fn acos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn asin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn atan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn atan2(
            __y: ::core::ffi::c_double,
            __x: ::core::ffi::c_double,
        ) -> ::core::ffi::c_double;

        pub fn cos(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn sin(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn tan(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn cosh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn sinh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn tanh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn acosh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn asinh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn atanh(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn log(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn log2(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double)
            -> ::core::ffi::c_double;

        pub fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

        pub fn fmod(
            __x: ::core::ffi::c_double,
            __y: ::core::ffi::c_double,
        ) -> ::core::ffi::c_double;

        pub fn trunc(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        pub fn pthread_create(
            __newthread: *mut crate::stdlib::pthread_t,
            __attr: *const crate::stdlib::pthread_attr_t,
            __start_routine: Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            >,
            __arg: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;

        pub fn pthread_mutex_init(
            __mutex: *mut crate::stdlib::pthread_mutex_t,
            __mutexattr: *const crate::stdlib::pthread_mutexattr_t,
        ) -> ::core::ffi::c_int;

        pub fn pthread_mutex_destroy(
            __mutex: *mut crate::stdlib::pthread_mutex_t,
        ) -> ::core::ffi::c_int;

        pub fn pthread_mutex_trylock(
            __mutex: *mut crate::stdlib::pthread_mutex_t,
        ) -> ::core::ffi::c_int;

        pub fn pthread_mutex_lock(
            __mutex: *mut crate::stdlib::pthread_mutex_t,
        ) -> ::core::ffi::c_int;

        pub fn pthread_mutex_unlock(
            __mutex: *mut crate::stdlib::pthread_mutex_t,
        ) -> ::core::ffi::c_int;

        pub fn pthread_mutexattr_init(
            __attr: *mut crate::stdlib::pthread_mutexattr_t,
        ) -> ::core::ffi::c_int;

        pub fn pthread_mutexattr_destroy(
            __attr: *mut crate::stdlib::pthread_mutexattr_t,
        ) -> ::core::ffi::c_int;

        pub fn pthread_mutexattr_settype(
            __attr: *mut crate::stdlib::pthread_mutexattr_t,
            __kind: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        pub fn stat(
            __file: *const ::core::ffi::c_char,
            __buf: *mut crate::stdlib::stat,
        ) -> ::core::ffi::c_int;

        pub fn fstat(
            __fd: ::core::ffi::c_int,
            __buf: *mut crate::stdlib::stat,
        ) -> ::core::ffi::c_int;

        pub fn lstat(
            __file: *const ::core::ffi::c_char,
            __buf: *mut crate::stdlib::stat,
        ) -> ::core::ffi::c_int;
        pub static mut stdout: *mut crate::stdlib::FILE;

        pub fn fflush(__stream: *mut crate::stdlib::FILE) -> ::core::ffi::c_int;

        pub fn fprintf(
            __stream: *mut crate::stdlib::FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn strcspn(
            __s: *const ::core::ffi::c_char,
            __reject: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_ulong;

        pub fn strspn(
            __s: *const ::core::ffi::c_char,
            __accept: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_ulong;
        pub type _IO_marker;

        pub type _IO_codecvt;

        pub type _IO_wide_data;

        #[cfg(feature = "test")]
        pub fn TclFreeObj(objPtr: *mut crate::stdlib::Tcl_Obj);

        #[cfg(feature = "test")]
        pub fn Tcl_ListObjAppendElement(
            interp: *mut crate::stdlib::Tcl_Interp,
            listPtr: *mut crate::stdlib::Tcl_Obj,
            objPtr: *mut crate::stdlib::Tcl_Obj,
        ) -> ::core::ffi::c_int;

        #[cfg(feature = "test")]
        pub fn Tcl_NewObj() -> *mut crate::stdlib::Tcl_Obj;

        #[cfg(feature = "test")]
        pub fn Tcl_NewStringObj(
            bytes: *const ::core::ffi::c_char,
            length: crate::stdlib::Tcl_Size,
        ) -> *mut crate::stdlib::Tcl_Obj;

        #[cfg(feature = "test")]
        pub fn Tcl_NewWideIntObj(
            wideValue: crate::stdlib::Tcl_WideInt,
        ) -> *mut crate::stdlib::Tcl_Obj;

        #[cfg(feature = "test")]
        pub fn Tcl_GetStringFromObj(
            objPtr: *mut crate::stdlib::Tcl_Obj,
            lengthPtr: *mut crate::stdlib::Tcl_Size,
        ) -> *mut ::core::ffi::c_char;
        pub type Tcl_Interp;

        pub fn gettimeofday(
            __tv: *mut ::libc::timeval,
            __tz: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
        pub fn read(
            __fd: ::core::ffi::c_int,
            __buf: *mut ::core::ffi::c_void,
            __nbytes: crate::__stddef_size_t_h::size_t,
        ) -> crate::stdlib::ssize_t;

        pub fn write(
            __fd: ::core::ffi::c_int,
            __buf: *const ::core::ffi::c_void,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> crate::stdlib::ssize_t;

        pub fn pread64(
            __fd: ::core::ffi::c_int,
            __buf: *mut ::core::ffi::c_void,
            __nbytes: crate::__stddef_size_t_h::size_t,
            __offset: crate::stdlib::__off64_t,
        ) -> crate::stdlib::ssize_t;

        pub fn pwrite64(
            __fd: ::core::ffi::c_int,
            __buf: *const ::core::ffi::c_void,
            __n: crate::__stddef_size_t_h::size_t,
            __offset: crate::stdlib::__off64_t,
        ) -> crate::stdlib::ssize_t;

        pub fn readlink(
            __path: *const ::core::ffi::c_char,
            __buf: *mut ::core::ffi::c_char,
            __len: crate::__stddef_size_t_h::size_t,
        ) -> crate::stdlib::ssize_t;
    }
    pub type FILE = crate::stdlib::_IO_FILE;
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
    pub const _SC_ARG_MAX: crate::stdlib::C2RustUnnamed_1 = 0;

    pub const _SC_CHILD_MAX: crate::stdlib::C2RustUnnamed_1 = 1;

    pub const _SC_CLK_TCK: crate::stdlib::C2RustUnnamed_1 = 2;

    pub const _SC_NGROUPS_MAX: crate::stdlib::C2RustUnnamed_1 = 3;

    pub const _SC_OPEN_MAX: crate::stdlib::C2RustUnnamed_1 = 4;

    pub const _SC_STREAM_MAX: crate::stdlib::C2RustUnnamed_1 = 5;

    pub const _SC_TZNAME_MAX: crate::stdlib::C2RustUnnamed_1 = 6;

    pub const _SC_JOB_CONTROL: crate::stdlib::C2RustUnnamed_1 = 7;

    pub const _SC_SAVED_IDS: crate::stdlib::C2RustUnnamed_1 = 8;

    pub const _SC_REALTIME_SIGNALS: crate::stdlib::C2RustUnnamed_1 = 9;

    pub const _SC_PRIORITY_SCHEDULING: crate::stdlib::C2RustUnnamed_1 = 10;

    pub const _SC_TIMERS: crate::stdlib::C2RustUnnamed_1 = 11;

    pub const _SC_ASYNCHRONOUS_IO: crate::stdlib::C2RustUnnamed_1 = 12;

    pub const _SC_PRIORITIZED_IO: crate::stdlib::C2RustUnnamed_1 = 13;

    pub const _SC_SYNCHRONIZED_IO: crate::stdlib::C2RustUnnamed_1 = 14;

    pub const _SC_FSYNC: crate::stdlib::C2RustUnnamed_1 = 15;

    pub const _SC_MAPPED_FILES: crate::stdlib::C2RustUnnamed_1 = 16;

    pub const _SC_MEMLOCK: crate::stdlib::C2RustUnnamed_1 = 17;

    pub const _SC_MEMLOCK_RANGE: crate::stdlib::C2RustUnnamed_1 = 18;

    pub const _SC_MEMORY_PROTECTION: crate::stdlib::C2RustUnnamed_1 = 19;

    pub const _SC_MESSAGE_PASSING: crate::stdlib::C2RustUnnamed_1 = 20;

    pub const _SC_SEMAPHORES: crate::stdlib::C2RustUnnamed_1 = 21;

    pub const _SC_SHARED_MEMORY_OBJECTS: crate::stdlib::C2RustUnnamed_1 = 22;

    pub const _SC_AIO_LISTIO_MAX: crate::stdlib::C2RustUnnamed_1 = 23;

    pub const _SC_AIO_MAX: crate::stdlib::C2RustUnnamed_1 = 24;

    pub const _SC_AIO_PRIO_DELTA_MAX: crate::stdlib::C2RustUnnamed_1 = 25;

    pub const _SC_DELAYTIMER_MAX: crate::stdlib::C2RustUnnamed_1 = 26;

    pub const _SC_MQ_OPEN_MAX: crate::stdlib::C2RustUnnamed_1 = 27;

    pub const _SC_MQ_PRIO_MAX: crate::stdlib::C2RustUnnamed_1 = 28;

    pub const _SC_VERSION: crate::stdlib::C2RustUnnamed_1 = 29;

    pub const _SC_PAGESIZE: crate::stdlib::C2RustUnnamed_1 = 30;

    pub const _SC_RTSIG_MAX: crate::stdlib::C2RustUnnamed_1 = 31;

    pub const _SC_SEM_NSEMS_MAX: crate::stdlib::C2RustUnnamed_1 = 32;

    pub const _SC_SEM_VALUE_MAX: crate::stdlib::C2RustUnnamed_1 = 33;

    pub const _SC_SIGQUEUE_MAX: crate::stdlib::C2RustUnnamed_1 = 34;

    pub const _SC_TIMER_MAX: crate::stdlib::C2RustUnnamed_1 = 35;

    pub const _SC_BC_BASE_MAX: crate::stdlib::C2RustUnnamed_1 = 36;

    pub const _SC_BC_DIM_MAX: crate::stdlib::C2RustUnnamed_1 = 37;

    pub const _SC_BC_SCALE_MAX: crate::stdlib::C2RustUnnamed_1 = 38;

    pub const _SC_BC_STRING_MAX: crate::stdlib::C2RustUnnamed_1 = 39;

    pub const _SC_COLL_WEIGHTS_MAX: crate::stdlib::C2RustUnnamed_1 = 40;

    pub const _SC_EQUIV_CLASS_MAX: crate::stdlib::C2RustUnnamed_1 = 41;

    pub const _SC_EXPR_NEST_MAX: crate::stdlib::C2RustUnnamed_1 = 42;

    pub const _SC_LINE_MAX: crate::stdlib::C2RustUnnamed_1 = 43;

    pub const _SC_RE_DUP_MAX: crate::stdlib::C2RustUnnamed_1 = 44;

    pub const _SC_CHARCLASS_NAME_MAX: crate::stdlib::C2RustUnnamed_1 = 45;

    pub const _SC_2_VERSION: crate::stdlib::C2RustUnnamed_1 = 46;

    pub const _SC_2_C_BIND: crate::stdlib::C2RustUnnamed_1 = 47;

    pub const _SC_2_C_DEV: crate::stdlib::C2RustUnnamed_1 = 48;

    pub const _SC_2_FORT_DEV: crate::stdlib::C2RustUnnamed_1 = 49;

    pub const _SC_2_FORT_RUN: crate::stdlib::C2RustUnnamed_1 = 50;

    pub const _SC_2_SW_DEV: crate::stdlib::C2RustUnnamed_1 = 51;

    pub const _SC_2_LOCALEDEF: crate::stdlib::C2RustUnnamed_1 = 52;

    pub const _SC_PII: crate::stdlib::C2RustUnnamed_1 = 53;

    pub const _SC_PII_XTI: crate::stdlib::C2RustUnnamed_1 = 54;

    pub const _SC_PII_SOCKET: crate::stdlib::C2RustUnnamed_1 = 55;

    pub const _SC_PII_INTERNET: crate::stdlib::C2RustUnnamed_1 = 56;

    pub const _SC_PII_OSI: crate::stdlib::C2RustUnnamed_1 = 57;

    pub const _SC_POLL: crate::stdlib::C2RustUnnamed_1 = 58;

    pub const _SC_SELECT: crate::stdlib::C2RustUnnamed_1 = 59;

    pub const _SC_UIO_MAXIOV: crate::stdlib::C2RustUnnamed_1 = 60;

    pub const _SC_IOV_MAX: crate::stdlib::C2RustUnnamed_1 = 60;

    pub const _SC_PII_INTERNET_STREAM: crate::stdlib::C2RustUnnamed_1 = 61;

    pub const _SC_PII_INTERNET_DGRAM: crate::stdlib::C2RustUnnamed_1 = 62;

    pub const _SC_PII_OSI_COTS: crate::stdlib::C2RustUnnamed_1 = 63;

    pub const _SC_PII_OSI_CLTS: crate::stdlib::C2RustUnnamed_1 = 64;

    pub const _SC_PII_OSI_M: crate::stdlib::C2RustUnnamed_1 = 65;

    pub const _SC_T_IOV_MAX: crate::stdlib::C2RustUnnamed_1 = 66;

    pub const _SC_THREADS: crate::stdlib::C2RustUnnamed_1 = 67;

    pub const _SC_THREAD_SAFE_FUNCTIONS: crate::stdlib::C2RustUnnamed_1 = 68;

    pub const _SC_GETGR_R_SIZE_MAX: crate::stdlib::C2RustUnnamed_1 = 69;

    pub const _SC_GETPW_R_SIZE_MAX: crate::stdlib::C2RustUnnamed_1 = 70;

    pub const _SC_LOGIN_NAME_MAX: crate::stdlib::C2RustUnnamed_1 = 71;

    pub const _SC_TTY_NAME_MAX: crate::stdlib::C2RustUnnamed_1 = 72;

    pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: crate::stdlib::C2RustUnnamed_1 = 73;

    pub const _SC_THREAD_KEYS_MAX: crate::stdlib::C2RustUnnamed_1 = 74;

    pub const _SC_THREAD_STACK_MIN: crate::stdlib::C2RustUnnamed_1 = 75;

    pub const _SC_THREAD_THREADS_MAX: crate::stdlib::C2RustUnnamed_1 = 76;

    pub const _SC_THREAD_ATTR_STACKADDR: crate::stdlib::C2RustUnnamed_1 = 77;

    pub const _SC_THREAD_ATTR_STACKSIZE: crate::stdlib::C2RustUnnamed_1 = 78;

    pub const _SC_THREAD_PRIORITY_SCHEDULING: crate::stdlib::C2RustUnnamed_1 = 79;

    pub const _SC_THREAD_PRIO_INHERIT: crate::stdlib::C2RustUnnamed_1 = 80;

    pub const _SC_THREAD_PRIO_PROTECT: crate::stdlib::C2RustUnnamed_1 = 81;

    pub const _SC_THREAD_PROCESS_SHARED: crate::stdlib::C2RustUnnamed_1 = 82;

    pub const _SC_NPROCESSORS_CONF: crate::stdlib::C2RustUnnamed_1 = 83;

    pub const _SC_NPROCESSORS_ONLN: crate::stdlib::C2RustUnnamed_1 = 84;

    pub const _SC_PHYS_PAGES: crate::stdlib::C2RustUnnamed_1 = 85;

    pub const _SC_AVPHYS_PAGES: crate::stdlib::C2RustUnnamed_1 = 86;

    pub const _SC_ATEXIT_MAX: crate::stdlib::C2RustUnnamed_1 = 87;

    pub const _SC_PASS_MAX: crate::stdlib::C2RustUnnamed_1 = 88;

    pub const _SC_XOPEN_VERSION: crate::stdlib::C2RustUnnamed_1 = 89;

    pub const _SC_XOPEN_XCU_VERSION: crate::stdlib::C2RustUnnamed_1 = 90;

    pub const _SC_XOPEN_UNIX: crate::stdlib::C2RustUnnamed_1 = 91;

    pub const _SC_XOPEN_CRYPT: crate::stdlib::C2RustUnnamed_1 = 92;

    pub const _SC_XOPEN_ENH_I18N: crate::stdlib::C2RustUnnamed_1 = 93;

    pub const _SC_XOPEN_SHM: crate::stdlib::C2RustUnnamed_1 = 94;

    pub const _SC_2_CHAR_TERM: crate::stdlib::C2RustUnnamed_1 = 95;

    pub const _SC_2_C_VERSION: crate::stdlib::C2RustUnnamed_1 = 96;

    pub const _SC_2_UPE: crate::stdlib::C2RustUnnamed_1 = 97;

    pub const _SC_XOPEN_XPG2: crate::stdlib::C2RustUnnamed_1 = 98;

    pub const _SC_XOPEN_XPG3: crate::stdlib::C2RustUnnamed_1 = 99;

    pub const _SC_XOPEN_XPG4: crate::stdlib::C2RustUnnamed_1 = 100;

    pub const _SC_CHAR_BIT: crate::stdlib::C2RustUnnamed_1 = 101;

    pub const _SC_CHAR_MAX: crate::stdlib::C2RustUnnamed_1 = 102;

    pub const _SC_CHAR_MIN: crate::stdlib::C2RustUnnamed_1 = 103;

    pub const _SC_INT_MAX: crate::stdlib::C2RustUnnamed_1 = 104;

    pub const _SC_INT_MIN: crate::stdlib::C2RustUnnamed_1 = 105;

    pub const _SC_LONG_BIT: crate::stdlib::C2RustUnnamed_1 = 106;

    pub const _SC_WORD_BIT: crate::stdlib::C2RustUnnamed_1 = 107;

    pub const _SC_MB_LEN_MAX: crate::stdlib::C2RustUnnamed_1 = 108;

    pub const _SC_NZERO: crate::stdlib::C2RustUnnamed_1 = 109;

    pub const _SC_SSIZE_MAX: crate::stdlib::C2RustUnnamed_1 = 110;

    pub const _SC_SCHAR_MAX: crate::stdlib::C2RustUnnamed_1 = 111;

    pub const _SC_SCHAR_MIN: crate::stdlib::C2RustUnnamed_1 = 112;

    pub const _SC_SHRT_MAX: crate::stdlib::C2RustUnnamed_1 = 113;

    pub const _SC_SHRT_MIN: crate::stdlib::C2RustUnnamed_1 = 114;

    pub const _SC_UCHAR_MAX: crate::stdlib::C2RustUnnamed_1 = 115;

    pub const _SC_UINT_MAX: crate::stdlib::C2RustUnnamed_1 = 116;

    pub const _SC_ULONG_MAX: crate::stdlib::C2RustUnnamed_1 = 117;

    pub const _SC_USHRT_MAX: crate::stdlib::C2RustUnnamed_1 = 118;

    pub const _SC_NL_ARGMAX: crate::stdlib::C2RustUnnamed_1 = 119;

    pub const _SC_NL_LANGMAX: crate::stdlib::C2RustUnnamed_1 = 120;

    pub const _SC_NL_MSGMAX: crate::stdlib::C2RustUnnamed_1 = 121;

    pub const _SC_NL_NMAX: crate::stdlib::C2RustUnnamed_1 = 122;

    pub const _SC_NL_SETMAX: crate::stdlib::C2RustUnnamed_1 = 123;

    pub const _SC_NL_TEXTMAX: crate::stdlib::C2RustUnnamed_1 = 124;

    pub const _SC_XBS5_ILP32_OFF32: crate::stdlib::C2RustUnnamed_1 = 125;

    pub const _SC_XBS5_ILP32_OFFBIG: crate::stdlib::C2RustUnnamed_1 = 126;

    pub const _SC_XBS5_LP64_OFF64: crate::stdlib::C2RustUnnamed_1 = 127;

    pub const _SC_XBS5_LPBIG_OFFBIG: crate::stdlib::C2RustUnnamed_1 = 128;

    pub const _SC_XOPEN_LEGACY: crate::stdlib::C2RustUnnamed_1 = 129;

    pub const _SC_XOPEN_REALTIME: crate::stdlib::C2RustUnnamed_1 = 130;

    pub const _SC_XOPEN_REALTIME_THREADS: crate::stdlib::C2RustUnnamed_1 = 131;

    pub const _SC_ADVISORY_INFO: crate::stdlib::C2RustUnnamed_1 = 132;

    pub const _SC_BARRIERS: crate::stdlib::C2RustUnnamed_1 = 133;

    pub const _SC_BASE: crate::stdlib::C2RustUnnamed_1 = 134;

    pub const _SC_C_LANG_SUPPORT: crate::stdlib::C2RustUnnamed_1 = 135;

    pub const _SC_C_LANG_SUPPORT_R: crate::stdlib::C2RustUnnamed_1 = 136;

    pub const _SC_CLOCK_SELECTION: crate::stdlib::C2RustUnnamed_1 = 137;

    pub const _SC_CPUTIME: crate::stdlib::C2RustUnnamed_1 = 138;

    pub const _SC_THREAD_CPUTIME: crate::stdlib::C2RustUnnamed_1 = 139;

    pub const _SC_DEVICE_IO: crate::stdlib::C2RustUnnamed_1 = 140;

    pub const _SC_DEVICE_SPECIFIC: crate::stdlib::C2RustUnnamed_1 = 141;

    pub const _SC_DEVICE_SPECIFIC_R: crate::stdlib::C2RustUnnamed_1 = 142;

    pub const _SC_FD_MGMT: crate::stdlib::C2RustUnnamed_1 = 143;

    pub const _SC_FIFO: crate::stdlib::C2RustUnnamed_1 = 144;

    pub const _SC_PIPE: crate::stdlib::C2RustUnnamed_1 = 145;

    pub const _SC_FILE_ATTRIBUTES: crate::stdlib::C2RustUnnamed_1 = 146;

    pub const _SC_FILE_LOCKING: crate::stdlib::C2RustUnnamed_1 = 147;

    pub const _SC_FILE_SYSTEM: crate::stdlib::C2RustUnnamed_1 = 148;

    pub const _SC_MONOTONIC_CLOCK: crate::stdlib::C2RustUnnamed_1 = 149;

    pub const _SC_MULTI_PROCESS: crate::stdlib::C2RustUnnamed_1 = 150;

    pub const _SC_SINGLE_PROCESS: crate::stdlib::C2RustUnnamed_1 = 151;

    pub const _SC_NETWORKING: crate::stdlib::C2RustUnnamed_1 = 152;

    pub const _SC_READER_WRITER_LOCKS: crate::stdlib::C2RustUnnamed_1 = 153;

    pub const _SC_SPIN_LOCKS: crate::stdlib::C2RustUnnamed_1 = 154;

    pub const _SC_REGEXP: crate::stdlib::C2RustUnnamed_1 = 155;

    pub const _SC_REGEX_VERSION: crate::stdlib::C2RustUnnamed_1 = 156;

    pub const _SC_SHELL: crate::stdlib::C2RustUnnamed_1 = 157;

    pub const _SC_SIGNALS: crate::stdlib::C2RustUnnamed_1 = 158;

    pub const _SC_SPAWN: crate::stdlib::C2RustUnnamed_1 = 159;

    pub const _SC_SPORADIC_SERVER: crate::stdlib::C2RustUnnamed_1 = 160;

    pub const _SC_THREAD_SPORADIC_SERVER: crate::stdlib::C2RustUnnamed_1 = 161;

    pub const _SC_SYSTEM_DATABASE: crate::stdlib::C2RustUnnamed_1 = 162;

    pub const _SC_SYSTEM_DATABASE_R: crate::stdlib::C2RustUnnamed_1 = 163;

    pub const _SC_TIMEOUTS: crate::stdlib::C2RustUnnamed_1 = 164;

    pub const _SC_TYPED_MEMORY_OBJECTS: crate::stdlib::C2RustUnnamed_1 = 165;

    pub const _SC_USER_GROUPS: crate::stdlib::C2RustUnnamed_1 = 166;

    pub const _SC_USER_GROUPS_R: crate::stdlib::C2RustUnnamed_1 = 167;

    pub const _SC_2_PBS: crate::stdlib::C2RustUnnamed_1 = 168;

    pub const _SC_2_PBS_ACCOUNTING: crate::stdlib::C2RustUnnamed_1 = 169;

    pub const _SC_2_PBS_LOCATE: crate::stdlib::C2RustUnnamed_1 = 170;

    pub const _SC_2_PBS_MESSAGE: crate::stdlib::C2RustUnnamed_1 = 171;

    pub const _SC_2_PBS_TRACK: crate::stdlib::C2RustUnnamed_1 = 172;

    pub const _SC_SYMLOOP_MAX: crate::stdlib::C2RustUnnamed_1 = 173;

    pub const _SC_STREAMS: crate::stdlib::C2RustUnnamed_1 = 174;

    pub const _SC_2_PBS_CHECKPOINT: crate::stdlib::C2RustUnnamed_1 = 175;

    pub const _SC_V6_ILP32_OFF32: crate::stdlib::C2RustUnnamed_1 = 176;

    pub const _SC_V6_ILP32_OFFBIG: crate::stdlib::C2RustUnnamed_1 = 177;

    pub const _SC_V6_LP64_OFF64: crate::stdlib::C2RustUnnamed_1 = 178;

    pub const _SC_V6_LPBIG_OFFBIG: crate::stdlib::C2RustUnnamed_1 = 179;

    pub const _SC_HOST_NAME_MAX: crate::stdlib::C2RustUnnamed_1 = 180;

    pub const _SC_TRACE: crate::stdlib::C2RustUnnamed_1 = 181;

    pub const _SC_TRACE_EVENT_FILTER: crate::stdlib::C2RustUnnamed_1 = 182;

    pub const _SC_TRACE_INHERIT: crate::stdlib::C2RustUnnamed_1 = 183;

    pub const _SC_TRACE_LOG: crate::stdlib::C2RustUnnamed_1 = 184;

    pub const _SC_LEVEL1_ICACHE_SIZE: crate::stdlib::C2RustUnnamed_1 = 185;

    pub const _SC_LEVEL1_ICACHE_ASSOC: crate::stdlib::C2RustUnnamed_1 = 186;

    pub const _SC_LEVEL1_ICACHE_LINESIZE: crate::stdlib::C2RustUnnamed_1 = 187;

    pub const _SC_LEVEL1_DCACHE_SIZE: crate::stdlib::C2RustUnnamed_1 = 188;

    pub const _SC_LEVEL1_DCACHE_ASSOC: crate::stdlib::C2RustUnnamed_1 = 189;

    pub const _SC_LEVEL1_DCACHE_LINESIZE: crate::stdlib::C2RustUnnamed_1 = 190;

    pub const _SC_LEVEL2_CACHE_SIZE: crate::stdlib::C2RustUnnamed_1 = 191;

    pub const _SC_LEVEL2_CACHE_ASSOC: crate::stdlib::C2RustUnnamed_1 = 192;

    pub const _SC_LEVEL2_CACHE_LINESIZE: crate::stdlib::C2RustUnnamed_1 = 193;

    pub const _SC_LEVEL3_CACHE_SIZE: crate::stdlib::C2RustUnnamed_1 = 194;

    pub const _SC_LEVEL3_CACHE_ASSOC: crate::stdlib::C2RustUnnamed_1 = 195;

    pub const _SC_LEVEL3_CACHE_LINESIZE: crate::stdlib::C2RustUnnamed_1 = 196;

    pub const _SC_LEVEL4_CACHE_SIZE: crate::stdlib::C2RustUnnamed_1 = 197;

    pub const _SC_LEVEL4_CACHE_ASSOC: crate::stdlib::C2RustUnnamed_1 = 198;

    pub const _SC_LEVEL4_CACHE_LINESIZE: crate::stdlib::C2RustUnnamed_1 = 199;

    pub const _SC_IPV6: crate::stdlib::C2RustUnnamed_1 = 235;

    pub const _SC_RAW_SOCKETS: crate::stdlib::C2RustUnnamed_1 = 236;

    pub const _SC_V7_ILP32_OFF32: crate::stdlib::C2RustUnnamed_1 = 237;

    pub const _SC_V7_ILP32_OFFBIG: crate::stdlib::C2RustUnnamed_1 = 238;

    pub const _SC_V7_LP64_OFF64: crate::stdlib::C2RustUnnamed_1 = 239;

    pub const _SC_V7_LPBIG_OFFBIG: crate::stdlib::C2RustUnnamed_1 = 240;

    pub const _SC_SS_REPL_MAX: crate::stdlib::C2RustUnnamed_1 = 241;

    pub const _SC_TRACE_EVENT_NAME_MAX: crate::stdlib::C2RustUnnamed_1 = 242;

    pub const _SC_TRACE_NAME_MAX: crate::stdlib::C2RustUnnamed_1 = 243;

    pub const _SC_TRACE_SYS_MAX: crate::stdlib::C2RustUnnamed_1 = 244;

    pub const _SC_TRACE_USER_EVENT_MAX: crate::stdlib::C2RustUnnamed_1 = 245;

    pub const _SC_XOPEN_STREAMS: crate::stdlib::C2RustUnnamed_1 = 246;

    pub const _SC_THREAD_ROBUST_PRIO_INHERIT: crate::stdlib::C2RustUnnamed_1 = 247;

    pub const _SC_THREAD_ROBUST_PRIO_PROTECT: crate::stdlib::C2RustUnnamed_1 = 248;

    pub const _SC_MINSIGSTKSZ: crate::stdlib::C2RustUnnamed_1 = 249;

    pub const _SC_SIGSTKSZ: crate::stdlib::C2RustUnnamed_1 = 250;
    pub type C2RustUnnamed_0_1 = ::core::ffi::c_uint;

    pub const _ISupper: crate::stdlib::C2RustUnnamed_0_1 = 256;

    pub const _ISlower: crate::stdlib::C2RustUnnamed_0_1 = 512;

    pub const _ISalpha: crate::stdlib::C2RustUnnamed_0_1 = 1024;

    pub const _ISdigit: crate::stdlib::C2RustUnnamed_0_1 = 2048;

    pub const _ISxdigit: crate::stdlib::C2RustUnnamed_0_1 = 4096;

    pub const _ISspace: crate::stdlib::C2RustUnnamed_0_1 = 8192;

    pub const _ISprint: crate::stdlib::C2RustUnnamed_0_1 = 16384;

    pub const _ISgraph: crate::stdlib::C2RustUnnamed_0_1 = 32768;

    pub const _ISblank: crate::stdlib::C2RustUnnamed_0_1 = 1;

    pub const _IScntrl: crate::stdlib::C2RustUnnamed_0_1 = 2;

    pub const _ISpunct: crate::stdlib::C2RustUnnamed_0_1 = 4;

    pub const _ISalnum: crate::stdlib::C2RustUnnamed_0_1 = 8;
    pub const __O_LARGEFILE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    pub const F_GETLK64: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

    pub const F_SETLK64: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;

    pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
    pub const M_PI: ::core::ffi::c_double = 3.14159265358979323846f64;
    pub type C2RustUnnamed_1 = ::core::ffi::c_uint;

    pub const PTHREAD_MUTEX_TIMED_NP: crate::stdlib::C2RustUnnamed_1 = 0;

    pub const PTHREAD_MUTEX_RECURSIVE_NP: crate::stdlib::C2RustUnnamed_1 = 1;

    pub const PTHREAD_MUTEX_ERRORCHECK_NP: crate::stdlib::C2RustUnnamed_1 = 2;

    pub const PTHREAD_MUTEX_ADAPTIVE_NP: crate::stdlib::C2RustUnnamed_1 = 3;

    pub const PTHREAD_MUTEX_NORMAL: crate::stdlib::C2RustUnnamed_1 = 0;

    pub const PTHREAD_MUTEX_RECURSIVE: crate::stdlib::C2RustUnnamed_1 = 1;

    pub const PTHREAD_MUTEX_ERRORCHECK: crate::stdlib::C2RustUnnamed_1 = 2;

    pub const PTHREAD_MUTEX_DEFAULT: crate::stdlib::C2RustUnnamed_1 = 0;

    pub const PTHREAD_MUTEX_FAST_NP: crate::stdlib::C2RustUnnamed_1 = 0;
    pub type pthread_t = ::core::ffi::c_ulong;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub union pthread_mutexattr_t {
        pub __size: [::core::ffi::c_char; 4],
        pub __align: ::core::ffi::c_int,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub union pthread_attr_t {
        pub __size: [::core::ffi::c_char; 56],
        pub __align: ::core::ffi::c_long,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub union pthread_mutex_t {
        pub __data: crate::stdlib::__pthread_mutex_s,
        pub __size: [::core::ffi::c_char; 40],
        pub __align: ::core::ffi::c_long,
    }
    pub type intptr_t = isize;

    pub type uintptr_t = usize;
    pub type int8_t = crate::stdlib::__int8_t;

    pub type int16_t = crate::stdlib::__int16_t;
    pub type uint8_t = crate::stdlib::__uint8_t;

    pub type uint16_t = crate::stdlib::__uint16_t;

    pub type uint32_t = crate::stdlib::__uint32_t;
    pub type va_list = crate::__stdarg___gnuc_va_list_h::__gnuc_va_list;

    pub type off_t = crate::stdlib::__off64_t;

    pub type off64_t = crate::stdlib::__off64_t;

    pub type ssize_t = crate::stdlib::__ssize_t;
    pub const FILENAME_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
    pub type __compar_fn_t = Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >;
    pub type _IO_lock_t = ();

    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]

    pub struct _IO_FILE {
        pub _flags: ::core::ffi::c_int,
        pub _IO_read_ptr: *mut ::core::ffi::c_char,
        pub _IO_read_end: *mut ::core::ffi::c_char,
        pub _IO_read_base: *mut ::core::ffi::c_char,
        pub _IO_write_base: *mut ::core::ffi::c_char,
        pub _IO_write_ptr: *mut ::core::ffi::c_char,
        pub _IO_write_end: *mut ::core::ffi::c_char,
        pub _IO_buf_base: *mut ::core::ffi::c_char,
        pub _IO_buf_end: *mut ::core::ffi::c_char,
        pub _IO_save_base: *mut ::core::ffi::c_char,
        pub _IO_backup_base: *mut ::core::ffi::c_char,
        pub _IO_save_end: *mut ::core::ffi::c_char,
        pub _markers: *mut crate::stdlib::_IO_marker,
        pub _chain: *mut crate::stdlib::_IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [::core::ffi::c_char; 1],
        pub _old_offset: crate::stdlib::__off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [::core::ffi::c_char; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: crate::stdlib::__off64_t,
        pub _codecvt: *mut crate::stdlib::_IO_codecvt,
        pub _wide_data: *mut crate::stdlib::_IO_wide_data,
        pub _freeres_list: *mut crate::stdlib::_IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut crate::stdlib::_IO_FILE,
        pub _mode: ::core::ffi::c_int,
        pub _unused2: [::core::ffi::c_char; 20],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __pthread_mutex_s {
        pub __lock: ::core::ffi::c_int,
        pub __count: ::core::ffi::c_uint,
        pub __owner: ::core::ffi::c_int,
        pub __nusers: ::core::ffi::c_uint,
        pub __kind: ::core::ffi::c_int,
        pub __spins: ::core::ffi::c_short,
        pub __elision: ::core::ffi::c_short,
        pub __list: crate::stdlib::__pthread_list_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct stat {
        pub st_dev: crate::stdlib::__dev_t,
        pub st_ino: crate::stdlib::__ino_t,
        pub st_nlink: crate::stdlib::__nlink_t,
        pub st_mode: crate::stdlib::__mode_t,
        pub st_uid: crate::stdlib::__uid_t,
        pub st_gid: crate::stdlib::__gid_t,
        pub __pad0: ::core::ffi::c_int,
        pub st_rdev: crate::stdlib::__dev_t,
        pub st_size: crate::stdlib::__off_t,
        pub st_blksize: crate::stdlib::__blksize_t,
        pub st_blocks: crate::stdlib::__blkcnt_t,
        pub st_atim: ::libc::timespec,
        pub st_mtim: ::libc::timespec,
        pub st_ctim: ::libc::timespec,
        pub __glibc_reserved: [crate::stdlib::__syscall_slong_t; 3],
    }
    pub type dev_t = crate::stdlib::__dev_t;

    pub type gid_t = crate::stdlib::__gid_t;

    pub type mode_t = crate::stdlib::__mode_t;

    pub type uid_t = crate::stdlib::__uid_t;

    pub type pid_t = crate::stdlib::__pid_t;

    #[cfg(feature = "test")]
    pub type Tcl_WideInt = ::core::ffi::c_longlong;

    #[cfg(feature = "test")]
    pub type Tcl_Size = crate::__stddef_ptrdiff_t_h::ptrdiff_t;

    #[cfg(feature = "test")]
    pub type Tcl_DupInternalRepProc =
        unsafe extern "C" fn(*mut crate::stdlib::Tcl_Obj, *mut crate::stdlib::Tcl_Obj) -> ();

    #[cfg(feature = "test")]

    pub type Tcl_FreeInternalRepProc = unsafe extern "C" fn(*mut crate::stdlib::Tcl_Obj) -> ();

    #[cfg(feature = "test")]
    pub type Tcl_SetFromAnyProc = unsafe extern "C" fn(
        *mut crate::stdlib::Tcl_Interp,
        *mut crate::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "test")]

    pub type Tcl_UpdateStringProc = unsafe extern "C" fn(*mut crate::stdlib::Tcl_Obj) -> ();

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeLengthProc =
        unsafe extern "C" fn(*mut crate::stdlib::Tcl_Obj) -> crate::stdlib::Tcl_Size;

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeIndexProc = unsafe extern "C" fn(
        *mut crate::stdlib::Tcl_Interp,
        *mut crate::stdlib::Tcl_Obj,
        crate::stdlib::Tcl_Size,
        *mut *mut crate::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeSliceProc = unsafe extern "C" fn(
        *mut crate::stdlib::Tcl_Interp,
        *mut crate::stdlib::Tcl_Obj,
        crate::stdlib::Tcl_Size,
        crate::stdlib::Tcl_Size,
        *mut *mut crate::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeReverseProc = unsafe extern "C" fn(
        *mut crate::stdlib::Tcl_Interp,
        *mut crate::stdlib::Tcl_Obj,
        *mut *mut crate::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeGetElements = unsafe extern "C" fn(
        *mut crate::stdlib::Tcl_Interp,
        *mut crate::stdlib::Tcl_Obj,
        *mut crate::stdlib::Tcl_Size,
        *mut *mut *mut crate::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeSetElement = unsafe extern "C" fn(
        *mut crate::stdlib::Tcl_Interp,
        *mut crate::stdlib::Tcl_Obj,
        crate::stdlib::Tcl_Size,
        *const *mut crate::stdlib::Tcl_Obj,
        *mut crate::stdlib::Tcl_Obj,
    ) -> *mut crate::stdlib::Tcl_Obj;

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeReplaceProc = unsafe extern "C" fn(
        *mut crate::stdlib::Tcl_Interp,
        *mut crate::stdlib::Tcl_Obj,
        crate::stdlib::Tcl_Size,
        crate::stdlib::Tcl_Size,
        crate::stdlib::Tcl_Size,
        *const *mut crate::stdlib::Tcl_Obj,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "test")]
    pub type Tcl_ObjTypeInOperatorProc = unsafe extern "C" fn(
        *mut crate::stdlib::Tcl_Interp,
        *mut crate::stdlib::Tcl_Obj,
        *mut crate::stdlib::Tcl_Obj,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    #[cfg(feature = "test")]
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct Tcl_ObjType {
        pub name: *const ::core::ffi::c_char,
        pub freeIntRepProc: Option<crate::stdlib::Tcl_FreeInternalRepProc>,
        pub dupIntRepProc: Option<crate::stdlib::Tcl_DupInternalRepProc>,
        pub updateStringProc: Option<crate::stdlib::Tcl_UpdateStringProc>,
        pub setFromAnyProc: Option<crate::stdlib::Tcl_SetFromAnyProc>,
        pub version: crate::__stddef_size_t_h::size_t,
        pub lengthProc: Option<crate::stdlib::Tcl_ObjTypeLengthProc>,
        pub indexProc: Option<crate::stdlib::Tcl_ObjTypeIndexProc>,
        pub sliceProc: Option<crate::stdlib::Tcl_ObjTypeSliceProc>,
        pub reverseProc: Option<crate::stdlib::Tcl_ObjTypeReverseProc>,
        pub getElementsProc: Option<crate::stdlib::Tcl_ObjTypeGetElements>,
        pub setElementProc: Option<crate::stdlib::Tcl_ObjTypeSetElement>,
        pub replaceProc: Option<crate::stdlib::Tcl_ObjTypeReplaceProc>,
        pub inOperProc: Option<crate::stdlib::Tcl_ObjTypeInOperatorProc>,
    }
    #[cfg(feature = "test")]
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub union Tcl_ObjInternalRep {
        pub longValue: ::core::ffi::c_long,
        pub doubleValue: ::core::ffi::c_double,
        pub otherValuePtr: *mut ::core::ffi::c_void,
        pub wideValue: crate::stdlib::Tcl_WideInt,
        pub twoPtrValue: crate::stdlib::C2RustUnnamed_0,
        pub ptrAndLongRep: crate::stdlib::C2RustUnnamed,
    }
    #[cfg(feature = "test")]
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct Tcl_Obj {
        pub refCount: crate::stdlib::Tcl_Size,
        pub bytes: *mut ::core::ffi::c_char,
        pub length: crate::stdlib::Tcl_Size,
        pub typePtr: *const crate::stdlib::Tcl_ObjType,
        pub internalRep: crate::stdlib::Tcl_ObjInternalRep,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct C2RustUnnamed_0 {
        pub ptr1: *mut ::core::ffi::c_void,
        pub ptr2: *mut ::core::ffi::c_void,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct C2RustUnnamed {
        pub ptr: *mut ::core::ffi::c_void,
        pub value: ::core::ffi::c_ulong,
    }

    pub type __pthread_list_t = crate::stdlib::__pthread_internal_list;

    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct __pthread_internal_list {
        pub __prev: *mut crate::stdlib::__pthread_internal_list,
        pub __next: *mut crate::stdlib::__pthread_internal_list,
    }
    pub type time_t = crate::stdlib::__time_t;
    pub type __int8_t = i8;

    pub type __uint8_t = u8;

    pub type __int16_t = i16;

    pub type __uint16_t = u16;

    pub type __uint32_t = u32;

    pub type __dev_t = ::core::ffi::c_ulong;

    pub type __uid_t = ::core::ffi::c_uint;

    pub type __gid_t = ::core::ffi::c_uint;

    pub type __ino_t = ::core::ffi::c_ulong;

    pub type __mode_t = ::core::ffi::c_uint;

    pub type __nlink_t = ::core::ffi::c_ulong;

    pub type __off_t = ::core::ffi::c_long;

    pub type __off64_t = ::core::ffi::c_long;

    pub type __pid_t = ::core::ffi::c_int;

    pub type __time_t = ::core::ffi::c_long;

    pub type __suseconds_t = ::core::ffi::c_long;

    pub type __blksize_t = ::core::ffi::c_long;

    pub type __blkcnt_t = ::core::ffi::c_long;

    pub type __ssize_t = ::core::ffi::c_long;

    pub type __syscall_slong_t = ::core::ffi::c_long;
}
#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod src {
    pub mod ctime;
    pub mod headers;
    pub mod ext {
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
