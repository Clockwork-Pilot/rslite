// C variadic functions module.
// This module documents the organization of C variadic functions.
// Only the minimum required functions that absolutely need c_variadic feature are here.
// All variadic function definitions are compiled with the c_variadic feature enabled.

// Re-export printf module for convenient access
pub use crate::src::src::printf;

// Import required types and functions from their original modules
use crate::src::fts5::{Fts5Config, Fts5FullTable};
use crate::src::src::btree::{checkOom, checkProgress};
use crate::src::src::main::{LOGFUNC_t, setupLookaside, sqlite3MisuseError, void_function};

// Old variadic functions removed — all callers now use _args versions.
// sqlite3_log uses C wrapper in c_code/log.c → sqlite3_log_formatted in Rust.

// sqlite3_vtab_config — C wrapper is in c_code/vtab_config.c
// C wrapper packs va_args into u64 slots, Rust parses into VtabConfigOp enum and dispatches.

use crate::src::headers::sqlite3_h::SqliteVtabConfig;

pub enum VtabConfigOp {
    ConstraintSupport(::core::ffi::c_int),
    Innocuous,
    DirectOnly,
    UsesAllSchemas,
    Noop,
}

impl VtabConfigOp {
    unsafe fn from_raw(op: ::core::ffi::c_int, args: *const u64) -> Self {
        let Some(cfg) = SqliteVtabConfig::from_repr(op) else {
            return Self::Noop;
        };
        match cfg {
            SqliteVtabConfig::CONSTRAINT_SUPPORT => {
                Self::ConstraintSupport(*args.offset(0) as ::core::ffi::c_int)
            }
            SqliteVtabConfig::INNOCUOUS => Self::Innocuous,
            SqliteVtabConfig::DIRECTONLY => Self::DirectOnly,
            SqliteVtabConfig::USES_ALL_SCHEMAS => Self::UsesAllSchemas,
        }
    }
}

#[unsafe(export_name = "sqlite3_vtab_config_args")]
pub unsafe extern "C" fn rs_vtab_config_dispatch(
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    op: ::core::ffi::c_int,
    args: *const u64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut p: *mut crate::src::src::vtab::VtabCtx =
        ::core::ptr::null_mut::<crate::src::src::vtab::VtabCtx>();
    let __db_ref = unsafe { &*db };
    crate::src::src::mutex::sqlite3_mutex_enter(__db_ref.mutex);
    p = __db_ref.pVtabCtx;
    if p.is_null() {
        rc = crate::src::src::main::sqlite3MisuseError(1346 as ::core::ffi::c_int);
    } else {
        match VtabConfigOp::from_raw(op, args) {
            VtabConfigOp::ConstraintSupport(val) => {
                (*(*p).pVTable).bConstraint = val as crate::src::ext::rtree::rtree::u8_0;
            }
            VtabConfigOp::Innocuous => {
                (*(*p).pVTable).eVtabRisk = crate::src::headers::sqliteInt_h::SQLITE_VTABRISK_Low
                    as crate::src::ext::rtree::rtree::u8_0;
            }
            VtabConfigOp::DirectOnly => {
                (*(*p).pVTable).eVtabRisk = crate::src::headers::sqliteInt_h::SQLITE_VTABRISK_High
                    as crate::src::ext::rtree::rtree::u8_0;
            }
            VtabConfigOp::UsesAllSchemas => {
                (*(*p).pVTable).bAllSchemas = 1 as crate::src::ext::rtree::rtree::u8_0;
            }
            VtabConfigOp::Noop => {
                rc = crate::src::src::main::sqlite3MisuseError(1368 as ::core::ffi::c_int);
            }
        }
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::util::sqlite3Error(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            rc,
        );
    }
    crate::src::src::mutex::sqlite3_mutex_leave(__db_ref.mutex);
    rc
}
// Non-variadic _args versions of printf-pattern functions

