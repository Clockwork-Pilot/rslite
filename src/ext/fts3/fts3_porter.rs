




pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;


pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer;pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor;pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module;pub use crate::src::src::malloc::sqlite3_free;pub use crate::src::src::malloc::sqlite3_malloc;pub use crate::src::src::malloc::sqlite3_realloc64;pub use crate::src::headers::sqlite3_h::sqlite3_uint64;pub use crate::src::headers::sqlite3_h::sqlite_uint64;pub use crate::src::headers::sqlite3_h::SQLITE_DONE;pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;pub use crate::src::headers::sqlite3_h::SQLITE_OK;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct porter_tokenizer {
    pub base: crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct porter_tokenizer_cursor {
    pub base: crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
    pub zInput: *const ::core::ffi::c_char,
    pub nInput: ::core::ffi::c_int,
    pub iOffset: ::core::ffi::c_int,
    pub iToken: ::core::ffi::c_int,
    pub zToken: *mut ::core::ffi::c_char,
    pub nAllocated: ::core::ffi::c_int,
}

unsafe extern "C" fn porterCreate(
    mut _argc: ::core::ffi::c_int,
    mut _argv: *const *const ::core::ffi::c_char,
    mut ppTokenizer: *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
) -> ::core::ffi::c_int {
    let mut t: *mut porter_tokenizer = ::core::ptr::null_mut::<porter_tokenizer>();
    t = crate::src::src::malloc::sqlite3_malloc(::core::mem::size_of::<porter_tokenizer>() as ::core::ffi::c_int)
        as *mut porter_tokenizer;
    if t.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<porter_tokenizer>() as crate::__stddef_size_t_h::size_t,
    );
    *ppTokenizer = &raw mut (*t).base;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn porterDestroy(mut pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer) -> ::core::ffi::c_int {
    crate::src::src::malloc::sqlite3_free(pTokenizer as *mut ::core::ffi::c_void);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn porterOpen(
    mut _pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
    mut zInput: *const ::core::ffi::c_char,
    mut nInput: ::core::ffi::c_int,
    mut ppCursor: *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
) -> ::core::ffi::c_int {
    let mut c: *mut porter_tokenizer_cursor = ::core::ptr::null_mut::<porter_tokenizer_cursor>();
    c = crate::src::src::malloc::sqlite3_malloc(::core::mem::size_of::<porter_tokenizer_cursor>() as ::core::ffi::c_int)
        as *mut porter_tokenizer_cursor;
    if c.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    (*c).zInput = zInput;
    if zInput.is_null() {
        (*c).nInput = 0 as ::core::ffi::c_int;
    } else if nInput < 0 as ::core::ffi::c_int {
        (*c).nInput = ::libc::strlen(zInput) as ::core::ffi::c_int;
    } else {
        (*c).nInput = nInput;
    }
    (*c).iOffset = 0 as ::core::ffi::c_int;
    (*c).iToken = 0 as ::core::ffi::c_int;
    (*c).zToken = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*c).nAllocated = 0 as ::core::ffi::c_int;
    *ppCursor = &raw mut (*c).base;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn porterClose(mut pCursor: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor) -> ::core::ffi::c_int {
    let mut c: *mut porter_tokenizer_cursor = pCursor as *mut porter_tokenizer_cursor;
    crate::src::src::malloc::sqlite3_free((*c).zToken as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(c as *mut ::core::ffi::c_void);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

static mut cType: [::core::ffi::c_char; 26] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    2 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
];

unsafe extern "C" fn isConsonant(mut z: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_char = *z;
    if x as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    j = cType[(x as ::core::ffi::c_int - 'a' as i32) as usize] as ::core::ffi::c_int;
    if j < 2 as ::core::ffi::c_int {
        return j;
    }
    (*z.offset(1 as isize) as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        || isVowel(z.offset(1 as isize)) != 0)
        as ::core::ffi::c_int
}

unsafe extern "C" fn isVowel(mut z: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut j: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_char = *z;
    if x as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    j = cType[(x as ::core::ffi::c_int - 'a' as i32) as usize] as ::core::ffi::c_int;
    if j < 2 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int - j;
    }
    isConsonant(z.offset(1 as isize))
}

unsafe extern "C" fn m_gt_0(mut z: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    while isVowel(z) != 0 {
        z = z.offset(1);
    }
    if *z as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    while isConsonant(z) != 0 {
        z = z.offset(1);
    }
    (*z as ::core::ffi::c_int != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
}

unsafe extern "C" fn m_eq_1(mut z: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    while isVowel(z) != 0 {
        z = z.offset(1);
    }
    if *z as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    while isConsonant(z) != 0 {
        z = z.offset(1);
    }
    if *z as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    while isVowel(z) != 0 {
        z = z.offset(1);
    }
    if *z as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    while isConsonant(z) != 0 {
        z = z.offset(1);
    }
    (*z as ::core::ffi::c_int == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
}

unsafe extern "C" fn m_gt_1(mut z: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    while isVowel(z) != 0 {
        z = z.offset(1);
    }
    if *z as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    while isConsonant(z) != 0 {
        z = z.offset(1);
    }
    if *z as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    while isVowel(z) != 0 {
        z = z.offset(1);
    }
    if *z as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    while isConsonant(z) != 0 {
        z = z.offset(1);
    }
    (*z as ::core::ffi::c_int != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
}

unsafe extern "C" fn hasVowel(mut z: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    while isConsonant(z) != 0 {
        z = z.offset(1);
    }
    (*z as ::core::ffi::c_int != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
}

unsafe extern "C" fn doubleConsonant(mut z: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    (isConsonant(z) != 0
        && *z.offset(0 as isize) as ::core::ffi::c_int
            == *z.offset(1 as isize) as ::core::ffi::c_int)
        as ::core::ffi::c_int
}

unsafe extern "C" fn star_oh(mut z: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    (isConsonant(z) != 0
        && *z.offset(0 as isize) as ::core::ffi::c_int != 'w' as i32
        && *z.offset(0 as isize) as ::core::ffi::c_int != 'x' as i32
        && *z.offset(0 as isize) as ::core::ffi::c_int != 'y' as i32
        && isVowel(z.offset(1 as isize)) != 0
        && isConsonant(z.offset(2 as isize)) != 0)
        as ::core::ffi::c_int
}

unsafe extern "C" fn stem(
    mut pz: *mut *mut ::core::ffi::c_char,
    mut zFrom: *const ::core::ffi::c_char,
    mut zTo: *const ::core::ffi::c_char,
    mut xCond: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    let mut z: *mut ::core::ffi::c_char = *pz;
    while *zFrom as ::core::ffi::c_int != 0
        && *zFrom as ::core::ffi::c_int == *z as ::core::ffi::c_int
    {
        z = z.offset(1);
        zFrom = zFrom.offset(1);
    }
    if *zFrom as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if xCond.is_some() && xCond.expect("non-null function pointer")(z) == 0 {
        return 1 as ::core::ffi::c_int;
    }
    while *zTo != 0 {
        let fresh0 = zTo;
        zTo = zTo.offset(1);
        z = z.offset(-1);
        *z = *fresh0;
    }
    *pz = z;
    1 as ::core::ffi::c_int
}

unsafe extern "C" fn copy_stemmer(
    mut zIn: *const ::core::ffi::c_char,
    mut nIn: ::core::ffi::c_int,
    mut zOut: *mut ::core::ffi::c_char,
    mut pnOut: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut mx: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut hasDigit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nIn {
        let mut c: ::core::ffi::c_char = *zIn.offset(i as isize);
        if c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'Z' as i32 {
            *zOut.offset(i as isize) =
                (c as ::core::ffi::c_int - 'A' as i32 + 'a' as i32) as ::core::ffi::c_char;
        } else {
            if c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32 {
                hasDigit = 1 as ::core::ffi::c_int;
            }
            *zOut.offset(i as isize) = c;
        }
        i += 1;
    }
    mx = if hasDigit != 0 {
        3 as ::core::ffi::c_int
    } else {
        10 as ::core::ffi::c_int
    };
    if nIn > mx * 2 as ::core::ffi::c_int {
        j = mx;
        i = nIn - mx;
        while i < nIn {
            *zOut.offset(j as isize) = *zOut.offset(i as isize);
            i += 1;
            j += 1;
        }
        i = j;
    }
    *zOut.offset(i as isize) = 0 as ::core::ffi::c_char;
    *pnOut = i;
}

unsafe extern "C" fn porter_stemmer(
    mut zIn: *const ::core::ffi::c_char,
    mut nIn: ::core::ffi::c_int,
    mut zOut: *mut ::core::ffi::c_char,
    mut pnOut: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut zReverse: [::core::ffi::c_char; 28] = [0; 28];
    let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut z2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if nIn < 3 as ::core::ffi::c_int
        || nIn
            >= ::core::mem::size_of::<[::core::ffi::c_char; 28]>() as ::core::ffi::c_int
                - 7 as ::core::ffi::c_int
    {
        copy_stemmer(zIn, nIn, zOut, pnOut);
        return;
    }
    i = 0 as ::core::ffi::c_int;
    j = (::core::mem::size_of::<[::core::ffi::c_char; 28]>() as usize).wrapping_sub(6 as usize)
        as ::core::ffi::c_int;
    while i < nIn {
        let mut c: ::core::ffi::c_char = *zIn.offset(i as isize);
        if c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'Z' as i32 {
            zReverse[j as usize] =
                (c as ::core::ffi::c_int + 'a' as i32 - 'A' as i32) as ::core::ffi::c_char;
        } else if c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'z' as i32 {
            zReverse[j as usize] = c;
        } else {
            copy_stemmer(zIn, nIn, zOut, pnOut);
            return;
        }
        i += 1;
        j -= 1;
    }
    ::libc::memset(
        (&raw mut zReverse as *mut ::core::ffi::c_char).offset(
            (::core::mem::size_of::<[::core::ffi::c_char; 28]>() as usize).wrapping_sub(5 as usize)
                as isize,
        ) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        5 as crate::__stddef_size_t_h::size_t,
    );
    z = (&raw mut zReverse as *mut ::core::ffi::c_char)
        .offset((j + 1 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_char;
    if *z.offset(0 as isize) as ::core::ffi::c_int == 's' as i32 {
        if stem(
            &raw mut z,
            b"sess\0" as *const u8 as *const ::core::ffi::c_char,
            b"ss\0" as *const u8 as *const ::core::ffi::c_char,
            None,
        ) == 0
            && stem(
                &raw mut z,
                b"sei\0" as *const u8 as *const ::core::ffi::c_char,
                b"i\0" as *const u8 as *const ::core::ffi::c_char,
                None,
            ) == 0
            && stem(
                &raw mut z,
                b"ss\0" as *const u8 as *const ::core::ffi::c_char,
                b"ss\0" as *const u8 as *const ::core::ffi::c_char,
                None,
            ) == 0
        {
            z = z.offset(1);
        }
    }
    z2 = z;
    if !(stem(
        &raw mut z,
        b"dee\0" as *const u8 as *const ::core::ffi::c_char,
        b"ee\0" as *const u8 as *const ::core::ffi::c_char,
        Some(m_gt_0 as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int),
    ) != 0)
    {
        if (stem(
            &raw mut z,
            b"gni\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            Some(
                hasVowel as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            ),
        ) != 0
            || stem(
                &raw mut z,
                b"de\0" as *const u8 as *const ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                Some(
                    hasVowel
                        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
                ),
            ) != 0)
            && z != z2
        {
            if !(stem(
                &raw mut z,
                b"ta\0" as *const u8 as *const ::core::ffi::c_char,
                b"ate\0" as *const u8 as *const ::core::ffi::c_char,
                None,
            ) != 0
                || stem(
                    &raw mut z,
                    b"lb\0" as *const u8 as *const ::core::ffi::c_char,
                    b"ble\0" as *const u8 as *const ::core::ffi::c_char,
                    None,
                ) != 0
                || stem(
                    &raw mut z,
                    b"zi\0" as *const u8 as *const ::core::ffi::c_char,
                    b"ize\0" as *const u8 as *const ::core::ffi::c_char,
                    None,
                ) != 0)
            {
                if doubleConsonant(z) != 0
                    && (*z as ::core::ffi::c_int != 'l' as i32
                        && *z as ::core::ffi::c_int != 's' as i32
                        && *z as ::core::ffi::c_int != 'z' as i32)
                {
                    z = z.offset(1);
                } else if m_eq_1(z) != 0 && star_oh(z) != 0 {
                    z = z.offset(-1);
                    *z = 'e' as i32 as ::core::ffi::c_char;
                }
            }
        }
    }
    if *z.offset(0 as isize) as ::core::ffi::c_int == 'y' as i32
        && hasVowel(z.offset(1 as isize)) != 0
    {
        *z.offset(0 as isize) = 'i' as i32 as ::core::ffi::c_char;
    }
    match *z.offset(1 as isize) as ::core::ffi::c_int {
        97 => {
            if stem(
                &raw mut z,
                b"lanoita\0" as *const u8 as *const ::core::ffi::c_char,
                b"ate\0" as *const u8 as *const ::core::ffi::c_char,
                Some(
                    m_gt_0
                        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
                ),
            ) == 0
            {
                stem(
                    &raw mut z,
                    b"lanoit\0" as *const u8 as *const ::core::ffi::c_char,
                    b"tion\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                );
            }
        }
        99 => {
            if stem(
                &raw mut z,
                b"icne\0" as *const u8 as *const ::core::ffi::c_char,
                b"ence\0" as *const u8 as *const ::core::ffi::c_char,
                Some(
                    m_gt_0
                        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
                ),
            ) == 0
            {
                stem(
                    &raw mut z,
                    b"icna\0" as *const u8 as *const ::core::ffi::c_char,
                    b"ance\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                );
            }
        }
        101 => {
            stem(
                &raw mut z,
                b"rezi\0" as *const u8 as *const ::core::ffi::c_char,
                b"ize\0" as *const u8 as *const ::core::ffi::c_char,
                Some(
                    m_gt_0
                        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
                ),
            );
        }
        103 => {
            stem(
                &raw mut z,
                b"igol\0" as *const u8 as *const ::core::ffi::c_char,
                b"log\0" as *const u8 as *const ::core::ffi::c_char,
                Some(
                    m_gt_0
                        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
                ),
            );
        }
        108 => {
            if stem(
                &raw mut z,
                b"ilb\0" as *const u8 as *const ::core::ffi::c_char,
                b"ble\0" as *const u8 as *const ::core::ffi::c_char,
                Some(
                    m_gt_0
                        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
                ),
            ) == 0
                && stem(
                    &raw mut z,
                    b"illa\0" as *const u8 as *const ::core::ffi::c_char,
                    b"al\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                ) == 0
                && stem(
                    &raw mut z,
                    b"iltne\0" as *const u8 as *const ::core::ffi::c_char,
                    b"ent\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                ) == 0
                && stem(
                    &raw mut z,
                    b"ile\0" as *const u8 as *const ::core::ffi::c_char,
                    b"e\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                ) == 0
            {
                stem(
                    &raw mut z,
                    b"ilsuo\0" as *const u8 as *const ::core::ffi::c_char,
                    b"ous\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                );
            }
        }
        111 => {
            if stem(
                &raw mut z,
                b"noitazi\0" as *const u8 as *const ::core::ffi::c_char,
                b"ize\0" as *const u8 as *const ::core::ffi::c_char,
                Some(
                    m_gt_0
                        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
                ),
            ) == 0
                && stem(
                    &raw mut z,
                    b"noita\0" as *const u8 as *const ::core::ffi::c_char,
                    b"ate\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                ) == 0
            {
                stem(
                    &raw mut z,
                    b"rota\0" as *const u8 as *const ::core::ffi::c_char,
                    b"ate\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                );
            }
        }
        115 => {
            if stem(
                &raw mut z,
                b"msila\0" as *const u8 as *const ::core::ffi::c_char,
                b"al\0" as *const u8 as *const ::core::ffi::c_char,
                Some(
                    m_gt_0
                        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
                ),
            ) == 0
                && stem(
                    &raw mut z,
                    b"ssenevi\0" as *const u8 as *const ::core::ffi::c_char,
                    b"ive\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                ) == 0
                && stem(
                    &raw mut z,
                    b"ssenluf\0" as *const u8 as *const ::core::ffi::c_char,
                    b"ful\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                ) == 0
            {
                stem(
                    &raw mut z,
                    b"ssensuo\0" as *const u8 as *const ::core::ffi::c_char,
                    b"ous\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                );
            }
        }
        116 => {
            if stem(
                &raw mut z,
                b"itila\0" as *const u8 as *const ::core::ffi::c_char,
                b"al\0" as *const u8 as *const ::core::ffi::c_char,
                Some(
                    m_gt_0
                        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
                ),
            ) == 0
                && stem(
                    &raw mut z,
                    b"itivi\0" as *const u8 as *const ::core::ffi::c_char,
                    b"ive\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                ) == 0
            {
                stem(
                    &raw mut z,
                    b"itilib\0" as *const u8 as *const ::core::ffi::c_char,
                    b"ble\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                );
            }
        }
        _ => {}
    }
    match *z.offset(0 as isize) as ::core::ffi::c_int {
        101 => {
            if stem(
                &raw mut z,
                b"etaci\0" as *const u8 as *const ::core::ffi::c_char,
                b"ic\0" as *const u8 as *const ::core::ffi::c_char,
                Some(
                    m_gt_0
                        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
                ),
            ) == 0
                && stem(
                    &raw mut z,
                    b"evita\0" as *const u8 as *const ::core::ffi::c_char,
                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                ) == 0
            {
                stem(
                    &raw mut z,
                    b"ezila\0" as *const u8 as *const ::core::ffi::c_char,
                    b"al\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                );
            }
        }
        105 => {
            stem(
                &raw mut z,
                b"itici\0" as *const u8 as *const ::core::ffi::c_char,
                b"ic\0" as *const u8 as *const ::core::ffi::c_char,
                Some(
                    m_gt_0
                        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
                ),
            );
        }
        108 => {
            if stem(
                &raw mut z,
                b"laci\0" as *const u8 as *const ::core::ffi::c_char,
                b"ic\0" as *const u8 as *const ::core::ffi::c_char,
                Some(
                    m_gt_0
                        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
                ),
            ) == 0
            {
                stem(
                    &raw mut z,
                    b"luf\0" as *const u8 as *const ::core::ffi::c_char,
                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_0
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                );
            }
        }
        115 => {
            stem(
                &raw mut z,
                b"ssen\0" as *const u8 as *const ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                Some(
                    m_gt_0
                        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
                ),
            );
        }
        _ => {}
    }
    match *z.offset(1 as isize) as ::core::ffi::c_int {
        97 => {
            if *z.offset(0 as isize) as ::core::ffi::c_int == 'l' as i32
                && m_gt_1(z.offset(2 as isize)) != 0
            {
                z = z.offset(2 as isize);
            }
        }
        99 => {
            if *z.offset(0 as isize) as ::core::ffi::c_int == 'e' as i32
                && *z.offset(2 as isize) as ::core::ffi::c_int == 'n' as i32
                && (*z.offset(3 as isize) as ::core::ffi::c_int == 'a' as i32
                    || *z.offset(3 as isize) as ::core::ffi::c_int
                        == 'e' as i32)
                && m_gt_1(z.offset(4 as isize)) != 0
            {
                z = z.offset(4 as isize);
            }
        }
        101 => {
            if *z.offset(0 as isize) as ::core::ffi::c_int == 'r' as i32
                && m_gt_1(z.offset(2 as isize)) != 0
            {
                z = z.offset(2 as isize);
            }
        }
        105 => {
            if *z.offset(0 as isize) as ::core::ffi::c_int == 'c' as i32
                && m_gt_1(z.offset(2 as isize)) != 0
            {
                z = z.offset(2 as isize);
            }
        }
        108 => {
            if *z.offset(0 as isize) as ::core::ffi::c_int == 'e' as i32
                && *z.offset(2 as isize) as ::core::ffi::c_int == 'b' as i32
                && (*z.offset(3 as isize) as ::core::ffi::c_int == 'a' as i32
                    || *z.offset(3 as isize) as ::core::ffi::c_int
                        == 'i' as i32)
                && m_gt_1(z.offset(4 as isize)) != 0
            {
                z = z.offset(4 as isize);
            }
        }
        110 => {
            if *z.offset(0 as isize) as ::core::ffi::c_int == 't' as i32 {
                if *z.offset(2 as isize) as ::core::ffi::c_int == 'a' as i32 {
                    if m_gt_1(z.offset(3 as isize)) != 0 {
                        z = z.offset(3 as isize);
                    }
                } else if *z.offset(2 as isize) as ::core::ffi::c_int
                    == 'e' as i32
                {
                    if stem(
                        &raw mut z,
                        b"tneme\0" as *const u8 as *const ::core::ffi::c_char,
                        b"\0" as *const u8 as *const ::core::ffi::c_char,
                        Some(
                            m_gt_1
                                as unsafe extern "C" fn(
                                    *const ::core::ffi::c_char,
                                )
                                    -> ::core::ffi::c_int,
                        ),
                    ) == 0
                        && stem(
                            &raw mut z,
                            b"tnem\0" as *const u8 as *const ::core::ffi::c_char,
                            b"\0" as *const u8 as *const ::core::ffi::c_char,
                            Some(
                                m_gt_1
                                    as unsafe extern "C" fn(
                                        *const ::core::ffi::c_char,
                                    )
                                        -> ::core::ffi::c_int,
                            ),
                        ) == 0
                    {
                        stem(
                            &raw mut z,
                            b"tne\0" as *const u8 as *const ::core::ffi::c_char,
                            b"\0" as *const u8 as *const ::core::ffi::c_char,
                            Some(
                                m_gt_1
                                    as unsafe extern "C" fn(
                                        *const ::core::ffi::c_char,
                                    )
                                        -> ::core::ffi::c_int,
                            ),
                        );
                    }
                }
            }
        }
        111 => {
            if *z.offset(0 as isize) as ::core::ffi::c_int == 'u' as i32 {
                if m_gt_1(z.offset(2 as isize)) != 0 {
                    z = z.offset(2 as isize);
                }
            } else if *z.offset(3 as isize) as ::core::ffi::c_int
                == 's' as i32
                || *z.offset(3 as isize) as ::core::ffi::c_int == 't' as i32
            {
                stem(
                    &raw mut z,
                    b"noi\0" as *const u8 as *const ::core::ffi::c_char,
                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_1
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                );
            }
        }
        115 => {
            if *z.offset(0 as isize) as ::core::ffi::c_int == 'm' as i32
                && *z.offset(2 as isize) as ::core::ffi::c_int == 'i' as i32
                && m_gt_1(z.offset(3 as isize)) != 0
            {
                z = z.offset(3 as isize);
            }
        }
        116 => {
            if stem(
                &raw mut z,
                b"eta\0" as *const u8 as *const ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                Some(
                    m_gt_1
                        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
                ),
            ) == 0
            {
                stem(
                    &raw mut z,
                    b"iti\0" as *const u8 as *const ::core::ffi::c_char,
                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                    Some(
                        m_gt_1
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                );
            }
        }
        117 => {
            if *z.offset(0 as isize) as ::core::ffi::c_int == 's' as i32
                && *z.offset(2 as isize) as ::core::ffi::c_int == 'o' as i32
                && m_gt_1(z.offset(3 as isize)) != 0
            {
                z = z.offset(3 as isize);
            }
        }
        118 | 122 => {
            if *z.offset(0 as isize) as ::core::ffi::c_int == 'e' as i32
                && *z.offset(2 as isize) as ::core::ffi::c_int == 'i' as i32
                && m_gt_1(z.offset(3 as isize)) != 0
            {
                z = z.offset(3 as isize);
            }
        }
        _ => {}
    }
    if *z.offset(0 as isize) as ::core::ffi::c_int == 'e' as i32 {
        if m_gt_1(z.offset(1 as isize)) != 0 {
            z = z.offset(1);
        } else if m_eq_1(z.offset(1 as isize)) != 0
            && star_oh(z.offset(1 as isize)) == 0
        {
            z = z.offset(1);
        }
    }
    if m_gt_1(z) != 0
        && *z.offset(0 as isize) as ::core::ffi::c_int == 'l' as i32
        && *z.offset(1 as isize) as ::core::ffi::c_int == 'l' as i32
    {
        z = z.offset(1);
    }
    i = ::libc::strlen(z) as ::core::ffi::c_int;
    *pnOut = i;
    *zOut.offset(i as isize) = 0 as ::core::ffi::c_char;
    while *z != 0 {
        let fresh1 = z;
        z = z.offset(1);
        i -= 1;
        *zOut.offset(i as isize) = *fresh1;
    }
}

static mut porterIdChar: [::core::ffi::c_char; 80] = [
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
];

unsafe extern "C" fn porterNext(
    mut pCursor: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
    mut pzToken: *mut *const ::core::ffi::c_char,
    mut pnBytes: *mut ::core::ffi::c_int,
    mut piStartOffset: *mut ::core::ffi::c_int,
    mut piEndOffset: *mut ::core::ffi::c_int,
    mut piPosition: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: *mut porter_tokenizer_cursor = pCursor as *mut porter_tokenizer_cursor;
    let __c_ref = unsafe { &mut *c };
    let mut z: *const ::core::ffi::c_char = __c_ref.zInput;
    while __c_ref.iOffset < __c_ref.nInput {
        let mut iStartOffset: ::core::ffi::c_int = 0;
        let mut ch: ::core::ffi::c_int = 0;
        while __c_ref.iOffset < __c_ref.nInput && {
            ch = *z.offset(__c_ref.iOffset as isize) as ::core::ffi::c_int;
            ch & 0x80 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && (ch < 0x30 as ::core::ffi::c_int
                    || porterIdChar[(ch - 0x30 as ::core::ffi::c_int) as usize] == 0)
        } {
            __c_ref.iOffset += 1;
        }
        iStartOffset = __c_ref.iOffset;
        while __c_ref.iOffset < __c_ref.nInput && {
            ch = *z.offset(__c_ref.iOffset as isize) as ::core::ffi::c_int;
            !(ch & 0x80 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && (ch < 0x30 as ::core::ffi::c_int
                    || porterIdChar[(ch - 0x30 as ::core::ffi::c_int) as usize] == 0))
        } {
            __c_ref.iOffset += 1;
        }
        if __c_ref.iOffset > iStartOffset {
            let mut n: ::core::ffi::c_int = __c_ref.iOffset - iStartOffset;
            if n > __c_ref.nAllocated {
                let mut pNew: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                __c_ref.nAllocated = n + 20 as ::core::ffi::c_int;
                pNew = crate::src::src::malloc::sqlite3_realloc64(
                    __c_ref.zToken as *mut ::core::ffi::c_void,
                    __c_ref.nAllocated as crate::src::headers::sqlite3_h::sqlite3_uint64,
                ) as *mut ::core::ffi::c_char;
                if pNew.is_null() {
                    return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                }
                __c_ref.zToken = pNew;
            }
            porter_stemmer(
                z.offset(iStartOffset as isize) as *const ::core::ffi::c_char,
                n,
                __c_ref.zToken,
                pnBytes,
            );
            *pzToken = __c_ref.zToken;
            *piStartOffset = iStartOffset;
            *piEndOffset = __c_ref.iOffset;
            let fresh2 = __c_ref.iToken;
            __c_ref.iToken += 1;
            *piPosition = fresh2;
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_DONE
}

static mut porterTokenizerModule: crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module = unsafe {
    crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module {
    iVersion:  0 as ::core::ffi::c_int,
    xCreate:  Some(
            porterCreate
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const *const ::core::ffi::c_char,
                    *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
                ) -> ::core::ffi::c_int,
        ),
    xDestroy:  Some(
            porterDestroy as unsafe extern "C" fn(*mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer) -> ::core::ffi::c_int,
        ),
    xOpen:  Some(
            porterOpen
                as unsafe extern "C" fn(
                    *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
                ) -> ::core::ffi::c_int,
        ),
    xClose:  Some(
            porterClose
                as unsafe extern "C" fn(*mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor) -> ::core::ffi::c_int,
        ),
    xNext:  Some(
            porterNext
                as unsafe extern "C" fn(
                    *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
                    *mut *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xLanguageid:  None,
}
};
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3PorterTokenizerModule(
    mut ppModule: *mut *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module,
) {
    *ppModule = &raw const porterTokenizerModule;
}
