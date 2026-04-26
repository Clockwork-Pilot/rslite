// =============== BEGIN os_h ================
pub const SQLITE_MAX_PATHLEN: ::core::ffi::c_int = crate::src::headers::stdlib::FILENAME_MAX;

pub const SQLITE_MAX_SYMLINK: ::core::ffi::c_int = 200 as ::core::ffi::c_int;

pub const SQLITE_DEFAULT_SECTOR_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;

pub const NO_LOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const SHARED_LOCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const RESERVED_LOCK: ::core::ffi::c_int = 2;

pub const RESERVED_LOCK_1: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const PENDING_LOCK: ::core::ffi::c_int = 3;

pub const EXCLUSIVE_LOCK: ::core::ffi::c_int = 4;

pub const EXCLUSIVE_LOCK_1: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const SHARED_SIZE: ::core::ffi::c_int = 510 as ::core::ffi::c_int;
pub use crate::__stddef_size_t_h::SizeT;


pub use crate::src::ext::rtree::rtree::I64_0;
pub use crate::src::ext::rtree::rtree::U8_0;
pub use crate::src::ext::rtree::rtree::U32_0;
pub use crate::src::ext::rtree::rtree::U64_0;
pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_CKPT_DONE;
pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_CKPT_START;
pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_COMMIT_PHASETWO;
pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_LOCK_TIMEOUT;
pub use crate::src::headers::sqlite3_h::SQLITE_IOERR;
pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_MUTEX_STATIC_MAIN;
pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_NOTFOUND;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SqliteInt64;
pub use crate::src::headers::sqlite3_h::SqliteUint64;
pub use crate::src::headers::sqlite3_h::sqlite3_file;
pub use crate::src::headers::sqlite3_h::Sqlite3Filename;
pub use crate::src::headers::sqlite3_h::Sqlite3Int64;
pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;
pub use crate::src::headers::sqlite3_h::sqlite3_mem_methods;
pub use crate::src::headers::sqlite3_h::sqlite3_mutex_methods;
pub use crate::src::headers::sqlite3_h::Sqlite3Pcache;
pub use crate::src::headers::sqlite3_h::sqlite3_pcache_methods2;
pub use crate::src::headers::sqlite3_h::sqlite3_pcache_page;
pub use crate::src::headers::sqlite3_h::Sqlite3SyscallPtr;
pub use crate::src::headers::sqlite3_h::sqlite3_vfs;
pub use crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
pub use crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
pub use crate::src::headers::sqliteInt_h::Sqlite3Config;
pub use crate::src::headers::stdlib::Uint8T;
pub use crate::src::headers::stdlib::Uint32T;
pub use crate::src::src::global::sqlite3Config;
pub use crate::src::src::main::sqlite3_initialize;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3_malloc;
pub use crate::src::src::malloc::sqlite3Malloc;
pub use crate::src::src::malloc::sqlite3MallocZero;
pub use crate::src::src::memjournal::sqlite3JournalIsInMemory;
pub use crate::src::src::mutex::sqlite3_mutex_enter;
pub use crate::src::src::mutex::sqlite3_mutex_leave;
pub use crate::src::src::mutex::sqlite3MutexAlloc;
pub use crate::src::src::mutex_unix::sqlite3_mutex;
pub use crate::src::src::os_unix::sqlite3_os_init;
#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub static mut sqlite3_io_error_hit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub static mut sqlite3_io_error_hardhit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub static mut sqlite3_io_error_pending: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub static mut sqlite3_io_error_persist: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub static mut sqlite3_io_error_benign: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub static mut sqlite3_diskfull_pending: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub static mut sqlite3_diskfull: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub static mut sqlite3_open_file_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub static mut sqlite3_memdebug_vfs_oom_test: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3OsClose(
    pId: *mut crate::src::headers::sqlite3_h::sqlite3_file,
) {
    if !(*pId).pMethods.is_null() {
        (*(*pId).pMethods)
            .xClose
            .expect("non-null function pointer")(pId);
        (*pId).pMethods = ::core::ptr::null::<crate::src::headers::sqlite3_h::sqlite3_io_methods>();
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsRead(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    pBuf: *mut ::core::ffi::c_void,
    amt: ::core::ffi::c_int,
    offset: crate::src::ext::rtree::rtree::I64_0,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::U64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    (*(*id).pMethods).xRead.expect("non-null function pointer")(
        id,
        pBuf,
        amt,
        offset as crate::src::headers::sqlite3_h::Sqlite3Int64,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsWrite(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    pBuf: *const ::core::ffi::c_void,
    amt: ::core::ffi::c_int,
    offset: crate::src::ext::rtree::rtree::I64_0,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::U64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    (*(*id).pMethods).xWrite.expect("non-null function pointer")(
        id,
        pBuf,
        amt,
        offset as crate::src::headers::sqlite3_h::Sqlite3Int64,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsTruncate(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    size: crate::src::ext::rtree::rtree::I64_0,
) -> ::core::ffi::c_int {
    (*(*id).pMethods)
        .xTruncate
        .expect("non-null function pointer")(
        id,
        size as crate::src::headers::sqlite3_h::Sqlite3Int64,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsSync(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::U64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    if flags != 0 {
        (*(*id).pMethods).xSync.expect("non-null function pointer")(id, flags)
    } else {
        crate::src::headers::sqlite3_h::SQLITE_OK
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsFileSize(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    pSize: *mut crate::src::ext::rtree::rtree::I64_0,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::U64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    (*(*id).pMethods)
        .xFileSize
        .expect("non-null function pointer")(
        id,
        pSize as *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsLock(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    lockType: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::U64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    (*(*id).pMethods).xLock.expect("non-null function pointer")(id, lockType)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsUnlock(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    lockType: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*(*id).pMethods)
        .xUnlock
        .expect("non-null function pointer")(id, lockType)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsCheckReservedLock(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::U64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    (*(*id).pMethods)
        .xCheckReservedLock
        .expect("non-null function pointer")(id, pResOut)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsFileControl(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    op: ::core::ffi::c_int,
    pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if (*id).pMethods.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOTFOUND;
    }
    if op != crate::src::headers::sqlite3_h::SQLITE_FCNTL_COMMIT_PHASETWO
        && op != crate::src::headers::sqlite3_h::SQLITE_FCNTL_LOCK_TIMEOUT
        && op != crate::src::headers::sqlite3_h::SQLITE_FCNTL_CKPT_DONE
        && op != crate::src::headers::sqlite3_h::SQLITE_FCNTL_CKPT_START
    {
        if sqlite3_memdebug_vfs_oom_test != 0
            && (id.is_null()
                || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                    id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                ) == 0)
        {
            let pTstAlloc: *mut ::core::ffi::c_void =
                crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::U64_0);
            if pTstAlloc.is_null() {
                return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
            }
            crate::src::src::malloc::sqlite3_free(pTstAlloc);
        }
    }
    (*(*id).pMethods)
        .xFileControl
        .expect("non-null function pointer")(id, op, pArg)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsFileControlHint(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    op: ::core::ffi::c_int,
    pArg: *mut ::core::ffi::c_void,
) {
    if !(*id).pMethods.is_null() {
        (*(*id).pMethods)
            .xFileControl
            .expect("non-null function pointer")(id, op, pArg);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsSectorSize(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
) -> ::core::ffi::c_int {
    let xSectorSize: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqlite3_h::sqlite3_file,
        ) -> ::core::ffi::c_int,
    > = (*(*id).pMethods).xSectorSize;
    if xSectorSize.is_some() {
        xSectorSize.expect("non-null function pointer")(id)
    } else {
        crate::src::src::os::SQLITE_DEFAULT_SECTOR_SIZE
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsDeviceCharacteristics(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
) -> ::core::ffi::c_int {
    if (*id).pMethods.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*(*id).pMethods)
        .xDeviceCharacteristics
        .expect("non-null function pointer")(id)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsShmLock(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    offset: ::core::ffi::c_int,
    n: ::core::ffi::c_int,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*(*id).pMethods)
        .xShmLock
        .expect("non-null function pointer")(id, offset, n, flags)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsShmBarrier(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
) {
    (*(*id).pMethods)
        .xShmBarrier
        .expect("non-null function pointer")(id);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsShmUnmap(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    deleteFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*(*id).pMethods)
        .xShmUnmap
        .expect("non-null function pointer")(id, deleteFlag)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsShmMap(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    iPage: ::core::ffi::c_int,
    pgsz: ::core::ffi::c_int,
    bExtend: ::core::ffi::c_int,
    pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::U64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    (*(*id).pMethods)
        .xShmMap
        .expect("non-null function pointer")(id, iPage, pgsz, bExtend, pp)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsFetch(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    iOff: crate::src::ext::rtree::rtree::I64_0,
    iAmt: ::core::ffi::c_int,
    pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::U64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    (*(*id).pMethods).xFetch.expect("non-null function pointer")(
        id,
        iOff as crate::src::headers::sqlite3_h::Sqlite3Int64,
        iAmt,
        pp,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsUnfetch(
    id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    iOff: crate::src::ext::rtree::rtree::I64_0,
    p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    (*(*id).pMethods)
        .xUnfetch
        .expect("non-null function pointer")(
        id,
        iOff as crate::src::headers::sqlite3_h::Sqlite3Int64,
        p,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsOpen(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    zPath: *const ::core::ffi::c_char,
    pFile: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    flags: ::core::ffi::c_int,
    pFlagsOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    if sqlite3_memdebug_vfs_oom_test != 0
        && (0 as ::core::ffi::c_int == 0
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(::core::ptr::null_mut::<
                crate::src::headers::sqlite3_h::sqlite3_file,
            >()
                as *mut crate::src::headers::sqlite3_h::sqlite3_file)
                == 0)
    {
        let pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::U64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    let rc: ::core::ffi::c_int = (*pVfs).xOpen.expect("non-null function pointer")(
        pVfs,
        zPath as crate::src::headers::sqlite3_h::Sqlite3Filename,
        pFile,
        flags & 0x1087f7f as ::core::ffi::c_int,
        pFlagsOut,
    );
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsDelete(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    zPath: *const ::core::ffi::c_char,
    dirSync: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (0 as ::core::ffi::c_int == 0
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(::core::ptr::null_mut::<
                crate::src::headers::sqlite3_h::sqlite3_file,
            >()
                as *mut crate::src::headers::sqlite3_h::sqlite3_file)
                == 0)
    {
        let pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::U64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    if (*pVfs).xDelete.is_some() {
        (*pVfs).xDelete.expect("non-null function pointer")(pVfs, zPath, dirSync)
    } else {
        crate::src::headers::sqlite3_h::SQLITE_OK
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsAccess(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    zPath: *const ::core::ffi::c_char,
    flags: ::core::ffi::c_int,
    pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (0 as ::core::ffi::c_int == 0
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(::core::ptr::null_mut::<
                crate::src::headers::sqlite3_h::sqlite3_file,
            >()
                as *mut crate::src::headers::sqlite3_h::sqlite3_file)
                == 0)
    {
        let pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::U64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    (*pVfs).xAccess.expect("non-null function pointer")(pVfs, zPath, flags, pResOut)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsFullPathname(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    zPath: *const ::core::ffi::c_char,
    nPathOut: ::core::ffi::c_int,
    zPathOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (0 as ::core::ffi::c_int == 0
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(::core::ptr::null_mut::<
                crate::src::headers::sqlite3_h::sqlite3_file,
            >()
                as *mut crate::src::headers::sqlite3_h::sqlite3_file)
                == 0)
    {
        let pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::U64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    *zPathOut.offset(0_isize) = 0 as ::core::ffi::c_char;
    (*pVfs).xFullPathname.expect("non-null function pointer")(pVfs, zPath, nPathOut, zPathOut)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsDlOpen(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    zPath: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    (*pVfs).xDlOpen.expect("non-null function pointer")(pVfs, zPath)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsDlError(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    nByte: ::core::ffi::c_int,
    zBufOut: *mut ::core::ffi::c_char,
) {
    (*pVfs).xDlError.expect("non-null function pointer")(pVfs, nByte, zBufOut);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsDlSym(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    pHdle: *mut ::core::ffi::c_void,
    zSym: *const ::core::ffi::c_char,
) -> Option<unsafe extern "C" fn() -> ()> {
    (*pVfs).xDlSym.expect("non-null function pointer")(pVfs, pHdle, zSym)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsDlClose(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    pHandle: *mut ::core::ffi::c_void,
) {
    (*pVfs).xDlClose.expect("non-null function pointer")(pVfs, pHandle);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsRandomness(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    zBufOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if crate::src::src::global::sqlite3Config.iPrngSeed != 0 {
        ::libc::memset(
            zBufOut as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            nByte as crate::__stddef_size_t_h::SizeT,
        );
        if nByte > ::core::mem::size_of::<::core::ffi::c_uint>() as ::core::ffi::c_int {
            nByte = ::core::mem::size_of::<::core::ffi::c_uint>() as ::core::ffi::c_int;
        }
        ::core::ptr::copy_nonoverlapping(
            &raw mut crate::src::src::global::sqlite3Config.iPrngSeed as *const u8,
            zBufOut as *mut u8,
            nByte as usize,
        );
        crate::src::headers::sqlite3_h::SQLITE_OK
    } else {
        (*pVfs).xRandomness.expect("non-null function pointer")(pVfs, nByte, zBufOut)
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsSleep(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    nMicro: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*pVfs).xSleep.expect("non-null function pointer")(pVfs, nMicro)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsGetLastError(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
) -> ::core::ffi::c_int {
    if (*pVfs).xGetLastError.is_some() {
        (*pVfs).xGetLastError.expect("non-null function pointer")(
            pVfs,
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        )
    } else {
        0 as ::core::ffi::c_int
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsCurrentTimeInt64(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    pTimeOut: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int;
    if (*pVfs).iVersion >= 2 as ::core::ffi::c_int && (*pVfs).xCurrentTimeInt64.is_some() {
        rc = (*pVfs)
            .xCurrentTimeInt64
            .expect("non-null function pointer")(pVfs, pTimeOut);
    } else {
        let mut r: ::core::ffi::c_double = 0.;
        rc = (*pVfs).xCurrentTime.expect("non-null function pointer")(pVfs, &raw mut r);
        *pTimeOut = (r * 86400000.0f64) as crate::src::headers::sqlite3_h::Sqlite3Int64;
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsOpenMalloc(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    zFile: *const ::core::ffi::c_char,
    ppFile: *mut *mut crate::src::headers::sqlite3_h::sqlite3_file,
    flags: ::core::ffi::c_int,
    pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int;
    
    let pFile: *mut crate::src::headers::sqlite3_h::sqlite3_file = crate::src::src::malloc::sqlite3MallocZero(
        (*pVfs).szOsFile as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::headers::sqlite3_h::sqlite3_file;
    if !pFile.is_null() {
        rc = sqlite3OsOpen(pVfs, zFile, pFile, flags, pOutFlags);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::malloc::sqlite3_free(pFile as *mut ::core::ffi::c_void);
            *ppFile = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_file>();
        } else {
            *ppFile = pFile;
        }
    } else {
        *ppFile = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_file>();
        rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsCloseFree(
    pFile: *mut crate::src::headers::sqlite3_h::sqlite3_file,
) {
    sqlite3OsClose(pFile);
    crate::src::src::malloc::sqlite3_free(pFile as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn sqlite3OsInit() -> ::core::ffi::c_int {
    let p: *mut ::core::ffi::c_void =
        crate::src::src::malloc::sqlite3_malloc(10 as ::core::ffi::c_int);
    if p.is_null() {
        return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
    }
    crate::src::src::malloc::sqlite3_free(p);
    crate::src::src::os_unix::sqlite3_os_init()
}

static mut vfsList: *mut crate::src::headers::sqlite3_h::sqlite3_vfs =
    ::core::ptr::null::<crate::src::headers::sqlite3_h::sqlite3_vfs>()
        as *mut crate::src::headers::sqlite3_h::sqlite3_vfs;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_vfs_find(
    zVfs: *const ::core::ffi::c_char,
) -> *mut crate::src::headers::sqlite3_h::sqlite3_vfs {
    let mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs;
    
    let rc: ::core::ffi::c_int = crate::src::src::main::sqlite3_initialize();
    if rc != 0 {
        return ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vfs>();
    }
    let mutex: *mut crate::src::src::mutex_unix::sqlite3_mutex = crate::src::src::mutex::sqlite3MutexAlloc(
        crate::src::headers::sqlite3_h::SQLITE_MUTEX_STATIC_MAIN,
    );
    crate::src::src::mutex::sqlite3_mutex_enter(mutex);
    pVfs = vfsList;
    while !pVfs.is_null() {
        if zVfs.is_null() {
            break;
        }
        if ::libc::strcmp(zVfs, (*pVfs).zName) == 0 as ::core::ffi::c_int {
            break;
        }
        pVfs = (*pVfs).pNext;
    }
    crate::src::src::mutex::sqlite3_mutex_leave(mutex);
    pVfs
}

unsafe extern "C" fn vfsUnlink(pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs) {
    if !pVfs.is_null() {
        if vfsList == pVfs {
            vfsList = (*pVfs).pNext;
        } else if !vfsList.is_null() {
            let mut p: *mut crate::src::headers::sqlite3_h::sqlite3_vfs = vfsList;
            while !(*p).pNext.is_null() && (*p).pNext != pVfs {
                p = (*p).pNext;
            }
            if (*p).pNext == pVfs {
                (*p).pNext = (*pVfs).pNext;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_vfs_register(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    makeDflt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    let rc: ::core::ffi::c_int = crate::src::src::main::sqlite3_initialize();
    if rc != 0 {
        return rc;
    }
    let mutex: *mut crate::src::src::mutex_unix::sqlite3_mutex = crate::src::src::mutex::sqlite3MutexAlloc(2 as ::core::ffi::c_int);
    crate::src::src::mutex::sqlite3_mutex_enter(mutex);
    vfsUnlink(pVfs);
    if makeDflt != 0 || vfsList.is_null() {
        (*pVfs).pNext = vfsList;
        vfsList = pVfs;
    } else {
        (*pVfs).pNext = (*vfsList).pNext;
        (*vfsList).pNext = pVfs;
    }
    crate::src::src::mutex::sqlite3_mutex_leave(mutex);
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_vfs_unregister(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
) -> ::core::ffi::c_int {
    
    let rc: ::core::ffi::c_int = crate::src::src::main::sqlite3_initialize();
    if rc != 0 {
        return rc;
    }
    let mutex: *mut crate::src::src::mutex_unix::sqlite3_mutex = crate::src::src::mutex::sqlite3MutexAlloc(2 as ::core::ffi::c_int);
    crate::src::src::mutex::sqlite3_mutex_enter(mutex);
    vfsUnlink(pVfs);
    crate::src::src::mutex::sqlite3_mutex_leave(mutex);
    crate::src::headers::sqlite3_h::SQLITE_OK
}