pub unsafe fn sqlite3MPrintf_args(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zFormat: *const ::core::ffi::c_char,
    args: &[crate::src::src::printf::PrintfArg],
) -> *mut ::core::ffi::c_char {
    crate::src::src::printf::sqlite3VMPrintf_args(db, zFormat, args)
}

pub unsafe fn sqlite3DebugPrintf_args(
    zFormat: *const ::core::ffi::c_char,
    args: &[crate::src::src::printf::PrintfArg],
) {
    let mut acc: crate::src::headers::sqliteInt_h::StrAccum = ::core::mem::zeroed();
    let mut zBuf: [::core::ffi::c_char; 700] = [0; 700];
    crate::src::src::printf::sqlite3StrAccumInit(
        &raw mut acc,
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>(),
        &raw mut zBuf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 700]>() as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    crate::src::src::printf::sqlite3_str_vappendf_args(&raw mut acc, zFormat, args);
    crate::src::src::printf::sqlite3StrAccumFinish(&raw mut acc);
    crate::src::headers::stdlib::fprintf(
        crate::src::headers::stdlib::stdout,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut zBuf as *mut ::core::ffi::c_char,
    );
    crate::src::headers::stdlib::fflush(crate::src::headers::stdlib::stdout);
}

pub unsafe fn sqlite3ErrorMsg_args(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    zFormat: *const ::core::ffi::c_char,
    args: &[crate::src::src::printf::PrintfArg],
) {
    let mut zMsg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    let __db_ref = &mut *db;
    __db_ref.errByteOffset = -(2 as ::core::ffi::c_int);
    zMsg = crate::src::src::printf::sqlite3VMPrintf_args(db, zFormat, args);
    if __db_ref.errByteOffset < -(1 as ::core::ffi::c_int) {
        __db_ref.errByteOffset = -(1 as ::core::ffi::c_int);
    }
    if __db_ref.suppressErr != 0 {
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            zMsg as *mut ::core::ffi::c_void,
        );
        if __db_ref.mallocFailed != 0 {
            (*pParse).nErr += 1;
            (*pParse).rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    } else {
        let __pParse_ref = &mut *pParse;
        __pParse_ref.nErr += 1;
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            __pParse_ref.zErrMsg as *mut ::core::ffi::c_void,
        );
        __pParse_ref.zErrMsg = zMsg;
        __pParse_ref.rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
        __pParse_ref.pWith = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::With>();
    };
}

