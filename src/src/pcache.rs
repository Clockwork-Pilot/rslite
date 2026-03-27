







// =============== BEGIN pcache_h ================
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PgHdr {
    pub pPage:  *mut crate::sqlite3_h::sqlite3_pcache_page,
    pub pData:  *mut ::core::ffi::c_void,
    pub pExtra:  *mut ::core::ffi::c_void,
    pub pCache:  *mut crate::pcache_h::PCache,
    pub pDirty:  *mut crate::src::src::pcache::PgHdr,
    pub pPager:  *mut crate::src::src::pager::Pager,
    pub pgno:  crate::src::src::pager::Pgno,
    pub flags:  crate::src::fts5::u16_0,
    pub nRef:  crate::src::ext::rtree::rtree::i64_0,
    pub pDirtyNext:  *mut crate::src::src::pcache::PgHdr,
    pub pDirtyPrev:  *mut crate::src::src::pcache::PgHdr,
}
    
    pub const PGHDR_CLEAN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    pub const PGHDR_DIRTY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    pub const PGHDR_WRITEABLE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    pub const PGHDR_NEED_SYNC: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    pub const PGHDR_DONT_WRITE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    pub const PGHDR_MMAP: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    pub const PGHDR_WAL_APPEND: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub use crate::__stddef_size_t_h::size_t;




pub use crate::src::src::pager::Pager;pub use crate::src::src::pager::Pgno;pub use crate::src::src::pcache1::sqlite3PCacheSetDefault;pub use crate::sqlite3_h::sqlite3_int64;pub use crate::sqlite3_h::sqlite3_mem_methods;pub use crate::src::src::mutex_unix::sqlite3_mutex;pub use crate::sqlite3_h::sqlite3_mutex_methods;pub use crate::sqlite3_h::sqlite3_pcache;pub use crate::sqlite3_h::sqlite3_pcache_methods2;pub use crate::sqlite3_h::sqlite3_pcache_page;pub use crate::sqlite3_h::sqlite_int64;pub use crate::sqlite3_h::SQLITE_BUSY;pub use crate::sqlite3_h::SQLITE_NOMEM;pub use crate::sqlite3_h::SQLITE_OK;pub use crate::src::ext::rtree::rtree::i64_0;pub use crate::src::src::global::sqlite3Config;pub use crate::src::fts5::u16_0;pub use crate::src::ext::rtree::rtree::u32_0;pub use crate::src::ext::rtree::rtree::u8_0;pub use crate::sqliteInt_h::Sqlite3Config;pub use crate::sqliteInt_h::SQLITE_NOMEM_BKPT;pub use crate::stdlib::uint16_t;pub use crate::stdlib::uint32_t;pub use crate::stdlib::uint8_t;

pub use crate::stdlib::__uint16_t;pub use crate::stdlib::__uint32_t;pub use crate::stdlib::__uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct PCache {
    pub pDirty: *mut crate::src::src::pcache::PgHdr,
    pub pDirtyTail: *mut crate::src::src::pcache::PgHdr,
    pub pSynced: *mut crate::src::src::pcache::PgHdr,
    pub nRefSum: crate::src::ext::rtree::rtree::i64_0,
    pub szCache: ::core::ffi::c_int,
    pub szSpill: ::core::ffi::c_int,
    pub szPage: ::core::ffi::c_int,
    pub szExtra: ::core::ffi::c_int,
    pub bPurgeable: crate::src::ext::rtree::rtree::u8_0,
    pub eCreate: crate::src::ext::rtree::rtree::u8_0,
    pub xStress:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut crate::src::src::pcache::PgHdr) -> ::core::ffi::c_int>,
    pub pStress: *mut ::core::ffi::c_void,
    pub pCache: *mut crate::sqlite3_h::sqlite3_pcache,
}

pub const PCACHE_DIRTYLIST_REMOVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const PCACHE_DIRTYLIST_ADD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const PCACHE_DIRTYLIST_FRONT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

