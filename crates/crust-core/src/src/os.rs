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
pub use crate::__stddef_size_t_h::size_t;


pub use crate::src::ext::rtree::rtree::i64_0;
pub use crate::src::ext::rtree::rtree::u8_0;
pub use crate::src::ext::rtree::rtree::u32_0;
pub use crate::src::ext::rtree::rtree::u64_0;
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
pub use crate::src::headers::sqlite3_h::sqlite_int64;
pub use crate::src::headers::sqlite3_h::sqlite_uint64;
pub use crate::src::headers::sqlite3_h::sqlite3_file;
pub use crate::src::headers::sqlite3_h::sqlite3_filename;
pub use crate::src::headers::sqlite3_h::sqlite3_int64;
pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;
pub use crate::src::headers::sqlite3_h::sqlite3_mem_methods;
pub use crate::src::headers::sqlite3_h::sqlite3_mutex_methods;
pub use crate::src::headers::sqlite3_h::sqlite3_pcache;
pub use crate::src::headers::sqlite3_h::sqlite3_pcache_methods2;
pub use crate::src::headers::sqlite3_h::sqlite3_pcache_page;
pub use crate::src::headers::sqlite3_h::sqlite3_syscall_ptr;
pub use crate::src::headers::sqlite3_h::sqlite3_vfs;
pub use crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
pub use crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
pub use crate::src::headers::sqliteInt_h::Sqlite3Config;
pub use crate::src::headers::stdlib::__uint8_t;
pub use crate::src::headers::stdlib::__uint32_t;
pub use crate::src::headers::stdlib::uint8_t;
pub use crate::src::headers::stdlib::uint32_t;
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
pub static mut sqlite3_io_error_hit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub static mut sqlite3_io_error_hardhit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub static mut sqlite3_io_error_pending: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub static mut sqlite3_io_error_persist: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub static mut sqlite3_io_error_benign: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub static mut sqlite3_diskfull_pending: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub static mut sqlite3_diskfull: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub static mut sqlite3_open_file_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub static mut sqlite3_memdebug_vfs_oom_test: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub unsafe extern "C" fn sqlite3OsClose(
    mut pId: *mut crate::src::headers::sqlite3_h::sqlite3_file,
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
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut pBuf: *mut ::core::ffi::c_void,
    mut amt: ::core::ffi::c_int,
    mut offset: crate::src::ext::rtree::rtree::i64_0,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let mut pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::u64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    (*(*id).pMethods).xRead.expect("non-null function pointer")(
        id,
        pBuf,
        amt,
        offset as crate::src::headers::sqlite3_h::sqlite3_int64,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsWrite(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut pBuf: *const ::core::ffi::c_void,
    mut amt: ::core::ffi::c_int,
    mut offset: crate::src::ext::rtree::rtree::i64_0,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let mut pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::u64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    (*(*id).pMethods).xWrite.expect("non-null function pointer")(
        id,
        pBuf,
        amt,
        offset as crate::src::headers::sqlite3_h::sqlite3_int64,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsTruncate(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut size: crate::src::ext::rtree::rtree::i64_0,
) -> ::core::ffi::c_int {
    (*(*id).pMethods)
        .xTruncate
        .expect("non-null function pointer")(
        id,
        size as crate::src::headers::sqlite3_h::sqlite3_int64,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsSync(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let mut pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::u64_0);
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
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut pSize: *mut crate::src::ext::rtree::rtree::i64_0,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let mut pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::u64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    (*(*id).pMethods)
        .xFileSize
        .expect("non-null function pointer")(
        id,
        pSize as *mut crate::src::headers::sqlite3_h::sqlite3_int64,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsLock(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut lockType: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let mut pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::u64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    (*(*id).pMethods).xLock.expect("non-null function pointer")(id, lockType)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsUnlock(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut lockType: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*(*id).pMethods)
        .xUnlock
        .expect("non-null function pointer")(id, lockType)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsCheckReservedLock(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let mut pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::u64_0);
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
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
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
            let mut pTstAlloc: *mut ::core::ffi::c_void =
                crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::u64_0);
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
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) {
    if !(*id).pMethods.is_null() {
        (*(*id).pMethods)
            .xFileControl
            .expect("non-null function pointer")(id, op, pArg);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsSectorSize(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
) -> ::core::ffi::c_int {
    let mut xSectorSize: Option<
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
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
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
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut offset: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*(*id).pMethods)
        .xShmLock
        .expect("non-null function pointer")(id, offset, n, flags)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsShmBarrier(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
) {
    (*(*id).pMethods)
        .xShmBarrier
        .expect("non-null function pointer")(id);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsShmUnmap(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut deleteFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*(*id).pMethods)
        .xShmUnmap
        .expect("non-null function pointer")(id, deleteFlag)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsShmMap(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut iPage: ::core::ffi::c_int,
    mut pgsz: ::core::ffi::c_int,
    mut bExtend: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let mut pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::u64_0);
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
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut iOff: crate::src::ext::rtree::rtree::i64_0,
    mut iAmt: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (id.is_null()
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(
                id as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            ) == 0)
    {
        let mut pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::u64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    (*(*id).pMethods).xFetch.expect("non-null function pointer")(
        id,
        iOff as crate::src::headers::sqlite3_h::sqlite3_int64,
        iAmt,
        pp,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsUnfetch(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut iOff: crate::src::ext::rtree::rtree::i64_0,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    (*(*id).pMethods)
        .xUnfetch
        .expect("non-null function pointer")(
        id,
        iOff as crate::src::headers::sqlite3_h::sqlite3_int64,
        p,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsOpen(
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut pFile: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pFlagsOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    if sqlite3_memdebug_vfs_oom_test != 0
        && (0 as ::core::ffi::c_int == 0
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(::core::ptr::null_mut::<
                crate::src::headers::sqlite3_h::sqlite3_file,
            >()
                as *mut crate::src::headers::sqlite3_h::sqlite3_file)
                == 0)
    {
        let mut pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::u64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    rc = (*pVfs).xOpen.expect("non-null function pointer")(
        pVfs,
        zPath as crate::src::headers::sqlite3_h::sqlite3_filename,
        pFile,
        flags & 0x1087f7f as ::core::ffi::c_int,
        pFlagsOut,
    );
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsDelete(
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut dirSync: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (0 as ::core::ffi::c_int == 0
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(::core::ptr::null_mut::<
                crate::src::headers::sqlite3_h::sqlite3_file,
            >()
                as *mut crate::src::headers::sqlite3_h::sqlite3_file)
                == 0)
    {
        let mut pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::u64_0);
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
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (0 as ::core::ffi::c_int == 0
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(::core::ptr::null_mut::<
                crate::src::headers::sqlite3_h::sqlite3_file,
            >()
                as *mut crate::src::headers::sqlite3_h::sqlite3_file)
                == 0)
    {
        let mut pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::u64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    (*pVfs).xAccess.expect("non-null function pointer")(pVfs, zPath, flags, pResOut)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsFullPathname(
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut nPathOut: ::core::ffi::c_int,
    mut zPathOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if sqlite3_memdebug_vfs_oom_test != 0
        && (0 as ::core::ffi::c_int == 0
            || crate::src::src::memjournal::sqlite3JournalIsInMemory(::core::ptr::null_mut::<
                crate::src::headers::sqlite3_h::sqlite3_file,
            >()
                as *mut crate::src::headers::sqlite3_h::sqlite3_file)
                == 0)
    {
        let mut pTstAlloc: *mut ::core::ffi::c_void =
            crate::src::src::malloc::sqlite3Malloc(10 as crate::src::ext::rtree::rtree::u64_0);
        if pTstAlloc.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
        }
        crate::src::src::malloc::sqlite3_free(pTstAlloc);
    }
    *zPathOut.offset(0 as isize) = 0 as ::core::ffi::c_char;
    (*pVfs).xFullPathname.expect("non-null function pointer")(pVfs, zPath, nPathOut, zPathOut)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsDlOpen(
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    (*pVfs).xDlOpen.expect("non-null function pointer")(pVfs, zPath)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsDlError(
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zBufOut: *mut ::core::ffi::c_char,
) {
    (*pVfs).xDlError.expect("non-null function pointer")(pVfs, nByte, zBufOut);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsDlSym(
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut pHdle: *mut ::core::ffi::c_void,
    mut zSym: *const ::core::ffi::c_char,
) -> Option<unsafe extern "C" fn() -> ()> {
    (*pVfs).xDlSym.expect("non-null function pointer")(pVfs, pHdle, zSym)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsDlClose(
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut pHandle: *mut ::core::ffi::c_void,
) {
    (*pVfs).xDlClose.expect("non-null function pointer")(pVfs, pHandle);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsRandomness(
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut nByte: ::core::ffi::c_int,
    mut zBufOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if crate::src::src::global::sqlite3Config.iPrngSeed != 0 {
        ::libc::memset(
            zBufOut as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            nByte as crate::__stddef_size_t_h::size_t,
        );
        if nByte > ::core::mem::size_of::<::core::ffi::c_uint>() as ::core::ffi::c_int {
            nByte = ::core::mem::size_of::<::core::ffi::c_uint>() as ::core::ffi::c_int;
        }
        ::core::ptr::copy_nonoverlapping(
            &raw mut crate::src::src::global::sqlite3Config.iPrngSeed as *const u8,
            zBufOut as *mut u8,
            nByte as usize,
        );
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    } else {
        return (*pVfs).xRandomness.expect("non-null function pointer")(pVfs, nByte, zBufOut);
    };
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsSleep(
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut nMicro: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*pVfs).xSleep.expect("non-null function pointer")(pVfs, nMicro)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsGetLastError(
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
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
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut pTimeOut: *mut crate::src::headers::sqlite3_h::sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    if (*pVfs).iVersion >= 2 as ::core::ffi::c_int && (*pVfs).xCurrentTimeInt64.is_some() {
        rc = (*pVfs)
            .xCurrentTimeInt64
            .expect("non-null function pointer")(pVfs, pTimeOut);
    } else {
        let mut r: ::core::ffi::c_double = 0.;
        rc = (*pVfs).xCurrentTime.expect("non-null function pointer")(pVfs, &raw mut r);
        *pTimeOut = (r * 86400000.0f64) as crate::src::headers::sqlite3_h::sqlite3_int64;
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3OsOpenMalloc(
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut zFile: *const ::core::ffi::c_char,
    mut ppFile: *mut *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pFile: *mut crate::src::headers::sqlite3_h::sqlite3_file =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_file>();
    pFile = crate::src::src::malloc::sqlite3MallocZero(
        (*pVfs).szOsFile as crate::src::ext::rtree::rtree::u64_0,
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
    mut pFile: *mut crate::src::headers::sqlite3_h::sqlite3_file,
) {
    sqlite3OsClose(pFile);
    crate::src::src::malloc::sqlite3_free(pFile as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn sqlite3OsInit() -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_void =
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
    mut zVfs: *const ::core::ffi::c_char,
) -> *mut crate::src::headers::sqlite3_h::sqlite3_vfs {
    let mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vfs>();
    let mut mutex: *mut crate::src::src::mutex_unix::sqlite3_mutex =
        ::core::ptr::null_mut::<crate::src::src::mutex_unix::sqlite3_mutex>();
    let mut rc: ::core::ffi::c_int = crate::src::src::main::sqlite3_initialize();
    if rc != 0 {
        return ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vfs>();
    }
    mutex = crate::src::src::mutex::sqlite3MutexAlloc(
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

unsafe extern "C" fn vfsUnlink(mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs) {
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
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut makeDflt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut mutex: *mut crate::src::src::mutex_unix::sqlite3_mutex =
        ::core::ptr::null_mut::<crate::src::src::mutex_unix::sqlite3_mutex>();
    let mut rc: ::core::ffi::c_int = crate::src::src::main::sqlite3_initialize();
    if rc != 0 {
        return rc;
    }
    mutex = crate::src::src::mutex::sqlite3MutexAlloc(2 as ::core::ffi::c_int);
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
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
) -> ::core::ffi::c_int {
    let mut mutex: *mut crate::src::src::mutex_unix::sqlite3_mutex =
        ::core::ptr::null_mut::<crate::src::src::mutex_unix::sqlite3_mutex>();
    let mut rc: ::core::ffi::c_int = crate::src::src::main::sqlite3_initialize();
    if rc != 0 {
        return rc;
    }
    mutex = crate::src::src::mutex::sqlite3MutexAlloc(2 as ::core::ffi::c_int);
    crate::src::src::mutex::sqlite3_mutex_enter(mutex);
    vfsUnlink(pVfs);
    crate::src::src::mutex::sqlite3_mutex_leave(mutex);
    crate::src::headers::sqlite3_h::SQLITE_OK
}
