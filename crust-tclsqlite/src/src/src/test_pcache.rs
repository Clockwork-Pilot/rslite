unsafe extern "C" {
    pub type sqlite3_pcache;
    fn sqlite3_config(_: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
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
    pub xInit: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub xShutdown: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub xCreate: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> *mut sqlite3_pcache,
    >,
    pub xCachesize: Option<
        unsafe extern "C" fn(*mut sqlite3_pcache, ::core::ffi::c_int) -> (),
    >,
    pub xPagecount: Option<
        unsafe extern "C" fn(*mut sqlite3_pcache) -> ::core::ffi::c_int,
    >,
    pub xFetch: Option<
        unsafe extern "C" fn(
            *mut sqlite3_pcache,
            ::core::ffi::c_uint,
            ::core::ffi::c_int,
        ) -> *mut sqlite3_pcache_page,
    >,
    pub xUnpin: Option<
        unsafe extern "C" fn(
            *mut sqlite3_pcache,
            *mut sqlite3_pcache_page,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub xRekey: Option<
        unsafe extern "C" fn(
            *mut sqlite3_pcache,
            *mut sqlite3_pcache_page,
            ::core::ffi::c_uint,
            ::core::ffi::c_uint,
        ) -> (),
    >,
    pub xTruncate: Option<
        unsafe extern "C" fn(*mut sqlite3_pcache, ::core::ffi::c_uint) -> (),
    >,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> ()>,
    pub xShrink: Option<unsafe extern "C" fn(*mut sqlite3_pcache) -> ()>,
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct testpcacheGlobalType {
    pub pDummy: *mut ::core::ffi::c_void,
    pub nInstance: ::core::ffi::c_int,
    pub discardChance: ::core::ffi::c_uint,
    pub prngSeed: ::core::ffi::c_uint,
    pub highStress: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct testpcache {
    pub szPage: sqlite3_int64,
    pub szExtra: ::core::ffi::c_int,
    pub bPurgeable: ::core::ffi::c_int,
    pub nFree: ::core::ffi::c_int,
    pub nPinned: ::core::ffi::c_int,
    pub iRand: ::core::ffi::c_uint,
    pub iMagic: ::core::ffi::c_uint,
    pub a: [testpcachePage; 217],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct testpcachePage {
    pub page: sqlite3_pcache_page,
    pub key: ::core::ffi::c_uint,
    pub isPinned: ::core::ffi::c_int,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_PCACHE2: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_GETPCACHE2: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
static mut testpcacheGlobal: testpcacheGlobalType = testpcacheGlobalType {
    pDummy: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    nInstance: 0,
    discardChance: 0,
    prngSeed: 0,
    highStress: 0,
};
unsafe extern "C" fn testpcacheInit(
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        testpcacheGlobal.pDummy = sqlite3_malloc(10 as ::core::ffi::c_int);
        return if testpcacheGlobal.pDummy.is_null() { SQLITE_NOMEM } else { SQLITE_OK };
    }
}
unsafe extern "C" fn testpcacheShutdown(mut pArg: *mut ::core::ffi::c_void) {
    unsafe {
        sqlite3_free(testpcacheGlobal.pDummy);
        testpcacheGlobal.pDummy = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
}
pub const TESTPCACHE_NPAGE: ::core::ffi::c_int = 217 as ::core::ffi::c_int;
pub const TESTPCACHE_RESERVE: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const TESTPCACHE_VALID: ::core::ffi::c_int = 0x364585fd as ::core::ffi::c_int;
pub const TESTPCACHE_CLEAR: ::core::ffi::c_uint = 0xd42670d4 as ::core::ffi::c_uint;
unsafe extern "C" fn testpcacheRandom(mut p: *mut testpcache) -> ::core::ffi::c_uint {
    unsafe {
        let mut x: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            (*p).iRand = (*p)
                .iRand
                .wrapping_mul(69069 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_add(5 as ::core::ffi::c_uint);
            x = x << 8 as ::core::ffi::c_int
                | (*p).iRand >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint;
            i += 1;
        }
        return x;
    }
}
unsafe extern "C" fn testpcacheCreate(
    mut szPage: ::core::ffi::c_int,
    mut szExtra: ::core::ffi::c_int,
    mut bPurgeable: ::core::ffi::c_int,
) -> *mut sqlite3_pcache {
    unsafe {
        let mut nMem: ::core::ffi::c_int = 0;
        let mut x: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut p: *mut testpcache = ::core::ptr::null_mut::<testpcache>();
        let mut i: ::core::ffi::c_int = 0;
        szPage = szPage + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int);
        szExtra = szPage + 7 as ::core::ffi::c_int & !(7 as ::core::ffi::c_int);
        nMem = (::core::mem::size_of::<testpcache>() as usize)
            .wrapping_add((TESTPCACHE_NPAGE * (szPage + szExtra)) as usize)
            as ::core::ffi::c_int;
        p = sqlite3_malloc(nMem) as *mut testpcache;
        if p.is_null() {
            return ::core::ptr::null_mut::<sqlite3_pcache>();
        }
        x = p.offset(1 as ::core::ffi::c_int as isize) as *mut testpcache
            as *mut ::core::ffi::c_char;
        (*p).szPage = szPage as sqlite3_int64;
        (*p).szExtra = szExtra;
        (*p).nFree = TESTPCACHE_NPAGE;
        (*p).nPinned = 0 as ::core::ffi::c_int;
        (*p).iRand = testpcacheGlobal.prngSeed;
        (*p).bPurgeable = bPurgeable;
        (*p).iMagic = TESTPCACHE_VALID as ::core::ffi::c_uint;
        i = 0 as ::core::ffi::c_int;
        while i < TESTPCACHE_NPAGE {
            (*p).a[i as usize].key = 0 as ::core::ffi::c_uint;
            (*p).a[i as usize].isPinned = 0 as ::core::ffi::c_int;
            (*p).a[i as usize].page.pBuf = x as *mut ::core::ffi::c_void;
            (*p).a[i as usize].page.pExtra = x.offset(szPage as isize)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
            i += 1;
            x = x.offset((szPage + szExtra) as isize);
        }
        testpcacheGlobal.nInstance += 1;
        return p as *mut sqlite3_pcache;
    }
}
unsafe extern "C" fn testpcacheCachesize(
    mut pCache: *mut sqlite3_pcache,
    mut newSize: ::core::ffi::c_int,
) {
    unsafe {
        let mut p: *mut testpcache = pCache as *mut testpcache;
    }
}
unsafe extern "C" fn testpcachePagecount(
    mut pCache: *mut sqlite3_pcache,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut testpcache = pCache as *mut testpcache;
        return TESTPCACHE_NPAGE - (*p).nFree;
    }
}
unsafe extern "C" fn testpcacheFetch(
    mut pCache: *mut sqlite3_pcache,
    mut key: ::core::ffi::c_uint,
    mut createFlag: ::core::ffi::c_int,
) -> *mut sqlite3_pcache_page {
    unsafe {
        let mut p: *mut testpcache = pCache as *mut testpcache;
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < TESTPCACHE_NPAGE {
            if (*p).a[i as usize].key == key {
                if (*p).a[i as usize].isPinned == 0 {
                    (*p).nPinned += 1;
                    (*p).a[i as usize].isPinned = 1 as ::core::ffi::c_int;
                }
                return &raw mut (*(&raw mut (*p).a as *mut testpcachePage)
                    .offset(i as isize))
                    .page;
            }
            i += 1;
        }
        if createFlag == 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<sqlite3_pcache_page>();
        }
        if (*p).nPinned == TESTPCACHE_NPAGE {
            return ::core::ptr::null_mut::<sqlite3_pcache_page>();
        }
        if (*p).nPinned >= TESTPCACHE_NPAGE - TESTPCACHE_RESERVE
            && createFlag < 2 as ::core::ffi::c_int
        {
            return ::core::ptr::null_mut::<sqlite3_pcache_page>();
        }
        if testpcacheGlobal.highStress != 0 && createFlag < 2 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<sqlite3_pcache_page>();
        }
        if (*p).nFree > TESTPCACHE_RESERVE
            || createFlag == 2 as ::core::ffi::c_int
                && (*p).nFree > 0 as ::core::ffi::c_int
        {
            j = testpcacheRandom(p).wrapping_rem(TESTPCACHE_NPAGE as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < TESTPCACHE_NPAGE {
                if (*p).a[j as usize].key == 0 as ::core::ffi::c_uint {
                    (*p).a[j as usize].key = key;
                    (*p).a[j as usize].isPinned = 1 as ::core::ffi::c_int;
                    memset(
                        (*p).a[j as usize].page.pBuf,
                        0 as ::core::ffi::c_int,
                        (*p).szPage as size_t,
                    );
                    memset(
                        (*p).a[j as usize].page.pExtra,
                        0 as ::core::ffi::c_int,
                        (*p).szExtra as size_t,
                    );
                    (*p).nPinned += 1;
                    (*p).nFree -= 1;
                    return &raw mut (*(&raw mut (*p).a as *mut testpcachePage)
                        .offset(j as isize))
                        .page;
                }
                i += 1;
                j = (j + 1 as ::core::ffi::c_int) % TESTPCACHE_NPAGE;
            }
        }
        if (*p).bPurgeable == 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<sqlite3_pcache_page>();
        }
        j = testpcacheRandom(p).wrapping_rem(TESTPCACHE_NPAGE as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < TESTPCACHE_NPAGE {
            if (*p).a[j as usize].key > 0 as ::core::ffi::c_uint
                && (*p).a[j as usize].isPinned == 0 as ::core::ffi::c_int
            {
                (*p).a[j as usize].key = key;
                (*p).a[j as usize].isPinned = 1 as ::core::ffi::c_int;
                memset(
                    (*p).a[j as usize].page.pBuf,
                    0 as ::core::ffi::c_int,
                    (*p).szPage as size_t,
                );
                memset(
                    (*p).a[j as usize].page.pExtra,
                    0 as ::core::ffi::c_int,
                    (*p).szExtra as size_t,
                );
                (*p).nPinned += 1;
                return &raw mut (*(&raw mut (*p).a as *mut testpcachePage)
                    .offset(j as isize))
                    .page;
            }
            i += 1;
            j = (j + 1 as ::core::ffi::c_int) % TESTPCACHE_NPAGE;
        }
        return ::core::ptr::null_mut::<sqlite3_pcache_page>();
    }
}
unsafe extern "C" fn testpcacheUnpin(
    mut pCache: *mut sqlite3_pcache,
    mut pOldPage: *mut sqlite3_pcache_page,
    mut discard: ::core::ffi::c_int,
) {
    unsafe {
        let mut p: *mut testpcache = pCache as *mut testpcache;
        let mut i: ::core::ffi::c_int = 0;
        if (*p).bPurgeable != 0
            && (100 as ::core::ffi::c_uint).wrapping_sub(testpcacheGlobal.discardChance)
                <= testpcacheRandom(p).wrapping_rem(100 as ::core::ffi::c_uint)
        {
            discard = 1 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while i < TESTPCACHE_NPAGE {
            if &raw mut (*(&raw mut (*p).a as *mut testpcachePage).offset(i as isize))
                .page == pOldPage
            {
                (*p).a[i as usize].isPinned = 0 as ::core::ffi::c_int;
                (*p).nPinned -= 1;
                if discard != 0 {
                    (*p).a[i as usize].key = 0 as ::core::ffi::c_uint;
                    (*p).nFree += 1;
                }
                return;
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn testpcacheRekey(
    mut pCache: *mut sqlite3_pcache,
    mut pOldPage: *mut sqlite3_pcache_page,
    mut oldKey: ::core::ffi::c_uint,
    mut newKey: ::core::ffi::c_uint,
) {
    unsafe {
        let mut p: *mut testpcache = pCache as *mut testpcache;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < TESTPCACHE_NPAGE {
            if (*p).a[i as usize].key == newKey {
                (*p).a[i as usize].key = 0 as ::core::ffi::c_uint;
                (*p).nFree += 1;
                break;
            } else {
                i += 1;
            }
        }
        i = 0 as ::core::ffi::c_int;
        while i < TESTPCACHE_NPAGE {
            if (*p).a[i as usize].key == oldKey {
                (*p).a[i as usize].key = newKey;
                return;
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn testpcacheTruncate(
    mut pCache: *mut sqlite3_pcache,
    mut iLimit: ::core::ffi::c_uint,
) {
    unsafe {
        let mut p: *mut testpcache = pCache as *mut testpcache;
        let mut i: ::core::ffi::c_uint = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < TESTPCACHE_NPAGE as ::core::ffi::c_uint {
            if (*p).a[i as usize].key >= iLimit {
                (*p).a[i as usize].key = 0 as ::core::ffi::c_uint;
                if (*p).a[i as usize].isPinned != 0 {
                    (*p).nPinned -= 1;
                }
                (*p).nFree += 1;
            }
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn testpcacheDestroy(mut pCache: *mut sqlite3_pcache) {
    unsafe {
        let mut p: *mut testpcache = pCache as *mut testpcache;
        (*p).iMagic = TESTPCACHE_CLEAR;
        sqlite3_free(p as *mut ::core::ffi::c_void);
        testpcacheGlobal.nInstance -= 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn installTestPCache(
    mut installFlag: ::core::ffi::c_int,
    mut discardChance: ::core::ffi::c_uint,
    mut prngSeed: ::core::ffi::c_uint,
    mut highStress: ::core::ffi::c_uint,
) {
    unsafe {
        static mut testPcache: sqlite3_pcache_methods2 = unsafe {
            sqlite3_pcache_methods2 {
                iVersion: 1 as ::core::ffi::c_int,
                pArg: &raw const testpcacheGlobal as *mut testpcacheGlobalType
                    as *mut ::core::ffi::c_void,
                xInit: Some(
                    testpcacheInit
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                xShutdown: Some(
                    testpcacheShutdown
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
                xCreate: Some(
                    testpcacheCreate
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> *mut sqlite3_pcache,
                ),
                xCachesize: Some(
                    testpcacheCachesize
                        as unsafe extern "C" fn(
                            *mut sqlite3_pcache,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
                xPagecount: Some(
                    testpcachePagecount
                        as unsafe extern "C" fn(
                            *mut sqlite3_pcache,
                        ) -> ::core::ffi::c_int,
                ),
                xFetch: Some(
                    testpcacheFetch
                        as unsafe extern "C" fn(
                            *mut sqlite3_pcache,
                            ::core::ffi::c_uint,
                            ::core::ffi::c_int,
                        ) -> *mut sqlite3_pcache_page,
                ),
                xUnpin: Some(
                    testpcacheUnpin
                        as unsafe extern "C" fn(
                            *mut sqlite3_pcache,
                            *mut sqlite3_pcache_page,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
                xRekey: Some(
                    testpcacheRekey
                        as unsafe extern "C" fn(
                            *mut sqlite3_pcache,
                            *mut sqlite3_pcache_page,
                            ::core::ffi::c_uint,
                            ::core::ffi::c_uint,
                        ) -> (),
                ),
                xTruncate: Some(
                    testpcacheTruncate
                        as unsafe extern "C" fn(
                            *mut sqlite3_pcache,
                            ::core::ffi::c_uint,
                        ) -> (),
                ),
                xDestroy: Some(
                    testpcacheDestroy as unsafe extern "C" fn(*mut sqlite3_pcache) -> (),
                ),
                xShrink: None,
            }
        };
        static mut defaultPcache: sqlite3_pcache_methods2 = sqlite3_pcache_methods2 {
            iVersion: 0,
            pArg: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            xInit: None,
            xShutdown: None,
            xCreate: None,
            xCachesize: None,
            xPagecount: None,
            xFetch: None,
            xUnpin: None,
            xRekey: None,
            xTruncate: None,
            xDestroy: None,
            xShrink: None,
        };
        static mut isInstalled: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        testpcacheGlobal.discardChance = discardChance;
        testpcacheGlobal.prngSeed = prngSeed ^ prngSeed << 16 as ::core::ffi::c_int;
        testpcacheGlobal.highStress = highStress;
        if installFlag != isInstalled {
            if installFlag != 0 {
                sqlite3_config(SQLITE_CONFIG_GETPCACHE2, &raw mut defaultPcache);
                sqlite3_config(SQLITE_CONFIG_PCACHE2, &raw const testPcache);
            } else {
                sqlite3_config(SQLITE_CONFIG_PCACHE2, &raw mut defaultPcache);
            }
            isInstalled = installFlag;
        }
    }
}
