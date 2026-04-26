#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(clippy::no_effect)]
#![allow(clippy::manual_c_str_literals)]
#![allow(clippy::unnecessary_operation)]
#![allow(clippy::useless_vec)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::unnecessary_literal_unwrap)]
#![allow(clippy::needless_ifs)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::duplicated_attributes)]
#![allow(clippy::deref_addrof)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_parens)]
// Note: crust-core is generated code from C2Rust. Many clippy warnings and
// compiler warnings are suppressed to keep it maintainable. Hand-written code
// in other crates is subject to stricter lint rules.

// Re-export sqlite_printf!, sqlite_snprintf! and json_printf! macros from proc-macro crate
pub use sqlite_printf_macros::json_printf;
pub use sqlite_printf_macros::sqlite_printf;
pub use sqlite_printf_macros::sqlite_snprintf;
pub use sqlite_printf_macros::sqlite_vmprintf;
pub use sqlite_printf_macros::sqlite_vsnprintf;

/// Platform compatibility shims for macOS (the C2Rust output targets Linux).
pub(crate) mod compat {
    #[cfg(target_os = "macos")]
    #[inline(always)]
    pub unsafe fn errno_location() -> *mut ::core::ffi::c_int {
        unsafe extern "C" {
            fn __error() -> *mut ::core::ffi::c_int;
        }
        __error()
    }
    #[cfg(not(target_os = "macos"))]
    #[inline(always)]
    pub unsafe fn errno_location() -> *mut ::core::ffi::c_int {
        ::libc::__errno_location()
    }

    #[cfg(target_os = "macos")]
    pub const O_LARGEFILE: ::core::ffi::c_int = 0;
    #[cfg(not(target_os = "macos"))]
    pub use ::libc::O_LARGEFILE;

    #[cfg(target_os = "macos")]
    pub const MREMAP_MAYMOVE: ::core::ffi::c_int = 1;
    #[cfg(not(target_os = "macos"))]
    pub use ::libc::MREMAP_MAYMOVE;

