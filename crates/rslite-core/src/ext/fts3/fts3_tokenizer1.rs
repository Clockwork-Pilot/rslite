pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::SizeT;

pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module;
pub use crate::src::headers::sqlite3_h::SQLITE_DONE;
pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;
pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SqliteUint64;
pub use crate::src::headers::sqlite3_h::Sqlite3Uint64;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3_malloc;
pub use crate::src::src::malloc::sqlite3_realloc64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_tokenizer {
    pub base: crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
    pub delim: [::core::ffi::c_char; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_tokenizer_cursor {
    pub base: crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
    pub pInput: *const ::core::ffi::c_char,
    pub nBytes: ::core::ffi::c_int,
    pub iOffset: ::core::ffi::c_int,
    pub iToken: ::core::ffi::c_int,
    pub pToken: *mut ::core::ffi::c_char,
    pub nTokenAllocated: ::core::ffi::c_int,
}

unsafe extern "C" fn simpleDelim(
    t: *mut simple_tokenizer,
    c: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    ((c as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int
        && (*t).delim[c as usize] as ::core::ffi::c_int != 0) as ::core::ffi::c_int
}

unsafe extern "C" fn fts3_isalnum(x: ::core::ffi::c_int) -> ::core::ffi::c_int {
    (x >= '0' as i32 && x <= '9' as i32
        || x >= 'A' as i32 && x <= 'Z' as i32
        || x >= 'a' as i32 && x <= 'z' as i32) as ::core::ffi::c_int
}

unsafe extern "C" fn simpleCreate(
    argc: ::core::ffi::c_int,
    argv: *const *const ::core::ffi::c_char,
    ppTokenizer: *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
) -> ::core::ffi::c_int {
    
    let t: *mut simple_tokenizer = crate::src::src::malloc::sqlite3_malloc(
        ::core::mem::size_of::<simple_tokenizer>() as ::core::ffi::c_int
    ) as *mut simple_tokenizer;
    if t.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<simple_tokenizer>() as crate::__stddef_size_t_h::SizeT,
    );
    if argc > 1 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int;
        let n: ::core::ffi::c_int =
            ::libc::strlen(*argv.offset(1_isize)) as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < n {
            let ch: ::core::ffi::c_uchar =
                *(*argv.offset(1_isize)).offset(i as isize) as ::core::ffi::c_uchar;
            if ch as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
                crate::src::src::malloc::sqlite3_free(t as *mut ::core::ffi::c_void);
                return crate::src::headers::sqlite3_h::SQLITE_ERROR;
            }
            (*t).delim[ch as usize] = 1 as ::core::ffi::c_char;
            i += 1;
        }
    } else {
        let mut i_0: ::core::ffi::c_int;
        i_0 = 1 as ::core::ffi::c_int;
        while i_0 < 0x80 as ::core::ffi::c_int {
            (*t).delim[i_0 as usize] = (if fts3_isalnum(i_0) == 0 {
                -(1 as ::core::ffi::c_int)
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_char;
            i_0 += 1;
        }
    }
    *ppTokenizer = &raw mut (*t).base;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn simpleDestroy(
    pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
) -> ::core::ffi::c_int {
    crate::src::src::malloc::sqlite3_free(pTokenizer as *mut ::core::ffi::c_void);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn simpleOpen(
    mut _pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
    pInput: *const ::core::ffi::c_char,
    nBytes: ::core::ffi::c_int,
    ppCursor: *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
) -> ::core::ffi::c_int {
    
    let c: *mut simple_tokenizer_cursor = crate::src::src::malloc::sqlite3_malloc(
        ::core::mem::size_of::<simple_tokenizer_cursor>() as ::core::ffi::c_int
    ) as *mut simple_tokenizer_cursor;
    if c.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    (*c).pInput = pInput;
    if pInput.is_null() {
        (*c).nBytes = 0 as ::core::ffi::c_int;
    } else if nBytes < 0 as ::core::ffi::c_int {
        (*c).nBytes = ::libc::strlen(pInput) as ::core::ffi::c_int;
    } else {
        (*c).nBytes = nBytes;
    }
    (*c).iOffset = 0 as ::core::ffi::c_int;
    (*c).iToken = 0 as ::core::ffi::c_int;
    (*c).pToken = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*c).nTokenAllocated = 0 as ::core::ffi::c_int;
    *ppCursor = &raw mut (*c).base;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn simpleClose(
    pCursor: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
) -> ::core::ffi::c_int {
    let c: *mut simple_tokenizer_cursor = pCursor as *mut simple_tokenizer_cursor;
    crate::src::src::malloc::sqlite3_free((*c).pToken as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(c as *mut ::core::ffi::c_void);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn simpleNext(
    pCursor: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
    ppToken: *mut *const ::core::ffi::c_char,
    pnBytes: *mut ::core::ffi::c_int,
    piStartOffset: *mut ::core::ffi::c_int,
    piEndOffset: *mut ::core::ffi::c_int,
    piPosition: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let c: *mut simple_tokenizer_cursor = pCursor as *mut simple_tokenizer_cursor;
    let t: *mut simple_tokenizer = (*pCursor).pTokenizer as *mut simple_tokenizer;
    let __c_ref = { &mut *c };
    let p: *mut ::core::ffi::c_uchar = __c_ref.pInput as *mut ::core::ffi::c_uchar;
    while __c_ref.iOffset < __c_ref.nBytes {
        
        while __c_ref.iOffset < __c_ref.nBytes
            && simpleDelim(t, *p.offset(__c_ref.iOffset as isize)) != 0
        {
            __c_ref.iOffset += 1;
        }
        let iStartOffset: ::core::ffi::c_int = __c_ref.iOffset;
        while __c_ref.iOffset < __c_ref.nBytes
            && simpleDelim(t, *p.offset(__c_ref.iOffset as isize)) == 0
        {
            __c_ref.iOffset += 1;
        }
        if __c_ref.iOffset > iStartOffset {
            let mut i: ::core::ffi::c_int;
            let n: ::core::ffi::c_int = __c_ref.iOffset - iStartOffset;
            if n > __c_ref.nTokenAllocated {
                
                __c_ref.nTokenAllocated = n + 20 as ::core::ffi::c_int;
                let pNew: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_realloc64(
                    __c_ref.pToken as *mut ::core::ffi::c_void,
                    __c_ref.nTokenAllocated as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                ) as *mut ::core::ffi::c_char;
                if pNew.is_null() {
                    return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                }
                __c_ref.pToken = pNew;
            }
            i = 0 as ::core::ffi::c_int;
            while i < n {
                let ch: ::core::ffi::c_uchar = *p.offset((iStartOffset + i) as isize);
                *__c_ref.pToken.offset(i as isize) = (if ch as ::core::ffi::c_int >= 'A' as i32
                    && ch as ::core::ffi::c_int <= 'Z' as i32
                {
                    ch as ::core::ffi::c_int - 'A' as i32 + 'a' as i32
                } else {
                    ch as ::core::ffi::c_int
                }) as ::core::ffi::c_char;
                i += 1;
            }
            *ppToken = __c_ref.pToken;
            *pnBytes = n;
            *piStartOffset = iStartOffset;
            *piEndOffset = __c_ref.iOffset;
            let fresh0 = __c_ref.iToken;
            __c_ref.iToken += 1;
            *piPosition = fresh0;
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_DONE
}

static mut simpleTokenizerModule: crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module = {
    crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module {
        iVersion: 0 as ::core::ffi::c_int,
        xCreate: Some(
            simpleCreate
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const *const ::core::ffi::c_char,
                    *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
                ) -> ::core::ffi::c_int,
        ),
        xDestroy: Some(
            simpleDestroy
                as unsafe extern "C" fn(
                    *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
                ) -> ::core::ffi::c_int,
        ),
        xOpen: Some(
            simpleOpen
                as unsafe extern "C" fn(
                    *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            simpleClose
                as unsafe extern "C" fn(
                    *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            simpleNext
                as unsafe extern "C" fn(
                    *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
                    *mut *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xLanguageid: None,
    }
};

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3Fts3SimpleTokenizerModule(
    ppModule: *mut *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module,
) {
    *ppModule = &raw const simpleTokenizerModule;
}
