pub mod geopoly_c {
    use crate::src::src::printf::sqlite3_str_vappendf2;

    pub static mut geopolyIsSpace: [::core::ffi::c_char; 256] = [
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
    ];

    pub unsafe extern "C" fn geopolySwab32(a: *mut ::core::ffi::c_uchar) {
        let mut t: ::core::ffi::c_uchar = *a.offset(0_isize);
        *a.offset(0_isize) = *a.offset(3_isize);
        *a.offset(3_isize) = t;
        t = *a.offset(1_isize);
        *a.offset(1_isize) = *a.offset(2_isize);
        *a.offset(2_isize) = t;
    }

    pub unsafe extern "C" fn geopolySkipSpace(
        p: *mut crate::geopoly_c::GeoParse,
    ) -> ::core::ffi::c_char {
        while geopolyIsSpace[*(*p).z.offset(0_isize) as usize] != 0 {
            (*p).z = (*p).z.offset(1);
        }
        *(*p).z.offset(0_isize) as ::core::ffi::c_char
    }

    pub unsafe extern "C" fn geopolyParseNumber(
        p: *mut crate::geopoly_c::GeoParse,
        pVal: *mut crate::geopoly_c::GeoCoord,
    ) -> ::core::ffi::c_int {
        let mut c: ::core::ffi::c_char = geopolySkipSpace(p);
        let __p_ref = { &mut *p };
        let z: *const ::core::ffi::c_uchar = __p_ref.z;
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut seenDP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut seenE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if c as ::core::ffi::c_int == '-' as i32 {
            j = 1 as ::core::ffi::c_int;
            c = *z.offset(j as isize) as ::core::ffi::c_char;
        }
        if c as ::core::ffi::c_int == '0' as i32
            && *z.offset((j + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int >= '0' as i32
            && *z.offset((j + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int <= '9' as i32
        {
            return 0 as ::core::ffi::c_int;
        }
        loop {
            c = *z.offset(j as isize) as ::core::ffi::c_char;
            if (*(*crate::src::headers::stdlib::__ctype_b_loc())
                .offset(c as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & crate::src::headers::stdlib::_ISdigit as ::core::ffi::c_int
                    as ::core::ffi::c_ushort as ::core::ffi::c_int == 0)
            {
                if c as ::core::ffi::c_int == '.' as i32 {
                    if *z.offset((j - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                        == '-' as i32
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                    if seenDP != 0 {
                        return 0 as ::core::ffi::c_int;
                    }
                    seenDP = 1 as ::core::ffi::c_int;
                } else {
                    if !(c as ::core::ffi::c_int == 'e' as i32
                        || c as ::core::ffi::c_int == 'E' as i32)
                    {
                        break;
                    }
                    if (*z.offset((j - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
                        < '0' as i32
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                    if seenE != 0 {
                        return -(1 as ::core::ffi::c_int);
                    }
                    seenE = 1 as ::core::ffi::c_int;
                    seenDP = seenE;
                    c = *z.offset((j + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_char;
                    if c as ::core::ffi::c_int == '+' as i32
                        || c as ::core::ffi::c_int == '-' as i32
                    {
                        j += 1;
                        c = *z.offset((j + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_char;
                    }
                    if (c as ::core::ffi::c_int) < '0' as i32
                        || c as ::core::ffi::c_int > '9' as i32
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
            j += 1;
        }
        if (*z.offset((j - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int) < '0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
        if !pVal.is_null() {
            *pVal = atof(__p_ref.z as *const ::core::ffi::c_char) as crate::geopoly_c::GeoCoord;
        }
        __p_ref.z = __p_ref.z.offset(j as isize);
        1 as ::core::ffi::c_int
    }

    pub unsafe extern "C" fn geopolyParseJson(
        z: *const ::core::ffi::c_uchar,
        pRc: *mut ::core::ffi::c_int,
    ) -> *mut crate::geopoly_c::GeoPoly {
        let current_block: u64;
        let mut s: crate::geopoly_c::GeoParse = { ::core::mem::zeroed() };
        let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
        s.z = z;
        if geopolySkipSpace(&raw mut s) as ::core::ffi::c_int == '[' as i32 {
            s.z = s.z.offset(1);
            's_17: loop {
                if (geopolySkipSpace(&raw mut s) as ::core::ffi::c_int != '[' as i32) {
                    current_block = 18317007320854588510;
                    break;
                }
                let mut ii: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut c: ::core::ffi::c_char;
                s.z = s.z.offset(1);
                if s.nVertex >= s.nAlloc {
                    
                    s.nAlloc = s.nAlloc * 2 as ::core::ffi::c_int + 16 as ::core::ffi::c_int;
                    let aNew: *mut crate::geopoly_c::GeoCoord = crate::src::src::malloc::sqlite3_realloc64(
                        s.a as *mut ::core::ffi::c_void,
                        (s.nAlloc as usize)
                            .wrapping_mul(
                                ::core::mem::size_of::<crate::geopoly_c::GeoCoord>() as usize
                            )
                            .wrapping_mul(2_usize)
                            as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                    ) as *mut crate::geopoly_c::GeoCoord;
                    if aNew.is_null() {
                        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                        s.nErr += 1;
                        current_block = 18317007320854588510;
                        break;
                    } else {
                        s.a = aNew;
                    }
                }
                while geopolyParseNumber(
                    &raw mut s,
                    if ii <= 1 as ::core::ffi::c_int {
                        s.a.offset((s.nVertex * 2 as ::core::ffi::c_int + ii) as isize)
                            as *mut crate::geopoly_c::GeoCoord
                    } else {
                        ::core::ptr::null_mut::<crate::geopoly_c::GeoCoord>()
                    },
                ) != 0
                {
                    ii += 1;
                    if ii == 2 as ::core::ffi::c_int {
                        s.nVertex += 1;
                    }
                    c = geopolySkipSpace(&raw mut s);
                    s.z = s.z.offset(1);
                    if c as ::core::ffi::c_int == ',' as i32 {
                        continue;
                    }
                    if c as ::core::ffi::c_int == ']' as i32 && ii >= 2 as ::core::ffi::c_int {
                        break;
                    }
                    s.nErr += 1;
                    rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                    current_block = 16754235392836356363;
                    break 's_17;
                }
                if (geopolySkipSpace(&raw mut s) as ::core::ffi::c_int != ',' as i32) {
                    current_block = 18317007320854588510;
                    break;
                }
                s.z = s.z.offset(1);
            }
            match current_block {
                16754235392836356363 => {}
                _ => {
                    if geopolySkipSpace(&raw mut s) as ::core::ffi::c_int == ']' as i32
                        && s.nVertex >= 4 as ::core::ffi::c_int
                        && *s.a.offset(0_isize)
                            == *s.a.offset(
                                (s.nVertex * 2 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                                    as isize,
                            )
                        && *s.a.offset(1_isize)
                            == *s.a.offset(
                                (s.nVertex * 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as isize,
                            )
                        && {
                            s.z = s.z.offset(1);
                            geopolySkipSpace(&raw mut s) as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        }
                    {
                        
                        let mut x: ::core::ffi::c_int;
                        s.nVertex -= 1;
                        let pOut: *mut crate::geopoly_c::GeoPoly = crate::src::src::malloc::sqlite3_malloc64(
                            (::core::mem::size_of::<crate::geopoly_c::GeoPoly>() as crate::src::headers::sqlite3_h::Sqlite3Uint64).wrapping_add(
                                ((::core::mem::size_of::<crate::geopoly_c::GeoCoord>() as usize)
                                    .wrapping_mul(2_usize)
                                    as crate::src::headers::sqlite3_h::Sqlite3Uint64)
                                    .wrapping_mul(
                                        (s.nVertex as crate::src::headers::sqlite3_h::Sqlite3Int64 - 4 as crate::src::headers::sqlite3_h::Sqlite3Int64)
                                            as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                                    ),
                            ),
                        ) as *mut crate::geopoly_c::GeoPoly;
                        x = 1 as ::core::ffi::c_int;
                        if !pOut.is_null() {
                            let __pOut_ref = { &mut *pOut };
                            __pOut_ref.nVertex = s.nVertex;
                            ::core::ptr::copy_nonoverlapping(
                                s.a as *const u8,
                                &raw mut __pOut_ref.a as *mut crate::geopoly_c::GeoCoord as *mut u8,
                                (((s.nVertex * 2 as ::core::ffi::c_int)
                                    as crate::__stddef_size_t_h::SizeT)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<crate::geopoly_c::GeoCoord>()
                                            as crate::__stddef_size_t_h::SizeT,
                                    )) as usize,
                            );
                            __pOut_ref.hdr[0 as ::core::ffi::c_int as usize] =
                                *(&raw mut x as *mut ::core::ffi::c_uchar);
                            __pOut_ref.hdr[1 as ::core::ffi::c_int as usize] = (s.nVertex
                                >> 16 as ::core::ffi::c_int
                                & 0xff as ::core::ffi::c_int)
                                as ::core::ffi::c_uchar;
                            __pOut_ref.hdr[2 as ::core::ffi::c_int as usize] =
                                (s.nVertex >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                                    as ::core::ffi::c_uchar;
                            __pOut_ref.hdr[3 as ::core::ffi::c_int as usize] =
                                (s.nVertex & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                            crate::src::src::malloc::sqlite3_free(s.a as *mut ::core::ffi::c_void);
                            if !pRc.is_null() {
                                *pRc = crate::src::headers::sqlite3_h::SQLITE_OK;
                            }
                            return pOut;
                        }
                    } else {
                        s.nErr += 1;
                        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                    }
                }
            }
        }
        if !pRc.is_null() {
            *pRc = rc;
        }
        crate::src::src::malloc::sqlite3_free(s.a as *mut ::core::ffi::c_void);
        ::core::ptr::null_mut::<crate::geopoly_c::GeoPoly>()
    }

    pub unsafe extern "C" fn geopolyFuncParam(
        pCtx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        pRc: *mut ::core::ffi::c_int,
    ) -> *mut crate::geopoly_c::GeoPoly {
        let mut p: *mut crate::geopoly_c::GeoPoly =
            ::core::ptr::null_mut::<crate::geopoly_c::GeoPoly>();
        let nByte: ::core::ffi::c_int;
        if crate::src::src::vdbeapi::sqlite3_value_type(pVal)
            == crate::src::headers::sqlite3_h::SQLITE_BLOB
            && {
                nByte = crate::src::src::vdbeapi::sqlite3_value_bytes(pVal);
                nByte
                    >= 4_usize.wrapping_add(
                        6_usize.wrapping_mul(
                            ::core::mem::size_of::<crate::geopoly_c::GeoCoord>() as usize,
                        ),
                    ) as ::core::ffi::c_int
            }
        {
            let a: *const ::core::ffi::c_uchar =
                crate::src::src::vdbeapi::sqlite3_value_blob(pVal) as *const ::core::ffi::c_uchar;
            
            if a.is_null() {
                if !pCtx.is_null() {
                    crate::src::src::vdbeapi::sqlite3_result_error_nomem(pCtx);
                }
                return ::core::ptr::null_mut::<crate::geopoly_c::GeoPoly>();
            }
            let nVertex: ::core::ffi::c_int = ((*a.offset(1_isize) as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                + ((*a.offset(2_isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                + *a.offset(3_isize) as ::core::ffi::c_int;
            if (*a.offset(0_isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || *a.offset(0_isize) as ::core::ffi::c_int == 1 as ::core::ffi::c_int)
                && ((nVertex * 2 as ::core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<crate::geopoly_c::GeoCoord>() as usize)
                    .wrapping_add(4_usize)
                    == nByte as ::core::ffi::c_uint as usize
            {
                p = crate::src::src::malloc::sqlite3_malloc64(
                    (::core::mem::size_of::<crate::geopoly_c::GeoPoly>() as usize).wrapping_add(
                        (((nVertex - 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int) as usize)
                            .wrapping_mul(
                                ::core::mem::size_of::<crate::geopoly_c::GeoCoord>() as usize
                            ),
                    ) as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                ) as *mut crate::geopoly_c::GeoPoly;
                if p.is_null() {
                    if !pRc.is_null() {
                        *pRc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                    }
                    if !pCtx.is_null() {
                        crate::src::src::vdbeapi::sqlite3_result_error_nomem(pCtx);
                    }
                } else {
                    let mut x: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    (*p).nVertex = nVertex;
                    ::core::ptr::copy_nonoverlapping(
                        a as *const u8,
                        &raw mut (*p).hdr as *mut ::core::ffi::c_uchar as *mut u8,
                        nByte as usize,
                    );
                    if *a.offset(0_isize) as ::core::ffi::c_int
                        != *(&raw mut x as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int
                    {
                        let mut ii: ::core::ffi::c_int;
                        ii = 0 as ::core::ffi::c_int;
                        while ii < nVertex {
                            geopolySwab32(
                                (&raw mut (*p).a as *mut crate::geopoly_c::GeoCoord)
                                    .offset((ii * 2 as ::core::ffi::c_int) as isize)
                                    as *mut crate::geopoly_c::GeoCoord
                                    as *mut ::core::ffi::c_uchar,
                            );
                            geopolySwab32(
                                (&raw mut (*p).a as *mut crate::geopoly_c::GeoCoord).offset(
                                    (ii * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut crate::geopoly_c::GeoCoord
                                    as *mut ::core::ffi::c_uchar,
                            );
                            ii += 1;
                        }
                        (*p).hdr[0 as ::core::ffi::c_int as usize] =
                            ((*p).hdr[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                                ^ 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_uchar;
                    }
                }
            }
            if !pRc.is_null() {
                *pRc = crate::src::headers::sqlite3_h::SQLITE_OK;
            }
            p
        } else if crate::src::src::vdbeapi::sqlite3_value_type(pVal)
            == crate::src::headers::sqlite3_h::SQLITE_TEXT
        {
            let zJson: *const ::core::ffi::c_uchar =
                crate::src::src::vdbeapi::sqlite3_value_text(pVal);
            if zJson.is_null() {
                if !pRc.is_null() {
                    *pRc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                }
                return ::core::ptr::null_mut::<crate::geopoly_c::GeoPoly>();
            }
            geopolyParseJson(zJson, pRc)
        } else {
            if !pRc.is_null() {
                *pRc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
            }
            ::core::ptr::null_mut::<crate::geopoly_c::GeoPoly>()
        }
    }

    pub unsafe extern "C" fn geopolyBlobFunc(
        context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        mut _argc: ::core::ffi::c_int,
        argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    ) {
        let p: *mut crate::geopoly_c::GeoPoly = geopolyFuncParam(
            context,
            *argv.offset(0_isize),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if !p.is_null() {
            crate::src::src::vdbeapi::sqlite3_result_blob(
                context,
                &raw mut (*p).hdr as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * (*p).nVertex,
                ::core::mem::transmute::<
                    crate::src::headers::stdlib::IntptrT,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
            );
            crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }

    pub unsafe extern "C" fn geopolyJsonFunc(
        context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        mut _argc: ::core::ffi::c_int,
        argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    ) {
        let p: *mut crate::geopoly_c::GeoPoly = geopolyFuncParam(
            context,
            *argv.offset(0_isize),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if !p.is_null() {
            let db: *mut crate::src::headers::sqliteInt_h::sqlite3 =
                crate::src::src::vdbeapi::sqlite3_context_db_handle(context);
            let x: *mut crate::src::headers::sqliteInt_h::sqlite3_str =
                crate::src::src::printf::sqlite3_str_new(db);
            let mut i: ::core::ffi::c_int;
            crate::src::src::printf::sqlite3_str_append(
                x,
                b"[\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
            i = 0 as ::core::ffi::c_int;
            let __p_ref = { &mut *p };
            while i < __p_ref.nVertex {
                sqlite3_str_vappendf2(
                    x,
                    "[%!g,%!g],",
                    crate::printf_args!(
                        *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                            .offset((i * 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_double,
                        *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord).offset(
                            (i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize
                        ) as ::core::ffi::c_double,
                    ),
                );
                i += 1;
            }
            sqlite3_str_vappendf2(
                x,
                "[%!g,%!g]]",
                crate::printf_args!(
                    *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                        .offset((0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_double,
                    *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord).offset(
                        (0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_double,
                ),
            );
            crate::src::src::vdbeapi::sqlite3_result_text(
                context,
                crate::src::src::printf::sqlite3_str_finish(x),
                -(1 as ::core::ffi::c_int),
                Some(
                    crate::src::src::malloc::sqlite3_free
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
            crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }

    pub unsafe extern "C" fn geopolySvgFunc(
        context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        argc: ::core::ffi::c_int,
        argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    ) {
        
        if argc < 1 as ::core::ffi::c_int {
            return;
        }
        let p: *mut crate::geopoly_c::GeoPoly = geopolyFuncParam(
            context,
            *argv.offset(0_isize),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if !p.is_null() {
            let db: *mut crate::src::headers::sqliteInt_h::sqlite3 =
                crate::src::src::vdbeapi::sqlite3_context_db_handle(context);
            let x: *mut crate::src::headers::sqliteInt_h::sqlite3_str =
                crate::src::src::printf::sqlite3_str_new(db);
            let mut i: ::core::ffi::c_int;
            let mut cSep: ::core::ffi::c_char = '\'' as i32 as ::core::ffi::c_char;
            sqlite3_str_vappendf2(x, "<polyline points=", crate::printf_args!());
            i = 0 as ::core::ffi::c_int;
            let __p_ref = { &mut *p };
            while i < __p_ref.nVertex {
                sqlite3_str_vappendf2(
                    x,
                    "%c%g,%g",
                    crate::printf_args!(
                        cSep as ::core::ffi::c_int,
                        *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                            .offset((i * 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_double,
                        *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord).offset(
                            (i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize
                        ) as ::core::ffi::c_double,
                    ),
                );
                cSep = ' ' as i32 as ::core::ffi::c_char;
                i += 1;
            }
            sqlite3_str_vappendf2(
                x,
                " %g,%g'",
                crate::printf_args!(
                    *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                        .offset((0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_double,
                    *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord).offset(
                        (0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_double,
                ),
            );
            i = 1 as ::core::ffi::c_int;
            while i < argc {
                let z: *const ::core::ffi::c_char =
                    crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(i as isize))
                        as *const ::core::ffi::c_char;
                if !z.is_null() && *z.offset(0_isize) as ::core::ffi::c_int != 0 {
                    sqlite3_str_vappendf2(x, " %s", crate::printf_args!(z));
                }
                i += 1;
            }
            sqlite3_str_vappendf2(x, "></polyline>", crate::printf_args!());
            crate::src::src::vdbeapi::sqlite3_result_text(
                context,
                crate::src::src::printf::sqlite3_str_finish(x),
                -(1 as ::core::ffi::c_int),
                Some(
                    crate::src::src::malloc::sqlite3_free
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
            crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }

    pub unsafe extern "C" fn geopolyXformFunc(
        context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        mut _argc: ::core::ffi::c_int,
        argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    ) {
        let p: *mut crate::geopoly_c::GeoPoly = geopolyFuncParam(
            context,
            *argv.offset(0_isize),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        let A: ::core::ffi::c_double =
            crate::src::src::vdbeapi::sqlite3_value_double(*argv.offset(1_isize));
        let B: ::core::ffi::c_double =
            crate::src::src::vdbeapi::sqlite3_value_double(*argv.offset(2_isize));
        let C: ::core::ffi::c_double =
            crate::src::src::vdbeapi::sqlite3_value_double(*argv.offset(3_isize));
        let D: ::core::ffi::c_double =
            crate::src::src::vdbeapi::sqlite3_value_double(*argv.offset(4_isize));
        let E: ::core::ffi::c_double =
            crate::src::src::vdbeapi::sqlite3_value_double(*argv.offset(5_isize));
        let F: ::core::ffi::c_double =
            crate::src::src::vdbeapi::sqlite3_value_double(*argv.offset(6_isize));
        let mut x1: crate::geopoly_c::GeoCoord;
        let mut y1: crate::geopoly_c::GeoCoord;
        let mut x0: crate::geopoly_c::GeoCoord;
        let mut y0: crate::geopoly_c::GeoCoord;
        let mut ii: ::core::ffi::c_int;
        if !p.is_null() {
            ii = 0 as ::core::ffi::c_int;
            let __p_ref = { &mut *p };
            while ii < __p_ref.nVertex {
                x0 = *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                    .offset((ii * 2 as ::core::ffi::c_int) as isize);
                y0 = *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                    .offset((ii * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
                x1 = (A * x0 as ::core::ffi::c_double + B * y0 as ::core::ffi::c_double + E)
                    as crate::geopoly_c::GeoCoord;
                y1 = (C * x0 as ::core::ffi::c_double + D * y0 as ::core::ffi::c_double + F)
                    as crate::geopoly_c::GeoCoord;
                *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                    .offset((ii * 2 as ::core::ffi::c_int) as isize) = x1;
                *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                    .offset((ii * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) = y1;
                ii += 1;
            }
            crate::src::src::vdbeapi::sqlite3_result_blob(
                context,
                &raw mut __p_ref.hdr as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * __p_ref.nVertex,
                ::core::mem::transmute::<
                    crate::src::headers::stdlib::IntptrT,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
            );
            crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }

    pub unsafe extern "C" fn geopolyArea(
        p: *mut crate::geopoly_c::GeoPoly,
    ) -> ::core::ffi::c_double {
        let mut rArea: ::core::ffi::c_double = 0.0f64;
        let mut ii: ::core::ffi::c_int;
        ii = 0 as ::core::ffi::c_int;
        let __p_ref = { &mut *p };
        while ii < __p_ref.nVertex - 1 as ::core::ffi::c_int {
            rArea += ((*(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                .offset((ii * 2 as ::core::ffi::c_int) as isize)
                - *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                    .offset(((ii + 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int) as isize))
                * (*(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                    .offset((ii * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    + *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord).offset(
                        ((ii + 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as isize,
                    ))) as ::core::ffi::c_double
                * 0.5f64;
            ii += 1;
        }
        rArea += ((*(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
            .offset((ii * 2 as ::core::ffi::c_int) as isize)
            - *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                .offset((0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize))
            * (*(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                .offset((ii * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                + *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord).offset(
                    (0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as isize,
                ))) as ::core::ffi::c_double
            * 0.5f64;
        rArea
    }

    pub unsafe extern "C" fn geopolyAreaFunc(
        context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        mut _argc: ::core::ffi::c_int,
        argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    ) {
        let p: *mut crate::geopoly_c::GeoPoly = geopolyFuncParam(
            context,
            *argv.offset(0_isize),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if !p.is_null() {
            crate::src::src::vdbeapi::sqlite3_result_double(context, geopolyArea(p));
            crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }

    pub unsafe extern "C" fn geopolyCcwFunc(
        context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        mut _argc: ::core::ffi::c_int,
        argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    ) {
        let p: *mut crate::geopoly_c::GeoPoly = geopolyFuncParam(
            context,
            *argv.offset(0_isize),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if !p.is_null() {
            if geopolyArea(p) < 0.0f64 {
                let mut ii: ::core::ffi::c_int;
                let mut jj: ::core::ffi::c_int;
                ii = 1 as ::core::ffi::c_int;
                jj = (*p).nVertex - 1 as ::core::ffi::c_int;
                while ii < jj {
                    let __p_ref = { &mut *p };
                    let mut t: crate::geopoly_c::GeoCoord = *(&raw mut __p_ref.a
                        as *mut crate::geopoly_c::GeoCoord)
                        .offset((ii * 2 as ::core::ffi::c_int) as isize);
                    *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                        .offset((ii * 2 as ::core::ffi::c_int) as isize) = *(&raw mut __p_ref.a
                        as *mut crate::geopoly_c::GeoCoord)
                        .offset((jj * 2 as ::core::ffi::c_int) as isize);
                    *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                        .offset((jj * 2 as ::core::ffi::c_int) as isize) = t;
                    t = *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                        .offset((ii * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
                    *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord).offset(
                        (ii * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                    ) = *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                        .offset((jj * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
                    *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord).offset(
                        (jj * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                    ) = t;
                    ii += 1;
                    jj -= 1;
                }
            }
            crate::src::src::vdbeapi::sqlite3_result_blob(
                context,
                &raw mut (*p).hdr as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * (*p).nVertex,
                ::core::mem::transmute::<
                    crate::src::headers::stdlib::IntptrT,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
            );
            crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }

    pub unsafe extern "C" fn geopolySine(mut r: ::core::ffi::c_double) -> ::core::ffi::c_double {
        if r >= 1.5f64 * crate::geopoly_c::GEOPOLY_PI {
            r -= 2.0f64 * crate::geopoly_c::GEOPOLY_PI;
        }
        if r >= 0.5f64 * crate::geopoly_c::GEOPOLY_PI {
            -geopolySine(r - crate::geopoly_c::GEOPOLY_PI)
        } else {
            let r2: ::core::ffi::c_double = r * r;
            let r3: ::core::ffi::c_double = r2 * r;
            let r5: ::core::ffi::c_double = r3 * r2;
            0.9996949f64 * r - 0.1656700f64 * r3 + 0.0075134f64 * r5
        }
    }

    pub unsafe extern "C" fn geopolyRegularFunc(
        context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        mut _argc: ::core::ffi::c_int,
        argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    ) {
        let x: ::core::ffi::c_double =
            crate::src::src::vdbeapi::sqlite3_value_double(*argv.offset(0_isize));
        let y: ::core::ffi::c_double =
            crate::src::src::vdbeapi::sqlite3_value_double(*argv.offset(1_isize));
        let r: ::core::ffi::c_double =
            crate::src::src::vdbeapi::sqlite3_value_double(*argv.offset(2_isize));
        let mut n: ::core::ffi::c_int =
            crate::src::src::vdbeapi::sqlite3_value_int(*argv.offset(3_isize));
        let mut i: ::core::ffi::c_int;
        
        if n < 3 as ::core::ffi::c_int || r <= 0.0f64 {
            return;
        }
        if n > 1000 as ::core::ffi::c_int {
            n = 1000 as ::core::ffi::c_int;
        }
        let p: *mut crate::geopoly_c::GeoPoly = crate::src::src::malloc::sqlite3_malloc64(
            (::core::mem::size_of::<crate::geopoly_c::GeoPoly>() as usize).wrapping_add(
                (((n - 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<crate::geopoly_c::GeoCoord>() as usize),
            ) as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut crate::geopoly_c::GeoPoly;
        if p.is_null() {
            crate::src::src::vdbeapi::sqlite3_result_error_nomem(context);
            return;
        }
        i = 1 as ::core::ffi::c_int;
        (*p).hdr[0 as ::core::ffi::c_int as usize] = *(&raw mut i as *mut ::core::ffi::c_uchar);
        (*p).hdr[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
        (*p).hdr[2 as ::core::ffi::c_int as usize] =
            (n >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        (*p).hdr[3 as ::core::ffi::c_int as usize] =
            (n & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        i = 0 as ::core::ffi::c_int;
        while i < n {
            let rAngle: ::core::ffi::c_double =
                2.0f64 * crate::geopoly_c::GEOPOLY_PI * i as ::core::ffi::c_double
                    / n as ::core::ffi::c_double;
            *(&raw mut (*p).a as *mut crate::geopoly_c::GeoCoord)
                .offset((i * 2 as ::core::ffi::c_int) as isize) =
                (x - r * geopolySine(rAngle - 0.5f64 * crate::geopoly_c::GEOPOLY_PI))
                    as crate::geopoly_c::GeoCoord;
            *(&raw mut (*p).a as *mut crate::geopoly_c::GeoCoord)
                .offset((i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                (y + r * geopolySine(rAngle)) as crate::geopoly_c::GeoCoord;
            i += 1;
        }
        crate::src::src::vdbeapi::sqlite3_result_blob(
            context,
            &raw mut (*p).hdr as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * n,
            ::core::mem::transmute::<
                crate::src::headers::stdlib::IntptrT,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
        );
        crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
    }

    pub unsafe extern "C" fn geopolyBBox(
        context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        pPoly: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        aCoord: *mut RtreeCoord,
        pRc: *mut ::core::ffi::c_int,
    ) -> *mut crate::geopoly_c::GeoPoly {
        let mut ii: ::core::ffi::c_int;
        let current_block: u64;
        let mut pOut: *mut crate::geopoly_c::GeoPoly =
            ::core::ptr::null_mut::<crate::geopoly_c::GeoPoly>();
        let p: *mut crate::geopoly_c::GeoPoly;
        let mut mnX: ::core::ffi::c_float = 0.;
        let mut mxX: ::core::ffi::c_float = 0.;
        let mut mnY: ::core::ffi::c_float = 0.;
        let mut mxY: ::core::ffi::c_float = 0.;
        if pPoly.is_null() && !aCoord.is_null() {
            p = ::core::ptr::null_mut::<crate::geopoly_c::GeoPoly>();
            mnX = (*aCoord.offset(0_isize)).f as ::core::ffi::c_float;
            mxX = (*aCoord.offset(1_isize)).f as ::core::ffi::c_float;
            mnY = (*aCoord.offset(2_isize)).f as ::core::ffi::c_float;
            mxY = (*aCoord.offset(3_isize)).f as ::core::ffi::c_float;
            current_block = 11512789485770614175;
        } else {
            p = geopolyFuncParam(context, pPoly, pRc);
            if !p.is_null() {
                let __p_ref = { &mut *p };
                mxX = *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                    .offset((0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_float;
                mnX = mxX;
                mxY = *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord).offset(
                    (0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as isize,
                ) as ::core::ffi::c_float;
                mnY = mxY;
                ii = 1 as ::core::ffi::c_int;
                while ii < __p_ref.nVertex {
                    let mut r: ::core::ffi::c_double = *(&raw mut __p_ref.a
                        as *mut crate::geopoly_c::GeoCoord)
                        .offset((ii * 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_double;
                    if r < mnX as ::core::ffi::c_double {
                        mnX = r as ::core::ffi::c_float;
                    } else if r > mxX as ::core::ffi::c_double {
                        mxX = r as ::core::ffi::c_float;
                    }
                    r = *(&raw mut __p_ref.a as *mut crate::geopoly_c::GeoCoord)
                        .offset((ii * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_double;
                    if r < mnY as ::core::ffi::c_double {
                        mnY = r as ::core::ffi::c_float;
                    } else if r > mxY as ::core::ffi::c_double {
                        mxY = r as ::core::ffi::c_float;
                    }
                    ii += 1;
                }
                if !pRc.is_null() {
                    *pRc = crate::src::headers::sqlite3_h::SQLITE_OK;
                }
                if aCoord.is_null() {
                    current_block = 11512789485770614175;
                } else {
                    crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
                    (*aCoord.offset(0_isize)).f = mnX as RtreeValue;
                    (*aCoord.offset(1_isize)).f = mxX as RtreeValue;
                    (*aCoord.offset(2_isize)).f = mnY as RtreeValue;
                    (*aCoord.offset(3_isize)).f = mxY as RtreeValue;
                    current_block = 6717214610478484138;
                }
            } else {
                if !aCoord.is_null() {
                    ::libc::memset(
                        aCoord as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (::core::mem::size_of::<RtreeCoord>() as crate::__stddef_size_t_h::SizeT)
                            .wrapping_mul(4 as crate::__stddef_size_t_h::SizeT),
                    );
                }
                current_block = 6717214610478484138;
            }
        }
        match current_block {
            11512789485770614175 => {
                pOut = crate::src::src::malloc::sqlite3_realloc64(
                    p as *mut ::core::ffi::c_void,
                    (::core::mem::size_of::<crate::geopoly_c::GeoPoly>() as usize).wrapping_add(
                        (::core::mem::size_of::<crate::geopoly_c::GeoCoord>() as usize)
                            .wrapping_mul(2_usize)
                            .wrapping_mul(
                                (4 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as usize,
                            ),
                    ) as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                ) as *mut crate::geopoly_c::GeoPoly;
                if pOut.is_null() {
                    crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
                    if !context.is_null() {
                        crate::src::src::vdbeapi::sqlite3_result_error_nomem(context);
                    }
                    if !pRc.is_null() {
                        *pRc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                    }
                    return ::core::ptr::null_mut::<crate::geopoly_c::GeoPoly>();
                }
                (*pOut).nVertex = 4 as ::core::ffi::c_int;
                ii = 1 as ::core::ffi::c_int;
                (*pOut).hdr[0 as ::core::ffi::c_int as usize] =
                    *(&raw mut ii as *mut ::core::ffi::c_uchar);
                (*pOut).hdr[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
                (*pOut).hdr[2 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
                (*pOut).hdr[3 as ::core::ffi::c_int as usize] = 4 as ::core::ffi::c_uchar;
                *(&raw mut (*pOut).a as *mut crate::geopoly_c::GeoCoord)
                    .offset((0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize) =
                    mnX as crate::geopoly_c::GeoCoord;
                *(&raw mut (*pOut).a as *mut crate::geopoly_c::GeoCoord).offset(
                    (0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as isize,
                ) = mnY as crate::geopoly_c::GeoCoord;
                *(&raw mut (*pOut).a as *mut crate::geopoly_c::GeoCoord)
                    .offset((1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize) =
                    mxX as crate::geopoly_c::GeoCoord;
                *(&raw mut (*pOut).a as *mut crate::geopoly_c::GeoCoord).offset(
                    (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as isize,
                ) = mnY as crate::geopoly_c::GeoCoord;
                *(&raw mut (*pOut).a as *mut crate::geopoly_c::GeoCoord)
                    .offset((2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize) =
                    mxX as crate::geopoly_c::GeoCoord;
                *(&raw mut (*pOut).a as *mut crate::geopoly_c::GeoCoord).offset(
                    (2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as isize,
                ) = mxY as crate::geopoly_c::GeoCoord;
                *(&raw mut (*pOut).a as *mut crate::geopoly_c::GeoCoord)
                    .offset((3 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize) =
                    mnX as crate::geopoly_c::GeoCoord;
                *(&raw mut (*pOut).a as *mut crate::geopoly_c::GeoCoord).offset(
                    (3 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as isize,
                ) = mxY as crate::geopoly_c::GeoCoord;
            }
            _ => {}
        }
        pOut
    }

    pub unsafe extern "C" fn geopolyBBoxFunc(
        context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        mut _argc: ::core::ffi::c_int,
        argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    ) {
        let p: *mut crate::geopoly_c::GeoPoly = geopolyBBox(
            context,
            *argv.offset(0_isize),
            ::core::ptr::null_mut::<RtreeCoord>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if !p.is_null() {
            crate::src::src::vdbeapi::sqlite3_result_blob(
                context,
                &raw mut (*p).hdr as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * (*p).nVertex,
                ::core::mem::transmute::<
                    crate::src::headers::stdlib::IntptrT,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
            );
            crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }

    pub unsafe extern "C" fn geopolyBBoxStep(
        context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        mut _argc: ::core::ffi::c_int,
        argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    ) {
        let mut a: [RtreeCoord; 4] = [RtreeCoord { f: 0. }; 4];
        let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
        geopolyBBox(
            context,
            *argv.offset(0_isize),
            &raw mut a as *mut RtreeCoord,
            &raw mut rc,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            
            let pBBox: *mut crate::geopoly_c::GeoBBox = crate::src::src::vdbeapi::sqlite3_aggregate_context(
                context,
                ::core::mem::size_of::<crate::geopoly_c::GeoBBox>() as ::core::ffi::c_int,
            ) as *mut crate::geopoly_c::GeoBBox;
            if pBBox.is_null() {
                return;
            }
            if (*pBBox).isInit == 0 as ::core::ffi::c_int {
                (*pBBox).isInit = 1 as ::core::ffi::c_int;
                ::core::ptr::copy_nonoverlapping(
                    &raw mut a as *mut RtreeCoord as *const u8,
                    &raw mut (*pBBox).a as *mut RtreeCoord as *mut u8,
                    ((::core::mem::size_of::<RtreeCoord>() as crate::__stddef_size_t_h::SizeT)
                        .wrapping_mul(4 as crate::__stddef_size_t_h::SizeT))
                        as usize,
                );
            } else {
                let __pBBox_ref = { &mut *pBBox };
                if a[0 as ::core::ffi::c_int as usize].f
                    < __pBBox_ref.a[0 as ::core::ffi::c_int as usize].f
                {
                    __pBBox_ref.a[0 as ::core::ffi::c_int as usize] =
                        a[0 as ::core::ffi::c_int as usize];
                }
                if a[1 as ::core::ffi::c_int as usize].f
                    > __pBBox_ref.a[1 as ::core::ffi::c_int as usize].f
                {
                    __pBBox_ref.a[1 as ::core::ffi::c_int as usize] =
                        a[1 as ::core::ffi::c_int as usize];
                }
                if a[2 as ::core::ffi::c_int as usize].f
                    < __pBBox_ref.a[2 as ::core::ffi::c_int as usize].f
                {
                    __pBBox_ref.a[2 as ::core::ffi::c_int as usize] =
                        a[2 as ::core::ffi::c_int as usize];
                }
                if a[3 as ::core::ffi::c_int as usize].f
                    > __pBBox_ref.a[3 as ::core::ffi::c_int as usize].f
                {
                    __pBBox_ref.a[3 as ::core::ffi::c_int as usize] =
                        a[3 as ::core::ffi::c_int as usize];
                }
            }
        }
    }

    pub unsafe extern "C" fn geopolyBBoxFinal(
        context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    ) {
        
        
        let pBBox: *mut crate::geopoly_c::GeoBBox = crate::src::src::vdbeapi::sqlite3_aggregate_context(context, 0 as ::core::ffi::c_int)
                as *mut crate::geopoly_c::GeoBBox;
        if pBBox.is_null() {
            return;
        }
        let p: *mut crate::geopoly_c::GeoPoly = geopolyBBox(
            context,
            ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>(),
            &raw mut (*pBBox).a as *mut RtreeCoord,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if !p.is_null() {
            crate::src::src::vdbeapi::sqlite3_result_blob(
                context,
                &raw mut (*p).hdr as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * (*p).nVertex,
                ::core::mem::transmute::<
                    crate::src::headers::stdlib::IntptrT,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
            );
            crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }

    pub unsafe extern "C" fn pointBeneathLine(
        x0: ::core::ffi::c_double,
        y0: ::core::ffi::c_double,
        x1: ::core::ffi::c_double,
        y1: ::core::ffi::c_double,
        x2: ::core::ffi::c_double,
        y2: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int {
        
        if x0 == x1 && y0 == y1 {
            return 2 as ::core::ffi::c_int;
        }
        if x1 < x2 {
            if x0 <= x1 || x0 > x2 {
                return 0 as ::core::ffi::c_int;
            }
        } else if x1 > x2 {
            if x0 <= x2 || x0 > x1 {
                return 0 as ::core::ffi::c_int;
            }
        } else {
            if x0 != x1 {
                return 0 as ::core::ffi::c_int;
            }
            if y0 < y1 && y0 < y2 {
                return 0 as ::core::ffi::c_int;
            }
            if y0 > y1 && y0 > y2 {
                return 0 as ::core::ffi::c_int;
            }
            return 2 as ::core::ffi::c_int;
        }
        let y: ::core::ffi::c_double = y1 + (y2 - y1) * (x0 - x1) / (x2 - x1);
        if y0 == y {
            return 2 as ::core::ffi::c_int;
        }
        if y0 < y {
            return 1 as ::core::ffi::c_int;
        }
        0 as ::core::ffi::c_int
    }

    pub unsafe extern "C" fn geopolyContainsPointFunc(
        context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        mut _argc: ::core::ffi::c_int,
        argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    ) {
        let p1: *mut crate::geopoly_c::GeoPoly = geopolyFuncParam(
            context,
            *argv.offset(0_isize),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        let x0: ::core::ffi::c_double =
            crate::src::src::vdbeapi::sqlite3_value_double(*argv.offset(1_isize));
        let y0: ::core::ffi::c_double =
            crate::src::src::vdbeapi::sqlite3_value_double(*argv.offset(2_isize));
        let mut v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut ii: ::core::ffi::c_int;
        if p1.is_null() {
            return;
        }
        ii = 0 as ::core::ffi::c_int;
        while ii < (*p1).nVertex - 1 as ::core::ffi::c_int {
            let __p1_ref = { &mut *p1 };
            v = pointBeneathLine(
                x0,
                y0,
                *(&raw mut __p1_ref.a as *mut crate::geopoly_c::GeoCoord)
                    .offset((ii * 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_double,
                *(&raw mut __p1_ref.a as *mut crate::geopoly_c::GeoCoord)
                    .offset((ii * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_double,
                *(&raw mut __p1_ref.a as *mut crate::geopoly_c::GeoCoord)
                    .offset(((ii + 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_double,
                *(&raw mut __p1_ref.a as *mut crate::geopoly_c::GeoCoord).offset(
                    ((ii + 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_double,
            );
            if v == 2 as ::core::ffi::c_int {
                break;
            }
            cnt += v;
            ii += 1;
        }
        if v != 2 as ::core::ffi::c_int {
            let __p1_ref = { &mut *p1 };
            v = pointBeneathLine(
                x0,
                y0,
                *(&raw mut __p1_ref.a as *mut crate::geopoly_c::GeoCoord)
                    .offset((ii * 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_double,
                *(&raw mut __p1_ref.a as *mut crate::geopoly_c::GeoCoord)
                    .offset((ii * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_double,
                *(&raw mut __p1_ref.a as *mut crate::geopoly_c::GeoCoord)
                    .offset((0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_double,
                *(&raw mut __p1_ref.a as *mut crate::geopoly_c::GeoCoord).offset(
                    (0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as isize,
                ) as ::core::ffi::c_double,
            );
        }
        if v == 2 as ::core::ffi::c_int {
            crate::src::src::vdbeapi::sqlite3_result_int(context, 1 as ::core::ffi::c_int);
        } else if v + cnt & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            crate::src::src::vdbeapi::sqlite3_result_int(context, 0 as ::core::ffi::c_int);
        } else {
            crate::src::src::vdbeapi::sqlite3_result_int(context, 2 as ::core::ffi::c_int);
        }
        crate::src::src::malloc::sqlite3_free(p1 as *mut ::core::ffi::c_void);
    }

    pub unsafe extern "C" fn geopolyWithinFunc(
        context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        mut _argc: ::core::ffi::c_int,
        argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    ) {
        let p1: *mut crate::geopoly_c::GeoPoly = geopolyFuncParam(
            context,
            *argv.offset(0_isize),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        let p2: *mut crate::geopoly_c::GeoPoly = geopolyFuncParam(
            context,
            *argv.offset(1_isize),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if !p1.is_null() && !p2.is_null() {
            let x: ::core::ffi::c_int = geopolyOverlap(p1, p2);
            if x < 0 as ::core::ffi::c_int {
                crate::src::src::vdbeapi::sqlite3_result_error_nomem(context);
            } else {
                crate::src::src::vdbeapi::sqlite3_result_int(
                    context,
                    if x == 2 as ::core::ffi::c_int {
                        1 as ::core::ffi::c_int
                    } else if x == 4 as ::core::ffi::c_int {
                        2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    },
                );
            }
        }
        crate::src::src::malloc::sqlite3_free(p1 as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(p2 as *mut ::core::ffi::c_void);
    }

    pub unsafe extern "C" fn geopolyAddOneSegment(
        p: *mut crate::geopoly_c::GeoOverlap,
        mut x0: crate::geopoly_c::GeoCoord,
        mut y0: crate::geopoly_c::GeoCoord,
        mut x1: crate::geopoly_c::GeoCoord,
        mut y1: crate::geopoly_c::GeoCoord,
        side: ::core::ffi::c_uchar,
        idx: ::core::ffi::c_uint,
    ) {
        
        let mut pEvent: *mut crate::geopoly_c::GeoEvent;
        if x0 == x1 {
            return;
        }
        if x0 > x1 {
            let mut t: crate::geopoly_c::GeoCoord = x0;
            x0 = x1;
            x1 = t;
            t = y0;
            y0 = y1;
            y1 = t;
        }
        let __p_ref = { &mut *p };
        let pSeg: *mut crate::geopoly_c::GeoSegment = __p_ref.aSegment.offset(__p_ref.nSegment as isize);
        __p_ref.nSegment += 1;
        (*pSeg).C = ((y1 - y0) / (x1 - x0)) as ::core::ffi::c_double;
        (*pSeg).B = y1 as ::core::ffi::c_double - x1 as ::core::ffi::c_double * (*pSeg).C;
        (*pSeg).y0 = y0 as ::core::ffi::c_float;
        (*pSeg).side = side;
        (*pSeg).idx = idx;
        pEvent = __p_ref.aEvent.offset(__p_ref.nEvent as isize);
        __p_ref.nEvent += 1;
        (*pEvent).x = x0 as ::core::ffi::c_double;
        (*pEvent).eType = 0 as ::core::ffi::c_int;
        (*pEvent).pSeg = pSeg;
        pEvent = __p_ref.aEvent.offset(__p_ref.nEvent as isize);
        __p_ref.nEvent += 1;
        (*pEvent).x = x1 as ::core::ffi::c_double;
        (*pEvent).eType = 1 as ::core::ffi::c_int;
        (*pEvent).pSeg = pSeg;
    }

    pub unsafe extern "C" fn geopolyAddSegments(
        p: *mut crate::geopoly_c::GeoOverlap,
        pPoly: *mut crate::geopoly_c::GeoPoly,
        side: ::core::ffi::c_uchar,
    ) {
        let mut i: ::core::ffi::c_uint;
        let mut x: *mut crate::geopoly_c::GeoCoord;
        i = 0 as ::core::ffi::c_uint;
        let __pPoly_ref = { &mut *pPoly };
        while i
            < (__pPoly_ref.nVertex as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
        {
            x = (&raw mut __pPoly_ref.a as *mut crate::geopoly_c::GeoCoord)
                .offset(i.wrapping_mul(2 as ::core::ffi::c_uint) as isize)
                as *mut crate::geopoly_c::GeoCoord;
            geopolyAddOneSegment(
                p,
                *x.offset(0_isize),
                *x.offset(1_isize),
                *x.offset(2_isize),
                *x.offset(3_isize),
                side,
                i,
            );
            i = i.wrapping_add(1);
        }
        x = (&raw mut __pPoly_ref.a as *mut crate::geopoly_c::GeoCoord)
            .offset(i.wrapping_mul(2 as ::core::ffi::c_uint) as isize)
            as *mut crate::geopoly_c::GeoCoord;
        geopolyAddOneSegment(
            p,
            *x.offset(0_isize),
            *x.offset(1_isize),
            __pPoly_ref.a[0 as ::core::ffi::c_int as usize],
            __pPoly_ref.a[1 as ::core::ffi::c_int as usize],
            side,
            i,
        );
    }

    pub unsafe extern "C" fn geopolyEventMerge(
        mut pLeft: *mut crate::geopoly_c::GeoEvent,
        mut pRight: *mut crate::geopoly_c::GeoEvent,
    ) -> *mut crate::geopoly_c::GeoEvent {
        let mut head: crate::geopoly_c::GeoEvent = crate::geopoly_c::GeoEvent {
            x: 0.,
            eType: 0,
            pSeg: ::core::ptr::null_mut::<crate::geopoly_c::GeoSegment>(),
            pNext: ::core::ptr::null_mut::<crate::geopoly_c::GeoEvent>(),
        };
        let mut pLast: *mut crate::geopoly_c::GeoEvent;
        head.pNext = ::core::ptr::null_mut::<crate::geopoly_c::GeoEvent>();
        pLast = &raw mut head;
        while !pRight.is_null() && !pLeft.is_null() {
            if (*pRight).x <= (*pLeft).x {
                (*pLast).pNext = pRight;
                pLast = pRight;
                pRight = (*pRight).pNext;
            } else {
                (*pLast).pNext = pLeft;
                pLast = pLeft;
                pLeft = (*pLeft).pNext;
            }
        }
        (*pLast).pNext = if !pRight.is_null() { pRight } else { pLeft };
        head.pNext
    }

    pub unsafe extern "C" fn geopolySortEventsByX(
        aEvent: *mut crate::geopoly_c::GeoEvent,
        nEvent: ::core::ffi::c_int,
    ) -> *mut crate::geopoly_c::GeoEvent {
        let mut mx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int;
        let mut j: ::core::ffi::c_int;
        let mut p: *mut crate::geopoly_c::GeoEvent;
        let mut a: [*mut crate::geopoly_c::GeoEvent; 50] =
            [::core::ptr::null_mut::<crate::geopoly_c::GeoEvent>(); 50];
        i = 0 as ::core::ffi::c_int;
        while i < nEvent {
            p = aEvent.offset(i as isize) as *mut crate::geopoly_c::GeoEvent;
            (*p).pNext = ::core::ptr::null_mut::<crate::geopoly_c::GeoEvent>();
            j = 0 as ::core::ffi::c_int;
            while j < mx && !a[j as usize].is_null() {
                p = geopolyEventMerge(a[j as usize], p);
                a[j as usize] = ::core::ptr::null_mut::<crate::geopoly_c::GeoEvent>();
                j += 1;
            }
            a[j as usize] = p;
            if j >= mx {
                mx = j + 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        p = ::core::ptr::null_mut::<crate::geopoly_c::GeoEvent>();
        i = 0 as ::core::ffi::c_int;
        while i < mx {
            p = geopolyEventMerge(a[i as usize], p);
            i += 1;
        }
        p
    }

    pub unsafe extern "C" fn geopolySegmentMerge(
        mut pLeft: *mut crate::geopoly_c::GeoSegment,
        mut pRight: *mut crate::geopoly_c::GeoSegment,
    ) -> *mut crate::geopoly_c::GeoSegment {
        let mut head: crate::geopoly_c::GeoSegment = crate::geopoly_c::GeoSegment {
            C: 0.,
            B: 0.,
            y: 0.,
            y0: 0.,
            side: 0,
            idx: 0,
            pNext: ::core::ptr::null_mut::<crate::geopoly_c::GeoSegment>(),
        };
        let mut pLast: *mut crate::geopoly_c::GeoSegment;
        head.pNext = ::core::ptr::null_mut::<crate::geopoly_c::GeoSegment>();
        pLast = &raw mut head;
        while !pRight.is_null() && !pLeft.is_null() {
            let mut r: ::core::ffi::c_double = (*pRight).y - (*pLeft).y;
            if r == 0.0f64 {
                r = (*pRight).C - (*pLeft).C;
            }
            if r < 0.0f64 {
                (*pLast).pNext = pRight;
                pLast = pRight;
                pRight = (*pRight).pNext;
            } else {
                (*pLast).pNext = pLeft;
                pLast = pLeft;
                pLeft = (*pLeft).pNext;
            }
        }
        (*pLast).pNext = if !pRight.is_null() { pRight } else { pLeft };
        head.pNext
    }

    pub unsafe extern "C" fn geopolySortSegmentsByYAndC(
        mut pList: *mut crate::geopoly_c::GeoSegment,
    ) -> *mut crate::geopoly_c::GeoSegment {
        let mut mx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int;
        let mut p: *mut crate::geopoly_c::GeoSegment;
        let mut a: [*mut crate::geopoly_c::GeoSegment; 50] =
            [::core::ptr::null_mut::<crate::geopoly_c::GeoSegment>(); 50];
        while !pList.is_null() {
            p = pList;
            pList = (*pList).pNext;
            (*p).pNext = ::core::ptr::null_mut::<crate::geopoly_c::GeoSegment>();
            i = 0 as ::core::ffi::c_int;
            while i < mx && !a[i as usize].is_null() {
                p = geopolySegmentMerge(a[i as usize], p);
                a[i as usize] = ::core::ptr::null_mut::<crate::geopoly_c::GeoSegment>();
                i += 1;
            }
            a[i as usize] = p;
            if i >= mx {
                mx = i + 1 as ::core::ffi::c_int;
            }
        }
        p = ::core::ptr::null_mut::<crate::geopoly_c::GeoSegment>();
        i = 0 as ::core::ffi::c_int;
        while i < mx {
            p = geopolySegmentMerge(a[i as usize], p);
            i += 1;
        }
        p
    }

    pub unsafe extern "C" fn geopolyOverlap(
        p1: *mut crate::geopoly_c::GeoPoly,
        p2: *mut crate::geopoly_c::GeoPoly,
    ) -> ::core::ffi::c_int {
        let current_block: u64;
        let nVertex: crate::src::headers::sqlite3_h::Sqlite3Int64 =
            ((*p1).nVertex + (*p2).nVertex + 2 as ::core::ffi::c_int)
                as crate::src::headers::sqlite3_h::Sqlite3Int64;
        
        
        let mut pThisEvent: *mut crate::geopoly_c::GeoEvent;
        let mut rX: ::core::ffi::c_double;
        let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut needSort: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pActive: *mut crate::geopoly_c::GeoSegment =
            ::core::ptr::null_mut::<crate::geopoly_c::GeoSegment>();
        let mut pSeg: *mut crate::geopoly_c::GeoSegment;
        let mut aOverlap: [::core::ffi::c_uchar; 4] = { ::core::mem::zeroed() };
        let nByte: crate::src::headers::sqlite3_h::Sqlite3Int64 = (::core::mem::size_of::<crate::geopoly_c::GeoEvent>() as ::core::ffi::c_ulonglong)
            .wrapping_mul(nVertex as ::core::ffi::c_ulonglong)
            .wrapping_mul(2 as ::core::ffi::c_ulonglong)
            .wrapping_add(
                (::core::mem::size_of::<crate::geopoly_c::GeoSegment>()
                    as ::core::ffi::c_ulonglong)
                    .wrapping_mul(nVertex as ::core::ffi::c_ulonglong),
            )
            .wrapping_add(
                ::core::mem::size_of::<crate::geopoly_c::GeoOverlap>() as ::core::ffi::c_ulonglong
            ) as crate::src::headers::sqlite3_h::Sqlite3Int64;
        let p: *mut crate::geopoly_c::GeoOverlap = crate::src::src::malloc::sqlite3_malloc64(
            nByte as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut crate::geopoly_c::GeoOverlap;
        if p.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        (*p).aEvent = p.offset(1_isize) as *mut crate::geopoly_c::GeoOverlap
            as *mut crate::geopoly_c::GeoEvent;
        (*p).aSegment =
            (*p).aEvent
                .offset((nVertex * 2 as crate::src::headers::sqlite3_h::Sqlite3Int64) as isize)
                as *mut crate::geopoly_c::GeoEvent as *mut crate::geopoly_c::GeoSegment;
        (*p).nSegment = 0 as ::core::ffi::c_int;
        (*p).nEvent = (*p).nSegment;
        geopolyAddSegments(p, p1, 1 as ::core::ffi::c_uchar);
        geopolyAddSegments(p, p2, 2 as ::core::ffi::c_uchar);
        pThisEvent = geopolySortEventsByX((*p).aEvent, (*p).nEvent);
        rX = if !pThisEvent.is_null() && (*pThisEvent).x == 0.0f64 {
            -1.0f64
        } else {
            0.0f64
        };
        's_58: loop {
            if pThisEvent.is_null() {
                current_block = 6072622540298447352;
                break;
            }
            if (*pThisEvent).x != rX {
                let mut pPrev: *mut crate::geopoly_c::GeoSegment =
                    ::core::ptr::null_mut::<crate::geopoly_c::GeoSegment>();
                let mut iMask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                rX = (*pThisEvent).x;
                if needSort != 0 {
                    pActive = geopolySortSegmentsByYAndC(pActive);
                    needSort = 0 as ::core::ffi::c_int;
                }
                pSeg = pActive;
                while !pSeg.is_null() {
                    if !pPrev.is_null() {
                        if (*pPrev).y != (*pSeg).y {
                            aOverlap[iMask as usize] = 1 as ::core::ffi::c_uchar;
                        }
                    }
                    iMask ^= (*pSeg).side as ::core::ffi::c_int;
                    pPrev = pSeg;
                    pSeg = (*pSeg).pNext;
                }
                pPrev = ::core::ptr::null_mut::<crate::geopoly_c::GeoSegment>();
                pSeg = pActive;
                while !pSeg.is_null() {
                    let y: ::core::ffi::c_double = (*pSeg).C * rX + (*pSeg).B;
                    (*pSeg).y = y;
                    if !pPrev.is_null() {
                        let __pSeg_ref = { &*pSeg };
                        let __pPrev_ref = { &*pPrev };
                        if __pPrev_ref.y > __pSeg_ref.y
                            && __pPrev_ref.side as ::core::ffi::c_int
                                != __pSeg_ref.side as ::core::ffi::c_int
                        {
                            rc = 1 as ::core::ffi::c_int;
                            current_block = 7007508497437696113;
                            break 's_58;
                        } else if __pPrev_ref.y != __pSeg_ref.y {
                            aOverlap[iMask as usize] = 1 as ::core::ffi::c_uchar;
                        }
                    }
                    iMask ^= (*pSeg).side as ::core::ffi::c_int;
                    pPrev = pSeg;
                    pSeg = (*pSeg).pNext;
                }
            }
            if (*pThisEvent).eType == 0 as ::core::ffi::c_int {
                pSeg = (*pThisEvent).pSeg;
                (*pSeg).y = (*pSeg).y0 as ::core::ffi::c_double;
                (*pSeg).pNext = pActive;
                pActive = pSeg;
                needSort = 1 as ::core::ffi::c_int;
            } else if pActive == (*pThisEvent).pSeg {
                pActive = if !pActive.is_null() {
                    (*pActive).pNext
                } else {
                    ::core::ptr::null_mut::<crate::geopoly_c::GeoSegment>()
                };
            } else {
                pSeg = pActive;
                while !pSeg.is_null() {
                    if (*pSeg).pNext == (*pThisEvent).pSeg {
                        (*pSeg).pNext = if !(*pSeg).pNext.is_null() {
                            (*(*pSeg).pNext).pNext
                        } else {
                            ::core::ptr::null_mut::<crate::geopoly_c::GeoSegment>()
                        };
                        break;
                    } else {
                        pSeg = (*pSeg).pNext;
                    }
                }
            }
            pThisEvent = (*pThisEvent).pNext;
        }
        match current_block {
            6072622540298447352 => {
                if aOverlap[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    rc = 0 as ::core::ffi::c_int;
                } else if aOverlap[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                    && aOverlap[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                {
                    rc = 3 as ::core::ffi::c_int;
                } else if aOverlap[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    && aOverlap[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                {
                    rc = 2 as ::core::ffi::c_int;
                } else if aOverlap[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    && aOverlap[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                {
                    rc = 4 as ::core::ffi::c_int;
                } else {
                    rc = 1 as ::core::ffi::c_int;
                }
            }
            _ => {}
        }
        crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
        rc
    }

    pub unsafe extern "C" fn geopolyOverlapFunc(
        context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        mut _argc: ::core::ffi::c_int,
        argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    ) {
        let p1: *mut crate::geopoly_c::GeoPoly = geopolyFuncParam(
            context,
            *argv.offset(0_isize),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        let p2: *mut crate::geopoly_c::GeoPoly = geopolyFuncParam(
            context,
            *argv.offset(1_isize),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if !p1.is_null() && !p2.is_null() {
            let x: ::core::ffi::c_int = geopolyOverlap(p1, p2);
            if x < 0 as ::core::ffi::c_int {
                crate::src::src::vdbeapi::sqlite3_result_error_nomem(context);
            } else {
                crate::src::src::vdbeapi::sqlite3_result_int(context, x);
            }
        }
        crate::src::src::malloc::sqlite3_free(p1 as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(p2 as *mut ::core::ffi::c_void);
    }

    pub unsafe extern "C" fn geopolyDebugFunc(
        mut _context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        mut _argc: ::core::ffi::c_int,
        mut _argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    ) {
    }

    pub unsafe extern "C" fn geopolyInit(
        db: *mut crate::src::headers::sqliteInt_h::sqlite3,
        mut _pAux: *mut ::core::ffi::c_void,
        argc: ::core::ffi::c_int,
        argv: *const *const ::core::ffi::c_char,
        ppVtab: *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
        pzErr: *mut *mut ::core::ffi::c_char,
        isCreate: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut rc: ::core::ffi::c_int;
        
        
        
        
        
        let mut ii: ::core::ffi::c_int;
        {
            let _vtab_args: [u64; 1] = [1 as ::core::ffi::c_int as u64];
            crate::src::src::vtab::sqlite3_vtab_config_args(
                db,
                crate::src::headers::sqlite3_h::SQLITE_VTAB_CONSTRAINT_SUPPORT,
                _vtab_args.as_ptr(),
            );
        }
        {
            let _vtab_args: [u64; 1] = [0];
            crate::src::src::vtab::sqlite3_vtab_config_args(
                db,
                crate::src::headers::sqlite3_h::SQLITE_VTAB_INNOCUOUS,
                _vtab_args.as_ptr(),
            );
        }
        let nDb: crate::src::headers::sqlite3_h::Sqlite3Int64 = ::libc::strlen(*argv.offset(1_isize))
            as crate::src::headers::sqlite3_h::Sqlite3Int64;
        let nName: crate::src::headers::sqlite3_h::Sqlite3Int64 = ::libc::strlen(*argv.offset(2_isize))
            as crate::src::headers::sqlite3_h::Sqlite3Int64;
        let pRtree: *mut Rtree = crate::src::src::malloc::sqlite3_malloc64(
            (::core::mem::size_of::<Rtree>() as crate::src::headers::sqlite3_h::Sqlite3Uint64)
                .wrapping_add(nDb as crate::src::headers::sqlite3_h::Sqlite3Uint64)
                .wrapping_add(
                    (nName * 2 as crate::src::headers::sqlite3_h::Sqlite3Int64)
                        as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                )
                .wrapping_add(8 as crate::src::headers::sqlite3_h::Sqlite3Uint64),
        ) as *mut Rtree;
        if pRtree.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        ::libc::memset(
            pRtree as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<Rtree>() as ::core::ffi::c_ulonglong)
                .wrapping_add(nDb as ::core::ffi::c_ulonglong)
                .wrapping_add(
                    (nName * 2 as crate::src::headers::sqlite3_h::Sqlite3Int64)
                        as ::core::ffi::c_ulonglong,
                )
                .wrapping_add(8 as ::core::ffi::c_ulonglong)
                as crate::__stddef_size_t_h::SizeT,
        );
        (*pRtree).nBusy = 1 as U32_0;
        (*pRtree).base.pModule = &raw mut rtreeModule;
        (*pRtree).zDb = pRtree.offset(1_isize) as *mut Rtree as *mut ::core::ffi::c_char;
        (*pRtree).zName = (*pRtree)
            .zDb
            .offset((nDb + 1 as crate::src::headers::sqlite3_h::Sqlite3Int64) as isize)
            as *mut ::core::ffi::c_char;
        (*pRtree).zNodeName = (*pRtree)
            .zName
            .offset((nName + 1 as crate::src::headers::sqlite3_h::Sqlite3Int64) as isize)
            as *mut ::core::ffi::c_char;
        (*pRtree).eCoordType = RTREE_COORD_REAL32 as U8_0;
        (*pRtree).nDim = 2 as U8_0;
        (*pRtree).nDim2 = 4 as U8_0;
        ::core::ptr::copy_nonoverlapping(
            *argv.offset(1_isize) as *const u8,
            (*pRtree).zDb as *mut u8,
            nDb as usize,
        );
        ::core::ptr::copy_nonoverlapping(
            *argv.offset(2_isize) as *const u8,
            (*pRtree).zName as *mut u8,
            nName as usize,
        );
        ::core::ptr::copy_nonoverlapping(
            *argv.offset(2_isize) as *const u8,
            (*pRtree).zNodeName as *mut u8,
            nName as usize,
        );
        ::core::ptr::copy_nonoverlapping(
            b"_node\0" as *const u8 as *const ::core::ffi::c_char,
            (*pRtree).zNodeName.offset(nName as isize) as *mut ::core::ffi::c_char,
            6_usize,
        );
        let pSql: *mut crate::src::headers::sqliteInt_h::sqlite3_str = crate::src::src::printf::sqlite3_str_new(db);
        sqlite3_str_vappendf2(pSql, "CREATE TABLE x(_shape", crate::printf_args!());
        (*pRtree).nAux = 1 as U8_0;
        (*pRtree).nAuxNotNull = 1 as U8_0;
        ii = 3 as ::core::ffi::c_int;
        while ii < argc {
            (*pRtree).nAux = (*pRtree).nAux.wrapping_add(1);
            sqlite3_str_vappendf2(pSql, ",%s", crate::printf_args!(*argv.offset(ii as isize)));
            ii += 1;
        }
        sqlite3_str_vappendf2(pSql, ");", crate::printf_args!());
        let zSql: *mut ::core::ffi::c_char = crate::src::src::printf::sqlite3_str_finish(pSql);
        if zSql.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            rc = crate::src::src::vtab::sqlite3_declare_vtab(db, zSql);
            if crate::src::headers::sqlite3_h::SQLITE_OK != rc {
                *pzErr = crate::sqlite_printf!("%s", crate::src::src::main::sqlite3_errmsg(db));
            }
        }
        crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
        if (rc == 0) {
            (*pRtree).nBytesPerCell = (8 as ::core::ffi::c_int
                + (*pRtree).nDim2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as U8_0;
            rc = getNodeSize(db, pRtree, isCreate, pzErr);
            if (rc == 0) {
                rc = rtreeSqlInit(
                    pRtree,
                    db,
                    *argv.offset(1_isize),
                    *argv.offset(2_isize),
                    isCreate,
                );
                if rc != 0 {
                    *pzErr = crate::sqlite_printf!("%s", crate::src::src::main::sqlite3_errmsg(db));
                } else {
                    *ppVtab = pRtree as *mut crate::src::headers::sqlite3_h::sqlite3_vtab;
                    return crate::src::headers::sqlite3_h::SQLITE_OK;
                }
            }
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
        }
        rtreeRelease(pRtree);
        rc
    }

    pub unsafe extern "C" fn geopolyCreate(
        db: *mut crate::src::headers::sqliteInt_h::sqlite3,
        pAux: *mut ::core::ffi::c_void,
        argc: ::core::ffi::c_int,
        argv: *const *const ::core::ffi::c_char,
        ppVtab: *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
        pzErr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        geopolyInit(db, pAux, argc, argv, ppVtab, pzErr, 1 as ::core::ffi::c_int)
    }

    pub unsafe extern "C" fn geopolyConnect(
        db: *mut crate::src::headers::sqliteInt_h::sqlite3,
        pAux: *mut ::core::ffi::c_void,
        argc: ::core::ffi::c_int,
        argv: *const *const ::core::ffi::c_char,
        ppVtab: *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
        pzErr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        geopolyInit(db, pAux, argc, argv, ppVtab, pzErr, 0 as ::core::ffi::c_int)
    }

    pub unsafe extern "C" fn geopolyFilter(
        pVtabCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
        idxNum: ::core::ffi::c_int,
        mut _idxStr: *const ::core::ffi::c_char,
        mut _argc: ::core::ffi::c_int,
        argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    ) -> ::core::ffi::c_int {
        let current_block: u64;
        let pRtree: *mut Rtree = (*pVtabCursor).pVtab as *mut Rtree;
        let pCsr: *mut RtreeCursor = pVtabCursor as *mut RtreeCursor;
        let mut pRoot: *mut RtreeNode = ::core::ptr::null_mut::<RtreeNode>();
        let mut rc: ::core::ffi::c_int;
        let mut iCell: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        rtreeReference(pRtree);
        resetCursor(pCsr);
        (*pCsr).iStrategy = idxNum;
        if idxNum == 1 as ::core::ffi::c_int {
            let mut pLeaf: *mut RtreeNode = ::core::ptr::null_mut::<RtreeNode>();
            let p: *mut RtreeSearchPoint;
            let iRowid: I64_0 =
                crate::src::src::vdbeapi::sqlite3_value_int64(*argv.offset(0_isize)) as I64_0;
            let mut iNode: I64_0 = 0 as I64_0;
            rc = findLeafNode(pRtree, iRowid, &raw mut pLeaf, &raw mut iNode);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !pLeaf.is_null() {
                p = rtreeSearchPointNew(pCsr, RTREE_ZERO, 0 as U8_0);
                (*pCsr).aNode[0 as ::core::ffi::c_int as usize] = pLeaf;
                (*p).id = iNode as crate::src::headers::sqlite3_h::Sqlite3Int64;
                (*p).eWithin = crate::src::headers::sqlite3_h::PARTLY_WITHIN as U8_0;
                rc = nodeRowidIndex(pRtree, pLeaf, iRowid, &raw mut iCell);
                (*p).iCell = iCell as U8_0;
            } else {
                (*pCsr).atEOF = 1 as U8_0;
            }
        } else {
            rc = nodeAcquire(
                pRtree,
                1 as I64_0,
                ::core::ptr::null_mut::<RtreeNode>(),
                &raw mut pRoot,
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK && idxNum <= 3 as ::core::ffi::c_int
            {
                let mut bbox: [RtreeCoord; 4] = [RtreeCoord { f: 0. }; 4];
                let mut p_0: *mut RtreeConstraint;
                geopolyBBox(
                    ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_context>(),
                    *argv.offset(0_isize),
                    &raw mut bbox as *mut RtreeCoord,
                    &raw mut rc,
                );
                if rc != 0 {
                    current_block = 17878652403481547450;
                } else {
                    p_0 = crate::src::src::malloc::sqlite3_malloc(
                        (::core::mem::size_of::<RtreeConstraint>() as usize)
                            .wrapping_mul(4_usize) as ::core::ffi::c_int,
                    ) as *mut RtreeConstraint;
                    (*pCsr).aConstraint = p_0;
                    (*pCsr).nConstraint = 4 as ::core::ffi::c_int;
                    if p_0.is_null() {
                        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                    } else {
                        ::libc::memset(
                            (*pCsr).aConstraint as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            (::core::mem::size_of::<RtreeConstraint>()
                                as crate::__stddef_size_t_h::SizeT)
                                .wrapping_mul(4 as crate::__stddef_size_t_h::SizeT),
                        );
                        ::libc::memset(
                            &raw mut (*pCsr).anQueue as *mut U32_0 as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            (::core::mem::size_of::<U32_0>() as crate::__stddef_size_t_h::SizeT)
                                .wrapping_mul(
                                    ((*pRtree).iDepth + 1 as ::core::ffi::c_int)
                                        as crate::__stddef_size_t_h::SizeT,
                                ),
                        );
                        if idxNum == 2 as ::core::ffi::c_int {
                            (*p_0).op = 'B' as i32;
                            (*p_0).iCoord = 0 as ::core::ffi::c_int;
                            (*p_0).u.rValue =
                                bbox[1 as ::core::ffi::c_int as usize].f as RtreeDValue;
                            p_0 = p_0.offset(1);
                            (*p_0).op = 'D' as i32;
                            (*p_0).iCoord = 1 as ::core::ffi::c_int;
                            (*p_0).u.rValue =
                                bbox[0 as ::core::ffi::c_int as usize].f as RtreeDValue;
                            p_0 = p_0.offset(1);
                            (*p_0).op = 'B' as i32;
                            (*p_0).iCoord = 2 as ::core::ffi::c_int;
                            (*p_0).u.rValue =
                                bbox[3 as ::core::ffi::c_int as usize].f as RtreeDValue;
                            p_0 = p_0.offset(1);
                            (*p_0).op = 'D' as i32;
                            (*p_0).iCoord = 3 as ::core::ffi::c_int;
                            (*p_0).u.rValue =
                                bbox[2 as ::core::ffi::c_int as usize].f as RtreeDValue;
                        } else {
                            (*p_0).op = 'D' as i32;
                            (*p_0).iCoord = 0 as ::core::ffi::c_int;
                            (*p_0).u.rValue =
                                bbox[0 as ::core::ffi::c_int as usize].f as RtreeDValue;
                            p_0 = p_0.offset(1);
                            (*p_0).op = 'B' as i32;
                            (*p_0).iCoord = 1 as ::core::ffi::c_int;
                            (*p_0).u.rValue =
                                bbox[1 as ::core::ffi::c_int as usize].f as RtreeDValue;
                            p_0 = p_0.offset(1);
                            (*p_0).op = 'D' as i32;
                            (*p_0).iCoord = 2 as ::core::ffi::c_int;
                            (*p_0).u.rValue =
                                bbox[2 as ::core::ffi::c_int as usize].f as RtreeDValue;
                            p_0 = p_0.offset(1);
                            (*p_0).op = 'B' as i32;
                            (*p_0).iCoord = 3 as ::core::ffi::c_int;
                            (*p_0).u.rValue =
                                bbox[3 as ::core::ffi::c_int as usize].f as RtreeDValue;
                        }
                    }
                    current_block = 6476622998065200121;
                }
            } else {
                current_block = 6476622998065200121;
            }
            match current_block {
                17878652403481547450 => {}
                _ => {
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        
                        let pNew: *mut RtreeSearchPoint = rtreeSearchPointNew(
                            pCsr,
                            RTREE_ZERO,
                            ((*pRtree).iDepth + 1 as ::core::ffi::c_int) as U8_0,
                        );
                        if pNew.is_null() {
                            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                        } else {
                            let __pNew_ref = { &mut *pNew };
                            __pNew_ref.id = 1 as crate::src::headers::sqlite3_h::Sqlite3Int64;
                            __pNew_ref.iCell = 0 as U8_0;
                            __pNew_ref.eWithin =
                                crate::src::headers::sqlite3_h::PARTLY_WITHIN as U8_0;
                            (*pCsr).aNode[0 as ::core::ffi::c_int as usize] = pRoot;
                            pRoot = ::core::ptr::null_mut::<RtreeNode>();
                            rc = rtreeStepToLeaf(pCsr);
                        }
                    }
                }
            }
        }
        nodeRelease(pRtree, pRoot);
        rtreeRelease(pRtree);
        rc
    }

    pub unsafe extern "C" fn geopolyBestIndex(
        mut _tab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
        pIdxInfo: *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
    ) -> ::core::ffi::c_int {
        let mut ii: ::core::ffi::c_int;
        let mut iRowidTerm: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut iFuncTerm: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut idxNum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        ii = 0 as ::core::ffi::c_int;
        let __pIdxInfo_ref = { &mut *pIdxInfo };
        while ii < __pIdxInfo_ref.nConstraint {
            let p: *mut crate::src::headers::sqlite3_h::sqlite3_index_constraint =
                __pIdxInfo_ref.aConstraint.offset(ii as isize)
                    as *mut crate::src::headers::sqlite3_h::sqlite3_index_constraint;
            if ((*p).usable != 0) {
                let __p_ref = { &*p };
                if __p_ref.iColumn < 0 as ::core::ffi::c_int
                    && __p_ref.op as ::core::ffi::c_int
                        == crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_EQ_1
                {
                    iRowidTerm = ii;
                    break;
                } else if __p_ref.iColumn == 0 as ::core::ffi::c_int
                    && __p_ref.op as ::core::ffi::c_int
                        >= crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_FUNCTION
                {
                    iFuncTerm = ii;
                    idxNum = __p_ref.op as ::core::ffi::c_int
                        - crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_FUNCTION
                        + 2 as ::core::ffi::c_int;
                }
            }
            ii += 1;
        }
        if iRowidTerm >= 0 as ::core::ffi::c_int {
            __pIdxInfo_ref.idxNum = 1 as ::core::ffi::c_int;
            __pIdxInfo_ref.idxStr =
                b"rowid\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            (*__pIdxInfo_ref.aConstraintUsage.offset(iRowidTerm as isize)).argvIndex =
                1 as ::core::ffi::c_int;
            (*__pIdxInfo_ref.aConstraintUsage.offset(iRowidTerm as isize)).omit =
                1 as ::core::ffi::c_uchar;
            __pIdxInfo_ref.estimatedCost = 30.0f64;
            __pIdxInfo_ref.estimatedRows = 1 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            __pIdxInfo_ref.idxFlags = crate::src::headers::sqlite3_h::SQLITE_INDEX_SCAN_UNIQUE;
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        if iFuncTerm >= 0 as ::core::ffi::c_int {
            __pIdxInfo_ref.idxNum = idxNum;
            __pIdxInfo_ref.idxStr =
                b"rtree\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            (*__pIdxInfo_ref.aConstraintUsage.offset(iFuncTerm as isize)).argvIndex =
                1 as ::core::ffi::c_int;
            (*__pIdxInfo_ref.aConstraintUsage.offset(iFuncTerm as isize)).omit =
                0 as ::core::ffi::c_uchar;
            __pIdxInfo_ref.estimatedCost = 300.0f64;
            __pIdxInfo_ref.estimatedRows = 10 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        __pIdxInfo_ref.idxNum = 4 as ::core::ffi::c_int;
        __pIdxInfo_ref.idxStr =
            b"fullscan\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        __pIdxInfo_ref.estimatedCost = 3000000.0f64;
        __pIdxInfo_ref.estimatedRows = 100000 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        crate::src::headers::sqlite3_h::SQLITE_OK
    }

    pub unsafe extern "C" fn geopolyColumn(
        cur: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
        ctx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        i: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let pRtree: *mut Rtree = (*cur).pVtab as *mut Rtree;
        let pCsr: *mut RtreeCursor = cur as *mut RtreeCursor;
        let p: *mut RtreeSearchPoint = rtreeSearchPointFirst(pCsr);
        let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
        let pNode: *mut RtreeNode = rtreeNodeOfFirstSearchPoint(pCsr, &raw mut rc);
        if rc != 0 {
            return rc;
        }
        if p.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        if i == 0 as ::core::ffi::c_int && crate::src::src::vdbeapi::sqlite3_vtab_nochange(ctx) != 0
        {
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        if i <= (*pRtree).nAux as ::core::ffi::c_int {
            if (*pCsr).bAuxValid == 0 {
                let __pCsr_ref = { &mut *pCsr };
                if __pCsr_ref.pReadAux.is_null() {
                    rc = crate::src::src::prepare::sqlite3_prepare_v3(
                        (*pRtree).db,
                        (*pRtree).zReadAuxSql,
                        -(1 as ::core::ffi::c_int),
                        0 as ::core::ffi::c_uint,
                        &raw mut __pCsr_ref.pReadAux,
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                    );
                    if rc != 0 {
                        return rc;
                    }
                }
                crate::src::src::vdbeapi::sqlite3_bind_int64(
                    __pCsr_ref.pReadAux,
                    1 as ::core::ffi::c_int,
                    nodeGetRowid(pRtree, pNode, (*p).iCell as ::core::ffi::c_int)
                        as crate::src::headers::sqlite3_h::Sqlite3Int64,
                );
                rc = crate::src::src::vdbeapi::sqlite3_step(__pCsr_ref.pReadAux);
                if rc == crate::src::headers::sqlite3_h::SQLITE_ROW {
                    __pCsr_ref.bAuxValid = 1 as U8_0;
                } else {
                    crate::src::src::vdbeapi::sqlite3_reset(__pCsr_ref.pReadAux);
                    if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
                        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                    }
                    return rc;
                }
            }
            crate::src::src::vdbeapi::sqlite3_result_value(
                ctx,
                crate::src::src::vdbeapi::sqlite3_column_value(
                    (*pCsr).pReadAux,
                    i + 2 as ::core::ffi::c_int,
                ),
            );
        }
        crate::src::headers::sqlite3_h::SQLITE_OK
    }

    pub unsafe extern "C" fn geopolyUpdate(
        pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
        nData: ::core::ffi::c_int,
        aData: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        pRowid: *mut crate::src::headers::sqlite3_h::SqliteInt64,
    ) -> ::core::ffi::c_int {
        let current_block: u64;
        let pRtree: *mut Rtree = pVtab as *mut Rtree;
        let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
        let mut cell: RtreeCell = RtreeCell {
            iRowid: 0,
            aCoord: [RtreeCoord { f: 0. }; 10],
        };
        
        
        
        
        let mut coordChange: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*pRtree).nNodeRef != 0 {
            return crate::src::headers::sqlite3_h::SQLITE_LOCKED_VTAB;
        }
        rtreeReference(pRtree);
        let oldRowidValid: ::core::ffi::c_int = (crate::src::src::vdbeapi::sqlite3_value_type(*aData.offset(0_isize))
            != crate::src::headers::sqlite3_h::SQLITE_NULL)
            as ::core::ffi::c_int;
        let oldRowid: I64_0 = (if oldRowidValid != 0 {
            crate::src::src::vdbeapi::sqlite3_value_int64(*aData.offset(0_isize))
        } else {
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64
        }) as I64_0;
        let newRowidValid: ::core::ffi::c_int = (nData > 1 as ::core::ffi::c_int
            && crate::src::src::vdbeapi::sqlite3_value_type(*aData.offset(1_isize))
                != crate::src::headers::sqlite3_h::SQLITE_NULL)
            as ::core::ffi::c_int;
        let newRowid: I64_0 = (if newRowidValid != 0 {
            crate::src::src::vdbeapi::sqlite3_value_int64(*aData.offset(1_isize))
        } else {
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64
        }) as I64_0;
        cell.iRowid = newRowid;
        if nData > 1 as ::core::ffi::c_int
            && (oldRowidValid == 0
                || crate::src::src::vdbeapi::sqlite3_value_nochange(*aData.offset(2_isize)) == 0
                || oldRowid != newRowid)
        {
            geopolyBBox(
                ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_context>(),
                *aData.offset(2_isize),
                &raw mut cell.aCoord as *mut RtreeCoord,
                &raw mut rc,
            );
            if rc != 0 {
                if rc == crate::src::headers::sqlite3_h::SQLITE_ERROR {
                    (*pVtab).zErrMsg =
                        crate::sqlite_printf!("_shape does not contain a valid polygon");
                }
                current_block = 12797311492648718431;
            } else {
                coordChange = 1 as ::core::ffi::c_int;
                if newRowidValid != 0 && (oldRowidValid == 0 || oldRowid != newRowid) {
                    
                    let __pRtree_ref = { &*pRtree };
                    crate::src::src::vdbeapi::sqlite3_bind_int64(
                        __pRtree_ref.pReadRowid,
                        1 as ::core::ffi::c_int,
                        cell.iRowid as crate::src::headers::sqlite3_h::Sqlite3Int64,
                    );
                    let steprc: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_step(__pRtree_ref.pReadRowid);
                    rc = crate::src::src::vdbeapi::sqlite3_reset(__pRtree_ref.pReadRowid);
                    if crate::src::headers::sqlite3_h::SQLITE_ROW == steprc {
                        if crate::src::src::vtab::sqlite3_vtab_on_conflict(__pRtree_ref.db)
                            == crate::src::headers::sqlite3_h::SQLITE_REPLACE
                        {
                            rc = rtreeDeleteRowid(
                                pRtree,
                                cell.iRowid as crate::src::headers::sqlite3_h::Sqlite3Int64,
                            );
                        } else {
                            rc = rtreeConstraintError(pRtree, 0 as ::core::ffi::c_int);
                        }
                    }
                }
                current_block = 7056779235015430508;
            }
        } else {
            current_block = 7056779235015430508;
        }
        match current_block {
            7056779235015430508 => {
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                    && (nData == 1 as ::core::ffi::c_int || coordChange != 0 && oldRowidValid != 0)
                {
                    rc = rtreeDeleteRowid(
                        pRtree,
                        oldRowid as crate::src::headers::sqlite3_h::Sqlite3Int64,
                    );
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                    && nData > 1 as ::core::ffi::c_int
                    && coordChange != 0
                {
                    let mut pLeaf: *mut RtreeNode = ::core::ptr::null_mut::<RtreeNode>();
                    if newRowidValid == 0 {
                        rc = rtreeNewRowid(pRtree, &raw mut cell.iRowid);
                    }
                    *pRowid = cell.iRowid as crate::src::headers::sqlite3_h::SqliteInt64;
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        rc = ChooseLeaf(
                            pRtree,
                            &raw mut cell,
                            0 as ::core::ffi::c_int,
                            &raw mut pLeaf,
                        );
                    }
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        
                        rc = rtreeInsertCell(pRtree, pLeaf, &raw mut cell, 0 as ::core::ffi::c_int);
                        let rc2: ::core::ffi::c_int = nodeRelease(pRtree, pLeaf);
                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                            rc = rc2;
                        }
                    }
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                    && nData > 1 as ::core::ffi::c_int
                {
                    let pUp: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
                        (*pRtree).pWriteAux;
                    let mut jj: ::core::ffi::c_int;
                    let mut nChange: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    crate::src::src::vdbeapi::sqlite3_bind_int64(
                        pUp,
                        1 as ::core::ffi::c_int,
                        cell.iRowid as crate::src::headers::sqlite3_h::Sqlite3Int64,
                    );
                    if crate::src::src::vdbeapi::sqlite3_value_nochange(*aData.offset(2_isize))
                        != 0
                    {
                        crate::src::src::vdbeapi::sqlite3_bind_null(pUp, 2 as ::core::ffi::c_int);
                    } else {
                        let mut p: *mut crate::geopoly_c::GeoPoly =
                            ::core::ptr::null_mut::<crate::geopoly_c::GeoPoly>();
                        if crate::src::src::vdbeapi::sqlite3_value_type(*aData.offset(2_isize))
                            == crate::src::headers::sqlite3_h::SQLITE_TEXT
                            && {
                                p = geopolyFuncParam(
                                    ::core::ptr::null_mut::<
                                        crate::src::headers::vdbeInt_h::sqlite3_context,
                                    >(),
                                    *aData.offset(2_isize),
                                    &raw mut rc,
                                );
                                !p.is_null()
                            }
                            && rc == crate::src::headers::sqlite3_h::SQLITE_OK
                        {
                            crate::src::src::vdbeapi::sqlite3_bind_blob(
                                pUp,
                                2 as ::core::ffi::c_int,
                                &raw mut (*p).hdr as *mut ::core::ffi::c_uchar
                                    as *const ::core::ffi::c_void,
                                4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * (*p).nVertex,
                                ::core::mem::transmute::<
                                    crate::src::headers::stdlib::IntptrT,
                                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                                >(
                                    -(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT
                                ),
                            );
                        } else {
                            crate::src::src::vdbeapi::sqlite3_bind_value(
                                pUp,
                                2 as ::core::ffi::c_int,
                                *aData.offset(2_isize),
                            );
                        }
                        crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
                        nChange = 1 as ::core::ffi::c_int;
                    }
                    jj = 1 as ::core::ffi::c_int;
                    while jj < nData - 2 as ::core::ffi::c_int {
                        nChange += 1;
                        crate::src::src::vdbeapi::sqlite3_bind_value(
                            pUp,
                            jj + 2 as ::core::ffi::c_int,
                            *aData.offset((jj + 2 as ::core::ffi::c_int) as isize),
                        );
                        jj += 1;
                    }
                    if nChange != 0 {
                        crate::src::src::vdbeapi::sqlite3_step(pUp);
                        rc = crate::src::src::vdbeapi::sqlite3_reset(pUp);
                    }
                }
            }
            _ => {}
        }
        rtreeRelease(pRtree);
        rc
    }

    pub unsafe extern "C" fn geopolyFindFunction(
        mut _pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
        mut _nArg: ::core::ffi::c_int,
        zName: *const ::core::ffi::c_char,
        pxFunc: *mut Option<
            unsafe extern "C" fn(
                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            ) -> (),
        >,
        ppArg: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int {
        if crate::src::src::util::sqlite3_stricmp(
            zName,
            b"geopoly_overlap\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            *pxFunc = Some(
                geopolyOverlapFunc
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> (),
                >;
            *ppArg = ::core::ptr::null_mut::<::core::ffi::c_void>();
            return crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_FUNCTION;
        }
        if crate::src::src::util::sqlite3_stricmp(
            zName,
            b"geopoly_within\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            *pxFunc = Some(
                geopolyWithinFunc
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> (),
                >;
            *ppArg = ::core::ptr::null_mut::<::core::ffi::c_void>();
            return crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_FUNCTION
                + 1 as ::core::ffi::c_int;
        }
        0 as ::core::ffi::c_int
    }

    pub static mut geopolyModule: crate::src::headers::sqlite3_h::sqlite3_module = {
        crate::src::headers::sqlite3_h::sqlite3_module {
            iVersion: 3 as ::core::ffi::c_int,
            xCreate: Some(
                geopolyCreate
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqliteInt_h::sqlite3,
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const *const ::core::ffi::c_char,
                        *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                        *mut *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            xConnect: Some(
                geopolyConnect
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqliteInt_h::sqlite3,
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const *const ::core::ffi::c_char,
                        *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                        *mut *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            xBestIndex: Some(
                geopolyBestIndex
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                        *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
                    ) -> ::core::ffi::c_int,
            ),
            xDisconnect: Some(
                rtreeDisconnect
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    ) -> ::core::ffi::c_int,
            ),
            xDestroy: Some(
                rtreeDestroy
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    ) -> ::core::ffi::c_int,
            ),
            xOpen: Some(
                rtreeOpen
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                        *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                    ) -> ::core::ffi::c_int,
            ),
            xClose: Some(
                rtreeClose
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                    ) -> ::core::ffi::c_int,
            ),
            xFilter: Some(
                geopolyFilter
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> ::core::ffi::c_int,
            ),
            xNext: Some(
                rtreeNext
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                    ) -> ::core::ffi::c_int,
            ),
            xEof: Some(
                rtreeEof
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                    ) -> ::core::ffi::c_int,
            ),
            xColumn: Some(
                geopolyColumn
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                        *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            xRowid: Some(
                rtreeRowid
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                        *mut crate::src::headers::sqlite3_h::SqliteInt64,
                    ) -> ::core::ffi::c_int,
            ),
            xUpdate: Some(
                geopolyUpdate
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        *mut crate::src::headers::sqlite3_h::SqliteInt64,
                    ) -> ::core::ffi::c_int,
            ),
            xBegin: Some(
                rtreeBeginTransaction
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    ) -> ::core::ffi::c_int,
            ),
            xSync: Some(
                rtreeEndTransaction
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    ) -> ::core::ffi::c_int,
            ),
            xCommit: Some(
                rtreeEndTransaction
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    ) -> ::core::ffi::c_int,
            ),
            xRollback: Some(
                rtreeEndTransaction
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    ) -> ::core::ffi::c_int,
            ),
            xFindFunction: Some(
                geopolyFindFunction
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        *mut Option<
                            unsafe extern "C" fn(
                                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            ) -> (),
                        >,
                        *mut *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            xRename: Some(
                rtreeRename
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            xSavepoint: Some(
                rtreeSavepoint
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            xRelease: None,
            xRollbackTo: None,
            xShadowName: Some(
                rtreeShadowName
                    as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            ),
            xIntegrity: Some(
                rtreeIntegrity
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        }
    };

    pub unsafe extern "C" fn sqlite3_geopoly_init(
        db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    ) -> ::core::ffi::c_int {
        let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
        pub static mut aFunc: [crate::geopoly_c::C2RustUnnamed_2; 12] = {
            [
                crate::geopoly_c::C2RustUnnamed_2 {
                    xFunc: Some(
                        geopolyAreaFunc
                            as unsafe extern "C" fn(
                                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            ) -> (),
                    ),
                    nArg: 1 as ::core::ffi::c_schar,
                    bPure: 1 as ::core::ffi::c_uchar,
                    zName: b"geopoly_area\0" as *const u8 as *const ::core::ffi::c_char,
                },
                crate::geopoly_c::C2RustUnnamed_2 {
                    xFunc: Some(
                        geopolyBlobFunc
                            as unsafe extern "C" fn(
                                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            ) -> (),
                    ),
                    nArg: 1 as ::core::ffi::c_schar,
                    bPure: 1 as ::core::ffi::c_uchar,
                    zName: b"geopoly_blob\0" as *const u8 as *const ::core::ffi::c_char,
                },
                crate::geopoly_c::C2RustUnnamed_2 {
                    xFunc: Some(
                        geopolyJsonFunc
                            as unsafe extern "C" fn(
                                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            ) -> (),
                    ),
                    nArg: 1 as ::core::ffi::c_schar,
                    bPure: 1 as ::core::ffi::c_uchar,
                    zName: b"geopoly_json\0" as *const u8 as *const ::core::ffi::c_char,
                },
                crate::geopoly_c::C2RustUnnamed_2 {
                    xFunc: Some(
                        geopolySvgFunc
                            as unsafe extern "C" fn(
                                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            ) -> (),
                    ),
                    nArg: -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
                    bPure: 1 as ::core::ffi::c_uchar,
                    zName: b"geopoly_svg\0" as *const u8 as *const ::core::ffi::c_char,
                },
                crate::geopoly_c::C2RustUnnamed_2 {
                    xFunc: Some(
                        geopolyWithinFunc
                            as unsafe extern "C" fn(
                                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            ) -> (),
                    ),
                    nArg: 2 as ::core::ffi::c_schar,
                    bPure: 1 as ::core::ffi::c_uchar,
                    zName: b"geopoly_within\0" as *const u8 as *const ::core::ffi::c_char,
                },
                crate::geopoly_c::C2RustUnnamed_2 {
                    xFunc: Some(
                        geopolyContainsPointFunc
                            as unsafe extern "C" fn(
                                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            ) -> (),
                    ),
                    nArg: 3 as ::core::ffi::c_schar,
                    bPure: 1 as ::core::ffi::c_uchar,
                    zName: b"geopoly_contains_point\0" as *const u8 as *const ::core::ffi::c_char,
                },
                crate::geopoly_c::C2RustUnnamed_2 {
                    xFunc: Some(
                        geopolyOverlapFunc
                            as unsafe extern "C" fn(
                                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            ) -> (),
                    ),
                    nArg: 2 as ::core::ffi::c_schar,
                    bPure: 1 as ::core::ffi::c_uchar,
                    zName: b"geopoly_overlap\0" as *const u8 as *const ::core::ffi::c_char,
                },
                crate::geopoly_c::C2RustUnnamed_2 {
                    xFunc: Some(
                        geopolyDebugFunc
                            as unsafe extern "C" fn(
                                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            ) -> (),
                    ),
                    nArg: 1 as ::core::ffi::c_schar,
                    bPure: 0 as ::core::ffi::c_uchar,
                    zName: b"geopoly_debug\0" as *const u8 as *const ::core::ffi::c_char,
                },
                crate::geopoly_c::C2RustUnnamed_2 {
                    xFunc: Some(
                        geopolyBBoxFunc
                            as unsafe extern "C" fn(
                                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            ) -> (),
                    ),
                    nArg: 1 as ::core::ffi::c_schar,
                    bPure: 1 as ::core::ffi::c_uchar,
                    zName: b"geopoly_bbox\0" as *const u8 as *const ::core::ffi::c_char,
                },
                crate::geopoly_c::C2RustUnnamed_2 {
                    xFunc: Some(
                        geopolyXformFunc
                            as unsafe extern "C" fn(
                                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            ) -> (),
                    ),
                    nArg: 7 as ::core::ffi::c_schar,
                    bPure: 1 as ::core::ffi::c_uchar,
                    zName: b"geopoly_xform\0" as *const u8 as *const ::core::ffi::c_char,
                },
                crate::geopoly_c::C2RustUnnamed_2 {
                    xFunc: Some(
                        geopolyRegularFunc
                            as unsafe extern "C" fn(
                                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            ) -> (),
                    ),
                    nArg: 4 as ::core::ffi::c_schar,
                    bPure: 1 as ::core::ffi::c_uchar,
                    zName: b"geopoly_regular\0" as *const u8 as *const ::core::ffi::c_char,
                },
                crate::geopoly_c::C2RustUnnamed_2 {
                    xFunc: Some(
                        geopolyCcwFunc
                            as unsafe extern "C" fn(
                                *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                                ::core::ffi::c_int,
                                *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            ) -> (),
                    ),
                    nArg: 1 as ::core::ffi::c_schar,
                    bPure: 1 as ::core::ffi::c_uchar,
                    zName: b"geopoly_ccw\0" as *const u8 as *const ::core::ffi::c_char,
                },
            ]
        };
        pub static mut aAgg: [crate::geopoly_c::C2RustUnnamed1; 1] = {
            [crate::geopoly_c::C2RustUnnamed1 {
                xStep: Some(
                    geopolyBBoxStep
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
                xFinal: Some(
                    geopolyBBoxFinal
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                        ) -> (),
                ),
                zName: b"geopoly_group_bbox\0" as *const u8 as *const ::core::ffi::c_char,
            }]
        };
        let mut i: ::core::ffi::c_uint;
        i = 0 as ::core::ffi::c_uint;
        while (i as usize)
            < (::core::mem::size_of::<[crate::geopoly_c::C2RustUnnamed_2; 12]>() as usize)
                .wrapping_div(::core::mem::size_of::<crate::geopoly_c::C2RustUnnamed_2>() as usize)
            && rc == crate::src::headers::sqlite3_h::SQLITE_OK
        {
            let enc: ::core::ffi::c_int;
            if aFunc[i as usize].bPure != 0 {
                enc = crate::src::headers::sqlite3_h::SQLITE_UTF8
                    | crate::src::headers::sqlite3_h::SQLITE_DETERMINISTIC
                    | crate::src::headers::sqlite3_h::SQLITE_INNOCUOUS;
            } else {
                enc = crate::src::headers::sqlite3_h::SQLITE_UTF8
                    | crate::src::headers::sqlite3_h::SQLITE_DIRECTONLY;
            }
            rc = crate::src::src::main::sqlite3_create_function(
                db,
                aFunc[i as usize].zName,
                aFunc[i as usize].nArg as ::core::ffi::c_int,
                enc,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                aFunc[i as usize].xFunc,
                None,
                None,
            );
            i = i.wrapping_add(1);
        }
        i = 0 as ::core::ffi::c_uint;
        while (i as usize)
            < (::core::mem::size_of::<[crate::geopoly_c::C2RustUnnamed1; 1]>() as usize)
                .wrapping_div(::core::mem::size_of::<crate::geopoly_c::C2RustUnnamed1>() as usize)
            && rc == crate::src::headers::sqlite3_h::SQLITE_OK
        {
            rc = crate::src::src::main::sqlite3_create_function(
                db,
                aAgg[i as usize].zName,
                1 as ::core::ffi::c_int,
                crate::src::headers::sqlite3_h::SQLITE_UTF8
                    | crate::src::headers::sqlite3_h::SQLITE_DETERMINISTIC
                    | crate::src::headers::sqlite3_h::SQLITE_INNOCUOUS,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
                aAgg[i as usize].xStep,
                aAgg[i as usize].xFinal,
            );
            i = i.wrapping_add(1);
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = crate::src::src::vtab::sqlite3_create_module_v2(
                db,
                b"geopoly\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut geopolyModule as *mut _
                    as *const crate::src::headers::sqlite3_h::sqlite3_module,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            );
        }
        rc
    }

    use crate::src::ext::rtree::rtree::stdlib_float_h::atof;

    use crate::src::ext::rtree::rtree::ChooseLeaf;
    use crate::src::ext::rtree::rtree::RTREE_COORD_REAL32;
    use crate::src::ext::rtree::rtree::RTREE_ZERO;
    use crate::src::ext::rtree::rtree::Rtree;
    use crate::src::ext::rtree::rtree::RtreeCell;
    use crate::src::ext::rtree::rtree::RtreeConstraint;
    use crate::src::ext::rtree::rtree::RtreeCoord;
    use crate::src::ext::rtree::rtree::RtreeCursor;
    use crate::src::ext::rtree::rtree::RtreeDValue;
    use crate::src::ext::rtree::rtree::RtreeNode;
    use crate::src::ext::rtree::rtree::RtreeSearchPoint;
    use crate::src::ext::rtree::rtree::RtreeValue;
    use crate::src::ext::rtree::rtree::findLeafNode;
    use crate::src::ext::rtree::rtree::getNodeSize;
    use crate::src::ext::rtree::rtree::I64_0;
    use crate::src::ext::rtree::rtree::nodeAcquire;
    use crate::src::ext::rtree::rtree::nodeGetRowid;
    use crate::src::ext::rtree::rtree::nodeRelease;
    use crate::src::ext::rtree::rtree::nodeRowidIndex;
    use crate::src::ext::rtree::rtree::resetCursor;
    use crate::src::ext::rtree::rtree::rtreeBeginTransaction;
    use crate::src::ext::rtree::rtree::rtreeClose;
    use crate::src::ext::rtree::rtree::rtreeConstraintError;
    use crate::src::ext::rtree::rtree::rtreeDeleteRowid;
    use crate::src::ext::rtree::rtree::rtreeDestroy;
    use crate::src::ext::rtree::rtree::rtreeDisconnect;
    use crate::src::ext::rtree::rtree::rtreeEndTransaction;
    use crate::src::ext::rtree::rtree::rtreeEof;
    use crate::src::ext::rtree::rtree::rtreeInsertCell;
    use crate::src::ext::rtree::rtree::rtreeIntegrity;
    use crate::src::ext::rtree::rtree::rtreeModule;
    use crate::src::ext::rtree::rtree::rtreeNewRowid;
    use crate::src::ext::rtree::rtree::rtreeNext;
    use crate::src::ext::rtree::rtree::rtreeNodeOfFirstSearchPoint;
    use crate::src::ext::rtree::rtree::rtreeOpen;
    use crate::src::ext::rtree::rtree::rtreeReference;
    use crate::src::ext::rtree::rtree::rtreeRelease;
    use crate::src::ext::rtree::rtree::rtreeRename;
    use crate::src::ext::rtree::rtree::rtreeRowid;
    use crate::src::ext::rtree::rtree::rtreeSavepoint;
    use crate::src::ext::rtree::rtree::rtreeSearchPointFirst;
    use crate::src::ext::rtree::rtree::rtreeSearchPointNew;
    use crate::src::ext::rtree::rtree::rtreeShadowName;
    use crate::src::ext::rtree::rtree::rtreeSqlInit;
    use crate::src::ext::rtree::rtree::rtreeStepToLeaf;
    use crate::src::ext::rtree::rtree::U8_0;
    use crate::src::ext::rtree::rtree::U32_0;
}

pub mod stdlib_float_h {
    #[inline]
    pub unsafe extern "C" fn atof(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_double {
        ::libc::strtod(
            __nptr,
            crate::__stddef_null_h::NULL as *mut *mut ::core::ffi::c_char,
        )
    }
}

use crate::src::src::printf::sqlite3_str_vappendf2;

pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::SizeT;
pub use crate::src::headers::stdlib::VaList;

pub use crate::geopoly_c::C2RustUnnamed1;
pub use crate::geopoly_c::C2RustUnnamed_2;
pub use crate::geopoly_c::GEOPOLY_PI;
pub use crate::geopoly_c::GeoBBox;
pub use crate::geopoly_c::GeoCoord;
pub use crate::geopoly_c::GeoEvent;
pub use crate::geopoly_c::GeoOverlap;
pub use crate::geopoly_c::GeoParse;
pub use crate::geopoly_c::GeoPoly;
pub use crate::geopoly_c::GeoSegment;
pub use crate::internal::BuiltinVaList;
pub use crate::internal::VaListTag;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyAddOneSegment;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyAddSegments;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyArea;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyAreaFunc;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyBBox;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyBBoxFinal;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyBBoxFunc;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyBBoxStep;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyBestIndex;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyBlobFunc;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyCcwFunc;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyColumn;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyConnect;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyContainsPointFunc;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyCreate;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyDebugFunc;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyEventMerge;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyFilter;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyFindFunction;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyFuncParam;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyInit;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyIsSpace;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyJsonFunc;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyModule;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyOverlap;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyOverlapFunc;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyParseJson;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyParseNumber;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyRegularFunc;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolySegmentMerge;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolySine;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolySkipSpace;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolySortEventsByX;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolySortSegmentsByYAndC;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolySvgFunc;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolySwab32;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyUpdate;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyWithinFunc;
pub use crate::src::ext::rtree::rtree::geopoly_c::geopolyXformFunc;
pub use crate::src::ext::rtree::rtree::geopoly_c::pointBeneathLine;
pub use crate::src::ext::rtree::rtree::geopoly_c::sqlite3_geopoly_init;
pub use crate::src::ext::rtree::rtree::stdlib_float_h::atof;
pub use crate::src::headers::sqlite3_h::FULLY_WITHIN;
pub use crate::src::headers::sqlite3_h::NOT_WITHIN;
pub use crate::src::headers::sqlite3_h::PARTLY_WITHIN;
pub use crate::src::headers::sqlite3_h::SQLITE_ABORT;
pub use crate::src::headers::sqlite3_h::SQLITE_ANY;
pub use crate::src::headers::sqlite3_h::SQLITE_BLOB;
pub use crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT;
pub use crate::src::headers::sqlite3_h::SQLITE_CORRUPT;
pub use crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
pub use crate::src::headers::sqlite3_h::SQLITE_DETERMINISTIC;
pub use crate::src::headers::sqlite3_h::SQLITE_DIRECTONLY;
pub use crate::src::headers::sqlite3_h::SQLITE_DONE;
pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;
pub use crate::src::headers::sqlite3_h::SQLITE_FLOAT;
pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_EQ_1;
pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_FUNCTION;
pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_GE;
pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_GT;
pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_LE;
pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_LT;
pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_MATCH_1;
pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_SCAN_UNIQUE;
pub use crate::src::headers::sqlite3_h::SQLITE_INNOCUOUS;
pub use crate::src::headers::sqlite3_h::SQLITE_INTEGER;
pub use crate::src::headers::sqlite3_h::SQLITE_LOCKED;
pub use crate::src::headers::sqlite3_h::SQLITE_LOCKED_VTAB;
pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_NULL;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SQLITE_PREPARE_NO_VTAB;
pub use crate::src::headers::sqlite3_h::SQLITE_PREPARE_PERSISTENT;
pub use crate::src::headers::sqlite3_h::SQLITE_REPLACE;
pub use crate::src::headers::sqlite3_h::SQLITE_ROW;
pub use crate::src::headers::sqlite3_h::SQLITE_STATIC;
pub use crate::src::headers::sqlite3_h::SQLITE_TEXT;
pub use crate::src::headers::sqlite3_h::SQLITE_UTF8;
pub use crate::src::headers::sqlite3_h::SQLITE_VTAB_CONSTRAINT_SUPPORT;
pub use crate::src::headers::sqlite3_h::SQLITE_VTAB_INNOCUOUS;
pub use crate::src::headers::sqlite3_h::SqliteInt64;
pub use crate::src::headers::sqlite3_h::SqliteUint64;
pub use crate::src::headers::sqlite3_h::Sqlite3Blob;
pub use crate::src::headers::sqlite3_h::Sqlite3DestructorType;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;
pub use crate::src::headers::sqlite3_h::sqlite3_index_info;
pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;
pub use crate::src::headers::sqlite3_h::Sqlite3Int64;
pub use crate::src::headers::sqlite3_h::sqlite3_module;
pub use crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
pub use crate::src::headers::sqlite3_h::sqlite3_rtree_geometry;
pub use crate::src::headers::sqlite3_h::sqlite3_rtree_query_info;
pub use crate::src::headers::sqlite3_h::Sqlite3Stmt;
pub use crate::src::headers::sqlite3_h::Sqlite3Uint64;
pub use crate::src::headers::sqlite3_h::sqlite3_vtab;
pub use crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;
pub use crate::src::headers::sqliteInt_h::sqlite3;
pub use crate::src::headers::sqliteInt_h::sqlite3_str;
pub use crate::src::headers::stdlib::__ctype_b_loc;
pub use crate::src::headers::stdlib::_ISalnum;
pub use crate::src::headers::stdlib::_ISalpha;
pub use crate::src::headers::stdlib::_ISblank;
pub use crate::src::headers::stdlib::_IScntrl;
pub use crate::src::headers::stdlib::_ISdigit;
pub use crate::src::headers::stdlib::_ISgraph;
pub use crate::src::headers::stdlib::_ISlower;
pub use crate::src::headers::stdlib::_ISprint;
pub use crate::src::headers::stdlib::_ISpunct;
pub use crate::src::headers::stdlib::_ISspace;
pub use crate::src::headers::stdlib::_ISupper;
pub use crate::src::headers::stdlib::_ISxdigit;
pub use crate::src::headers::stdlib::C2RustUnnamed0_1;
pub use crate::src::headers::vdbeInt_h::sqlite3_context;
pub use crate::src::headers::vdbeInt_h::sqlite3_value;
pub use crate::src::src::legacy::sqlite3_exec;
pub use crate::src::src::main::sqlite3_create_function;
pub use crate::src::src::main::sqlite3_create_function_v2;
pub use crate::src::src::main::sqlite3_errmsg;
pub use crate::src::src::main::sqlite3_last_insert_rowid;
pub use crate::src::src::main::sqlite3_table_column_metadata;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3_malloc;
pub use crate::src::src::malloc::sqlite3_malloc64;
pub use crate::src::src::malloc::sqlite3_realloc64;
pub use crate::src::src::prepare::sqlite3_prepare_v2;
pub use crate::src::src::prepare::sqlite3_prepare_v3;
pub use crate::src::src::printf::sqlite3_str_append;
pub use crate::src::src::printf::sqlite3_str_errcode;
pub use crate::src::src::printf::sqlite3_str_finish;
pub use crate::src::src::printf::sqlite3_str_new;
pub use crate::src::src::util::sqlite3_stricmp;
pub use crate::src::src::vdbe::sqlite3_value_numeric_type;
pub use crate::src::src::vdbeapi::sqlite3_aggregate_context;
pub use crate::src::src::vdbeapi::sqlite3_bind_blob;
pub use crate::src::src::vdbeapi::sqlite3_bind_int64;
pub use crate::src::src::vdbeapi::sqlite3_bind_null;
pub use crate::src::src::vdbeapi::sqlite3_bind_value;
pub use crate::src::src::vdbeapi::sqlite3_column_blob;
pub use crate::src::src::vdbeapi::sqlite3_column_bytes;
pub use crate::src::src::vdbeapi::sqlite3_column_count;
pub use crate::src::src::vdbeapi::sqlite3_column_int;
pub use crate::src::src::vdbeapi::sqlite3_column_int64;
pub use crate::src::src::vdbeapi::sqlite3_column_name;
pub use crate::src::src::vdbeapi::sqlite3_column_type;
pub use crate::src::src::vdbeapi::sqlite3_column_value;
pub use crate::src::src::vdbeapi::sqlite3_context_db_handle;
pub use crate::src::src::vdbeapi::sqlite3_finalize;
pub use crate::src::src::vdbeapi::sqlite3_reset;
pub use crate::src::src::vdbeapi::sqlite3_result_blob;
pub use crate::src::src::vdbeapi::sqlite3_result_double;
pub use crate::src::src::vdbeapi::sqlite3_result_error;
pub use crate::src::src::vdbeapi::sqlite3_result_error_code;
pub use crate::src::src::vdbeapi::sqlite3_result_error_nomem;
pub use crate::src::src::vdbeapi::sqlite3_result_int;
pub use crate::src::src::vdbeapi::sqlite3_result_int64;
pub use crate::src::src::vdbeapi::sqlite3_result_pointer;
pub use crate::src::src::vdbeapi::sqlite3_result_text;
pub use crate::src::src::vdbeapi::sqlite3_result_value;
pub use crate::src::src::vdbeapi::sqlite3_step;
pub use crate::src::src::vdbeapi::sqlite3_user_data;
pub use crate::src::src::vdbeapi::sqlite3_value_blob;
pub use crate::src::src::vdbeapi::sqlite3_value_bytes;
pub use crate::src::src::vdbeapi::sqlite3_value_double;
pub use crate::src::src::vdbeapi::sqlite3_value_dup;
pub use crate::src::src::vdbeapi::sqlite3_value_free;
pub use crate::src::src::vdbeapi::sqlite3_value_int;
pub use crate::src::src::vdbeapi::sqlite3_value_int64;
pub use crate::src::src::vdbeapi::sqlite3_value_nochange;
pub use crate::src::src::vdbeapi::sqlite3_value_pointer;
pub use crate::src::src::vdbeapi::sqlite3_value_text;
pub use crate::src::src::vdbeapi::sqlite3_value_type;
pub use crate::src::src::vdbeapi::sqlite3_vtab_nochange;
pub use crate::src::src::vdbeblob::sqlite3_blob_bytes;
pub use crate::src::src::vdbeblob::sqlite3_blob_close;
pub use crate::src::src::vdbeblob::sqlite3_blob_open;
pub use crate::src::src::vdbeblob::sqlite3_blob_read;
pub use crate::src::src::vdbeblob::sqlite3_blob_reopen;
pub use crate::src::src::vtab::sqlite3_create_module_v2;
pub use crate::src::src::vtab::sqlite3_declare_vtab;
pub use crate::src::src::vtab::sqlite3_vtab_on_conflict;


pub use crate::src::src::tokenize::sqlite3GetToken;
pub use crate::src::src::vdbeaux::sqlite3IntFloatCompare;

pub type RtreeDValue = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RtreeGeomCallback {
    pub xGeom: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_rtree_geometry,
            ::core::ffi::c_int,
            *mut RtreeDValue,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xQueryFunc: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_rtree_query_info,
        ) -> ::core::ffi::c_int,
    >,
    pub xDestructor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pContext: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RtreeMatchArg {
    pub iSize: U32_0,
    pub cb: RtreeGeomCallback,
    pub nParam: ::core::ffi::c_int,
    pub apSqlParam: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pub aParam: [RtreeDValue; 0],
}

pub type U32_0 = ::core::ffi::c_uint;

pub type I64_0 = crate::src::headers::sqlite3_h::Sqlite3Int64;

pub type U64_0 = crate::src::headers::sqlite3_h::Sqlite3Uint64;

pub type U8_0 = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rtree {
    pub base: crate::src::headers::sqlite3_h::sqlite3_vtab,
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub iNodeSize: ::core::ffi::c_int,
    pub nDim: U8_0,
    pub nDim2: U8_0,
    pub eCoordType: U8_0,
    pub nBytesPerCell: U8_0,
    pub inWrTrans: U8_0,
    pub nAux: U8_0,
    pub nAuxNotNull: U8_0,
    pub iDepth: ::core::ffi::c_int,
    pub zDb: *mut ::core::ffi::c_char,
    pub zName: *mut ::core::ffi::c_char,
    pub zNodeName: *mut ::core::ffi::c_char,
    pub nBusy: U32_0,
    pub nRowEst: I64_0,
    pub nCursor: U32_0,
    pub nNodeRef: U32_0,
    pub zReadAuxSql: *mut ::core::ffi::c_char,
    pub pDeleted: *mut RtreeNode,
    pub pNodeBlob: *mut crate::src::headers::sqlite3_h::Sqlite3Blob,
    pub pWriteNode: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub pDeleteNode: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub pReadRowid: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub pWriteRowid: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub pDeleteRowid: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub pReadParent: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub pWriteParent: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub pDeleteParent: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub pWriteAux: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub aHash: [*mut RtreeNode; 97],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RtreeNode {
    pub pParent: *mut RtreeNode,
    pub iNode: I64_0,
    pub nRef: ::core::ffi::c_int,
    pub isDirty: ::core::ffi::c_int,
    pub zData: *mut U8_0,
    pub pNext: *mut RtreeNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RtreeCursor {
    pub base: crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    pub atEOF: U8_0,
    pub bPoint: U8_0,
    pub bAuxValid: U8_0,
    pub iStrategy: ::core::ffi::c_int,
    pub nConstraint: ::core::ffi::c_int,
    pub aConstraint: *mut RtreeConstraint,
    pub nPointAlloc: ::core::ffi::c_int,
    pub nPoint: ::core::ffi::c_int,
    pub mxLevel: ::core::ffi::c_int,
    pub aPoint: *mut RtreeSearchPoint,
    pub pReadAux: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub sPoint: RtreeSearchPoint,
    pub aNode: [*mut RtreeNode; 5],
    pub anQueue: [U32_0; 41],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RtreeSearchPoint {
    pub rScore: RtreeDValue,
    pub id: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub iLevel: U8_0,
    pub eWithin: U8_0,
    pub iCell: U8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RtreeConstraint {
    pub iCoord: ::core::ffi::c_int,
    pub op: ::core::ffi::c_int,
    pub u: C2RustUnnamed,
    pub pInfo: *mut crate::src::headers::sqlite3_h::sqlite3_rtree_query_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub rValue: RtreeDValue,
    pub xGeom: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_rtree_geometry,
            ::core::ffi::c_int,
            *mut RtreeDValue,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xQueryFunc: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_rtree_query_info,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RtreeCell {
    pub iRowid: I64_0,
    pub aCoord: [RtreeCoord; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union RtreeCoord {
    pub f: RtreeValue,
    pub i: ::core::ffi::c_int,
    pub u: U32_0,
}

pub type RtreeValue = ::core::ffi::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RtreeCheck {
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub zDb: *const ::core::ffi::c_char,
    pub zTab: *const ::core::ffi::c_char,
    pub bInt: ::core::ffi::c_int,
    pub nDim: ::core::ffi::c_int,
    pub pGetNode: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pub aCheckMapping: [*mut crate::src::headers::sqlite3_h::Sqlite3Stmt; 2],
    pub nLeaf: ::core::ffi::c_int,
    pub nNonLeaf: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub zReport: *mut ::core::ffi::c_char,
    pub nErr: ::core::ffi::c_int,
}

pub const RTREE_MAX_DIMENSIONS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const RTREE_MAX_AUX_COLUMN: ::core::ffi::c_int = 100 as ::core::ffi::c_int;

pub const HASHSIZE: ::core::ffi::c_int = 97 as ::core::ffi::c_int;

pub const RTREE_DEFAULT_ROWEST: ::core::ffi::c_int = 1048576 as ::core::ffi::c_int;

pub const RTREE_MIN_ROWEST: ::core::ffi::c_int = 100 as ::core::ffi::c_int;

pub const RTREE_COORD_REAL32: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const RTREE_COORD_INT32: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const RTREE_ZERO: ::core::ffi::c_double = 0.0f64;

pub const RTREE_MAXCELLS: ::core::ffi::c_int = 51 as ::core::ffi::c_int;

pub const RTREE_MAX_DEPTH: ::core::ffi::c_int = 40 as ::core::ffi::c_int;

pub const RTREE_CACHE_SZ: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const RTREE_EQ: ::core::ffi::c_int = 65;

pub const RTREE_LE: ::core::ffi::c_int = 66;

pub const RTREE_LT: ::core::ffi::c_int = 67;

pub const RTREE_GE: ::core::ffi::c_int = 68;

pub const RTREE_GT: ::core::ffi::c_int = 69;

pub const RTREE_MATCH: ::core::ffi::c_int = 0x46 as ::core::ffi::c_int;

pub const RTREE_QUERY: ::core::ffi::c_int = 0x47 as ::core::ffi::c_int;

pub const RTREE_TRUE: ::core::ffi::c_int = 63;

pub const RTREE_FALSE: ::core::ffi::c_int = 64;

unsafe extern "C" fn readInt16(p: *mut U8_0) -> ::core::ffi::c_int {
    ((*p.offset(0_isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        + *p.offset(1_isize) as ::core::ffi::c_int
}

unsafe extern "C" fn readCoord(p: *mut U8_0, pCoord: *mut RtreeCoord) {
    (*pCoord).u = ((*p.offset(0_isize) as U32_0) << 24 as ::core::ffi::c_int)
        .wrapping_add((*p.offset(1_isize) as U32_0) << 16 as ::core::ffi::c_int)
        .wrapping_add((*p.offset(2_isize) as U32_0) << 8 as ::core::ffi::c_int)
        .wrapping_add((*p.offset(3_isize) as U32_0) << 0 as ::core::ffi::c_int);
}

unsafe extern "C" fn readInt64(p: *mut U8_0) -> I64_0 {
    ((*p.offset(0_isize) as U64_0) << 56 as ::core::ffi::c_int)
        .wrapping_add((*p.offset(1_isize) as U64_0) << 48 as ::core::ffi::c_int)
        .wrapping_add((*p.offset(2_isize) as U64_0) << 40 as ::core::ffi::c_int)
        .wrapping_add((*p.offset(3_isize) as U64_0) << 32 as ::core::ffi::c_int)
        .wrapping_add((*p.offset(4_isize) as U64_0) << 24 as ::core::ffi::c_int)
        .wrapping_add((*p.offset(5_isize) as U64_0) << 16 as ::core::ffi::c_int)
        .wrapping_add((*p.offset(6_isize) as U64_0) << 8 as ::core::ffi::c_int)
        .wrapping_add((*p.offset(7_isize) as U64_0) << 0 as ::core::ffi::c_int) as I64_0
}

unsafe extern "C" fn writeInt16(p: *mut U8_0, i: ::core::ffi::c_int) {
    *p.offset(0_isize) = (i >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as U8_0;
    *p.offset(1_isize) = (i >> 0 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as U8_0;
}

unsafe extern "C" fn writeCoord(
    p: *mut U8_0,
    pCoord: *mut RtreeCoord,
) -> ::core::ffi::c_int {
    
    let i: U32_0 = (*pCoord).u;
    *p.offset(0_isize) = (i >> 24 as ::core::ffi::c_int & 0xff as U32_0) as U8_0;
    *p.offset(1_isize) = (i >> 16 as ::core::ffi::c_int & 0xff as U32_0) as U8_0;
    *p.offset(2_isize) = (i >> 8 as ::core::ffi::c_int & 0xff as U32_0) as U8_0;
    *p.offset(3_isize) = (i >> 0 as ::core::ffi::c_int & 0xff as U32_0) as U8_0;
    4 as ::core::ffi::c_int
}

unsafe extern "C" fn writeInt64(p: *mut U8_0, i: I64_0) -> ::core::ffi::c_int {
    *p.offset(0_isize) = (i >> 56 as ::core::ffi::c_int & 0xff as I64_0) as U8_0;
    *p.offset(1_isize) = (i >> 48 as ::core::ffi::c_int & 0xff as I64_0) as U8_0;
    *p.offset(2_isize) = (i >> 40 as ::core::ffi::c_int & 0xff as I64_0) as U8_0;
    *p.offset(3_isize) = (i >> 32 as ::core::ffi::c_int & 0xff as I64_0) as U8_0;
    *p.offset(4_isize) = (i >> 24 as ::core::ffi::c_int & 0xff as I64_0) as U8_0;
    *p.offset(5_isize) = (i >> 16 as ::core::ffi::c_int & 0xff as I64_0) as U8_0;
    *p.offset(6_isize) = (i >> 8 as ::core::ffi::c_int & 0xff as I64_0) as U8_0;
    *p.offset(7_isize) = (i >> 0 as ::core::ffi::c_int & 0xff as I64_0) as U8_0;
    8 as ::core::ffi::c_int
}

unsafe extern "C" fn nodeReference(p: *mut RtreeNode) {
    if !p.is_null() {
        (*p).nRef += 1;
    }
}

unsafe extern "C" fn nodeZero(pRtree: *mut Rtree, p: *mut RtreeNode) {
    ::libc::memset(
        (*p).zData.offset(2_isize) as *mut U8_0 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ((*pRtree).iNodeSize - 2 as ::core::ffi::c_int) as crate::__stddef_size_t_h::SizeT,
    );
    (*p).isDirty = 1 as ::core::ffi::c_int;
}

unsafe extern "C" fn nodeHash(iNode: I64_0) -> ::core::ffi::c_uint {
    (iNode as ::core::ffi::c_uint).wrapping_rem(HASHSIZE as ::core::ffi::c_uint)
}

unsafe extern "C" fn nodeHashLookup(pRtree: *mut Rtree, iNode: I64_0) -> *mut RtreeNode {
    let mut p: *mut RtreeNode;
    p = (*pRtree).aHash[nodeHash(iNode) as usize];
    while !p.is_null() && (*p).iNode != iNode {
        p = (*p).pNext;
    }
    p
}

unsafe extern "C" fn nodeHashInsert(pRtree: *mut Rtree, pNode: *mut RtreeNode) {
    
    let iHash: ::core::ffi::c_int = nodeHash((*pNode).iNode) as ::core::ffi::c_int;
    (*pNode).pNext = (*pRtree).aHash[iHash as usize];
    (*pRtree).aHash[iHash as usize] = pNode;
}

unsafe extern "C" fn nodeHashDelete(pRtree: *mut Rtree, pNode: *mut RtreeNode) {
    let mut pp: *mut *mut RtreeNode;
    if (*pNode).iNode != 0 as I64_0 {
        let __pNode_ref = { &mut *pNode };
        pp = (&raw mut (*pRtree).aHash as *mut *mut RtreeNode).offset((nodeHash
            as unsafe extern "C" fn(I64_0) -> ::core::ffi::c_uint)(
            __pNode_ref.iNode
        ) as isize) as *mut *mut RtreeNode;
        while *pp != pNode {
            pp = &raw mut (**pp).pNext;
        }
        *pp = __pNode_ref.pNext;
        __pNode_ref.pNext = ::core::ptr::null_mut::<RtreeNode>();
    }
}

unsafe extern "C" fn nodeNew(
    pRtree: *mut Rtree,
    pParent: *mut RtreeNode,
) -> *mut RtreeNode {
    
    let pNode: *mut RtreeNode = crate::src::src::malloc::sqlite3_malloc64(
        (::core::mem::size_of::<RtreeNode>() as usize).wrapping_add((*pRtree).iNodeSize as usize)
            as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut RtreeNode;
    if !pNode.is_null() {
        let __pRtree_ref = { &mut *pRtree };
        ::libc::memset(
            pNode as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<RtreeNode>() as crate::__stddef_size_t_h::SizeT)
                .wrapping_add(__pRtree_ref.iNodeSize as crate::__stddef_size_t_h::SizeT),
        );
        let __pNode_ref = { &mut *pNode };
        __pNode_ref.zData = pNode.offset(1_isize) as *mut RtreeNode as *mut U8_0;
        __pNode_ref.nRef = 1 as ::core::ffi::c_int;
        __pRtree_ref.nNodeRef = __pRtree_ref.nNodeRef.wrapping_add(1);
        __pNode_ref.pParent = pParent;
        __pNode_ref.isDirty = 1 as ::core::ffi::c_int;
        nodeReference(pParent);
    }
    pNode
}

unsafe extern "C" fn nodeBlobReset(pRtree: *mut Rtree) {
    let pBlob: *mut crate::src::headers::sqlite3_h::Sqlite3Blob = (*pRtree).pNodeBlob;
    (*pRtree).pNodeBlob = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Blob>();
    crate::src::src::vdbeblob::sqlite3_blob_close(pBlob);
}

unsafe extern "C" fn nodeAcquire(
    pRtree: *mut Rtree,
    iNode: I64_0,
    pParent: *mut RtreeNode,
    ppNode: *mut *mut RtreeNode,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pNode: *mut RtreeNode;
    pNode = nodeHashLookup(pRtree, iNode);
    if !pNode.is_null() {
        if !pParent.is_null() && pParent != (*pNode).pParent {
            return crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
        }
        (*pNode).nRef += 1;
        *ppNode = pNode;
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    let __pRtree_ref = { &mut *pRtree };
    if !__pRtree_ref.pNodeBlob.is_null() {
        let pBlob: *mut crate::src::headers::sqlite3_h::Sqlite3Blob = __pRtree_ref.pNodeBlob;
        __pRtree_ref.pNodeBlob =
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Blob>();
        rc = crate::src::src::vdbeblob::sqlite3_blob_reopen(
            pBlob,
            iNode as crate::src::headers::sqlite3_h::Sqlite3Int64,
        );
        __pRtree_ref.pNodeBlob = pBlob;
        if rc != 0 {
            nodeBlobReset(pRtree);
            if rc == crate::src::headers::sqlite3_h::SQLITE_NOMEM {
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
        }
    }
    if __pRtree_ref.pNodeBlob.is_null() {
        rc = crate::src::src::vdbeblob::sqlite3_blob_open(
            __pRtree_ref.db,
            __pRtree_ref.zDb,
            __pRtree_ref.zNodeName,
            b"data\0" as *const u8 as *const ::core::ffi::c_char,
            iNode as crate::src::headers::sqlite3_h::Sqlite3Int64,
            0 as ::core::ffi::c_int,
            &raw mut __pRtree_ref.pNodeBlob,
        );
    }
    if rc != 0 {
        *ppNode = ::core::ptr::null_mut::<RtreeNode>();
        if rc == crate::src::headers::sqlite3_h::SQLITE_ERROR {
            rc = crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
        }
    } else if __pRtree_ref.iNodeSize
        == crate::src::src::vdbeblob::sqlite3_blob_bytes(__pRtree_ref.pNodeBlob)
    {
        pNode = crate::src::src::malloc::sqlite3_malloc64(
            (::core::mem::size_of::<RtreeNode>() as usize)
                .wrapping_add(__pRtree_ref.iNodeSize as usize)
                as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut RtreeNode;
        if pNode.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            let __pNode_ref = { &mut *pNode };
            __pNode_ref.pParent = pParent;
            __pNode_ref.zData = pNode.offset(1_isize) as *mut RtreeNode as *mut U8_0;
            __pNode_ref.nRef = 1 as ::core::ffi::c_int;
            __pRtree_ref.nNodeRef = __pRtree_ref.nNodeRef.wrapping_add(1);
            __pNode_ref.iNode = iNode;
            __pNode_ref.isDirty = 0 as ::core::ffi::c_int;
            __pNode_ref.pNext = ::core::ptr::null_mut::<RtreeNode>();
            rc = crate::src::src::vdbeblob::sqlite3_blob_read(
                __pRtree_ref.pNodeBlob,
                __pNode_ref.zData as *mut ::core::ffi::c_void,
                __pRtree_ref.iNodeSize,
                0 as ::core::ffi::c_int,
            );
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !pNode.is_null() && iNode == 1 as I64_0 {
        __pRtree_ref.iDepth = readInt16((*pNode).zData);
        if __pRtree_ref.iDepth > RTREE_MAX_DEPTH {
            rc = crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
        }
    }
    if !pNode.is_null() && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if readInt16((*pNode).zData.offset(2_isize) as *mut U8_0)
            > (__pRtree_ref.iNodeSize - 4 as ::core::ffi::c_int)
                / __pRtree_ref.nBytesPerCell as ::core::ffi::c_int
        {
            rc = crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if !pNode.is_null() {
            nodeReference(pParent);
            nodeHashInsert(pRtree, pNode);
        } else {
            rc = crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
        }
        *ppNode = pNode;
    } else {
        nodeBlobReset(pRtree);
        if !pNode.is_null() {
            __pRtree_ref.nNodeRef = __pRtree_ref.nNodeRef.wrapping_sub(1);
            crate::src::src::malloc::sqlite3_free(pNode as *mut ::core::ffi::c_void);
        }
        *ppNode = ::core::ptr::null_mut::<RtreeNode>();
    }
    rc
}

unsafe extern "C" fn nodeOverwriteCell(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
    pCell: *mut RtreeCell,
    iCell: ::core::ffi::c_int,
) {
    let mut ii: ::core::ffi::c_int;
    let mut p: *mut U8_0 = (*pNode).zData.offset(
        (4 as ::core::ffi::c_int + (*pRtree).nBytesPerCell as ::core::ffi::c_int * iCell) as isize,
    ) as *mut U8_0;
    p = p.offset(writeInt64(p, (*pCell).iRowid) as isize);
    ii = 0 as ::core::ffi::c_int;
    while ii < (*pRtree).nDim2 as ::core::ffi::c_int {
        p = p.offset(writeCoord(
            p,
            (&raw mut (*pCell).aCoord as *mut RtreeCoord).offset(ii as isize) as *mut RtreeCoord,
        ) as isize);
        ii += 1;
    }
    (*pNode).isDirty = 1 as ::core::ffi::c_int;
}

unsafe extern "C" fn nodeDeleteCell(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
    iCell: ::core::ffi::c_int,
) {
    let __pRtree_ref = { &*pRtree };
    let __pNode_ref = { &mut *pNode };
    let pDst: *mut U8_0 = __pNode_ref.zData.offset(
        (4 as ::core::ffi::c_int + __pRtree_ref.nBytesPerCell as ::core::ffi::c_int * iCell)
            as isize,
    ) as *mut U8_0;
    let pSrc: *mut U8_0 = pDst.offset(__pRtree_ref.nBytesPerCell as isize) as *mut U8_0;
    let nByte: ::core::ffi::c_int =
        (readInt16(__pNode_ref.zData.offset(2_isize) as *mut U8_0)
            - iCell
            - 1 as ::core::ffi::c_int)
            * __pRtree_ref.nBytesPerCell as ::core::ffi::c_int;
    ::core::ptr::copy(pSrc as *const u8, pDst as *mut u8, nByte as usize);
    writeInt16(
        __pNode_ref.zData.offset(2_isize) as *mut U8_0,
        readInt16(__pNode_ref.zData.offset(2_isize) as *mut U8_0) - 1 as ::core::ffi::c_int,
    );
    __pNode_ref.isDirty = 1 as ::core::ffi::c_int;
}

unsafe extern "C" fn nodeInsertCell(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
    pCell: *mut RtreeCell,
) -> ::core::ffi::c_int {
    
    
    let nMaxCell: ::core::ffi::c_int = ((*pRtree).iNodeSize - 4 as ::core::ffi::c_int)
        / (*pRtree).nBytesPerCell as ::core::ffi::c_int;
    let nCell: ::core::ffi::c_int = readInt16((*pNode).zData.offset(2_isize) as *mut U8_0);
    if nCell < nMaxCell {
        nodeOverwriteCell(pRtree, pNode, pCell, nCell);
        writeInt16(
            (*pNode).zData.offset(2_isize) as *mut U8_0,
            nCell + 1 as ::core::ffi::c_int,
        );
        (*pNode).isDirty = 1 as ::core::ffi::c_int;
    }
    (nCell == nMaxCell) as ::core::ffi::c_int
}

unsafe extern "C" fn nodeWrite(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pNode).isDirty != 0 {
        let p: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt = (*pRtree).pWriteNode;
        let __pNode_ref = { &mut *pNode };
        if __pNode_ref.iNode != 0 {
            crate::src::src::vdbeapi::sqlite3_bind_int64(
                p,
                1 as ::core::ffi::c_int,
                __pNode_ref.iNode as crate::src::headers::sqlite3_h::Sqlite3Int64,
            );
        } else {
            crate::src::src::vdbeapi::sqlite3_bind_null(p, 1 as ::core::ffi::c_int);
        }
        crate::src::src::vdbeapi::sqlite3_bind_blob(
            p,
            2 as ::core::ffi::c_int,
            __pNode_ref.zData as *const ::core::ffi::c_void,
            (*pRtree).iNodeSize,
            crate::src::headers::sqlite3_h::SQLITE_STATIC,
        );
        crate::src::src::vdbeapi::sqlite3_step(p);
        __pNode_ref.isDirty = 0 as ::core::ffi::c_int;
        rc = crate::src::src::vdbeapi::sqlite3_reset(p);
        crate::src::src::vdbeapi::sqlite3_bind_null(p, 2 as ::core::ffi::c_int);
        if __pNode_ref.iNode == 0 as I64_0 && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            __pNode_ref.iNode =
                crate::src::src::main::sqlite3_last_insert_rowid((*pRtree).db) as I64_0;
            nodeHashInsert(pRtree, pNode);
        }
    }
    rc
}

unsafe extern "C" fn nodeRelease(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if !pNode.is_null() {
        (*pNode).nRef -= 1;
        if (*pNode).nRef == 0 as ::core::ffi::c_int {
            (*pRtree).nNodeRef = (*pRtree).nNodeRef.wrapping_sub(1);
            if (*pNode).iNode == 1 as I64_0 {
                (*pRtree).iDepth = -(1 as ::core::ffi::c_int);
            }
            if !(*pNode).pParent.is_null() {
                rc = nodeRelease(pRtree, (*pNode).pParent);
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = nodeWrite(pRtree, pNode);
            }
            nodeHashDelete(pRtree, pNode);
            crate::src::src::malloc::sqlite3_free(pNode as *mut ::core::ffi::c_void);
        }
    }
    rc
}

unsafe extern "C" fn nodeGetRowid(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
    iCell: ::core::ffi::c_int,
) -> I64_0 {
    readInt64((*pNode).zData.offset(
        (4 as ::core::ffi::c_int + (*pRtree).nBytesPerCell as ::core::ffi::c_int * iCell) as isize,
    ) as *mut U8_0)
}

unsafe extern "C" fn nodeGetCoord(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
    iCell: ::core::ffi::c_int,
    iCoord: ::core::ffi::c_int,
    pCoord: *mut RtreeCoord,
) {
    readCoord(
        (*pNode).zData.offset(
            (12 as ::core::ffi::c_int
                + (*pRtree).nBytesPerCell as ::core::ffi::c_int * iCell
                + 4 as ::core::ffi::c_int * iCoord) as isize,
        ) as *mut U8_0,
        pCoord,
    );
}

unsafe extern "C" fn nodeGetCell(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
    iCell: ::core::ffi::c_int,
    pCell: *mut RtreeCell,
) {
    let mut pData: *mut U8_0;
    
    let mut ii: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*pCell).iRowid = nodeGetRowid(pRtree, pNode, iCell);
    pData = (*pNode).zData.offset(
        (12 as ::core::ffi::c_int + (*pRtree).nBytesPerCell as ::core::ffi::c_int * iCell) as isize,
    );
    let pCoord: *mut RtreeCoord = &raw mut (*pCell).aCoord as *mut RtreeCoord;
    loop {
        readCoord(pData, pCoord.offset(ii as isize) as *mut RtreeCoord);
        readCoord(
            pData.offset(4_isize),
            pCoord.offset((ii + 1 as ::core::ffi::c_int) as isize) as *mut RtreeCoord,
        );
        pData = pData.offset(8_isize);
        ii += 2 as ::core::ffi::c_int;
        if (ii >= (*pRtree).nDim2 as ::core::ffi::c_int) {
            break;
        }
    }
}

unsafe extern "C" fn rtreeCreate(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pAux: *mut ::core::ffi::c_void,
    argc: ::core::ffi::c_int,
    argv: *const *const ::core::ffi::c_char,
    ppVtab: *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    rtreeInit(db, pAux, argc, argv, ppVtab, pzErr, 1 as ::core::ffi::c_int)
}

unsafe extern "C" fn rtreeConnect(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pAux: *mut ::core::ffi::c_void,
    argc: ::core::ffi::c_int,
    argv: *const *const ::core::ffi::c_char,
    ppVtab: *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    rtreeInit(db, pAux, argc, argv, ppVtab, pzErr, 0 as ::core::ffi::c_int)
}

unsafe extern "C" fn rtreeReference(pRtree: *mut Rtree) {
    (*pRtree).nBusy = (*pRtree).nBusy.wrapping_add(1);
}

unsafe extern "C" fn rtreeRelease(pRtree: *mut Rtree) {
    let __pRtree_ref = { &mut *pRtree };
    __pRtree_ref.nBusy = __pRtree_ref.nBusy.wrapping_sub(1);
    if __pRtree_ref.nBusy == 0 as U32_0 {
        __pRtree_ref.inWrTrans = 0 as U8_0;
        nodeBlobReset(pRtree);
        crate::src::src::vdbeapi::sqlite3_finalize(__pRtree_ref.pWriteNode);
        crate::src::src::vdbeapi::sqlite3_finalize(__pRtree_ref.pDeleteNode);
        crate::src::src::vdbeapi::sqlite3_finalize(__pRtree_ref.pReadRowid);
        crate::src::src::vdbeapi::sqlite3_finalize(__pRtree_ref.pWriteRowid);
        crate::src::src::vdbeapi::sqlite3_finalize(__pRtree_ref.pDeleteRowid);
        crate::src::src::vdbeapi::sqlite3_finalize(__pRtree_ref.pReadParent);
        crate::src::src::vdbeapi::sqlite3_finalize(__pRtree_ref.pWriteParent);
        crate::src::src::vdbeapi::sqlite3_finalize(__pRtree_ref.pDeleteParent);
        crate::src::src::vdbeapi::sqlite3_finalize(__pRtree_ref.pWriteAux);
        crate::src::src::malloc::sqlite3_free(__pRtree_ref.zReadAuxSql as *mut ::core::ffi::c_void);
        crate::src::src::malloc::sqlite3_free(pRtree as *mut ::core::ffi::c_void);
    }
}

unsafe extern "C" fn rtreeDisconnect(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
) -> ::core::ffi::c_int {
    rtreeRelease(pVtab as *mut Rtree);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn rtreeDestroy(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
) -> ::core::ffi::c_int {
    let pRtree: *mut Rtree = pVtab as *mut Rtree;
    let rc: ::core::ffi::c_int;
    let __pRtree_ref = { &*pRtree };
    let zCreate: *mut ::core::ffi::c_char = crate::sqlite_printf!(
        "DROP TABLE '%q'.'%q_node';DROP TABLE '%q'.'%q_rowid';DROP TABLE '%q'.'%q_parent';",
        __pRtree_ref.zDb,
        __pRtree_ref.zName,
        __pRtree_ref.zDb,
        __pRtree_ref.zName,
        __pRtree_ref.zDb,
        __pRtree_ref.zName,
    );
    if zCreate.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    } else {
        nodeBlobReset(pRtree);
        rc = crate::src::src::legacy::sqlite3_exec(
            __pRtree_ref.db,
            zCreate,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        crate::src::src::malloc::sqlite3_free(zCreate as *mut ::core::ffi::c_void);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rtreeRelease(pRtree);
    }
    rc
}

unsafe extern "C" fn rtreeOpen(
    pVTab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    ppCursor: *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    let pRtree: *mut Rtree = pVTab as *mut Rtree;
    
    let pCsr: *mut RtreeCursor = crate::src::src::malloc::sqlite3_malloc64(
        ::core::mem::size_of::<RtreeCursor>() as crate::src::headers::sqlite3_h::Sqlite3Uint64
    ) as *mut RtreeCursor;
    if !pCsr.is_null() {
        ::libc::memset(
            pCsr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<RtreeCursor>() as crate::__stddef_size_t_h::SizeT,
        );
        (*pCsr).base.pVtab = pVTab;
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        (*pRtree).nCursor = (*pRtree).nCursor.wrapping_add(1);
    }
    *ppCursor = pCsr as *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;
    rc
}

unsafe extern "C" fn resetCursor(pCsr: *mut RtreeCursor) {
    let __pCsr_ref = { &mut *pCsr };
    let pRtree: *mut Rtree = __pCsr_ref.base.pVtab as *mut Rtree;
    let mut ii: ::core::ffi::c_int;
    
    if !__pCsr_ref.aConstraint.is_null() {
        let mut i: ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < __pCsr_ref.nConstraint {
            let pInfo: *mut crate::src::headers::sqlite3_h::sqlite3_rtree_query_info =
                (*__pCsr_ref.aConstraint.offset(i as isize)).pInfo;
            if !pInfo.is_null() {
                if (*pInfo).xDelUser.is_some() {
                    (*pInfo).xDelUser.expect("non-null function pointer")((*pInfo).pUser);
                }
                crate::src::src::malloc::sqlite3_free(pInfo as *mut ::core::ffi::c_void);
            }
            i += 1;
        }
        crate::src::src::malloc::sqlite3_free(__pCsr_ref.aConstraint as *mut ::core::ffi::c_void);
        __pCsr_ref.aConstraint = ::core::ptr::null_mut::<RtreeConstraint>();
    }
    ii = 0 as ::core::ffi::c_int;
    while ii < RTREE_CACHE_SZ {
        nodeRelease(pRtree, __pCsr_ref.aNode[ii as usize]);
        ii += 1;
    }
    crate::src::src::malloc::sqlite3_free(__pCsr_ref.aPoint as *mut ::core::ffi::c_void);
    let pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt = __pCsr_ref.pReadAux;
    ::libc::memset(
        pCsr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<RtreeCursor>() as crate::__stddef_size_t_h::SizeT,
    );
    __pCsr_ref.base.pVtab = pRtree as *mut crate::src::headers::sqlite3_h::sqlite3_vtab;
    __pCsr_ref.pReadAux = pStmt;
    crate::src::src::vdbeapi::sqlite3_reset(pStmt);
}

unsafe extern "C" fn rtreeClose(
    cur: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let pRtree: *mut Rtree = (*cur).pVtab as *mut Rtree;
    let pCsr: *mut RtreeCursor = cur as *mut RtreeCursor;
    resetCursor(pCsr);
    crate::src::src::vdbeapi::sqlite3_finalize((*pCsr).pReadAux);
    crate::src::src::malloc::sqlite3_free(pCsr as *mut ::core::ffi::c_void);
    let __pRtree_ref = { &mut *pRtree };
    __pRtree_ref.nCursor = __pRtree_ref.nCursor.wrapping_sub(1);
    if __pRtree_ref.nCursor == 0 as U32_0
        && __pRtree_ref.inWrTrans as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        nodeBlobReset(pRtree);
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn rtreeEof(
    cur: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let pCsr = &*(cur as *mut RtreeCursor);
    pCsr.atEOF as ::core::ffi::c_int
}

unsafe extern "C" fn rtreeCallbackConstraint(
    pConstraint: *mut RtreeConstraint,
    eInt: ::core::ffi::c_int,
    mut pCellData: *mut U8_0,
    pSearch: *mut RtreeSearchPoint,
    prScore: *mut crate::src::headers::sqlite3_h::Sqlite3RtreeDbl,
    peWithin: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pConstraint_ref = { &mut *pConstraint };
    let pInfo: *mut crate::src::headers::sqlite3_h::sqlite3_rtree_query_info =
        __pConstraint_ref.pInfo;
    let nCoord: ::core::ffi::c_int = (*pInfo).nCoord;
    let rc: ::core::ffi::c_int;
    let mut c: RtreeCoord = RtreeCoord { f: 0. };
    let mut aCoord: [crate::src::headers::sqlite3_h::Sqlite3RtreeDbl; 10] = [0.; 10];
    if __pConstraint_ref.op == RTREE_QUERY
        && (*pSearch).iLevel as ::core::ffi::c_int == 1 as ::core::ffi::c_int
    {
        (*pInfo).iRowid = readInt64(pCellData) as crate::src::headers::sqlite3_h::Sqlite3Int64;
    }
    pCellData = pCellData.offset(8_isize);
    if eInt == 0 as ::core::ffi::c_int {
        let mut current_block_25: u64;
        match nCoord {
            10 => {
                readCoord(pCellData.offset(36_isize), &raw mut c);
                aCoord[9 as ::core::ffi::c_int as usize] =
                    c.f as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
                readCoord(pCellData.offset(32_isize), &raw mut c);
                aCoord[8 as ::core::ffi::c_int as usize] =
                    c.f as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
                current_block_25 = 17245806410740177172;
            }
            8 => {
                current_block_25 = 17245806410740177172;
            }
            6 => {
                current_block_25 = 6238933575658261055;
            }
            4 => {
                current_block_25 = 14661562966503102838;
            }
            _ => {
                current_block_25 = 17345315872032947249;
            }
        }
        match current_block_25 {
            17245806410740177172 => {
                readCoord(pCellData.offset(28_isize), &raw mut c);
                aCoord[7 as ::core::ffi::c_int as usize] =
                    c.f as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
                readCoord(pCellData.offset(24_isize), &raw mut c);
                aCoord[6 as ::core::ffi::c_int as usize] =
                    c.f as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
                current_block_25 = 6238933575658261055;
            }
            _ => {}
        }
        match current_block_25 {
            6238933575658261055 => {
                readCoord(pCellData.offset(20_isize), &raw mut c);
                aCoord[5 as ::core::ffi::c_int as usize] =
                    c.f as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
                readCoord(pCellData.offset(16_isize), &raw mut c);
                aCoord[4 as ::core::ffi::c_int as usize] =
                    c.f as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
                current_block_25 = 14661562966503102838;
            }
            _ => {}
        }
        match current_block_25 {
            14661562966503102838 => {
                readCoord(pCellData.offset(12_isize), &raw mut c);
                aCoord[3 as ::core::ffi::c_int as usize] =
                    c.f as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
                readCoord(pCellData.offset(8_isize), &raw mut c);
                aCoord[2 as ::core::ffi::c_int as usize] =
                    c.f as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
            }
            _ => {}
        }
        readCoord(pCellData.offset(4_isize), &raw mut c);
        aCoord[1 as ::core::ffi::c_int as usize] =
            c.f as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
        readCoord(pCellData, &raw mut c);
        aCoord[0 as ::core::ffi::c_int as usize] =
            c.f as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
    } else {
        let mut current_block_47: u64;
        match nCoord {
            10 => {
                readCoord(pCellData.offset(36_isize), &raw mut c);
                aCoord[9 as ::core::ffi::c_int as usize] =
                    c.i as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
                readCoord(pCellData.offset(32_isize), &raw mut c);
                aCoord[8 as ::core::ffi::c_int as usize] =
                    c.i as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
                current_block_47 = 6970119114131639077;
            }
            8 => {
                current_block_47 = 6970119114131639077;
            }
            6 => {
                current_block_47 = 9261908759940751603;
            }
            4 => {
                current_block_47 = 14392866125239670488;
            }
            _ => {
                current_block_47 = 12071572069003698848;
            }
        }
        match current_block_47 {
            6970119114131639077 => {
                readCoord(pCellData.offset(28_isize), &raw mut c);
                aCoord[7 as ::core::ffi::c_int as usize] =
                    c.i as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
                readCoord(pCellData.offset(24_isize), &raw mut c);
                aCoord[6 as ::core::ffi::c_int as usize] =
                    c.i as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
                current_block_47 = 9261908759940751603;
            }
            _ => {}
        }
        match current_block_47 {
            9261908759940751603 => {
                readCoord(pCellData.offset(20_isize), &raw mut c);
                aCoord[5 as ::core::ffi::c_int as usize] =
                    c.i as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
                readCoord(pCellData.offset(16_isize), &raw mut c);
                aCoord[4 as ::core::ffi::c_int as usize] =
                    c.i as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
                current_block_47 = 14392866125239670488;
            }
            _ => {}
        }
        match current_block_47 {
            14392866125239670488 => {
                readCoord(pCellData.offset(12_isize), &raw mut c);
                aCoord[3 as ::core::ffi::c_int as usize] =
                    c.i as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
                readCoord(pCellData.offset(8_isize), &raw mut c);
                aCoord[2 as ::core::ffi::c_int as usize] =
                    c.i as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
            }
            _ => {}
        }
        readCoord(pCellData.offset(4_isize), &raw mut c);
        aCoord[1 as ::core::ffi::c_int as usize] =
            c.i as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
        readCoord(pCellData, &raw mut c);
        aCoord[0 as ::core::ffi::c_int as usize] =
            c.i as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
    }
    if __pConstraint_ref.op == RTREE_MATCH {
        let mut eWithin: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        rc = __pConstraint_ref
            .u
            .xGeom
            .expect("non-null function pointer")(
            pInfo as *mut crate::src::headers::sqlite3_h::sqlite3_rtree_geometry,
            nCoord,
            &raw mut aCoord as *mut RtreeDValue,
            &raw mut eWithin,
        );
        if eWithin == 0 as ::core::ffi::c_int {
            *peWithin = crate::src::headers::sqlite3_h::NOT_WITHIN;
        }
        *prScore = RTREE_ZERO as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
    } else {
        let __pInfo_ref = { &mut *pInfo };
        __pInfo_ref.aCoord =
            &raw mut aCoord as *mut crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
        let __pSearch_ref = { &*pSearch };
        __pInfo_ref.iLevel = __pSearch_ref.iLevel as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        __pInfo_ref.rParentScore =
            __pSearch_ref.rScore as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
        __pInfo_ref.rScore = __pInfo_ref.rParentScore;
        __pInfo_ref.eParentWithin = __pSearch_ref.eWithin as ::core::ffi::c_int;
        __pInfo_ref.eWithin = __pInfo_ref.eParentWithin;
        rc = (*pConstraint)
            .u
            .xQueryFunc
            .expect("non-null function pointer")(pInfo);
        if __pInfo_ref.eWithin < *peWithin {
            *peWithin = __pInfo_ref.eWithin;
        }
        if __pInfo_ref.rScore < *prScore || *prScore < RTREE_ZERO {
            *prScore = __pInfo_ref.rScore;
        }
    }
    rc
}

unsafe extern "C" fn rtreeNonleafConstraint(
    p: *mut RtreeConstraint,
    eInt: ::core::ffi::c_int,
    mut pCellData: *mut U8_0,
    peWithin: *mut ::core::ffi::c_int,
) {
    let mut val: crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
    pCellData = pCellData.offset(
        (8 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * ((*p).iCoord & 0xfe as ::core::ffi::c_int))
            as isize,
    );
    match (*p).op {
        RTREE_TRUE => return,
        RTREE_FALSE => {}
        RTREE_EQ => {
            let mut c: RtreeCoord = RtreeCoord { f: 0. };
            ::core::ptr::copy_nonoverlapping(
                pCellData as *const u8,
                &raw mut c.u as *mut u8,
                4_usize,
            );
            c.u = c.u >> 24 as ::core::ffi::c_int & 0xff as U32_0
                | c.u >> 8 as ::core::ffi::c_int & 0xff00 as U32_0
                | (c.u & 0xff as U32_0) << 24 as ::core::ffi::c_int
                | (c.u & 0xff00 as U32_0) << 8 as ::core::ffi::c_int;
            val = if eInt != 0 {
                c.i as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl
            } else {
                c.f as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl
            };
            if (*p).u.rValue >= val {
                pCellData = pCellData.offset(4_isize);
                let mut c_0: RtreeCoord = RtreeCoord { f: 0. };
                ::core::ptr::copy_nonoverlapping(
                    pCellData as *const u8,
                    &raw mut c_0.u as *mut u8,
                    4_usize,
                );
                c_0.u = c_0.u >> 24 as ::core::ffi::c_int & 0xff as U32_0
                    | c_0.u >> 8 as ::core::ffi::c_int & 0xff00 as U32_0
                    | (c_0.u & 0xff as U32_0) << 24 as ::core::ffi::c_int
                    | (c_0.u & 0xff00 as U32_0) << 8 as ::core::ffi::c_int;
                val = if eInt != 0 {
                    c_0.i as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl
                } else {
                    c_0.f as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl
                };
                if (*p).u.rValue <= val {
                    return;
                }
            }
        }
        RTREE_LE | RTREE_LT => {
            let mut c_1: RtreeCoord = RtreeCoord { f: 0. };
            ::core::ptr::copy_nonoverlapping(
                pCellData as *const u8,
                &raw mut c_1.u as *mut u8,
                4_usize,
            );
            c_1.u = c_1.u >> 24 as ::core::ffi::c_int & 0xff as U32_0
                | c_1.u >> 8 as ::core::ffi::c_int & 0xff00 as U32_0
                | (c_1.u & 0xff as U32_0) << 24 as ::core::ffi::c_int
                | (c_1.u & 0xff00 as U32_0) << 8 as ::core::ffi::c_int;
            val = if eInt != 0 {
                c_1.i as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl
            } else {
                c_1.f as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl
            };
            if (*p).u.rValue >= val {
                return;
            }
        }
        _ => {
            pCellData = pCellData.offset(4_isize);
            let mut c_2: RtreeCoord = RtreeCoord { f: 0. };
            ::core::ptr::copy_nonoverlapping(
                pCellData as *const u8,
                &raw mut c_2.u as *mut u8,
                4_usize,
            );
            c_2.u = c_2.u >> 24 as ::core::ffi::c_int & 0xff as U32_0
                | c_2.u >> 8 as ::core::ffi::c_int & 0xff00 as U32_0
                | (c_2.u & 0xff as U32_0) << 24 as ::core::ffi::c_int
                | (c_2.u & 0xff00 as U32_0) << 8 as ::core::ffi::c_int;
            val = if eInt != 0 {
                c_2.i as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl
            } else {
                c_2.f as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl
            };
            if (*p).u.rValue <= val {
                return;
            }
        }
    }
    *peWithin = crate::src::headers::sqlite3_h::NOT_WITHIN;
}

unsafe extern "C" fn rtreeLeafConstraint(
    p: *mut RtreeConstraint,
    eInt: ::core::ffi::c_int,
    mut pCellData: *mut U8_0,
    peWithin: *mut ::core::ffi::c_int,
) {
    
    pCellData = pCellData
        .offset((8 as ::core::ffi::c_int + (*p).iCoord * 4 as ::core::ffi::c_int) as isize);
    let mut c: RtreeCoord = RtreeCoord { f: 0. };
    ::core::ptr::copy_nonoverlapping(pCellData as *const u8, &raw mut c.u as *mut u8, 4_usize);
    c.u = c.u >> 24 as ::core::ffi::c_int & 0xff as U32_0
        | c.u >> 8 as ::core::ffi::c_int & 0xff00 as U32_0
        | (c.u & 0xff as U32_0) << 24 as ::core::ffi::c_int
        | (c.u & 0xff00 as U32_0) << 8 as ::core::ffi::c_int;
    let xN: RtreeDValue = (if eInt != 0 {
        c.i as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl
    } else {
        c.f as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl
    }) as RtreeDValue;
    match (*p).op {
        RTREE_TRUE => return,
        RTREE_FALSE => {}
        RTREE_LE => {
            if xN <= (*p).u.rValue {
                return;
            }
        }
        RTREE_LT => {
            if xN < (*p).u.rValue {
                return;
            }
        }
        RTREE_GE => {
            if xN >= (*p).u.rValue {
                return;
            }
        }
        RTREE_GT => {
            if xN > (*p).u.rValue {
                return;
            }
        }
        _ => {
            if xN == (*p).u.rValue {
                return;
            }
        }
    }
    *peWithin = crate::src::headers::sqlite3_h::NOT_WITHIN;
}

unsafe extern "C" fn nodeRowidIndex(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
    iRowid: I64_0,
    piIndex: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ii: ::core::ffi::c_int;
    let nCell: ::core::ffi::c_int = readInt16((*pNode).zData.offset(2_isize) as *mut U8_0);
    ii = 0 as ::core::ffi::c_int;
    while ii < nCell {
        if nodeGetRowid(pRtree, pNode, ii) == iRowid {
            *piIndex = ii;
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        ii += 1;
    }
    crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB
}

unsafe extern "C" fn nodeParentIndex(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
    piIndex: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pParent: *mut RtreeNode = (*pNode).pParent;
    if !pParent.is_null() {
        nodeRowidIndex(pRtree, pParent, (*pNode).iNode, piIndex)
    } else {
        *piIndex = -(1 as ::core::ffi::c_int);
        crate::src::headers::sqlite3_h::SQLITE_OK
    }
}

unsafe extern "C" fn rtreeSearchPointCompare(
    pA: *const RtreeSearchPoint,
    pB: *const RtreeSearchPoint,
) -> ::core::ffi::c_int {
    let __pB_ref = { &*pB };
    let __pA_ref = { &*pA };
    if __pA_ref.rScore < __pB_ref.rScore {
        return -(1 as ::core::ffi::c_int);
    }
    if __pA_ref.rScore > __pB_ref.rScore {
        return 1 as ::core::ffi::c_int;
    }
    if (__pA_ref.iLevel as ::core::ffi::c_int) < __pB_ref.iLevel as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if __pA_ref.iLevel as ::core::ffi::c_int > __pB_ref.iLevel as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn rtreeSearchPointSwap(
    p: *mut RtreeCursor,
    mut i: ::core::ffi::c_int,
    mut j: ::core::ffi::c_int,
) {
    let __p_ref = { &mut *p };
    let t: RtreeSearchPoint = *__p_ref.aPoint.offset(i as isize);
    *__p_ref.aPoint.offset(i as isize) = *__p_ref.aPoint.offset(j as isize);
    *__p_ref.aPoint.offset(j as isize) = t;
    i += 1;
    j += 1;
    if i < RTREE_CACHE_SZ {
        if j >= RTREE_CACHE_SZ {
            nodeRelease(__p_ref.base.pVtab as *mut Rtree, __p_ref.aNode[i as usize]);
            __p_ref.aNode[i as usize] = ::core::ptr::null_mut::<RtreeNode>();
        } else {
            let pTemp: *mut RtreeNode = __p_ref.aNode[i as usize];
            __p_ref.aNode[i as usize] = __p_ref.aNode[j as usize];
            __p_ref.aNode[j as usize] = pTemp;
        }
    }
}

unsafe extern "C" fn rtreeSearchPointFirst(pCur: *mut RtreeCursor) -> *mut RtreeSearchPoint {
    if (*pCur).bPoint as ::core::ffi::c_int != 0 {
        &raw mut (*pCur).sPoint
    } else if (*pCur).nPoint != 0 {
        (*pCur).aPoint
    } else {
        ::core::ptr::null_mut::<RtreeSearchPoint>()
    }
}

unsafe extern "C" fn rtreeNodeOfFirstSearchPoint(
    pCur: *mut RtreeCursor,
    pRC: *mut ::core::ffi::c_int,
) -> *mut RtreeNode {
    let id: crate::src::headers::sqlite3_h::Sqlite3Int64;
    let __pCur_ref = { &mut *pCur };
    let ii: ::core::ffi::c_int =
        1 as ::core::ffi::c_int - __pCur_ref.bPoint as ::core::ffi::c_int;
    if __pCur_ref.aNode[ii as usize].is_null() {
        id = if ii != 0 {
            (*__pCur_ref.aPoint.offset(0_isize)).id
        } else {
            __pCur_ref.sPoint.id
        };
        *pRC = nodeAcquire(
            __pCur_ref.base.pVtab as *mut Rtree,
            id as I64_0,
            ::core::ptr::null_mut::<RtreeNode>(),
            (&raw mut __pCur_ref.aNode as *mut *mut RtreeNode).offset(ii as isize)
                as *mut *mut RtreeNode,
        );
    }
    __pCur_ref.aNode[ii as usize]
}

unsafe extern "C" fn rtreeEnqueue(
    pCur: *mut RtreeCursor,
    rScore: RtreeDValue,
    iLevel: U8_0,
) -> *mut RtreeSearchPoint {
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    let mut pNew: *mut RtreeSearchPoint;
    let __pCur_ref = { &mut *pCur };
    if __pCur_ref.nPoint >= __pCur_ref.nPointAlloc {
        let nNew: ::core::ffi::c_int =
            __pCur_ref.nPointAlloc * 2 as ::core::ffi::c_int + 8 as ::core::ffi::c_int;
        pNew = crate::src::src::malloc::sqlite3_realloc64(
            __pCur_ref.aPoint as *mut ::core::ffi::c_void,
            (nNew as usize).wrapping_mul(::core::mem::size_of::<RtreeSearchPoint>() as usize)
                as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut RtreeSearchPoint;
        if pNew.is_null() {
            return ::core::ptr::null_mut::<RtreeSearchPoint>();
        }
        __pCur_ref.aPoint = pNew;
        __pCur_ref.nPointAlloc = nNew;
    }
    let fresh2 = __pCur_ref.nPoint;
    __pCur_ref.nPoint += 1;
    i = fresh2;
    pNew = __pCur_ref.aPoint.offset(i as isize);
    (*pNew).rScore = rScore;
    (*pNew).iLevel = iLevel;
    while i > 0 as ::core::ffi::c_int {
        
        j = (i - 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
        let pParent: *mut RtreeSearchPoint = __pCur_ref.aPoint.offset(j as isize);
        if rtreeSearchPointCompare(pNew, pParent) >= 0 as ::core::ffi::c_int {
            break;
        }
        rtreeSearchPointSwap(pCur, j, i);
        i = j;
        pNew = pParent;
    }
    pNew
}

unsafe extern "C" fn rtreeSearchPointNew(
    pCur: *mut RtreeCursor,
    rScore: RtreeDValue,
    iLevel: U8_0,
) -> *mut RtreeSearchPoint {
    let pNew: *mut RtreeSearchPoint;
    
    let pFirst: *mut RtreeSearchPoint = rtreeSearchPointFirst(pCur);
    (*pCur).anQueue[iLevel as usize] = (*pCur).anQueue[iLevel as usize].wrapping_add(1);
    if pFirst.is_null()
        || (*pFirst).rScore > rScore
        || (*pFirst).rScore == rScore
            && (*pFirst).iLevel as ::core::ffi::c_int > iLevel as ::core::ffi::c_int
    {
        let __pCur_ref = { &mut *pCur };
        if __pCur_ref.bPoint != 0 {
            
            pNew = rtreeEnqueue(pCur, rScore, iLevel);
            if pNew.is_null() {
                return ::core::ptr::null_mut::<RtreeSearchPoint>();
            }
            let ii: ::core::ffi::c_int = pNew.offset_from(__pCur_ref.aPoint) as ::core::ffi::c_long as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int;
            if ii < 5 as ::core::ffi::c_int {
                __pCur_ref.aNode[ii as usize] = __pCur_ref.aNode[0 as ::core::ffi::c_int as usize];
            } else {
                nodeRelease(
                    __pCur_ref.base.pVtab as *mut Rtree,
                    __pCur_ref.aNode[0 as ::core::ffi::c_int as usize],
                );
            }
            __pCur_ref.aNode[0 as ::core::ffi::c_int as usize] =
                ::core::ptr::null_mut::<RtreeNode>();
            *pNew = __pCur_ref.sPoint;
        }
        __pCur_ref.sPoint.rScore = rScore;
        __pCur_ref.sPoint.iLevel = iLevel;
        __pCur_ref.bPoint = 1 as U8_0;
        &raw mut __pCur_ref.sPoint
    } else {
        rtreeEnqueue(pCur, rScore, iLevel)
    }
}

unsafe extern "C" fn rtreeSearchPointPop(p: *mut RtreeCursor) {
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    let mut k: ::core::ffi::c_int;
    let n: ::core::ffi::c_int;
    let __p_ref = { &mut *p };
    i = 1 as ::core::ffi::c_int - __p_ref.bPoint as ::core::ffi::c_int;
    if !__p_ref.aNode[i as usize].is_null() {
        nodeRelease(__p_ref.base.pVtab as *mut Rtree, __p_ref.aNode[i as usize]);
        __p_ref.aNode[i as usize] = ::core::ptr::null_mut::<RtreeNode>();
    }
    if __p_ref.bPoint != 0 {
        __p_ref.anQueue[__p_ref.sPoint.iLevel as usize] =
            __p_ref.anQueue[__p_ref.sPoint.iLevel as usize].wrapping_sub(1);
        __p_ref.bPoint = 0 as U8_0;
    } else if __p_ref.nPoint != 0 {
        __p_ref.anQueue[(*__p_ref.aPoint.offset(0_isize)).iLevel as usize] =
            __p_ref.anQueue[(*__p_ref.aPoint.offset(0_isize)).iLevel as usize].wrapping_sub(1);
        __p_ref.nPoint -= 1;
        n = __p_ref.nPoint;
        *__p_ref.aPoint.offset(0_isize) = *__p_ref.aPoint.offset(n as isize);
        if n < RTREE_CACHE_SZ - 1 as ::core::ffi::c_int {
            __p_ref.aNode[1 as ::core::ffi::c_int as usize] =
                __p_ref.aNode[(n + 1 as ::core::ffi::c_int) as usize];
            __p_ref.aNode[(n + 1 as ::core::ffi::c_int) as usize] =
                ::core::ptr::null_mut::<RtreeNode>();
        }
        i = 0 as ::core::ffi::c_int;
        loop {
            j = i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
            if (j >= n) {
                break;
            }
            k = j + 1 as ::core::ffi::c_int;
            if k < n
                && rtreeSearchPointCompare(
                    __p_ref.aPoint.offset(k as isize) as *mut RtreeSearchPoint,
                    __p_ref.aPoint.offset(j as isize) as *mut RtreeSearchPoint,
                ) < 0 as ::core::ffi::c_int
            {
                if (rtreeSearchPointCompare(
                    __p_ref.aPoint.offset(k as isize) as *mut RtreeSearchPoint,
                    __p_ref.aPoint.offset(i as isize) as *mut RtreeSearchPoint,
                ) >= 0 as ::core::ffi::c_int)
                {
                    break;
                }
                rtreeSearchPointSwap(p, i, k);
                i = k;
            } else {
                if (rtreeSearchPointCompare(
                    __p_ref.aPoint.offset(j as isize) as *mut RtreeSearchPoint,
                    __p_ref.aPoint.offset(i as isize) as *mut RtreeSearchPoint,
                ) >= 0 as ::core::ffi::c_int)
                {
                    break;
                }
                rtreeSearchPointSwap(p, i, j);
                i = j;
            }
        }
    }
}

unsafe extern "C" fn rtreeStepToLeaf(pCur: *mut RtreeCursor) -> ::core::ffi::c_int {
    let mut p: *mut RtreeSearchPoint;
    let __pCur_ref = { &mut *pCur };
    let pRtree: *mut Rtree = __pCur_ref.base.pVtab as *mut Rtree;
    let mut pNode: *mut RtreeNode;
    let mut eWithin: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut nCell: ::core::ffi::c_int;
    let nConstraint: ::core::ffi::c_int = __pCur_ref.nConstraint;
    let mut ii: ::core::ffi::c_int;
    
    let mut x: RtreeSearchPoint = RtreeSearchPoint {
        rScore: 0.,
        id: 0,
        iLevel: 0,
        eWithin: 0,
        iCell: 0,
    };
    let eInt: ::core::ffi::c_int = ((*pRtree).eCoordType as ::core::ffi::c_int == RTREE_COORD_INT32) as ::core::ffi::c_int;
    loop {
        p = rtreeSearchPointFirst(pCur);
        if !(!p.is_null() && (*p).iLevel as ::core::ffi::c_int > 0 as ::core::ffi::c_int) {
            break;
        }
        let mut pCellData: *mut U8_0;
        pNode = rtreeNodeOfFirstSearchPoint(pCur, &raw mut rc);
        if rc != 0 {
            return rc;
        }
        nCell = readInt16((*pNode).zData.offset(2_isize) as *mut U8_0);
        pCellData = (*pNode).zData.offset(
            (4 as ::core::ffi::c_int
                + (*pRtree).nBytesPerCell as ::core::ffi::c_int * (*p).iCell as ::core::ffi::c_int)
                as isize,
        );
        while ((*p).iCell as ::core::ffi::c_int) < nCell {
            let mut rScore: crate::src::headers::sqlite3_h::Sqlite3RtreeDbl =
                -(1 as ::core::ffi::c_int) as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
            eWithin = crate::src::headers::sqlite3_h::FULLY_WITHIN;
            ii = 0 as ::core::ffi::c_int;
            while ii < nConstraint {
                let pConstraint: *mut RtreeConstraint =
                    __pCur_ref.aConstraint.offset(ii as isize);
                if (*pConstraint).op >= RTREE_MATCH {
                    rc = rtreeCallbackConstraint(
                        pConstraint,
                        eInt,
                        pCellData,
                        p,
                        &raw mut rScore,
                        &raw mut eWithin,
                    );
                    if rc != 0 {
                        return rc;
                    }
                } else if (*p).iLevel as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    rtreeLeafConstraint(pConstraint, eInt, pCellData, &raw mut eWithin);
                } else {
                    rtreeNonleafConstraint(pConstraint, eInt, pCellData, &raw mut eWithin);
                }
                if eWithin == crate::src::headers::sqlite3_h::NOT_WITHIN {
                    (*p).iCell = (*p).iCell.wrapping_add(1);
                    pCellData =
                        pCellData.offset((*pRtree).nBytesPerCell as ::core::ffi::c_int as isize);
                    break;
                } else {
                    ii += 1;
                }
            }
            if eWithin == crate::src::headers::sqlite3_h::NOT_WITHIN {
                continue;
            }
            (*p).iCell = (*p).iCell.wrapping_add(1);
            x.iLevel = ((*p).iLevel as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as U8_0;
            if x.iLevel != 0 {
                x.id = readInt64(pCellData) as crate::src::headers::sqlite3_h::Sqlite3Int64;
                ii = 0 as ::core::ffi::c_int;
                while ii < __pCur_ref.nPoint {
                    if (*__pCur_ref.aPoint.offset(ii as isize)).id == x.id {
                        return crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
                    }
                    ii += 1;
                }
                x.iCell = 0 as U8_0;
            } else {
                x.id = (*p).id;
                x.iCell = ((*p).iCell as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as U8_0;
            }
            if (*p).iCell as ::core::ffi::c_int >= nCell {
                rtreeSearchPointPop(pCur);
            }
            if rScore < RTREE_ZERO {
                rScore = RTREE_ZERO as crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
            }
            p = rtreeSearchPointNew(pCur, rScore as RtreeDValue, x.iLevel);
            if p.is_null() {
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
            (*p).eWithin = eWithin as U8_0;
            (*p).id = x.id;
            (*p).iCell = x.iCell;
            break;
        }
        if (*p).iCell as ::core::ffi::c_int >= nCell {
            rtreeSearchPointPop(pCur);
        }
    }
    __pCur_ref.atEOF =
        (p == ::core::ptr::null_mut::<RtreeSearchPoint>()) as ::core::ffi::c_int as U8_0;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn rtreeNext(
    pVtabCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let pCsr: *mut RtreeCursor = pVtabCursor as *mut RtreeCursor;
    
    if (*pCsr).bAuxValid != 0 {
        (*pCsr).bAuxValid = 0 as U8_0;
        crate::src::src::vdbeapi::sqlite3_reset((*pCsr).pReadAux);
    }
    rtreeSearchPointPop(pCsr);
    let rc: ::core::ffi::c_int = rtreeStepToLeaf(pCsr);
    rc
}

unsafe extern "C" fn rtreeRowid(
    pVtabCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    pRowid: *mut crate::src::headers::sqlite3_h::SqliteInt64,
) -> ::core::ffi::c_int {
    let pCsr: *mut RtreeCursor = pVtabCursor as *mut RtreeCursor;
    let p: *mut RtreeSearchPoint = rtreeSearchPointFirst(pCsr);
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let pNode: *mut RtreeNode = rtreeNodeOfFirstSearchPoint(pCsr, &raw mut rc);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !p.is_null() {
        if (*p).iCell as ::core::ffi::c_int
            >= readInt16((*pNode).zData.offset(2_isize) as *mut U8_0)
        {
            rc = crate::src::headers::sqlite3_h::SQLITE_ABORT;
        } else {
            *pRowid = nodeGetRowid(
                (*pCsr).base.pVtab as *mut Rtree,
                pNode,
                (*p).iCell as ::core::ffi::c_int,
            ) as crate::src::headers::sqlite3_h::SqliteInt64;
        }
    }
    rc
}

unsafe extern "C" fn rtreeColumn(
    cur: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    ctx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pRtree: *mut Rtree = (*cur).pVtab as *mut Rtree;
    let pCsr: *mut RtreeCursor = cur as *mut RtreeCursor;
    let p: *mut RtreeSearchPoint = rtreeSearchPointFirst(pCsr);
    let mut c: RtreeCoord = RtreeCoord { f: 0. };
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let pNode: *mut RtreeNode = rtreeNodeOfFirstSearchPoint(pCsr, &raw mut rc);
    if rc != 0 {
        return rc;
    }
    if p.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    if (*p).iCell as ::core::ffi::c_int >= readInt16((*pNode).zData.offset(2_isize) as *mut U8_0)
    {
        return crate::src::headers::sqlite3_h::SQLITE_ABORT;
    }
    if i == 0 as ::core::ffi::c_int {
        crate::src::src::vdbeapi::sqlite3_result_int64(
            ctx,
            nodeGetRowid(pRtree, pNode, (*p).iCell as ::core::ffi::c_int)
                as crate::src::headers::sqlite3_h::Sqlite3Int64,
        );
    } else if i <= (*pRtree).nDim2 as ::core::ffi::c_int {
        nodeGetCoord(
            pRtree,
            pNode,
            (*p).iCell as ::core::ffi::c_int,
            i - 1 as ::core::ffi::c_int,
            &raw mut c,
        );
        if (*pRtree).eCoordType as ::core::ffi::c_int == RTREE_COORD_REAL32 {
            crate::src::src::vdbeapi::sqlite3_result_double(ctx, c.f as ::core::ffi::c_double);
        } else {
            crate::src::src::vdbeapi::sqlite3_result_int(ctx, c.i);
        }
    } else {
        if (*pCsr).bAuxValid == 0 {
            let __pCsr_ref = { &mut *pCsr };
            if __pCsr_ref.pReadAux.is_null() {
                rc = crate::src::src::prepare::sqlite3_prepare_v3(
                    (*pRtree).db,
                    (*pRtree).zReadAuxSql,
                    -(1 as ::core::ffi::c_int),
                    0 as ::core::ffi::c_uint,
                    &raw mut __pCsr_ref.pReadAux,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                if rc != 0 {
                    return rc;
                }
            }
            crate::src::src::vdbeapi::sqlite3_bind_int64(
                __pCsr_ref.pReadAux,
                1 as ::core::ffi::c_int,
                nodeGetRowid(pRtree, pNode, (*p).iCell as ::core::ffi::c_int)
                    as crate::src::headers::sqlite3_h::Sqlite3Int64,
            );
            rc = crate::src::src::vdbeapi::sqlite3_step(__pCsr_ref.pReadAux);
            if rc == crate::src::headers::sqlite3_h::SQLITE_ROW {
                __pCsr_ref.bAuxValid = 1 as U8_0;
            } else {
                crate::src::src::vdbeapi::sqlite3_reset(__pCsr_ref.pReadAux);
                if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
                    rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                }
                return rc;
            }
        }
        crate::src::src::vdbeapi::sqlite3_result_value(
            ctx,
            crate::src::src::vdbeapi::sqlite3_column_value(
                (*pCsr).pReadAux,
                i - (*pRtree).nDim2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
            ),
        );
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn findLeafNode(
    pRtree: *mut Rtree,
    iRowid: I64_0,
    ppLeaf: *mut *mut RtreeNode,
    piNode: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int;
    *ppLeaf = ::core::ptr::null_mut::<RtreeNode>();
    crate::src::src::vdbeapi::sqlite3_bind_int64(
        (*pRtree).pReadRowid,
        1 as ::core::ffi::c_int,
        iRowid as crate::src::headers::sqlite3_h::Sqlite3Int64,
    );
    if crate::src::src::vdbeapi::sqlite3_step((*pRtree).pReadRowid)
        == crate::src::headers::sqlite3_h::SQLITE_ROW
    {
        let iNode: I64_0 = crate::src::src::vdbeapi::sqlite3_column_int64(
            (*pRtree).pReadRowid,
            0 as ::core::ffi::c_int,
        ) as I64_0;
        if !piNode.is_null() {
            *piNode = iNode as crate::src::headers::sqlite3_h::Sqlite3Int64;
        }
        rc = nodeAcquire(pRtree, iNode, ::core::ptr::null_mut::<RtreeNode>(), ppLeaf);
        crate::src::src::vdbeapi::sqlite3_reset((*pRtree).pReadRowid);
    } else {
        rc = crate::src::src::vdbeapi::sqlite3_reset((*pRtree).pReadRowid);
    }
    rc
}

unsafe extern "C" fn deserializeGeometry(
    pValue: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pCons: *mut RtreeConstraint,
) -> ::core::ffi::c_int {
    
    
    
    let pSrc: *mut RtreeMatchArg = crate::src::src::vdbeapi::sqlite3_value_pointer(
        pValue,
        b"RtreeMatchArg\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut RtreeMatchArg;
    if pSrc.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    let pInfo: *mut crate::src::headers::sqlite3_h::sqlite3_rtree_query_info = crate::src::src::malloc::sqlite3_malloc64(
        (::core::mem::size_of::<crate::src::headers::sqlite3_h::sqlite3_rtree_query_info>()
            as usize)
            .wrapping_add((*pSrc).iSize as usize)
            as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut crate::src::headers::sqlite3_h::sqlite3_rtree_query_info;
    if pInfo.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        pInfo as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<crate::src::headers::sqlite3_h::sqlite3_rtree_query_info>()
            as crate::__stddef_size_t_h::SizeT,
    );
    let pBlob: *mut RtreeMatchArg = pInfo.offset(1_isize)
        as *mut crate::src::headers::sqlite3_h::sqlite3_rtree_query_info
        as *mut RtreeMatchArg;
    ::core::ptr::copy_nonoverlapping(pSrc as *const u8, pBlob as *mut u8, (*pSrc).iSize as usize);
    (*pInfo).pContext = (*pBlob).cb.pContext;
    (*pInfo).nParam = (*pBlob).nParam;
    (*pInfo).aParam = &raw mut (*pBlob).aParam as *mut RtreeDValue
        as *mut crate::src::headers::sqlite3_h::Sqlite3RtreeDbl;
    (*pInfo).apSqlParam = (*pBlob).apSqlParam;
    if (*pBlob).cb.xGeom.is_some() {
        (*pCons).u.xGeom = (*pBlob).cb.xGeom;
    } else {
        (*pCons).op = RTREE_QUERY;
        (*pCons).u.xQueryFunc = (*pBlob).cb.xQueryFunc;
    }
    (*pCons).pInfo = pInfo;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn rtreeFilter(
    pVtabCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    idxNum: ::core::ffi::c_int,
    idxStr: *const ::core::ffi::c_char,
    argc: ::core::ffi::c_int,
    argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let pRtree: *mut Rtree = (*pVtabCursor).pVtab as *mut Rtree;
    let pCsr: *mut RtreeCursor = pVtabCursor as *mut RtreeCursor;
    let mut pRoot: *mut RtreeNode = ::core::ptr::null_mut::<RtreeNode>();
    let mut ii: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int;
    let mut iCell: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rtreeReference(pRtree);
    resetCursor(pCsr);
    (*pCsr).iStrategy = idxNum;
    if idxNum == 1 as ::core::ffi::c_int {
        let mut pLeaf: *mut RtreeNode = ::core::ptr::null_mut::<RtreeNode>();
        let p: *mut RtreeSearchPoint;
        let iRowid: I64_0 =
            crate::src::src::vdbeapi::sqlite3_value_int64(*argv.offset(0_isize)) as I64_0;
        let mut iNode: I64_0 = 0 as I64_0;
        let eType: ::core::ffi::c_int =
            crate::src::src::vdbe::sqlite3_value_numeric_type(*argv.offset(0_isize));
        if eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER
            || eType == crate::src::headers::sqlite3_h::SQLITE_FLOAT
                && 0 as ::core::ffi::c_int
                    == sqlite3IntFloatCompare(
                        iRowid,
                        crate::src::src::vdbeapi::sqlite3_value_double(*argv.offset(0_isize)),
                    )
        {
            rc = findLeafNode(pRtree, iRowid, &raw mut pLeaf, &raw mut iNode);
        } else {
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
            pLeaf = ::core::ptr::null_mut::<RtreeNode>();
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !pLeaf.is_null() {
            p = rtreeSearchPointNew(pCsr, RTREE_ZERO, 0 as U8_0);
            (*pCsr).aNode[0 as ::core::ffi::c_int as usize] = pLeaf;
            (*p).id = iNode as crate::src::headers::sqlite3_h::Sqlite3Int64;
            (*p).eWithin = crate::src::headers::sqlite3_h::PARTLY_WITHIN as U8_0;
            rc = nodeRowidIndex(pRtree, pLeaf, iRowid, &raw mut iCell);
            (*p).iCell = iCell as U8_0;
        } else {
            (*pCsr).atEOF = 1 as U8_0;
        }
    } else {
        rc = nodeAcquire(
            pRtree,
            1 as I64_0,
            ::core::ptr::null_mut::<RtreeNode>(),
            &raw mut pRoot,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && argc > 0 as ::core::ffi::c_int {
            let __pCsr_ref = { &mut *pCsr };
            __pCsr_ref.aConstraint = crate::src::src::malloc::sqlite3_malloc64(
                (::core::mem::size_of::<RtreeConstraint>() as usize).wrapping_mul(argc as usize)
                    as crate::src::headers::sqlite3_h::Sqlite3Uint64,
            ) as *mut RtreeConstraint;
            __pCsr_ref.nConstraint = argc;
            if __pCsr_ref.aConstraint.is_null() {
                rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            } else {
                ::libc::memset(
                    __pCsr_ref.aConstraint as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (::core::mem::size_of::<RtreeConstraint>() as crate::__stddef_size_t_h::SizeT)
                        .wrapping_mul(argc as crate::__stddef_size_t_h::SizeT),
                );
                ::libc::memset(
                    &raw mut __pCsr_ref.anQueue as *mut U32_0 as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (::core::mem::size_of::<U32_0>() as crate::__stddef_size_t_h::SizeT)
                        .wrapping_mul(
                            ((*pRtree).iDepth + 1 as ::core::ffi::c_int)
                                as crate::__stddef_size_t_h::SizeT,
                        ),
                );
                ii = 0 as ::core::ffi::c_int;
                while ii < argc {
                    let p_0: *mut RtreeConstraint =
                        __pCsr_ref.aConstraint.offset(ii as isize) as *mut RtreeConstraint;
                    let eType_0: ::core::ffi::c_int =
                        crate::src::src::vdbe::sqlite3_value_numeric_type(
                            *argv.offset(ii as isize),
                        );
                    let __p_0_ref = { &mut *p_0 };
                    __p_0_ref.op = *idxStr.offset((ii * 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int;
                    __p_0_ref.iCoord = *idxStr
                        .offset((ii * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        - '0' as i32;
                    if __p_0_ref.op >= RTREE_MATCH {
                        rc = deserializeGeometry(*argv.offset(ii as isize), p_0);
                        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                            break;
                        }
                        (*__p_0_ref.pInfo).nCoord = (*pRtree).nDim2 as ::core::ffi::c_int;
                        (*__p_0_ref.pInfo).anQueue =
                            &raw mut __pCsr_ref.anQueue as *mut U32_0 as *mut ::core::ffi::c_uint;
                        (*__p_0_ref.pInfo).mxLevel = (*pRtree).iDepth + 1 as ::core::ffi::c_int;
                    } else if eType_0 == crate::src::headers::sqlite3_h::SQLITE_INTEGER {
                        let iVal: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                            crate::src::src::vdbeapi::sqlite3_value_int64(
                                *argv.offset(ii as isize),
                            );
                        __p_0_ref.u.rValue = iVal as ::core::ffi::c_double as RtreeDValue;
                        if iVal
                            >= (1 as ::core::ffi::c_int
                                as crate::src::headers::sqlite3_h::Sqlite3Int64)
                                << 48 as ::core::ffi::c_int
                            || iVal
                                <= -((1 as ::core::ffi::c_int
                                    as crate::src::headers::sqlite3_h::Sqlite3Int64)
                                    << 48 as ::core::ffi::c_int)
                        {
                            if __p_0_ref.op == RTREE_LT {
                                __p_0_ref.op = RTREE_LE;
                            }
                            if __p_0_ref.op == RTREE_GT {
                                __p_0_ref.op = RTREE_GE;
                            }
                        }
                    } else if eType_0 == crate::src::headers::sqlite3_h::SQLITE_FLOAT {
                        __p_0_ref.u.rValue = crate::src::src::vdbeapi::sqlite3_value_double(
                            *argv.offset(ii as isize),
                        ) as RtreeDValue;
                    } else {
                        __p_0_ref.u.rValue = RTREE_ZERO as RtreeDValue;
                        if eType_0 == crate::src::headers::sqlite3_h::SQLITE_NULL {
                            __p_0_ref.op = RTREE_FALSE;
                        } else if __p_0_ref.op == RTREE_LT || __p_0_ref.op == RTREE_LE {
                            __p_0_ref.op = RTREE_TRUE;
                        } else {
                            __p_0_ref.op = RTREE_FALSE;
                        }
                    }
                    ii += 1;
                }
            }
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            
            let pNew: *mut RtreeSearchPoint = rtreeSearchPointNew(
                pCsr,
                RTREE_ZERO,
                ((*pRtree).iDepth + 1 as ::core::ffi::c_int) as U8_0,
            );
            if pNew.is_null() {
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
            (*pNew).id = 1 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            (*pNew).iCell = 0 as U8_0;
            (*pNew).eWithin = crate::src::headers::sqlite3_h::PARTLY_WITHIN as U8_0;
            (*pCsr).aNode[0 as ::core::ffi::c_int as usize] = pRoot;
            pRoot = ::core::ptr::null_mut::<RtreeNode>();
            rc = rtreeStepToLeaf(pCsr);
        }
    }
    nodeRelease(pRtree, pRoot);
    rtreeRelease(pRtree);
    rc
}

unsafe extern "C" fn rtreeBestIndex(
    tab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    pIdxInfo: *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
) -> ::core::ffi::c_int {
    let pRtree = &*(tab as *mut Rtree);
    let rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut ii: ::core::ffi::c_int;
    let mut bMatch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    let mut iIdx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut zIdxStr: [::core::ffi::c_char; 41] = { ::core::mem::zeroed() };
    ii = 0 as ::core::ffi::c_int;
    let __pIdxInfo_ref = { &mut *pIdxInfo };
    while ii < __pIdxInfo_ref.nConstraint {
        if (*__pIdxInfo_ref.aConstraint.offset(ii as isize)).op as ::core::ffi::c_int
            == crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_MATCH_1
        {
            bMatch = 1 as ::core::ffi::c_int;
        }
        ii += 1;
    }
    ii = 0 as ::core::ffi::c_int;
    while ii < __pIdxInfo_ref.nConstraint
        && iIdx
            < (::core::mem::size_of::<[::core::ffi::c_char; 41]>() as usize)
                .wrapping_sub(1_usize) as ::core::ffi::c_int
    {
        let p: *mut crate::src::headers::sqlite3_h::sqlite3_index_constraint =
            __pIdxInfo_ref.aConstraint.offset(ii as isize)
                as *mut crate::src::headers::sqlite3_h::sqlite3_index_constraint;
        let __p_ref = { &*p };
        if bMatch == 0 as ::core::ffi::c_int
            && __p_ref.usable as ::core::ffi::c_int != 0
            && __p_ref.iColumn <= 0 as ::core::ffi::c_int
            && __p_ref.op as ::core::ffi::c_int
                == crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_EQ_1
        {
            let mut jj: ::core::ffi::c_int;
            jj = 0 as ::core::ffi::c_int;
            while jj < ii {
                (*__pIdxInfo_ref.aConstraintUsage.offset(jj as isize)).argvIndex =
                    0 as ::core::ffi::c_int;
                (*__pIdxInfo_ref.aConstraintUsage.offset(jj as isize)).omit =
                    0 as ::core::ffi::c_uchar;
                jj += 1;
            }
            __pIdxInfo_ref.idxNum = 1 as ::core::ffi::c_int;
            (*__pIdxInfo_ref.aConstraintUsage.offset(ii as isize)).argvIndex =
                1 as ::core::ffi::c_int;
            (*__pIdxInfo_ref.aConstraintUsage.offset(jj as isize)).omit = 1 as ::core::ffi::c_uchar;
            __pIdxInfo_ref.estimatedCost = 30.0f64;
            __pIdxInfo_ref.estimatedRows = 1 as crate::src::headers::sqlite3_h::Sqlite3Int64;
            __pIdxInfo_ref.idxFlags = crate::src::headers::sqlite3_h::SQLITE_INDEX_SCAN_UNIQUE;
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        if __p_ref.usable as ::core::ffi::c_int != 0
            && (__p_ref.iColumn > 0 as ::core::ffi::c_int
                && __p_ref.iColumn <= pRtree.nDim2 as ::core::ffi::c_int
                || __p_ref.op as ::core::ffi::c_int
                    == crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_MATCH_1)
        {
            let op: U8_0;
            let mut doOmit: U8_0 = 1 as U8_0;
            match __p_ref.op as ::core::ffi::c_int {
                crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_EQ_1 => {
                    op = RTREE_EQ as U8_0;
                    doOmit = 0 as U8_0;
                }
                crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_GT => {
                    op = RTREE_GT as U8_0;
                    doOmit = 0 as U8_0;
                }
                crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_LE => {
                    op = RTREE_LE as U8_0;
                }
                crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_LT => {
                    op = RTREE_LT as U8_0;
                    doOmit = 0 as U8_0;
                }
                crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_GE => {
                    op = RTREE_GE as U8_0;
                }
                crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_MATCH_1 => {
                    op = RTREE_MATCH as U8_0;
                }
                _ => {
                    op = 0 as U8_0;
                }
            }
            if op != 0 {
                let fresh3 = iIdx;
                iIdx += 1;
                zIdxStr[fresh3 as usize] = op as ::core::ffi::c_char;
                let fresh4 = iIdx;
                iIdx += 1;
                zIdxStr[fresh4 as usize] =
                    (__p_ref.iColumn - 1 as ::core::ffi::c_int + '0' as i32) as ::core::ffi::c_char;
                (*__pIdxInfo_ref.aConstraintUsage.offset(ii as isize)).argvIndex =
                    iIdx / 2 as ::core::ffi::c_int;
                (*__pIdxInfo_ref.aConstraintUsage.offset(ii as isize)).omit =
                    doOmit as ::core::ffi::c_uchar;
            }
        }
        ii += 1;
    }
    __pIdxInfo_ref.idxNum = 2 as ::core::ffi::c_int;
    __pIdxInfo_ref.needToFreeIdxStr = 1 as ::core::ffi::c_int;
    if iIdx > 0 as ::core::ffi::c_int {
        __pIdxInfo_ref.idxStr =
            crate::src::src::malloc::sqlite3_malloc(iIdx + 1 as ::core::ffi::c_int)
                as *mut ::core::ffi::c_char;
        if __pIdxInfo_ref.idxStr.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        ::core::ptr::copy_nonoverlapping(
            &raw mut zIdxStr as *mut ::core::ffi::c_char as *const u8,
            __pIdxInfo_ref.idxStr as *mut u8,
            (iIdx + 1 as ::core::ffi::c_int) as usize,
        );
    }
    let nRow: I64_0 = pRtree.nRowEst >> iIdx / 2 as ::core::ffi::c_int;
    __pIdxInfo_ref.estimatedCost = 6.0f64 * nRow as ::core::ffi::c_double;
    __pIdxInfo_ref.estimatedRows = nRow as crate::src::headers::sqlite3_h::Sqlite3Int64;
    rc
}

unsafe extern "C" fn cellArea(pRtree: *mut Rtree, p: *mut RtreeCell) -> RtreeDValue {
    let mut area: RtreeDValue = 1 as ::core::ffi::c_int as RtreeDValue;
    if (*pRtree).eCoordType as ::core::ffi::c_int == RTREE_COORD_REAL32 {
        let mut current_block_5: u64;
        match (*pRtree).nDim as ::core::ffi::c_int {
            5 => {
                area = ((*p).aCoord[9 as ::core::ffi::c_int as usize].f
                    - (*p).aCoord[8 as ::core::ffi::c_int as usize].f)
                    as RtreeDValue;
                current_block_5 = 6269710938687375737;
            }
            4 => {
                current_block_5 = 6269710938687375737;
            }
            3 => {
                current_block_5 = 13605341943804237952;
            }
            2 => {
                current_block_5 = 3848145320424515447;
            }
            _ => {
                current_block_5 = 16329828092235511802;
            }
        }
        match current_block_5 {
            6269710938687375737 => {
                area *= ((*p).aCoord[7 as ::core::ffi::c_int as usize].f
                    - (*p).aCoord[6 as ::core::ffi::c_int as usize].f)
                    as RtreeDValue;
                current_block_5 = 13605341943804237952;
            }
            _ => {}
        }
        match current_block_5 {
            13605341943804237952 => {
                area *= ((*p).aCoord[5 as ::core::ffi::c_int as usize].f
                    - (*p).aCoord[4 as ::core::ffi::c_int as usize].f)
                    as RtreeDValue;
                current_block_5 = 3848145320424515447;
            }
            _ => {}
        }
        match current_block_5 {
            3848145320424515447 => {
                area *= ((*p).aCoord[3 as ::core::ffi::c_int as usize].f
                    - (*p).aCoord[2 as ::core::ffi::c_int as usize].f)
                    as RtreeDValue;
            }
            _ => {}
        }
        area *= ((*p).aCoord[1 as ::core::ffi::c_int as usize].f
            - (*p).aCoord[0 as ::core::ffi::c_int as usize].f) as RtreeDValue;
    } else {
        let mut current_block_12: u64;
        match (*pRtree).nDim as ::core::ffi::c_int {
            5 => {
                area = ((*p).aCoord[9 as ::core::ffi::c_int as usize].i as I64_0
                    - (*p).aCoord[8 as ::core::ffi::c_int as usize].i as I64_0)
                    as RtreeDValue;
                current_block_12 = 4804969265056275772;
            }
            4 => {
                current_block_12 = 4804969265056275772;
            }
            3 => {
                current_block_12 = 10117027413191050543;
            }
            2 => {
                current_block_12 = 12112996543100875946;
            }
            _ => {
                current_block_12 = 13221271166207665853;
            }
        }
        match current_block_12 {
            4804969265056275772 => {
                area *= ((*p).aCoord[7 as ::core::ffi::c_int as usize].i as I64_0
                    - (*p).aCoord[6 as ::core::ffi::c_int as usize].i as I64_0)
                    as RtreeDValue;
                current_block_12 = 10117027413191050543;
            }
            _ => {}
        }
        match current_block_12 {
            10117027413191050543 => {
                area *= ((*p).aCoord[5 as ::core::ffi::c_int as usize].i as I64_0
                    - (*p).aCoord[4 as ::core::ffi::c_int as usize].i as I64_0)
                    as RtreeDValue;
                current_block_12 = 12112996543100875946;
            }
            _ => {}
        }
        match current_block_12 {
            12112996543100875946 => {
                area *= ((*p).aCoord[3 as ::core::ffi::c_int as usize].i as I64_0
                    - (*p).aCoord[2 as ::core::ffi::c_int as usize].i as I64_0)
                    as RtreeDValue;
            }
            _ => {}
        }
        area *= ((*p).aCoord[1 as ::core::ffi::c_int as usize].i as I64_0
            - (*p).aCoord[0 as ::core::ffi::c_int as usize].i as I64_0)
            as RtreeDValue;
    }
    area
}

unsafe extern "C" fn cellMargin(pRtree: *mut Rtree, p: *mut RtreeCell) -> RtreeDValue {
    let mut margin: RtreeDValue = 0 as ::core::ffi::c_int as RtreeDValue;
    let mut ii: ::core::ffi::c_int =
        (*pRtree).nDim2 as ::core::ffi::c_int - 2 as ::core::ffi::c_int;
    loop {
        margin += (if (*pRtree).eCoordType as ::core::ffi::c_int == RTREE_COORD_REAL32 {
            (*p).aCoord[(ii + 1 as ::core::ffi::c_int) as usize].f as ::core::ffi::c_double
        } else {
            (*p).aCoord[(ii + 1 as ::core::ffi::c_int) as usize].i as ::core::ffi::c_double
        }) - (if (*pRtree).eCoordType as ::core::ffi::c_int == RTREE_COORD_REAL32 {
            (*p).aCoord[ii as usize].f as ::core::ffi::c_double
        } else {
            (*p).aCoord[ii as usize].i as ::core::ffi::c_double
        });
        ii -= 2 as ::core::ffi::c_int;
        if (ii < 0 as ::core::ffi::c_int) {
            break;
        }
    }
    margin
}

unsafe extern "C" fn cellUnion(
    pRtree: *mut Rtree,
    p1: *mut RtreeCell,
    p2: *mut RtreeCell,
) {
    let mut ii: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*pRtree).eCoordType as ::core::ffi::c_int == RTREE_COORD_REAL32 {
        loop {
            let __p1_ref = { &mut *p1 };
            __p1_ref.aCoord[ii as usize].f =
                if __p1_ref.aCoord[ii as usize].f > (*p2).aCoord[ii as usize].f {
                    (*p2).aCoord[ii as usize].f
                } else {
                    __p1_ref.aCoord[ii as usize].f
                };
            __p1_ref.aCoord[(ii + 1 as ::core::ffi::c_int) as usize].f =
                if __p1_ref.aCoord[(ii + 1 as ::core::ffi::c_int) as usize].f
                    < (*p2).aCoord[(ii + 1 as ::core::ffi::c_int) as usize].f
                {
                    (*p2).aCoord[(ii + 1 as ::core::ffi::c_int) as usize].f
                } else {
                    __p1_ref.aCoord[(ii + 1 as ::core::ffi::c_int) as usize].f
                };
            ii += 2 as ::core::ffi::c_int;
            if (ii >= (*pRtree).nDim2 as ::core::ffi::c_int) {
                break;
            }
        }
    } else {
        loop {
            let __p1_ref = { &mut *p1 };
            __p1_ref.aCoord[ii as usize].i =
                if __p1_ref.aCoord[ii as usize].i > (*p2).aCoord[ii as usize].i {
                    (*p2).aCoord[ii as usize].i
                } else {
                    __p1_ref.aCoord[ii as usize].i
                };
            __p1_ref.aCoord[(ii + 1 as ::core::ffi::c_int) as usize].i =
                if __p1_ref.aCoord[(ii + 1 as ::core::ffi::c_int) as usize].i
                    < (*p2).aCoord[(ii + 1 as ::core::ffi::c_int) as usize].i
                {
                    (*p2).aCoord[(ii + 1 as ::core::ffi::c_int) as usize].i
                } else {
                    __p1_ref.aCoord[(ii + 1 as ::core::ffi::c_int) as usize].i
                };
            ii += 2 as ::core::ffi::c_int;
            if (ii >= (*pRtree).nDim2 as ::core::ffi::c_int) {
                break;
            }
        }
    };
}

unsafe extern "C" fn cellContains(
    pRtree: *mut Rtree,
    p1: *mut RtreeCell,
    p2: *mut RtreeCell,
) -> ::core::ffi::c_int {
    let mut ii: ::core::ffi::c_int;
    if (*pRtree).eCoordType as ::core::ffi::c_int == RTREE_COORD_INT32 {
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pRtree).nDim2 as ::core::ffi::c_int {
            let a1: *mut RtreeCoord =
                (&raw mut (*p1).aCoord as *mut RtreeCoord).offset(ii as isize) as *mut RtreeCoord;
            let a2: *mut RtreeCoord =
                (&raw mut (*p2).aCoord as *mut RtreeCoord).offset(ii as isize) as *mut RtreeCoord;
            if (*a2.offset(0_isize)).i < (*a1.offset(0_isize)).i
                || (*a2.offset(1_isize)).i > (*a1.offset(1_isize)).i
            {
                return 0 as ::core::ffi::c_int;
            }
            ii += 2 as ::core::ffi::c_int;
        }
    } else {
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pRtree).nDim2 as ::core::ffi::c_int {
            let a1_0: *mut RtreeCoord =
                (&raw mut (*p1).aCoord as *mut RtreeCoord).offset(ii as isize) as *mut RtreeCoord;
            let a2_0: *mut RtreeCoord =
                (&raw mut (*p2).aCoord as *mut RtreeCoord).offset(ii as isize) as *mut RtreeCoord;
            if (*a2_0.offset(0_isize)).f < (*a1_0.offset(0_isize)).f
                || (*a2_0.offset(1_isize)).f > (*a1_0.offset(1_isize)).f
            {
                return 0 as ::core::ffi::c_int;
            }
            ii += 2 as ::core::ffi::c_int;
        }
    }
    1 as ::core::ffi::c_int
}

unsafe extern "C" fn cellOverlap(
    pRtree: *mut Rtree,
    p: *mut RtreeCell,
    aCell: *mut RtreeCell,
    nCell: ::core::ffi::c_int,
) -> RtreeDValue {
    let mut ii: ::core::ffi::c_int;
    let mut overlap: RtreeDValue = RTREE_ZERO;
    ii = 0 as ::core::ffi::c_int;
    while ii < nCell {
        let mut jj: ::core::ffi::c_int;
        let mut o: RtreeDValue = 1 as ::core::ffi::c_int as RtreeDValue;
        jj = 0 as ::core::ffi::c_int;
        while jj < (*pRtree).nDim2 as ::core::ffi::c_int {
            
            
            let __pRtree_ref = { &*pRtree };
            let x1: RtreeDValue = (if (if __pRtree_ref.eCoordType as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*p).aCoord[jj as usize].f as ::core::ffi::c_double
            } else {
                (*p).aCoord[jj as usize].i as ::core::ffi::c_double
            }) < (if __pRtree_ref.eCoordType as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*aCell.offset(ii as isize)).aCoord[jj as usize].f as ::core::ffi::c_double
            } else {
                (*aCell.offset(ii as isize)).aCoord[jj as usize].i as ::core::ffi::c_double
            }) {
                if __pRtree_ref.eCoordType as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*aCell.offset(ii as isize)).aCoord[jj as usize].f as ::core::ffi::c_double
                } else {
                    (*aCell.offset(ii as isize)).aCoord[jj as usize].i as ::core::ffi::c_double
                }
            } else if __pRtree_ref.eCoordType as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*p).aCoord[jj as usize].f as ::core::ffi::c_double
            } else {
                (*p).aCoord[jj as usize].i as ::core::ffi::c_double
            }) as RtreeDValue;
            let x2: RtreeDValue = (if (if __pRtree_ref.eCoordType as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*p).aCoord[(jj + 1 as ::core::ffi::c_int) as usize].f as ::core::ffi::c_double
            } else {
                (*p).aCoord[(jj + 1 as ::core::ffi::c_int) as usize].i as ::core::ffi::c_double
            }) > (if __pRtree_ref.eCoordType as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*aCell.offset(ii as isize)).aCoord[(jj + 1 as ::core::ffi::c_int) as usize].f
                    as ::core::ffi::c_double
            } else {
                (*aCell.offset(ii as isize)).aCoord[(jj + 1 as ::core::ffi::c_int) as usize].i
                    as ::core::ffi::c_double
            }) {
                if __pRtree_ref.eCoordType as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*aCell.offset(ii as isize)).aCoord[(jj + 1 as ::core::ffi::c_int) as usize].f
                        as ::core::ffi::c_double
                } else {
                    (*aCell.offset(ii as isize)).aCoord[(jj + 1 as ::core::ffi::c_int) as usize].i
                        as ::core::ffi::c_double
                }
            } else if __pRtree_ref.eCoordType as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*p).aCoord[(jj + 1 as ::core::ffi::c_int) as usize].f as ::core::ffi::c_double
            } else {
                (*p).aCoord[(jj + 1 as ::core::ffi::c_int) as usize].i as ::core::ffi::c_double
            }) as RtreeDValue;
            if x2 < x1 {
                o = 0 as ::core::ffi::c_int as RtreeDValue;
                break;
            } else {
                o *= x2 - x1;
                jj += 2 as ::core::ffi::c_int;
            }
        }
        overlap += o;
        ii += 1;
    }
    overlap
}

unsafe extern "C" fn ChooseLeaf(
    pRtree: *mut Rtree,
    pCell: *mut RtreeCell,
    iHeight: ::core::ffi::c_int,
    ppLeaf: *mut *mut RtreeNode,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut ii: ::core::ffi::c_int;
    let mut pNode: *mut RtreeNode = ::core::ptr::null_mut::<RtreeNode>();
    rc = nodeAcquire(
        pRtree,
        1 as I64_0,
        ::core::ptr::null_mut::<RtreeNode>(),
        &raw mut pNode,
    );
    ii = 0 as ::core::ffi::c_int;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && ii < (*pRtree).iDepth - iHeight {
        let mut iCell: ::core::ffi::c_int;
        let mut iBest: crate::src::headers::sqlite3_h::Sqlite3Int64 =
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        let mut bFound: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut fMinGrowth: RtreeDValue = RTREE_ZERO;
        let mut fMinArea: RtreeDValue = RTREE_ZERO;
        let nCell: ::core::ffi::c_int =
            readInt16((*pNode).zData.offset(2_isize) as *mut U8_0);
        let mut pChild: *mut RtreeNode = ::core::ptr::null_mut::<RtreeNode>();
        iCell = 0 as ::core::ffi::c_int;
        while iCell < nCell {
            let mut cell: RtreeCell = RtreeCell {
                iRowid: 0,
                aCoord: [RtreeCoord { f: 0. }; 10],
            };
            nodeGetCell(pRtree, pNode, iCell, &raw mut cell);
            if cellContains(pRtree, &raw mut cell, pCell) != 0 {
                let area: RtreeDValue = cellArea(pRtree, &raw mut cell);
                if bFound == 0 as ::core::ffi::c_int || area < fMinArea {
                    iBest = cell.iRowid as crate::src::headers::sqlite3_h::Sqlite3Int64;
                    fMinArea = area;
                    bFound = 1 as ::core::ffi::c_int;
                }
            }
            iCell += 1;
        }
        if bFound == 0 {
            iCell = 0 as ::core::ffi::c_int;
            while iCell < nCell {
                let mut cell_0: RtreeCell = RtreeCell {
                    iRowid: 0,
                    aCoord: [RtreeCoord { f: 0. }; 10],
                };
                
                
                nodeGetCell(pRtree, pNode, iCell, &raw mut cell_0);
                let area_0: RtreeDValue = cellArea(pRtree, &raw mut cell_0);
                cellUnion(pRtree, &raw mut cell_0, pCell);
                let growth: RtreeDValue = cellArea(pRtree, &raw mut cell_0) - area_0;
                if iCell == 0 as ::core::ffi::c_int
                    || growth < fMinGrowth
                    || growth == fMinGrowth && area_0 < fMinArea
                {
                    fMinGrowth = growth;
                    fMinArea = area_0;
                    iBest = cell_0.iRowid as crate::src::headers::sqlite3_h::Sqlite3Int64;
                }
                iCell += 1;
            }
        }
        rc = nodeAcquire(pRtree, iBest as I64_0, pNode, &raw mut pChild);
        nodeRelease(pRtree, pNode);
        pNode = pChild;
        ii += 1;
    }
    *ppLeaf = pNode;
    rc
}

unsafe extern "C" fn AdjustTree(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
    pCell: *mut RtreeCell,
) -> ::core::ffi::c_int {
    let mut p: *mut RtreeNode = pNode;
    let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int;
    while !(*p).pParent.is_null() {
        let pParent: *mut RtreeNode = (*p).pParent;
        let mut cell: RtreeCell = RtreeCell {
            iRowid: 0,
            aCoord: [RtreeCoord { f: 0. }; 10],
        };
        let mut iCell: ::core::ffi::c_int = 0;
        cnt += 1;
        if cnt > 100 as ::core::ffi::c_int {
            return crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
        }
        rc = nodeParentIndex(pRtree, p, &raw mut iCell);
        if rc != 0 as ::core::ffi::c_int {
            return crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
        }
        nodeGetCell(pRtree, pParent, iCell, &raw mut cell);
        if cellContains(pRtree, &raw mut cell, pCell) == 0 {
            cellUnion(pRtree, &raw mut cell, pCell);
            nodeOverwriteCell(pRtree, pParent, &raw mut cell, iCell);
        }
        p = pParent;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn rowidWrite(
    pRtree: *mut Rtree,
    iRowid: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iNode: crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    let __pRtree_ref = { &*pRtree };
    crate::src::src::vdbeapi::sqlite3_bind_int64(
        __pRtree_ref.pWriteRowid,
        1 as ::core::ffi::c_int,
        iRowid,
    );
    crate::src::src::vdbeapi::sqlite3_bind_int64(
        __pRtree_ref.pWriteRowid,
        2 as ::core::ffi::c_int,
        iNode,
    );
    crate::src::src::vdbeapi::sqlite3_step(__pRtree_ref.pWriteRowid);
    crate::src::src::vdbeapi::sqlite3_reset(__pRtree_ref.pWriteRowid)
}

unsafe extern "C" fn parentWrite(
    pRtree: *mut Rtree,
    iNode: crate::src::headers::sqlite3_h::Sqlite3Int64,
    iPar: crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    let __pRtree_ref = { &*pRtree };
    crate::src::src::vdbeapi::sqlite3_bind_int64(
        __pRtree_ref.pWriteParent,
        1 as ::core::ffi::c_int,
        iNode,
    );
    crate::src::src::vdbeapi::sqlite3_bind_int64(
        __pRtree_ref.pWriteParent,
        2 as ::core::ffi::c_int,
        iPar,
    );
    crate::src::src::vdbeapi::sqlite3_step(__pRtree_ref.pWriteParent);
    crate::src::src::vdbeapi::sqlite3_reset(__pRtree_ref.pWriteParent)
}

unsafe extern "C" fn SortByDimension(
    pRtree: *mut Rtree,
    aIdx: *mut ::core::ffi::c_int,
    nIdx: ::core::ffi::c_int,
    iDim: ::core::ffi::c_int,
    aCell: *mut RtreeCell,
    aSpare: *mut ::core::ffi::c_int,
) {
    if nIdx > 1 as ::core::ffi::c_int {
        let mut iLeft: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iRight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let nLeft: ::core::ffi::c_int = nIdx / 2 as ::core::ffi::c_int;
        let nRight: ::core::ffi::c_int = nIdx - nLeft;
        let mut aLeft: *mut ::core::ffi::c_int = aIdx;
        let aRight: *mut ::core::ffi::c_int =
            aIdx.offset(nLeft as isize) as *mut ::core::ffi::c_int;
        SortByDimension(pRtree, aLeft, nLeft, iDim, aCell, aSpare);
        SortByDimension(pRtree, aRight, nRight, iDim, aCell, aSpare);
        ::core::ptr::copy_nonoverlapping(
            aLeft as *const u8,
            aSpare as *mut u8,
            ((::core::mem::size_of::<::core::ffi::c_int>() as crate::__stddef_size_t_h::SizeT)
                .wrapping_mul(nLeft as crate::__stddef_size_t_h::SizeT)) as usize,
        );
        aLeft = aSpare;
        while iLeft < nLeft || iRight < nRight {
            let __pRtree_ref = { &*pRtree };
            let xleft1: RtreeDValue =
                if __pRtree_ref.eCoordType as ::core::ffi::c_int == RTREE_COORD_REAL32 {
                    (*aCell.offset(*aLeft.offset(iLeft as isize) as isize)).aCoord
                        [(iDim * 2 as ::core::ffi::c_int) as usize]
                        .f as RtreeDValue
                } else {
                    (*aCell.offset(*aLeft.offset(iLeft as isize) as isize)).aCoord
                        [(iDim * 2 as ::core::ffi::c_int) as usize]
                        .i as RtreeDValue
                };
            let xleft2: RtreeDValue =
                if __pRtree_ref.eCoordType as ::core::ffi::c_int == RTREE_COORD_REAL32 {
                    (*aCell.offset(*aLeft.offset(iLeft as isize) as isize)).aCoord
                        [(iDim * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        .f as RtreeDValue
                } else {
                    (*aCell.offset(*aLeft.offset(iLeft as isize) as isize)).aCoord
                        [(iDim * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        .i as RtreeDValue
                };
            let xright1: RtreeDValue =
                if __pRtree_ref.eCoordType as ::core::ffi::c_int == RTREE_COORD_REAL32 {
                    (*aCell.offset(*aRight.offset(iRight as isize) as isize)).aCoord
                        [(iDim * 2 as ::core::ffi::c_int) as usize]
                        .f as RtreeDValue
                } else {
                    (*aCell.offset(*aRight.offset(iRight as isize) as isize)).aCoord
                        [(iDim * 2 as ::core::ffi::c_int) as usize]
                        .i as RtreeDValue
                };
            let xright2: RtreeDValue =
                if __pRtree_ref.eCoordType as ::core::ffi::c_int == RTREE_COORD_REAL32 {
                    (*aCell.offset(*aRight.offset(iRight as isize) as isize)).aCoord
                        [(iDim * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        .f as RtreeDValue
                } else {
                    (*aCell.offset(*aRight.offset(iRight as isize) as isize)).aCoord
                        [(iDim * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                        .i as RtreeDValue
                };
            if iLeft != nLeft
                && (iRight == nRight || xleft1 < xright1 || xleft1 == xright1 && xleft2 < xright2)
            {
                *aIdx.offset((iLeft + iRight) as isize) = *aLeft.offset(iLeft as isize);
                iLeft += 1;
            } else {
                *aIdx.offset((iLeft + iRight) as isize) = *aRight.offset(iRight as isize);
                iRight += 1;
            }
        }
    }
}

unsafe extern "C" fn splitNodeStartree(
    pRtree: *mut Rtree,
    aCell: *mut RtreeCell,
    nCell: ::core::ffi::c_int,
    pLeft: *mut RtreeNode,
    pRight: *mut RtreeNode,
    pBboxLeft: *mut RtreeCell,
    pBboxRight: *mut RtreeCell,
) -> ::core::ffi::c_int {
    
    
    let mut ii: ::core::ffi::c_int;
    let mut iBestDim: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iBestSplit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fBestMargin: RtreeDValue = RTREE_ZERO;
    let __pRtree_ref = { &mut *pRtree };
    let nByte: crate::src::headers::sqlite3_h::Sqlite3Int64 = ((__pRtree_ref.nDim
        as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int)
        as usize)
        .wrapping_mul(
            (::core::mem::size_of::<*mut ::core::ffi::c_int>() as usize).wrapping_add(
                (nCell as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize),
            ),
        )
        as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let aaSorted: *mut *mut ::core::ffi::c_int = crate::src::src::malloc::sqlite3_malloc64(
        nByte as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut *mut ::core::ffi::c_int;
    if aaSorted.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    let aSpare: *mut ::core::ffi::c_int = (aaSorted.offset(__pRtree_ref.nDim as isize) as *mut *mut ::core::ffi::c_int
        as *mut ::core::ffi::c_int)
        .offset((__pRtree_ref.nDim as ::core::ffi::c_int * nCell) as isize)
        as *mut ::core::ffi::c_int;
    ::libc::memset(
        aaSorted as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        nByte as crate::__stddef_size_t_h::SizeT,
    );
    ii = 0 as ::core::ffi::c_int;
    while ii < __pRtree_ref.nDim as ::core::ffi::c_int {
        let mut jj: ::core::ffi::c_int;
        let fresh1 = &mut *aaSorted.offset(ii as isize);
        *fresh1 = (aaSorted.offset(__pRtree_ref.nDim as isize) as *mut *mut ::core::ffi::c_int
            as *mut ::core::ffi::c_int)
            .offset((ii * nCell) as isize) as *mut ::core::ffi::c_int;
        jj = 0 as ::core::ffi::c_int;
        while jj < nCell {
            *(*aaSorted.offset(ii as isize)).offset(jj as isize) = jj;
            jj += 1;
        }
        SortByDimension(
            pRtree,
            *aaSorted.offset(ii as isize),
            nCell,
            ii,
            aCell,
            aSpare,
        );
        ii += 1;
    }
    ii = 0 as ::core::ffi::c_int;
    while ii < __pRtree_ref.nDim as ::core::ffi::c_int {
        let mut margin: RtreeDValue = RTREE_ZERO;
        let mut fBestOverlap: RtreeDValue = RTREE_ZERO;
        let mut fBestArea: RtreeDValue = RTREE_ZERO;
        let mut iBestLeft: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nLeft: ::core::ffi::c_int;
        nLeft = (__pRtree_ref.iNodeSize - 4 as ::core::ffi::c_int)
            / __pRtree_ref.nBytesPerCell as ::core::ffi::c_int
            / 3 as ::core::ffi::c_int;
        while nLeft
            <= nCell
                - (__pRtree_ref.iNodeSize - 4 as ::core::ffi::c_int)
                    / __pRtree_ref.nBytesPerCell as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
        {
            let mut left: RtreeCell = RtreeCell {
                iRowid: 0,
                aCoord: [RtreeCoord { f: 0. }; 10],
            };
            let mut right: RtreeCell = RtreeCell {
                iRowid: 0,
                aCoord: [RtreeCoord { f: 0. }; 10],
            };
            let mut kk: ::core::ffi::c_int;
            
            
            ::core::ptr::copy_nonoverlapping(
                aCell.offset(*(*aaSorted.offset(ii as isize)).offset(0_isize) as isize)
                    as *mut RtreeCell as *const u8,
                &raw mut left as *mut u8,
                ::core::mem::size_of::<RtreeCell>() as usize,
            );
            ::core::ptr::copy_nonoverlapping(
                aCell.offset(
                    *(*aaSorted.offset(ii as isize))
                        .offset((nCell - 1 as ::core::ffi::c_int) as isize)
                        as isize,
                ) as *mut RtreeCell as *const u8,
                &raw mut right as *mut u8,
                ::core::mem::size_of::<RtreeCell>() as usize,
            );
            kk = 1 as ::core::ffi::c_int;
            while kk < nCell - 1 as ::core::ffi::c_int {
                if kk < nLeft {
                    cellUnion(
                        pRtree,
                        &raw mut left,
                        aCell.offset(*(*aaSorted.offset(ii as isize)).offset(kk as isize) as isize)
                            as *mut RtreeCell,
                    );
                } else {
                    cellUnion(
                        pRtree,
                        &raw mut right,
                        aCell.offset(*(*aaSorted.offset(ii as isize)).offset(kk as isize) as isize)
                            as *mut RtreeCell,
                    );
                }
                kk += 1;
            }
            margin += cellMargin(pRtree, &raw mut left);
            margin += cellMargin(pRtree, &raw mut right);
            let overlap: RtreeDValue = cellOverlap(
                pRtree,
                &raw mut left,
                &raw mut right,
                1 as ::core::ffi::c_int,
            );
            let area: RtreeDValue = cellArea(pRtree, &raw mut left) + cellArea(pRtree, &raw mut right);
            if nLeft
                == (__pRtree_ref.iNodeSize - 4 as ::core::ffi::c_int)
                    / __pRtree_ref.nBytesPerCell as ::core::ffi::c_int
                    / 3 as ::core::ffi::c_int
                || overlap < fBestOverlap
                || overlap == fBestOverlap && area < fBestArea
            {
                iBestLeft = nLeft;
                fBestOverlap = overlap;
                fBestArea = area;
            }
            nLeft += 1;
        }
        if ii == 0 as ::core::ffi::c_int || margin < fBestMargin {
            iBestDim = ii;
            fBestMargin = margin;
            iBestSplit = iBestLeft;
        }
        ii += 1;
    }
    ::core::ptr::copy_nonoverlapping(
        aCell.offset(*(*aaSorted.offset(iBestDim as isize)).offset(0_isize) as isize)
            as *mut RtreeCell as *const u8,
        pBboxLeft as *mut u8,
        ::core::mem::size_of::<RtreeCell>() as usize,
    );
    ::core::ptr::copy_nonoverlapping(
        aCell.offset(*(*aaSorted.offset(iBestDim as isize)).offset(iBestSplit as isize) as isize)
            as *mut RtreeCell as *const u8,
        pBboxRight as *mut u8,
        ::core::mem::size_of::<RtreeCell>() as usize,
    );
    ii = 0 as ::core::ffi::c_int;
    while ii < nCell {
        let pTarget: *mut RtreeNode = if ii < iBestSplit { pLeft } else { pRight };
        let pBbox: *mut RtreeCell = if ii < iBestSplit {
            pBboxLeft
        } else {
            pBboxRight
        };
        let pCell: *mut RtreeCell = aCell
            .offset(*(*aaSorted.offset(iBestDim as isize)).offset(ii as isize) as isize)
            as *mut RtreeCell;
        nodeInsertCell(pRtree, pTarget, pCell);
        cellUnion(pRtree, pBbox, pCell);
        ii += 1;
    }
    crate::src::src::malloc::sqlite3_free(aaSorted as *mut ::core::ffi::c_void);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn updateMapping(
    pRtree: *mut Rtree,
    iRowid: I64_0,
    pNode: *mut RtreeNode,
    iHeight: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    let xSetMapping: Option<
        unsafe extern "C" fn(
            *mut Rtree,
            crate::src::headers::sqlite3_h::Sqlite3Int64,
            crate::src::headers::sqlite3_h::Sqlite3Int64,
        ) -> ::core::ffi::c_int,
    > = (if iHeight == 0 as ::core::ffi::c_int {
        Some(
            rowidWrite
                as unsafe extern "C" fn(
                    *mut Rtree,
                    crate::src::headers::sqlite3_h::Sqlite3Int64,
                    crate::src::headers::sqlite3_h::Sqlite3Int64,
                ) -> ::core::ffi::c_int,
        )
    } else {
        Some(
            parentWrite
                as unsafe extern "C" fn(
                    *mut Rtree,
                    crate::src::headers::sqlite3_h::Sqlite3Int64,
                    crate::src::headers::sqlite3_h::Sqlite3Int64,
                ) -> ::core::ffi::c_int,
        )
    })
        as Option<
            unsafe extern "C" fn(
                *mut Rtree,
                crate::src::headers::sqlite3_h::Sqlite3Int64,
                crate::src::headers::sqlite3_h::Sqlite3Int64,
            ) -> ::core::ffi::c_int,
        >;
    if iHeight > 0 as ::core::ffi::c_int {
        let pChild: *mut RtreeNode = nodeHashLookup(pRtree, iRowid);
        let mut p: *mut RtreeNode;
        p = pNode;
        while !p.is_null() {
            if p == pChild {
                return crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
            }
            p = (*p).pParent;
        }
        if !pChild.is_null() {
            nodeRelease(pRtree, (*pChild).pParent);
            nodeReference(pNode);
            (*pChild).pParent = pNode;
        }
    }
    if pNode.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    xSetMapping.expect("non-null function pointer")(
        pRtree,
        iRowid as crate::src::headers::sqlite3_h::Sqlite3Int64,
        (*pNode).iNode as crate::src::headers::sqlite3_h::Sqlite3Int64,
    )
}

unsafe extern "C" fn SplitNode(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
    pCell: *mut RtreeCell,
    iHeight: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int;
    let mut newCellIsRight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int;
    let mut nCell: ::core::ffi::c_int = readInt16((*pNode).zData.offset(2_isize) as *mut U8_0);
    
    let aiUsed: *mut ::core::ffi::c_int;
    let mut pLeft: *mut RtreeNode = ::core::ptr::null_mut::<RtreeNode>();
    let mut pRight: *mut RtreeNode = ::core::ptr::null_mut::<RtreeNode>();
    let mut leftbbox: RtreeCell = RtreeCell {
        iRowid: 0,
        aCoord: [RtreeCoord { f: 0. }; 10],
    };
    let mut rightbbox: RtreeCell = RtreeCell {
        iRowid: 0,
        aCoord: [RtreeCoord { f: 0. }; 10],
    };
    let aCell: *mut RtreeCell = crate::src::src::malloc::sqlite3_malloc64(
        (::core::mem::size_of::<RtreeCell>() as usize)
            .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as usize)
            .wrapping_mul((nCell + 1 as ::core::ffi::c_int) as usize)
            as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut RtreeCell;
    if aCell.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    } else {
        aiUsed = aCell.offset((nCell + 1 as ::core::ffi::c_int) as isize) as *mut RtreeCell
            as *mut ::core::ffi::c_int;
        ::libc::memset(
            aiUsed as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<::core::ffi::c_int>() as crate::__stddef_size_t_h::SizeT)
                .wrapping_mul(
                    (nCell + 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::SizeT,
                ),
        );
        i = 0 as ::core::ffi::c_int;
        while i < nCell {
            nodeGetCell(pRtree, pNode, i, aCell.offset(i as isize) as *mut RtreeCell);
            i += 1;
        }
        nodeZero(pRtree, pNode);
        ::core::ptr::copy_nonoverlapping(
            pCell as *const u8,
            aCell.offset(nCell as isize) as *mut RtreeCell as *mut u8,
            ::core::mem::size_of::<RtreeCell>() as usize,
        );
        nCell += 1;
        if (*pNode).iNode == 1 as I64_0 {
            pRight = nodeNew(pRtree, pNode);
            pLeft = nodeNew(pRtree, pNode);
            (*pRtree).iDepth += 1;
            (*pNode).isDirty = 1 as ::core::ffi::c_int;
            writeInt16((*pNode).zData, (*pRtree).iDepth);
        } else {
            pLeft = pNode;
            pRight = nodeNew(pRtree, (*pLeft).pParent);
            (*pLeft).nRef += 1;
        }
        if pLeft.is_null() || pRight.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            ::libc::memset(
                (*pLeft).zData as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (*pRtree).iNodeSize as crate::__stddef_size_t_h::SizeT,
            );
            ::libc::memset(
                (*pRight).zData as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (*pRtree).iNodeSize as crate::__stddef_size_t_h::SizeT,
            );
            rc = splitNodeStartree(
                pRtree,
                aCell,
                nCell,
                pLeft,
                pRight,
                &raw mut leftbbox,
                &raw mut rightbbox,
            );
            if (rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
                rc = nodeWrite(pRtree, pRight);
                if !(crate::src::headers::sqlite3_h::SQLITE_OK != rc
                    || 0 as I64_0 == (*pLeft).iNode && {
                        rc = nodeWrite(pRtree, pLeft);
                        crate::src::headers::sqlite3_h::SQLITE_OK != rc
                    })
                {
                    rightbbox.iRowid = (*pRight).iNode;
                    leftbbox.iRowid = (*pLeft).iNode;
                    if (*pNode).iNode == 1 as I64_0 {
                        rc = rtreeInsertCell(
                            pRtree,
                            (*pLeft).pParent,
                            &raw mut leftbbox,
                            iHeight + 1 as ::core::ffi::c_int,
                        );
                        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                            current_block = 18165040450919919112;
                        } else {
                            current_block = 14072441030219150333;
                        }
                    } else {
                        let pParent: *mut RtreeNode = (*pLeft).pParent;
                        let mut iCell: ::core::ffi::c_int = 0;
                        rc = nodeParentIndex(pRtree, pLeft, &raw mut iCell);
                        if rc == 0 as ::core::ffi::c_int {
                            nodeOverwriteCell(pRtree, pParent, &raw mut leftbbox, iCell);
                            rc = AdjustTree(pRtree, pParent, &raw mut leftbbox);
                        }
                        if rc != 0 as ::core::ffi::c_int {
                            current_block = 18165040450919919112;
                        } else {
                            current_block = 14072441030219150333;
                        }
                    }
                    match current_block {
                        18165040450919919112 => {}
                        _ => {
                            rc = rtreeInsertCell(
                                pRtree,
                                (*pRight).pParent,
                                &raw mut rightbbox,
                                iHeight + 1 as ::core::ffi::c_int,
                            );
                            if (rc == 0) {
                                i = 0 as ::core::ffi::c_int;
                                loop {
                                    if (i >= readInt16(
                                        (*pRight).zData.offset(2_isize) as *mut U8_0
                                    )) {
                                        current_block = 313581471991351815;
                                        break;
                                    }
                                    let iRowid: I64_0 = nodeGetRowid(pRtree, pRight, i);
                                    rc = updateMapping(pRtree, iRowid, pRight, iHeight);
                                    if iRowid == (*pCell).iRowid {
                                        newCellIsRight = 1 as ::core::ffi::c_int;
                                    }
                                    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                        current_block = 18165040450919919112;
                                        break;
                                    }
                                    i += 1;
                                }
                                match current_block {
                                    18165040450919919112 => {}
                                    _ => {
                                        if (*pNode).iNode == 1 as I64_0 {
                                            i = 0 as ::core::ffi::c_int;
                                            loop {
                                                if (i >= readInt16(
                                                    (*pLeft).zData.offset(2_isize) as *mut U8_0,
                                                )) {
                                                    current_block = 16415152177862271243;
                                                    break;
                                                }
                                                let iRowid_0: I64_0 =
                                                    nodeGetRowid(pRtree, pLeft, i);
                                                rc =
                                                    updateMapping(pRtree, iRowid_0, pLeft, iHeight);
                                                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                                    current_block = 18165040450919919112;
                                                    break;
                                                }
                                                i += 1;
                                            }
                                        } else {
                                            if newCellIsRight == 0 as ::core::ffi::c_int {
                                                rc = updateMapping(
                                                    pRtree,
                                                    (*pCell).iRowid,
                                                    pLeft,
                                                    iHeight,
                                                );
                                            }
                                            current_block = 16415152177862271243;
                                        }
                                        match current_block {
                                            18165040450919919112 => {}
                                            _ => {
                                                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                                    rc = nodeRelease(pRtree, pRight);
                                                    pRight = ::core::ptr::null_mut::<RtreeNode>();
                                                }
                                                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                                    rc = nodeRelease(pRtree, pLeft);
                                                    pLeft = ::core::ptr::null_mut::<RtreeNode>();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    nodeRelease(pRtree, pRight);
    nodeRelease(pRtree, pLeft);
    crate::src::src::malloc::sqlite3_free(aCell as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn fixLeafParent(
    pRtree: *mut Rtree,
    pLeaf: *mut RtreeNode,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pChild: *mut RtreeNode = pLeaf;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && (*pChild).iNode != 1 as I64_0
        && (*pChild).pParent.is_null()
    {
        let mut rc2: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
        let __pRtree_ref = { &*pRtree };
        crate::src::src::vdbeapi::sqlite3_bind_int64(
            __pRtree_ref.pReadParent,
            1 as ::core::ffi::c_int,
            (*pChild).iNode as crate::src::headers::sqlite3_h::Sqlite3Int64,
        );
        rc = crate::src::src::vdbeapi::sqlite3_step(__pRtree_ref.pReadParent);
        if rc == crate::src::headers::sqlite3_h::SQLITE_ROW {
            let mut pTest: *mut RtreeNode;
            
            let iNode: I64_0 = crate::src::src::vdbeapi::sqlite3_column_int64(
                __pRtree_ref.pReadParent,
                0 as ::core::ffi::c_int,
            ) as I64_0;
            pTest = pLeaf;
            while !pTest.is_null() && (*pTest).iNode != iNode {
                pTest = (*pTest).pParent;
            }
            if pTest.is_null() {
                rc2 = nodeAcquire(
                    pRtree,
                    iNode,
                    ::core::ptr::null_mut::<RtreeNode>(),
                    &raw mut (*pChild).pParent,
                );
            }
        }
        rc = crate::src::src::vdbeapi::sqlite3_reset(__pRtree_ref.pReadParent);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && (*pChild).pParent.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
        }
        pChild = (*pChild).pParent;
    }
    rc
}

unsafe extern "C" fn removeNode(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
    iHeight: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    
    let mut pParent: *mut RtreeNode = ::core::ptr::null_mut::<RtreeNode>();
    let mut iCell: ::core::ffi::c_int = 0;
    rc = nodeParentIndex(pRtree, pNode, &raw mut iCell);
    let __pNode_ref = { &mut *pNode };
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        pParent = __pNode_ref.pParent;
        __pNode_ref.pParent = ::core::ptr::null_mut::<RtreeNode>();
        rc = deleteCell(pRtree, pParent, iCell, iHeight + 1 as ::core::ffi::c_int);
    }
    let rc2: ::core::ffi::c_int = nodeRelease(pRtree, pParent);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = rc2;
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    let __pRtree_ref = { &mut *pRtree };
    crate::src::src::vdbeapi::sqlite3_bind_int64(
        __pRtree_ref.pDeleteNode,
        1 as ::core::ffi::c_int,
        __pNode_ref.iNode as crate::src::headers::sqlite3_h::Sqlite3Int64,
    );
    crate::src::src::vdbeapi::sqlite3_step(__pRtree_ref.pDeleteNode);
    rc = crate::src::src::vdbeapi::sqlite3_reset(__pRtree_ref.pDeleteNode);
    if crate::src::headers::sqlite3_h::SQLITE_OK != rc {
        return rc;
    }
    crate::src::src::vdbeapi::sqlite3_bind_int64(
        __pRtree_ref.pDeleteParent,
        1 as ::core::ffi::c_int,
        __pNode_ref.iNode as crate::src::headers::sqlite3_h::Sqlite3Int64,
    );
    crate::src::src::vdbeapi::sqlite3_step(__pRtree_ref.pDeleteParent);
    rc = crate::src::src::vdbeapi::sqlite3_reset(__pRtree_ref.pDeleteParent);
    if crate::src::headers::sqlite3_h::SQLITE_OK != rc {
        return rc;
    }
    nodeHashDelete(pRtree, pNode);
    __pNode_ref.iNode = iHeight as I64_0;
    __pNode_ref.pNext = __pRtree_ref.pDeleted;
    __pNode_ref.nRef += 1;
    __pRtree_ref.pDeleted = pNode;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fixBoundingBox(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
) -> ::core::ffi::c_int {
    let pParent: *mut RtreeNode = (*pNode).pParent;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if !pParent.is_null() {
        let mut ii: ::core::ffi::c_int;
        let nCell: ::core::ffi::c_int =
            readInt16((*pNode).zData.offset(2_isize) as *mut U8_0);
        let mut box_0: RtreeCell = RtreeCell {
            iRowid: 0,
            aCoord: [RtreeCoord { f: 0. }; 10],
        };
        nodeGetCell(pRtree, pNode, 0 as ::core::ffi::c_int, &raw mut box_0);
        ii = 1 as ::core::ffi::c_int;
        while ii < nCell {
            let mut cell: RtreeCell = RtreeCell {
                iRowid: 0,
                aCoord: [RtreeCoord { f: 0. }; 10],
            };
            nodeGetCell(pRtree, pNode, ii, &raw mut cell);
            cellUnion(pRtree, &raw mut box_0, &raw mut cell);
            ii += 1;
        }
        box_0.iRowid = (*pNode).iNode;
        rc = nodeParentIndex(pRtree, pNode, &raw mut ii);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            nodeOverwriteCell(pRtree, pParent, &raw mut box_0, ii);
            rc = fixBoundingBox(pRtree, pParent);
        }
    }
    rc
}

unsafe extern "C" fn deleteCell(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
    iCell: ::core::ffi::c_int,
    iHeight: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    let mut rc: ::core::ffi::c_int;
    rc = fixLeafParent(pRtree, pNode);
    if crate::src::headers::sqlite3_h::SQLITE_OK != rc {
        return rc;
    }
    nodeDeleteCell(pRtree, pNode, iCell);
    let pParent: *mut RtreeNode = (*pNode).pParent;
    if !pParent.is_null() {
        if readInt16((*pNode).zData.offset(2_isize) as *mut U8_0)
            < ((*pRtree).iNodeSize - 4 as ::core::ffi::c_int)
                / (*pRtree).nBytesPerCell as ::core::ffi::c_int
                / 3 as ::core::ffi::c_int
        {
            rc = removeNode(pRtree, pNode, iHeight);
        } else {
            rc = fixBoundingBox(pRtree, pNode);
        }
    }
    rc
}

unsafe extern "C" fn rtreeInsertCell(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
    pCell: *mut RtreeCell,
    iHeight: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    if iHeight > 0 as ::core::ffi::c_int {
        let pChild: *mut RtreeNode = nodeHashLookup(pRtree, (*pCell).iRowid);
        if !pChild.is_null() {
            nodeRelease(pRtree, (*pChild).pParent);
            nodeReference(pNode);
            (*pChild).pParent = pNode;
        }
    }
    if nodeInsertCell(pRtree, pNode, pCell) != 0 {
        rc = SplitNode(pRtree, pNode, pCell, iHeight);
    } else {
        rc = AdjustTree(pRtree, pNode, pCell);
        if rc == 0 as ::core::ffi::c_int {
            if iHeight == 0 as ::core::ffi::c_int {
                rc = rowidWrite(
                    pRtree,
                    (*pCell).iRowid as crate::src::headers::sqlite3_h::Sqlite3Int64,
                    (*pNode).iNode as crate::src::headers::sqlite3_h::Sqlite3Int64,
                );
            } else {
                rc = parentWrite(
                    pRtree,
                    (*pCell).iRowid as crate::src::headers::sqlite3_h::Sqlite3Int64,
                    (*pNode).iNode as crate::src::headers::sqlite3_h::Sqlite3Int64,
                );
            }
        }
    }
    rc
}

unsafe extern "C" fn reinsertNodeContent(
    pRtree: *mut Rtree,
    pNode: *mut RtreeNode,
) -> ::core::ffi::c_int {
    let mut ii: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let nCell: ::core::ffi::c_int = readInt16((*pNode).zData.offset(2_isize) as *mut U8_0);
    ii = 0 as ::core::ffi::c_int;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && ii < nCell {
        let mut pInsert: *mut RtreeNode = ::core::ptr::null_mut::<RtreeNode>();
        let mut cell: RtreeCell = RtreeCell {
            iRowid: 0,
            aCoord: [RtreeCoord { f: 0. }; 10],
        };
        nodeGetCell(pRtree, pNode, ii, &raw mut cell);
        rc = ChooseLeaf(
            pRtree,
            &raw mut cell,
            (*pNode).iNode as ::core::ffi::c_int,
            &raw mut pInsert,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            
            rc = rtreeInsertCell(
                pRtree,
                pInsert,
                &raw mut cell,
                (*pNode).iNode as ::core::ffi::c_int,
            );
            let rc2: ::core::ffi::c_int = nodeRelease(pRtree, pInsert);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = rc2;
            }
        }
        ii += 1;
    }
    rc
}

unsafe extern "C" fn rtreeNewRowid(
    pRtree: *mut Rtree,
    piRowid: *mut I64_0,
) -> ::core::ffi::c_int {
    
    let __pRtree_ref = { &*pRtree };
    crate::src::src::vdbeapi::sqlite3_bind_null(__pRtree_ref.pWriteRowid, 1 as ::core::ffi::c_int);
    crate::src::src::vdbeapi::sqlite3_bind_null(__pRtree_ref.pWriteRowid, 2 as ::core::ffi::c_int);
    crate::src::src::vdbeapi::sqlite3_step(__pRtree_ref.pWriteRowid);
    let rc: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_reset(__pRtree_ref.pWriteRowid);
    *piRowid = crate::src::src::main::sqlite3_last_insert_rowid(__pRtree_ref.db) as I64_0;
    rc
}

unsafe extern "C" fn rtreeDeleteRowid(
    pRtree: *mut Rtree,
    iDelete: crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut pLeaf: *mut RtreeNode = ::core::ptr::null_mut::<RtreeNode>();
    let mut iCell: ::core::ffi::c_int = 0;
    let mut pRoot: *mut RtreeNode = ::core::ptr::null_mut::<RtreeNode>();
    rc = nodeAcquire(
        pRtree,
        1 as I64_0,
        ::core::ptr::null_mut::<RtreeNode>(),
        &raw mut pRoot,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = findLeafNode(
            pRtree,
            iDelete as I64_0,
            &raw mut pLeaf,
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Int64>(),
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !pLeaf.is_null() {
        
        rc = nodeRowidIndex(pRtree, pLeaf, iDelete as I64_0, &raw mut iCell);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = deleteCell(pRtree, pLeaf, iCell, 0 as ::core::ffi::c_int);
        }
        let rc2: ::core::ffi::c_int = nodeRelease(pRtree, pLeaf);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let __pRtree_ref = { &*pRtree };
        crate::src::src::vdbeapi::sqlite3_bind_int64(
            __pRtree_ref.pDeleteRowid,
            1 as ::core::ffi::c_int,
            iDelete,
        );
        crate::src::src::vdbeapi::sqlite3_step(__pRtree_ref.pDeleteRowid);
        rc = crate::src::src::vdbeapi::sqlite3_reset(__pRtree_ref.pDeleteRowid);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && (*pRtree).iDepth > 0 as ::core::ffi::c_int
        && readInt16((*pRoot).zData.offset(2_isize) as *mut U8_0) == 1 as ::core::ffi::c_int
    {
        
        let mut pChild: *mut RtreeNode = ::core::ptr::null_mut::<RtreeNode>();
        let iChild: I64_0 = nodeGetRowid(pRtree, pRoot, 0 as ::core::ffi::c_int);
        rc = nodeAcquire(pRtree, iChild, pRoot, &raw mut pChild);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = removeNode(pRtree, pChild, (*pRtree).iDepth - 1 as ::core::ffi::c_int);
        }
        let rc2_0: ::core::ffi::c_int = nodeRelease(pRtree, pChild);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = rc2_0;
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            (*pRtree).iDepth -= 1;
            writeInt16((*pRoot).zData, (*pRtree).iDepth);
            (*pRoot).isDirty = 1 as ::core::ffi::c_int;
        }
    }
    pLeaf = (*pRtree).pDeleted;
    while !pLeaf.is_null() {
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = reinsertNodeContent(pRtree, pLeaf);
        }
        let __pRtree_ref = { &mut *pRtree };
        __pRtree_ref.pDeleted = (*pLeaf).pNext;
        __pRtree_ref.nNodeRef = __pRtree_ref.nNodeRef.wrapping_sub(1);
        crate::src::src::malloc::sqlite3_free(pLeaf as *mut ::core::ffi::c_void);
        pLeaf = __pRtree_ref.pDeleted;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = nodeRelease(pRtree, pRoot);
    } else {
        nodeRelease(pRtree, pRoot);
    }
    rc
}

pub const RNDTOWARDS: ::core::ffi::c_double = 1.0f64 - 1.0f64 / 8388608.0f64;

pub const RNDAWAY: ::core::ffi::c_double = 1.0f64 + 1.0f64 / 8388608.0f64;

unsafe extern "C" fn rtreeValueDown(
    v: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> RtreeValue {
    let d: ::core::ffi::c_double = crate::src::src::vdbeapi::sqlite3_value_double(v);
    let mut f: ::core::ffi::c_float = d as ::core::ffi::c_float;
    if f as ::core::ffi::c_double > d {
        f = (d
            * (if d < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                RNDAWAY
            } else {
                RNDTOWARDS
            })) as ::core::ffi::c_float;
    }
    f as RtreeValue
}

unsafe extern "C" fn rtreeValueUp(
    v: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> RtreeValue {
    let d: ::core::ffi::c_double = crate::src::src::vdbeapi::sqlite3_value_double(v);
    let mut f: ::core::ffi::c_float = d as ::core::ffi::c_float;
    if (f as ::core::ffi::c_double) < d {
        f = (d
            * (if d < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                RNDTOWARDS
            } else {
                RNDAWAY
            })) as ::core::ffi::c_float;
    }
    f as RtreeValue
}

unsafe extern "C" fn rtreeConstraintError(
    pRtree: *mut Rtree,
    iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    
    let rc: ::core::ffi::c_int;
    let zSql: *mut ::core::ffi::c_char = crate::sqlite_printf!("SELECT * FROM %Q.%Q", (*pRtree).zDb, (*pRtree).zName);
    if !zSql.is_null() {
        rc = crate::src::src::prepare::sqlite3_prepare_v2(
            (*pRtree).db,
            zSql,
            -(1 as ::core::ffi::c_int),
            &raw mut pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
    } else {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if iCol == 0 as ::core::ffi::c_int {
            let zCol: *const ::core::ffi::c_char =
                crate::src::src::vdbeapi::sqlite3_column_name(pStmt, 0 as ::core::ffi::c_int);
            (*pRtree).base.zErrMsg =
                crate::sqlite_printf!("UNIQUE constraint failed: %s.%s", (*pRtree).zName, zCol);
        } else {
            let zCol1: *const ::core::ffi::c_char =
                crate::src::src::vdbeapi::sqlite3_column_name(pStmt, iCol);
            let zCol2: *const ::core::ffi::c_char =
                crate::src::src::vdbeapi::sqlite3_column_name(
                    pStmt,
                    iCol + 1 as ::core::ffi::c_int,
                );
            (*pRtree).base.zErrMsg = crate::sqlite_printf!(
                "rtree constraint failed: %s.(%s<=%s)",
                (*pRtree).zName,
                zCol1,
                zCol2
            );
        }
    }
    crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT
    } else {
        rc
    }
}

unsafe extern "C" fn rtreeUpdate(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    nData: ::core::ffi::c_int,
    aData: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pRowid: *mut crate::src::headers::sqlite3_h::SqliteInt64,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let pRtree: *mut Rtree = pVtab as *mut Rtree;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut cell: RtreeCell = { ::core::mem::zeroed() };
    let mut bHaveRowid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*pRtree).nNodeRef != 0 {
        return crate::src::headers::sqlite3_h::SQLITE_LOCKED_VTAB;
    }
    rtreeReference(pRtree);
    if nData > 1 as ::core::ffi::c_int {
        let mut ii: ::core::ffi::c_int;
        let mut nn: ::core::ffi::c_int = nData - 4 as ::core::ffi::c_int;
        if nn > (*pRtree).nDim2 as ::core::ffi::c_int {
            nn = (*pRtree).nDim2 as ::core::ffi::c_int;
        }
        if (*pRtree).eCoordType as ::core::ffi::c_int == RTREE_COORD_REAL32 {
            ii = 0 as ::core::ffi::c_int;
            loop {
                if (ii >= nn) {
                    current_block = 2668756484064249700;
                    break;
                }
                cell.aCoord[ii as usize].f =
                    rtreeValueDown(*aData.offset((ii + 3 as ::core::ffi::c_int) as isize));
                cell.aCoord[(ii + 1 as ::core::ffi::c_int) as usize].f =
                    rtreeValueUp(*aData.offset((ii + 4 as ::core::ffi::c_int) as isize));
                if cell.aCoord[ii as usize].f
                    > cell.aCoord[(ii + 1 as ::core::ffi::c_int) as usize].f
                {
                    rc = rtreeConstraintError(pRtree, ii + 1 as ::core::ffi::c_int);
                    current_block = 13133027019111582244;
                    break;
                } else {
                    ii += 2 as ::core::ffi::c_int;
                }
            }
        } else {
            ii = 0 as ::core::ffi::c_int;
            loop {
                if (ii >= nn) {
                    current_block = 2668756484064249700;
                    break;
                }
                cell.aCoord[ii as usize].i = crate::src::src::vdbeapi::sqlite3_value_int(
                    *aData.offset((ii + 3 as ::core::ffi::c_int) as isize),
                );
                cell.aCoord[(ii + 1 as ::core::ffi::c_int) as usize].i =
                    crate::src::src::vdbeapi::sqlite3_value_int(
                        *aData.offset((ii + 4 as ::core::ffi::c_int) as isize),
                    );
                if cell.aCoord[ii as usize].i
                    > cell.aCoord[(ii + 1 as ::core::ffi::c_int) as usize].i
                {
                    rc = rtreeConstraintError(pRtree, ii + 1 as ::core::ffi::c_int);
                    current_block = 13133027019111582244;
                    break;
                } else {
                    ii += 2 as ::core::ffi::c_int;
                }
            }
        }
        match current_block {
            13133027019111582244 => {}
            _ => {
                if crate::src::src::vdbeapi::sqlite3_value_type(*aData.offset(2_isize))
                    != crate::src::headers::sqlite3_h::SQLITE_NULL
                {
                    cell.iRowid =
                        crate::src::src::vdbeapi::sqlite3_value_int64(*aData.offset(2_isize))
                            as I64_0;
                    if crate::src::src::vdbeapi::sqlite3_value_type(*aData.offset(0_isize))
                        == crate::src::headers::sqlite3_h::SQLITE_NULL
                        || crate::src::src::vdbeapi::sqlite3_value_int64(*aData.offset(0_isize))
                            != cell.iRowid
                    {
                        
                        let __pRtree_ref = { &*pRtree };
                        crate::src::src::vdbeapi::sqlite3_bind_int64(
                            __pRtree_ref.pReadRowid,
                            1 as ::core::ffi::c_int,
                            cell.iRowid as crate::src::headers::sqlite3_h::Sqlite3Int64,
                        );
                        let steprc: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_step(__pRtree_ref.pReadRowid);
                        rc = crate::src::src::vdbeapi::sqlite3_reset(__pRtree_ref.pReadRowid);
                        if crate::src::headers::sqlite3_h::SQLITE_ROW == steprc {
                            if crate::src::src::vtab::sqlite3_vtab_on_conflict(__pRtree_ref.db)
                                == crate::src::headers::sqlite3_h::SQLITE_REPLACE
                            {
                                rc = rtreeDeleteRowid(
                                    pRtree,
                                    cell.iRowid as crate::src::headers::sqlite3_h::Sqlite3Int64,
                                );
                                current_block = 5494826135382683477;
                            } else {
                                rc = rtreeConstraintError(pRtree, 0 as ::core::ffi::c_int);
                                current_block = 13133027019111582244;
                            }
                        } else {
                            current_block = 5494826135382683477;
                        }
                    } else {
                        current_block = 5494826135382683477;
                    }
                    match current_block {
                        13133027019111582244 => {}
                        _ => {
                            bHaveRowid = 1 as ::core::ffi::c_int;
                            current_block = 14434620278749266018;
                        }
                    }
                } else {
                    current_block = 14434620278749266018;
                }
            }
        }
    } else {
        current_block = 14434620278749266018;
    }
    match current_block {
        14434620278749266018 => {
            if crate::src::src::vdbeapi::sqlite3_value_type(*aData.offset(0_isize))
                != crate::src::headers::sqlite3_h::SQLITE_NULL
            {
                rc = rtreeDeleteRowid(
                    pRtree,
                    crate::src::src::vdbeapi::sqlite3_value_int64(*aData.offset(0_isize)),
                );
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK && nData > 1 as ::core::ffi::c_int {
                let mut pLeaf: *mut RtreeNode = ::core::ptr::null_mut::<RtreeNode>();
                if bHaveRowid == 0 as ::core::ffi::c_int {
                    rc = rtreeNewRowid(pRtree, &raw mut cell.iRowid);
                }
                *pRowid = cell.iRowid as crate::src::headers::sqlite3_h::SqliteInt64;
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    rc = ChooseLeaf(
                        pRtree,
                        &raw mut cell,
                        0 as ::core::ffi::c_int,
                        &raw mut pLeaf,
                    );
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    
                    rc = rtreeInsertCell(pRtree, pLeaf, &raw mut cell, 0 as ::core::ffi::c_int);
                    let rc2: ::core::ffi::c_int = nodeRelease(pRtree, pLeaf);
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        rc = rc2;
                    }
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                    && (*pRtree).nAux as ::core::ffi::c_int != 0
                {
                    let pUp: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
                        (*pRtree).pWriteAux;
                    let mut jj: ::core::ffi::c_int;
                    crate::src::src::vdbeapi::sqlite3_bind_int64(
                        pUp,
                        1 as ::core::ffi::c_int,
                        *pRowid,
                    );
                    jj = 0 as ::core::ffi::c_int;
                    while jj < (*pRtree).nAux as ::core::ffi::c_int {
                        crate::src::src::vdbeapi::sqlite3_bind_value(
                            pUp,
                            jj + 2 as ::core::ffi::c_int,
                            *aData.offset(
                                ((*pRtree).nDim2 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int
                                    + jj) as isize,
                            ),
                        );
                        jj += 1;
                    }
                    crate::src::src::vdbeapi::sqlite3_step(pUp);
                    rc = crate::src::src::vdbeapi::sqlite3_reset(pUp);
                }
            }
        }
        _ => {}
    }
    rtreeRelease(pRtree);
    rc
}

unsafe extern "C" fn rtreeBeginTransaction(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
) -> ::core::ffi::c_int {
    let pRtree = &mut *(pVtab as *mut Rtree);
    pRtree.inWrTrans = 1 as U8_0;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn rtreeEndTransaction(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
) -> ::core::ffi::c_int {
    let pRtree: *mut Rtree = pVtab as *mut Rtree;
    (*pRtree).inWrTrans = 0 as U8_0;
    nodeBlobReset(pRtree);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn rtreeRollback(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
) -> ::core::ffi::c_int {
    rtreeEndTransaction(pVtab)
}

unsafe extern "C" fn rtreeRename(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    zNewName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let pRtree: *mut Rtree = pVtab as *mut Rtree;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    let __pRtree_ref = { &*pRtree };
    let zSql: *mut ::core::ffi::c_char = crate::sqlite_printf!(
        "ALTER TABLE %Q.'%q_node'   RENAME TO \"%w_node\";ALTER TABLE %Q.'%q_parent' RENAME TO \"%w_parent\";ALTER TABLE %Q.'%q_rowid'  RENAME TO \"%w_rowid\";",
        __pRtree_ref.zDb,
        __pRtree_ref.zName,
        zNewName,
        __pRtree_ref.zDb,
        __pRtree_ref.zName,
        zNewName,
        __pRtree_ref.zDb,
        __pRtree_ref.zName,
        zNewName,
    );
    if !zSql.is_null() {
        nodeBlobReset(pRtree);
        rc = crate::src::src::legacy::sqlite3_exec(
            __pRtree_ref.db,
            zSql,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
    }
    rc
}

unsafe extern "C" fn rtreeSavepoint(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    mut _iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pRtree: *mut Rtree = pVtab as *mut Rtree;
    let __pRtree_ref = { &mut *pRtree };
    let iwt: U8_0 = __pRtree_ref.inWrTrans;
    __pRtree_ref.inWrTrans = 0 as U8_0;
    nodeBlobReset(pRtree);
    __pRtree_ref.inWrTrans = iwt;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn rtreeQueryStat1(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pRtree: *mut Rtree,
) -> ::core::ffi::c_int {
    
    let mut p: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let mut rc: ::core::ffi::c_int;
    let mut nRow: I64_0 = RTREE_MIN_ROWEST as I64_0;
    let __pRtree_ref = { &mut *pRtree };
    rc = crate::src::src::main::sqlite3_table_column_metadata(
        db,
        __pRtree_ref.zDb,
        b"sqlite_stat1\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        __pRtree_ref.nRowEst = RTREE_DEFAULT_ROWEST as I64_0;
        return if rc == crate::src::headers::sqlite3_h::SQLITE_ERROR {
            crate::src::headers::sqlite3_h::SQLITE_OK
        } else {
            rc
        };
    }
    let zSql: *mut ::core::ffi::c_char = crate::sqlite_printf!(
        "SELECT stat FROM %Q.sqlite_stat1 WHERE tbl = '%q_rowid'",
        __pRtree_ref.zDb,
        __pRtree_ref.zName
    );
    if zSql.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    } else {
        rc = crate::src::src::prepare::sqlite3_prepare_v2(
            db,
            zSql,
            -(1 as ::core::ffi::c_int),
            &raw mut p,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            if crate::src::src::vdbeapi::sqlite3_step(p)
                == crate::src::headers::sqlite3_h::SQLITE_ROW
            {
                nRow = crate::src::src::vdbeapi::sqlite3_column_int64(p, 0 as ::core::ffi::c_int)
                    as I64_0;
            }
            rc = crate::src::src::vdbeapi::sqlite3_finalize(p);
        }
        crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
    }
    __pRtree_ref.nRowEst = if nRow < 100 as I64_0 {
        100 as I64_0
    } else {
        nRow
    };
    rc
}

unsafe extern "C" fn rtreeShadowName(zName: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    static mut azName: [*const ::core::ffi::c_char; 3] = [
        b"node\0" as *const u8 as *const ::core::ffi::c_char,
        b"parent\0" as *const u8 as *const ::core::ffi::c_char,
        b"rowid\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut i: ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_uint;
    while (i as usize)
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if crate::src::src::util::sqlite3_stricmp(zName, azName[i as usize])
            == 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    0 as ::core::ffi::c_int
}

static mut rtreeModule: crate::src::headers::sqlite3_h::sqlite3_module = {
    crate::src::headers::sqlite3_h::sqlite3_module {
        iVersion: 4 as ::core::ffi::c_int,
        xCreate: Some(
            rtreeCreate
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqliteInt_h::sqlite3,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const *const ::core::ffi::c_char,
                    *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xConnect: Some(
            rtreeConnect
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqliteInt_h::sqlite3,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const *const ::core::ffi::c_char,
                    *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xBestIndex: Some(
            rtreeBestIndex
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            rtreeDisconnect
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                ) -> ::core::ffi::c_int,
        ),
        xDestroy: Some(
            rtreeDestroy
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                ) -> ::core::ffi::c_int,
        ),
        xOpen: Some(
            rtreeOpen
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            rtreeClose
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            rtreeFilter
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            rtreeNext
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            rtreeEof
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            rtreeColumn
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                    *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            rtreeRowid
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                    *mut crate::src::headers::sqlite3_h::SqliteInt64,
                ) -> ::core::ffi::c_int,
        ),
        xUpdate: Some(
            rtreeUpdate
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    ::core::ffi::c_int,
                    *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    *mut crate::src::headers::sqlite3_h::SqliteInt64,
                ) -> ::core::ffi::c_int,
        ),
        xBegin: Some(
            rtreeBeginTransaction
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                ) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            rtreeEndTransaction
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                ) -> ::core::ffi::c_int,
        ),
        xCommit: Some(
            rtreeEndTransaction
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                ) -> ::core::ffi::c_int,
        ),
        xRollback: Some(
            rtreeRollback
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                ) -> ::core::ffi::c_int,
        ),
        xFindFunction: None,
        xRename: Some(
            rtreeRename
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xSavepoint: Some(
            rtreeSavepoint
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRelease: None,
        xRollbackTo: None,
        xShadowName: Some(
            rtreeShadowName
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
        ),
        xIntegrity: Some(
            rtreeIntegrity
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    }
};

unsafe extern "C" fn rtreeSqlInit(
    pRtree: *mut Rtree,
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zDb: *const ::core::ffi::c_char,
    zPrefix: *const ::core::ffi::c_char,
    isCreate: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut appStmt: [*mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt; 8] =
        [::core::ptr::null_mut::<*mut crate::src::headers::sqlite3_h::Sqlite3Stmt>(); 8];
    let mut i: ::core::ffi::c_int;
    let f: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_PREPARE_PERSISTENT
        | crate::src::headers::sqlite3_h::SQLITE_PREPARE_NO_VTAB;
    let __pRtree_ref = { &mut *pRtree };
    __pRtree_ref.db = db;
    if isCreate != 0 {
        
        let p: *mut crate::src::headers::sqliteInt_h::sqlite3_str =
            crate::src::src::printf::sqlite3_str_new(db);
        let mut ii: ::core::ffi::c_int;
        sqlite3_str_vappendf2(
            p,
            "CREATE TABLE \"%w\".\"%w_rowid\"(rowid INTEGER PRIMARY KEY,nodeno",
            crate::printf_args!(zDb, zPrefix),
        );
        ii = 0 as ::core::ffi::c_int;
        while ii < __pRtree_ref.nAux as ::core::ffi::c_int {
            sqlite3_str_vappendf2(p, ",a%d", crate::printf_args!(ii));
            ii += 1;
        }
        sqlite3_str_vappendf2(
            p,
            ");CREATE TABLE \"%w\".\"%w_node\"(nodeno INTEGER PRIMARY KEY,data);",
            crate::printf_args!(zDb, zPrefix),
        );
        sqlite3_str_vappendf2(
            p,
            "CREATE TABLE \"%w\".\"%w_parent\"(nodeno INTEGER PRIMARY KEY,parentnode);",
            crate::printf_args!(zDb, zPrefix),
        );
        sqlite3_str_vappendf2(
            p,
            "INSERT INTO \"%w\".\"%w_node\"VALUES(1,zeroblob(%d))",
            crate::printf_args!(zDb, zPrefix, __pRtree_ref.iNodeSize),
        );
        let zCreate: *mut ::core::ffi::c_char = crate::src::src::printf::sqlite3_str_finish(p);
        if zCreate.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        rc = crate::src::src::legacy::sqlite3_exec(
            db,
            zCreate,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        crate::src::src::malloc::sqlite3_free(zCreate as *mut ::core::ffi::c_void);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
    }
    appStmt[0 as ::core::ffi::c_int as usize] = &raw mut __pRtree_ref.pWriteNode;
    appStmt[1 as ::core::ffi::c_int as usize] = &raw mut __pRtree_ref.pDeleteNode;
    appStmt[2 as ::core::ffi::c_int as usize] = &raw mut __pRtree_ref.pReadRowid;
    appStmt[3 as ::core::ffi::c_int as usize] = &raw mut __pRtree_ref.pWriteRowid;
    appStmt[4 as ::core::ffi::c_int as usize] = &raw mut __pRtree_ref.pDeleteRowid;
    appStmt[5 as ::core::ffi::c_int as usize] = &raw mut __pRtree_ref.pReadParent;
    appStmt[6 as ::core::ffi::c_int as usize] = &raw mut __pRtree_ref.pWriteParent;
    appStmt[7 as ::core::ffi::c_int as usize] = &raw mut __pRtree_ref.pDeleteParent;
    rc = rtreeQueryStat1(db, pRtree);
    i = 0 as ::core::ffi::c_int;
    while i < N_STATEMENT && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let zSql: *mut ::core::ffi::c_char = if i == 0 as ::core::ffi::c_int {
            crate::sqlite_printf!(
                "INSERT OR REPLACE INTO '%q'.'%q_node' VALUES(?1, ?2)",
                zDb,
                zPrefix
            )
        } else if i == 1 as ::core::ffi::c_int {
            crate::sqlite_printf!("DELETE FROM '%q'.'%q_node' WHERE nodeno = ?1", zDb, zPrefix)
        } else if i == 2 as ::core::ffi::c_int {
            crate::sqlite_printf!(
                "SELECT nodeno FROM '%q'.'%q_rowid' WHERE rowid = ?1",
                zDb,
                zPrefix
            )
        } else if i == 3 as ::core::ffi::c_int
            && __pRtree_ref.nAux as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            crate::sqlite_printf!(
                "INSERT INTO\"%w\".\"%w_rowid\"(rowid,nodeno)VALUES(?1,?2)ON CONFLICT(rowid)DO UPDATE SET nodeno=excluded.nodeno",
                zDb,
                zPrefix
            )
        } else if i == 3 as ::core::ffi::c_int {
            crate::sqlite_printf!(
                "INSERT OR REPLACE INTO '%q'.'%q_rowid' VALUES(?1, ?2)",
                zDb,
                zPrefix
            )
        } else if i == 4 as ::core::ffi::c_int {
            crate::sqlite_printf!("DELETE FROM '%q'.'%q_rowid' WHERE rowid = ?1", zDb, zPrefix)
        } else if i == 5 as ::core::ffi::c_int {
            crate::sqlite_printf!(
                "SELECT parentnode FROM '%q'.'%q_parent' WHERE nodeno = ?1",
                zDb,
                zPrefix
            )
        } else if i == 6 as ::core::ffi::c_int {
            crate::sqlite_printf!(
                "INSERT OR REPLACE INTO '%q'.'%q_parent' VALUES(?1, ?2)",
                zDb,
                zPrefix
            )
        } else {
            crate::sqlite_printf!(
                "DELETE FROM '%q'.'%q_parent' WHERE nodeno = ?1",
                zDb,
                zPrefix
            )
        };
        if !zSql.is_null() {
            rc = crate::src::src::prepare::sqlite3_prepare_v3(
                db,
                zSql,
                -(1 as ::core::ffi::c_int),
                f as ::core::ffi::c_uint,
                appStmt[i as usize],
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
        } else {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
        i += 1;
    }
    if __pRtree_ref.nAux as ::core::ffi::c_int != 0
        && rc != crate::src::headers::sqlite3_h::SQLITE_NOMEM
    {
        __pRtree_ref.zReadAuxSql = crate::sqlite_printf!(
            "SELECT * FROM \"%w\".\"%w_rowid\" WHERE rowid=?1",
            zDb,
            zPrefix
        );
        if __pRtree_ref.zReadAuxSql.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            let p_0: *mut crate::src::headers::sqliteInt_h::sqlite3_str =
                crate::src::src::printf::sqlite3_str_new(db);
            let mut ii_0: ::core::ffi::c_int;
            
            sqlite3_str_vappendf2(
                p_0,
                "UPDATE \"%w\".\"%w_rowid\"SET ",
                crate::printf_args!(zDb, zPrefix),
            );
            ii_0 = 0 as ::core::ffi::c_int;
            while ii_0 < __pRtree_ref.nAux as ::core::ffi::c_int {
                if ii_0 != 0 {
                    crate::src::src::printf::sqlite3_str_append(
                        p_0,
                        b",\0" as *const u8 as *const ::core::ffi::c_char,
                        1 as ::core::ffi::c_int,
                    );
                }
                if ii_0 < __pRtree_ref.nAuxNotNull as ::core::ffi::c_int {
                    sqlite3_str_vappendf2(
                        p_0,
                        "a%d=coalesce(?%d,a%d)",
                        crate::printf_args!(ii_0, ii_0 + 2 as ::core::ffi::c_int, ii_0),
                    );
                } else {
                    sqlite3_str_vappendf2(
                        p_0,
                        "a%d=?%d",
                        crate::printf_args!(ii_0, ii_0 + 2 as ::core::ffi::c_int),
                    );
                }
                ii_0 += 1;
            }
            sqlite3_str_vappendf2(p_0, " WHERE rowid=?1", crate::printf_args!());
            let zSql_0: *mut ::core::ffi::c_char = crate::src::src::printf::sqlite3_str_finish(p_0);
            if zSql_0.is_null() {
                rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            } else {
                rc = crate::src::src::prepare::sqlite3_prepare_v3(
                    db,
                    zSql_0,
                    -(1 as ::core::ffi::c_int),
                    f as ::core::ffi::c_uint,
                    &raw mut __pRtree_ref.pWriteAux,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                crate::src::src::malloc::sqlite3_free(zSql_0 as *mut ::core::ffi::c_void);
            }
        }
    }
    rc
}

pub const N_STATEMENT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

unsafe extern "C" fn getIntFromStmt(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zSql: *const ::core::ffi::c_char,
    piVal: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    if !zSql.is_null() {
        let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
        rc = crate::src::src::prepare::sqlite3_prepare_v2(
            db,
            zSql,
            -(1 as ::core::ffi::c_int),
            &raw mut pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            if crate::src::headers::sqlite3_h::SQLITE_ROW
                == crate::src::src::vdbeapi::sqlite3_step(pStmt)
            {
                *piVal =
                    crate::src::src::vdbeapi::sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
            }
            rc = crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
        }
    }
    rc
}

unsafe extern "C" fn getNodeSize(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pRtree: *mut Rtree,
    isCreate: ::core::ffi::c_int,
    pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let zSql: *mut ::core::ffi::c_char;
    if isCreate != 0 {
        let mut iPageSize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        zSql = crate::sqlite_printf!("PRAGMA %Q.page_size", (*pRtree).zDb);
        rc = getIntFromStmt(db, zSql, &raw mut iPageSize);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let __pRtree_ref = { &mut *pRtree };
            __pRtree_ref.iNodeSize = iPageSize - 64 as ::core::ffi::c_int;
            if 4 as ::core::ffi::c_int
                + __pRtree_ref.nBytesPerCell as ::core::ffi::c_int * RTREE_MAXCELLS
                < __pRtree_ref.iNodeSize
            {
                __pRtree_ref.iNodeSize = 4 as ::core::ffi::c_int
                    + __pRtree_ref.nBytesPerCell as ::core::ffi::c_int * RTREE_MAXCELLS;
            }
        } else {
            *pzErr = crate::sqlite_printf!("%s", crate::src::src::main::sqlite3_errmsg(db));
        }
    } else {
        let __pRtree_ref = { &mut *pRtree };
        zSql = crate::sqlite_printf!(
            "SELECT length(data) FROM '%q'.'%q_node' WHERE nodeno = 1",
            __pRtree_ref.zDb,
            __pRtree_ref.zName
        );
        rc = getIntFromStmt(db, zSql, &raw mut __pRtree_ref.iNodeSize);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            *pzErr = crate::sqlite_printf!("%s", crate::src::src::main::sqlite3_errmsg(db));
        } else if __pRtree_ref.iNodeSize < 512 as ::core::ffi::c_int - 64 as ::core::ffi::c_int {
            rc = crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
            *pzErr =
                crate::sqlite_printf!("undersize RTree blobs in \"%q_node\"", __pRtree_ref.zName);
        }
    }
    crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn rtreeTokenLength(z: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut dummy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    sqlite3GetToken(z as *const ::core::ffi::c_uchar, &raw mut dummy) as ::core::ffi::c_int
}

unsafe extern "C" fn rtreeInit(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pAux: *mut ::core::ffi::c_void,
    argc: ::core::ffi::c_int,
    argv: *const *const ::core::ffi::c_char,
    ppVtab: *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    pzErr: *mut *mut ::core::ffi::c_char,
    isCreate: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    
    
    
    let eCoordType: ::core::ffi::c_int = if !pAux.is_null() {
        RTREE_COORD_INT32
    } else {
        RTREE_COORD_REAL32
    };
    
    
    let mut ii: ::core::ffi::c_int;
    let iErr: ::core::ffi::c_int;
    let aErrMsg: [*const ::core::ffi::c_char; 5] = [
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"Wrong number of columns for an rtree table\0" as *const u8 as *const ::core::ffi::c_char,
        b"Too few columns for an rtree table\0" as *const u8 as *const ::core::ffi::c_char,
        b"Too many columns for an rtree table\0" as *const u8 as *const ::core::ffi::c_char,
        b"Auxiliary rtree columns must be last\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    if argc < 6 as ::core::ffi::c_int || argc > RTREE_MAX_AUX_COLUMN + 3 as ::core::ffi::c_int {
        *pzErr = crate::sqlite_printf!(
            "%s",
            aErrMsg[(2 as ::core::ffi::c_int
                + (argc >= 6 as ::core::ffi::c_int) as ::core::ffi::c_int)
                as usize]
        );
        return crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    {
        let _vtab_args: [u64; 1] = [1 as ::core::ffi::c_int as u64];
        crate::src::src::vtab::sqlite3_vtab_config_args(
            db,
            crate::src::headers::sqlite3_h::SQLITE_VTAB_CONSTRAINT_SUPPORT,
            _vtab_args.as_ptr(),
        );
    }
    {
        let _vtab_args: [u64; 1] = [0];
        crate::src::src::vtab::sqlite3_vtab_config_args(
            db,
            crate::src::headers::sqlite3_h::SQLITE_VTAB_INNOCUOUS,
            _vtab_args.as_ptr(),
        );
    }
    let nDb: ::core::ffi::c_int = ::libc::strlen(*argv.offset(1_isize)) as ::core::ffi::c_int;
    let nName: ::core::ffi::c_int = ::libc::strlen(*argv.offset(2_isize)) as ::core::ffi::c_int;
    let pRtree: *mut Rtree = crate::src::src::malloc::sqlite3_malloc64(
        (::core::mem::size_of::<Rtree>() as usize)
            .wrapping_add(nDb as usize)
            .wrapping_add((nName * 2 as ::core::ffi::c_int) as usize)
            .wrapping_add(8_usize) as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut Rtree;
    if pRtree.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        pRtree as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<Rtree>() as crate::__stddef_size_t_h::SizeT)
            .wrapping_add(nDb as crate::__stddef_size_t_h::SizeT)
            .wrapping_add((nName * 2 as ::core::ffi::c_int) as crate::__stddef_size_t_h::SizeT)
            .wrapping_add(8 as crate::__stddef_size_t_h::SizeT),
    );
    (*pRtree).nBusy = 1 as U32_0;
    (*pRtree).base.pModule = &raw mut rtreeModule;
    (*pRtree).zDb = pRtree.offset(1_isize) as *mut Rtree as *mut ::core::ffi::c_char;
    (*pRtree).zName = (*pRtree)
        .zDb
        .offset((nDb + 1 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_char;
    (*pRtree).zNodeName = (*pRtree)
        .zName
        .offset((nName + 1 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_char;
    (*pRtree).eCoordType = eCoordType as U8_0;
    ::core::ptr::copy_nonoverlapping(
        *argv.offset(1_isize) as *const u8,
        (*pRtree).zDb as *mut u8,
        nDb as usize,
    );
    ::core::ptr::copy_nonoverlapping(
        *argv.offset(2_isize) as *const u8,
        (*pRtree).zName as *mut u8,
        nName as usize,
    );
    ::core::ptr::copy_nonoverlapping(
        *argv.offset(2_isize) as *const u8,
        (*pRtree).zNodeName as *mut u8,
        nName as usize,
    );
    ::core::ptr::copy_nonoverlapping(
        b"_node\0" as *const u8 as *const ::core::ffi::c_char,
        (*pRtree).zNodeName.offset(nName as isize) as *mut ::core::ffi::c_char,
        6_usize,
    );
    let pSql: *mut crate::src::headers::sqliteInt_h::sqlite3_str = crate::src::src::printf::sqlite3_str_new(db);
    sqlite3_str_vappendf2(
        pSql,
        "CREATE TABLE x(%.*s INT",
        crate::printf_args!(
            rtreeTokenLength(*argv.offset(3_isize)),
            *argv.offset(3_isize)
        ),
    );
    ii = 4 as ::core::ffi::c_int;
    while ii < argc {
        let zArg: *const ::core::ffi::c_char = *argv.offset(ii as isize);
        if *zArg.offset(0_isize) as ::core::ffi::c_int == '+' as i32 {
            (*pRtree).nAux = (*pRtree).nAux.wrapping_add(1);
            sqlite3_str_vappendf2(
                pSql,
                ",%.*s",
                crate::printf_args!(
                    rtreeTokenLength(zArg.offset(1_isize)),
                    zArg.offset(1_isize)
                ),
            );
        } else {
            let __pRtree_ref = { &mut *pRtree };
            if __pRtree_ref.nAux as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                break;
            }
            __pRtree_ref.nDim2 = __pRtree_ref.nDim2.wrapping_add(1);
            if eCoordType as usize == 0 {
                sqlite3_str_vappendf2(
                    pSql,
                    ",%.*s REAL",
                    crate::printf_args!(rtreeTokenLength(zArg), zArg),
                );
            } else {
                sqlite3_str_vappendf2(
                    pSql,
                    ",%.*s INT",
                    crate::printf_args!(rtreeTokenLength(zArg), zArg),
                );
            }
        }
        ii += 1;
    }
    sqlite3_str_vappendf2(pSql, ");", crate::printf_args!());
    let zSql: *mut ::core::ffi::c_char = crate::src::src::printf::sqlite3_str_finish(pSql);
    if zSql.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    } else if ii < argc {
        *pzErr = crate::sqlite_printf!("%s", aErrMsg[4 as ::core::ffi::c_int as usize]);
        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    } else {
        rc = crate::src::src::vtab::sqlite3_declare_vtab(db, zSql);
        if crate::src::headers::sqlite3_h::SQLITE_OK != rc {
            *pzErr = crate::sqlite_printf!("%s", crate::src::src::main::sqlite3_errmsg(db));
        }
    }
    crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
    if (rc == 0) {
        let __pRtree_ref = { &mut *pRtree };
        __pRtree_ref.nDim =
            (__pRtree_ref.nDim2 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as U8_0;
        if (__pRtree_ref.nDim as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
            iErr = 2 as ::core::ffi::c_int;
        } else if __pRtree_ref.nDim2 as ::core::ffi::c_int
            > RTREE_MAX_DIMENSIONS * 2 as ::core::ffi::c_int
        {
            iErr = 3 as ::core::ffi::c_int;
        } else if __pRtree_ref.nDim2 as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0 {
            iErr = 1 as ::core::ffi::c_int;
        } else {
            iErr = 0 as ::core::ffi::c_int;
        }
        if iErr != 0 {
            *pzErr = crate::sqlite_printf!("%s", aErrMsg[iErr as usize]);
        } else {
            __pRtree_ref.nBytesPerCell = (8 as ::core::ffi::c_int
                + __pRtree_ref.nDim2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as U8_0;
            rc = getNodeSize(db, pRtree, isCreate, pzErr);
            if (rc == 0) {
                rc = rtreeSqlInit(
                    pRtree,
                    db,
                    *argv.offset(1_isize),
                    *argv.offset(2_isize),
                    isCreate,
                );
                if rc != 0 {
                    *pzErr = crate::sqlite_printf!("%s", crate::src::src::main::sqlite3_errmsg(db));
                } else {
                    *ppVtab = pRtree as *mut crate::src::headers::sqlite3_h::sqlite3_vtab;
                    return crate::src::headers::sqlite3_h::SQLITE_OK;
                }
            }
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    rtreeRelease(pRtree);
    rc
}

unsafe extern "C" fn rtreenode(
    ctx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut _nArg: ::core::ffi::c_int,
    apArg: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let mut node: RtreeNode = { ::core::mem::zeroed() };
    let mut tree: Rtree = { ::core::mem::zeroed() };
    let mut ii: ::core::ffi::c_int;
    
    
    
    tree.nDim = crate::src::src::vdbeapi::sqlite3_value_int(*apArg.offset(0_isize)) as U8_0;
    if (tree.nDim as ::core::ffi::c_int) < 1 as ::core::ffi::c_int
        || tree.nDim as ::core::ffi::c_int > 5 as ::core::ffi::c_int
    {
        return;
    }
    tree.nDim2 = (tree.nDim as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as U8_0;
    tree.nBytesPerCell = (8 as ::core::ffi::c_int
        + 8 as ::core::ffi::c_int * tree.nDim as ::core::ffi::c_int)
        as U8_0;
    node.zData =
        crate::src::src::vdbeapi::sqlite3_value_blob(*apArg.offset(1_isize)) as *mut U8_0;
    if node.zData.is_null() {
        return;
    }
    let nData: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_value_bytes(*apArg.offset(1_isize));
    if nData < 4 as ::core::ffi::c_int {
        return;
    }
    if nData
        < 4 as ::core::ffi::c_int
            + readInt16(node.zData.offset(2_isize) as *mut U8_0)
                * tree.nBytesPerCell as ::core::ffi::c_int
    {
        return;
    }
    let pOut: *mut crate::src::headers::sqliteInt_h::sqlite3_str = crate::src::src::printf::sqlite3_str_new(::core::ptr::null_mut::<
        crate::src::headers::sqliteInt_h::sqlite3,
    >());
    ii = 0 as ::core::ffi::c_int;
    while ii < readInt16(node.zData.offset(2_isize) as *mut U8_0) {
        let mut cell: RtreeCell = RtreeCell {
            iRowid: 0,
            aCoord: [RtreeCoord { f: 0. }; 10],
        };
        let mut jj: ::core::ffi::c_int;
        nodeGetCell(&raw mut tree, &raw mut node, ii, &raw mut cell);
        if ii > 0 as ::core::ffi::c_int {
            crate::src::src::printf::sqlite3_str_append(
                pOut,
                b" \0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
        }
        sqlite3_str_vappendf2(pOut, "{%lld", crate::printf_args!(cell.iRowid));
        jj = 0 as ::core::ffi::c_int;
        while jj < tree.nDim2 as ::core::ffi::c_int {
            sqlite3_str_vappendf2(
                pOut,
                " %g",
                crate::printf_args!(cell.aCoord[jj as usize].f as ::core::ffi::c_double),
            );
            jj += 1;
        }
        crate::src::src::printf::sqlite3_str_append(
            pOut,
            b"}\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        ii += 1;
    }
    let errCode: ::core::ffi::c_int = crate::src::src::printf::sqlite3_str_errcode(pOut);
    crate::src::src::vdbeapi::sqlite3_result_error_code(ctx, errCode);
    crate::src::src::vdbeapi::sqlite3_result_text(
        ctx,
        crate::src::src::printf::sqlite3_str_finish(pOut),
        -(1 as ::core::ffi::c_int),
        Some(
            crate::src::src::malloc::sqlite3_free
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
    );
}

unsafe extern "C" fn rtreedepth(
    ctx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut _nArg: ::core::ffi::c_int,
    apArg: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    if crate::src::src::vdbeapi::sqlite3_value_type(*apArg.offset(0_isize))
        != crate::src::headers::sqlite3_h::SQLITE_BLOB
        || crate::src::src::vdbeapi::sqlite3_value_bytes(*apArg.offset(0_isize))
            < 2 as ::core::ffi::c_int
    {
        crate::src::src::vdbeapi::sqlite3_result_error(
            ctx,
            b"Invalid argument to rtreedepth()\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int),
        );
    } else {
        let zBlob: *mut U8_0 =
            crate::src::src::vdbeapi::sqlite3_value_blob(*apArg.offset(0_isize)) as *mut U8_0;
        if !zBlob.is_null() {
            crate::src::src::vdbeapi::sqlite3_result_int(ctx, readInt16(zBlob));
        } else {
            crate::src::src::vdbeapi::sqlite3_result_error_nomem(ctx);
        }
    };
}

pub const RTREE_CHECK_MAX_ERROR: ::core::ffi::c_int = 100 as ::core::ffi::c_int;

pub unsafe fn rtreeCheckPrepare(
    pCheck: *mut RtreeCheck,
    zFmt: *const ::core::ffi::c_char,
    args: &[crate::src::src::printf::PrintfArg],
) -> *mut crate::src::headers::sqlite3_h::Sqlite3Stmt {
    
    let mut pRet: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let z: *mut ::core::ffi::c_char = crate::src::src::printf::sqlite3_vmprintf_args(zFmt, args);
    if (*pCheck).rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        if z.is_null() {
            (*pCheck).rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            (*pCheck).rc = crate::src::src::prepare::sqlite3_prepare_v2(
                (*pCheck).db,
                z,
                -(1 as ::core::ffi::c_int),
                &raw mut pRet,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
        }
    }
    crate::src::src::malloc::sqlite3_free(z as *mut ::core::ffi::c_void);
    pRet
}

pub unsafe fn rtreeCheckAppendMsg(
    pCheck: *mut RtreeCheck,
    zFmt: *const ::core::ffi::c_char,
    args: &[crate::src::src::printf::PrintfArg],
) {
    if (*pCheck).rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && (*pCheck).nErr < RTREE_CHECK_MAX_ERROR
    {
        let z: *mut ::core::ffi::c_char =
            crate::src::src::printf::sqlite3_vmprintf_args(zFmt, args);
        if z.is_null() {
            (*pCheck).rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            let __pCheck_ref = { &mut *pCheck };
            let zOldReport: *mut ::core::ffi::c_char = __pCheck_ref.zReport;
            __pCheck_ref.zReport = crate::sqlite_printf!(
                "%s%s%s",
                zOldReport,
                if !zOldReport.is_null() {
                    b"\n\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                z,
            );
            crate::src::src::malloc::sqlite3_free(zOldReport as *mut ::core::ffi::c_void);
            crate::src::src::malloc::sqlite3_free(z as *mut ::core::ffi::c_void);
            if __pCheck_ref.zReport.is_null() {
                __pCheck_ref.rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
        }
        (*pCheck).nErr += 1;
    }
}

unsafe extern "C" fn rtreeCheckReset(
    pCheck: *mut RtreeCheck,
    pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
) {
    let rc: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_reset(pStmt);
    if (*pCheck).rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        (*pCheck).rc = rc;
    }
}

unsafe extern "C" fn rtreeCheckGetNode(
    pCheck: *mut RtreeCheck,
    iNode: I64_0,
    pnNode: *mut ::core::ffi::c_int,
) -> *mut U8_0 {
    let mut pRet: *mut U8_0 = ::core::ptr::null_mut::<U8_0>();
    let __pCheck_ref = { &mut *pCheck };
    if __pCheck_ref.rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && __pCheck_ref.pGetNode.is_null()
    {
        __pCheck_ref.pGetNode = rtreeCheckPrepare(
            pCheck,
            b"SELECT data FROM %Q.'%q_node' WHERE nodeno=?\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                crate::src::src::printf::PrintfArg::Str(
                    __pCheck_ref.zDb as *mut ::core::ffi::c_char,
                ),
                crate::src::src::printf::PrintfArg::Str(
                    __pCheck_ref.zTab as *mut ::core::ffi::c_char,
                ),
            ],
        );
    }
    if __pCheck_ref.rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_bind_int64(
            __pCheck_ref.pGetNode,
            1 as ::core::ffi::c_int,
            iNode as crate::src::headers::sqlite3_h::Sqlite3Int64,
        );
        if crate::src::src::vdbeapi::sqlite3_step(__pCheck_ref.pGetNode)
            == crate::src::headers::sqlite3_h::SQLITE_ROW
        {
            let nNode: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_column_bytes(
                __pCheck_ref.pGetNode,
                0 as ::core::ffi::c_int,
            );
            let pNode: *const U8_0 = crate::src::src::vdbeapi::sqlite3_column_blob(
                __pCheck_ref.pGetNode,
                0 as ::core::ffi::c_int,
            ) as *const U8_0;
            pRet = crate::src::src::malloc::sqlite3_malloc64(
                nNode as crate::src::headers::sqlite3_h::Sqlite3Uint64,
            ) as *mut U8_0;
            if pRet.is_null() {
                __pCheck_ref.rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            } else {
                ::core::ptr::copy_nonoverlapping(
                    pNode as *const u8,
                    pRet as *mut u8,
                    nNode as usize,
                );
                *pnNode = nNode;
            }
        }
        rtreeCheckReset(pCheck, __pCheck_ref.pGetNode);
        if __pCheck_ref.rc == crate::src::headers::sqlite3_h::SQLITE_OK && pRet.is_null() {
            rtreeCheckAppendMsg(
                pCheck,
                b"Node %lld missing from database\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Int(iNode)],
            );
        }
    }
    pRet
}

unsafe extern "C" fn rtreeCheckMapping(
    pCheck: *mut RtreeCheck,
    bLeaf: ::core::ffi::c_int,
    iKey: I64_0,
    iVal: I64_0,
) {
    
    
    let azSql: [*const ::core::ffi::c_char; 2] = [
        b"SELECT parentnode FROM %Q.'%q_parent' WHERE nodeno=?1\0" as *const u8
            as *const ::core::ffi::c_char,
        b"SELECT nodeno FROM %Q.'%q_rowid' WHERE rowid=?1\0" as *const u8
            as *const ::core::ffi::c_char,
    ];
    let __pCheck_ref = { &mut *pCheck };
    if __pCheck_ref.aCheckMapping[bLeaf as usize].is_null() {
        __pCheck_ref.aCheckMapping[bLeaf as usize] = rtreeCheckPrepare(
            pCheck,
            azSql[bLeaf as usize],
            &[
                crate::src::src::printf::PrintfArg::Str(
                    __pCheck_ref.zDb as *mut ::core::ffi::c_char,
                ),
                crate::src::src::printf::PrintfArg::Str(
                    __pCheck_ref.zTab as *mut ::core::ffi::c_char,
                ),
            ],
        );
    }
    if __pCheck_ref.rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return;
    }
    let pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt = __pCheck_ref.aCheckMapping[bLeaf as usize];
    crate::src::src::vdbeapi::sqlite3_bind_int64(
        pStmt,
        1 as ::core::ffi::c_int,
        iKey as crate::src::headers::sqlite3_h::Sqlite3Int64,
    );
    let rc: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_step(pStmt);
    if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
        rtreeCheckAppendMsg(
            pCheck,
            b"Mapping (%lld -> %lld) missing from %s table\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                crate::src::src::printf::PrintfArg::Int(iKey),
                crate::src::src::printf::PrintfArg::Int(iVal),
                crate::src::src::printf::PrintfArg::Str(
                    (if bLeaf != 0 {
                        b"%_rowid\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"%_parent\0" as *const u8 as *const ::core::ffi::c_char
                    }) as *mut ::core::ffi::c_char,
                ),
            ],
        );
    } else if rc == crate::src::headers::sqlite3_h::SQLITE_ROW {
        let ii: I64_0 =
            crate::src::src::vdbeapi::sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int) as I64_0;
        if ii != iVal {
            rtreeCheckAppendMsg(
                pCheck,
                b"Found (%lld -> %lld) in %s table, expected (%lld -> %lld)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[
                    crate::src::src::printf::PrintfArg::Int(iKey),
                    crate::src::src::printf::PrintfArg::Int(ii as i64),
                    crate::src::src::printf::PrintfArg::Str(
                        (if bLeaf != 0 {
                            b"%_rowid\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"%_parent\0" as *const u8 as *const ::core::ffi::c_char
                        }) as *mut ::core::ffi::c_char,
                    ),
                    crate::src::src::printf::PrintfArg::Int(iKey),
                    crate::src::src::printf::PrintfArg::Int(iVal),
                ],
            );
        }
    }
    rtreeCheckReset(pCheck, pStmt);
}

unsafe extern "C" fn rtreeCheckCellCoord(
    pCheck: *mut RtreeCheck,
    iNode: I64_0,
    iCell: ::core::ffi::c_int,
    pCell: *mut U8_0,
    pParent: *mut U8_0,
) {
    let mut c1: RtreeCoord = RtreeCoord { f: 0. };
    let mut c2: RtreeCoord = RtreeCoord { f: 0. };
    let mut p1: RtreeCoord = RtreeCoord { f: 0. };
    let mut p2: RtreeCoord = RtreeCoord { f: 0. };
    let mut i: ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*pCheck).nDim {
        readCoord(
            pCell.offset((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int * i) as isize)
                as *mut U8_0,
            &raw mut c1,
        );
        readCoord(
            pCell.offset(
                (4 as ::core::ffi::c_int * (2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int))
                    as isize,
            ) as *mut U8_0,
            &raw mut c2,
        );
        if if (*pCheck).bInt != 0 {
            (c1.i > c2.i) as ::core::ffi::c_int
        } else {
            (c1.f > c2.f) as ::core::ffi::c_int
        } != 0
        {
            rtreeCheckAppendMsg(
                pCheck,
                b"Dimension %d of cell %d on node %lld is corrupt\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[
                    crate::src::src::printf::PrintfArg::Int(i as i64),
                    crate::src::src::printf::PrintfArg::Int(iCell as i64),
                    crate::src::src::printf::PrintfArg::Int(iNode),
                ],
            );
        }
        if !pParent.is_null() {
            readCoord(
                pParent.offset((4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int * i) as isize)
                    as *mut U8_0,
                &raw mut p1,
            );
            readCoord(
                pParent.offset(
                    (4 as ::core::ffi::c_int
                        * (2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int))
                        as isize,
                ) as *mut U8_0,
                &raw mut p2,
            );
            if (if (*pCheck).bInt != 0 {
                (c1.i < p1.i) as ::core::ffi::c_int
            } else {
                (c1.f < p1.f) as ::core::ffi::c_int
            }) != 0
                || (if (*pCheck).bInt != 0 {
                    (c2.i > p2.i) as ::core::ffi::c_int
                } else {
                    (c2.f > p2.f) as ::core::ffi::c_int
                }) != 0
            {
                rtreeCheckAppendMsg(
                    pCheck,
                    b"Dimension %d of cell %d on node %lld is corrupt relative to parent\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::src::src::printf::PrintfArg::Int(i as i64),
                        crate::src::src::printf::PrintfArg::Int(iCell as i64),
                        crate::src::src::printf::PrintfArg::Int(iNode),
                    ],
                );
            }
        }
        i += 1;
    }
}

unsafe extern "C" fn rtreeCheckNode(
    pCheck: *mut RtreeCheck,
    mut iDepth: ::core::ffi::c_int,
    aParent: *mut U8_0,
    iNode: I64_0,
) {
    
    let mut nNode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let aNode: *mut U8_0 = rtreeCheckGetNode(pCheck, iNode, &raw mut nNode);
    if !aNode.is_null() {
        if nNode < 4 as ::core::ffi::c_int {
            rtreeCheckAppendMsg(
                pCheck,
                b"Node %lld is too small (%d bytes)\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::src::src::printf::PrintfArg::Int(iNode),
                    crate::src::src::printf::PrintfArg::Int(nNode as i64),
                ],
            );
        } else {
            
            let mut i: ::core::ffi::c_int;
            if aParent.is_null() {
                iDepth = readInt16(aNode);
                if iDepth > RTREE_MAX_DEPTH {
                    rtreeCheckAppendMsg(
                        pCheck,
                        b"Rtree depth out of range (%d)\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[crate::src::src::printf::PrintfArg::Int(iDepth as i64)],
                    );
                    crate::src::src::malloc::sqlite3_free(aNode as *mut ::core::ffi::c_void);
                    return;
                }
            }
            let nCell: ::core::ffi::c_int = readInt16(aNode.offset(2_isize) as *mut U8_0);
            if 4 as ::core::ffi::c_int
                + nCell
                    * (8 as ::core::ffi::c_int
                        + (*pCheck).nDim * 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                > nNode
            {
                rtreeCheckAppendMsg(
                    pCheck,
                    b"Node %lld is too small for cell count of %d (%d bytes)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[
                        crate::src::src::printf::PrintfArg::Int(iNode),
                        crate::src::src::printf::PrintfArg::Int(nCell as i64),
                        crate::src::src::printf::PrintfArg::Int(nNode as i64),
                    ],
                );
            } else {
                i = 0 as ::core::ffi::c_int;
                while i < nCell {
                    let pCell: *mut U8_0 = aNode.offset(
                        (4 as ::core::ffi::c_int
                            + i * (8 as ::core::ffi::c_int
                                + (*pCheck).nDim
                                    * 2 as ::core::ffi::c_int
                                    * 4 as ::core::ffi::c_int)) as isize,
                    ) as *mut U8_0;
                    let iVal: I64_0 = readInt64(pCell);
                    rtreeCheckCellCoord(
                        pCheck,
                        iNode,
                        i,
                        pCell.offset(8_isize) as *mut U8_0,
                        aParent,
                    );
                    if iDepth > 0 as ::core::ffi::c_int {
                        rtreeCheckMapping(pCheck, 0 as ::core::ffi::c_int, iVal, iNode);
                        rtreeCheckNode(
                            pCheck,
                            iDepth - 1 as ::core::ffi::c_int,
                            pCell.offset(8_isize) as *mut U8_0,
                            iVal,
                        );
                        (*pCheck).nNonLeaf += 1;
                    } else {
                        rtreeCheckMapping(pCheck, 1 as ::core::ffi::c_int, iVal, iNode);
                        (*pCheck).nLeaf += 1;
                    }
                    i += 1;
                }
            }
        }
        crate::src::src::malloc::sqlite3_free(aNode as *mut ::core::ffi::c_void);
    }
}

unsafe extern "C" fn rtreeCheckCount(
    pCheck: *mut RtreeCheck,
    zTbl: *const ::core::ffi::c_char,
    nExpect: I64_0,
) {
    if (*pCheck).rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        
        let pCount: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt = rtreeCheckPrepare(
            pCheck,
            b"SELECT count(*) FROM %Q.'%q%s'\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::src::src::printf::PrintfArg::Str((*pCheck).zDb as *mut ::core::ffi::c_char),
                crate::src::src::printf::PrintfArg::Str((*pCheck).zTab as *mut ::core::ffi::c_char),
                crate::src::src::printf::PrintfArg::Str(zTbl as *mut ::core::ffi::c_char),
            ],
        );
        if !pCount.is_null() {
            if crate::src::src::vdbeapi::sqlite3_step(pCount)
                == crate::src::headers::sqlite3_h::SQLITE_ROW
            {
                let nActual: I64_0 =
                    crate::src::src::vdbeapi::sqlite3_column_int64(pCount, 0 as ::core::ffi::c_int)
                        as I64_0;
                if nActual != nExpect {
                    rtreeCheckAppendMsg(
                        pCheck,
                        b"Wrong number of entries in %%%s table - expected %lld, actual %lld\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        &[
                            crate::src::src::printf::PrintfArg::Str(
                                zTbl as *mut ::core::ffi::c_char,
                            ),
                            crate::src::src::printf::PrintfArg::Int(nExpect),
                            crate::src::src::printf::PrintfArg::Int(nActual as i64),
                        ],
                    );
                }
            }
            (*pCheck).rc = crate::src::src::vdbeapi::sqlite3_finalize(pCount);
        }
    }
}

unsafe extern "C" fn rtreeCheckTable(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zDb: *const ::core::ffi::c_char,
    zTab: *const ::core::ffi::c_char,
    pzReport: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut check: RtreeCheck = { ::core::mem::zeroed() };
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt;
    let mut nAux: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    check.db = db;
    check.zDb = zDb;
    check.zTab = zTab;
    pStmt = rtreeCheckPrepare(
        &raw mut check,
        b"SELECT * FROM %Q.'%q_rowid'\0" as *const u8 as *const ::core::ffi::c_char,
        &[
            crate::src::src::printf::PrintfArg::Str(zDb as *mut ::core::ffi::c_char),
            crate::src::src::printf::PrintfArg::Str(zTab as *mut ::core::ffi::c_char),
        ],
    );
    if !pStmt.is_null() {
        nAux = crate::src::src::vdbeapi::sqlite3_column_count(pStmt) - 2 as ::core::ffi::c_int;
        crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
    } else if check.rc != crate::src::headers::sqlite3_h::SQLITE_NOMEM {
        check.rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    pStmt = rtreeCheckPrepare(
        &raw mut check,
        b"SELECT * FROM %Q.%Q\0" as *const u8 as *const ::core::ffi::c_char,
        &[
            crate::src::src::printf::PrintfArg::Str(zDb as *mut ::core::ffi::c_char),
            crate::src::src::printf::PrintfArg::Str(zTab as *mut ::core::ffi::c_char),
        ],
    );
    if !pStmt.is_null() {
        
        check.nDim = (crate::src::src::vdbeapi::sqlite3_column_count(pStmt)
            - 1 as ::core::ffi::c_int
            - nAux)
            / 2 as ::core::ffi::c_int;
        if check.nDim < 1 as ::core::ffi::c_int {
            rtreeCheckAppendMsg(
                &raw mut check,
                b"Schema corrupt or not an rtree\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        } else if crate::src::headers::sqlite3_h::SQLITE_ROW
            == crate::src::src::vdbeapi::sqlite3_step(pStmt)
        {
            check.bInt =
                (crate::src::src::vdbeapi::sqlite3_column_type(pStmt, 1 as ::core::ffi::c_int)
                    == crate::src::headers::sqlite3_h::SQLITE_INTEGER)
                    as ::core::ffi::c_int;
        }
        let rc: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
        if rc != crate::src::headers::sqlite3_h::SQLITE_CORRUPT {
            check.rc = rc;
        }
    }
    if check.nDim >= 1 as ::core::ffi::c_int {
        if check.rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rtreeCheckNode(
                &raw mut check,
                0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<U8_0>(),
                1 as I64_0,
            );
        }
        rtreeCheckCount(
            &raw mut check,
            b"_rowid\0" as *const u8 as *const ::core::ffi::c_char,
            check.nLeaf as I64_0,
        );
        rtreeCheckCount(
            &raw mut check,
            b"_parent\0" as *const u8 as *const ::core::ffi::c_char,
            check.nNonLeaf as I64_0,
        );
    }
    crate::src::src::vdbeapi::sqlite3_finalize(check.pGetNode);
    crate::src::src::vdbeapi::sqlite3_finalize(
        check.aCheckMapping[0 as ::core::ffi::c_int as usize],
    );
    crate::src::src::vdbeapi::sqlite3_finalize(
        check.aCheckMapping[1 as ::core::ffi::c_int as usize],
    );
    *pzReport = check.zReport;
    check.rc
}

unsafe extern "C" fn rtreeIntegrity(
    pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    mut _zSchema: *const ::core::ffi::c_char,
    mut _zName: *const ::core::ffi::c_char,
    mut _isQuick: ::core::ffi::c_int,
    pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let pRtree: *mut Rtree = pVtab as *mut Rtree;
    let mut rc: ::core::ffi::c_int;
    let __pRtree_ref = { &*pRtree };
    rc = rtreeCheckTable(__pRtree_ref.db, __pRtree_ref.zDb, __pRtree_ref.zName, pzErr);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !(*pzErr).is_null() {
        let zOldErr: *mut ::core::ffi::c_char = *pzErr;
        *pzErr = crate::sqlite_printf!(
            "In RTree %s.%s:\n%s",
            __pRtree_ref.zDb,
            __pRtree_ref.zName,
            zOldErr
        );
        crate::src::src::malloc::sqlite3_free(zOldErr as *mut ::core::ffi::c_void);
        if (*pzErr).is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    }
    rc
}

unsafe extern "C" fn rtreecheck(
    ctx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    nArg: ::core::ffi::c_int,
    apArg: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    if nArg != 1 as ::core::ffi::c_int && nArg != 2 as ::core::ffi::c_int {
        crate::src::src::vdbeapi::sqlite3_result_error(
            ctx,
            b"wrong number of arguments to function rtreecheck()\0" as *const u8
                as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int),
        );
    } else {
        
        let mut zReport: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut zDb: *const ::core::ffi::c_char =
            crate::src::src::vdbeapi::sqlite3_value_text(*apArg.offset(0_isize))
                as *const ::core::ffi::c_char;
        let zTab: *const ::core::ffi::c_char;
        if nArg == 1 as ::core::ffi::c_int {
            zTab = zDb;
            zDb = b"main\0" as *const u8 as *const ::core::ffi::c_char;
        } else {
            zTab = crate::src::src::vdbeapi::sqlite3_value_text(*apArg.offset(1_isize))
                as *const ::core::ffi::c_char;
        }
        let rc: ::core::ffi::c_int = rtreeCheckTable(
            crate::src::src::vdbeapi::sqlite3_context_db_handle(ctx),
            zDb,
            zTab,
            &raw mut zReport,
        );
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::vdbeapi::sqlite3_result_text(
                ctx,
                if !zReport.is_null() {
                    zReport as *const ::core::ffi::c_char
                } else {
                    b"ok\0" as *const u8 as *const ::core::ffi::c_char
                },
                -(1 as ::core::ffi::c_int),
                ::core::mem::transmute::<
                    crate::src::headers::stdlib::IntptrT,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
            );
        } else {
            crate::src::src::vdbeapi::sqlite3_result_error_code(ctx, rc);
        }
        crate::src::src::malloc::sqlite3_free(zReport as *mut ::core::ffi::c_void);
    };
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3RtreeInit(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
) -> ::core::ffi::c_int {
    let utf8: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_UTF8;
    let mut rc: ::core::ffi::c_int;
    rc = crate::src::src::main::sqlite3_create_function(
        db,
        b"rtreenode\0" as *const u8 as *const ::core::ffi::c_char,
        2 as ::core::ffi::c_int,
        utf8,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        Some(
            rtreenode
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
            b"rtreedepth\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            utf8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                rtreedepth
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
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = crate::src::src::main::sqlite3_create_function(
            db,
            b"rtreecheck\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int),
            utf8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                rtreecheck
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
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let c: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        rc = crate::src::src::vtab::sqlite3_create_module_v2(
            db,
            b"rtree\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut rtreeModule as *mut _ as *const crate::src::headers::sqlite3_h::sqlite3_module,
            c,
            None,
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let c_0: *mut ::core::ffi::c_void = RTREE_COORD_INT32 as *mut ::core::ffi::c_void;
        rc = crate::src::src::vtab::sqlite3_create_module_v2(
            db,
            b"rtree_i32\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut rtreeModule as *mut _ as *const crate::src::headers::sqlite3_h::sqlite3_module,
            c_0,
            None,
        );
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = sqlite3_geopoly_init(db);
    }
    rc
}

unsafe extern "C" fn rtreeFreeCallback(p: *mut ::core::ffi::c_void) {
    let pInfo: *mut RtreeGeomCallback = p as *mut RtreeGeomCallback;
    if (*pInfo).xDestructor.is_some() {
        (*pInfo).xDestructor.expect("non-null function pointer")((*pInfo).pContext);
    }
    crate::src::src::malloc::sqlite3_free(p);
}

unsafe extern "C" fn rtreeMatchArgFree(pArg: *mut ::core::ffi::c_void) {
    let mut i: ::core::ffi::c_int;
    let p: *mut RtreeMatchArg = pArg as *mut RtreeMatchArg;
    i = 0 as ::core::ffi::c_int;
    while i < (*p).nParam {
        crate::src::src::vdbeapi::sqlite3_value_free(*(*p).apSqlParam.offset(i as isize));
        i += 1;
    }
    crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn geomCallback(
    ctx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    nArg: ::core::ffi::c_int,
    aArg: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let pGeomCtx: *mut RtreeGeomCallback =
        crate::src::src::vdbeapi::sqlite3_user_data(ctx) as *mut RtreeGeomCallback;
    
    
    let mut memErr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let nBlob: crate::src::headers::sqlite3_h::Sqlite3Int64 = 56_usize
        .wrapping_add((nArg as usize).wrapping_mul(::core::mem::size_of::<RtreeDValue>() as usize))
        .wrapping_add((nArg as usize).wrapping_mul(::core::mem::size_of::<
            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        >() as usize)) as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let pBlob: *mut RtreeMatchArg = crate::src::src::malloc::sqlite3_malloc64(
        nBlob as crate::src::headers::sqlite3_h::Sqlite3Uint64,
    ) as *mut RtreeMatchArg;
    if pBlob.is_null() {
        crate::src::src::vdbeapi::sqlite3_result_error_nomem(ctx);
    } else {
        let mut i: ::core::ffi::c_int;
        let __pBlob_ref = { &mut *pBlob };
        __pBlob_ref.iSize = nBlob as U32_0;
        __pBlob_ref.cb = *pGeomCtx.offset(0_isize);
        __pBlob_ref.apSqlParam = (&raw mut __pBlob_ref.aParam as *mut RtreeDValue)
            .offset(nArg as isize) as *mut RtreeDValue
            as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value;
        __pBlob_ref.nParam = nArg;
        i = 0 as ::core::ffi::c_int;
        while i < nArg {
            let fresh0 = &mut *__pBlob_ref.apSqlParam.offset(i as isize);
            *fresh0 = crate::src::src::vdbeapi::sqlite3_value_dup(*aArg.offset(i as isize));
            if (*__pBlob_ref.apSqlParam.offset(i as isize)).is_null() {
                memErr = 1 as ::core::ffi::c_int;
            }
            *(&raw mut __pBlob_ref.aParam as *mut RtreeDValue).offset(i as isize) =
                crate::src::src::vdbeapi::sqlite3_value_double(*aArg.offset(i as isize))
                    as RtreeDValue;
            i += 1;
        }
        if memErr != 0 {
            crate::src::src::vdbeapi::sqlite3_result_error_nomem(ctx);
            rtreeMatchArgFree(pBlob as *mut ::core::ffi::c_void);
        } else {
            crate::src::src::vdbeapi::sqlite3_result_pointer(
                ctx,
                pBlob as *mut ::core::ffi::c_void,
                b"RtreeMatchArg\0" as *const u8 as *const ::core::ffi::c_char,
                Some(rtreeMatchArgFree as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            );
        }
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_rtree_geometry_callback(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zGeom: *const ::core::ffi::c_char,
    xGeom: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_rtree_geometry,
            ::core::ffi::c_int,
            *mut RtreeDValue,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pContext: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    
    let pGeomCtx: *mut RtreeGeomCallback = crate::src::src::malloc::sqlite3_malloc(
        ::core::mem::size_of::<RtreeGeomCallback>() as ::core::ffi::c_int
    ) as *mut RtreeGeomCallback;
    if pGeomCtx.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    (*pGeomCtx).xGeom = xGeom;
    (*pGeomCtx).xQueryFunc = None;
    (*pGeomCtx).xDestructor = None;
    (*pGeomCtx).pContext = pContext;
    crate::src::src::main::sqlite3_create_function_v2(
        db,
        zGeom,
        -(1 as ::core::ffi::c_int),
        crate::src::headers::sqlite3_h::SQLITE_ANY,
        pGeomCtx as *mut ::core::ffi::c_void,
        Some(
            geomCallback
                as unsafe extern "C" fn(
                    *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                ) -> (),
        ),
        None,
        None,
        Some(rtreeFreeCallback as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    )
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_rtree_query_callback(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zQueryFunc: *const ::core::ffi::c_char,
    xQueryFunc: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_rtree_query_info,
        ) -> ::core::ffi::c_int,
    >,
    pContext: *mut ::core::ffi::c_void,
    xDestructor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    
    let pGeomCtx: *mut RtreeGeomCallback = crate::src::src::malloc::sqlite3_malloc(
        ::core::mem::size_of::<RtreeGeomCallback>() as ::core::ffi::c_int
    ) as *mut RtreeGeomCallback;
    if pGeomCtx.is_null() {
        if xDestructor.is_some() {
            xDestructor.expect("non-null function pointer")(pContext);
        }
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    (*pGeomCtx).xGeom = None;
    (*pGeomCtx).xQueryFunc = xQueryFunc;
    (*pGeomCtx).xDestructor = xDestructor;
    (*pGeomCtx).pContext = pContext;
    crate::src::src::main::sqlite3_create_function_v2(
        db,
        zQueryFunc,
        -(1 as ::core::ffi::c_int),
        crate::src::headers::sqlite3_h::SQLITE_ANY,
        pGeomCtx as *mut ::core::ffi::c_void,
        Some(
            geomCallback
                as unsafe extern "C" fn(
                    *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                ) -> (),
        ),
        None,
        None,
        Some(rtreeFreeCallback as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    )
}
