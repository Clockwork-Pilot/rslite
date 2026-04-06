use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_blob;
    pub type Fts3SegReader;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
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
    fn sqlite3Fts3ErrMsg(
        _: *mut *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    );
    fn sqlite3Fts3GetVarint(
        _: *const ::core::ffi::c_char,
        _: *mut sqlite_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3Dequote(_: *mut ::core::ffi::c_char);
    fn sqlite3Fts3MallocZero(nByte: i64_0) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
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
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *mut sqlite3_index_info,
        ) -> ::core::ffi::c_int,
    >,
    pub xDisconnect: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
    >,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *mut *mut sqlite3_vtab_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xFilter: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub xNext: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xEof: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xColumn: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            *mut sqlite3_context,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xRowid: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            *mut sqlite3_int64,
        ) -> ::core::ffi::c_int,
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
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xSavepoint: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xRelease: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xRollbackTo: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xShadowName: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    >,
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
    pub xDestroy: Option<
        unsafe extern "C" fn(*mut sqlite3_tokenizer) -> ::core::ffi::c_int,
    >,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_tokenizer,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_tokenizer_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<
        unsafe extern "C" fn(*mut sqlite3_tokenizer_cursor) -> ::core::ffi::c_int,
    >,
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
pub struct Fts3termCursor {
    pub base: sqlite3_vtab_cursor,
    pub csr: Fts3MultiSegReader,
    pub filter: Fts3SegFilter,
    pub isEof: ::core::ffi::c_int,
    pub pNext: *mut ::core::ffi::c_char,
    pub iRowid: sqlite3_int64,
    pub iDocid: sqlite3_int64,
    pub iCol: ::core::ffi::c_int,
    pub iPos: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3termTable {
    pub base: sqlite3_vtab,
    pub iIndex: ::core::ffi::c_int,
    pub pFts3Tab: *mut Fts3Table,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const FTS3_SEGCURSOR_ALL: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const FTS3_SEGMENT_REQUIRE_POS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const FTS3_SEGMENT_IGNORE_EMPTY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const FTS3_SEGMENT_SCAN: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const FTS3_TERMS_SCHEMA: [::core::ffi::c_char; 38] = unsafe {
    ::core::mem::transmute::<
        [u8; 38],
        [::core::ffi::c_char; 38],
    >(*b"CREATE TABLE x(term, docid, col, pos)\0")
};
unsafe extern "C" fn fts3termConnectMethod(
    mut db: *mut sqlite3,
    mut pCtx: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zFts3: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut nDb: ::core::ffi::c_int = 0;
        let mut nFts3: ::core::ffi::c_int = 0;
        let mut nByte: sqlite3_int64 = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut Fts3termTable = ::core::ptr::null_mut::<Fts3termTable>();
        let mut iIndex: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if argc == 5 as ::core::ffi::c_int {
            iIndex = atoi(*argv.offset(4 as ::core::ffi::c_int as isize));
            argc -= 1;
        }
        *ppVtab = ::core::ptr::null_mut::<sqlite3_vtab>();
        if argc != 4 as ::core::ffi::c_int {
            sqlite3Fts3ErrMsg(
                pzErr,
                b"wrong number of arguments to fts4term constructor\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return SQLITE_ERROR;
        }
        zDb = *argv.offset(1 as ::core::ffi::c_int as isize);
        nDb = strlen(zDb) as ::core::ffi::c_int;
        zFts3 = *argv.offset(3 as ::core::ffi::c_int as isize);
        nFts3 = strlen(zFts3) as ::core::ffi::c_int;
        rc = sqlite3_declare_vtab(db, FTS3_TERMS_SCHEMA.as_ptr());
        if rc != SQLITE_OK {
            return rc;
        }
        nByte = ::core::mem::size_of::<Fts3termTable>() as sqlite3_int64;
        p = sqlite3Fts3MallocZero(nByte as i64_0) as *mut Fts3termTable;
        if p.is_null() {
            return SQLITE_NOMEM;
        }
        (*p).pFts3Tab = sqlite3Fts3MallocZero(
            (::core::mem::size_of::<Fts3Table>() as usize)
                .wrapping_add(nDb as usize)
                .wrapping_add(nFts3 as usize)
                .wrapping_add(2 as usize) as i64_0,
        ) as *mut Fts3Table;
        if (*p).pFts3Tab.is_null() {
            sqlite3_free(p as *mut ::core::ffi::c_void);
            return SQLITE_NOMEM;
        }
        (*(*p).pFts3Tab).zDb = (*p).pFts3Tab.offset(1 as ::core::ffi::c_int as isize)
            as *mut Fts3Table as *mut ::core::ffi::c_char;
        (*(*p).pFts3Tab).zName = (*(*p).pFts3Tab)
            .zDb
            .offset((nDb + 1 as ::core::ffi::c_int) as isize)
            as *const ::core::ffi::c_char;
        (*(*p).pFts3Tab).db = db;
        (*(*p).pFts3Tab).nIndex = iIndex + 1 as ::core::ffi::c_int;
        (*p).iIndex = iIndex;
        memcpy(
            (*(*p).pFts3Tab).zDb as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            zDb as *const ::core::ffi::c_void,
            nDb as size_t,
        );
        memcpy(
            (*(*p).pFts3Tab).zName as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            zFts3 as *const ::core::ffi::c_void,
            nFts3 as size_t,
        );
        sqlite3Fts3Dequote((*(*p).pFts3Tab).zName as *mut ::core::ffi::c_char);
        *ppVtab = p as *mut sqlite3_vtab;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fts3termDisconnectMethod(
    mut pVtab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut Fts3termTable = pVtab as *mut Fts3termTable;
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
        sqlite3_free(pFts3 as *mut ::core::ffi::c_void);
        sqlite3_free(p as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fts3termBestIndexMethod(
    mut pVTab: *mut sqlite3_vtab,
    mut pInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        if (*pInfo).nOrderBy != 0 {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*pInfo).nOrderBy {
                if (*(*pInfo).aOrderBy.offset(i as isize)).iColumn != i
                    || (*(*pInfo).aOrderBy.offset(i as isize)).desc as ::core::ffi::c_int
                        != 0
                {
                    break;
                }
                i += 1;
            }
            if i == (*pInfo).nOrderBy {
                (*pInfo).orderByConsumed = 1 as ::core::ffi::c_int;
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fts3termOpenMethod(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCsr: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut Fts3termCursor = ::core::ptr::null_mut::<Fts3termCursor>();
        pCsr = sqlite3_malloc(
            ::core::mem::size_of::<Fts3termCursor>() as ::core::ffi::c_int,
        ) as *mut Fts3termCursor;
        if pCsr.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pCsr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Fts3termCursor>() as size_t,
        );
        *ppCsr = pCsr as *mut sqlite3_vtab_cursor;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fts3termCloseMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pFts3: *mut Fts3Table = (*((*pCursor).pVtab as *mut Fts3termTable))
            .pFts3Tab;
        let mut pCsr: *mut Fts3termCursor = pCursor as *mut Fts3termCursor;
        sqlite3Fts3SegmentsClose(pFts3);
        sqlite3Fts3SegReaderFinish(&raw mut (*pCsr).csr);
        sqlite3_free(pCsr as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fts3termNextMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut Fts3termCursor = pCursor as *mut Fts3termCursor;
        let mut pFts3: *mut Fts3Table = (*((*pCursor).pVtab as *mut Fts3termTable))
            .pFts3Tab;
        let mut rc: ::core::ffi::c_int = 0;
        let mut v: sqlite3_int64 = 0;
        (*pCsr).iRowid += 1;
        if (*pCsr).csr.aDoclist.is_null()
            || (*pCsr).pNext
                >= (*pCsr)
                    .csr
                    .aDoclist
                    .offset(((*pCsr).csr.nDoclist - 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char
        {
            rc = sqlite3Fts3SegReaderStep(pFts3, &raw mut (*pCsr).csr);
            if rc != SQLITE_ROW {
                (*pCsr).isEof = 1 as ::core::ffi::c_int;
                return rc;
            }
            (*pCsr).iCol = 0 as ::core::ffi::c_int;
            (*pCsr).iPos = 0 as ::core::ffi::c_int;
            (*pCsr).iDocid = 0 as sqlite3_int64;
            (*pCsr).pNext = (*pCsr).csr.aDoclist;
            (*pCsr).pNext = (*pCsr)
                .pNext
                .offset(
                    sqlite3Fts3GetVarint((*pCsr).pNext, &raw mut (*pCsr).iDocid) as isize,
                );
        }
        (*pCsr).pNext = (*pCsr)
            .pNext
            .offset(sqlite3Fts3GetVarint((*pCsr).pNext, &raw mut v) as isize);
        if v == 0 as ::core::ffi::c_longlong {
            (*pCsr).pNext = (*pCsr)
                .pNext
                .offset(sqlite3Fts3GetVarint((*pCsr).pNext, &raw mut v) as isize);
            (*pCsr).iDocid += v as ::core::ffi::c_longlong;
            (*pCsr).pNext = (*pCsr)
                .pNext
                .offset(sqlite3Fts3GetVarint((*pCsr).pNext, &raw mut v) as isize);
            (*pCsr).iCol = 0 as ::core::ffi::c_int;
            (*pCsr).iPos = 0 as ::core::ffi::c_int;
        }
        if v == 1 as ::core::ffi::c_longlong {
            (*pCsr).pNext = (*pCsr)
                .pNext
                .offset(sqlite3Fts3GetVarint((*pCsr).pNext, &raw mut v) as isize);
            (*pCsr).iCol += v as ::core::ffi::c_int;
            (*pCsr).iPos = 0 as ::core::ffi::c_int;
            (*pCsr).pNext = (*pCsr)
                .pNext
                .offset(sqlite3Fts3GetVarint((*pCsr).pNext, &raw mut v) as isize);
        }
        (*pCsr).iPos
            += (v as ::core::ffi::c_longlong - 2 as ::core::ffi::c_longlong)
                as ::core::ffi::c_int;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fts3termFilterMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut Fts3termCursor = pCursor as *mut Fts3termCursor;
        let mut p: *mut Fts3termTable = (*pCursor).pVtab as *mut Fts3termTable;
        let mut pFts3: *mut Fts3Table = (*p).pFts3Tab;
        let mut rc: ::core::ffi::c_int = 0;
        sqlite3Fts3SegReaderFinish(&raw mut (*pCsr).csr);
        memset(
            &raw mut (*pCsr).csr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (pCsr.offset(1 as ::core::ffi::c_int as isize) as *mut Fts3termCursor
                as *mut u8_0)
                .offset_from(&raw mut (*pCsr).csr as *mut u8_0) as ::core::ffi::c_long
                as size_t,
        );
        (*pCsr).filter.flags = FTS3_SEGMENT_REQUIRE_POS | FTS3_SEGMENT_IGNORE_EMPTY;
        (*pCsr).filter.flags |= FTS3_SEGMENT_SCAN;
        rc = sqlite3Fts3SegReaderCursor(
            pFts3,
            0 as ::core::ffi::c_int,
            (*p).iIndex,
            FTS3_SEGCURSOR_ALL,
            (*pCsr).filter.zTerm,
            (*pCsr).filter.nTerm,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            &raw mut (*pCsr).csr,
        );
        if rc == SQLITE_OK {
            rc = sqlite3Fts3SegReaderStart(
                pFts3,
                &raw mut (*pCsr).csr,
                &raw mut (*pCsr).filter,
            );
        }
        if rc == SQLITE_OK {
            rc = fts3termNextMethod(pCursor);
        }
        return rc;
    }
}
unsafe extern "C" fn fts3termEofMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut Fts3termCursor = pCursor as *mut Fts3termCursor;
        return (*pCsr).isEof;
    }
}
unsafe extern "C" fn fts3termColumnMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pCtx: *mut sqlite3_context,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut Fts3termCursor = pCursor as *mut Fts3termCursor;
        match iCol {
            0 => {
                sqlite3_result_text(
                    pCtx,
                    (*p).csr.zTerm,
                    (*p).csr.nTerm,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            }
            1 => {
                sqlite3_result_int64(pCtx, (*p).iDocid);
            }
            2 => {
                sqlite3_result_int64(pCtx, (*p).iCol as sqlite3_int64);
            }
            _ => {
                sqlite3_result_int64(pCtx, (*p).iPos as sqlite3_int64);
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fts3termRowidMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut Fts3termCursor = pCursor as *mut Fts3termCursor;
        *pRowid = (*pCsr).iRowid as sqlite_int64;
        return SQLITE_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3Fts3InitTerm(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        static mut fts3term_module: sqlite3_module = unsafe {
            sqlite3_module {
                iVersion: 0 as ::core::ffi::c_int,
                xCreate: Some(
                    fts3termConnectMethod
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
                    fts3termConnectMethod
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
                    fts3termBestIndexMethod
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            *mut sqlite3_index_info,
                        ) -> ::core::ffi::c_int,
                ),
                xDisconnect: Some(
                    fts3termDisconnectMethod
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xDestroy: Some(
                    fts3termDisconnectMethod
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xOpen: Some(
                    fts3termOpenMethod
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            *mut *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xClose: Some(
                    fts3termCloseMethod
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xFilter: Some(
                    fts3termFilterMethod
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> ::core::ffi::c_int,
                ),
                xNext: Some(
                    fts3termNextMethod
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xEof: Some(
                    fts3termEofMethod
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xColumn: Some(
                    fts3termColumnMethod
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xRowid: Some(
                    fts3termRowidMethod
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
            b"fts4term\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const fts3term_module,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        return rc;
    }
}