pub unsafe fn sqlite3ErrorWithMsg_args(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    err_code: ::core::ffi::c_int,
    zFormat: *const ::core::ffi::c_char,
    args: &[crate::src::src::printf::PrintfArg],
) {
    (*db).errCode = err_code;
    crate::src::src::util::sqlite3SystemError(db, err_code);
    if zFormat.is_null() {
        crate::src::src::util::sqlite3Error(db, err_code);
    } else if !(*db).pErr.is_null() || {
        (*db).pErr = crate::src::src::vdbemem::sqlite3ValueNew(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        );
        !(*db).pErr.is_null()
    } {
        let z = crate::src::src::printf::sqlite3VMPrintf_args(db, zFormat, args);
        crate::src::src::vdbemem::sqlite3ValueSetStr(
            (*db).pErr,
            -(1 as ::core::ffi::c_int),
            z as *const ::core::ffi::c_void,
            crate::src::headers::sqlite3_h::SQLITE_UTF8 as crate::src::ext::rtree::rtree::u8_0,
            Some(
                crate::src::src::rowset::sqlite3RowSetClear
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
        );
    }
}

pub unsafe fn execSqlF_args(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pzErrMsg: *mut *mut ::core::ffi::c_char,
    zSql: *const ::core::ffi::c_char,
    args: &[crate::src::src::printf::PrintfArg],
) -> ::core::ffi::c_int {
    let z = crate::src::src::printf::sqlite3VMPrintf_args(db, zSql, args);
    if z.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    let rc = crate::src::src::vacuum::execSql(db, pzErrMsg, z);
    crate::src::src::malloc::sqlite3DbFree(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        z as *mut ::core::ffi::c_void,
    );
    rc
}

pub unsafe fn sqlite3_log_args(
    iErrCode: ::core::ffi::c_int,
    zFormat: *const ::core::ffi::c_char,
    args: &[crate::src::src::printf::PrintfArg],
) {
    if crate::src::src::global::sqlite3Config.xLog.is_some() {
        crate::src::src::printf::renderLogMsg_args(iErrCode, zFormat, args);
    }
}

// sqlite3_log — C wrapper is in c_code/log.c
// C wrapper formats the message with sqlite3_vsnprintf, then calls sqlite3_log_formatted.
#[unsafe(export_name = "sqlite3_log_formatted")]
pub unsafe extern "C" fn rs_log_dispatch(
    iErrCode: ::core::ffi::c_int,
    zMsg: *const ::core::ffi::c_char,
) {
    if let Some(xLog) = crate::src::src::global::sqlite3Config.xLog {
        xLog(
            crate::src::src::global::sqlite3Config.pLogArg,
            iErrCode,
            zMsg,
        );
    }
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
    Pagecache(
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ),
    PcacheHdrsz(*mut ::core::ffi::c_int),
    Pcache,
    GetPcache,
    Pcache2(*mut crate::src::headers::sqlite3_h::sqlite3_pcache_methods2),
    GetPcache2(*mut crate::src::headers::sqlite3_h::sqlite3_pcache_methods2),
    Lookaside(::core::ffi::c_int, ::core::ffi::c_int),
    Log(LOGFUNC_t, *mut ::core::ffi::c_void),
    Uri(::core::ffi::c_int),
    CoveringIndexScan(::core::ffi::c_int),
    MmapSize(
        crate::src::headers::sqlite3_h::sqlite3_int64,
        crate::src::headers::sqlite3_h::sqlite3_int64,
    ),
    Pmasz(::core::ffi::c_uint),
    StmtjrnlSpill(::core::ffi::c_int),
    MemdbMaxsize(crate::src::headers::sqlite3_h::sqlite3_int64),
    RowidInView(*mut ::core::ffi::c_int),
    Noop,
}

impl ConfigOp {
    unsafe fn from_raw(op: ::core::ffi::c_int, args: *const u64) -> Self {
        let Some(cfg) = SqliteConfig::from_repr(op) else {
            return Self::Noop;
        };
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
            SqliteConfig::COVERING_INDEX_SCAN => {
                Self::CoveringIndexScan(*args.offset(0) as ::core::ffi::c_int)
            }
            SqliteConfig::MMAP_SIZE => Self::MmapSize(
                *args.offset(0) as crate::src::headers::sqlite3_h::sqlite3_int64,
                *args.offset(1) as crate::src::headers::sqlite3_h::sqlite3_int64,
            ),
            SqliteConfig::PMASZ => Self::Pmasz(*args.offset(0) as ::core::ffi::c_uint),
            SqliteConfig::STMTJRNL_SPILL => {
                Self::StmtjrnlSpill(*args.offset(0) as ::core::ffi::c_int)
            }
            SqliteConfig::MEMDB_MAXSIZE => {
                Self::MemdbMaxsize(*args.offset(0) as crate::src::headers::sqlite3_h::sqlite3_int64)
            }
            SqliteConfig::ROWID_IN_VIEW => Self::RowidInView(*args.offset(0) as usize as *mut _),
        }
    }
}

