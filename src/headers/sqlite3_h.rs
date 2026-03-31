pub type sqlite3_stmt = crate::src::headers::vdbeInt_h::Vdbe;

pub type sqlite3_blob = *mut std::ffi::c_void;

pub type sqlite3_pcache = *mut std::ffi::c_void;

pub const SQLITE_VERSION: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"3.51.2\0") };

pub const SQLITE_VERSION_NUMBER: ::core::ffi::c_int = 3051002 as ::core::ffi::c_int;

const fn str_to_i8_array<const N: usize>(s: &str) -> [i8; N] {
    assert!(s.len() == N, "string length does not match array size");
    let mut arr = [0i8; N];
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < N {
        arr[i] = bytes[i] as i8;
        i += 1;
    }
    arr
}

pub const SQLITE_SOURCE_ID: [i8; 85] = str_to_i8_array::<85>("2026-01-09 17:27:48 b270f8339eb13b504d0b2ba154ebca966b7dde08e40c3ed7d559749818cbalt1\0");

pub type sqlite_int64 = ::core::ffi::c_longlong;

pub type sqlite_uint64 = ::core::ffi::c_ulonglong;

pub type sqlite3_int64 = crate::src::headers::sqlite3_h::sqlite_int64;

pub type sqlite3_uint64 = crate::src::headers::sqlite3_h::sqlite_uint64;

pub type sqlite3_callback = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
        *mut *mut ::core::ffi::c_char,
        *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;

pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_INTERNAL: ::core::ffi::c_int = 2;

pub const SQLITE_INTERNAL_1: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_PERM: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const SQLITE_ABORT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const SQLITE_LOCKED: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub const SQLITE_READONLY: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

pub const SQLITE_INTERRUPT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;

pub const SQLITE_INTERRUPT_1: ::core::ffi::c_int = 9;

pub const SQLITE_IOERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

pub const SQLITE_CORRUPT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;

pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;

pub const SQLITE_NOTFOUND_1: ::core::ffi::c_int = 12;

pub const SQLITE_FULL: ::core::ffi::c_int = 13 as ::core::ffi::c_int;

pub const SQLITE_FULL_1: ::core::ffi::c_int = 13;

pub const SQLITE_CANTOPEN: ::core::ffi::c_int = 14 as ::core::ffi::c_int;

pub const SQLITE_PROTOCOL: ::core::ffi::c_int = 15;

pub const SQLITE_PROTOCOL_1: ::core::ffi::c_int = 15 as ::core::ffi::c_int;

pub const SQLITE_EMPTY: ::core::ffi::c_int = 16 as ::core::ffi::c_int;

pub const SQLITE_EMPTY_1: ::core::ffi::c_int = 16;

pub const SQLITE_SCHEMA: ::core::ffi::c_int = 17 as ::core::ffi::c_int;

pub const SQLITE_SCHEMA_1: ::core::ffi::c_int = 17;

pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18 as ::core::ffi::c_int;

pub const SQLITE_TOOBIG_1: ::core::ffi::c_int = 18;

pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;

pub const SQLITE_MISMATCH: ::core::ffi::c_int = 20 as ::core::ffi::c_int;

pub const SQLITE_MISMATCH_1: ::core::ffi::c_int = 20;

pub const SQLITE_MISUSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;

pub const SQLITE_NOLFS: ::core::ffi::c_int = 22;

pub const SQLITE_AUTH: ::core::ffi::c_int = 23 as ::core::ffi::c_int;

pub const SQLITE_AUTH_1: ::core::ffi::c_int = 23;

pub const SQLITE_FORMAT: ::core::ffi::c_int = 24;

pub const SQLITE_RANGE: ::core::ffi::c_int = 25 as ::core::ffi::c_int;

pub const SQLITE_NOTADB: ::core::ffi::c_int = 26 as ::core::ffi::c_int;

pub const SQLITE_NOTADB_1: ::core::ffi::c_int = 26;

pub const SQLITE_NOTICE: ::core::ffi::c_int = 27 as ::core::ffi::c_int;

pub const SQLITE_WARNING: ::core::ffi::c_int = 28 as ::core::ffi::c_int;

pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;

pub const SQLITE_ROW_1: ::core::ffi::c_int = 100;

pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;

pub const SQLITE_DONE_1: ::core::ffi::c_int = 101;

pub const SQLITE_ERROR_MISSING_COLLSEQ: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_ERROR | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_ERROR_MISSING_COLLSEQ_1: ::core::ffi::c_int = 257;

pub const SQLITE_ERROR_RETRY: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_ERROR | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_ERROR_RETRY_1: ::core::ffi::c_int = 513;

pub const SQLITE_ERROR_SNAPSHOT: ::core::ffi::c_int = 769;

pub const SQLITE_IOERR_READ: ::core::ffi::c_int = 266;

pub const SQLITE_IOERR_READ_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_IOERR_SHORT_READ: ::core::ffi::c_int = 522;

pub const SQLITE_IOERR_SHORT_READ_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_IOERR_WRITE: ::core::ffi::c_int = 778;

pub const SQLITE_IOERR_WRITE_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_IOERR_FSYNC: ::core::ffi::c_int = 1034;

pub const SQLITE_IOERR_DIR_FSYNC: ::core::ffi::c_int = 1290;

pub const SQLITE_IOERR_TRUNCATE: ::core::ffi::c_int = 1546;

pub const SQLITE_IOERR_FSTAT: ::core::ffi::c_int = 1802;

