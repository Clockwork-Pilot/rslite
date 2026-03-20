extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_blob;
    pub type Fts3DeferredToken;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_column_blob(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    fn sqlite3_column_bytes(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3_column_type(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_result_blob(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_error_code(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3Fts3SelectDoctotal(
        _: *mut Fts3Table,
        _: *mut *mut sqlite3_stmt,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3SelectDocsize(
        _: *mut Fts3Table,
        _: sqlite3_int64,
        _: *mut *mut sqlite3_stmt,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3SegmentsClose(_: *mut Fts3Table);
    fn sqlite3Fts3ErrMsg(_: *mut *mut ::core::ffi::c_char, _: *const ::core::ffi::c_char, ...);
    fn sqlite3Fts3GetVarint(
        _: *const ::core::ffi::c_char,
        _: *mut sqlite_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3GetVarintBounded(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *mut sqlite3_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3GetVarint32(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3EvalPhraseStats(
        _: *mut Fts3Cursor,
        _: *mut Fts3Expr,
        _: *mut u32_0,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3EvalTestDeferred(
        pCsr: *mut Fts3Cursor,
        pRc: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3MallocZero(nByte: i64_0) -> *mut ::core::ffi::c_void;
    fn sqlite3Fts3OpenTokenizer(
        _: *mut sqlite3_tokenizer,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut *mut sqlite3_tokenizer_cursor,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3EvalPhrasePoslist(
        _: *mut Fts3Cursor,
        _: *mut Fts3Expr,
        iCol: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3MsrCancel(_: *mut Fts3Cursor, _: *mut Fts3Expr) -> ::core::ffi::c_int;
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
pub type i16_0 = ::core::ffi::c_short;
pub type u32_0 = ::core::ffi::c_uint;
pub type u64_0 = sqlite3_uint64;
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
pub struct Fts3Cursor {
    pub base: sqlite3_vtab_cursor,
    pub eSearch: i16_0,
    pub isEof: u8_0,
    pub isRequireSeek: u8_0,
    pub bSeekStmt: u8_0,
    pub pStmt: *mut sqlite3_stmt,
    pub pExpr: *mut Fts3Expr,
    pub iLangid: ::core::ffi::c_int,
    pub nPhrase: ::core::ffi::c_int,
    pub pDeferred: *mut Fts3DeferredToken,
    pub iPrevId: sqlite3_int64,
    pub pNextId: *mut ::core::ffi::c_char,
    pub aDoclist: *mut ::core::ffi::c_char,
    pub nDoclist: ::core::ffi::c_int,
    pub bDesc: u8_0,
    pub eEvalmode: ::core::ffi::c_int,
    pub nRowAvg: ::core::ffi::c_int,
    pub nDoc: sqlite3_int64,
    pub iMinDocid: i64_0,
    pub iMaxDocid: i64_0,
    pub isMatchinfoNeeded: ::core::ffi::c_int,
    pub pMIBuffer: *mut MatchinfoBuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchinfoBuffer {
    pub aRef: [u8_0; 3],
    pub nElem: ::core::ffi::c_int,
    pub bGlobal: ::core::ffi::c_int,
    pub zMatchinfo: *mut ::core::ffi::c_char,
    pub aMI: [u32_0; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Expr {
    pub eType: ::core::ffi::c_int,
    pub nNear: ::core::ffi::c_int,
    pub pParent: *mut Fts3Expr,
    pub pLeft: *mut Fts3Expr,
    pub pRight: *mut Fts3Expr,
    pub pPhrase: *mut Fts3Phrase,
    pub iDocid: sqlite3_int64,
    pub bEof: u8_0,
    pub bStart: u8_0,
    pub bDeferred: u8_0,
    pub iPhrase: ::core::ffi::c_int,
    pub aMI: *mut u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Phrase {
    pub doclist: Fts3Doclist,
    pub bIncr: ::core::ffi::c_int,
    pub iDoclistToken: ::core::ffi::c_int,
    pub pOrPoslist: *mut ::core::ffi::c_char,
    pub iOrDocid: i64_0,
    pub nToken: ::core::ffi::c_int,
    pub iColumn: ::core::ffi::c_int,
    pub aToken: [Fts3PhraseToken; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3PhraseToken {
    pub z: *mut ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub isPrefix: ::core::ffi::c_int,
    pub bFirst: ::core::ffi::c_int,
    pub pDeferred: *mut Fts3DeferredToken,
    pub pSegcsr: *mut Fts3MultiSegReader,
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
pub struct Fts3Doclist {
    pub aAll: *mut ::core::ffi::c_char,
    pub nAll: ::core::ffi::c_int,
    pub pNextDocid: *mut ::core::ffi::c_char,
    pub iDocid: sqlite3_int64,
    pub bFreeList: ::core::ffi::c_int,
    pub pList: *mut ::core::ffi::c_char,
    pub nList: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StrBuffer {
    pub z: *mut ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub nAlloc: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TermOffset {
    pub pList: *mut ::core::ffi::c_char,
    pub iPos: i64_0,
    pub iOff: i64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TermOffsetCtx {
    pub pCsr: *mut Fts3Cursor,
    pub iCol: ::core::ffi::c_int,
    pub iTerm: ::core::ffi::c_int,
    pub iDocid: sqlite3_int64,
    pub aTerm: *mut TermOffset,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadDoclistCtx {
    pub pCsr: *mut Fts3Cursor,
    pub nPhrase: ::core::ffi::c_int,
    pub nToken: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SnippetFragment {
    pub iCol: ::core::ffi::c_int,
    pub iPos: ::core::ffi::c_int,
    pub covered: u64_0,
    pub hlmask: u64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SnippetPhrase {
    pub nToken: ::core::ffi::c_int,
    pub pList: *mut ::core::ffi::c_char,
    pub iHead: i64_0,
    pub pHead: *mut ::core::ffi::c_char,
    pub iTail: i64_0,
    pub pTail: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SnippetIter {
    pub pCsr: *mut Fts3Cursor,
    pub iCol: ::core::ffi::c_int,
    pub nSnippet: ::core::ffi::c_int,
    pub nPhrase: ::core::ffi::c_int,
    pub aPhrase: *mut SnippetPhrase,
    pub iCurrent: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchInfo {
    pub pCursor: *mut Fts3Cursor,
    pub nCol: ::core::ffi::c_int,
    pub nPhrase: ::core::ffi::c_int,
    pub nDoc: sqlite3_int64,
    pub flag: ::core::ffi::c_char,
    pub aMatchinfo: *mut u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LcsIterator {
    pub pExpr: *mut Fts3Expr,
    pub iPosOffset: ::core::ffi::c_int,
    pub pRead: *mut ::core::ffi::c_char,
    pub iPos: ::core::ffi::c_int,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT_VTAB: ::core::ffi::c_int =
    SQLITE_CORRUPT | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const FTS_CORRUPT_VTAB: ::core::ffi::c_int = SQLITE_CORRUPT_VTAB;
pub const FTSQUERY_NOT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FTSQUERY_PHRASE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FTS3_MATCHINFO_NPHRASE: ::core::ffi::c_int = 112;
pub const FTS3_MATCHINFO_NCOL: ::core::ffi::c_int = 99;
pub const FTS3_MATCHINFO_NDOC: ::core::ffi::c_int = 110;
pub const FTS3_MATCHINFO_AVGLENGTH: ::core::ffi::c_int = 97;
pub const FTS3_MATCHINFO_LENGTH: ::core::ffi::c_int = 108;
pub const FTS3_MATCHINFO_LCS: ::core::ffi::c_int = 115;
pub const FTS3_MATCHINFO_HITS: ::core::ffi::c_int = 'x' as i32;
pub const FTS3_MATCHINFO_LHITS: ::core::ffi::c_int = 121;
pub const FTS3_MATCHINFO_LHITS_BM: ::core::ffi::c_int = 98;
pub const FTS3_MATCHINFO_DEFAULT: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"pcx\0") };
unsafe extern "C" fn fts3MIBufferNew(
    mut nElem: size_t,
    mut zMatchinfo: *const ::core::ffi::c_char,
) -> *mut MatchinfoBuffer {
    let mut pRet: *mut MatchinfoBuffer = ::core::ptr::null_mut::<MatchinfoBuffer>();
    let mut nByte: sqlite3_int64 = (::core::mem::size_of::<u32_0>() as ::core::ffi::c_ulonglong)
        .wrapping_mul(
            (2 as sqlite3_int64 * nElem as sqlite3_int64 + 1 as sqlite3_int64)
                as ::core::ffi::c_ulonglong,
        )
        .wrapping_add(
            (24 as usize).wrapping_add(
                (((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int)
                    as usize)
                    .wrapping_mul(::core::mem::size_of::<u64_0>() as usize),
            ) as ::core::ffi::c_ulonglong,
        ) as sqlite3_int64;
    let mut nStr: sqlite3_int64 = strlen(zMatchinfo) as sqlite3_int64;
    pRet =
        sqlite3Fts3MallocZero(nByte as i64_0 + nStr as i64_0 + 1 as i64_0) as *mut MatchinfoBuffer;
    if !pRet.is_null() {
        *(&raw mut (*pRet).aMI as *mut u32_0).offset(0 as ::core::ffi::c_int as isize) =
            ((&raw mut (*pRet).aMI as *mut u32_0).offset(1 as ::core::ffi::c_int as isize)
                as *mut u32_0 as *mut u8_0)
                .offset_from(pRet as *mut u8_0) as ::core::ffi::c_long as u32_0;
        *(&raw mut (*pRet).aMI as *mut u32_0).offset((1 as size_t).wrapping_add(nElem) as isize) =
            (*(&raw mut (*pRet).aMI as *mut u32_0).offset(0 as ::core::ffi::c_int as isize)
                as usize)
                .wrapping_add(
                    (::core::mem::size_of::<u32_0>() as usize).wrapping_mul(
                        (nElem as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize,
                    ),
                ) as u32_0;
        (*pRet).nElem = nElem as ::core::ffi::c_int;
        (*pRet).zMatchinfo = (pRet as *mut ::core::ffi::c_char).offset(nByte as isize);
        memcpy(
            (*pRet).zMatchinfo as *mut ::core::ffi::c_void,
            zMatchinfo as *const ::core::ffi::c_void,
            (nStr + 1 as sqlite3_int64) as size_t,
        );
        (*pRet).aRef[0 as ::core::ffi::c_int as usize] = 1 as u8_0;
    }
    return pRet;
}
unsafe extern "C" fn fts3MIBufferFree(mut p: *mut ::core::ffi::c_void) {
    let mut pBuf: *mut MatchinfoBuffer = (p as *mut u8_0)
        .offset(-(*(p as *mut u32_0).offset(-(1 as ::core::ffi::c_int) as isize) as isize))
        as *mut MatchinfoBuffer;
    if p as *mut u32_0
        == (&raw mut (*pBuf).aMI as *mut u32_0).offset(1 as ::core::ffi::c_int as isize)
            as *mut u32_0
    {
        (*pBuf).aRef[1 as ::core::ffi::c_int as usize] = 0 as u8_0;
    } else {
        (*pBuf).aRef[2 as ::core::ffi::c_int as usize] = 0 as u8_0;
    }
    if (*pBuf).aRef[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        && (*pBuf).aRef[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        && (*pBuf).aRef[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        sqlite3_free(pBuf as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn fts3MIBufferAlloc(
    mut p: *mut MatchinfoBuffer,
    mut paOut: *mut *mut u32_0,
) -> Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> {
    let mut xRet: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> = None;
    let mut aOut: *mut u32_0 = ::core::ptr::null_mut::<u32_0>();
    if (*p).aRef[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        (*p).aRef[1 as ::core::ffi::c_int as usize] = 1 as u8_0;
        aOut = (&raw mut (*p).aMI as *mut u32_0).offset(1 as ::core::ffi::c_int as isize)
            as *mut u32_0;
        xRet = Some(fts3MIBufferFree as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    } else if (*p).aRef[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        (*p).aRef[2 as ::core::ffi::c_int as usize] = 1 as u8_0;
        aOut = (&raw mut (*p).aMI as *mut u32_0)
            .offset(((*p).nElem + 2 as ::core::ffi::c_int) as isize) as *mut u32_0;
        xRet = Some(fts3MIBufferFree as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    } else {
        aOut = sqlite3_malloc64(
            ((*p).nElem as usize).wrapping_mul(::core::mem::size_of::<u32_0>() as usize)
                as sqlite3_uint64,
        ) as *mut u32_0;
        if !aOut.is_null() {
            xRet = Some(sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
                as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
            if (*p).bGlobal != 0 {
                memcpy(
                    aOut as *mut ::core::ffi::c_void,
                    (&raw mut (*p).aMI as *mut u32_0).offset(1 as ::core::ffi::c_int as isize)
                        as *mut u32_0 as *const ::core::ffi::c_void,
                    ((*p).nElem as size_t).wrapping_mul(::core::mem::size_of::<u32_0>() as size_t),
                );
            }
        }
    }
    *paOut = aOut;
    return xRet;
}
unsafe extern "C" fn fts3MIBufferSetGlobal(mut p: *mut MatchinfoBuffer) {
    (*p).bGlobal = 1 as ::core::ffi::c_int;
    memcpy(
        (&raw mut (*p).aMI as *mut u32_0).offset((2 as ::core::ffi::c_int + (*p).nElem) as isize)
            as *mut u32_0 as *mut ::core::ffi::c_void,
        (&raw mut (*p).aMI as *mut u32_0).offset(1 as ::core::ffi::c_int as isize) as *mut u32_0
            as *const ::core::ffi::c_void,
        ((*p).nElem as size_t).wrapping_mul(::core::mem::size_of::<u32_0>() as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3MIBufferFree(mut p: *mut MatchinfoBuffer) {
    if !p.is_null() {
        (*p).aRef[0 as ::core::ffi::c_int as usize] = 0 as u8_0;
        if (*p).aRef[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && (*p).aRef[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            && (*p).aRef[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }
}
unsafe extern "C" fn fts3GetDeltaPosition(
    mut pp: *mut *mut ::core::ffi::c_char,
    mut piPos: *mut i64_0,
) {
    let mut iVal: ::core::ffi::c_int = 0;
    *pp = (*pp).offset(
        (if *(*pp as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
            sqlite3Fts3GetVarint32(*pp, &raw mut iVal)
        } else {
            iVal = *(*pp as *mut u8_0) as ::core::ffi::c_int;
            1 as ::core::ffi::c_int
        }) as isize,
    );
    *piPos += (iVal - 2 as ::core::ffi::c_int) as i64_0;
}
unsafe extern "C" fn fts3ExprIterate2(
    mut pExpr: *mut Fts3Expr,
    mut piPhrase: *mut ::core::ffi::c_int,
    mut x: Option<
        unsafe extern "C" fn(
            *mut Fts3Expr,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    mut pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut eType: ::core::ffi::c_int = (*pExpr).eType;
    if eType != FTSQUERY_PHRASE {
        rc = fts3ExprIterate2((*pExpr).pLeft, piPhrase, x, pCtx);
        if rc == SQLITE_OK && eType != FTSQUERY_NOT {
            rc = fts3ExprIterate2((*pExpr).pRight, piPhrase, x, pCtx);
        }
    } else {
        rc = x.expect("non-null function pointer")(pExpr, *piPhrase, pCtx);
        *piPhrase += 1;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3ExprIterate(
    mut pExpr: *mut Fts3Expr,
    mut x: Option<
        unsafe extern "C" fn(
            *mut Fts3Expr,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    mut pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut iPhrase: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    return fts3ExprIterate2(pExpr, &raw mut iPhrase, x, pCtx);
}
unsafe extern "C" fn fts3ExprLoadDoclistsCb(
    mut pExpr: *mut Fts3Expr,
    mut _iPhrase: ::core::ffi::c_int,
    mut ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut pPhrase: *mut Fts3Phrase = (*pExpr).pPhrase;
    let mut p: *mut LoadDoclistCtx = ctx as *mut LoadDoclistCtx;
    (*p).nPhrase += 1;
    (*p).nToken += (*pPhrase).nToken;
    return rc;
}
unsafe extern "C" fn fts3ExprLoadDoclists(
    mut pCsr: *mut Fts3Cursor,
    mut pnPhrase: *mut ::core::ffi::c_int,
    mut pnToken: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut sCtx: LoadDoclistCtx = LoadDoclistCtx {
        pCsr: ::core::ptr::null_mut::<Fts3Cursor>(),
        nPhrase: 0 as ::core::ffi::c_int,
        nToken: 0 as ::core::ffi::c_int,
    };
    sCtx.pCsr = pCsr;
    rc = sqlite3Fts3ExprIterate(
        (*pCsr).pExpr,
        Some(
            fts3ExprLoadDoclistsCb
                as unsafe extern "C" fn(
                    *mut Fts3Expr,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut sCtx as *mut ::core::ffi::c_void,
    );
    if !pnPhrase.is_null() {
        *pnPhrase = sCtx.nPhrase;
    }
    if !pnToken.is_null() {
        *pnToken = sCtx.nToken;
    }
    return rc;
}
unsafe extern "C" fn fts3ExprPhraseCountCb(
    mut pExpr: *mut Fts3Expr,
    mut iPhrase: ::core::ffi::c_int,
    mut ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let ref mut fresh3 = *(ctx as *mut ::core::ffi::c_int);
    *fresh3 += 1;
    (*pExpr).iPhrase = iPhrase;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3ExprPhraseCount(mut pExpr: *mut Fts3Expr) -> ::core::ffi::c_int {
    let mut nPhrase: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    sqlite3Fts3ExprIterate(
        pExpr,
        Some(
            fts3ExprPhraseCountCb
                as unsafe extern "C" fn(
                    *mut Fts3Expr,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut nPhrase as *mut ::core::ffi::c_void,
    );
    return nPhrase;
}
unsafe extern "C" fn fts3SnippetAdvance(
    mut ppIter: *mut *mut ::core::ffi::c_char,
    mut piIter: *mut i64_0,
    mut iNext: ::core::ffi::c_int,
) {
    let mut pIter: *mut ::core::ffi::c_char = *ppIter;
    if !pIter.is_null() {
        let mut iIter: i64_0 = *piIter;
        while iIter < iNext as i64_0 {
            if 0 as ::core::ffi::c_int == *pIter as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int
            {
                iIter = -(1 as ::core::ffi::c_int) as i64_0;
                pIter = ::core::ptr::null_mut::<::core::ffi::c_char>();
                break;
            } else {
                fts3GetDeltaPosition(&raw mut pIter, &raw mut iIter);
            }
        }
        *piIter = iIter;
        *ppIter = pIter;
    }
}
unsafe extern "C" fn fts3SnippetNextCandidate(mut pIter: *mut SnippetIter) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if (*pIter).iCurrent < 0 as ::core::ffi::c_int {
        (*pIter).iCurrent = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIter).nPhrase {
            let mut pPhrase: *mut SnippetPhrase =
                (*pIter).aPhrase.offset(i as isize) as *mut SnippetPhrase;
            fts3SnippetAdvance(
                &raw mut (*pPhrase).pHead,
                &raw mut (*pPhrase).iHead,
                (*pIter).nSnippet,
            );
            i += 1;
        }
    } else {
        let mut iStart: ::core::ffi::c_int = 0;
        let mut iEnd: ::core::ffi::c_int = 0x7fffffff as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIter).nPhrase {
            let mut pPhrase_0: *mut SnippetPhrase =
                (*pIter).aPhrase.offset(i as isize) as *mut SnippetPhrase;
            if !(*pPhrase_0).pHead.is_null() && (*pPhrase_0).iHead < iEnd as i64_0 {
                iEnd = (*pPhrase_0).iHead as ::core::ffi::c_int;
            }
            i += 1;
        }
        if iEnd == 0x7fffffff as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        iStart = iEnd - (*pIter).nSnippet + 1 as ::core::ffi::c_int;
        (*pIter).iCurrent = iStart;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIter).nPhrase {
            let mut pPhrase_1: *mut SnippetPhrase =
                (*pIter).aPhrase.offset(i as isize) as *mut SnippetPhrase;
            fts3SnippetAdvance(
                &raw mut (*pPhrase_1).pHead,
                &raw mut (*pPhrase_1).iHead,
                iEnd + 1 as ::core::ffi::c_int,
            );
            fts3SnippetAdvance(
                &raw mut (*pPhrase_1).pTail,
                &raw mut (*pPhrase_1).iTail,
                iStart,
            );
            i += 1;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3SnippetDetails(
    mut pIter: *mut SnippetIter,
    mut mCovered: u64_0,
    mut piToken: *mut ::core::ffi::c_int,
    mut piScore: *mut ::core::ffi::c_int,
    mut pmCover: *mut u64_0,
    mut pmHighlight: *mut u64_0,
) {
    let mut iStart: ::core::ffi::c_int = (*pIter).iCurrent;
    let mut iScore: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut mCover: u64_0 = 0 as u64_0;
    let mut mHighlight: u64_0 = 0 as u64_0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pIter).nPhrase {
        let mut pPhrase: *mut SnippetPhrase =
            (*pIter).aPhrase.offset(i as isize) as *mut SnippetPhrase;
        if !(*pPhrase).pTail.is_null() {
            let mut pCsr: *mut ::core::ffi::c_char = (*pPhrase).pTail;
            let mut iCsr: i64_0 = (*pPhrase).iTail;
            while iCsr < (iStart + (*pIter).nSnippet) as i64_0 && iCsr >= iStart as i64_0 {
                let mut j: ::core::ffi::c_int = 0;
                let mut mPhrase: u64_0 =
                    (1 as ::core::ffi::c_int as u64_0) << i % 64 as ::core::ffi::c_int;
                let mut mPos: u64_0 = (1 as ::core::ffi::c_int as u64_0) << iCsr - iStart as i64_0;
                if (mCover | mCovered) & mPhrase != 0 {
                    iScore += 1;
                } else {
                    iScore += 1000 as ::core::ffi::c_int;
                }
                mCover |= mPhrase;
                j = 0 as ::core::ffi::c_int;
                while j < (*pPhrase).nToken && j < (*pIter).nSnippet {
                    mHighlight |= mPos >> j;
                    j += 1;
                }
                if 0 as ::core::ffi::c_int
                    == *pCsr as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int
                {
                    break;
                }
                fts3GetDeltaPosition(&raw mut pCsr, &raw mut iCsr);
            }
        }
        i += 1;
    }
    *piToken = iStart;
    *piScore = iScore;
    *pmCover = mCover;
    *pmHighlight = mHighlight;
}
unsafe extern "C" fn fts3SnippetFindPositions(
    mut pExpr: *mut Fts3Expr,
    mut iPhrase: ::core::ffi::c_int,
    mut ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut p: *mut SnippetIter = ctx as *mut SnippetIter;
    let mut pPhrase: *mut SnippetPhrase =
        (*p).aPhrase.offset(iPhrase as isize) as *mut SnippetPhrase;
    let mut pCsr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rc: ::core::ffi::c_int = 0;
    (*pPhrase).nToken = (*(*pExpr).pPhrase).nToken;
    rc = sqlite3Fts3EvalPhrasePoslist((*p).pCsr, pExpr, (*p).iCol, &raw mut pCsr);
    if !pCsr.is_null() {
        let mut iFirst: i64_0 = 0 as i64_0;
        (*pPhrase).pList = pCsr;
        fts3GetDeltaPosition(&raw mut pCsr, &raw mut iFirst);
        if iFirst < 0 as i64_0 {
            rc = FTS_CORRUPT_VTAB;
        } else {
            (*pPhrase).pHead = pCsr;
            (*pPhrase).pTail = pCsr;
            (*pPhrase).iHead = iFirst;
            (*pPhrase).iTail = iFirst;
        }
    }
    return rc;
}
unsafe extern "C" fn fts3BestSnippet(
    mut nSnippet: ::core::ffi::c_int,
    mut pCsr: *mut Fts3Cursor,
    mut iCol: ::core::ffi::c_int,
    mut mCovered: u64_0,
    mut pmSeen: *mut u64_0,
    mut pFragment: *mut SnippetFragment,
    mut piScore: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut nList: ::core::ffi::c_int = 0;
    let mut sIter: SnippetIter = SnippetIter {
        pCsr: ::core::ptr::null_mut::<Fts3Cursor>(),
        iCol: 0,
        nSnippet: 0,
        nPhrase: 0,
        aPhrase: ::core::ptr::null_mut::<SnippetPhrase>(),
        iCurrent: 0,
    };
    let mut nByte: sqlite3_int64 = 0;
    let mut iBestScore: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    memset(
        &raw mut sIter as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<SnippetIter>() as size_t,
    );
    rc = fts3ExprLoadDoclists(
        pCsr,
        &raw mut nList,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    if rc != SQLITE_OK {
        return rc;
    }
    nByte = (::core::mem::size_of::<SnippetPhrase>() as usize).wrapping_mul(nList as usize)
        as sqlite3_int64;
    sIter.aPhrase = sqlite3Fts3MallocZero(nByte as i64_0) as *mut SnippetPhrase;
    if sIter.aPhrase.is_null() {
        return SQLITE_NOMEM;
    }
    sIter.pCsr = pCsr;
    sIter.iCol = iCol;
    sIter.nSnippet = nSnippet;
    sIter.nPhrase = nList;
    sIter.iCurrent = -(1 as ::core::ffi::c_int);
    rc = sqlite3Fts3ExprIterate(
        (*pCsr).pExpr,
        Some(
            fts3SnippetFindPositions
                as unsafe extern "C" fn(
                    *mut Fts3Expr,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut sIter as *mut ::core::ffi::c_void,
    );
    if rc == SQLITE_OK {
        i = 0 as ::core::ffi::c_int;
        while i < nList {
            if !(*sIter.aPhrase.offset(i as isize)).pHead.is_null() {
                *pmSeen |= (1 as ::core::ffi::c_int as u64_0) << i % 64 as ::core::ffi::c_int;
            }
            i += 1;
        }
        (*pFragment).iCol = iCol;
        while fts3SnippetNextCandidate(&raw mut sIter) == 0 {
            let mut iPos: ::core::ffi::c_int = 0;
            let mut iScore: ::core::ffi::c_int = 0;
            let mut mCover: u64_0 = 0;
            let mut mHighlite: u64_0 = 0;
            fts3SnippetDetails(
                &raw mut sIter,
                mCovered,
                &raw mut iPos,
                &raw mut iScore,
                &raw mut mCover,
                &raw mut mHighlite,
            );
            if iScore > iBestScore {
                (*pFragment).iPos = iPos;
                (*pFragment).hlmask = mHighlite;
                (*pFragment).covered = mCover;
                iBestScore = iScore;
            }
        }
        *piScore = iBestScore;
    }
    sqlite3_free(sIter.aPhrase as *mut ::core::ffi::c_void);
    return rc;
}
unsafe extern "C" fn fts3StringAppend(
    mut pStr: *mut StrBuffer,
    mut zAppend: *const ::core::ffi::c_char,
    mut nAppend: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if nAppend < 0 as ::core::ffi::c_int {
        nAppend = strlen(zAppend) as ::core::ffi::c_int;
    }
    if (*pStr).n + nAppend + 1 as ::core::ffi::c_int >= (*pStr).nAlloc {
        let mut nAlloc: sqlite3_int64 =
            (*pStr).nAlloc as sqlite3_int64 + nAppend as sqlite3_int64 + 100 as sqlite3_int64;
        let mut zNew: *mut ::core::ffi::c_char = sqlite3_realloc64(
            (*pStr).z as *mut ::core::ffi::c_void,
            nAlloc as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if zNew.is_null() {
            return SQLITE_NOMEM;
        }
        (*pStr).z = zNew;
        (*pStr).nAlloc = nAlloc as ::core::ffi::c_int;
    }
    memcpy(
        (*pStr).z.offset((*pStr).n as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        zAppend as *const ::core::ffi::c_void,
        nAppend as size_t,
    );
    (*pStr).n += nAppend;
    *(*pStr).z.offset((*pStr).n as isize) = '\0' as i32 as ::core::ffi::c_char;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3SnippetShift(
    mut pTab: *mut Fts3Table,
    mut iLangid: ::core::ffi::c_int,
    mut nSnippet: ::core::ffi::c_int,
    mut zDoc: *const ::core::ffi::c_char,
    mut nDoc: ::core::ffi::c_int,
    mut piPos: *mut ::core::ffi::c_int,
    mut pHlmask: *mut u64_0,
) -> ::core::ffi::c_int {
    let mut hlmask: u64_0 = *pHlmask;
    if hlmask != 0 {
        let mut nLeft: ::core::ffi::c_int = 0;
        let mut nRight: ::core::ffi::c_int = 0;
        let mut nDesired: ::core::ffi::c_int = 0;
        nLeft = 0 as ::core::ffi::c_int;
        while hlmask & (1 as ::core::ffi::c_int as u64_0) << nLeft == 0 {
            nLeft += 1;
        }
        nRight = 0 as ::core::ffi::c_int;
        while hlmask
            & (1 as ::core::ffi::c_int as u64_0) << nSnippet - 1 as ::core::ffi::c_int - nRight
            == 0
        {
            nRight += 1;
        }
        nDesired = (nLeft - nRight) / 2 as ::core::ffi::c_int;
        if nDesired > 0 as ::core::ffi::c_int {
            let mut nShift: ::core::ffi::c_int = 0;
            let mut iCurrent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut rc: ::core::ffi::c_int = 0;
            let mut pMod: *mut sqlite3_tokenizer_module =
                ::core::ptr::null_mut::<sqlite3_tokenizer_module>();
            let mut pC: *mut sqlite3_tokenizer_cursor =
                ::core::ptr::null_mut::<sqlite3_tokenizer_cursor>();
            pMod = (*(*pTab).pTokenizer).pModule as *mut sqlite3_tokenizer_module;
            rc = sqlite3Fts3OpenTokenizer((*pTab).pTokenizer, iLangid, zDoc, nDoc, &raw mut pC);
            if rc != SQLITE_OK {
                return rc;
            }
            while rc == SQLITE_OK && iCurrent < nSnippet + nDesired {
                let mut ZDUMMY: *const ::core::ffi::c_char =
                    ::core::ptr::null::<::core::ffi::c_char>();
                let mut DUMMY1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut DUMMY2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut DUMMY3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                rc = (*pMod).xNext.expect("non-null function pointer")(
                    pC,
                    &raw mut ZDUMMY,
                    &raw mut DUMMY1,
                    &raw mut DUMMY2,
                    &raw mut DUMMY3,
                    &raw mut iCurrent,
                );
            }
            (*pMod).xClose.expect("non-null function pointer")(pC);
            if rc != SQLITE_OK && rc != SQLITE_DONE {
                return rc;
            }
            nShift = (rc == SQLITE_DONE) as ::core::ffi::c_int + iCurrent - nSnippet;
            if nShift > 0 as ::core::ffi::c_int {
                *piPos += nShift;
                *pHlmask = hlmask >> nShift;
            }
        }
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3SnippetText(
    mut pCsr: *mut Fts3Cursor,
    mut pFragment: *mut SnippetFragment,
    mut iFragment: ::core::ffi::c_int,
    mut isLast: ::core::ffi::c_int,
    mut nSnippet: ::core::ffi::c_int,
    mut zOpen: *const ::core::ffi::c_char,
    mut zClose: *const ::core::ffi::c_char,
    mut zEllipsis: *const ::core::ffi::c_char,
    mut pOut: *mut StrBuffer,
) -> ::core::ffi::c_int {
    let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
    let mut rc: ::core::ffi::c_int = 0;
    let mut zDoc: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nDoc: ::core::ffi::c_int = 0;
    let mut iCurrent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iEnd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut isShiftDone: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iPos: ::core::ffi::c_int = (*pFragment).iPos;
    let mut hlmask: u64_0 = (*pFragment).hlmask;
    let mut iCol: ::core::ffi::c_int = (*pFragment).iCol + 1 as ::core::ffi::c_int;
    let mut pMod: *mut sqlite3_tokenizer_module =
        ::core::ptr::null_mut::<sqlite3_tokenizer_module>();
    let mut pC: *mut sqlite3_tokenizer_cursor = ::core::ptr::null_mut::<sqlite3_tokenizer_cursor>();
    zDoc = sqlite3_column_text((*pCsr).pStmt, iCol) as *const ::core::ffi::c_char;
    if zDoc.is_null() {
        if sqlite3_column_type((*pCsr).pStmt, iCol) != SQLITE_NULL {
            return SQLITE_NOMEM;
        }
        return SQLITE_OK;
    }
    nDoc = sqlite3_column_bytes((*pCsr).pStmt, iCol);
    pMod = (*(*pTab).pTokenizer).pModule as *mut sqlite3_tokenizer_module;
    rc = sqlite3Fts3OpenTokenizer((*pTab).pTokenizer, (*pCsr).iLangid, zDoc, nDoc, &raw mut pC);
    if rc != SQLITE_OK {
        return rc;
    }
    while rc == SQLITE_OK {
        let mut ZDUMMY: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut DUMMY1: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut iBegin: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iFin: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut isHighlight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        rc = (*pMod).xNext.expect("non-null function pointer")(
            pC,
            &raw mut ZDUMMY,
            &raw mut DUMMY1,
            &raw mut iBegin,
            &raw mut iFin,
            &raw mut iCurrent,
        );
        if rc != SQLITE_OK {
            if rc == SQLITE_DONE {
                rc = fts3StringAppend(
                    pOut,
                    zDoc.offset(iEnd as isize) as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int),
                );
            }
            break;
        } else {
            if iCurrent < iPos {
                continue;
            }
            if isShiftDone == 0 {
                let mut n: ::core::ffi::c_int = nDoc - iBegin;
                rc = fts3SnippetShift(
                    pTab,
                    (*pCsr).iLangid,
                    nSnippet,
                    zDoc.offset(iBegin as isize) as *const ::core::ffi::c_char,
                    n,
                    &raw mut iPos,
                    &raw mut hlmask,
                );
                isShiftDone = 1 as ::core::ffi::c_int;
                if rc == SQLITE_OK {
                    if iPos > 0 as ::core::ffi::c_int || iFragment > 0 as ::core::ffi::c_int {
                        rc = fts3StringAppend(pOut, zEllipsis, -(1 as ::core::ffi::c_int));
                    } else if iBegin != 0 {
                        rc = fts3StringAppend(pOut, zDoc, iBegin);
                    }
                }
                if rc != SQLITE_OK || iCurrent < iPos {
                    continue;
                }
            }
            if iCurrent >= iPos + nSnippet {
                if isLast != 0 {
                    rc = fts3StringAppend(pOut, zEllipsis, -(1 as ::core::ffi::c_int));
                }
                break;
            } else {
                isHighlight = (hlmask & (1 as ::core::ffi::c_int as u64_0) << iCurrent - iPos
                    != 0 as u64_0) as ::core::ffi::c_int;
                if iCurrent > iPos {
                    rc = fts3StringAppend(
                        pOut,
                        zDoc.offset(iEnd as isize) as *const ::core::ffi::c_char,
                        iBegin - iEnd,
                    );
                }
                if rc == SQLITE_OK && isHighlight != 0 {
                    rc = fts3StringAppend(pOut, zOpen, -(1 as ::core::ffi::c_int));
                }
                if rc == SQLITE_OK {
                    rc = fts3StringAppend(
                        pOut,
                        zDoc.offset(iBegin as isize) as *const ::core::ffi::c_char,
                        iFin - iBegin,
                    );
                }
                if rc == SQLITE_OK && isHighlight != 0 {
                    rc = fts3StringAppend(pOut, zClose, -(1 as ::core::ffi::c_int));
                }
                iEnd = iFin;
            }
        }
    }
    (*pMod).xClose.expect("non-null function pointer")(pC);
    return rc;
}
unsafe extern "C" fn fts3ColumnlistCount(
    mut ppCollist: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pEnd: *mut ::core::ffi::c_char = *ppCollist;
    let mut c: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    let mut nEntry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while 0xfe as ::core::ffi::c_int & (*pEnd as ::core::ffi::c_int | c as ::core::ffi::c_int) != 0
    {
        let fresh1 = pEnd;
        pEnd = pEnd.offset(1);
        c = (*fresh1 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        if c == 0 {
            nEntry += 1;
        }
    }
    *ppCollist = pEnd;
    return nEntry;
}
unsafe extern "C" fn fts3ExprLHits(
    mut pExpr: *mut Fts3Expr,
    mut p: *mut MatchInfo,
) -> ::core::ffi::c_int {
    let mut pTab: *mut Fts3Table = (*(*p).pCursor).base.pVtab as *mut Fts3Table;
    let mut iStart: ::core::ffi::c_int = 0;
    let mut pPhrase: *mut Fts3Phrase = (*pExpr).pPhrase;
    let mut pIter: *mut ::core::ffi::c_char = (*pPhrase).doclist.pList;
    let mut iCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*p).flag as ::core::ffi::c_int == FTS3_MATCHINFO_LHITS {
        iStart = (*pExpr).iPhrase * (*p).nCol;
    } else {
        iStart =
            (*pExpr).iPhrase * (((*p).nCol + 31 as ::core::ffi::c_int) / 32 as ::core::ffi::c_int);
    }
    if !pIter.is_null() {
        loop {
            let mut nHit: ::core::ffi::c_int = fts3ColumnlistCount(&raw mut pIter);
            if (*pPhrase).iColumn >= (*pTab).nColumn || (*pPhrase).iColumn == iCol {
                if (*p).flag as ::core::ffi::c_int == FTS3_MATCHINFO_LHITS {
                    *(*p).aMatchinfo.offset((iStart + iCol) as isize) = nHit as u32_0;
                } else if nHit != 0 {
                    *(*p).aMatchinfo.offset(
                        (iStart + (iCol + 1 as ::core::ffi::c_int) / 32 as ::core::ffi::c_int)
                            as isize,
                    ) |=
                        ((1 as ::core::ffi::c_int) << (iCol & 0x1f as ::core::ffi::c_int)) as u32_0;
                }
            }
            if *pIter as ::core::ffi::c_int != 0x1 as ::core::ffi::c_int {
                break;
            }
            pIter = pIter.offset(1);
            pIter = pIter.offset(
                (if *(pIter as *mut u8_0) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                    sqlite3Fts3GetVarint32(pIter, &raw mut iCol)
                } else {
                    iCol = *(pIter as *mut u8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                }) as isize,
            );
            if iCol >= (*p).nCol {
                return FTS_CORRUPT_VTAB;
            }
        }
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3ExprLHitGather(
    mut pExpr: *mut Fts3Expr,
    mut p: *mut MatchInfo,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if (*pExpr).bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*pExpr).iDocid == (*(*p).pCursor).iPrevId
    {
        if !(*pExpr).pLeft.is_null() {
            rc = fts3ExprLHitGather((*pExpr).pLeft, p);
            if rc == SQLITE_OK {
                rc = fts3ExprLHitGather((*pExpr).pRight, p);
            }
        } else {
            rc = fts3ExprLHits(pExpr, p);
        }
    }
    return rc;
}
unsafe extern "C" fn fts3ExprGlobalHitsCb(
    mut pExpr: *mut Fts3Expr,
    mut iPhrase: ::core::ffi::c_int,
    mut pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut p: *mut MatchInfo = pCtx as *mut MatchInfo;
    return sqlite3Fts3EvalPhraseStats(
        (*p).pCursor,
        pExpr,
        (*p).aMatchinfo
            .offset((3 as ::core::ffi::c_int * iPhrase * (*p).nCol) as isize) as *mut u32_0,
    );
}
unsafe extern "C" fn fts3ExprLocalHitsCb(
    mut pExpr: *mut Fts3Expr,
    mut iPhrase: ::core::ffi::c_int,
    mut pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut p: *mut MatchInfo = pCtx as *mut MatchInfo;
    let mut iStart: ::core::ffi::c_int = iPhrase * (*p).nCol * 3 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*p).nCol && rc == SQLITE_OK {
        let mut pCsr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        rc = sqlite3Fts3EvalPhrasePoslist((*p).pCursor, pExpr, i, &raw mut pCsr);
        if !pCsr.is_null() {
            *(*p)
                .aMatchinfo
                .offset((iStart + i * 3 as ::core::ffi::c_int) as isize) =
                fts3ColumnlistCount(&raw mut pCsr) as u32_0;
        } else {
            *(*p)
                .aMatchinfo
                .offset((iStart + i * 3 as ::core::ffi::c_int) as isize) = 0 as u32_0;
        }
        i += 1;
    }
    return rc;
}
unsafe extern "C" fn fts3MatchinfoCheck(
    mut pTab: *mut Fts3Table,
    mut cArg: ::core::ffi::c_char,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if cArg as ::core::ffi::c_int == FTS3_MATCHINFO_NPHRASE
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_NCOL
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_NDOC
            && (*pTab).bFts4 as ::core::ffi::c_int != 0
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_AVGLENGTH
            && (*pTab).bFts4 as ::core::ffi::c_int != 0
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_LENGTH
            && (*pTab).bHasDocsize as ::core::ffi::c_int != 0
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_LCS
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_HITS
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_LHITS
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_LHITS_BM
    {
        return SQLITE_OK;
    }
    sqlite3Fts3ErrMsg(
        pzErr,
        b"unrecognized matchinfo request: %c\0" as *const u8 as *const ::core::ffi::c_char,
        cArg as ::core::ffi::c_int,
    );
    return SQLITE_ERROR;
}
unsafe extern "C" fn fts3MatchinfoSize(
    mut pInfo: *mut MatchInfo,
    mut cArg: ::core::ffi::c_char,
) -> size_t {
    let mut nVal: size_t = 0;
    match cArg as ::core::ffi::c_int {
        FTS3_MATCHINFO_NDOC | FTS3_MATCHINFO_NPHRASE | FTS3_MATCHINFO_NCOL => {
            nVal = 1 as size_t;
        }
        FTS3_MATCHINFO_AVGLENGTH | FTS3_MATCHINFO_LENGTH | FTS3_MATCHINFO_LCS => {
            nVal = (*pInfo).nCol as size_t;
        }
        FTS3_MATCHINFO_LHITS => {
            nVal = ((*pInfo).nCol as size_t).wrapping_mul((*pInfo).nPhrase as size_t);
        }
        FTS3_MATCHINFO_LHITS_BM => {
            nVal = ((*pInfo).nPhrase as size_t).wrapping_mul(
                (((*pInfo).nCol + 31 as ::core::ffi::c_int) / 32 as ::core::ffi::c_int) as size_t,
            );
        }
        _ => {
            nVal = ((*pInfo).nCol as size_t)
                .wrapping_mul((*pInfo).nPhrase as size_t)
                .wrapping_mul(3 as size_t);
        }
    }
    return nVal;
}
unsafe extern "C" fn fts3MatchinfoSelectDoctotal(
    mut pTab: *mut Fts3Table,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pnDoc: *mut sqlite3_int64,
    mut paLen: *mut *const ::core::ffi::c_char,
    mut ppEnd: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut a: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut pEnd: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nDoc: sqlite3_int64 = 0;
    let mut n: ::core::ffi::c_int = 0;
    if (*ppStmt).is_null() {
        let mut rc: ::core::ffi::c_int = sqlite3Fts3SelectDoctotal(pTab, ppStmt);
        if rc != SQLITE_OK {
            return rc;
        }
    }
    pStmt = *ppStmt;
    n = sqlite3_column_bytes(pStmt, 0 as ::core::ffi::c_int);
    a = sqlite3_column_blob(pStmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    if a.is_null() {
        return FTS_CORRUPT_VTAB;
    }
    pEnd = a.offset(n as isize);
    a = a.offset(sqlite3Fts3GetVarintBounded(a, pEnd, &raw mut nDoc) as isize);
    if nDoc <= 0 as sqlite3_int64 || a > pEnd {
        return FTS_CORRUPT_VTAB;
    }
    *pnDoc = nDoc;
    if !paLen.is_null() {
        *paLen = a;
    }
    if !ppEnd.is_null() {
        *ppEnd = pEnd;
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3MatchinfoLcsCb(
    mut pExpr: *mut Fts3Expr,
    mut iPhrase: ::core::ffi::c_int,
    mut pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut aIter: *mut LcsIterator = pCtx as *mut LcsIterator;
    let ref mut fresh2 = (*aIter.offset(iPhrase as isize)).pExpr;
    *fresh2 = pExpr;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3LcsIteratorAdvance(mut pIter: *mut LcsIterator) -> ::core::ffi::c_int {
    let mut pRead: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut iRead: sqlite3_int64 = 0;
    let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if pIter.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    pRead = (*pIter).pRead;
    pRead = pRead.offset(sqlite3Fts3GetVarint(pRead, &raw mut iRead) as isize);
    if iRead == 0 as sqlite3_int64 || iRead == 1 as sqlite3_int64 {
        pRead = ::core::ptr::null_mut::<::core::ffi::c_char>();
        rc = 1 as ::core::ffi::c_int;
    } else {
        (*pIter).iPos += (iRead - 2 as sqlite3_int64) as ::core::ffi::c_int;
    }
    (*pIter).pRead = pRead;
    return rc;
}
unsafe extern "C" fn fts3MatchinfoLcs(
    mut pCsr: *mut Fts3Cursor,
    mut pInfo: *mut MatchInfo,
) -> ::core::ffi::c_int {
    let mut aIter: *mut LcsIterator = ::core::ptr::null_mut::<LcsIterator>();
    let mut i: ::core::ffi::c_int = 0;
    let mut iCol: ::core::ffi::c_int = 0;
    let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    aIter = sqlite3Fts3MallocZero(
        (::core::mem::size_of::<LcsIterator>() as usize).wrapping_mul((*pCsr).nPhrase as usize)
            as i64_0,
    ) as *mut LcsIterator;
    if aIter.is_null() {
        return SQLITE_NOMEM;
    }
    sqlite3Fts3ExprIterate(
        (*pCsr).pExpr,
        Some(
            fts3MatchinfoLcsCb
                as unsafe extern "C" fn(
                    *mut Fts3Expr,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        aIter as *mut ::core::ffi::c_void,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*pInfo).nPhrase {
        let mut pIter: *mut LcsIterator = aIter.offset(i as isize) as *mut LcsIterator;
        nToken -= (*(*(*pIter).pExpr).pPhrase).nToken;
        (*pIter).iPosOffset = nToken;
        i += 1;
    }
    iCol = 0 as ::core::ffi::c_int;
    's_48: while iCol < (*pInfo).nCol {
        let mut nLcs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nLive: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*pInfo).nPhrase {
            let mut pIt: *mut LcsIterator = aIter.offset(i as isize) as *mut LcsIterator;
            rc = sqlite3Fts3EvalPhrasePoslist(pCsr, (*pIt).pExpr, iCol, &raw mut (*pIt).pRead);
            if rc != SQLITE_OK {
                break 's_48;
            }
            if !(*pIt).pRead.is_null() {
                (*pIt).iPos = (*pIt).iPosOffset;
                fts3LcsIteratorAdvance(pIt);
                if (*pIt).pRead.is_null() {
                    rc = FTS_CORRUPT_VTAB;
                    break 's_48;
                } else {
                    nLive += 1;
                }
            }
            i += 1;
        }
        while nLive > 0 as ::core::ffi::c_int {
            let mut pAdv: *mut LcsIterator = ::core::ptr::null_mut::<LcsIterator>();
            let mut nThisLcs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < (*pInfo).nPhrase {
                let mut pIter_0: *mut LcsIterator = aIter.offset(i as isize) as *mut LcsIterator;
                if (*pIter_0).pRead.is_null() {
                    nThisLcs = 0 as ::core::ffi::c_int;
                } else {
                    if pAdv.is_null() || (*pIter_0).iPos < (*pAdv).iPos {
                        pAdv = pIter_0;
                    }
                    if nThisLcs == 0 as ::core::ffi::c_int
                        || (*pIter_0).iPos
                            == (*pIter_0.offset(-(1 as ::core::ffi::c_int) as isize)).iPos
                    {
                        nThisLcs += 1;
                    } else {
                        nThisLcs = 1 as ::core::ffi::c_int;
                    }
                    if nThisLcs > nLcs {
                        nLcs = nThisLcs;
                    }
                }
                i += 1;
            }
            if fts3LcsIteratorAdvance(pAdv) != 0 {
                nLive -= 1;
            }
        }
        *(*pInfo).aMatchinfo.offset(iCol as isize) = nLcs as u32_0;
        iCol += 1;
    }
    sqlite3_free(aIter as *mut ::core::ffi::c_void);
    return rc;
}
unsafe extern "C" fn fts3MatchinfoValues(
    mut pCsr: *mut Fts3Cursor,
    mut bGlobal: ::core::ffi::c_int,
    mut pInfo: *mut MatchInfo,
    mut zArg: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut i: ::core::ffi::c_int = 0;
    let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
    let mut pSelect: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    i = 0 as ::core::ffi::c_int;
    while rc == SQLITE_OK && *zArg.offset(i as isize) as ::core::ffi::c_int != 0 {
        (*pInfo).flag = *zArg.offset(i as isize);
        let mut current_block_42: u64;
        match *zArg.offset(i as isize) as ::core::ffi::c_int {
            FTS3_MATCHINFO_NPHRASE => {
                if bGlobal != 0 {
                    *(*pInfo).aMatchinfo.offset(0 as ::core::ffi::c_int as isize) =
                        (*pInfo).nPhrase as u32_0;
                }
            }
            FTS3_MATCHINFO_NCOL => {
                if bGlobal != 0 {
                    *(*pInfo).aMatchinfo.offset(0 as ::core::ffi::c_int as isize) =
                        (*pInfo).nCol as u32_0;
                }
            }
            FTS3_MATCHINFO_NDOC => {
                if bGlobal != 0 {
                    let mut nDoc: sqlite3_int64 = 0 as sqlite3_int64;
                    rc = fts3MatchinfoSelectDoctotal(
                        pTab,
                        &raw mut pSelect,
                        &raw mut nDoc,
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                    );
                    *(*pInfo).aMatchinfo.offset(0 as ::core::ffi::c_int as isize) = nDoc as u32_0;
                }
            }
            FTS3_MATCHINFO_AVGLENGTH => {
                if bGlobal != 0 {
                    let mut nDoc_0: sqlite3_int64 = 0;
                    let mut a: *const ::core::ffi::c_char =
                        ::core::ptr::null::<::core::ffi::c_char>();
                    let mut pEnd: *const ::core::ffi::c_char =
                        ::core::ptr::null::<::core::ffi::c_char>();
                    rc = fts3MatchinfoSelectDoctotal(
                        pTab,
                        &raw mut pSelect,
                        &raw mut nDoc_0,
                        &raw mut a,
                        &raw mut pEnd,
                    );
                    if rc == SQLITE_OK {
                        let mut iCol: ::core::ffi::c_int = 0;
                        iCol = 0 as ::core::ffi::c_int;
                        while iCol < (*pInfo).nCol {
                            let mut iVal: u32_0 = 0;
                            let mut nToken: sqlite3_int64 = 0;
                            a = a.offset(sqlite3Fts3GetVarint(a, &raw mut nToken) as isize);
                            if a > pEnd {
                                rc = SQLITE_CORRUPT_VTAB;
                                break;
                            } else {
                                iVal = (((nToken & 0xffffffff as sqlite3_int64) as u32_0
                                    as sqlite3_int64
                                    + nDoc_0 / 2 as sqlite3_int64)
                                    / nDoc_0) as u32_0;
                                *(*pInfo).aMatchinfo.offset(iCol as isize) = iVal;
                                iCol += 1;
                            }
                        }
                    }
                }
            }
            FTS3_MATCHINFO_LENGTH => {
                let mut pSelectDocsize: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
                rc = sqlite3Fts3SelectDocsize(pTab, (*pCsr).iPrevId, &raw mut pSelectDocsize);
                if rc == SQLITE_OK {
                    let mut iCol_0: ::core::ffi::c_int = 0;
                    let mut a_0: *const ::core::ffi::c_char =
                        sqlite3_column_blob(pSelectDocsize, 0 as ::core::ffi::c_int)
                            as *const ::core::ffi::c_char;
                    let mut pEnd_0: *const ::core::ffi::c_char =
                        a_0.offset(
                            sqlite3_column_bytes(pSelectDocsize, 0 as ::core::ffi::c_int) as isize,
                        );
                    iCol_0 = 0 as ::core::ffi::c_int;
                    while iCol_0 < (*pInfo).nCol {
                        let mut nToken_0: sqlite3_int64 = 0;
                        a_0 = a_0
                            .offset(sqlite3Fts3GetVarintBounded(a_0, pEnd_0, &raw mut nToken_0)
                                as isize);
                        if a_0 > pEnd_0 {
                            rc = SQLITE_CORRUPT_VTAB;
                            break;
                        } else {
                            *(*pInfo).aMatchinfo.offset(iCol_0 as isize) = nToken_0 as u32_0;
                            iCol_0 += 1;
                        }
                    }
                }
                sqlite3_reset(pSelectDocsize);
            }
            FTS3_MATCHINFO_LCS => {
                rc = fts3ExprLoadDoclists(
                    pCsr,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                if rc == SQLITE_OK {
                    rc = fts3MatchinfoLcs(pCsr, pInfo);
                }
            }
            FTS3_MATCHINFO_LHITS_BM | FTS3_MATCHINFO_LHITS => {
                let mut nZero: size_t = fts3MatchinfoSize(pInfo, *zArg.offset(i as isize))
                    .wrapping_mul(::core::mem::size_of::<u32_0>() as size_t);
                memset(
                    (*pInfo).aMatchinfo as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    nZero,
                );
                rc = fts3ExprLHitGather((*pCsr).pExpr, pInfo);
            }
            _ => {
                let mut pExpr: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
                pExpr = (*pCsr).pExpr;
                rc = fts3ExprLoadDoclists(
                    pCsr,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                if !(rc != SQLITE_OK) {
                    if bGlobal != 0 {
                        if !(*pCsr).pDeferred.is_null() {
                            rc = fts3MatchinfoSelectDoctotal(
                                pTab,
                                &raw mut pSelect,
                                &raw mut (*pInfo).nDoc,
                                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                            );
                            if rc != SQLITE_OK {
                                current_block_42 = 2520131295878969859;
                            } else {
                                current_block_42 = 13826291924415791078;
                            }
                        } else {
                            current_block_42 = 13826291924415791078;
                        }
                        match current_block_42 {
                            2520131295878969859 => {}
                            _ => {
                                rc = sqlite3Fts3ExprIterate(
                                    pExpr,
                                    Some(
                                        fts3ExprGlobalHitsCb
                                            as unsafe extern "C" fn(
                                                *mut Fts3Expr,
                                                ::core::ffi::c_int,
                                                *mut ::core::ffi::c_void,
                                            )
                                                -> ::core::ffi::c_int,
                                    ),
                                    pInfo as *mut ::core::ffi::c_void,
                                );
                                sqlite3Fts3EvalTestDeferred(pCsr, &raw mut rc);
                                if rc != SQLITE_OK {
                                    current_block_42 = 2520131295878969859;
                                } else {
                                    current_block_42 = 7343950298149844727;
                                }
                            }
                        }
                    } else {
                        current_block_42 = 7343950298149844727;
                    }
                    match current_block_42 {
                        2520131295878969859 => {}
                        _ => {
                            sqlite3Fts3ExprIterate(
                                pExpr,
                                Some(
                                    fts3ExprLocalHitsCb
                                        as unsafe extern "C" fn(
                                            *mut Fts3Expr,
                                            ::core::ffi::c_int,
                                            *mut ::core::ffi::c_void,
                                        )
                                            -> ::core::ffi::c_int,
                                ),
                                pInfo as *mut ::core::ffi::c_void,
                            );
                        }
                    }
                }
            }
        }
        (*pInfo).aMatchinfo = (*pInfo)
            .aMatchinfo
            .offset(fts3MatchinfoSize(pInfo, *zArg.offset(i as isize)) as isize);
        i += 1;
    }
    sqlite3_reset(pSelect);
    return rc;
}
unsafe extern "C" fn fts3GetMatchinfo(
    mut pCtx: *mut sqlite3_context,
    mut pCsr: *mut Fts3Cursor,
    mut zArg: *const ::core::ffi::c_char,
) {
    let mut sInfo: MatchInfo = MatchInfo {
        pCursor: ::core::ptr::null_mut::<Fts3Cursor>(),
        nCol: 0,
        nPhrase: 0,
        nDoc: 0,
        flag: 0,
        aMatchinfo: ::core::ptr::null_mut::<u32_0>(),
    };
    let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut bGlobal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut aOut: *mut u32_0 = ::core::ptr::null_mut::<u32_0>();
    let mut xDestroyOut: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> = None;
    memset(
        &raw mut sInfo as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MatchInfo>() as size_t,
    );
    sInfo.pCursor = pCsr;
    sInfo.nCol = (*pTab).nColumn;
    if !(*pCsr).pMIBuffer.is_null() && strcmp((*(*pCsr).pMIBuffer).zMatchinfo, zArg) != 0 {
        sqlite3Fts3MIBufferFree((*pCsr).pMIBuffer);
        (*pCsr).pMIBuffer = ::core::ptr::null_mut::<MatchinfoBuffer>();
    }
    if (*pCsr).pMIBuffer.is_null() {
        let mut nMatchinfo: size_t = 0 as size_t;
        let mut i: ::core::ffi::c_int = 0;
        (*pCsr).nPhrase = fts3ExprPhraseCount((*pCsr).pExpr);
        sInfo.nPhrase = (*pCsr).nPhrase;
        i = 0 as ::core::ffi::c_int;
        while *zArg.offset(i as isize) != 0 {
            let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if fts3MatchinfoCheck(pTab, *zArg.offset(i as isize), &raw mut zErr) != 0 {
                sqlite3_result_error(pCtx, zErr, -(1 as ::core::ffi::c_int));
                sqlite3_free(zErr as *mut ::core::ffi::c_void);
                return;
            }
            nMatchinfo = nMatchinfo
                .wrapping_add(fts3MatchinfoSize(&raw mut sInfo, *zArg.offset(i as isize)));
            i += 1;
        }
        (*pCsr).pMIBuffer = fts3MIBufferNew(nMatchinfo, zArg);
        if (*pCsr).pMIBuffer.is_null() {
            rc = SQLITE_NOMEM;
        }
        (*pCsr).isMatchinfoNeeded = 1 as ::core::ffi::c_int;
        bGlobal = 1 as ::core::ffi::c_int;
    }
    if rc == SQLITE_OK {
        xDestroyOut = fts3MIBufferAlloc((*pCsr).pMIBuffer, &raw mut aOut);
        if xDestroyOut.is_none() {
            rc = SQLITE_NOMEM;
        }
    }
    if rc == SQLITE_OK {
        sInfo.aMatchinfo = aOut;
        sInfo.nPhrase = (*pCsr).nPhrase;
        rc = fts3MatchinfoValues(pCsr, bGlobal, &raw mut sInfo, zArg);
        if bGlobal != 0 {
            fts3MIBufferSetGlobal((*pCsr).pMIBuffer);
        }
    }
    if rc != SQLITE_OK {
        sqlite3_result_error_code(pCtx, rc);
        if xDestroyOut.is_some() {
            xDestroyOut.expect("non-null function pointer")(aOut as *mut ::core::ffi::c_void);
        }
    } else {
        let mut n: ::core::ffi::c_int = ((*(*pCsr).pMIBuffer).nElem as usize)
            .wrapping_mul(::core::mem::size_of::<u32_0>() as usize)
            as ::core::ffi::c_int;
        sqlite3_result_blob(pCtx, aOut as *const ::core::ffi::c_void, n, xDestroyOut);
    };
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3Snippet(
    mut pCtx: *mut sqlite3_context,
    mut pCsr: *mut Fts3Cursor,
    mut zStart: *const ::core::ffi::c_char,
    mut zEnd: *const ::core::ffi::c_char,
    mut zEllipsis: *const ::core::ffi::c_char,
    mut iCol: ::core::ffi::c_int,
    mut nToken: ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut i: ::core::ffi::c_int = 0;
    let mut res: StrBuffer = StrBuffer {
        z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        n: 0 as ::core::ffi::c_int,
        nAlloc: 0 as ::core::ffi::c_int,
    };
    let mut nSnippet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut aSnippet: [SnippetFragment; 4] = [SnippetFragment {
        iCol: 0,
        iPos: 0,
        covered: 0,
        hlmask: 0,
    }; 4];
    let mut nFToken: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if (*pCsr).pExpr.is_null() {
        sqlite3_result_text(
            pCtx,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        return;
    }
    if nToken < -(64 as ::core::ffi::c_int) {
        nToken = -(64 as ::core::ffi::c_int);
    }
    if nToken > 64 as ::core::ffi::c_int {
        nToken = 64 as ::core::ffi::c_int;
    }
    nSnippet = 1 as ::core::ffi::c_int;
    's_52: loop {
        let mut iSnip: ::core::ffi::c_int = 0;
        let mut mCovered: u64_0 = 0 as u64_0;
        let mut mSeen: u64_0 = 0 as u64_0;
        if nToken >= 0 as ::core::ffi::c_int {
            nFToken = (nToken + nSnippet - 1 as ::core::ffi::c_int) / nSnippet;
        } else {
            nFToken = -(1 as ::core::ffi::c_int) * nToken;
        }
        iSnip = 0 as ::core::ffi::c_int;
        while iSnip < nSnippet {
            let mut iBestScore: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
            let mut iRead: ::core::ffi::c_int = 0;
            let mut pFragment: *mut SnippetFragment = (&raw mut aSnippet as *mut SnippetFragment)
                .offset(iSnip as isize)
                as *mut SnippetFragment;
            memset(
                pFragment as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<SnippetFragment>() as size_t,
            );
            iRead = 0 as ::core::ffi::c_int;
            while iRead < (*pTab).nColumn {
                let mut sF: SnippetFragment = SnippetFragment {
                    iCol: 0 as ::core::ffi::c_int,
                    iPos: 0 as ::core::ffi::c_int,
                    covered: 0 as u64_0,
                    hlmask: 0 as u64_0,
                };
                let mut iS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if !(iCol >= 0 as ::core::ffi::c_int && iRead != iCol) {
                    rc = fts3BestSnippet(
                        nFToken,
                        pCsr,
                        iRead,
                        mCovered,
                        &raw mut mSeen,
                        &raw mut sF,
                        &raw mut iS,
                    );
                    if rc != SQLITE_OK {
                        current_block = 4442782934716781880;
                        break 's_52;
                    }
                    if iS > iBestScore {
                        *pFragment = sF;
                        iBestScore = iS;
                    }
                }
                iRead += 1;
            }
            mCovered |= (*pFragment).covered;
            iSnip += 1;
        }
        if mSeen == mCovered
            || nSnippet
                == (::core::mem::size_of::<[SnippetFragment; 4]>() as usize)
                    .wrapping_div(::core::mem::size_of::<SnippetFragment>() as usize)
                    as ::core::ffi::c_int
        {
            current_block = 1538046216550696469;
            break;
        }
        nSnippet += 1;
    }
    match current_block {
        1538046216550696469 => {
            i = 0 as ::core::ffi::c_int;
            while i < nSnippet && rc == SQLITE_OK {
                rc = fts3SnippetText(
                    pCsr,
                    (&raw mut aSnippet as *mut SnippetFragment).offset(i as isize)
                        as *mut SnippetFragment,
                    i,
                    (i == nSnippet - 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    nFToken,
                    zStart,
                    zEnd,
                    zEllipsis,
                    &raw mut res,
                );
                i += 1;
            }
        }
        _ => {}
    }
    sqlite3Fts3SegmentsClose(pTab);
    if rc != SQLITE_OK {
        sqlite3_result_error_code(pCtx, rc);
        sqlite3_free(res.z as *mut ::core::ffi::c_void);
    } else {
        sqlite3_result_text(
            pCtx,
            res.z,
            -(1 as ::core::ffi::c_int),
            Some(sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
    };
}
unsafe extern "C" fn fts3ExprTermOffsetInit(
    mut pExpr: *mut Fts3Expr,
    mut _iPhrase: ::core::ffi::c_int,
    mut ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut p: *mut TermOffsetCtx = ctx as *mut TermOffsetCtx;
    let mut nTerm: ::core::ffi::c_int = 0;
    let mut iTerm: ::core::ffi::c_int = 0;
    let mut pList: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut iPos: i64_0 = 0 as i64_0;
    let mut rc: ::core::ffi::c_int = 0;
    rc = sqlite3Fts3EvalPhrasePoslist((*p).pCsr, pExpr, (*p).iCol, &raw mut pList);
    nTerm = (*(*pExpr).pPhrase).nToken;
    if !pList.is_null() {
        fts3GetDeltaPosition(&raw mut pList, &raw mut iPos);
    }
    iTerm = 0 as ::core::ffi::c_int;
    while iTerm < nTerm {
        let fresh0 = (*p).iTerm;
        (*p).iTerm = (*p).iTerm + 1;
        let mut pT: *mut TermOffset = (*p).aTerm.offset(fresh0 as isize) as *mut TermOffset;
        (*pT).iOff = (nTerm - iTerm - 1 as ::core::ffi::c_int) as i64_0;
        (*pT).pList = pList;
        (*pT).iPos = iPos;
        iTerm += 1;
    }
    return rc;
}
unsafe extern "C" fn fts3ExprRestartIfCb(
    mut pExpr: *mut Fts3Expr,
    mut _iPhrase: ::core::ffi::c_int,
    mut ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut p: *mut TermOffsetCtx = ctx as *mut TermOffsetCtx;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if !(*pExpr).pPhrase.is_null() && (*(*pExpr).pPhrase).bIncr != 0 {
        rc = sqlite3Fts3MsrCancel((*p).pCsr, pExpr);
        (*(*pExpr).pPhrase).bIncr = 0 as ::core::ffi::c_int;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3Offsets(
    mut pCtx: *mut sqlite3_context,
    mut pCsr: *mut Fts3Cursor,
) {
    let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
    let mut pMod: *const sqlite3_tokenizer_module = (*(*pTab).pTokenizer).pModule;
    let mut rc: ::core::ffi::c_int = 0;
    let mut nToken: ::core::ffi::c_int = 0;
    let mut iCol: ::core::ffi::c_int = 0;
    let mut res: StrBuffer = StrBuffer {
        z: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        n: 0 as ::core::ffi::c_int,
        nAlloc: 0 as ::core::ffi::c_int,
    };
    let mut sCtx: TermOffsetCtx = TermOffsetCtx {
        pCsr: ::core::ptr::null_mut::<Fts3Cursor>(),
        iCol: 0,
        iTerm: 0,
        iDocid: 0,
        aTerm: ::core::ptr::null_mut::<TermOffset>(),
    };
    if (*pCsr).pExpr.is_null() {
        sqlite3_result_text(
            pCtx,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        return;
    }
    memset(
        &raw mut sCtx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<TermOffsetCtx>() as size_t,
    );
    rc = fts3ExprLoadDoclists(
        pCsr,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut nToken,
    );
    if !(rc != SQLITE_OK) {
        sCtx.aTerm = sqlite3Fts3MallocZero(
            (::core::mem::size_of::<TermOffset>() as usize).wrapping_mul(nToken as usize) as i64_0,
        ) as *mut TermOffset;
        if sCtx.aTerm.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            sCtx.iDocid = (*pCsr).iPrevId;
            sCtx.pCsr = pCsr;
            rc = sqlite3Fts3ExprIterate(
                (*pCsr).pExpr,
                Some(
                    fts3ExprRestartIfCb
                        as unsafe extern "C" fn(
                            *mut Fts3Expr,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                &raw mut sCtx as *mut ::core::ffi::c_void,
            );
            if !(rc != SQLITE_OK) {
                iCol = 0 as ::core::ffi::c_int;
                while iCol < (*pTab).nColumn {
                    let mut pC: *mut sqlite3_tokenizer_cursor =
                        ::core::ptr::null_mut::<sqlite3_tokenizer_cursor>();
                    let mut ZDUMMY: *const ::core::ffi::c_char =
                        ::core::ptr::null::<::core::ffi::c_char>();
                    let mut NDUMMY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut iStart: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut iEnd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut iCurrent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut zDoc: *const ::core::ffi::c_char =
                        ::core::ptr::null::<::core::ffi::c_char>();
                    let mut nDoc: ::core::ffi::c_int = 0;
                    sCtx.iCol = iCol;
                    sCtx.iTerm = 0 as ::core::ffi::c_int;
                    rc = sqlite3Fts3ExprIterate(
                        (*pCsr).pExpr,
                        Some(
                            fts3ExprTermOffsetInit
                                as unsafe extern "C" fn(
                                    *mut Fts3Expr,
                                    ::core::ffi::c_int,
                                    *mut ::core::ffi::c_void,
                                )
                                    -> ::core::ffi::c_int,
                        ),
                        &raw mut sCtx as *mut ::core::ffi::c_void,
                    );
                    if rc != SQLITE_OK {
                        break;
                    }
                    zDoc = sqlite3_column_text((*pCsr).pStmt, iCol + 1 as ::core::ffi::c_int)
                        as *const ::core::ffi::c_char;
                    nDoc = sqlite3_column_bytes((*pCsr).pStmt, iCol + 1 as ::core::ffi::c_int);
                    if zDoc.is_null() {
                        if !(sqlite3_column_type((*pCsr).pStmt, iCol + 1 as ::core::ffi::c_int)
                            == SQLITE_NULL)
                        {
                            rc = SQLITE_NOMEM;
                            break;
                        }
                    } else {
                        rc = sqlite3Fts3OpenTokenizer(
                            (*pTab).pTokenizer,
                            (*pCsr).iLangid,
                            zDoc,
                            nDoc,
                            &raw mut pC,
                        );
                        if rc != SQLITE_OK {
                            break;
                        }
                        rc = (*pMod).xNext.expect("non-null function pointer")(
                            pC,
                            &raw mut ZDUMMY,
                            &raw mut NDUMMY,
                            &raw mut iStart,
                            &raw mut iEnd,
                            &raw mut iCurrent,
                        );
                        while rc == SQLITE_OK {
                            let mut i: ::core::ffi::c_int = 0;
                            let mut iMinPos: ::core::ffi::c_int = 0x7fffffff as ::core::ffi::c_int;
                            let mut pTerm: *mut TermOffset = ::core::ptr::null_mut::<TermOffset>();
                            i = 0 as ::core::ffi::c_int;
                            while i < nToken {
                                let mut pT: *mut TermOffset =
                                    sCtx.aTerm.offset(i as isize) as *mut TermOffset;
                                if !(*pT).pList.is_null()
                                    && (*pT).iPos - (*pT).iOff < iMinPos as i64_0
                                {
                                    iMinPos = ((*pT).iPos - (*pT).iOff) as ::core::ffi::c_int;
                                    pTerm = pT;
                                }
                                i += 1;
                            }
                            if pTerm.is_null() {
                                rc = SQLITE_DONE;
                            } else {
                                if 0 as ::core::ffi::c_int
                                    == 0xfe as ::core::ffi::c_int
                                        & *(*pTerm).pList as ::core::ffi::c_int
                                {
                                    (*pTerm).pList = ::core::ptr::null_mut::<::core::ffi::c_char>();
                                } else {
                                    fts3GetDeltaPosition(
                                        &raw mut (*pTerm).pList,
                                        &raw mut (*pTerm).iPos,
                                    );
                                }
                                while rc == SQLITE_OK && iCurrent < iMinPos {
                                    rc = (*pMod).xNext.expect("non-null function pointer")(
                                        pC,
                                        &raw mut ZDUMMY,
                                        &raw mut NDUMMY,
                                        &raw mut iStart,
                                        &raw mut iEnd,
                                        &raw mut iCurrent,
                                    );
                                }
                                if rc == SQLITE_OK {
                                    let mut aBuffer: [::core::ffi::c_char; 64] = [0; 64];
                                    sqlite3_snprintf(
                                        ::core::mem::size_of::<[::core::ffi::c_char; 64]>()
                                            as ::core::ffi::c_int,
                                        &raw mut aBuffer as *mut ::core::ffi::c_char,
                                        b"%d %d %d %d \0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        iCol,
                                        pTerm.offset_from(sCtx.aTerm) as ::core::ffi::c_long,
                                        iStart,
                                        iEnd - iStart,
                                    );
                                    rc = fts3StringAppend(
                                        &raw mut res,
                                        &raw mut aBuffer as *mut ::core::ffi::c_char,
                                        -(1 as ::core::ffi::c_int),
                                    );
                                } else if rc == SQLITE_DONE && (*pTab).zContentTbl.is_null() {
                                    rc = FTS_CORRUPT_VTAB;
                                }
                            }
                        }
                        if rc == SQLITE_DONE {
                            rc = SQLITE_OK;
                        }
                        (*pMod).xClose.expect("non-null function pointer")(pC);
                        if rc != SQLITE_OK {
                            break;
                        }
                    }
                    iCol += 1;
                }
            }
        }
    }
    sqlite3_free(sCtx.aTerm as *mut ::core::ffi::c_void);
    sqlite3Fts3SegmentsClose(pTab);
    if rc != SQLITE_OK {
        sqlite3_result_error_code(pCtx, rc);
        sqlite3_free(res.z as *mut ::core::ffi::c_void);
    } else {
        sqlite3_result_text(
            pCtx,
            res.z,
            res.n - 1 as ::core::ffi::c_int,
            Some(sqlite3_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3Matchinfo(
    mut pContext: *mut sqlite3_context,
    mut pCsr: *mut Fts3Cursor,
    mut zArg: *const ::core::ffi::c_char,
) {
    let mut pTab: *mut Fts3Table = (*pCsr).base.pVtab as *mut Fts3Table;
    let mut zFormat: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !zArg.is_null() {
        zFormat = zArg;
    } else {
        zFormat = FTS3_MATCHINFO_DEFAULT.as_ptr();
    }
    if (*pCsr).pExpr.is_null() {
        sqlite3_result_blob(
            pContext,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        return;
    } else {
        fts3GetMatchinfo(pContext, pCsr, zFormat);
        sqlite3Fts3SegmentsClose(pTab);
    };
}