#[unsafe(export_name = "sqlite3_config_args")]
pub unsafe extern "C" fn rs_config_dispatch(
    op: ::core::ffi::c_int,
    args: *const u64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if crate::src::src::global::sqlite3Config.isInit != 0 {
        static mut mAnytimeConfigOption: crate::src::ext::rtree::rtree::u64_0 = 0
            as crate::src::ext::rtree::rtree::u64_0
            | (1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0)
                << 16 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0)
                << 24 as ::core::ffi::c_int;
        if op < 0 as ::core::ffi::c_int
            || op > 63 as ::core::ffi::c_int
            || (1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0) << op
                & mAnytimeConfigOption
                == 0 as crate::src::ext::rtree::rtree::u64_0
        {
            return sqlite3MisuseError(440 as ::core::ffi::c_int);
        }
    }
    match ConfigOp::from_raw(op, args) {
        ConfigOp::Singlethread => {
            crate::src::src::global::sqlite3Config.bCoreMutex =
                0 as crate::src::ext::rtree::rtree::u8_0;
            crate::src::src::global::sqlite3Config.bFullMutex =
                0 as crate::src::ext::rtree::rtree::u8_0;
        }
        ConfigOp::Multithread => {
            crate::src::src::global::sqlite3Config.bCoreMutex =
                1 as crate::src::ext::rtree::rtree::u8_0;
            crate::src::src::global::sqlite3Config.bFullMutex =
                0 as crate::src::ext::rtree::rtree::u8_0;
        }
        ConfigOp::Serialized => {
            crate::src::src::global::sqlite3Config.bCoreMutex =
                1 as crate::src::ext::rtree::rtree::u8_0;
            crate::src::src::global::sqlite3Config.bFullMutex =
                1 as crate::src::ext::rtree::rtree::u8_0;
        }
        ConfigOp::Mutex(p) => {
            crate::src::src::global::sqlite3Config.mutex = *p;
        }
        ConfigOp::GetMutex(p) => {
            *p = crate::src::src::global::sqlite3Config.mutex;
        }
        ConfigOp::Malloc(p) => {
            crate::src::src::global::sqlite3Config.m = *p;
        }
        ConfigOp::GetMalloc(p) => {
            if crate::src::src::global::sqlite3Config.m.xMalloc.is_none() {
                crate::src::src::mem1::sqlite3MemSetDefault();
            }
            *p = crate::src::src::global::sqlite3Config.m;
        }
        ConfigOp::Memstatus(val) => {
            crate::src::src::global::sqlite3Config.bMemstat = val;
        }
        ConfigOp::SmallMalloc(val) => {
            crate::src::src::global::sqlite3Config.bSmallMalloc =
                val as crate::src::ext::rtree::rtree::u8_0;
        }
        ConfigOp::Pagecache(pPage, szPage, nPage) => {
            crate::src::src::global::sqlite3Config.pPage = pPage;
            crate::src::src::global::sqlite3Config.szPage = szPage;
            crate::src::src::global::sqlite3Config.nPage = nPage;
        }
        ConfigOp::PcacheHdrsz(p) => {
            *p = crate::src::src::btree::sqlite3HeaderSizeBtree()
                + crate::src::src::pcache::sqlite3HeaderSizePcache()
                + crate::src::src::pcache1::sqlite3HeaderSizePcache1();
        }
        ConfigOp::Pcache => {}
        ConfigOp::GetPcache => {
            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
        }
        ConfigOp::Pcache2(p) => {
            crate::src::src::global::sqlite3Config.pcache2 = *p;
        }
        ConfigOp::GetPcache2(p) => {
            if crate::src::src::global::sqlite3Config
                .pcache2
                .xInit
                .is_none()
            {
                crate::src::src::pcache1::sqlite3PCacheSetDefault();
            }
            *p = crate::src::src::global::sqlite3Config.pcache2;
        }
        ConfigOp::Lookaside(szLookaside, nLookaside) => {
            crate::src::src::global::sqlite3Config.szLookaside = szLookaside;
            crate::src::src::global::sqlite3Config.nLookaside = nLookaside;
        }
        ConfigOp::Log(xLog, pLogArg) => {
            (*(&raw mut crate::src::src::global::sqlite3Config.xLog as *mut LOGFUNC_t
                as *mut std::sync::atomic::AtomicUsize))
                .store(
                    ::core::mem::transmute::<LOGFUNC_t, usize>(xLog),
                    std::sync::atomic::Ordering::Relaxed,
                );
            (*(&raw mut crate::src::src::global::sqlite3Config.pLogArg
                as *mut *mut ::core::ffi::c_void
                as *mut std::sync::atomic::AtomicUsize))
                .store(pLogArg as usize, std::sync::atomic::Ordering::Relaxed);
        }
        ConfigOp::Uri(bOpenUri) => {
            (*((&raw mut crate::src::src::global::sqlite3Config.bOpenUri)
                as *mut std::sync::atomic::AtomicU8))
                .store(
                    bOpenUri as crate::src::ext::rtree::rtree::u8_0,
                    std::sync::atomic::Ordering::Relaxed,
                );
        }
        ConfigOp::CoveringIndexScan(val) => {
            crate::src::src::global::sqlite3Config.bUseCis =
                val as crate::src::ext::rtree::rtree::u8_0;
        }
        ConfigOp::MmapSize(mut szMmap, mut mxMmap) => {
            if mxMmap < 0 as crate::src::headers::sqlite3_h::sqlite3_int64
                || mxMmap
                    > crate::src::headers::sqliteInt_h::SQLITE_MAX_MMAP_SIZE
                        as crate::src::headers::sqlite3_h::sqlite3_int64
            {
                mxMmap = crate::src::headers::sqliteInt_h::SQLITE_MAX_MMAP_SIZE
                    as crate::src::headers::sqlite3_h::sqlite3_int64;
            }
            if szMmap < 0 as crate::src::headers::sqlite3_h::sqlite3_int64 {
                szMmap = crate::src::headers::sqliteInt_h::SQLITE_DEFAULT_MMAP_SIZE
                    as crate::src::headers::sqlite3_h::sqlite3_int64;
            }
            if szMmap > mxMmap {
                szMmap = mxMmap;
            }
            crate::src::src::global::sqlite3Config.mxMmap = mxMmap;
            crate::src::src::global::sqlite3Config.szMmap = szMmap;
        }
        ConfigOp::Pmasz(val) => {
            crate::src::src::global::sqlite3Config.szPma =
                val as crate::src::ext::rtree::rtree::u32_0;
        }
        ConfigOp::StmtjrnlSpill(val) => {
            crate::src::src::global::sqlite3Config.nStmtSpill = val;
        }
        ConfigOp::MemdbMaxsize(val) => {
            crate::src::src::global::sqlite3Config.mxMemdbSize = val;
        }
        ConfigOp::RowidInView(pVal) => {
            *pVal = 0 as ::core::ffi::c_int;
        }
        ConfigOp::Noop => {
            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
        }
    }
    rc
}
// sqlite3_db_config — C wrapper is in c_code/db_config.c
// C wrapper packs va_args into u64 slots, Rust parses and dispatches.

