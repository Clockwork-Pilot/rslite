unsafe extern "C" {
    pub type Tcl_Command_;
    pub type sqlite3;
    pub type sqlite3_backup;
    fn Tcl_GetIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
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
    fn Tcl_GetCommandInfo(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        infoPtr: *mut Tcl_CmdInfo,
    ) -> ::core::ffi::c_int;
    fn Tcl_SetCommandInfo(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        infoPtr: *const Tcl_CmdInfo,
    ) -> ::core::ffi::c_int;
    fn Tcl_SetResult(
        interp: *mut Tcl_Interp,
        result: *mut ::core::ffi::c_char,
        freeProc: Option<Tcl_FreeProc>,
    );
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
    fn sqlite3_backup_init(
        pDest: *mut sqlite3,
        zDestName: *const ::core::ffi::c_char,
        pSource: *mut sqlite3,
        zSourceName: *const ::core::ffi::c_char,
    ) -> *mut sqlite3_backup;
    fn sqlite3_backup_step(
        p: *mut sqlite3_backup,
        nPage: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_backup_finish(p: *mut sqlite3_backup) -> ::core::ffi::c_int;
    fn sqlite3_backup_remaining(p: *mut sqlite3_backup) -> ::core::ffi::c_int;
    fn sqlite3_backup_pagecount(p: *mut sqlite3_backup) -> ::core::ffi::c_int;
    fn sqlite3ErrName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn getDbPointer(
        _: *mut Tcl_Interp,
        _: *const ::core::ffi::c_char,
        _: *mut *mut sqlite3,
    ) -> ::core::ffi::c_int;
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
pub type Tcl_FreeProc = unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ();
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
pub const BACKUP_PAGECOUNT: BackupSubCommandEnum = 3;
pub const BACKUP_REMAINING: BackupSubCommandEnum = 2;
pub const BACKUP_STEP: BackupSubCommandEnum = 0;
pub const BACKUP_FINISH: BackupSubCommandEnum = 1;
pub type BackupSubCommandEnum = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BackupSubCommand {
    pub zCmd: *const ::core::ffi::c_char,
    pub eCmd: BackupSubCommandEnum,
    pub nArg: ::core::ffi::c_int,
    pub zArg: *const ::core::ffi::c_char,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_STATIC: Option<Tcl_FreeProc> = None;
unsafe extern "C" fn backupTestCmd(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aSub: [BackupSubCommand; 5] = [
            BackupSubCommand {
                zCmd: b"step\0".as_ptr() as *const ::core::ffi::c_char,
                eCmd: BACKUP_STEP,
                nArg: 1 as ::core::ffi::c_int,
                zArg: b"npage\0".as_ptr() as *const ::core::ffi::c_char,
            },
            BackupSubCommand {
                zCmd: b"finish\0".as_ptr() as *const ::core::ffi::c_char,
                eCmd: BACKUP_FINISH,
                nArg: 0 as ::core::ffi::c_int,
                zArg: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            BackupSubCommand {
                zCmd: b"remaining\0".as_ptr() as *const ::core::ffi::c_char,
                eCmd: BACKUP_REMAINING,
                nArg: 0 as ::core::ffi::c_int,
                zArg: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            BackupSubCommand {
                zCmd: b"pagecount\0".as_ptr() as *const ::core::ffi::c_char,
                eCmd: BACKUP_PAGECOUNT,
                nArg: 0 as ::core::ffi::c_int,
                zArg: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            BackupSubCommand {
                zCmd: ::core::ptr::null::<::core::ffi::c_char>(),
                eCmd: BACKUP_STEP,
                nArg: 0 as ::core::ffi::c_int,
                zArg: ::core::ptr::null::<::core::ffi::c_char>(),
            },
        ];
        let mut p: *mut sqlite3_backup = clientData as *mut sqlite3_backup;
        let mut iCmd: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        rc = Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut aSub as *mut BackupSubCommand as *const ::core::ffi::c_void,
            ::core::mem::size_of::<BackupSubCommand>() as ::core::ffi::c_int,
            b"option\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut iCmd,
        );
        if rc != TCL_OK {
            return rc;
        }
        if objc != 2 as ::core::ffi::c_int + aSub[iCmd as usize].nArg {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv as *const *mut Tcl_Obj,
                aSub[iCmd as usize].zArg,
            );
            return TCL_ERROR;
        }
        match aSub[iCmd as usize].eCmd as ::core::ffi::c_uint {
            1 => {
                let mut zCmdName: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut cmdInfo: Tcl_CmdInfo = Tcl_CmdInfo {
                    isNativeObjectProc: 0,
                    objProc: None,
                    objClientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    proc: None,
                    clientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    deleteProc: None,
                    deleteData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    namespacePtr: ::core::ptr::null_mut::<Tcl_Namespace>(),
                };
                zCmdName = Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize));
                Tcl_GetCommandInfo(interp, zCmdName, &raw mut cmdInfo);
                cmdInfo.deleteProc = None;
                Tcl_SetCommandInfo(interp, zCmdName, &raw mut cmdInfo);
                Tcl_DeleteCommand(interp, zCmdName);
                rc = sqlite3_backup_finish(p);
                Tcl_SetResult(
                    interp,
                    sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
                    TCL_STATIC,
                );
            }
            0 => {
                let mut nPage: ::core::ffi::c_int = 0;
                if TCL_OK
                    != Tcl_GetIntFromObj(
                        interp,
                        *objv.offset(2 as ::core::ffi::c_int as isize),
                        &raw mut nPage,
                    )
                {
                    return TCL_ERROR;
                }
                rc = sqlite3_backup_step(p, nPage);
                Tcl_SetResult(
                    interp,
                    sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
                    TCL_STATIC,
                );
            }
            2 => {
                Tcl_SetObjResult(interp, Tcl_NewIntObj(sqlite3_backup_remaining(p)));
            }
            3 => {
                Tcl_SetObjResult(interp, Tcl_NewIntObj(sqlite3_backup_pagecount(p)));
            }
            _ => {}
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn backupTestFinish(mut clientData: ClientData) {
    unsafe {
        let mut pBackup: *mut sqlite3_backup = clientData as *mut sqlite3_backup;
        sqlite3_backup_finish(pBackup);
    }
}
unsafe extern "C" fn backupTestInit(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBackup: *mut sqlite3_backup = ::core::ptr::null_mut::<sqlite3_backup>();
        let mut pDestDb: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pSrcDb: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zDestName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zSrcName: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zCmd: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        if objc != 6 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv as *const *mut Tcl_Obj,
                b"CMDNAME DESTHANDLE DESTNAME SRCHANDLE SRCNAME\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zCmd = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
            &raw mut pDestDb,
        );
        zDestName = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
        getDbPointer(
            interp,
            Tcl_GetString(*objv.offset(4 as ::core::ffi::c_int as isize)),
            &raw mut pSrcDb,
        );
        zSrcName = Tcl_GetString(*objv.offset(5 as ::core::ffi::c_int as isize));
        pBackup = sqlite3_backup_init(pDestDb, zDestName, pSrcDb, zSrcName);
        if pBackup.is_null() {
            Tcl_AppendResult(
                interp,
                b"sqlite3_backup_init() failed\0".as_ptr() as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        Tcl_CreateObjCommand(
            interp,
            zCmd,
            Some(
                backupTestCmd
                    as unsafe extern "C" fn(
                        ClientData,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            pBackup as ClientData,
            Some(backupTestFinish as unsafe extern "C" fn(ClientData) -> ()),
        );
        Tcl_SetObjResult(interp, *objv.offset(1 as ::core::ffi::c_int as isize));
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetestbackup_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3_backup\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                backupTestInit
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
