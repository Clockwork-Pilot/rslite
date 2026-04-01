






pub use crate::__stddef_size_t_h::size_t;




pub use crate::src::ext::rtree::rtree::i64_0;pub use crate::src::ext::fts3::fts3::sqlite3Fts3EvalPhraseCleanup;pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3Fts3InitTokenizer;pub use crate::src::ext::fts3::fts3::sqlite3Fts3ReadInt;pub use crate::src::ext::rtree::rtree::u32_0;pub use crate::src::ext::rtree::rtree::u8_0;pub use crate::fts3Int_h::Fts3DeferredToken;pub use crate::fts3Int_h::Fts3Doclist;pub use crate::fts3Int_h::Fts3Expr;pub use crate::fts3Int_h::Fts3MultiSegReader;pub use crate::fts3Int_h::Fts3Phrase;pub use crate::fts3Int_h::Fts3PhraseToken;pub use crate::fts3Int_h::Fts3SegFilter;pub use crate::fts3Int_h::Fts3SegReader;pub use crate::fts3Int_h::FTSQUERY_AND_1;pub use crate::fts3Int_h::FTSQUERY_NEAR;pub use crate::fts3Int_h::FTSQUERY_NOT_1;pub use crate::fts3Int_h::FTSQUERY_OR_1;pub use crate::fts3Int_h::FTSQUERY_PHRASE;pub use crate::fts3Int_h::SQLITE_FTS3_MAX_EXPR_DEPTH;pub use crate::src::ext::fts3::fts3_hash::Fts3Hash;pub use crate::src::ext::fts3::fts3_hash::Fts3HashElem;pub use crate::src::ext::fts3::fts3_hash::_fts3ht;pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer;pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor;pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module;pub use crate::src::headers::sqliteInt_h::sqlite3;pub use crate::src::headers::vdbeInt_h::sqlite3_context;pub use crate::src::src::main::sqlite3_create_function;pub use crate::src::headers::sqlite3_h::sqlite3_destructor_type;pub use crate::src::src::malloc::sqlite3_free;pub use crate::src::headers::sqlite3_h::sqlite3_int64;pub use crate::src::src::malloc::sqlite3_malloc64;pub use crate::src::src::malloc::sqlite3_realloc64;pub use crate::src::src::vdbeapi::sqlite3_result_error;pub use crate::src::src::vdbeapi::sqlite3_result_error_nomem;pub use crate::src::src::vdbeapi::sqlite3_result_text;pub use crate::src::src::util::sqlite3_strnicmp;pub use crate::src::headers::sqlite3_h::sqlite3_uint64;pub use crate::src::src::vdbeapi::sqlite3_user_data;pub use crate::src::headers::vdbeInt_h::sqlite3_value;pub use crate::src::src::vdbeapi::sqlite3_value_bytes;pub use crate::src::src::vdbeapi::sqlite3_value_text;pub use crate::src::headers::sqlite3_h::sqlite_int64;pub use crate::src::headers::sqlite3_h::sqlite_uint64;pub use crate::src::headers::sqlite3_h::SQLITE_DONE;pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;pub use crate::src::headers::sqlite3_h::SQLITE_OK;pub use crate::src::headers::sqlite3_h::SQLITE_TOOBIG;pub use crate::src::headers::sqlite3_h::SQLITE_UTF8;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ParseContext {
    pub pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
    pub iLangid: ::core::ffi::c_int,
    pub azCol: *mut *const ::core::ffi::c_char,
    pub bFts4: ::core::ffi::c_int,
    pub nCol: ::core::ffi::c_int,
    pub iDefaultCol: ::core::ffi::c_int,
    pub isNot: ::core::ffi::c_int,
    pub pCtx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
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
#[unsafe(no_mangle)]

pub static mut sqlite3_fts3_enable_parentheses: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQLITE_FTS3_DEFAULT_NEAR_PARAM: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

unsafe extern "C" fn fts3isspace(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    (c as ::core::ffi::c_int == ' ' as i32
        || c as ::core::ffi::c_int == '\t' as i32
        || c as ::core::ffi::c_int == '\n' as i32
        || c as ::core::ffi::c_int == '\r' as i32
        || c as ::core::ffi::c_int == '\u{b}' as i32
        || c as ::core::ffi::c_int == '\u{c}' as i32) as ::core::ffi::c_int
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3Fts3MallocZero(
    mut nByte: crate::src::headers::sqlite3_h::sqlite3_int64,
) -> *mut ::core::ffi::c_void {
    let mut pRet: *mut ::core::ffi::c_void = crate::src::src::malloc::sqlite3_malloc64(nByte as crate::src::headers::sqlite3_h::sqlite3_uint64);
    if !pRet.is_null() {
        ::libc::memset(pRet, 0 as ::core::ffi::c_int, nByte as crate::__stddef_size_t_h::size_t);
    }
    pRet
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3Fts3OpenTokenizer(
    mut pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
    mut iLangid: ::core::ffi::c_int,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut ppCsr: *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
) -> ::core::ffi::c_int {
    let mut pModule: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module = (*pTokenizer).pModule;
    let mut pCsr: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor =
        ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = (*pModule).xOpen.expect("non-null function pointer")(pTokenizer, z, n, &raw mut pCsr);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        (*pCsr).pTokenizer = pTokenizer;
        if (*pModule).iVersion >= 1 as ::core::ffi::c_int {
            rc = (*pModule).xLanguageid.expect("non-null function pointer")(pCsr, iLangid);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                (*pModule).xClose.expect("non-null function pointer")(pCsr);
                pCsr = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor>();
            }
        }
    }
    *ppCsr = pCsr;
    rc
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
    -(1 as ::core::ffi::c_int)
}

unsafe extern "C" fn getNextToken(
    mut pParse: *mut ParseContext,
    mut iCol: ::core::ffi::c_int,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut ppExpr: *mut *mut crate::fts3Int_h::Fts3Expr,
    mut pnConsumed: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer = (*pParse).pTokenizer;
    let mut pModule: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module = (*pTokenizer).pModule;
    let mut rc: ::core::ffi::c_int = 0;
    let mut pCursor: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor =
        ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor>();
    let mut pRet: *mut crate::fts3Int_h::Fts3Expr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
    *pnConsumed = n;
    rc = sqlite3Fts3OpenTokenizer(pTokenizer, (*pParse).iLangid, z, n, &raw mut pCursor);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut zToken: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iStart: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iEnd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iPosition: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nByte: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
        rc = (*pModule).xNext.expect("non-null function pointer")(
            pCursor,
            &raw mut zToken,
            &raw mut nToken,
            &raw mut iStart,
            &raw mut iEnd,
            &raw mut iPosition,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let mut iBarred: ::core::ffi::c_int = findBarredChar(z, iEnd);
            if iBarred >= 0 as ::core::ffi::c_int {
                (*pModule).xClose.expect("non-null function pointer")(pCursor);
                return getNextToken(pParse, iCol, z, iBarred, ppExpr, pnConsumed);
            }
            nByte = (::core::mem::size_of::<crate::fts3Int_h::Fts3Expr>() as usize)
                .wrapping_add((88 as usize).wrapping_add(
                    (1 as usize).wrapping_mul(::core::mem::size_of::<crate::fts3Int_h::Fts3PhraseToken>() as usize),
                ))
                .wrapping_add(nToken as usize) as crate::src::headers::sqlite3_h::sqlite3_int64;
            pRet = sqlite3Fts3MallocZero(nByte) as *mut crate::fts3Int_h::Fts3Expr;
            if pRet.is_null() {
                rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            } else {
                let __pRet_ref = unsafe { &mut *pRet };
                __pRet_ref.eType = crate::fts3Int_h::FTSQUERY_PHRASE;
                __pRet_ref.pPhrase = pRet.offset(1 as isize) as *mut crate::fts3Int_h::Fts3Expr
                    as *mut crate::fts3Int_h::Fts3Phrase;
                (*__pRet_ref.pPhrase).nToken = 1 as ::core::ffi::c_int;
                (*__pRet_ref.pPhrase).iColumn = iCol;
                (*(&raw mut (*__pRet_ref.pPhrase).aToken as *mut crate::fts3Int_h::Fts3PhraseToken)
                    .offset(0 as isize))
                .n = nToken;
                let ref mut fresh0 = (*(&raw mut (*__pRet_ref.pPhrase).aToken
                    as *mut crate::fts3Int_h::Fts3PhraseToken)
                    .offset(0 as isize))
                .z;
                *fresh0 = (&raw mut (*__pRet_ref.pPhrase).aToken as *mut crate::fts3Int_h::Fts3PhraseToken)
                    .offset(1 as isize)
                    as *mut crate::fts3Int_h::Fts3PhraseToken as *mut ::core::ffi::c_char;
                ::core::ptr::copy_nonoverlapping(
                    zToken as *const u8,
                    (*(&raw mut (*__pRet_ref.pPhrase).aToken as *mut crate::fts3Int_h::Fts3PhraseToken)
                        .offset(0 as isize))
                    .z as *mut u8,
                    nToken as usize,
                );
                if iEnd < n && *z.offset(iEnd as isize) as ::core::ffi::c_int == '*' as i32 {
                    (*(&raw mut (*__pRet_ref.pPhrase).aToken as *mut crate::fts3Int_h::Fts3PhraseToken)
                        .offset(0 as isize))
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
                        (*(&raw mut (*__pRet_ref.pPhrase).aToken as *mut crate::fts3Int_h::Fts3PhraseToken)
                            .offset(0 as isize))
                        .bFirst = 1 as ::core::ffi::c_int;
                        iStart -= 1;
                    }
                }
            }
            *pnConsumed = iEnd;
        } else if n != 0 && rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
            let mut iBarred_0: ::core::ffi::c_int = findBarredChar(z, n);
            if iBarred_0 >= 0 as ::core::ffi::c_int {
                *pnConsumed = iBarred_0;
            }
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        (*pModule).xClose.expect("non-null function pointer")(pCursor);
    }
    *ppExpr = pRet;
    rc
}

