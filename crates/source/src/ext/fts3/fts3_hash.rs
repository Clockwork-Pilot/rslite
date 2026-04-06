



// =============== BEGIN fts3_hash_h ================
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Hash {
    pub keyClass:  ::core::ffi::c_char,
    pub copyKey:  ::core::ffi::c_char,
    pub count:  ::core::ffi::c_int,
    pub first:  *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem,
    pub htsize:  ::core::ffi::c_int,
    pub ht:  *mut crate::src::ext::fts3::fts3_hash::_fts3ht,
}
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct _fts3ht {
        pub count: ::core::ffi::c_int,
        pub chain: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct Fts3HashElem {
        pub next: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem,
        pub prev: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem,
        pub data: *mut ::core::ffi::c_void,
        pub pKey: *mut ::core::ffi::c_void,
        pub nKey: ::core::ffi::c_int,
    }
    
    pub const FTS3_HASH_STRING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub use crate::__stddef_size_t_h::size_t;


pub use crate::src::src::malloc::sqlite3_free;pub use crate::src::headers::sqlite3_h::sqlite3_int64;pub use crate::src::src::malloc::sqlite3_malloc64;pub use crate::src::headers::sqlite3_h::sqlite3_uint64;pub use crate::src::headers::sqlite3_h::sqlite_int64;pub use crate::src::headers::sqlite3_h::sqlite_uint64;

unsafe extern "C" fn fts3HashMalloc(mut n: crate::src::headers::sqlite3_h::sqlite3_int64) -> *mut ::core::ffi::c_void {
    let mut p: *mut ::core::ffi::c_void = crate::src::src::malloc::sqlite3_malloc64(n as crate::src::headers::sqlite3_h::sqlite3_uint64);
    if !p.is_null() {
        ::libc::memset(p, 0 as ::core::ffi::c_int, n as crate::__stddef_size_t_h::size_t);
    }
    p
}

unsafe extern "C" fn fts3HashFree(mut p: *mut ::core::ffi::c_void) {
    crate::src::src::malloc::sqlite3_free(p);
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3Fts3HashInit(
    mut pNew: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
    mut keyClass: ::core::ffi::c_char,
    mut copyKey: ::core::ffi::c_char,
) {
    let __pNew_ref = unsafe { &mut *pNew };
    __pNew_ref.keyClass = keyClass;
    __pNew_ref.copyKey = copyKey;
    __pNew_ref.first = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    __pNew_ref.count = 0 as ::core::ffi::c_int;
    __pNew_ref.htsize = 0 as ::core::ffi::c_int;
    __pNew_ref.ht = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::_fts3ht>();
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3Fts3HashClear(mut pH: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash) {
    let mut elem: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    let __pH_ref = unsafe { &mut *pH };
    elem = __pH_ref.first;
    __pH_ref.first = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    fts3HashFree(__pH_ref.ht as *mut ::core::ffi::c_void);
    __pH_ref.ht = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::_fts3ht>();
    __pH_ref.htsize = 0 as ::core::ffi::c_int;
    while !elem.is_null() {
        let mut next_elem: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem = (*elem).next;
        if __pH_ref.copyKey as ::core::ffi::c_int != 0 && !(*elem).pKey.is_null() {
            fts3HashFree((*elem).pKey);
        }
        fts3HashFree(elem as *mut ::core::ffi::c_void);
        elem = next_elem;
    }
    __pH_ref.count = 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn fts3StrHash(
    mut pKey: *const ::core::ffi::c_void,
    mut nKey: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut z: *const ::core::ffi::c_char = pKey as *const ::core::ffi::c_char;
    let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if nKey <= 0 as ::core::ffi::c_int {
        nKey = ::libc::strlen(z) as ::core::ffi::c_int;
    }
    while nKey > 0 as ::core::ffi::c_int {
        let fresh2 = z;
        z = z.offset(1);
        h = h << 3 as ::core::ffi::c_int ^ h ^ *fresh2 as ::core::ffi::c_uint;
        nKey -= 1;
    }
    (h & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint) as ::core::ffi::c_int
}

unsafe extern "C" fn fts3StrCompare(
    mut pKey1: *const ::core::ffi::c_void,
    mut n1: ::core::ffi::c_int,
    mut pKey2: *const ::core::ffi::c_void,
    mut n2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if n1 != n2 {
        return 1 as ::core::ffi::c_int;
    }
    ::libc::strncmp(
        pKey1 as *const ::core::ffi::c_char,
        pKey2 as *const ::core::ffi::c_char,
        n1 as crate::__stddef_size_t_h::size_t,
    )
}

unsafe extern "C" fn fts3BinHash(
    mut pKey: *const ::core::ffi::c_void,
    mut nKey: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut z: *const ::core::ffi::c_char = pKey as *const ::core::ffi::c_char;
    loop {
        let fresh0 = nKey;
        nKey -= 1;
        if !(fresh0 > 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh1 = z;
        z = z.offset(1);
        h = h << 3 as ::core::ffi::c_int ^ h ^ *fresh1 as ::core::ffi::c_int;
    }
    h & 0x7fffffff as ::core::ffi::c_int
}

unsafe extern "C" fn fts3BinCompare(
    mut pKey1: *const ::core::ffi::c_void,
    mut n1: ::core::ffi::c_int,
    mut pKey2: *const ::core::ffi::c_void,
    mut n2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if n1 != n2 {
        return 1 as ::core::ffi::c_int;
    }
    ::libc::memcmp(pKey1, pKey2, n1 as crate::__stddef_size_t_h::size_t)
}

unsafe extern "C" fn ftsHashFunction(
    mut keyClass: ::core::ffi::c_int,
) -> Option<
    unsafe extern "C" fn(*const ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
> {
    if keyClass == crate::src::ext::fts3::fts3_hash::FTS3_HASH_STRING {
        return Some(
            fts3StrHash
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        );
    } else {
        return Some(
            fts3BinHash
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        );
    };
}

unsafe extern "C" fn ftsCompareFunction(
    mut keyClass: ::core::ffi::c_int,
) -> Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        ::core::ffi::c_int,
        *const ::core::ffi::c_void,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
> {
    if keyClass == crate::src::ext::fts3::fts3_hash::FTS3_HASH_STRING {
        return Some(
            fts3StrCompare
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        );
    } else {
        return Some(
            fts3BinCompare
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        );
    };
}

unsafe extern "C" fn fts3HashInsertElement(
    mut pH: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
    mut pEntry: *mut crate::src::ext::fts3::fts3_hash::_fts3ht,
    mut pNew: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem,
) {
    let mut pHead: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    let __pEntry_ref = unsafe { &mut *pEntry };
    pHead = __pEntry_ref.chain;
    if !pHead.is_null() {
        (*pNew).next = pHead;
        let __pHead_ref = unsafe { &mut *pHead };
        (*pNew).prev = __pHead_ref.prev;
        if !__pHead_ref.prev.is_null() {
            (*__pHead_ref.prev).next = pNew;
        } else {
            (*pH).first = pNew;
        }
        __pHead_ref.prev = pNew;
    } else {
        let __pH_ref = unsafe { &mut *pH };
        (*pNew).next = __pH_ref.first;
        if !__pH_ref.first.is_null() {
            (*__pH_ref.first).prev = pNew;
        }
        (*pNew).prev = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
        __pH_ref.first = pNew;
    }
    __pEntry_ref.count += 1;
    __pEntry_ref.chain = pNew;
}

unsafe extern "C" fn fts3Rehash(
    mut pH: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
    mut new_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut new_ht: *mut crate::src::ext::fts3::fts3_hash::_fts3ht = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::_fts3ht>();
    let mut elem: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    let mut next_elem: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    let mut xHash: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
    > = None;
    new_ht = fts3HashMalloc(
        (new_size as usize).wrapping_mul(::core::mem::size_of::<crate::src::ext::fts3::fts3_hash::_fts3ht>() as usize)
            as crate::src::headers::sqlite3_h::sqlite3_int64,
    ) as *mut crate::src::ext::fts3::fts3_hash::_fts3ht;
    if new_ht.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    let __pH_ref = unsafe { &mut *pH };
    fts3HashFree(__pH_ref.ht as *mut ::core::ffi::c_void);
    __pH_ref.ht = new_ht as *mut crate::src::ext::fts3::fts3_hash::_fts3ht;
    __pH_ref.htsize = new_size;
    xHash = ftsHashFunction(__pH_ref.keyClass as ::core::ffi::c_int);
    elem = __pH_ref.first;
    __pH_ref.first = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    while !elem.is_null() {
        let mut h: ::core::ffi::c_int = Some(xHash.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*elem).pKey, (*elem).nKey
        ) & new_size - 1 as ::core::ffi::c_int;
        next_elem = (*elem).next;
        fts3HashInsertElement(pH, new_ht.offset(h as isize) as *mut crate::src::ext::fts3::fts3_hash::_fts3ht, elem);
        elem = next_elem;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn fts3FindElementByHash(
    mut pH: *const crate::src::ext::fts3::fts3_hash::Fts3Hash,
    mut pKey: *const ::core::ffi::c_void,
    mut nKey: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) -> *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem {
    let mut elem: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    let mut count: ::core::ffi::c_int = 0;
    let mut xCompare: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    > = None;
    if !(*pH).ht.is_null() {
        let mut pEntry: *mut crate::src::ext::fts3::fts3_hash::_fts3ht = (*pH).ht.offset(h as isize) as *mut crate::src::ext::fts3::fts3_hash::_fts3ht;
        elem = (*pEntry).chain;
        count = (*pEntry).count;
        xCompare = ftsCompareFunction((*pH).keyClass as ::core::ffi::c_int);
        loop {
            let fresh3 = count;
            count -= 1;
            if !(fresh3 != 0 && !elem.is_null()) {
                break;
            }
            if Some(xCompare.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*elem).pKey, (*elem).nKey, pKey, nKey
            ) == 0 as ::core::ffi::c_int
            {
                return elem;
            }
            elem = (*elem).next;
        }
    }
    ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>()
}

unsafe extern "C" fn fts3RemoveElementByHash(
    mut pH: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
    mut elem: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem,
    mut h: ::core::ffi::c_int,
) {
    let mut pEntry: *mut crate::src::ext::fts3::fts3_hash::_fts3ht = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::_fts3ht>();
    let __pH_ref = unsafe { &mut *pH };
    let __elem_ref = unsafe { &mut *elem };
    if !__elem_ref.prev.is_null() {
        (*__elem_ref.prev).next = __elem_ref.next;
    } else {
        __pH_ref.first = __elem_ref.next;
    }
    if !__elem_ref.next.is_null() {
        (*__elem_ref.next).prev = __elem_ref.prev;
    }
    pEntry = __pH_ref.ht.offset(h as isize) as *mut crate::src::ext::fts3::fts3_hash::_fts3ht as *mut crate::src::ext::fts3::fts3_hash::_fts3ht;
    if (*pEntry).chain == elem {
        (*pEntry).chain = __elem_ref.next;
    }
    (*pEntry).count -= 1;
    if (*pEntry).count <= 0 as ::core::ffi::c_int {
        (*pEntry).chain = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    }
    if __pH_ref.copyKey as ::core::ffi::c_int != 0 && !__elem_ref.pKey.is_null() {
        fts3HashFree(__elem_ref.pKey);
    }
    fts3HashFree(elem as *mut ::core::ffi::c_void);
    __pH_ref.count -= 1;
    if __pH_ref.count <= 0 as ::core::ffi::c_int {
        sqlite3Fts3HashClear(pH);
    }
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3Fts3HashFindElem(
    mut pH: *const crate::src::ext::fts3::fts3_hash::Fts3Hash,
    mut pKey: *const ::core::ffi::c_void,
    mut nKey: ::core::ffi::c_int,
) -> *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem {
    let mut h: ::core::ffi::c_int = 0;
    let mut xHash: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
    > = None;
    if pH.is_null() || (*pH).ht.is_null() {
        return ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    }
    xHash = ftsHashFunction((*pH).keyClass as ::core::ffi::c_int);
    h = Some(xHash.expect("non-null function pointer")).expect("non-null function pointer")(
        pKey, nKey,
    );
    fts3FindElementByHash(pH, pKey, nKey, h & (*pH).htsize - 1 as ::core::ffi::c_int)
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3Fts3HashFind(
    mut pH: *const crate::src::ext::fts3::fts3_hash::Fts3Hash,
    mut pKey: *const ::core::ffi::c_void,
    mut nKey: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut pElem: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    pElem = sqlite3Fts3HashFindElem(pH, pKey, nKey);
    if !pElem.is_null() {
        (*pElem).data
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    }
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3Fts3HashInsert(
    mut pH: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
    mut pKey: *const ::core::ffi::c_void,
    mut nKey: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut hraw: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut elem: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    let mut new_elem: *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem = ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>();
    let mut xHash: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
    > = None;
    let __pH_ref = unsafe { &mut *pH };
    xHash = ftsHashFunction(__pH_ref.keyClass as ::core::ffi::c_int);
    hraw = Some(xHash.expect("non-null function pointer")).expect("non-null function pointer")(
        pKey, nKey,
    );
    h = hraw & __pH_ref.htsize - 1 as ::core::ffi::c_int;
    elem = fts3FindElementByHash(pH, pKey, nKey, h);
    if !elem.is_null() {
        let mut old_data: *mut ::core::ffi::c_void = (*elem).data;
        if data.is_null() {
            fts3RemoveElementByHash(pH, elem, h);
        } else {
            (*elem).data = data;
        }
        return old_data;
    }
    if data.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if __pH_ref.htsize == 0 as ::core::ffi::c_int && fts3Rehash(pH, 8 as ::core::ffi::c_int) != 0
        || __pH_ref.count >= __pH_ref.htsize
            && fts3Rehash(pH, __pH_ref.htsize * 2 as ::core::ffi::c_int) != 0
    {
        __pH_ref.count = 0 as ::core::ffi::c_int;
        return data;
    }
    new_elem = fts3HashMalloc(::core::mem::size_of::<crate::src::ext::fts3::fts3_hash::Fts3HashElem>() as crate::src::headers::sqlite3_h::sqlite3_int64)
        as *mut crate::src::ext::fts3::fts3_hash::Fts3HashElem;
    if new_elem.is_null() {
        return data;
    }
    if __pH_ref.copyKey as ::core::ffi::c_int != 0 && !pKey.is_null() {
        let __new_elem_ref = unsafe { &mut *new_elem };
        __new_elem_ref.pKey = fts3HashMalloc(nKey as crate::src::headers::sqlite3_h::sqlite3_int64);
        if __new_elem_ref.pKey.is_null() {
            fts3HashFree(new_elem as *mut ::core::ffi::c_void);
            return data;
        }
        ::libc::memcpy(__new_elem_ref.pKey, pKey, nKey as crate::__stddef_size_t_h::size_t);
    } else {
        (*new_elem).pKey = pKey as *mut ::core::ffi::c_void;
    }
    (*new_elem).nKey = nKey;
    __pH_ref.count += 1;
    h = hraw & __pH_ref.htsize - 1 as ::core::ffi::c_int;
    fts3HashInsertElement(pH, __pH_ref.ht.offset(h as isize) as *mut crate::src::ext::fts3::fts3_hash::_fts3ht, new_elem);
    (*new_elem).data = data;
    ::core::ptr::null_mut::<::core::ffi::c_void>()
}
