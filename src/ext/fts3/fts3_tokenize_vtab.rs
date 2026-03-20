
extern "C" {
    pub type sqlite3;
    pub type sqlite3_value;
    pub type sqlite3_context;
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
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_create_module_v2(
        db: *mut sqlite3,
        zName: *const ::core::ffi::c_char,
        p: *const sqlite3_module,
        pClientData: *mut ::core::ffi::c_void,
        xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_declare_vtab(
        _: *mut sqlite3,
        zSQL: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3HashFind(
        _: *const Fts3Hash,
        pKey: *const ::core::ffi::c_void,
        nKey: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3Fts3ErrMsg(_: *mut *mut ::core::ffi::c_char, _: *const ::core::ffi::c_char, ...);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3tokCursor {
    pub base: sqlite3_vtab_cursor,
    pub zInput: *mut ::core::ffi::c_char,
    pub pCsr: *mut sqlite3_tokenizer_cursor,
    pub iRowid: ::core::ffi::c_int,
    pub zToken: *const ::core::ffi::c_char,
    pub nToken: ::core::ffi::c_int,
    pub iStart: ::core::ffi::c_int,
    pub iEnd: ::core::ffi::c_int,
    pub iPos: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3tokTable {
    pub base: sqlite3_vtab,
    pub pMod: *const sqlite3_tokenizer_module,
    pub pTok: *mut sqlite3_tokenizer,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn fts3tokQueryTokenizer(
    mut pHash: *mut Fts3Hash,
    mut zName: *const ::core::ffi::c_char,
    mut pp: *mut *const sqlite3_tokenizer_module,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut sqlite3_tokenizer_module = ::core::ptr::null_mut::<sqlite3_tokenizer_module>();
    let mut nName: ::core::ffi::c_int = strlen(zName) as ::core::ffi::c_int;
    p = sqlite3Fts3HashFind(
        pHash,
        zName as *const ::core::ffi::c_void,
        nName + 1 as ::core::ffi::c_int,
    ) as *mut sqlite3_tokenizer_module;
    if p.is_null() {
        sqlite3Fts3ErrMsg(
            pzErr,
            b"unknown tokenizer: %s\0" as *const u8 as *const ::core::ffi::c_char,
            zName,
        );
        return SQLITE_ERROR;
    }
    *pp = p;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3tokDequoteArray(
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut pazDequote: *mut *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if argc == 0 as ::core::ffi::c_int {
        *pazDequote = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    } else {
        let mut i: ::core::ffi::c_int = 0;
        let mut nByte: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut azDequote: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            nByte +=
                strlen(*argv.offset(i as isize)).wrapping_add(1 as size_t) as ::core::ffi::c_int;
            i += 1;
        }
        azDequote = sqlite3_malloc64(
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                .wrapping_mul(argc as usize)
                .wrapping_add(nByte as usize) as sqlite3_uint64,
        ) as *mut *mut ::core::ffi::c_char;
        *pazDequote = azDequote;
        if azDequote.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            let mut pSpace: *mut ::core::ffi::c_char = azDequote.offset(argc as isize)
                as *mut *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            i = 0 as ::core::ffi::c_int;
            while i < argc {
                let mut n: ::core::ffi::c_int =
                    strlen(*argv.offset(i as isize)) as ::core::ffi::c_int;
                let ref mut fresh0 = *azDequote.offset(i as isize);
                *fresh0 = pSpace;
                memcpy(
                    pSpace as *mut ::core::ffi::c_void,
                    *argv.offset(i as isize) as *const ::core::ffi::c_void,
                    (n + 1 as ::core::ffi::c_int) as size_t,
                );
                sqlite3Fts3Dequote(pSpace);
                pSpace = pSpace.offset((n + 1 as ::core::ffi::c_int) as isize);
                i += 1;
            }
        }
    }
    return rc;
}
pub const FTS3_TOK_SCHEMA: [::core::ffi::c_char; 51] = unsafe {
    ::core::mem::transmute::<[u8; 51], [::core::ffi::c_char; 51]>(
        *b"CREATE TABLE x(input, token, start, end, position)\0",
    )
};
unsafe extern "C" fn fts3tokConnectMethod(
    mut db: *mut sqlite3,
    mut pHash: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pTab: *mut Fts3tokTable = ::core::ptr::null_mut::<Fts3tokTable>();
    let mut pMod: *const sqlite3_tokenizer_module = ::core::ptr::null::<sqlite3_tokenizer_module>();
    let mut pTok: *mut sqlite3_tokenizer = ::core::ptr::null_mut::<sqlite3_tokenizer>();
    let mut rc: ::core::ffi::c_int = 0;
    let mut azDequote: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut nDequote: ::core::ffi::c_int = 0;
    rc = sqlite3_declare_vtab(db, FTS3_TOK_SCHEMA.as_ptr());
    if rc != SQLITE_OK {
        return rc;
    }
    nDequote = argc - 3 as ::core::ffi::c_int;
    rc = fts3tokDequoteArray(
        nDequote,
        argv.offset(3 as ::core::ffi::c_int as isize) as *const *const ::core::ffi::c_char,
        &raw mut azDequote,
    );
    if rc == SQLITE_OK {
        let mut zModule: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if nDequote < 1 as ::core::ffi::c_int {
            zModule = b"simple\0" as *const u8 as *const ::core::ffi::c_char;
        } else {
            zModule = *azDequote.offset(0 as ::core::ffi::c_int as isize);
        }
        rc = fts3tokQueryTokenizer(pHash as *mut Fts3Hash, zModule, &raw mut pMod, pzErr);
    }
    if rc == SQLITE_OK {
        let mut azArg: *const *const ::core::ffi::c_char =
            ::core::ptr::null::<*const ::core::ffi::c_char>();
        if nDequote > 1 as ::core::ffi::c_int {
            azArg = azDequote.offset(1 as ::core::ffi::c_int as isize)
                as *mut *mut ::core::ffi::c_char
                as *const *const ::core::ffi::c_char;
        }
        rc = (*pMod).xCreate.expect("non-null function pointer")(
            if nDequote > 1 as ::core::ffi::c_int {
                nDequote - 1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
            azArg,
            &raw mut pTok,
        );
    }
    if rc == SQLITE_OK {
        pTab = sqlite3_malloc(::core::mem::size_of::<Fts3tokTable>() as ::core::ffi::c_int)
            as *mut Fts3tokTable;
        if pTab.is_null() {
            rc = SQLITE_NOMEM;
        }
    }
    if rc == SQLITE_OK {
        memset(
            pTab as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Fts3tokTable>() as size_t,
        );
        (*pTab).pMod = pMod;
        (*pTab).pTok = pTok;
        *ppVtab = &raw mut (*pTab).base;
    } else if !pTok.is_null() {
        (*pMod).xDestroy.expect("non-null function pointer")(pTok);
    }
    sqlite3_free(azDequote as *mut ::core::ffi::c_void);
    return rc;
}
unsafe extern "C" fn fts3tokDisconnectMethod(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    let mut pTab: *mut Fts3tokTable = pVtab as *mut Fts3tokTable;
    (*(*pTab).pMod).xDestroy.expect("non-null function pointer")((*pTab).pTok);
    sqlite3_free(pTab as *mut ::core::ffi::c_void);
    return SQLITE_OK;
}
unsafe extern "C" fn fts3tokBestIndexMethod(
    mut _pVTab: *mut sqlite3_vtab,
    mut pInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pInfo).nConstraint {
        if (*(*pInfo).aConstraint.offset(i as isize)).usable as ::core::ffi::c_int != 0
            && (*(*pInfo).aConstraint.offset(i as isize)).iColumn == 0 as ::core::ffi::c_int
            && (*(*pInfo).aConstraint.offset(i as isize)).op as ::core::ffi::c_int
                == SQLITE_INDEX_CONSTRAINT_EQ
        {
            (*pInfo).idxNum = 1 as ::core::ffi::c_int;
            (*(*pInfo).aConstraintUsage.offset(i as isize)).argvIndex = 1 as ::core::ffi::c_int;
            (*(*pInfo).aConstraintUsage.offset(i as isize)).omit = 1 as ::core::ffi::c_uchar;
            (*pInfo).estimatedCost = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            return SQLITE_OK;
        }
        i += 1;
    }
    (*pInfo).idxNum = 0 as ::core::ffi::c_int;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3tokOpenMethod(
    mut _pVTab: *mut sqlite3_vtab,
    mut ppCsr: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3tokCursor = ::core::ptr::null_mut::<Fts3tokCursor>();
    pCsr = sqlite3_malloc(::core::mem::size_of::<Fts3tokCursor>() as ::core::ffi::c_int)
        as *mut Fts3tokCursor;
    if pCsr.is_null() {
        return SQLITE_NOMEM;
    }
    memset(
        pCsr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Fts3tokCursor>() as size_t,
    );
    *ppCsr = pCsr as *mut sqlite3_vtab_cursor;
    return SQLITE_OK;
}
unsafe extern "C" fn fts3tokResetCursor(mut pCsr: *mut Fts3tokCursor) {
    if !(*pCsr).pCsr.is_null() {
        let mut pTab: *mut Fts3tokTable = (*pCsr).base.pVtab as *mut Fts3tokTable;
        (*(*pTab).pMod).xClose.expect("non-null function pointer")((*pCsr).pCsr);
        (*pCsr).pCsr = ::core::ptr::null_mut::<sqlite3_tokenizer_cursor>();
    }
    sqlite3_free((*pCsr).zInput as *mut ::core::ffi::c_void);
    (*pCsr).zInput = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*pCsr).zToken = ::core::ptr::null::<::core::ffi::c_char>();
    (*pCsr).nToken = 0 as ::core::ffi::c_int;
    (*pCsr).iStart = 0 as ::core::ffi::c_int;
    (*pCsr).iEnd = 0 as ::core::ffi::c_int;
    (*pCsr).iPos = 0 as ::core::ffi::c_int;
    (*pCsr).iRowid = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3tokCloseMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    fts3tokResetCursor(pCsr);
    sqlite3_free(pCsr as *mut ::core::ffi::c_void);
    return SQLITE_OK;
}
unsafe extern "C" fn fts3tokNextMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    let mut pTab: *mut Fts3tokTable = (*pCursor).pVtab as *mut Fts3tokTable;
    let mut rc: ::core::ffi::c_int = 0;
    (*pCsr).iRowid += 1;
    rc = (*(*pTab).pMod).xNext.expect("non-null function pointer")(
        (*pCsr).pCsr,
        &raw mut (*pCsr).zToken,
        &raw mut (*pCsr).nToken,
        &raw mut (*pCsr).iStart,
        &raw mut (*pCsr).iEnd,
        &raw mut (*pCsr).iPos,
    );
    if rc != SQLITE_OK {
        fts3tokResetCursor(pCsr);
        if rc == SQLITE_DONE {
            rc = SQLITE_OK;
        }
    }
    return rc;
}
unsafe extern "C" fn fts3tokFilterMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut _idxStr: *const ::core::ffi::c_char,
    mut _nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_ERROR;
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    let mut pTab: *mut Fts3tokTable = (*pCursor).pVtab as *mut Fts3tokTable;
    fts3tokResetCursor(pCsr);
    if idxNum == 1 as ::core::ffi::c_int {
        let mut zByte: *const ::core::ffi::c_char =
            sqlite3_value_text(*apVal.offset(0 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char;
        let mut nByte: sqlite3_int64 =
            sqlite3_value_bytes(*apVal.offset(0 as ::core::ffi::c_int as isize)) as sqlite3_int64;
        (*pCsr).zInput = sqlite3_malloc64((nByte + 1 as sqlite3_int64) as sqlite3_uint64)
            as *mut ::core::ffi::c_char;
        if (*pCsr).zInput.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            if nByte > 0 as sqlite3_int64 {
                memcpy(
                    (*pCsr).zInput as *mut ::core::ffi::c_void,
                    zByte as *const ::core::ffi::c_void,
                    nByte as size_t,
                );
            }
            *(*pCsr).zInput.offset(nByte as isize) = 0 as ::core::ffi::c_char;
            rc = (*(*pTab).pMod).xOpen.expect("non-null function pointer")(
                (*pTab).pTok,
                (*pCsr).zInput,
                nByte as ::core::ffi::c_int,
                &raw mut (*pCsr).pCsr,
            );
            if rc == SQLITE_OK {
                (*(*pCsr).pCsr).pTokenizer = (*pTab).pTok;
            }
        }
    }
    if rc != SQLITE_OK {
        return rc;
    }
    return fts3tokNextMethod(pCursor);
}
unsafe extern "C" fn fts3tokEofMethod(mut pCursor: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    return ((*pCsr).zToken == ::core::ptr::null::<::core::ffi::c_char>()) as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3tokColumnMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pCtx: *mut sqlite3_context,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    match iCol {
        0 => {
            sqlite3_result_text(
                pCtx,
                (*pCsr).zInput,
                -(1 as ::core::ffi::c_int),
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
            );
        }
        1 => {
            sqlite3_result_text(
                pCtx,
                (*pCsr).zToken,
                (*pCsr).nToken,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
            );
        }
        2 => {
            sqlite3_result_int(pCtx, (*pCsr).iStart);
        }
        3 => {
            sqlite3_result_int(pCtx, (*pCsr).iEnd);
        }
        _ => {
            sqlite3_result_int(pCtx, (*pCsr).iPos);
        }
    }
    return SQLITE_OK;
}
unsafe extern "C" fn fts3tokRowidMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    *pRowid = (*pCsr).iRowid as sqlite3_int64 as sqlite_int64;
    return SQLITE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3InitTok(
    mut db: *mut sqlite3,
    mut pHash: *mut Fts3Hash,
    mut xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    static mut fts3tok_module: sqlite3_module = unsafe {
        sqlite3_module {
            iVersion: 0 as ::core::ffi::c_int,
            xCreate: Some(
                fts3tokConnectMethod
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
                fts3tokConnectMethod
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
                fts3tokBestIndexMethod
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab,
                        *mut sqlite3_index_info,
                    ) -> ::core::ffi::c_int,
            ),
            xDisconnect: Some(
                fts3tokDisconnectMethod
                    as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
            ),
            xDestroy: Some(
                fts3tokDisconnectMethod
                    as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
            ),
            xOpen: Some(
                fts3tokOpenMethod
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab,
                        *mut *mut sqlite3_vtab_cursor,
                    ) -> ::core::ffi::c_int,
            ),
            xClose: Some(
                fts3tokCloseMethod
                    as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
            xFilter: Some(
                fts3tokFilterMethod
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab_cursor,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> ::core::ffi::c_int,
            ),
            xNext: Some(
                fts3tokNextMethod
                    as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
            xEof: Some(
                fts3tokEofMethod
                    as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
            xColumn: Some(
                fts3tokColumnMethod
                    as unsafe extern "C" fn(
                        *mut sqlite3_vtab_cursor,
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            xRowid: Some(
                fts3tokRowidMethod
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
    rc = sqlite3_create_module_v2(
        db,
        b"fts3tokenize\0" as *const u8 as *const ::core::ffi::c_char,
        &raw const fts3tok_module,
        pHash as *mut ::core::ffi::c_void,
        xDestroy,
    );
    return rc;
}
