unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type Tcl_Command_;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetByteArrayFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_uchar;
    fn Tcl_GetIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetStringFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn Tcl_ListObjGetElements(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objcPtr: *mut ::core::ffi::c_int,
        objvPtr: *mut *mut *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewByteArrayObj(
        bytes: *const ::core::ffi::c_uchar,
        length: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
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
    fn Tcl_ObjSetVar2(
        interp: *mut Tcl_Interp,
        part1Ptr: *mut Tcl_Obj,
        part2Ptr: *mut Tcl_Obj,
        newValuePtr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj);
    fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::core::ffi::c_char,
    );
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
    fn Tcl_GetWideIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        widePtr: *mut Tcl_WideInt,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewWideIntObj(wideValue: Tcl_WideInt) -> *mut Tcl_Obj;
}
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type u32_0 = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub struct C2Rust_Unnamed_2 {
    pub zName: *mut ::core::ffi::c_char,
    pub xProc: Option<Tcl_ObjCmdProc>,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3TestBinToHex(
    mut zBuf: *mut ::core::ffi::c_uchar,
    mut N: ::core::ffi::c_int,
) {
    unsafe {
        let zHex: [::core::ffi::c_uchar; 17] = ::core::mem::transmute::<
            [u8; 17],
            [::core::ffi::c_uchar; 17],
        >(*b"0123456789ABCDEF\0");
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_uchar = 0;
        i = N * 2 as ::core::ffi::c_int;
        let c2rust_fresh0 = i;
        i = i - 1;
        *zBuf.offset(c2rust_fresh0 as isize) = 0 as ::core::ffi::c_uchar;
        j = N - 1 as ::core::ffi::c_int;
        while j >= 0 as ::core::ffi::c_int {
            c = *zBuf.offset(j as isize);
            let c2rust_fresh1 = i;
            i = i - 1;
            *zBuf.offset(c2rust_fresh1 as isize) = zHex[(c as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as usize];
            let c2rust_fresh2 = i;
            i = i - 1;
            *zBuf.offset(c2rust_fresh2 as isize) = zHex[(c as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as usize];
            j -= 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3TestHexToBin(
    mut zIn: *const ::core::ffi::c_uchar,
    mut N: ::core::ffi::c_int,
    mut aOut: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    unsafe {
        let aMap: [::core::ffi::c_uchar; 256] = [
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ];
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut hi: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut c: ::core::ffi::c_uchar = 0;
        j = 0 as ::core::ffi::c_int;
        i = j;
        while i < N {
            c = aMap[*zIn.offset(i as isize) as usize];
            if !(c as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
                if hi != 0 {
                    *aOut.offset(j as isize) = ((c as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                    hi = 0 as ::core::ffi::c_int;
                } else {
                    let c2rust_fresh3 = j;
                    j = j + 1;
                    let ref mut c2rust_fresh4 = *aOut.offset(c2rust_fresh3 as isize);
                    *c2rust_fresh4 = (*c2rust_fresh4 as ::core::ffi::c_int
                        | c as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                    hi = 1 as ::core::ffi::c_int;
                }
            }
            i += 1;
        }
        return j;
    }
}
unsafe extern "C" fn hexio_read(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut offset: ::core::ffi::c_int = 0;
        let mut amt: ::core::ffi::c_int = 0;
        let mut got: ::core::ffi::c_int = 0;
        let mut zFile: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut in_0: *mut FILE = ::core::ptr::null_mut::<FILE>();
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"FILENAME OFFSET AMT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut offset,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut amt,
        ) != 0
        {
            return TCL_ERROR;
        }
        zFile = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        zBuf = sqlite3_malloc(amt * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            as *mut ::core::ffi::c_uchar;
        if zBuf.is_null() {
            return TCL_ERROR;
        }
        in_0 = fopen(zFile, b"rb\0".as_ptr() as *const ::core::ffi::c_char);
        if in_0.is_null() {
            in_0 = fopen(zFile, b"r\0".as_ptr() as *const ::core::ffi::c_char);
        }
        if in_0.is_null() {
            Tcl_AppendResult(
                interp,
                b"cannot open input file \0".as_ptr() as *const ::core::ffi::c_char,
                zFile,
                NULL,
            );
            return TCL_ERROR;
        }
        fseek(in_0, offset as ::core::ffi::c_long, SEEK_SET);
        got = fread(zBuf as *mut ::core::ffi::c_void, 1 as size_t, amt as size_t, in_0)
            as ::core::ffi::c_int;
        fclose(in_0);
        if got < 0 as ::core::ffi::c_int {
            got = 0 as ::core::ffi::c_int;
        }
        sqlite3TestBinToHex(zBuf, got);
        Tcl_AppendResult(interp, zBuf, NULL);
        sqlite3_free(zBuf as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
unsafe extern "C" fn hexio_write(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut offset: ::core::ffi::c_int = 0;
        let mut nIn: ::core::ffi::c_int = 0;
        let mut nOut: ::core::ffi::c_int = 0;
        let mut written: ::core::ffi::c_int = 0;
        let mut zFile: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zIn: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut aOut: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut out: *mut FILE = ::core::ptr::null_mut::<FILE>();
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"FILENAME OFFSET HEXDATA\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut offset,
        ) != 0
        {
            return TCL_ERROR;
        }
        zFile = Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize));
        zIn = Tcl_GetStringFromObj(
            *objv.offset(3 as ::core::ffi::c_int as isize),
            &raw mut nIn,
        ) as *const ::core::ffi::c_uchar;
        aOut = sqlite3_malloc64(
            (1 as ::core::ffi::c_int + nIn / 2 as ::core::ffi::c_int) as sqlite3_uint64,
        ) as *mut ::core::ffi::c_uchar;
        if aOut.is_null() {
            return TCL_ERROR;
        }
        nOut = sqlite3TestHexToBin(zIn, nIn, aOut);
        out = fopen(zFile, b"r+b\0".as_ptr() as *const ::core::ffi::c_char);
        if out.is_null() {
            out = fopen(zFile, b"r+\0".as_ptr() as *const ::core::ffi::c_char);
        }
        if out.is_null() {
            Tcl_AppendResult(
                interp,
                b"cannot open output file \0".as_ptr() as *const ::core::ffi::c_char,
                zFile,
                NULL,
            );
            return TCL_ERROR;
        }
        fseek(out, offset as ::core::ffi::c_long, SEEK_SET);
        written = fwrite(
            aOut as *const ::core::ffi::c_void,
            1 as size_t,
            nOut as size_t,
            out,
        ) as ::core::ffi::c_int;
        sqlite3_free(aOut as *mut ::core::ffi::c_void);
        fclose(out);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(written));
        return TCL_OK;
    }
}
unsafe extern "C" fn hexio_get_int(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut val: ::core::ffi::c_int = 0;
        let mut nIn: ::core::ffi::c_int = 0;
        let mut nOut: ::core::ffi::c_int = 0;
        let mut zIn: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut aOut: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut aNum: [::core::ffi::c_uchar; 4] = [0; 4];
        let mut bLittle: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if objc == 3 as ::core::ffi::c_int {
            let mut n: ::core::ffi::c_int = 0;
            let mut z: *mut ::core::ffi::c_char = Tcl_GetStringFromObj(
                *objv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut n,
            );
            if n >= 2 as ::core::ffi::c_int && n <= 13 as ::core::ffi::c_int
                && memcmp(
                    z as *const ::core::ffi::c_void,
                    b"-littleendian\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    n as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                bLittle = 1 as ::core::ffi::c_int;
            }
        }
        if objc - bLittle != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"[-littleendian] HEXDATA\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zIn = Tcl_GetStringFromObj(
            *objv.offset((1 as ::core::ffi::c_int + bLittle) as isize),
            &raw mut nIn,
        ) as *const ::core::ffi::c_uchar;
        aOut = sqlite3_malloc64(
            (1 as ::core::ffi::c_int + nIn / 2 as ::core::ffi::c_int) as sqlite3_uint64,
        ) as *mut ::core::ffi::c_uchar;
        if aOut.is_null() {
            return TCL_ERROR;
        }
        nOut = sqlite3TestHexToBin(zIn, nIn, aOut);
        if nOut >= 4 as ::core::ffi::c_int {
            memcpy(
                &raw mut aNum as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                aOut as *const ::core::ffi::c_void,
                4 as size_t,
            );
        } else {
            memset(
                &raw mut aNum as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<[::core::ffi::c_uchar; 4]>() as size_t,
            );
            memcpy(
                (&raw mut aNum as *mut ::core::ffi::c_uchar)
                    .offset((4 as ::core::ffi::c_int - nOut) as isize)
                    as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                aOut as *const ::core::ffi::c_void,
                nOut as size_t,
            );
        }
        sqlite3_free(aOut as *mut ::core::ffi::c_void);
        if bLittle != 0 {
            val = ((aNum[3 as ::core::ffi::c_int as usize] as u32_0)
                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int
                | (aNum[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                    << 16 as ::core::ffi::c_int
                | (aNum[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                | aNum[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        } else {
            val = ((aNum[0 as ::core::ffi::c_int as usize] as u32_0)
                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int
                | (aNum[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                    << 16 as ::core::ffi::c_int
                | (aNum[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                | aNum[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(val));
        return TCL_OK;
    }
}
unsafe extern "C" fn hexio_render_int16(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut val: ::core::ffi::c_int = 0;
        let mut aNum: [::core::ffi::c_uchar; 10] = [0; 10];
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"INTEGER\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut val,
        ) != 0
        {
            return TCL_ERROR;
        }
        aNum[0 as ::core::ffi::c_int as usize] = (val >> 8 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        aNum[1 as ::core::ffi::c_int as usize] = val as ::core::ffi::c_uchar;
        sqlite3TestBinToHex(
            &raw mut aNum as *mut ::core::ffi::c_uchar,
            2 as ::core::ffi::c_int,
        );
        Tcl_SetObjResult(
            interp,
            Tcl_NewStringObj(
                &raw mut aNum as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
                4 as ::core::ffi::c_int,
            ),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn hexio_render_int32(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut val: ::core::ffi::c_int = 0;
        let mut aNum: [::core::ffi::c_uchar; 10] = [0; 10];
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"INTEGER\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut val,
        ) != 0
        {
            return TCL_ERROR;
        }
        aNum[0 as ::core::ffi::c_int as usize] = (val >> 24 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        aNum[1 as ::core::ffi::c_int as usize] = (val >> 16 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        aNum[2 as ::core::ffi::c_int as usize] = (val >> 8 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        aNum[3 as ::core::ffi::c_int as usize] = val as ::core::ffi::c_uchar;
        sqlite3TestBinToHex(
            &raw mut aNum as *mut ::core::ffi::c_uchar,
            4 as ::core::ffi::c_int,
        );
        Tcl_SetObjResult(
            interp,
            Tcl_NewStringObj(
                &raw mut aNum as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
                8 as ::core::ffi::c_int,
            ),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn utf8_to_utf8(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        Tcl_AppendResult(
            interp,
            b"[utf8_to_utf8] unavailable - SQLITE_DEBUG not defined\0".as_ptr()
                as *const ::core::ffi::c_char,
            NULL,
        );
        return TCL_ERROR;
    }
}
unsafe extern "C" fn getFts3Varint(
    mut p: *const ::core::ffi::c_char,
    mut v: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut q: *const ::core::ffi::c_uchar = p as *const ::core::ffi::c_uchar;
        let mut x: sqlite_uint64 = 0 as sqlite_uint64;
        let mut y: sqlite_uint64 = 1 as sqlite_uint64;
        while *q as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
            == 0x80 as ::core::ffi::c_int
        {
            let c2rust_fresh5 = q;
            q = q.offset(1);
            x = (x as ::core::ffi::c_ulonglong)
                .wrapping_add(
                    (y as ::core::ffi::c_ulonglong)
                        .wrapping_mul(
                            (*c2rust_fresh5 as ::core::ffi::c_int
                                & 0x7f as ::core::ffi::c_int) as ::core::ffi::c_ulonglong,
                        ),
                ) as sqlite_uint64 as sqlite_uint64;
            y <<= 7 as ::core::ffi::c_int;
        }
        let c2rust_fresh6 = q;
        q = q.offset(1);
        x = (x as ::core::ffi::c_ulonglong)
            .wrapping_add(
                (y as ::core::ffi::c_ulonglong)
                    .wrapping_mul(*c2rust_fresh6 as ::core::ffi::c_ulonglong),
            ) as sqlite_uint64 as sqlite_uint64;
        *v = x as sqlite_int64;
        return q.offset_from(p as *mut ::core::ffi::c_uchar) as ::core::ffi::c_long
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn putFts3Varint(
    mut p: *mut ::core::ffi::c_char,
    mut v: sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut q: *mut ::core::ffi::c_uchar = p as *mut ::core::ffi::c_uchar;
        let mut vu: sqlite_uint64 = v as sqlite_uint64;
        loop {
            let c2rust_fresh7 = q;
            q = q.offset(1);
            *c2rust_fresh7 = (vu as ::core::ffi::c_ulonglong
                & 0x7f as ::core::ffi::c_ulonglong | 0x80 as ::core::ffi::c_ulonglong)
                as ::core::ffi::c_uchar;
            vu >>= 7 as ::core::ffi::c_int;
            if !(vu != 0 as ::core::ffi::c_ulonglong) {
                break;
            }
        }
        let ref mut c2rust_fresh8 = *q.offset(-1 as ::core::ffi::c_int as isize);
        *c2rust_fresh8 = (*c2rust_fresh8 as ::core::ffi::c_int
            & 0x7f as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        return q.offset_from(p as *mut ::core::ffi::c_uchar) as ::core::ffi::c_long
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn read_fts3varint(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nBlob: ::core::ffi::c_int = 0;
        let mut zBlob: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut iVal: sqlite3_int64 = 0;
        let mut nVal: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"BLOB VARNAME\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        zBlob = Tcl_GetByteArrayFromObj(
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut nBlob,
        );
        nVal = getFts3Varint(
            zBlob as *mut ::core::ffi::c_char,
            &raw mut iVal as *mut sqlite_int64,
        );
        Tcl_ObjSetVar2(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            ::core::ptr::null_mut::<Tcl_Obj>(),
            Tcl_NewWideIntObj(iVal as Tcl_WideInt),
            0 as ::core::ffi::c_int,
        );
        Tcl_SetObjResult(interp, Tcl_NewIntObj(nVal));
        return TCL_OK;
    }
}
unsafe extern "C" fn make_fts3record(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aArg: *mut *mut Tcl_Obj = ::core::ptr::null_mut::<*mut Tcl_Obj>();
        let mut nArg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut aOut: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut nOut: sqlite3_int64 = 0 as sqlite3_int64;
        let mut nAlloc: sqlite3_int64 = 0 as sqlite3_int64;
        let mut i: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"LIST\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_ListObjGetElements(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut nArg,
            &raw mut aArg,
        ) != 0
        {
            return TCL_ERROR;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nArg {
            let mut iVal: Tcl_WideInt = 0;
            if TCL_OK
                == Tcl_GetWideIntFromObj(
                    ::core::ptr::null_mut::<Tcl_Interp>(),
                    *aArg.offset(i as isize),
                    &raw mut iVal,
                )
            {
                if nOut as ::core::ffi::c_longlong + 10 as ::core::ffi::c_longlong
                    > nAlloc
                {
                    let mut nNew: ::core::ffi::c_int = (if nAlloc != 0 {
                        nAlloc as ::core::ffi::c_longlong * 2 as ::core::ffi::c_longlong
                    } else {
                        128 as ::core::ffi::c_longlong
                    }) as ::core::ffi::c_int;
                    let mut aNew: *mut ::core::ffi::c_uchar = sqlite3_realloc(
                        aOut as *mut ::core::ffi::c_void,
                        nNew,
                    ) as *mut ::core::ffi::c_uchar;
                    if aNew.is_null() {
                        sqlite3_free(aOut as *mut ::core::ffi::c_void);
                        return TCL_ERROR;
                    }
                    aOut = aNew;
                    nAlloc = nNew as sqlite3_int64;
                }
                nOut
                    += putFts3Varint(
                        aOut.offset(nOut as isize) as *mut ::core::ffi::c_uchar
                            as *mut ::core::ffi::c_char,
                        iVal as sqlite_int64,
                    ) as ::core::ffi::c_longlong;
            } else {
                let mut nVal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut zVal: *mut ::core::ffi::c_char = Tcl_GetStringFromObj(
                    *aArg.offset(i as isize),
                    &raw mut nVal,
                );
                while nOut as ::core::ffi::c_longlong + nVal as ::core::ffi::c_longlong
                    > nAlloc
                {
                    let mut nNew_0: sqlite3_int64 = if nAlloc != 0 {
                        nAlloc * 2 as sqlite3_int64
                    } else {
                        128 as sqlite3_int64
                    };
                    let mut aNew_0: *mut ::core::ffi::c_uchar = sqlite3_realloc64(
                        aOut as *mut ::core::ffi::c_void,
                        nNew_0 as sqlite3_uint64,
                    ) as *mut ::core::ffi::c_uchar;
                    if aNew_0.is_null() {
                        sqlite3_free(aOut as *mut ::core::ffi::c_void);
                        return TCL_ERROR;
                    }
                    aOut = aNew_0;
                    nAlloc = nNew_0;
                }
                memcpy(
                    aOut.offset(nOut as isize) as *mut ::core::ffi::c_uchar
                        as *mut ::core::ffi::c_void,
                    zVal as *const ::core::ffi::c_void,
                    nVal as size_t,
                );
                nOut += nVal as ::core::ffi::c_longlong;
            }
            i += 1;
        }
        Tcl_SetObjResult(interp, Tcl_NewByteArrayObj(aOut, nOut as ::core::ffi::c_int));
        sqlite3_free(aOut as *mut ::core::ffi::c_void);
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetest_hexio_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aObjCmd: [C2Rust_Unnamed_2; 8] = unsafe {
            [
                C2Rust_Unnamed_2 {
                    zName: b"hexio_read\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        hexio_read
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_2 {
                    zName: b"hexio_write\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        hexio_write
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_2 {
                    zName: b"hexio_get_int\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        hexio_get_int
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_2 {
                    zName: b"hexio_render_int16\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        hexio_render_int16
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_2 {
                    zName: b"hexio_render_int32\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        hexio_render_int32
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_2 {
                    zName: b"utf8_to_utf8\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        utf8_to_utf8
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_2 {
                    zName: b"read_fts3varint\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        read_fts3varint
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                C2Rust_Unnamed_2 {
                    zName: b"make_fts3record\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        make_fts3record
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
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
            < (::core::mem::size_of::<[C2Rust_Unnamed_2; 8]>() as usize)
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
