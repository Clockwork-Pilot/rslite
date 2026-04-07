unsafe extern "C" {
    fn Tcl_LinkVar(
        interp: *mut Tcl_Interp,
        varName: *const ::core::ffi::c_char,
        addr: *mut ::core::ffi::c_char,
        type_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_SetVar2(
        interp: *mut Tcl_Interp,
        part1: *const ::core::ffi::c_char,
        part2: *const ::core::ffi::c_char,
        newValue: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Interp {
    pub resultDontUse: *mut ::core::ffi::c_char,
    pub freeProcDontUse: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ()>,
    pub errorLineDontUse: ::core::ffi::c_int,
}
pub const SQLITE_DEFAULT_FILE_FORMAT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_TEMP_STORE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_MAX_WORKER_THREADS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_GLOBAL_ONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_LINK_INT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_LINK_READ_ONLY: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_SYNCHRONOUS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_WAL_SYNCHRONOUS: ::core::ffi::c_int = SQLITE_DEFAULT_SYNCHRONOUS;
unsafe extern "C" fn set_options(mut interp: *mut Tcl_Interp) {
    unsafe {
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"malloc_usable_size\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"rowid32\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"allow_rowid_in_view\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"carray\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"casesensitivelike\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"configslower\0".as_ptr() as *const ::core::ffi::c_char,
            b"1.0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"curdir\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"win32malloc\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"winrt\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"debug\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"default_ckptfullfsync\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"direct_read\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"dirsync\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"lfs\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"pagecache_overflow_stats\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"mmap\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"worker_threads\0".as_ptr() as *const ::core::ffi::c_char,
            b"8\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"memdebug\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"8_3_names\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"cursorhints\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"hiddencolumns\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"deserialize\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"mathlib\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"mem3\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"mem5\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"offset_sql_func\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"ordered_set_aggregates\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"preupdate\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"snapshot\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"mutex\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"mutex_noop\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"altertable\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"analyze\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"api_armor\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"atomicwrite\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"geopoly\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"json1\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"has_codec\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"like_match_blobs\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"attach\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"auth\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"autoinc\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"autoindex\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"autoreset\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"autovacuum\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"default_autovacuum\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"between_opt\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"builtin_test\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"bloblit\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"cast\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"check\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"cte\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"columnmetadata\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"ordered_set_funcs\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"oversize_cell_check\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"compileoption_diags\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"complete\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"compound\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"conflict\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"crashtest\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"datetime\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"decltype\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"deprecated\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"diskio\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"explain\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"floatingpoint\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"foreignkey\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"fts3\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"fts5\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"fts3_unicode\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"fts4_deferred\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"gettable\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"icu\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"icu_collations\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"incrblob\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"integrityck\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"legacyformat\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"like_opt\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"load_ext\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"localtime\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"lookaside\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"memorydb\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"memorymanage\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"mergesort\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"null_trim\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"or_opt\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"rbu\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"pager_pragmas\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"pragma\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"progress\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"reindex\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"rtree\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"rtree_int_only\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"schema_pragmas\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"schema_version\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"session\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"stat4\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"stmtvtab\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"scanstatus\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"lock_proxy_pragmas\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"prefer_proxy_locking\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"shared_cache\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"subquery\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"tclvar\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"threadsafe\0".as_ptr() as *const ::core::ffi::c_char,
            if SQLITE_THREADSAFE != 0 {
                b"1\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"0\0".as_ptr() as *const ::core::ffi::c_char
            },
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"threadsafe1\0".as_ptr() as *const ::core::ffi::c_char,
            if SQLITE_THREADSAFE == 1 as ::core::ffi::c_int {
                b"1\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"0\0".as_ptr() as *const ::core::ffi::c_char
            },
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"threadsafe2\0".as_ptr() as *const ::core::ffi::c_char,
            if SQLITE_THREADSAFE == 2 as ::core::ffi::c_int {
                b"1\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"0\0".as_ptr() as *const ::core::ffi::c_char
            },
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"tempdb\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"trace\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"trigger\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"truncate_opt\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"utf16\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"vacuum\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"view\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"vtab\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"wal\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"wsd\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"update_delete_limit\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"unlock_notify\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"fast_secure_delete\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"secure_delete\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"multiplex_ext_overwrite\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"yytrackmaxstackdepth\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"sqllog\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"uri_00_error\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"normalize\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"windowfunc\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        Tcl_SetVar2(
            interp,
            b"sqlite_options\0".as_ptr() as *const ::core::ffi::c_char,
            b"setlk_timeout\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
            TCL_GLOBAL_ONLY,
        );
        static mut cv_MAX_LENGTH: ::core::ffi::c_int = SQLITE_MAX_LENGTH;
        Tcl_LinkVar(
            interp,
            b"SQLITE_MAX_LENGTH\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_MAX_LENGTH as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_MAX_COLUMN: ::core::ffi::c_int = SQLITE_MAX_COLUMN;
        Tcl_LinkVar(
            interp,
            b"SQLITE_MAX_COLUMN\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_MAX_COLUMN as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_MAX_SQL_LENGTH: ::core::ffi::c_int = SQLITE_MAX_SQL_LENGTH;
        Tcl_LinkVar(
            interp,
            b"SQLITE_MAX_SQL_LENGTH\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_MAX_SQL_LENGTH as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_MAX_EXPR_DEPTH: ::core::ffi::c_int = SQLITE_MAX_EXPR_DEPTH;
        Tcl_LinkVar(
            interp,
            b"SQLITE_MAX_EXPR_DEPTH\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_MAX_EXPR_DEPTH as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_MAX_COMPOUND_SELECT: ::core::ffi::c_int = SQLITE_MAX_COMPOUND_SELECT;
        Tcl_LinkVar(
            interp,
            b"SQLITE_MAX_COMPOUND_SELECT\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_MAX_COMPOUND_SELECT as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_MAX_VDBE_OP: ::core::ffi::c_int = SQLITE_MAX_VDBE_OP;
        Tcl_LinkVar(
            interp,
            b"SQLITE_MAX_VDBE_OP\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_MAX_VDBE_OP as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_MAX_FUNCTION_ARG: ::core::ffi::c_int = SQLITE_MAX_FUNCTION_ARG;
        Tcl_LinkVar(
            interp,
            b"SQLITE_MAX_FUNCTION_ARG\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_MAX_FUNCTION_ARG as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_MAX_VARIABLE_NUMBER: ::core::ffi::c_int = SQLITE_MAX_VARIABLE_NUMBER;
        Tcl_LinkVar(
            interp,
            b"SQLITE_MAX_VARIABLE_NUMBER\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_MAX_VARIABLE_NUMBER as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_MAX_PAGE_SIZE: ::core::ffi::c_int = SQLITE_MAX_PAGE_SIZE;
        Tcl_LinkVar(
            interp,
            b"SQLITE_MAX_PAGE_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_MAX_PAGE_SIZE as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_MAX_PAGE_COUNT: ::core::ffi::c_int = SQLITE_MAX_PAGE_COUNT
            as ::core::ffi::c_int;
        Tcl_LinkVar(
            interp,
            b"SQLITE_MAX_PAGE_COUNT\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_MAX_PAGE_COUNT as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_MAX_LIKE_PATTERN_LENGTH: ::core::ffi::c_int = SQLITE_MAX_LIKE_PATTERN_LENGTH;
        Tcl_LinkVar(
            interp,
            b"SQLITE_MAX_LIKE_PATTERN_LENGTH\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_MAX_LIKE_PATTERN_LENGTH as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_MAX_TRIGGER_DEPTH: ::core::ffi::c_int = SQLITE_MAX_TRIGGER_DEPTH;
        Tcl_LinkVar(
            interp,
            b"SQLITE_MAX_TRIGGER_DEPTH\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_MAX_TRIGGER_DEPTH as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_DEFAULT_CACHE_SIZE: ::core::ffi::c_int = SQLITE_DEFAULT_CACHE_SIZE;
        Tcl_LinkVar(
            interp,
            b"SQLITE_DEFAULT_CACHE_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_DEFAULT_CACHE_SIZE as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_DEFAULT_PAGE_SIZE: ::core::ffi::c_int = SQLITE_DEFAULT_PAGE_SIZE;
        Tcl_LinkVar(
            interp,
            b"SQLITE_DEFAULT_PAGE_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_DEFAULT_PAGE_SIZE as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_DEFAULT_FILE_FORMAT: ::core::ffi::c_int = SQLITE_DEFAULT_FILE_FORMAT;
        Tcl_LinkVar(
            interp,
            b"SQLITE_DEFAULT_FILE_FORMAT\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_DEFAULT_FILE_FORMAT as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_DEFAULT_SYNCHRONOUS: ::core::ffi::c_int = SQLITE_DEFAULT_SYNCHRONOUS;
        Tcl_LinkVar(
            interp,
            b"SQLITE_DEFAULT_SYNCHRONOUS\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_DEFAULT_SYNCHRONOUS as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_DEFAULT_WAL_SYNCHRONOUS: ::core::ffi::c_int = SQLITE_DEFAULT_WAL_SYNCHRONOUS;
        Tcl_LinkVar(
            interp,
            b"SQLITE_DEFAULT_WAL_SYNCHRONOUS\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_DEFAULT_WAL_SYNCHRONOUS as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_MAX_ATTACHED: ::core::ffi::c_int = SQLITE_MAX_ATTACHED;
        Tcl_LinkVar(
            interp,
            b"SQLITE_MAX_ATTACHED\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_MAX_ATTACHED as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_MAX_DEFAULT_PAGE_SIZE: ::core::ffi::c_int = SQLITE_MAX_DEFAULT_PAGE_SIZE;
        Tcl_LinkVar(
            interp,
            b"SQLITE_MAX_DEFAULT_PAGE_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_MAX_DEFAULT_PAGE_SIZE as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_MAX_WORKER_THREADS: ::core::ffi::c_int = SQLITE_MAX_WORKER_THREADS;
        Tcl_LinkVar(
            interp,
            b"SQLITE_MAX_WORKER_THREADS\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_MAX_WORKER_THREADS as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv_TEMP_STORE: ::core::ffi::c_int = SQLITE_TEMP_STORE;
        Tcl_LinkVar(
            interp,
            b"TEMP_STORE\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv_TEMP_STORE as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
        static mut cv___GNUC__: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        Tcl_LinkVar(
            interp,
            b"__GNUC__\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const cv___GNUC__ as *mut ::core::ffi::c_char,
            TCL_LINK_INT | TCL_LINK_READ_ONLY,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqliteconfig_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        set_options(interp);
        return TCL_OK;
    }
}
pub const SQLITE_MAX_LENGTH: ::core::ffi::c_int = 1000000000 as ::core::ffi::c_int;
pub const SQLITE_MAX_COLUMN: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
pub const SQLITE_MAX_SQL_LENGTH: ::core::ffi::c_int = 1000000000 as ::core::ffi::c_int;
pub const SQLITE_MAX_EXPR_DEPTH: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const SQLITE_MAX_COMPOUND_SELECT: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
pub const SQLITE_MAX_VDBE_OP: ::core::ffi::c_int = 250000000 as ::core::ffi::c_int;
pub const SQLITE_MAX_FUNCTION_ARG: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_CACHE_SIZE: ::core::ffi::c_int = -2000 as ::core::ffi::c_int;
pub const SQLITE_MAX_ATTACHED: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SQLITE_MAX_VARIABLE_NUMBER: ::core::ffi::c_int = 32766 as ::core::ffi::c_int;
pub const SQLITE_MAX_DEFAULT_PAGE_SIZE: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const SQLITE_MAX_PAGE_COUNT: ::core::ffi::c_uint = 0xfffffffe as ::core::ffi::c_uint;
pub const SQLITE_MAX_LIKE_PATTERN_LENGTH: ::core::ffi::c_int = 50000
    as ::core::ffi::c_int;
pub const SQLITE_MAX_TRIGGER_DEPTH: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const SQLITE_MAX_PAGE_SIZE: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const SQLITE_THREADSAFE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DEFAULT_PAGE_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
