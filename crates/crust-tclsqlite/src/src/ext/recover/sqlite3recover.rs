use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_mutex;
    pub type sqlite3_api_routines;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type sqlite3_backup;
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
    fn sqlite3_vmprintf(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_snprintf(
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_randomness(N: ::core::ffi::c_int, P: *mut ::core::ffi::c_void);
    fn sqlite3_open(
        filename: *const ::core::ffi::c_char,
        ppDb: *mut *mut sqlite3,
    ) -> ::core::ffi::c_int;
    fn sqlite3_open_v2(
        filename: *const ::core::ffi::c_char,
        ppDb: *mut *mut sqlite3,
        flags: ::core::ffi::c_int,
        zVfs: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_errcode(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_int(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_int64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_value(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const sqlite3_value,
    ) -> ::core::ffi::c_int;
    fn sqlite3_clear_bindings(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_blob(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_column_int64(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> sqlite3_int64;
    fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    fn sqlite3_column_value(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *mut sqlite3_value;
    fn sqlite3_column_bytes(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
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
    fn sqlite3_value_blob(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_dup(_: *const sqlite3_value) -> *mut sqlite3_value;
    fn sqlite3_value_free(_: *mut sqlite3_value);
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    fn sqlite3_result_blob(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_error_nomem(_: *mut sqlite3_context);
    fn sqlite3_result_error_code(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_value(_: *mut sqlite3_context, _: *mut sqlite3_value);
    fn sqlite3_get_autocommit(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_db_handle(_: *mut sqlite3_stmt) -> *mut sqlite3;
    fn sqlite3_mutex_alloc(_: ::core::ffi::c_int) -> *mut sqlite3_mutex;
    fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    fn sqlite3_file_control(
        _: *mut sqlite3,
        zDbName: *const ::core::ffi::c_char,
        op: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
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
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
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
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3_dbdata_init(
        _: *mut sqlite3,
        _: *mut *mut ::core::ffi::c_char,
        _: *const sqlite3_api_routines,
    ) -> ::core::ffi::c_int;
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
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
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
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_recover {
    pub dbIn: *mut sqlite3,
    pub zDb: *mut ::core::ffi::c_char,
    pub zUri: *mut ::core::ffi::c_char,
    pub pSqlCtx: *mut ::core::ffi::c_void,
    pub xSql: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub zStateDb: *mut ::core::ffi::c_char,
    pub zLostAndFound: *mut ::core::ffi::c_char,
    pub bFreelistCorrupt: ::core::ffi::c_int,
    pub bRecoverRowid: ::core::ffi::c_int,
    pub bSlowIndexes: ::core::ffi::c_int,
    pub pgsz: ::core::ffi::c_int,
    pub detected_pgsz: ::core::ffi::c_int,
    pub nReserve: ::core::ffi::c_int,
    pub pPage1Disk: *mut u8_0,
    pub pPage1Cache: *mut u8_0,
    pub errCode: ::core::ffi::c_int,
    pub zErrMsg: *mut ::core::ffi::c_char,
    pub eState: ::core::ffi::c_int,
    pub bCloseTransaction: ::core::ffi::c_int,
    pub w1: RecoverStateW1,
    pub laf: RecoverStateLAF,
    pub dbOut: *mut sqlite3,
    pub pGetPage: *mut sqlite3_stmt,
    pub pTblList: *mut RecoverTable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RecoverTable {
    pub iRoot: u32_0,
    pub zTab: *mut ::core::ffi::c_char,
    pub nCol: ::core::ffi::c_int,
    pub aCol: *mut RecoverColumn,
    pub bIntkey: ::core::ffi::c_int,
    pub iRowidBind: ::core::ffi::c_int,
    pub pNext: *mut RecoverTable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RecoverColumn {
    pub iField: ::core::ffi::c_int,
    pub iBind: ::core::ffi::c_int,
    pub bIPK: ::core::ffi::c_int,
    pub zCol: *mut ::core::ffi::c_char,
    pub eHidden: ::core::ffi::c_int,
}
pub type u32_0 = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RecoverStateLAF {
    pub pUsed: *mut RecoverBitmap,
    pub nPg: i64_0,
    pub pAllAndParent: *mut sqlite3_stmt,
    pub pMapInsert: *mut sqlite3_stmt,
    pub pMaxField: *mut sqlite3_stmt,
    pub pUsedPages: *mut sqlite3_stmt,
    pub pFindRoot: *mut sqlite3_stmt,
    pub pInsert: *mut sqlite3_stmt,
    pub pAllPage: *mut sqlite3_stmt,
    pub pPageData: *mut sqlite3_stmt,
    pub apVal: *mut *mut sqlite3_value,
    pub nMaxField: ::core::ffi::c_int,
}
pub type i64_0 = sqlite3_int64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RecoverBitmap {
    pub nPg: i64_0,
    pub aElem: [u32_0; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RecoverStateW1 {
    pub pTbls: *mut sqlite3_stmt,
    pub pSel: *mut sqlite3_stmt,
    pub pInsert: *mut sqlite3_stmt,
    pub nInsert: ::core::ffi::c_int,
    pub pTab: *mut RecoverTable,
    pub nMax: ::core::ffi::c_int,
    pub apVal: *mut *mut sqlite3_value,
    pub nVal: ::core::ffi::c_int,
    pub bHaveRowid: ::core::ffi::c_int,
    pub iRowid: i64_0,
    pub iPrevPage: i64_0,
    pub iPrevCell: ::core::ffi::c_int,
}
pub type u8_0 = ::core::ffi::c_uchar;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RecoverGlobal {
    pub pMethods: *const sqlite3_io_methods,
    pub p: *mut sqlite3_recover,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Func {
    pub zName: *const ::core::ffi::c_char,
    pub nArg: ::core::ffi::c_int,
    pub xFunc: Option<
        unsafe extern "C" fn(
            *mut sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> (),
    >,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_MISUSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQLITE_NOTADB: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_OPEN_READWRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_OPEN_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SQLITE_OPEN_URI: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_FILE_POINTER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_RESET_CACHE: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_UTF16LE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_UTF16BE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_MUTEX_STATIC_APP2: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_RECOVER_LOST_AND_FOUND: ::core::ffi::c_int = 1;
pub const SQLITE_RECOVER_FREELIST_CORRUPT: ::core::ffi::c_int = 2;
pub const SQLITE_RECOVER_ROWIDS: ::core::ffi::c_int = 3;
pub const SQLITE_RECOVER_SLOWINDEXES: ::core::ffi::c_int = 4;
pub const RECOVER_EHIDDEN_VIRTUAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const RECOVER_EHIDDEN_STORED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SZ_RECOVERBITMAP_32: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const RECOVER_STATE_INIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RECOVER_STATE_WRITING: ::core::ffi::c_int = 1;
pub const RECOVER_STATE_LOSTANDFOUND1: ::core::ffi::c_int = 2;
pub const RECOVER_STATE_LOSTANDFOUND2: ::core::ffi::c_int = 3;
pub const RECOVER_STATE_LOSTANDFOUND3: ::core::ffi::c_int = 4;
pub const RECOVER_STATE_SCHEMA2: ::core::ffi::c_int = 5;
pub const RECOVER_STATE_DONE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
static mut recover_g: RecoverGlobal = RecoverGlobal {
    pMethods: ::core::ptr::null::<sqlite3_io_methods>(),
    p: ::core::ptr::null::<sqlite3_recover>() as *mut sqlite3_recover,
};
pub const RECOVER_MUTEX_ID: ::core::ffi::c_int = SQLITE_MUTEX_STATIC_APP2;
pub const RECOVER_ROWID_DEFAULT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn recoverEnterMutex() {
    unsafe {
        sqlite3_mutex_enter(sqlite3_mutex_alloc(RECOVER_MUTEX_ID));
    }
}
unsafe extern "C" fn recoverLeaveMutex() {
    unsafe {
        sqlite3_mutex_leave(sqlite3_mutex_alloc(RECOVER_MUTEX_ID));
    }
}
unsafe extern "C" fn recoverStrlen(
    mut zStr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if zStr.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        return (strlen(zStr) & 0x7fffffff as ::core::ffi::c_int as size_t)
            as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn recoverMalloc(
    mut p: *mut sqlite3_recover,
    mut nByte: i64_0,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pRet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        if (*p).errCode == SQLITE_OK {
            pRet = sqlite3_malloc64(nByte as sqlite3_uint64);
            if !pRet.is_null() {
                memset(pRet, 0 as ::core::ffi::c_int, nByte as size_t);
            } else {
                (*p).errCode = SQLITE_NOMEM;
            }
        }
        return pRet;
    }
}
unsafe extern "C" fn recoverError(
    mut p: *mut sqlite3_recover,
    mut errCode: ::core::ffi::c_int,
    mut zFmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> ::core::ffi::c_int {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut ap = c2rust_args.clone();
        if !zFmt.is_null() {
            z = sqlite3_vmprintf(zFmt, ap);
        }
        sqlite3_free((*p).zErrMsg as *mut ::core::ffi::c_void);
        (*p).zErrMsg = z;
        (*p).errCode = errCode;
        return errCode;
    }
}
unsafe extern "C" fn recoverBitmapAlloc(
    mut p: *mut sqlite3_recover,
    mut nPg: i64_0,
) -> *mut RecoverBitmap {
    unsafe {
        let mut nElem: ::core::ffi::c_int = ((nPg as ::core::ffi::c_longlong
            + 1 as ::core::ffi::c_longlong + 31 as ::core::ffi::c_longlong)
            / 32 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
        let mut nByte: ::core::ffi::c_int = (SZ_RECOVERBITMAP_32 as usize)
            .wrapping_add(
                (nElem as usize).wrapping_mul(::core::mem::size_of::<u32_0>() as usize),
            ) as ::core::ffi::c_int;
        let mut pRet: *mut RecoverBitmap = recoverMalloc(p, nByte as i64_0)
            as *mut RecoverBitmap;
        if !pRet.is_null() {
            (*pRet).nPg = nPg;
        }
        return pRet;
    }
}
unsafe extern "C" fn recoverBitmapFree(mut pMap: *mut RecoverBitmap) {
    unsafe {
        sqlite3_free(pMap as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn recoverBitmapSet(mut pMap: *mut RecoverBitmap, mut iPg: i64_0) {
    unsafe {
        if iPg <= (*pMap).nPg {
            let mut iElem: ::core::ffi::c_int = (iPg as ::core::ffi::c_longlong
                / 32 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
            let mut iBit: ::core::ffi::c_int = (iPg as ::core::ffi::c_longlong
                % 32 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
            let ref mut c2rust_fresh4 = *(&raw mut (*pMap).aElem as *mut u32_0)
                .offset(iElem as isize);
            *c2rust_fresh4
                |= ((1 as ::core::ffi::c_int as u32_0) << iBit) as ::core::ffi::c_uint;
        }
    }
}
unsafe extern "C" fn recoverBitmapQuery(
    mut pMap: *mut RecoverBitmap,
    mut iPg: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if iPg <= (*pMap).nPg && iPg > 0 as ::core::ffi::c_longlong {
            let mut iElem: ::core::ffi::c_int = (iPg as ::core::ffi::c_longlong
                / 32 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
            let mut iBit: ::core::ffi::c_int = (iPg as ::core::ffi::c_longlong
                % 32 as ::core::ffi::c_longlong) as ::core::ffi::c_int;
            ret = if *(&raw mut (*pMap).aElem as *mut u32_0).offset(iElem as isize)
                & (1 as ::core::ffi::c_int as u32_0) << iBit != 0
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
        }
        return ret;
    }
}
unsafe extern "C" fn recoverDbError(
    mut p: *mut sqlite3_recover,
    mut db: *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        return recoverError(
            p,
            sqlite3_errcode(db),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
}
unsafe extern "C" fn recoverPrepare(
    mut p: *mut sqlite3_recover,
    mut db: *mut sqlite3,
    mut zSql: *const ::core::ffi::c_char,
) -> *mut sqlite3_stmt {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if (*p).errCode == SQLITE_OK {
            if sqlite3_prepare_v2(
                db,
                zSql,
                -1 as ::core::ffi::c_int,
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            ) != 0
            {
                recoverDbError(p, db);
            }
        }
        return pStmt;
    }
}
unsafe extern "C" fn recoverPreparePrintf(
    mut p: *mut sqlite3_recover,
    mut db: *mut sqlite3,
    mut zFmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> *mut sqlite3_stmt {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if (*p).errCode == SQLITE_OK {
            let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut ap = c2rust_args.clone();
            z = sqlite3_vmprintf(zFmt, ap);
            if z.is_null() {
                (*p).errCode = SQLITE_NOMEM;
            } else {
                pStmt = recoverPrepare(p, db, z);
                sqlite3_free(z as *mut ::core::ffi::c_void);
            }
        }
        return pStmt;
    }
}
unsafe extern "C" fn recoverReset(
    mut p: *mut sqlite3_recover,
    mut pStmt: *mut sqlite3_stmt,
) -> *mut sqlite3_stmt {
    unsafe {
        let mut rc: ::core::ffi::c_int = sqlite3_reset(pStmt);
        if rc != SQLITE_OK && rc != SQLITE_CONSTRAINT && (*p).errCode == SQLITE_OK {
            recoverDbError(p, sqlite3_db_handle(pStmt));
        }
        return pStmt;
    }
}
unsafe extern "C" fn recoverFinalize(
    mut p: *mut sqlite3_recover,
    mut pStmt: *mut sqlite3_stmt,
) {
    unsafe {
        let mut db: *mut sqlite3 = sqlite3_db_handle(pStmt);
        let mut rc: ::core::ffi::c_int = sqlite3_finalize(pStmt);
        if rc != SQLITE_OK && (*p).errCode == SQLITE_OK {
            recoverDbError(p, db);
        }
    }
}
unsafe extern "C" fn recoverExec(
    mut p: *mut sqlite3_recover,
    mut db: *mut sqlite3,
    mut zSql: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if (*p).errCode == SQLITE_OK {
            let mut rc: ::core::ffi::c_int = sqlite3_exec(
                db,
                zSql,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            if rc != 0 {
                recoverDbError(p, db);
            }
        }
        return (*p).errCode;
    }
}
unsafe extern "C" fn recoverBindValue(
    mut p: *mut sqlite3_recover,
    mut pStmt: *mut sqlite3_stmt,
    mut iBind: ::core::ffi::c_int,
    mut pVal: *mut sqlite3_value,
) {
    unsafe {
        if (*p).errCode == SQLITE_OK {
            let mut rc: ::core::ffi::c_int = sqlite3_bind_value(pStmt, iBind, pVal);
            if rc != 0 {
                recoverError(p, rc, ::core::ptr::null::<::core::ffi::c_char>());
            }
        }
    }
}
unsafe extern "C" fn recoverMPrintf(
    mut p: *mut sqlite3_recover,
    mut zFmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut ap = c2rust_args.clone();
        z = sqlite3_vmprintf(zFmt, ap);
        if (*p).errCode == SQLITE_OK {
            if z.is_null() {
                (*p).errCode = SQLITE_NOMEM;
            }
        } else {
            sqlite3_free(z as *mut ::core::ffi::c_void);
            z = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return z;
    }
}
unsafe extern "C" fn recoverPageCount(mut p: *mut sqlite3_recover) -> i64_0 {
    unsafe {
        let mut nPg: i64_0 = 0 as i64_0;
        if (*p).errCode == SQLITE_OK {
            let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            pStmt = recoverPreparePrintf(
                p,
                (*p).dbIn,
                b"PRAGMA %Q.page_count\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).zDb,
            );
            if !pStmt.is_null() {
                sqlite3_step(pStmt);
                nPg = sqlite3_column_int64(pStmt, 0 as ::core::ffi::c_int) as i64_0;
            }
            recoverFinalize(p, pStmt);
        }
        return nPg;
    }
}
unsafe extern "C" fn recoverReadI32(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut pBlob: *const ::core::ffi::c_uchar = ::core::ptr::null::<
            ::core::ffi::c_uchar,
        >();
        let mut nBlob: ::core::ffi::c_int = 0;
        let mut iInt: ::core::ffi::c_int = 0;
        nBlob = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
        pBlob = sqlite3_value_blob(*argv.offset(0 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_uchar;
        iInt = sqlite3_value_int(*argv.offset(1 as ::core::ffi::c_int as isize))
            & 0xffff as ::core::ffi::c_int;
        if (iInt + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int <= nBlob {
            let mut a: *const ::core::ffi::c_uchar = pBlob
                .offset((iInt * 4 as ::core::ffi::c_int) as isize)
                as *const ::core::ffi::c_uchar;
            let mut iVal: i64_0 = ((*a.offset(0 as ::core::ffi::c_int as isize) as i64_0)
                << 24 as ::core::ffi::c_int)
                + ((*a.offset(1 as ::core::ffi::c_int as isize) as i64_0)
                    << 16 as ::core::ffi::c_int)
                + ((*a.offset(2 as ::core::ffi::c_int as isize) as i64_0)
                    << 8 as ::core::ffi::c_int)
                + ((*a.offset(3 as ::core::ffi::c_int as isize) as i64_0)
                    << 0 as ::core::ffi::c_int);
            sqlite3_result_int64(context, iVal as sqlite3_int64);
        }
    }
}
unsafe extern "C" fn recoverPageIsUsed(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut sqlite3_recover = sqlite3_user_data(pCtx)
            as *mut sqlite3_recover;
        let mut pgno: i64_0 = sqlite3_value_int64(
            *apArg.offset(0 as ::core::ffi::c_int as isize),
        ) as i64_0;
        sqlite3_result_int(pCtx, recoverBitmapQuery((*p).laf.pUsed, pgno));
    }
}
unsafe extern "C" fn recoverGetPage(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut sqlite3_recover = sqlite3_user_data(pCtx)
            as *mut sqlite3_recover;
        let mut pgno: i64_0 = sqlite3_value_int64(
            *apArg.offset(0 as ::core::ffi::c_int as isize),
        ) as i64_0;
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if pgno == 0 as ::core::ffi::c_longlong {
            let mut nPg: i64_0 = recoverPageCount(p);
            sqlite3_result_int64(pCtx, nPg as sqlite3_int64);
            return;
        } else {
            if (*p).pGetPage.is_null() {
                (*p).pGetPage = recoverPreparePrintf(
                    p,
                    (*p).dbIn,
                    b"SELECT data FROM sqlite_dbpage(%Q) WHERE pgno=?\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*p).zDb,
                );
                pStmt = (*p).pGetPage;
            } else if (*p).errCode == SQLITE_OK {
                pStmt = (*p).pGetPage;
            }
            if !pStmt.is_null() {
                sqlite3_bind_int64(
                    pStmt,
                    1 as ::core::ffi::c_int,
                    pgno as sqlite3_int64,
                );
                if SQLITE_ROW == sqlite3_step(pStmt) {
                    let mut aPg: *const u8_0 = ::core::ptr::null::<u8_0>();
                    let mut nPg_0: ::core::ffi::c_int = 0;
                    aPg = sqlite3_column_blob(pStmt, 0 as ::core::ffi::c_int)
                        as *const u8_0;
                    nPg_0 = sqlite3_column_bytes(pStmt, 0 as ::core::ffi::c_int);
                    if pgno == 1 as ::core::ffi::c_longlong && nPg_0 == (*p).pgsz
                        && 0 as ::core::ffi::c_int
                            == memcmp(
                                (*p).pPage1Cache as *const ::core::ffi::c_void,
                                aPg as *const ::core::ffi::c_void,
                                nPg_0 as size_t,
                            )
                    {
                        aPg = (*p).pPage1Disk;
                    }
                    sqlite3_result_blob(
                        pCtx,
                        aPg as *const ::core::ffi::c_void,
                        nPg_0 - (*p).nReserve,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                }
                recoverReset(p, pStmt);
            }
        }
        if (*p).errCode != 0 {
            if !(*p).zErrMsg.is_null() {
                sqlite3_result_error(pCtx, (*p).zErrMsg, -1 as ::core::ffi::c_int);
            }
            sqlite3_result_error_code(pCtx, (*p).errCode);
        }
    }
}
unsafe extern "C" fn recoverUnusedString(
    mut z: *const ::core::ffi::c_char,
    mut zA: *const ::core::ffi::c_char,
    mut zB: *const ::core::ffi::c_char,
    mut zBuf: *mut ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        if strstr(z, zA).is_null() {
            return zA;
        }
        if strstr(z, zB).is_null() {
            return zB;
        }
        loop {
            let c2rust_fresh11 = i;
            i = i.wrapping_add(1);
            sqlite3_snprintf(
                20 as ::core::ffi::c_int,
                zBuf,
                b"(%s%u)\0".as_ptr() as *const ::core::ffi::c_char,
                zA,
                c2rust_fresh11,
            );
            if strstr(z, zBuf).is_null() {
                break;
            }
        }
        return zBuf;
    }
}
unsafe extern "C" fn recoverEscapeCrlf(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zText: *const ::core::ffi::c_char = sqlite3_value_text(
            *argv.offset(0 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        if !zText.is_null()
            && *zText.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\'' as i32
        {
            let mut nText: ::core::ffi::c_int = sqlite3_value_bytes(
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            let mut i: ::core::ffi::c_int = 0;
            let mut zBuf1: [::core::ffi::c_char; 20] = [0; 20];
            let mut zBuf2: [::core::ffi::c_char; 20] = [0; 20];
            let mut zNL: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            let mut zCR: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            let mut nCR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut nNL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while *zText.offset(i as isize) != 0 {
                if zNL.is_null()
                    && *zText.offset(i as isize) as ::core::ffi::c_int == '\n' as i32
                {
                    zNL = recoverUnusedString(
                        zText,
                        b"\\n\0".as_ptr() as *const ::core::ffi::c_char,
                        b"\\012\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut zBuf1 as *mut ::core::ffi::c_char,
                    );
                    nNL = strlen(zNL) as ::core::ffi::c_int;
                }
                if zCR.is_null()
                    && *zText.offset(i as isize) as ::core::ffi::c_int == '\r' as i32
                {
                    zCR = recoverUnusedString(
                        zText,
                        b"\\r\0".as_ptr() as *const ::core::ffi::c_char,
                        b"\\015\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut zBuf2 as *mut ::core::ffi::c_char,
                    );
                    nCR = strlen(zCR) as ::core::ffi::c_int;
                }
                i += 1;
            }
            if !zNL.is_null() || !zCR.is_null() {
                let mut iOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut nMax: i64_0 = (if nNL > nCR { nNL } else { nCR }) as i64_0;
                let mut nAlloc: i64_0 = nMax * nText as i64_0
                    + (nMax + 64 as i64_0) * 2 as i64_0;
                let mut zOut: *mut ::core::ffi::c_char = sqlite3_malloc64(
                    nAlloc as sqlite3_uint64,
                ) as *mut ::core::ffi::c_char;
                if zOut.is_null() {
                    sqlite3_result_error_nomem(context);
                    return;
                }
                if !zNL.is_null() && !zCR.is_null() {
                    memcpy(
                        zOut.offset(iOut as isize) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        b"replace(replace(\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        16 as size_t,
                    );
                    iOut += 16 as ::core::ffi::c_int;
                } else {
                    memcpy(
                        zOut.offset(iOut as isize) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        b"replace(\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        8 as size_t,
                    );
                    iOut += 8 as ::core::ffi::c_int;
                }
                i = 0 as ::core::ffi::c_int;
                while *zText.offset(i as isize) != 0 {
                    if *zText.offset(i as isize) as ::core::ffi::c_int == '\n' as i32 {
                        memcpy(
                            zOut.offset(iOut as isize) as *mut ::core::ffi::c_char
                                as *mut ::core::ffi::c_void,
                            zNL as *const ::core::ffi::c_void,
                            nNL as size_t,
                        );
                        iOut += nNL;
                    } else if *zText.offset(i as isize) as ::core::ffi::c_int
                        == '\r' as i32
                    {
                        memcpy(
                            zOut.offset(iOut as isize) as *mut ::core::ffi::c_char
                                as *mut ::core::ffi::c_void,
                            zCR as *const ::core::ffi::c_void,
                            nCR as size_t,
                        );
                        iOut += nCR;
                    } else {
                        *zOut.offset(iOut as isize) = *zText.offset(i as isize);
                        iOut += 1;
                    }
                    i += 1;
                }
                if !zNL.is_null() {
                    memcpy(
                        zOut.offset(iOut as isize) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        b",'\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        2 as size_t,
                    );
                    iOut += 2 as ::core::ffi::c_int;
                    memcpy(
                        zOut.offset(iOut as isize) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        zNL as *const ::core::ffi::c_void,
                        nNL as size_t,
                    );
                    iOut += nNL;
                    memcpy(
                        zOut.offset(iOut as isize) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        b"', char(10))\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        12 as size_t,
                    );
                    iOut += 12 as ::core::ffi::c_int;
                }
                if !zCR.is_null() {
                    memcpy(
                        zOut.offset(iOut as isize) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        b",'\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        2 as size_t,
                    );
                    iOut += 2 as ::core::ffi::c_int;
                    memcpy(
                        zOut.offset(iOut as isize) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        zCR as *const ::core::ffi::c_void,
                        nCR as size_t,
                    );
                    iOut += nCR;
                    memcpy(
                        zOut.offset(iOut as isize) as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        b"', char(13))\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        12 as size_t,
                    );
                    iOut += 12 as ::core::ffi::c_int;
                }
                sqlite3_result_text(
                    context,
                    zOut,
                    iOut,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                sqlite3_free(zOut as *mut ::core::ffi::c_void);
                return;
            }
        }
        sqlite3_result_value(context, *argv.offset(0 as ::core::ffi::c_int as isize));
    }
}
unsafe extern "C" fn recoverCacheSchema(
    mut p: *mut sqlite3_recover,
) -> ::core::ffi::c_int {
    unsafe {
        return recoverExec(
            p,
            (*p).dbOut,
            b"WITH RECURSIVE pages(p) AS (  SELECT 1    UNION  SELECT child FROM sqlite_dbptr('getpage()'), pages WHERE pgno=p)INSERT INTO recovery.schema SELECT  max(CASE WHEN field=0 THEN value ELSE NULL END),  max(CASE WHEN field=1 THEN value ELSE NULL END),  max(CASE WHEN field=2 THEN value ELSE NULL END),  max(CASE WHEN field=3 THEN value ELSE NULL END),  max(CASE WHEN field=4 THEN value ELSE NULL END)FROM sqlite_dbdata('getpage()') WHERE pgno IN (  SELECT p FROM pages) GROUP BY pgno, cell\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn recoverSqlCallback(
    mut p: *mut sqlite3_recover,
    mut zSql: *const ::core::ffi::c_char,
) {
    unsafe {
        if (*p).errCode == SQLITE_OK && (*p).xSql.is_some() {
            let mut res: ::core::ffi::c_int = (*p)
                .xSql
                .expect("non-null function pointer")((*p).pSqlCtx, zSql);
            if res != 0 {
                recoverError(
                    p,
                    SQLITE_ERROR,
                    b"callback returned an error - %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    res,
                );
            }
        }
    }
}
unsafe extern "C" fn recoverTransferSettings(mut p: *mut sqlite3_recover) {
    unsafe {
        let mut aPragma: [*const ::core::ffi::c_char; 5] = [
            b"encoding\0".as_ptr() as *const ::core::ffi::c_char,
            b"page_size\0".as_ptr() as *const ::core::ffi::c_char,
            b"auto_vacuum\0".as_ptr() as *const ::core::ffi::c_char,
            b"user_version\0".as_ptr() as *const ::core::ffi::c_char,
            b"application_id\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut ii: ::core::ffi::c_int = 0;
        if (*p).errCode == SQLITE_OK {
            let mut db2: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
            let mut rc: ::core::ffi::c_int = sqlite3_open(
                b"\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut db2,
            );
            if rc != SQLITE_OK {
                recoverDbError(p, db2);
                return;
            }
            ii = 0 as ::core::ffi::c_int;
            while ii
                < (::core::mem::size_of::<[*const ::core::ffi::c_char; 5]>() as usize)
                    .wrapping_div(
                        ::core::mem::size_of::<*const ::core::ffi::c_char>() as usize,
                    ) as ::core::ffi::c_int
            {
                let mut zPrag: *const ::core::ffi::c_char = aPragma[ii as usize];
                let mut p1: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
                p1 = recoverPreparePrintf(
                    p,
                    (*p).dbIn,
                    b"PRAGMA %Q.%s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*p).zDb,
                    zPrag,
                );
                if (*p).errCode == SQLITE_OK && sqlite3_step(p1) == SQLITE_ROW {
                    let mut zArg: *const ::core::ffi::c_char = sqlite3_column_text(
                        p1,
                        0 as ::core::ffi::c_int,
                    ) as *const ::core::ffi::c_char;
                    let mut z2: *mut ::core::ffi::c_char = recoverMPrintf(
                        p,
                        b"PRAGMA %s = %Q\0".as_ptr() as *const ::core::ffi::c_char,
                        zPrag,
                        zArg,
                    );
                    recoverSqlCallback(p, z2);
                    recoverExec(p, db2, z2);
                    sqlite3_free(z2 as *mut ::core::ffi::c_void);
                    if zArg.is_null() {
                        recoverError(
                            p,
                            SQLITE_NOMEM,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                    }
                }
                recoverFinalize(p, p1);
                ii += 1;
            }
            recoverExec(
                p,
                db2,
                b"CREATE TABLE t1(a); DROP TABLE t1;\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            if (*p).errCode == SQLITE_OK {
                let mut db: *mut sqlite3 = (*p).dbOut;
                let mut pBackup: *mut sqlite3_backup = sqlite3_backup_init(
                    db,
                    b"main\0".as_ptr() as *const ::core::ffi::c_char,
                    db2,
                    b"main\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if !pBackup.is_null() {
                    sqlite3_backup_step(pBackup, -1 as ::core::ffi::c_int);
                    (*p).errCode = sqlite3_backup_finish(pBackup);
                } else {
                    recoverDbError(p, db);
                }
            }
            sqlite3_close(db2);
        }
    }
}
unsafe extern "C" fn recoverOpenOutput(
    mut p: *mut sqlite3_recover,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aFunc: [Func; 4] = [
            Func {
                zName: b"getpage\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                xFunc: Some(
                    recoverGetPage
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
            },
            Func {
                zName: b"page_is_used\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                xFunc: Some(
                    recoverPageIsUsed
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
            },
            Func {
                zName: b"read_i32\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 2 as ::core::ffi::c_int,
                xFunc: Some(
                    recoverReadI32
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
            },
            Func {
                zName: b"escape_crlf\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                xFunc: Some(
                    recoverEscapeCrlf
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
            },
        ];
        let flags: ::core::ffi::c_int = SQLITE_OPEN_URI | SQLITE_OPEN_CREATE
            | SQLITE_OPEN_READWRITE;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut ii: ::core::ffi::c_int = 0;
        if sqlite3_open_v2(
            (*p).zUri,
            &raw mut db,
            flags,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ) != 0
        {
            recoverDbError(p, db);
        }
        if (*p).errCode == SQLITE_OK {
            (*p).errCode = sqlite3_dbdata_init(
                db,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                ::core::ptr::null::<sqlite3_api_routines>(),
            );
        }
        ii = 0 as ::core::ffi::c_int;
        while (*p).errCode == SQLITE_OK
            && ii
                < (::core::mem::size_of::<[Func; 4]>() as usize)
                    .wrapping_div(::core::mem::size_of::<Func>() as usize)
                    as ::core::ffi::c_int
        {
            (*p).errCode = sqlite3_create_function(
                db,
                aFunc[ii as usize].zName,
                aFunc[ii as usize].nArg,
                SQLITE_UTF8,
                p as *mut ::core::ffi::c_void,
                aFunc[ii as usize].xFunc,
                None,
                None,
            );
            ii += 1;
        }
        (*p).dbOut = db;
        return (*p).errCode;
    }
}
unsafe extern "C" fn recoverOpenRecovery(mut p: *mut sqlite3_recover) {
    unsafe {
        let mut zSql: *mut ::core::ffi::c_char = recoverMPrintf(
            p,
            b"ATTACH %Q AS recovery;\0".as_ptr() as *const ::core::ffi::c_char,
            (*p).zStateDb,
        );
        recoverExec(p, (*p).dbOut, zSql);
        recoverExec(
            p,
            (*p).dbOut,
            b"PRAGMA writable_schema = 1;CREATE TABLE recovery.map(pgno INTEGER PRIMARY KEY, parent INT);CREATE TABLE recovery.schema(type, name, tbl_name, rootpage, sql);\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn recoverAddTable(
    mut p: *mut sqlite3_recover,
    mut zName: *const ::core::ffi::c_char,
    mut iRoot: i64_0,
) {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = recoverPreparePrintf(
            p,
            (*p).dbOut,
            b"PRAGMA table_xinfo(%Q)\0".as_ptr() as *const ::core::ffi::c_char,
            zName,
        );
        if !pStmt.is_null() {
            let mut iPk: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
            let mut iBind: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut pNew: *mut RecoverTable = ::core::ptr::null_mut::<RecoverTable>();
            let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut nName: ::core::ffi::c_int = recoverStrlen(zName);
            let mut nByte: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while sqlite3_step(pStmt) == SQLITE_ROW {
                nCol += 1;
                nByte
                    += sqlite3_column_bytes(pStmt, 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int;
            }
            nByte = (nByte as ::core::ffi::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<RecoverTable>() as usize)
                        .wrapping_add(
                            (nCol as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<RecoverColumn>() as usize,
                                ),
                        )
                        .wrapping_add(nName as usize)
                        .wrapping_add(1 as usize) as ::core::ffi::c_ulong,
                ) as ::core::ffi::c_int as ::core::ffi::c_int;
            recoverReset(p, pStmt);
            pNew = recoverMalloc(p, nByte as i64_0) as *mut RecoverTable;
            if !pNew.is_null() {
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut iField: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut csr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                (*pNew).aCol = pNew.offset(1 as ::core::ffi::c_int as isize)
                    as *mut RecoverTable as *mut RecoverColumn;
                csr = (*pNew).aCol.offset(nCol as isize) as *mut RecoverColumn
                    as *mut ::core::ffi::c_char;
                (*pNew).zTab = csr;
                (*pNew).nCol = nCol;
                (*pNew).iRoot = iRoot as u32_0;
                memcpy(
                    csr as *mut ::core::ffi::c_void,
                    zName as *const ::core::ffi::c_void,
                    nName as size_t,
                );
                csr = csr.offset((nName + 1 as ::core::ffi::c_int) as isize);
                i = 0 as ::core::ffi::c_int;
                while sqlite3_step(pStmt) == SQLITE_ROW {
                    let mut iPKF: ::core::ffi::c_int = sqlite3_column_int(
                        pStmt,
                        5 as ::core::ffi::c_int,
                    );
                    let mut n: ::core::ffi::c_int = sqlite3_column_bytes(
                        pStmt,
                        1 as ::core::ffi::c_int,
                    );
                    let mut z: *const ::core::ffi::c_char = sqlite3_column_text(
                        pStmt,
                        1 as ::core::ffi::c_int,
                    ) as *const ::core::ffi::c_char;
                    let mut zType: *const ::core::ffi::c_char = sqlite3_column_text(
                        pStmt,
                        2 as ::core::ffi::c_int,
                    ) as *const ::core::ffi::c_char;
                    let mut eHidden: ::core::ffi::c_int = sqlite3_column_int(
                        pStmt,
                        6 as ::core::ffi::c_int,
                    );
                    if iPk == -1 as ::core::ffi::c_int && iPKF == 1 as ::core::ffi::c_int
                        && sqlite3_stricmp(
                            b"integer\0".as_ptr() as *const ::core::ffi::c_char,
                            zType,
                        ) == 0
                    {
                        iPk = i;
                    }
                    if iPKF > 1 as ::core::ffi::c_int {
                        iPk = -2 as ::core::ffi::c_int;
                    }
                    let ref mut c2rust_fresh7 = (*(*pNew).aCol.offset(i as isize)).zCol;
                    *c2rust_fresh7 = csr;
                    (*(*pNew).aCol.offset(i as isize)).eHidden = eHidden;
                    if eHidden == RECOVER_EHIDDEN_VIRTUAL {
                        (*(*pNew).aCol.offset(i as isize)).iField = -1
                            as ::core::ffi::c_int;
                    } else {
                        let c2rust_fresh8 = iField;
                        iField = iField + 1;
                        (*(*pNew).aCol.offset(i as isize)).iField = c2rust_fresh8;
                    }
                    if eHidden != RECOVER_EHIDDEN_VIRTUAL
                        && eHidden != RECOVER_EHIDDEN_STORED
                    {
                        let c2rust_fresh9 = iBind;
                        iBind = iBind + 1;
                        (*(*pNew).aCol.offset(i as isize)).iBind = c2rust_fresh9;
                    }
                    memcpy(
                        csr as *mut ::core::ffi::c_void,
                        z as *const ::core::ffi::c_void,
                        n as size_t,
                    );
                    csr = csr.offset((n + 1 as ::core::ffi::c_int) as isize);
                    i += 1;
                }
                (*pNew).pNext = (*p).pTblList;
                (*p).pTblList = pNew;
                (*pNew).bIntkey = 1 as ::core::ffi::c_int;
            }
            recoverFinalize(p, pStmt);
            pStmt = recoverPreparePrintf(
                p,
                (*p).dbOut,
                b"PRAGMA index_xinfo(%Q)\0".as_ptr() as *const ::core::ffi::c_char,
                zName,
            );
            while !pStmt.is_null() && sqlite3_step(pStmt) == SQLITE_ROW {
                let mut iField_0: ::core::ffi::c_int = sqlite3_column_int(
                    pStmt,
                    0 as ::core::ffi::c_int,
                );
                let mut iCol: ::core::ffi::c_int = sqlite3_column_int(
                    pStmt,
                    1 as ::core::ffi::c_int,
                );
                (*(*pNew).aCol.offset(iCol as isize)).iField = iField_0;
                (*pNew).bIntkey = 0 as ::core::ffi::c_int;
                iPk = -2 as ::core::ffi::c_int;
            }
            recoverFinalize(p, pStmt);
            if (*p).errCode == SQLITE_OK {
                if iPk >= 0 as ::core::ffi::c_int {
                    (*(*pNew).aCol.offset(iPk as isize)).bIPK = 1 as ::core::ffi::c_int;
                } else if (*pNew).bIntkey != 0 {
                    let c2rust_fresh10 = iBind;
                    iBind = iBind + 1;
                    (*pNew).iRowidBind = c2rust_fresh10;
                }
            }
        }
    }
}
unsafe extern "C" fn recoverWriteSchema1(
    mut p: *mut sqlite3_recover,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSelect: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut pTblname: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        pSelect = recoverPrepare(
            p,
            (*p).dbOut,
            b"WITH dbschema(rootpage, name, sql, tbl, isVirtual, isIndex) AS (  SELECT rootpage, name, sql,     type='table',     sql LIKE 'create virtual%',    (type='index' AND (sql LIKE '%unique%' OR ?1))  FROM recovery.schema)SELECT rootpage, tbl, isVirtual, name, sql FROM dbschema   WHERE tbl OR isIndex  ORDER BY tbl DESC, name=='sqlite_sequence' DESC\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        pTblname = recoverPrepare(
            p,
            (*p).dbOut,
            b"SELECT name FROM sqlite_schema WHERE type='table' ORDER BY rowid DESC LIMIT 1\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        if !pSelect.is_null() {
            sqlite3_bind_int(pSelect, 1 as ::core::ffi::c_int, (*p).bSlowIndexes);
            while sqlite3_step(pSelect) == SQLITE_ROW {
                let mut iRoot: i64_0 = sqlite3_column_int64(
                    pSelect,
                    0 as ::core::ffi::c_int,
                ) as i64_0;
                let mut bTable: ::core::ffi::c_int = sqlite3_column_int(
                    pSelect,
                    1 as ::core::ffi::c_int,
                );
                let mut bVirtual: ::core::ffi::c_int = sqlite3_column_int(
                    pSelect,
                    2 as ::core::ffi::c_int,
                );
                let mut zName: *const ::core::ffi::c_char = sqlite3_column_text(
                    pSelect,
                    3 as ::core::ffi::c_int,
                ) as *const ::core::ffi::c_char;
                let mut zSql: *const ::core::ffi::c_char = sqlite3_column_text(
                    pSelect,
                    4 as ::core::ffi::c_int,
                ) as *const ::core::ffi::c_char;
                let mut zFree: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut rc: ::core::ffi::c_int = SQLITE_OK;
                if bVirtual != 0 {
                    zFree = recoverMPrintf(
                        p,
                        b"INSERT INTO sqlite_schema VALUES('table', %Q, %Q, 0, %Q)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        zName,
                        zName,
                        zSql,
                    );
                    zSql = zFree as *const ::core::ffi::c_char;
                }
                rc = sqlite3_exec(
                    (*p).dbOut,
                    zSql,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
                if rc == SQLITE_OK {
                    recoverSqlCallback(p, zSql);
                    if bTable != 0 && bVirtual == 0 {
                        if SQLITE_ROW == sqlite3_step(pTblname) {
                            let mut zTbl: *const ::core::ffi::c_char = sqlite3_column_text(
                                pTblname,
                                0 as ::core::ffi::c_int,
                            ) as *const ::core::ffi::c_char;
                            if !zTbl.is_null() {
                                recoverAddTable(p, zTbl, iRoot);
                            }
                        }
                        recoverReset(p, pTblname);
                    }
                } else if rc != SQLITE_ERROR {
                    recoverDbError(p, (*p).dbOut);
                }
                sqlite3_free(zFree as *mut ::core::ffi::c_void);
            }
        }
        recoverFinalize(p, pSelect);
        recoverFinalize(p, pTblname);
        return (*p).errCode;
    }
}
unsafe extern "C" fn recoverWriteSchema2(
    mut p: *mut sqlite3_recover,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSelect: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        pSelect = recoverPrepare(
            p,
            (*p).dbOut,
            if (*p).bSlowIndexes != 0 {
                b"SELECT rootpage, sql FROM recovery.schema   WHERE type!='table' AND type!='index'\0"
                    .as_ptr() as *const ::core::ffi::c_char
            } else {
                b"SELECT rootpage, sql FROM recovery.schema   WHERE type!='table' AND (type!='index' OR sql NOT LIKE '%unique%')\0"
                    .as_ptr() as *const ::core::ffi::c_char
            },
        );
        if !pSelect.is_null() {
            while sqlite3_step(pSelect) == SQLITE_ROW {
                let mut zSql: *const ::core::ffi::c_char = sqlite3_column_text(
                    pSelect,
                    1 as ::core::ffi::c_int,
                ) as *const ::core::ffi::c_char;
                let mut rc: ::core::ffi::c_int = sqlite3_exec(
                    (*p).dbOut,
                    zSql,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
                if rc == SQLITE_OK {
                    recoverSqlCallback(p, zSql);
                } else if rc != SQLITE_ERROR {
                    recoverDbError(p, (*p).dbOut);
                }
            }
        }
        recoverFinalize(p, pSelect);
        return (*p).errCode;
    }
}
unsafe extern "C" fn recoverInsertStmt(
    mut p: *mut sqlite3_recover,
    mut pTab: *mut RecoverTable,
    mut nField: ::core::ffi::c_int,
) -> *mut sqlite3_stmt {
    unsafe {
        let mut pRet: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut zSep: *const ::core::ffi::c_char = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        let mut zSqlSep: *const ::core::ffi::c_char = b"\0".as_ptr()
            as *const ::core::ffi::c_char;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zFinal: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zBind: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut ii: ::core::ffi::c_int = 0;
        let mut bSql: ::core::ffi::c_int = if (*p).xSql.is_some() {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        if nField <= 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<sqlite3_stmt>();
        }
        zSql = recoverMPrintf(
            p,
            b"INSERT OR IGNORE INTO %Q(\0".as_ptr() as *const ::core::ffi::c_char,
            (*pTab).zTab,
        );
        if (*pTab).iRowidBind != 0 {
            zSql = recoverMPrintf(
                p,
                b"%z_rowid_\0".as_ptr() as *const ::core::ffi::c_char,
                zSql,
            );
            if bSql != 0 {
                zBind = recoverMPrintf(
                    p,
                    b"%zquote(?%d)\0".as_ptr() as *const ::core::ffi::c_char,
                    zBind,
                    (*pTab).iRowidBind,
                );
            } else {
                zBind = recoverMPrintf(
                    p,
                    b"%z?%d\0".as_ptr() as *const ::core::ffi::c_char,
                    zBind,
                    (*pTab).iRowidBind,
                );
            }
            zSqlSep = b"||', '||\0".as_ptr() as *const ::core::ffi::c_char;
            zSep = b", \0".as_ptr() as *const ::core::ffi::c_char;
        }
        ii = 0 as ::core::ffi::c_int;
        while ii < nField {
            let mut eHidden: ::core::ffi::c_int = (*(*pTab).aCol.offset(ii as isize))
                .eHidden;
            if eHidden != RECOVER_EHIDDEN_VIRTUAL && eHidden != RECOVER_EHIDDEN_STORED {
                zSql = recoverMPrintf(
                    p,
                    b"%z%s%Q\0".as_ptr() as *const ::core::ffi::c_char,
                    zSql,
                    zSep,
                    (*(*pTab).aCol.offset(ii as isize)).zCol,
                );
                if bSql != 0 {
                    zBind = recoverMPrintf(
                        p,
                        b"%z%sescape_crlf(quote(?%d))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        zBind,
                        zSqlSep,
                        (*(*pTab).aCol.offset(ii as isize)).iBind,
                    );
                    zSqlSep = b"||', '||\0".as_ptr() as *const ::core::ffi::c_char;
                } else {
                    zBind = recoverMPrintf(
                        p,
                        b"%z%s?%d\0".as_ptr() as *const ::core::ffi::c_char,
                        zBind,
                        zSep,
                        (*(*pTab).aCol.offset(ii as isize)).iBind,
                    );
                }
                zSep = b", \0".as_ptr() as *const ::core::ffi::c_char;
            }
            ii += 1;
        }
        if bSql != 0 {
            zFinal = recoverMPrintf(
                p,
                b"SELECT %Q || ') VALUES (' || %s || ')'\0".as_ptr()
                    as *const ::core::ffi::c_char,
                zSql,
                zBind,
            );
        } else {
            zFinal = recoverMPrintf(
                p,
                b"%s) VALUES (%s)\0".as_ptr() as *const ::core::ffi::c_char,
                zSql,
                zBind,
            );
        }
        pRet = recoverPrepare(p, (*p).dbOut, zFinal);
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        sqlite3_free(zBind as *mut ::core::ffi::c_void);
        sqlite3_free(zFinal as *mut ::core::ffi::c_void);
        return pRet;
    }
}
unsafe extern "C" fn recoverFindTable(
    mut p: *mut sqlite3_recover,
    mut iRoot: u32_0,
) -> *mut RecoverTable {
    unsafe {
        let mut pRet: *mut RecoverTable = ::core::ptr::null_mut::<RecoverTable>();
        pRet = (*p).pTblList;
        while !pRet.is_null() && (*pRet).iRoot != iRoot {
            pRet = (*pRet).pNext;
        }
        return pRet;
    }
}
unsafe extern "C" fn recoverLostAndFoundCreate(
    mut p: *mut sqlite3_recover,
    mut nField: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut zTbl: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pProbe: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut ii: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        pProbe = recoverPrepare(
            p,
            (*p).dbOut,
            b"SELECT 1 FROM sqlite_schema WHERE name=?\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        ii = -1 as ::core::ffi::c_int;
        while zTbl.is_null() && (*p).errCode == SQLITE_OK
            && ii < 1000 as ::core::ffi::c_int
        {
            let mut bFail: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if ii < 0 as ::core::ffi::c_int {
                zTbl = recoverMPrintf(
                    p,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*p).zLostAndFound,
                );
            } else {
                zTbl = recoverMPrintf(
                    p,
                    b"%s_%d\0".as_ptr() as *const ::core::ffi::c_char,
                    (*p).zLostAndFound,
                    ii,
                );
            }
            if (*p).errCode == SQLITE_OK {
                sqlite3_bind_text(
                    pProbe,
                    1 as ::core::ffi::c_int,
                    zTbl,
                    -1 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
                if SQLITE_ROW == sqlite3_step(pProbe) {
                    bFail = 1 as ::core::ffi::c_int;
                }
                recoverReset(p, pProbe);
            }
            if bFail != 0 {
                sqlite3_clear_bindings(pProbe);
                sqlite3_free(zTbl as *mut ::core::ffi::c_void);
                zTbl = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            ii += 1;
        }
        recoverFinalize(p, pProbe);
        if !zTbl.is_null() {
            let mut zSep: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            let mut zField: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            zSep = b"rootpgno INTEGER, pgno INTEGER, nfield INTEGER, id INTEGER, \0"
                .as_ptr() as *const ::core::ffi::c_char;
            ii = 0 as ::core::ffi::c_int;
            while (*p).errCode == SQLITE_OK && ii < nField {
                zField = recoverMPrintf(
                    p,
                    b"%z%sc%d\0".as_ptr() as *const ::core::ffi::c_char,
                    zField,
                    zSep,
                    ii,
                );
                zSep = b", \0".as_ptr() as *const ::core::ffi::c_char;
                ii += 1;
            }
            zSql = recoverMPrintf(
                p,
                b"CREATE TABLE %s(%s)\0".as_ptr() as *const ::core::ffi::c_char,
                zTbl,
                zField,
            );
            sqlite3_free(zField as *mut ::core::ffi::c_void);
            recoverExec(p, (*p).dbOut, zSql);
            recoverSqlCallback(p, zSql);
            sqlite3_free(zSql as *mut ::core::ffi::c_void);
        } else if (*p).errCode == SQLITE_OK {
            recoverError(
                p,
                SQLITE_ERROR,
                b"failed to create %s output table\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).zLostAndFound,
            );
        }
        return zTbl;
    }
}
unsafe extern "C" fn recoverLostAndFoundInsert(
    mut p: *mut sqlite3_recover,
    mut zTab: *const ::core::ffi::c_char,
    mut nField: ::core::ffi::c_int,
) -> *mut sqlite3_stmt {
    unsafe {
        let mut nTotal: ::core::ffi::c_int = nField + 4 as ::core::ffi::c_int;
        let mut ii: ::core::ffi::c_int = 0;
        let mut zBind: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pRet: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if (*p).xSql.is_none() {
            ii = 0 as ::core::ffi::c_int;
            while ii < nTotal {
                zBind = recoverMPrintf(
                    p,
                    b"%z%s?\0".as_ptr() as *const ::core::ffi::c_char,
                    zBind,
                    if !zBind.is_null() {
                        b", \0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    ii,
                );
                ii += 1;
            }
            pRet = recoverPreparePrintf(
                p,
                (*p).dbOut,
                b"INSERT INTO %s VALUES(%s)\0".as_ptr() as *const ::core::ffi::c_char,
                zTab,
                zBind,
            );
        } else {
            let mut zSep: *const ::core::ffi::c_char = b"\0".as_ptr()
                as *const ::core::ffi::c_char;
            ii = 0 as ::core::ffi::c_int;
            while ii < nTotal {
                zBind = recoverMPrintf(
                    p,
                    b"%z%squote(?)\0".as_ptr() as *const ::core::ffi::c_char,
                    zBind,
                    zSep,
                );
                zSep = b"|| ', ' ||\0".as_ptr() as *const ::core::ffi::c_char;
                ii += 1;
            }
            pRet = recoverPreparePrintf(
                p,
                (*p).dbOut,
                b"SELECT 'INSERT INTO %s VALUES(' || %s || ')'\0".as_ptr()
                    as *const ::core::ffi::c_char,
                zTab,
                zBind,
            );
        }
        sqlite3_free(zBind as *mut ::core::ffi::c_void);
        return pRet;
    }
}
unsafe extern "C" fn recoverLostAndFoundFindRoot(
    mut p: *mut sqlite3_recover,
    mut iPg: i64_0,
    mut piRoot: *mut i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pLaf: *mut RecoverStateLAF = &raw mut (*p).laf;
        if (*pLaf).pFindRoot.is_null() {
            (*pLaf).pFindRoot = recoverPrepare(
                p,
                (*p).dbOut,
                b"WITH RECURSIVE p(pgno) AS (  SELECT ?    UNION  SELECT parent FROM recovery.map AS m, p WHERE m.pgno=p.pgno) SELECT p.pgno FROM p, recovery.map m WHERE m.pgno=p.pgno     AND m.parent IS NULL\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if (*p).errCode == SQLITE_OK {
            sqlite3_bind_int64(
                (*pLaf).pFindRoot,
                1 as ::core::ffi::c_int,
                iPg as sqlite3_int64,
            );
            if sqlite3_step((*pLaf).pFindRoot) == SQLITE_ROW {
                *piRoot = sqlite3_column_int64(
                    (*pLaf).pFindRoot,
                    0 as ::core::ffi::c_int,
                ) as i64_0;
            } else {
                *piRoot = iPg;
            }
            recoverReset(p, (*pLaf).pFindRoot);
        }
        return (*p).errCode;
    }
}
unsafe extern "C" fn recoverLostAndFoundOnePage(
    mut p: *mut sqlite3_recover,
    mut iPage: i64_0,
) {
    unsafe {
        let mut pLaf: *mut RecoverStateLAF = &raw mut (*p).laf;
        let mut apVal: *mut *mut sqlite3_value = (*pLaf).apVal;
        let mut pPageData: *mut sqlite3_stmt = (*pLaf).pPageData;
        let mut pInsert: *mut sqlite3_stmt = (*pLaf).pInsert;
        let mut nVal: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut iPrevCell: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iRoot: i64_0 = 0 as i64_0;
        let mut bHaveRowid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iRowid: i64_0 = 0 as i64_0;
        let mut ii: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if recoverLostAndFoundFindRoot(p, iPage, &raw mut iRoot) != 0 {
            return;
        }
        sqlite3_bind_int64(pPageData, 1 as ::core::ffi::c_int, iPage as sqlite3_int64);
        while (*p).errCode == SQLITE_OK && SQLITE_ROW == sqlite3_step(pPageData) {
            let mut iCell: ::core::ffi::c_int = sqlite3_column_int64(
                pPageData,
                0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
            let mut iField: ::core::ffi::c_int = sqlite3_column_int64(
                pPageData,
                1 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
            if iPrevCell != iCell && nVal >= 0 as ::core::ffi::c_int {
                sqlite3_bind_int64(
                    pInsert,
                    1 as ::core::ffi::c_int,
                    iRoot as sqlite3_int64,
                );
                sqlite3_bind_int64(
                    pInsert,
                    2 as ::core::ffi::c_int,
                    iPage as sqlite3_int64,
                );
                sqlite3_bind_int(pInsert, 3 as ::core::ffi::c_int, nVal);
                if bHaveRowid != 0 {
                    sqlite3_bind_int64(
                        pInsert,
                        4 as ::core::ffi::c_int,
                        iRowid as sqlite3_int64,
                    );
                }
                ii = 0 as ::core::ffi::c_int;
                while ii < nVal {
                    recoverBindValue(
                        p,
                        pInsert,
                        5 as ::core::ffi::c_int + ii,
                        *apVal.offset(ii as isize),
                    );
                    ii += 1;
                }
                if sqlite3_step(pInsert) == SQLITE_ROW {
                    recoverSqlCallback(
                        p,
                        sqlite3_column_text(pInsert, 0 as ::core::ffi::c_int)
                            as *const ::core::ffi::c_char,
                    );
                }
                recoverReset(p, pInsert);
                ii = 0 as ::core::ffi::c_int;
                while ii < nVal {
                    sqlite3_value_free(*apVal.offset(ii as isize));
                    let ref mut c2rust_fresh1 = *apVal.offset(ii as isize);
                    *c2rust_fresh1 = ::core::ptr::null_mut::<sqlite3_value>();
                    ii += 1;
                }
                sqlite3_clear_bindings(pInsert);
                bHaveRowid = 0 as ::core::ffi::c_int;
                nVal = -1 as ::core::ffi::c_int;
            }
            if iCell < 0 as ::core::ffi::c_int {
                break;
            }
            if iField < 0 as ::core::ffi::c_int {
                iRowid = sqlite3_column_int64(pPageData, 2 as ::core::ffi::c_int)
                    as i64_0;
                bHaveRowid = 1 as ::core::ffi::c_int;
                nVal = 0 as ::core::ffi::c_int;
            } else if iField < (*pLaf).nMaxField {
                let mut pVal: *mut sqlite3_value = sqlite3_column_value(
                    pPageData,
                    2 as ::core::ffi::c_int,
                );
                let ref mut c2rust_fresh2 = *apVal.offset(iField as isize);
                *c2rust_fresh2 = sqlite3_value_dup(pVal);
                nVal = iField + 1 as ::core::ffi::c_int;
                if (*apVal.offset(iField as isize)).is_null() {
                    recoverError(
                        p,
                        SQLITE_NOMEM,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
            }
            iPrevCell = iCell;
        }
        recoverReset(p, pPageData);
        ii = 0 as ::core::ffi::c_int;
        while ii < nVal {
            sqlite3_value_free(*apVal.offset(ii as isize));
            let ref mut c2rust_fresh3 = *apVal.offset(ii as isize);
            *c2rust_fresh3 = ::core::ptr::null_mut::<sqlite3_value>();
            ii += 1;
        }
    }
}
unsafe extern "C" fn recoverLostAndFound3Step(
    mut p: *mut sqlite3_recover,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pLaf: *mut RecoverStateLAF = &raw mut (*p).laf;
        if (*p).errCode == SQLITE_OK {
            if (*pLaf).pInsert.is_null() {
                return SQLITE_DONE
            } else if (*p).errCode == SQLITE_OK {
                let mut res: ::core::ffi::c_int = sqlite3_step((*pLaf).pAllPage);
                if res == SQLITE_ROW {
                    let mut iPage: i64_0 = sqlite3_column_int64(
                        (*pLaf).pAllPage,
                        0 as ::core::ffi::c_int,
                    ) as i64_0;
                    if recoverBitmapQuery((*pLaf).pUsed, iPage)
                        == 0 as ::core::ffi::c_int
                    {
                        recoverLostAndFoundOnePage(p, iPage);
                    }
                } else {
                    recoverReset(p, (*pLaf).pAllPage);
                    return SQLITE_DONE;
                }
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn recoverLostAndFound3Init(mut p: *mut sqlite3_recover) {
    unsafe {
        let mut pLaf: *mut RecoverStateLAF = &raw mut (*p).laf;
        if (*pLaf).nMaxField > 0 as ::core::ffi::c_int {
            let mut zTab: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            zTab = recoverLostAndFoundCreate(p, (*pLaf).nMaxField);
            (*pLaf).pInsert = recoverLostAndFoundInsert(p, zTab, (*pLaf).nMaxField);
            sqlite3_free(zTab as *mut ::core::ffi::c_void);
            (*pLaf).pAllPage = recoverPreparePrintf(
                p,
                (*p).dbOut,
                b"WITH RECURSIVE seq(ii) AS (  SELECT 1 UNION ALL SELECT ii+1 FROM seq WHERE ii<%lld)SELECT ii FROM seq\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*p).laf.nPg,
            );
            (*pLaf).pPageData = recoverPrepare(
                p,
                (*p).dbOut,
                b"SELECT cell, field, value FROM sqlite_dbdata('getpage()') d WHERE d.pgno=? UNION ALL SELECT -1, -1, -1\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            (*pLaf).apVal = recoverMalloc(
                p,
                ((*pLaf).nMaxField as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut sqlite3_value>() as usize)
                    as i64_0,
            ) as *mut *mut sqlite3_value;
        }
    }
}
unsafe extern "C" fn recoverWriteDataInit(
    mut p: *mut sqlite3_recover,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p1: *mut RecoverStateW1 = &raw mut (*p).w1;
        let mut pTbl: *mut RecoverTable = ::core::ptr::null_mut::<RecoverTable>();
        let mut nByte: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        pTbl = (*p).pTblList;
        while !pTbl.is_null() {
            if (*pTbl).nCol > (*p1).nMax {
                (*p1).nMax = (*pTbl).nCol;
            }
            pTbl = (*pTbl).pNext;
        }
        nByte = (::core::mem::size_of::<*mut sqlite3_value>() as usize)
            .wrapping_mul(((*p1).nMax + 1 as ::core::ffi::c_int) as usize)
            as ::core::ffi::c_int;
        (*p1).apVal = recoverMalloc(p, nByte as i64_0) as *mut *mut sqlite3_value;
        if (*p1).apVal.is_null() {
            return (*p).errCode;
        }
        (*p1).pTbls = recoverPrepare(
            p,
            (*p).dbOut,
            b"SELECT rootpage FROM recovery.schema   WHERE type='table' AND (sql NOT LIKE 'create virtual%')  ORDER BY (tbl_name='sqlite_sequence') ASC\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        (*p1).pSel = recoverPrepare(
            p,
            (*p).dbOut,
            b"WITH RECURSIVE pages(page) AS (  SELECT ?1    UNION  SELECT child FROM sqlite_dbptr('getpage()'), pages     WHERE pgno=page) SELECT page, cell, field, value FROM sqlite_dbdata('getpage()') d, pages p WHERE p.page=d.pgno UNION ALL SELECT 0, 0, 0, 0\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return (*p).errCode;
    }
}
unsafe extern "C" fn recoverWriteDataCleanup(mut p: *mut sqlite3_recover) {
    unsafe {
        let mut p1: *mut RecoverStateW1 = &raw mut (*p).w1;
        let mut ii: ::core::ffi::c_int = 0;
        ii = 0 as ::core::ffi::c_int;
        while ii < (*p1).nVal {
            sqlite3_value_free(*(*p1).apVal.offset(ii as isize));
            ii += 1;
        }
        sqlite3_free((*p1).apVal as *mut ::core::ffi::c_void);
        recoverFinalize(p, (*p1).pInsert);
        recoverFinalize(p, (*p1).pTbls);
        recoverFinalize(p, (*p1).pSel);
        memset(
            p1 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<RecoverStateW1>() as size_t,
        );
    }
}
unsafe extern "C" fn recoverWriteDataStep(
    mut p: *mut sqlite3_recover,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p1: *mut RecoverStateW1 = &raw mut (*p).w1;
        let mut pSel: *mut sqlite3_stmt = (*p1).pSel;
        let mut apVal: *mut *mut sqlite3_value = (*p1).apVal;
        if (*p).errCode == SQLITE_OK && (*p1).pTab.is_null() {
            if sqlite3_step((*p1).pTbls) == SQLITE_ROW {
                let mut iRoot: i64_0 = sqlite3_column_int64(
                    (*p1).pTbls,
                    0 as ::core::ffi::c_int,
                ) as i64_0;
                (*p1).pTab = recoverFindTable(p, iRoot as u32_0);
                recoverFinalize(p, (*p1).pInsert);
                (*p1).pInsert = ::core::ptr::null_mut::<sqlite3_stmt>();
                if (*p1).pTab.is_null() {
                    return (*p).errCode;
                }
                if sqlite3_stricmp(
                    b"sqlite_sequence\0".as_ptr() as *const ::core::ffi::c_char,
                    (*(*p1).pTab).zTab,
                ) == 0 as ::core::ffi::c_int
                {
                    recoverExec(
                        p,
                        (*p).dbOut,
                        b"DELETE FROM sqlite_sequence\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    recoverSqlCallback(
                        p,
                        b"DELETE FROM sqlite_sequence\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                sqlite3_bind_int64(
                    pSel,
                    1 as ::core::ffi::c_int,
                    iRoot as sqlite3_int64,
                );
                (*p1).nVal = 0 as ::core::ffi::c_int;
                (*p1).bHaveRowid = 0 as ::core::ffi::c_int;
                (*p1).iPrevPage = -1 as ::core::ffi::c_int as i64_0;
                (*p1).iPrevCell = -1 as ::core::ffi::c_int;
            } else {
                return SQLITE_DONE
            }
        }
        if (*p).errCode == SQLITE_OK && sqlite3_step(pSel) == SQLITE_ROW {
            let mut pTab: *mut RecoverTable = (*p1).pTab;
            let mut iPage: i64_0 = sqlite3_column_int64(pSel, 0 as ::core::ffi::c_int)
                as i64_0;
            let mut iCell: ::core::ffi::c_int = sqlite3_column_int(
                pSel,
                1 as ::core::ffi::c_int,
            );
            let mut iField: ::core::ffi::c_int = sqlite3_column_int(
                pSel,
                2 as ::core::ffi::c_int,
            );
            let mut pVal: *mut sqlite3_value = sqlite3_column_value(
                pSel,
                3 as ::core::ffi::c_int,
            );
            let mut bNewCell: ::core::ffi::c_int = ((*p1).iPrevPage != iPage
                || (*p1).iPrevCell != iCell) as ::core::ffi::c_int;
            if bNewCell != 0 {
                let mut ii: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if (*p1).nVal >= 0 as ::core::ffi::c_int {
                    if (*p1).pInsert.is_null() || (*p1).nVal != (*p1).nInsert {
                        recoverFinalize(p, (*p1).pInsert);
                        (*p1).pInsert = recoverInsertStmt(p, pTab, (*p1).nVal);
                        (*p1).nInsert = (*p1).nVal;
                    }
                    if (*p1).nVal > 0 as ::core::ffi::c_int {
                        let mut pInsert: *mut sqlite3_stmt = (*p1).pInsert;
                        ii = 0 as ::core::ffi::c_int;
                        while ii < (*pTab).nCol {
                            let mut pCol: *mut RecoverColumn = (*pTab)
                                .aCol
                                .offset(ii as isize) as *mut RecoverColumn;
                            let mut iBind: ::core::ffi::c_int = (*pCol).iBind;
                            if iBind > 0 as ::core::ffi::c_int {
                                if (*pCol).bIPK != 0 {
                                    sqlite3_bind_int64(
                                        pInsert,
                                        iBind,
                                        (*p1).iRowid as sqlite3_int64,
                                    );
                                } else if (*pCol).iField < (*p1).nVal {
                                    recoverBindValue(
                                        p,
                                        pInsert,
                                        iBind,
                                        *apVal.offset((*pCol).iField as isize),
                                    );
                                }
                            }
                            ii += 1;
                        }
                        if (*p).bRecoverRowid != 0
                            && (*pTab).iRowidBind > 0 as ::core::ffi::c_int
                            && (*p1).bHaveRowid != 0
                        {
                            sqlite3_bind_int64(
                                pInsert,
                                (*pTab).iRowidBind,
                                (*p1).iRowid as sqlite3_int64,
                            );
                        }
                        if SQLITE_ROW == sqlite3_step(pInsert) {
                            let mut z: *const ::core::ffi::c_char = sqlite3_column_text(
                                pInsert,
                                0 as ::core::ffi::c_int,
                            ) as *const ::core::ffi::c_char;
                            recoverSqlCallback(p, z);
                        }
                        recoverReset(p, pInsert);
                        if !pInsert.is_null() {
                            sqlite3_clear_bindings(pInsert);
                        }
                    }
                }
                ii = 0 as ::core::ffi::c_int;
                while ii < (*p1).nVal {
                    sqlite3_value_free(*apVal.offset(ii as isize));
                    let ref mut c2rust_fresh5 = *apVal.offset(ii as isize);
                    *c2rust_fresh5 = ::core::ptr::null_mut::<sqlite3_value>();
                    ii += 1;
                }
                (*p1).nVal = -1 as ::core::ffi::c_int;
                (*p1).bHaveRowid = 0 as ::core::ffi::c_int;
            }
            if iPage != 0 as ::core::ffi::c_longlong {
                if iField < 0 as ::core::ffi::c_int {
                    (*p1).iRowid = sqlite3_column_int64(pSel, 3 as ::core::ffi::c_int)
                        as i64_0;
                    (*p1).nVal = 0 as ::core::ffi::c_int;
                    (*p1).bHaveRowid = 1 as ::core::ffi::c_int;
                } else if iField < (*pTab).nCol {
                    let ref mut c2rust_fresh6 = *apVal.offset(iField as isize);
                    *c2rust_fresh6 = sqlite3_value_dup(pVal);
                    if (*apVal.offset(iField as isize)).is_null() {
                        recoverError(
                            p,
                            SQLITE_NOMEM,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                    }
                    (*p1).nVal = iField + 1 as ::core::ffi::c_int;
                } else if (*pTab).nCol == 0 as ::core::ffi::c_int {
                    (*p1).nVal = (*pTab).nCol;
                }
                (*p1).iPrevCell = iCell;
                (*p1).iPrevPage = iPage;
            }
        } else {
            recoverReset(p, pSel);
            (*p1).pTab = ::core::ptr::null_mut::<RecoverTable>();
        }
        return (*p).errCode;
    }
}
unsafe extern "C" fn recoverLostAndFound1Init(mut p: *mut sqlite3_recover) {
    unsafe {
        let mut pLaf: *mut RecoverStateLAF = &raw mut (*p).laf;
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        (*pLaf).nPg = recoverPageCount(p);
        (*pLaf).pUsed = recoverBitmapAlloc(p, (*pLaf).nPg);
        pStmt = recoverPrepare(
            p,
            (*p).dbOut,
            b"WITH trunk(pgno) AS (  SELECT read_i32(getpage(1), 8) AS x WHERE x>0    UNION  SELECT read_i32(getpage(trunk.pgno), 0) AS x FROM trunk WHERE x>0),trunkdata(pgno, data) AS (  SELECT pgno, getpage(pgno) FROM trunk),freelist(data, n, freepgno) AS (  SELECT data, min(16384, read_i32(data, 1)-1), pgno FROM trunkdata    UNION ALL  SELECT data, n-1, read_i32(data, 2+n) FROM freelist WHERE n>=0),roots(r) AS (  SELECT 1 UNION ALL  SELECT rootpage FROM recovery.schema WHERE rootpage>0),used(page) AS (  SELECT r FROM roots    UNION  SELECT child FROM sqlite_dbptr('getpage()'), used     WHERE pgno=page) SELECT page FROM used UNION ALL SELECT freepgno FROM freelist WHERE NOT ?\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        if !pStmt.is_null() {
            sqlite3_bind_int(pStmt, 1 as ::core::ffi::c_int, (*p).bFreelistCorrupt);
        }
        (*pLaf).pUsedPages = pStmt;
    }
}
unsafe extern "C" fn recoverLostAndFound1Step(
    mut p: *mut sqlite3_recover,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pLaf: *mut RecoverStateLAF = &raw mut (*p).laf;
        let mut rc: ::core::ffi::c_int = (*p).errCode;
        if rc == SQLITE_OK {
            rc = sqlite3_step((*pLaf).pUsedPages);
            if rc == SQLITE_ROW {
                let mut iPg: i64_0 = sqlite3_column_int64(
                    (*pLaf).pUsedPages,
                    0 as ::core::ffi::c_int,
                ) as i64_0;
                recoverBitmapSet((*pLaf).pUsed, iPg);
                rc = SQLITE_OK;
            } else {
                recoverFinalize(p, (*pLaf).pUsedPages);
                (*pLaf).pUsedPages = ::core::ptr::null_mut::<sqlite3_stmt>();
            }
        }
        return rc;
    }
}
unsafe extern "C" fn recoverLostAndFound2Init(mut p: *mut sqlite3_recover) {
    unsafe {
        let mut pLaf: *mut RecoverStateLAF = &raw mut (*p).laf;
        (*pLaf).pMapInsert = recoverPrepare(
            p,
            (*p).dbOut,
            b"INSERT OR IGNORE INTO recovery.map(pgno, parent) VALUES(?, ?)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        (*pLaf).pAllAndParent = recoverPreparePrintf(
            p,
            (*p).dbOut,
            b"WITH RECURSIVE seq(ii) AS (  SELECT 1 UNION ALL SELECT ii+1 FROM seq WHERE ii<%lld)SELECT pgno, child FROM sqlite_dbptr('getpage()')  UNION ALL SELECT NULL, ii FROM seq\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*p).laf.nPg,
        );
        (*pLaf).pMaxField = recoverPreparePrintf(
            p,
            (*p).dbOut,
            b"SELECT max(field)+1 FROM sqlite_dbdata('getpage') WHERE pgno = ?\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn recoverLostAndFound2Step(
    mut p: *mut sqlite3_recover,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pLaf: *mut RecoverStateLAF = &raw mut (*p).laf;
        if (*p).errCode == SQLITE_OK {
            let mut res: ::core::ffi::c_int = sqlite3_step((*pLaf).pAllAndParent);
            if res == SQLITE_ROW {
                let mut iChild: i64_0 = sqlite3_column_int(
                    (*pLaf).pAllAndParent,
                    1 as ::core::ffi::c_int,
                ) as i64_0;
                if recoverBitmapQuery((*pLaf).pUsed, iChild) == 0 as ::core::ffi::c_int {
                    sqlite3_bind_int64(
                        (*pLaf).pMapInsert,
                        1 as ::core::ffi::c_int,
                        iChild as sqlite3_int64,
                    );
                    sqlite3_bind_value(
                        (*pLaf).pMapInsert,
                        2 as ::core::ffi::c_int,
                        sqlite3_column_value(
                            (*pLaf).pAllAndParent,
                            0 as ::core::ffi::c_int,
                        ),
                    );
                    sqlite3_step((*pLaf).pMapInsert);
                    recoverReset(p, (*pLaf).pMapInsert);
                    sqlite3_bind_int64(
                        (*pLaf).pMaxField,
                        1 as ::core::ffi::c_int,
                        iChild as sqlite3_int64,
                    );
                    if SQLITE_ROW == sqlite3_step((*pLaf).pMaxField) {
                        let mut nMax: ::core::ffi::c_int = sqlite3_column_int(
                            (*pLaf).pMaxField,
                            0 as ::core::ffi::c_int,
                        );
                        if nMax > (*pLaf).nMaxField {
                            (*pLaf).nMaxField = nMax;
                        }
                    }
                    recoverReset(p, (*pLaf).pMaxField);
                }
            } else {
                recoverFinalize(p, (*pLaf).pAllAndParent);
                (*pLaf).pAllAndParent = ::core::ptr::null_mut::<sqlite3_stmt>();
                return SQLITE_DONE;
            }
        }
        return (*p).errCode;
    }
}
unsafe extern "C" fn recoverLostAndFoundCleanup(mut p: *mut sqlite3_recover) {
    unsafe {
        recoverBitmapFree((*p).laf.pUsed);
        (*p).laf.pUsed = ::core::ptr::null_mut::<RecoverBitmap>();
        sqlite3_finalize((*p).laf.pUsedPages);
        sqlite3_finalize((*p).laf.pAllAndParent);
        sqlite3_finalize((*p).laf.pMapInsert);
        sqlite3_finalize((*p).laf.pMaxField);
        sqlite3_finalize((*p).laf.pFindRoot);
        sqlite3_finalize((*p).laf.pInsert);
        sqlite3_finalize((*p).laf.pAllPage);
        sqlite3_finalize((*p).laf.pPageData);
        (*p).laf.pUsedPages = ::core::ptr::null_mut::<sqlite3_stmt>();
        (*p).laf.pAllAndParent = ::core::ptr::null_mut::<sqlite3_stmt>();
        (*p).laf.pMapInsert = ::core::ptr::null_mut::<sqlite3_stmt>();
        (*p).laf.pMaxField = ::core::ptr::null_mut::<sqlite3_stmt>();
        (*p).laf.pFindRoot = ::core::ptr::null_mut::<sqlite3_stmt>();
        (*p).laf.pInsert = ::core::ptr::null_mut::<sqlite3_stmt>();
        (*p).laf.pAllPage = ::core::ptr::null_mut::<sqlite3_stmt>();
        (*p).laf.pPageData = ::core::ptr::null_mut::<sqlite3_stmt>();
        sqlite3_free((*p).laf.apVal as *mut ::core::ffi::c_void);
        (*p).laf.apVal = ::core::ptr::null_mut::<*mut sqlite3_value>();
    }
}
unsafe extern "C" fn recoverFinalCleanup(mut p: *mut sqlite3_recover) {
    unsafe {
        let mut pTab: *mut RecoverTable = ::core::ptr::null_mut::<RecoverTable>();
        let mut pNext: *mut RecoverTable = ::core::ptr::null_mut::<RecoverTable>();
        recoverWriteDataCleanup(p);
        recoverLostAndFoundCleanup(p);
        pTab = (*p).pTblList;
        while !pTab.is_null() {
            pNext = (*pTab).pNext;
            sqlite3_free(pTab as *mut ::core::ffi::c_void);
            pTab = pNext;
        }
        (*p).pTblList = ::core::ptr::null_mut::<RecoverTable>();
        sqlite3_finalize((*p).pGetPage);
        (*p).pGetPage = ::core::ptr::null_mut::<sqlite3_stmt>();
        sqlite3_file_control(
            (*p).dbIn,
            (*p).zDb,
            SQLITE_FCNTL_RESET_CACHE,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        sqlite3_close((*p).dbOut);
        (*p).dbOut = ::core::ptr::null_mut::<sqlite3>();
    }
}
unsafe extern "C" fn recoverGetU16(mut a: *const u8_0) -> u32_0 {
    unsafe {
        return ((*a.offset(0 as ::core::ffi::c_int as isize) as u32_0)
            << 8 as ::core::ffi::c_int)
            .wrapping_add(*a.offset(1 as ::core::ffi::c_int as isize) as u32_0);
    }
}
unsafe extern "C" fn recoverGetU32(mut a: *const u8_0) -> u32_0 {
    unsafe {
        return ((*a.offset(0 as ::core::ffi::c_int as isize) as u32_0)
            << 24 as ::core::ffi::c_int)
            .wrapping_add(
                (*a.offset(1 as ::core::ffi::c_int as isize) as u32_0)
                    << 16 as ::core::ffi::c_int,
            )
            .wrapping_add(
                (*a.offset(2 as ::core::ffi::c_int as isize) as u32_0)
                    << 8 as ::core::ffi::c_int,
            )
            .wrapping_add(*a.offset(3 as ::core::ffi::c_int as isize) as u32_0);
    }
}
unsafe extern "C" fn recoverGetVarint(
    mut a: *const u8_0,
    mut pVal: *mut i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut u: sqlite3_uint64 = 0 as sqlite3_uint64;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            u = ((u as ::core::ffi::c_ulonglong) << 7 as ::core::ffi::c_int)
                .wrapping_add(
                    (*a.offset(i as isize) as ::core::ffi::c_int
                        & 0x7f as ::core::ffi::c_int) as ::core::ffi::c_ulonglong,
                ) as sqlite3_uint64;
            if *a.offset(i as isize) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                *pVal = u as sqlite3_int64 as i64_0;
                return i + 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        u = ((u as ::core::ffi::c_ulonglong) << 8 as ::core::ffi::c_int)
            .wrapping_add(
                (*a.offset(i as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int) as ::core::ffi::c_ulonglong,
            ) as sqlite3_uint64;
        *pVal = u as sqlite3_int64 as i64_0;
        return 9 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn recoverIsValidPage(
    mut aTmp: *mut u8_0,
    mut a: *const u8_0,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aUsed: *mut u8_0 = aTmp;
        let mut nFrag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nActual: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iFree: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nCell: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iCellOff: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iContent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut eType: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut ii: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        eType = *a.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        if eType != 0x2 as ::core::ffi::c_int && eType != 0x5 as ::core::ffi::c_int
            && eType != 0xa as ::core::ffi::c_int && eType != 0xd as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        iFree = recoverGetU16(a.offset(1 as ::core::ffi::c_int as isize) as *const u8_0)
            as ::core::ffi::c_int;
        nCell = recoverGetU16(a.offset(3 as ::core::ffi::c_int as isize) as *const u8_0)
            as ::core::ffi::c_int;
        iContent = recoverGetU16(
            a.offset(5 as ::core::ffi::c_int as isize) as *const u8_0,
        ) as ::core::ffi::c_int;
        if iContent == 0 as ::core::ffi::c_int {
            iContent = 65536 as ::core::ffi::c_int;
        }
        nFrag = *a.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        if iContent > n {
            return 0 as ::core::ffi::c_int;
        }
        memset(aUsed as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, n as size_t);
        memset(
            aUsed as *mut ::core::ffi::c_void,
            0xff as ::core::ffi::c_int,
            iContent as size_t,
        );
        if iFree != 0 && iFree <= iContent {
            return 0 as ::core::ffi::c_int;
        }
        while iFree != 0 {
            let mut iNext: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut nByte: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if iFree > n - 4 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            iNext = recoverGetU16(a.offset(iFree as isize) as *const u8_0)
                as ::core::ffi::c_int;
            nByte = recoverGetU16(
                a.offset((iFree + 2 as ::core::ffi::c_int) as isize) as *const u8_0,
            ) as ::core::ffi::c_int;
            if iFree + nByte > n || nByte < 4 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            if iNext != 0 && iNext < iFree + nByte {
                return 0 as ::core::ffi::c_int;
            }
            memset(
                aUsed.offset(iFree as isize) as *mut u8_0 as *mut ::core::ffi::c_void,
                0xff as ::core::ffi::c_int,
                nByte as size_t,
            );
            iFree = iNext;
        }
        if eType == 0x2 as ::core::ffi::c_int || eType == 0x5 as ::core::ffi::c_int {
            iCellOff = 12 as ::core::ffi::c_int;
        } else {
            iCellOff = 8 as ::core::ffi::c_int;
        }
        if iCellOff + 2 as ::core::ffi::c_int * nCell > iContent {
            return 0 as ::core::ffi::c_int;
        }
        ii = 0 as ::core::ffi::c_int;
        while ii < nCell {
            let mut iByte: ::core::ffi::c_int = 0;
            let mut nPayload: i64_0 = 0 as i64_0;
            let mut nByte_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut iOff: ::core::ffi::c_int = recoverGetU16(
                a.offset((iCellOff + 2 as ::core::ffi::c_int * ii) as isize)
                    as *const u8_0,
            ) as ::core::ffi::c_int;
            if iOff < iContent || iOff > n {
                return 0 as ::core::ffi::c_int;
            }
            if eType == 0x5 as ::core::ffi::c_int || eType == 0x2 as ::core::ffi::c_int {
                nByte_0 += 4 as ::core::ffi::c_int;
            }
            nByte_0
                += recoverGetVarint(
                    a.offset((iOff + nByte_0) as isize) as *const u8_0,
                    &raw mut nPayload,
                );
            if eType == 0xd as ::core::ffi::c_int {
                let mut dummy: i64_0 = 0 as i64_0;
                nByte_0
                    += recoverGetVarint(
                        a.offset((iOff + nByte_0) as isize) as *const u8_0,
                        &raw mut dummy,
                    );
            }
            if eType != 0x5 as ::core::ffi::c_int {
                let mut X: ::core::ffi::c_int = if eType == 0xd as ::core::ffi::c_int {
                    n - 35 as ::core::ffi::c_int
                } else {
                    (n - 12 as ::core::ffi::c_int) * 64 as ::core::ffi::c_int
                        / 255 as ::core::ffi::c_int - 23 as ::core::ffi::c_int
                };
                let mut M: ::core::ffi::c_int = (n - 12 as ::core::ffi::c_int)
                    * 32 as ::core::ffi::c_int / 255 as ::core::ffi::c_int
                    - 23 as ::core::ffi::c_int;
                let mut K: ::core::ffi::c_int = (M as ::core::ffi::c_longlong
                    + (nPayload as ::core::ffi::c_longlong
                        - M as ::core::ffi::c_longlong)
                        % (n - 4 as ::core::ffi::c_int) as ::core::ffi::c_longlong)
                    as ::core::ffi::c_int;
                if nPayload < X as ::core::ffi::c_longlong {
                    nByte_0 = (nByte_0 as ::core::ffi::c_longlong
                        + nPayload as ::core::ffi::c_longlong) as ::core::ffi::c_int;
                } else if K <= X {
                    nByte_0 += K + 4 as ::core::ffi::c_int;
                } else {
                    nByte_0 += M + 4 as ::core::ffi::c_int;
                }
            }
            if iOff + nByte_0 > n {
                return 0 as ::core::ffi::c_int;
            }
            iByte = iOff;
            while iByte < iOff + nByte_0 {
                if *aUsed.offset(iByte as isize) as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                }
                *aUsed.offset(iByte as isize) = 0xff as u8_0;
                iByte += 1;
            }
            ii += 1;
        }
        nActual = 0 as ::core::ffi::c_int;
        ii = 0 as ::core::ffi::c_int;
        while ii < n {
            if *aUsed.offset(ii as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                nActual += 1;
            }
            ii += 1;
        }
        return (nActual == nFrag) as ::core::ffi::c_int;
    }
}
static mut recover_methods: sqlite3_io_methods = unsafe {
    sqlite3_io_methods {
        iVersion: 2 as ::core::ffi::c_int,
        xClose: Some(
            recoverVfsClose
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xRead: Some(
            recoverVfsRead
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    i64_0,
                ) -> ::core::ffi::c_int,
        ),
        xWrite: Some(
            recoverVfsWrite
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    i64_0,
                ) -> ::core::ffi::c_int,
        ),
        xTruncate: Some(
            recoverVfsTruncate
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
        xSync: Some(
            recoverVfsSync
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileSize: Some(
            recoverVfsFileSize
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
        xLock: Some(
            recoverVfsLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xUnlock: Some(
            recoverVfsUnlock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xCheckReservedLock: Some(
            recoverVfsCheckReservedLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFileControl: Some(
            recoverVfsFileControl
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xSectorSize: Some(
            recoverVfsSectorSize
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xDeviceCharacteristics: Some(
            recoverVfsDeviceCharacteristics
                as unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
        ),
        xShmMap: Some(
            recoverVfsShmMap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xShmLock: Some(
            recoverVfsShmLock
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xShmBarrier: Some(
            recoverVfsShmBarrier as unsafe extern "C" fn(*mut sqlite3_file) -> (),
        ),
        xShmUnmap: Some(
            recoverVfsShmUnmap
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xFetch: Some(
            recoverVfsFetch
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite3_int64,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        xUnfetch: Some(
            recoverVfsUnfetch
                as unsafe extern "C" fn(
                    *mut sqlite3_file,
                    sqlite3_int64,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
unsafe extern "C" fn recoverVfsClose(mut pFd: *mut sqlite3_file) -> ::core::ffi::c_int {
    unsafe {
        return (*(*pFd).pMethods).xClose.expect("non-null function pointer")(pFd);
    }
}
unsafe extern "C" fn recoverPutU16(mut a: *mut u8_0, mut v: u32_0) {
    unsafe {
        *a.offset(0 as ::core::ffi::c_int as isize) = (v as ::core::ffi::c_uint
            >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as u8_0;
        *a.offset(1 as ::core::ffi::c_int as isize) = (v as ::core::ffi::c_uint
            >> 0 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as u8_0;
    }
}
unsafe extern "C" fn recoverPutU32(mut a: *mut u8_0, mut v: u32_0) {
    unsafe {
        *a.offset(0 as ::core::ffi::c_int as isize) = (v as ::core::ffi::c_uint
            >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as u8_0;
        *a.offset(1 as ::core::ffi::c_int as isize) = (v as ::core::ffi::c_uint
            >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as u8_0;
        *a.offset(2 as ::core::ffi::c_int as isize) = (v as ::core::ffi::c_uint
            >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as u8_0;
        *a.offset(3 as ::core::ffi::c_int as isize) = (v as ::core::ffi::c_uint
            >> 0 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as u8_0;
    }
}
unsafe extern "C" fn recoverVfsDetectPagesize(
    mut p: *mut sqlite3_recover,
    mut pFd: *mut sqlite3_file,
    mut nReserve: u32_0,
    mut nSz: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let nMin: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
        let nMax: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
        let nMaxBlk: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
        let mut pgsz: u32_0 = 0 as u32_0;
        let mut iBlk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut aPg: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut aTmp: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        let mut nBlk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        aPg = sqlite3_malloc(2 as ::core::ffi::c_int * nMax) as *mut u8_0;
        if aPg.is_null() {
            return SQLITE_NOMEM;
        }
        aTmp = aPg.offset(nMax as isize) as *mut u8_0;
        nBlk = ((nSz as ::core::ffi::c_longlong + nMax as ::core::ffi::c_longlong
            - 1 as ::core::ffi::c_longlong) / nMax as ::core::ffi::c_longlong)
            as ::core::ffi::c_int;
        if nBlk > nMaxBlk {
            nBlk = nMaxBlk;
        }
        loop {
            iBlk = 0 as ::core::ffi::c_int;
            while rc == SQLITE_OK && iBlk < nBlk {
                let mut nByte: ::core::ffi::c_int = (if nSz
                    >= ((iBlk + 1 as ::core::ffi::c_int) * nMax)
                        as ::core::ffi::c_longlong
                {
                    nMax as ::core::ffi::c_longlong
                } else {
                    nSz as ::core::ffi::c_longlong % nMax as ::core::ffi::c_longlong
                }) as ::core::ffi::c_int;
                memset(
                    aPg as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    nMax as size_t,
                );
                rc = (*(*pFd).pMethods)
                    .xRead
                    .expect(
                        "non-null function pointer",
                    )(
                    pFd,
                    aPg as *mut ::core::ffi::c_void,
                    nByte,
                    (iBlk * nMax) as sqlite3_int64,
                );
                if rc == SQLITE_OK {
                    let mut pgsz2: ::core::ffi::c_int = 0;
                    pgsz2 = (if pgsz != 0 {
                        (pgsz as ::core::ffi::c_uint)
                            .wrapping_mul(2 as ::core::ffi::c_uint)
                    } else {
                        nMin as ::core::ffi::c_uint
                    }) as ::core::ffi::c_int;
                    while pgsz2 <= nMax {
                        let mut iOff: ::core::ffi::c_int = 0;
                        iOff = 0 as ::core::ffi::c_int;
                        while iOff < nMax {
                            if recoverIsValidPage(
                                aTmp,
                                aPg.offset(iOff as isize) as *mut u8_0,
                                (pgsz2 as u32_0).wrapping_sub(nReserve)
                                    as ::core::ffi::c_int,
                            ) != 0
                            {
                                pgsz = pgsz2 as u32_0;
                                break;
                            } else {
                                iOff += pgsz2;
                            }
                        }
                        pgsz2 = pgsz2 * 2 as ::core::ffi::c_int;
                    }
                }
                iBlk += 1;
            }
            if pgsz > (*p).detected_pgsz as u32_0 {
                (*p).detected_pgsz = pgsz as ::core::ffi::c_int;
                (*p).nReserve = nReserve as ::core::ffi::c_int;
            }
            if nReserve == 0 as ::core::ffi::c_uint {
                break;
            }
            nReserve = 0 as u32_0;
        }
        (*p).detected_pgsz = pgsz as ::core::ffi::c_int;
        sqlite3_free(aPg as *mut ::core::ffi::c_void);
        return rc;
    }
}
unsafe extern "C" fn recoverVfsRead(
    mut pFd: *mut sqlite3_file,
    mut aBuf: *mut ::core::ffi::c_void,
    mut nByte: ::core::ffi::c_int,
    mut iOff: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFd).pMethods == &raw mut recover_methods as *const sqlite3_io_methods {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            rc = (*(*pFd).pMethods)
                .xRead
                .expect(
                    "non-null function pointer",
                )(pFd, aBuf, nByte, iOff as sqlite3_int64);
            if nByte == 16 as ::core::ffi::c_int {
                sqlite3_randomness(16 as ::core::ffi::c_int, aBuf);
            } else if rc == SQLITE_OK && iOff == 0 as ::core::ffi::c_longlong
                && nByte >= 108 as ::core::ffi::c_int
            {
                let aPreserve: [::core::ffi::c_int; 6] = [
                    32 as ::core::ffi::c_int,
                    36 as ::core::ffi::c_int,
                    52 as ::core::ffi::c_int,
                    60 as ::core::ffi::c_int,
                    64 as ::core::ffi::c_int,
                    68 as ::core::ffi::c_int,
                ];
                let mut aHdr: [u8_0; 108] = [
                    0x53 as ::core::ffi::c_int as u8_0,
                    0x51 as ::core::ffi::c_int as u8_0,
                    0x4c as ::core::ffi::c_int as u8_0,
                    0x69 as ::core::ffi::c_int as u8_0,
                    0x74 as ::core::ffi::c_int as u8_0,
                    0x65 as ::core::ffi::c_int as u8_0,
                    0x20 as ::core::ffi::c_int as u8_0,
                    0x66 as ::core::ffi::c_int as u8_0,
                    0x6f as ::core::ffi::c_int as u8_0,
                    0x72 as ::core::ffi::c_int as u8_0,
                    0x6d as ::core::ffi::c_int as u8_0,
                    0x61 as ::core::ffi::c_int as u8_0,
                    0x74 as ::core::ffi::c_int as u8_0,
                    0x20 as ::core::ffi::c_int as u8_0,
                    0x33 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0x1 as ::core::ffi::c_int as u8_0,
                    0x1 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0x40 as ::core::ffi::c_int as u8_0,
                    0x20 as ::core::ffi::c_int as u8_0,
                    0x20 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0x4 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0x10 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0x2e as ::core::ffi::c_int as u8_0,
                    0x5b as ::core::ffi::c_int as u8_0,
                    0x30 as ::core::ffi::c_int as u8_0,
                    0xd as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0xff as ::core::ffi::c_int as u8_0,
                    0 as ::core::ffi::c_int as u8_0,
                ];
                let mut a: *mut u8_0 = aBuf as *mut u8_0;
                let mut pgsz: u32_0 = recoverGetU16(
                    a.offset(16 as ::core::ffi::c_int as isize) as *mut u8_0,
                );
                let mut nReserve: u32_0 = *a.offset(20 as ::core::ffi::c_int as isize)
                    as u32_0;
                let mut enc: u32_0 = recoverGetU32(
                    a.offset(56 as ::core::ffi::c_int as isize) as *mut u8_0,
                );
                let mut dbsz: u32_0 = 0 as u32_0;
                let mut dbFileSize: i64_0 = 0 as i64_0;
                let mut ii: ::core::ffi::c_int = 0;
                let mut p: *mut sqlite3_recover = recover_g.p;
                if pgsz == 0x1 as ::core::ffi::c_uint {
                    pgsz = 65536 as ::core::ffi::c_int as u32_0;
                }
                rc = (*(*pFd).pMethods)
                    .xFileSize
                    .expect("non-null function pointer")(pFd, &raw mut dbFileSize);
                if rc == SQLITE_OK && (*p).detected_pgsz == 0 as ::core::ffi::c_int {
                    rc = recoverVfsDetectPagesize(p, pFd, nReserve, dbFileSize);
                }
                if (*p).detected_pgsz != 0 {
                    pgsz = (*p).detected_pgsz as u32_0;
                    nReserve = (*p).nReserve as u32_0;
                }
                if pgsz != 0 {
                    dbsz = (dbFileSize as ::core::ffi::c_longlong
                        / pgsz as ::core::ffi::c_longlong) as u32_0;
                }
                if enc != SQLITE_UTF8 as ::core::ffi::c_uint
                    && enc != SQLITE_UTF16BE as ::core::ffi::c_uint
                    && enc != SQLITE_UTF16LE as ::core::ffi::c_uint
                {
                    enc = SQLITE_UTF8 as u32_0;
                }
                sqlite3_free((*p).pPage1Cache as *mut ::core::ffi::c_void);
                (*p).pPage1Cache = ::core::ptr::null_mut::<u8_0>();
                (*p).pPage1Disk = ::core::ptr::null_mut::<u8_0>();
                (*p).pgsz = nByte;
                (*p).pPage1Cache = recoverMalloc(
                    p,
                    (nByte * 2 as ::core::ffi::c_int) as i64_0,
                ) as *mut u8_0;
                if !(*p).pPage1Cache.is_null() {
                    (*p).pPage1Disk = (*p).pPage1Cache.offset(nByte as isize)
                        as *mut u8_0;
                    memcpy(
                        (*p).pPage1Disk as *mut ::core::ffi::c_void,
                        aBuf,
                        nByte as size_t,
                    );
                    aHdr[18 as ::core::ffi::c_int as usize] = *a
                        .offset(18 as ::core::ffi::c_int as isize);
                    aHdr[19 as ::core::ffi::c_int as usize] = *a
                        .offset(19 as ::core::ffi::c_int as isize);
                    recoverPutU32(
                        (&raw mut aHdr as *mut u8_0)
                            .offset(28 as ::core::ffi::c_int as isize) as *mut u8_0,
                        dbsz,
                    );
                    recoverPutU32(
                        (&raw mut aHdr as *mut u8_0)
                            .offset(56 as ::core::ffi::c_int as isize) as *mut u8_0,
                        enc,
                    );
                    recoverPutU16(
                        (&raw mut aHdr as *mut u8_0)
                            .offset(105 as ::core::ffi::c_int as isize) as *mut u8_0,
                        pgsz.wrapping_sub(nReserve),
                    );
                    if pgsz == 65536 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        pgsz = 1 as u32_0;
                    }
                    recoverPutU16(
                        (&raw mut aHdr as *mut u8_0)
                            .offset(16 as ::core::ffi::c_int as isize) as *mut u8_0,
                        pgsz,
                    );
                    aHdr[20 as ::core::ffi::c_int as usize] = nReserve as u8_0;
                    ii = 0 as ::core::ffi::c_int;
                    while ii
                        < (::core::mem::size_of::<[::core::ffi::c_int; 6]>() as usize)
                            .wrapping_div(
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            ) as ::core::ffi::c_int
                    {
                        memcpy(
                            (&raw mut aHdr as *mut u8_0)
                                .offset(
                                    *(&raw const aPreserve as *const ::core::ffi::c_int)
                                        .offset(ii as isize) as isize,
                                ) as *mut u8_0 as *mut ::core::ffi::c_void,
                            a
                                .offset(
                                    *(&raw const aPreserve as *const ::core::ffi::c_int)
                                        .offset(ii as isize) as isize,
                                ) as *mut u8_0 as *const ::core::ffi::c_void,
                            4 as size_t,
                        );
                        ii += 1;
                    }
                    memcpy(
                        aBuf,
                        &raw mut aHdr as *mut u8_0 as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<[u8_0; 108]>() as size_t,
                    );
                    memset(
                        (aBuf as *mut u8_0)
                            .offset(::core::mem::size_of::<[u8_0; 108]>() as isize)
                            as *mut u8_0 as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (nByte as size_t)
                            .wrapping_sub(
                                ::core::mem::size_of::<[u8_0; 108]>() as size_t,
                            ),
                    );
                    memcpy(
                        (*p).pPage1Cache as *mut ::core::ffi::c_void,
                        aBuf,
                        nByte as size_t,
                    );
                } else {
                    rc = (*p).errCode;
                }
            }
            (*pFd).pMethods = &raw mut recover_methods;
        } else {
            rc = (*(*pFd).pMethods)
                .xRead
                .expect(
                    "non-null function pointer",
                )(pFd, aBuf, nByte, iOff as sqlite3_int64);
        }
        return rc;
    }
}
unsafe extern "C" fn recoverVfsWrite(
    mut pFd: *mut sqlite3_file,
    mut aBuf: *const ::core::ffi::c_void,
    mut nByte: ::core::ffi::c_int,
    mut iOff: i64_0,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFd).pMethods == &raw mut recover_methods as *const sqlite3_io_methods {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            rc = (*(*pFd).pMethods)
                .xWrite
                .expect(
                    "non-null function pointer",
                )(pFd, aBuf, nByte, iOff as sqlite3_int64);
            (*pFd).pMethods = &raw mut recover_methods;
        } else {
            rc = (*(*pFd).pMethods)
                .xWrite
                .expect(
                    "non-null function pointer",
                )(pFd, aBuf, nByte, iOff as sqlite3_int64);
        }
        return rc;
    }
}
unsafe extern "C" fn recoverVfsTruncate(
    mut pFd: *mut sqlite3_file,
    mut size: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFd).pMethods == &raw mut recover_methods as *const sqlite3_io_methods {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            rc = (*(*pFd).pMethods)
                .xTruncate
                .expect("non-null function pointer")(pFd, size);
            (*pFd).pMethods = &raw mut recover_methods;
        } else {
            rc = (*(*pFd).pMethods)
                .xTruncate
                .expect("non-null function pointer")(pFd, size);
        }
        return rc;
    }
}
unsafe extern "C" fn recoverVfsSync(
    mut pFd: *mut sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFd).pMethods == &raw mut recover_methods as *const sqlite3_io_methods {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            rc = (*(*pFd).pMethods)
                .xSync
                .expect("non-null function pointer")(pFd, flags);
            (*pFd).pMethods = &raw mut recover_methods;
        } else {
            rc = (*(*pFd).pMethods)
                .xSync
                .expect("non-null function pointer")(pFd, flags);
        }
        return rc;
    }
}
unsafe extern "C" fn recoverVfsFileSize(
    mut pFd: *mut sqlite3_file,
    mut pSize: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFd).pMethods == &raw mut recover_methods as *const sqlite3_io_methods {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            rc = (*(*pFd).pMethods)
                .xFileSize
                .expect("non-null function pointer")(pFd, pSize);
            (*pFd).pMethods = &raw mut recover_methods;
        } else {
            rc = (*(*pFd).pMethods)
                .xFileSize
                .expect("non-null function pointer")(pFd, pSize);
        }
        return rc;
    }
}
unsafe extern "C" fn recoverVfsLock(
    mut pFd: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFd).pMethods == &raw mut recover_methods as *const sqlite3_io_methods {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            rc = (*(*pFd).pMethods)
                .xLock
                .expect("non-null function pointer")(pFd, eLock);
            (*pFd).pMethods = &raw mut recover_methods;
        } else {
            rc = (*(*pFd).pMethods)
                .xLock
                .expect("non-null function pointer")(pFd, eLock);
        }
        return rc;
    }
}
unsafe extern "C" fn recoverVfsUnlock(
    mut pFd: *mut sqlite3_file,
    mut eLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFd).pMethods == &raw mut recover_methods as *const sqlite3_io_methods {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            rc = (*(*pFd).pMethods)
                .xUnlock
                .expect("non-null function pointer")(pFd, eLock);
            (*pFd).pMethods = &raw mut recover_methods;
        } else {
            rc = (*(*pFd).pMethods)
                .xUnlock
                .expect("non-null function pointer")(pFd, eLock);
        }
        return rc;
    }
}
unsafe extern "C" fn recoverVfsCheckReservedLock(
    mut pFd: *mut sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFd).pMethods == &raw mut recover_methods as *const sqlite3_io_methods {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            rc = (*(*pFd).pMethods)
                .xCheckReservedLock
                .expect("non-null function pointer")(pFd, pResOut);
            (*pFd).pMethods = &raw mut recover_methods;
        } else {
            rc = (*(*pFd).pMethods)
                .xCheckReservedLock
                .expect("non-null function pointer")(pFd, pResOut);
        }
        return rc;
    }
}
unsafe extern "C" fn recoverVfsFileControl(
    mut pFd: *mut sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFd).pMethods == &raw mut recover_methods as *const sqlite3_io_methods {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            rc = if !(*pFd).pMethods.is_null() {
                (*(*pFd).pMethods)
                    .xFileControl
                    .expect("non-null function pointer")(pFd, op, pArg)
            } else {
                12 as ::core::ffi::c_int
            };
            (*pFd).pMethods = &raw mut recover_methods;
        } else {
            rc = if !(*pFd).pMethods.is_null() {
                (*(*pFd).pMethods)
                    .xFileControl
                    .expect("non-null function pointer")(pFd, op, pArg)
            } else {
                12 as ::core::ffi::c_int
            };
        }
        return rc;
    }
}
unsafe extern "C" fn recoverVfsSectorSize(
    mut pFd: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFd).pMethods == &raw mut recover_methods as *const sqlite3_io_methods {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            rc = (*(*pFd).pMethods).xSectorSize.expect("non-null function pointer")(pFd);
            (*pFd).pMethods = &raw mut recover_methods;
        } else {
            rc = (*(*pFd).pMethods).xSectorSize.expect("non-null function pointer")(pFd);
        }
        return rc;
    }
}
unsafe extern "C" fn recoverVfsDeviceCharacteristics(
    mut pFd: *mut sqlite3_file,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFd).pMethods == &raw mut recover_methods as *const sqlite3_io_methods {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            rc = (*(*pFd).pMethods)
                .xDeviceCharacteristics
                .expect("non-null function pointer")(pFd);
            (*pFd).pMethods = &raw mut recover_methods;
        } else {
            rc = (*(*pFd).pMethods)
                .xDeviceCharacteristics
                .expect("non-null function pointer")(pFd);
        }
        return rc;
    }
}
unsafe extern "C" fn recoverVfsShmMap(
    mut pFd: *mut sqlite3_file,
    mut iPg: ::core::ffi::c_int,
    mut pgsz: ::core::ffi::c_int,
    mut bExtend: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFd).pMethods == &raw mut recover_methods as *const sqlite3_io_methods {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            rc = (*(*pFd).pMethods)
                .xShmMap
                .expect("non-null function pointer")(pFd, iPg, pgsz, bExtend, pp);
            (*pFd).pMethods = &raw mut recover_methods;
        } else {
            rc = (*(*pFd).pMethods)
                .xShmMap
                .expect("non-null function pointer")(pFd, iPg, pgsz, bExtend, pp);
        }
        return rc;
    }
}
unsafe extern "C" fn recoverVfsShmLock(
    mut pFd: *mut sqlite3_file,
    mut offset: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFd).pMethods == &raw mut recover_methods as *const sqlite3_io_methods {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            rc = (*(*pFd).pMethods)
                .xShmLock
                .expect("non-null function pointer")(pFd, offset, n, flags);
            (*pFd).pMethods = &raw mut recover_methods;
        } else {
            rc = (*(*pFd).pMethods)
                .xShmLock
                .expect("non-null function pointer")(pFd, offset, n, flags);
        }
        return rc;
    }
}
unsafe extern "C" fn recoverVfsShmBarrier(mut pFd: *mut sqlite3_file) {
    unsafe {
        if (*pFd).pMethods == &raw mut recover_methods as *const sqlite3_io_methods {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            (*(*pFd).pMethods).xShmBarrier.expect("non-null function pointer")(pFd);
            (*pFd).pMethods = &raw mut recover_methods;
        } else {
            (*(*pFd).pMethods).xShmBarrier.expect("non-null function pointer")(pFd);
        };
    }
}
unsafe extern "C" fn recoverVfsShmUnmap(
    mut pFd: *mut sqlite3_file,
    mut deleteFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if (*pFd).pMethods == &raw mut recover_methods as *const sqlite3_io_methods {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            rc = (*(*pFd).pMethods)
                .xShmUnmap
                .expect("non-null function pointer")(pFd, deleteFlag);
            (*pFd).pMethods = &raw mut recover_methods;
        } else {
            rc = (*(*pFd).pMethods)
                .xShmUnmap
                .expect("non-null function pointer")(pFd, deleteFlag);
        }
        return rc;
    }
}
unsafe extern "C" fn recoverVfsFetch(
    mut pFd: *mut sqlite3_file,
    mut iOff: sqlite3_int64,
    mut iAmt: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        *pp = ::core::ptr::null_mut::<::core::ffi::c_void>();
        return SQLITE_OK;
    }
}
unsafe extern "C" fn recoverVfsUnfetch(
    mut pFd: *mut sqlite3_file,
    mut iOff: sqlite3_int64,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        return SQLITE_OK;
    }
}
unsafe extern "C" fn recoverInstallWrapper(mut p: *mut sqlite3_recover) {
    unsafe {
        let mut pFd: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        sqlite3_file_control(
            (*p).dbIn,
            (*p).zDb,
            SQLITE_FCNTL_FILE_POINTER,
            &raw mut pFd as *mut ::core::ffi::c_void,
        );
        if !pFd.is_null() && !(*pFd).pMethods.is_null() {
            let mut iVersion: ::core::ffi::c_int = 1 as ::core::ffi::c_int
                + ((*(*pFd).pMethods).iVersion > 1 as ::core::ffi::c_int
                    && (*(*pFd).pMethods).xShmMap.is_some()) as ::core::ffi::c_int;
            recover_g.pMethods = (*pFd).pMethods as *const sqlite3_io_methods;
            recover_g.p = p;
            recover_methods.iVersion = iVersion;
            (*pFd).pMethods = &raw mut recover_methods;
        }
    }
}
unsafe extern "C" fn recoverUninstallWrapper(mut p: *mut sqlite3_recover) {
    unsafe {
        let mut pFd: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
        sqlite3_file_control(
            (*p).dbIn,
            (*p).zDb,
            SQLITE_FCNTL_FILE_POINTER,
            &raw mut pFd as *mut ::core::ffi::c_void,
        );
        if !pFd.is_null() && !(*pFd).pMethods.is_null() {
            (*pFd).pMethods = recover_g.pMethods as *const sqlite3_io_methods;
            recover_g.pMethods = ::core::ptr::null::<sqlite3_io_methods>();
            recover_g.p = ::core::ptr::null_mut::<sqlite3_recover>();
        }
    }
}
unsafe extern "C" fn recoverStep(mut p: *mut sqlite3_recover) {
    unsafe {
        match (*p).eState {
            RECOVER_STATE_INIT => {
                let mut bUseWrapper: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                recoverSqlCallback(p, b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char);
                recoverSqlCallback(
                    p,
                    b"PRAGMA writable_schema = on\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                recoverSqlCallback(
                    p,
                    b"PRAGMA foreign_keys = off\0".as_ptr() as *const ::core::ffi::c_char,
                );
                recoverEnterMutex();
                recoverOpenOutput(p);
                if (*p).errCode == SQLITE_OK {
                    loop {
                        (*p).errCode = SQLITE_OK;
                        if bUseWrapper != 0 {
                            recoverInstallWrapper(p);
                        }
                        sqlite3_file_control(
                            (*p).dbIn,
                            (*p).zDb,
                            SQLITE_FCNTL_RESET_CACHE,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        );
                        recoverExec(
                            p,
                            (*p).dbIn,
                            b"PRAGMA writable_schema = on\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        recoverExec(
                            p,
                            (*p).dbIn,
                            b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if (*p).errCode == SQLITE_OK {
                            (*p).bCloseTransaction = 1 as ::core::ffi::c_int;
                        }
                        recoverExec(
                            p,
                            (*p).dbIn,
                            b"SELECT 1 FROM sqlite_schema\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        recoverTransferSettings(p);
                        recoverOpenRecovery(p);
                        recoverCacheSchema(p);
                        if bUseWrapper != 0 {
                            recoverUninstallWrapper(p);
                        }
                        if !((*p).errCode == SQLITE_NOTADB
                            && {
                                let c2rust_fresh0 = bUseWrapper;
                                bUseWrapper = bUseWrapper - 1;
                                c2rust_fresh0 != 0
                            }
                            && SQLITE_OK
                                == sqlite3_exec(
                                    (*p).dbIn,
                                    b"ROLLBACK\0".as_ptr() as *const ::core::ffi::c_char,
                                    None,
                                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                ))
                        {
                            break;
                        }
                    }
                }
                recoverLeaveMutex();
                recoverExec(
                    p,
                    (*p).dbOut,
                    b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char,
                );
                recoverWriteSchema1(p);
                (*p).eState = RECOVER_STATE_WRITING;
            }
            RECOVER_STATE_WRITING => {
                if (*p).w1.pTbls.is_null() {
                    recoverWriteDataInit(p);
                }
                if SQLITE_DONE == recoverWriteDataStep(p) {
                    recoverWriteDataCleanup(p);
                    if !(*p).zLostAndFound.is_null() {
                        (*p).eState = RECOVER_STATE_LOSTANDFOUND1;
                    } else {
                        (*p).eState = RECOVER_STATE_SCHEMA2;
                    }
                }
            }
            RECOVER_STATE_LOSTANDFOUND1 => {
                if (*p).laf.pUsed.is_null() {
                    recoverLostAndFound1Init(p);
                }
                if SQLITE_DONE == recoverLostAndFound1Step(p) {
                    (*p).eState = RECOVER_STATE_LOSTANDFOUND2;
                }
            }
            RECOVER_STATE_LOSTANDFOUND2 => {
                if (*p).laf.pAllAndParent.is_null() {
                    recoverLostAndFound2Init(p);
                }
                if SQLITE_DONE == recoverLostAndFound2Step(p) {
                    (*p).eState = RECOVER_STATE_LOSTANDFOUND3;
                }
            }
            RECOVER_STATE_LOSTANDFOUND3 => {
                if (*p).laf.pInsert.is_null() {
                    recoverLostAndFound3Init(p);
                }
                if SQLITE_DONE == recoverLostAndFound3Step(p) {
                    (*p).eState = RECOVER_STATE_SCHEMA2;
                }
            }
            RECOVER_STATE_SCHEMA2 => {
                let mut rc: ::core::ffi::c_int = SQLITE_OK;
                recoverWriteSchema2(p);
                (*p).eState = RECOVER_STATE_DONE;
                recoverExec(
                    p,
                    (*p).dbOut,
                    b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char,
                );
                rc = sqlite3_exec(
                    (*p).dbIn,
                    b"END\0".as_ptr() as *const ::core::ffi::c_char,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
                if (*p).errCode == SQLITE_OK {
                    (*p).errCode = rc;
                }
                recoverSqlCallback(
                    p,
                    b"PRAGMA writable_schema = off\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                recoverSqlCallback(
                    p,
                    b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char,
                );
                (*p).eState = RECOVER_STATE_DONE;
                recoverFinalCleanup(p);
            }
            RECOVER_STATE_DONE | _ => {}
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recoverInit(
    mut db: *mut sqlite3,
    mut zDb: *const ::core::ffi::c_char,
    mut zUri: *const ::core::ffi::c_char,
    mut xSql: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    mut pSqlCtx: *mut ::core::ffi::c_void,
) -> *mut sqlite3_recover {
    unsafe {
        let mut pRet: *mut sqlite3_recover = ::core::ptr::null_mut::<sqlite3_recover>();
        let mut nDb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nUri: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nByte: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if zDb.is_null() {
            zDb = b"main\0".as_ptr() as *const ::core::ffi::c_char;
        }
        nDb = recoverStrlen(zDb);
        nUri = recoverStrlen(zUri);
        nByte = (::core::mem::size_of::<sqlite3_recover>() as usize)
            .wrapping_add(nDb as usize)
            .wrapping_add(1 as usize)
            .wrapping_add(nUri as usize)
            .wrapping_add(1 as usize) as ::core::ffi::c_int;
        pRet = sqlite3_malloc(nByte) as *mut sqlite3_recover;
        if !pRet.is_null() {
            memset(
                pRet as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                nByte as size_t,
            );
            (*pRet).dbIn = db;
            (*pRet).zDb = pRet.offset(1 as ::core::ffi::c_int as isize)
                as *mut sqlite3_recover as *mut ::core::ffi::c_char;
            (*pRet).zUri = (*pRet).zDb.offset((nDb + 1 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_char;
            memcpy(
                (*pRet).zDb as *mut ::core::ffi::c_void,
                zDb as *const ::core::ffi::c_void,
                nDb as size_t,
            );
            if nUri > 0 as ::core::ffi::c_int && !zUri.is_null() {
                memcpy(
                    (*pRet).zUri as *mut ::core::ffi::c_void,
                    zUri as *const ::core::ffi::c_void,
                    nUri as size_t,
                );
            }
            (*pRet).xSql = xSql;
            (*pRet).pSqlCtx = pSqlCtx;
            (*pRet).bRecoverRowid = RECOVER_ROWID_DEFAULT;
        }
        return pRet;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_recover_init(
    mut db: *mut sqlite3,
    mut zDb: *const ::core::ffi::c_char,
    mut zUri: *const ::core::ffi::c_char,
) -> *mut sqlite3_recover {
    unsafe {
        return recoverInit(
            db,
            zDb,
            zUri,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_recover_init_sql(
    mut db: *mut sqlite3,
    mut zDb: *const ::core::ffi::c_char,
    mut xSql: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    mut pSqlCtx: *mut ::core::ffi::c_void,
) -> *mut sqlite3_recover {
    unsafe {
        return recoverInit(
            db,
            zDb,
            ::core::ptr::null::<::core::ffi::c_char>(),
            xSql,
            pSqlCtx,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_recover_errmsg(
    mut p: *mut sqlite3_recover,
) -> *const ::core::ffi::c_char {
    unsafe {
        return if !p.is_null() && (*p).errCode != SQLITE_NOMEM {
            (*p).zErrMsg as *const ::core::ffi::c_char
        } else {
            b"out of memory\0".as_ptr() as *const ::core::ffi::c_char
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_recover_errcode(
    mut p: *mut sqlite3_recover,
) -> ::core::ffi::c_int {
    unsafe {
        return if !p.is_null() { (*p).errCode } else { SQLITE_NOMEM };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_recover_config(
    mut p: *mut sqlite3_recover,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if p.is_null() {
            rc = SQLITE_NOMEM;
        } else if (*p).eState != RECOVER_STATE_INIT {
            rc = SQLITE_MISUSE;
        } else {
            match op {
                789 => {
                    sqlite3_free((*p).zStateDb as *mut ::core::ffi::c_void);
                    (*p).zStateDb = recoverMPrintf(
                        p,
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        pArg as *mut ::core::ffi::c_char,
                    );
                }
                SQLITE_RECOVER_LOST_AND_FOUND => {
                    let mut zArg: *const ::core::ffi::c_char = pArg
                        as *const ::core::ffi::c_char;
                    sqlite3_free((*p).zLostAndFound as *mut ::core::ffi::c_void);
                    if !zArg.is_null() {
                        (*p).zLostAndFound = recoverMPrintf(
                            p,
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            zArg,
                        );
                    } else {
                        (*p).zLostAndFound = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                    }
                }
                SQLITE_RECOVER_FREELIST_CORRUPT => {
                    (*p).bFreelistCorrupt = *(pArg as *mut ::core::ffi::c_int);
                }
                SQLITE_RECOVER_ROWIDS => {
                    (*p).bRecoverRowid = *(pArg as *mut ::core::ffi::c_int);
                }
                SQLITE_RECOVER_SLOWINDEXES => {
                    (*p).bSlowIndexes = *(pArg as *mut ::core::ffi::c_int);
                }
                _ => {
                    rc = SQLITE_NOTFOUND;
                }
            }
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_recover_step(
    mut p: *mut sqlite3_recover,
) -> ::core::ffi::c_int {
    unsafe {
        if p.is_null() {
            return SQLITE_NOMEM;
        }
        if (*p).errCode == SQLITE_OK {
            recoverStep(p);
        }
        if (*p).eState == RECOVER_STATE_DONE && (*p).errCode == SQLITE_OK {
            return SQLITE_DONE;
        }
        return (*p).errCode;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_recover_run(
    mut p: *mut sqlite3_recover,
) -> ::core::ffi::c_int {
    unsafe {
        while SQLITE_OK == sqlite3_recover_step(p) {}
        return sqlite3_recover_errcode(p);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_recover_finish(
    mut p: *mut sqlite3_recover,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        if p.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            recoverFinalCleanup(p);
            if (*p).bCloseTransaction != 0
                && sqlite3_get_autocommit((*p).dbIn) == 0 as ::core::ffi::c_int
            {
                rc = sqlite3_exec(
                    (*p).dbIn,
                    b"END\0".as_ptr() as *const ::core::ffi::c_char,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
                if (*p).errCode == SQLITE_OK {
                    (*p).errCode = rc;
                }
            }
            rc = (*p).errCode;
            sqlite3_free((*p).zErrMsg as *mut ::core::ffi::c_void);
            sqlite3_free((*p).zStateDb as *mut ::core::ffi::c_void);
            sqlite3_free((*p).zLostAndFound as *mut ::core::ffi::c_void);
            sqlite3_free((*p).pPage1Cache as *mut ::core::ffi::c_void);
            sqlite3_free(p as *mut ::core::ffi::c_void);
        }
        return rc;
    }
}
