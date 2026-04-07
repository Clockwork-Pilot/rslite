use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_api_routines;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type Tcl_Command_;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_create_function(
        db: *mut sqlite3,
        zFunctionName: *const ::core::ffi::c_char,
        nArg: ::core::ffi::c_int,
        eTextRep: ::core::ffi::c_int,
        pApp: *mut ::core::ffi::c_void,
        xFunc: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        xStep: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        xFinal: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_aggregate_context(
        _: *mut sqlite3_context,
        nBytes: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_overload_function(
        _: *mut sqlite3,
        zFuncName: *const ::core::ffi::c_char,
        nArg: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_CreateCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_CmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
}
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Interp {
    pub resultDontUse: *mut ::core::ffi::c_char,
    pub freeProcDontUse: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ()>,
    pub errorLineDontUse: ::core::ffi::c_int,
}
pub type Tcl_Command = *mut Tcl_Command_;
pub type Tcl_CmdDeleteProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_CmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5Context {
    pub isInit: ::core::ffi::c_int,
    pub buf: [::core::ffi::c_uint; 4],
    pub bits: [::core::ffi::c_uint; 2],
    pub in_0: [::core::ffi::c_uchar; 64],
}
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn byteReverse(
    mut buf: *mut ::core::ffi::c_uchar,
    mut longs: ::core::ffi::c_uint,
) {
    unsafe {
        let mut t: ::core::ffi::c_uint = 0;
        loop {
            t = ((*buf.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_int
                | *buf.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                << 16 as ::core::ffi::c_int
                | ((*buf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int
                    | *buf.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uint);
            *(buf as *mut ::core::ffi::c_uint) = t;
            buf = buf.offset(4 as ::core::ffi::c_int as isize);
            longs = longs.wrapping_sub(1);
            if !(longs != 0) {
                break;
            }
        };
    }
}
unsafe extern "C" fn MD5Transform(
    mut buf: *mut ::core::ffi::c_uint,
    mut in_0: *const ::core::ffi::c_uint,
) {
    unsafe {
        let mut a: ::core::ffi::c_uint = 0;
        let mut b: ::core::ffi::c_uint = 0;
        let mut c: ::core::ffi::c_uint = 0;
        let mut d: ::core::ffi::c_uint = 0;
        a = *buf.offset(0 as ::core::ffi::c_int as isize);
        b = *buf.offset(1 as ::core::ffi::c_int as isize);
        c = *buf.offset(2 as ::core::ffi::c_int as isize);
        d = *buf.offset(3 as ::core::ffi::c_int as isize);
        a = a
            .wrapping_add(
                (d ^ b & (c ^ d))
                    .wrapping_add(*in_0.offset(0 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xd76aa478 as ::core::ffi::c_uint),
            );
        a = a << 7 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (c ^ a & (b ^ c))
                    .wrapping_add(*in_0.offset(1 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xe8c7b756 as ::core::ffi::c_uint),
            );
        d = d << 12 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (b ^ d & (a ^ b))
                    .wrapping_add(*in_0.offset(2 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x242070db as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        c = c << 17 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (a ^ c & (d ^ a))
                    .wrapping_add(*in_0.offset(3 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xc1bdceee as ::core::ffi::c_uint),
            );
        b = b << 22 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a
            .wrapping_add(
                (d ^ b & (c ^ d))
                    .wrapping_add(*in_0.offset(4 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xf57c0faf as ::core::ffi::c_uint),
            );
        a = a << 7 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (c ^ a & (b ^ c))
                    .wrapping_add(*in_0.offset(5 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x4787c62a as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        d = d << 12 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (b ^ d & (a ^ b))
                    .wrapping_add(*in_0.offset(6 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xa8304613 as ::core::ffi::c_uint),
            );
        c = c << 17 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (a ^ c & (d ^ a))
                    .wrapping_add(*in_0.offset(7 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xfd469501 as ::core::ffi::c_uint),
            );
        b = b << 22 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a
            .wrapping_add(
                (d ^ b & (c ^ d))
                    .wrapping_add(*in_0.offset(8 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x698098d8 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        a = a << 7 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (c ^ a & (b ^ c))
                    .wrapping_add(*in_0.offset(9 as ::core::ffi::c_int as isize))
                    .wrapping_add(0x8b44f7af as ::core::ffi::c_uint),
            );
        d = d << 12 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (b ^ d & (a ^ b))
                    .wrapping_add(*in_0.offset(10 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xffff5bb1 as ::core::ffi::c_uint),
            );
        c = c << 17 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (a ^ c & (d ^ a))
                    .wrapping_add(*in_0.offset(11 as ::core::ffi::c_int as isize))
                    .wrapping_add(0x895cd7be as ::core::ffi::c_uint),
            );
        b = b << 22 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a
            .wrapping_add(
                (d ^ b & (c ^ d))
                    .wrapping_add(*in_0.offset(12 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x6b901122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        a = a << 7 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (c ^ a & (b ^ c))
                    .wrapping_add(*in_0.offset(13 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xfd987193 as ::core::ffi::c_uint),
            );
        d = d << 12 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (b ^ d & (a ^ b))
                    .wrapping_add(*in_0.offset(14 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xa679438e as ::core::ffi::c_uint),
            );
        c = c << 17 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (a ^ c & (d ^ a))
                    .wrapping_add(*in_0.offset(15 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x49b40821 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        b = b << 22 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a
            .wrapping_add(
                (c ^ d & (b ^ c))
                    .wrapping_add(*in_0.offset(1 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xf61e2562 as ::core::ffi::c_uint),
            );
        a = a << 5 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (b ^ c & (a ^ b))
                    .wrapping_add(*in_0.offset(6 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xc040b340 as ::core::ffi::c_uint),
            );
        d = d << 9 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (a ^ b & (d ^ a))
                    .wrapping_add(*in_0.offset(11 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x265e5a51 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        c = c << 14 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (d ^ a & (c ^ d))
                    .wrapping_add(*in_0.offset(0 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xe9b6c7aa as ::core::ffi::c_uint),
            );
        b = b << 20 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a
            .wrapping_add(
                (c ^ d & (b ^ c))
                    .wrapping_add(*in_0.offset(5 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xd62f105d as ::core::ffi::c_uint),
            );
        a = a << 5 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (b ^ c & (a ^ b))
                    .wrapping_add(*in_0.offset(10 as ::core::ffi::c_int as isize))
                    .wrapping_add(0x2441453 as ::core::ffi::c_int as ::core::ffi::c_uint),
            );
        d = d << 9 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (a ^ b & (d ^ a))
                    .wrapping_add(*in_0.offset(15 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xd8a1e681 as ::core::ffi::c_uint),
            );
        c = c << 14 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (d ^ a & (c ^ d))
                    .wrapping_add(*in_0.offset(4 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xe7d3fbc8 as ::core::ffi::c_uint),
            );
        b = b << 20 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a
            .wrapping_add(
                (c ^ d & (b ^ c))
                    .wrapping_add(*in_0.offset(9 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x21e1cde6 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        a = a << 5 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (b ^ c & (a ^ b))
                    .wrapping_add(*in_0.offset(14 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xc33707d6 as ::core::ffi::c_uint),
            );
        d = d << 9 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (a ^ b & (d ^ a))
                    .wrapping_add(*in_0.offset(3 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xf4d50d87 as ::core::ffi::c_uint),
            );
        c = c << 14 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (d ^ a & (c ^ d))
                    .wrapping_add(*in_0.offset(8 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x455a14ed as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        b = b << 20 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a
            .wrapping_add(
                (c ^ d & (b ^ c))
                    .wrapping_add(*in_0.offset(13 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xa9e3e905 as ::core::ffi::c_uint),
            );
        a = a << 5 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (b ^ c & (a ^ b))
                    .wrapping_add(*in_0.offset(2 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xfcefa3f8 as ::core::ffi::c_uint),
            );
        d = d << 9 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (a ^ b & (d ^ a))
                    .wrapping_add(*in_0.offset(7 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x676f02d9 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        c = c << 14 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (d ^ a & (c ^ d))
                    .wrapping_add(*in_0.offset(12 as ::core::ffi::c_int as isize))
                    .wrapping_add(0x8d2a4c8a as ::core::ffi::c_uint),
            );
        b = b << 20 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(*in_0.offset(5 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xfffa3942 as ::core::ffi::c_uint),
            );
        a = a << 4 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(*in_0.offset(8 as ::core::ffi::c_int as isize))
                    .wrapping_add(0x8771f681 as ::core::ffi::c_uint),
            );
        d = d << 11 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(*in_0.offset(11 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x6d9d6122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        c = c << 16 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(*in_0.offset(14 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xfde5380c as ::core::ffi::c_uint),
            );
        b = b << 23 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(*in_0.offset(1 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xa4beea44 as ::core::ffi::c_uint),
            );
        a = a << 4 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(*in_0.offset(4 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x4bdecfa9 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        d = d << 11 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(*in_0.offset(7 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xf6bb4b60 as ::core::ffi::c_uint),
            );
        c = c << 16 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(*in_0.offset(10 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xbebfbc70 as ::core::ffi::c_uint),
            );
        b = b << 23 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(*in_0.offset(13 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x289b7ec6 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        a = a << 4 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(*in_0.offset(0 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xeaa127fa as ::core::ffi::c_uint),
            );
        d = d << 11 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(*in_0.offset(3 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xd4ef3085 as ::core::ffi::c_uint),
            );
        c = c << 16 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(*in_0.offset(6 as ::core::ffi::c_int as isize))
                    .wrapping_add(0x4881d05 as ::core::ffi::c_int as ::core::ffi::c_uint),
            );
        b = b << 23 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(*in_0.offset(9 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xd9d4d039 as ::core::ffi::c_uint),
            );
        a = a << 4 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(*in_0.offset(12 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xe6db99e5 as ::core::ffi::c_uint),
            );
        d = d << 11 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(*in_0.offset(15 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x1fa27cf8 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        c = c << 16 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(*in_0.offset(2 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xc4ac5665 as ::core::ffi::c_uint),
            );
        b = b << 23 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a
            .wrapping_add(
                (c ^ (b | !d))
                    .wrapping_add(*in_0.offset(0 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xf4292244 as ::core::ffi::c_uint),
            );
        a = a << 6 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (b ^ (a | !c))
                    .wrapping_add(*in_0.offset(7 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x432aff97 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        d = d << 10 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (a ^ (d | !b))
                    .wrapping_add(*in_0.offset(14 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xab9423a7 as ::core::ffi::c_uint),
            );
        c = c << 15 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (d ^ (c | !a))
                    .wrapping_add(*in_0.offset(5 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xfc93a039 as ::core::ffi::c_uint),
            );
        b = b << 21 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a
            .wrapping_add(
                (c ^ (b | !d))
                    .wrapping_add(*in_0.offset(12 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x655b59c3 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        a = a << 6 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (b ^ (a | !c))
                    .wrapping_add(*in_0.offset(3 as ::core::ffi::c_int as isize))
                    .wrapping_add(0x8f0ccc92 as ::core::ffi::c_uint),
            );
        d = d << 10 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (a ^ (d | !b))
                    .wrapping_add(*in_0.offset(10 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xffeff47d as ::core::ffi::c_uint),
            );
        c = c << 15 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (d ^ (c | !a))
                    .wrapping_add(*in_0.offset(1 as ::core::ffi::c_int as isize))
                    .wrapping_add(0x85845dd1 as ::core::ffi::c_uint),
            );
        b = b << 21 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a
            .wrapping_add(
                (c ^ (b | !d))
                    .wrapping_add(*in_0.offset(8 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x6fa87e4f as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        a = a << 6 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (b ^ (a | !c))
                    .wrapping_add(*in_0.offset(15 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xfe2ce6e0 as ::core::ffi::c_uint),
            );
        d = d << 10 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (a ^ (d | !b))
                    .wrapping_add(*in_0.offset(6 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xa3014314 as ::core::ffi::c_uint),
            );
        c = c << 15 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (d ^ (c | !a))
                    .wrapping_add(*in_0.offset(13 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x4e0811a1 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        b = b << 21 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        a = a
            .wrapping_add(
                (c ^ (b | !d))
                    .wrapping_add(*in_0.offset(4 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xf7537e82 as ::core::ffi::c_uint),
            );
        a = a << 6 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
        a = a.wrapping_add(b);
        d = d
            .wrapping_add(
                (b ^ (a | !c))
                    .wrapping_add(*in_0.offset(11 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xbd3af235 as ::core::ffi::c_uint),
            );
        d = d << 10 as ::core::ffi::c_int
            | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
        d = d.wrapping_add(a);
        c = c
            .wrapping_add(
                (a ^ (d | !b))
                    .wrapping_add(*in_0.offset(2 as ::core::ffi::c_int as isize))
                    .wrapping_add(
                        0x2ad7d2bb as ::core::ffi::c_int as ::core::ffi::c_uint,
                    ),
            );
        c = c << 15 as ::core::ffi::c_int
            | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
        c = c.wrapping_add(d);
        b = b
            .wrapping_add(
                (d ^ (c | !a))
                    .wrapping_add(*in_0.offset(9 as ::core::ffi::c_int as isize))
                    .wrapping_add(0xeb86d391 as ::core::ffi::c_uint),
            );
        b = b << 21 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
        b = b.wrapping_add(c);
        let ref mut c2rust_fresh0 = *buf.offset(0 as ::core::ffi::c_int as isize);
        *c2rust_fresh0 = (*c2rust_fresh0).wrapping_add(a);
        let ref mut c2rust_fresh1 = *buf.offset(1 as ::core::ffi::c_int as isize);
        *c2rust_fresh1 = (*c2rust_fresh1).wrapping_add(b);
        let ref mut c2rust_fresh2 = *buf.offset(2 as ::core::ffi::c_int as isize);
        *c2rust_fresh2 = (*c2rust_fresh2).wrapping_add(c);
        let ref mut c2rust_fresh3 = *buf.offset(3 as ::core::ffi::c_int as isize);
        *c2rust_fresh3 = (*c2rust_fresh3).wrapping_add(d);
    }
}
unsafe extern "C" fn MD5Init(mut ctx: *mut MD5Context) {
    unsafe {
        (*ctx).isInit = 1 as ::core::ffi::c_int;
        (*ctx).buf[0 as ::core::ffi::c_int as usize] = 0x67452301 as ::core::ffi::c_int
            as ::core::ffi::c_uint;
        (*ctx).buf[1 as ::core::ffi::c_int as usize] = 0xefcdab89 as ::core::ffi::c_uint;
        (*ctx).buf[2 as ::core::ffi::c_int as usize] = 0x98badcfe as ::core::ffi::c_uint;
        (*ctx).buf[3 as ::core::ffi::c_int as usize] = 0x10325476 as ::core::ffi::c_int
            as ::core::ffi::c_uint;
        (*ctx).bits[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uint;
        (*ctx).bits[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uint;
    }
}
unsafe extern "C" fn MD5Update(
    mut ctx: *mut MD5Context,
    mut buf: *const ::core::ffi::c_uchar,
    mut len: ::core::ffi::c_uint,
) {
    unsafe {
        let mut t: ::core::ffi::c_uint = 0;
        t = (*ctx).bits[0 as ::core::ffi::c_int as usize];
        (*ctx).bits[0 as ::core::ffi::c_int as usize] = t
            .wrapping_add(len << 3 as ::core::ffi::c_int);
        if (*ctx).bits[0 as ::core::ffi::c_int as usize] < t {
            (*ctx).bits[1 as ::core::ffi::c_int as usize] = (*ctx)
                .bits[1 as ::core::ffi::c_int as usize]
                .wrapping_add(1);
        }
        (*ctx).bits[1 as ::core::ffi::c_int as usize] = (*ctx)
            .bits[1 as ::core::ffi::c_int as usize]
            .wrapping_add(len >> 29 as ::core::ffi::c_int);
        t = t >> 3 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_uint;
        if t != 0 {
            let mut p: *mut ::core::ffi::c_uchar = (&raw mut (*ctx).in_0
                as *mut ::core::ffi::c_uchar)
                .offset(t as isize);
            t = (64 as ::core::ffi::c_uint).wrapping_sub(t);
            if len < t {
                memcpy(
                    p as *mut ::core::ffi::c_void,
                    buf as *const ::core::ffi::c_void,
                    len as size_t,
                );
                return;
            }
            memcpy(
                p as *mut ::core::ffi::c_void,
                buf as *const ::core::ffi::c_void,
                t as size_t,
            );
            byteReverse(
                &raw mut (*ctx).in_0 as *mut ::core::ffi::c_uchar,
                16 as ::core::ffi::c_uint,
            );
            MD5Transform(
                &raw mut (*ctx).buf as *mut ::core::ffi::c_uint,
                &raw mut (*ctx).in_0 as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_uint as *const ::core::ffi::c_uint,
            );
            buf = buf.offset(t as isize);
            len = len.wrapping_sub(t);
        }
        while len >= 64 as ::core::ffi::c_uint {
            memcpy(
                &raw mut (*ctx).in_0 as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_void,
                buf as *const ::core::ffi::c_void,
                64 as size_t,
            );
            byteReverse(
                &raw mut (*ctx).in_0 as *mut ::core::ffi::c_uchar,
                16 as ::core::ffi::c_uint,
            );
            MD5Transform(
                &raw mut (*ctx).buf as *mut ::core::ffi::c_uint,
                &raw mut (*ctx).in_0 as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_uint as *const ::core::ffi::c_uint,
            );
            buf = buf.offset(64 as ::core::ffi::c_int as isize);
            len = len.wrapping_sub(64 as ::core::ffi::c_uint);
        }
        memcpy(
            &raw mut (*ctx).in_0 as *mut ::core::ffi::c_uchar
                as *mut ::core::ffi::c_void,
            buf as *const ::core::ffi::c_void,
            len as size_t,
        );
    }
}
unsafe extern "C" fn MD5Final(
    mut digest: *mut ::core::ffi::c_uchar,
    mut ctx: *mut MD5Context,
) {
    unsafe {
        let mut count: ::core::ffi::c_uint = 0;
        let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        count = (*ctx).bits[0 as ::core::ffi::c_int as usize] >> 3 as ::core::ffi::c_int
            & 0x3f as ::core::ffi::c_uint;
        p = (&raw mut (*ctx).in_0 as *mut ::core::ffi::c_uchar).offset(count as isize);
        let c2rust_fresh4 = p;
        p = p.offset(1);
        *c2rust_fresh4 = 0x80 as ::core::ffi::c_uchar;
        count = ((64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_uint)
            .wrapping_sub(count);
        if count < 8 as ::core::ffi::c_uint {
            memset(
                p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                count as size_t,
            );
            byteReverse(
                &raw mut (*ctx).in_0 as *mut ::core::ffi::c_uchar,
                16 as ::core::ffi::c_uint,
            );
            MD5Transform(
                &raw mut (*ctx).buf as *mut ::core::ffi::c_uint,
                &raw mut (*ctx).in_0 as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_uint as *const ::core::ffi::c_uint,
            );
            memset(
                &raw mut (*ctx).in_0 as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                56 as size_t,
            );
        } else {
            memset(
                p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                count.wrapping_sub(8 as ::core::ffi::c_uint) as size_t,
            );
        }
        byteReverse(
            &raw mut (*ctx).in_0 as *mut ::core::ffi::c_uchar,
            14 as ::core::ffi::c_uint,
        );
        memcpy(
            (&raw mut (*ctx).in_0 as *mut ::core::ffi::c_uchar)
                .offset((14 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_void,
            &raw mut (*ctx).bits as *mut ::core::ffi::c_uint
                as *const ::core::ffi::c_void,
            8 as size_t,
        );
        MD5Transform(
            &raw mut (*ctx).buf as *mut ::core::ffi::c_uint,
            &raw mut (*ctx).in_0 as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_uint
                as *const ::core::ffi::c_uint,
        );
        byteReverse(
            &raw mut (*ctx).buf as *mut ::core::ffi::c_uint as *mut ::core::ffi::c_uchar,
            4 as ::core::ffi::c_uint,
        );
        memcpy(
            digest as *mut ::core::ffi::c_void,
            &raw mut (*ctx).buf as *mut ::core::ffi::c_uint
                as *const ::core::ffi::c_void,
            16 as size_t,
        );
    }
}
unsafe extern "C" fn MD5DigestToBase16(
    mut digest: *mut ::core::ffi::c_uchar,
    mut zBuf: *mut ::core::ffi::c_char,
) {
    unsafe {
        static mut zEncode: [::core::ffi::c_char; 17] = unsafe {
            ::core::mem::transmute::<
                [u8; 17],
                [::core::ffi::c_char; 17],
            >(*b"0123456789abcdef\0")
        };
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        j = i;
        while i < 16 as ::core::ffi::c_int {
            let mut a: ::core::ffi::c_int = *digest.offset(i as isize)
                as ::core::ffi::c_int;
            let c2rust_fresh5 = j;
            j = j + 1;
            *zBuf.offset(c2rust_fresh5 as isize) = zEncode[(a >> 4 as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as usize];
            let c2rust_fresh6 = j;
            j = j + 1;
            *zBuf.offset(c2rust_fresh6 as isize) = zEncode[(a
                & 0xf as ::core::ffi::c_int) as usize];
            i += 1;
        }
        *zBuf.offset(j as isize) = 0 as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn MD5DigestToBase10x8(
    mut digest: *mut ::core::ffi::c_uchar,
    mut zDigest: *mut ::core::ffi::c_char,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut x: ::core::ffi::c_uint = 0;
        j = 0 as ::core::ffi::c_int;
        i = j;
        while i < 16 as ::core::ffi::c_int {
            x = (*digest.offset(i as isize) as ::core::ffi::c_int
                * 256 as ::core::ffi::c_int
                + *digest.offset((i + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int) as ::core::ffi::c_uint;
            if i > 0 as ::core::ffi::c_int {
                let c2rust_fresh7 = j;
                j = j + 1;
                *zDigest.offset(c2rust_fresh7 as isize) = '-' as i32
                    as ::core::ffi::c_char;
            }
            sqlite3_snprintf(
                50 as ::core::ffi::c_int - j,
                zDigest.offset(j as isize) as *mut ::core::ffi::c_char,
                b"%05u\0".as_ptr() as *const ::core::ffi::c_char,
                x,
            );
            j += 5 as ::core::ffi::c_int;
            i += 2 as ::core::ffi::c_int;
        }
        *zDigest.offset(j as isize) = 0 as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn md5_cmd(
    mut cd: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ctx: MD5Context = MD5Context {
            isInit: 0,
            buf: [0; 4],
            bits: [0; 2],
            in_0: [0; 64],
        };
        let mut digest: [::core::ffi::c_uchar; 16] = [0; 16];
        let mut zBuf: [::core::ffi::c_char; 50] = [0; 50];
        let mut converter: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_uchar,
                *mut ::core::ffi::c_char,
            ) -> (),
        > = None;
        if argc != 2 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" TEXT\"\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        MD5Init(&raw mut ctx);
        MD5Update(
            &raw mut ctx,
            *argv.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar,
            strlen(*argv.offset(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_uint,
        );
        MD5Final(&raw mut digest as *mut ::core::ffi::c_uchar, &raw mut ctx);
        converter = ::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_uchar,
                    *mut ::core::ffi::c_char,
                ) -> (),
            >,
        >(cd);
        converter
            .expect(
                "non-null function pointer",
            )(
            &raw mut digest as *mut ::core::ffi::c_uchar,
            &raw mut zBuf as *mut ::core::ffi::c_char,
        );
        Tcl_AppendResult(
            interp,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn md5file_cmd(
    mut cd: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut in_0: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut ofst: ::core::ffi::c_int = 0;
        let mut amt: ::core::ffi::c_int = 0;
        let mut ctx: MD5Context = MD5Context {
            isInit: 0,
            buf: [0; 4],
            bits: [0; 2],
            in_0: [0; 64],
        };
        let mut converter: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_uchar,
                *mut ::core::ffi::c_char,
            ) -> (),
        > = None;
        let mut digest: [::core::ffi::c_uchar; 16] = [0; 16];
        let mut zBuf: [::core::ffi::c_char; 10240] = [0; 10240];
        if argc != 2 as ::core::ffi::c_int && argc != 4 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"wrong # args: should be \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                b" FILENAME [OFFSET AMT]\"\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        if argc == 4 as ::core::ffi::c_int {
            ofst = atoi(*argv.offset(2 as ::core::ffi::c_int as isize));
            amt = atoi(*argv.offset(3 as ::core::ffi::c_int as isize));
        } else {
            ofst = 0 as ::core::ffi::c_int;
            amt = 2147483647 as ::core::ffi::c_int;
        }
        in_0 = fopen(
            *argv.offset(1 as ::core::ffi::c_int as isize),
            b"rb\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if in_0.is_null() {
            Tcl_AppendResult(
                interp,
                b"unable to open file \"\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(1 as ::core::ffi::c_int as isize),
                b"\" for reading\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        fseek(in_0, ofst as ::core::ffi::c_long, SEEK_SET);
        MD5Init(&raw mut ctx);
        while amt > 0 as ::core::ffi::c_int {
            let mut n: ::core::ffi::c_int = 0;
            n = fread(
                &raw mut zBuf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                1 as size_t,
                if ::core::mem::size_of::<[::core::ffi::c_char; 10240]>() as usize
                    <= amt as usize
                {
                    ::core::mem::size_of::<[::core::ffi::c_char; 10240]>() as size_t
                } else {
                    amt as size_t
                },
                in_0,
            ) as ::core::ffi::c_int;
            if n <= 0 as ::core::ffi::c_int {
                break;
            }
            MD5Update(
                &raw mut ctx,
                &raw mut zBuf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
                n as ::core::ffi::c_uint,
            );
            amt -= n;
        }
        fclose(in_0);
        MD5Final(&raw mut digest as *mut ::core::ffi::c_uchar, &raw mut ctx);
        converter = ::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_uchar,
                    *mut ::core::ffi::c_char,
                ) -> (),
            >,
        >(cd);
        converter
            .expect(
                "non-null function pointer",
            )(
            &raw mut digest as *mut ::core::ffi::c_uchar,
            &raw mut zBuf as *mut ::core::ffi::c_char,
        );
        Tcl_AppendResult(
            interp,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Md5_Init(mut interp: *mut Tcl_Interp) -> ::core::ffi::c_int {
    unsafe {
        Tcl_CreateCommand(
            interp,
            b"md5\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
                >,
                Option<Tcl_CmdProc>,
            >(
                Some(
                    md5_cmd
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        *mut ::core::ffi::c_char,
                    ) -> (),
                >,
                ClientData,
            >(
                Some(
                    MD5DigestToBase16
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_uchar,
                            *mut ::core::ffi::c_char,
                        ) -> (),
                ),
            ),
            None,
        );
        Tcl_CreateCommand(
            interp,
            b"md5-10x8\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
                >,
                Option<Tcl_CmdProc>,
            >(
                Some(
                    md5_cmd
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        *mut ::core::ffi::c_char,
                    ) -> (),
                >,
                ClientData,
            >(
                Some(
                    MD5DigestToBase10x8
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_uchar,
                            *mut ::core::ffi::c_char,
                        ) -> (),
                ),
            ),
            None,
        );
        Tcl_CreateCommand(
            interp,
            b"md5file\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
                >,
                Option<Tcl_CmdProc>,
            >(
                Some(
                    md5file_cmd
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        *mut ::core::ffi::c_char,
                    ) -> (),
                >,
                ClientData,
            >(
                Some(
                    MD5DigestToBase16
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_uchar,
                            *mut ::core::ffi::c_char,
                        ) -> (),
                ),
            ),
            None,
        );
        Tcl_CreateCommand(
            interp,
            b"md5file-10x8\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
                >,
                Option<Tcl_CmdProc>,
            >(
                Some(
                    md5file_cmd
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ),
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_uchar,
                        *mut ::core::ffi::c_char,
                    ) -> (),
                >,
                ClientData,
            >(
                Some(
                    MD5DigestToBase10x8
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_uchar,
                            *mut ::core::ffi::c_char,
                        ) -> (),
                ),
            ),
            None,
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn md5step(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut MD5Context = ::core::ptr::null_mut::<MD5Context>();
        let mut i: ::core::ffi::c_int = 0;
        if argc < 1 as ::core::ffi::c_int {
            return;
        }
        p = sqlite3_aggregate_context(
            context,
            ::core::mem::size_of::<MD5Context>() as ::core::ffi::c_int,
        ) as *mut MD5Context;
        if p.is_null() {
            return;
        }
        if (*p).isInit == 0 {
            MD5Init(p);
        }
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            let mut zData: *const ::core::ffi::c_char = sqlite3_value_text(
                *argv.offset(i as isize),
            ) as *mut ::core::ffi::c_char;
            if !zData.is_null() {
                MD5Update(
                    p,
                    zData as *mut ::core::ffi::c_uchar,
                    strlen(zData) as ::core::ffi::c_int as ::core::ffi::c_uint,
                );
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn md5finalize(mut context: *mut sqlite3_context) {
    unsafe {
        let mut p: *mut MD5Context = ::core::ptr::null_mut::<MD5Context>();
        let mut digest: [::core::ffi::c_uchar; 16] = [0; 16];
        let mut zBuf: [::core::ffi::c_char; 33] = [0; 33];
        p = sqlite3_aggregate_context(
            context,
            ::core::mem::size_of::<MD5Context>() as ::core::ffi::c_int,
        ) as *mut MD5Context;
        MD5Final(&raw mut digest as *mut ::core::ffi::c_uchar, p);
        MD5DigestToBase16(
            &raw mut digest as *mut ::core::ffi::c_uchar,
            &raw mut zBuf as *mut ::core::ffi::c_char,
        );
        sqlite3_result_text(
            context,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Md5_Register(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pThunk: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = sqlite3_create_function(
            db,
            b"md5sum\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_UTF8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
            Some(
                md5step
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            Some(md5finalize as unsafe extern "C" fn(*mut sqlite3_context) -> ()),
        );
        sqlite3_overload_function(
            db,
            b"md5sum\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
        );
        return rc;
    }
}