unsafe extern "C" fn pcacheManageDirtyList(mut pPage: *mut crate::src::src::pcache::PgHdr, mut addRemove: crate::src::ext::rtree::rtree::u8_0) {
    let mut p: *mut PCache = (*pPage).pCache;
    if addRemove as ::core::ffi::c_int & PCACHE_DIRTYLIST_REMOVE != 0 {
        if (*p).pSynced == pPage {
            (*p).pSynced = (*pPage).pDirtyPrev;
        }
        if !(*pPage).pDirtyNext.is_null() {
            (*(*pPage).pDirtyNext).pDirtyPrev = (*pPage).pDirtyPrev;
        } else {
            (*p).pDirtyTail = (*pPage).pDirtyPrev;
        }
        if !(*pPage).pDirtyPrev.is_null() {
            (*(*pPage).pDirtyPrev).pDirtyNext = (*pPage).pDirtyNext;
        } else {
            (*p).pDirty = (*pPage).pDirtyNext;
            if (*p).pDirty.is_null() {
                (*p).eCreate = 2 as crate::src::ext::rtree::rtree::u8_0;
            }
        }
    }
    if addRemove as ::core::ffi::c_int & PCACHE_DIRTYLIST_ADD != 0 {
        let __pPage_ref = unsafe { &mut *pPage };
        __pPage_ref.pDirtyPrev = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
        let __p_ref = unsafe { &mut *p };
        __pPage_ref.pDirtyNext = __p_ref.pDirty;
        if !__pPage_ref.pDirtyNext.is_null() {
            (*__pPage_ref.pDirtyNext).pDirtyPrev = pPage;
        } else {
            __p_ref.pDirtyTail = pPage;
            if __p_ref.bPurgeable != 0 {
                __p_ref.eCreate = 1 as crate::src::ext::rtree::rtree::u8_0;
            }
        }
        __p_ref.pDirty = pPage;
        if __p_ref.pSynced.is_null()
            && 0 as ::core::ffi::c_int == __pPage_ref.flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_NEED_SYNC
        {
            __p_ref.pSynced = pPage;
        }
    }
}

unsafe extern "C" fn pcacheUnpin(mut p: *mut crate::src::src::pcache::PgHdr) {
    if (*(*p).pCache).bPurgeable != 0 {
        crate::src::src::global::sqlite3Config
            .pcache2
            .xUnpin
            .expect("non-null function pointer")(
            (*(*p).pCache).pCache,
            (*p).pPage,
            0 as ::core::ffi::c_int,
        );
    }
}

