unsafe extern "C" {
    pub type Tcl_Command_;
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_GetCommandInfo(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        infoPtr: *mut Tcl_CmdInfo,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetSlave(
        interp: *mut Tcl_Interp,
        name: *const ::core::ffi::c_char,
    ) -> *mut Tcl_Interp;
    fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::core::ffi::c_char,
    );
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
    fn getrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *mut rlimit,
    ) -> ::core::ffi::c_int;
    fn setrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit,
    ) -> ::core::ffi::c_int;
}
pub type __rlim_t = ::core::ffi::c_ulong;
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
pub type __rlimit_resource = ::core::ffi::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = ::core::ffi::c_int;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3TestInit(
    mut interp: *mut Tcl_Interp,
) -> *const ::core::ffi::c_char {
    unsafe {
        unsafe extern "C" {
            #[link_name = "Sqlite3_Init"]
            fn Sqlite3_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqliteconfig_Init"]
            fn Sqliteconfig_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest1_Init"]
            fn Sqlitetest1_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest2_Init"]
            fn Sqlitetest2_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest3_Init"]
            fn Sqlitetest3_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest4_Init"]
            fn Sqlitetest4_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest5_Init"]
            fn Sqlitetest5_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest6_Init"]
            fn Sqlitetest6_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest8_Init"]
            fn Sqlitetest8_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest9_Init"]
            fn Sqlitetest9_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest_autoext_Init"]
            fn Sqlitetest_autoext_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest_blob_Init"]
            fn Sqlitetest_blob_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest_demovfs_Init"]
            fn Sqlitetest_demovfs_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest_func_Init"]
            fn Sqlitetest_func_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest_hexio_Init"]
            fn Sqlitetest_hexio_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest_init_Init"]
            fn Sqlitetest_init_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest_malloc_Init"]
            fn Sqlitetest_malloc_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest_mutex_Init"]
            fn Sqlitetest_mutex_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetestschema_Init"]
            fn Sqlitetestschema_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetestsse_Init"]
            fn Sqlitetestsse_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetesttclvar_Init"]
            fn Sqlitetesttclvar_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetestfs_Init"]
            fn Sqlitetestfs_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "SqlitetestThread_Init"]
            fn SqlitetestThread_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "SqlitetestOnefile_Init"]
            fn SqlitetestOnefile_Init_0() -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "SqlitetestOsinst_Init"]
            fn SqlitetestOsinst_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetestbackup_Init"]
            fn Sqlitetestbackup_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetestintarray_Init"]
            fn Sqlitetestintarray_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetestvfs_Init"]
            fn Sqlitetestvfs_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetestrtree_Init"]
            fn Sqlitetestrtree_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetestrtreedoc_Init"]
            fn Sqlitetestrtreedoc_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitequota_Init"]
            fn Sqlitequota_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitemultiplex_Init"]
            fn Sqlitemultiplex_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "SqliteSuperlock_Init"]
            fn SqliteSuperlock_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "SqlitetestSyscall_Init"]
            fn SqlitetestSyscall_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "TestSession_Init"]
            fn TestSession_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Md5_Init"]
            fn Md5_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Fts5tcl_Init"]
            fn Fts5tcl_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "SqliteRbu_Init"]
            fn SqliteRbu_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetesttcl_Init"]
            fn Sqlitetesttcl_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetestfts3_Init"]
            fn Sqlitetestfts3_Init_0(interp_0: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "TestExpert_Init"]
            fn TestExpert_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetest_window_Init"]
            fn Sqlitetest_window_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetestvdbecov_Init"]
            fn Sqlitetestvdbecov_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "TestRecover_Init"]
            fn TestRecover_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
        unsafe extern "C" {
            #[link_name = "Sqlitetestintck_Init"]
            fn Sqlitetestintck_Init_0(_: *mut Tcl_Interp) -> ::core::ffi::c_int;
        }
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
        let mut x: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
        getrlimit(RLIMIT_CORE as ::core::ffi::c_int as __rlimit_resource_t, &raw mut x);
        x.rlim_cur = x.rlim_max;
        setrlimit(RLIMIT_CORE as ::core::ffi::c_int as __rlimit_resource_t, &raw mut x);
        if Tcl_GetCommandInfo(
            interp,
            b"sqlite3\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut cmdInfo,
        ) == 0 as ::core::ffi::c_int
        {
            Sqlite3_Init_0(interp);
        }
        Md5_Init_0(interp);
        Sqliteconfig_Init_0(interp);
        Sqlitetest1_Init_0(interp);
        Sqlitetest2_Init_0(interp);
        Sqlitetest3_Init_0(interp);
        Sqlitetest4_Init_0(interp);
        Sqlitetest5_Init_0(interp);
        Sqlitetest6_Init_0(interp);
        Sqlitetest8_Init_0(interp);
        Sqlitetest9_Init_0(interp);
        Sqlitetest_autoext_Init_0(interp);
        Sqlitetest_blob_Init_0(interp);
        Sqlitetest_demovfs_Init_0(interp);
        Sqlitetest_func_Init_0(interp);
        Sqlitetest_hexio_Init_0(interp);
        Sqlitetest_init_Init_0(interp);
        Sqlitetest_malloc_Init_0(interp);
        Sqlitetest_mutex_Init_0(interp);
        Sqlitetestschema_Init_0(interp);
        Sqlitetesttclvar_Init_0(interp);
        Sqlitetestfs_Init_0(interp);
        SqlitetestThread_Init_0(interp);
        SqlitetestOnefile_Init_0();
        SqlitetestOsinst_Init_0(interp);
        Sqlitetestbackup_Init_0(interp);
        Sqlitetestintarray_Init_0(interp);
        Sqlitetestvfs_Init_0(interp);
        Sqlitetestrtree_Init_0(interp);
        Sqlitetestrtreedoc_Init_0(interp);
        Sqlitequota_Init_0(interp);
        Sqlitemultiplex_Init_0(interp);
        SqliteSuperlock_Init_0(interp);
        SqlitetestSyscall_Init_0(interp);
        TestSession_Init_0(interp);
        Fts5tcl_Init_0(interp);
        SqliteRbu_Init_0(interp);
        Sqlitetesttcl_Init_0(interp);
        Sqlitetestfts3_Init_0(interp);
        TestExpert_Init_0(interp);
        Sqlitetest_window_Init_0(interp);
        Sqlitetestvdbecov_Init_0(interp);
        TestRecover_Init_0(interp);
        Sqlitetestintck_Init_0(interp);
        Tcl_CreateObjCommand(
            interp,
            b"load_testfixture_extensions\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                load_testfixture_extensions
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
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
}
unsafe extern "C" fn load_testfixture_extensions(
    mut cd: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut slave: *mut Tcl_Interp = ::core::ptr::null_mut::<Tcl_Interp>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SLAVE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        slave = Tcl_GetSlave(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
        );
        if slave.is_null() {
            return TCL_ERROR;
        }
        sqlite3TestInit(slave);
        return TCL_OK;
    }
}