unsafe extern "C" fn fts3ReallocOrFree(
    mut pOrig: *mut ::core::ffi::c_void,
    mut nNew: crate::src::headers::sqlite3_h::sqlite3_int64,
) -> *mut ::core::ffi::c_void {
    let mut pRet: *mut ::core::ffi::c_void = crate::src::src::malloc::sqlite3_realloc64(pOrig, nNew as crate::src::headers::sqlite3_h::sqlite3_uint64);
    if pRet.is_null() {
        crate::src::src::malloc::sqlite3_free(pOrig);
    }
    pRet
}

unsafe extern "C" fn getNextString(
    mut pParse: *mut ParseContext,
    mut zInput: *const ::core::ffi::c_char,
    mut nInput: ::core::ffi::c_int,
    mut ppExpr: *mut *mut crate::fts3Int_h::Fts3Expr,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer = (*pParse).pTokenizer;
    let mut pModule: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module = (*pTokenizer).pModule;
    let mut rc: ::core::ffi::c_int = 0;
    let mut p: *mut crate::fts3Int_h::Fts3Expr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
    let mut pCursor: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor =
        ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor>();
    let mut zTemp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nTemp: crate::src::ext::rtree::rtree::i64_0 = 0 as crate::src::ext::rtree::rtree::i64_0;
    let nSpace: ::core::ffi::c_int =
        (::core::mem::size_of::<crate::fts3Int_h::Fts3Expr>() as usize).wrapping_add((88 as usize).wrapping_add(
            (1 as usize).wrapping_mul(::core::mem::size_of::<crate::fts3Int_h::Fts3PhraseToken>() as usize),
        )) as ::core::ffi::c_int;
    let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rc = sqlite3Fts3OpenTokenizer(
        pTokenizer,
        (*pParse).iLangid,
        zInput,
        nInput,
        &raw mut pCursor,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut ii: ::core::ffi::c_int = 0;
        ii = 0 as ::core::ffi::c_int;
        loop {
            if !(rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
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
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                let mut pToken: *mut crate::fts3Int_h::Fts3PhraseToken = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3PhraseToken>();
                p = fts3ReallocOrFree(
                    p as *mut ::core::ffi::c_void,
                    (nSpace as usize).wrapping_add(
                        (ii as usize)
                            .wrapping_mul(::core::mem::size_of::<crate::fts3Int_h::Fts3PhraseToken>() as usize),
                    ) as crate::src::headers::sqlite3_h::sqlite3_int64,
                ) as *mut crate::fts3Int_h::Fts3Expr;
                zTemp = fts3ReallocOrFree(
                    zTemp as *mut ::core::ffi::c_void,
                    nTemp as crate::src::headers::sqlite3_h::sqlite3_int64 + nByte as crate::src::headers::sqlite3_h::sqlite3_int64,
                ) as *mut ::core::ffi::c_char;
                if zTemp.is_null() || p.is_null() {
                    rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                    current_block = 12690954132298350795;
                    break;
                } else {
                    pToken = (&raw mut (*(p.offset(1 as isize)
                        as *mut crate::fts3Int_h::Fts3Expr
                        as *mut crate::fts3Int_h::Fts3Phrase))
                        .aToken as *mut crate::fts3Int_h::Fts3PhraseToken)
                        .offset(ii as isize) as *mut crate::fts3Int_h::Fts3PhraseToken;
                    ::libc::memset(
                        pToken as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<crate::fts3Int_h::Fts3PhraseToken>() as crate::__stddef_size_t_h::size_t,
                    );
                    ::core::ptr::copy_nonoverlapping(
                    zByte as *const u8,
                    zTemp.offset(nTemp as isize) as *mut ::core::ffi::c_char as *mut u8,
                    nByte as usize,
                );
                    nTemp += nByte as crate::src::ext::rtree::rtree::i64_0;
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
            if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
                let mut jj: ::core::ffi::c_int = 0;
                let mut zBuf: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                p = fts3ReallocOrFree(
                    p as *mut ::core::ffi::c_void,
                    ((nSpace as usize).wrapping_add(
                        (nToken as usize)
                            .wrapping_mul(::core::mem::size_of::<crate::fts3Int_h::Fts3PhraseToken>() as usize),
                    ) as ::core::ffi::c_ulonglong)
                        .wrapping_add(nTemp as ::core::ffi::c_ulonglong)
                        as crate::src::headers::sqlite3_h::sqlite3_int64,
                ) as *mut crate::fts3Int_h::Fts3Expr;
                if p.is_null() {
                    rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                } else {
                    ::libc::memset(
                        p as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ((&raw mut (*(p.offset(1 as isize) as *mut crate::fts3Int_h::Fts3Expr
                            as *mut crate::fts3Int_h::Fts3Phrase))
                            .aToken as *mut crate::fts3Int_h::Fts3PhraseToken)
                            .offset(0 as isize)
                            as *mut crate::fts3Int_h::Fts3PhraseToken
                            as *mut ::core::ffi::c_char)
                            .offset_from(p as *mut ::core::ffi::c_char)
                            as ::core::ffi::c_long as crate::__stddef_size_t_h::size_t,
                    );
                    let __p_ref = unsafe { &mut *p };
                    __p_ref.eType = crate::fts3Int_h::FTSQUERY_PHRASE;
                    __p_ref.pPhrase = p.offset(1 as isize) as *mut crate::fts3Int_h::Fts3Expr
                        as *mut crate::fts3Int_h::Fts3Phrase;
                    (*__p_ref.pPhrase).iColumn = (*pParse).iDefaultCol;
                    (*__p_ref.pPhrase).nToken = nToken;
                    zBuf = (&raw mut (*__p_ref.pPhrase).aToken as *mut crate::fts3Int_h::Fts3PhraseToken)
                        .offset(nToken as isize) as *mut crate::fts3Int_h::Fts3PhraseToken
                        as *mut ::core::ffi::c_char;
                    if !zTemp.is_null() {
                        ::core::ptr::copy_nonoverlapping(
                    zTemp as *const u8,
                    zBuf as *mut u8,
                    nTemp as usize,
                );
                    }
                    jj = 0 as ::core::ffi::c_int;
                    while jj < (*__p_ref.pPhrase).nToken {
                        let ref mut fresh1 = (*(&raw mut (*__p_ref.pPhrase).aToken
                            as *mut crate::fts3Int_h::Fts3PhraseToken)
                            .offset(jj as isize))
                        .z;
                        *fresh1 = zBuf;
                        zBuf = zBuf.offset(
                            (*(&raw mut (*__p_ref.pPhrase).aToken as *mut crate::fts3Int_h::Fts3PhraseToken)
                                .offset(jj as isize))
                            .n as isize,
                        );
                        jj += 1;
                    }
                    rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                }
            }
        }
        _ => {}
    }
    if !pCursor.is_null() {
        (*pModule).xClose.expect("non-null function pointer")(pCursor);
    }
    crate::src::src::malloc::sqlite3_free(zTemp as *mut ::core::ffi::c_void);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
        p = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
    }
    *ppExpr = p;
    rc
}

