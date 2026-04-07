use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Fts5Context;
    pub type Fts5Tokenizer;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_prepare(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_pointer(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_char,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
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
    fn sqlite3_result_error_nomem(_: *mut sqlite3_context);
    fn sqlite3_result_error_code(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
}
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts5ExtensionApi {
    pub iVersion: ::core::ffi::c_int,
    pub xUserData: Option<
        unsafe extern "C" fn(*mut Fts5Context) -> *mut ::core::ffi::c_void,
    >,
    pub xColumnCount: Option<
        unsafe extern "C" fn(*mut Fts5Context) -> ::core::ffi::c_int,
    >,
    pub xRowCount: Option<
        unsafe extern "C" fn(*mut Fts5Context, *mut sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub xColumnTotalSize: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xTokenize: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        ) -> ::core::ffi::c_int,
    >,
    pub xPhraseCount: Option<
        unsafe extern "C" fn(*mut Fts5Context) -> ::core::ffi::c_int,
    >,
    pub xPhraseSize: Option<
        unsafe extern "C" fn(*mut Fts5Context, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xInstCount: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xInst: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xRowid: Option<unsafe extern "C" fn(*mut Fts5Context) -> sqlite3_int64>,
    pub xColumnText: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xColumnSize: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xQueryPhrase: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *const Fts5ExtensionApi,
                    *mut Fts5Context,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >,
        ) -> ::core::ffi::c_int,
    >,
    pub xSetAuxdata: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            *mut ::core::ffi::c_void,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub xGetAuxdata: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub xPhraseFirst: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut Fts5PhraseIter,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xPhraseNext: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            *mut Fts5PhraseIter,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub xPhraseFirstColumn: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut Fts5PhraseIter,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xPhraseNextColumn: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            *mut Fts5PhraseIter,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub xQueryToken: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xInstToken: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xColumnLocale: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xTokenize_v2: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts5PhraseIter {
    pub a: *const ::core::ffi::c_uchar,
    pub b: *const ::core::ffi::c_uchar,
}
pub type fts5_extension_function = Option<
    unsafe extern "C" fn(
        *const Fts5ExtensionApi,
        *mut Fts5Context,
        *mut sqlite3_context,
        ::core::ffi::c_int,
        *mut *mut sqlite3_value,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fts5_tokenizer_v2 {
    pub iVersion: ::core::ffi::c_int,
    pub xCreate: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut Fts5Tokenizer,
        ) -> ::core::ffi::c_int,
    >,
    pub xDelete: Option<unsafe extern "C" fn(*mut Fts5Tokenizer) -> ()>,
    pub xTokenize: Option<
        unsafe extern "C" fn(
            *mut Fts5Tokenizer,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fts5_tokenizer {
    pub xCreate: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut Fts5Tokenizer,
        ) -> ::core::ffi::c_int,
    >,
    pub xDelete: Option<unsafe extern "C" fn(*mut Fts5Tokenizer) -> ()>,
    pub xTokenize: Option<
        unsafe extern "C" fn(
            *mut Fts5Tokenizer,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fts5_api {
    pub iVersion: ::core::ffi::c_int,
    pub xCreateTokenizer: Option<
        unsafe extern "C" fn(
            *mut fts5_api,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
            *mut fts5_tokenizer,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub xFindTokenizer: Option<
        unsafe extern "C" fn(
            *mut fts5_api,
            *const ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_void,
            *mut fts5_tokenizer,
        ) -> ::core::ffi::c_int,
    >,
    pub xCreateFunction: Option<
        unsafe extern "C" fn(
            *mut fts5_api,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
            fts5_extension_function,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub xCreateTokenizer_v2: Option<
        unsafe extern "C" fn(
            *mut fts5_api,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
            *mut fts5_tokenizer_v2,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub xFindTokenizer_v2: Option<
        unsafe extern "C" fn(
            *mut fts5_api,
            *const ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_void,
            *mut *mut fts5_tokenizer_v2,
        ) -> ::core::ffi::c_int,
    >,
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts5MatchinfoCtx {
    pub nCol: ::core::ffi::c_int,
    pub nPhrase: ::core::ffi::c_int,
    pub zArg: *mut ::core::ffi::c_char,
    pub nRet: ::core::ffi::c_int,
    pub aRet: *mut u32_0,
}
pub type u32_0 = ::core::ffi::c_uint;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
unsafe extern "C" fn fts5_api_from_db(
    mut db: *mut sqlite3,
    mut ppApi: *mut *mut fts5_api,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = 0;
        *ppApi = ::core::ptr::null_mut::<fts5_api>();
        rc = sqlite3_prepare(
            db,
            b"SELECT fts5(?1)\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_pointer(
                pStmt,
                1 as ::core::ffi::c_int,
                ppApi as *mut ::core::ffi::c_void,
                b"fts5_api_ptr\0".as_ptr() as *const ::core::ffi::c_char,
                None,
            );
            sqlite3_step(pStmt);
            rc = sqlite3_finalize(pStmt);
        }
        return rc;
    }
}
unsafe extern "C" fn fts5MatchinfoFlagsize(
    mut nCol: ::core::ffi::c_int,
    mut nPhrase: ::core::ffi::c_int,
    mut f: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        match f as ::core::ffi::c_int {
            112 => {
                ret = 1 as ::core::ffi::c_int;
            }
            99 => {
                ret = 1 as ::core::ffi::c_int;
            }
            120 => {
                ret = 3 as ::core::ffi::c_int * nCol * nPhrase;
            }
            121 => {
                ret = nCol * nPhrase;
            }
            98 => {
                ret = (nCol + 31 as ::core::ffi::c_int) / 32 as ::core::ffi::c_int
                    * nPhrase;
            }
            110 => {
                ret = 1 as ::core::ffi::c_int;
            }
            97 => {
                ret = nCol;
            }
            108 => {
                ret = nCol;
            }
            115 => {
                ret = nCol;
            }
            _ => {}
        }
        return ret;
    }
}
unsafe extern "C" fn fts5MatchinfoIter(
    mut pApi: *const Fts5ExtensionApi,
    mut pFts: *mut Fts5Context,
    mut p: *mut Fts5MatchinfoCtx,
    mut x: Option<
        unsafe extern "C" fn(
            *const Fts5ExtensionApi,
            *mut Fts5Context,
            *mut Fts5MatchinfoCtx,
            ::core::ffi::c_char,
            *mut u32_0,
        ) -> ::core::ffi::c_int,
    >,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut f: ::core::ffi::c_char = 0;
        i = 0 as ::core::ffi::c_int;
        loop {
            f = *(*p).zArg.offset(i as isize);
            if !(f != 0) {
                break;
            }
            rc = x
                .expect(
                    "non-null function pointer",
                )(pApi, pFts, p, f, (*p).aRet.offset(n as isize) as *mut u32_0);
            if rc != SQLITE_OK {
                break;
            }
            n += fts5MatchinfoFlagsize((*p).nCol, (*p).nPhrase, f);
            i += 1;
        }
        return rc;
    }
}
unsafe extern "C" fn fts5MatchinfoXCb(
    mut pApi: *const Fts5ExtensionApi,
    mut pFts: *mut Fts5Context,
    mut pUserData: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iter: Fts5PhraseIter = Fts5PhraseIter {
            a: ::core::ptr::null::<::core::ffi::c_uchar>(),
            b: ::core::ptr::null::<::core::ffi::c_uchar>(),
        };
        let mut iCol: ::core::ffi::c_int = 0;
        let mut iOff: ::core::ffi::c_int = 0;
        let mut aOut: *mut u32_0 = pUserData as *mut u32_0;
        let mut iPrev: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        (*pApi)
            .xPhraseFirst
            .expect(
                "non-null function pointer",
            )(
            pFts,
            0 as ::core::ffi::c_int,
            &raw mut iter,
            &raw mut iCol,
            &raw mut iOff,
        );
        while iCol >= 0 as ::core::ffi::c_int {
            let ref mut c2rust_fresh0 = *aOut
                .offset(
                    (iCol * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                );
            *c2rust_fresh0 = (*c2rust_fresh0).wrapping_add(1);
            if iCol != iPrev {
                let ref mut c2rust_fresh1 = *aOut
                    .offset(
                        (iCol * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                            as isize,
                    );
                *c2rust_fresh1 = (*c2rust_fresh1).wrapping_add(1);
            }
            iPrev = iCol;
            (*pApi)
                .xPhraseNext
                .expect(
                    "non-null function pointer",
                )(pFts, &raw mut iter, &raw mut iCol, &raw mut iOff);
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fts5MatchinfoGlobalCb(
    mut pApi: *const Fts5ExtensionApi,
    mut pFts: *mut Fts5Context,
    mut p: *mut Fts5MatchinfoCtx,
    mut f: ::core::ffi::c_char,
    mut aOut: *mut u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        match f as ::core::ffi::c_int {
            112 => {
                *aOut.offset(0 as ::core::ffi::c_int as isize) = (*p).nPhrase as u32_0;
            }
            99 => {
                *aOut.offset(0 as ::core::ffi::c_int as isize) = (*p).nCol as u32_0;
            }
            120 => {
                let mut i: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                while i < (*p).nPhrase && rc == SQLITE_OK {
                    let mut pPtr: *mut ::core::ffi::c_void = aOut
                        .offset((i * (*p).nCol * 3 as ::core::ffi::c_int) as isize)
                        as *mut u32_0 as *mut ::core::ffi::c_void;
                    rc = (*pApi)
                        .xQueryPhrase
                        .expect(
                            "non-null function pointer",
                        )(
                        pFts,
                        i,
                        pPtr,
                        Some(
                            fts5MatchinfoXCb
                                as unsafe extern "C" fn(
                                    *const Fts5ExtensionApi,
                                    *mut Fts5Context,
                                    *mut ::core::ffi::c_void,
                                ) -> ::core::ffi::c_int,
                        ),
                    );
                    i += 1;
                }
            }
            110 => {
                let mut nRow: sqlite3_int64 = 0;
                rc = (*pApi)
                    .xRowCount
                    .expect("non-null function pointer")(pFts, &raw mut nRow);
                *aOut.offset(0 as ::core::ffi::c_int as isize) = nRow as u32_0;
            }
            97 => {
                let mut nRow_0: sqlite3_int64 = 0 as sqlite3_int64;
                rc = (*pApi)
                    .xRowCount
                    .expect("non-null function pointer")(pFts, &raw mut nRow_0);
                if nRow_0 == 0 as ::core::ffi::c_longlong {
                    memset(
                        aOut as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (::core::mem::size_of::<u32_0>() as size_t)
                            .wrapping_mul((*p).nCol as size_t),
                    );
                } else {
                    let mut i_0: ::core::ffi::c_int = 0;
                    i_0 = 0 as ::core::ffi::c_int;
                    while rc == SQLITE_OK && i_0 < (*p).nCol {
                        let mut nToken: sqlite3_int64 = 0;
                        rc = (*pApi)
                            .xColumnTotalSize
                            .expect(
                                "non-null function pointer",
                            )(pFts, i_0, &raw mut nToken);
                        if rc == SQLITE_OK {
                            *aOut.offset(i_0 as isize) = ((2 as sqlite3_int64 * nToken
                                + nRow_0) / (2 as sqlite3_int64 * nRow_0)) as u32_0;
                        }
                        i_0 += 1;
                    }
                }
            }
            _ => {}
        }
        return rc;
    }
}
unsafe extern "C" fn fts5MatchinfoLocalCb(
    mut pApi: *const Fts5ExtensionApi,
    mut pFts: *mut Fts5Context,
    mut p: *mut Fts5MatchinfoCtx,
    mut f: ::core::ffi::c_char,
    mut aOut: *mut u32_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        match f as ::core::ffi::c_int {
            98 => {
                let mut iPhrase: ::core::ffi::c_int = 0;
                let mut nInt: ::core::ffi::c_int = ((*p).nCol + 31 as ::core::ffi::c_int)
                    / 32 as ::core::ffi::c_int * (*p).nPhrase;
                i = 0 as ::core::ffi::c_int;
                while i < nInt {
                    *aOut.offset(i as isize) = 0 as u32_0;
                    i += 1;
                }
                iPhrase = 0 as ::core::ffi::c_int;
                while iPhrase < (*p).nPhrase {
                    let mut iter: Fts5PhraseIter = Fts5PhraseIter {
                        a: ::core::ptr::null::<::core::ffi::c_uchar>(),
                        b: ::core::ptr::null::<::core::ffi::c_uchar>(),
                    };
                    let mut iCol: ::core::ffi::c_int = 0;
                    (*pApi)
                        .xPhraseFirstColumn
                        .expect(
                            "non-null function pointer",
                        )(pFts, iPhrase, &raw mut iter, &raw mut iCol);
                    while iCol >= 0 as ::core::ffi::c_int {
                        let ref mut c2rust_fresh2 = *aOut
                            .offset(
                                (iPhrase
                                    * (((*p).nCol + 31 as ::core::ffi::c_int)
                                        / 32 as ::core::ffi::c_int)
                                    + iCol / 32 as ::core::ffi::c_int) as isize,
                            );
                        *c2rust_fresh2
                            |= ((1 as ::core::ffi::c_int as u32_0)
                                << iCol % 32 as ::core::ffi::c_int) as ::core::ffi::c_uint;
                        (*pApi)
                            .xPhraseNextColumn
                            .expect(
                                "non-null function pointer",
                            )(pFts, &raw mut iter, &raw mut iCol);
                    }
                    iPhrase += 1;
                }
            }
            120 | 121 => {
                let mut nMul: ::core::ffi::c_int = if f as ::core::ffi::c_int
                    == 'x' as i32
                {
                    3 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                };
                let mut iPhrase_0: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                while i < (*p).nCol * (*p).nPhrase {
                    *aOut.offset((i * nMul) as isize) = 0 as u32_0;
                    i += 1;
                }
                iPhrase_0 = 0 as ::core::ffi::c_int;
                while iPhrase_0 < (*p).nPhrase {
                    let mut iter_0: Fts5PhraseIter = Fts5PhraseIter {
                        a: ::core::ptr::null::<::core::ffi::c_uchar>(),
                        b: ::core::ptr::null::<::core::ffi::c_uchar>(),
                    };
                    let mut iOff: ::core::ffi::c_int = 0;
                    let mut iCol_0: ::core::ffi::c_int = 0;
                    (*pApi)
                        .xPhraseFirst
                        .expect(
                            "non-null function pointer",
                        )(
                        pFts,
                        iPhrase_0,
                        &raw mut iter_0,
                        &raw mut iCol_0,
                        &raw mut iOff,
                    );
                    while iOff >= 0 as ::core::ffi::c_int {
                        let ref mut c2rust_fresh3 = *aOut
                            .offset((nMul * (iCol_0 + iPhrase_0 * (*p).nCol)) as isize);
                        *c2rust_fresh3 = (*c2rust_fresh3).wrapping_add(1);
                        (*pApi)
                            .xPhraseNext
                            .expect(
                                "non-null function pointer",
                            )(pFts, &raw mut iter_0, &raw mut iCol_0, &raw mut iOff);
                    }
                    iPhrase_0 += 1;
                }
            }
            108 => {
                i = 0 as ::core::ffi::c_int;
                while rc == SQLITE_OK && i < (*p).nCol {
                    let mut nToken: ::core::ffi::c_int = 0;
                    rc = (*pApi)
                        .xColumnSize
                        .expect("non-null function pointer")(pFts, i, &raw mut nToken);
                    *aOut.offset(i as isize) = nToken as u32_0;
                    i += 1;
                }
            }
            115 => {
                let mut nInst: ::core::ffi::c_int = 0;
                memset(
                    aOut as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (::core::mem::size_of::<u32_0>() as size_t)
                        .wrapping_mul((*p).nCol as size_t),
                );
                rc = (*pApi)
                    .xInstCount
                    .expect("non-null function pointer")(pFts, &raw mut nInst);
                i = 0 as ::core::ffi::c_int;
                while rc == SQLITE_OK && i < nInst {
                    let mut iPhrase_1: ::core::ffi::c_int = 0;
                    let mut iOff_0: ::core::ffi::c_int = 0;
                    let mut iCol_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut iNextPhrase: ::core::ffi::c_int = 0;
                    let mut iNextOff: ::core::ffi::c_int = 0;
                    let mut nSeq: u32_0 = 1 as u32_0;
                    let mut j: ::core::ffi::c_int = 0;
                    rc = (*pApi)
                        .xInst
                        .expect(
                            "non-null function pointer",
                        )(pFts, i, &raw mut iPhrase_1, &raw mut iCol_1, &raw mut iOff_0);
                    iNextPhrase = iPhrase_1 + 1 as ::core::ffi::c_int;
                    iNextOff = iOff_0
                        + (*pApi)
                            .xPhraseSize
                            .expect(
                                "non-null function pointer",
                            )(pFts, 0 as ::core::ffi::c_int);
                    j = i + 1 as ::core::ffi::c_int;
                    while rc == SQLITE_OK && j < nInst {
                        let mut ip: ::core::ffi::c_int = 0;
                        let mut ic: ::core::ffi::c_int = 0;
                        let mut io: ::core::ffi::c_int = 0;
                        rc = (*pApi)
                            .xInst
                            .expect(
                                "non-null function pointer",
                            )(pFts, j, &raw mut ip, &raw mut ic, &raw mut io);
                        if ic != iCol_1 || io > iNextOff {
                            break;
                        }
                        if ip == iNextPhrase && io == iNextOff {
                            nSeq = nSeq.wrapping_add(1);
                            iNextPhrase = ip + 1 as ::core::ffi::c_int;
                            iNextOff = io
                                + (*pApi)
                                    .xPhraseSize
                                    .expect("non-null function pointer")(pFts, ip);
                        }
                        j += 1;
                    }
                    if nSeq > *aOut.offset(iCol_1 as isize) {
                        *aOut.offset(iCol_1 as isize) = nSeq;
                    }
                    i += 1;
                }
            }
            _ => {}
        }
        return rc;
    }
}
unsafe extern "C" fn fts5MatchinfoNew(
    mut pApi: *const Fts5ExtensionApi,
    mut pFts: *mut Fts5Context,
    mut pCtx: *mut sqlite3_context,
    mut zArg: *const ::core::ffi::c_char,
) -> *mut Fts5MatchinfoCtx {
    unsafe {
        let mut p: *mut Fts5MatchinfoCtx = ::core::ptr::null_mut::<Fts5MatchinfoCtx>();
        let mut nCol: ::core::ffi::c_int = 0;
        let mut nPhrase: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut nInt: ::core::ffi::c_int = 0;
        let mut nByte: sqlite3_int64 = 0;
        let mut rc: ::core::ffi::c_int = 0;
        nCol = (*pApi).xColumnCount.expect("non-null function pointer")(pFts);
        nPhrase = (*pApi).xPhraseCount.expect("non-null function pointer")(pFts);
        nInt = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while *zArg.offset(i as isize) != 0 {
            let mut n: ::core::ffi::c_int = fts5MatchinfoFlagsize(
                nCol,
                nPhrase,
                *zArg.offset(i as isize),
            );
            if n < 0 as ::core::ffi::c_int {
                let mut zErr: *mut ::core::ffi::c_char = sqlite3_mprintf(
                    b"unrecognized matchinfo flag: %c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    *zArg.offset(i as isize) as ::core::ffi::c_int,
                );
                sqlite3_result_error(pCtx, zErr, -1 as ::core::ffi::c_int);
                sqlite3_free(zErr as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<Fts5MatchinfoCtx>();
            }
            nInt += n;
            i += 1;
        }
        nByte = (::core::mem::size_of::<Fts5MatchinfoCtx>() as usize)
            .wrapping_add(
                (::core::mem::size_of::<u32_0>() as usize).wrapping_mul(nInt as usize),
            )
            .wrapping_add((i + 1 as ::core::ffi::c_int) as usize) as sqlite3_int64;
        p = sqlite3_malloc64(nByte as sqlite3_uint64) as *mut Fts5MatchinfoCtx;
        if p.is_null() {
            sqlite3_result_error_nomem(pCtx);
            return ::core::ptr::null_mut::<Fts5MatchinfoCtx>();
        }
        memset(p as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, nByte as size_t);
        (*p).nCol = nCol;
        (*p).nPhrase = nPhrase;
        (*p).aRet = p.offset(1 as ::core::ffi::c_int as isize) as *mut Fts5MatchinfoCtx
            as *mut u32_0;
        (*p).nRet = nInt;
        (*p).zArg = (*p).aRet.offset(nInt as isize) as *mut u32_0
            as *mut ::core::ffi::c_char;
        memcpy(
            (*p).zArg as *mut ::core::ffi::c_void,
            zArg as *const ::core::ffi::c_void,
            i as size_t,
        );
        rc = fts5MatchinfoIter(
            pApi,
            pFts,
            p,
            Some(
                fts5MatchinfoGlobalCb
                    as unsafe extern "C" fn(
                        *const Fts5ExtensionApi,
                        *mut Fts5Context,
                        *mut Fts5MatchinfoCtx,
                        ::core::ffi::c_char,
                        *mut u32_0,
                    ) -> ::core::ffi::c_int,
            ),
        );
        if rc != SQLITE_OK {
            sqlite3_result_error_code(pCtx, rc);
            sqlite3_free(p as *mut ::core::ffi::c_void);
            p = ::core::ptr::null_mut::<Fts5MatchinfoCtx>();
        }
        return p;
    }
}
unsafe extern "C" fn fts5MatchinfoFunc(
    mut pApi: *const Fts5ExtensionApi,
    mut pFts: *mut Fts5Context,
    mut pCtx: *mut sqlite3_context,
    mut nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zArg: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut p: *mut Fts5MatchinfoCtx = ::core::ptr::null_mut::<Fts5MatchinfoCtx>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if nVal > 0 as ::core::ffi::c_int {
            zArg = sqlite3_value_text(*apVal.offset(0 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char;
        } else {
            zArg = b"pcx\0".as_ptr() as *const ::core::ffi::c_char;
        }
        p = (*pApi)
            .xGetAuxdata
            .expect("non-null function pointer")(pFts, 0 as ::core::ffi::c_int)
            as *mut Fts5MatchinfoCtx;
        if p.is_null() || sqlite3_stricmp(zArg, (*p).zArg) != 0 {
            p = fts5MatchinfoNew(pApi, pFts, pCtx, zArg);
            if p.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                rc = (*pApi)
                    .xSetAuxdata
                    .expect(
                        "non-null function pointer",
                    )(
                    pFts,
                    p as *mut ::core::ffi::c_void,
                    Some(
                        sqlite3_free
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                );
            }
        }
        if rc == SQLITE_OK {
            rc = fts5MatchinfoIter(
                pApi,
                pFts,
                p,
                Some(
                    fts5MatchinfoLocalCb
                        as unsafe extern "C" fn(
                            *const Fts5ExtensionApi,
                            *mut Fts5Context,
                            *mut Fts5MatchinfoCtx,
                            ::core::ffi::c_char,
                            *mut u32_0,
                        ) -> ::core::ffi::c_int,
                ),
            );
        }
        if rc != SQLITE_OK {
            sqlite3_result_error_code(pCtx, rc);
        } else {
            let mut nByte: ::core::ffi::c_int = ((*p).nRet as usize)
                .wrapping_mul(::core::mem::size_of::<u32_0>() as usize)
                as ::core::ffi::c_int;
            sqlite3_result_blob(
                pCtx,
                (*p).aRet as *mut ::core::ffi::c_void,
                nByte,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3Fts5TestRegisterMatchinfoAPI(
    mut pApi: *mut fts5_api,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if pApi.is_null() || (*pApi).iVersion < 2 as ::core::ffi::c_int {
            return SQLITE_ERROR;
        }
        rc = (*pApi)
            .xCreateFunction
            .expect(
                "non-null function pointer",
            )(
            pApi,
            b"matchinfo\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                fts5MatchinfoFunc
                    as unsafe extern "C" fn(
                        *const Fts5ExtensionApi,
                        *mut Fts5Context,
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
        );
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3Fts5TestRegisterMatchinfo(
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pApi: *mut fts5_api = ::core::ptr::null_mut::<fts5_api>();
        rc = fts5_api_from_db(db, &raw mut pApi);
        if rc != SQLITE_OK {
            return rc;
        }
        return sqlite3Fts5TestRegisterMatchinfoAPI(pApi);
    }
}
