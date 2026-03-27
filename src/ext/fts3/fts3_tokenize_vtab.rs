






pub use crate::__stddef_size_t_h::size_t;




pub use crate::src::ext::fts3::fts3_hash::Fts3Hash;pub use crate::src::ext::fts3::fts3_hash::Fts3HashElem;pub use crate::src::ext::fts3::fts3_hash::_fts3ht;pub use crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashFind;pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer;pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor;pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module;pub use crate::sqliteInt_h::sqlite3;pub use crate::vdbeInt_h::sqlite3_context;pub use crate::src::src::vtab::sqlite3_create_module_v2;pub use crate::src::src::vtab::sqlite3_declare_vtab;pub use crate::sqlite3_h::sqlite3_destructor_type;pub use crate::src::src::malloc::sqlite3_free;pub use crate::sqlite3_h::sqlite3_index_constraint;pub use crate::sqlite3_h::sqlite3_index_constraint_usage;pub use crate::sqlite3_h::sqlite3_index_info;pub use crate::sqlite3_h::sqlite3_index_orderby;pub use crate::sqlite3_h::sqlite3_int64;pub use crate::src::src::malloc::sqlite3_malloc;pub use crate::src::src::malloc::sqlite3_malloc64;pub use crate::sqlite3_h::sqlite3_module;pub use crate::src::src::vdbeapi::sqlite3_result_int;pub use crate::src::src::vdbeapi::sqlite3_result_text;pub use crate::sqlite3_h::sqlite3_uint64;pub use crate::vdbeInt_h::sqlite3_value;pub use crate::src::src::vdbeapi::sqlite3_value_bytes;pub use crate::src::src::vdbeapi::sqlite3_value_text;pub use crate::sqlite3_h::sqlite3_vtab;pub use crate::sqlite3_h::sqlite3_vtab_cursor;pub use crate::sqlite3_h::sqlite_int64;pub use crate::sqlite3_h::sqlite_uint64;pub use crate::sqlite3_h::SQLITE_DONE;pub use crate::sqlite3_h::SQLITE_ERROR;pub use crate::sqlite3_h::SQLITE_INDEX_CONSTRAINT_EQ;pub use crate::sqlite3_h::SQLITE_NOMEM;pub use crate::sqlite3_h::SQLITE_OK;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct Fts3tokCursor {
    pub base: crate::sqlite3_h::sqlite3_vtab_cursor,
    pub zInput: *mut ::core::ffi::c_char,
    pub pCsr: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
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
    pub base: crate::sqlite3_h::sqlite3_vtab,
    pub pMod: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module,
    pub pTok: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
}

