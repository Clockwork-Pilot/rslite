unsafe extern "C" {
    pub type sqlite3_mutex;
    pub type sqlite3_pcache;
    pub type Tcl_Command_;
    fn sqlite3_shutdown() -> ::core::ffi::c_int;
    fn sqlite3_config(_: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn Tcl_ListObjAppendElement(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objPtr: *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewObj() -> *mut Tcl_Obj;
    fn Tcl_NewStringObj(
        bytes: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj);
    fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::core::ffi::c_char,
    );
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_mem_methods {
    pub xMalloc: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> *mut ::core::ffi::c_void,
    >,
    pub xFree: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub xRealloc: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub xSize: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub xRoundup: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xInit: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub xShutdown: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pAppData: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_mutex_methods {
    pub xMutexInit: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub xMutexEnd: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub xMutexAlloc: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> *mut sqlite3_mutex,
    >,
    pub xMutexFree: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    pub xMutexEnter: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    pub xMutexTry: Option<
        unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
    >,
    pub xMutexLeave: Option<unsafe extern "C" fn(*mut sqlite3_mutex) -> ()>,
    pub xMutexHeld: Option<
        unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
    >,
    pub xMutexNotheld: Option<
        unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
    >,
}
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
pub type ClientData = *mut ::core::ffi::c_void;
pub type Tcl_WideInt = ::core::ffi::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Interp {
    pub resultDontUse: *mut ::core::ffi::c_char,
    pub freeProcDontUse: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ()>,
    pub errorLineDontUse: ::core::ffi::c_int,
}
pub type Tcl_Command = *mut Tcl_Command_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Obj {
    pub refCount: ::core::ffi::c_int,
    pub bytes: *mut ::core::ffi::c_char,
    pub length: ::core::ffi::c_int,
    pub typePtr: *const Tcl_ObjType,
    pub internalRep: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub longValue: ::core::ffi::c_long,
    pub doubleValue: ::core::ffi::c_double,
    pub otherValuePtr: *mut ::core::ffi::c_void,
    pub wideValue: Tcl_WideInt,
    pub twoPtrValue: C2Rust_Unnamed_1,
    pub ptrAndLongRep: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub ptr: *mut ::core::ffi::c_void,
    pub value: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub ptr1: *mut ::core::ffi::c_void,
    pub ptr2: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_ObjType {
    pub name: *const ::core::ffi::c_char,
    pub freeIntRepProc: Option<Tcl_FreeInternalRepProc>,
    pub dupIntRepProc: Option<Tcl_DupInternalRepProc>,
    pub updateStringProc: Option<Tcl_UpdateStringProc>,
    pub setFromAnyProc: Option<Tcl_SetFromAnyProc>,
}
pub type Tcl_SetFromAnyProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
) -> ::core::ffi::c_int;
pub type Tcl_UpdateStringProc = unsafe extern "C" fn(*mut Tcl_Obj) -> ();
pub type Tcl_DupInternalRepProc = unsafe extern "C" fn(*mut Tcl_Obj, *mut Tcl_Obj) -> ();
pub type Tcl_FreeInternalRepProc = unsafe extern "C" fn(*mut Tcl_Obj) -> ();
pub type Tcl_CmdDeleteProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_ObjCmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *const *mut Tcl_Obj,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Wrapped {
    pub pcache: sqlite3_pcache_methods2,
    pub mem: sqlite3_mem_methods,
    pub mutex: sqlite3_mutex_methods,
    pub mem_init: ::core::ffi::c_int,
    pub mem_fail: ::core::ffi::c_int,
    pub mutex_init: ::core::ffi::c_int,
    pub mutex_fail: ::core::ffi::c_int,
    pub pcache_init: ::core::ffi::c_int,
    pub pcache_fail: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub zName: *mut ::core::ffi::c_char,
    pub xProc: Option<Tcl_ObjCmdProc>,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_MALLOC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_GETMALLOC: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_MUTEX: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_GETMUTEX: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_PCACHE2: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_CONFIG_GETPCACHE2: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut wrapped: Wrapped = Wrapped {
    pcache: sqlite3_pcache_methods2 {
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
    },
    mem: sqlite3_mem_methods {
        xMalloc: None,
        xFree: None,
        xRealloc: None,
        xSize: None,
        xRoundup: None,
        xInit: None,
        xShutdown: None,
        pAppData: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
    mutex: sqlite3_mutex_methods {
        xMutexInit: None,
        xMutexEnd: None,
        xMutexAlloc: None,
        xMutexFree: None,
        xMutexEnter: None,
        xMutexTry: None,
        xMutexLeave: None,
        xMutexHeld: None,
        xMutexNotheld: None,
    },
    mem_init: 0,
    mem_fail: 0,
    mutex_init: 0,
    mutex_fail: 0,
    pcache_init: 0,
    pcache_fail: 0,
};
unsafe extern "C" fn wrMemInit(
    mut pAppData: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if wrapped.mem_fail != 0 {
            rc = SQLITE_ERROR;
        } else {
            rc = wrapped
                .mem
                .xInit
                .expect("non-null function pointer")(wrapped.mem.pAppData);
        }
        if rc == SQLITE_OK {
            wrapped.mem_init = 1 as ::core::ffi::c_int;
        }
        return rc;
    }
}
unsafe extern "C" fn wrMemShutdown(mut pAppData: *mut ::core::ffi::c_void) {
    unsafe {
        wrapped.mem.xShutdown.expect("non-null function pointer")(wrapped.mem.pAppData);
        wrapped.mem_init = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn wrMemMalloc(mut n: ::core::ffi::c_int) -> *mut ::core::ffi::c_void {
    unsafe {
        return wrapped.mem.xMalloc.expect("non-null function pointer")(n);
    }
}
unsafe extern "C" fn wrMemFree(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        wrapped.mem.xFree.expect("non-null function pointer")(p);
    }
}
unsafe extern "C" fn wrMemRealloc(
    mut p: *mut ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return wrapped.mem.xRealloc.expect("non-null function pointer")(p, n);
    }
}
unsafe extern "C" fn wrMemSize(mut p: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    unsafe {
        return wrapped.mem.xSize.expect("non-null function pointer")(p);
    }
}
unsafe extern "C" fn wrMemRoundup(mut n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return wrapped.mem.xRoundup.expect("non-null function pointer")(n);
    }
}
unsafe extern "C" fn wrMutexInit() -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if wrapped.mutex_fail != 0 {
            rc = SQLITE_ERROR;
        } else {
            rc = wrapped.mutex.xMutexInit.expect("non-null function pointer")();
        }
        if rc == SQLITE_OK {
            wrapped.mutex_init = 1 as ::core::ffi::c_int;
        }
        return rc;
    }
}
unsafe extern "C" fn wrMutexEnd() -> ::core::ffi::c_int {
    unsafe {
        wrapped.mutex.xMutexEnd.expect("non-null function pointer")();
        wrapped.mutex_init = 0 as ::core::ffi::c_int;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn wrMutexAlloc(mut e: ::core::ffi::c_int) -> *mut sqlite3_mutex {
    unsafe {
        return wrapped.mutex.xMutexAlloc.expect("non-null function pointer")(e);
    }
}
unsafe extern "C" fn wrMutexFree(mut p: *mut sqlite3_mutex) {
    unsafe {
        wrapped.mutex.xMutexFree.expect("non-null function pointer")(p);
    }
}
unsafe extern "C" fn wrMutexEnter(mut p: *mut sqlite3_mutex) {
    unsafe {
        wrapped.mutex.xMutexEnter.expect("non-null function pointer")(p);
    }
}
unsafe extern "C" fn wrMutexTry(mut p: *mut sqlite3_mutex) -> ::core::ffi::c_int {
    unsafe {
        return wrapped.mutex.xMutexTry.expect("non-null function pointer")(p);
    }
}
unsafe extern "C" fn wrMutexLeave(mut p: *mut sqlite3_mutex) {
    unsafe {
        wrapped.mutex.xMutexLeave.expect("non-null function pointer")(p);
    }
}
unsafe extern "C" fn wrMutexHeld(mut p: *mut sqlite3_mutex) -> ::core::ffi::c_int {
    unsafe {
        return wrapped.mutex.xMutexHeld.expect("non-null function pointer")(p);
    }
}
unsafe extern "C" fn wrMutexNotheld(mut p: *mut sqlite3_mutex) -> ::core::ffi::c_int {
    unsafe {
        return wrapped.mutex.xMutexNotheld.expect("non-null function pointer")(p);
    }
}
unsafe extern "C" fn wrPCacheInit(
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if wrapped.pcache_fail != 0 {
            rc = SQLITE_ERROR;
        } else {
            rc = wrapped
                .pcache
                .xInit
                .expect("non-null function pointer")(wrapped.pcache.pArg);
        }
        if rc == SQLITE_OK {
            wrapped.pcache_init = 1 as ::core::ffi::c_int;
        }
        return rc;
    }
}
unsafe extern "C" fn wrPCacheShutdown(mut pArg: *mut ::core::ffi::c_void) {
    unsafe {
        wrapped
            .pcache
            .xShutdown
            .expect("non-null function pointer")(wrapped.pcache.pArg);
        wrapped.pcache_init = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn wrPCacheCreate(
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
) -> *mut sqlite3_pcache {
    unsafe {
        return wrapped.pcache.xCreate.expect("non-null function pointer")(a, b, c);
    }
}
unsafe extern "C" fn wrPCacheCachesize(
    mut p: *mut sqlite3_pcache,
    mut n: ::core::ffi::c_int,
) {
    unsafe {
        wrapped.pcache.xCachesize.expect("non-null function pointer")(p, n);
    }
}
unsafe extern "C" fn wrPCachePagecount(
    mut p: *mut sqlite3_pcache,
) -> ::core::ffi::c_int {
    unsafe {
        return wrapped.pcache.xPagecount.expect("non-null function pointer")(p);
    }
}
unsafe extern "C" fn wrPCacheFetch(
    mut p: *mut sqlite3_pcache,
    mut a: ::core::ffi::c_uint,
    mut b: ::core::ffi::c_int,
) -> *mut sqlite3_pcache_page {
    unsafe {
        return wrapped.pcache.xFetch.expect("non-null function pointer")(p, a, b);
    }
}
unsafe extern "C" fn wrPCacheUnpin(
    mut p: *mut sqlite3_pcache,
    mut a: *mut sqlite3_pcache_page,
    mut b: ::core::ffi::c_int,
) {
    unsafe {
        wrapped.pcache.xUnpin.expect("non-null function pointer")(p, a, b);
    }
}
unsafe extern "C" fn wrPCacheRekey(
    mut p: *mut sqlite3_pcache,
    mut a: *mut sqlite3_pcache_page,
    mut b: ::core::ffi::c_uint,
    mut c: ::core::ffi::c_uint,
) {
    unsafe {
        wrapped.pcache.xRekey.expect("non-null function pointer")(p, a, b, c);
    }
}
unsafe extern "C" fn wrPCacheTruncate(
    mut p: *mut sqlite3_pcache,
    mut a: ::core::ffi::c_uint,
) {
    unsafe {
        wrapped.pcache.xTruncate.expect("non-null function pointer")(p, a);
    }
}
unsafe extern "C" fn wrPCacheDestroy(mut p: *mut sqlite3_pcache) {
    unsafe {
        wrapped.pcache.xDestroy.expect("non-null function pointer")(p);
    }
}
unsafe extern "C" fn installInitWrappers() {
    unsafe {
        let mut mutexmethods: sqlite3_mutex_methods = sqlite3_mutex_methods {
            xMutexInit: Some(
                wrMutexInit as unsafe extern "C" fn() -> ::core::ffi::c_int,
            ),
            xMutexEnd: Some(wrMutexEnd as unsafe extern "C" fn() -> ::core::ffi::c_int),
            xMutexAlloc: Some(
                wrMutexAlloc
                    as unsafe extern "C" fn(::core::ffi::c_int) -> *mut sqlite3_mutex,
            ),
            xMutexFree: Some(
                wrMutexFree as unsafe extern "C" fn(*mut sqlite3_mutex) -> (),
            ),
            xMutexEnter: Some(
                wrMutexEnter as unsafe extern "C" fn(*mut sqlite3_mutex) -> (),
            ),
            xMutexTry: Some(
                wrMutexTry
                    as unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
            ),
            xMutexLeave: Some(
                wrMutexLeave as unsafe extern "C" fn(*mut sqlite3_mutex) -> (),
            ),
            xMutexHeld: Some(
                wrMutexHeld
                    as unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
            ),
            xMutexNotheld: Some(
                wrMutexNotheld
                    as unsafe extern "C" fn(*mut sqlite3_mutex) -> ::core::ffi::c_int,
            ),
        };
        let mut pcachemethods: sqlite3_pcache_methods2 = sqlite3_pcache_methods2 {
            iVersion: 1 as ::core::ffi::c_int,
            pArg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            xInit: Some(
                wrPCacheInit
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            xShutdown: Some(
                wrPCacheShutdown as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
            xCreate: Some(
                wrPCacheCreate
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> *mut sqlite3_pcache,
            ),
            xCachesize: Some(
                wrPCacheCachesize
                    as unsafe extern "C" fn(
                        *mut sqlite3_pcache,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
            xPagecount: Some(
                wrPCachePagecount
                    as unsafe extern "C" fn(*mut sqlite3_pcache) -> ::core::ffi::c_int,
            ),
            xFetch: Some(
                wrPCacheFetch
                    as unsafe extern "C" fn(
                        *mut sqlite3_pcache,
                        ::core::ffi::c_uint,
                        ::core::ffi::c_int,
                    ) -> *mut sqlite3_pcache_page,
            ),
            xUnpin: Some(
                wrPCacheUnpin
                    as unsafe extern "C" fn(
                        *mut sqlite3_pcache,
                        *mut sqlite3_pcache_page,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
            xRekey: Some(
                wrPCacheRekey
                    as unsafe extern "C" fn(
                        *mut sqlite3_pcache,
                        *mut sqlite3_pcache_page,
                        ::core::ffi::c_uint,
                        ::core::ffi::c_uint,
                    ) -> (),
            ),
            xTruncate: Some(
                wrPCacheTruncate
                    as unsafe extern "C" fn(
                        *mut sqlite3_pcache,
                        ::core::ffi::c_uint,
                    ) -> (),
            ),
            xDestroy: Some(
                wrPCacheDestroy as unsafe extern "C" fn(*mut sqlite3_pcache) -> (),
            ),
            xShrink: None,
        };
        let mut memmethods: sqlite3_mem_methods = sqlite3_mem_methods {
            xMalloc: Some(
                wrMemMalloc
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                    ) -> *mut ::core::ffi::c_void,
            ),
            xFree: Some(
                wrMemFree as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
            xRealloc: Some(
                wrMemRealloc
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                    ) -> *mut ::core::ffi::c_void,
            ),
            xSize: Some(
                wrMemSize
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            xRoundup: Some(
                wrMemRoundup
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
            xInit: Some(
                wrMemInit
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            xShutdown: Some(
                wrMemShutdown as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
            pAppData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        memset(
            &raw mut wrapped as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Wrapped>() as size_t,
        );
        sqlite3_shutdown();
        sqlite3_config(SQLITE_CONFIG_GETMUTEX, &raw mut wrapped.mutex);
        sqlite3_config(SQLITE_CONFIG_GETMALLOC, &raw mut wrapped.mem);
        sqlite3_config(SQLITE_CONFIG_GETPCACHE2, &raw mut wrapped.pcache);
        sqlite3_config(SQLITE_CONFIG_MUTEX, &raw mut mutexmethods);
        sqlite3_config(SQLITE_CONFIG_MALLOC, &raw mut memmethods);
        sqlite3_config(SQLITE_CONFIG_PCACHE2, &raw mut pcachemethods);
    }
}
unsafe extern "C" fn init_wrapper_install(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        installInitWrappers();
        i = 1 as ::core::ffi::c_int;
        while i < objc {
            let mut z: *mut ::core::ffi::c_char = Tcl_GetString(
                *objv.offset(i as isize),
            );
            if strcmp(z, b"mem\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                wrapped.mem_fail = 1 as ::core::ffi::c_int;
            } else if strcmp(z, b"mutex\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                wrapped.mutex_fail = 1 as ::core::ffi::c_int;
            } else if strcmp(z, b"pcache\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                wrapped.pcache_fail = 1 as ::core::ffi::c_int;
            } else {
                Tcl_AppendResult(
                    interp,
                    b"Unknown argument: \"\0".as_ptr() as *const ::core::ffi::c_char,
                    z,
                    b"\"\0".as_ptr() as *const ::core::ffi::c_char,
                    NULL,
                );
                return TCL_ERROR;
            }
            i += 1;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn init_wrapper_uninstall(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        sqlite3_shutdown();
        sqlite3_config(SQLITE_CONFIG_MUTEX, &raw mut wrapped.mutex);
        sqlite3_config(SQLITE_CONFIG_MALLOC, &raw mut wrapped.mem);
        sqlite3_config(SQLITE_CONFIG_PCACHE2, &raw mut wrapped.pcache);
        return TCL_OK;
    }
}
unsafe extern "C" fn init_wrapper_clear(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        wrapped.mem_fail = 0 as ::core::ffi::c_int;
        wrapped.mutex_fail = 0 as ::core::ffi::c_int;
        wrapped.pcache_fail = 0 as ::core::ffi::c_int;
        return TCL_OK;
    }
}
unsafe extern "C" fn init_wrapper_query(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pRet: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        pRet = Tcl_NewObj();
        if wrapped.mutex_init != 0 {
            Tcl_ListObjAppendElement(
                interp,
                pRet,
                Tcl_NewStringObj(
                    b"mutex\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                ),
            );
        }
        if wrapped.mem_init != 0 {
            Tcl_ListObjAppendElement(
                interp,
                pRet,
                Tcl_NewStringObj(
                    b"mem\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                ),
            );
        }
        if wrapped.pcache_init != 0 {
            Tcl_ListObjAppendElement(
                interp,
                pRet,
                Tcl_NewStringObj(
                    b"pcache\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                ),
            );
        }
        Tcl_SetObjResult(interp, pRet);
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetest_init_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aObjCmd: [C2Rust_Unnamed_2; 4] = unsafe {
            [
                C2Rust_Unnamed_2 {
                    zName: b"init_wrapper_install\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        init_wrapper_install
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_2 {
                    zName: b"init_wrapper_query\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        init_wrapper_query
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_2 {
                    zName: b"init_wrapper_uninstall\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        init_wrapper_uninstall
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_2 {
                    zName: b"init_wrapper_clear\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        init_wrapper_clear
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
            ]
        };
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[C2Rust_Unnamed_2; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_2>() as usize)
        {
            Tcl_CreateObjCommand(
                interp,
                aObjCmd[i as usize].zName,
                aObjCmd[i as usize].xProc,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            );
            i += 1;
        }
        return TCL_OK;
    }
}
