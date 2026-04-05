// C variadic functions module.
// This module documents the organization of C variadic functions.
// Only the minimum required functions that absolutely need c_variadic feature are here.
// All variadic function definitions are compiled with the c_variadic feature enabled.

// Re-export printf module for convenient access
pub use crate::src::src::printf;

// Import required types and functions from their original modules
use crate::src::src::main::{C2RustUnnamed, LOGFUNC_t, void_function, sqlite3MisuseError, setupLookaside};
use crate::src::fts5::{Fts5Config, Fts5FullTable};
use crate::src::src::btree::{checkOom, checkProgress};


// Variadic function implementations - these require c_variadic feature
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3MPrintf(
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut zFormat: *const ::core::ffi::c_char,
    mut args: ...
) -> *mut ::core::ffi::c_char {
    let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    z = crate::src::src::printf::sqlite3VMPrintf(db, zFormat, args);
    z
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_log(
    mut iErrCode: ::core::ffi::c_int,
    mut zFormat: *const ::core::ffi::c_char,
    mut args: ...
) {
    // VaListImpl type handling - using args directly
    if crate::src::src::global::sqlite3Config.xLog.is_some() {
        renderLogMsg(iErrCode, zFormat, args);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3DebugPrintf(
    mut zFormat: *const ::core::ffi::c_char,
    mut args: ...
) {
    // VaListImpl type handling - using args directly
    let mut acc: crate::src::headers::sqliteInt_h::StrAccum = unsafe { ::core::mem::zeroed() };
    let mut zBuf: [::core::ffi::c_char; 700] = [0; 700];
    crate::src::src::printf::sqlite3StrAccumInit(
        &raw mut acc,
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>(),
        &raw mut zBuf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 700]>() as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    let (_s, a) = crate::src::src::printf::extract_printf_args(zFormat, args, false, ::core::ptr::null_mut());
    crate::src::src::printf::sqlite3_str_vappendf_args(&raw mut acc, zFormat, &a);
    crate::src::src::printf::sqlite3StrAccumFinish(&raw mut acc);
    crate::src::headers::stdlib::fprintf(
        crate::src::headers::stdlib::stdout,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut zBuf as *mut ::core::ffi::c_char,
    );
    crate::src::headers::stdlib::fflush(crate::src::headers::stdlib::stdout);
}

// sqlite3_str_appendf is now implemented in c_code/printf_c.c
// It calls sqlite3_str_vappendf_sqlfunc and sqlite3_str_vappendf_va from printf.rs

// VaList functions below are defined in printf.rs - we re-export them here
// (they don't use VaListImpl, just VaList which is already available)
pub use crate::src::src::printf::{sqlite3VMPrintf, sqlite3_vmprintf, renderLogMsg};

// Variadic functions that use VaListImpl - must be defined here to use c_variadic feature
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3VdbeError(
    mut p: *mut crate::src::headers::vdbeInt_h::Vdbe,
    mut zFormat: *const ::core::ffi::c_char,
    mut args: ...
) {
    let __p_ref = unsafe { &mut *p };
    crate::src::src::malloc::sqlite3DbFree(__p_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3, __p_ref.zErrMsg as *mut ::core::ffi::c_void);
    __p_ref.zErrMsg = crate::src::src::printf::sqlite3VMPrintf(__p_ref.db, zFormat, args);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3VdbeMultiLoad(
    mut p: *mut crate::src::headers::vdbeInt_h::Vdbe,
    mut iDest: ::core::ffi::c_int,
    mut zTypes: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut current_block: u64;
    // VaListImpl type handling - using args directly
    let mut i: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_char = 0;
    i = 0 as ::core::ffi::c_int;
    loop {
        c = *zTypes.offset(i as isize);
        if !(c as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            current_block = 11812396948646013369;
            break;
        }
        if c as ::core::ffi::c_int == 's' as i32 {
            let mut z: *const ::core::ffi::c_char = args.arg::<*const ::core::ffi::c_char>();
            crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                p,
                if z.is_null() { crate::src::headers::opcodes_h::OP_Null } else { crate::src::headers::opcodes_h::OP_String8 },
                0 as ::core::ffi::c_int,
                iDest + i,
                0 as ::core::ffi::c_int,
                z,
                0 as ::core::ffi::c_int,
            );
        } else {
            if !(c as ::core::ffi::c_int == 'i' as i32) {
                current_block = 2968425633554183086;
                break;
            }
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(p, crate::src::headers::opcodes_h::OP_Integer, args.arg::<::core::ffi::c_int>(), iDest + i);
        }
        i += 1;
    }
    match current_block {
        11812396948646013369 => {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(p, crate::src::headers::opcodes_h::OP_ResultRow, iDest, i);
        }
        _ => {}
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3VdbeExplain(
    mut pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut bPush: crate::src::ext::rtree::rtree::u8_0,
    mut zFmt: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut addr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*pParse).explain as ::core::ffi::c_int == 2 as ::core::ffi::c_int
        || 0 as ::core::ffi::c_int != 0
    {
        let mut zMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut v: *mut crate::src::headers::vdbeInt_h::Vdbe = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::Vdbe>();
        let mut iThis: ::core::ffi::c_int = 0;
        let __pParse_ref = unsafe { &mut *pParse };
                zMsg = crate::src::src::printf::sqlite3VMPrintf(__pParse_ref.db, zFmt, args);
        v = __pParse_ref.pVdbe;
        iThis = (*v).nOp;
        addr = crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
            v,
            crate::src::headers::opcodes_h::OP_Explain,
            iThis,
            __pParse_ref.addrExplain,
            0 as ::core::ffi::c_int,
            zMsg,
            crate::src::src::vdbe::P4_DYNAMIC,
        );
        if bPush != 0 {
            __pParse_ref.addrExplain = iThis;
        }
    }
    addr
}

// Variadic function from vtab.rs - must be defined here to use c_variadic feature
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_vtab_config(
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut op: ::core::ffi::c_int,
    mut args: ...
) -> ::core::ffi::c_int {
    // VaListImpl type handling - using args directly
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut p: *mut crate::src::src::vtab::VtabCtx = ::core::ptr::null_mut::<crate::src::src::vtab::VtabCtx>();
    let __db_ref = unsafe { &*db };
    crate::src::src::mutex::sqlite3_mutex_enter(__db_ref.mutex);
    p = __db_ref.pVtabCtx;
    if p.is_null() {
        rc = crate::src::src::main::sqlite3MisuseError(1346 as ::core::ffi::c_int);
    } else {
        match  op {
    crate::src::headers::sqlite3_h::SQLITE_VTAB_CONSTRAINT_SUPPORT_1 =>  {
                (*(*p).pVTable).bConstraint = args.arg::<::core::ffi::c_int>() as crate::src::ext::rtree::rtree::u8_0;
            }
    crate::src::headers::sqlite3_h::SQLITE_VTAB_INNOCUOUS_1 =>  {
                (*(*p).pVTable).eVtabRisk = crate::src::headers::sqliteInt_h::SQLITE_VTABRISK_Low as crate::src::ext::rtree::rtree::u8_0;
            }
    crate::src::headers::sqlite3_h::SQLITE_VTAB_DIRECTONLY_1 =>  {
                (*(*p).pVTable).eVtabRisk = crate::src::headers::sqliteInt_h::SQLITE_VTABRISK_High as crate::src::ext::rtree::rtree::u8_0;
            }
    crate::src::headers::sqlite3_h::SQLITE_VTAB_USES_ALL_SCHEMAS_1 =>  {
                (*(*p).pVTable).bAllSchemas = 1 as crate::src::ext::rtree::rtree::u8_0;
            }
    _ =>  {
                rc = crate::src::src::main::sqlite3MisuseError(1368 as ::core::ffi::c_int);
            }
}
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::util::sqlite3Error(db as *mut crate::src::headers::sqliteInt_h::sqlite3, rc);
    }
    crate::src::src::mutex::sqlite3_mutex_leave(__db_ref.mutex);
    rc
}

// Variadic functions from util.rs and vacuum.rs - must be defined here to use c_variadic feature
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3ErrorWithMsg(
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut err_code: ::core::ffi::c_int,
    mut zFormat: *const ::core::ffi::c_char,
    mut args: ...
) {
    (*db).errCode = err_code;
    crate::src::src::util::sqlite3SystemError(db, err_code);
    if zFormat.is_null() {
        crate::src::src::util::sqlite3Error(db, err_code);
    } else if !(*db).pErr.is_null() || {
        (*db).pErr = crate::src::src::vdbemem::sqlite3ValueNew(db as *mut crate::src::headers::sqliteInt_h::sqlite3);
        !(*db).pErr.is_null()
    } {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
                z = crate::src::src::printf::sqlite3VMPrintf(db, zFormat, args);
        crate::src::src::vdbemem::sqlite3ValueSetStr(
            (*db).pErr,
            -(1 as ::core::ffi::c_int),
            z as *const ::core::ffi::c_void,
            crate::src::headers::sqlite3_h::SQLITE_UTF8 as crate::src::ext::rtree::rtree::u8_0,
            Some(crate::src::src::rowset::sqlite3RowSetClear as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3ErrorMsg(
    mut pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut zFormat: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut zMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    let __db_ref = unsafe { &mut *db };
    __db_ref.errByteOffset = -(2 as ::core::ffi::c_int);
    zMsg = crate::src::src::printf::sqlite3VMPrintf(db, zFormat, args);
    if __db_ref.errByteOffset < -(1 as ::core::ffi::c_int) {
        __db_ref.errByteOffset = -(1 as ::core::ffi::c_int);
    }
    if __db_ref.suppressErr != 0 {
        crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, zMsg as *mut ::core::ffi::c_void);
        if __db_ref.mallocFailed != 0 {
            (*pParse).nErr += 1;
            (*pParse).rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    } else {
        let __pParse_ref = unsafe { &mut *pParse };
        __pParse_ref.nErr += 1;
        crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, __pParse_ref.zErrMsg as *mut ::core::ffi::c_void);
        __pParse_ref.zErrMsg = zMsg;
        __pParse_ref.rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
        __pParse_ref.pWith = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::With>();
    };
}

pub unsafe extern "C" fn execSqlF(
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut zSql: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rc: ::core::ffi::c_int = 0;
    z = crate::src::src::printf::sqlite3VMPrintf(db, zSql, args);
    if z.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    rc = crate::src::src::vacuum::execSql(db, pzErrMsg, z);
    crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, z as *mut ::core::ffi::c_void);
    rc
}


#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3_config(
    mut op: ::core::ffi::c_int,
    mut args: ...
) -> ::core::ffi::c_int {
    // VaListImpl type handling - using args directly
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if crate::src::src::global::sqlite3Config.isInit != 0 {
        static mut mAnytimeConfigOption: crate::src::ext::rtree::rtree::u64_0 = 0 as crate::src::ext::rtree::rtree::u64_0
            | (1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0) << 16 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0) << 24 as ::core::ffi::c_int;
        if op < 0 as ::core::ffi::c_int
            || op > 63 as ::core::ffi::c_int
            || (1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0) << op & mAnytimeConfigOption == 0 as crate::src::ext::rtree::rtree::u64_0
        {
            return sqlite3MisuseError(440 as ::core::ffi::c_int);
        }
    }
    match  op {
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_SINGLETHREAD =>  {
            crate::src::src::global::sqlite3Config.bCoreMutex = 0 as crate::src::ext::rtree::rtree::u8_0;
            crate::src::src::global::sqlite3Config.bFullMutex = 0 as crate::src::ext::rtree::rtree::u8_0;
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_MULTITHREAD =>  {
            crate::src::src::global::sqlite3Config.bCoreMutex = 1 as crate::src::ext::rtree::rtree::u8_0;
            crate::src::src::global::sqlite3Config.bFullMutex = 0 as crate::src::ext::rtree::rtree::u8_0;
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_SERIALIZED =>  {
            crate::src::src::global::sqlite3Config.bCoreMutex = 1 as crate::src::ext::rtree::rtree::u8_0;
            crate::src::src::global::sqlite3Config.bFullMutex = 1 as crate::src::ext::rtree::rtree::u8_0;
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_MUTEX =>  {
            crate::src::src::global::sqlite3Config.mutex = *args.arg::<*mut crate::src::headers::sqlite3_h::sqlite3_mutex_methods>();
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_GETMUTEX =>  {
            *args.arg::<*mut crate::src::headers::sqlite3_h::sqlite3_mutex_methods>() = crate::src::src::global::sqlite3Config.mutex;
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_MALLOC =>  {
            crate::src::src::global::sqlite3Config.m = *args.arg::<*mut crate::src::headers::sqlite3_h::sqlite3_mem_methods>();
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_GETMALLOC =>  {
            if crate::src::src::global::sqlite3Config.m.xMalloc.is_none() {
                crate::src::src::mem1::sqlite3MemSetDefault();
            }
            *args.arg::<*mut crate::src::headers::sqlite3_h::sqlite3_mem_methods>() = crate::src::src::global::sqlite3Config.m;
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_MEMSTATUS =>  {
            crate::src::src::global::sqlite3Config.bMemstat = args.arg::<::core::ffi::c_int>();
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_SMALL_MALLOC =>  {
            crate::src::src::global::sqlite3Config.bSmallMalloc = args.arg::<::core::ffi::c_int>() as crate::src::ext::rtree::rtree::u8_0;
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_PAGECACHE =>  {
            crate::src::src::global::sqlite3Config.pPage = args.arg::<*mut ::core::ffi::c_void>();
            crate::src::src::global::sqlite3Config.szPage = args.arg::<::core::ffi::c_int>();
            crate::src::src::global::sqlite3Config.nPage = args.arg::<::core::ffi::c_int>();
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_PCACHE_HDRSZ =>  {
            *args.arg::<*mut ::core::ffi::c_int>() =
                crate::src::src::btree::sqlite3HeaderSizeBtree() + crate::src::src::pcache::sqlite3HeaderSizePcache() + crate::src::src::pcache1::sqlite3HeaderSizePcache1();
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_PCACHE =>  {}
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_GETPCACHE =>  {
            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_PCACHE2 =>  {
            crate::src::src::global::sqlite3Config.pcache2 = *args.arg::<*mut crate::src::headers::sqlite3_h::sqlite3_pcache_methods2>();
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_GETPCACHE2 =>  {
            if crate::src::src::global::sqlite3Config.pcache2.xInit.is_none() {
                crate::src::src::pcache1::sqlite3PCacheSetDefault();
            }
            *args.arg::<*mut crate::src::headers::sqlite3_h::sqlite3_pcache_methods2>() = crate::src::src::global::sqlite3Config.pcache2;
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_LOOKASIDE =>  {
            crate::src::src::global::sqlite3Config.szLookaside = args.arg::<::core::ffi::c_int>();
            crate::src::src::global::sqlite3Config.nLookaside = args.arg::<::core::ffi::c_int>();
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_LOG =>  {
            let mut xLog: LOGFUNC_t = ::core::mem::transmute(args.arg::<*mut unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
            )
                -> ()>());
            let mut pLogArg: *mut ::core::ffi::c_void = args.arg::<*mut ::core::ffi::c_void>();
            (*(&raw mut crate::src::src::global::sqlite3Config.xLog as *mut LOGFUNC_t as *mut std::sync::atomic::AtomicUsize)).store(::core::mem::transmute::<LOGFUNC_t, usize>(xLog), std::sync::atomic::Ordering::Relaxed);
            (*(&raw mut crate::src::src::global::sqlite3Config.pLogArg as *mut *mut ::core::ffi::c_void as *mut std::sync::atomic::AtomicUsize)).store(pLogArg as usize, std::sync::atomic::Ordering::Relaxed);
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_URI =>  {
            let mut bOpenUri: ::core::ffi::c_int = args.arg::<::core::ffi::c_int>();
            (*((&raw mut crate::src::src::global::sqlite3Config.bOpenUri) as *mut std::sync::atomic::AtomicU8)).store(bOpenUri as crate::src::ext::rtree::rtree::u8_0, std::sync::atomic::Ordering::Relaxed);
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_COVERING_INDEX_SCAN =>  {
            crate::src::src::global::sqlite3Config.bUseCis = args.arg::<::core::ffi::c_int>() as crate::src::ext::rtree::rtree::u8_0;
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_MMAP_SIZE =>  {
            let mut szMmap: crate::src::headers::sqlite3_h::sqlite3_int64 = args.arg::<crate::src::headers::sqlite3_h::sqlite3_int64>();
            let mut mxMmap: crate::src::headers::sqlite3_h::sqlite3_int64 = args.arg::<crate::src::headers::sqlite3_h::sqlite3_int64>();
            if mxMmap < 0 as crate::src::headers::sqlite3_h::sqlite3_int64 || mxMmap > crate::src::headers::sqliteInt_h::SQLITE_MAX_MMAP_SIZE as crate::src::headers::sqlite3_h::sqlite3_int64 {
                mxMmap = crate::src::headers::sqliteInt_h::SQLITE_MAX_MMAP_SIZE as crate::src::headers::sqlite3_h::sqlite3_int64;
            }
            if szMmap < 0 as crate::src::headers::sqlite3_h::sqlite3_int64 {
                szMmap = crate::src::headers::sqliteInt_h::SQLITE_DEFAULT_MMAP_SIZE as crate::src::headers::sqlite3_h::sqlite3_int64;
            }
            if szMmap > mxMmap {
                szMmap = mxMmap;
            }
            crate::src::src::global::sqlite3Config.mxMmap = mxMmap;
            crate::src::src::global::sqlite3Config.szMmap = szMmap;
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_PMASZ =>  {
            crate::src::src::global::sqlite3Config.szPma = args.arg::<::core::ffi::c_uint>() as crate::src::ext::rtree::rtree::u32_0;
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_STMTJRNL_SPILL =>  {
            crate::src::src::global::sqlite3Config.nStmtSpill = args.arg::<::core::ffi::c_int>();
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_MEMDB_MAXSIZE =>  {
            crate::src::src::global::sqlite3Config.mxMemdbSize = args.arg::<crate::src::headers::sqlite3_h::sqlite3_int64>();
        }
    crate::src::headers::sqlite3_h::SQLITE_CONFIG_ROWID_IN_VIEW =>  {
            let mut pVal: *mut ::core::ffi::c_int = args.arg::<*mut ::core::ffi::c_int>();
            *pVal = 0 as ::core::ffi::c_int;
        }
    _ =>  {
            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
        }
}
    rc
}


#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3_db_config(
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut op: ::core::ffi::c_int,
    mut args: ...
) -> ::core::ffi::c_int {
    // VaListImpl type handling - using args directly
    let mut rc: ::core::ffi::c_int = 0;
    crate::src::src::mutex::sqlite3_mutex_enter((*db).mutex);
    match  op {
    crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_MAINDBNAME =>  {
            let ref mut fresh0 = (*(*db).aDb.offset(0 as isize)).zDbSName;
            *fresh0 = args.arg::<*mut ::core::ffi::c_char>();
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_LOOKASIDE =>  {
            let mut pBuf: *mut ::core::ffi::c_void = args.arg::<*mut ::core::ffi::c_void>();
            let mut sz: ::core::ffi::c_int = args.arg::<::core::ffi::c_int>();
            let mut cnt: ::core::ffi::c_int = args.arg::<::core::ffi::c_int>();
            rc = setupLookaside(db, pBuf, sz, cnt);
        }
    _ =>  {
            static mut aFlagOp: [C2RustUnnamed; 21] = [
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_ENABLE_FKEY,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_ForeignKeys as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_ENABLE_TRIGGER,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_EnableTrigger as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_ENABLE_VIEW,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_EnableView as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_Fts3Tokenizer as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_LoadExtension as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_NoCkptOnClose as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_ENABLE_QPSG,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_EnableQPSG as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_TRIGGER_EQP,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_TriggerEQP as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_RESET_DATABASE,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_ResetDatabase as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_DEFENSIVE,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_Defensive as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_WRITABLE_SCHEMA,
                    mask: (crate::src::headers::sqliteInt_h::SQLITE_WriteSchema | crate::src::headers::sqliteInt_h::SQLITE_NoSchemaError) as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_LEGACY_ALTER_TABLE,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_LegacyAlter as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_DQS_DDL,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_DqsDDL as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_DQS_DML,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_DqsDML as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_LEGACY_FILE_FORMAT,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_LegacyFileFmt as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_TRUSTED_SCHEMA,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_TrustedSchema as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_STMT_SCANSTATUS,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_StmtScanStatus as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_REVERSE_SCANORDER,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_ReverseOrder as crate::src::ext::rtree::rtree::u64_0,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_ENABLE_ATTACH_CREATE,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_AttachCreate,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_ENABLE_ATTACH_WRITE,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_AttachWrite,
                },
                C2RustUnnamed {
                    op: crate::src::headers::sqlite3_h::SQLITE_DBCONFIG_ENABLE_COMMENTS,
                    mask: crate::src::headers::sqliteInt_h::SQLITE_Comments,
                },
            ];
            let mut i: ::core::ffi::c_uint = 0;
            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
            i = 0 as ::core::ffi::c_uint;
            while i
                < (::core::mem::size_of::<[C2RustUnnamed; 21]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2RustUnnamed>() as usize)
                    as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if aFlagOp[i as usize].op == op {
                    let mut onoff: ::core::ffi::c_int = args.arg::<::core::ffi::c_int>();
                    let mut pRes: *mut ::core::ffi::c_int = args.arg::<*mut ::core::ffi::c_int>();
                    let mut oldFlags: crate::src::ext::rtree::rtree::u64_0 = (*db).flags;
                    if onoff > 0 as ::core::ffi::c_int {
                        (*db).flags |= aFlagOp[i as usize].mask;
                    } else if onoff == 0 as ::core::ffi::c_int {
                        (*db).flags &= !aFlagOp[i as usize].mask;
                    }
                    if oldFlags != (*db).flags {
                        crate::src::src::vdbeaux::sqlite3ExpirePreparedStatements(db as *mut crate::src::headers::sqliteInt_h::sqlite3, 0 as ::core::ffi::c_int);
                    }
                    if !pRes.is_null() {
                        *pRes = ((*db).flags & aFlagOp[i as usize].mask != 0 as crate::src::ext::rtree::rtree::u64_0)
                            as ::core::ffi::c_int;
                    }
                    rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
        }
}
    crate::src::src::mutex::sqlite3_mutex_leave((*db).mutex);
    rc
}


#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3_test_control(
    mut op: ::core::ffi::c_int,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    // VaListImpl type handling - using args directly
    match  op {
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_PRNG_SAVE =>  {
            crate::src::src::random::sqlite3PrngSaveState();
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_PRNG_RESTORE =>  {
            crate::src::src::random::sqlite3PrngRestoreState();
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_PRNG_SEED =>  {
            let mut x: ::core::ffi::c_int = args.arg::<::core::ffi::c_int>();
            let mut y: ::core::ffi::c_int = 0;
            let mut db: *mut crate::src::headers::sqliteInt_h::sqlite3 = args.arg::<*mut crate::src::headers::sqliteInt_h::sqlite3>();
            if !db.is_null() && {
                y = (*(*(*db).aDb.offset(0 as isize)).pSchema).schema_cookie;
                y != 0 as ::core::ffi::c_int
            } {
                x = y;
            }
            crate::src::src::global::sqlite3Config.iPrngSeed = x as ::core::ffi::c_uint;
            crate::src::src::random::sqlite3_randomness(
                0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_FK_NO_ACTION =>  {
            let mut db_0: *mut crate::src::headers::sqliteInt_h::sqlite3 = args.arg::<*mut crate::src::headers::sqliteInt_h::sqlite3>();
            let mut b: ::core::ffi::c_int = args.arg::<::core::ffi::c_int>();
            if b != 0 {
                (*db_0).flags |= crate::src::headers::sqliteInt_h::SQLITE_FkNoAction;
            } else {
                (*db_0).flags &= !crate::src::headers::sqliteInt_h::SQLITE_FkNoAction;
            }
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_BITVEC_TEST =>  {
            let mut sz: ::core::ffi::c_int = args.arg::<::core::ffi::c_int>();
            let mut aProg: *mut ::core::ffi::c_int = args.arg::<*mut ::core::ffi::c_int>();
            rc = crate::src::src::bitvec::sqlite3BitvecBuiltinTest(sz, aProg);
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_FAULT_INSTALL =>  {
            crate::src::src::global::sqlite3Config.xTestCallback = ::core::mem::transmute::<
                *mut unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
                Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
            >(
                args.arg::<*mut unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>(),
            );
            rc = crate::src::src::util::sqlite3FaultSim(0 as ::core::ffi::c_int);
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_BENIGN_MALLOC_HOOKS =>  {
            let mut xBenignBegin: void_function = None;
            let mut xBenignEnd: void_function = None;
            xBenignBegin = ::core::mem::transmute(args.arg::<*mut unsafe extern "C" fn() -> ()>());
            xBenignEnd = ::core::mem::transmute(args.arg::<*mut unsafe extern "C" fn() -> ()>());
            crate::src::src::fault::sqlite3BenignMallocHooks(
                xBenignBegin as Option<unsafe extern "C" fn() -> ()>,
                xBenignEnd as Option<unsafe extern "C" fn() -> ()>,
            );
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_PENDING_BYTE =>  {
            rc = crate::src::src::global::sqlite3PendingByte;
            let mut newVal: ::core::ffi::c_uint = args.arg::<::core::ffi::c_uint>();
            if newVal != 0 {
                crate::src::src::global::sqlite3PendingByte = newVal as ::core::ffi::c_int;
            }
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_ASSERT =>  {
            let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            rc = x_0;
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_ALWAYS =>  {
            let mut x_1: ::core::ffi::c_int = args.arg::<::core::ffi::c_int>();
            rc = if x_1 != 0 {
                x_1
            } else {
                0 as ::core::ffi::c_int
            };
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_BYTEORDER =>  {
            rc = crate::src::headers::sqliteInt_h::SQLITE_BYTEORDER * 100 as ::core::ffi::c_int
                + crate::src::headers::sqliteInt_h::SQLITE_LITTLEENDIAN * 10 as ::core::ffi::c_int
                + crate::src::headers::sqliteInt_h::SQLITE_BIGENDIAN;
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_OPTIMIZATIONS =>  {
            let mut db_1: *mut crate::src::headers::sqliteInt_h::sqlite3 = args.arg::<*mut crate::src::headers::sqliteInt_h::sqlite3>();
            (*db_1).dbOptFlags = args.arg::<crate::src::ext::rtree::rtree::u32_0>();
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_GETOPT =>  {
            let mut db_2: *mut crate::src::headers::sqliteInt_h::sqlite3 = args.arg::<*mut crate::src::headers::sqliteInt_h::sqlite3>();
            let mut pN: *mut ::core::ffi::c_int = args.arg::<*mut ::core::ffi::c_int>();
            *pN = (*db_2).dbOptFlags as ::core::ffi::c_int;
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_LOCALTIME_FAULT =>  {
            crate::src::src::global::sqlite3Config.bLocaltimeFault = args.arg::<::core::ffi::c_int>();
            if crate::src::src::global::sqlite3Config.bLocaltimeFault == 2 as ::core::ffi::c_int {
                crate::src::src::global::sqlite3Config.xAltLocaltime = ::core::mem::transmute::<
                    *mut unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
                    Option<
                        unsafe extern "C" fn(
                            *const ::core::ffi::c_void,
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                    >,
                >(
                    args.arg::<*mut unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int>(),
                );
            } else {
                crate::src::src::global::sqlite3Config.xAltLocaltime = None;
            }
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_INTERNAL_FUNCTIONS =>  {
            let mut db_3: *mut crate::src::headers::sqliteInt_h::sqlite3 = args.arg::<*mut crate::src::headers::sqliteInt_h::sqlite3>();
            (*db_3).mDbFlags ^= crate::src::headers::sqliteInt_h::DBFLAG_InternalFunc as crate::src::ext::rtree::rtree::u32_0;
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_NEVER_CORRUPT =>  {
            crate::src::src::global::sqlite3Config.neverCorrupt = args.arg::<::core::ffi::c_int>();
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_EXTRA_SCHEMA_CHECKS =>  {
            crate::src::src::global::sqlite3Config.bExtraSchemaChecks = args.arg::<::core::ffi::c_int>() as crate::src::ext::rtree::rtree::u8_0;
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_ONCE_RESET_THRESHOLD =>  {
            crate::src::src::global::sqlite3Config.iOnceResetThreshold = args.arg::<::core::ffi::c_int>();
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_SORTER_MMAP =>  {
            let mut db_4: *mut crate::src::headers::sqliteInt_h::sqlite3 = args.arg::<*mut crate::src::headers::sqliteInt_h::sqlite3>();
            (*db_4).nMaxSorterMmap = args.arg::<::core::ffi::c_int>();
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_ISINIT =>  {
            if crate::src::src::global::sqlite3Config.isInit == 0 as ::core::ffi::c_int {
                rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
            }
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_IMPOSTER =>  {
            let mut db_5: *mut crate::src::headers::sqliteInt_h::sqlite3 = args.arg::<*mut crate::src::headers::sqliteInt_h::sqlite3>();
            let mut iDb: ::core::ffi::c_int = 0;
            crate::src::src::mutex::sqlite3_mutex_enter((*db_5).mutex);
            iDb = crate::src::src::build::sqlite3FindDbName(db_5 as *mut crate::src::headers::sqliteInt_h::sqlite3, args.arg::<*const ::core::ffi::c_char>());
            if iDb >= 0 as ::core::ffi::c_int {
                let __db_5_ref = unsafe { &mut *db_5 };
                __db_5_ref.init.iDb = iDb as crate::src::ext::rtree::rtree::u8_0;
                (*db_5)
                    .init
                    .set_imposterTable(args.arg::<::core::ffi::c_int>() as ::core::ffi::c_uint
                        as ::core::ffi::c_uint);
                __db_5_ref.init.busy = __db_5_ref.init.imposterTable() as crate::src::ext::rtree::rtree::u8_0;
                __db_5_ref.init.newTnum = args.arg::<::core::ffi::c_int>() as crate::src::src::pager::Pgno;
                if __db_5_ref.init.busy as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && __db_5_ref.init.newTnum > 0 as crate::src::src::pager::Pgno
                {
                    crate::src::src::build::sqlite3ResetAllSchemasOfConnection(db_5 as *mut crate::src::headers::sqliteInt_h::sqlite3);
                }
            }
            crate::src::src::mutex::sqlite3_mutex_leave((*db_5).mutex);
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_RESULT_INTREAL =>  {
            let mut pCtx: *mut crate::src::headers::vdbeInt_h::sqlite3_context =
                args.arg::<*mut ::core::ffi::c_void>() as *mut crate::src::headers::vdbeInt_h::sqlite3_context;
            crate::src::src::vdbeapi::sqlite3ResultIntReal(pCtx);
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_SEEK_COUNT =>  {
            let mut _db_6: *mut crate::src::headers::sqliteInt_h::sqlite3 = args.arg::<*mut crate::src::headers::sqliteInt_h::sqlite3>();
            let mut pn: *mut crate::src::ext::rtree::rtree::u64_0 = args.arg::<*mut crate::src::headers::sqlite3_h::sqlite3_uint64>();
            *pn = 0 as crate::src::ext::rtree::rtree::u64_0;
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_TRACEFLAGS =>  {
            let mut opTrace: ::core::ffi::c_int = args.arg::<::core::ffi::c_int>();
            let mut ptr: *mut crate::src::ext::rtree::rtree::u32_0 = args.arg::<*mut crate::src::ext::rtree::rtree::u32_0>();
            match opTrace {
                0 => {
                    *ptr = crate::src::src::global::sqlite3TreeTrace;
                }
                1 => {
                    crate::src::src::global::sqlite3TreeTrace = *ptr;
                }
                2 => {
                    *ptr = crate::src::src::global::sqlite3WhereTrace;
                }
                3 => {
                    crate::src::src::global::sqlite3WhereTrace = *ptr;
                }
                _ => {}
            }
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_LOGEST =>  {
            let mut rIn: ::core::ffi::c_double = args.arg::<::core::ffi::c_double>();
            let mut rLogEst: crate::src::headers::sqliteInt_h::LogEst = crate::src::src::util::sqlite3LogEstFromDouble(rIn);
            let mut pI1: *mut ::core::ffi::c_int = args.arg::<*mut ::core::ffi::c_int>();
            let mut pU64: *mut crate::src::ext::rtree::rtree::u64_0 = args.arg::<*mut crate::src::ext::rtree::rtree::u64_0>();
            let mut pI2: *mut ::core::ffi::c_int = args.arg::<*mut ::core::ffi::c_int>();
            *pI1 = rLogEst as ::core::ffi::c_int;
            *pU64 = crate::src::src::util::sqlite3LogEstToInt(rLogEst);
            *pI2 = crate::src::src::util::sqlite3LogEst(*pU64) as ::core::ffi::c_int;
        }
    crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_VDBE_COVERAGE |
        crate::src::headers::sqlite3_h::SQLITE_TESTCTRL_JSON_SELFCHECK | _ =>  {}
}
    rc
}




#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3NestedParse(
    mut pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut zFormat: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let __pParse_ref = unsafe { &mut *pParse };
    let mut db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
    let __db_ref = unsafe { &mut *db };
    let mut savedDbFlags: crate::src::ext::rtree::rtree::u32_0 = __db_ref.mDbFlags;
    let mut saveBuf: [::core::ffi::c_char; 136] = [0; 136];
    if __pParse_ref.nErr != 0 {
        return;
    }
    if __pParse_ref.eParseMode != 0 {
        return;
    }
    zSql = crate::src::src::printf::sqlite3VMPrintf(db, zFormat, args);
    if zSql.is_null() {
        if __db_ref.mallocFailed == 0 {
            __pParse_ref.rc = crate::src::headers::sqlite3_h::SQLITE_TOOBIG;
        }
        __pParse_ref.nErr += 1;
        return;
    }
    __pParse_ref.nested = __pParse_ref.nested.wrapping_add(1);
    ::core::ptr::copy_nonoverlapping(
                    (pParse as *mut ::core::ffi::c_char).offset(crate::src::headers::sqliteInt_h::PARSE_RECURSE_SZ as isize) as *const u8,
                    &raw mut saveBuf as *mut ::core::ffi::c_char as *mut u8,
                    (crate::src::headers::sqliteInt_h::PARSE_TAIL_SZ) as usize,
                );
    ::libc::memset(
        (pParse as *mut ::core::ffi::c_char).offset(crate::src::headers::sqliteInt_h::PARSE_RECURSE_SZ as isize)
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        crate::src::headers::sqliteInt_h::PARSE_TAIL_SZ,
    );
    __db_ref.mDbFlags |= crate::src::headers::sqliteInt_h::DBFLAG_PreferBuiltin as crate::src::ext::rtree::rtree::u32_0;
    crate::src::src::tokenize::sqlite3RunParser(pParse as *mut crate::src::headers::sqliteInt_h::Parse, zSql);
    __db_ref.mDbFlags = savedDbFlags;
    crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, zSql as *mut ::core::ffi::c_void);
    ::core::ptr::copy_nonoverlapping(
                    &raw mut saveBuf as *mut ::core::ffi::c_char as *const u8,
                    (pParse as *mut ::core::ffi::c_char).offset(crate::src::headers::sqliteInt_h::PARSE_RECURSE_SZ as isize) as *mut u8,
                    (crate::src::headers::sqliteInt_h::PARSE_TAIL_SZ) as usize,
                );
    __pParse_ref.nested = __pParse_ref.nested.wrapping_sub(1);
}






// getDigits removed — replaced by getDigits_args in date.rs


// checkAppendMsg removed — replaced by checkAppendMsg_args in btree.rs
