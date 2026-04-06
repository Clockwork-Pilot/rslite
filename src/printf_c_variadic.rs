// C variadic functions module.
// This module documents the organization of C variadic functions.
// Only the minimum required functions that absolutely need c_variadic feature are here.
// All variadic function definitions are compiled with the c_variadic feature enabled.

// Re-export printf module for convenient access
pub use crate::src::src::printf;

// Import required types and functions from their original modules
use crate::src::src::main::{LOGFUNC_t, void_function, sqlite3MisuseError, setupLookaside};
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
    z = { let (_s, _a) = crate::src::src::printf::extract_printf_args(zFormat, args, false, ::core::ptr::null_mut()); crate::src::src::printf::sqlite3VMPrintf_args(db, zFormat, &_a) };
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
pub use crate::src::src::printf::renderLogMsg;

// Variadic functions that use VaListImpl - must be defined here to use c_variadic feature
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3VdbeError(
    mut p: *mut crate::src::headers::vdbeInt_h::Vdbe,
    mut zFormat: *const ::core::ffi::c_char,
    mut args: ...
) {
    let __p_ref = unsafe { &mut *p };
    crate::src::src::malloc::sqlite3DbFree(__p_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3, __p_ref.zErrMsg as *mut ::core::ffi::c_void);
    let (_s, _a) = crate::src::src::printf::extract_printf_args(zFormat, args, false, ::core::ptr::null_mut());
    __p_ref.zErrMsg = crate::src::src::printf::sqlite3VMPrintf_args(__p_ref.db, zFormat, &_a);
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
                let (_s, _a) = crate::src::src::printf::extract_printf_args(zFmt, args, false, ::core::ptr::null_mut());
                zMsg = crate::src::src::printf::sqlite3VMPrintf_args(__pParse_ref.db, zFmt, &_a);
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
                z = { let (_s, _a) = crate::src::src::printf::extract_printf_args(zFormat, args, false, ::core::ptr::null_mut()); crate::src::src::printf::sqlite3VMPrintf_args(db, zFormat, &_a) };
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
    zMsg = { let (_s, _a) = crate::src::src::printf::extract_printf_args(zFormat, args, false, ::core::ptr::null_mut()); crate::src::src::printf::sqlite3VMPrintf_args(db, zFormat, &_a) };
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
    z = { let (_s, _a) = crate::src::src::printf::extract_printf_args(zSql, args, false, ::core::ptr::null_mut()); crate::src::src::printf::sqlite3VMPrintf_args(db, zSql, &_a) };
    if z.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    rc = crate::src::src::vacuum::execSql(db, pzErrMsg, z);
    crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, z as *mut ::core::ffi::c_void);
    rc
}


// sqlite3_config — C wrapper is in c_code/config.c
// C wrapper packs va_args into u64 slots, Rust parses into ConfigOp enum and dispatches.

use crate::src::headers::sqlite3_h::SqliteConfig;

pub enum ConfigOp {
    Singlethread,
    Multithread,
    Serialized,
    Mutex(*mut crate::src::headers::sqlite3_h::sqlite3_mutex_methods),
    GetMutex(*mut crate::src::headers::sqlite3_h::sqlite3_mutex_methods),
    Malloc(*mut crate::src::headers::sqlite3_h::sqlite3_mem_methods),
    GetMalloc(*mut crate::src::headers::sqlite3_h::sqlite3_mem_methods),
    Memstatus(::core::ffi::c_int),
    SmallMalloc(::core::ffi::c_int),
    Pagecache(*mut ::core::ffi::c_void, ::core::ffi::c_int, ::core::ffi::c_int),
    PcacheHdrsz(*mut ::core::ffi::c_int),
    Pcache,
    GetPcache,
    Pcache2(*mut crate::src::headers::sqlite3_h::sqlite3_pcache_methods2),
    GetPcache2(*mut crate::src::headers::sqlite3_h::sqlite3_pcache_methods2),
    Lookaside(::core::ffi::c_int, ::core::ffi::c_int),
    Log(LOGFUNC_t, *mut ::core::ffi::c_void),
    Uri(::core::ffi::c_int),
    CoveringIndexScan(::core::ffi::c_int),
    MmapSize(crate::src::headers::sqlite3_h::sqlite3_int64, crate::src::headers::sqlite3_h::sqlite3_int64),
    Pmasz(::core::ffi::c_uint),
    StmtjrnlSpill(::core::ffi::c_int),
    MemdbMaxsize(crate::src::headers::sqlite3_h::sqlite3_int64),
    RowidInView(*mut ::core::ffi::c_int),
    Noop,
}

