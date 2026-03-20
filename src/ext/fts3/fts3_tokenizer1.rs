extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite3_uint64 = sqlite_uint64;
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
pub struct simple_tokenizer {
    pub base: sqlite3_tokenizer,
    pub delim: [::core::ffi::c_char; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_tokenizer_cursor {
    pub base: sqlite3_tokenizer_cursor,
    pub pInput: *const ::core::ffi::c_char,
    pub nBytes: ::core::ffi::c_int,
    pub iOffset: ::core::ffi::c_int,
    pub iToken: ::core::ffi::c_int,
    pub pToken: *mut ::core::ffi::c_char,
    pub nTokenAllocated: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
unsafe extern "C" fn simpleDelim(
    mut t: *mut simple_tokenizer,
    mut c: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    return ((c as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int
        && (*t).delim[c as usize] as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn fts3_isalnum(mut x: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (x >= '0' as i32 && x <= '9' as i32
        || x >= 'A' as i32 && x <= 'Z' as i32
        || x >= 'a' as i32 && x <= 'z' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn simpleCreate(
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppTokenizer: *mut *mut sqlite3_tokenizer,
) -> ::core::ffi::c_int {
    let mut t: *mut simple_tokenizer = ::core::ptr::null_mut::<simple_tokenizer>();
    t = sqlite3_malloc(::core::mem::size_of::<simple_tokenizer>() as ::core::ffi::c_int)
        as *mut simple_tokenizer;
    if t.is_null() {
        return SQLITE_NOMEM;
    }
    memset(
        t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<simple_tokenizer>() as size_t,
    );
    if argc > 1 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int =
            strlen(*argv.offset(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < n {
            let mut ch: ::core::ffi::c_uchar = *(*argv.offset(1 as ::core::ffi::c_int as isize))
                .offset(i as isize)
                as ::core::ffi::c_uchar;
            if ch as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int {
                sqlite3_free(t as *mut ::core::ffi::c_void);
                return SQLITE_ERROR;
            }
            (*t).delim[ch as usize] = 1 as ::core::ffi::c_char;
            i += 1;
        }
    } else {
        let mut i_0: ::core::ffi::c_int = 0;
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
    return SQLITE_OK;
}
unsafe extern "C" fn simpleDestroy(mut pTokenizer: *mut sqlite3_tokenizer) -> ::core::ffi::c_int {
    sqlite3_free(pTokenizer as *mut ::core::ffi::c_void);
    return SQLITE_OK;
}
unsafe extern "C" fn simpleOpen(
    mut _pTokenizer: *mut sqlite3_tokenizer,
    mut pInput: *const ::core::ffi::c_char,
    mut nBytes: ::core::ffi::c_int,
    mut ppCursor: *mut *mut sqlite3_tokenizer_cursor,
) -> ::core::ffi::c_int {
    let mut c: *mut simple_tokenizer_cursor = ::core::ptr::null_mut::<simple_tokenizer_cursor>();
    c = sqlite3_malloc(::core::mem::size_of::<simple_tokenizer_cursor>() as ::core::ffi::c_int)
        as *mut simple_tokenizer_cursor;
    if c.is_null() {
        return SQLITE_NOMEM;
    }
    (*c).pInput = pInput;
    if pInput.is_null() {
        (*c).nBytes = 0 as ::core::ffi::c_int;
    } else if nBytes < 0 as ::core::ffi::c_int {
        (*c).nBytes = strlen(pInput) as ::core::ffi::c_int;
    } else {
        (*c).nBytes = nBytes;
    }
    (*c).iOffset = 0 as ::core::ffi::c_int;
    (*c).iToken = 0 as ::core::ffi::c_int;
    (*c).pToken = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*c).nTokenAllocated = 0 as ::core::ffi::c_int;
    *ppCursor = &raw mut (*c).base;
    return SQLITE_OK;
}
unsafe extern "C" fn simpleClose(mut pCursor: *mut sqlite3_tokenizer_cursor) -> ::core::ffi::c_int {
    let mut c: *mut simple_tokenizer_cursor = pCursor as *mut simple_tokenizer_cursor;
    sqlite3_free((*c).pToken as *mut ::core::ffi::c_void);
    sqlite3_free(c as *mut ::core::ffi::c_void);
    return SQLITE_OK;
}
unsafe extern "C" fn simpleNext(
    mut pCursor: *mut sqlite3_tokenizer_cursor,
    mut ppToken: *mut *const ::core::ffi::c_char,
    mut pnBytes: *mut ::core::ffi::c_int,
    mut piStartOffset: *mut ::core::ffi::c_int,
    mut piEndOffset: *mut ::core::ffi::c_int,
    mut piPosition: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: *mut simple_tokenizer_cursor = pCursor as *mut simple_tokenizer_cursor;
    let mut t: *mut simple_tokenizer = (*pCursor).pTokenizer as *mut simple_tokenizer;
    let mut p: *mut ::core::ffi::c_uchar = (*c).pInput as *mut ::core::ffi::c_uchar;
    while (*c).iOffset < (*c).nBytes {
        let mut iStartOffset: ::core::ffi::c_int = 0;
        while (*c).iOffset < (*c).nBytes && simpleDelim(t, *p.offset((*c).iOffset as isize)) != 0 {
            (*c).iOffset += 1;
        }
        iStartOffset = (*c).iOffset;
        while (*c).iOffset < (*c).nBytes && simpleDelim(t, *p.offset((*c).iOffset as isize)) == 0 {
            (*c).iOffset += 1;
        }
        if (*c).iOffset > iStartOffset {
            let mut i: ::core::ffi::c_int = 0;
            let mut n: ::core::ffi::c_int = (*c).iOffset - iStartOffset;
            if n > (*c).nTokenAllocated {
                let mut pNew: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                (*c).nTokenAllocated = n + 20 as ::core::ffi::c_int;
                pNew = sqlite3_realloc64(
                    (*c).pToken as *mut ::core::ffi::c_void,
                    (*c).nTokenAllocated as sqlite3_uint64,
                ) as *mut ::core::ffi::c_char;
                if pNew.is_null() {
                    return SQLITE_NOMEM;
                }
                (*c).pToken = pNew;
            }
            i = 0 as ::core::ffi::c_int;
            while i < n {
                let mut ch: ::core::ffi::c_uchar = *p.offset((iStartOffset + i) as isize);
                *(*c).pToken.offset(i as isize) = (if ch as ::core::ffi::c_int >= 'A' as i32
                    && ch as ::core::ffi::c_int <= 'Z' as i32
                {
                    ch as ::core::ffi::c_int - 'A' as i32 + 'a' as i32
                } else {
                    ch as ::core::ffi::c_int
                }) as ::core::ffi::c_char;
                i += 1;
            }
            *ppToken = (*c).pToken;
            *pnBytes = n;
            *piStartOffset = iStartOffset;
            *piEndOffset = (*c).iOffset;
            let fresh0 = (*c).iToken;
            (*c).iToken = (*c).iToken + 1;
            *piPosition = fresh0;
            return SQLITE_OK;
        }
    }
    return SQLITE_DONE;
}
static mut simpleTokenizerModule: sqlite3_tokenizer_module = unsafe {
    sqlite3_tokenizer_module {
        iVersion: 0 as ::core::ffi::c_int,
        xCreate: Some(
            simpleCreate
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const *const ::core::ffi::c_char,
                    *mut *mut sqlite3_tokenizer,
                ) -> ::core::ffi::c_int,
        ),
        xDestroy: Some(
            simpleDestroy as unsafe extern "C" fn(*mut sqlite3_tokenizer) -> ::core::ffi::c_int,
        ),
        xOpen: Some(
            simpleOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_tokenizer,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_tokenizer_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            simpleClose
                as unsafe extern "C" fn(*mut sqlite3_tokenizer_cursor) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            simpleNext
                as unsafe extern "C" fn(
                    *mut sqlite3_tokenizer_cursor,
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
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3SimpleTokenizerModule(
    mut ppModule: *mut *const sqlite3_tokenizer_module,
) {
    *ppModule = &raw const simpleTokenizerModule;
}
