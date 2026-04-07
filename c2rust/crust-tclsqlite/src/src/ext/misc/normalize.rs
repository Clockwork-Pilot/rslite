unsafe extern "C" {
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
pub type size_t = usize;
pub const CC_X: ::core::ffi::c_int = 0;
pub const CC_KYWD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CC_ID: ::core::ffi::c_int = 2;
pub const CC_DIGIT: ::core::ffi::c_int = 3;
pub const CC_DOLLAR: ::core::ffi::c_int = 4;
pub const CC_VARALPHA: ::core::ffi::c_int = 5;
pub const CC_VARNUM: ::core::ffi::c_int = 6;
pub const CC_SPACE: ::core::ffi::c_int = 7;
pub const CC_QUOTE: ::core::ffi::c_int = 8;
pub const CC_QUOTE2: ::core::ffi::c_int = 9;
pub const CC_PIPE: ::core::ffi::c_int = 10;
pub const CC_MINUS: ::core::ffi::c_int = 11;
pub const CC_LT: ::core::ffi::c_int = 12;
pub const CC_GT: ::core::ffi::c_int = 13;
pub const CC_EQ: ::core::ffi::c_int = 14;
pub const CC_BANG: ::core::ffi::c_int = 15;
pub const CC_SLASH: ::core::ffi::c_int = 16;
pub const CC_LP: ::core::ffi::c_int = 17;
pub const CC_RP: ::core::ffi::c_int = 18;
pub const CC_SEMI: ::core::ffi::c_int = 19;
pub const CC_PLUS: ::core::ffi::c_int = 20;
pub const CC_STAR: ::core::ffi::c_int = 21;
pub const CC_PERCENT: ::core::ffi::c_int = 22;
pub const CC_COMMA: ::core::ffi::c_int = 23;
pub const CC_AND: ::core::ffi::c_int = 24;
pub const CC_TILDA: ::core::ffi::c_int = 25;
pub const CC_DOT: ::core::ffi::c_int = 26;
static mut aiClass: [::core::ffi::c_uchar; 256] = [
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    26 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static mut sqlite3UpperToLower: [::core::ffi::c_uchar; 256] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    26 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    34 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    36 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    37 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    38 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    39 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    41 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    42 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    43 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    44 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    47 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    48 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    51 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    52 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    54 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    56 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    58 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    59 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    63 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    64 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    97 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    98 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    100 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    102 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    103 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    104 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    105 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    106 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    107 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    108 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    109 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    110 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    111 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    112 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    113 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    114 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    115 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    116 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    117 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    118 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    119 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    120 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    121 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    122 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    91 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    92 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    93 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    94 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    95 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    96 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    97 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    98 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    99 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    100 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    101 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    102 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    103 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    104 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    105 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    106 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    107 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    108 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    109 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    110 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    111 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    112 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    113 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    114 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    115 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    116 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    117 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    118 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    119 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    120 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    121 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    122 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    123 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    124 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    125 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    126 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    127 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    128 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    129 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    130 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    131 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    132 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    133 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    134 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    135 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    136 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    137 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    138 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    139 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    140 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    141 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    142 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    143 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    144 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    145 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    146 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    147 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    148 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    149 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    150 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    151 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    152 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    153 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    154 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    155 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    156 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    157 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    158 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    159 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    160 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    161 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    162 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    163 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    164 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    165 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    166 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    167 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    168 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    169 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    170 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    171 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    172 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    173 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    174 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    175 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    176 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    177 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    178 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    179 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    180 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    181 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    182 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    183 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    184 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    185 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    186 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    187 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    188 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    189 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    190 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    191 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    192 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    193 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    194 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    195 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    196 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    197 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    198 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    199 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    200 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    201 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    202 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    203 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    204 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    205 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    206 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    207 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    208 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    209 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    210 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    211 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    212 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    213 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    214 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    215 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    216 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    217 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    218 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    219 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    220 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    221 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    222 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    223 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    224 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    225 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    226 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    227 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    228 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    229 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    230 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    231 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    232 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    233 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    234 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    235 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    236 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    237 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    238 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    239 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    240 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    241 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    242 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    243 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    244 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    245 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    246 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    247 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    248 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    249 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    250 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    251 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    252 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    253 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    254 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    255 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static mut sqlite3CtypeMap: [::core::ffi::c_uchar; 256] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
pub const TK_SPACE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TK_NAME: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TK_LITERAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TK_PUNCT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const TK_ERROR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TK_MINUS: ::core::ffi::c_int = TK_PUNCT;
pub const TK_LP: ::core::ffi::c_int = TK_PUNCT;
pub const TK_RP: ::core::ffi::c_int = TK_PUNCT;
pub const TK_SEMI: ::core::ffi::c_int = TK_PUNCT;
pub const TK_PLUS: ::core::ffi::c_int = TK_PUNCT;
pub const TK_STAR: ::core::ffi::c_int = TK_PUNCT;
pub const TK_SLASH: ::core::ffi::c_int = TK_PUNCT;
pub const TK_REM: ::core::ffi::c_int = TK_PUNCT;
pub const TK_EQ: ::core::ffi::c_int = TK_PUNCT;
pub const TK_LE: ::core::ffi::c_int = TK_PUNCT;
pub const TK_NE: ::core::ffi::c_int = TK_PUNCT;
pub const TK_LSHIFT: ::core::ffi::c_int = TK_PUNCT;
pub const TK_LT: ::core::ffi::c_int = TK_PUNCT;
pub const TK_RSHIFT: ::core::ffi::c_int = TK_PUNCT;
pub const TK_GT: ::core::ffi::c_int = TK_PUNCT;
pub const TK_GE: ::core::ffi::c_int = TK_PUNCT;
pub const TK_BITOR: ::core::ffi::c_int = TK_PUNCT;
pub const TK_CONCAT: ::core::ffi::c_int = TK_PUNCT;
pub const TK_COMMA: ::core::ffi::c_int = TK_PUNCT;
pub const TK_BITAND: ::core::ffi::c_int = TK_PUNCT;
pub const TK_BITNOT: ::core::ffi::c_int = TK_PUNCT;
pub const TK_STRING: ::core::ffi::c_int = TK_LITERAL;
pub const TK_ID: ::core::ffi::c_int = TK_NAME;
pub const TK_ILLEGAL: ::core::ffi::c_int = TK_ERROR;
pub const TK_DOT: ::core::ffi::c_int = TK_PUNCT;
pub const TK_INTEGER: ::core::ffi::c_int = TK_LITERAL;
pub const TK_FLOAT: ::core::ffi::c_int = TK_LITERAL;
pub const TK_VARIABLE: ::core::ffi::c_int = TK_LITERAL;
pub const TK_BLOB: ::core::ffi::c_int = TK_LITERAL;
unsafe extern "C" fn sqlite3GetToken(
    mut z: *const ::core::ffi::c_uchar,
    mut tokenType: *mut ::core::ffi::c_int,
) -> sqlite3_int64 {
    unsafe {
        let mut i: sqlite3_int64 = 0;
        let mut c: ::core::ffi::c_int = 0;
        let mut c2rust_current_block_218: u64;
        match aiClass[*z as usize] as ::core::ffi::c_int {
            CC_SPACE => {
                i = 1 as sqlite3_int64;
                while sqlite3CtypeMap[*z.offset(i as isize) as usize]
                    as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0
                {
                    i += 1;
                }
                *tokenType = TK_SPACE;
                return i;
            }
            CC_MINUS => {
                if *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '-' as i32
                {
                    i = 2 as sqlite3_int64;
                    loop {
                        c = *z.offset(i as isize) as ::core::ffi::c_int;
                        if !(c != 0 as ::core::ffi::c_int && c != '\n' as i32) {
                            break;
                        }
                        i += 1;
                    }
                    *tokenType = TK_SPACE;
                    return i;
                }
                *tokenType = TK_MINUS;
                return 1 as sqlite3_int64;
            }
            CC_LP => {
                *tokenType = TK_LP;
                return 1 as sqlite3_int64;
            }
            CC_RP => {
                *tokenType = TK_RP;
                return 1 as sqlite3_int64;
            }
            CC_SEMI => {
                *tokenType = TK_SEMI;
                return 1 as sqlite3_int64;
            }
            CC_PLUS => {
                *tokenType = TK_PLUS;
                return 1 as sqlite3_int64;
            }
            CC_STAR => {
                *tokenType = TK_STAR;
                return 1 as sqlite3_int64;
            }
            CC_SLASH => {
                if *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '*' as i32
                    || *z.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                {
                    *tokenType = TK_SLASH;
                    return 1 as sqlite3_int64;
                }
                i = 3 as sqlite3_int64;
                c = *z.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                while (c != '*' as i32
                    || *z.offset(i as isize) as ::core::ffi::c_int != '/' as i32)
                    && {
                        c = *z.offset(i as isize) as ::core::ffi::c_int;
                        c != 0 as ::core::ffi::c_int
                    }
                {
                    i += 1;
                }
                if c != 0 {
                    i += 1;
                }
                *tokenType = TK_SPACE;
                return i;
            }
            CC_PERCENT => {
                *tokenType = TK_REM;
                return 1 as sqlite3_int64;
            }
            CC_EQ => {
                *tokenType = TK_EQ;
                return (1 as ::core::ffi::c_int
                    + (*z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '=' as i32) as ::core::ffi::c_int) as sqlite3_int64;
            }
            CC_LT => {
                c = *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                if c == '=' as i32 {
                    *tokenType = TK_LE;
                    return 2 as sqlite3_int64;
                } else if c == '>' as i32 {
                    *tokenType = TK_NE;
                    return 2 as sqlite3_int64;
                } else if c == '<' as i32 {
                    *tokenType = TK_LSHIFT;
                    return 2 as sqlite3_int64;
                } else {
                    *tokenType = TK_LT;
                    return 1 as sqlite3_int64;
                }
            }
            CC_GT => {
                c = *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                if c == '=' as i32 {
                    *tokenType = TK_GE;
                    return 2 as sqlite3_int64;
                } else if c == '>' as i32 {
                    *tokenType = TK_RSHIFT;
                    return 2 as sqlite3_int64;
                } else {
                    *tokenType = TK_GT;
                    return 1 as sqlite3_int64;
                }
            }
            CC_BANG => {
                if *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '=' as i32
                {
                    *tokenType = TK_ILLEGAL;
                    return 1 as sqlite3_int64;
                } else {
                    *tokenType = TK_NE;
                    return 2 as sqlite3_int64;
                }
            }
            CC_PIPE => {
                if *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '|' as i32
                {
                    *tokenType = TK_BITOR;
                    return 1 as sqlite3_int64;
                } else {
                    *tokenType = TK_CONCAT;
                    return 2 as sqlite3_int64;
                }
            }
            CC_COMMA => {
                *tokenType = TK_COMMA;
                return 1 as sqlite3_int64;
            }
            CC_AND => {
                *tokenType = TK_BITAND;
                return 1 as sqlite3_int64;
            }
            CC_TILDA => {
                *tokenType = TK_BITNOT;
                return 1 as sqlite3_int64;
            }
            CC_QUOTE => {
                let mut delim: ::core::ffi::c_int = *z
                    .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                i = 1 as sqlite3_int64;
                loop {
                    c = *z.offset(i as isize) as ::core::ffi::c_int;
                    if !(c != 0 as ::core::ffi::c_int) {
                        break;
                    }
                    if c == delim {
                        if !(*z
                            .offset(
                                (i as ::core::ffi::c_longlong
                                    + 1 as ::core::ffi::c_longlong) as isize,
                            ) as ::core::ffi::c_int == delim)
                        {
                            break;
                        }
                        i += 1;
                    }
                    i += 1;
                }
                if c == '\'' as i32 {
                    *tokenType = TK_STRING;
                    return i + 1 as sqlite3_int64;
                } else if c != 0 as ::core::ffi::c_int {
                    *tokenType = TK_ID;
                    return i + 1 as sqlite3_int64;
                } else {
                    *tokenType = TK_ILLEGAL;
                    return i;
                }
            }
            CC_DOT => {
                if sqlite3CtypeMap[*z.offset(1 as ::core::ffi::c_int as isize) as usize]
                    as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int == 0
                {
                    *tokenType = TK_DOT;
                    return 1 as sqlite3_int64;
                }
                c2rust_current_block_218 = 17685150860668569253;
            }
            CC_DIGIT => {
                c2rust_current_block_218 = 17685150860668569253;
            }
            CC_QUOTE2 => {
                i = 1 as sqlite3_int64;
                c = *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                while c != ']' as i32
                    && {
                        c = *z.offset(i as isize) as ::core::ffi::c_int;
                        c != 0 as ::core::ffi::c_int
                    }
                {
                    i += 1;
                }
                *tokenType = if c == ']' as i32 { TK_ID } else { TK_ILLEGAL };
                return i;
            }
            CC_VARNUM => {
                *tokenType = TK_VARIABLE;
                i = 1 as sqlite3_int64;
                while sqlite3CtypeMap[*z.offset(i as isize) as usize]
                    as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                {
                    i += 1;
                }
                return i;
            }
            CC_DOLLAR | CC_VARALPHA => {
                let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                *tokenType = TK_VARIABLE;
                i = 1 as sqlite3_int64;
                loop {
                    c = *z.offset(i as isize) as ::core::ffi::c_int;
                    if !(c != 0 as ::core::ffi::c_int) {
                        break;
                    }
                    if sqlite3CtypeMap[c as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int & 0x46 as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                    {
                        n += 1;
                    } else if c == '(' as i32 && n > 0 as ::core::ffi::c_int {
                        loop {
                            i += 1;
                            c = *z.offset(i as isize) as ::core::ffi::c_int;
                            if !(c != 0 as ::core::ffi::c_int
                                && sqlite3CtypeMap[c as ::core::ffi::c_uchar as usize]
                                    as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int == 0
                                && c != ')' as i32)
                            {
                                break;
                            }
                        }
                        if c == ')' as i32 {
                            i += 1;
                        } else {
                            *tokenType = TK_ILLEGAL;
                        }
                        break;
                    } else {
                        if !(c == ':' as i32
                            && *z
                                .offset(
                                    (i as ::core::ffi::c_longlong
                                        + 1 as ::core::ffi::c_longlong) as isize,
                                ) as ::core::ffi::c_int == ':' as i32)
                        {
                            break;
                        }
                        i += 1;
                    }
                    i += 1;
                }
                if n == 0 as ::core::ffi::c_int {
                    *tokenType = TK_ILLEGAL;
                }
                return i;
            }
            CC_KYWD => {
                i = 1 as sqlite3_int64;
                while aiClass[*z.offset(i as isize) as usize] as ::core::ffi::c_int
                    <= CC_KYWD
                {
                    i += 1;
                }
                if sqlite3CtypeMap[*z.offset(i as isize) as usize] as ::core::ffi::c_int
                    & 0x46 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    i += 1;
                } else {
                    *tokenType = TK_ID;
                    return i;
                }
                c2rust_current_block_218 = 4127803603908737533;
            }
            CC_X => {
                if *z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\'' as i32
                {
                    *tokenType = TK_BLOB;
                    i = 2 as sqlite3_int64;
                    while sqlite3CtypeMap[*z.offset(i as isize) as usize]
                        as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
                    {
                        i += 1;
                    }
                    if *z.offset(i as isize) as ::core::ffi::c_int != '\'' as i32
                        || i as ::core::ffi::c_longlong % 2 as ::core::ffi::c_longlong
                            != 0
                    {
                        *tokenType = TK_ILLEGAL;
                        while *z.offset(i as isize) as ::core::ffi::c_int != 0
                            && *z.offset(i as isize) as ::core::ffi::c_int != '\'' as i32
                        {
                            i += 1;
                        }
                    }
                    if *z.offset(i as isize) != 0 {
                        i += 1;
                    }
                    return i;
                }
                c2rust_current_block_218 = 6880299496751257707;
            }
            CC_ID => {
                c2rust_current_block_218 = 6880299496751257707;
            }
            _ => {
                *tokenType = TK_ILLEGAL;
                return 1 as sqlite3_int64;
            }
        }
        match c2rust_current_block_218 {
            17685150860668569253 => {
                *tokenType = TK_INTEGER;
                if *z.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '0' as i32
                    && (*z.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'x' as i32
                        || *z.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == 'X' as i32)
                    && sqlite3CtypeMap[*z.offset(2 as ::core::ffi::c_int as isize)
                        as usize] as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
                {
                    i = 3 as sqlite3_int64;
                    while sqlite3CtypeMap[*z.offset(i as isize) as usize]
                        as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
                    {
                        i += 1;
                    }
                    return i;
                }
                i = 0 as sqlite3_int64;
                while sqlite3CtypeMap[*z.offset(i as isize) as usize]
                    as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                {
                    i += 1;
                }
                if *z.offset(i as isize) as ::core::ffi::c_int == '.' as i32 {
                    i += 1;
                    while sqlite3CtypeMap[*z.offset(i as isize) as usize]
                        as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                    {
                        i += 1;
                    }
                    *tokenType = TK_FLOAT;
                }
                if (*z.offset(i as isize) as ::core::ffi::c_int == 'e' as i32
                    || *z.offset(i as isize) as ::core::ffi::c_int == 'E' as i32)
                    && (sqlite3CtypeMap[*z
                        .offset(
                            (i as ::core::ffi::c_longlong + 1 as ::core::ffi::c_longlong)
                                as isize,
                        ) as usize] as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int
                        != 0
                        || (*z
                            .offset(
                                (i as ::core::ffi::c_longlong
                                    + 1 as ::core::ffi::c_longlong) as isize,
                            ) as ::core::ffi::c_int == '+' as i32
                            || *z
                                .offset(
                                    (i as ::core::ffi::c_longlong
                                        + 1 as ::core::ffi::c_longlong) as isize,
                                ) as ::core::ffi::c_int == '-' as i32)
                            && sqlite3CtypeMap[*z
                                .offset(
                                    (i as ::core::ffi::c_longlong
                                        + 2 as ::core::ffi::c_longlong) as isize,
                                ) as usize] as ::core::ffi::c_int
                                & 0x4 as ::core::ffi::c_int != 0)
                {
                    i += 2 as ::core::ffi::c_longlong;
                    while sqlite3CtypeMap[*z.offset(i as isize) as usize]
                        as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0
                    {
                        i += 1;
                    }
                    *tokenType = TK_FLOAT;
                }
                while sqlite3CtypeMap[*z.offset(i as isize) as usize]
                    as ::core::ffi::c_int & 0x46 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
                    *tokenType = TK_ILLEGAL;
                    i += 1;
                }
                return i;
            }
            6880299496751257707 => {
                i = 1 as sqlite3_int64;
            }
            _ => {}
        }
        while sqlite3CtypeMap[*z.offset(i as isize) as usize] as ::core::ffi::c_int
            & 0x46 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            i += 1;
        }
        *tokenType = TK_ID;
        return i;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_normalize(
    mut zSql: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nZ: sqlite3_int64 = 0;
        let mut nSql: sqlite3_int64 = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut tokenType: ::core::ffi::c_int = 0;
        let mut n: sqlite3_int64 = 0;
        let mut k: ::core::ffi::c_int = 0;
        nSql = strlen(zSql) as sqlite3_int64;
        nZ = nSql;
        z = sqlite3_malloc64(
            (nZ as ::core::ffi::c_longlong + 2 as ::core::ffi::c_longlong)
                as sqlite3_uint64,
        ) as *mut ::core::ffi::c_char;
        if z.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        j = 0 as ::core::ffi::c_int;
        i = j;
        while *zSql.offset(i as isize) != 0 {
            n = sqlite3GetToken(
                (zSql as *mut ::core::ffi::c_uchar).offset(i as isize),
                &raw mut tokenType,
            );
            let mut c2rust_current_block_18: u64;
            match tokenType {
                TK_ERROR => {
                    sqlite3_free(z as *mut ::core::ffi::c_void);
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                TK_LITERAL => {
                    let c2rust_fresh0 = j;
                    j = j + 1;
                    *z.offset(c2rust_fresh0 as isize) = '?' as i32
                        as ::core::ffi::c_char;
                }
                TK_PUNCT | TK_NAME => {
                    if n == 4 as ::core::ffi::c_longlong
                        && sqlite3_strnicmp(
                            zSql.offset(i as isize),
                            b"NULL\0".as_ptr() as *const ::core::ffi::c_char,
                            4 as ::core::ffi::c_int,
                        ) == 0 as ::core::ffi::c_int
                    {
                        if j >= 3 as ::core::ffi::c_int
                            && strncmp(
                                z
                                    .offset(j as isize)
                                    .offset(-(2 as ::core::ffi::c_int as isize)),
                                b"is\0".as_ptr() as *const ::core::ffi::c_char,
                                2 as size_t,
                            ) == 0 as ::core::ffi::c_int
                            && !(sqlite3CtypeMap[*z
                                .offset((j - 3 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
                                & 0x46 as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                            || j >= 4 as ::core::ffi::c_int
                                && strncmp(
                                    z
                                        .offset(j as isize)
                                        .offset(-(3 as ::core::ffi::c_int as isize)),
                                    b"not\0".as_ptr() as *const ::core::ffi::c_char,
                                    3 as size_t,
                                ) == 0 as ::core::ffi::c_int
                                && !(sqlite3CtypeMap[*z
                                    .offset((j - 4 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
                                    & 0x46 as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                        {
                            c2rust_current_block_18 = 5948590327928692120;
                        } else {
                            let c2rust_fresh1 = j;
                            j = j + 1;
                            *z.offset(c2rust_fresh1 as isize) = '?' as i32
                                as ::core::ffi::c_char;
                            c2rust_current_block_18 = 2719512138335094285;
                        }
                    } else {
                        c2rust_current_block_18 = 5948590327928692120;
                    }
                    match c2rust_current_block_18 {
                        2719512138335094285 => {}
                        _ => {
                            if j > 0 as ::core::ffi::c_int
                                && sqlite3CtypeMap[*z
                                    .offset((j - 1 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
                                    & 0x46 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                && sqlite3CtypeMap[*zSql.offset(i as isize)
                                    as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
                                    & 0x46 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                            {
                                let c2rust_fresh2 = j;
                                j = j + 1;
                                *z.offset(c2rust_fresh2 as isize) = ' ' as i32
                                    as ::core::ffi::c_char;
                            }
                            k = 0 as ::core::ffi::c_int;
                            while (k as ::core::ffi::c_longlong) < n {
                                let c2rust_fresh3 = j;
                                j = j + 1;
                                *z.offset(c2rust_fresh3 as isize) = sqlite3UpperToLower[*zSql
                                    .offset((i + k) as isize) as ::core::ffi::c_uchar as usize]
                                    as ::core::ffi::c_char;
                                k += 1;
                            }
                        }
                    }
                }
                TK_SPACE | _ => {}
            }
            i = (i as ::core::ffi::c_longlong + n as ::core::ffi::c_longlong)
                as ::core::ffi::c_int;
        }
        while j > 0 as ::core::ffi::c_int
            && *z.offset((j - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == ' ' as i32
        {
            j -= 1;
        }
        if j > 0 as ::core::ffi::c_int
            && *z.offset((j - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                != ';' as i32
        {
            let c2rust_fresh4 = j;
            j = j + 1;
            *z.offset(c2rust_fresh4 as isize) = ';' as i32 as ::core::ffi::c_char;
        }
        *z.offset(j as isize) = 0 as ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while i < j {
            let mut zIn: *mut ::core::ffi::c_char = strstr(
                z.offset(i as isize),
                b"in(\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut nParen: ::core::ffi::c_int = 0;
            if zIn.is_null() {
                break;
            }
            n = (zIn.offset_from(z) as ::core::ffi::c_long as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) as sqlite3_int64;
            if !(n != 0
                && sqlite3CtypeMap[*zIn.offset(-1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
                    & 0x46 as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
            {
                if !(strncmp(
                    zIn,
                    b"in(select\0".as_ptr() as *const ::core::ffi::c_char,
                    9 as size_t,
                ) == 0 as ::core::ffi::c_int
                    && !(sqlite3CtypeMap[*zIn.offset(9 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
                        & 0x46 as ::core::ffi::c_int != 0 as ::core::ffi::c_int))
                {
                    if !(strncmp(
                        zIn,
                        b"in(with\0".as_ptr() as *const ::core::ffi::c_char,
                        7 as size_t,
                    ) == 0 as ::core::ffi::c_int
                        && !(sqlite3CtypeMap[*zIn
                            .offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
                            & 0x46 as ::core::ffi::c_int != 0 as ::core::ffi::c_int))
                    {
                        nParen = 1 as ::core::ffi::c_int;
                        k = 0 as ::core::ffi::c_int;
                        while *z
                            .offset(
                                (n as ::core::ffi::c_longlong
                                    + k as ::core::ffi::c_longlong) as isize,
                            ) != 0
                        {
                            if *z
                                .offset(
                                    (n as ::core::ffi::c_longlong
                                        + k as ::core::ffi::c_longlong) as isize,
                                ) as ::core::ffi::c_int == '(' as i32
                            {
                                nParen += 1;
                            }
                            if *z
                                .offset(
                                    (n as ::core::ffi::c_longlong
                                        + k as ::core::ffi::c_longlong) as isize,
                                ) as ::core::ffi::c_int == ')' as i32
                            {
                                nParen -= 1;
                                if nParen == 0 as ::core::ffi::c_int {
                                    break;
                                }
                            }
                            k += 1;
                        }
                        if k < 5 as ::core::ffi::c_int {
                            z = sqlite3_realloc64(
                                z as *mut ::core::ffi::c_void,
                                (j + (5 as ::core::ffi::c_int - k)
                                    + 1 as ::core::ffi::c_int) as sqlite3_uint64,
                            ) as *mut ::core::ffi::c_char;
                            if z.is_null() {
                                return ::core::ptr::null_mut::<::core::ffi::c_char>();
                            }
                            memmove(
                                z
                                    .offset(n as isize)
                                    .offset(5 as ::core::ffi::c_int as isize)
                                    as *mut ::core::ffi::c_void,
                                z.offset(n as isize).offset(k as isize)
                                    as *const ::core::ffi::c_void,
                                (j as ::core::ffi::c_longlong
                                    - (n as ::core::ffi::c_longlong
                                        + k as ::core::ffi::c_longlong)) as size_t,
                            );
                        } else if k > 5 as ::core::ffi::c_int {
                            memmove(
                                z
                                    .offset(n as isize)
                                    .offset(5 as ::core::ffi::c_int as isize)
                                    as *mut ::core::ffi::c_void,
                                z.offset(n as isize).offset(k as isize)
                                    as *const ::core::ffi::c_void,
                                (j as ::core::ffi::c_longlong
                                    - (n as ::core::ffi::c_longlong
                                        + k as ::core::ffi::c_longlong)) as size_t,
                            );
                        }
                        j = j - k + 5 as ::core::ffi::c_int;
                        *z.offset(j as isize) = 0 as ::core::ffi::c_char;
                        memcpy(
                            z.offset(n as isize) as *mut ::core::ffi::c_void,
                            b"?,?,?\0".as_ptr() as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            5 as size_t,
                        );
                    }
                }
            }
            i = n as ::core::ffi::c_int;
        }
        return z;
    }
}
