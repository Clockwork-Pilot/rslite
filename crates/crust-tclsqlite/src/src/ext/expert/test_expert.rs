unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3expert;
    pub type Tcl_Command_;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_expert_new(
        db: *mut sqlite3,
        pzErr: *mut *mut ::core::ffi::c_char,
    ) -> *mut sqlite3expert;
    fn sqlite3_expert_sql(
        p: *mut sqlite3expert,
        zSql: *const ::core::ffi::c_char,
        pzErr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_expert_analyze(
        p: *mut sqlite3expert,
        pzErr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_expert_count(_: *mut sqlite3expert) -> ::core::ffi::c_int;
    fn sqlite3_expert_report(
        _: *mut sqlite3expert,
        iStmt: ::core::ffi::c_int,
        eReport: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3_expert_destroy(_: *mut sqlite3expert);
    fn Tcl_GetIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewIntObj(intValue: ::core::ffi::c_int) -> *mut Tcl_Obj;
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
pub struct Subcmd {
    pub zSub: *const ::core::ffi::c_char,
    pub nArg: ::core::ffi::c_int,
    pub zMsg: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cmd {
    pub zCmd: *const ::core::ffi::c_char,
    pub xProc: Option<Tcl_ObjCmdProc>,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn dbHandleFromObj(
    mut interp: *mut Tcl_Interp,
    mut pObj: *mut Tcl_Obj,
    mut pDb: *mut *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
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
        if 0 as ::core::ffi::c_int
            == Tcl_GetCommandInfo(interp, Tcl_GetString(pObj), &raw mut info)
        {
            Tcl_AppendResult(
                interp,
                b"no such handle: \0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(pObj),
                NULL,
            );
            return TCL_ERROR;
        }
        *pDb = *(info.objClientData as *mut *mut sqlite3);
        return TCL_OK;
    }
}
unsafe extern "C" fn testExpertCmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pExpert: *mut sqlite3expert = clientData as *mut sqlite3expert;
        let mut aSub: [Subcmd; 6] = [
            Subcmd {
                zSub: b"sql\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"TABLE\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Subcmd {
                zSub: b"analyze\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Subcmd {
                zSub: b"count\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Subcmd {
                zSub: b"report\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 2 as ::core::ffi::c_int,
                zMsg: b"STMT EREPORT\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Subcmd {
                zSub: b"destroy\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Subcmd {
                zSub: ::core::ptr::null::<::core::ffi::c_char>(),
                nArg: 0,
                zMsg: ::core::ptr::null::<::core::ffi::c_char>(),
            },
        ];
        let mut iSub: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = TCL_OK;
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SUBCOMMAND ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        rc = Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut aSub as *mut Subcmd as *const ::core::ffi::c_void,
            ::core::mem::size_of::<Subcmd>() as ::core::ffi::c_int,
            b"sub-command\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut iSub,
        );
        if rc != TCL_OK {
            return rc;
        }
        if objc != 2 as ::core::ffi::c_int + aSub[iSub as usize].nArg {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                aSub[iSub as usize].zMsg,
            );
            return TCL_ERROR;
        }
        match iSub {
            0 => {
                let mut zArg: *mut ::core::ffi::c_char = Tcl_GetString(
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                );
                rc = sqlite3_expert_sql(pExpert, zArg, &raw mut zErr);
            }
            1 => {
                rc = sqlite3_expert_analyze(pExpert, &raw mut zErr);
            }
            2 => {
                let mut n: ::core::ffi::c_int = sqlite3_expert_count(pExpert);
                Tcl_SetObjResult(interp, Tcl_NewIntObj(n));
            }
            3 => {
                let mut aEnum: [*const ::core::ffi::c_char; 5] = [
                    b"sql\0".as_ptr() as *const ::core::ffi::c_char,
                    b"indexes\0".as_ptr() as *const ::core::ffi::c_char,
                    b"plan\0".as_ptr() as *const ::core::ffi::c_char,
                    b"candidates\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                ];
                let mut iEnum: ::core::ffi::c_int = 0;
                let mut iStmt: ::core::ffi::c_int = 0;
                let mut zReport: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iStmt,
                ) != 0
                    || Tcl_GetIndexFromObjStruct(
                        interp,
                        *objv.offset(3 as ::core::ffi::c_int as isize),
                        &raw mut aEnum as *mut *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<*mut ::core::ffi::c_char>()
                            as ::core::ffi::c_int,
                        b"report\0".as_ptr() as *const ::core::ffi::c_char,
                        0 as ::core::ffi::c_int,
                        &raw mut iEnum,
                    ) != 0
                {
                    return TCL_ERROR;
                }
                zReport = sqlite3_expert_report(
                    pExpert,
                    iStmt,
                    1 as ::core::ffi::c_int + iEnum,
                );
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewStringObj(zReport, -1 as ::core::ffi::c_int),
                );
            }
            _ => {
                Tcl_DeleteCommand(
                    interp,
                    Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                );
            }
        }
        if rc != TCL_OK {
            if !zErr.is_null() {
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewStringObj(zErr, -1 as ::core::ffi::c_int),
                );
            } else {
                unsafe extern "C" {
                    #[link_name = "sqlite3ErrName"]
                    fn sqlite3ErrName_0(
                        _: ::core::ffi::c_int,
                    ) -> *const ::core::ffi::c_char;
                }
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewStringObj(sqlite3ErrName_0(rc), -1 as ::core::ffi::c_int),
                );
            }
        }
        sqlite3_free(zErr as *mut ::core::ffi::c_void);
        return rc;
    }
}
unsafe extern "C" fn testExpertDel(mut clientData: *mut ::core::ffi::c_void) {
    unsafe {
        let mut pExpert: *mut sqlite3expert = clientData as *mut sqlite3expert;
        sqlite3_expert_destroy(pExpert);
    }
}
unsafe extern "C" fn test_sqlite3_expert_new(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        static mut iCmd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zCmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pExpert: *mut sqlite3expert = ::core::ptr::null_mut::<sqlite3expert>();
        let mut rc: ::core::ffi::c_int = TCL_OK;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if dbHandleFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        iCmd += 1;
        zCmd = sqlite3_mprintf(
            b"sqlite3expert%d\0".as_ptr() as *const ::core::ffi::c_char,
            iCmd,
        );
        if zCmd.is_null() {
            Tcl_AppendResult(
                interp,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        pExpert = sqlite3_expert_new(db, &raw mut zErr);
        if pExpert.is_null() {
            Tcl_AppendResult(
                interp,
                zErr,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            rc = TCL_ERROR;
        } else {
            let mut p: *mut ::core::ffi::c_void = pExpert as *mut ::core::ffi::c_void;
            Tcl_CreateObjCommand(
                interp,
                zCmd,
                Some(
                    testExpertCmd
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
                p as ClientData,
                Some(
                    testExpertDel as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
            Tcl_SetObjResult(interp, Tcl_NewStringObj(zCmd, -1 as ::core::ffi::c_int));
        }
        sqlite3_free(zCmd as *mut ::core::ffi::c_void);
        sqlite3_free(zErr as *mut ::core::ffi::c_void);
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TestExpert_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aCmd: [Cmd; 1] = [
            Cmd {
                zCmd: b"sqlite3_expert_new\0".as_ptr() as *const ::core::ffi::c_char,
                xProc: Some(
                    test_sqlite3_expert_new
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[Cmd; 1]>() as usize)
                .wrapping_div(::core::mem::size_of::<Cmd>() as usize)
        {
            let mut p: *mut Cmd = (&raw mut aCmd as *mut Cmd).offset(i as isize)
                as *mut Cmd;
            Tcl_CreateObjCommand(
                interp,
                (*p).zCmd,
                (*p).xProc,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            );
            i += 1;
        }
        return TCL_OK;
    }
}
