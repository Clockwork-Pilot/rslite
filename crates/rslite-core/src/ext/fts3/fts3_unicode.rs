pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::SizeT;

pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module;
pub use crate::src::ext::fts3::fts3_unicode2::sqlite3FtsUnicodeFold;
pub use crate::src::ext::fts3::fts3_unicode2::sqlite3FtsUnicodeIsalnum;
pub use crate::src::ext::fts3::fts3_unicode2::sqlite3FtsUnicodeIsdiacritic;
pub use crate::src::ext::rtree::rtree::U8_0;
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
pub struct unicode_tokenizer {
    pub base: crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
    pub eRemoveDiacritic: ::core::ffi::c_int,
    pub nException: ::core::ffi::c_int,
    pub aiException: *mut ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unicode_cursor {
    pub base: crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
    pub aInput: *const ::core::ffi::c_uchar,
    pub nInput: ::core::ffi::c_int,
    pub iOff: ::core::ffi::c_int,
    pub iToken: ::core::ffi::c_int,
    pub zToken: *mut ::core::ffi::c_char,
    pub nAlloc: ::core::ffi::c_int,
}

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

unsafe extern "C" fn unicodeDestroy(
    pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
) -> ::core::ffi::c_int {
    if !pTokenizer.is_null() {
        let p: *mut unicode_tokenizer = pTokenizer as *mut unicode_tokenizer;
        crate::src::src::malloc::sqlite3_free((*p).aiException as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn unicodeAddExceptions(
    p: *mut unicode_tokenizer,
    bAlnum: ::core::ffi::c_int,
    zIn: *const ::core::ffi::c_char,
    nIn: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut z: *const ::core::ffi::c_uchar = zIn as *const ::core::ffi::c_uchar;
    let zTerm: *const ::core::ffi::c_uchar =
        z.offset(nIn as isize) as *const ::core::ffi::c_uchar;
    let mut iCode: ::core::ffi::c_uint;
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
        if crate::src::ext::fts3::fts3_unicode2::sqlite3FtsUnicodeIsalnum(
            iCode as ::core::ffi::c_int,
        ) != bAlnum
            && crate::src::ext::fts3::fts3_unicode2::sqlite3FtsUnicodeIsdiacritic(
                iCode as ::core::ffi::c_int,
            ) == 0 as ::core::ffi::c_int
        {
            nEntry += 1;
        }
    }
    if nEntry != 0 {
        
        let mut nNew: ::core::ffi::c_int;
        let __p_ref = { &mut *p };
        let aNew: *mut ::core::ffi::c_int = crate::src::src::malloc::sqlite3_realloc64(
            __p_ref.aiException as *mut ::core::ffi::c_void,
            ((__p_ref.nException + nEntry) as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut ::core::ffi::c_int;
        if aNew.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        nNew = __p_ref.nException;
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
            if crate::src::ext::fts3::fts3_unicode2::sqlite3FtsUnicodeIsalnum(
                iCode as ::core::ffi::c_int,
            ) != bAlnum
                && crate::src::ext::fts3::fts3_unicode2::sqlite3FtsUnicodeIsdiacritic(
                    iCode as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
            {
                let mut i: ::core::ffi::c_int;
                let mut j: ::core::ffi::c_int;
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
        __p_ref.aiException = aNew;
        __p_ref.nException = nNew;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn unicodeIsException(
    p: *mut unicode_tokenizer,
    iCode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*p).nException > 0 as ::core::ffi::c_int {
        let a: *mut ::core::ffi::c_int = (*p).aiException;
        let mut iLo: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iHi: ::core::ffi::c_int = (*p).nException - 1 as ::core::ffi::c_int;
        while iHi >= iLo {
            let iTest: ::core::ffi::c_int = (iHi + iLo) / 2 as ::core::ffi::c_int;
            if iCode == *a.offset(iTest as isize) {
                return 1 as ::core::ffi::c_int;
            } else if iCode > *a.offset(iTest as isize) {
                iLo = iTest + 1 as ::core::ffi::c_int;
            } else {
                iHi = iTest - 1 as ::core::ffi::c_int;
            }
        }
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn unicodeIsAlnum(
    p: *mut unicode_tokenizer,
    iCode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    crate::src::ext::fts3::fts3_unicode2::sqlite3FtsUnicodeIsalnum(iCode)
        ^ unicodeIsException(p, iCode)
}

unsafe extern "C" fn unicodeCreate(
    nArg: ::core::ffi::c_int,
    azArg: *const *const ::core::ffi::c_char,
    pp: *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
) -> ::core::ffi::c_int {
    let mut pNew: *mut unicode_tokenizer;
    let mut i: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    pNew = crate::src::src::malloc::sqlite3_malloc(
        ::core::mem::size_of::<unicode_tokenizer>() as ::core::ffi::c_int
    ) as *mut unicode_tokenizer;
    if pNew.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        pNew as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<unicode_tokenizer>() as crate::__stddef_size_t_h::SizeT,
    );
    (*pNew).eRemoveDiacritic = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < nArg {
        let z: *const ::core::ffi::c_char = *azArg.offset(i as isize);
        let n: ::core::ffi::c_int = ::libc::strlen(z) as ::core::ffi::c_int;
        if n == 19 as ::core::ffi::c_int
            && ::libc::memcmp(
                b"remove_diacritics=1\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                z as *const ::core::ffi::c_void,
                19 as crate::__stddef_size_t_h::SizeT,
            ) == 0 as ::core::ffi::c_int
        {
            (*pNew).eRemoveDiacritic = 1 as ::core::ffi::c_int;
        } else if n == 19 as ::core::ffi::c_int
            && ::libc::memcmp(
                b"remove_diacritics=0\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                z as *const ::core::ffi::c_void,
                19 as crate::__stddef_size_t_h::SizeT,
            ) == 0 as ::core::ffi::c_int
        {
            (*pNew).eRemoveDiacritic = 0 as ::core::ffi::c_int;
        } else if n == 19 as ::core::ffi::c_int
            && ::libc::memcmp(
                b"remove_diacritics=2\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                z as *const ::core::ffi::c_void,
                19 as crate::__stddef_size_t_h::SizeT,
            ) == 0 as ::core::ffi::c_int
        {
            (*pNew).eRemoveDiacritic = 2 as ::core::ffi::c_int;
        } else if n >= 11 as ::core::ffi::c_int
            && ::libc::memcmp(
                b"tokenchars=\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                z as *const ::core::ffi::c_void,
                11 as crate::__stddef_size_t_h::SizeT,
            ) == 0 as ::core::ffi::c_int
        {
            rc = unicodeAddExceptions(
                pNew,
                1 as ::core::ffi::c_int,
                z.offset(11_isize) as *const ::core::ffi::c_char,
                n - 11 as ::core::ffi::c_int,
            );
        } else if n >= 11 as ::core::ffi::c_int
            && ::libc::memcmp(
                b"separators=\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                z as *const ::core::ffi::c_void,
                11 as crate::__stddef_size_t_h::SizeT,
            ) == 0 as ::core::ffi::c_int
        {
            rc = unicodeAddExceptions(
                pNew,
                0 as ::core::ffi::c_int,
                z.offset(11_isize) as *const ::core::ffi::c_char,
                n - 11 as ::core::ffi::c_int,
            );
        } else {
            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
        }
        i += 1;
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        unicodeDestroy(pNew as *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer);
        pNew = ::core::ptr::null_mut::<unicode_tokenizer>();
    }
    *pp = pNew as *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer;
    rc
}

unsafe extern "C" fn unicodeOpen(
    mut _p: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
    aInput: *const ::core::ffi::c_char,
    nInput: ::core::ffi::c_int,
    pp: *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
) -> ::core::ffi::c_int {
    
    let pCsr: *mut unicode_cursor = crate::src::src::malloc::sqlite3_malloc(
        ::core::mem::size_of::<unicode_cursor>() as ::core::ffi::c_int
    ) as *mut unicode_cursor;
    if pCsr.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        pCsr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<unicode_cursor>() as crate::__stddef_size_t_h::SizeT,
    );
    (*pCsr).aInput = aInput as *const ::core::ffi::c_uchar;
    if aInput.is_null() {
        (*pCsr).nInput = 0 as ::core::ffi::c_int;
        (*pCsr).aInput =
            b"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_uchar;
    } else if nInput < 0 as ::core::ffi::c_int {
        (*pCsr).nInput = ::libc::strlen(aInput) as ::core::ffi::c_int;
    } else {
        (*pCsr).nInput = nInput;
    }
    *pp = &raw mut (*pCsr).base;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn unicodeClose(
    pCursor: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
) -> ::core::ffi::c_int {
    let pCsr: *mut unicode_cursor = pCursor as *mut unicode_cursor;
    crate::src::src::malloc::sqlite3_free((*pCsr).zToken as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(pCsr as *mut ::core::ffi::c_void);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn unicodeNext(
    pC: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
    paToken: *mut *const ::core::ffi::c_char,
    pnToken: *mut ::core::ffi::c_int,
    piStart: *mut ::core::ffi::c_int,
    piEnd: *mut ::core::ffi::c_int,
    piPos: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pCsr: *mut unicode_cursor = pC as *mut unicode_cursor;
    let __pCsr_ref = { &mut *pCsr };
    let p: *mut unicode_tokenizer = __pCsr_ref.base.pTokenizer as *mut unicode_tokenizer;
    let mut iCode: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut zOut: *mut ::core::ffi::c_char;
    let mut z: *const ::core::ffi::c_uchar =
        __pCsr_ref.aInput.offset(__pCsr_ref.iOff as isize) as *const ::core::ffi::c_uchar;
    let mut zStart: *const ::core::ffi::c_uchar = z;
    let mut zEnd: *const ::core::ffi::c_uchar;
    let zTerm: *const ::core::ffi::c_uchar =
        __pCsr_ref.aInput.offset(__pCsr_ref.nInput as isize) as *const ::core::ffi::c_uchar;
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
        return crate::src::headers::sqlite3_h::SQLITE_DONE;
    }
    zOut = __pCsr_ref.zToken;
    loop {
        
        if zOut.offset_from(__pCsr_ref.zToken) as ::core::ffi::c_long
            >= (__pCsr_ref.nAlloc - 4 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            let zNew: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_realloc64(
                __pCsr_ref.zToken as *mut ::core::ffi::c_void,
                (__pCsr_ref.nAlloc + 64 as ::core::ffi::c_int)
                    as crate::src::headers::sqlite3_h::Sqlite3Uint64,
            ) as *mut ::core::ffi::c_char;
            if zNew.is_null() {
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
            zOut = zNew.offset(zOut.offset_from(__pCsr_ref.zToken) as ::core::ffi::c_long as isize)
                as *mut ::core::ffi::c_char;
            __pCsr_ref.zToken = zNew;
            __pCsr_ref.nAlloc += 64 as ::core::ffi::c_int;
        }
        zEnd = z;
        let iOut: ::core::ffi::c_int = crate::src::ext::fts3::fts3_unicode2::sqlite3FtsUnicodeFold(
            iCode as ::core::ffi::c_int,
            (*p).eRemoveDiacritic,
        );
        if iOut != 0 {
            if iOut < 0x80 as ::core::ffi::c_int {
                let fresh6 = zOut;
                zOut = zOut.offset(1);
                *fresh6 = (iOut & 0xff as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U8_0
                    as ::core::ffi::c_char;
            } else if iOut < 0x800 as ::core::ffi::c_int {
                let fresh7 = zOut;
                zOut = zOut.offset(1);
                *fresh7 = (0xc0 as ::core::ffi::c_int
                    + (iOut >> 6 as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
                        as crate::src::ext::rtree::rtree::U8_0
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
                let fresh8 = zOut;
                zOut = zOut.offset(1);
                *fresh8 = (0x80 as ::core::ffi::c_int
                    + (iOut & 0x3f as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U8_0
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
            } else if iOut < 0x10000 as ::core::ffi::c_int {
                let fresh9 = zOut;
                zOut = zOut.offset(1);
                *fresh9 = (0xe0 as ::core::ffi::c_int
                    + (iOut >> 12 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                        as crate::src::ext::rtree::rtree::U8_0
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
                let fresh10 = zOut;
                zOut = zOut.offset(1);
                *fresh10 = (0x80 as ::core::ffi::c_int
                    + (iOut >> 6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                        as crate::src::ext::rtree::rtree::U8_0
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
                let fresh11 = zOut;
                zOut = zOut.offset(1);
                *fresh11 = (0x80 as ::core::ffi::c_int
                    + (iOut & 0x3f as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U8_0
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
            } else {
                let fresh12 = zOut;
                zOut = zOut.offset(1);
                *fresh12 = (0xf0 as ::core::ffi::c_int
                    + (iOut >> 18 as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int)
                        as crate::src::ext::rtree::rtree::U8_0
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
                let fresh13 = zOut;
                zOut = zOut.offset(1);
                *fresh13 = (0x80 as ::core::ffi::c_int
                    + (iOut >> 12 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                        as crate::src::ext::rtree::rtree::U8_0
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
                let fresh14 = zOut;
                zOut = zOut.offset(1);
                *fresh14 = (0x80 as ::core::ffi::c_int
                    + (iOut >> 6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                        as crate::src::ext::rtree::rtree::U8_0
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
                let fresh15 = zOut;
                zOut = zOut.offset(1);
                *fresh15 = (0x80 as ::core::ffi::c_int
                    + (iOut & 0x3f as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U8_0
                        as ::core::ffi::c_int) as ::core::ffi::c_char;
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
            || crate::src::ext::fts3::fts3_unicode2::sqlite3FtsUnicodeIsdiacritic(
                iCode as ::core::ffi::c_int,
            ) != 0)
        {
            break;
        }
    }
    __pCsr_ref.iOff = z.offset_from(__pCsr_ref.aInput) as ::core::ffi::c_long as ::core::ffi::c_int;
    *paToken = __pCsr_ref.zToken;
    *pnToken = zOut.offset_from(__pCsr_ref.zToken) as ::core::ffi::c_long as ::core::ffi::c_int;
    *piStart = zStart.offset_from(__pCsr_ref.aInput) as ::core::ffi::c_long as ::core::ffi::c_int;
    *piEnd = zEnd.offset_from(__pCsr_ref.aInput) as ::core::ffi::c_long as ::core::ffi::c_int;
    let fresh18 = __pCsr_ref.iToken;
    __pCsr_ref.iToken += 1;
    *piPos = fresh18;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3Fts3UnicodeTokenizer(
    ppModule: *mut *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module,
) {
    static mut module: crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module = {
        crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module {
            iVersion: 0 as ::core::ffi::c_int,
            xCreate: Some(
                unicodeCreate
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const *const ::core::ffi::c_char,
                        *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
                    ) -> ::core::ffi::c_int,
            ),
            xDestroy: Some(
                unicodeDestroy
                    as unsafe extern "C" fn(
                        *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
                    ) -> ::core::ffi::c_int,
            ),
            xOpen: Some(
                unicodeOpen
                    as unsafe extern "C" fn(
                        *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
                    ) -> ::core::ffi::c_int,
            ),
            xClose: Some(
                unicodeClose
                    as unsafe extern "C" fn(
                        *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
                    ) -> ::core::ffi::c_int,
            ),
            xNext: Some(
                unicodeNext
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
    *ppModule = &raw const module;
}