unsafe extern "C" fn getNextNode(
    mut pParse: *mut ParseContext,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut ppExpr: *mut *mut crate::fts3Int_h::Fts3Expr,
    mut pnConsumed: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    static mut aKeyword: [Fts3Keyword; 4] = [
        Fts3Keyword {
            z: b"OR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n: 2 as ::core::ffi::c_uchar,
            parenOnly: 0 as ::core::ffi::c_uchar,
            eType: crate::fts3Int_h::FTSQUERY_OR_1 as ::core::ffi::c_uchar,
        },
        Fts3Keyword {
            z: b"AND\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n: 3 as ::core::ffi::c_uchar,
            parenOnly: 1 as ::core::ffi::c_uchar,
            eType: crate::fts3Int_h::FTSQUERY_AND_1 as ::core::ffi::c_uchar,
        },
        Fts3Keyword {
            z: b"NOT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n: 3 as ::core::ffi::c_uchar,
            parenOnly: 1 as ::core::ffi::c_uchar,
            eType: crate::fts3Int_h::FTSQUERY_NOT_1 as ::core::ffi::c_uchar,
        },
        Fts3Keyword {
            z: b"NEAR\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n: 4 as ::core::ffi::c_uchar,
            parenOnly: 0 as ::core::ffi::c_uchar,
            eType: crate::fts3Int_h::FTSQUERY_NEAR as ::core::ffi::c_uchar,
        },
    ];
    let mut ii: ::core::ffi::c_int = 0;
    let mut iCol: ::core::ffi::c_int = 0;
    let mut iColLen: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0;
    let mut pRet: *mut crate::fts3Int_h::Fts3Expr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
    let mut zInput: *const ::core::ffi::c_char = z;
    let mut nInput: ::core::ffi::c_int = n;
    let __pParse_ref = unsafe { &mut *pParse };
    __pParse_ref.isNot = 0 as ::core::ffi::c_int;
    while nInput > 0 as ::core::ffi::c_int && fts3isspace(*zInput) != 0 {
        nInput -= 1;
        zInput = zInput.offset(1);
    }
    if nInput == 0 as ::core::ffi::c_int {
        return crate::src::headers::sqlite3_h::SQLITE_DONE;
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
            let __pKey_ref = unsafe { &*pKey };
            if nInput >= __pKey_ref.n as ::core::ffi::c_int
                && 0 as ::core::ffi::c_int
                    == ::libc::memcmp(
                        zInput as *const ::core::ffi::c_void,
                        __pKey_ref.z as *const ::core::ffi::c_void,
                        __pKey_ref.n as crate::__stddef_size_t_h::size_t,
                    )
            {
                let mut nNear: ::core::ffi::c_int = SQLITE_FTS3_DEFAULT_NEAR_PARAM;
                let mut nKey: ::core::ffi::c_int = __pKey_ref.n as ::core::ffi::c_int;
                let mut cNext: ::core::ffi::c_char = 0;
                if __pKey_ref.eType as ::core::ffi::c_int == crate::fts3Int_h::FTSQUERY_NEAR {
                    if *zInput.offset(4 as isize) as ::core::ffi::c_int
                        == '/' as i32
                        && *zInput.offset(5 as isize) as ::core::ffi::c_int
                            >= '0' as i32
                        && *zInput.offset(5 as isize) as ::core::ffi::c_int
                            <= '9' as i32
                    {
                        nKey += 1 as ::core::ffi::c_int
                            + crate::src::ext::fts3::fts3::sqlite3Fts3ReadInt(
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
                    pRet = sqlite3Fts3MallocZero(::core::mem::size_of::<crate::fts3Int_h::Fts3Expr>() as crate::src::headers::sqlite3_h::sqlite3_int64)
                        as *mut crate::fts3Int_h::Fts3Expr;
                    if pRet.is_null() {
                        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                    }
                    (*pRet).eType = __pKey_ref.eType as ::core::ffi::c_int;
                    (*pRet).nNear = nNear;
                    *ppExpr = pRet;
                    *pnConsumed = (zInput.offset_from(z) as ::core::ffi::c_long
                        + nKey as ::core::ffi::c_long)
                        as ::core::ffi::c_int;
                    return crate::src::headers::sqlite3_h::SQLITE_OK;
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
            return crate::src::headers::sqlite3_h::SQLITE_ERROR;
        }
        return getNextString(
            pParse,
            zInput.offset(1 as isize) as *const ::core::ffi::c_char,
            ii - 1 as ::core::ffi::c_int,
            ppExpr,
        );
    }
    if sqlite3_fts3_enable_parentheses != 0 {
        if *zInput as ::core::ffi::c_int == '(' as i32 {
            let mut nConsumed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            __pParse_ref.nNest += 1;
            if __pParse_ref.nNest > 1000 as ::core::ffi::c_int {
                return crate::src::headers::sqlite3_h::SQLITE_ERROR;
            }
            rc = fts3ExprParse(
                pParse,
                zInput.offset(1 as isize),
                nInput - 1 as ::core::ffi::c_int,
                ppExpr,
                &raw mut nConsumed,
            );
            *pnConsumed = zInput.offset_from(z) as ::core::ffi::c_long as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + nConsumed;
            return rc;
        } else if *zInput as ::core::ffi::c_int == ')' as i32 {
            __pParse_ref.nNest -= 1;
            *pnConsumed = (zInput.offset_from(z) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                as ::core::ffi::c_int;
            *ppExpr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
            return crate::src::headers::sqlite3_h::SQLITE_DONE;
        }
    }
    iCol = __pParse_ref.iDefaultCol;
    iColLen = 0 as ::core::ffi::c_int;
    ii = 0 as ::core::ffi::c_int;
    while ii < __pParse_ref.nCol {
        let mut zStr: *const ::core::ffi::c_char = *__pParse_ref.azCol.offset(ii as isize);
        let mut nStr: ::core::ffi::c_int = ::libc::strlen(zStr) as ::core::ffi::c_int;
        if nInput > nStr
            && *zInput.offset(nStr as isize) as ::core::ffi::c_int == ':' as i32
            && crate::src::src::util::sqlite3_strnicmp(zStr, zInput, nStr) == 0 as ::core::ffi::c_int
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
    rc
}

unsafe extern "C" fn opPrecedence(mut p: *mut crate::fts3Int_h::Fts3Expr) -> ::core::ffi::c_int {
    if sqlite3_fts3_enable_parentheses != 0 {
        return (*p).eType;
    } else if (*p).eType == crate::fts3Int_h::FTSQUERY_NEAR {
        return 1 as ::core::ffi::c_int;
    } else if (*p).eType == crate::fts3Int_h::FTSQUERY_OR_1 {
        return 2 as ::core::ffi::c_int;
    }
    3 as ::core::ffi::c_int
}

unsafe extern "C" fn insertBinaryOperator(
    mut ppHead: *mut *mut crate::fts3Int_h::Fts3Expr,
    mut pPrev: *mut crate::fts3Int_h::Fts3Expr,
    mut pNew: *mut crate::fts3Int_h::Fts3Expr,
) {
    let mut pSplit: *mut crate::fts3Int_h::Fts3Expr = pPrev;
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
    mut ppExpr: *mut *mut crate::fts3Int_h::Fts3Expr,
    mut pnConsumed: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut pRet: *mut crate::fts3Int_h::Fts3Expr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
    let mut pPrev: *mut crate::fts3Int_h::Fts3Expr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
    let mut pNotBranch: *mut crate::fts3Int_h::Fts3Expr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
    let mut nIn: ::core::ffi::c_int = n;
    let mut zIn: *const ::core::ffi::c_char = z;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut isRequirePhrase: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    loop {
        if !(rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
            current_block = 1434579379687443766;
            break;
        }
        let mut p: *mut crate::fts3Int_h::Fts3Expr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
        let mut nByte: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        rc = getNextNode(pParse, zIn, nIn, &raw mut p, &raw mut nByte);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            if !p.is_null() {
                let mut isPhrase: ::core::ffi::c_int = 0;
                if sqlite3_fts3_enable_parentheses == 0
                    && (*p).eType == crate::fts3Int_h::FTSQUERY_PHRASE
                    && (*pParse).isNot != 0
                {
                    let mut pNot: *mut crate::fts3Int_h::Fts3Expr =
                        sqlite3Fts3MallocZero(::core::mem::size_of::<crate::fts3Int_h::Fts3Expr>() as crate::src::headers::sqlite3_h::sqlite3_int64)
                            as *mut crate::fts3Int_h::Fts3Expr;
                    if pNot.is_null() {
                        sqlite3Fts3ExprFree(p);
                        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                        current_block = 16633387481569624112;
                        break;
                    } else {
                        (*pNot).eType = crate::fts3Int_h::FTSQUERY_NOT_1;
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
                        (eType == crate::fts3Int_h::FTSQUERY_PHRASE || !(*p).pLeft.is_null()) as ::core::ffi::c_int;
                    if isPhrase == 0 && isRequirePhrase != 0 {
                        sqlite3Fts3ExprFree(p);
                        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                        current_block = 16633387481569624112;
                        break;
                    } else {
                        if isPhrase != 0 && isRequirePhrase == 0 {
                            let mut pAnd: *mut crate::fts3Int_h::Fts3Expr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
                            pAnd = sqlite3Fts3MallocZero(
                                ::core::mem::size_of::<crate::fts3Int_h::Fts3Expr>() as crate::src::headers::sqlite3_h::sqlite3_int64
                            ) as *mut crate::fts3Int_h::Fts3Expr;
                            if pAnd.is_null() {
                                sqlite3Fts3ExprFree(p);
                                rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                                current_block = 16633387481569624112;
                                break;
                            } else {
                                (*pAnd).eType = crate::fts3Int_h::FTSQUERY_AND_1;
                                insertBinaryOperator(&raw mut pRet, pPrev, pAnd);
                                pPrev = pAnd;
                            }
                        }
                        if !pPrev.is_null()
                            && (eType == crate::fts3Int_h::FTSQUERY_NEAR
                                && isPhrase == 0
                                && (*pPrev).eType != crate::fts3Int_h::FTSQUERY_PHRASE
                                || eType != crate::fts3Int_h::FTSQUERY_PHRASE
                                    && isPhrase != 0
                                    && (*pPrev).eType == crate::fts3Int_h::FTSQUERY_NEAR)
                        {
                            sqlite3Fts3ExprFree(p);
                            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
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
            if rc == crate::src::headers::sqlite3_h::SQLITE_DONE && !pRet.is_null() && isRequirePhrase != 0 {
                rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
                rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                if sqlite3_fts3_enable_parentheses == 0 && !pNotBranch.is_null() {
                    if pRet.is_null() {
                        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                    } else {
                        let mut pIter: *mut crate::fts3Int_h::Fts3Expr = pNotBranch;
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
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        sqlite3Fts3ExprFree(pRet);
        sqlite3Fts3ExprFree(pNotBranch);
        pRet = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
    }
    *ppExpr = pRet;
    rc
}

unsafe extern "C" fn fts3ExprCheckDepth(
    mut p: *mut crate::fts3Int_h::Fts3Expr,
    mut nMaxDepth: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if !p.is_null() {
        if nMaxDepth < 0 as ::core::ffi::c_int {
            rc = crate::src::headers::sqlite3_h::SQLITE_TOOBIG;
        } else {
            rc = fts3ExprCheckDepth((*p).pLeft, nMaxDepth - 1 as ::core::ffi::c_int);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = fts3ExprCheckDepth((*p).pRight, nMaxDepth - 1 as ::core::ffi::c_int);
            }
        }
    }
    rc
}

unsafe extern "C" fn fts3ExprBalance(
    mut pp: *mut *mut crate::fts3Int_h::Fts3Expr,
    mut nMaxDepth: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pRoot: *mut crate::fts3Int_h::Fts3Expr = *pp;
    let mut pFree: *mut crate::fts3Int_h::Fts3Expr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
    let mut eType: ::core::ffi::c_int = (*pRoot).eType;
    if nMaxDepth == 0 as ::core::ffi::c_int {
        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if eType == crate::fts3Int_h::FTSQUERY_AND_1 || eType == crate::fts3Int_h::FTSQUERY_OR_1 {
            let mut apLeaf: *mut *mut crate::fts3Int_h::Fts3Expr = ::core::ptr::null_mut::<*mut crate::fts3Int_h::Fts3Expr>();
            apLeaf = crate::src::src::malloc::sqlite3_malloc64(
                (::core::mem::size_of::<*mut crate::fts3Int_h::Fts3Expr>() as usize).wrapping_mul(nMaxDepth as usize)
                    as crate::src::headers::sqlite3_h::sqlite3_uint64,
            ) as *mut *mut crate::fts3Int_h::Fts3Expr;
            if apLeaf.is_null() {
                rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            } else {
                ::libc::memset(
                    apLeaf as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (::core::mem::size_of::<*mut crate::fts3Int_h::Fts3Expr>() as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(nMaxDepth as crate::__stddef_size_t_h::size_t),
                );
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                let mut i: ::core::ffi::c_int = 0;
                let mut p: *mut crate::fts3Int_h::Fts3Expr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
                p = pRoot;
                while (*p).eType == eType {
                    p = (*p).pLeft;
                }
                loop {
                    let mut iLvl: ::core::ffi::c_int = 0;
                    let mut pParent: *mut crate::fts3Int_h::Fts3Expr = (*p).pParent;
                    (*p).pParent = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
                    if !pParent.is_null() {
                        (*pParent).pLeft = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
                    } else {
                        pRoot = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
                    }
                    rc = fts3ExprBalance(&raw mut p, nMaxDepth - 1 as ::core::ffi::c_int);
                    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                        break;
                    }
                    iLvl = 0 as ::core::ffi::c_int;
                    while !p.is_null() && iLvl < nMaxDepth {
                        if (*apLeaf.offset(iLvl as isize)).is_null() {
                            let ref mut fresh2 = *apLeaf.offset(iLvl as isize);
                            *fresh2 = p;
                            p = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
                        } else {
                            (*pFree).pLeft = *apLeaf.offset(iLvl as isize);
                            (*pFree).pRight = p;
                            (*(*pFree).pLeft).pParent = pFree;
                            (*(*pFree).pRight).pParent = pFree;
                            p = pFree;
                            pFree = (*pFree).pParent;
                            (*p).pParent = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
                            let ref mut fresh3 = *apLeaf.offset(iLvl as isize);
                            *fresh3 = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
                        }
                        iLvl += 1;
                    }
                    if !p.is_null() {
                        sqlite3Fts3ExprFree(p);
                        rc = crate::src::headers::sqlite3_h::SQLITE_TOOBIG;
                        break;
                    } else {
                        if pParent.is_null() {
                            break;
                        }
                        let __pParent_ref = unsafe { &mut *pParent };
                        p = __pParent_ref.pRight;
                        while (*p).eType == eType {
                            p = (*p).pLeft;
                        }
                        (*__pParent_ref.pRight).pParent = __pParent_ref.pParent;
                        if !__pParent_ref.pParent.is_null() {
                            (*__pParent_ref.pParent).pLeft = __pParent_ref.pRight;
                        } else {
                            pRoot = __pParent_ref.pRight;
                        }
                        __pParent_ref.pParent = pFree;
                        pFree = pParent;
                    }
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    p = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
                    i = 0 as ::core::ffi::c_int;
                    while i < nMaxDepth {
                        if !(*apLeaf.offset(i as isize)).is_null() {
                            if p.is_null() {
                                p = *apLeaf.offset(i as isize);
                                (*p).pParent = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
                            } else {
                                (*pFree).pRight = p;
                                (*pFree).pLeft = *apLeaf.offset(i as isize);
                                (*(*pFree).pLeft).pParent = pFree;
                                (*(*pFree).pRight).pParent = pFree;
                                p = pFree;
                                pFree = (*pFree).pParent;
                                (*p).pParent = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
                            }
                        }
                        i += 1;
                    }
                    pRoot = p;
                } else {
                    let mut pDel: *mut crate::fts3Int_h::Fts3Expr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
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
                        crate::src::src::malloc::sqlite3_free(pDel as *mut ::core::ffi::c_void);
                    }
                }
                crate::src::src::malloc::sqlite3_free(apLeaf as *mut ::core::ffi::c_void);
            }
        } else if eType == crate::fts3Int_h::FTSQUERY_NOT_1 {
            let __pRoot_ref = unsafe { &mut *pRoot };
            let mut pLeft: *mut crate::fts3Int_h::Fts3Expr = __pRoot_ref.pLeft;
            let mut pRight: *mut crate::fts3Int_h::Fts3Expr = __pRoot_ref.pRight;
            __pRoot_ref.pLeft = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
            __pRoot_ref.pRight = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
            (*pLeft).pParent = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
            (*pRight).pParent = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
            rc = fts3ExprBalance(&raw mut pLeft, nMaxDepth - 1 as ::core::ffi::c_int);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = fts3ExprBalance(&raw mut pRight, nMaxDepth - 1 as ::core::ffi::c_int);
            }
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                sqlite3Fts3ExprFree(pRight);
                sqlite3Fts3ExprFree(pLeft);
            } else {
                __pRoot_ref.pLeft = pLeft;
                (*pLeft).pParent = pRoot;
                __pRoot_ref.pRight = pRight;
                (*pRight).pParent = pRoot;
            }
        }
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        sqlite3Fts3ExprFree(pRoot);
        pRoot = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
    }
    *pp = pRoot;
    rc
}

unsafe extern "C" fn fts3ExprParseUnbalanced(
    mut pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
    mut iLangid: ::core::ffi::c_int,
    mut azCol: *mut *mut ::core::ffi::c_char,
    mut bFts4: ::core::ffi::c_int,
    mut nCol: ::core::ffi::c_int,
    mut iDefaultCol: ::core::ffi::c_int,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut ppExpr: *mut *mut crate::fts3Int_h::Fts3Expr,
) -> ::core::ffi::c_int {
    let mut nParsed: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0;
    let mut sParse: ParseContext = unsafe { ::core::mem::zeroed() };
    sParse.pTokenizer = pTokenizer;
    sParse.iLangid = iLangid;
    sParse.azCol = azCol as *mut *const ::core::ffi::c_char;
    sParse.nCol = nCol;
    sParse.iDefaultCol = iDefaultCol;
    sParse.bFts4 = bFts4;
    if z.is_null() {
        *ppExpr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    if n < 0 as ::core::ffi::c_int {
        n = ::libc::strlen(z) as ::core::ffi::c_int;
    }
    rc = fts3ExprParse(&raw mut sParse, z, n, ppExpr, &raw mut nParsed);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && sParse.nNest != 0 {
        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    rc
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3Fts3ExprParse(
    mut pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
    mut iLangid: ::core::ffi::c_int,
    mut azCol: *mut *mut ::core::ffi::c_char,
    mut bFts4: ::core::ffi::c_int,
    mut nCol: ::core::ffi::c_int,
    mut iDefaultCol: ::core::ffi::c_int,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut ppExpr: *mut *mut crate::fts3Int_h::Fts3Expr,
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
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !(*ppExpr).is_null() {
        rc = fts3ExprBalance(ppExpr, crate::fts3Int_h::SQLITE_FTS3_MAX_EXPR_DEPTH);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = fts3ExprCheckDepth(*ppExpr, crate::fts3Int_h::SQLITE_FTS3_MAX_EXPR_DEPTH);
        }
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        sqlite3Fts3ExprFree(*ppExpr);
        *ppExpr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
        if rc == crate::src::headers::sqlite3_h::SQLITE_TOOBIG {
            crate::src::ext::fts3::fts3::sqlite3Fts3ErrMsg(
                pzErr,
                b"FTS expression tree is too large (maximum depth %d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                crate::fts3Int_h::SQLITE_FTS3_MAX_EXPR_DEPTH,
            );
            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
        } else if rc == crate::src::headers::sqlite3_h::SQLITE_ERROR {
            crate::src::ext::fts3::fts3::sqlite3Fts3ErrMsg(
                pzErr,
                b"malformed MATCH expression: [%s]\0" as *const u8 as *const ::core::ffi::c_char,
                z,
            );
        }
    }
    rc
}

unsafe extern "C" fn fts3FreeExprNode(mut p: *mut crate::fts3Int_h::Fts3Expr) {
    crate::src::ext::fts3::fts3::sqlite3Fts3EvalPhraseCleanup((*p).pPhrase as *mut crate::fts3Int_h::Fts3Phrase);
    crate::src::src::malloc::sqlite3_free((*p).aMI as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3Fts3ExprFree(mut pDel: *mut crate::fts3Int_h::Fts3Expr) {
    let mut p: *mut crate::fts3Int_h::Fts3Expr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
    p = pDel;
    while !p.is_null() && (!(*p).pLeft.is_null() || !(*p).pRight.is_null()) {
        p = if !(*p).pLeft.is_null() {
            (*p).pLeft
        } else {
            (*p).pRight
        };
    }
    while !p.is_null() {
        let mut pParent: *mut crate::fts3Int_h::Fts3Expr = (*p).pParent;
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
    mut pExpr: *mut crate::fts3Int_h::Fts3Expr,
    mut zBuf: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if pExpr.is_null() {
        return crate::sqlite_printf!("");
    }
    match  (*pExpr).eType {
    crate::fts3Int_h::FTSQUERY_PHRASE =>  {
            let mut pPhrase: *mut crate::fts3Int_h::Fts3Phrase = (*pExpr).pPhrase;
            let mut i: ::core::ffi::c_int = 0;
            zBuf = crate::sqlite_printf!("%zPHRASE %d 0", zBuf, (*pPhrase).iColumn);
            i = 0 as ::core::ffi::c_int;
            while !zBuf.is_null() && i < (*pPhrase).nToken {
                let __pPhrase_ref = unsafe { &mut *pPhrase };
                let tok_ptr = (&raw mut __pPhrase_ref.aToken as *mut crate::fts3Int_h::Fts3PhraseToken).offset(i as isize);
                let tok_n = (*tok_ptr).n as usize;
                let tok_z = (*tok_ptr).z;
                let tok_is_prefix = (*tok_ptr).isPrefix;
                // Create null-terminated copy of token (token.z is not guaranteed null-terminated)
                let zTok = crate::src::src::malloc::sqlite3_malloc64((tok_n as u64).wrapping_add(1)) as *mut ::core::ffi::c_char;
                if zTok.is_null() {
                    crate::src::src::malloc::sqlite3_free(zBuf as *mut ::core::ffi::c_void);
                    zBuf = ::core::ptr::null_mut();
                } else {
                    ::core::ptr::copy_nonoverlapping(tok_z, zTok, tok_n);
                    *zTok.add(tok_n) = 0;
                    let suffix = if tok_is_prefix != 0 {
                        b"+\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    };
                    zBuf = crate::sqlite_printf!("%z %s%s", zBuf, zTok, suffix);
                    crate::src::src::malloc::sqlite3_free(zTok as *mut ::core::ffi::c_void);
                }
                i += 1;
            }
            return zBuf;
        }
    crate::fts3Int_h::FTSQUERY_NEAR =>  {
            zBuf = crate::sqlite_printf!("%zNEAR/%d ", zBuf, (*pExpr).nNear);
        }
    crate::fts3Int_h::FTSQUERY_NOT_1 =>  {
            zBuf = crate::sqlite_printf!("%zNOT ", zBuf);
        }
    crate::fts3Int_h::FTSQUERY_AND_1 =>  {
            zBuf = crate::sqlite_printf!("%zAND ", zBuf);
        }
    crate::fts3Int_h::FTSQUERY_OR_1 =>  {
            zBuf = crate::sqlite_printf!("%zOR ", zBuf);
        }
    _ =>  {}
}
    if !zBuf.is_null() {
        zBuf = crate::sqlite_printf!("%z{{", zBuf);
    }
    if !zBuf.is_null() {
        zBuf = exprToString((*pExpr).pLeft, zBuf);
    }
    if !zBuf.is_null() {
        zBuf = crate::sqlite_printf!("%z}} {{", zBuf);
    }
    if !zBuf.is_null() {
        zBuf = exprToString((*pExpr).pRight, zBuf);
    }
    if !zBuf.is_null() {
        zBuf = crate::sqlite_printf!("%z}}", zBuf);
    }
    zBuf
}

unsafe extern "C" fn fts3ExprTestCommon(
    mut bRebalance: ::core::ffi::c_int,
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let mut pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer>();
    let mut rc: ::core::ffi::c_int = 0;
    let mut azCol: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut zExpr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nExpr: ::core::ffi::c_int = 0;
    let mut nCol: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    let mut pExpr: *mut crate::fts3Int_h::Fts3Expr = ::core::ptr::null_mut::<crate::fts3Int_h::Fts3Expr>();
    let mut zBuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pHash: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash = crate::src::src::vdbeapi::sqlite3_user_data(context) as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash;
    let mut zTokenizer: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if argc < 3 as ::core::ffi::c_int {
        crate::src::src::vdbeapi::sqlite3_result_error(
            context,
            b"Usage: fts3_exprtest(tokenizer, expr, col1, ...\0" as *const u8
                as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int),
        );
        return;
    }
    zTokenizer = crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(0 as isize))
        as *const ::core::ffi::c_char;
    rc = crate::src::ext::fts3::fts3_tokenizer::sqlite3Fts3InitTokenizer(pHash as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash, zTokenizer,  &raw mut pTokenizer as *mut _ as
    *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer, &raw mut zErr);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        if rc == crate::src::headers::sqlite3_h::SQLITE_NOMEM {
            crate::src::src::vdbeapi::sqlite3_result_error_nomem(context);
        } else {
            crate::src::src::vdbeapi::sqlite3_result_error(context, zErr, -(1 as ::core::ffi::c_int));
        }
        crate::src::src::malloc::sqlite3_free(zErr as *mut ::core::ffi::c_void);
        return;
    }
    zExpr = crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(1 as isize))
        as *const ::core::ffi::c_char;
    nExpr = crate::src::src::vdbeapi::sqlite3_value_bytes(*argv.offset(1 as isize));
    nCol = argc - 2 as ::core::ffi::c_int;
    azCol = crate::src::src::malloc::sqlite3_malloc64(
        (nCol as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            as crate::src::headers::sqlite3_h::sqlite3_uint64,
    ) as *mut *mut ::core::ffi::c_char;
    if azCol.is_null() {
        crate::src::src::vdbeapi::sqlite3_result_error_nomem(context);
    } else {
        ii = 0 as ::core::ffi::c_int;
        while ii < nCol {
            let ref mut fresh4 = *azCol.offset(ii as isize);
            *fresh4 = crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset((ii + 2 as ::core::ffi::c_int) as isize))
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
            crate::src::src::malloc::sqlite3_free(zDummy as *mut ::core::ffi::c_void);
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
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK && rc != crate::src::headers::sqlite3_h::SQLITE_NOMEM {
            crate::src::src::vdbeapi::sqlite3_result_error(
                context,
                b"Error parsing expression\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int),
            );
        } else if rc == crate::src::headers::sqlite3_h::SQLITE_NOMEM || {
            zBuf = exprToString(pExpr, ::core::ptr::null_mut::<::core::ffi::c_char>());
            zBuf.is_null()
        } {
            crate::src::src::vdbeapi::sqlite3_result_error_nomem(context);
        } else {
            crate::src::src::vdbeapi::sqlite3_result_text(
                context,
                zBuf,
                -(1 as ::core::ffi::c_int),
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
            );
            crate::src::src::malloc::sqlite3_free(zBuf as *mut ::core::ffi::c_void);
        }
        sqlite3Fts3ExprFree(pExpr);
    }
    if !pTokenizer.is_null() {
        rc = (*(*pTokenizer).pModule)
            .xDestroy
            .expect("non-null function pointer")(pTokenizer);
    }
    crate::src::src::malloc::sqlite3_free(azCol as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn fts3ExprTest(
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    fts3ExprTestCommon(0 as ::core::ffi::c_int, context, argc, argv);
}

unsafe extern "C" fn fts3ExprTestRebalance(
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    fts3ExprTestCommon(1 as ::core::ffi::c_int, context, argc, argv);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3Fts3ExprInitTestInterface(
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut pHash: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::src::main::sqlite3_create_function(
        db,
        b"fts3_exprtest\0" as *const u8 as *const ::core::ffi::c_char,
        -(1 as ::core::ffi::c_int),
        crate::src::headers::sqlite3_h::SQLITE_UTF8,
        pHash as *mut ::core::ffi::c_void,
        Some(
            fts3ExprTest
                as unsafe extern "C" fn(
                    *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                ) -> (),
        ),
        None,
        None,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = crate::src::src::main::sqlite3_create_function(
            db,
            b"fts3_exprtest_rebalance\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int),
            crate::src::headers::sqlite3_h::SQLITE_UTF8,
            pHash as *mut ::core::ffi::c_void,
            Some(
                fts3ExprTestRebalance
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
    }
    rc
}
