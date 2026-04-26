pub use crate::__stddef_size_t_h::SizeT;

pub use crate::src::ext::rtree::rtree::I64_0;
pub use crate::src::ext::rtree::rtree::U8_0;
pub use crate::src::headers::sqlite3_h::SQLITE_IOERR;
pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_SHORT_READ_1;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SqliteInt64;
pub use crate::src::headers::sqlite3_h::sqlite3_file;
pub use crate::src::headers::sqlite3_h::Sqlite3Filename;
pub use crate::src::headers::sqlite3_h::Sqlite3Int64;
pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;
pub use crate::src::headers::sqlite3_h::Sqlite3SyscallPtr;
pub use crate::src::headers::sqlite3_h::sqlite3_vfs;
pub use crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
pub use crate::src::headers::stdlib::Uint8T;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3_malloc;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemJournal {
    pub pMethod: *const crate::src::headers::sqlite3_h::sqlite3_io_methods,
    pub nChunkSize: ::core::ffi::c_int,
    pub nSpill: ::core::ffi::c_int,
    pub pFirst: *mut FileChunk,
    pub endpoint: FilePoint,
    pub readpoint: FilePoint,
    pub flags: ::core::ffi::c_int,
    pub pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    pub zJournal: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FilePoint {
    pub iOffset: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub pChunk: *mut FileChunk,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileChunk {
    pub pNext: *mut FileChunk,
    pub zChunk: [crate::src::ext::rtree::rtree::U8_0; 8],
}

pub const MEMJOURNAL_DFLT_FILECHUNKSIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

unsafe extern "C" fn memjrnlRead(
    pJfd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    zBuf: *mut ::core::ffi::c_void,
    iAmt: ::core::ffi::c_int,
    iOfst: crate::src::headers::sqlite3_h::SqliteInt64,
) -> ::core::ffi::c_int {
    let p: *mut MemJournal = pJfd as *mut MemJournal;
    let mut zOut: *mut crate::src::ext::rtree::rtree::U8_0 =
        zBuf as *mut crate::src::ext::rtree::rtree::U8_0;
    let mut nRead: ::core::ffi::c_int = iAmt;
    let mut iChunkOffset: ::core::ffi::c_int;
    let mut pChunk: *mut FileChunk;
    let __p_ref = { &mut *p };
    if iAmt as crate::src::headers::sqlite3_h::SqliteInt64 + iOfst > __p_ref.endpoint.iOffset {
        return crate::src::headers::sqlite3_h::SQLITE_IOERR_SHORT_READ_1;
    }
    if __p_ref.readpoint.iOffset != iOfst
        || iOfst == 0 as crate::src::headers::sqlite3_h::SqliteInt64
    {
        let mut iOff: crate::src::headers::sqlite3_h::Sqlite3Int64 =
            0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        pChunk = __p_ref.pFirst;
        while !pChunk.is_null()
            && iOff + __p_ref.nChunkSize as crate::src::headers::sqlite3_h::Sqlite3Int64 <= iOfst
        {
            iOff += __p_ref.nChunkSize as crate::src::headers::sqlite3_h::Sqlite3Int64;
            pChunk = (*pChunk).pNext;
        }
    } else {
        pChunk = __p_ref.readpoint.pChunk;
    }
    iChunkOffset = (iOfst % __p_ref.nChunkSize as crate::src::headers::sqlite3_h::SqliteInt64)
        as ::core::ffi::c_int;
    loop {
        let iSpace: ::core::ffi::c_int = __p_ref.nChunkSize - iChunkOffset;
        let nCopy: ::core::ffi::c_int = if nRead < __p_ref.nChunkSize - iChunkOffset {
            nRead
        } else {
            __p_ref.nChunkSize - iChunkOffset
        };
        ::core::ptr::copy_nonoverlapping(
            (&raw mut (*pChunk).zChunk as *mut crate::src::ext::rtree::rtree::U8_0)
                .offset(iChunkOffset as isize) as *const u8,
            zOut as *mut u8,
            nCopy as usize,
        );
        zOut = zOut.offset(nCopy as isize);
        nRead -= iSpace;
        iChunkOffset = 0 as ::core::ffi::c_int;
        if !(nRead >= 0 as ::core::ffi::c_int
            && {
                pChunk = (*pChunk).pNext;
                !pChunk.is_null()
            }
            && nRead > 0 as ::core::ffi::c_int)
        {
            break;
        }
    }
    __p_ref.readpoint.iOffset = (if !pChunk.is_null() {
        iOfst + iAmt as crate::src::headers::sqlite3_h::SqliteInt64
    } else {
        0 as crate::src::headers::sqlite3_h::SqliteInt64
    }) as crate::src::headers::sqlite3_h::Sqlite3Int64;
    __p_ref.readpoint.pChunk = pChunk;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn memjrnlFreeChunks(pFirst: *mut FileChunk) {
    let mut pIter: *mut FileChunk;
    let mut pNext: *mut FileChunk;
    pIter = pFirst;
    while !pIter.is_null() {
        pNext = (*pIter).pNext;
        crate::src::src::malloc::sqlite3_free(pIter as *mut ::core::ffi::c_void);
        pIter = pNext;
    }
}

unsafe extern "C" fn memjrnlCreateFile(p: *mut MemJournal) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let pReal: *mut crate::src::headers::sqlite3_h::sqlite3_file =
        p as *mut crate::src::headers::sqlite3_h::sqlite3_file;
    let copy: MemJournal = *p;
    ::libc::memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MemJournal>() as crate::__stddef_size_t_h::SizeT,
    );
    rc = crate::src::src::os::sqlite3OsOpen(
        copy.pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
        copy.zJournal,
        pReal as *mut crate::src::headers::sqlite3_h::sqlite3_file,
        copy.flags,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut nChunk: ::core::ffi::c_int = copy.nChunkSize;
        let mut iOff: crate::src::ext::rtree::rtree::I64_0 =
            0 as crate::src::ext::rtree::rtree::I64_0;
        let mut pIter: *mut FileChunk;
        pIter = copy.pFirst;
        while !pIter.is_null() {
            if iOff + nChunk as crate::src::ext::rtree::rtree::I64_0 > copy.endpoint.iOffset {
                nChunk = (copy.endpoint.iOffset as crate::src::ext::rtree::rtree::I64_0 - iOff)
                    as ::core::ffi::c_int;
            }
            rc = crate::src::src::os::sqlite3OsWrite(
                pReal as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                &raw mut (*pIter).zChunk as *mut crate::src::ext::rtree::rtree::U8_0
                    as *const ::core::ffi::c_void,
                nChunk,
                iOff,
            );
            if rc != 0 {
                break;
            }
            iOff += nChunk as crate::src::ext::rtree::rtree::I64_0;
            pIter = (*pIter).pNext;
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            memjrnlFreeChunks(copy.pFirst);
        }
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::os::sqlite3OsClose(
            pReal as *mut crate::src::headers::sqlite3_h::sqlite3_file,
        );
        *p = copy;
    }
    rc
}

unsafe extern "C" fn memjrnlWrite(
    pJfd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    zBuf: *const ::core::ffi::c_void,
    iAmt: ::core::ffi::c_int,
    iOfst: crate::src::headers::sqlite3_h::SqliteInt64,
) -> ::core::ffi::c_int {
    let p: *mut MemJournal = pJfd as *mut MemJournal;
    let mut nWrite: ::core::ffi::c_int = iAmt;
    let mut zWrite: *mut crate::src::ext::rtree::rtree::U8_0 =
        zBuf as *mut crate::src::ext::rtree::rtree::U8_0;
    if (*p).nSpill > 0 as ::core::ffi::c_int
        && iAmt as crate::src::headers::sqlite3_h::SqliteInt64 + iOfst
            > (*p).nSpill as crate::src::headers::sqlite3_h::SqliteInt64
    {
        let mut rc: ::core::ffi::c_int = memjrnlCreateFile(p);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = crate::src::src::os::sqlite3OsWrite(
                pJfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
                zBuf,
                iAmt,
                iOfst as crate::src::ext::rtree::rtree::I64_0,
            );
        }
        return rc;
    } else {
        if iOfst > 0 as crate::src::headers::sqlite3_h::SqliteInt64
            && iOfst != (*p).endpoint.iOffset
        {
            memjrnlTruncate(pJfd, iOfst);
        }
        if iOfst == 0 as crate::src::headers::sqlite3_h::SqliteInt64 && !(*p).pFirst.is_null() {
            ::libc::memcpy(
                &raw mut (*(*p).pFirst).zChunk as *mut crate::src::ext::rtree::rtree::U8_0
                    as *mut ::core::ffi::c_void,
                zBuf,
                iAmt as crate::__stddef_size_t_h::SizeT,
            );
        } else {
            while nWrite > 0 as ::core::ffi::c_int {
                let __p_ref = { &mut *p };
                let mut pChunk: *mut FileChunk = __p_ref.endpoint.pChunk;
                let iChunkOffset: ::core::ffi::c_int = (__p_ref.endpoint.iOffset
                    % __p_ref.nChunkSize as crate::src::headers::sqlite3_h::Sqlite3Int64)
                    as ::core::ffi::c_int;
                let iSpace: ::core::ffi::c_int = if nWrite < __p_ref.nChunkSize - iChunkOffset {
                    nWrite
                } else {
                    __p_ref.nChunkSize - iChunkOffset
                };
                if iChunkOffset == 0 as ::core::ffi::c_int {
                    let pNew: *mut FileChunk = crate::src::src::malloc::sqlite3_malloc(
                        (::core::mem::size_of::<FileChunk>() as usize)
                            .wrapping_add((__p_ref.nChunkSize - 8 as ::core::ffi::c_int) as usize)
                            as ::core::ffi::c_int,
                    ) as *mut FileChunk;
                    if pNew.is_null() {
                        return crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
                    }
                    (*pNew).pNext = ::core::ptr::null_mut::<FileChunk>();
                    if !pChunk.is_null() {
                        (*pChunk).pNext = pNew;
                    } else {
                        __p_ref.pFirst = pNew;
                    }
                    __p_ref.endpoint.pChunk = pNew;
                    pChunk = __p_ref.endpoint.pChunk;
                }
                ::core::ptr::copy_nonoverlapping(
                    zWrite as *const u8,
                    (&raw mut (*pChunk).zChunk as *mut crate::src::ext::rtree::rtree::U8_0)
                        .offset(iChunkOffset as isize) as *mut u8,
                    iSpace as usize,
                );
                zWrite = zWrite.offset(iSpace as isize);
                nWrite -= iSpace;
                __p_ref.endpoint.iOffset += iSpace as crate::src::headers::sqlite3_h::Sqlite3Int64;
            }
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn memjrnlTruncate(
    pJfd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    size: crate::src::headers::sqlite3_h::SqliteInt64,
) -> ::core::ffi::c_int {
    let p: *mut MemJournal = pJfd as *mut MemJournal;
    if size < (*p).endpoint.iOffset {
        let mut pIter: *mut FileChunk = ::core::ptr::null_mut::<FileChunk>();
        let __p_ref = { &mut *p };
        if size == 0 as crate::src::headers::sqlite3_h::SqliteInt64 {
            memjrnlFreeChunks(__p_ref.pFirst);
            __p_ref.pFirst = ::core::ptr::null_mut::<FileChunk>();
        } else {
            let mut iOff: crate::src::ext::rtree::rtree::I64_0 =
                __p_ref.nChunkSize as crate::src::ext::rtree::rtree::I64_0;
            pIter = __p_ref.pFirst;
            while !pIter.is_null() && iOff < size {
                iOff += __p_ref.nChunkSize as crate::src::ext::rtree::rtree::I64_0;
                pIter = (*pIter).pNext;
            }
            if !pIter.is_null() {
                memjrnlFreeChunks((*pIter).pNext);
                (*pIter).pNext = ::core::ptr::null_mut::<FileChunk>();
            }
        }
        __p_ref.endpoint.pChunk = pIter;
        __p_ref.endpoint.iOffset = size as crate::src::headers::sqlite3_h::Sqlite3Int64;
        __p_ref.readpoint.pChunk = ::core::ptr::null_mut::<FileChunk>();
        __p_ref.readpoint.iOffset = 0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn memjrnlClose(
    pJfd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
) -> ::core::ffi::c_int {
    let p: *mut MemJournal = pJfd as *mut MemJournal;
    memjrnlFreeChunks((*p).pFirst);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn memjrnlSync(
    mut _pJfd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut _flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn memjrnlFileSize(
    pJfd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    pSize: *mut crate::src::headers::sqlite3_h::SqliteInt64,
) -> ::core::ffi::c_int {
    let p = &*(pJfd as *mut MemJournal);
    *pSize = p.endpoint.iOffset;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

static mut MemJournalMethods: crate::src::headers::sqlite3_h::sqlite3_io_methods = {
    crate::src::headers::sqlite3_h::sqlite3_io_methods {
        iVersion: 1 as ::core::ffi::c_int,
        xClose: Some(
            memjrnlClose
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                ) -> ::core::ffi::c_int,
        ),
        xRead: Some(
            memjrnlRead
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    crate::src::headers::sqlite3_h::SqliteInt64,
                ) -> ::core::ffi::c_int,
        ),
        xWrite: Some(
            memjrnlWrite
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    crate::src::headers::sqlite3_h::SqliteInt64,
                ) -> ::core::ffi::c_int,
        ),
        xTruncate: Some(
            memjrnlTruncate
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    crate::src::headers::sqlite3_h::SqliteInt64,
                ) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            memjrnlSync
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileSize: Some(
            memjrnlFileSize
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    *mut crate::src::headers::sqlite3_h::SqliteInt64,
                ) -> ::core::ffi::c_int,
        ),
        xLock: None,
        xUnlock: None,
        xCheckReservedLock: None,
        xFileControl: None,
        xSectorSize: None,
        xDeviceCharacteristics: None,
        xShmMap: None,
        xShmLock: None,
        xShmBarrier: None,
        xShmUnmap: None,
        xFetch: None,
        xUnfetch: None,
    }
};
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3JournalOpen(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    zName: *const ::core::ffi::c_char,
    pJfd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    flags: ::core::ffi::c_int,
    nSpill: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let p: *mut MemJournal = pJfd as *mut MemJournal;
    ::libc::memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MemJournal>() as crate::__stddef_size_t_h::SizeT,
    );
    if nSpill == 0 as ::core::ffi::c_int {
        return crate::src::src::os::sqlite3OsOpen(
            pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            zName,
            pJfd as *mut crate::src::headers::sqlite3_h::sqlite3_file,
            flags,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
    }
    let __p_ref = { &mut *p };
    if nSpill > 0 as ::core::ffi::c_int {
        __p_ref.nChunkSize = nSpill;
    } else {
        __p_ref.nChunkSize = ((8 as ::core::ffi::c_int + MEMJOURNAL_DFLT_FILECHUNKSIZE) as usize)
            .wrapping_sub(::core::mem::size_of::<FileChunk>() as usize)
            as ::core::ffi::c_int;
    }
    (*pJfd).pMethods = &raw const MemJournalMethods
        as *const crate::src::headers::sqlite3_h::sqlite3_io_methods
        as *const crate::src::headers::sqlite3_h::sqlite3_io_methods;
    __p_ref.nSpill = nSpill;
    __p_ref.flags = flags;
    __p_ref.zJournal = zName;
    __p_ref.pVfs = pVfs;
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3MemJournalOpen(
    pJfd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
) {
    sqlite3JournalOpen(
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vfs>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        pJfd,
        0 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
    );
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3JournalIsInMemory(
    p: *mut crate::src::headers::sqlite3_h::sqlite3_file,
) -> ::core::ffi::c_int {
    ((*p).pMethods == &raw const MemJournalMethods) as ::core::ffi::c_int
}
pub unsafe extern "C" fn sqlite3JournalSize(
    pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
) -> ::core::ffi::c_int {
    if (*pVfs).szOsFile > ::core::mem::size_of::<MemJournal>() as ::core::ffi::c_int {
        (*pVfs).szOsFile
    } else {
        ::core::mem::size_of::<MemJournal>() as ::core::ffi::c_int
    }
}