impl ConfigOp {
    unsafe fn from_raw(op: ::core::ffi::c_int, args: *const u64) -> Self {
        let Some(cfg) = SqliteConfig::from_repr(op) else { return Self::Noop };
        match cfg {
            SqliteConfig::SINGLETHREAD => Self::Singlethread,
            SqliteConfig::MULTITHREAD => Self::Multithread,
            SqliteConfig::SERIALIZED => Self::Serialized,
            SqliteConfig::MUTEX => Self::Mutex(*args.offset(0) as usize as *mut _),
            SqliteConfig::GETMUTEX => Self::GetMutex(*args.offset(0) as usize as *mut _),
            SqliteConfig::MALLOC => Self::Malloc(*args.offset(0) as usize as *mut _),
            SqliteConfig::GETMALLOC => Self::GetMalloc(*args.offset(0) as usize as *mut _),
            SqliteConfig::MEMSTATUS => Self::Memstatus(*args.offset(0) as ::core::ffi::c_int),
            SqliteConfig::SMALL_MALLOC => Self::SmallMalloc(*args.offset(0) as ::core::ffi::c_int),
            SqliteConfig::PAGECACHE => Self::Pagecache(
                *args.offset(0) as usize as *mut _,
                *args.offset(1) as ::core::ffi::c_int,
                *args.offset(2) as ::core::ffi::c_int,
            ),
            SqliteConfig::PCACHE_HDRSZ => Self::PcacheHdrsz(*args.offset(0) as usize as *mut _),
            SqliteConfig::PCACHE => Self::Pcache,
            SqliteConfig::GETPCACHE => Self::GetPcache,
            SqliteConfig::PCACHE2 => Self::Pcache2(*args.offset(0) as usize as *mut _),
            SqliteConfig::GETPCACHE2 => Self::GetPcache2(*args.offset(0) as usize as *mut _),
            SqliteConfig::LOOKASIDE => Self::Lookaside(
                *args.offset(0) as ::core::ffi::c_int,
                *args.offset(1) as ::core::ffi::c_int,
            ),
            SqliteConfig::LOG => Self::Log(
                ::core::mem::transmute(*args.offset(0) as usize),
                *args.offset(1) as usize as *mut _,
            ),
            SqliteConfig::URI => Self::Uri(*args.offset(0) as ::core::ffi::c_int),
            SqliteConfig::COVERING_INDEX_SCAN => Self::CoveringIndexScan(*args.offset(0) as ::core::ffi::c_int),
            SqliteConfig::MMAP_SIZE => Self::MmapSize(
                *args.offset(0) as crate::src::headers::sqlite3_h::sqlite3_int64,
                *args.offset(1) as crate::src::headers::sqlite3_h::sqlite3_int64,
            ),
            SqliteConfig::PMASZ => Self::Pmasz(*args.offset(0) as ::core::ffi::c_uint),
            SqliteConfig::STMTJRNL_SPILL => Self::StmtjrnlSpill(*args.offset(0) as ::core::ffi::c_int),
            SqliteConfig::MEMDB_MAXSIZE => Self::MemdbMaxsize(*args.offset(0) as crate::src::headers::sqlite3_h::sqlite3_int64),
            SqliteConfig::ROWID_IN_VIEW => Self::RowidInView(*args.offset(0) as usize as *mut _),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_config_args(
    op: ::core::ffi::c_int,
    args: *const u64,
) -> ::core::ffi::c_int {
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
    match ConfigOp::from_raw(op, args) {
    ConfigOp::Singlethread =>  {
            crate::src::src::global::sqlite3Config.bCoreMutex = 0 as crate::src::ext::rtree::rtree::u8_0;
            crate::src::src::global::sqlite3Config.bFullMutex = 0 as crate::src::ext::rtree::rtree::u8_0;
        }
    ConfigOp::Multithread =>  {
            crate::src::src::global::sqlite3Config.bCoreMutex = 1 as crate::src::ext::rtree::rtree::u8_0;
            crate::src::src::global::sqlite3Config.bFullMutex = 0 as crate::src::ext::rtree::rtree::u8_0;
        }
    ConfigOp::Serialized =>  {
            crate::src::src::global::sqlite3Config.bCoreMutex = 1 as crate::src::ext::rtree::rtree::u8_0;
            crate::src::src::global::sqlite3Config.bFullMutex = 1 as crate::src::ext::rtree::rtree::u8_0;
        }
    ConfigOp::Mutex(p) =>  {
            crate::src::src::global::sqlite3Config.mutex = *p;
        }
    ConfigOp::GetMutex(p) =>  {
            *p = crate::src::src::global::sqlite3Config.mutex;
        }
    ConfigOp::Malloc(p) =>  {
            crate::src::src::global::sqlite3Config.m = *p;
        }
    ConfigOp::GetMalloc(p) =>  {
            if crate::src::src::global::sqlite3Config.m.xMalloc.is_none() {
                crate::src::src::mem1::sqlite3MemSetDefault();
            }
            *p = crate::src::src::global::sqlite3Config.m;
        }
    ConfigOp::Memstatus(val) =>  {
            crate::src::src::global::sqlite3Config.bMemstat = val;
        }
    ConfigOp::SmallMalloc(val) =>  {
            crate::src::src::global::sqlite3Config.bSmallMalloc = val as crate::src::ext::rtree::rtree::u8_0;
        }
    ConfigOp::Pagecache(pPage, szPage, nPage) =>  {
            crate::src::src::global::sqlite3Config.pPage = pPage;
            crate::src::src::global::sqlite3Config.szPage = szPage;
            crate::src::src::global::sqlite3Config.nPage = nPage;
        }
    ConfigOp::PcacheHdrsz(p) =>  {
            *p = crate::src::src::btree::sqlite3HeaderSizeBtree() + crate::src::src::pcache::sqlite3HeaderSizePcache() + crate::src::src::pcache1::sqlite3HeaderSizePcache1();
        }
    ConfigOp::Pcache =>  {}
    ConfigOp::GetPcache =>  {
            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
        }
    ConfigOp::Pcache2(p) =>  {
            crate::src::src::global::sqlite3Config.pcache2 = *p;
        }
    ConfigOp::GetPcache2(p) =>  {
            if crate::src::src::global::sqlite3Config.pcache2.xInit.is_none() {
                crate::src::src::pcache1::sqlite3PCacheSetDefault();
            }
            *p = crate::src::src::global::sqlite3Config.pcache2;
        }
    ConfigOp::Lookaside(szLookaside, nLookaside) =>  {
            crate::src::src::global::sqlite3Config.szLookaside = szLookaside;
            crate::src::src::global::sqlite3Config.nLookaside = nLookaside;
        }
    ConfigOp::Log(xLog, pLogArg) =>  {
            (*(&raw mut crate::src::src::global::sqlite3Config.xLog as *mut LOGFUNC_t as *mut std::sync::atomic::AtomicUsize)).store(::core::mem::transmute::<LOGFUNC_t, usize>(xLog), std::sync::atomic::Ordering::Relaxed);
            (*(&raw mut crate::src::src::global::sqlite3Config.pLogArg as *mut *mut ::core::ffi::c_void as *mut std::sync::atomic::AtomicUsize)).store(pLogArg as usize, std::sync::atomic::Ordering::Relaxed);
        }
    ConfigOp::Uri(bOpenUri) =>  {
            (*((&raw mut crate::src::src::global::sqlite3Config.bOpenUri) as *mut std::sync::atomic::AtomicU8)).store(bOpenUri as crate::src::ext::rtree::rtree::u8_0, std::sync::atomic::Ordering::Relaxed);
        }
    ConfigOp::CoveringIndexScan(val) =>  {
            crate::src::src::global::sqlite3Config.bUseCis = val as crate::src::ext::rtree::rtree::u8_0;
        }
    ConfigOp::MmapSize(mut szMmap, mut mxMmap) =>  {
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
    ConfigOp::Pmasz(val) =>  {
            crate::src::src::global::sqlite3Config.szPma = val as crate::src::ext::rtree::rtree::u32_0;
        }
    ConfigOp::StmtjrnlSpill(val) =>  {
            crate::src::src::global::sqlite3Config.nStmtSpill = val;
        }
    ConfigOp::MemdbMaxsize(val) =>  {
            crate::src::src::global::sqlite3Config.mxMemdbSize = val;
        }
    ConfigOp::RowidInView(pVal) =>  {
            *pVal = 0 as ::core::ffi::c_int;
        }
    ConfigOp::Noop =>  {
            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
        }
}
    rc
}


// sqlite3_db_config — C wrapper is in c_code/db_config.c
// C wrapper packs va_args into u64 slots, Rust parses and dispatches.

use crate::src::headers::sqlite3_h::SqliteDbConfig;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_db_config_args(
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    op: ::core::ffi::c_int,
    args: *const u64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    crate::src::src::mutex::sqlite3_mutex_enter((*db).mutex);

    let Some(cfg) = SqliteDbConfig::from_repr(op) else {
        crate::src::src::mutex::sqlite3_mutex_leave((*db).mutex);
        return crate::src::headers::sqlite3_h::SQLITE_ERROR;
    };

    match cfg {
        SqliteDbConfig::MAINDBNAME => {
            let ref mut fresh0 = (*(*db).aDb.offset(0 as isize)).zDbSName;
            *fresh0 = *args.offset(0) as usize as *mut ::core::ffi::c_char;
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        SqliteDbConfig::LOOKASIDE => {
            let pBuf = *args.offset(0) as usize as *mut ::core::ffi::c_void;
            let sz = *args.offset(1) as ::core::ffi::c_int;
            let cnt = *args.offset(2) as ::core::ffi::c_int;
            rc = setupLookaside(db, pBuf, sz, cnt);
        }
        _ => {
            if let Some(mask) = cfg.flag_mask() {
                let onoff = *args.offset(0) as ::core::ffi::c_int;
                let pRes = *args.offset(1) as usize as *mut ::core::ffi::c_int;
                let oldFlags: crate::src::ext::rtree::rtree::u64_0 = (*db).flags;
                if onoff > 0 as ::core::ffi::c_int {
                    (*db).flags |= mask;
                } else if onoff == 0 as ::core::ffi::c_int {
                    (*db).flags &= !mask;
                }
                if oldFlags != (*db).flags {
                    crate::src::src::vdbeaux::sqlite3ExpirePreparedStatements(
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        0 as ::core::ffi::c_int,
                    );
                }
                if !pRes.is_null() {
                    *pRes = ((*db).flags & mask
                        != 0 as crate::src::ext::rtree::rtree::u64_0)
                        as ::core::ffi::c_int;
                }
                rc = crate::src::headers::sqlite3_h::SQLITE_OK;
            } else {
                rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
            }
        }
    }

    crate::src::src::mutex::sqlite3_mutex_leave((*db).mutex);
    rc
}


// sqlite3_test_control — C wrapper is in c_code/test_control.c
// C wrapper packs va_args into u64 slots, Rust parses into TestControlOp enum and dispatches.

use crate::src::headers::sqlite3_h::SqliteTestCtrl;

type FaultCallback = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>;
type LocaltimeCallback = Option<unsafe extern "C" fn(*const ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>;

pub enum TestControlOp {
    PrngSave,
    PrngRestore,
    PrngSeed(::core::ffi::c_int, *mut crate::src::headers::sqliteInt_h::sqlite3),
    FkNoAction(*mut crate::src::headers::sqliteInt_h::sqlite3, ::core::ffi::c_int),
    BitvecTest(::core::ffi::c_int, *mut ::core::ffi::c_int),
    FaultInstall(FaultCallback),
    BenignMallocHooks(void_function, void_function),
    PendingByte(::core::ffi::c_uint),
    Assert,
    Always(::core::ffi::c_int),
    ByteOrder,
    Optimizations(*mut crate::src::headers::sqliteInt_h::sqlite3, crate::src::ext::rtree::rtree::u32_0),
    GetOpt(*mut crate::src::headers::sqliteInt_h::sqlite3, *mut ::core::ffi::c_int),
    LocaltimeFault(::core::ffi::c_int, LocaltimeCallback),
    InternalFunctions(*mut crate::src::headers::sqliteInt_h::sqlite3),
    NeverCorrupt(::core::ffi::c_int),
    ExtraSchemaChecks(::core::ffi::c_int),
    OnceResetThreshold(::core::ffi::c_int),
    SorterMmap(*mut crate::src::headers::sqliteInt_h::sqlite3, ::core::ffi::c_int),
    IsInit,
    Imposter(*mut crate::src::headers::sqliteInt_h::sqlite3, *const ::core::ffi::c_char, ::core::ffi::c_int, ::core::ffi::c_int),
    ResultIntReal(*mut crate::src::headers::vdbeInt_h::sqlite3_context),
    SeekCount(*mut crate::src::headers::sqliteInt_h::sqlite3, *mut crate::src::ext::rtree::rtree::u64_0),
    TraceFlags(::core::ffi::c_int, *mut crate::src::ext::rtree::rtree::u32_0),
    LogEst(::core::ffi::c_double, *mut ::core::ffi::c_int, *mut crate::src::ext::rtree::rtree::u64_0, *mut ::core::ffi::c_int),
    Noop,
}

impl TestControlOp {
    unsafe fn from_raw(op: ::core::ffi::c_int, args: *const u64) -> Self {
        let Some(ctrl) = SqliteTestCtrl::from_repr(op) else { return Self::Noop };
        match ctrl {
            SqliteTestCtrl::PRNG_SAVE => Self::PrngSave,
            SqliteTestCtrl::PRNG_RESTORE => Self::PrngRestore,
            SqliteTestCtrl::PRNG_SEED => Self::PrngSeed(
                *args.offset(0) as ::core::ffi::c_int,
                *args.offset(1) as usize as *mut _,
            ),
            SqliteTestCtrl::FK_NO_ACTION => Self::FkNoAction(
                *args.offset(0) as usize as *mut _,
                *args.offset(1) as ::core::ffi::c_int,
            ),
            SqliteTestCtrl::BITVEC_TEST => Self::BitvecTest(
                *args.offset(0) as ::core::ffi::c_int,
                *args.offset(1) as usize as *mut _,
            ),
            SqliteTestCtrl::FAULT_INSTALL => Self::FaultInstall(
                ::core::mem::transmute(*args.offset(0) as usize),
            ),
            SqliteTestCtrl::BENIGN_MALLOC_HOOKS => Self::BenignMallocHooks(
                ::core::mem::transmute(*args.offset(0) as usize),
                ::core::mem::transmute(*args.offset(1) as usize),
            ),
            SqliteTestCtrl::PENDING_BYTE => Self::PendingByte(
                *args.offset(0) as ::core::ffi::c_uint,
            ),
            SqliteTestCtrl::ASSERT => Self::Assert,
            SqliteTestCtrl::ALWAYS => Self::Always(
                *args.offset(0) as ::core::ffi::c_int,
            ),
            SqliteTestCtrl::BYTEORDER => Self::ByteOrder,
            SqliteTestCtrl::OPTIMIZATIONS => Self::Optimizations(
                *args.offset(0) as usize as *mut _,
                *args.offset(1) as crate::src::ext::rtree::rtree::u32_0,
            ),
            SqliteTestCtrl::GETOPT => Self::GetOpt(
                *args.offset(0) as usize as *mut _,
                *args.offset(1) as usize as *mut _,
            ),
            SqliteTestCtrl::LOCALTIME_FAULT => {
                let bFault = *args.offset(0) as ::core::ffi::c_int;
                let xAlt = if bFault == 2 {
                    ::core::mem::transmute(*args.offset(1) as usize)
                } else {
                    None
                };
                Self::LocaltimeFault(bFault, xAlt)
            },
            SqliteTestCtrl::INTERNAL_FUNCTIONS => Self::InternalFunctions(
                *args.offset(0) as usize as *mut _,
            ),
            SqliteTestCtrl::NEVER_CORRUPT => Self::NeverCorrupt(
                *args.offset(0) as ::core::ffi::c_int,
            ),
            SqliteTestCtrl::EXTRA_SCHEMA_CHECKS => Self::ExtraSchemaChecks(
                *args.offset(0) as ::core::ffi::c_int,
            ),
            SqliteTestCtrl::ONCE_RESET_THRESHOLD => Self::OnceResetThreshold(
                *args.offset(0) as ::core::ffi::c_int,
            ),
            SqliteTestCtrl::SORTER_MMAP => Self::SorterMmap(
                *args.offset(0) as usize as *mut _,
                *args.offset(1) as ::core::ffi::c_int,
            ),
            SqliteTestCtrl::ISINIT => Self::IsInit,
            SqliteTestCtrl::IMPOSTER => Self::Imposter(
                *args.offset(0) as usize as *mut _,
                *args.offset(1) as usize as *const _,
                *args.offset(2) as ::core::ffi::c_int,
                *args.offset(3) as ::core::ffi::c_int,
            ),
            SqliteTestCtrl::RESULT_INTREAL => Self::ResultIntReal(
                *args.offset(0) as usize as *mut _,
            ),
            SqliteTestCtrl::SEEK_COUNT => Self::SeekCount(
                *args.offset(0) as usize as *mut _,
                *args.offset(1) as usize as *mut _,
            ),
            SqliteTestCtrl::TRACEFLAGS => Self::TraceFlags(
                *args.offset(0) as ::core::ffi::c_int,
                *args.offset(1) as usize as *mut _,
            ),
            SqliteTestCtrl::LOGEST => Self::LogEst(
                f64::from_bits(*args.offset(0)),
                *args.offset(1) as usize as *mut _,
                *args.offset(2) as usize as *mut _,
                *args.offset(3) as usize as *mut _,
            ),
            SqliteTestCtrl::JSON_SELFCHECK | SqliteTestCtrl::VDBE_COVERAGE => Self::Noop,
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_test_control_args(
    op: ::core::ffi::c_int,
    args: *const u64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    match TestControlOp::from_raw(op, args) {
    TestControlOp::PrngSave => {
            crate::src::src::random::sqlite3PrngSaveState();
        }
    TestControlOp::PrngRestore => {
            crate::src::src::random::sqlite3PrngRestoreState();
        }
    TestControlOp::PrngSeed(mut x, db) => {
            let mut y: ::core::ffi::c_int = 0;
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
    TestControlOp::FkNoAction(db, b) => {
            if b != 0 {
                (*db).flags |= crate::src::headers::sqliteInt_h::SQLITE_FkNoAction;
            } else {
                (*db).flags &= !crate::src::headers::sqliteInt_h::SQLITE_FkNoAction;
            }
        }
    TestControlOp::BitvecTest(sz, aProg) => {
            rc = crate::src::src::bitvec::sqlite3BitvecBuiltinTest(sz, aProg);
        }
    TestControlOp::FaultInstall(xCallback) => {
            crate::src::src::global::sqlite3Config.xTestCallback = xCallback;
            rc = crate::src::src::util::sqlite3FaultSim(0 as ::core::ffi::c_int);
        }
    TestControlOp::BenignMallocHooks(xBegin, xEnd) => {
            crate::src::src::fault::sqlite3BenignMallocHooks(
                xBegin as Option<unsafe extern "C" fn() -> ()>,
                xEnd as Option<unsafe extern "C" fn() -> ()>,
            );
        }
    TestControlOp::PendingByte(newVal) => {
            rc = crate::src::src::global::sqlite3PendingByte;
            if newVal != 0 {
                crate::src::src::global::sqlite3PendingByte = newVal as ::core::ffi::c_int;
            }
        }
    TestControlOp::Assert => {
            rc = 0;
        }
    TestControlOp::Always(x) => {
            rc = if x != 0 { x } else { 0 };
        }
    TestControlOp::ByteOrder => {
            rc = crate::src::headers::sqliteInt_h::SQLITE_BYTEORDER * 100
                + crate::src::headers::sqliteInt_h::SQLITE_LITTLEENDIAN * 10
                + crate::src::headers::sqliteInt_h::SQLITE_BIGENDIAN;
        }
    TestControlOp::Optimizations(db, flags) => {
            (*db).dbOptFlags = flags;
        }
    TestControlOp::GetOpt(db, pN) => {
            *pN = (*db).dbOptFlags as ::core::ffi::c_int;
        }
    TestControlOp::LocaltimeFault(bFault, xAlt) => {
            crate::src::src::global::sqlite3Config.bLocaltimeFault = bFault;
            if bFault == 2 {
                crate::src::src::global::sqlite3Config.xAltLocaltime = xAlt;
            } else {
                crate::src::src::global::sqlite3Config.xAltLocaltime = None;
            }
        }
    TestControlOp::InternalFunctions(db) => {
            (*db).mDbFlags ^= crate::src::headers::sqliteInt_h::DBFLAG_InternalFunc as crate::src::ext::rtree::rtree::u32_0;
        }
    TestControlOp::NeverCorrupt(flag) => {
            crate::src::src::global::sqlite3Config.neverCorrupt = flag;
        }
    TestControlOp::ExtraSchemaChecks(flag) => {
            crate::src::src::global::sqlite3Config.bExtraSchemaChecks = flag as crate::src::ext::rtree::rtree::u8_0;
        }
    TestControlOp::OnceResetThreshold(val) => {
            crate::src::src::global::sqlite3Config.iOnceResetThreshold = val;
        }
    TestControlOp::SorterMmap(db, nMax) => {
            (*db).nMaxSorterMmap = nMax;
        }
    TestControlOp::IsInit => {
            if crate::src::src::global::sqlite3Config.isInit == 0 as ::core::ffi::c_int {
                rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
            }
        }
    TestControlOp::Imposter(db, zSchema, onOff, tnum) => {
            crate::src::src::mutex::sqlite3_mutex_enter((*db).mutex);
            let iDb = crate::src::src::build::sqlite3FindDbName(db, zSchema);
            if iDb >= 0 as ::core::ffi::c_int {
                let db_ref = &mut *db;
                db_ref.init.iDb = iDb as crate::src::ext::rtree::rtree::u8_0;
                (*db).init.set_imposterTable(onOff as ::core::ffi::c_uint);
                db_ref.init.busy = db_ref.init.imposterTable() as crate::src::ext::rtree::rtree::u8_0;
                db_ref.init.newTnum = tnum as crate::src::src::pager::Pgno;
                if db_ref.init.busy as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && db_ref.init.newTnum > 0 as crate::src::src::pager::Pgno
                {
                    crate::src::src::build::sqlite3ResetAllSchemasOfConnection(db);
                }
            }
            crate::src::src::mutex::sqlite3_mutex_leave((*db).mutex);
        }
    TestControlOp::ResultIntReal(pCtx) => {
            crate::src::src::vdbeapi::sqlite3ResultIntReal(pCtx);
        }
    TestControlOp::SeekCount(_db, pn) => {
            *pn = 0 as crate::src::ext::rtree::rtree::u64_0;
        }
    TestControlOp::TraceFlags(opTrace, ptr) => {
            match opTrace {
                0 => { *ptr = crate::src::src::global::sqlite3TreeTrace; }
                1 => { crate::src::src::global::sqlite3TreeTrace = *ptr; }
                2 => { *ptr = crate::src::src::global::sqlite3WhereTrace; }
                3 => { crate::src::src::global::sqlite3WhereTrace = *ptr; }
                _ => {}
            }
        }
    TestControlOp::LogEst(rIn, pI1, pU64, pI2) => {
            let rLogEst: crate::src::headers::sqliteInt_h::LogEst = crate::src::src::util::sqlite3LogEstFromDouble(rIn);
            *pI1 = rLogEst as ::core::ffi::c_int;
            *pU64 = crate::src::src::util::sqlite3LogEstToInt(rLogEst);
            *pI2 = crate::src::src::util::sqlite3LogEst(*pU64) as ::core::ffi::c_int;
        }
    TestControlOp::Noop => {}
    }
    rc
}


// getDigits removed — replaced by getDigits_args in date.rs


// checkAppendMsg removed — replaced by checkAppendMsg_args in btree.rs
