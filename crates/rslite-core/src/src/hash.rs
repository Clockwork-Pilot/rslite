// =============== BEGIN hash_h ================
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hash {
    pub htsize: ::core::ffi::c_uint,
    pub count: ::core::ffi::c_uint,
    pub first: *mut crate::src::src::hash::HashElem,
    pub ht: *mut crate::src::src::hash::_ht,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ht {
    pub count: ::core::ffi::c_uint,
    pub chain: *mut crate::src::src::hash::HashElem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashElem {
    pub next: *mut crate::src::src::hash::HashElem,
    pub prev: *mut crate::src::src::hash::HashElem,
    pub data: *mut ::core::ffi::c_void,
    pub pKey: *const ::core::ffi::c_char,
    pub h: ::core::ffi::c_uint,
}
pub use crate::__stddef_size_t_h::SizeT;

pub use crate::src::ext::rtree::rtree::U64_0;
pub use crate::src::headers::sqlite3_h::SqliteUint64;
pub use crate::src::headers::sqliteInt_h::SQLITE_MALLOC_SOFT_LIMIT;
pub use crate::src::src::fault::sqlite3BeginBenignMalloc;
pub use crate::src::src::fault::sqlite3EndBenignMalloc;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3Malloc;
pub use crate::src::src::malloc::sqlite3MallocSize;
pub use crate::src::src::util::sqlite3StrICmp;

#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3HashInit(pNew: *mut crate::src::src::hash::Hash) {
    let __pNew_ref = unsafe { &mut *pNew };
    __pNew_ref.first = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
    __pNew_ref.count = 0 as ::core::ffi::c_uint;
    __pNew_ref.htsize = 0 as ::core::ffi::c_uint;
    __pNew_ref.ht = ::core::ptr::null_mut::<crate::src::src::hash::_ht>();
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3HashClear(pH: *mut crate::src::src::hash::Hash) {
    let mut elem: *mut crate::src::src::hash::HashElem;
    let __pH_ref = unsafe { &mut *pH };
    elem = __pH_ref.first;
    __pH_ref.first = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
    crate::src::src::malloc::sqlite3_free(__pH_ref.ht as *mut ::core::ffi::c_void);
    __pH_ref.ht = ::core::ptr::null_mut::<crate::src::src::hash::_ht>();
    __pH_ref.htsize = 0 as ::core::ffi::c_uint;
    while !elem.is_null() {
        let next_elem: *mut crate::src::src::hash::HashElem = (*elem).next;
        crate::src::src::malloc::sqlite3_free(elem as *mut ::core::ffi::c_void);
        elem = next_elem;
    }
    __pH_ref.count = 0 as ::core::ffi::c_uint;
}

unsafe extern "C" fn strHash(mut z: *const ::core::ffi::c_char) -> ::core::ffi::c_uint {
    let mut h: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while *z.offset(0 as isize) != 0 {
        let fresh0 = z;
        z = z.offset(1);
        h = h.wrapping_add(
            (0xdf as ::core::ffi::c_int & *fresh0 as ::core::ffi::c_uchar as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
        h = h.wrapping_mul(0x9e3779b1 as ::core::ffi::c_uint);
    }
    h
}

unsafe extern "C" fn insertElement(
    pH: *mut crate::src::src::hash::Hash,
    pEntry: *mut crate::src::src::hash::_ht,
    pNew: *mut crate::src::src::hash::HashElem,
) {
    let pHead: *mut crate::src::src::hash::HashElem;
    if !pEntry.is_null() {
        let __pEntry_ref = unsafe { &mut *pEntry };
        pHead = if __pEntry_ref.count != 0 {
            __pEntry_ref.chain
        } else {
            ::core::ptr::null_mut::<crate::src::src::hash::HashElem>()
        };
        __pEntry_ref.count = __pEntry_ref.count.wrapping_add(1);
        __pEntry_ref.chain = pNew;
    } else {
        pHead = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
    }
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
        (*pNew).prev = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
        __pH_ref.first = pNew;
    };
}

unsafe extern "C" fn rehash(
    pH: *mut crate::src::src::hash::Hash,
    mut new_size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let new_ht: *mut crate::src::src::hash::_ht;
    let mut elem: *mut crate::src::src::hash::HashElem;
    let mut next_elem: *mut crate::src::src::hash::HashElem;
    if (new_size as usize)
        .wrapping_mul(::core::mem::size_of::<crate::src::src::hash::_ht>() as usize)
        > crate::src::headers::sqliteInt_h::SQLITE_MALLOC_SOFT_LIMIT as usize
    {
        new_size = (crate::src::headers::sqliteInt_h::SQLITE_MALLOC_SOFT_LIMIT as usize)
            .wrapping_div(::core::mem::size_of::<crate::src::src::hash::_ht>() as usize)
            as ::core::ffi::c_uint;
    }
    let __pH_ref = unsafe { &mut *pH };
    if new_size == __pH_ref.htsize {
        return 0 as ::core::ffi::c_int;
    }
    crate::src::src::fault::sqlite3BeginBenignMalloc();
    new_ht = crate::src::src::malloc::sqlite3Malloc(
        (new_size as usize)
            .wrapping_mul(::core::mem::size_of::<crate::src::src::hash::_ht>() as usize)
            as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::src::hash::_ht;
    crate::src::src::fault::sqlite3EndBenignMalloc();
    if new_ht.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    crate::src::src::malloc::sqlite3_free(__pH_ref.ht as *mut ::core::ffi::c_void);
    __pH_ref.ht = new_ht as *mut crate::src::src::hash::_ht;
    new_size = (crate::src::src::malloc::sqlite3MallocSize(new_ht as *const ::core::ffi::c_void)
        as usize)
        .wrapping_div(::core::mem::size_of::<crate::src::src::hash::_ht>() as usize)
        as ::core::ffi::c_uint;
    __pH_ref.htsize = new_size;
    ::libc::memset(
        new_ht as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (new_size as crate::__stddef_size_t_h::SizeT)
            .wrapping_mul(::core::mem::size_of::<crate::src::src::hash::_ht>()
                as crate::__stddef_size_t_h::SizeT),
    );
    elem = __pH_ref.first;
    __pH_ref.first = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
    while !elem.is_null() {
        next_elem = (*elem).next;
        insertElement(
            pH,
            new_ht.offset((*elem).h.wrapping_rem(new_size) as isize)
                as *mut crate::src::src::hash::_ht,
            elem,
        );
        elem = next_elem;
    }
    1 as ::core::ffi::c_int
}

unsafe extern "C" fn findElementWithHash(
    pH: *const crate::src::src::hash::Hash,
    pKey: *const ::core::ffi::c_char,
    pHash: *mut ::core::ffi::c_uint,
) -> *mut crate::src::src::hash::HashElem {
    let mut elem: *mut crate::src::src::hash::HashElem;
    let mut count: ::core::ffi::c_uint;
    let h: ::core::ffi::c_uint;
    static mut nullElement: crate::src::src::hash::HashElem = crate::src::src::hash::HashElem {
        next: ::core::ptr::null::<crate::src::src::hash::HashElem>()
            as *mut crate::src::src::hash::HashElem,
        prev: ::core::ptr::null::<crate::src::src::hash::HashElem>()
            as *mut crate::src::src::hash::HashElem,
        data: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        pKey: ::core::ptr::null::<::core::ffi::c_char>(),
        h: 0 as ::core::ffi::c_uint,
    };
    h = strHash(pKey);
    if !(*pH).ht.is_null() {
        let pEntry: *mut crate::src::src::hash::_ht;
        pEntry = (*pH).ht.offset(h.wrapping_rem((*pH).htsize) as isize)
            as *mut crate::src::src::hash::_ht as *mut crate::src::src::hash::_ht;
        elem = (*pEntry).chain;
        count = (*pEntry).count;
    } else {
        elem = (*pH).first;
        count = (*pH).count;
    }
    if !pHash.is_null() {
        *pHash = h;
    }
    while count != 0 {
        if h == (*elem).h
            && crate::src::src::util::sqlite3StrICmp((*elem).pKey, pKey) == 0 as ::core::ffi::c_int
        {
            return elem;
        }
        elem = (*elem).next;
        count = count.wrapping_sub(1);
    }
    &raw mut nullElement
}

unsafe extern "C" fn removeElement(
    pH: *mut crate::src::src::hash::Hash,
    elem: *mut crate::src::src::hash::HashElem,
) {
    let pEntry: *mut crate::src::src::hash::_ht;
    let __pH_ref = unsafe { &mut *pH };
    if !(*elem).prev.is_null() {
        (*(*elem).prev).next = (*elem).next;
    } else {
        __pH_ref.first = (*elem).next;
    }
    if !(*elem).next.is_null() {
        (*(*elem).next).prev = (*elem).prev;
    }
    if !__pH_ref.ht.is_null() {
        pEntry = (*pH)
            .ht
            .offset((*elem).h.wrapping_rem(__pH_ref.htsize) as isize)
            as *mut crate::src::src::hash::_ht as *mut crate::src::src::hash::_ht;
        if (*pEntry).chain == elem {
            (*pEntry).chain = (*elem).next;
        }
        (*pEntry).count = (*pEntry).count.wrapping_sub(1);
    }
    crate::src::src::malloc::sqlite3_free(elem as *mut ::core::ffi::c_void);
    __pH_ref.count = __pH_ref.count.wrapping_sub(1);
    if __pH_ref.count == 0 as ::core::ffi::c_uint {
        sqlite3HashClear(pH);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3HashFind(
    pH: *const crate::src::src::hash::Hash,
    pKey: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    (*findElementWithHash(pH, pKey, ::core::ptr::null_mut::<::core::ffi::c_uint>())).data
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3HashInsert(
    pH: *mut crate::src::src::hash::Hash,
    pKey: *const ::core::ffi::c_char,
    data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut h: ::core::ffi::c_uint = 0;
    let elem: *mut crate::src::src::hash::HashElem;
    let new_elem: *mut crate::src::src::hash::HashElem;
    elem = findElementWithHash(pH, pKey, &raw mut h);
    if !(*elem).data.is_null() {
        let old_data: *mut ::core::ffi::c_void = (*elem).data;
        if data.is_null() {
            removeElement(pH, elem);
        } else {
            (*elem).data = data;
            (*elem).pKey = pKey;
        }
        return old_data;
    }
    if data.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    new_elem = crate::src::src::malloc::sqlite3Malloc(::core::mem::size_of::<
        crate::src::src::hash::HashElem,
    >()
        as crate::src::ext::rtree::rtree::U64_0)
        as *mut crate::src::src::hash::HashElem;
    if new_elem.is_null() {
        return data;
    }
    (*new_elem).pKey = pKey;
    (*new_elem).h = h;
    (*new_elem).data = data;
    let __pH_ref = unsafe { &mut *pH };
    __pH_ref.count = __pH_ref.count.wrapping_add(1);
    if __pH_ref.count >= 5 as ::core::ffi::c_uint
        && __pH_ref.count > (2 as ::core::ffi::c_uint).wrapping_mul(__pH_ref.htsize)
    {
        rehash(pH, __pH_ref.count.wrapping_mul(3 as ::core::ffi::c_uint));
    }
    insertElement(
        pH,
        if !__pH_ref.ht.is_null() {
            (*pH)
                .ht
                .offset((*new_elem).h.wrapping_rem(__pH_ref.htsize) as isize)
                as *mut crate::src::src::hash::_ht
        } else {
            ::core::ptr::null_mut::<crate::src::src::hash::_ht>()
        },
        new_elem,
    );
    ::core::ptr::null_mut::<::core::ffi::c_void>()
}