pub const SQLITE_IOERR_FSTAT_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (7 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_IOERR_UNLOCK: ::core::ffi::c_int = 2058;

pub const SQLITE_IOERR_UNLOCK_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_IOERR_RDLOCK: ::core::ffi::c_int = 2314;

pub const SQLITE_IOERR_RDLOCK_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (9 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_IOERR_DELETE: ::core::ffi::c_int = 2570;

pub const SQLITE_IOERR_NOMEM: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_IOERR_ACCESS: ::core::ffi::c_int = 3338;

pub const SQLITE_IOERR_ACCESS_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (13 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_IOERR_CHECKRESERVEDLOCK: ::core::ffi::c_int = 3594;

pub const SQLITE_IOERR_CHECKRESERVEDLOCK_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (14 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_IOERR_LOCK: ::core::ffi::c_int = 3850;

pub const SQLITE_IOERR_LOCK_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (15 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_IOERR_CLOSE: ::core::ffi::c_int = 4106;

pub const SQLITE_IOERR_CLOSE_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_IOERR_DIR_CLOSE: ::core::ffi::c_int = 4362;

pub const SQLITE_IOERR_SHMOPEN: ::core::ffi::c_int = 4618;

pub const SQLITE_IOERR_SHMSIZE: ::core::ffi::c_int = 4874;

pub const SQLITE_IOERR_SHMSIZE_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (19 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_IOERR_SHMLOCK: ::core::ffi::c_int = 5130;

pub const SQLITE_IOERR_SHMLOCK_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (20 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_IOERR_SHMMAP: ::core::ffi::c_int = 5386;

pub const SQLITE_IOERR_SEEK: ::core::ffi::c_int = 5642;

pub const SQLITE_IOERR_DELETE_NOENT: ::core::ffi::c_int = 5898;

pub const SQLITE_IOERR_DELETE_NOENT_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (23 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_IOERR_MMAP: ::core::ffi::c_int = 6154;

pub const SQLITE_IOERR_GETTEMPPATH: ::core::ffi::c_int = 6410;

pub const SQLITE_IOERR_GETTEMPPATH_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (25 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_IOERR_CONVPATH: ::core::ffi::c_int = 6666;

pub const SQLITE_IOERR_CORRUPTFS: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_IOERR | (33 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_LOCKED_SHAREDCACHE: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_LOCKED | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_LOCKED_SHAREDCACHE_1: ::core::ffi::c_int = 262;

pub const SQLITE_LOCKED_VTAB: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_LOCKED | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_BUSY_RECOVERY: ::core::ffi::c_int = 261;

pub const SQLITE_BUSY_RECOVERY_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_BUSY | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_BUSY_SNAPSHOT: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_BUSY | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_BUSY_SNAPSHOT_1: ::core::ffi::c_int = 517;

pub const SQLITE_CANTOPEN_NOTEMPDIR: ::core::ffi::c_int = 270;

pub const SQLITE_CANTOPEN_ISDIR: ::core::ffi::c_int = 526;

pub const SQLITE_CANTOPEN_FULLPATH: ::core::ffi::c_int = 782;

pub const SQLITE_CANTOPEN_CONVPATH: ::core::ffi::c_int = 1038;

pub const SQLITE_CANTOPEN_SYMLINK: ::core::ffi::c_int = 1550;

pub const SQLITE_CANTOPEN_SYMLINK_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_CANTOPEN | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_CORRUPT_VTAB: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_CORRUPT | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_CORRUPT_VTAB_1: ::core::ffi::c_int = 267;

pub const SQLITE_CORRUPT_SEQUENCE: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_CORRUPT | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_CORRUPT_INDEX: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_CORRUPT | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_READONLY_RECOVERY: ::core::ffi::c_int = 264;

pub const SQLITE_READONLY_RECOVERY_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_READONLY | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_READONLY_ROLLBACK: ::core::ffi::c_int = 776;

pub const SQLITE_READONLY_ROLLBACK_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_READONLY | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_READONLY_DBMOVED: ::core::ffi::c_int = 1032;

pub const SQLITE_READONLY_DBMOVED_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_READONLY | (4 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_READONLY_CANTINIT: ::core::ffi::c_int = 1288;

pub const SQLITE_READONLY_CANTINIT_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_READONLY | (5 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_READONLY_DIRECTORY: ::core::ffi::c_int = 1544;

pub const SQLITE_READONLY_DIRECTORY_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_READONLY | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_ABORT_ROLLBACK: ::core::ffi::c_int = 516;

pub const SQLITE_ABORT_ROLLBACK_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_ABORT | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_CONSTRAINT_CHECK: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_CONSTRAINT_CHECK_1: ::core::ffi::c_int = 275;

pub const SQLITE_CONSTRAINT_COMMITHOOK: ::core::ffi::c_int = 531;

pub const SQLITE_CONSTRAINT_COMMITHOOK_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_CONSTRAINT_FOREIGNKEY: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT | (3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_CONSTRAINT_FOREIGNKEY_1: ::core::ffi::c_int = 787;

pub const SQLITE_CONSTRAINT_FUNCTION: ::core::ffi::c_int = 1043;

pub const SQLITE_CONSTRAINT_NOTNULL: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT | (5 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_CONSTRAINT_NOTNULL_1: ::core::ffi::c_int = 1299;

pub const SQLITE_CONSTRAINT_PRIMARYKEY: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_CONSTRAINT_PRIMARYKEY_1: ::core::ffi::c_int = 1555;

pub const SQLITE_CONSTRAINT_TRIGGER: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT | (7 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_CONSTRAINT_TRIGGER_1: ::core::ffi::c_int = 1811;

pub const SQLITE_CONSTRAINT_UNIQUE: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT | (8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_CONSTRAINT_UNIQUE_1: ::core::ffi::c_int = 2067;

pub const SQLITE_CONSTRAINT_VTAB: ::core::ffi::c_int = 2323;

pub const SQLITE_CONSTRAINT_ROWID: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT | (10 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_CONSTRAINT_ROWID_1: ::core::ffi::c_int = 2579;

pub const SQLITE_CONSTRAINT_PINNED: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT | (11 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_CONSTRAINT_DATATYPE: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT | (12 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_NOTICE_RECOVER_WAL: ::core::ffi::c_int = 283;

pub const SQLITE_NOTICE_RECOVER_WAL_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_NOTICE | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_NOTICE_RECOVER_ROLLBACK: ::core::ffi::c_int = 539;

pub const SQLITE_NOTICE_RECOVER_ROLLBACK_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_NOTICE | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_NOTICE_RBU: ::core::ffi::c_int = 795;

pub const SQLITE_WARNING_AUTOINDEX: ::core::ffi::c_int = 284;

pub const SQLITE_WARNING_AUTOINDEX_1: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_WARNING | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_OK_LOAD_PERMANENTLY: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_OK | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_OK_SYMLINK: ::core::ffi::c_int =
    crate::src::headers::sqlite3_h::SQLITE_OK | (2 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;

pub const SQLITE_OPEN_READONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const SQLITE_OPEN_DELETEONCLOSE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const SQLITE_OPEN_EXCLUSIVE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const SQLITE_OPEN_URI: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const SQLITE_OPEN_MEMORY: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const SQLITE_OPEN_MAIN_DB: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;

pub const SQLITE_OPEN_TEMP_DB: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;

pub const SQLITE_OPEN_TRANSIENT_DB: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;

pub const SQLITE_OPEN_MAIN_JOURNAL: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;

pub const SQLITE_OPEN_TEMP_JOURNAL: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;

pub const SQLITE_OPEN_SUBJOURNAL: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;

pub const SQLITE_OPEN_SUPER_JOURNAL: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;

pub const SQLITE_OPEN_NOMUTEX: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;

pub const SQLITE_OPEN_FULLMUTEX: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;

pub const SQLITE_OPEN_SHAREDCACHE: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;

pub const SQLITE_OPEN_PRIVATECACHE: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;

pub const SQLITE_OPEN_WAL: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;

pub const SQLITE_OPEN_NOFOLLOW: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;

pub const SQLITE_OPEN_EXRESCODE: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;

pub const SQLITE_IOCAP_ATOMIC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const SQLITE_IOCAP_SAFE_APPEND: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;

pub const SQLITE_IOCAP_SEQUENTIAL: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;

pub const SQLITE_IOCAP_UNDELETABLE_WHEN_OPEN: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;

pub const SQLITE_IOCAP_POWERSAFE_OVERWRITE: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;

pub const SQLITE_IOCAP_IMMUTABLE: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;

pub const SQLITE_IOCAP_SUBPAGE_READ: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;

pub const SQLITE_LOCK_SHARED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_LOCK_RESERVED: ::core::ffi::c_int = 2;

pub const SQLITE_LOCK_PENDING: ::core::ffi::c_int = 3;

pub const SQLITE_LOCK_EXCLUSIVE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SQLITE_SYNC_NORMAL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const SQLITE_SYNC_FULL: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;

pub const SQLITE_SYNC_DATAONLY: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_file {
    pub pMethods: *const crate::src::headers::sqlite3_h::sqlite3_io_methods,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_io_methods {
    pub iVersion: ::core::ffi::c_int,
    pub xClose:
        Option<unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int>,
    pub xRead: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            crate::src::headers::sqlite3_h::sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xWrite: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            crate::src::headers::sqlite3_h::sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xTruncate: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
            crate::src::headers::sqlite3_h::sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xSync: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xFileSize: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
            *mut crate::src::headers::sqlite3_h::sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xLock: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xUnlock: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xCheckReservedLock: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xFileControl: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xSectorSize:
        Option<unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int>,
    pub xDeviceCharacteristics:
        Option<unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int>,
    pub xShmMap: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xShmLock: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xShmBarrier: Option<unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ()>,
    pub xShmUnmap: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xFetch: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
            crate::src::headers::sqlite3_h::sqlite3_int64,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xUnfetch: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
            crate::src::headers::sqlite3_h::sqlite3_int64,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}

pub const SQLITE_FCNTL_LOCKSTATE: ::core::ffi::c_int = 1;

pub const SQLITE_FCNTL_LOCKSTATE_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_LAST_ERRNO: ::core::ffi::c_int = 4;

pub const SQLITE_FCNTL_SIZE_HINT: ::core::ffi::c_int = 5;

pub const SQLITE_FCNTL_SIZE_HINT_1: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_CHUNK_SIZE: ::core::ffi::c_int = 6;

pub const SQLITE_FCNTL_CHUNK_SIZE_1: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_FILE_POINTER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_PERSIST_WAL: ::core::ffi::c_int = 10;

pub const SQLITE_FCNTL_PERSIST_WAL_1: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_OVERWRITE: ::core::ffi::c_int = 11 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_VFSNAME: ::core::ffi::c_int = 12 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_VFSNAME_1: ::core::ffi::c_int = 12;

pub const SQLITE_FCNTL_POWERSAFE_OVERWRITE: ::core::ffi::c_int = 13;

pub const SQLITE_FCNTL_PRAGMA: ::core::ffi::c_int = 14 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_BUSYHANDLER: ::core::ffi::c_int = 15 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_TEMPFILENAME: ::core::ffi::c_int = 16;

pub const SQLITE_FCNTL_MMAP_SIZE: ::core::ffi::c_int = 18;

pub const SQLITE_FCNTL_MMAP_SIZE_1: ::core::ffi::c_int = 18 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_HAS_MOVED: ::core::ffi::c_int = 20;

pub const SQLITE_FCNTL_HAS_MOVED_1: ::core::ffi::c_int = 20 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_SYNC: ::core::ffi::c_int = 21 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_COMMIT_PHASETWO: ::core::ffi::c_int = 22 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_VFS_POINTER: ::core::ffi::c_int = 27 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_JOURNAL_POINTER: ::core::ffi::c_int = 28 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_PDB: ::core::ffi::c_int = 30 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_LOCK_TIMEOUT: ::core::ffi::c_int = 34 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_DATA_VERSION: ::core::ffi::c_int = 35 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_SIZE_LIMIT: ::core::ffi::c_int = 36 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_CKPT_DONE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_RESERVE_BYTES: ::core::ffi::c_int = 38 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_CKPT_START: ::core::ffi::c_int = 39 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_EXTERNAL_READER: ::core::ffi::c_int = 40;

pub const SQLITE_FCNTL_RESET_CACHE: ::core::ffi::c_int = 42 as ::core::ffi::c_int;

pub const SQLITE_FCNTL_NULL_IO: ::core::ffi::c_int = 43;

pub type sqlite3_filename = *const ::core::ffi::c_char;

pub type sqlite3_syscall_ptr = Option<unsafe extern "C" fn() -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_vfs {
    pub iVersion: ::core::ffi::c_int,
    pub szOsFile: ::core::ffi::c_int,
    pub mxPathname: ::core::ffi::c_int,
    pub pNext: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    pub zName: *const ::core::ffi::c_char,
    pub pAppData: *mut ::core::ffi::c_void,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            crate::src::headers::sqlite3_h::sqlite3_filename,
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xDelete: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xAccess: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xFullPathname: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xDlOpen: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub xDlError: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> (),
    >,
    pub xDlSym: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> Option<unsafe extern "C" fn() -> ()>,
    >,
    pub xDlClose: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
    >,
    pub xRandomness: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xSleep: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xCurrentTime: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            *mut ::core::ffi::c_double,
        ) -> ::core::ffi::c_int,
    >,
    pub xGetLastError: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xCurrentTimeInt64: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            *mut crate::src::headers::sqlite3_h::sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xSetSystemCall: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            *const ::core::ffi::c_char,
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
        ) -> ::core::ffi::c_int,
    >,
    pub xGetSystemCall: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            *const ::core::ffi::c_char,
        ) -> crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
    >,
    pub xNextSystemCall: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            *const ::core::ffi::c_char,
        ) -> *const ::core::ffi::c_char,
    >,
}

pub const SQLITE_ACCESS_EXISTS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQLITE_ACCESS_READWRITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_SHM_UNLOCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_SHM_LOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_SHM_SHARED: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SQLITE_SHM_EXCLUSIVE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

pub const SQLITE_SHM_NLOCK: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_mem_methods {
    pub xMalloc: Option<unsafe extern "C" fn(::core::ffi::c_int) -> *mut ::core::ffi::c_void>,
    pub xFree: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub xRealloc: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub xSize: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub xRoundup: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xInit: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub xShutdown: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pAppData: *mut ::core::ffi::c_void,
}

pub const SQLITE_CONFIG_SINGLETHREAD: ::core::ffi::c_int = 1;

pub const SQLITE_CONFIG_MULTITHREAD: ::core::ffi::c_int = 2;

pub const SQLITE_CONFIG_SERIALIZED: ::core::ffi::c_int = 3;

pub const SQLITE_CONFIG_MALLOC: ::core::ffi::c_int = 4;

pub const SQLITE_CONFIG_MALLOC_1: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SQLITE_CONFIG_GETMALLOC: ::core::ffi::c_int = 5;

pub const SQLITE_CONFIG_PAGECACHE: ::core::ffi::c_int = 7;

pub const SQLITE_CONFIG_MEMSTATUS: ::core::ffi::c_int = 9;

pub const SQLITE_CONFIG_MUTEX: ::core::ffi::c_int = 10;

pub const SQLITE_CONFIG_GETMUTEX: ::core::ffi::c_int = 11;

pub const SQLITE_CONFIG_LOOKASIDE: ::core::ffi::c_int = 13;

pub const SQLITE_CONFIG_PCACHE: ::core::ffi::c_int = 14;

pub const SQLITE_CONFIG_GETPCACHE: ::core::ffi::c_int = 15;

pub const SQLITE_CONFIG_LOG: ::core::ffi::c_int = 16;

pub const SQLITE_CONFIG_URI: ::core::ffi::c_int = 17;

pub const SQLITE_CONFIG_PCACHE2: ::core::ffi::c_int = 18;

pub const SQLITE_CONFIG_PCACHE2_1: ::core::ffi::c_int = 18 as ::core::ffi::c_int;

pub const SQLITE_CONFIG_GETPCACHE2: ::core::ffi::c_int = 19;

pub const SQLITE_CONFIG_COVERING_INDEX_SCAN: ::core::ffi::c_int = 20;

pub const SQLITE_CONFIG_MMAP_SIZE: ::core::ffi::c_int = 22;

pub const SQLITE_CONFIG_PCACHE_HDRSZ: ::core::ffi::c_int = 24;

pub const SQLITE_CONFIG_PMASZ: ::core::ffi::c_int = 25;

pub const SQLITE_CONFIG_STMTJRNL_SPILL: ::core::ffi::c_int = 26;

pub const SQLITE_CONFIG_SMALL_MALLOC: ::core::ffi::c_int = 27;

pub const SQLITE_CONFIG_MEMDB_MAXSIZE: ::core::ffi::c_int = 29;

pub const SQLITE_CONFIG_ROWID_IN_VIEW: ::core::ffi::c_int = 30;

pub const SQLITE_DBCONFIG_MAINDBNAME: ::core::ffi::c_int = 1000;

pub const SQLITE_DBCONFIG_LOOKASIDE: ::core::ffi::c_int = 1001;

pub const SQLITE_DBCONFIG_ENABLE_FKEY: ::core::ffi::c_int = 1002 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_ENABLE_TRIGGER: ::core::ffi::c_int = 1003 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER: ::core::ffi::c_int = 1004 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION: ::core::ffi::c_int = 1005 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE: ::core::ffi::c_int = 1006 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_ENABLE_QPSG: ::core::ffi::c_int = 1007 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_TRIGGER_EQP: ::core::ffi::c_int = 1008 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_RESET_DATABASE: ::core::ffi::c_int = 1009 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_DEFENSIVE: ::core::ffi::c_int = 1010 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_WRITABLE_SCHEMA: ::core::ffi::c_int = 1011 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_LEGACY_ALTER_TABLE: ::core::ffi::c_int = 1012 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_DQS_DML: ::core::ffi::c_int = 1013 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_DQS_DDL: ::core::ffi::c_int = 1014 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_ENABLE_VIEW: ::core::ffi::c_int = 1015 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_LEGACY_FILE_FORMAT: ::core::ffi::c_int = 1016 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_TRUSTED_SCHEMA: ::core::ffi::c_int = 1017 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_STMT_SCANSTATUS: ::core::ffi::c_int = 1018 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_REVERSE_SCANORDER: ::core::ffi::c_int = 1019 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_ENABLE_ATTACH_CREATE: ::core::ffi::c_int = 1020 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_ENABLE_ATTACH_WRITE: ::core::ffi::c_int = 1021 as ::core::ffi::c_int;

pub const SQLITE_DBCONFIG_ENABLE_COMMENTS: ::core::ffi::c_int = 1022 as ::core::ffi::c_int;

pub const SQLITE_DENY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_IGNORE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_CREATE_INDEX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_CREATE_TABLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_CREATE_TEMP_INDEX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const SQLITE_CREATE_TEMP_TABLE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SQLITE_CREATE_TEMP_TRIGGER: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const SQLITE_CREATE_TEMP_VIEW: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub const SQLITE_CREATE_TRIGGER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub const SQLITE_CREATE_VIEW: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

pub const SQLITE_DELETE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;

pub const SQLITE_DROP_INDEX: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

pub const SQLITE_DROP_TABLE: ::core::ffi::c_int = 11 as ::core::ffi::c_int;

pub const SQLITE_DROP_TEMP_INDEX: ::core::ffi::c_int = 12 as ::core::ffi::c_int;

pub const SQLITE_DROP_TEMP_TABLE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;

pub const SQLITE_DROP_TEMP_TRIGGER: ::core::ffi::c_int = 14 as ::core::ffi::c_int;

pub const SQLITE_DROP_TEMP_VIEW: ::core::ffi::c_int = 15 as ::core::ffi::c_int;

pub const SQLITE_DROP_TRIGGER: ::core::ffi::c_int = 16 as ::core::ffi::c_int;

pub const SQLITE_DROP_VIEW: ::core::ffi::c_int = 17 as ::core::ffi::c_int;

pub const SQLITE_INSERT: ::core::ffi::c_int = 18 as ::core::ffi::c_int;

pub const SQLITE_PRAGMA: ::core::ffi::c_int = 19 as ::core::ffi::c_int;

pub const SQLITE_READ: ::core::ffi::c_int = 20 as ::core::ffi::c_int;

pub const SQLITE_SELECT: ::core::ffi::c_int = 21 as ::core::ffi::c_int;

pub const SQLITE_TRANSACTION: ::core::ffi::c_int = 22 as ::core::ffi::c_int;

pub const SQLITE_UPDATE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;

pub const SQLITE_ATTACH: ::core::ffi::c_int = 24 as ::core::ffi::c_int;

pub const SQLITE_DETACH: ::core::ffi::c_int = 25 as ::core::ffi::c_int;

pub const SQLITE_ALTER_TABLE: ::core::ffi::c_int = 26 as ::core::ffi::c_int;

pub const SQLITE_REINDEX: ::core::ffi::c_int = 27 as ::core::ffi::c_int;

pub const SQLITE_ANALYZE: ::core::ffi::c_int = 28 as ::core::ffi::c_int;

pub const SQLITE_CREATE_VTABLE: ::core::ffi::c_int = 29 as ::core::ffi::c_int;

pub const SQLITE_DROP_VTABLE: ::core::ffi::c_int = 30 as ::core::ffi::c_int;

pub const SQLITE_FUNCTION: ::core::ffi::c_int = 31 as ::core::ffi::c_int;

pub const SQLITE_SAVEPOINT: ::core::ffi::c_int = 32 as ::core::ffi::c_int;

pub const SQLITE_RECURSIVE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;

pub const SQLITE_TRACE_STMT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const SQLITE_TRACE_PROFILE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const SQLITE_TRACE_ROW: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const SQLITE_TRACE_CLOSE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const SQLITE_LIMIT_LENGTH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQLITE_LIMIT_SQL_LENGTH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_LIMIT_COLUMN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_LIMIT_EXPR_DEPTH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const SQLITE_LIMIT_COMPOUND_SELECT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SQLITE_LIMIT_VDBE_OP: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const SQLITE_LIMIT_FUNCTION_ARG: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub const SQLITE_LIMIT_ATTACHED: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub const SQLITE_LIMIT_LIKE_PATTERN_LENGTH: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

pub const SQLITE_LIMIT_VARIABLE_NUMBER: ::core::ffi::c_int = 9 as ::core::ffi::c_int;

pub const SQLITE_LIMIT_TRIGGER_DEPTH: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

pub const SQLITE_LIMIT_WORKER_THREADS: ::core::ffi::c_int = 11 as ::core::ffi::c_int;

pub const SQLITE_PREPARE_PERSISTENT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const SQLITE_PREPARE_NO_VTAB: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const SQLITE_PREPARE_DONT_LOG: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const SQLITE_INTEGER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_INTEGER_1: ::core::ffi::c_int = 1;

pub const SQLITE_FLOAT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_FLOAT_1: ::core::ffi::c_int = 2;

pub const SQLITE_BLOB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SQLITE_BLOB_1: ::core::ffi::c_int = 4;

pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const SQLITE_NULL_1: ::core::ffi::c_int = 5;

pub const SQLITE_TEXT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const SQLITE_TEXT_1: ::core::ffi::c_int = 3;

pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_UTF16LE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_UTF16BE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const SQLITE_UTF16: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SQLITE_ANY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const SQLITE_ANY_1: ::core::ffi::c_int = 5;

pub const SQLITE_UTF16_ALIGNED: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

pub const SQLITE_DETERMINISTIC: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;

pub const SQLITE_DIRECTONLY: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;

pub const SQLITE_SUBTYPE: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;

pub const SQLITE_INNOCUOUS: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;

pub const SQLITE_RESULT_SUBTYPE: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;

pub const SQLITE_SELFORDER1: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;

pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;

pub const SQLITE_STATIC: crate::src::headers::sqlite3_h::sqlite3_destructor_type = None;

pub const SQLITE_TXN_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQLITE_TXN_WRITE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_module {
    pub iVersion: ::core::ffi::c_int,
    pub xCreate: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xConnect: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xBestIndex: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
            *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
        ) -> ::core::ffi::c_int,
    >,
    pub xDisconnect:
        Option<unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xDestroy:
        Option<unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
            *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xFilter: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub xNext: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xEof: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xColumn: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xRowid: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
            *mut crate::src::headers::sqlite3_h::sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xUpdate: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
            *mut crate::src::headers::sqlite3_h::sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xBegin:
        Option<unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xSync:
        Option<unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xCommit:
        Option<unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xRollback:
        Option<unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xFindFunction: Option<
        unsafe extern "C" fn(
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
    >,
    pub xRename: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xSavepoint: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xRelease: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xRollbackTo: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xShadowName: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub xIntegrity: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_index_info {
    pub nConstraint: ::core::ffi::c_int,
    pub aConstraint: *mut crate::src::headers::sqlite3_h::sqlite3_index_constraint,
    pub nOrderBy: ::core::ffi::c_int,
    pub aOrderBy: *mut crate::src::headers::sqlite3_h::sqlite3_index_orderby,
    pub aConstraintUsage: *mut crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage,
    pub idxNum: ::core::ffi::c_int,
    pub idxStr: *mut ::core::ffi::c_char,
    pub needToFreeIdxStr: ::core::ffi::c_int,
    pub orderByConsumed: ::core::ffi::c_int,
    pub estimatedCost: ::core::ffi::c_double,
    pub estimatedRows: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub idxFlags: ::core::ffi::c_int,
    pub colUsed: crate::src::headers::sqlite3_h::sqlite3_uint64,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_index_constraint {
    pub iColumn: ::core::ffi::c_int,
    pub op: ::core::ffi::c_uchar,
    pub usable: ::core::ffi::c_uchar,
    pub iTermOffset: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_index_orderby {
    pub iColumn: ::core::ffi::c_int,
    pub desc: ::core::ffi::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_index_constraint_usage {
    pub argvIndex: ::core::ffi::c_int,
    pub omit: ::core::ffi::c_uchar,
}

pub const SQLITE_INDEX_SCAN_UNIQUE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const SQLITE_INDEX_SCAN_HEX: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_EQ_1: ::core::ffi::c_int = 2;

pub const SQLITE_INDEX_CONSTRAINT_GT: ::core::ffi::c_int = 4;

pub const SQLITE_INDEX_CONSTRAINT_GT_1: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_LE: ::core::ffi::c_int = 8;

pub const SQLITE_INDEX_CONSTRAINT_LE_1: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_LT: ::core::ffi::c_int = 16;

pub const SQLITE_INDEX_CONSTRAINT_LT_1: ::core::ffi::c_int = 16 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_GE: ::core::ffi::c_int = 32;

pub const SQLITE_INDEX_CONSTRAINT_GE_1: ::core::ffi::c_int = 32 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_MATCH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_MATCH_1: ::core::ffi::c_int = 64;

pub const SQLITE_INDEX_CONSTRAINT_LIKE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_GLOB: ::core::ffi::c_int = 66 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_REGEXP: ::core::ffi::c_int = 67 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_NE: ::core::ffi::c_int = 68 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_ISNOT: ::core::ffi::c_int = 69 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_ISNOTNULL: ::core::ffi::c_int = 70 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_ISNULL: ::core::ffi::c_int = 71 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_IS: ::core::ffi::c_int = 72 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_LIMIT: ::core::ffi::c_int = 73 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_OFFSET: ::core::ffi::c_int = 74 as ::core::ffi::c_int;

pub const SQLITE_INDEX_CONSTRAINT_FUNCTION: ::core::ffi::c_int = 150 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_vtab {
    pub pModule: *const crate::src::headers::sqlite3_h::sqlite3_module,
    pub nRef: ::core::ffi::c_int,
    pub zErrMsg: *mut ::core::ffi::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_vtab_cursor {
    pub pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_mutex_methods {
    pub xMutexInit: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub xMutexEnd: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub xMutexAlloc: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> *mut crate::src::src::mutex_unix::sqlite3_mutex,
    >,
    pub xMutexFree:
        Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ()>,
    pub xMutexEnter:
        Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ()>,
    pub xMutexTry: Option<
        unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ::core::ffi::c_int,
    >,
    pub xMutexLeave:
        Option<unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ()>,
    pub xMutexHeld: Option<
        unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ::core::ffi::c_int,
    >,
    pub xMutexNotheld: Option<
        unsafe extern "C" fn(*mut crate::src::src::mutex_unix::sqlite3_mutex) -> ::core::ffi::c_int,
    >,
}

pub const SQLITE_MUTEX_FAST: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQLITE_MUTEX_FAST_1: ::core::ffi::c_int = 0;

pub const SQLITE_MUTEX_RECURSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_MUTEX_RECURSIVE_1: ::core::ffi::c_int = 1;

pub const SQLITE_MUTEX_STATIC_MAIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_MUTEX_STATIC_MEM: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const SQLITE_MUTEX_STATIC_OPEN: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SQLITE_MUTEX_STATIC_PRNG: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const SQLITE_MUTEX_STATIC_LRU: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub const SQLITE_MUTEX_STATIC_PMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub const SQLITE_MUTEX_STATIC_VFS1: ::core::ffi::c_int = 11 as ::core::ffi::c_int;

pub const SQLITE_TESTCTRL_PRNG_SAVE: ::core::ffi::c_int = 5;

pub const SQLITE_TESTCTRL_PRNG_RESTORE: ::core::ffi::c_int = 6;

pub const SQLITE_TESTCTRL_FK_NO_ACTION: ::core::ffi::c_int = 7;

pub const SQLITE_TESTCTRL_BITVEC_TEST: ::core::ffi::c_int = 8;

pub const SQLITE_TESTCTRL_FAULT_INSTALL: ::core::ffi::c_int = 9;

pub const SQLITE_TESTCTRL_BENIGN_MALLOC_HOOKS: ::core::ffi::c_int = 10;

pub const SQLITE_TESTCTRL_PENDING_BYTE: ::core::ffi::c_int = 11;

pub const SQLITE_TESTCTRL_ASSERT: ::core::ffi::c_int = 12;

pub const SQLITE_TESTCTRL_ALWAYS: ::core::ffi::c_int = 13;

pub const SQLITE_TESTCTRL_JSON_SELFCHECK: ::core::ffi::c_int = 14;

pub const SQLITE_TESTCTRL_OPTIMIZATIONS: ::core::ffi::c_int = 15;

pub const SQLITE_TESTCTRL_GETOPT: ::core::ffi::c_int = 16;

pub const SQLITE_TESTCTRL_INTERNAL_FUNCTIONS: ::core::ffi::c_int = 17;

pub const SQLITE_TESTCTRL_LOCALTIME_FAULT: ::core::ffi::c_int = 18;

pub const SQLITE_TESTCTRL_ONCE_RESET_THRESHOLD: ::core::ffi::c_int = 19;

pub const SQLITE_TESTCTRL_NEVER_CORRUPT: ::core::ffi::c_int = 20;

pub const SQLITE_TESTCTRL_VDBE_COVERAGE: ::core::ffi::c_int = 21;

pub const SQLITE_TESTCTRL_BYTEORDER: ::core::ffi::c_int = 22;

pub const SQLITE_TESTCTRL_ISINIT: ::core::ffi::c_int = 23;

pub const SQLITE_TESTCTRL_SORTER_MMAP: ::core::ffi::c_int = 24;

pub const SQLITE_TESTCTRL_IMPOSTER: ::core::ffi::c_int = 25;

pub const SQLITE_TESTCTRL_RESULT_INTREAL: ::core::ffi::c_int = 27;

pub const SQLITE_TESTCTRL_PRNG_SEED: ::core::ffi::c_int = 28;

pub const SQLITE_TESTCTRL_EXTRA_SCHEMA_CHECKS: ::core::ffi::c_int = 29;

pub const SQLITE_TESTCTRL_SEEK_COUNT: ::core::ffi::c_int = 30;

pub const SQLITE_TESTCTRL_TRACEFLAGS: ::core::ffi::c_int = 31;

pub const SQLITE_TESTCTRL_LOGEST: ::core::ffi::c_int = 33;

pub const SQLITE_STATUS_MEMORY_USED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQLITE_STATUS_PAGECACHE_USED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_STATUS_PAGECACHE_OVERFLOW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_STATUS_MALLOC_SIZE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const SQLITE_STATUS_PAGECACHE_SIZE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub const SQLITE_STATUS_MALLOC_COUNT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;

pub const SQLITE_DBSTATUS_LOOKASIDE_USED: ::core::ffi::c_int = 0;

pub const SQLITE_DBSTATUS_CACHE_USED: ::core::ffi::c_int = 1;

pub const SQLITE_DBSTATUS_SCHEMA_USED: ::core::ffi::c_int = 2;

pub const SQLITE_DBSTATUS_STMT_USED: ::core::ffi::c_int = 3;

pub const SQLITE_DBSTATUS_LOOKASIDE_HIT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_SIZE: ::core::ffi::c_int = 5;

pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_FULL: ::core::ffi::c_int = 6;

pub const SQLITE_DBSTATUS_CACHE_HIT: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub const SQLITE_DBSTATUS_CACHE_HIT_1: ::core::ffi::c_int = 7;

pub const SQLITE_DBSTATUS_CACHE_MISS: ::core::ffi::c_int = 8;

pub const SQLITE_DBSTATUS_CACHE_WRITE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;

pub const SQLITE_DBSTATUS_DEFERRED_FKS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

pub const SQLITE_DBSTATUS_DEFERRED_FKS_1: ::core::ffi::c_int = 10;

pub const SQLITE_DBSTATUS_CACHE_USED_SHARED: ::core::ffi::c_int = 11 as ::core::ffi::c_int;

pub const SQLITE_DBSTATUS_CACHE_SPILL: ::core::ffi::c_int = 12;

pub const SQLITE_DBSTATUS_TEMPBUF_SPILL: ::core::ffi::c_int = 13;

pub const SQLITE_STMTSTATUS_FULLSCAN_STEP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_STMTSTATUS_SORT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_STMTSTATUS_AUTOINDEX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const SQLITE_STMTSTATUS_VM_STEP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SQLITE_STMTSTATUS_REPREPARE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const SQLITE_STMTSTATUS_RUN: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub const SQLITE_STMTSTATUS_FILTER_MISS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub const SQLITE_STMTSTATUS_FILTER_HIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

pub const SQLITE_STMTSTATUS_MEMUSED: ::core::ffi::c_int = 99 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_pcache_page {
    pub pBuf: *mut ::core::ffi::c_void,
    pub pExtra: *mut ::core::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_pcache_methods2 {
    pub iVersion: ::core::ffi::c_int,
    pub pArg: *mut ::core::ffi::c_void,
    pub xInit: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub xShutdown: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub xCreate: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> *mut crate::src::headers::sqlite3_h::sqlite3_pcache,
    >,
    pub xCachesize: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_pcache, ::core::ffi::c_int) -> (),
    >,
    pub xPagecount:
        Option<unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_pcache) -> ::core::ffi::c_int>,
    pub xFetch: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_pcache,
            ::core::ffi::c_uint,
            ::core::ffi::c_int,
        ) -> *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page,
    >,
    pub xUnpin: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_pcache,
            *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub xRekey: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_pcache,
            *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page,
            ::core::ffi::c_uint,
            ::core::ffi::c_uint,
        ) -> (),
    >,
    pub xTruncate: Option<
        unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_pcache, ::core::ffi::c_uint) -> (),
    >,
    pub xDestroy: Option<unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_pcache) -> ()>,
    pub xShrink: Option<unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_pcache) -> ()>,
}

pub const SQLITE_CHECKPOINT_NOOP: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);

pub const SQLITE_CHECKPOINT_PASSIVE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SQLITE_CHECKPOINT_FULL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_CHECKPOINT_RESTART: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_CHECKPOINT_TRUNCATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const SQLITE_VTAB_CONSTRAINT_SUPPORT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_VTAB_CONSTRAINT_SUPPORT_1: ::core::ffi::c_int = 1;

pub const SQLITE_VTAB_INNOCUOUS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_VTAB_INNOCUOUS_1: ::core::ffi::c_int = 2;

pub const SQLITE_VTAB_DIRECTONLY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const SQLITE_VTAB_DIRECTONLY_1: ::core::ffi::c_int = 3;

pub const SQLITE_VTAB_USES_ALL_SCHEMAS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SQLITE_VTAB_USES_ALL_SCHEMAS_1: ::core::ffi::c_int = 4;

pub const SQLITE_ROLLBACK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_FAIL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const SQLITE_REPLACE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const SQLITE_SERIALIZE_NOCOPY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const SQLITE_DESERIALIZE_FREEONCLOSE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const SQLITE_DESERIALIZE_RESIZEABLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const SQLITE_DESERIALIZE_READONLY: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const CARRAY_INT32: ::core::ffi::c_int = 0;

pub const CARRAY_INT64: ::core::ffi::c_int = 1;

pub const CARRAY_DOUBLE: ::core::ffi::c_int = 2;

pub const CARRAY_TEXT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const CARRAY_BLOB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub type sqlite3_rtree_dbl = ::core::ffi::c_double;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_rtree_geometry {
    pub pContext: *mut ::core::ffi::c_void,
    pub nParam: ::core::ffi::c_int,
    pub aParam: *mut crate::src::headers::sqlite3_h::sqlite3_rtree_dbl,
    pub pUser: *mut ::core::ffi::c_void,
    pub xDelUser: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_rtree_query_info {
    pub pContext: *mut ::core::ffi::c_void,
    pub nParam: ::core::ffi::c_int,
    pub aParam: *mut crate::src::headers::sqlite3_h::sqlite3_rtree_dbl,
    pub pUser: *mut ::core::ffi::c_void,
    pub xDelUser: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub aCoord: *mut crate::src::headers::sqlite3_h::sqlite3_rtree_dbl,
    pub anQueue: *mut ::core::ffi::c_uint,
    pub nCoord: ::core::ffi::c_int,
    pub iLevel: ::core::ffi::c_int,
    pub mxLevel: ::core::ffi::c_int,
    pub iRowid: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub rParentScore: crate::src::headers::sqlite3_h::sqlite3_rtree_dbl,
    pub eParentWithin: ::core::ffi::c_int,
    pub eWithin: ::core::ffi::c_int,
    pub rScore: crate::src::headers::sqlite3_h::sqlite3_rtree_dbl,
    pub apSqlParam: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
}

pub const NOT_WITHIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const PARTLY_WITHIN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const FULLY_WITHIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
