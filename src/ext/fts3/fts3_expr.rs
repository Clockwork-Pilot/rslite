
extern "C" {
    pub type sqlite3;
    pub type sqlite3_value;
    pub type sqlite3_context;
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_create_function(
        db: *mut sqlite3,
        zFunctionName: *const ::core::ffi::c_char,
        nArg: ::core::ffi::c_int,
        eTextRep: ::core::ffi::c_int,
        pApp: *mut ::core::ffi::c_void,
        xFunc: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        xStep: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        xFinal: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_error_nomem(_: *mut sqlite3_context);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3ErrMsg(_: *mut *mut ::core::ffi::c_char, _: *const ::core::ffi::c_char, ...);
    fn sqlite3Fts3ReadInt(
        z: *const ::core::ffi::c_char,
        pnOut: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3InitTokenizer(
        pHash: *mut Fts3Hash,
        _: *const ::core::ffi::c_char,
        _: *mut *mut sqlite3_tokenizer,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3EvalPhraseCleanup(_: *mut Fts3Phrase);
}
pub type size_t = usize;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
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
pub struct ParseContext {
    pub pTokenizer: *mut sqlite3_tokenizer,
    pub iLangid: ::core::ffi::c_int,
    pub azCol: *mut *const ::core::ffi::c_char,
    pub bFts4: ::core::ffi::c_int,
    pub nCol: ::core::ffi::c_int,
    pub iDefaultCol: ::core::ffi::c_int,
    pub isNot: ::core::ffi::c_int,
    pub pCtx: *mut sqlite3_context,
    pub nNest: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Keyword {
    pub z: *mut ::core::ffi::c_char,
    pub n: ::core::ffi::c_uchar,
    pub parenOnly: ::core::ffi::c_uchar,
    pub eType: ::core::ffi::c_uchar,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_FTS3_MAX_EXPR_DEPTH: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const FTSQUERY_NEAR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FTSQUERY_NOT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FTSQUERY_AND: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const FTSQUERY_OR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FTSQUERY_PHRASE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[no_mangle]
pub static mut sqlite3_fts3_enable_parentheses: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_FTS3_DEFAULT_NEAR_PARAM: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
unsafe extern "C" fn fts3isspace(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int == ' ' as i32
        || c as ::core::ffi::c_int == '\t' as i32
        || c as ::core::ffi::c_int == '\n' as i32
        || c as ::core::ffi::c_int == '\r' as i32
        || c as ::core::ffi::c_int == '\u{b}' as i32
        || c as ::core::ffi::c_int == '\u{c}' as i32) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3MallocZero(
    mut nByte: sqlite3_int64,
) -> *mut ::core::ffi::c_void {
    let mut pRet: *mut ::core::ffi::c_void = sqlite3_malloc64(nByte as sqlite3_uint64);
    if !pRet.is_null() {
        memset(pRet, 0 as ::core::ffi::c_int, nByte as size_t);
    }
    return pRet;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3OpenTokenizer(
    mut pTokenizer: *mut sqlite3_tokenizer,
    mut iLangid: ::core::ffi::c_int,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut ppCsr: *mut *mut sqlite3_tokenizer_cursor,
) -> ::core::ffi::c_int {
    let mut pModule: *const sqlite3_tokenizer_module = (*pTokenizer).pModule;
    let mut pCsr: *mut sqlite3_tokenizer_cursor =
        ::core::ptr::null_mut::<sqlite3_tokenizer_cursor>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = (*pModule).xOpen.expect("non-null function pointer")(pTokenizer, z, n, &raw mut pCsr);
    if rc == SQLITE_OK {
        (*pCsr).pTokenizer = pTokenizer;
        if (*pModule).iVersion >= 1 as ::core::ffi::c_int {
            rc = (*pModule).xLanguageid.expect("non-null function pointer")(pCsr, iLangid);
            if rc != SQLITE_OK {
                (*pModule).xClose.expect("non-null function pointer")(pCsr);
                pCsr = ::core::ptr::null_mut::<sqlite3_tokenizer_cursor>();
            }
        }
    }
    *ppCsr = pCsr;
    return rc;
}
unsafe extern "C" fn findBarredChar(
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ii: ::core::ffi::c_int = 0;
    ii = 0 as ::core::ffi::c_int;
    while ii < n {
        if *z.offset(ii as isize) as ::core::ffi::c_int == '"' as i32
            || sqlite3_fts3_enable_parentheses != 0
                && (*z.offset(ii as isize) as ::core::ffi::c_int == '(' as i32
                    || *z.offset(ii as isize) as ::core::ffi::c_int == ')' as i32)
        {
            return ii;
        }
        ii += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn getNextToken(
    mut pParse: *mut ParseContext,
    mut iCol: ::core::ffi::c_int,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut ppExpr: *mut *mut Fts3Expr,
    mut pnConsumed: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pTokenizer: *mut sqlite3_tokenizer = (*pParse).pTokenizer;
    let mut pModule: *const sqlite3_tokenizer_module = (*pTokenizer).pModule;
    let mut rc: ::core::ffi::c_int = 0;
    let mut pCursor: *mut sqlite3_tokenizer_cursor =
        ::core::ptr::null_mut::<sqlite3_tokenizer_cursor>();
    let mut pRet: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
    *pnConsumed = n;
    rc = sqlite3Fts3OpenTokenizer(pTokenizer, (*pParse).iLangid, z, n, &raw mut pCursor);
    if rc == SQLITE_OK {
        let mut zToken: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iStart: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iEnd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iPosition: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nByte: sqlite3_int64 = 0;
        rc = (*pModule).xNext.expect("non-null function pointer")(
            pCursor,
            &raw mut zToken,
            &raw mut nToken,
            &raw mut iStart,
            &raw mut iEnd,
            &raw mut iPosition,
        );
        if rc == SQLITE_OK {
            let mut iBarred: ::core::ffi::c_int = findBarredChar(z, iEnd);
            if iBarred >= 0 as ::core::ffi::c_int {
                (*pModule).xClose.expect("non-null function pointer")(pCursor);
                return getNextToken(pParse, iCol, z, iBarred, ppExpr, pnConsumed);
            }
            nByte = (::core::mem::size_of::<Fts3Expr>() as usize)
                .wrapping_add((88 as usize).wrapping_add(
                    (1 as usize).wrapping_mul(::core::mem::size_of::<Fts3PhraseToken>() as usize),
                ))
                .wrapping_add(nToken as usize) as sqlite3_int64;
            pRet = sqlite3Fts3MallocZero(nByte) as *mut Fts3Expr;
            if pRet.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                (*pRet).eType = FTSQUERY_PHRASE;
                (*pRet).pPhrase = pRet.offset(1 as ::core::ffi::c_int as isize) as *mut Fts3Expr
                    as *mut Fts3Phrase;
                (*(*pRet).pPhrase).nToken = 1 as ::core::ffi::c_int;
                (*(*pRet).pPhrase).iColumn = iCol;
                (*(&raw mut (*(*pRet).pPhrase).aToken as *mut Fts3PhraseToken)
                    .offset(0 as ::core::ffi::c_int as isize))
                .n = nToken;
                let ref mut fresh0 = (*(&raw mut (*(*pRet).pPhrase).aToken
                    as *mut Fts3PhraseToken)
                    .offset(0 as ::core::ffi::c_int as isize))
                .z;
                *fresh0 = (&raw mut (*(*pRet).pPhrase).aToken as *mut Fts3PhraseToken)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut Fts3PhraseToken as *mut ::core::ffi::c_char;
                memcpy(
                    (*(&raw mut (*(*pRet).pPhrase).aToken as *mut Fts3PhraseToken)
                        .offset(0 as ::core::ffi::c_int as isize))
                    .z as *mut ::core::ffi::c_void,
                    zToken as *const ::core::ffi::c_void,
                    nToken as size_t,
                );
                if iEnd < n && *z.offset(iEnd as isize) as ::core::ffi::c_int == '*' as i32 {
                    (*(&raw mut (*(*pRet).pPhrase).aToken as *mut Fts3PhraseToken)
                        .offset(0 as ::core::ffi::c_int as isize))
                    .isPrefix = 1 as ::core::ffi::c_int;
                    iEnd += 1;
                }
                loop {
                    if sqlite3_fts3_enable_parentheses == 0
                        && iStart > 0 as ::core::ffi::c_int
                        && *z.offset((iStart - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == '-' as i32
                    {
                        (*pParse).isNot = 1 as ::core::ffi::c_int;
                        iStart -= 1;
                    } else {
                        if !((*pParse).bFts4 != 0
                            && iStart > 0 as ::core::ffi::c_int
                            && *z.offset((iStart - 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int
                                == '^' as i32)
                        {
                            break;
                        }
                        (*(&raw mut (*(*pRet).pPhrase).aToken as *mut Fts3PhraseToken)
                            .offset(0 as ::core::ffi::c_int as isize))
                        .bFirst = 1 as ::core::ffi::c_int;
                        iStart -= 1;
                    }
                }
            }
            *pnConsumed = iEnd;
        } else if n != 0 && rc == SQLITE_DONE {
            let mut iBarred_0: ::core::ffi::c_int = findBarredChar(z, n);
            if iBarred_0 >= 0 as ::core::ffi::c_int {
                *pnConsumed = iBarred_0;
            }
            rc = SQLITE_OK;
        }
        (*pModule).xClose.expect("non-null function pointer")(pCursor);
    }
    *ppExpr = pRet;
    return rc;
}
unsafe extern "C" fn fts3ReallocOrFree(
    mut pOrig: *mut ::core::ffi::c_void,
    mut nNew: sqlite3_int64,
) -> *mut ::core::ffi::c_void {
    let mut pRet: *mut ::core::ffi::c_void = sqlite3_realloc64(pOrig, nNew as sqlite3_uint64);
    if pRet.is_null() {
        sqlite3_free(pOrig);
    }
    return pRet;
}
unsafe extern "C" fn getNextString(
    mut pParse: *mut ParseContext,
    mut zInput: *const ::core::ffi::c_char,
    mut nInput: ::core::ffi::c_int,
    mut ppExpr: *mut *mut Fts3Expr,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut pTokenizer: *mut sqlite3_tokenizer = (*pParse).pTokenizer;
    let mut pModule: *const sqlite3_tokenizer_module = (*pTokenizer).pModule;
    let mut rc: ::core::ffi::c_int = 0;
    let mut p: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
    let mut pCursor: *mut sqlite3_tokenizer_cursor =
        ::core::ptr::null_mut::<sqlite3_tokenizer_cursor>();
    let mut zTemp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nTemp: i64_0 = 0 as i64_0;
    let nSpace: ::core::ffi::c_int =
        (::core::mem::size_of::<Fts3Expr>() as usize).wrapping_add((88 as usize).wrapping_add(
            (1 as usize).wrapping_mul(::core::mem::size_of::<Fts3PhraseToken>() as usize),
        )) as ::core::ffi::c_int;
    let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rc = sqlite3Fts3OpenTokenizer(
        pTokenizer,
        (*pParse).iLangid,
        zInput,
        nInput,
        &raw mut pCursor,
    );
    if rc == SQLITE_OK {
        let mut ii: ::core::ffi::c_int = 0;
        ii = 0 as ::core::ffi::c_int;
        loop {
            if !(rc == SQLITE_OK) {
                current_block = 4808432441040389987;
                break;
            }
            let mut zByte: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            let mut nByte: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut iBegin: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut iEnd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut iPos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            rc = (*pModule).xNext.expect("non-null function pointer")(
                pCursor,
                &raw mut zByte,
                &raw mut nByte,
                &raw mut iBegin,
                &raw mut iEnd,
                &raw mut iPos,
            );
            if rc == SQLITE_OK {
                let mut pToken: *mut Fts3PhraseToken = ::core::ptr::null_mut::<Fts3PhraseToken>();
                p = fts3ReallocOrFree(
                    p as *mut ::core::ffi::c_void,
                    (nSpace as usize).wrapping_add(
                        (ii as usize)
                            .wrapping_mul(::core::mem::size_of::<Fts3PhraseToken>() as usize),
                    ) as sqlite3_int64,
                ) as *mut Fts3Expr;
                zTemp = fts3ReallocOrFree(
                    zTemp as *mut ::core::ffi::c_void,
                    nTemp as sqlite3_int64 + nByte as sqlite3_int64,
                ) as *mut ::core::ffi::c_char;
                if zTemp.is_null() || p.is_null() {
                    rc = SQLITE_NOMEM;
                    current_block = 12690954132298350795;
                    break;
                } else {
                    pToken = (&raw mut (*(p.offset(1 as ::core::ffi::c_int as isize)
                        as *mut Fts3Expr
                        as *mut Fts3Phrase))
                        .aToken as *mut Fts3PhraseToken)
                        .offset(ii as isize) as *mut Fts3PhraseToken;
                    memset(
                        pToken as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<Fts3PhraseToken>() as size_t,
                    );
                    memcpy(
                        zTemp.offset(nTemp as isize) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        zByte as *const ::core::ffi::c_void,
                        nByte as size_t,
                    );
                    nTemp += nByte as i64_0;
                    (*pToken).n = nByte;
                    (*pToken).isPrefix = (iEnd < nInput
                        && *zInput.offset(iEnd as isize) as ::core::ffi::c_int == '*' as i32)
                        as ::core::ffi::c_int;
                    (*pToken).bFirst = (iBegin > 0 as ::core::ffi::c_int
                        && *zInput.offset((iBegin - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == '^' as i32)
                        as ::core::ffi::c_int;
                    nToken = ii + 1 as ::core::ffi::c_int;
                }
            }
            ii += 1;
        }
    } else {
        current_block = 4808432441040389987;
    }
    match current_block {
        4808432441040389987 => {
            if rc == SQLITE_DONE {
                let mut jj: ::core::ffi::c_int = 0;
                let mut zBuf: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                p = fts3ReallocOrFree(
                    p as *mut ::core::ffi::c_void,
                    ((nSpace as usize).wrapping_add(
                        (nToken as usize)
                            .wrapping_mul(::core::mem::size_of::<Fts3PhraseToken>() as usize),
                    ) as ::core::ffi::c_ulonglong)
                        .wrapping_add(nTemp as ::core::ffi::c_ulonglong)
                        as sqlite3_int64,
                ) as *mut Fts3Expr;
                if p.is_null() {
                    rc = SQLITE_NOMEM;
                } else {
                    memset(
                        p as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ((&raw mut (*(p.offset(1 as ::core::ffi::c_int as isize) as *mut Fts3Expr
                            as *mut Fts3Phrase))
                            .aToken as *mut Fts3PhraseToken)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut Fts3PhraseToken
                            as *mut ::core::ffi::c_char)
                            .offset_from(p as *mut ::core::ffi::c_char)
                            as ::core::ffi::c_long as size_t,
                    );
                    (*p).eType = FTSQUERY_PHRASE;
                    (*p).pPhrase = p.offset(1 as ::core::ffi::c_int as isize) as *mut Fts3Expr
                        as *mut Fts3Phrase;
                    (*(*p).pPhrase).iColumn = (*pParse).iDefaultCol;
                    (*(*p).pPhrase).nToken = nToken;
                    zBuf = (&raw mut (*(*p).pPhrase).aToken as *mut Fts3PhraseToken)
                        .offset(nToken as isize) as *mut Fts3PhraseToken
                        as *mut ::core::ffi::c_char;
                    if !zTemp.is_null() {
                        memcpy(
                            zBuf as *mut ::core::ffi::c_void,
                            zTemp as *const ::core::ffi::c_void,
                            nTemp as size_t,
                        );
                    }
                    jj = 0 as ::core::ffi::c_int;
                    while jj < (*(*p).pPhrase).nToken {
                        let ref mut fresh1 = (*(&raw mut (*(*p).pPhrase).aToken
                            as *mut Fts3PhraseToken)
                            .offset(jj as isize))
                        .z;
                        *fresh1 = zBuf;
                        zBuf = zBuf.offset(
                            (*(&raw mut (*(*p).pPhrase).aToken as *mut Fts3PhraseToken)
                                .offset(jj as isize))
                            .n as isize,
                        );
                        jj += 1;
                    }
                    rc = SQLITE_OK;
                }
            }
        }
        _ => {}
    }
    if !pCursor.is_null() {
        (*pModule).xClose.expect("non-null function pointer")(pCursor);
    }
    sqlite3_free(zTemp as *mut ::core::ffi::c_void);
    if rc != SQLITE_OK {
        sqlite3_free(p as *mut ::core::ffi::c_void);
        p = ::core::ptr::null_mut::<Fts3Expr>();
    }
    *ppExpr = p;
    return rc;
}
unsafe extern "C" fn getNextNode(
    mut pParse: *mut ParseContext,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut ppExpr: *mut *mut Fts3Expr,
    mut pnConsumed: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    static mut aKeyword: [Fts3Keyword; 4] = [
        Fts3Keyword {
            z: b"OR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n: 2 as ::core::ffi::c_uchar,
            parenOnly: 0 as ::core::ffi::c_uchar,
            eType: FTSQUERY_OR as ::core::ffi::c_uchar,
        },
        Fts3Keyword {
            z: b"AND\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n: 3 as ::core::ffi::c_uchar,
            parenOnly: 1 as ::core::ffi::c_uchar,
            eType: FTSQUERY_AND as ::core::ffi::c_uchar,
        },
        Fts3Keyword {
            z: b"NOT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n: 3 as ::core::ffi::c_uchar,
            parenOnly: 1 as ::core::ffi::c_uchar,
            eType: FTSQUERY_NOT as ::core::ffi::c_uchar,
        },
        Fts3Keyword {
            z: b"NEAR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n: 4 as ::core::ffi::c_uchar,
            parenOnly: 0 as ::core::ffi::c_uchar,
            eType: FTSQUERY_NEAR as ::core::ffi::c_uchar,
        },
    ];
    let mut ii: ::core::ffi::c_int = 0;
    let mut iCol: ::core::ffi::c_int = 0;
    let mut iColLen: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0;
    let mut pRet: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
    let mut zInput: *const ::core::ffi::c_char = z;
    let mut nInput: ::core::ffi::c_int = n;
    (*pParse).isNot = 0 as ::core::ffi::c_int;
    while nInput > 0 as ::core::ffi::c_int && fts3isspace(*zInput) != 0 {
        nInput -= 1;
        zInput = zInput.offset(1);
    }
    if nInput == 0 as ::core::ffi::c_int {
        return SQLITE_DONE;
    }
    ii = 0 as ::core::ffi::c_int;
    while ii
        < (::core::mem::size_of::<[Fts3Keyword; 4]>() as usize)
            .wrapping_div(::core::mem::size_of::<Fts3Keyword>() as usize)
            as ::core::ffi::c_int
    {
        let mut pKey: *const Fts3Keyword =
            (&raw const aKeyword as *const Fts3Keyword).offset(ii as isize) as *const Fts3Keyword;
        if !((*pKey).parenOnly as ::core::ffi::c_int & !sqlite3_fts3_enable_parentheses
            != 0 as ::core::ffi::c_int)
        {
            if nInput >= (*pKey).n as ::core::ffi::c_int
                && 0 as ::core::ffi::c_int
                    == memcmp(
                        zInput as *const ::core::ffi::c_void,
                        (*pKey).z as *const ::core::ffi::c_void,
                        (*pKey).n as size_t,
                    )
            {
                let mut nNear: ::core::ffi::c_int = SQLITE_FTS3_DEFAULT_NEAR_PARAM;
                let mut nKey: ::core::ffi::c_int = (*pKey).n as ::core::ffi::c_int;
                let mut cNext: ::core::ffi::c_char = 0;
                if (*pKey).eType as ::core::ffi::c_int == FTSQUERY_NEAR {
                    if *zInput.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '/' as i32
                        && *zInput.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            >= '0' as i32
                        && *zInput.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            <= '9' as i32
                    {
                        nKey += 1 as ::core::ffi::c_int
                            + sqlite3Fts3ReadInt(
                                zInput.offset((nKey + 1 as ::core::ffi::c_int) as isize)
                                    as *const ::core::ffi::c_char,
                                &raw mut nNear,
                            );
                    }
                }
                cNext = *zInput.offset(nKey as isize);
                if fts3isspace(cNext) != 0
                    || cNext as ::core::ffi::c_int == '"' as i32
                    || cNext as ::core::ffi::c_int == '(' as i32
                    || cNext as ::core::ffi::c_int == ')' as i32
                    || cNext as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    pRet = sqlite3Fts3MallocZero(::core::mem::size_of::<Fts3Expr>() as sqlite3_int64)
                        as *mut Fts3Expr;
                    if pRet.is_null() {
                        return SQLITE_NOMEM;
                    }
                    (*pRet).eType = (*pKey).eType as ::core::ffi::c_int;
                    (*pRet).nNear = nNear;
                    *ppExpr = pRet;
                    *pnConsumed = (zInput.offset_from(z) as ::core::ffi::c_long
                        + nKey as ::core::ffi::c_long)
                        as ::core::ffi::c_int;
                    return SQLITE_OK;
                }
            }
        }
        ii += 1;
    }
    if *zInput as ::core::ffi::c_int == '"' as i32 {
        ii = 1 as ::core::ffi::c_int;
        while ii < nInput && *zInput.offset(ii as isize) as ::core::ffi::c_int != '"' as i32 {
            ii += 1;
        }
        *pnConsumed = (zInput.offset_from(z) as ::core::ffi::c_long
            + ii as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as ::core::ffi::c_int;
        if ii == nInput {
            return SQLITE_ERROR;
        }
        return getNextString(
            pParse,
            zInput.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
            ii - 1 as ::core::ffi::c_int,
            ppExpr,
        );
    }
    if sqlite3_fts3_enable_parentheses != 0 {
        if *zInput as ::core::ffi::c_int == '(' as i32 {
            let mut nConsumed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            (*pParse).nNest += 1;
            if (*pParse).nNest > 1000 as ::core::ffi::c_int {
                return SQLITE_ERROR;
            }
            rc = fts3ExprParse(
                pParse,
                zInput.offset(1 as ::core::ffi::c_int as isize),
                nInput - 1 as ::core::ffi::c_int,
                ppExpr,
                &raw mut nConsumed,
            );
            *pnConsumed = zInput.offset_from(z) as ::core::ffi::c_long as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + nConsumed;
            return rc;
        } else if *zInput as ::core::ffi::c_int == ')' as i32 {
            (*pParse).nNest -= 1;
            *pnConsumed = (zInput.offset_from(z) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                as ::core::ffi::c_int;
            *ppExpr = ::core::ptr::null_mut::<Fts3Expr>();
            return SQLITE_DONE;
        }
    }
    iCol = (*pParse).iDefaultCol;
    iColLen = 0 as ::core::ffi::c_int;
    ii = 0 as ::core::ffi::c_int;
    while ii < (*pParse).nCol {
        let mut zStr: *const ::core::ffi::c_char = *(*pParse).azCol.offset(ii as isize);
        let mut nStr: ::core::ffi::c_int = strlen(zStr) as ::core::ffi::c_int;
        if nInput > nStr
            && *zInput.offset(nStr as isize) as ::core::ffi::c_int == ':' as i32
            && sqlite3_strnicmp(zStr, zInput, nStr) == 0 as ::core::ffi::c_int
        {
            iCol = ii;
            iColLen = (zInput.offset_from(z) as ::core::ffi::c_long
                + nStr as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long) as ::core::ffi::c_int;
            break;
        } else {
            ii += 1;
        }
    }
    rc = getNextToken(
        pParse,
        iCol,
        z.offset(iColLen as isize) as *const ::core::ffi::c_char,
        n - iColLen,
        ppExpr,
        pnConsumed,
    );
    *pnConsumed += iColLen;
    return rc;
}
unsafe extern "C" fn opPrecedence(mut p: *mut Fts3Expr) -> ::core::ffi::c_int {
    if sqlite3_fts3_enable_parentheses != 0 {
        return (*p).eType;
    } else if (*p).eType == FTSQUERY_NEAR {
        return 1 as ::core::ffi::c_int;
    } else if (*p).eType == FTSQUERY_OR {
        return 2 as ::core::ffi::c_int;
    }
    return 3 as ::core::ffi::c_int;
}
unsafe extern "C" fn insertBinaryOperator(
    mut ppHead: *mut *mut Fts3Expr,
    mut pPrev: *mut Fts3Expr,
    mut pNew: *mut Fts3Expr,
) {
    let mut pSplit: *mut Fts3Expr = pPrev;
    while !(*pSplit).pParent.is_null() && opPrecedence((*pSplit).pParent) <= opPrecedence(pNew) {
        pSplit = (*pSplit).pParent;
    }
    if !(*pSplit).pParent.is_null() {
        (*(*pSplit).pParent).pRight = pNew;
        (*pNew).pParent = (*pSplit).pParent;
    } else {
        *ppHead = pNew;
    }
    (*pNew).pLeft = pSplit;
    (*pSplit).pParent = pNew;
}
unsafe extern "C" fn fts3ExprParse(
    mut pParse: *mut ParseContext,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut ppExpr: *mut *mut Fts3Expr,
    mut pnConsumed: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut pRet: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
    let mut pPrev: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
    let mut pNotBranch: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
    let mut nIn: ::core::ffi::c_int = n;
    let mut zIn: *const ::core::ffi::c_char = z;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut isRequirePhrase: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    loop {
        if !(rc == SQLITE_OK) {
            current_block = 1434579379687443766;
            break;
        }
        let mut p: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
        let mut nByte: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        rc = getNextNode(pParse, zIn, nIn, &raw mut p, &raw mut nByte);
        if rc == SQLITE_OK {
            if !p.is_null() {
                let mut isPhrase: ::core::ffi::c_int = 0;
                if sqlite3_fts3_enable_parentheses == 0
                    && (*p).eType == FTSQUERY_PHRASE
                    && (*pParse).isNot != 0
                {
                    let mut pNot: *mut Fts3Expr =
                        sqlite3Fts3MallocZero(::core::mem::size_of::<Fts3Expr>() as sqlite3_int64)
                            as *mut Fts3Expr;
                    if pNot.is_null() {
                        sqlite3Fts3ExprFree(p);
                        rc = SQLITE_NOMEM;
                        current_block = 16633387481569624112;
                        break;
                    } else {
                        (*pNot).eType = FTSQUERY_NOT;
                        (*pNot).pRight = p;
                        (*p).pParent = pNot;
                        if !pNotBranch.is_null() {
                            (*pNot).pLeft = pNotBranch;
                            (*pNotBranch).pParent = pNot;
                        }
                        pNotBranch = pNot;
                        p = pPrev;
                    }
                } else {
                    let mut eType: ::core::ffi::c_int = (*p).eType;
                    isPhrase =
                        (eType == FTSQUERY_PHRASE || !(*p).pLeft.is_null()) as ::core::ffi::c_int;
                    if isPhrase == 0 && isRequirePhrase != 0 {
                        sqlite3Fts3ExprFree(p);
                        rc = SQLITE_ERROR;
                        current_block = 16633387481569624112;
                        break;
                    } else {
                        if isPhrase != 0 && isRequirePhrase == 0 {
                            let mut pAnd: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
                            pAnd = sqlite3Fts3MallocZero(
                                ::core::mem::size_of::<Fts3Expr>() as sqlite3_int64
                            ) as *mut Fts3Expr;
                            if pAnd.is_null() {
                                sqlite3Fts3ExprFree(p);
                                rc = SQLITE_NOMEM;
                                current_block = 16633387481569624112;
                                break;
                            } else {
                                (*pAnd).eType = FTSQUERY_AND;
                                insertBinaryOperator(&raw mut pRet, pPrev, pAnd);
                                pPrev = pAnd;
                            }
                        }
                        if !pPrev.is_null()
                            && (eType == FTSQUERY_NEAR
                                && isPhrase == 0
                                && (*pPrev).eType != FTSQUERY_PHRASE
                                || eType != FTSQUERY_PHRASE
                                    && isPhrase != 0
                                    && (*pPrev).eType == FTSQUERY_NEAR)
                        {
                            sqlite3Fts3ExprFree(p);
                            rc = SQLITE_ERROR;
                            current_block = 16633387481569624112;
                            break;
                        } else {
                            if isPhrase != 0 {
                                if !pRet.is_null() {
                                    (*pPrev).pRight = p;
                                    (*p).pParent = pPrev;
                                } else {
                                    pRet = p;
                                }
                            } else {
                                insertBinaryOperator(&raw mut pRet, pPrev, p);
                            }
                            isRequirePhrase = (isPhrase == 0) as ::core::ffi::c_int;
                        }
                    }
                }
                pPrev = p;
            }
        }
        nIn -= nByte;
        zIn = zIn.offset(nByte as isize);
    }
    match current_block {
        1434579379687443766 => {
            if rc == SQLITE_DONE && !pRet.is_null() && isRequirePhrase != 0 {
                rc = SQLITE_ERROR;
            }
            if rc == SQLITE_DONE {
                rc = SQLITE_OK;
                if sqlite3_fts3_enable_parentheses == 0 && !pNotBranch.is_null() {
                    if pRet.is_null() {
                        rc = SQLITE_ERROR;
                    } else {
                        let mut pIter: *mut Fts3Expr = pNotBranch;
                        while !(*pIter).pLeft.is_null() {
                            pIter = (*pIter).pLeft;
                        }
                        (*pIter).pLeft = pRet;
                        (*pRet).pParent = pIter;
                        pRet = pNotBranch;
                    }
                }
            }
            *pnConsumed = n - nIn;
        }
        _ => {}
    }
    if rc != SQLITE_OK {
        sqlite3Fts3ExprFree(pRet);
        sqlite3Fts3ExprFree(pNotBranch);
        pRet = ::core::ptr::null_mut::<Fts3Expr>();
    }
    *ppExpr = pRet;
    return rc;
}
unsafe extern "C" fn fts3ExprCheckDepth(
    mut p: *mut Fts3Expr,
    mut nMaxDepth: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    if !p.is_null() {
        if nMaxDepth < 0 as ::core::ffi::c_int {
            rc = SQLITE_TOOBIG;
        } else {
            rc = fts3ExprCheckDepth((*p).pLeft, nMaxDepth - 1 as ::core::ffi::c_int);
            if rc == SQLITE_OK {
                rc = fts3ExprCheckDepth((*p).pRight, nMaxDepth - 1 as ::core::ffi::c_int);
            }
        }
    }
    return rc;
}
unsafe extern "C" fn fts3ExprBalance(
    mut pp: *mut *mut Fts3Expr,
    mut nMaxDepth: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut pRoot: *mut Fts3Expr = *pp;
    let mut pFree: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
    let mut eType: ::core::ffi::c_int = (*pRoot).eType;
    if nMaxDepth == 0 as ::core::ffi::c_int {
        rc = SQLITE_ERROR;
    }
    if rc == SQLITE_OK {
        if eType == FTSQUERY_AND || eType == FTSQUERY_OR {
            let mut apLeaf: *mut *mut Fts3Expr = ::core::ptr::null_mut::<*mut Fts3Expr>();
            apLeaf = sqlite3_malloc64(
                (::core::mem::size_of::<*mut Fts3Expr>() as usize).wrapping_mul(nMaxDepth as usize)
                    as sqlite3_uint64,
            ) as *mut *mut Fts3Expr;
            if apLeaf.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                memset(
                    apLeaf as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (::core::mem::size_of::<*mut Fts3Expr>() as size_t)
                        .wrapping_mul(nMaxDepth as size_t),
                );
            }
            if rc == SQLITE_OK {
                let mut i: ::core::ffi::c_int = 0;
                let mut p: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
                p = pRoot;
                while (*p).eType == eType {
                    p = (*p).pLeft;
                }
                loop {
                    let mut iLvl: ::core::ffi::c_int = 0;
                    let mut pParent: *mut Fts3Expr = (*p).pParent;
                    (*p).pParent = ::core::ptr::null_mut::<Fts3Expr>();
                    if !pParent.is_null() {
                        (*pParent).pLeft = ::core::ptr::null_mut::<Fts3Expr>();
                    } else {
                        pRoot = ::core::ptr::null_mut::<Fts3Expr>();
                    }
                    rc = fts3ExprBalance(&raw mut p, nMaxDepth - 1 as ::core::ffi::c_int);
                    if rc != SQLITE_OK {
                        break;
                    }
                    iLvl = 0 as ::core::ffi::c_int;
                    while !p.is_null() && iLvl < nMaxDepth {
                        if (*apLeaf.offset(iLvl as isize)).is_null() {
                            let ref mut fresh2 = *apLeaf.offset(iLvl as isize);
                            *fresh2 = p;
                            p = ::core::ptr::null_mut::<Fts3Expr>();
                        } else {
                            (*pFree).pLeft = *apLeaf.offset(iLvl as isize);
                            (*pFree).pRight = p;
                            (*(*pFree).pLeft).pParent = pFree;
                            (*(*pFree).pRight).pParent = pFree;
                            p = pFree;
                            pFree = (*pFree).pParent;
                            (*p).pParent = ::core::ptr::null_mut::<Fts3Expr>();
                            let ref mut fresh3 = *apLeaf.offset(iLvl as isize);
                            *fresh3 = ::core::ptr::null_mut::<Fts3Expr>();
                        }
                        iLvl += 1;
                    }
                    if !p.is_null() {
                        sqlite3Fts3ExprFree(p);
                        rc = SQLITE_TOOBIG;
                        break;
                    } else {
                        if pParent.is_null() {
                            break;
                        }
                        p = (*pParent).pRight;
                        while (*p).eType == eType {
                            p = (*p).pLeft;
                        }
                        (*(*pParent).pRight).pParent = (*pParent).pParent;
                        if !(*pParent).pParent.is_null() {
                            (*(*pParent).pParent).pLeft = (*pParent).pRight;
                        } else {
                            pRoot = (*pParent).pRight;
                        }
                        (*pParent).pParent = pFree;
                        pFree = pParent;
                    }
                }
                if rc == SQLITE_OK {
                    p = ::core::ptr::null_mut::<Fts3Expr>();
                    i = 0 as ::core::ffi::c_int;
                    while i < nMaxDepth {
                        if !(*apLeaf.offset(i as isize)).is_null() {
                            if p.is_null() {
                                p = *apLeaf.offset(i as isize);
                                (*p).pParent = ::core::ptr::null_mut::<Fts3Expr>();
                            } else {
                                (*pFree).pRight = p;
                                (*pFree).pLeft = *apLeaf.offset(i as isize);
                                (*(*pFree).pLeft).pParent = pFree;
                                (*(*pFree).pRight).pParent = pFree;
                                p = pFree;
                                pFree = (*pFree).pParent;
                                (*p).pParent = ::core::ptr::null_mut::<Fts3Expr>();
                            }
                        }
                        i += 1;
                    }
                    pRoot = p;
                } else {
                    let mut pDel: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
                    i = 0 as ::core::ffi::c_int;
                    while i < nMaxDepth {
                        sqlite3Fts3ExprFree(*apLeaf.offset(i as isize));
                        i += 1;
                    }
                    loop {
                        pDel = pFree;
                        if pDel.is_null() {
                            break;
                        }
                        pFree = (*pDel).pParent;
                        sqlite3_free(pDel as *mut ::core::ffi::c_void);
                    }
                }
                sqlite3_free(apLeaf as *mut ::core::ffi::c_void);
            }
        } else if eType == FTSQUERY_NOT {
            let mut pLeft: *mut Fts3Expr = (*pRoot).pLeft;
            let mut pRight: *mut Fts3Expr = (*pRoot).pRight;
            (*pRoot).pLeft = ::core::ptr::null_mut::<Fts3Expr>();
            (*pRoot).pRight = ::core::ptr::null_mut::<Fts3Expr>();
            (*pLeft).pParent = ::core::ptr::null_mut::<Fts3Expr>();
            (*pRight).pParent = ::core::ptr::null_mut::<Fts3Expr>();
            rc = fts3ExprBalance(&raw mut pLeft, nMaxDepth - 1 as ::core::ffi::c_int);
            if rc == SQLITE_OK {
                rc = fts3ExprBalance(&raw mut pRight, nMaxDepth - 1 as ::core::ffi::c_int);
            }
            if rc != SQLITE_OK {
                sqlite3Fts3ExprFree(pRight);
                sqlite3Fts3ExprFree(pLeft);
            } else {
                (*pRoot).pLeft = pLeft;
                (*pLeft).pParent = pRoot;
                (*pRoot).pRight = pRight;
                (*pRight).pParent = pRoot;
            }
        }
    }
    if rc != SQLITE_OK {
        sqlite3Fts3ExprFree(pRoot);
        pRoot = ::core::ptr::null_mut::<Fts3Expr>();
    }
    *pp = pRoot;
    return rc;
}
unsafe extern "C" fn fts3ExprParseUnbalanced(
    mut pTokenizer: *mut sqlite3_tokenizer,
    mut iLangid: ::core::ffi::c_int,
    mut azCol: *mut *mut ::core::ffi::c_char,
    mut bFts4: ::core::ffi::c_int,
    mut nCol: ::core::ffi::c_int,
    mut iDefaultCol: ::core::ffi::c_int,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut ppExpr: *mut *mut Fts3Expr,
) -> ::core::ffi::c_int {
    let mut nParsed: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0;
    let mut sParse: ParseContext = ParseContext {
        pTokenizer: ::core::ptr::null_mut::<sqlite3_tokenizer>(),
        iLangid: 0,
        azCol: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        bFts4: 0,
        nCol: 0,
        iDefaultCol: 0,
        isNot: 0,
        pCtx: ::core::ptr::null_mut::<sqlite3_context>(),
        nNest: 0,
    };
    memset(
        &raw mut sParse as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ParseContext>() as size_t,
    );
    sParse.pTokenizer = pTokenizer;
    sParse.iLangid = iLangid;
    sParse.azCol = azCol as *mut *const ::core::ffi::c_char;
    sParse.nCol = nCol;
    sParse.iDefaultCol = iDefaultCol;
    sParse.bFts4 = bFts4;
    if z.is_null() {
        *ppExpr = ::core::ptr::null_mut::<Fts3Expr>();
        return SQLITE_OK;
    }
    if n < 0 as ::core::ffi::c_int {
        n = strlen(z) as ::core::ffi::c_int;
    }
    rc = fts3ExprParse(&raw mut sParse, z, n, ppExpr, &raw mut nParsed);
    if rc == SQLITE_OK && sParse.nNest != 0 {
        rc = SQLITE_ERROR;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3ExprParse(
    mut pTokenizer: *mut sqlite3_tokenizer,
    mut iLangid: ::core::ffi::c_int,
    mut azCol: *mut *mut ::core::ffi::c_char,
    mut bFts4: ::core::ffi::c_int,
    mut nCol: ::core::ffi::c_int,
    mut iDefaultCol: ::core::ffi::c_int,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut ppExpr: *mut *mut Fts3Expr,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = fts3ExprParseUnbalanced(
        pTokenizer,
        iLangid,
        azCol,
        bFts4,
        nCol,
        iDefaultCol,
        z,
        n,
        ppExpr,
    );
    if rc == SQLITE_OK && !(*ppExpr).is_null() {
        rc = fts3ExprBalance(ppExpr, SQLITE_FTS3_MAX_EXPR_DEPTH);
        if rc == SQLITE_OK {
            rc = fts3ExprCheckDepth(*ppExpr, SQLITE_FTS3_MAX_EXPR_DEPTH);
        }
    }
    if rc != SQLITE_OK {
        sqlite3Fts3ExprFree(*ppExpr);
        *ppExpr = ::core::ptr::null_mut::<Fts3Expr>();
        if rc == SQLITE_TOOBIG {
            sqlite3Fts3ErrMsg(
                pzErr,
                b"FTS expression tree is too large (maximum depth %d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                SQLITE_FTS3_MAX_EXPR_DEPTH,
            );
            rc = SQLITE_ERROR;
        } else if rc == SQLITE_ERROR {
            sqlite3Fts3ErrMsg(
                pzErr,
                b"malformed MATCH expression: [%s]\0" as *const u8 as *const ::core::ffi::c_char,
                z,
            );
        }
    }
    return rc;
}
unsafe extern "C" fn fts3FreeExprNode(mut p: *mut Fts3Expr) {
    sqlite3Fts3EvalPhraseCleanup((*p).pPhrase);
    sqlite3_free((*p).aMI as *mut ::core::ffi::c_void);
    sqlite3_free(p as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3ExprFree(mut pDel: *mut Fts3Expr) {
    let mut p: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
    p = pDel;
    while !p.is_null() && (!(*p).pLeft.is_null() || !(*p).pRight.is_null()) {
        p = if !(*p).pLeft.is_null() {
            (*p).pLeft
        } else {
            (*p).pRight
        };
    }
    while !p.is_null() {
        let mut pParent: *mut Fts3Expr = (*p).pParent;
        fts3FreeExprNode(p);
        if !pParent.is_null() && p == (*pParent).pLeft && !(*pParent).pRight.is_null() {
            p = (*pParent).pRight;
            while !p.is_null() && (!(*p).pLeft.is_null() || !(*p).pRight.is_null()) {
                p = if !(*p).pLeft.is_null() {
                    (*p).pLeft
                } else {
                    (*p).pRight
                };
            }
        } else {
            p = pParent;
        }
    }
}
unsafe extern "C" fn exprToString(
    mut pExpr: *mut Fts3Expr,
    mut zBuf: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if pExpr.is_null() {
        return sqlite3_mprintf(b"\0" as *const u8 as *const ::core::ffi::c_char);
    }
    match (*pExpr).eType {
        FTSQUERY_PHRASE => {
            let mut pPhrase: *mut Fts3Phrase = (*pExpr).pPhrase;
            let mut i: ::core::ffi::c_int = 0;
            zBuf = sqlite3_mprintf(
                b"%zPHRASE %d 0\0" as *const u8 as *const ::core::ffi::c_char,
                zBuf,
                (*pPhrase).iColumn,
            );
            i = 0 as ::core::ffi::c_int;
            while !zBuf.is_null() && i < (*pPhrase).nToken {
                zBuf = sqlite3_mprintf(
                    b"%z %.*s%s\0" as *const u8 as *const ::core::ffi::c_char,
                    zBuf,
                    (*(&raw mut (*pPhrase).aToken as *mut Fts3PhraseToken).offset(i as isize)).n,
                    (*(&raw mut (*pPhrase).aToken as *mut Fts3PhraseToken).offset(i as isize)).z,
                    if (*(&raw mut (*pPhrase).aToken as *mut Fts3PhraseToken).offset(i as isize))
                        .isPrefix
                        != 0
                    {
                        b"+\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    },
                );
                i += 1;
            }
            return zBuf;
        }
        FTSQUERY_NEAR => {
            zBuf = sqlite3_mprintf(
                b"%zNEAR/%d \0" as *const u8 as *const ::core::ffi::c_char,
                zBuf,
                (*pExpr).nNear,
            );
        }
        FTSQUERY_NOT => {
            zBuf = sqlite3_mprintf(b"%zNOT \0" as *const u8 as *const ::core::ffi::c_char, zBuf);
        }
        FTSQUERY_AND => {
            zBuf = sqlite3_mprintf(b"%zAND \0" as *const u8 as *const ::core::ffi::c_char, zBuf);
        }
        FTSQUERY_OR => {
            zBuf = sqlite3_mprintf(b"%zOR \0" as *const u8 as *const ::core::ffi::c_char, zBuf);
        }
        _ => {}
    }
    if !zBuf.is_null() {
        zBuf = sqlite3_mprintf(b"%z{\0" as *const u8 as *const ::core::ffi::c_char, zBuf);
    }
    if !zBuf.is_null() {
        zBuf = exprToString((*pExpr).pLeft, zBuf);
    }
    if !zBuf.is_null() {
        zBuf = sqlite3_mprintf(b"%z} {\0" as *const u8 as *const ::core::ffi::c_char, zBuf);
    }
    if !zBuf.is_null() {
        zBuf = exprToString((*pExpr).pRight, zBuf);
    }
    if !zBuf.is_null() {
        zBuf = sqlite3_mprintf(b"%z}\0" as *const u8 as *const ::core::ffi::c_char, zBuf);
    }
    return zBuf;
}
unsafe extern "C" fn fts3ExprTestCommon(
    mut bRebalance: ::core::ffi::c_int,
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut pTokenizer: *mut sqlite3_tokenizer = ::core::ptr::null_mut::<sqlite3_tokenizer>();
    let mut rc: ::core::ffi::c_int = 0;
    let mut azCol: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut zExpr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nExpr: ::core::ffi::c_int = 0;
    let mut nCol: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut pExpr: *mut Fts3Expr = ::core::ptr::null_mut::<Fts3Expr>();
    let mut zBuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pHash: *mut Fts3Hash = sqlite3_user_data(context) as *mut Fts3Hash;
    let mut zTokenizer: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if argc < 3 as ::core::ffi::c_int {
        sqlite3_result_error(
            context,
            b"Usage: fts3_exprtest(tokenizer, expr, col1, ...\0" as *const u8
                as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int),
        );
        return;
    }
    zTokenizer = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
        as *const ::core::ffi::c_char;
    rc = sqlite3Fts3InitTokenizer(pHash, zTokenizer, &raw mut pTokenizer, &raw mut zErr);
    if rc != SQLITE_OK {
        if rc == SQLITE_NOMEM {
            sqlite3_result_error_nomem(context);
        } else {
            sqlite3_result_error(context, zErr, -(1 as ::core::ffi::c_int));
        }
        sqlite3_free(zErr as *mut ::core::ffi::c_void);
        return;
    }
    zExpr = sqlite3_value_text(*argv.offset(1 as ::core::ffi::c_int as isize))
        as *const ::core::ffi::c_char;
    nExpr = sqlite3_value_bytes(*argv.offset(1 as ::core::ffi::c_int as isize));
    nCol = argc - 2 as ::core::ffi::c_int;
    azCol = sqlite3_malloc64(
        (nCol as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            as sqlite3_uint64,
    ) as *mut *mut ::core::ffi::c_char;
    if azCol.is_null() {
        sqlite3_result_error_nomem(context);
    } else {
        ii = 0 as ::core::ffi::c_int;
        while ii < nCol {
            let ref mut fresh4 = *azCol.offset(ii as isize);
            *fresh4 = sqlite3_value_text(*argv.offset((ii + 2 as ::core::ffi::c_int) as isize))
                as *mut ::core::ffi::c_char;
            ii += 1;
        }
        if bRebalance != 0 {
            let mut zDummy: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            rc = sqlite3Fts3ExprParse(
                pTokenizer,
                0 as ::core::ffi::c_int,
                azCol,
                0 as ::core::ffi::c_int,
                nCol,
                nCol,
                zExpr,
                nExpr,
                &raw mut pExpr,
                &raw mut zDummy,
            );
            sqlite3_free(zDummy as *mut ::core::ffi::c_void);
        } else {
            rc = fts3ExprParseUnbalanced(
                pTokenizer,
                0 as ::core::ffi::c_int,
                azCol,
                0 as ::core::ffi::c_int,
                nCol,
                nCol,
                zExpr,
                nExpr,
                &raw mut pExpr,
            );
        }
        if rc != SQLITE_OK && rc != SQLITE_NOMEM {
            sqlite3_result_error(
                context,
                b"Error parsing expression\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int),
            );
        } else if rc == SQLITE_NOMEM || {
            zBuf = exprToString(pExpr, ::core::ptr::null_mut::<::core::ffi::c_char>());
            zBuf.is_null()
        } {
            sqlite3_result_error_nomem(context);
        } else {
            sqlite3_result_text(
                context,
                zBuf,
                -(1 as ::core::ffi::c_int),
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
            );
            sqlite3_free(zBuf as *mut ::core::ffi::c_void);
        }
        sqlite3Fts3ExprFree(pExpr);
    }
    if !pTokenizer.is_null() {
        rc = (*(*pTokenizer).pModule)
            .xDestroy
            .expect("non-null function pointer")(pTokenizer);
    }
    sqlite3_free(azCol as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn fts3ExprTest(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    fts3ExprTestCommon(0 as ::core::ffi::c_int, context, argc, argv);
}
unsafe extern "C" fn fts3ExprTestRebalance(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    fts3ExprTestCommon(1 as ::core::ffi::c_int, context, argc, argv);
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3ExprInitTestInterface(
    mut db: *mut sqlite3,
    mut pHash: *mut Fts3Hash,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = sqlite3_create_function(
        db,
        b"fts3_exprtest\0" as *const u8 as *const ::core::ffi::c_char,
        -(1 as ::core::ffi::c_int),
        SQLITE_UTF8,
        pHash as *mut ::core::ffi::c_void,
        Some(
            fts3ExprTest
                as unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
        ),
        None,
        None,
    );
    if rc == SQLITE_OK {
        rc = sqlite3_create_function(
            db,
            b"fts3_exprtest_rebalance\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int),
            SQLITE_UTF8,
            pHash as *mut ::core::ffi::c_void,
            Some(
                fts3ExprTestRebalance
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
    }
    return rc;
}
