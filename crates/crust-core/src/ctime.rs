
static mut sqlite3azCompileOpt: [*const ::core::ffi::c_char; 55] = [
    b"ATOMIC_INTRINSICS=1\0" as *const u8 as *const ::core::ffi::c_char,
    b"COMPILER=clang-20.1.8\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEFAULT_AUTOVACUUM\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEFAULT_CACHE_SIZE=-2000\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEFAULT_FILE_FORMAT=4\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEFAULT_JOURNAL_SIZE_LIMIT=-1\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEFAULT_MMAP_SIZE=0\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEFAULT_PAGE_SIZE=1024\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEFAULT_PCACHE_INITSZ=20\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEFAULT_RECURSIVE_TRIGGERS\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEFAULT_SECTOR_SIZE=4096\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEFAULT_SYNCHRONOUS=2\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEFAULT_WAL_AUTOCHECKPOINT=1000\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEFAULT_WAL_SYNCHRONOUS=2\0" as *const u8 as *const ::core::ffi::c_char,
    b"DEFAULT_WORKER_THREADS=0\0" as *const u8 as *const ::core::ffi::c_char,
    b"DIRECT_OVERFLOW_READ\0" as *const u8 as *const ::core::ffi::c_char,
    b"ENABLE_BYTECODE_VTAB\0" as *const u8 as *const ::core::ffi::c_char,
    b"ENABLE_CARRAY\0" as *const u8 as *const ::core::ffi::c_char,
    b"ENABLE_DBPAGE_VTAB\0" as *const u8 as *const ::core::ffi::c_char,
    b"ENABLE_DBSTAT_VTAB\0" as *const u8 as *const ::core::ffi::c_char,
    b"ENABLE_FTS3\0" as *const u8 as *const ::core::ffi::c_char,
    b"ENABLE_FTS4\0" as *const u8 as *const ::core::ffi::c_char,
    b"ENABLE_FTS5\0" as *const u8 as *const ::core::ffi::c_char,
    b"ENABLE_GEOPOLY\0" as *const u8 as *const ::core::ffi::c_char,
    b"ENABLE_MATH_FUNCTIONS\0" as *const u8 as *const ::core::ffi::c_char,
    b"ENABLE_PERCENTILE\0" as *const u8 as *const ::core::ffi::c_char,
    b"ENABLE_PREUPDATE_HOOK\0" as *const u8 as *const ::core::ffi::c_char,
    b"ENABLE_RTREE\0" as *const u8 as *const ::core::ffi::c_char,
    b"ENABLE_SESSION\0" as *const u8 as *const ::core::ffi::c_char,
    b"ENABLE_STMTVTAB\0" as *const u8 as *const ::core::ffi::c_char,
    b"HAVE_ISNAN\0" as *const u8 as *const ::core::ffi::c_char,
    b"MALLOC_SOFT_LIMIT=1024\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_ATTACHED=10\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_COLUMN=2000\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_COMPOUND_SELECT=500\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_DEFAULT_PAGE_SIZE=8192\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_EXPR_DEPTH=1000\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_FUNCTION_ARG=1000\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_LENGTH=1000000000\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_LIKE_PATTERN_LENGTH=50000\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_MMAP_SIZE=0x7fff0000\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_PAGE_COUNT=0xfffffffe\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_PAGE_SIZE=65536\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_SQL_LENGTH=1000000000\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_TRIGGER_DEPTH=1000\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_VARIABLE_NUMBER=32766\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_VDBE_OP=250000000\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAX_WORKER_THREADS=8\0" as *const u8 as *const ::core::ffi::c_char,
    b"MUTEX_PTHREADS\0" as *const u8 as *const ::core::ffi::c_char,
    b"NEED_ERR_NAME\0" as *const u8 as *const ::core::ffi::c_char,
    b"NO_SYNC\0" as *const u8 as *const ::core::ffi::c_char,
    b"SYSTEM_MALLOC\0" as *const u8 as *const ::core::ffi::c_char,
    b"TEMP_STORE=1\0" as *const u8 as *const ::core::ffi::c_char,
    b"TEST\0" as *const u8 as *const ::core::ffi::c_char,
    b"THREADSAFE=1\0" as *const u8 as *const ::core::ffi::c_char,
];
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3CompileOptions(
    mut pnOpt: *mut ::core::ffi::c_int,
) -> *mut *const ::core::ffi::c_char {
    *pnOpt = (::core::mem::size_of::<[*const ::core::ffi::c_char; 55]>() as usize)
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
        as ::core::ffi::c_int;
    &raw const sqlite3azCompileOpt as *const *const ::core::ffi::c_char
        as *mut *const ::core::ffi::c_char
}
