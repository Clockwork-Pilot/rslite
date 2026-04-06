use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_api_routines;
    pub type sqlite3_value;
    pub type sqlite3_context;
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
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Prng {
    pub x: ::core::ffi::c_uint,
    pub y: ::core::ffi::c_uint,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DETERMINISTIC: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SQLITE_INNOCUOUS: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;
unsafe extern "C" fn prngSeed(mut p: *mut Prng, mut iSeed: ::core::ffi::c_uint) {
    unsafe {
        (*p).x = iSeed | 1 as ::core::ffi::c_uint;
        (*p).y = iSeed;
    }
}
unsafe extern "C" fn prngInt(mut p: *mut Prng) -> ::core::ffi::c_uint {
    unsafe {
        (*p).x = (*p).x >> 1 as ::core::ffi::c_int
            ^ (1 as ::core::ffi::c_uint)
                .wrapping_add(!((*p).x & 1 as ::core::ffi::c_uint))
                & 0xd0000001 as ::core::ffi::c_uint;
        (*p).y = (*p)
            .y
            .wrapping_mul(1103515245 as ::core::ffi::c_int as ::core::ffi::c_uint)
            .wrapping_add(12345 as ::core::ffi::c_uint);
        return (*p).x ^ (*p).y;
    }
}
static mut azJsonAtoms: [*mut ::core::ffi::c_char; 84] = [
    b"0\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"0\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"1\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"1\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"-1\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"-1\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"2\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"+2\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"3DDDD\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"3DDDD\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"2.5DD\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"2.5DD\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"0.75\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b".75\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"-4.0e2\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"-4.e2\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"5.0e-3\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"+5e-3\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"6.DDe+0DD\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"6.DDe+0DD\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"0\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"0x0\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"512\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"0x200\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"256\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"+0x100\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"-2748\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"-0xabc\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"true\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"true\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"false\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"false\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"null\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"null\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"9.0e999\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Infinity\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"-9.0e999\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"-Infinity\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"9.0e999\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"+Infinity\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"null\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"NaN\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"-0.0005DD\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"-0.0005DD\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"4.35e-3\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"+4.35e-3\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"gem\\\"hay\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"\"gem\\\"hay\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"\"icy'joy\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"'icy\\'joy'\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"keylog\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"key\\\nlog\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"\"mix\\\\\\tnet\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"\"mix\\\\\\tnet\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"\"oat\\r\\n\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"\"oat\\r\\n\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"\"\\fpan\\b\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"\"\\fpan\\b\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"{}\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"{}\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"[]\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"[]\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"[]\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"[/*empty*/]\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"{}\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"{//empty\n}\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"ask\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"ask\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"bag\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"bag\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"can\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"can\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"day\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"day\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"end\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"'end'\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"fly\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"fly\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"\\u00XX\\u00XX\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"\"\\xXX\\xXX\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"\"y\\uXXXXz\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"\"y\\uXXXXz\"\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"\"\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"\"\"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
];
static mut azJsonTemplate: [*mut ::core::ffi::c_char; 28] = [
    b"{\"a\":%,\"b\":%,\"cDD\":%}\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"{a:%,b:%,cDD:%}\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"{\"a\":%,\"b\":%,\"c\":%,\"d\":%,\"e\":%}\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"{a:%,b:%,c:%,d:%,e:%}\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"{\"a\":%,\"b\":%,\"c\":%,\"d\":%,\"\":%}\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"{a:%,b:%,c:%,d:%,'':%}\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"{\"d\":%}\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"{d:%}\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"{\"eeee\":%, \"ffff\":%}\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"{eeee:% /*and*/, ffff:%}\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"{\"$g\":%,\"_h_\":%,\"a b c d\":%}\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"{$g:%,_h_:%,\"a b c d\":%}\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"{\"x\":%,\n  \"y\":%}\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"{\"x\":%,\n  \"y\":%}\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"{\"\\u00XX\":%,\"\\uXXXX\":%}\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"{\"\\xXX\":%,\"\\uXXXX\":%}\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"{\"Z\":%}\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"{Z:%,}\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"[%]\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"[%,]\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"[%,%]\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"[%,%]\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"[%,%,%]\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"[%,%,%,]\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"[%,%,%,%]\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"[%,%,%,%]\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"[%,%,%,%,%]\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"[%,%,%,%,%]\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
];
pub const STRSZ: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
unsafe extern "C" fn jsonExpand(
    mut zSrc: *const ::core::ffi::c_char,
    mut zDest: *mut ::core::ffi::c_char,
    mut p: *mut Prng,
    mut eType: ::core::ffi::c_int,
    mut r: ::core::ffi::c_uint,
) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut j: ::core::ffi::c_uint = 0;
        let mut k: ::core::ffi::c_uint = 0;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zX: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut n: size_t = 0;
        let mut zBuf: [::core::ffi::c_char; 200] = [0; 200];
        j = 0 as ::core::ffi::c_uint;
        if zSrc.is_null() {
            zSrc = b"%\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if strlen(zSrc) >= (STRSZ / 10 as ::core::ffi::c_int) as size_t {
            r = 0 as ::core::ffi::c_uint;
        }
        i = 0 as ::core::ffi::c_uint;
        while *zSrc.offset(i as isize) != 0 {
            if *zSrc.offset(i as isize) as ::core::ffi::c_int != '%' as i32 {
                if j < STRSZ as ::core::ffi::c_uint {
                    let c2rust_fresh0 = j;
                    j = j.wrapping_add(1);
                    *zDest.offset(c2rust_fresh0 as isize) = *zSrc.offset(i as isize);
                }
            } else {
                if r == 0 as ::core::ffi::c_uint
                    || r < 1000 as ::core::ffi::c_uint
                        && prngInt(p).wrapping_rem(1000 as ::core::ffi::c_uint) <= r
                {
                    k = (prngInt(p) as usize)
                        .wrapping_rem(
                            (::core::mem::size_of::<[*mut ::core::ffi::c_char; 84]>()
                                as usize)
                                .wrapping_div(
                                    ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize,
                                )
                                .wrapping_div(2 as usize),
                        ) as ::core::ffi::c_uint;
                    k = k
                        .wrapping_mul(2 as ::core::ffi::c_uint)
                        .wrapping_add(eType as ::core::ffi::c_uint);
                    z = azJsonAtoms[k as usize];
                } else {
                    k = (prngInt(p) as usize)
                        .wrapping_rem(
                            (::core::mem::size_of::<[*mut ::core::ffi::c_char; 28]>()
                                as usize)
                                .wrapping_div(
                                    ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize,
                                )
                                .wrapping_div(2 as usize),
                        ) as ::core::ffi::c_uint;
                    k = k
                        .wrapping_mul(2 as ::core::ffi::c_uint)
                        .wrapping_add(eType as ::core::ffi::c_uint);
                    z = azJsonTemplate[k as usize];
                }
                n = strlen(z);
                zX = strstr(z, b"XX\0".as_ptr() as *const ::core::ffi::c_char);
                if !zX.is_null() {
                    let mut y: ::core::ffi::c_uint = prngInt(p);
                    if y & 0xff as ::core::ffi::c_uint
                        == y >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint
                    {
                        y = y.wrapping_add(0x100 as ::core::ffi::c_uint);
                    }
                    while y & 0xff as ::core::ffi::c_uint
                        == y >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint
                        || y >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint
                            == y >> 16 as ::core::ffi::c_int
                                & 0xff as ::core::ffi::c_uint
                    {
                        y = y
                            .wrapping_add(
                                0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            );
                    }
                    memcpy(
                        &raw mut zBuf as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        z as *const ::core::ffi::c_void,
                        n.wrapping_add(1 as size_t),
                    );
                    z = &raw mut zBuf as *mut ::core::ffi::c_char;
                    zX = strstr(z, b"XX\0".as_ptr() as *const ::core::ffi::c_char);
                    while !zX.is_null() {
                        *zX.offset(0 as ::core::ffi::c_int as isize) = ::core::mem::transmute::<
                            [u8; 17],
                            [::core::ffi::c_char; 17],
                        >(
                            *b"0123456789abcdef\0",
                        )[y.wrapping_rem(16 as ::core::ffi::c_uint) as usize];
                        y = y.wrapping_div(16 as ::core::ffi::c_uint);
                        *zX.offset(1 as ::core::ffi::c_int as isize) = ::core::mem::transmute::<
                            [u8; 17],
                            [::core::ffi::c_char; 17],
                        >(
                            *b"0123456789abcdef\0",
                        )[y.wrapping_rem(16 as ::core::ffi::c_uint) as usize];
                        y = y.wrapping_div(16 as ::core::ffi::c_uint);
                        zX = strstr(zX, b"XX\0".as_ptr() as *const ::core::ffi::c_char);
                    }
                } else {
                    zX = strstr(z, b"DD\0".as_ptr() as *const ::core::ffi::c_char);
                    if !zX.is_null() {
                        let mut y_0: ::core::ffi::c_uint = prngInt(p);
                        memcpy(
                            &raw mut zBuf as *mut ::core::ffi::c_char
                                as *mut ::core::ffi::c_void,
                            z as *const ::core::ffi::c_void,
                            n.wrapping_add(1 as size_t),
                        );
                        z = &raw mut zBuf as *mut ::core::ffi::c_char;
                        zX = strstr(z, b"DD\0".as_ptr() as *const ::core::ffi::c_char);
                        while !zX.is_null() {
                            *zX.offset(0 as ::core::ffi::c_int as isize) = ::core::mem::transmute::<
                                [u8; 11],
                                [::core::ffi::c_char; 11],
                            >(
                                *b"0123456789\0",
                            )[y_0.wrapping_rem(10 as ::core::ffi::c_uint) as usize];
                            y_0 = y_0.wrapping_div(10 as ::core::ffi::c_uint);
                            *zX.offset(1 as ::core::ffi::c_int as isize) = ::core::mem::transmute::<
                                [u8; 11],
                                [::core::ffi::c_char; 11],
                            >(
                                *b"0123456789\0",
                            )[y_0.wrapping_rem(10 as ::core::ffi::c_uint) as usize];
                            y_0 = y_0.wrapping_div(10 as ::core::ffi::c_uint);
                            zX = strstr(
                                zX,
                                b"DD\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    }
                }
                if (j as size_t).wrapping_add(n) < STRSZ as size_t {
                    memcpy(
                        zDest.offset(j as isize) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        z as *const ::core::ffi::c_void,
                        n,
                    );
                    j = j.wrapping_add(n as ::core::ffi::c_int as ::core::ffi::c_uint);
                }
            }
            i = i.wrapping_add(1);
        }
        *zDest.offset((STRSZ - 1 as ::core::ffi::c_int) as isize) = 0
            as ::core::ffi::c_char;
        if j < STRSZ as ::core::ffi::c_uint {
            *zDest.offset(j as isize) = 0 as ::core::ffi::c_char;
        }
    }
}
unsafe extern "C" fn randJsonFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut iSeed: ::core::ffi::c_uint = 0;
        let mut eType: ::core::ffi::c_int = *(sqlite3_user_data(context)
            as *mut ::core::ffi::c_int);
        let mut prng: Prng = Prng { x: 0, y: 0 };
        let mut z1: [::core::ffi::c_char; 10001] = [0; 10001];
        let mut z2: [::core::ffi::c_char; 10001] = [0; 10001];
        iSeed = sqlite3_value_int(*argv.offset(0 as ::core::ffi::c_int as isize))
            as ::core::ffi::c_uint;
        prngSeed(&raw mut prng, iSeed);
        jsonExpand(
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw mut z2 as *mut ::core::ffi::c_char,
            &raw mut prng,
            eType,
            1000 as ::core::ffi::c_uint,
        );
        jsonExpand(
            &raw mut z2 as *mut ::core::ffi::c_char,
            &raw mut z1 as *mut ::core::ffi::c_char,
            &raw mut prng,
            eType,
            1000 as ::core::ffi::c_uint,
        );
        jsonExpand(
            &raw mut z1 as *mut ::core::ffi::c_char,
            &raw mut z2 as *mut ::core::ffi::c_char,
            &raw mut prng,
            eType,
            100 as ::core::ffi::c_uint,
        );
        jsonExpand(
            &raw mut z2 as *mut ::core::ffi::c_char,
            &raw mut z1 as *mut ::core::ffi::c_char,
            &raw mut prng,
            eType,
            0 as ::core::ffi::c_uint,
        );
        sqlite3_result_text(
            context,
            &raw mut z1 as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_randomjson_init(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        static mut cOne: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        static mut cZero: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        rc = sqlite3_create_function(
            db,
            b"random_json\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            SQLITE_UTF8 | SQLITE_INNOCUOUS | SQLITE_DETERMINISTIC,
            &raw mut cZero as *mut ::core::ffi::c_void,
            Some(
                randJsonFunc
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
                b"random_json5\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8 | SQLITE_INNOCUOUS | SQLITE_DETERMINISTIC,
                &raw mut cOne as *mut ::core::ffi::c_void,
                Some(
                    randJsonFunc
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
}
