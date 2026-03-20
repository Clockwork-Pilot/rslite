extern "C" {
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
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3FtsUnicodeFold(_: ::core::ffi::c_int, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3FtsUnicodeIsalnum(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3FtsUnicodeIsdiacritic(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub type u8_0 = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unicode_tokenizer {
    pub base: sqlite3_tokenizer,
    pub eRemoveDiacritic: ::core::ffi::c_int,
    pub nException: ::core::ffi::c_int,
    pub aiException: *mut ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unicode_cursor {
    pub base: sqlite3_tokenizer_cursor,
    pub aInput: *const ::core::ffi::c_uchar,
    pub nInput: ::core::ffi::c_int,
    pub iOff: ::core::ffi::c_int,
    pub iToken: ::core::ffi::c_int,
    pub zToken: *mut ::core::ffi::c_char,
    pub nAlloc: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
static mut sqlite3Utf8Trans1: [::core::ffi::c_uchar; 64] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1b as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xb as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
unsafe extern "C" fn unicodeDestroy(mut pTokenizer: *mut sqlite3_tokenizer) -> ::core::ffi::c_int {
    if !pTokenizer.is_null() {
        let mut p: *mut unicode_tokenizer = pTokenizer as *mut unicode_tokenizer;
        sqlite3_free((*p).aiException as *mut ::core::ffi::c_void);
        sqlite3_free(p as *mut ::core::ffi::c_void);
    }
    return SQLITE_OK;
}
unsafe extern "C" fn unicodeAddExceptions(
    mut p: *mut unicode_tokenizer,
    mut bAlnum: ::core::ffi::c_int,
    mut zIn: *const ::core::ffi::c_char,
    mut nIn: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut z: *const ::core::ffi::c_uchar = zIn as *const ::core::ffi::c_uchar;
    let mut zTerm: *const ::core::ffi::c_uchar =
        z.offset(nIn as isize) as *const ::core::ffi::c_uchar;
    let mut iCode: ::core::ffi::c_uint = 0;
    let mut nEntry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while z < zTerm {
        let fresh0 = z;
        z = z.offset(1);
        iCode = *fresh0 as ::core::ffi::c_uint;
        if iCode >= 0xc0 as ::core::ffi::c_uint {
            iCode = sqlite3Utf8Trans1[iCode.wrapping_sub(0xc0 as ::core::ffi::c_uint) as usize]
                as ::core::ffi::c_uint;
            while z != zTerm
                && *z as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    == 0x80 as ::core::ffi::c_int
            {
                let fresh1 = z;
                z = z.offset(1);
                iCode = (iCode << 6 as ::core::ffi::c_int).wrapping_add(
                    (0x3f as ::core::ffi::c_int & *fresh1 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                );
            }
            if iCode < 0x80 as ::core::ffi::c_uint
                || iCode & 0xfffff800 as ::core::ffi::c_uint == 0xd800 as ::core::ffi::c_uint
                || iCode & 0xfffffffe as ::core::ffi::c_uint == 0xfffe as ::core::ffi::c_uint
            {
                iCode = 0xfffd as ::core::ffi::c_uint;
            }
        }
        if sqlite3FtsUnicodeIsalnum(iCode as ::core::ffi::c_int) != bAlnum
            && sqlite3FtsUnicodeIsdiacritic(iCode as ::core::ffi::c_int) == 0 as ::core::ffi::c_int
        {
            nEntry += 1;
        }
    }
    if nEntry != 0 {
        let mut aNew: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        let mut nNew: ::core::ffi::c_int = 0;
        aNew = sqlite3_realloc64(
            (*p).aiException as *mut ::core::ffi::c_void,
            (((*p).nException + nEntry) as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                as sqlite3_uint64,
        ) as *mut ::core::ffi::c_int;
        if aNew.is_null() {
            return SQLITE_NOMEM;
        }
        nNew = (*p).nException;
        z = zIn as *const ::core::ffi::c_uchar;
        while z < zTerm {
            let fresh2 = z;
            z = z.offset(1);
            iCode = *fresh2 as ::core::ffi::c_uint;
            if iCode >= 0xc0 as ::core::ffi::c_uint {
                iCode = sqlite3Utf8Trans1[iCode.wrapping_sub(0xc0 as ::core::ffi::c_uint) as usize]
                    as ::core::ffi::c_uint;
                while z != zTerm
                    && *z as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                        == 0x80 as ::core::ffi::c_int
                {
                    let fresh3 = z;
                    z = z.offset(1);
                    iCode = (iCode << 6 as ::core::ffi::c_int).wrapping_add(
                        (0x3f as ::core::ffi::c_int & *fresh3 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint,
                    );
                }
                if iCode < 0x80 as ::core::ffi::c_uint
                    || iCode & 0xfffff800 as ::core::ffi::c_uint == 0xd800 as ::core::ffi::c_uint
                    || iCode & 0xfffffffe as ::core::ffi::c_uint == 0xfffe as ::core::ffi::c_uint
                {
                    iCode = 0xfffd as ::core::ffi::c_uint;
                }
            }
            if sqlite3FtsUnicodeIsalnum(iCode as ::core::ffi::c_int) != bAlnum
                && sqlite3FtsUnicodeIsdiacritic(iCode as ::core::ffi::c_int)
                    == 0 as ::core::ffi::c_int
            {
                let mut i: ::core::ffi::c_int = 0;
                let mut j: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                while i < nNew && *aNew.offset(i as isize) < iCode as ::core::ffi::c_int {
                    i += 1;
                }
                j = nNew;
                while j > i {
                    *aNew.offset(j as isize) = *aNew.offset((j - 1 as ::core::ffi::c_int) as isize);
                    j -= 1;
                }
                *aNew.offset(i as isize) = iCode as ::core::ffi::c_int;
                nNew += 1;
            }
        }
        (*p).aiException = aNew;
        (*p).nException = nNew;
    }
    return SQLITE_OK;
}
unsafe extern "C" fn unicodeIsException(
    mut p: *mut unicode_tokenizer,
    mut iCode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*p).nException > 0 as ::core::ffi::c_int {
        let mut a: *mut ::core::ffi::c_int = (*p).aiException;
        let mut iLo: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iHi: ::core::ffi::c_int = (*p).nException - 1 as ::core::ffi::c_int;
        while iHi >= iLo {
            let mut iTest: ::core::ffi::c_int = (iHi + iLo) / 2 as ::core::ffi::c_int;
            if iCode == *a.offset(iTest as isize) {
                return 1 as ::core::ffi::c_int;
            } else if iCode > *a.offset(iTest as isize) {
                iLo = iTest + 1 as ::core::ffi::c_int;
            } else {
                iHi = iTest - 1 as ::core::ffi::c_int;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn unicodeIsAlnum(
    mut p: *mut unicode_tokenizer,
    mut iCode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return sqlite3FtsUnicodeIsalnum(iCode) ^ unicodeIsException(p, iCode);
}
unsafe extern "C" fn unicodeCreate(
    mut nArg: ::core::ffi::c_int,
    mut azArg: *const *const ::core::ffi::c_char,
    mut pp: *mut *mut sqlite3_tokenizer,
) -> ::core::ffi::c_int {
    let mut pNew: *mut unicode_tokenizer = ::core::ptr::null_mut::<unicode_tokenizer>();
    let mut i: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    pNew = sqlite3_malloc(::core::mem::size_of::<unicode_tokenizer>() as ::core::ffi::c_int)
        as *mut unicode_tokenizer;
    if pNew.is_null() {
        return SQLITE_NOMEM;
    }
    memset(
        pNew as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<unicode_tokenizer>() as size_t,
    );
    (*pNew).eRemoveDiacritic = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while rc == SQLITE_OK && i < nArg {
        let mut z: *const ::core::ffi::c_char = *azArg.offset(i as isize);
        let mut n: ::core::ffi::c_int = strlen(z) as ::core::ffi::c_int;
        if n == 19 as ::core::ffi::c_int
            && memcmp(
                b"remove_diacritics=1\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                z as *const ::core::ffi::c_void,
                19 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            (*pNew).eRemoveDiacritic = 1 as ::core::ffi::c_int;
        } else if n == 19 as ::core::ffi::c_int
            && memcmp(
                b"remove_diacritics=0\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                z as *const ::core::ffi::c_void,
                19 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            (*pNew).eRemoveDiacritic = 0 as ::core::ffi::c_int;
        } else if n == 19 as ::core::ffi::c_int
            && memcmp(
                b"remove_diacritics=2\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                z as *const ::core::ffi::c_void,
                19 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            (*pNew).eRemoveDiacritic = 2 as ::core::ffi::c_int;
        } else if n >= 11 as ::core::ffi::c_int
            && memcmp(
                b"tokenchars=\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                z as *const ::core::ffi::c_void,
                11 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            rc = unicodeAddExceptions(
                pNew,
                1 as ::core::ffi::c_int,
                z.offset(11 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
                n - 11 as ::core::ffi::c_int,
            );
        } else if n >= 11 as ::core::ffi::c_int
            && memcmp(
                b"separators=\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                z as *const ::core::ffi::c_void,
                11 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            rc = unicodeAddExceptions(
                pNew,
                0 as ::core::ffi::c_int,
                z.offset(11 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
                n - 11 as ::core::ffi::c_int,
            );
        } else {
            rc = SQLITE_ERROR;
        }
        i += 1;
    }
    if rc != SQLITE_OK {
        unicodeDestroy(pNew as *mut sqlite3_tokenizer);
        pNew = ::core::ptr::null_mut::<unicode_tokenizer>();
    }
    *pp = pNew as *mut sqlite3_tokenizer;
    return rc;
}
unsafe extern "C" fn unicodeOpen(
    mut _p: *mut sqlite3_tokenizer,
    mut aInput: *const ::core::ffi::c_char,
    mut nInput: ::core::ffi::c_int,
    mut pp: *mut *mut sqlite3_tokenizer_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut unicode_cursor = ::core::ptr::null_mut::<unicode_cursor>();
    pCsr = sqlite3_malloc(::core::mem::size_of::<unicode_cursor>() as ::core::ffi::c_int)
        as *mut unicode_cursor;
    if pCsr.is_null() {
        return SQLITE_NOMEM;
    }
    memset(
        pCsr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<unicode_cursor>() as size_t,
    );
    (*pCsr).aInput = aInput as *const ::core::ffi::c_uchar;
    if aInput.is_null() {
        (*pCsr).nInput = 0 as ::core::ffi::c_int;
        (*pCsr).aInput =
            b"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_uchar;
    } else if nInput < 0 as ::core::ffi::c_int {
        (*pCsr).nInput = strlen(aInput) as ::core::ffi::c_int;
    } else {
        (*pCsr).nInput = nInput;
    }
    *pp = &raw mut (*pCsr).base;
    return SQLITE_OK;
}
unsafe extern "C" fn unicodeClose(
    mut pCursor: *mut sqlite3_tokenizer_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut unicode_cursor = pCursor as *mut unicode_cursor;
    sqlite3_free((*pCsr).zToken as *mut ::core::ffi::c_void);
    sqlite3_free(pCsr as *mut ::core::ffi::c_void);
    return SQLITE_OK;
}
unsafe extern "C" fn unicodeNext(
    mut pC: *mut sqlite3_tokenizer_cursor,
    mut paToken: *mut *const ::core::ffi::c_char,
    mut pnToken: *mut ::core::ffi::c_int,
    mut piStart: *mut ::core::ffi::c_int,
    mut piEnd: *mut ::core::ffi::c_int,
    mut piPos: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut unicode_cursor = pC as *mut unicode_cursor;
    let mut p: *mut unicode_tokenizer = (*pCsr).base.pTokenizer as *mut unicode_tokenizer;
    let mut iCode: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut zOut: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut z: *const ::core::ffi::c_uchar =
        (*pCsr).aInput.offset((*pCsr).iOff as isize) as *const ::core::ffi::c_uchar;
    let mut zStart: *const ::core::ffi::c_uchar = z;
    let mut zEnd: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut zTerm: *const ::core::ffi::c_uchar =
        (*pCsr).aInput.offset((*pCsr).nInput as isize) as *const ::core::ffi::c_uchar;
    while z < zTerm {
        let fresh4 = z;
        z = z.offset(1);
        iCode = *fresh4 as ::core::ffi::c_uint;
        if iCode >= 0xc0 as ::core::ffi::c_uint {
            iCode = sqlite3Utf8Trans1[iCode.wrapping_sub(0xc0 as ::core::ffi::c_uint) as usize]
                as ::core::ffi::c_uint;
            while z != zTerm
                && *z as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    == 0x80 as ::core::ffi::c_int
            {
                let fresh5 = z;
                z = z.offset(1);
                iCode = (iCode << 6 as ::core::ffi::c_int).wrapping_add(
                    (0x3f as ::core::ffi::c_int & *fresh5 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                );
            }
            if iCode < 0x80 as ::core::ffi::c_uint
                || iCode & 0xfffff800 as ::core::ffi::c_uint == 0xd800 as ::core::ffi::c_uint
                || iCode & 0xfffffffe as ::core::ffi::c_uint == 0xfffe as ::core::ffi::c_uint
            {
                iCode = 0xfffd as ::core::ffi::c_uint;
            }
        }
        if unicodeIsAlnum(p, iCode as ::core::ffi::c_int) != 0 {
            break;
        }
        zStart = z;
    }
    if zStart >= zTerm {
        return SQLITE_DONE;
    }
    zOut = (*pCsr).zToken;
    loop {
        let mut iOut: ::core::ffi::c_int = 0;
        if zOut.offset_from((*pCsr).zToken) as ::core::ffi::c_long
            >= ((*pCsr).nAlloc - 4 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            let mut zNew: *mut ::core::ffi::c_char = sqlite3_realloc64(
                (*pCsr).zToken as *mut ::core::ffi::c_void,
                ((*pCsr).nAlloc + 64 as ::core::ffi::c_int) as sqlite3_uint64,
            ) as *mut ::core::ffi::c_char;
            if zNew.is_null() {
                return SQLITE_NOMEM;
            }
            zOut = zNew.offset(zOut.offset_from((*pCsr).zToken) as ::core::ffi::c_long as isize)
                as *mut ::core::ffi::c_char;
            (*pCsr).zToken = zNew;
            (*pCsr).nAlloc += 64 as ::core::ffi::c_int;
        }
        zEnd = z;
        iOut = sqlite3FtsUnicodeFold(iCode as ::core::ffi::c_int, (*p).eRemoveDiacritic);
        if iOut != 0 {
            if iOut < 0x80 as ::core::ffi::c_int {
                let fresh6 = zOut;
                zOut = zOut.offset(1);
                *fresh6 = (iOut & 0xff as ::core::ffi::c_int) as u8_0 as ::core::ffi::c_char;
            } else if iOut < 0x800 as ::core::ffi::c_int {
                let fresh7 = zOut;
                zOut = zOut.offset(1);
                *fresh7 = (0xc0 as ::core::ffi::c_int
                    + (iOut >> 6 as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as u8_0
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
                let fresh8 = zOut;
                zOut = zOut.offset(1);
                *fresh8 = (0x80 as ::core::ffi::c_int
                    + (iOut & 0x3f as ::core::ffi::c_int) as u8_0 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            } else if iOut < 0x10000 as ::core::ffi::c_int {
                let fresh9 = zOut;
                zOut = zOut.offset(1);
                *fresh9 = (0xe0 as ::core::ffi::c_int
                    + (iOut >> 12 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as u8_0
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
                let fresh10 = zOut;
                zOut = zOut.offset(1);
                *fresh10 = (0x80 as ::core::ffi::c_int
                    + (iOut >> 6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as u8_0
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
                let fresh11 = zOut;
                zOut = zOut.offset(1);
                *fresh11 = (0x80 as ::core::ffi::c_int
                    + (iOut & 0x3f as ::core::ffi::c_int) as u8_0 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            } else {
                let fresh12 = zOut;
                zOut = zOut.offset(1);
                *fresh12 = (0xf0 as ::core::ffi::c_int
                    + (iOut >> 18 as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as u8_0
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
                let fresh13 = zOut;
                zOut = zOut.offset(1);
                *fresh13 = (0x80 as ::core::ffi::c_int
                    + (iOut >> 12 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as u8_0
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
                let fresh14 = zOut;
                zOut = zOut.offset(1);
                *fresh14 = (0x80 as ::core::ffi::c_int
                    + (iOut >> 6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as u8_0
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
                let fresh15 = zOut;
                zOut = zOut.offset(1);
                *fresh15 = (0x80 as ::core::ffi::c_int
                    + (iOut & 0x3f as ::core::ffi::c_int) as u8_0 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
        }
        if z >= zTerm {
            break;
        }
        let fresh16 = z;
        z = z.offset(1);
        iCode = *fresh16 as ::core::ffi::c_uint;
        if iCode >= 0xc0 as ::core::ffi::c_uint {
            iCode = sqlite3Utf8Trans1[iCode.wrapping_sub(0xc0 as ::core::ffi::c_uint) as usize]
                as ::core::ffi::c_uint;
            while z != zTerm
                && *z as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    == 0x80 as ::core::ffi::c_int
            {
                let fresh17 = z;
                z = z.offset(1);
                iCode = (iCode << 6 as ::core::ffi::c_int).wrapping_add(
                    (0x3f as ::core::ffi::c_int & *fresh17 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                );
            }
            if iCode < 0x80 as ::core::ffi::c_uint
                || iCode & 0xfffff800 as ::core::ffi::c_uint == 0xd800 as ::core::ffi::c_uint
                || iCode & 0xfffffffe as ::core::ffi::c_uint == 0xfffe as ::core::ffi::c_uint
            {
                iCode = 0xfffd as ::core::ffi::c_uint;
            }
        }
        if !(unicodeIsAlnum(p, iCode as ::core::ffi::c_int) != 0
            || sqlite3FtsUnicodeIsdiacritic(iCode as ::core::ffi::c_int) != 0)
        {
            break;
        }
    }
    (*pCsr).iOff = z.offset_from((*pCsr).aInput) as ::core::ffi::c_long as ::core::ffi::c_int;
    *paToken = (*pCsr).zToken;
    *pnToken = zOut.offset_from((*pCsr).zToken) as ::core::ffi::c_long as ::core::ffi::c_int;
    *piStart = zStart.offset_from((*pCsr).aInput) as ::core::ffi::c_long as ::core::ffi::c_int;
    *piEnd = zEnd.offset_from((*pCsr).aInput) as ::core::ffi::c_long as ::core::ffi::c_int;
    let fresh18 = (*pCsr).iToken;
    (*pCsr).iToken = (*pCsr).iToken + 1;
    *piPos = fresh18;
    return SQLITE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3UnicodeTokenizer(
    mut ppModule: *mut *const sqlite3_tokenizer_module,
) {
    static mut module: sqlite3_tokenizer_module = unsafe {
        sqlite3_tokenizer_module {
            iVersion: 0 as ::core::ffi::c_int,
            xCreate: Some(
                unicodeCreate
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const *const ::core::ffi::c_char,
                        *mut *mut sqlite3_tokenizer,
                    ) -> ::core::ffi::c_int,
            ),
            xDestroy: Some(
                unicodeDestroy
                    as unsafe extern "C" fn(*mut sqlite3_tokenizer) -> ::core::ffi::c_int,
            ),
            xOpen: Some(
                unicodeOpen
                    as unsafe extern "C" fn(
                        *mut sqlite3_tokenizer,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_tokenizer_cursor,
                    ) -> ::core::ffi::c_int,
            ),
            xClose: Some(
                unicodeClose
                    as unsafe extern "C" fn(*mut sqlite3_tokenizer_cursor) -> ::core::ffi::c_int,
            ),
            xNext: Some(
                unicodeNext
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
    *ppModule = &raw const module;
}
