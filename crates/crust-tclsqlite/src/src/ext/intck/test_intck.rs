unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_intck;
    pub type Tcl_Command_;
    fn sqlite3_errstr(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn sqlite3_intck_open(
        db: *mut sqlite3,
        zDb: *const ::core::ffi::c_char,
        ppOut: *mut *mut sqlite3_intck,
    ) -> ::core::ffi::c_int;
    fn sqlite3_intck_close(pCk: *mut sqlite3_intck);
    fn sqlite3_intck_step(pCk: *mut sqlite3_intck) -> ::core::ffi::c_int;
    fn sqlite3_intck_message(pCk: *mut sqlite3_intck) -> *const ::core::ffi::c_char;
    fn sqlite3_intck_unlock(pCk: *mut sqlite3_intck) -> ::core::ffi::c_int;
    fn sqlite3_intck_error(
        pCk: *mut sqlite3_intck,
        pzErr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_intck_test_sql(
        pCk: *mut sqlite3_intck,
        zObj: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn Tcl_Alloc(size: ::core::ffi::c_uint) -> *mut ::core::ffi::c_char;
    fn Tcl_Free(ptr: *mut ::core::ffi::c_char);
    fn TclFreeObj(objPtr: *mut Tcl_Obj);
    fn Tcl_GetStringFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
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
    fn Tcl_GetCommandInfo(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        infoPtr: *mut Tcl_CmdInfo,
    ) -> ::core::ffi::c_int;
    fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj);
    fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::core::ffi::c_char,
    );
    fn Tcl_GetIndexFromObjStruct(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        tablePtr: *const ::core::ffi::c_void,
        offset: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        indexPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn getDbPointer(
        interp: *mut Tcl_Interp,
        zA: *const ::core::ffi::c_char,
        ppDb: *mut *mut sqlite3,
    ) -> ::core::ffi::c_int;
    fn sqlite3ErrName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
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
pub type Tcl_CmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int;
pub type Tcl_NamespaceDeleteProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_ObjCmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *const *mut Tcl_Obj,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Namespace {
    pub name: *mut ::core::ffi::c_char,
    pub fullName: *mut ::core::ffi::c_char,
    pub clientData: ClientData,
    pub deleteProc: Option<Tcl_NamespaceDeleteProc>,
    pub parentPtr: *mut Tcl_Namespace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_CmdInfo {
    pub isNativeObjectProc: ::core::ffi::c_int,
    pub objProc: Option<Tcl_ObjCmdProc>,
    pub objClientData: ClientData,
    pub proc: Option<Tcl_CmdProc>,
    pub clientData: ClientData,
    pub deleteProc: Option<Tcl_CmdDeleteProc>,
    pub deleteData: ClientData,
    pub namespacePtr: *mut Tcl_Namespace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestIntck {
    pub intck: *mut sqlite3_intck,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Subcmd {
    pub zName: *const ::core::ffi::c_char,
    pub nArg: ::core::ffi::c_int,
    pub zExpect: *const ::core::ffi::c_char,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn testIntckCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aCmd: [Subcmd; 7] = [
            Subcmd {
                zName: b"close\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zExpect: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Subcmd {
                zName: b"step\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zExpect: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Subcmd {
                zName: b"message\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zExpect: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Subcmd {
                zName: b"error\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zExpect: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Subcmd {
                zName: b"unlock\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zExpect: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Subcmd {
                zName: b"test_sql\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zExpect: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Subcmd {
                zName: ::core::ptr::null::<::core::ffi::c_char>(),
                nArg: 0 as ::core::ffi::c_int,
                zExpect: ::core::ptr::null::<::core::ffi::c_char>(),
            },
        ];
        let mut rc: ::core::ffi::c_int = TCL_OK;
        let mut iIdx: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut p: *mut TestIntck = clientData as *mut TestIntck;
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SUB-COMMAND ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        rc = Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut aCmd as *mut Subcmd as *const ::core::ffi::c_void,
            ::core::mem::size_of::<Subcmd>() as ::core::ffi::c_int,
            b"SUB-COMMAND\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut iIdx,
        );
        if rc != 0 {
            return rc;
        }
        if objc != 2 as ::core::ffi::c_int + aCmd[iIdx as usize].nArg {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                aCmd[iIdx as usize].zExpect,
            );
            return TCL_ERROR;
        }
        match iIdx {
            0 => {
                Tcl_DeleteCommand(
                    interp,
                    Tcl_GetStringFromObj(
                        *objv.offset(0 as ::core::ffi::c_int as isize),
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ),
                );
            }
            1 => {
                rc = sqlite3_intck_step((*p).intck);
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewStringObj(sqlite3ErrName(rc), -1 as ::core::ffi::c_int),
                );
            }
            2 => {
                let mut z: *const ::core::ffi::c_char = sqlite3_intck_message(
                    (*p).intck,
                );
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewStringObj(
                        if !z.is_null() {
                            z
                        } else {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        -1 as ::core::ffi::c_int,
                    ),
                );
            }
            3 => {
                let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut pRes: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
                rc = sqlite3_intck_error(
                    (*p).intck,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                pRes = Tcl_NewObj();
                Tcl_ListObjAppendElement(
                    interp,
                    pRes,
                    Tcl_NewStringObj(sqlite3ErrName(rc), -1 as ::core::ffi::c_int),
                );
                sqlite3_intck_error((*p).intck, &raw mut zErr);
                Tcl_ListObjAppendElement(
                    interp,
                    pRes,
                    Tcl_NewStringObj(
                        if !zErr.is_null() {
                            zErr
                        } else {
                            ::core::ptr::null::<::core::ffi::c_char>()
                        },
                        -1 as ::core::ffi::c_int,
                    ),
                );
                Tcl_SetObjResult(interp, pRes);
            }
            4 => {
                rc = sqlite3_intck_unlock((*p).intck);
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewStringObj(sqlite3ErrName(rc), -1 as ::core::ffi::c_int),
                );
            }
            5 => {
                let mut zObj: *const ::core::ffi::c_char = Tcl_GetString(
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                );
                let mut zSql: *const ::core::ffi::c_char = sqlite3_intck_test_sql(
                    (*p).intck,
                    if *zObj.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int != 0
                    {
                        zObj
                    } else {
                        ::core::ptr::null::<::core::ffi::c_char>()
                    },
                );
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewStringObj(zSql, -1 as ::core::ffi::c_int),
                );
            }
            _ => {}
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn testIntckFree(mut clientData: *mut ::core::ffi::c_void) {
    unsafe {
        let mut p: *mut TestIntck = clientData as *mut TestIntck;
        sqlite3_intck_close((*p).intck);
        Tcl_Free(p as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn test_sqlite3_intck(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zName: [::core::ffi::c_char; 64] = [0; 64];
        let mut iName: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut info: Tcl_CmdInfo = Tcl_CmdInfo {
            isNativeObjectProc: 0,
            objProc: None,
            objClientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            proc: None,
            clientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            deleteProc: None,
            deleteData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            namespacePtr: ::core::ptr::null_mut::<Tcl_Namespace>(),
        };
        let mut p: *mut TestIntck = ::core::ptr::null_mut::<TestIntck>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB DBNAME\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        p = Tcl_Alloc(::core::mem::size_of::<TestIntck>() as ::core::ffi::c_uint)
            as *mut ::core::ffi::c_void as *mut TestIntck;
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<TestIntck>() as size_t,
        );
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zDb = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        if *zDb.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\0' as i32
        {
            zDb = ::core::ptr::null::<::core::ffi::c_char>();
        }
        rc = sqlite3_intck_open(db, zDb, &raw mut (*p).intck);
        if rc != SQLITE_OK {
            Tcl_Free(p as *mut ::core::ffi::c_char);
            Tcl_SetObjResult(
                interp,
                Tcl_NewStringObj(sqlite3_errstr(rc), -1 as ::core::ffi::c_int),
            );
            return TCL_ERROR;
        }
        loop {
            let c2rust_fresh0 = iName;
            iName = iName + 1;
            sprintf(
                &raw mut zName as *mut ::core::ffi::c_char,
                b"intck%d\0".as_ptr() as *const ::core::ffi::c_char,
                c2rust_fresh0,
            );
            if !(Tcl_GetCommandInfo(
                interp,
                &raw mut zName as *mut ::core::ffi::c_char,
                &raw mut info,
            ) != 0 as ::core::ffi::c_int)
            {
                break;
            }
        }
        Tcl_CreateObjCommand(
            interp,
            &raw mut zName as *mut ::core::ffi::c_char,
            Some(
                testIntckCmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            p as ClientData,
            Some(testIntckFree as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        Tcl_SetObjResult(
            interp,
            Tcl_NewStringObj(
                &raw mut zName as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_do_intck(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pCk: *mut sqlite3_intck = ::core::ptr::null_mut::<sqlite3_intck>();
        let mut pRet: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB DBNAME\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zDb = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        pRet = Tcl_NewObj();
        (*pRet).refCount += 1;
        rc = sqlite3_intck_open(db, zDb, &raw mut pCk);
        if rc == SQLITE_OK {
            while sqlite3_intck_step(pCk) == SQLITE_OK {
                let mut zMsg: *const ::core::ffi::c_char = sqlite3_intck_message(pCk);
                if !zMsg.is_null() {
                    Tcl_ListObjAppendElement(
                        interp,
                        pRet,
                        Tcl_NewStringObj(zMsg, -1 as ::core::ffi::c_int),
                    );
                }
            }
            rc = sqlite3_intck_error(pCk, &raw mut zErr);
        }
        if rc != SQLITE_OK {
            if !zErr.is_null() {
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewStringObj(zErr, -1 as ::core::ffi::c_int),
                );
            } else {
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewStringObj(sqlite3ErrName(rc), -1 as ::core::ffi::c_int),
                );
            }
        } else {
            Tcl_SetObjResult(interp, pRet);
        }
        let mut _objPtr: *mut Tcl_Obj = pRet;
        let c2rust_fresh1 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh1 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        sqlite3_intck_close(pCk);
        sqlite3_intck_close(::core::ptr::null_mut::<sqlite3_intck>());
        return if rc != 0 { TCL_ERROR } else { TCL_OK };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetestintck_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3_intck\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                test_sqlite3_intck
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"test_do_intck\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                test_do_intck
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
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
