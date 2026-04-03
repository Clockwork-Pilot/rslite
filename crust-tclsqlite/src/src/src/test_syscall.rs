unsafe extern "C" {
    pub type stat;
    pub type Tcl_Command_;
    fn sqlite3_vfs_find(zVfsName: *const ::core::ffi::c_char) -> *mut sqlite3_vfs;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn TclFreeObj(objPtr: *mut Tcl_Obj);
    fn Tcl_GetBooleanFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetStringFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn Tcl_ListObjAppendElement(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objPtr: *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_ListObjGetElements(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objcPtr: *mut ::core::ffi::c_int,
        objvPtr: *mut *mut *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewIntObj(intValue: ::core::ffi::c_int) -> *mut Tcl_Obj;
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
    fn Tcl_ResetResult(interp: *mut Tcl_Interp);
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
    fn sqlite3ErrName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn __errno_location() -> *mut ::core::ffi::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type va_list = __builtin_va_list;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vfs {
    pub iVersion: ::core::ffi::c_int,
    pub szOsFile: ::core::ffi::c_int,
    pub mxPathname: ::core::ffi::c_int,
    pub pNext: *mut sqlite3_vfs,
    pub zName: *const ::core::ffi::c_char,
    pub pAppData: *mut ::core::ffi::c_void,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            sqlite3_filename,
            *mut sqlite3_file,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xDelete: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xAccess: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xFullPathname: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xDlOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub xDlError: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> (),
    >,
    pub xDlSym: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> Option<unsafe extern "C" fn() -> ()>,
    >,
    pub xDlClose: Option<
        unsafe extern "C" fn(*mut sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
    >,
    pub xRandomness: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xSleep: Option<
        unsafe extern "C" fn(*mut sqlite3_vfs, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xCurrentTime: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *mut ::core::ffi::c_double,
        ) -> ::core::ffi::c_int,
    >,
    pub xGetLastError: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xCurrentTimeInt64: Option<
        unsafe extern "C" fn(*mut sqlite3_vfs, *mut sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub xSetSystemCall: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
            sqlite3_syscall_ptr,
        ) -> ::core::ffi::c_int,
    >,
    pub xGetSystemCall: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
        ) -> sqlite3_syscall_ptr,
    >,
    pub xNextSystemCall: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vfs,
            *const ::core::ffi::c_char,
        ) -> *const ::core::ffi::c_char,
    >,
}
pub type sqlite3_syscall_ptr = Option<unsafe extern "C" fn() -> ()>;
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
pub type sqlite3_filename = *const ::core::ffi::c_char;
pub type __mode_t = ::core::ffi::c_uint;
pub type __off64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type size_t = usize;
pub type off_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type mode_t = __mode_t;
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
pub struct TestSyscallGlobal {
    pub bPersist: ::core::ffi::c_int,
    pub nCount: ::core::ffi::c_int,
    pub nFail: ::core::ffi::c_int,
    pub pgsz: ::core::ffi::c_int,
    pub orig_getpagesize: sqlite3_syscall_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestSyscallArray {
    pub zName: *const ::core::ffi::c_char,
    pub xTest: sqlite3_syscall_ptr,
    pub xOrig: sqlite3_syscall_ptr,
    pub default_errno: ::core::ffi::c_int,
    pub custom_errno: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Errno {
    pub z: *const ::core::ffi::c_char,
    pub i: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SyscallCmd {
    pub zName: *const ::core::ffi::c_char,
    pub xCmd: Option<Tcl_ObjCmdProc>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SyscallCmd_0 {
    pub zName: *const ::core::ffi::c_char,
    pub xCmd: Option<Tcl_ObjCmdProc>,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const EDEADLK: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const ENOLCK: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const EOVERFLOW: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut gSyscall: TestSyscallGlobal = TestSyscallGlobal {
    bPersist: 0 as ::core::ffi::c_int,
    nCount: 0 as ::core::ffi::c_int,
    nFail: 0 as ::core::ffi::c_int,
    pgsz: 0 as ::core::ffi::c_int,
    orig_getpagesize: None,
};
#[unsafe(no_mangle)]
pub static mut aSyscall: [TestSyscallArray; 19] = unsafe {
    [
        TestSyscallArray {
            zName: b"open\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_open
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            xOrig: None,
            default_errno: EACCES,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"close\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_close
                        as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
                ),
            ),
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"access\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_access
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"getcwd\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        size_t,
                    ) -> *mut ::core::ffi::c_char,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_getcwd
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_char,
                            size_t,
                        ) -> *mut ::core::ffi::c_char,
                ),
            ),
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"stat\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut stat,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_stat
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *mut stat,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"fstat\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *mut stat,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_fstat
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *mut stat,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"ftruncate\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(::core::ffi::c_int, off_t) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_ftruncate
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            off_t,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            xOrig: None,
            default_errno: EIO,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"fcntl\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ...
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_fcntl
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            ...
                        ) -> ::core::ffi::c_int,
                ),
            ),
            xOrig: None,
            default_errno: EACCES,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"read\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_void,
                        size_t,
                    ) -> ssize_t,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_read
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_void,
                            size_t,
                        ) -> ssize_t,
                ),
            ),
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"pread\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_void,
                        size_t,
                        off_t,
                    ) -> ssize_t,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_pread
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_void,
                            size_t,
                            off_t,
                        ) -> ssize_t,
                ),
            ),
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"pread64\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_void,
                        size_t,
                        sqlite3_uint64,
                    ) -> ssize_t,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_pread64
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_void,
                            size_t,
                            sqlite3_uint64,
                        ) -> ssize_t,
                ),
            ),
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"write\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        size_t,
                    ) -> ssize_t,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_write
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                            size_t,
                        ) -> ssize_t,
                ),
            ),
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"pwrite\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        size_t,
                        off_t,
                    ) -> ssize_t,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_pwrite
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                            size_t,
                            off_t,
                        ) -> ssize_t,
                ),
            ),
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"pwrite64\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        size_t,
                        sqlite3_uint64,
                    ) -> ssize_t,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_pwrite64
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                            size_t,
                            sqlite3_uint64,
                        ) -> ssize_t,
                ),
            ),
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"fchmod\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        mode_t,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_fchmod
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            mode_t,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"fallocate\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        off_t,
                        off_t,
                    ) -> ::core::ffi::c_int,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_fallocate
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            off_t,
                            off_t,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"mmap\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        off_t,
                    ) -> *mut ::core::ffi::c_void,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_mmap
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            size_t,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            off_t,
                        ) -> *mut ::core::ffi::c_void,
                ),
            ),
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: b"mremap\0".as_ptr() as *const ::core::ffi::c_char,
            xTest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                        size_t,
                        ::core::ffi::c_int,
                        ...
                    ) -> *mut ::core::ffi::c_void,
                >,
                sqlite3_syscall_ptr,
            >(
                Some(
                    ts_mremap
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            size_t,
                            size_t,
                            ::core::ffi::c_int,
                            ...
                        ) -> *mut ::core::ffi::c_void,
                ),
            ),
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
        TestSyscallArray {
            zName: ::core::ptr::null::<::core::ffi::c_char>(),
            xTest: None,
            xOrig: None,
            default_errno: 0 as ::core::ffi::c_int,
            custom_errno: 0 as ::core::ffi::c_int,
        },
    ]
};
unsafe extern "C" fn tsIsFail() -> ::core::ffi::c_int {
    unsafe {
        gSyscall.nCount -= 1;
        if gSyscall.nCount == 0 as ::core::ffi::c_int
            || gSyscall.nFail != 0 && gSyscall.bPersist != 0
        {
            gSyscall.nFail += 1;
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn tsErrno(
    mut zFunc: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut nFunc: size_t = strlen(zFunc);
        i = 0 as ::core::ffi::c_int;
        while !aSyscall[i as usize].zName.is_null() {
            if !(strlen(aSyscall[i as usize].zName) != nFunc) {
                if !(memcmp(
                    aSyscall[i as usize].zName as *const ::core::ffi::c_void,
                    zFunc as *const ::core::ffi::c_void,
                    nFunc,
                ) != 0)
                {
                    return aSyscall[i as usize].custom_errno;
                }
            }
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn tsIsFailErrno(
    mut zFunc: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if tsIsFail() != 0 {
            *__errno_location() = tsErrno(zFunc);
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn ts_open(
    mut zFile: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if tsIsFailErrno(b"open\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            return -1 as ::core::ffi::c_int;
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[0 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(zFile, flags, mode);
    }
}
unsafe extern "C" fn ts_close(mut fd: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        if tsIsFail() != 0 {
            ::core::mem::transmute::<
                sqlite3_syscall_ptr,
                Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
            >(aSyscall[1 as ::core::ffi::c_int as usize].xOrig)
                .expect("non-null function pointer")(fd);
            return -1 as ::core::ffi::c_int;
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
        >(aSyscall[1 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(fd);
    }
}
unsafe extern "C" fn ts_access(
    mut zPath: *const ::core::ffi::c_char,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if tsIsFail() != 0 {
            return -1 as ::core::ffi::c_int;
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[2 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(zPath, mode);
    }
}
unsafe extern "C" fn ts_getcwd(
    mut zPath: *mut ::core::ffi::c_char,
    mut nPath: size_t,
) -> *mut ::core::ffi::c_char {
    unsafe {
        if tsIsFail() != 0 {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    size_t,
                ) -> *mut ::core::ffi::c_char,
            >,
        >(aSyscall[3 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(zPath, nPath);
    }
}
unsafe extern "C" fn ts_stat(
    mut zPath: *const ::core::ffi::c_char,
    mut p: *mut stat,
) -> ::core::ffi::c_int {
    unsafe {
        if tsIsFail() != 0 {
            return -1 as ::core::ffi::c_int;
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut stat,
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[4 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(zPath, p);
    }
}
unsafe extern "C" fn ts_fstat(
    mut fd: ::core::ffi::c_int,
    mut p: *mut stat,
) -> ::core::ffi::c_int {
    unsafe {
        if tsIsFailErrno(b"fstat\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            return -1 as ::core::ffi::c_int;
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(::core::ffi::c_int, *mut stat) -> ::core::ffi::c_int,
            >,
        >(aSyscall[5 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(fd, p);
    }
}
unsafe extern "C" fn ts_ftruncate(
    mut fd: ::core::ffi::c_int,
    mut n: off_t,
) -> ::core::ffi::c_int {
    unsafe {
        if tsIsFailErrno(b"ftruncate\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            return -1 as ::core::ffi::c_int;
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<unsafe extern "C" fn(::core::ffi::c_int, off_t) -> ::core::ffi::c_int>,
        >(aSyscall[6 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(fd, n);
    }
}
unsafe extern "C" fn ts_fcntl(
    mut fd: ::core::ffi::c_int,
    mut cmd: ::core::ffi::c_int,
    mut c2rust_args: ...
) -> ::core::ffi::c_int {
    unsafe {
        let mut pArg: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        if tsIsFailErrno(b"fcntl\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            return -1 as ::core::ffi::c_int;
        }
        let mut ap = c2rust_args.clone();
        pArg = ap.arg::<*mut ::core::ffi::c_void>();
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ...
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[7 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(fd, cmd, pArg);
    }
}
unsafe extern "C" fn ts_read(
    mut fd: ::core::ffi::c_int,
    mut aBuf: *mut ::core::ffi::c_void,
    mut nBuf: size_t,
) -> ssize_t {
    unsafe {
        if tsIsFailErrno(b"read\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            return -1 as ::core::ffi::c_int as ssize_t;
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> ssize_t,
            >,
        >(aSyscall[8 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(fd, aBuf, nBuf);
    }
}
unsafe extern "C" fn ts_pread(
    mut fd: ::core::ffi::c_int,
    mut aBuf: *mut ::core::ffi::c_void,
    mut nBuf: size_t,
    mut off: off_t,
) -> ssize_t {
    unsafe {
        if tsIsFailErrno(b"pread\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            return -1 as ::core::ffi::c_int as ssize_t;
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                    size_t,
                    off_t,
                ) -> ssize_t,
            >,
        >(aSyscall[9 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(fd, aBuf, nBuf, off);
    }
}
unsafe extern "C" fn ts_pread64(
    mut fd: ::core::ffi::c_int,
    mut aBuf: *mut ::core::ffi::c_void,
    mut nBuf: size_t,
    mut off: sqlite3_uint64,
) -> ssize_t {
    unsafe {
        if tsIsFailErrno(b"pread64\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            return -1 as ::core::ffi::c_int as ssize_t;
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                    size_t,
                    sqlite3_uint64,
                ) -> ssize_t,
            >,
        >(aSyscall[10 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(fd, aBuf, nBuf, off);
    }
}
unsafe extern "C" fn ts_write(
    mut fd: ::core::ffi::c_int,
    mut aBuf: *const ::core::ffi::c_void,
    mut nBuf: size_t,
) -> ssize_t {
    unsafe {
        if tsIsFailErrno(b"write\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            if tsErrno(b"write\0".as_ptr() as *const ::core::ffi::c_char) == EINTR {
                ::core::mem::transmute::<
                    sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_void,
                            size_t,
                        ) -> ssize_t,
                    >,
                >(aSyscall[11 as ::core::ffi::c_int as usize].xOrig)
                    .expect(
                        "non-null function pointer",
                    )(fd, aBuf, nBuf.wrapping_div(2 as size_t));
            }
            return -1 as ::core::ffi::c_int as ssize_t;
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> ssize_t,
            >,
        >(aSyscall[11 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(fd, aBuf, nBuf);
    }
}
unsafe extern "C" fn ts_pwrite(
    mut fd: ::core::ffi::c_int,
    mut aBuf: *const ::core::ffi::c_void,
    mut nBuf: size_t,
    mut off: off_t,
) -> ssize_t {
    unsafe {
        if tsIsFailErrno(b"pwrite\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            return -1 as ::core::ffi::c_int as ssize_t;
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                    size_t,
                    off_t,
                ) -> ssize_t,
            >,
        >(aSyscall[12 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(fd, aBuf, nBuf, off);
    }
}
unsafe extern "C" fn ts_pwrite64(
    mut fd: ::core::ffi::c_int,
    mut aBuf: *const ::core::ffi::c_void,
    mut nBuf: size_t,
    mut off: sqlite3_uint64,
) -> ssize_t {
    unsafe {
        if tsIsFailErrno(b"pwrite64\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            return -1 as ::core::ffi::c_int as ssize_t;
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                    size_t,
                    sqlite3_uint64,
                ) -> ssize_t,
            >,
        >(aSyscall[13 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(fd, aBuf, nBuf, off);
    }
}
unsafe extern "C" fn ts_fchmod(
    mut fd: ::core::ffi::c_int,
    mut mode: mode_t,
) -> ::core::ffi::c_int {
    unsafe {
        if tsIsFail() != 0 {
            return -1 as ::core::ffi::c_int;
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(::core::ffi::c_int, mode_t) -> ::core::ffi::c_int,
            >,
        >(aSyscall[14 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(fd, mode);
    }
}
unsafe extern "C" fn ts_fallocate(
    mut fd: ::core::ffi::c_int,
    mut off: off_t,
    mut len: off_t,
) -> ::core::ffi::c_int {
    unsafe {
        if tsIsFail() != 0 {
            return tsErrno(b"fallocate\0".as_ptr() as *const ::core::ffi::c_char);
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    off_t,
                    off_t,
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[15 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(fd, off, len);
    }
}
unsafe extern "C" fn ts_mmap(
    mut pAddr: *mut ::core::ffi::c_void,
    mut nByte: size_t,
    mut prot: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut fd: ::core::ffi::c_int,
    mut iOff: off_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        if tsIsFailErrno(b"mmap\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            return MAP_FAILED;
        }
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    off_t,
                ) -> *mut ::core::ffi::c_void,
            >,
        >(aSyscall[16 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(pAddr, nByte, prot, flags, fd, iOff);
    }
}
unsafe extern "C" fn ts_mremap(
    mut a: *mut ::core::ffi::c_void,
    mut b: size_t,
    mut c: size_t,
    mut d: ::core::ffi::c_int,
    mut c2rust_args: ...
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pArg: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        if tsIsFailErrno(b"mremap\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            return MAP_FAILED;
        }
        let mut ap = c2rust_args.clone();
        pArg = ap.arg::<*mut ::core::ffi::c_void>();
        return ::core::mem::transmute::<
            sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                    size_t,
                    ::core::ffi::c_int,
                    ...
                ) -> *mut ::core::ffi::c_void,
            >,
        >(aSyscall[17 as ::core::ffi::c_int as usize].xOrig)
            .expect("non-null function pointer")(a, b, c, d, pArg);
    }
}
unsafe extern "C" fn test_syscall_install(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        let mut nElem: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut apElem: *mut *mut Tcl_Obj = ::core::ptr::null_mut::<*mut Tcl_Obj>();
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                b"SYSCALL-LIST\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_ListObjGetElements(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut nElem,
            &raw mut apElem,
        ) != 0
        {
            return TCL_ERROR;
        }
        pVfs = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
        i = 0 as ::core::ffi::c_int;
        while i < nElem {
            let mut iCall: ::core::ffi::c_int = 0;
            let mut rc: ::core::ffi::c_int = Tcl_GetIndexFromObjStruct(
                interp,
                *apElem.offset(i as isize),
                &raw mut aSyscall as *mut TestSyscallArray as *const ::core::ffi::c_void,
                ::core::mem::size_of::<TestSyscallArray>() as ::core::ffi::c_int,
                b"system-call\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                &raw mut iCall,
            );
            if rc != 0 {
                return rc;
            }
            if aSyscall[iCall as usize].xOrig.is_none() {
                aSyscall[iCall as usize].xOrig = (*pVfs)
                    .xGetSystemCall
                    .expect(
                        "non-null function pointer",
                    )(pVfs, aSyscall[iCall as usize].zName);
                (*pVfs)
                    .xSetSystemCall
                    .expect(
                        "non-null function pointer",
                    )(
                    pVfs,
                    aSyscall[iCall as usize].zName,
                    aSyscall[iCall as usize].xTest,
                );
            }
            aSyscall[iCall as usize].custom_errno = aSyscall[iCall as usize]
                .default_errno;
            i += 1;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_syscall_uninstall(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        let mut i: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        pVfs = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
        i = 0 as ::core::ffi::c_int;
        while !aSyscall[i as usize].zName.is_null() {
            if aSyscall[i as usize].xOrig.is_some() {
                (*pVfs)
                    .xSetSystemCall
                    .expect(
                        "non-null function pointer",
                    )(pVfs, aSyscall[i as usize].zName, None);
                aSyscall[i as usize].xOrig = None;
            }
            i += 1;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_syscall_reset(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        pVfs = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
        if objc == 2 as ::core::ffi::c_int {
            rc = (*pVfs)
                .xSetSystemCall
                .expect(
                    "non-null function pointer",
                )(pVfs, ::core::ptr::null::<::core::ffi::c_char>(), None);
            i = 0 as ::core::ffi::c_int;
            while !aSyscall[i as usize].zName.is_null() {
                aSyscall[i as usize].xOrig = None;
                i += 1;
            }
        } else {
            let mut nFunc: ::core::ffi::c_int = 0;
            let mut zFunc: *mut ::core::ffi::c_char = Tcl_GetStringFromObj(
                *objv.offset(2 as ::core::ffi::c_int as isize),
                &raw mut nFunc,
            );
            rc = (*pVfs)
                .xSetSystemCall
                .expect(
                    "non-null function pointer",
                )(
                pVfs,
                Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                None,
            );
            i = 0 as ::core::ffi::c_int;
            while rc == SQLITE_OK && !aSyscall[i as usize].zName.is_null() {
                if !(strlen(aSyscall[i as usize].zName) != nFunc as size_t) {
                    if !(memcmp(
                        aSyscall[i as usize].zName as *const ::core::ffi::c_void,
                        zFunc as *const ::core::ffi::c_void,
                        nFunc as size_t,
                    ) != 0)
                    {
                        aSyscall[i as usize].xOrig = None;
                    }
                }
                i += 1;
            }
        }
        if rc != SQLITE_OK {
            Tcl_SetObjResult(
                interp,
                Tcl_NewStringObj(sqlite3ErrName(rc), -1 as ::core::ffi::c_int),
            );
            return TCL_ERROR;
        }
        Tcl_ResetResult(interp);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_syscall_exists(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        let mut x: sqlite3_syscall_ptr = None;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        pVfs = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
        x = (*pVfs)
            .xGetSystemCall
            .expect(
                "non-null function pointer",
            )(pVfs, Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)));
        Tcl_SetObjResult(
            interp,
            Tcl_NewIntObj(
                (x.is_some() as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int,
            ),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn test_syscall_fault(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nCount: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut bPersist: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if objc != 2 as ::core::ffi::c_int && objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                b"?COUNT PERSIST?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if objc == 4 as ::core::ffi::c_int {
            if Tcl_GetIntFromObj(
                interp,
                *objv.offset(2 as ::core::ffi::c_int as isize),
                &raw mut nCount,
            ) != 0
                || Tcl_GetBooleanFromObj(
                    interp,
                    *objv.offset(3 as ::core::ffi::c_int as isize),
                    &raw mut bPersist,
                ) != 0
            {
                return TCL_ERROR;
            }
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(gSyscall.nFail));
        gSyscall.nCount = nCount;
        gSyscall.bPersist = bPersist;
        gSyscall.nFail = 0 as ::core::ffi::c_int;
        return TCL_OK;
    }
}
unsafe extern "C" fn test_syscall_errno(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iCall: ::core::ffi::c_int = 0;
        let mut iErrno: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut aErrno: [Errno; 12] = [
            Errno {
                z: b"EACCES\0".as_ptr() as *const ::core::ffi::c_char,
                i: EACCES,
            },
            Errno {
                z: b"EINTR\0".as_ptr() as *const ::core::ffi::c_char,
                i: EINTR,
            },
            Errno {
                z: b"EIO\0".as_ptr() as *const ::core::ffi::c_char,
                i: EIO,
            },
            Errno {
                z: b"EOVERFLOW\0".as_ptr() as *const ::core::ffi::c_char,
                i: EOVERFLOW,
            },
            Errno {
                z: b"ENOMEM\0".as_ptr() as *const ::core::ffi::c_char,
                i: ENOMEM,
            },
            Errno {
                z: b"EAGAIN\0".as_ptr() as *const ::core::ffi::c_char,
                i: EAGAIN,
            },
            Errno {
                z: b"ETIMEDOUT\0".as_ptr() as *const ::core::ffi::c_char,
                i: ETIMEDOUT,
            },
            Errno {
                z: b"EBUSY\0".as_ptr() as *const ::core::ffi::c_char,
                i: EBUSY,
            },
            Errno {
                z: b"EPERM\0".as_ptr() as *const ::core::ffi::c_char,
                i: EPERM,
            },
            Errno {
                z: b"EDEADLK\0".as_ptr() as *const ::core::ffi::c_char,
                i: EDEADLK,
            },
            Errno {
                z: b"ENOLCK\0".as_ptr() as *const ::core::ffi::c_char,
                i: ENOLCK,
            },
            Errno {
                z: ::core::ptr::null::<::core::ffi::c_char>(),
                i: 0 as ::core::ffi::c_int,
            },
        ];
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                b"SYSCALL ERRNO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        rc = Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut aSyscall as *mut TestSyscallArray as *const ::core::ffi::c_void,
            ::core::mem::size_of::<TestSyscallArray>() as ::core::ffi::c_int,
            b"system-call\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut iCall,
        );
        if rc != TCL_OK {
            return rc;
        }
        rc = Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut aErrno as *mut Errno as *const ::core::ffi::c_void,
            ::core::mem::size_of::<Errno>() as ::core::ffi::c_int,
            b"errno\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut iErrno,
        );
        if rc != TCL_OK {
            return rc;
        }
        aSyscall[iCall as usize].custom_errno = aErrno[iErrno as usize].i;
        return TCL_OK;
    }
}
unsafe extern "C" fn test_syscall_list(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zSys: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        let mut pList: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        pVfs = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
        pList = Tcl_NewObj();
        (*pList).refCount += 1;
        zSys = (*pVfs)
            .xNextSystemCall
            .expect(
                "non-null function pointer",
            )(pVfs, ::core::ptr::null::<::core::ffi::c_char>());
        while !zSys.is_null() {
            Tcl_ListObjAppendElement(
                interp,
                pList,
                Tcl_NewStringObj(zSys, -1 as ::core::ffi::c_int),
            );
            zSys = (*pVfs)
                .xNextSystemCall
                .expect("non-null function pointer")(pVfs, zSys);
        }
        Tcl_SetObjResult(interp, pList);
        let mut _objPtr: *mut Tcl_Obj = pList;
        let c2rust_fresh0 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh0 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_syscall_defaultvfs(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = ::core::ptr::null_mut::<sqlite3_vfs>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        pVfs = sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>());
        Tcl_SetObjResult(
            interp,
            Tcl_NewStringObj((*pVfs).zName, -1 as ::core::ffi::c_int),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn ts_getpagesize() -> ::core::ffi::c_int {
    unsafe {
        return gSyscall.pgsz;
    }
}
unsafe extern "C" fn test_syscall_pagesize(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pVfs: *mut sqlite3_vfs = sqlite3_vfs_find(
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        let mut pgsz: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                b"PGSZ\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut pgsz,
        ) != 0
        {
            return TCL_ERROR;
        }
        if pgsz < 0 as ::core::ffi::c_int {
            if gSyscall.orig_getpagesize.is_some() {
                (*pVfs)
                    .xSetSystemCall
                    .expect(
                        "non-null function pointer",
                    )(
                    pVfs,
                    b"getpagesize\0".as_ptr() as *const ::core::ffi::c_char,
                    gSyscall.orig_getpagesize,
                );
            }
        } else {
            if pgsz < 512 as ::core::ffi::c_int
                || pgsz & pgsz - 1 as ::core::ffi::c_int != 0
            {
                Tcl_AppendResult(
                    interp,
                    b"pgsz out of range\0".as_ptr() as *const ::core::ffi::c_char,
                    NULL,
                );
                return TCL_ERROR;
            }
            gSyscall.orig_getpagesize = (*pVfs)
                .xGetSystemCall
                .expect(
                    "non-null function pointer",
                )(pVfs, b"getpagesize\0".as_ptr() as *const ::core::ffi::c_char);
            gSyscall.pgsz = pgsz;
            (*pVfs)
                .xSetSystemCall
                .expect(
                    "non-null function pointer",
                )(
                pVfs,
                b"getpagesize\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
                    sqlite3_syscall_ptr,
                >(Some(ts_getpagesize as unsafe extern "C" fn() -> ::core::ffi::c_int)),
            );
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_syscall(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aCmd: [SyscallCmd; 10] = [
            SyscallCmd {
                zName: b"fault\0".as_ptr() as *const ::core::ffi::c_char,
                xCmd: Some(
                    test_syscall_fault
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            SyscallCmd {
                zName: b"install\0".as_ptr() as *const ::core::ffi::c_char,
                xCmd: Some(
                    test_syscall_install
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            SyscallCmd {
                zName: b"uninstall\0".as_ptr() as *const ::core::ffi::c_char,
                xCmd: Some(
                    test_syscall_uninstall
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            SyscallCmd {
                zName: b"reset\0".as_ptr() as *const ::core::ffi::c_char,
                xCmd: Some(
                    test_syscall_reset
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            SyscallCmd {
                zName: b"errno\0".as_ptr() as *const ::core::ffi::c_char,
                xCmd: Some(
                    test_syscall_errno
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            SyscallCmd {
                zName: b"exists\0".as_ptr() as *const ::core::ffi::c_char,
                xCmd: Some(
                    test_syscall_exists
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            SyscallCmd {
                zName: b"list\0".as_ptr() as *const ::core::ffi::c_char,
                xCmd: Some(
                    test_syscall_list
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            SyscallCmd {
                zName: b"defaultvfs\0".as_ptr() as *const ::core::ffi::c_char,
                xCmd: Some(
                    test_syscall_defaultvfs
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            SyscallCmd {
                zName: b"pagesize\0".as_ptr() as *const ::core::ffi::c_char,
                xCmd: Some(
                    test_syscall_pagesize
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            SyscallCmd {
                zName: ::core::ptr::null::<::core::ffi::c_char>(),
                xCmd: None,
            },
        ];
        let mut iCmd: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        let mut pVfs: *mut sqlite3_vfs = sqlite3_vfs_find(
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SUB-COMMAND ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if (*pVfs).iVersion < 3 as ::core::ffi::c_int || (*pVfs).xSetSystemCall.is_none()
        {
            Tcl_AppendResult(
                interp,
                b"VFS does not support xSetSystemCall\0".as_ptr()
                    as *const ::core::ffi::c_char,
                NULL,
            );
            rc = TCL_ERROR;
        } else {
            rc = Tcl_GetIndexFromObjStruct(
                interp,
                *objv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut aCmd as *mut SyscallCmd as *const ::core::ffi::c_void,
                ::core::mem::size_of::<SyscallCmd>() as ::core::ffi::c_int,
                b"sub-command\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                &raw mut iCmd,
            );
        }
        if rc != TCL_OK {
            return rc;
        }
        return aCmd[iCmd as usize]
            .xCmd
            .expect(
                "non-null function pointer",
            )(clientData as ClientData, interp, objc, objv as *const *mut Tcl_Obj);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SqlitetestSyscall_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aCmd: [SyscallCmd_0; 1] = [
            SyscallCmd_0 {
                zName: b"test_syscall\0".as_ptr() as *const ::core::ffi::c_char,
                xCmd: Some(
                    test_syscall
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
            < (::core::mem::size_of::<[SyscallCmd_0; 1]>() as usize)
                .wrapping_div(::core::mem::size_of::<SyscallCmd_0>() as usize)
        {
            Tcl_CreateObjCommand(
                interp,
                aCmd[i as usize].zName,
                aCmd[i as usize].xCmd,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            );
            i += 1;
        }
        return TCL_OK;
    }
}
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const EBUSY: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MAP_FAILED: *mut ::core::ffi::c_void = -1 as ::core::ffi::c_int
    as *mut ::core::ffi::c_void;