unsafe extern "C" fn numberOfCachePages(mut p: *mut PCache) -> ::core::ffi::c_int {
    if (*p).szCache >= 0 as ::core::ffi::c_int {
        return (*p).szCache;
    } else {
        let mut n: crate::src::ext::rtree::rtree::i64_0 = 0;
        let __p_ref = unsafe { &*p };
        n = -(1024 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0 * __p_ref.szCache as crate::src::ext::rtree::rtree::i64_0
            / (__p_ref.szPage + __p_ref.szExtra) as crate::src::ext::rtree::rtree::i64_0;
        if n > 1000000000 as crate::src::ext::rtree::rtree::i64_0 {
            n = 1000000000 as crate::src::ext::rtree::rtree::i64_0;
        }
        return n as ::core::ffi::c_int;
    };
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheInitialize() -> ::core::ffi::c_int {
    if crate::src::src::global::sqlite3Config.pcache2.xInit.is_none() {
        crate::src::src::pcache1::sqlite3PCacheSetDefault();
    }
    return crate::src::src::global::sqlite3Config
        .pcache2
        .xInit
        .expect("non-null function pointer")(crate::src::src::global::sqlite3Config.pcache2.pArg);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheShutdown() {
    if crate::src::src::global::sqlite3Config.pcache2.xShutdown.is_some() {
        crate::src::src::global::sqlite3Config
            .pcache2
            .xShutdown
            .expect("non-null function pointer")(crate::src::src::global::sqlite3Config.pcache2.pArg);
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheSize() -> ::core::ffi::c_int {
    return ::core::mem::size_of::<PCache>() as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheOpen(
    mut szPage: ::core::ffi::c_int,
    mut szExtra: ::core::ffi::c_int,
    mut bPurgeable: ::core::ffi::c_int,
    mut xStress: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut crate::src::src::pcache::PgHdr) -> ::core::ffi::c_int,
    >,
    mut pStress: *mut ::core::ffi::c_void,
    mut p: *mut PCache,
) -> ::core::ffi::c_int {
    ::libc::memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PCache>() as crate::__stddef_size_t_h::size_t,
    );
    let __p_ref = unsafe { &mut *p };
    __p_ref.szPage = 1 as ::core::ffi::c_int;
    __p_ref.szExtra = szExtra;
    __p_ref.bPurgeable = bPurgeable as crate::src::ext::rtree::rtree::u8_0;
    __p_ref.eCreate = 2 as crate::src::ext::rtree::rtree::u8_0;
    __p_ref.xStress = xStress;
    __p_ref.pStress = pStress;
    __p_ref.szCache = 100 as ::core::ffi::c_int;
    __p_ref.szSpill = 1 as ::core::ffi::c_int;
    return sqlite3PcacheSetPageSize(p, szPage);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheSetPageSize(
    mut pCache: *mut PCache,
    mut szPage: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*pCache).szPage != 0 {
        let mut pNew: *mut crate::sqlite3_h::sqlite3_pcache = ::core::ptr::null_mut::<crate::sqlite3_h::sqlite3_pcache>();
        let __pCache_ref = unsafe { &mut *pCache };
        pNew = crate::src::src::global::sqlite3Config
            .pcache2
            .xCreate
            .expect("non-null function pointer")(
            szPage,
            (__pCache_ref.szExtra as usize).wrapping_add(
                (::core::mem::size_of::<crate::src::src::pcache::PgHdr>() as usize).wrapping_add(7 as usize)
                    & !(7 as ::core::ffi::c_int) as usize,
            ) as ::core::ffi::c_int,
            __pCache_ref.bPurgeable as ::core::ffi::c_int,
        );
        if pNew.is_null() {
            return crate::sqliteInt_h::SQLITE_NOMEM_BKPT;
        }
        crate::src::src::global::sqlite3Config
            .pcache2
            .xCachesize
            .expect("non-null function pointer")(pNew, numberOfCachePages(pCache));
        if !__pCache_ref.pCache.is_null() {
            crate::src::src::global::sqlite3Config
                .pcache2
                .xDestroy
                .expect("non-null function pointer")(__pCache_ref.pCache);
        }
        __pCache_ref.pCache = pNew;
        __pCache_ref.szPage = szPage;
    }
    return crate::sqlite3_h::SQLITE_OK;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheFetch(
    mut pCache: *mut PCache,
    mut pgno: crate::src::src::pager::Pgno,
    mut createFlag: ::core::ffi::c_int,
) -> *mut crate::sqlite3_h::sqlite3_pcache_page {
    let mut eCreate: ::core::ffi::c_int = 0;
    let mut pRes: *mut crate::sqlite3_h::sqlite3_pcache_page = ::core::ptr::null_mut::<crate::sqlite3_h::sqlite3_pcache_page>();
    eCreate = createFlag & (*pCache).eCreate as ::core::ffi::c_int;
    pRes = crate::src::src::global::sqlite3Config
        .pcache2
        .xFetch
        .expect("non-null function pointer")(
        (*pCache).pCache,
        pgno as ::core::ffi::c_uint,
        eCreate,
    );
    return pRes;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheFetchStress(
    mut pCache: *mut PCache,
    mut pgno: crate::src::src::pager::Pgno,
    mut ppPage: *mut *mut crate::sqlite3_h::sqlite3_pcache_page,
) -> ::core::ffi::c_int {
    let mut pPg: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    let __pCache_ref = unsafe { &mut *pCache };
    if __pCache_ref.eCreate as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if sqlite3PcachePagecount(pCache) > __pCache_ref.szSpill {
        pPg = __pCache_ref.pSynced;
        while !pPg.is_null()
            && ((*pPg).nRef != 0 || (*pPg).flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_NEED_SYNC != 0)
        {
            pPg = (*pPg).pDirtyPrev;
        }
        __pCache_ref.pSynced = pPg;
        if pPg.is_null() {
            pPg = __pCache_ref.pDirtyTail;
            while !pPg.is_null() && (*pPg).nRef != 0 {
                pPg = (*pPg).pDirtyPrev;
            }
        }
        if !pPg.is_null() {
            let mut rc: ::core::ffi::c_int = 0;
            rc = __pCache_ref.xStress.expect("non-null function pointer")(__pCache_ref.pStress, pPg);
            if rc != crate::sqlite3_h::SQLITE_OK && rc != crate::sqlite3_h::SQLITE_BUSY {
                return rc;
            }
        }
    }
    *ppPage = crate::src::src::global::sqlite3Config
        .pcache2
        .xFetch
        .expect("non-null function pointer")(
        __pCache_ref.pCache,
        pgno as ::core::ffi::c_uint,
        2 as ::core::ffi::c_int,
    );
    return if (*ppPage).is_null() {
        crate::sqliteInt_h::SQLITE_NOMEM_BKPT
    } else {
        crate::sqlite3_h::SQLITE_OK
    };
}
#[inline(never)]

unsafe extern "C" fn pcacheFetchFinishWithInit(
    mut pCache: *mut PCache,
    mut pgno: crate::src::src::pager::Pgno,
    mut pPage: *mut crate::sqlite3_h::sqlite3_pcache_page,
) -> *mut crate::src::src::pcache::PgHdr {
    let mut pPgHdr: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    pPgHdr = (*pPage).pExtra as *mut crate::src::src::pcache::PgHdr;
    ::libc::memset(
        &raw mut (*pPgHdr).pDirty as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<crate::src::src::pcache::PgHdr>() as crate::__stddef_size_t_h::size_t).wrapping_sub(32 as crate::__stddef_size_t_h::size_t),
    );
    (*pPgHdr).pPage = pPage;
    (*pPgHdr).pData = (*pPage).pBuf;
    (*pPgHdr).pExtra =
        pPgHdr.offset(1 as isize) as *mut crate::src::src::pcache::PgHdr as *mut ::core::ffi::c_void;
    ::libc::memset((*pPgHdr).pExtra, 0 as ::core::ffi::c_int, 8 as crate::__stddef_size_t_h::size_t);
    (*pPgHdr).pCache = pCache;
    (*pPgHdr).pgno = pgno;
    (*pPgHdr).flags = crate::src::src::pcache::PGHDR_CLEAN as crate::src::fts5::u16_0;
    return sqlite3PcacheFetchFinish(pCache, pgno, pPage);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheFetchFinish(
    mut pCache: *mut PCache,
    mut pgno: crate::src::src::pager::Pgno,
    mut pPage: *mut crate::sqlite3_h::sqlite3_pcache_page,
) -> *mut crate::src::src::pcache::PgHdr {
    let mut pPgHdr: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    pPgHdr = (*pPage).pExtra as *mut crate::src::src::pcache::PgHdr;
    if (*pPgHdr).pPage.is_null() {
        return pcacheFetchFinishWithInit(pCache, pgno, pPage);
    }
    (*pCache).nRefSum += 1;
    (*pPgHdr).nRef += 1;
    return pPgHdr;
}
#[no_mangle]
#[inline(never)]

pub unsafe extern "C" fn sqlite3PcacheRelease(mut p: *mut crate::src::src::pcache::PgHdr) {
    let __p_ref = unsafe { &mut *p };
    (*__p_ref.pCache).nRefSum -= 1;
    __p_ref.nRef -= 1;
    if __p_ref.nRef == 0 as crate::src::ext::rtree::rtree::i64_0 {
        if __p_ref.flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_CLEAN != 0 {
            pcacheUnpin(p);
        } else {
            pcacheManageDirtyList(p, PCACHE_DIRTYLIST_FRONT as crate::src::ext::rtree::rtree::u8_0);
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheRef(mut p: *mut crate::src::src::pcache::PgHdr) {
    (*p).nRef += 1;
    (*(*p).pCache).nRefSum += 1;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheDrop(mut p: *mut crate::src::src::pcache::PgHdr) {
    let __p_ref = unsafe { &mut *p };
    if __p_ref.flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_DIRTY != 0 {
        pcacheManageDirtyList(p, PCACHE_DIRTYLIST_REMOVE as crate::src::ext::rtree::rtree::u8_0);
    }
    (*__p_ref.pCache).nRefSum -= 1;
    crate::src::src::global::sqlite3Config
        .pcache2
        .xUnpin
        .expect("non-null function pointer")(
        (*__p_ref.pCache).pCache,
        __p_ref.pPage,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheMakeDirty(mut p: *mut crate::src::src::pcache::PgHdr) {
    if (*p).flags as ::core::ffi::c_int & (crate::src::src::pcache::PGHDR_CLEAN | crate::src::src::pcache::PGHDR_DONT_WRITE) != 0 {
        let __p_ref = unsafe { &mut *p };
        __p_ref.flags = (__p_ref.flags as ::core::ffi::c_int & !crate::src::src::pcache::PGHDR_DONT_WRITE) as crate::src::fts5::u16_0;
        if __p_ref.flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_CLEAN != 0 {
            __p_ref.flags = (__p_ref.flags as ::core::ffi::c_int ^ (crate::src::src::pcache::PGHDR_DIRTY | crate::src::src::pcache::PGHDR_CLEAN)) as crate::src::fts5::u16_0;
            pcacheManageDirtyList(p, PCACHE_DIRTYLIST_ADD as crate::src::ext::rtree::rtree::u8_0);
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheMakeClean(mut p: *mut crate::src::src::pcache::PgHdr) {
    pcacheManageDirtyList(p, PCACHE_DIRTYLIST_REMOVE as crate::src::ext::rtree::rtree::u8_0);
    let __p_ref = unsafe { &mut *p };
    __p_ref.flags = (__p_ref.flags as ::core::ffi::c_int
        & !(crate::src::src::pcache::PGHDR_DIRTY | crate::src::src::pcache::PGHDR_NEED_SYNC | crate::src::src::pcache::PGHDR_WRITEABLE)) as crate::src::fts5::u16_0;
    __p_ref.flags = (__p_ref.flags as ::core::ffi::c_int | crate::src::src::pcache::PGHDR_CLEAN) as crate::src::fts5::u16_0;
    if __p_ref.nRef == 0 as crate::src::ext::rtree::rtree::i64_0 {
        pcacheUnpin(p);
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheCleanAll(mut pCache: *mut PCache) {
    let mut p: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    loop {
        p = (*pCache).pDirty;
        if p.is_null() {
            break;
        }
        sqlite3PcacheMakeClean(p);
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheClearWritable(mut pCache: *mut PCache) {
    let mut p: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    let __pCache_ref = unsafe { &mut *pCache };
    p = __pCache_ref.pDirty;
    while !p.is_null() {
        (*p).flags =
            ((*p).flags as ::core::ffi::c_int & !(crate::src::src::pcache::PGHDR_NEED_SYNC | crate::src::src::pcache::PGHDR_WRITEABLE)) as crate::src::fts5::u16_0;
        p = (*p).pDirtyNext;
    }
    __pCache_ref.pSynced = __pCache_ref.pDirtyTail;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheClearSyncFlags(mut pCache: *mut PCache) {
    let mut p: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    let __pCache_ref = unsafe { &mut *pCache };
    p = __pCache_ref.pDirty;
    while !p.is_null() {
        (*p).flags = ((*p).flags as ::core::ffi::c_int & !crate::src::src::pcache::PGHDR_NEED_SYNC) as crate::src::fts5::u16_0;
        p = (*p).pDirtyNext;
    }
    __pCache_ref.pSynced = __pCache_ref.pDirtyTail;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheMove(mut p: *mut crate::src::src::pcache::PgHdr, mut newPgno: crate::src::src::pager::Pgno) {
    let __p_ref = unsafe { &mut *p };
    let mut pCache: *mut PCache = __p_ref.pCache;
    let mut pOther: *mut crate::sqlite3_h::sqlite3_pcache_page = ::core::ptr::null_mut::<crate::sqlite3_h::sqlite3_pcache_page>();
    pOther = crate::src::src::global::sqlite3Config
        .pcache2
        .xFetch
        .expect("non-null function pointer")(
        (*pCache).pCache,
        newPgno as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int,
    );
    if !pOther.is_null() {
        let mut pXPage: *mut crate::src::src::pcache::PgHdr = (*pOther).pExtra as *mut crate::src::src::pcache::PgHdr;
        (*pXPage).nRef += 1;
        (*pCache).nRefSum += 1;
        sqlite3PcacheDrop(pXPage);
    }
    crate::src::src::global::sqlite3Config
        .pcache2
        .xRekey
        .expect("non-null function pointer")(
        (*pCache).pCache,
        __p_ref.pPage,
        __p_ref.pgno as ::core::ffi::c_uint,
        newPgno as ::core::ffi::c_uint,
    );
    __p_ref.pgno = newPgno;
    if __p_ref.flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_DIRTY != 0
        && __p_ref.flags as ::core::ffi::c_int & crate::src::src::pcache::PGHDR_NEED_SYNC != 0
    {
        pcacheManageDirtyList(p, PCACHE_DIRTYLIST_FRONT as crate::src::ext::rtree::rtree::u8_0);
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheTruncate(mut pCache: *mut PCache, mut pgno: crate::src::src::pager::Pgno) {
    if !(*pCache).pCache.is_null() {
        let mut p: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
        let mut pNext: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
        let __pCache_ref = unsafe { &*pCache };
        p = __pCache_ref.pDirty;
        while !p.is_null() {
            pNext = (*p).pDirtyNext;
            if (*p).pgno > pgno {
                sqlite3PcacheMakeClean(p);
            }
            p = pNext;
        }
        if pgno == 0 as crate::src::src::pager::Pgno && __pCache_ref.nRefSum != 0 {
            let mut pPage1: *mut crate::sqlite3_h::sqlite3_pcache_page =
                ::core::ptr::null_mut::<crate::sqlite3_h::sqlite3_pcache_page>();
            pPage1 = crate::src::src::global::sqlite3Config
                .pcache2
                .xFetch
                .expect("non-null function pointer")(
                __pCache_ref.pCache,
                1 as ::core::ffi::c_uint,
                0 as ::core::ffi::c_int,
            );
            if !pPage1.is_null() {
                ::libc::memset(
                    (*pPage1).pBuf,
                    0 as ::core::ffi::c_int,
                    __pCache_ref.szPage as crate::__stddef_size_t_h::size_t,
                );
                pgno = 1 as crate::src::src::pager::Pgno;
            }
        }
        crate::src::src::global::sqlite3Config
            .pcache2
            .xTruncate
            .expect("non-null function pointer")(
            __pCache_ref.pCache,
            (pgno as ::core::ffi::c_uint).wrapping_add(1 as ::core::ffi::c_uint),
        );
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheClose(mut pCache: *mut PCache) {
    crate::src::src::global::sqlite3Config
        .pcache2
        .xDestroy
        .expect("non-null function pointer")((*pCache).pCache);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheClear(mut pCache: *mut PCache) {
    sqlite3PcacheTruncate(pCache, 0 as crate::src::src::pager::Pgno);
}

unsafe extern "C" fn pcacheMergeDirtyList(mut pA: *mut crate::src::src::pcache::PgHdr, mut pB: *mut crate::src::src::pcache::PgHdr) -> *mut crate::src::src::pcache::PgHdr {
    let mut result: crate::src::src::pcache::PgHdr = crate::src::src::pcache::PgHdr {
    pPage:  ::core::ptr::null_mut::<crate::sqlite3_h::sqlite3_pcache_page>(),
    pData:  ::core::ptr::null_mut::<::core::ffi::c_void>(),
    pExtra:  ::core::ptr::null_mut::<::core::ffi::c_void>(),
    pCache:  ::core::ptr::null_mut::<PCache>(),
    pDirty:  ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>(),
    pPager:  ::core::ptr::null_mut::<crate::src::src::pager::Pager>(),
    pgno:  0,
    flags:  0,
    nRef:  0,
    pDirtyNext:  ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>(),
    pDirtyPrev:  ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>(),
};
    let mut pTail: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    pTail = &raw mut result;
    loop {
        if (*pA).pgno < (*pB).pgno {
            (*pTail).pDirty = pA;
            pTail = pA;
            pA = (*pA).pDirty;
            if !pA.is_null() {
                continue;
            }
            (*pTail).pDirty = pB;
            break;
        } else {
            (*pTail).pDirty = pB;
            pTail = pB;
            pB = (*pB).pDirty;
            if !pB.is_null() {
                continue;
            }
            (*pTail).pDirty = pA;
            break;
        }
    }
    return result.pDirty;
}

pub const N_SORT_BUCKET: ::core::ffi::c_int = 32 as ::core::ffi::c_int;

unsafe extern "C" fn pcacheSortDirtyList(mut pIn: *mut crate::src::src::pcache::PgHdr) -> *mut crate::src::src::pcache::PgHdr {
    let mut a: [*mut crate::src::src::pcache::PgHdr; 32] = unsafe { ::core::mem::zeroed() };
    let mut p: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    let mut i: ::core::ffi::c_int = 0;
    while !pIn.is_null() {
        p = pIn;
        pIn = (*p).pDirty;
        (*p).pDirty = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
        i = 0 as ::core::ffi::c_int;
        while i < 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
            if a[i as usize].is_null() {
                a[i as usize] = p;
                break;
            } else {
                p = pcacheMergeDirtyList(a[i as usize], p);
                a[i as usize] = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
                i += 1;
            }
        }
        if i == 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
            a[i as usize] = pcacheMergeDirtyList(a[i as usize], p);
        }
    }
    p = a[0 as ::core::ffi::c_int as usize];
    i = 1 as ::core::ffi::c_int;
    while i < N_SORT_BUCKET {
        if !a[i as usize].is_null() {
            p = if !p.is_null() {
                pcacheMergeDirtyList(p, a[i as usize])
            } else {
                a[i as usize]
            };
        }
        i += 1;
    }
    return p;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheDirtyList(mut pCache: *mut PCache) -> *mut crate::src::src::pcache::PgHdr {
    let mut p: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    p = (*pCache).pDirty;
    while !p.is_null() {
        (*p).pDirty = (*p).pDirtyNext;
        p = (*p).pDirtyNext;
    }
    return pcacheSortDirtyList((*pCache).pDirty);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheRefCount(mut pCache: *mut PCache) -> crate::src::ext::rtree::rtree::i64_0 {
    return (*pCache).nRefSum;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcachePageRefcount(mut p: *mut crate::src::src::pcache::PgHdr) -> crate::src::ext::rtree::rtree::i64_0 {
    return (*p).nRef;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcachePagecount(mut pCache: *mut PCache) -> ::core::ffi::c_int {
    return crate::src::src::global::sqlite3Config
        .pcache2
        .xPagecount
        .expect("non-null function pointer")((*pCache).pCache);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheGetCachesize(mut pCache: *mut PCache) -> ::core::ffi::c_int {
    return numberOfCachePages(pCache);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheSetCachesize(
    mut pCache: *mut PCache,
    mut mxPage: ::core::ffi::c_int,
) {
    (*pCache).szCache = mxPage;
    crate::src::src::global::sqlite3Config
        .pcache2
        .xCachesize
        .expect("non-null function pointer")((*pCache).pCache, numberOfCachePages(pCache));
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheSetSpillsize(
    mut p: *mut PCache,
    mut mxPage: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 0;
    if mxPage != 0 {
        if mxPage < 0 as ::core::ffi::c_int {
            mxPage = (-(1024 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0 * mxPage as crate::src::ext::rtree::rtree::i64_0
                / ((*p).szPage + (*p).szExtra) as crate::src::ext::rtree::rtree::i64_0) as ::core::ffi::c_int;
        }
        (*p).szSpill = mxPage;
    }
    res = numberOfCachePages(p);
    if res < (*p).szSpill {
        res = (*p).szSpill;
    }
    return res;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PcacheShrink(mut pCache: *mut PCache) {
    crate::src::src::global::sqlite3Config
        .pcache2
        .xShrink
        .expect("non-null function pointer")((*pCache).pCache);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3HeaderSizePcache() -> ::core::ffi::c_int {
    return ((::core::mem::size_of::<crate::src::src::pcache::PgHdr>() as usize).wrapping_add(7 as usize)
        & !(7 as ::core::ffi::c_int) as usize) as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PCachePercentDirty(mut pCache: *mut PCache) -> ::core::ffi::c_int {
    let mut pDirty: *mut crate::src::src::pcache::PgHdr = ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>();
    let mut nDirty: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nCache: ::core::ffi::c_int = numberOfCachePages(pCache);
    pDirty = (*pCache).pDirty;
    while !pDirty.is_null() {
        nDirty += 1;
        pDirty = (*pDirty).pDirtyNext;
    }
    return if nCache != 0 {
        (nDirty as crate::src::ext::rtree::rtree::i64_0 * 100 as crate::src::ext::rtree::rtree::i64_0 / nCache as crate::src::ext::rtree::rtree::i64_0) as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3PCacheIsDirty(mut pCache: *mut PCache) -> ::core::ffi::c_int {
    return ((*pCache).pDirty != ::core::ptr::null_mut::<crate::src::src::pcache::PgHdr>()) as ::core::ffi::c_int;
}