    #[cfg(target_os = "macos")]
    pub unsafe extern "C" fn mremap(
        old_address: *mut ::core::ffi::c_void,
        old_size: usize,
        new_size: usize,
        _flags: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void {
        let new_ptr = ::libc::mmap(
            ::core::ptr::null_mut(),
            new_size,
            ::libc::PROT_READ | ::libc::PROT_WRITE,
            ::libc::MAP_SHARED,
            -1,
            0,
        );
        if new_ptr == ::libc::MAP_FAILED {
            return ::libc::MAP_FAILED;
        }
        let copy_size = if old_size < new_size {
            old_size
        } else {
            new_size
        };
        ::core::ptr::copy_nonoverlapping(old_address as *const u8, new_ptr as *mut u8, copy_size);
        ::libc::munmap(old_address, old_size);
        new_ptr
    }

    #[cfg(target_os = "macos")]
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn pread64(
        fd: ::core::ffi::c_int,
        buf: *mut ::core::ffi::c_void,
        count: usize,
        offset: i64,
    ) -> isize {
        ::libc::pread(fd, buf, count, offset as ::libc::OffT)
    }
    #[cfg(target_os = "macos")]
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn pwrite64(
        fd: ::core::ffi::c_int,
        buf: *const ::core::ffi::c_void,
        count: usize,
        offset: i64,
    ) -> isize {
        ::libc::pwrite(fd, buf, count, offset as ::libc::OffT)
    }

    #[cfg(target_os = "macos")]
    mod ctype_compat {
        use core::sync::atomic::{AtomicBool, Ordering};
        const TABLE_SIZE: usize = 384;
        const OFFSET: usize = 128;
        static mut TABLE: [::core::ffi::c_ushort; TABLE_SIZE] = [0u16; TABLE_SIZE];
        static mut TABLE_PTR: *const ::core::ffi::c_ushort = ::core::ptr::null();
        static INIT: AtomicBool = AtomicBool::new(false);

        const _ISUPPER: u16 = 256;
        const _ISLOWER: u16 = 512;
        const _ISALPHA: u16 = 1024;
        const _ISDIGIT: u16 = 2048;
        const _ISXDIGIT: u16 = 4096;
        const _ISSPACE: u16 = 8192;
        const _ISPRINT: u16 = 16384;
        const _ISALNUM: u16 = _ISALPHA | _ISDIGIT;

        unsafe fn init_table() {
            for i in 0..TABLE_SIZE {
                let c = (i as isize - OFFSET as isize) as ::core::ffi::c_int;
                if c >= 0 && c <= 127 {
                    let mut flags: u16 = 0;
                    if ::libc::isupper(c) != 0 {
                        flags |= _ISUPPER;
                    }
                    if ::libc::islower(c) != 0 {
                        flags |= _ISLOWER;
                    }
                    if ::libc::isalpha(c) != 0 {
                        flags |= _ISALPHA;
                    }
                    if ::libc::isdigit(c) != 0 {
                        flags |= _ISDIGIT;
                    }
                    if ::libc::isxdigit(c) != 0 {
                        flags |= _ISXDIGIT;
                    }
                    if ::libc::isspace(c) != 0 {
                        flags |= _ISSPACE;
                    }
                    if ::libc::isprint(c) != 0 {
                        flags |= _ISPRINT;
                    }
                    if ::libc::isalnum(c) != 0 {
                        flags |= _ISALNUM;
                    }
                    TABLE[i] = flags;
                }
            }
            TABLE_PTR = TABLE.as_ptr().add(OFFSET);
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort {
            if !INIT.swap(true, Ordering::SeqCst) {
                init_table();
            }
            &raw mut TABLE_PTR
        }
    }
}

pub(crate) mod pcache_h {
    pub use crate::src::src::pcache::PCache;
}
pub(crate) mod __stddef_ptrdiff_t_h {
    pub type PtrdiffT = isize;
}
pub(crate) mod __stdarg_GnucVaList_h {
    pub type GnucVaList = crate::internal::BuiltinVaList;
}
pub(crate) mod __stddef_size_t_h {
    pub type SizeT = usize;
}
pub(crate) mod keywordhash_h {
    pub const SQLITE_N_KEYWORD: ::core::ffi::c_int = 147 as ::core::ffi::c_int;
}
pub(crate) mod vxworks_h {
    pub const OS_VXWORKS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
pub(crate) mod sqliteLimit_h {
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
pub(crate) mod internal {
    pub type BuiltinVaList = [crate::internal::VaListTag; 1];

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct VaListTag {
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

pub(crate) mod geopoly_c {
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

    pub const GEOPOLY_PI: ::core::ffi::c_double = 3.141_592_653_589_793_f64;

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
    pub struct C2RustUnnamed1 {
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
pub(crate) mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

    pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
}
pub(crate) mod fts3Int_h {
    #[cfg(feature = "fts3")]
    #[cfg(feature = "test")]
    unsafe extern "C" {
        #[link_name = "sqlite3Fts3InitTerm"]
        fn sqlite3Fts3InitTerm_c(db: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    }
    #[cfg(feature = "fts3")]
    #[cfg(feature = "test")]
    pub unsafe fn sqlite3Fts3InitTerm(
        db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    ) -> ::core::ffi::c_int {
        unsafe { sqlite3Fts3InitTerm_c(db as *mut ::core::ffi::c_void) }
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
    pub const LARGEST_INT64: crate::src::ext::rtree::rtree::I64_0 = 0xffffffff
        as crate::src::ext::rtree::rtree::I64_0
        | (0x7fffffff as ::core::ffi::c_int as crate::src::ext::rtree::rtree::I64_0)
            << 32 as ::core::ffi::c_int;

    pub const SMALLEST_INT64: crate::src::ext::rtree::rtree::I64_0 = -(1 as ::core::ffi::c_int)
        as crate::src::ext::rtree::rtree::I64_0
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
        pub abNotindexed: *mut crate::src::ext::rtree::rtree::U8_0,
        pub pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
        pub zContentTbl: *mut ::core::ffi::c_char,
        pub zLanguageid: *mut ::core::ffi::c_char,
        pub nAutoincrmerge: ::core::ffi::c_int,
        pub nLeafAdd: crate::src::ext::rtree::rtree::U32_0,
        pub bLock: ::core::ffi::c_int,
        pub aStmt: [*mut crate::src::headers::sqlite3_h::Sqlite3Stmt; 40],
        pub pSeekStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        pub zReadExprlist: *mut ::core::ffi::c_char,
        pub zWriteExprlist: *mut ::core::ffi::c_char,
        pub nNodeSize: ::core::ffi::c_int,
        pub bFts4: crate::src::ext::rtree::rtree::U8_0,
        pub bHasStat: crate::src::ext::rtree::rtree::U8_0,
        pub bHasDocsize: crate::src::ext::rtree::rtree::U8_0,
        pub bDescIdx: crate::src::ext::rtree::rtree::U8_0,
        pub bIgnoreSavepoint: crate::src::ext::rtree::rtree::U8_0,
        pub nPgsz: ::core::ffi::c_int,
        pub zSegmentsTbl: *mut ::core::ffi::c_char,
        pub pSegments: *mut crate::src::headers::sqlite3_h::Sqlite3Blob,
        pub iSavepoint: ::core::ffi::c_int,
        pub nIndex: ::core::ffi::c_int,
        pub aIndex: *mut crate::fts3Int_h::Fts3Index,
        pub nMaxPendingData: ::core::ffi::c_int,
        pub nPendingData: ::core::ffi::c_int,
        pub iPrevDocid: crate::src::headers::sqlite3_h::SqliteInt64,
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
        pub eSearch: crate::src::fts5::I16_0,
        pub isEof: crate::src::ext::rtree::rtree::U8_0,
        pub isRequireSeek: crate::src::ext::rtree::rtree::U8_0,
        pub bSeekStmt: crate::src::ext::rtree::rtree::U8_0,
        pub pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
        pub pExpr: *mut crate::fts3Int_h::Fts3Expr,
        pub iLangid: ::core::ffi::c_int,
        pub nPhrase: ::core::ffi::c_int,
        pub pDeferred: *mut crate::fts3Int_h::Fts3DeferredToken,
        pub iPrevId: crate::src::headers::sqlite3_h::Sqlite3Int64,
        pub pNextId: *mut ::core::ffi::c_char,
        pub aDoclist: *mut ::core::ffi::c_char,
        pub nDoclist: ::core::ffi::c_int,
        pub bDesc: crate::src::ext::rtree::rtree::U8_0,
        pub eEvalmode: ::core::ffi::c_int,
        pub nRowAvg: ::core::ffi::c_int,
        pub nDoc: crate::src::headers::sqlite3_h::Sqlite3Int64,
        pub iMinDocid: crate::src::ext::rtree::rtree::I64_0,
        pub iMaxDocid: crate::src::ext::rtree::rtree::I64_0,
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
        pub iDocid: crate::src::headers::sqlite3_h::Sqlite3Int64,
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
        pub iOrDocid: crate::src::ext::rtree::rtree::I64_0,
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
        pub iDocid: crate::src::headers::sqlite3_h::Sqlite3Int64,
        pub bEof: crate::src::ext::rtree::rtree::U8_0,
        pub bStart: crate::src::ext::rtree::rtree::U8_0,
        pub bDeferred: crate::src::ext::rtree::rtree::U8_0,
        pub iPhrase: ::core::ffi::c_int,
        pub aMI: *mut crate::src::ext::rtree::rtree::U32_0,
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
        pub nBuffer: crate::src::ext::rtree::rtree::I64_0,
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

pub(crate) mod src {
    pub mod ctime;
    pub mod headers;
    pub mod ext {
        #[cfg(feature = "fts3")]
        pub mod fts3 {
            #[path = "fts3.rs"]
            pub mod core;
            pub use core as fts3;
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
            #[path = "icu.rs"]
            pub mod core;
            pub use core as icu;
        } // mod icu
        pub mod misc {
            pub mod stmt;
        } // mod misc
        pub mod rbu {
            pub mod sqlite3rbu;
        } // mod rbu
        pub mod rtree {
            #[path = "rtree.rs"]
            pub mod core;
            pub use core as rtree;
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
    #[path = "src"]
    pub mod core {
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
        pub mod main;
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
        pub mod r#where;
        pub mod wherecode;
        pub mod whereexpr;
        pub mod window;
    } // mod core
    pub use core as src;
} // mod src

// ── Public API re-exports (stable C API, matching the rslite-raw surface) ────

pub use crate::src::headers::sqlite3_h::Sqlite3Stmt;
pub use crate::src::headers::vdbeInt_h::sqlite3_value;
pub use crate::src::headers::vdbeInt_h::sqlite3_context;
pub use crate::src::headers::sqlite3_h::sqlite3_vfs;
pub use crate::src::headers::sqlite3_h::sqlite3_file;
pub use crate::src::headers::sqlite3_h::Sqlite3Filename;
pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;
pub use crate::src::headers::sqlite3_h::sqlite3_module;
pub use crate::src::headers::sqlite3_h::sqlite3_vtab;
pub use crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;
pub use crate::src::headers::sqlite3_h::sqlite3_index_info;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;
pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;
pub use crate::src::headers::sqlite3_h::sqlite3_mem_methods;
pub use crate::src::headers::sqlite3_h::Sqlite3Pcache;
pub use crate::src::headers::sqlite3_h::sqlite3_pcache_methods2;
pub use crate::src::headers::sqlite3_h::sqlite3_pcache_page;
pub use crate::src::headers::sqlite3_h::sqlite3_mutex_methods;
pub use crate::src::headers::sqlite3_h::Sqlite3Int64;
pub use crate::src::headers::sqlite3_h::Sqlite3Uint64;
pub use crate::src::headers::sqlite3_h::Sqlite3DestructorType;
pub use crate::src::headers::sqlite3_h::Sqlite3SyscallPtr;
pub use crate::src::headers::sqliteInt_h::sqlite3;
pub use crate::src::src::mutex_unix::sqlite3_mutex;
pub use crate::src::src::backup::sqlite3_backup;

pub use crate::src::headers::sqlite3_h::{
    SQLITE_OK, SQLITE_ERROR, SQLITE_INTERNAL, SQLITE_PERM, SQLITE_ABORT, SQLITE_BUSY,
    SQLITE_LOCKED, SQLITE_NOMEM, SQLITE_READONLY, SQLITE_INTERRUPT, SQLITE_IOERR,
    SQLITE_CORRUPT, SQLITE_NOTFOUND, SQLITE_FULL, SQLITE_CANTOPEN, SQLITE_PROTOCOL,
    SQLITE_EMPTY, SQLITE_SCHEMA, SQLITE_TOOBIG, SQLITE_CONSTRAINT, SQLITE_MISMATCH,
    SQLITE_MISUSE, SQLITE_NOLFS, SQLITE_AUTH, SQLITE_FORMAT, SQLITE_RANGE,
    SQLITE_NOTADB, SQLITE_NOTICE, SQLITE_WARNING, SQLITE_ROW, SQLITE_DONE,
    SQLITE_ABORT_ROLLBACK, SQLITE_BUSY_RECOVERY, SQLITE_BUSY_SNAPSHOT,
    SQLITE_CANTOPEN_CONVPATH, SQLITE_CANTOPEN_FULLPATH, SQLITE_CANTOPEN_ISDIR,
    SQLITE_CANTOPEN_NOTEMPDIR, SQLITE_CANTOPEN_SYMLINK,
    SQLITE_CONSTRAINT_CHECK, SQLITE_CONSTRAINT_COMMITHOOK, SQLITE_CONSTRAINT_FOREIGNKEY,
    SQLITE_CONSTRAINT_FUNCTION, SQLITE_CONSTRAINT_NOTNULL, SQLITE_CONSTRAINT_PRIMARYKEY,
    SQLITE_CONSTRAINT_ROWID, SQLITE_CONSTRAINT_TRIGGER, SQLITE_CONSTRAINT_UNIQUE,
    SQLITE_CONSTRAINT_VTAB, SQLITE_CORRUPT_VTAB, SQLITE_ERROR_MISSING_COLLSEQ,
    SQLITE_ERROR_RETRY, SQLITE_ERROR_SNAPSHOT, SQLITE_IOERR_ACCESS,
    SQLITE_IOERR_CHECKRESERVEDLOCK, SQLITE_IOERR_CLOSE, SQLITE_IOERR_CONVPATH,
    SQLITE_IOERR_DELETE, SQLITE_IOERR_DELETE_NOENT, SQLITE_IOERR_DIR_CLOSE,
    SQLITE_IOERR_DIR_FSYNC, SQLITE_IOERR_FSTAT, SQLITE_IOERR_FSYNC,
    SQLITE_IOERR_GETTEMPPATH, SQLITE_IOERR_LOCK, SQLITE_IOERR_MMAP, SQLITE_IOERR_NOMEM,
    SQLITE_IOERR_RDLOCK, SQLITE_IOERR_READ, SQLITE_IOERR_SEEK, SQLITE_IOERR_SHMLOCK,
    SQLITE_IOERR_SHMMAP, SQLITE_IOERR_SHMOPEN, SQLITE_IOERR_SHMSIZE,
    SQLITE_IOERR_SHORT_READ, SQLITE_IOERR_TRUNCATE, SQLITE_IOERR_UNLOCK,
    SQLITE_IOERR_WRITE, SQLITE_LOCKED_SHAREDCACHE, SQLITE_NOTICE_RBU,
    SQLITE_NOTICE_RECOVER_ROLLBACK, SQLITE_NOTICE_RECOVER_WAL,
    SQLITE_READONLY_CANTINIT, SQLITE_READONLY_DBMOVED, SQLITE_READONLY_DIRECTORY,
    SQLITE_READONLY_RECOVERY, SQLITE_READONLY_ROLLBACK, SQLITE_WARNING_AUTOINDEX,
    SQLITE_OPEN_READONLY, SQLITE_OPEN_READWRITE, SQLITE_OPEN_CREATE,
    SQLITE_OPEN_DELETEONCLOSE, SQLITE_OPEN_EXCLUSIVE, SQLITE_OPEN_URI,
    SQLITE_OPEN_MEMORY, SQLITE_OPEN_MAIN_DB, SQLITE_OPEN_TEMP_DB,
    SQLITE_OPEN_TRANSIENT_DB, SQLITE_OPEN_MAIN_JOURNAL, SQLITE_OPEN_TEMP_JOURNAL,
    SQLITE_OPEN_SUBJOURNAL, SQLITE_OPEN_SUPER_JOURNAL, SQLITE_OPEN_NOMUTEX,
    SQLITE_OPEN_FULLMUTEX, SQLITE_OPEN_SHAREDCACHE, SQLITE_OPEN_PRIVATECACHE,
    SQLITE_OPEN_WAL, SQLITE_OPEN_NOFOLLOW, SQLITE_OPEN_EXRESCODE,
    SQLITE_UTF8, SQLITE_UTF16LE, SQLITE_UTF16BE, SQLITE_UTF16, SQLITE_UTF16_ALIGNED,
    SQLITE_DETERMINISTIC, SQLITE_DIRECTONLY, SQLITE_SUBTYPE, SQLITE_INNOCUOUS,
    SQLITE_RESULT_SUBTYPE, SQLITE_SELFORDER1, SQLITE_STATIC,
    SQLITE_CHECKPOINT_PASSIVE, SQLITE_CHECKPOINT_FULL, SQLITE_CHECKPOINT_RESTART,
    SQLITE_CHECKPOINT_TRUNCATE, SQLITE_TRACE_STMT, SQLITE_TRACE_PROFILE,
    SQLITE_TRACE_ROW, SQLITE_TRACE_CLOSE, SQLITE_TXN_NONE, SQLITE_TXN_WRITE,
    SQLITE_MUTEX_FAST, SQLITE_MUTEX_RECURSIVE,
    SQLITE_FCNTL_DATA_VERSION, SQLITE_FCNTL_FILE_POINTER, SQLITE_FCNTL_JOURNAL_POINTER,
    SQLITE_FCNTL_RESERVE_BYTES, SQLITE_FCNTL_RESET_CACHE, SQLITE_FCNTL_VFS_POINTER,
    SQLITE_INTEGER, SQLITE_FLOAT, SQLITE_TEXT, SQLITE_BLOB, SQLITE_NULL,
    SQLITE_VERSION, SQLITE_VERSION_NUMBER, SQLITE_SOURCE_ID,
};

pub use crate::src::src::main::sqlite3_version;
pub use crate::src::src::main::{
    sqlite3_open, sqlite3_open_v2, sqlite3_open16, sqlite3_close, sqlite3_close_v2,
    sqlite3_initialize, sqlite3_shutdown, sqlite3_libversion, sqlite3_libversion_number,
    sqlite3_sourceid, sqlite3_threadsafe, sqlite3_db_mutex, sqlite3_db_release_memory,
    sqlite3_db_cacheflush, sqlite3_db_name, sqlite3_db_filename, sqlite3_db_readonly,
    sqlite3_txn_state, sqlite3_get_autocommit, sqlite3_extended_result_codes,
    sqlite3_limit, sqlite3_sleep, sqlite3_file_control, sqlite3_table_column_metadata,
    sqlite3_get_clientdata, sqlite3_set_clientdata,
    sqlite3_errmsg, sqlite3_errmsg16, sqlite3_errcode, sqlite3_extended_errcode,
    sqlite3_errstr, sqlite3_system_errno, sqlite3_error_offset, sqlite3_set_errmsg,
    sqlite3_changes, sqlite3_changes64, sqlite3_total_changes, sqlite3_total_changes64,
    sqlite3_last_insert_rowid, sqlite3_set_last_insert_rowid,
    sqlite3_busy_handler, sqlite3_busy_timeout, sqlite3_setlk_timeout,
    sqlite3_progress_handler, sqlite3_interrupt, sqlite3_is_interrupted,
    sqlite3_commit_hook, sqlite3_rollback_hook, sqlite3_update_hook,
    sqlite3_preupdate_hook, sqlite3_autovacuum_pages, sqlite3_trace,
    sqlite3_trace_v2, sqlite3_profile, sqlite3_wal_hook, sqlite3_wal_autocheckpoint,
    sqlite3_wal_checkpoint, sqlite3_wal_checkpoint_v2,
    sqlite3_create_function, sqlite3_create_function_v2, sqlite3_create_function16,
    sqlite3_create_window_function, sqlite3_overload_function,
    sqlite3_create_collation, sqlite3_create_collation_v2, sqlite3_create_collation16,
    sqlite3_collation_needed, sqlite3_collation_needed16,
    sqlite3_create_filename, sqlite3_free_filename, sqlite3_uri_parameter,
    sqlite3_uri_key, sqlite3_uri_boolean, sqlite3_uri_int64,
    sqlite3_filename_database, sqlite3_filename_journal, sqlite3_filename_wal,
    sqlite3_compileoption_used, sqlite3_compileoption_get,
    sqlite3_global_recover, sqlite3_thread_cleanup,
};
pub use crate::src::src::auth::sqlite3_set_authorizer;
pub use crate::src::src::prepare::{
    sqlite3_prepare, sqlite3_prepare_v2, sqlite3_prepare_v3,
    sqlite3_prepare16, sqlite3_prepare16_v2, sqlite3_prepare16_v3,
};
pub use crate::src::src::vdbeapi::{
    sqlite3_step, sqlite3_finalize, sqlite3_reset, sqlite3_clear_bindings,
    sqlite3_expired,
    sqlite3_bind_blob, sqlite3_bind_blob64, sqlite3_bind_double, sqlite3_bind_int,
    sqlite3_bind_int64, sqlite3_bind_null, sqlite3_bind_pointer, sqlite3_bind_text,
    sqlite3_bind_text64, sqlite3_bind_text16, sqlite3_bind_value, sqlite3_bind_zeroblob,
    sqlite3_bind_zeroblob64, sqlite3_bind_parameter_count, sqlite3_bind_parameter_name,
    sqlite3_bind_parameter_index, sqlite3_transfer_bindings,
    sqlite3_column_count, sqlite3_data_count, sqlite3_column_name, sqlite3_column_name16,
    sqlite3_column_decltype, sqlite3_column_decltype16, sqlite3_column_blob,
    sqlite3_column_bytes, sqlite3_column_bytes16, sqlite3_column_double,
    sqlite3_column_int, sqlite3_column_int64, sqlite3_column_text, sqlite3_column_text16,
    sqlite3_column_type, sqlite3_column_value,
    sqlite3_stmt_readonly, sqlite3_stmt_isexplain, sqlite3_stmt_explain,
    sqlite3_stmt_busy, sqlite3_stmt_status, sqlite3_next_stmt, sqlite3_sql,
    sqlite3_expanded_sql, sqlite3_db_handle,
    sqlite3_value_blob, sqlite3_value_bytes, sqlite3_value_bytes16,
    sqlite3_value_double, sqlite3_value_int, sqlite3_value_int64, sqlite3_value_subtype,
    sqlite3_value_pointer, sqlite3_value_text, sqlite3_value_text16,
    sqlite3_value_text16be, sqlite3_value_text16le, sqlite3_value_type,
    sqlite3_value_encoding, sqlite3_value_nochange, sqlite3_value_frombind,
    sqlite3_value_dup, sqlite3_value_free,
    sqlite3_result_blob, sqlite3_result_blob64, sqlite3_result_double,
    sqlite3_result_error, sqlite3_result_error16, sqlite3_result_error_code,
    sqlite3_result_error_nomem, sqlite3_result_error_toobig, sqlite3_result_int,
    sqlite3_result_int64, sqlite3_result_null, sqlite3_result_pointer,
    sqlite3_result_subtype, sqlite3_result_text, sqlite3_result_text64,
    sqlite3_result_text16, sqlite3_result_text16be, sqlite3_result_text16le,
    sqlite3_result_value, sqlite3_result_zeroblob, sqlite3_result_zeroblob64,
    sqlite3_user_data, sqlite3_context_db_handle, sqlite3_aggregate_context,
    sqlite3_aggregate_count, sqlite3_get_auxdata, sqlite3_set_auxdata,
    sqlite3_vtab_nochange, sqlite3_vtab_in_first, sqlite3_vtab_in_next,
    sqlite3_preupdate_old, sqlite3_preupdate_new, sqlite3_preupdate_count,
    sqlite3_preupdate_depth, sqlite3_preupdate_blobwrite,
};
pub use crate::src::src::malloc::{
    sqlite3_malloc, sqlite3_malloc64, sqlite3_realloc, sqlite3_realloc64, sqlite3_free,
    sqlite3_msize, sqlite3_release_memory, sqlite3_memory_used, sqlite3_memory_highwater,
    sqlite3_soft_heap_limit64, sqlite3_soft_heap_limit, sqlite3_hard_heap_limit64,
    sqlite3_memory_alarm,
};
pub use crate::src::src::mutex::{
    sqlite3_mutex_alloc, sqlite3_mutex_free, sqlite3_mutex_enter, sqlite3_mutex_try,
    sqlite3_mutex_leave,
};
pub use crate::src::src::backup::{
    sqlite3_backup_init, sqlite3_backup_step, sqlite3_backup_finish,
    sqlite3_backup_remaining, sqlite3_backup_pagecount,
};
pub use crate::src::src::vdbeblob::{
    sqlite3_blob_open, sqlite3_blob_close, sqlite3_blob_read, sqlite3_blob_write,
    sqlite3_blob_bytes, sqlite3_blob_reopen,
};
pub use crate::src::src::memdb::{sqlite3_serialize, sqlite3_deserialize};
pub use crate::src::src::os::{
    sqlite3_vfs_find, sqlite3_vfs_register, sqlite3_vfs_unregister,
};
pub use crate::src::src::os_unix::{sqlite3_os_init, sqlite3_os_end};
pub use crate::src::src::loadext::{
    sqlite3_load_extension, sqlite3_enable_load_extension, sqlite3_auto_extension,
    sqlite3_cancel_auto_extension, sqlite3_reset_auto_extension,
};
pub use crate::src::src::vtab::{
    sqlite3_create_module, sqlite3_create_module_v2, sqlite3_drop_modules,
    sqlite3_declare_vtab, sqlite3_vtab_on_conflict,
};
pub use crate::src::src::r#where::{
    sqlite3_vtab_collation, sqlite3_vtab_in, sqlite3_vtab_rhs_value, sqlite3_vtab_distinct,
};
pub use crate::src::src::legacy::sqlite3_exec;
pub use crate::src::src::table::{sqlite3_get_table, sqlite3_free_table};
pub use crate::src::src::status::{
    sqlite3_status, sqlite3_status64, sqlite3_db_status, sqlite3_db_status64,
};
pub use crate::src::src::util::{sqlite3_stricmp, sqlite3_strnicmp};
pub use crate::src::src::func::{sqlite3_strglob, sqlite3_strlike};
pub use crate::src::src::printf::{
    sqlite3_str_new, sqlite3_str_finish, sqlite3_str_append, sqlite3_str_appendall,
    sqlite3_str_appendchar, sqlite3_str_errcode, sqlite3_str_length, sqlite3_str_value,
    sqlite3_str_reset,
    PrintfArg, sqlite3_vmprintf_args, sqlite3_snprintf_args, sqlite3_str_vappendf_args,
};
pub use crate::src::printf_c_variadic::sqlite3_log_args;
pub use crate::src::printf_c_variadic::{
    ConfigOp, rs_config_dispatch as sqlite3_config_args,
};
pub use crate::src::printf_c_variadic::rs_db_config_dispatch as sqlite3_db_config_args;
pub use crate::src::printf_c_variadic::{
    VtabConfigOp, rs_vtab_config_dispatch as sqlite3_vtab_config_args,
};
pub use crate::src::printf_c_variadic::{
    TestControlOp, rs_test_control_dispatch as sqlite3_test_control_args,
};
pub use crate::src::src::complete::{sqlite3_complete, sqlite3_complete16};
pub use crate::src::src::random::sqlite3_randomness;
pub use crate::src::src::tokenize::{
    sqlite3_keyword_name, sqlite3_keyword_count, sqlite3_keyword_check,
};
pub use crate::src::src::btree::sqlite3_enable_shared_cache;
pub use crate::src::src::pager::sqlite3_database_file_object;
pub use crate::src::src::carray::sqlite3_carray_bind;
pub use crate::src::src::vdbe::sqlite3_value_numeric_type;
pub use crate::src::ext::rtree::rtree::{
    sqlite3_geopoly_init, sqlite3_rtree_geometry_callback, sqlite3_rtree_query_callback,
};
