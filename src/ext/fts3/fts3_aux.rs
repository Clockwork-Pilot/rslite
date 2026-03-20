
extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_blob;
    pub type Fts3SegReader;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_create_module(
        db: *mut sqlite3,
        zName: *const ::core::ffi::c_char,
        p: *const sqlite3_module,
        pClientData: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_declare_vtab(
        _: *mut sqlite3,
        zSQL: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3SegmentsClose(_: *mut Fts3Table);
    fn sqlite3Fts3SegReaderStart(
        _: *mut Fts3Table,
        _: *mut Fts3MultiSegReader,
        _: *mut Fts3SegFilter,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3SegReaderStep(
        _: *mut Fts3Table,
        _: *mut Fts3MultiSegReader,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3SegReaderFinish(_: *mut Fts3MultiSegReader);
    fn sqlite3Fts3SegReaderCursor(
        _: *mut Fts3Table,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut Fts3MultiSegReader,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3ErrMsg(_: *mut *mut ::core::ffi::c_char, _: *const ::core::ffi::c_char, ...);
    fn sqlite3Fts3GetVarint(
        _: *const ::core::ffi::c_char,
        _: *mut sqlite_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3Dequote(_: *mut ::core::ffi::c_char);
}
pub type size_t = usize;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab {
    pub pModule: *const sqlite3_module,
    pub nRef: ::core::ffi::c_int,
    pub zErrMsg: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_module {
    pub iVersion: ::core::ffi::c_int,
    pub xCreate: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xConnect: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xBestIndex: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, *mut sqlite3_index_info) -> ::core::ffi::c_int,
    >,
    pub xDisconnect: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *mut *mut sqlite3_vtab_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int>,
    pub xFilter: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub xNext: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int>,
    pub xEof: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int>,
    pub xColumn: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            *mut sqlite3_context,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xRowid: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor, *mut sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub xUpdate: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
            *mut sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xBegin: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xSync: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xCommit: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xRollback: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xFindFunction: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *mut Option<
                unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
            >,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xRename: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, *const ::core::ffi::c_char) -> ::core::ffi::c_int,
    >,
    pub xSavepoint:
        Option<unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xRelease:
        Option<unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xRollbackTo:
        Option<unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xShadowName: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub xIntegrity: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab_cursor {
    pub pVtab: *mut sqlite3_vtab,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_info {
    pub nConstraint: ::core::ffi::c_int,
    pub aConstraint: *mut sqlite3_index_constraint,
    pub nOrderBy: ::core::ffi::c_int,
    pub aOrderBy: *mut sqlite3_index_orderby,
    pub aConstraintUsage: *mut sqlite3_index_constraint_usage,
    pub idxNum: ::core::ffi::c_int,
    pub idxStr: *mut ::core::ffi::c_char,
    pub needToFreeIdxStr: ::core::ffi::c_int,
    pub orderByConsumed: ::core::ffi::c_int,
    pub estimatedCost: ::core::ffi::c_double,
    pub estimatedRows: sqlite3_int64,
    pub idxFlags: ::core::ffi::c_int,
    pub colUsed: sqlite3_uint64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_constraint_usage {
    pub argvIndex: ::core::ffi::c_int,
    pub omit: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_orderby {
    pub iColumn: ::core::ffi::c_int,
    pub desc: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_constraint {
    pub iColumn: ::core::ffi::c_int,
    pub op: ::core::ffi::c_uchar,
    pub usable: ::core::ffi::c_uchar,
    pub iTermOffset: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_tokenizer_module {
    pub iVersion: ::core::ffi::c_int,
    pub xCreate: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_tokenizer,
        ) -> ::core::ffi::c_int,
    >,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_tokenizer) -> ::core::ffi::c_int>,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_tokenizer,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_tokenizer_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<unsafe extern "C" fn(*mut sqlite3_tokenizer_cursor) -> ::core::ffi::c_int>,
    pub xNext: Option<
        unsafe extern "C" fn(
            *mut sqlite3_tokenizer_cursor,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xLanguageid: Option<
        unsafe extern "C" fn(
            *mut sqlite3_tokenizer_cursor,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_tokenizer_cursor {
    pub pTokenizer: *mut sqlite3_tokenizer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_tokenizer {
    pub pModule: *const sqlite3_tokenizer_module,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Hash {
    pub keyClass: ::core::ffi::c_char,
    pub copyKey: ::core::ffi::c_char,
    pub count: ::core::ffi::c_int,
    pub first: *mut Fts3HashElem,
    pub htsize: ::core::ffi::c_int,
    pub ht: *mut _fts3ht,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fts3ht {
    pub count: ::core::ffi::c_int,
    pub chain: *mut Fts3HashElem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3HashElem {
    pub next: *mut Fts3HashElem,
    pub prev: *mut Fts3HashElem,
    pub data: *mut ::core::ffi::c_void,
    pub pKey: *mut ::core::ffi::c_void,
    pub nKey: ::core::ffi::c_int,
}
pub type u8_0 = ::core::ffi::c_uchar;
pub type u32_0 = ::core::ffi::c_uint;
pub type i64_0 = sqlite3_int64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Table {
    pub base: sqlite3_vtab,
    pub db: *mut sqlite3,
    pub zDb: *const ::core::ffi::c_char,
    pub zName: *const ::core::ffi::c_char,
    pub nColumn: ::core::ffi::c_int,
    pub azColumn: *mut *mut ::core::ffi::c_char,
    pub abNotindexed: *mut u8_0,
    pub pTokenizer: *mut sqlite3_tokenizer,
    pub zContentTbl: *mut ::core::ffi::c_char,
    pub zLanguageid: *mut ::core::ffi::c_char,
    pub nAutoincrmerge: ::core::ffi::c_int,
    pub nLeafAdd: u32_0,
    pub bLock: ::core::ffi::c_int,
    pub aStmt: [*mut sqlite3_stmt; 40],
    pub pSeekStmt: *mut sqlite3_stmt,
    pub zReadExprlist: *mut ::core::ffi::c_char,
    pub zWriteExprlist: *mut ::core::ffi::c_char,
    pub nNodeSize: ::core::ffi::c_int,
    pub bFts4: u8_0,
    pub bHasStat: u8_0,
    pub bHasDocsize: u8_0,
    pub bDescIdx: u8_0,
    pub bIgnoreSavepoint: u8_0,
    pub nPgsz: ::core::ffi::c_int,
    pub zSegmentsTbl: *mut ::core::ffi::c_char,
    pub pSegments: *mut sqlite3_blob,
    pub iSavepoint: ::core::ffi::c_int,
    pub nIndex: ::core::ffi::c_int,
    pub aIndex: *mut Fts3Index,
    pub nMaxPendingData: ::core::ffi::c_int,
    pub nPendingData: ::core::ffi::c_int,
    pub iPrevDocid: sqlite_int64,
    pub iPrevLangid: ::core::ffi::c_int,
    pub bPrevDelete: ::core::ffi::c_int,
    pub bNoIncrDoclist: ::core::ffi::c_int,
    pub nMergeCount: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Index {
    pub nPrefix: ::core::ffi::c_int,
    pub hPending: Fts3Hash,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3MultiSegReader {
    pub apSegment: *mut *mut Fts3SegReader,
    pub nSegment: ::core::ffi::c_int,
    pub nAdvance: ::core::ffi::c_int,
    pub pFilter: *mut Fts3SegFilter,
    pub aBuffer: *mut ::core::ffi::c_char,
    pub nBuffer: i64_0,
    pub iColFilter: ::core::ffi::c_int,
    pub bRestart: ::core::ffi::c_int,
    pub nCost: ::core::ffi::c_int,
    pub bLookup: ::core::ffi::c_int,
    pub zTerm: *mut ::core::ffi::c_char,
    pub nTerm: ::core::ffi::c_int,
    pub aDoclist: *mut ::core::ffi::c_char,
    pub nDoclist: ::core::ffi::c_int,
}
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
pub struct Fts3auxCursor {
    pub base: sqlite3_vtab_cursor,
    pub csr: Fts3MultiSegReader,
    pub filter: Fts3SegFilter,
    pub zStop: *mut ::core::ffi::c_char,
    pub nStop: ::core::ffi::c_int,
    pub iLangid: ::core::ffi::c_int,
    pub isEof: ::core::ffi::c_int,
    pub iRowid: sqlite3_int64,
    pub iCol: ::core::ffi::c_int,
    pub nStat: ::core::ffi::c_int,
    pub aStat: *mut Fts3auxColstats,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3auxColstats {
    pub nDoc: sqlite3_int64,
    pub nOcc: sqlite3_int64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3auxTable {
    pub base: sqlite3_vtab,
    pub pFts3Tab: *mut Fts3Table,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT_VTAB: ::core::ffi::c_int =
    SQLITE_CORRUPT | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_GT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_LT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_GE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const FTS3_SEGCURSOR_ALL: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const FTS3_SEGMENT_REQUIRE_POS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const FTS3_SEGMENT_IGNORE_EMPTY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const FTS3_SEGMENT_SCAN: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const FTS3_AUX_SCHEMA: [::core::ffi::c_char; 69] = unsafe {
    ::core::mem::transmute::<[u8; 69], [::core::ffi::c_char; 69]>(
        *b"CREATE TABLE x(term, col, documents, occurrences, languageid HIDDEN)\0",
    )
};
unsafe extern "C" fn fts3auxConnectMethod(
    mut db: *mut sqlite3,
    mut _pUnused: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut zFts3: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nDb: ::core::ffi::c_int = 0;
    let mut nFts3: ::core::ffi::c_int = 0;
    let mut nByte: sqlite3_int64 = 0;
    let mut rc: ::core::ffi::c_int = 0;
    let mut p: *mut Fts3auxTable = ::core::ptr::null_mut::<Fts3auxTable>();
    if !(argc != 4 as ::core::ffi::c_int && argc != 5 as ::core::ffi::c_int) {
        zDb = *argv.offset(1 as ::core::ffi::c_int as isize);
        nDb = strlen(zDb) as ::core::ffi::c_int;
        if argc == 5 as ::core::ffi::c_int {
            if nDb == 4 as ::core::ffi::c_int
                && 0 as ::core::ffi::c_int
                    == sqlite3_strnicmp(
                        b"temp\0" as *const u8 as *const ::core::ffi::c_char,
                        zDb,
                        4 as ::core::ffi::c_int,
                    )
            {
                zDb = *argv.offset(3 as ::core::ffi::c_int as isize);
                nDb = strlen(zDb) as ::core::ffi::c_int;
                zFts3 = *argv.offset(4 as ::core::ffi::c_int as isize);
                current_block = 1856101646708284338;
            } else {
                current_block = 10728627893248296414;
            }
        } else {
            zFts3 = *argv.offset(3 as ::core::ffi::c_int as isize);
            current_block = 1856101646708284338;
        }
        match current_block {
            10728627893248296414 => {}
            _ => {
                nFts3 = strlen(zFts3) as ::core::ffi::c_int;
                rc = sqlite3_declare_vtab(db, FTS3_AUX_SCHEMA.as_ptr());
                if rc != SQLITE_OK {
                    return rc;
                }
                nByte = (::core::mem::size_of::<Fts3auxTable>() as usize)
                    .wrapping_add(::core::mem::size_of::<Fts3Table>() as usize)
                    .wrapping_add(nDb as usize)
                    .wrapping_add(nFts3 as usize)
                    .wrapping_add(2 as usize) as sqlite3_int64;
                p = sqlite3_malloc64(nByte as sqlite3_uint64) as *mut Fts3auxTable;
                if p.is_null() {
                    return SQLITE_NOMEM;
                }
                memset(
                    p as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    nByte as size_t,
                );
                (*p).pFts3Tab = p.offset(1 as ::core::ffi::c_int as isize) as *mut Fts3auxTable
                    as *mut Fts3Table;
                (*(*p).pFts3Tab).zDb = (*p).pFts3Tab.offset(1 as ::core::ffi::c_int as isize)
                    as *mut Fts3Table
                    as *mut ::core::ffi::c_char;
                (*(*p).pFts3Tab).zName = (*(*p).pFts3Tab)
                    .zDb
                    .offset((nDb + 1 as ::core::ffi::c_int) as isize)
                    as *const ::core::ffi::c_char;
                (*(*p).pFts3Tab).db = db;
                (*(*p).pFts3Tab).nIndex = 1 as ::core::ffi::c_int;
                memcpy(
                    (*(*p).pFts3Tab).zDb as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    zDb as *const ::core::ffi::c_void,
                    nDb as size_t,
                );
                memcpy(
                    (*(*p).pFts3Tab).zName as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    zFts3 as *const ::core::ffi::c_void,
                    nFts3 as size_t,
                );
                sqlite3Fts3Dequote((*(*p).pFts3Tab).zName as *mut ::core::ffi::c_char);
                *ppVtab = p as *mut sqlite3_vtab;
                return SQLITE_OK;
            }
        }
    }
    sqlite3Fts3ErrMsg(
        pzErr,
        b"invalid arguments to fts4aux constructor\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return SQLITE_ERROR;
}
unsafe extern "C" fn fts3auxDisconnectMethod(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    let mut p: *mut Fts3auxTable = pVtab as *mut Fts3auxTable;
    let mut pFts3: *mut Fts3Table = (*p).pFts3Tab;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[*mut sqlite3_stmt; 40]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut sqlite3_stmt>() as usize)
            as ::core::ffi::c_int
    {
        sqlite3_finalize((*pFts3).aStmt[i as usize]);
        i += 1;
    }
    sqlite3_free((*pFts3).zSegmentsTbl as *mut ::core::ffi::c_void);
    sqlite3_free(p as *mut ::core::ffi::c_void);
    return SQLITE_OK;
}
pub const FTS4AUX_EQ_CONSTRAINT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FTS4AUX_GE_CONSTRAINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FTS4AUX_LE_CONSTRAINT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn fts3auxBestIndexMethod(
    mut _pVTab: *mut sqlite3_vtab,
    mut pInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut iEq: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iGe: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iLe: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iLangid: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iNext: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if (*pInfo).nOrderBy == 1 as ::core::ffi::c_int
        && (*(*pInfo).aOrderBy.offset(0 as ::core::ffi::c_int as isize)).iColumn
            == 0 as ::core::ffi::c_int
        && (*(*pInfo).aOrderBy.offset(0 as ::core::ffi::c_int as isize)).desc as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        (*pInfo).orderByConsumed = 1 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*pInfo).nConstraint {
        if (*(*pInfo).aConstraint.offset(i as isize)).usable != 0 {
            let mut op: ::core::ffi::c_int =
                (*(*pInfo).aConstraint.offset(i as isize)).op as ::core::ffi::c_int;
            let mut iCol: ::core::ffi::c_int = (*(*pInfo).aConstraint.offset(i as isize)).iColumn;
            if iCol == 0 as ::core::ffi::c_int {
                if op == SQLITE_INDEX_CONSTRAINT_EQ {
                    iEq = i;
                }
                if op == SQLITE_INDEX_CONSTRAINT_LT {
                    iLe = i;
                }
                if op == SQLITE_INDEX_CONSTRAINT_LE {
                    iLe = i;
                }
                if op == SQLITE_INDEX_CONSTRAINT_GT {
                    iGe = i;
                }
                if op == SQLITE_INDEX_CONSTRAINT_GE {
                    iGe = i;
                }
            }
            if iCol == 4 as ::core::ffi::c_int {
                if op == SQLITE_INDEX_CONSTRAINT_EQ {
                    iLangid = i;
                }
            }
        }
        i += 1;
    }
    if iEq >= 0 as ::core::ffi::c_int {
        (*pInfo).idxNum = FTS4AUX_EQ_CONSTRAINT;
        let fresh9 = iNext;
        iNext = iNext + 1;
        (*(*pInfo).aConstraintUsage.offset(iEq as isize)).argvIndex = fresh9;
        (*pInfo).estimatedCost = 5 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else {
        (*pInfo).idxNum = 0 as ::core::ffi::c_int;
        (*pInfo).estimatedCost = 20000 as ::core::ffi::c_int as ::core::ffi::c_double;
        if iGe >= 0 as ::core::ffi::c_int {
            (*pInfo).idxNum += FTS4AUX_GE_CONSTRAINT;
            let fresh10 = iNext;
            iNext = iNext + 1;
            (*(*pInfo).aConstraintUsage.offset(iGe as isize)).argvIndex = fresh10;
            (*pInfo).estimatedCost /= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        if iLe >= 0 as ::core::ffi::c_int {
            (*pInfo).idxNum += FTS4AUX_LE_CONSTRAINT;
            let fresh11 = iNext;
            iNext = iNext + 1;
            (*(*pInfo).aConstraintUsage.offset(iLe as isize)).argvIndex = fresh11;
            (*pInfo).estimatedCost /= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    if iLangid >= 0 as ::core::ffi::c_int {
        let fresh12 = iNext;
        iNext = iNext + 1;
        (*(*pInfo).aConstraintUsage.offset(iLangid as isize)).argvIndex = fresh12;
        (*pInfo).estimatedCost -= 1.;
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3auxOpenMethod(
    mut _pVTab: *mut sqlite3_vtab,
    mut ppCsr: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3auxCursor = ::core::ptr::null_mut::<Fts3auxCursor>();
    pCsr = sqlite3_malloc(::core::mem::size_of::<Fts3auxCursor>() as ::core::ffi::c_int)
        as *mut Fts3auxCursor;
    if pCsr.is_null() {
        return SQLITE_NOMEM;
    }
    memset(
        pCsr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Fts3auxCursor>() as size_t,
    );
    *ppCsr = pCsr as *mut sqlite3_vtab_cursor;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3auxCloseMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pFts3: *mut Fts3Table = (*((*pCursor).pVtab as *mut Fts3auxTable)).pFts3Tab;
    let mut pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    sqlite3Fts3SegmentsClose(pFts3);
    sqlite3Fts3SegReaderFinish(&raw mut (*pCsr).csr);
    sqlite3_free((*pCsr).filter.zTerm as *mut ::core::ffi::c_void);
    sqlite3_free((*pCsr).zStop as *mut ::core::ffi::c_void);
    sqlite3_free((*pCsr).aStat as *mut ::core::ffi::c_void);
    sqlite3_free(pCsr as *mut ::core::ffi::c_void);
    return SQLITE_OK;
}
unsafe extern "C" fn fts3auxGrowStatArray(
    mut pCsr: *mut Fts3auxCursor,
    mut nSize: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if nSize > (*pCsr).nStat {
        let mut aNew: *mut Fts3auxColstats = ::core::ptr::null_mut::<Fts3auxColstats>();
        aNew = sqlite3_realloc64(
            (*pCsr).aStat as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<Fts3auxColstats>() as usize).wrapping_mul(nSize as usize)
                as sqlite3_uint64,
        ) as *mut Fts3auxColstats;
        if aNew.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            aNew.offset((*pCsr).nStat as isize) as *mut Fts3auxColstats as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<Fts3auxColstats>() as size_t)
                .wrapping_mul((nSize - (*pCsr).nStat) as size_t),
        );
        (*pCsr).aStat = aNew as *mut Fts3auxColstats;
        (*pCsr).nStat = nSize;
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3auxNextMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    let mut pFts3: *mut Fts3Table = (*((*pCursor).pVtab as *mut Fts3auxTable)).pFts3Tab;
    let mut rc: ::core::ffi::c_int = 0;
    (*pCsr).iRowid += 1;
    (*pCsr).iCol += 1;
    while (*pCsr).iCol < (*pCsr).nStat {
        if (*(*pCsr).aStat.offset((*pCsr).iCol as isize)).nDoc > 0 as sqlite3_int64 {
            return SQLITE_OK;
        }
        (*pCsr).iCol += 1;
    }
    rc = sqlite3Fts3SegReaderStep(pFts3, &raw mut (*pCsr).csr);
    if rc == SQLITE_ROW {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nDoclist: ::core::ffi::c_int = (*pCsr).csr.nDoclist;
        let mut aDoclist: *mut ::core::ffi::c_char = (*pCsr).csr.aDoclist;
        let mut iCol: ::core::ffi::c_int = 0;
        let mut eState: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !(*pCsr).zStop.is_null() {
            let mut n: ::core::ffi::c_int = if (*pCsr).nStop < (*pCsr).csr.nTerm {
                (*pCsr).nStop
            } else {
                (*pCsr).csr.nTerm
            };
            let mut mc: ::core::ffi::c_int = memcmp(
                (*pCsr).zStop as *const ::core::ffi::c_void,
                (*pCsr).csr.zTerm as *const ::core::ffi::c_void,
                n as size_t,
            );
            if mc < 0 as ::core::ffi::c_int
                || mc == 0 as ::core::ffi::c_int && (*pCsr).csr.nTerm > (*pCsr).nStop
            {
                (*pCsr).isEof = 1 as ::core::ffi::c_int;
                return SQLITE_OK;
            }
        }
        if fts3auxGrowStatArray(pCsr, 2 as ::core::ffi::c_int) != 0 {
            return SQLITE_NOMEM;
        }
        memset(
            (*pCsr).aStat as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<Fts3auxColstats>() as size_t)
                .wrapping_mul((*pCsr).nStat as size_t),
        );
        iCol = 0 as ::core::ffi::c_int;
        rc = SQLITE_OK;
        while i < nDoclist {
            let mut v: sqlite3_int64 = 0 as sqlite3_int64;
            i += sqlite3Fts3GetVarint(
                aDoclist.offset(i as isize) as *mut ::core::ffi::c_char,
                &raw mut v,
            );
            let mut current_block_42: u64;
            match eState {
                0 => {
                    let ref mut fresh0 =
                        (*(*pCsr).aStat.offset(0 as ::core::ffi::c_int as isize)).nDoc;
                    *fresh0 += 1;
                    eState = 1 as ::core::ffi::c_int;
                    iCol = 0 as ::core::ffi::c_int;
                    current_block_42 = 18435049525520518667;
                }
                1 => {
                    if v > 1 as sqlite3_int64 {
                        let ref mut fresh1 =
                            (*(*pCsr).aStat.offset(1 as ::core::ffi::c_int as isize)).nDoc;
                        *fresh1 += 1;
                    }
                    eState = 2 as ::core::ffi::c_int;
                    current_block_42 = 1445032439528402095;
                }
                2 => {
                    current_block_42 = 1445032439528402095;
                }
                _ => {
                    iCol = v as ::core::ffi::c_int;
                    if iCol < 1 as ::core::ffi::c_int {
                        rc = SQLITE_CORRUPT_VTAB;
                    } else {
                        if fts3auxGrowStatArray(pCsr, iCol + 2 as ::core::ffi::c_int) != 0 {
                            return SQLITE_NOMEM;
                        }
                        let ref mut fresh4 = (*(*pCsr)
                            .aStat
                            .offset((iCol + 1 as ::core::ffi::c_int) as isize))
                        .nDoc;
                        *fresh4 += 1;
                        eState = 2 as ::core::ffi::c_int;
                    }
                    current_block_42 = 18435049525520518667;
                }
            }
            match current_block_42 {
                1445032439528402095 => {
                    if v == 0 as sqlite3_int64 {
                        eState = 0 as ::core::ffi::c_int;
                    } else if v == 1 as sqlite3_int64 {
                        eState = 3 as ::core::ffi::c_int;
                    } else {
                        let ref mut fresh2 = (*(*pCsr)
                            .aStat
                            .offset((iCol + 1 as ::core::ffi::c_int) as isize))
                        .nOcc;
                        *fresh2 += 1;
                        let ref mut fresh3 =
                            (*(*pCsr).aStat.offset(0 as ::core::ffi::c_int as isize)).nOcc;
                        *fresh3 += 1;
                    }
                }
                _ => {}
            }
        }
        (*pCsr).iCol = 0 as ::core::ffi::c_int;
    } else {
        (*pCsr).isEof = 1 as ::core::ffi::c_int;
    }
    return rc;
}
unsafe extern "C" fn fts3auxFilterMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut _idxStr: *const ::core::ffi::c_char,
    mut nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    let mut pFts3: *mut Fts3Table = (*((*pCursor).pVtab as *mut Fts3auxTable)).pFts3Tab;
    let mut rc: ::core::ffi::c_int = 0;
    let mut isScan: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iLangVal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iEq: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iGe: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iLe: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iLangid: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iNext: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if idxNum == FTS4AUX_EQ_CONSTRAINT {
        let fresh5 = iNext;
        iNext = iNext + 1;
        iEq = fresh5;
    } else {
        isScan = 1 as ::core::ffi::c_int;
        if idxNum & FTS4AUX_GE_CONSTRAINT != 0 {
            let fresh6 = iNext;
            iNext = iNext + 1;
            iGe = fresh6;
        }
        if idxNum & FTS4AUX_LE_CONSTRAINT != 0 {
            let fresh7 = iNext;
            iNext = iNext + 1;
            iLe = fresh7;
        }
    }
    if iNext < nVal {
        let fresh8 = iNext;
        iNext = iNext + 1;
        iLangid = fresh8;
    }
    sqlite3Fts3SegReaderFinish(&raw mut (*pCsr).csr);
    sqlite3_free((*pCsr).filter.zTerm as *mut ::core::ffi::c_void);
    sqlite3_free((*pCsr).aStat as *mut ::core::ffi::c_void);
    sqlite3_free((*pCsr).zStop as *mut ::core::ffi::c_void);
    memset(
        &raw mut (*pCsr).csr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (pCsr.offset(1 as ::core::ffi::c_int as isize) as *mut Fts3auxCursor as *mut u8_0)
            .offset_from(&raw mut (*pCsr).csr as *mut u8_0) as ::core::ffi::c_long
            as size_t,
    );
    (*pCsr).filter.flags = FTS3_SEGMENT_REQUIRE_POS | FTS3_SEGMENT_IGNORE_EMPTY;
    if isScan != 0 {
        (*pCsr).filter.flags |= FTS3_SEGMENT_SCAN;
    }
    if iEq >= 0 as ::core::ffi::c_int || iGe >= 0 as ::core::ffi::c_int {
        let mut zStr: *const ::core::ffi::c_uchar =
            sqlite3_value_text(*apVal.offset(0 as ::core::ffi::c_int as isize));
        if !zStr.is_null() {
            (*pCsr).filter.zTerm =
                sqlite3_mprintf(b"%s\0" as *const u8 as *const ::core::ffi::c_char, zStr);
            if (*pCsr).filter.zTerm.is_null() {
                return SQLITE_NOMEM;
            }
            (*pCsr).filter.nTerm = strlen((*pCsr).filter.zTerm) as ::core::ffi::c_int;
        }
    }
    if iLe >= 0 as ::core::ffi::c_int {
        (*pCsr).zStop = sqlite3_mprintf(
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            sqlite3_value_text(*apVal.offset(iLe as isize)),
        );
        if (*pCsr).zStop.is_null() {
            return SQLITE_NOMEM;
        }
        (*pCsr).nStop = strlen((*pCsr).zStop) as ::core::ffi::c_int;
    }
    if iLangid >= 0 as ::core::ffi::c_int {
        iLangVal = sqlite3_value_int(*apVal.offset(iLangid as isize));
        if iLangVal < 0 as ::core::ffi::c_int {
            iLangVal = 0 as ::core::ffi::c_int;
        }
    }
    (*pCsr).iLangid = iLangVal;
    rc = sqlite3Fts3SegReaderCursor(
        pFts3,
        iLangVal,
        0 as ::core::ffi::c_int,
        FTS3_SEGCURSOR_ALL,
        (*pCsr).filter.zTerm,
        (*pCsr).filter.nTerm,
        0 as ::core::ffi::c_int,
        isScan,
        &raw mut (*pCsr).csr,
    );
    if rc == SQLITE_OK {
        rc = sqlite3Fts3SegReaderStart(pFts3, &raw mut (*pCsr).csr, &raw mut (*pCsr).filter);
    }
    if rc == SQLITE_OK {
        rc = fts3auxNextMethod(pCursor);
    }
    return rc;
}
unsafe extern "C" fn fts3auxEofMethod(mut pCursor: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    return (*pCsr).isEof;
}
unsafe extern "C" fn fts3auxColumnMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pCtx: *mut sqlite3_context,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    match iCol {
        0 => {
            sqlite3_result_text(
                pCtx,
                (*p).csr.zTerm,
                (*p).csr.nTerm,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
            );
        }
        1 => {
            if (*p).iCol != 0 {
                sqlite3_result_int(pCtx, (*p).iCol - 1 as ::core::ffi::c_int);
            } else {
                sqlite3_result_text(
                    pCtx,
                    b"*\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int),
                    SQLITE_STATIC,
                );
            }
        }
        2 => {
            sqlite3_result_int64(pCtx, (*(*p).aStat.offset((*p).iCol as isize)).nDoc);
        }
        3 => {
            sqlite3_result_int64(pCtx, (*(*p).aStat.offset((*p).iCol as isize)).nOcc);
        }
        _ => {
            sqlite3_result_int(pCtx, (*p).iLangid);
        }
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3auxRowidMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    *pRowid = (*pCsr).iRowid as sqlite_int64;
    return SQLITE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3InitAux(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    static mut fts3aux_module: sqlite3_module = unsafe {
        sqlite3_module {
            iVersion: 0 as ::core::ffi::c_int,
            xCreate: Some(
                fts3auxConnectMethod
                    as unsafe extern "C" fn(
                        *mut sqlite3,
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const *const ::core::ffi::c_char,
                        *mut *mut sqlite3_vtab,
                        *mut *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            xConnect: Some(
                fts3auxConnectMethod
                    as unsafe extern "C" fn(
                        *mut sqlite3,
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const *const ::core::ffi::c_char,
                        *mut *mut sqlite3_vtab,
                        *mut *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            xBestIndex: Some(
                fts3auxBestIndexMethod
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab,
                        *mut sqlite3_index_info,
                    ) -> ::core::ffi::c_int,
            ),
            xDisconnect: Some(
                fts3auxDisconnectMethod
                    as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
            ),
            xDestroy: Some(
                fts3auxDisconnectMethod
                    as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
            ),
            xOpen: Some(
                fts3auxOpenMethod
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab,
                        *mut *mut sqlite3_vtab_cursor,
                    ) -> ::core::ffi::c_int,
            ),
            xClose: Some(
                fts3auxCloseMethod
                    as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
            xFilter: Some(
                fts3auxFilterMethod
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab_cursor,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> ::core::ffi::c_int,
            ),
            xNext: Some(
                fts3auxNextMethod
                    as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
            xEof: Some(
                fts3auxEofMethod
                    as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
            xColumn: Some(
                fts3auxColumnMethod
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab_cursor,
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            xRowid: Some(
                fts3auxRowidMethod
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab_cursor,
                        *mut sqlite_int64,
                    ) -> ::core::ffi::c_int,
            ),
            xUpdate: None,
            xBegin: None,
            xSync: None,
            xCommit: None,
            xRollback: None,
            xFindFunction: None,
            xRename: None,
            xSavepoint: None,
            xRelease: None,
            xRollbackTo: None,
            xShadowName: None,
            xIntegrity: None,
        }
    };
    let mut rc: ::core::ffi::c_int = 0;
    rc = sqlite3_create_module(
        db,
        b"fts4aux\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const fts3aux_module,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    return rc;
}
