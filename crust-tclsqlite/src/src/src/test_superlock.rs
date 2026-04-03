unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type Tcl_Command_;
    fn sqlite3_close(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_exec(
        _: *mut sqlite3,
        sql: *const ::core::ffi::c_char,
        callback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
        errmsg: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_busy_handler(
        _: *mut sqlite3,
        _: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_open_v2(
        filename: *const ::core::ffi::c_char,
        ppDb: *mut *mut sqlite3,
        flags: ::core::ffi::c_int,
        zVfs: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_prepare(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_file_control(
        _: *mut sqlite3,
        zDbName: *const ::core::ffi::c_char,
        op: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn Tcl_DuplicateObj(objPtr: *mut Tcl_Obj) -> *mut Tcl_Obj;
    fn TclFreeObj(objPtr: *mut Tcl_Obj);
    fn Tcl_GetIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_ListObjAppendElement(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objPtr: *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewIntObj(intValue: ::core::ffi::c_int) -> *mut Tcl_Obj;
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_DeleteCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetObjResult(interp: *mut Tcl_Interp) -> *mut Tcl_Obj;
    fn Tcl_ResetResult(interp: *mut Tcl_Interp);
    fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj);
    fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::core::ffi::c_char,
    );
    fn Tcl_EvalObjEx(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
}
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_file {
    pub pMethods: *const sqlite3_io_methods,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_io_methods {
    pub iVersion: ::core::ffi::c_int,
    pub xClose: Option<unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int>,
    pub xRead: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xWrite: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xTruncate: Option<
        unsafe extern "C" fn(*mut sqlite3_file, sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub xSync: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xFileSize: Option<
        unsafe extern "C" fn(*mut sqlite3_file, *mut sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub xLock: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xUnlock: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xCheckReservedLock: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xFileControl: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xSectorSize: Option<
        unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
    >,
    pub xDeviceCharacteristics: Option<
        unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
    >,
    pub xShmMap: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xShmLock: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xShmBarrier: Option<unsafe extern "C" fn(*mut sqlite3_file) -> ()>,
    pub xShmUnmap: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xFetch: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            sqlite3_int64,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xUnfetch: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            sqlite3_int64,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SuperlockBusy {
    pub xBusy: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub pBusyArg: *mut ::core::ffi::c_void,
    pub nBusy: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Superlock {
    pub db: *mut sqlite3,
    pub bWal: ::core::ffi::c_int,
}
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
pub struct InterpAndScript {
    pub interp: *mut Tcl_Interp,
    pub pScript: *mut Tcl_Obj,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_BUSY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_FILE_POINTER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_SHM_UNLOCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_SHM_LOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_SHM_EXCLUSIVE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_SHM_NLOCK: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
unsafe extern "C" fn superlockBusyHandler(
    mut pCtx: *mut ::core::ffi::c_void,
    mut UNUSED: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBusy: *mut SuperlockBusy = pCtx as *mut SuperlockBusy;
        if (*pBusy).xBusy.is_none() {
            return 0 as ::core::ffi::c_int;
        }
        let c2rust_fresh0 = (*pBusy).nBusy;
        (*pBusy).nBusy = (*pBusy).nBusy + 1;
        return (*pBusy)
            .xBusy
            .expect("non-null function pointer")((*pBusy).pBusyArg, c2rust_fresh0);
    }
}
unsafe extern "C" fn superlockIsWal(mut pLock: *mut Superlock) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = sqlite3_prepare(
            (*pLock).db,
            b"PRAGMA main.journal_mode\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc != SQLITE_OK {
            return rc;
        }
        (*pLock).bWal = 0 as ::core::ffi::c_int;
        if SQLITE_ROW == sqlite3_step(pStmt) {
            let mut zMode: *const ::core::ffi::c_char = sqlite3_column_text(
                pStmt,
                0 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
            if !zMode.is_null() && strlen(zMode) == 3 as size_t
                && sqlite3_strnicmp(
                    b"wal\0".as_ptr() as *const ::core::ffi::c_char,
                    zMode,
                    3 as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
            {
                (*pLock).bWal = 1 as ::core::ffi::c_int;
            }
        }
        return sqlite3_finalize(pStmt);
    }
}
unsafe extern "C" fn superlockShmLock(
    mut fd: *mut sqlite3_file,
    mut idx: ::core::ffi::c_int,
    mut nByte: ::core::ffi::c_int,
    mut pBusy: *mut SuperlockBusy,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut xShmLock: Option<
            unsafe extern "C" fn(
                *mut sqlite3_file,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        > = (*(*fd).pMethods).xShmLock;
        loop {
            rc = xShmLock
                .expect(
                    "non-null function pointer",
                )(fd, idx, nByte, SQLITE_SHM_LOCK | SQLITE_SHM_EXCLUSIVE);
            if !(rc == SQLITE_BUSY
                && superlockBusyHandler(
                    pBusy as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                ) != 0)
            {
                break;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn superlockWalLock(
    mut db: *mut sqlite3,
    mut pBusy: *mut SuperlockBusy,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut fd: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        rc = sqlite3_file_control(
            db,
            b"main\0".as_ptr() as *const ::core::ffi::c_char,
            SQLITE_FCNTL_FILE_POINTER,
            &raw mut fd as *mut ::core::ffi::c_void,
        );
        if rc != SQLITE_OK {
            return rc;
        }
        rc = superlockShmLock(
            fd,
            2 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            pBusy,
        );
        if rc != SQLITE_OK {
            return rc;
        }
        rc = (*(*fd).pMethods)
            .xShmMap
            .expect(
                "non-null function pointer",
            )(
            fd,
            0 as ::core::ffi::c_int,
            32 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            &raw mut p,
        );
        if rc != SQLITE_OK {
            return rc;
        }
        memset(p as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, 32 as size_t);
        rc = superlockShmLock(
            fd,
            3 as ::core::ffi::c_int,
            SQLITE_SHM_NLOCK - 3 as ::core::ffi::c_int,
            pBusy,
        );
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3demo_superunlock(mut pLock: *mut ::core::ffi::c_void) {
    unsafe {
        let mut p: *mut Superlock = pLock as *mut Superlock;
        if (*p).bWal != 0 {
            let mut rc: ::core::ffi::c_int = 0;
            let mut flags: ::core::ffi::c_int = SQLITE_SHM_UNLOCK | SQLITE_SHM_EXCLUSIVE;
            let mut fd: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
            rc = sqlite3_file_control(
                (*p).db,
                b"main\0".as_ptr() as *const ::core::ffi::c_char,
                SQLITE_FCNTL_FILE_POINTER,
                &raw mut fd as *mut ::core::ffi::c_void,
            );
            if rc == SQLITE_OK {
                (*(*fd).pMethods)
                    .xShmLock
                    .expect(
                        "non-null function pointer",
                    )(fd, 2 as ::core::ffi::c_int, 1 as ::core::ffi::c_int, flags);
                (*(*fd).pMethods)
                    .xShmLock
                    .expect(
                        "non-null function pointer",
                    )(
                    fd,
                    3 as ::core::ffi::c_int,
                    SQLITE_SHM_NLOCK - 3 as ::core::ffi::c_int,
                    flags,
                );
            }
        }
        sqlite3_close((*p).db);
        sqlite3_free(p as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3demo_superlock(
    mut zPath: *const ::core::ffi::c_char,
    mut zVfs: *const ::core::ffi::c_char,
    mut xBusy: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    mut pBusyArg: *mut ::core::ffi::c_void,
    mut ppLock: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut busy: SuperlockBusy = SuperlockBusy {
            xBusy: None,
            pBusyArg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            nBusy: 0 as ::core::ffi::c_int,
        };
        let mut rc: ::core::ffi::c_int = 0;
        let mut pLock: *mut Superlock = ::core::ptr::null_mut::<Superlock>();
        pLock = sqlite3_malloc(::core::mem::size_of::<Superlock>() as ::core::ffi::c_int)
            as *mut Superlock;
        if pLock.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pLock as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Superlock>() as size_t,
        );
        rc = sqlite3_open_v2(
            zPath,
            &raw mut (*pLock).db,
            SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE,
            zVfs,
        );
        if rc == SQLITE_OK {
            busy.xBusy = xBusy;
            busy.pBusyArg = pBusyArg;
            sqlite3_busy_handler(
                (*pLock).db,
                Some(
                    superlockBusyHandler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                &raw mut busy as *mut ::core::ffi::c_void,
            );
            rc = sqlite3_exec(
                (*pLock).db,
                b"BEGIN EXCLUSIVE\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
        if rc == SQLITE_OK {
            rc = superlockIsWal(pLock);
            if SQLITE_OK == rc && (*pLock).bWal != 0 {
                rc = sqlite3_exec(
                    (*pLock).db,
                    b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
                if rc == SQLITE_OK {
                    rc = superlockWalLock((*pLock).db, &raw mut busy);
                }
            }
        }
        if rc != SQLITE_OK {
            sqlite3demo_superunlock(pLock as *mut ::core::ffi::c_void);
            *ppLock = ::core::ptr::null_mut::<::core::ffi::c_void>();
        } else {
            *ppLock = pLock as *mut ::core::ffi::c_void;
        }
        return rc;
    }
}
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_EVAL_GLOBAL: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
unsafe extern "C" fn superunlock_del(mut cd: ClientData) {
    unsafe {
        sqlite3demo_superunlock(cd);
    }
}
unsafe extern "C" fn superunlock_cmd(
    mut cd: ClientData,
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
        Tcl_DeleteCommand(
            interp,
            Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn superlock_busy(
    mut pCtx: *mut ::core::ffi::c_void,
    mut nBusy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut InterpAndScript = pCtx as *mut InterpAndScript;
        let mut pEval: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut iVal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        pEval = Tcl_DuplicateObj((*p).pScript);
        (*pEval).refCount += 1;
        Tcl_ListObjAppendElement((*p).interp, pEval, Tcl_NewIntObj(nBusy));
        Tcl_EvalObjEx((*p).interp, pEval, TCL_EVAL_GLOBAL);
        Tcl_GetIntFromObj((*p).interp, Tcl_GetObjResult((*p).interp), &raw mut iVal);
        let mut _objPtr: *mut Tcl_Obj = pEval;
        let c2rust_fresh1 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh1 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        return iVal;
    }
}
unsafe extern "C" fn superlock_cmd(
    mut cd: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pLock: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut zPath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zVfs: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut busy: InterpAndScript = InterpAndScript {
            interp: ::core::ptr::null_mut::<Tcl_Interp>(),
            pScript: ::core::ptr::null_mut::<Tcl_Obj>(),
        };
        let mut xBusy: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        > = None;
        let mut rc: ::core::ffi::c_int = 0;
        if objc < 3 as ::core::ffi::c_int || objc > 5 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"CMDNAME PATH ?VFS? ?BUSY-HANDLER-SCRIPT?\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zPath = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        if objc > 3 as ::core::ffi::c_int {
            zVfs = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
            if strlen(zVfs) == 0 as size_t {
                zVfs = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
        }
        if objc > 4 as ::core::ffi::c_int {
            busy.interp = interp;
            busy.pScript = *objv.offset(4 as ::core::ffi::c_int as isize);
            xBusy = Some(
                superlock_busy
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
        }
        rc = sqlite3demo_superlock(
            zPath,
            zVfs,
            xBusy,
            &raw mut busy as *mut ::core::ffi::c_void,
            &raw mut pLock,
        );
        if rc != SQLITE_OK {
            unsafe extern "C" {
                #[link_name = "sqlite3ErrStr"]
                fn sqlite3ErrStr_0(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
            }
            Tcl_ResetResult(interp);
            Tcl_AppendResult(interp, sqlite3ErrStr_0(rc), NULL);
            return TCL_ERROR;
        }
        Tcl_CreateObjCommand(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            Some(
                superunlock_cmd
                    as unsafe extern "C" fn(
                        ClientData,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            pLock as ClientData,
            Some(superunlock_del as unsafe extern "C" fn(ClientData) -> ()),
        );
        Tcl_SetObjResult(interp, *objv.offset(1 as ::core::ffi::c_int as isize));
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SqliteSuperlock_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3demo_superlock\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                superlock_cmd
                    as unsafe extern "C" fn(
                        ClientData,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        return TCL_OK;
    }
}