use crate::src::headers::sqlite3_h::SqliteDbConfig;

#[unsafe(export_name = "sqlite3_db_config_args")]
pub unsafe extern "C" fn rs_db_config_dispatch(
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
                    *pRes = ((*db).flags & mask != 0 as crate::src::ext::rtree::rtree::u64_0)
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
type LocaltimeCallback = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;

pub enum TestControlOp {
    PrngSave,
    PrngRestore,
    PrngSeed(
        ::core::ffi::c_int,
        *mut crate::src::headers::sqliteInt_h::sqlite3,
    ),
    FkNoAction(
        *mut crate::src::headers::sqliteInt_h::sqlite3,
        ::core::ffi::c_int,
    ),
    BitvecTest(::core::ffi::c_int, *mut ::core::ffi::c_int),
    FaultInstall(FaultCallback),
    BenignMallocHooks(void_function, void_function),
    PendingByte(::core::ffi::c_uint),
    Assert,
    Always(::core::ffi::c_int),
    ByteOrder,
    Optimizations(
        *mut crate::src::headers::sqliteInt_h::sqlite3,
        crate::src::ext::rtree::rtree::u32_0,
    ),
    GetOpt(
        *mut crate::src::headers::sqliteInt_h::sqlite3,
        *mut ::core::ffi::c_int,
    ),
    LocaltimeFault(::core::ffi::c_int, LocaltimeCallback),
    InternalFunctions(*mut crate::src::headers::sqliteInt_h::sqlite3),
    NeverCorrupt(::core::ffi::c_int),
    ExtraSchemaChecks(::core::ffi::c_int),
    OnceResetThreshold(::core::ffi::c_int),
    SorterMmap(
        *mut crate::src::headers::sqliteInt_h::sqlite3,
        ::core::ffi::c_int,
    ),
    IsInit,
    Imposter(
        *mut crate::src::headers::sqliteInt_h::sqlite3,
        *const ::core::ffi::c_char,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ),
    ResultIntReal(*mut crate::src::headers::vdbeInt_h::sqlite3_context),
    SeekCount(
        *mut crate::src::headers::sqliteInt_h::sqlite3,
        *mut crate::src::ext::rtree::rtree::u64_0,
    ),
    TraceFlags(
        ::core::ffi::c_int,
        *mut crate::src::ext::rtree::rtree::u32_0,
    ),
    LogEst(
        ::core::ffi::c_double,
        *mut ::core::ffi::c_int,
        *mut crate::src::ext::rtree::rtree::u64_0,
        *mut ::core::ffi::c_int,
    ),
    Noop,
}

impl TestControlOp {
    unsafe fn from_raw(op: ::core::ffi::c_int, args: *const u64) -> Self {
        let Some(ctrl) = SqliteTestCtrl::from_repr(op) else {
            return Self::Noop;
        };
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
            SqliteTestCtrl::FAULT_INSTALL => {
                Self::FaultInstall(::core::mem::transmute(*args.offset(0) as usize))
            }
            SqliteTestCtrl::BENIGN_MALLOC_HOOKS => Self::BenignMallocHooks(
                ::core::mem::transmute(*args.offset(0) as usize),
                ::core::mem::transmute(*args.offset(1) as usize),
            ),
            SqliteTestCtrl::PENDING_BYTE => {
                Self::PendingByte(*args.offset(0) as ::core::ffi::c_uint)
            }
            SqliteTestCtrl::ASSERT => Self::Assert,
            SqliteTestCtrl::ALWAYS => Self::Always(*args.offset(0) as ::core::ffi::c_int),
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
            }
            SqliteTestCtrl::INTERNAL_FUNCTIONS => {
                Self::InternalFunctions(*args.offset(0) as usize as *mut _)
            }
            SqliteTestCtrl::NEVER_CORRUPT => {
                Self::NeverCorrupt(*args.offset(0) as ::core::ffi::c_int)
            }
            SqliteTestCtrl::EXTRA_SCHEMA_CHECKS => {
                Self::ExtraSchemaChecks(*args.offset(0) as ::core::ffi::c_int)
            }
            SqliteTestCtrl::ONCE_RESET_THRESHOLD => {
                Self::OnceResetThreshold(*args.offset(0) as ::core::ffi::c_int)
            }
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
            SqliteTestCtrl::RESULT_INTREAL => {
                Self::ResultIntReal(*args.offset(0) as usize as *mut _)
            }
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

#[unsafe(export_name = "sqlite3_test_control_args")]
pub unsafe extern "C" fn rs_test_control_dispatch(
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
            (*db).mDbFlags ^= crate::src::headers::sqliteInt_h::DBFLAG_InternalFunc
                as crate::src::ext::rtree::rtree::u32_0;
        }
        TestControlOp::NeverCorrupt(flag) => {
            crate::src::src::global::sqlite3Config.neverCorrupt = flag;
        }
        TestControlOp::ExtraSchemaChecks(flag) => {
            crate::src::src::global::sqlite3Config.bExtraSchemaChecks =
                flag as crate::src::ext::rtree::rtree::u8_0;
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
                db_ref.init.busy =
                    db_ref.init.imposterTable() as crate::src::ext::rtree::rtree::u8_0;
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
        TestControlOp::TraceFlags(opTrace, ptr) => match opTrace {
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
        },
        TestControlOp::LogEst(rIn, pI1, pU64, pI2) => {
            let rLogEst: crate::src::headers::sqliteInt_h::LogEst =
                crate::src::src::util::sqlite3LogEstFromDouble(rIn);
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
