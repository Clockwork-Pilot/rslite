pub use crate::__stddef_size_t_h::SizeT;
pub use crate::internal::__ATOMIC_RELAXED;

pub use crate::src::ext::rtree::rtree::I64_0;
pub use crate::src::ext::rtree::rtree::U8_0;
pub use crate::src::ext::rtree::rtree::U32_0;
pub use crate::src::ext::rtree::rtree::U64_0;
pub use crate::src::fts5::U16_0;
pub use crate::src::headers::sqlite3_h::SQLITE_CONFIG_PCACHE2_1;
pub use crate::src::headers::sqlite3_h::SQLITE_MUTEX_STATIC_LRU;
pub use crate::src::headers::sqlite3_h::SQLITE_MUTEX_STATIC_PMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SQLITE_STATUS_PAGECACHE_OVERFLOW;
pub use crate::src::headers::sqlite3_h::SQLITE_STATUS_PAGECACHE_SIZE;
pub use crate::src::headers::sqlite3_h::SQLITE_STATUS_PAGECACHE_USED;
pub use crate::src::headers::sqlite3_h::SqliteInt64;
pub use crate::src::headers::sqlite3_h::SqliteUint64;
pub use crate::src::headers::sqlite3_h::Sqlite3Int64;
pub use crate::src::headers::sqlite3_h::sqlite3_mem_methods;
pub use crate::src::headers::sqlite3_h::sqlite3_mutex_methods;
pub use crate::src::headers::sqlite3_h::Sqlite3Pcache;
pub use crate::src::headers::sqlite3_h::sqlite3_pcache_methods2;
pub use crate::src::headers::sqlite3_h::sqlite3_pcache_page;
pub use crate::src::headers::sqliteInt_h::Sqlite3Config;
pub use crate::src::headers::sqliteInt_h::Uptr;
pub use crate::src::headers::stdlib::Uint8T;
pub use crate::src::headers::stdlib::Uint16T;
pub use crate::src::headers::stdlib::Uint32T;
pub use crate::src::headers::stdlib::UintptrT;
pub use crate::src::src::fault::sqlite3BeginBenignMalloc;
pub use crate::src::src::fault::sqlite3EndBenignMalloc;
pub use crate::src::src::global::sqlite3Config;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3HeapNearlyFull;
pub use crate::src::src::malloc::sqlite3Malloc;
pub use crate::src::src::malloc::sqlite3MallocSize;
pub use crate::src::src::malloc::sqlite3MallocZero;
pub use crate::src::src::mutex::sqlite3_mutex_enter;
pub use crate::src::src::mutex::sqlite3_mutex_leave;
pub use crate::src::src::mutex::sqlite3MutexAlloc;
pub use crate::src::src::mutex_unix::sqlite3_mutex;
pub use crate::src::src::status::sqlite3StatusDown;
pub use crate::src::src::status::sqlite3StatusHighwater;
pub use crate::src::src::status::sqlite3StatusUp;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PCacheGlobal {
    pub grp: PGroup,
    pub isInit: ::core::ffi::c_int,
    pub separateCache: ::core::ffi::c_int,
    pub nInitPage: ::core::ffi::c_int,
    pub szSlot: ::core::ffi::c_int,
    pub nSlot: ::core::ffi::c_int,
    pub nReserve: ::core::ffi::c_int,
    pub pStart: *mut ::core::ffi::c_void,
    pub pEnd: *mut ::core::ffi::c_void,
    pub mutex: *mut crate::src::src::mutex_unix::sqlite3_mutex,
    pub pFree: *mut PgFreeslot,
    pub nFreeSlot: ::core::ffi::c_int,
    pub bUnderPressure: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PgFreeslot {
    pub pNext: *mut PgFreeslot,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PGroup {
    pub mutex: *mut crate::src::src::mutex_unix::sqlite3_mutex,
    pub nMaxPage: ::core::ffi::c_uint,
    pub nMinPage: ::core::ffi::c_uint,
    pub mxPinned: ::core::ffi::c_uint,
    pub nPurgeable: ::core::ffi::c_uint,
    pub lru: PgHdr1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PgHdr1 {
    pub page: crate::src::headers::sqlite3_h::sqlite3_pcache_page,
    pub iKey: ::core::ffi::c_uint,
    pub isBulkLocal: crate::src::fts5::U16_0,
    pub isAnchor: crate::src::fts5::U16_0,
    pub pNext: *mut PgHdr1,
    pub pCache: *mut PCache1,
    pub pLruNext: *mut PgHdr1,
    pub pLruPrev: *mut PgHdr1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PCache1 {
    pub pGroup: *mut PGroup,
    pub pnPurgeable: *mut ::core::ffi::c_uint,
    pub szPage: ::core::ffi::c_int,
    pub szExtra: ::core::ffi::c_int,
    pub szAlloc: ::core::ffi::c_int,
    pub bPurgeable: ::core::ffi::c_int,
    pub nMin: ::core::ffi::c_uint,
    pub nMax: ::core::ffi::c_uint,
    pub n90pct: ::core::ffi::c_uint,
    pub iMaxKey: ::core::ffi::c_uint,
    pub nPurgeableDummy: ::core::ffi::c_uint,
    pub nRecyclable: ::core::ffi::c_uint,
    pub nPage: ::core::ffi::c_uint,
    pub nHash: ::core::ffi::c_uint,
    pub apHash: *mut *mut PgHdr1,
    pub pFree: *mut PgHdr1,
    pub pBulk: *mut ::core::ffi::c_void,
}

static mut pcache1_g: PCacheGlobal = PCacheGlobal {
    grp: PGroup {
        mutex: ::core::ptr::null::<crate::src::src::mutex_unix::sqlite3_mutex>()
            as *mut crate::src::src::mutex_unix::sqlite3_mutex,
        nMaxPage: 0,
        nMinPage: 0,
        mxPinned: 0,
        nPurgeable: 0,
        lru: PgHdr1 {
            page: crate::src::headers::sqlite3_h::sqlite3_pcache_page {
                pBuf: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
                pExtra: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            },
            iKey: 0,
            isBulkLocal: 0,
            isAnchor: 0,
            pNext: ::core::ptr::null::<PgHdr1>() as *mut PgHdr1,
            pCache: ::core::ptr::null::<PCache1>() as *mut PCache1,
            pLruNext: ::core::ptr::null::<PgHdr1>() as *mut PgHdr1,
            pLruPrev: ::core::ptr::null::<PgHdr1>() as *mut PgHdr1,
        },
    },
    isInit: 0,
    separateCache: 0,
    nInitPage: 0,
    szSlot: 0,
    nSlot: 0,
    nReserve: 0,
    pStart: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    pEnd: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    mutex: ::core::ptr::null::<crate::src::src::mutex_unix::sqlite3_mutex>()
        as *mut crate::src::src::mutex_unix::sqlite3_mutex,
    pFree: ::core::ptr::null::<PgFreeslot>() as *mut PgFreeslot,
    nFreeSlot: 0,
    bUnderPressure: 0,
};
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3PCacheBufferSetup(
    mut pBuf: *mut ::core::ffi::c_void,
    mut sz: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) {
    if pcache1_g.isInit != 0 {
        let mut p: *mut PgFreeslot;
        if pBuf.is_null() {
            n = 0 as ::core::ffi::c_int;
            sz = n;
        }
        if n == 0 as ::core::ffi::c_int {
            sz = 0 as ::core::ffi::c_int;
        }
        sz &= !(7 as ::core::ffi::c_int);
        pcache1_g.szSlot = sz;
        pcache1_g.nFreeSlot = n;
        pcache1_g.nSlot = pcache1_g.nFreeSlot;
        pcache1_g.nReserve = if n > 90 as ::core::ffi::c_int {
            10 as ::core::ffi::c_int
        } else {
            n / 10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        };
        pcache1_g.pStart = pBuf;
        pcache1_g.pFree = ::core::ptr::null_mut::<PgFreeslot>();
        (*((&raw mut pcache1_g.bUnderPressure) as *mut std::sync::atomic::AtomicI32)).store(
            0 as ::core::ffi::c_int,
            std::sync::atomic::Ordering::Relaxed,
        );
        loop {
            let fresh0 = n;
            n -= 1;
            if (fresh0 == 0) {
                break;
            }
            p = pBuf as *mut PgFreeslot;
            (*p).pNext = pcache1_g.pFree;
            pcache1_g.pFree = p;
            pBuf = (pBuf as *mut ::core::ffi::c_char).offset(sz as isize)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        }
        pcache1_g.pEnd = pBuf;
    }
}

unsafe extern "C" fn pcache1InitBulk(pCache: *mut PCache1) -> ::core::ffi::c_int {
    let mut szBulk: crate::src::ext::rtree::rtree::I64_0;
    let mut zBulk: *mut ::core::ffi::c_char;
    if pcache1_g.nInitPage == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let __pCache_ref = { &mut *pCache };
    if __pCache_ref.nMax < 3 as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_int;
    }
    crate::src::src::fault::sqlite3BeginBenignMalloc();
    if pcache1_g.nInitPage > 0 as ::core::ffi::c_int {
        szBulk = __pCache_ref.szAlloc as crate::src::ext::rtree::rtree::I64_0
            * pcache1_g.nInitPage as crate::src::ext::rtree::rtree::I64_0;
    } else {
        szBulk = -(1024 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::I64_0
            * pcache1_g.nInitPage as crate::src::ext::rtree::rtree::I64_0;
    }
    if szBulk
        > __pCache_ref.szAlloc as crate::src::ext::rtree::rtree::I64_0
            * __pCache_ref.nMax as crate::src::ext::rtree::rtree::I64_0
    {
        szBulk = __pCache_ref.szAlloc as crate::src::ext::rtree::rtree::I64_0
            * __pCache_ref.nMax as crate::src::ext::rtree::rtree::I64_0;
    }
    __pCache_ref.pBulk =
        crate::src::src::malloc::sqlite3Malloc(szBulk as crate::src::ext::rtree::rtree::U64_0);
    zBulk = __pCache_ref.pBulk as *mut ::core::ffi::c_char;
    crate::src::src::fault::sqlite3EndBenignMalloc();
    if !zBulk.is_null() {
        let mut nBulk: ::core::ffi::c_int =
            crate::src::src::malloc::sqlite3MallocSize(zBulk as *const ::core::ffi::c_void)
                / __pCache_ref.szAlloc;
        loop {
            let pX: *mut PgHdr1 = zBulk.offset(__pCache_ref.szPage as isize)
                as *mut ::core::ffi::c_char as *mut PgHdr1;
            let __pX_ref = { &mut *pX };
            __pX_ref.page.pBuf = zBulk as *mut ::core::ffi::c_void;
            __pX_ref.page.pExtra = (pX as *mut crate::src::ext::rtree::rtree::U8_0).add(
                ((::core::mem::size_of::<PgHdr1>() as usize).wrapping_add(7_usize)
                    & !(7 as ::core::ffi::c_int) as usize),
            ) as *mut ::core::ffi::c_void;
            __pX_ref.isBulkLocal = 1 as crate::src::fts5::U16_0;
            __pX_ref.isAnchor = 0 as crate::src::fts5::U16_0;
            __pX_ref.pNext = __pCache_ref.pFree;
            __pX_ref.pLruPrev = ::core::ptr::null_mut::<PgHdr1>();
            __pCache_ref.pFree = pX;
            zBulk = zBulk.offset(__pCache_ref.szAlloc as isize);
            nBulk -= 1;
            if (nBulk == 0) {
                break;
            }
        }
    }
    !__pCache_ref.pFree.is_null() as ::core::ffi::c_int
}

unsafe extern "C" fn pcache1Alloc(nByte: ::core::ffi::c_int) -> *mut ::core::ffi::c_void {
    let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if nByte <= pcache1_g.szSlot {
        crate::src::src::mutex::sqlite3_mutex_enter(pcache1_g.mutex);
        p = pcache1_g.pFree as *mut PgHdr1 as *mut ::core::ffi::c_void;
        if !p.is_null() {
            pcache1_g.pFree = (*pcache1_g.pFree).pNext;
            pcache1_g.nFreeSlot -= 1;
            (*((&raw mut pcache1_g.bUnderPressure) as *mut std::sync::atomic::AtomicI32)).store(
                (pcache1_g.nFreeSlot < pcache1_g.nReserve) as ::core::ffi::c_int,
                std::sync::atomic::Ordering::Relaxed,
            );
            crate::src::src::status::sqlite3StatusHighwater(
                crate::src::headers::sqlite3_h::SQLITE_STATUS_PAGECACHE_SIZE,
                nByte,
            );
            crate::src::src::status::sqlite3StatusUp(
                crate::src::headers::sqlite3_h::SQLITE_STATUS_PAGECACHE_USED,
                1 as ::core::ffi::c_int,
            );
        }
        crate::src::src::mutex::sqlite3_mutex_leave(pcache1_g.mutex);
    }
    if p.is_null() {
        p = crate::src::src::malloc::sqlite3Malloc(nByte as crate::src::ext::rtree::rtree::U64_0);
        if !p.is_null() {
            let sz: ::core::ffi::c_int = crate::src::src::malloc::sqlite3MallocSize(p);
            crate::src::src::mutex::sqlite3_mutex_enter(pcache1_g.mutex);
            crate::src::src::status::sqlite3StatusHighwater(
                crate::src::headers::sqlite3_h::SQLITE_STATUS_PAGECACHE_SIZE,
                nByte,
            );
            crate::src::src::status::sqlite3StatusUp(
                crate::src::headers::sqlite3_h::SQLITE_STATUS_PAGECACHE_OVERFLOW,
                sz,
            );
            crate::src::src::mutex::sqlite3_mutex_leave(pcache1_g.mutex);
        }
    }
    p
}

unsafe extern "C" fn pcache1Free(p: *mut ::core::ffi::c_void) {
    if p.is_null() {
        return;
    }
    if p as crate::src::headers::sqliteInt_h::Uptr
        >= pcache1_g.pStart as crate::src::headers::sqliteInt_h::Uptr
        && (p as crate::src::headers::sqliteInt_h::Uptr)
            < pcache1_g.pEnd as crate::src::headers::sqliteInt_h::Uptr
    {
        
        crate::src::src::mutex::sqlite3_mutex_enter(pcache1_g.mutex);
        crate::src::src::status::sqlite3StatusDown(
            crate::src::headers::sqlite3_h::SQLITE_STATUS_PAGECACHE_USED,
            1 as ::core::ffi::c_int,
        );
        let pSlot: *mut PgFreeslot = p as *mut PgFreeslot;
        (*pSlot).pNext = pcache1_g.pFree;
        pcache1_g.pFree = pSlot;
        pcache1_g.nFreeSlot += 1;
        (*((&raw mut pcache1_g.bUnderPressure) as *mut std::sync::atomic::AtomicI32)).store(
            (pcache1_g.nFreeSlot < pcache1_g.nReserve) as ::core::ffi::c_int,
            std::sync::atomic::Ordering::Relaxed,
        );
        crate::src::src::mutex::sqlite3_mutex_leave(pcache1_g.mutex);
    } else {
        
        let nFreed: ::core::ffi::c_int = crate::src::src::malloc::sqlite3MallocSize(p);
        crate::src::src::mutex::sqlite3_mutex_enter(pcache1_g.mutex);
        crate::src::src::status::sqlite3StatusDown(
            crate::src::headers::sqlite3_h::SQLITE_STATUS_PAGECACHE_OVERFLOW,
            nFreed,
        );
        crate::src::src::mutex::sqlite3_mutex_leave(pcache1_g.mutex);
        crate::src::src::malloc::sqlite3_free(p);
    };
}

unsafe extern "C" fn pcache1AllocPage(
    pCache: *mut PCache1,
    benignMalloc: ::core::ffi::c_int,
) -> *mut PgHdr1 {
    let p: *mut PgHdr1;
    let pPg: *mut ::core::ffi::c_void;
    let __pCache_ref = { &mut *pCache };
    if !__pCache_ref.pFree.is_null()
        || __pCache_ref.nPage == 0 as ::core::ffi::c_uint && pcache1InitBulk(pCache) != 0
    {
        p = __pCache_ref.pFree;
        __pCache_ref.pFree = (*p).pNext;
        (*p).pNext = ::core::ptr::null_mut::<PgHdr1>();
    } else {
        if benignMalloc != 0 {
            crate::src::src::fault::sqlite3BeginBenignMalloc();
        }
        pPg = pcache1Alloc(__pCache_ref.szAlloc);
        if benignMalloc != 0 {
            crate::src::src::fault::sqlite3EndBenignMalloc();
        }
        if pPg.is_null() {
            return ::core::ptr::null_mut::<PgHdr1>();
        }
        p = (pPg as *mut crate::src::ext::rtree::rtree::U8_0).offset(__pCache_ref.szPage as isize)
            as *mut crate::src::ext::rtree::rtree::U8_0 as *mut PgHdr1;
        (*p).page.pBuf = pPg;
        (*p).page.pExtra = (p as *mut crate::src::ext::rtree::rtree::U8_0).add(
            ((::core::mem::size_of::<PgHdr1>() as usize).wrapping_add(7_usize)
                & !(7 as ::core::ffi::c_int) as usize),
        ) as *mut ::core::ffi::c_void;
        (*p).isBulkLocal = 0 as crate::src::fts5::U16_0;
        (*p).isAnchor = 0 as crate::src::fts5::U16_0;
        (*p).pLruPrev = ::core::ptr::null_mut::<PgHdr1>();
    }
    *__pCache_ref.pnPurgeable = (*__pCache_ref.pnPurgeable).wrapping_add(1);
    p
}

unsafe extern "C" fn pcache1FreePage(p: *mut PgHdr1) {
    
    let pCache: *mut PCache1 = (*p).pCache;
    if (*p).isBulkLocal != 0 {
        (*p).pNext = (*pCache).pFree;
        (*pCache).pFree = p;
    } else {
        pcache1Free((*p).page.pBuf);
    }
    *(*pCache).pnPurgeable = (*(*pCache).pnPurgeable).wrapping_sub(1);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3PageMalloc(sz: ::core::ffi::c_int) -> *mut ::core::ffi::c_void {
    pcache1Alloc(sz)
}
pub unsafe extern "C" fn sqlite3PageFree(p: *mut ::core::ffi::c_void) {
    pcache1Free(p);
}

unsafe extern "C" fn pcache1UnderMemoryPressure(pCache: *mut PCache1) -> ::core::ffi::c_int {
    if pcache1_g.nSlot != 0 && (*pCache).szPage + (*pCache).szExtra <= pcache1_g.szSlot {
        (*((&raw mut pcache1_g.bUnderPressure) as *mut std::sync::atomic::AtomicI32))
            .load(std::sync::atomic::Ordering::Relaxed)
    } else {
        crate::src::src::malloc::sqlite3HeapNearlyFull()
    }
}

unsafe extern "C" fn pcache1ResizeHash(p: *mut PCache1) {
    
    let mut nNew: crate::src::ext::rtree::rtree::U64_0;
    let mut i: crate::src::ext::rtree::rtree::U32_0;
    let __p_ref = { &mut *p };
    nNew = (2 as crate::src::ext::rtree::rtree::U64_0)
        .wrapping_mul(__p_ref.nHash as crate::src::ext::rtree::rtree::U64_0);
    if nNew < 256 as crate::src::ext::rtree::rtree::U64_0 {
        nNew = 256 as crate::src::ext::rtree::rtree::U64_0;
    }
    if __p_ref.nHash != 0 {
        crate::src::src::fault::sqlite3BeginBenignMalloc();
    }
    let apNew: *mut *mut PgHdr1 = crate::src::src::malloc::sqlite3MallocZero(
        (::core::mem::size_of::<*mut PgHdr1>() as crate::src::ext::rtree::rtree::U64_0)
            .wrapping_mul(nNew),
    ) as *mut *mut PgHdr1;
    if __p_ref.nHash != 0 {
        crate::src::src::fault::sqlite3EndBenignMalloc();
    }
    if !apNew.is_null() {
        i = 0 as crate::src::ext::rtree::rtree::U32_0;
        while i < __p_ref.nHash as crate::src::ext::rtree::rtree::U32_0 {
            let mut pPage: *mut PgHdr1;
            let mut pNext: *mut PgHdr1 = *__p_ref.apHash.offset(i as isize);
            loop {
                pPage = pNext;
                if pPage.is_null() {
                    break;
                }
                let h: ::core::ffi::c_uint =
                    ((*pPage).iKey as crate::src::ext::rtree::rtree::U64_0).wrapping_rem(nNew)
                        as ::core::ffi::c_uint;
                pNext = (*pPage).pNext;
                (*pPage).pNext = *apNew.offset(h as isize);
                let fresh4 = &mut *apNew.offset(h as isize);
                *fresh4 = pPage;
            }
            i = i.wrapping_add(1);
        }
        crate::src::src::malloc::sqlite3_free(__p_ref.apHash as *mut ::core::ffi::c_void);
        __p_ref.apHash = apNew;
        __p_ref.nHash = nNew as ::core::ffi::c_uint;
    }
}

unsafe extern "C" fn pcache1PinPage(pPage: *mut PgHdr1) -> *mut PgHdr1 {
    let __pPage_ref = { &mut *pPage };
    (*__pPage_ref.pLruPrev).pLruNext = __pPage_ref.pLruNext;
    (*__pPage_ref.pLruNext).pLruPrev = __pPage_ref.pLruPrev;
    __pPage_ref.pLruNext = ::core::ptr::null_mut::<PgHdr1>();
    (*__pPage_ref.pCache).nRecyclable = (*__pPage_ref.pCache).nRecyclable.wrapping_sub(1);
    pPage
}

unsafe extern "C" fn pcache1RemoveFromHash(
    pPage: *mut PgHdr1,
    freeFlag: ::core::ffi::c_int,
) {
    
    let pCache: *mut PCache1 = (*pPage).pCache;
    let mut pp: *mut *mut PgHdr1;
    let __pCache_ref = { &mut *pCache };
    let h: ::core::ffi::c_uint = (*pPage).iKey.wrapping_rem(__pCache_ref.nHash);
    pp = __pCache_ref.apHash.offset(h as isize) as *mut *mut PgHdr1;
    while *pp != pPage {
        pp = &raw mut (**pp).pNext;
    }
    *pp = (**pp).pNext;
    __pCache_ref.nPage = __pCache_ref.nPage.wrapping_sub(1);
    if freeFlag != 0 {
        pcache1FreePage(pPage);
    }
}

unsafe extern "C" fn pcache1EnforceMaxPage(pCache: *mut PCache1) {
    let __pCache_ref = { &mut *pCache };
    let pGroup: *mut PGroup = __pCache_ref.pGroup;
    let mut p: *mut PgHdr1;
    while (*pGroup).nPurgeable > (*pGroup).nMaxPage && {
        p = (*pGroup).lru.pLruPrev;
        (*p).isAnchor as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    } {
        pcache1PinPage(p);
        pcache1RemoveFromHash(p, 1 as ::core::ffi::c_int);
    }
    if __pCache_ref.nPage == 0 as ::core::ffi::c_uint && !__pCache_ref.pBulk.is_null() {
        crate::src::src::malloc::sqlite3_free(__pCache_ref.pBulk);
        __pCache_ref.pFree = ::core::ptr::null_mut::<PgHdr1>();
        __pCache_ref.pBulk = __pCache_ref.pFree as *mut ::core::ffi::c_void;
    }
}

unsafe extern "C" fn pcache1TruncateUnsafe(
    pCache: *mut PCache1,
    iLimit: ::core::ffi::c_uint,
) {
    let mut h: ::core::ffi::c_uint;
    let iStop: ::core::ffi::c_uint;
    if (*pCache).iMaxKey.wrapping_sub(iLimit) < (*pCache).nHash {
        let __pCache_ref = { &mut *pCache };
        h = iLimit.wrapping_rem(__pCache_ref.nHash);
        iStop = __pCache_ref.iMaxKey.wrapping_rem(__pCache_ref.nHash);
    } else {
        h = (*pCache).nHash.wrapping_div(2 as ::core::ffi::c_uint);
        iStop = h.wrapping_sub(1 as ::core::ffi::c_uint);
    }
    loop {
        let mut pp: *mut *mut PgHdr1;
        let mut pPage: *mut PgHdr1;
        pp = (*pCache).apHash.offset(h as isize) as *mut *mut PgHdr1;
        loop {
            pPage = *pp;
            if pPage.is_null() {
                break;
            }
            if (*pPage).iKey >= iLimit {
                (*pCache).nPage = (*pCache).nPage.wrapping_sub(1);
                *pp = (*pPage).pNext;
                if !(*pPage).pLruNext.is_null() {
                    pcache1PinPage(pPage);
                }
                pcache1FreePage(pPage);
            } else {
                pp = &raw mut (*pPage).pNext;
            }
        }
        if h == iStop {
            break;
        }
        h = h
            .wrapping_add(1 as ::core::ffi::c_uint)
            .wrapping_rem((*pCache).nHash);
    }
}

unsafe extern "C" fn pcache1Init(mut _NotUsed: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    ::libc::memset(
        &raw mut pcache1_g as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PCacheGlobal>() as crate::__stddef_size_t_h::SizeT,
    );
    pcache1_g.separateCache = (crate::src::src::global::sqlite3Config.pPage.is_null()
        || crate::src::src::global::sqlite3Config.bCoreMutex as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if crate::src::src::global::sqlite3Config.bCoreMutex != 0 {
        pcache1_g.grp.mutex = crate::src::src::mutex::sqlite3MutexAlloc(
            crate::src::headers::sqlite3_h::SQLITE_MUTEX_STATIC_LRU,
        );
        pcache1_g.mutex = crate::src::src::mutex::sqlite3MutexAlloc(
            crate::src::headers::sqlite3_h::SQLITE_MUTEX_STATIC_PMEM,
        );
    }
    if pcache1_g.separateCache != 0
        && crate::src::src::global::sqlite3Config.nPage != 0 as ::core::ffi::c_int
        && crate::src::src::global::sqlite3Config.pPage.is_null()
    {
        pcache1_g.nInitPage = crate::src::src::global::sqlite3Config.nPage;
    } else {
        pcache1_g.nInitPage = 0 as ::core::ffi::c_int;
    }
    pcache1_g.grp.mxPinned = 10 as ::core::ffi::c_uint;
    pcache1_g.isInit = 1 as ::core::ffi::c_int;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn pcache1Shutdown(mut _NotUsed: *mut ::core::ffi::c_void) {
    ::libc::memset(
        &raw mut pcache1_g as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PCacheGlobal>() as crate::__stddef_size_t_h::SizeT,
    );
}

unsafe extern "C" fn pcache1Create(
    szPage: ::core::ffi::c_int,
    szExtra: ::core::ffi::c_int,
    bPurgeable: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqlite3_h::Sqlite3Pcache {
    let mut pCache: *mut PCache1;
    let pGroup: *mut PGroup;
    
    let sz: crate::src::ext::rtree::rtree::I64_0 = (::core::mem::size_of::<PCache1>() as usize).wrapping_add(
        (::core::mem::size_of::<PGroup>() as usize).wrapping_mul(pcache1_g.separateCache as usize),
    ) as crate::src::ext::rtree::rtree::I64_0;
    pCache = crate::src::src::malloc::sqlite3MallocZero(sz as crate::src::ext::rtree::rtree::U64_0)
        as *mut PCache1;
    if !pCache.is_null() {
        if pcache1_g.separateCache != 0 {
            pGroup = pCache.offset(1_isize) as *mut PCache1 as *mut PGroup;
            (*pGroup).mxPinned = 10 as ::core::ffi::c_uint;
        } else {
            pGroup = &raw mut pcache1_g.grp;
        }
        if (*pGroup).lru.isAnchor as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let __pGroup_ref = { &mut *pGroup };
            __pGroup_ref.lru.isAnchor = 1 as crate::src::fts5::U16_0;
            __pGroup_ref.lru.pLruNext = &raw mut __pGroup_ref.lru;
            __pGroup_ref.lru.pLruPrev = __pGroup_ref.lru.pLruNext;
        }
        (*pCache).pGroup = pGroup;
        (*pCache).szPage = szPage;
        (*pCache).szExtra = szExtra;
        (*pCache).szAlloc = ((szPage + szExtra) as usize).wrapping_add(
            (::core::mem::size_of::<PgHdr1>() as usize).wrapping_add(7_usize)
                & !(7 as ::core::ffi::c_int) as usize,
        ) as ::core::ffi::c_int;
        (*pCache).bPurgeable = if bPurgeable != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        pcache1ResizeHash(pCache);
        if bPurgeable != 0 {
            let __pCache_ref = { &mut *pCache };
            __pCache_ref.nMin = 10 as ::core::ffi::c_uint;
            let __pGroup_ref = { &mut *pGroup };
            __pGroup_ref.nMinPage = __pGroup_ref.nMinPage.wrapping_add(__pCache_ref.nMin);
            __pGroup_ref.mxPinned = (*pGroup)
                .nMaxPage
                .wrapping_add(10 as ::core::ffi::c_uint)
                .wrapping_sub(__pGroup_ref.nMinPage);
            __pCache_ref.pnPurgeable = &raw mut __pGroup_ref.nPurgeable;
        } else {
            (*pCache).pnPurgeable = &raw mut (*pCache).nPurgeableDummy;
        }
        if (*pCache).nHash == 0 as ::core::ffi::c_uint {
            pcache1Destroy(pCache as *mut crate::src::headers::sqlite3_h::Sqlite3Pcache);
            pCache = ::core::ptr::null_mut::<PCache1>();
        }
    }
    pCache as *mut crate::src::headers::sqlite3_h::Sqlite3Pcache
}

unsafe extern "C" fn pcache1Cachesize(
    p: *mut crate::src::headers::sqlite3_h::Sqlite3Pcache,
    nMax: ::core::ffi::c_int,
) {
    let pCache: *mut PCache1 = p as *mut PCache1;
    let mut n: crate::src::ext::rtree::rtree::U32_0;
    if (*pCache).bPurgeable != 0 {
        let __pCache_ref = { &mut *pCache };
        let pGroup: *mut PGroup = __pCache_ref.pGroup;
        n = nMax as crate::src::ext::rtree::rtree::U32_0;
        let __pGroup_ref = { &mut *pGroup };
        if n > (0x7fff0000 as crate::src::ext::rtree::rtree::U32_0)
            .wrapping_sub(__pGroup_ref.nMaxPage as crate::src::ext::rtree::rtree::U32_0)
            .wrapping_add(__pCache_ref.nMax as crate::src::ext::rtree::rtree::U32_0)
        {
            n = (0x7fff0000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_sub(__pGroup_ref.nMaxPage)
                .wrapping_add(__pCache_ref.nMax)
                as crate::src::ext::rtree::rtree::U32_0;
        }
        __pGroup_ref.nMaxPage = (*pGroup).nMaxPage.wrapping_add(
            n.wrapping_sub(__pCache_ref.nMax as crate::src::ext::rtree::rtree::U32_0)
                as ::core::ffi::c_uint,
        );
        __pGroup_ref.mxPinned = (*pGroup)
            .nMaxPage
            .wrapping_add(10 as ::core::ffi::c_uint)
            .wrapping_sub(__pGroup_ref.nMinPage);
        __pCache_ref.nMax = n as ::core::ffi::c_uint;
        __pCache_ref.n90pct = (*pCache)
            .nMax
            .wrapping_mul(9 as ::core::ffi::c_uint)
            .wrapping_div(10 as ::core::ffi::c_uint);
        pcache1EnforceMaxPage(pCache);
    }
}

unsafe extern "C" fn pcache1Shrink(p: *mut crate::src::headers::sqlite3_h::Sqlite3Pcache) {
    let pCache: *mut PCache1 = p as *mut PCache1;
    if (*pCache).bPurgeable != 0 {
        let pGroup: *mut PGroup = (*pCache).pGroup;
        
        let __pGroup_ref = { &mut *pGroup };
        let savedMaxPage: ::core::ffi::c_uint = __pGroup_ref.nMaxPage;
        __pGroup_ref.nMaxPage = 0 as ::core::ffi::c_uint;
        pcache1EnforceMaxPage(pCache);
        __pGroup_ref.nMaxPage = savedMaxPage;
    }
}

unsafe extern "C" fn pcache1Pagecount(
    p: *mut crate::src::headers::sqlite3_h::Sqlite3Pcache,
) -> ::core::ffi::c_int {
    
    let pCache = &*(p as *mut PCache1);
    let n: ::core::ffi::c_int = pCache.nPage as ::core::ffi::c_int;
    n
}
#[inline(never)]
unsafe extern "C" fn pcache1FetchStage2(
    pCache: *mut PCache1,
    iKey: ::core::ffi::c_uint,
    createFlag: ::core::ffi::c_int,
) -> *mut PgHdr1 {
    
    let __pCache_ref = { &mut *pCache };
    let pGroup: *mut PGroup = __pCache_ref.pGroup;
    let mut pPage: *mut PgHdr1 = ::core::ptr::null_mut::<PgHdr1>();
    let nPinned: ::core::ffi::c_uint = __pCache_ref.nPage.wrapping_sub(__pCache_ref.nRecyclable);
    if createFlag == 1 as ::core::ffi::c_int
        && (nPinned >= (*pGroup).mxPinned
            || nPinned >= __pCache_ref.n90pct
            || pcache1UnderMemoryPressure(pCache) != 0 && __pCache_ref.nRecyclable < nPinned)
    {
        return ::core::ptr::null_mut::<PgHdr1>();
    }
    if __pCache_ref.nPage >= __pCache_ref.nHash {
        pcache1ResizeHash(pCache);
    }
    if __pCache_ref.bPurgeable != 0
        && (*(*pGroup).lru.pLruPrev).isAnchor == 0
        && (__pCache_ref.nPage.wrapping_add(1 as ::core::ffi::c_uint) >= __pCache_ref.nMax
            || pcache1UnderMemoryPressure(pCache) != 0)
    {
        
        pPage = (*pGroup).lru.pLruPrev;
        pcache1RemoveFromHash(pPage, 0 as ::core::ffi::c_int);
        pcache1PinPage(pPage);
        let pOther: *mut PCache1 = (*pPage).pCache;
        if (*pOther).szAlloc != __pCache_ref.szAlloc {
            pcache1FreePage(pPage);
            pPage = ::core::ptr::null_mut::<PgHdr1>();
        } else {
            (*pGroup).nPurgeable = (*pGroup).nPurgeable.wrapping_sub(
                ((*pOther).bPurgeable - __pCache_ref.bPurgeable) as ::core::ffi::c_uint,
            );
        }
    }
    if pPage.is_null() {
        pPage = pcache1AllocPage(
            pCache,
            (createFlag == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
    }
    if !pPage.is_null() {
        let h: ::core::ffi::c_uint = iKey.wrapping_rem(__pCache_ref.nHash);
        __pCache_ref.nPage = __pCache_ref.nPage.wrapping_add(1);
        let __pPage_ref = { &mut *pPage };
        __pPage_ref.iKey = iKey;
        __pPage_ref.pNext = *__pCache_ref.apHash.offset(h as isize);
        __pPage_ref.pCache = pCache;
        __pPage_ref.pLruNext = ::core::ptr::null_mut::<PgHdr1>();
        let fresh2 = &mut *(__pPage_ref.page.pExtra as *mut *mut ::core::ffi::c_void);
        *fresh2 = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let fresh3 = &mut *__pCache_ref.apHash.offset(h as isize);
        *fresh3 = pPage;
        if iKey > __pCache_ref.iMaxKey {
            __pCache_ref.iMaxKey = iKey;
        }
    }
    pPage
}

unsafe extern "C" fn pcache1FetchNoMutex(
    p: *mut crate::src::headers::sqlite3_h::Sqlite3Pcache,
    iKey: ::core::ffi::c_uint,
    createFlag: ::core::ffi::c_int,
) -> *mut PgHdr1 {
    let pCache: *mut PCache1 = p as *mut PCache1;
    let mut pPage: *mut PgHdr1;
    pPage = *(*pCache)
        .apHash
        .offset(iKey.wrapping_rem((*pCache).nHash) as isize);
    while !pPage.is_null() && (*pPage).iKey != iKey {
        pPage = (*pPage).pNext;
    }
    if !pPage.is_null() {
        if !(*pPage).pLruNext.is_null() {
            pcache1PinPage(pPage)
        } else {
            pPage
        }
    } else if createFlag != 0 {
        pcache1FetchStage2(pCache, iKey, createFlag)
    } else {
        ::core::ptr::null_mut::<PgHdr1>()
    }
}

unsafe extern "C" fn pcache1Fetch(
    p: *mut crate::src::headers::sqlite3_h::Sqlite3Pcache,
    iKey: ::core::ffi::c_uint,
    createFlag: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page {
    pcache1FetchNoMutex(p, iKey, createFlag)
        as *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page
}

unsafe extern "C" fn pcache1Unpin(
    p: *mut crate::src::headers::sqlite3_h::Sqlite3Pcache,
    pPg: *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page,
    reuseUnlikely: ::core::ffi::c_int,
) {
    let pCache = &mut *(p as *mut PCache1);
    let pPage: *mut PgHdr1 = pPg as *mut PgHdr1;
    let pGroup: *mut PGroup = pCache.pGroup;
    if reuseUnlikely != 0 || (*pGroup).nPurgeable > (*pGroup).nMaxPage {
        pcache1RemoveFromHash(pPage, 1 as ::core::ffi::c_int);
    } else {
        let ppFirst: *mut *mut PgHdr1 = &raw mut (*pGroup).lru.pLruNext;
        let __pPage_ref = { &mut *pPage };
        __pPage_ref.pLruPrev = &raw mut (*pGroup).lru;
        __pPage_ref.pLruNext = *ppFirst;
        (*__pPage_ref.pLruNext).pLruPrev = pPage;
        *ppFirst = pPage;
        pCache.nRecyclable = pCache.nRecyclable.wrapping_add(1);
    };
}

unsafe extern "C" fn pcache1Rekey(
    p: *mut crate::src::headers::sqlite3_h::Sqlite3Pcache,
    pPg: *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page,
    iOld: ::core::ffi::c_uint,
    iNew: ::core::ffi::c_uint,
) {
    let pCache: *mut PCache1 = p as *mut PCache1;
    let pPage: *mut PgHdr1 = pPg as *mut PgHdr1;
    let mut pp: *mut *mut PgHdr1;
    
    
    let __pCache_ref = { &mut *pCache };
    let hOld: ::core::ffi::c_uint = iOld.wrapping_rem(__pCache_ref.nHash);
    pp = __pCache_ref.apHash.offset(hOld as isize) as *mut *mut PgHdr1;
    while *pp != pPage {
        pp = &raw mut (**pp).pNext;
    }
    let __pPage_ref = { &mut *pPage };
    *pp = __pPage_ref.pNext;
    let hNew: ::core::ffi::c_uint = iNew.wrapping_rem(__pCache_ref.nHash);
    __pPage_ref.iKey = iNew;
    __pPage_ref.pNext = *__pCache_ref.apHash.offset(hNew as isize);
    let fresh1 = &mut *__pCache_ref.apHash.offset(hNew as isize);
    *fresh1 = pPage;
    if iNew > __pCache_ref.iMaxKey {
        __pCache_ref.iMaxKey = iNew;
    }
}

unsafe extern "C" fn pcache1Truncate(
    p: *mut crate::src::headers::sqlite3_h::Sqlite3Pcache,
    iLimit: ::core::ffi::c_uint,
) {
    let pCache: *mut PCache1 = p as *mut PCache1;
    if iLimit <= (*pCache).iMaxKey {
        pcache1TruncateUnsafe(pCache, iLimit);
        (*pCache).iMaxKey = iLimit.wrapping_sub(1 as ::core::ffi::c_uint);
    }
}

unsafe extern "C" fn pcache1Destroy(p: *mut crate::src::headers::sqlite3_h::Sqlite3Pcache) {
    let pCache: *mut PCache1 = p as *mut PCache1;
    let __pCache_ref = { &*pCache };
    let pGroup: *mut PGroup = __pCache_ref.pGroup;
    if __pCache_ref.nPage != 0 {
        pcache1TruncateUnsafe(pCache, 0 as ::core::ffi::c_uint);
    }
    let __pGroup_ref = { &mut *pGroup };
    __pGroup_ref.nMaxPage = __pGroup_ref.nMaxPage.wrapping_sub(__pCache_ref.nMax);
    __pGroup_ref.nMinPage = __pGroup_ref.nMinPage.wrapping_sub(__pCache_ref.nMin);
    __pGroup_ref.mxPinned = (*pGroup)
        .nMaxPage
        .wrapping_add(10 as ::core::ffi::c_uint)
        .wrapping_sub(__pGroup_ref.nMinPage);
    pcache1EnforceMaxPage(pCache);
    crate::src::src::malloc::sqlite3_free(__pCache_ref.pBulk);
    crate::src::src::malloc::sqlite3_free(__pCache_ref.apHash as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(pCache as *mut ::core::ffi::c_void);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3PCacheSetDefault() {
    static mut defaultMethods: crate::src::headers::sqlite3_h::sqlite3_pcache_methods2 = {
        crate::src::headers::sqlite3_h::sqlite3_pcache_methods2 {
    iVersion:  1 as ::core::ffi::c_int,
    pArg:  ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    xInit:  Some(
                pcache1Init as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
    xShutdown:  Some(
                pcache1Shutdown as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
    xCreate:  Some(
                pcache1Create
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> *mut crate::src::headers::sqlite3_h::Sqlite3Pcache,
            ),
    xCachesize:  Some(
                pcache1Cachesize
                    as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::Sqlite3Pcache, ::core::ffi::c_int) -> (),
            ),
    xPagecount:  Some(
                pcache1Pagecount as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::Sqlite3Pcache) -> ::core::ffi::c_int,
            ),
    xFetch:  Some(
                pcache1Fetch
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::Sqlite3Pcache,
                        ::core::ffi::c_uint,
                        ::core::ffi::c_int,
                    ) -> *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page,
            ),
    xUnpin:  Some(
                pcache1Unpin
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::Sqlite3Pcache,
                        *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
    xRekey:  Some(
                pcache1Rekey
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::Sqlite3Pcache,
                        *mut crate::src::headers::sqlite3_h::sqlite3_pcache_page,
                        ::core::ffi::c_uint,
                        ::core::ffi::c_uint,
                    ) -> (),
            ),
    xTruncate:  Some(
                pcache1Truncate
                    as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::Sqlite3Pcache, ::core::ffi::c_uint) -> (),
            ),
    xDestroy:  Some(pcache1Destroy as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::Sqlite3Pcache) -> ()),
    xShrink:  Some(pcache1Shrink as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::Sqlite3Pcache) -> ()),
}
    };
    let args: [u64; 4] = [(&raw const defaultMethods) as usize as u64, 0, 0, 0];
    crate::src::src::main::sqlite3_config_args(
        crate::src::headers::sqlite3_h::SQLITE_CONFIG_PCACHE2_1,
        args.as_ptr(),
    );
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3HeaderSizePcache1() -> ::core::ffi::c_int {
    ((::core::mem::size_of::<PgHdr1>() as usize).wrapping_add(7_usize)
        & !(7 as ::core::ffi::c_int) as usize) as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Pcache1Mutex() -> *mut crate::src::src::mutex_unix::sqlite3_mutex {
    pcache1_g.mutex
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3PcacheStats(
    pnCurrent: *mut ::core::ffi::c_int,
    pnMax: *mut ::core::ffi::c_int,
    pnMin: *mut ::core::ffi::c_int,
    pnRecyclable: *mut ::core::ffi::c_int,
) {
    let mut p: *mut PgHdr1;
    let mut nRecyclable: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    p = pcache1_g.grp.lru.pLruNext;
    while !p.is_null() && (*p).isAnchor == 0 {
        nRecyclable += 1;
        p = (*p).pLruNext;
    }
    *pnCurrent = pcache1_g.grp.nPurgeable as ::core::ffi::c_int;
    *pnMax = pcache1_g.grp.nMaxPage as ::core::ffi::c_int;
    *pnMin = pcache1_g.grp.nMinPage as ::core::ffi::c_int;
    *pnRecyclable = nRecyclable;
}