unsafe extern "C" fn fts3tokQueryTokenizer(
    mut pHash: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
    mut zName: *const ::core::ffi::c_char,
    mut pp: *mut *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module>();
    let mut nName: ::core::ffi::c_int = ::libc::strlen(zName) as ::core::ffi::c_int;
    p = crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashFind(
        
        pHash as *const crate::src::ext::fts3::fts3_hash::Fts3Hash,
        zName as *const ::core::ffi::c_void,
        nName + 1 as ::core::ffi::c_int,
    ) as *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module;
    if p.is_null() {
        crate::src::ext::fts3::fts3::sqlite3Fts3ErrMsg(
            pzErr,
            b"unknown tokenizer: %s\0" as *const u8 as *const ::core::ffi::c_char,
            zName,
        );
        return crate::sqlite3_h::SQLITE_ERROR;
    }
    *pp = p;
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn fts3tokDequoteArray(
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut pazDequote: *mut *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::sqlite3_h::SQLITE_OK;
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
                ::libc::strlen(*argv.offset(i as isize)).wrapping_add(1 as crate::__stddef_size_t_h::size_t) as ::core::ffi::c_int;
            i += 1;
        }
        azDequote = crate::src::src::malloc::sqlite3_malloc64(
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                .wrapping_mul(argc as usize)
                .wrapping_add(nByte as usize) as crate::sqlite3_h::sqlite3_uint64,
        ) as *mut *mut ::core::ffi::c_char;
        *pazDequote = azDequote;
        if azDequote.is_null() {
            rc = crate::sqlite3_h::SQLITE_NOMEM;
        } else {
            let mut pSpace: *mut ::core::ffi::c_char = azDequote.offset(argc as isize)
                as *mut *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            i = 0 as ::core::ffi::c_int;
            while i < argc {
                let mut n: ::core::ffi::c_int =
                    ::libc::strlen(*argv.offset(i as isize)) as ::core::ffi::c_int;
                let ref mut fresh0 = *azDequote.offset(i as isize);
                *fresh0 = pSpace;
                ::libc::memcpy(
                    pSpace as *mut ::core::ffi::c_void,
                    *argv.offset(i as isize) as *const ::core::ffi::c_void,
                    (n + 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t,
                );
                crate::src::ext::fts3::fts3::sqlite3Fts3Dequote(pSpace);
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
    mut db: *mut crate::sqliteInt_h::sqlite3,
    mut pHash: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut crate::sqlite3_h::sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pTab: *mut Fts3tokTable = ::core::ptr::null_mut::<Fts3tokTable>();
    let mut pMod: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module = ::core::ptr::null::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module>();
    let mut pTok: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer>();
    let mut rc: ::core::ffi::c_int = 0;
    let mut azDequote: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut nDequote: ::core::ffi::c_int = 0;
    rc = crate::src::src::vtab::sqlite3_declare_vtab(db, FTS3_TOK_SCHEMA.as_ptr());
    if rc != crate::sqlite3_h::SQLITE_OK {
        return rc;
    }
    nDequote = argc - 3 as ::core::ffi::c_int;
    rc = fts3tokDequoteArray(
        nDequote,
        argv.offset(3 as isize) as *const *const ::core::ffi::c_char,
        &raw mut azDequote,
    );
    if rc == crate::sqlite3_h::SQLITE_OK {
        let mut zModule: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if nDequote < 1 as ::core::ffi::c_int {
            zModule = b"simple\0" as *const u8 as *const ::core::ffi::c_char;
        } else {
            zModule = *azDequote.offset(0 as isize);
        }
        rc = fts3tokQueryTokenizer(pHash as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash, zModule, &raw mut pMod, pzErr);
    }
    if rc == crate::sqlite3_h::SQLITE_OK {
        let mut azArg: *const *const ::core::ffi::c_char =
            ::core::ptr::null::<*const ::core::ffi::c_char>();
        if nDequote > 1 as ::core::ffi::c_int {
            azArg = azDequote.offset(1 as isize)
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
    if rc == crate::sqlite3_h::SQLITE_OK {
        pTab = crate::src::src::malloc::sqlite3_malloc(::core::mem::size_of::<Fts3tokTable>() as ::core::ffi::c_int)
            as *mut Fts3tokTable;
        if pTab.is_null() {
            rc = crate::sqlite3_h::SQLITE_NOMEM;
        }
    }
    if rc == crate::sqlite3_h::SQLITE_OK {
        ::libc::memset(
            pTab as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Fts3tokTable>() as crate::__stddef_size_t_h::size_t,
        );
        let __pTab_ref = unsafe { &mut *pTab };
        __pTab_ref.pMod = pMod;
        __pTab_ref.pTok = pTok;
        *ppVtab = &raw mut __pTab_ref.base;
    } else if !pTok.is_null() {
        (*pMod).xDestroy.expect("non-null function pointer")(pTok);
    }
    crate::src::src::malloc::sqlite3_free(azDequote as *mut ::core::ffi::c_void);
    return rc;
}

unsafe extern "C" fn fts3tokDisconnectMethod(mut pVtab: *mut crate::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int {
    let mut pTab: *mut Fts3tokTable = pVtab as *mut Fts3tokTable;
    (*(*pTab).pMod).xDestroy.expect("non-null function pointer")((*pTab).pTok);
    crate::src::src::malloc::sqlite3_free(pTab as *mut ::core::ffi::c_void);
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn fts3tokBestIndexMethod(
    mut _pVTab: *mut crate::sqlite3_h::sqlite3_vtab,
    mut pInfo: *mut crate::sqlite3_h::sqlite3_index_info,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pInfo).nConstraint {
        let __pInfo_ref = unsafe { &mut *pInfo };
        if (*__pInfo_ref.aConstraint.offset(i as isize)).usable as ::core::ffi::c_int != 0
            && (*__pInfo_ref.aConstraint.offset(i as isize)).iColumn == 0 as ::core::ffi::c_int
            && (*__pInfo_ref.aConstraint.offset(i as isize)).op as ::core::ffi::c_int
                == crate::sqlite3_h::SQLITE_INDEX_CONSTRAINT_EQ
        {
            __pInfo_ref.idxNum = 1 as ::core::ffi::c_int;
            (*__pInfo_ref.aConstraintUsage.offset(i as isize)).argvIndex = 1 as ::core::ffi::c_int;
            (*__pInfo_ref.aConstraintUsage.offset(i as isize)).omit = 1 as ::core::ffi::c_uchar;
            __pInfo_ref.estimatedCost = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            return crate::sqlite3_h::SQLITE_OK;
        }
        i += 1;
    }
    (*pInfo).idxNum = 0 as ::core::ffi::c_int;
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn fts3tokOpenMethod(
    mut _pVTab: *mut crate::sqlite3_h::sqlite3_vtab,
    mut ppCsr: *mut *mut crate::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3tokCursor = ::core::ptr::null_mut::<Fts3tokCursor>();
    pCsr = crate::src::src::malloc::sqlite3_malloc(::core::mem::size_of::<Fts3tokCursor>() as ::core::ffi::c_int)
        as *mut Fts3tokCursor;
    if pCsr.is_null() {
        return crate::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        pCsr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Fts3tokCursor>() as crate::__stddef_size_t_h::size_t,
    );
    *ppCsr = pCsr as *mut crate::sqlite3_h::sqlite3_vtab_cursor;
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn fts3tokResetCursor(mut pCsr: *mut Fts3tokCursor) {
    let __pCsr_ref = unsafe { &mut *pCsr };
    if !__pCsr_ref.pCsr.is_null() {
        let mut pTab: *mut Fts3tokTable = __pCsr_ref.base.pVtab as *mut Fts3tokTable;
        (*(*pTab).pMod).xClose.expect("non-null function pointer")(__pCsr_ref.pCsr);
        __pCsr_ref.pCsr = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor>();
    }
    crate::src::src::malloc::sqlite3_free(__pCsr_ref.zInput as *mut ::core::ffi::c_void);
    __pCsr_ref.zInput = ::core::ptr::null_mut::<::core::ffi::c_char>();
    __pCsr_ref.zToken = ::core::ptr::null::<::core::ffi::c_char>();
    __pCsr_ref.nToken = 0 as ::core::ffi::c_int;
    __pCsr_ref.iStart = 0 as ::core::ffi::c_int;
    __pCsr_ref.iEnd = 0 as ::core::ffi::c_int;
    __pCsr_ref.iPos = 0 as ::core::ffi::c_int;
    __pCsr_ref.iRowid = 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn fts3tokCloseMethod(
    mut pCursor: *mut crate::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    fts3tokResetCursor(pCsr);
    crate::src::src::malloc::sqlite3_free(pCsr as *mut ::core::ffi::c_void);
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn fts3tokNextMethod(
    mut pCursor: *mut crate::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    let mut pTab: *mut Fts3tokTable = (*pCursor).pVtab as *mut Fts3tokTable;
    let mut rc: ::core::ffi::c_int = 0;
    let __pCsr_ref = unsafe { &mut *pCsr };
    __pCsr_ref.iRowid += 1;
    rc = (*(*pTab).pMod).xNext.expect("non-null function pointer")(
        __pCsr_ref.pCsr,
        &raw mut __pCsr_ref.zToken,
        &raw mut __pCsr_ref.nToken,
        &raw mut __pCsr_ref.iStart,
        &raw mut __pCsr_ref.iEnd,
        &raw mut __pCsr_ref.iPos,
    );
    if rc != crate::sqlite3_h::SQLITE_OK {
        fts3tokResetCursor(pCsr);
        if rc == crate::sqlite3_h::SQLITE_DONE {
            rc = crate::sqlite3_h::SQLITE_OK;
        }
    }
    return rc;
}

unsafe extern "C" fn fts3tokFilterMethod(
    mut pCursor: *mut crate::sqlite3_h::sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut _idxStr: *const ::core::ffi::c_char,
    mut _nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut crate::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::sqlite3_h::SQLITE_ERROR;
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    let mut pTab: *mut Fts3tokTable = (*pCursor).pVtab as *mut Fts3tokTable;
    fts3tokResetCursor(pCsr);
    if idxNum == 1 as ::core::ffi::c_int {
        let mut zByte: *const ::core::ffi::c_char =
            crate::src::src::vdbeapi::sqlite3_value_text(*apVal.offset(0 as isize))
                as *const ::core::ffi::c_char;
        let mut nByte: crate::sqlite3_h::sqlite3_int64 =
            crate::src::src::vdbeapi::sqlite3_value_bytes(*apVal.offset(0 as isize)) as crate::sqlite3_h::sqlite3_int64;
        (*pCsr).zInput = crate::src::src::malloc::sqlite3_malloc64((nByte + 1 as crate::sqlite3_h::sqlite3_int64) as crate::sqlite3_h::sqlite3_uint64)
            as *mut ::core::ffi::c_char;
        if (*pCsr).zInput.is_null() {
            rc = crate::sqlite3_h::SQLITE_NOMEM;
        } else {
            let __pCsr_ref = unsafe { &mut *pCsr };
            if nByte > 0 as crate::sqlite3_h::sqlite3_int64 {
                ::libc::memcpy(
                    __pCsr_ref.zInput as *mut ::core::ffi::c_void,
                    zByte as *const ::core::ffi::c_void,
                    nByte as crate::__stddef_size_t_h::size_t,
                );
            }
            *__pCsr_ref.zInput.offset(nByte as isize) = 0 as ::core::ffi::c_char;
            rc = (*(*pTab).pMod).xOpen.expect("non-null function pointer")(
                (*pTab).pTok,
                __pCsr_ref.zInput,
                nByte as ::core::ffi::c_int,
                &raw mut __pCsr_ref.pCsr,
            );
            if rc == crate::sqlite3_h::SQLITE_OK {
                (*__pCsr_ref.pCsr).pTokenizer = (*pTab).pTok;
            }
        }
    }
    if rc != crate::sqlite3_h::SQLITE_OK {
        return rc;
    }
    return fts3tokNextMethod(pCursor);
}

unsafe extern "C" fn fts3tokEofMethod(mut pCursor: *mut crate::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    return ((*pCsr).zToken == ::core::ptr::null::<::core::ffi::c_char>()) as ::core::ffi::c_int;
}

unsafe extern "C" fn fts3tokColumnMethod(
    mut pCursor: *mut crate::sqlite3_h::sqlite3_vtab_cursor,
    mut pCtx: *mut crate::vdbeInt_h::sqlite3_context,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    match iCol {
        0 => {
            crate::src::src::vdbeapi::sqlite3_result_text(
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
            crate::src::src::vdbeapi::sqlite3_result_text(
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
            crate::src::src::vdbeapi::sqlite3_result_int(pCtx, (*pCsr).iStart);
        }
        3 => {
            crate::src::src::vdbeapi::sqlite3_result_int(pCtx, (*pCsr).iEnd);
        }
        _ => {
            crate::src::src::vdbeapi::sqlite3_result_int(pCtx, (*pCsr).iPos);
        }
    }
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn fts3tokRowidMethod(
    mut pCursor: *mut crate::sqlite3_h::sqlite3_vtab_cursor,
    mut pRowid: *mut crate::sqlite3_h::sqlite_int64,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3tokCursor = pCursor as *mut Fts3tokCursor;
    *pRowid = (*pCsr).iRowid as crate::sqlite3_h::sqlite3_int64 as crate::sqlite3_h::sqlite_int64;
    return crate::sqlite3_h::SQLITE_OK;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3InitTok(
    mut db: *mut crate::sqliteInt_h::sqlite3,
    mut pHash: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
    mut xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    static mut fts3tok_module: crate::sqlite3_h::sqlite3_module = unsafe {
        crate::sqlite3_h::sqlite3_module {
    iVersion:  0 as ::core::ffi::c_int,
    xCreate:  Some(
                fts3tokConnectMethod
                    as unsafe extern "C" fn(
                        *mut crate::sqliteInt_h::sqlite3,
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const *const ::core::ffi::c_char,
                        *mut *mut crate::sqlite3_h::sqlite3_vtab,
                        *mut *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
    xConnect:  Some(
                fts3tokConnectMethod
                    as unsafe extern "C" fn(
                        *mut crate::sqliteInt_h::sqlite3,
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const *const ::core::ffi::c_char,
                        *mut *mut crate::sqlite3_h::sqlite3_vtab,
                        *mut *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
    xBestIndex:  Some(
                fts3tokBestIndexMethod
                    as unsafe extern "C" fn(
                        *mut crate::sqlite3_h::sqlite3_vtab,
                        *mut crate::sqlite3_h::sqlite3_index_info,
                    ) -> ::core::ffi::c_int,
            ),
    xDisconnect:  Some(
                fts3tokDisconnectMethod
                    as unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int,
            ),
    xDestroy:  Some(
                fts3tokDisconnectMethod
                    as unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int,
            ),
    xOpen:  Some(
                fts3tokOpenMethod
                    as unsafe extern "C" fn(
                        *mut crate::sqlite3_h::sqlite3_vtab,
                        *mut *mut crate::sqlite3_h::sqlite3_vtab_cursor,
                    ) -> ::core::ffi::c_int,
            ),
    xClose:  Some(
                fts3tokCloseMethod
                    as unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
    xFilter:  Some(
                fts3tokFilterMethod
                    as unsafe extern "C" fn(
                        *mut crate::sqlite3_h::sqlite3_vtab_cursor,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut *mut crate::vdbeInt_h::sqlite3_value,
                    ) -> ::core::ffi::c_int,
            ),
    xNext:  Some(
                fts3tokNextMethod
                    as unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
    xEof:  Some(
                fts3tokEofMethod
                    as unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
    xColumn:  Some(
                fts3tokColumnMethod
                    as unsafe extern "C" fn(
                        *mut crate::sqlite3_h::sqlite3_vtab_cursor,
                        *mut crate::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
    xRowid:  Some(
                fts3tokRowidMethod
                    as unsafe extern "C" fn(
                        *mut crate::sqlite3_h::sqlite3_vtab_cursor,
                        *mut crate::sqlite3_h::sqlite_int64,
                    ) -> ::core::ffi::c_int,
            ),
    xUpdate:  None,
    xBegin:  None,
    xSync:  None,
    xCommit:  None,
    xRollback:  None,
    xFindFunction:  None,
    xRename:  None,
    xSavepoint:  None,
    xRelease:  None,
    xRollbackTo:  None,
    xShadowName:  None,
    xIntegrity:  None,
}
    };
    let mut rc: ::core::ffi::c_int = 0;
    rc = crate::src::src::vtab::sqlite3_create_module_v2(
        db,
        b"fts3tokenize\0" as *const u8 as *const ::core::ffi::c_char,
        
        &raw const fts3tok_module as *const _ as
    *const crate::sqlite3_h::sqlite3_module,
        pHash as *mut ::core::ffi::c_void,
        xDestroy,
    );
    return rc;
}
