use ::c2rust_bitfields;
use crate::printf_args;

















pub use crate::src::headers::stdlib::va_list;
pub use crate::__stddef_size_t_h::size_t;


pub use crate::src::src::hash::Hash;pub use crate::src::src::hash::HashElem;pub use crate::src::src::hash::_ht;pub use crate::internal::__builtin_va_list;pub use crate::internal::__va_list_tag;
pub use crate::src::src::pager::Pgno;

pub use crate::src::headers::vdbeInt_h::sqlite3_context;pub use crate::src::src::vdbeapi::sqlite3_context_db_handle;pub use crate::src::headers::sqlite3_h::sqlite3_destructor_type;pub use crate::src::headers::sqlite3_h::sqlite3_file;pub use crate::src::headers::sqlite3_h::sqlite3_filename;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;pub use crate::src::headers::sqlite3_h::sqlite3_index_info;pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;pub use crate::src::headers::sqlite3_h::sqlite3_int64;pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;pub use crate::src::headers::sqlite3_h::sqlite3_mem_methods;pub use crate::src::headers::sqlite3_h::sqlite3_module;pub use crate::src::src::mutex_unix::sqlite3_mutex;pub use crate::src::headers::sqlite3_h::sqlite3_mutex_methods;pub use crate::src::headers::sqlite3_h::sqlite3_pcache;pub use crate::src::headers::sqlite3_h::sqlite3_pcache_methods2;pub use crate::src::headers::sqlite3_h::sqlite3_pcache_page;pub use crate::src::src::vdbeapi::sqlite3_result_double;pub use crate::src::src::vdbeapi::sqlite3_result_error;pub use crate::src::src::vdbeapi::sqlite3_result_int64;pub use crate::src::src::vdbeapi::sqlite3_result_text;pub use crate::src::src::printf::sqlite3_str_append;pub use crate::src::src::printf::sqlite3_str_appendchar;pub use crate::src::src::printf::sqlite3_str_appendf;pub use crate::src::src::printf::sqlite3_str_reset;pub use crate::src::src::util::sqlite3_stricmp;pub use crate::src::src::util::sqlite3_strnicmp;pub use crate::src::headers::sqlite3_h::sqlite3_syscall_ptr;pub use crate::src::headers::sqlite3_h::sqlite3_uint64;pub use crate::src::headers::vdbeInt_h::sqlite3_value;pub use crate::src::src::vdbeapi::sqlite3_value_bytes;pub use crate::src::src::vdbeapi::sqlite3_value_double;pub use crate::src::src::vdbeapi::sqlite3_value_text;pub use crate::src::src::vdbeapi::sqlite3_value_type;pub use crate::src::headers::sqlite3_h::sqlite3_vfs;pub use crate::src::headers::sqlite3_h::sqlite3_vtab;pub use crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;pub use crate::src::headers::sqlite3_h::sqlite_int64;pub use crate::src::headers::sqlite3_h::sqlite_uint64;pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;pub use crate::src::headers::sqlite3_h::SQLITE_FLOAT;pub use crate::src::headers::sqlite3_h::SQLITE_INTEGER;pub use crate::src::headers::sqlite3_h::SQLITE_LIMIT_LENGTH;pub use crate::src::headers::sqlite3_h::SQLITE_OK;pub use crate::src::headers::sqlite3_h::SQLITE_UTF8;pub use crate::src::headers::sqliteInt_h::__anon_struct_0;pub use crate::src::headers::sqliteInt_h::__anon_struct_1;pub use crate::src::headers::sqliteInt_h::__anon_struct_2;pub use crate::src::headers::sqliteInt_h::__anon_struct_3;pub use crate::src::headers::sqliteInt_h::__anon_struct_4;pub use crate::src::headers::sqliteInt_h::__anon_struct_5;pub use crate::src::headers::sqliteInt_h::__anon_struct_6;pub use crate::src::headers::sqliteInt_h::__anon_struct_7;pub use crate::src::headers::sqliteInt_h::__anon_struct_8;pub use crate::src::headers::sqliteInt_h::__anon_union_0;pub use crate::src::headers::sqliteInt_h::__anon_union_1;pub use crate::src::headers::sqliteInt_h::__anon_union_10;pub use crate::src::headers::sqliteInt_h::__anon_union_11;pub use crate::src::headers::sqliteInt_h::__anon_union_12;pub use crate::src::headers::sqliteInt_h::__anon_union_13;pub use crate::src::headers::sqliteInt_h::__anon_union_15;pub use crate::src::headers::sqliteInt_h::__anon_union_2;pub use crate::src::headers::sqliteInt_h::__anon_union_3;pub use crate::src::headers::sqliteInt_h::__anon_union_5;pub use crate::src::headers::sqliteInt_h::__anon_union_6;pub use crate::src::headers::sqliteInt_h::__anon_union_7;pub use crate::src::headers::sqliteInt_h::__anon_union_8;pub use crate::src::headers::sqliteInt_h::__anon_union_9;pub use crate::src::headers::sqliteInt_h::bft;pub use crate::src::fts5::i16_0;pub use crate::src::ext::rtree::rtree::i64_0;pub use crate::src::headers::sqliteInt_h::sColMap;pub use crate::src::headers::sqliteInt_h::sqlite3;pub use crate::src::src::util::sqlite3AtoF;pub use crate::src::src::global::sqlite3Config;pub use crate::src::src::global::sqlite3CtypeMap;pub use crate::src::headers::sqliteInt_h::sqlite3InitInfo;pub use crate::src::src::callback::sqlite3InsertBuiltinFuncs;pub use crate::src::src::printf::sqlite3ResultStrAccum;pub use crate::src::src::vdbeapi::sqlite3StmtCurrentTime;pub use crate::src::src::printf::sqlite3StrAccumInit;pub use crate::src::src::util::sqlite3StrICmp;pub use crate::src::src::util::sqlite3Strlen30;pub use crate::src::src::global::sqlite3UpperToLower;pub use crate::src::headers::sqliteInt_h::sqlite3_str;pub use crate::src::headers::sqliteInt_h::sqlite3_xauth;pub use crate::src::fts5::u16_0;pub use crate::src::ext::rtree::rtree::u32_0;pub use crate::src::ext::rtree::rtree::u64_0;pub use crate::src::ext::rtree::rtree::u8_0;pub use crate::src::headers::sqliteInt_h::yDbMask;pub use crate::src::headers::sqliteInt_h::ynVar;pub use crate::src::headers::sqliteInt_h::AggInfo;pub use crate::src::headers::sqliteInt_h::AggInfo_col;pub use crate::src::headers::sqliteInt_h::AggInfo_func;pub use crate::src::headers::sqliteInt_h::AutoincInfo;pub use crate::src::headers::sqliteInt_h::Bitmask;pub use crate::src::headers::sqliteInt_h::BusyHandler;pub use crate::src::headers::sqliteInt_h::CollSeq;pub use crate::src::headers::sqliteInt_h::Column;pub use crate::src::headers::sqliteInt_h::Cte;pub use crate::src::headers::sqliteInt_h::CteUse;pub use crate::src::headers::sqliteInt_h::Db;pub use crate::src::headers::sqliteInt_h::DbClientData;pub use crate::src::headers::sqliteInt_h::Expr;pub use crate::src::headers::sqliteInt_h::ExprList;pub use crate::src::headers::sqliteInt_h::ExprList_item;pub use crate::src::headers::sqliteInt_h::FKey;pub use crate::src::headers::sqliteInt_h::FuncDef;pub use crate::src::headers::sqliteInt_h::FuncDestructor;pub use crate::src::headers::sqliteInt_h::IdList;pub use crate::src::headers::sqliteInt_h::IdList_item;pub use crate::src::headers::sqliteInt_h::Index;pub use crate::src::headers::sqliteInt_h::IndexedExpr;pub use crate::src::headers::sqliteInt_h::KeyInfo;pub use crate::src::headers::sqliteInt_h::LogEst;pub use crate::src::headers::sqliteInt_h::Lookaside;pub use crate::src::headers::sqliteInt_h::LookasideSlot;pub use crate::src::headers::sqliteInt_h::Module;pub use crate::src::headers::sqliteInt_h::Parse;pub use crate::src::headers::sqliteInt_h::ParseCleanup;pub use crate::src::headers::vdbeInt_h::PreUpdate;pub use crate::src::headers::sqliteInt_h::RenameToken;pub use crate::src::headers::sqliteInt_h::Returning;pub use crate::src::headers::sqliteInt_h::Savepoint;pub use crate::src::headers::sqliteInt_h::Schema;pub use crate::src::headers::sqliteInt_h::Select;pub use crate::src::headers::sqliteInt_h::Sqlite3Config;pub use crate::src::headers::sqliteInt_h::SrcItem;pub use crate::src::headers::sqliteInt_h::SrcList;pub use crate::src::headers::sqliteInt_h::StrAccum;pub use crate::src::headers::sqliteInt_h::Subquery;pub use crate::src::headers::sqliteInt_h::Table;pub use crate::src::headers::sqliteInt_h::TableLock;pub use crate::src::headers::sqliteInt_h::Token;pub use crate::src::headers::sqliteInt_h::Trigger;pub use crate::src::headers::sqliteInt_h::TriggerPrg;pub use crate::src::headers::sqliteInt_h::TriggerStep;pub use crate::src::headers::sqliteInt_h::Upsert;pub use crate::src::headers::sqliteInt_h::VList;pub use crate::src::headers::sqliteInt_h::VTable;pub use crate::src::headers::sqliteInt_h::VtabCtx;pub use crate::src::headers::sqliteInt_h::Window;pub use crate::src::headers::sqliteInt_h::With;pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN;pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT;pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_SLOCHNG;
pub use crate::src::headers::stdlib::int16_t;
pub use crate::src::headers::stdlib::uint16_t;pub use crate::src::headers::stdlib::uint32_t;pub use crate::src::headers::stdlib::uint8_t;

pub use ::libc::tm;

pub use crate::src::headers::stdlib::time_t;

pub use crate::src::headers::stdlib::__int16_t;pub use crate::src::headers::stdlib::__time_t;pub use crate::src::headers::stdlib::__uint16_t;pub use crate::src::headers::stdlib::__uint32_t;pub use crate::src::headers::stdlib::__uint8_t;pub use crate::src::src::vdbe::p4union;pub use crate::src::src::vdbeaux::sqlite3NotPureFunc;pub use crate::src::src::vdbe::Mem;pub use crate::src::src::vdbe::SubProgram;pub use crate::src::src::vdbe::SubrtnSig;pub use crate::src::headers::vdbeInt_h::Vdbe;pub use crate::src::src::vdbe::VdbeOp;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]

pub struct DateTime {
    pub iJD: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub Y: ::core::ffi::c_int,
    pub M: ::core::ffi::c_int,
    pub D: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
    pub m: ::core::ffi::c_int,
    pub tz: ::core::ffi::c_int,
    pub s: ::core::ffi::c_double,
    pub validJD: ::core::ffi::c_char,
    pub validYMD: ::core::ffi::c_char,
    pub validHMS: ::core::ffi::c_char,
    pub nFloor: ::core::ffi::c_char,
    #[bitfield(name = "rawS", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "isError", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "useSubsec", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "isUtc", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "isLocal", ty = "::core::ffi::c_uint", bits = "4..=4")]
    pub rawS_isError_useSubsec_isUtc_isLocal: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2RustUnnamed {
    pub nName: crate::src::ext::rtree::rtree::u8_0,
    pub zName: [::core::ffi::c_char; 7],
    pub rLimit: ::core::ffi::c_float,
    pub rXform: ::core::ffi::c_float,
}

unsafe extern "C" fn parseTimezone(
    mut zDate: *const ::core::ffi::c_char,
    mut p: *mut DateTime,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut sgn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nHr: ::core::ffi::c_int = 0;
    let mut nMn: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    while *(&raw const crate::src::src::global::sqlite3CtypeMap as *const ::core::ffi::c_uchar)
        .offset(*zDate as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & 0x1 as ::core::ffi::c_int
        != 0
    {
        zDate = zDate.offset(1);
    }
    (*p).tz = 0 as ::core::ffi::c_int;
    c = *zDate as ::core::ffi::c_int;
    if c == '-' as i32 {
        sgn = -(1 as ::core::ffi::c_int);
        current_block = 3512920355445576850;
    } else if c == '+' as i32 {
        sgn = 1 as ::core::ffi::c_int;
        current_block = 3512920355445576850;
    } else {
        if c == 'Z' as i32 || c == 'z' as i32 {
            zDate = zDate.offset(1);
            (*p).set_isLocal(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*p).set_isUtc(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        } else {
            return (c != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
        current_block = 11362447018299814777;
    }
    match current_block {
        3512920355445576850 => {
            zDate = zDate.offset(1);
            if getDigits(
                zDate,
                b"20b:20e\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut nHr,
                &raw mut nMn,
            ) != 2 as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            zDate = zDate.offset(5 as isize);
            (*p).tz = sgn * (nMn + nHr * 60 as ::core::ffi::c_int);
            if (*p).tz == 0 as ::core::ffi::c_int {
                (*p).set_isLocal(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*p).set_isUtc(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
        _ => {}
    }
    while *(&raw const crate::src::src::global::sqlite3CtypeMap as *const ::core::ffi::c_uchar)
        .offset(*zDate as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & 0x1 as ::core::ffi::c_int
        != 0
    {
        zDate = zDate.offset(1);
    }
    (*zDate as ::core::ffi::c_int != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
}

unsafe extern "C" fn parseHhMmSs(
    mut zDate: *const ::core::ffi::c_char,
    mut p: *mut DateTime,
) -> ::core::ffi::c_int {
    let mut h: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    let mut ms: ::core::ffi::c_double = 0.0f64;
    if getDigits(
        zDate,
        b"20c:20e\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut h,
        &raw mut m,
    ) != 2 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    zDate = zDate.offset(5 as isize);
    if *zDate as ::core::ffi::c_int == ':' as i32 {
        zDate = zDate.offset(1);
        if getDigits(
            zDate,
            b"20e\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut s,
        ) != 1 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        zDate = zDate.offset(2 as isize);
        if *zDate as ::core::ffi::c_int == '.' as i32
            && *(&raw const crate::src::src::global::sqlite3CtypeMap as *const ::core::ffi::c_uchar).offset(
                *zDate.offset(1 as isize) as ::core::ffi::c_uchar as isize,
            ) as ::core::ffi::c_int
                & 0x4 as ::core::ffi::c_int
                != 0
        {
            let mut rScale: ::core::ffi::c_double = 1.0f64;
            zDate = zDate.offset(1);
            while *(&raw const crate::src::src::global::sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                .offset(*zDate as ::core::ffi::c_uchar as isize)
                as ::core::ffi::c_int
                & 0x4 as ::core::ffi::c_int
                != 0
            {
                ms = ms * 10.0f64 + *zDate as ::core::ffi::c_int as ::core::ffi::c_double
                    - '0' as i32 as ::core::ffi::c_double;
                rScale *= 10.0f64;
                zDate = zDate.offset(1);
            }
            ms /= rScale;
            if ms > 0.999f64 {
                ms = 0.999f64;
            }
        }
    } else {
        s = 0 as ::core::ffi::c_int;
    }
    let __p_ref = { &mut *p };
    __p_ref.validJD = 0 as ::core::ffi::c_char;
    __p_ref.set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    __p_ref.validHMS = 1 as ::core::ffi::c_char;
    __p_ref.h = h;
    __p_ref.m = m;
    __p_ref.s = s as ::core::ffi::c_double + ms;
    if parseTimezone(zDate, p) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn datetimeError(mut p: *mut DateTime) {
    ::libc::memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<DateTime>() as crate::__stddef_size_t_h::size_t,
    );
    (*p).set_isError(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}

unsafe extern "C" fn computeJD(mut p: *mut DateTime) {
    let mut Y: ::core::ffi::c_int = 0;
    let mut M: ::core::ffi::c_int = 0;
    let mut D: ::core::ffi::c_int = 0;
    let mut A: ::core::ffi::c_int = 0;
    let mut B: ::core::ffi::c_int = 0;
    let mut X1: ::core::ffi::c_int = 0;
    let mut X2: ::core::ffi::c_int = 0;
    let __p_ref = { &mut *p };
    if __p_ref.validJD != 0 {
        return;
    }
    if __p_ref.validYMD != 0 {
        Y = __p_ref.Y;
        M = __p_ref.M;
        D = __p_ref.D;
    } else {
        Y = 2000 as ::core::ffi::c_int;
        M = 1 as ::core::ffi::c_int;
        D = 1 as ::core::ffi::c_int;
    }
    if Y < -(4713 as ::core::ffi::c_int)
        || Y > 9999 as ::core::ffi::c_int
        || __p_ref.rawS() as ::core::ffi::c_int != 0
    {
        datetimeError(p);
        return;
    }
    if M <= 2 as ::core::ffi::c_int {
        Y -= 1;
        M += 12 as ::core::ffi::c_int;
    }
    A = (Y + 4800 as ::core::ffi::c_int) / 100 as ::core::ffi::c_int;
    B = 38 as ::core::ffi::c_int - A + A / 4 as ::core::ffi::c_int;
    X1 = 36525 as ::core::ffi::c_int * (Y + 4716 as ::core::ffi::c_int) / 100 as ::core::ffi::c_int;
    X2 = 306001 as ::core::ffi::c_int * (M + 1 as ::core::ffi::c_int) / 10000 as ::core::ffi::c_int;
    __p_ref.iJD = (((X1 + X2 + D + B) as ::core::ffi::c_double - 1524.5f64)
        * 86400000 as ::core::ffi::c_int as ::core::ffi::c_double) as crate::src::headers::sqlite3_h::sqlite3_int64;
    __p_ref.validJD = 1 as ::core::ffi::c_char;
    if __p_ref.validHMS != 0 {
        __p_ref.iJD += (__p_ref.h * 3600000 as ::core::ffi::c_int + __p_ref.m * 60000 as ::core::ffi::c_int)
            as crate::src::headers::sqlite3_h::sqlite3_int64
            + (__p_ref.s * 1000 as ::core::ffi::c_int as ::core::ffi::c_double + 0.5f64)
                as crate::src::headers::sqlite3_h::sqlite3_int64;
        if __p_ref.tz != 0 {
            __p_ref.iJD -= (__p_ref.tz * 60000 as ::core::ffi::c_int) as crate::src::headers::sqlite3_h::sqlite3_int64;
            __p_ref.validYMD = 0 as ::core::ffi::c_char;
            __p_ref.validHMS = 0 as ::core::ffi::c_char;
            __p_ref.tz = 0 as ::core::ffi::c_int;
            __p_ref.set_isUtc(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            __p_ref.set_isLocal(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
}

unsafe extern "C" fn computeFloor(mut p: *mut DateTime) {
    let __p_ref = { &mut *p };
    if __p_ref.D <= 28 as ::core::ffi::c_int {
        __p_ref.nFloor = 0 as ::core::ffi::c_char;
    } else if (1 as ::core::ffi::c_int) << __p_ref.M & 0x15aa as ::core::ffi::c_int != 0 {
        __p_ref.nFloor = 0 as ::core::ffi::c_char;
    } else if __p_ref.M != 2 as ::core::ffi::c_int {
        __p_ref.nFloor =
            (__p_ref.D == 31 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_char;
    } else if __p_ref.Y % 4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        || __p_ref.Y % 100 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && __p_ref.Y % 400 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        __p_ref.nFloor = (__p_ref.D - 28 as ::core::ffi::c_int) as ::core::ffi::c_char;
    } else {
        __p_ref.nFloor = (__p_ref.D - 29 as ::core::ffi::c_int) as ::core::ffi::c_char;
    };
}

unsafe extern "C" fn parseYyyyMmDd(
    mut zDate: *const ::core::ffi::c_char,
    mut p: *mut DateTime,
) -> ::core::ffi::c_int {
    let mut Y: ::core::ffi::c_int = 0;
    let mut M: ::core::ffi::c_int = 0;
    let mut D: ::core::ffi::c_int = 0;
    let mut neg: ::core::ffi::c_int = 0;
    if *zDate.offset(0 as isize) as ::core::ffi::c_int == '-' as i32 {
        zDate = zDate.offset(1);
        neg = 1 as ::core::ffi::c_int;
    } else {
        neg = 0 as ::core::ffi::c_int;
    }
    if getDigits(
        zDate,
        b"40f-21a-21d\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut Y,
        &raw mut M,
        &raw mut D,
    ) != 3 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    zDate = zDate.offset(10 as isize);
    while *(&raw const crate::src::src::global::sqlite3CtypeMap as *const ::core::ffi::c_uchar)
        .offset(*zDate as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & 0x1 as ::core::ffi::c_int
        != 0
        || 'T' as i32 == *(zDate as *mut crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_int
    {
        zDate = zDate.offset(1);
    }
    let __p_ref = { &mut *p };
    if !(parseHhMmSs(zDate, p) == 0 as ::core::ffi::c_int) {
        if *zDate as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            __p_ref.validHMS = 0 as ::core::ffi::c_char;
        } else {
            return 1 as ::core::ffi::c_int;
        }
    }
    __p_ref.validJD = 0 as ::core::ffi::c_char;
    __p_ref.validYMD = 1 as ::core::ffi::c_char;
    __p_ref.Y = if neg != 0 { -Y } else { Y };
    __p_ref.M = M;
    __p_ref.D = D;
    computeFloor(p);
    if __p_ref.tz != 0 {
        computeJD(p);
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn setDateTimeToCurrent(
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut p: *mut DateTime,
) -> ::core::ffi::c_int {
    (*p).iJD = crate::src::src::vdbeapi::sqlite3StmtCurrentTime(context);
    if (*p).iJD > 0 as crate::src::headers::sqlite3_h::sqlite3_int64 {
        let __p_ref = { &mut *p };
        __p_ref.validJD = 1 as ::core::ffi::c_char;
        __p_ref.set_isUtc(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        __p_ref.set_isLocal(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        clearYMD_HMS_TZ(p);
        return 0 as ::core::ffi::c_int;
    } else {
        return 1 as ::core::ffi::c_int;
    };
}

unsafe extern "C" fn setRawDateNumber(mut p: *mut DateTime, mut r: ::core::ffi::c_double) {
    (*p).s = r;
    (*p).set_rawS(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if r >= 0.0f64 && r < 5373484.5f64 {
        (*p).iJD = (r * 86400000.0f64 + 0.5f64) as crate::src::headers::sqlite3_h::sqlite3_int64;
        (*p).validJD = 1 as ::core::ffi::c_char;
    }
}

unsafe extern "C" fn parseDateOrTime(
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut zDate: *const ::core::ffi::c_char,
    mut p: *mut DateTime,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_double = 0.;
    if parseYyyyMmDd(zDate, p) == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    } else if parseHhMmSs(zDate, p) == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    } else if crate::src::src::util::sqlite3StrICmp(zDate, b"now\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        && crate::src::src::vdbeaux::sqlite3NotPureFunc(context) != 0
    {
        return setDateTimeToCurrent(context, p);
    } else if crate::src::src::util::sqlite3AtoF(
        zDate,
        &raw mut r,
        crate::src::src::util::sqlite3Strlen30(zDate),
        crate::src::headers::sqlite3_h::SQLITE_UTF8 as crate::src::ext::rtree::rtree::u8_0,
    ) > 0 as ::core::ffi::c_int
    {
        setRawDateNumber(p, r);
        return 0 as ::core::ffi::c_int;
    } else if (crate::src::src::util::sqlite3StrICmp(
        zDate,
        b"subsec\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || crate::src::src::util::sqlite3StrICmp(
            zDate,
            b"subsecond\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        && crate::src::src::vdbeaux::sqlite3NotPureFunc(context) != 0
    {
        (*p).set_useSubsec(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        return setDateTimeToCurrent(context, p);
    }
    1 as ::core::ffi::c_int
}

pub const INT_464269060799999: crate::src::ext::rtree::rtree::i64_0 =
    (0x1a640 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0) << 32 as ::core::ffi::c_int | 0x1072fdff as crate::src::ext::rtree::rtree::i64_0;

unsafe extern "C" fn validJulianDay(mut iJD: crate::src::headers::sqlite3_h::sqlite3_int64) -> ::core::ffi::c_int {
    (iJD >= 0 as crate::src::headers::sqlite3_h::sqlite3_int64 && iJD <= INT_464269060799999) as ::core::ffi::c_int
}

unsafe extern "C" fn computeYMD(mut p: *mut DateTime) {
    let mut Z: ::core::ffi::c_int = 0;
    let mut alpha: ::core::ffi::c_int = 0;
    let mut A: ::core::ffi::c_int = 0;
    let mut B: ::core::ffi::c_int = 0;
    let mut C: ::core::ffi::c_int = 0;
    let mut D: ::core::ffi::c_int = 0;
    let mut E: ::core::ffi::c_int = 0;
    let mut X1: ::core::ffi::c_int = 0;
    let __p_ref = { &mut *p };
    if __p_ref.validYMD != 0 {
        return;
    }
    if __p_ref.validJD == 0 {
        __p_ref.Y = 2000 as ::core::ffi::c_int;
        __p_ref.M = 1 as ::core::ffi::c_int;
        __p_ref.D = 1 as ::core::ffi::c_int;
    } else if validJulianDay(__p_ref.iJD) == 0 {
        datetimeError(p);
        return;
    } else {
        Z = ((__p_ref.iJD + 43200000 as crate::src::headers::sqlite3_h::sqlite3_int64) / 86400000 as crate::src::headers::sqlite3_h::sqlite3_int64)
            as ::core::ffi::c_int;
        alpha = ((Z as ::core::ffi::c_double + 32044.75f64) / 36524.25f64) as ::core::ffi::c_int
            - 52 as ::core::ffi::c_int;
        A = Z + 1 as ::core::ffi::c_int + alpha
            - (alpha + 100 as ::core::ffi::c_int) / 4 as ::core::ffi::c_int
            + 25 as ::core::ffi::c_int;
        B = A + 1524 as ::core::ffi::c_int;
        C = ((B as ::core::ffi::c_double - 122.1f64) / 365.25f64) as ::core::ffi::c_int;
        D = 36525 as ::core::ffi::c_int * (C & 32767 as ::core::ffi::c_int)
            / 100 as ::core::ffi::c_int;
        E = ((B - D) as ::core::ffi::c_double / 30.6001f64) as ::core::ffi::c_int;
        X1 = (30.6001f64 * E as ::core::ffi::c_double) as ::core::ffi::c_int;
        __p_ref.D = B - D - X1;
        __p_ref.M = if E < 14 as ::core::ffi::c_int {
            E - 1 as ::core::ffi::c_int
        } else {
            E - 13 as ::core::ffi::c_int
        };
        __p_ref.Y = if __p_ref.M > 2 as ::core::ffi::c_int {
            C - 4716 as ::core::ffi::c_int
        } else {
            C - 4715 as ::core::ffi::c_int
        };
    }
    __p_ref.validYMD = 1 as ::core::ffi::c_char;
}

unsafe extern "C" fn computeHMS(mut p: *mut DateTime) {
    let mut day_ms: ::core::ffi::c_int = 0;
    let mut day_min: ::core::ffi::c_int = 0;
    let __p_ref = { &mut *p };
    if __p_ref.validHMS != 0 {
        return;
    }
    computeJD(p);
    day_ms =
        ((__p_ref.iJD + 43200000 as crate::src::headers::sqlite3_h::sqlite3_int64) % 86400000 as crate::src::headers::sqlite3_h::sqlite3_int64) as ::core::ffi::c_int;
    __p_ref.s = (day_ms % 60000 as ::core::ffi::c_int) as ::core::ffi::c_double / 1000.0f64;
    day_min = day_ms / 60000 as ::core::ffi::c_int;
    __p_ref.m = day_min % 60 as ::core::ffi::c_int;
    __p_ref.h = day_min / 60 as ::core::ffi::c_int;
    __p_ref.set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    __p_ref.validHMS = 1 as ::core::ffi::c_char;
}

unsafe extern "C" fn computeYMD_HMS(mut p: *mut DateTime) {
    computeYMD(p);
    computeHMS(p);
}

unsafe extern "C" fn clearYMD_HMS_TZ(mut p: *mut DateTime) {
    let __p_ref = { &mut *p };
    __p_ref.validYMD = 0 as ::core::ffi::c_char;
    __p_ref.validHMS = 0 as ::core::ffi::c_char;
    __p_ref.tz = 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn osLocaltime(mut t: *mut crate::src::headers::stdlib::time_t, mut pTm: *mut ::libc::tm) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    if crate::src::src::global::sqlite3Config.bLocaltimeFault != 0 {
        if crate::src::src::global::sqlite3Config.xAltLocaltime.is_some() {
            return crate::src::src::global::sqlite3Config
                .xAltLocaltime
                .expect("non-null function pointer")(
                t as *const ::core::ffi::c_void,
                pTm as *mut ::core::ffi::c_void,
            );
        } else {
            return 1 as ::core::ffi::c_int;
        }
    }
    rc = (::libc::localtime_r(t,  pTm as *mut ::libc::tm) as *mut ::libc::tm == ::core::ptr::null_mut::<::libc::tm>()) as ::core::ffi::c_int;
    rc
}

unsafe extern "C" fn toLocaltime(
    mut p: *mut DateTime,
    mut pCtx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
) -> ::core::ffi::c_int {
    let mut t: crate::src::headers::stdlib::time_t = 0;
    let mut sLocal: ::libc::tm = { ::core::mem::zeroed() };
    let mut iYearDiff: ::core::ffi::c_int = 0;
    computeJD(p);
    let __p_ref = { &mut *p };
    if __p_ref.iJD < 2108667600 as crate::src::ext::rtree::rtree::i64_0 * 100000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0
        || __p_ref.iJD > 2130141456 as crate::src::ext::rtree::rtree::i64_0 * 100000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0
    {
        let mut x: DateTime = *p;
        computeYMD_HMS(&raw mut x);
        iYearDiff = 2000 as ::core::ffi::c_int + x.Y % 4 as ::core::ffi::c_int - x.Y;
        x.Y += iYearDiff;
        x.validJD = 0 as ::core::ffi::c_char;
        computeJD(&raw mut x);
        t = (x.iJD as crate::src::ext::rtree::rtree::i64_0 / 1000 as crate::src::ext::rtree::rtree::i64_0
            - 21086676 as crate::src::ext::rtree::rtree::i64_0 * 10000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0) as crate::src::headers::stdlib::time_t;
    } else {
        iYearDiff = 0 as ::core::ffi::c_int;
        t = (__p_ref.iJD as crate::src::ext::rtree::rtree::i64_0 / 1000 as crate::src::ext::rtree::rtree::i64_0
            - 21086676 as crate::src::ext::rtree::rtree::i64_0 * 10000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0) as crate::src::headers::stdlib::time_t;
    }
    if osLocaltime(&raw mut t, &raw mut sLocal) != 0 {
        crate::src::src::vdbeapi::sqlite3_result_error(
            pCtx,
            b"local time unavailable\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int),
        );
        return crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    __p_ref.Y = sLocal.tm_year + 1900 as ::core::ffi::c_int - iYearDiff;
    __p_ref.M = sLocal.tm_mon + 1 as ::core::ffi::c_int;
    __p_ref.D = sLocal.tm_mday;
    __p_ref.h = sLocal.tm_hour;
    __p_ref.m = sLocal.tm_min;
    __p_ref.s = sLocal.tm_sec as ::core::ffi::c_double
        + (__p_ref.iJD % 1000 as crate::src::headers::sqlite3_h::sqlite3_int64) as ::core::ffi::c_double * 0.001f64;
    __p_ref.validYMD = 1 as ::core::ffi::c_char;
    __p_ref.validHMS = 1 as ::core::ffi::c_char;
    __p_ref.validJD = 0 as ::core::ffi::c_char;
    __p_ref.set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    __p_ref.tz = 0 as ::core::ffi::c_int;
    __p_ref.set_isError(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

static mut aXformType: [C2RustUnnamed; 6] = {
    [
        C2RustUnnamed {
            nName: 6 as crate::src::ext::rtree::rtree::u8_0,
            zName: unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"second\0") },
            rLimit: 4.6427e+14f32,
            rXform: 1.0f32,
        },
        C2RustUnnamed {
            nName: 6 as crate::src::ext::rtree::rtree::u8_0,
            zName: unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"minute\0") },
            rLimit: 7.7379e+12f32,
            rXform: 60.0f32,
        },
        C2RustUnnamed {
            nName: 4 as crate::src::ext::rtree::rtree::u8_0,
            zName: unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"hour\0\0\0") },
            rLimit: 1.2897e+11f32,
            rXform: 3600.0f32,
        },
        C2RustUnnamed {
            nName: 3 as crate::src::ext::rtree::rtree::u8_0,
            zName: unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"day\0\0\0\0") },
            rLimit: 5373485.0f32,
            rXform: 86400.0f32,
        },
        C2RustUnnamed {
            nName: 5 as crate::src::ext::rtree::rtree::u8_0,
            zName: unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"month\0\0") },
            rLimit: 176546.0f32,
            rXform: 2592000.0f32,
        },
        C2RustUnnamed {
            nName: 4 as crate::src::ext::rtree::rtree::u8_0,
            zName: unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"year\0\0\0") },
            rLimit: 14713.0f32,
            rXform: 31536000.0f32,
        },
    ]
};

unsafe extern "C" fn autoAdjustDate(mut p: *mut DateTime) {
    let __p_ref = { &mut *p };
    if __p_ref.rawS() == 0 || __p_ref.validJD as ::core::ffi::c_int != 0 {
        __p_ref.set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else if __p_ref.s
        >= (-(21086676 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0 * 10000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0)
            as ::core::ffi::c_double
        && __p_ref.s
            <= (25340230 as crate::src::ext::rtree::rtree::i64_0 * 10000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0 + 799 as crate::src::ext::rtree::rtree::i64_0)
                as ::core::ffi::c_double
    {
        let mut r: ::core::ffi::c_double = __p_ref.s * 1000.0f64 + 210866760000000.0f64;
        clearYMD_HMS_TZ(p);
        __p_ref.iJD = (r + 0.5f64) as crate::src::headers::sqlite3_h::sqlite3_int64;
        __p_ref.validJD = 1 as ::core::ffi::c_char;
        __p_ref.set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}

unsafe extern "C" fn parseModifier(
    mut pCtx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut p: *mut DateTime,
    mut idx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut r: ::core::ffi::c_double = 0.;
    let mut current_block_175: u64;
    match *(&raw const crate::src::src::global::sqlite3UpperToLower as *const ::core::ffi::c_uchar)
        .offset(*z.offset(0 as isize) as crate::src::ext::rtree::rtree::u8_0 as isize)
        as ::core::ffi::c_int
    {
        97 => {
            if crate::src::src::util::sqlite3_stricmp(z, b"auto\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                if idx > 1 as ::core::ffi::c_int {
                    return 1 as ::core::ffi::c_int;
                }
                autoAdjustDate(p);
                rc = 0 as ::core::ffi::c_int;
            }
        }
        99 => {
            if crate::src::src::util::sqlite3_stricmp(z, b"ceiling\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                computeJD(p);
                clearYMD_HMS_TZ(p);
                rc = 0 as ::core::ffi::c_int;
                (*p).nFloor = 0 as ::core::ffi::c_char;
            }
        }
        102 => {
            if crate::src::src::util::sqlite3_stricmp(z, b"floor\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                computeJD(p);
                (*p).iJD -= ((*p).nFloor as ::core::ffi::c_int * 86400000 as ::core::ffi::c_int)
                    as crate::src::headers::sqlite3_h::sqlite3_int64;
                clearYMD_HMS_TZ(p);
                rc = 0 as ::core::ffi::c_int;
            }
        }
        106 => {
            if crate::src::src::util::sqlite3_stricmp(z, b"julianday\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                if idx > 1 as ::core::ffi::c_int {
                    return 1 as ::core::ffi::c_int;
                }
                if (*p).validJD as ::core::ffi::c_int != 0 && (*p).rawS() as ::core::ffi::c_int != 0
                {
                    rc = 0 as ::core::ffi::c_int;
                    (*p).set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
            }
        }
        108 => {
            if crate::src::src::util::sqlite3_stricmp(z, b"localtime\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                && crate::src::src::vdbeaux::sqlite3NotPureFunc(pCtx) != 0
            {
                let __p_ref = { &mut *p };
                rc = if __p_ref.isLocal() as ::core::ffi::c_int != 0 {
                    crate::src::headers::sqlite3_h::SQLITE_OK
                } else {
                    toLocaltime(p, pCtx)
                };
                __p_ref.set_isUtc(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                __p_ref.set_isLocal(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
        117 => {
            if crate::src::src::util::sqlite3_stricmp(z, b"unixepoch\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                && (*p).rawS() as ::core::ffi::c_int != 0
            {
                if idx > 1 as ::core::ffi::c_int {
                    return 1 as ::core::ffi::c_int;
                }
                r = (*p).s * 1000.0f64 + 210866760000000.0f64;
                if r >= 0.0f64 && r < 464269060800000.0f64 {
                    clearYMD_HMS_TZ(p);
                    let __p_ref = { &mut *p };
                    __p_ref.iJD = (r + 0.5f64) as crate::src::headers::sqlite3_h::sqlite3_int64;
                    __p_ref.validJD = 1 as ::core::ffi::c_char;
                    __p_ref.set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    rc = 0 as ::core::ffi::c_int;
                }
            } else if crate::src::src::util::sqlite3_stricmp(z, b"utc\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                && crate::src::src::vdbeaux::sqlite3NotPureFunc(pCtx) != 0
            {
                if (*p).isUtc() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    let mut iOrigJD: crate::src::ext::rtree::rtree::i64_0 = 0;
                    let mut iGuess: crate::src::ext::rtree::rtree::i64_0 = 0;
                    let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut iErr: crate::src::ext::rtree::rtree::i64_0 = 0;
                    computeJD(p);
                    let __p_ref = { &mut *p };
                    iOrigJD = __p_ref.iJD as crate::src::ext::rtree::rtree::i64_0;
                    iGuess = iOrigJD;
                    iErr = 0 as crate::src::ext::rtree::rtree::i64_0;
                    loop {
                        let mut new: DateTime = { ::core::mem::zeroed() };
                        iGuess -= iErr;
                        new.iJD = iGuess as crate::src::headers::sqlite3_h::sqlite3_int64;
                        new.validJD = 1 as ::core::ffi::c_char;
                        rc = toLocaltime(&raw mut new, pCtx);
                        if rc != 0 {
                            return rc;
                        }
                        computeJD(&raw mut new);
                        iErr = new.iJD as crate::src::ext::rtree::rtree::i64_0 - iOrigJD;
                        if !(iErr != 0 && {
                            let fresh0 = cnt;
                            cnt += 1;
                            fresh0 < 3 as ::core::ffi::c_int
                        }) {
                            break;
                        }
                    }
                    ::libc::memset(
                        p as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<DateTime>() as crate::__stddef_size_t_h::size_t,
                    );
                    __p_ref.iJD = iGuess as crate::src::headers::sqlite3_h::sqlite3_int64;
                    __p_ref.validJD = 1 as ::core::ffi::c_char;
                    __p_ref.set_isUtc(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    __p_ref.set_isLocal(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
                rc = crate::src::headers::sqlite3_h::SQLITE_OK;
            }
        }
        119 => {
            if crate::src::src::util::sqlite3_strnicmp(
                z,
                b"weekday \0" as *const u8 as *const ::core::ffi::c_char,
                8 as ::core::ffi::c_int,
            ) == 0 as ::core::ffi::c_int
                && crate::src::src::util::sqlite3AtoF(
                    z.offset(8 as isize) as *const ::core::ffi::c_char,
                    &raw mut r,
                    crate::src::src::util::sqlite3Strlen30(
                        z.offset(8 as isize) as *const ::core::ffi::c_char
                    ),
                    crate::src::headers::sqlite3_h::SQLITE_UTF8 as crate::src::ext::rtree::rtree::u8_0,
                ) > 0 as ::core::ffi::c_int
                && r >= 0.0f64
                && r < 7.0f64
                && {
                    n = r as ::core::ffi::c_int;
                    n as ::core::ffi::c_double == r
                }
            {
                let mut Z: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
                computeYMD_HMS(p);
                let __p_ref = { &mut *p };
                __p_ref.tz = 0 as ::core::ffi::c_int;
                __p_ref.validJD = 0 as ::core::ffi::c_char;
                computeJD(p);
                Z = (__p_ref.iJD + 129600000 as crate::src::headers::sqlite3_h::sqlite3_int64) / 86400000 as crate::src::headers::sqlite3_h::sqlite3_int64
                    % 7 as crate::src::headers::sqlite3_h::sqlite3_int64;
                if Z > n as crate::src::headers::sqlite3_h::sqlite3_int64 {
                    Z -= 7 as crate::src::headers::sqlite3_h::sqlite3_int64;
                }
                __p_ref.iJD += (n as crate::src::headers::sqlite3_h::sqlite3_int64 - Z) * 86400000 as crate::src::headers::sqlite3_h::sqlite3_int64;
                clearYMD_HMS_TZ(p);
                rc = 0 as ::core::ffi::c_int;
            }
        }
        115 => {
            let __p_ref = { &mut *p };
            if crate::src::src::util::sqlite3_strnicmp(
                z,
                b"start of \0" as *const u8 as *const ::core::ffi::c_char,
                9 as ::core::ffi::c_int,
            ) != 0 as ::core::ffi::c_int
            {
                if crate::src::src::util::sqlite3_stricmp(z, b"subsec\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || crate::src::src::util::sqlite3_stricmp(z, b"subsecond\0" as *const u8 as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                {
                    __p_ref.set_useSubsec(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    rc = 0 as ::core::ffi::c_int;
                }
            } else if !(__p_ref.validJD == 0 && __p_ref.validYMD == 0 && __p_ref.validHMS == 0) {
                z = z.offset(9 as isize);
                computeYMD(p);
                __p_ref.validHMS = 1 as ::core::ffi::c_char;
                __p_ref.m = 0 as ::core::ffi::c_int;
                __p_ref.h = __p_ref.m;
                __p_ref.s = 0.0f64;
                __p_ref.set_rawS(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                __p_ref.tz = 0 as ::core::ffi::c_int;
                __p_ref.validJD = 0 as ::core::ffi::c_char;
                if crate::src::src::util::sqlite3_stricmp(z, b"month\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    __p_ref.D = 1 as ::core::ffi::c_int;
                    rc = 0 as ::core::ffi::c_int;
                } else if crate::src::src::util::sqlite3_stricmp(z, b"year\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    __p_ref.M = 1 as ::core::ffi::c_int;
                    __p_ref.D = 1 as ::core::ffi::c_int;
                    rc = 0 as ::core::ffi::c_int;
                } else if crate::src::src::util::sqlite3_stricmp(z, b"day\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    rc = 0 as ::core::ffi::c_int;
                }
            }
        }
        43 | 45 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            let mut rRounder: ::core::ffi::c_double = 0.;
            let mut i: ::core::ffi::c_int = 0;
            let mut Y: ::core::ffi::c_int = 0;
            let mut M: ::core::ffi::c_int = 0;
            let mut D: ::core::ffi::c_int = 0;
            let mut h: ::core::ffi::c_int = 0;
            let mut m: ::core::ffi::c_int = 0;
            let mut x: ::core::ffi::c_int = 0;
            let mut z2: *const ::core::ffi::c_char = z;
            let mut z0: ::core::ffi::c_char = *z.offset(0 as isize);
            n = 1 as ::core::ffi::c_int;
            while *z.offset(n as isize) != 0 {
                if *z.offset(n as isize) as ::core::ffi::c_int == ':' as i32 {
                    break;
                }
                if *(&raw const crate::src::src::global::sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                    .offset(*z.offset(n as isize) as ::core::ffi::c_uchar as isize)
                    as ::core::ffi::c_int
                    & 0x1 as ::core::ffi::c_int
                    != 0
                {
                    break;
                }
                if *z.offset(n as isize) as ::core::ffi::c_int == '-' as i32 {
                    if n == 5 as ::core::ffi::c_int
                        && getDigits(
                            z.offset(1 as isize)
                                as *const ::core::ffi::c_char,
                            b"40f\0" as *const u8 as *const ::core::ffi::c_char,
                            &raw mut Y,
                        ) == 1 as ::core::ffi::c_int
                    {
                        break;
                    }
                    if n == 6 as ::core::ffi::c_int
                        && getDigits(
                            z.offset(1 as isize)
                                as *const ::core::ffi::c_char,
                            b"50f\0" as *const u8 as *const ::core::ffi::c_char,
                            &raw mut Y,
                        ) == 1 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                n += 1;
            }
            if !(crate::src::src::util::sqlite3AtoF(z, &raw mut r, n, crate::src::headers::sqlite3_h::SQLITE_UTF8 as crate::src::ext::rtree::rtree::u8_0) <= 0 as ::core::ffi::c_int) {
                if *z.offset(n as isize) as ::core::ffi::c_int == '-' as i32 {
                    if z0 as ::core::ffi::c_int != '+' as i32
                        && z0 as ::core::ffi::c_int != '-' as i32
                    {
                        current_block_175 = 2413388577390654262;
                    } else {
                        if n == 5 as ::core::ffi::c_int {
                            if getDigits(
                                z.offset(1 as isize)
                                    as *const ::core::ffi::c_char,
                                b"40f-20a-20d\0" as *const u8 as *const ::core::ffi::c_char,
                                &raw mut Y,
                                &raw mut M,
                                &raw mut D,
                            ) != 3 as ::core::ffi::c_int
                            {
                                current_block_175 = 2413388577390654262;
                            } else {
                                current_block_175 = 3024367268842933116;
                            }
                        } else if getDigits(
                            z.offset(1 as isize)
                                as *const ::core::ffi::c_char,
                            b"50f-20a-20d\0" as *const u8 as *const ::core::ffi::c_char,
                            &raw mut Y,
                            &raw mut M,
                            &raw mut D,
                        ) != 3 as ::core::ffi::c_int
                        {
                            current_block_175 = 2413388577390654262;
                        } else {
                            z = z.offset(1);
                            current_block_175 = 3024367268842933116;
                        }
                        match current_block_175 {
                            2413388577390654262 => {}
                            _ => {
                                if M >= 12 as ::core::ffi::c_int {
                                    current_block_175 = 2413388577390654262;
                                } else if D >= 31 as ::core::ffi::c_int {
                                    current_block_175 = 2413388577390654262;
                                } else {
                                    computeYMD_HMS(p);
                                    let __p_ref = { &mut *p };
                                    __p_ref.validJD = 0 as ::core::ffi::c_char;
                                    if z0 as ::core::ffi::c_int == '-' as i32 {
                                        __p_ref.Y -= Y;
                                        __p_ref.M -= M;
                                        D = -D;
                                    } else {
                                        __p_ref.Y += Y;
                                        __p_ref.M += M;
                                    }
                                    x = if __p_ref.M > 0 as ::core::ffi::c_int {
                                        (__p_ref.M - 1 as ::core::ffi::c_int)
                                            / 12 as ::core::ffi::c_int
                                    } else {
                                        (__p_ref.M - 12 as ::core::ffi::c_int)
                                            / 12 as ::core::ffi::c_int
                                    };
                                    __p_ref.Y += x;
                                    __p_ref.M -= x * 12 as ::core::ffi::c_int;
                                    computeFloor(p);
                                    computeJD(p);
                                    __p_ref.validHMS = 0 as ::core::ffi::c_char;
                                    __p_ref.validYMD = 0 as ::core::ffi::c_char;
                                    __p_ref.iJD += (D as crate::src::ext::rtree::rtree::i64_0 * 86400000 as crate::src::ext::rtree::rtree::i64_0) as crate::src::headers::sqlite3_h::sqlite_int64;
                                    if *z.offset(11 as isize)
                                        as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int
                                    {
                                        rc = 0 as ::core::ffi::c_int;
                                        current_block_175 = 2413388577390654262;
                                    } else if *(&raw const crate::src::src::global::sqlite3CtypeMap
                                        as *const ::core::ffi::c_uchar)
                                        .offset(*z.offset(11 as isize)
                                            as ::core::ffi::c_uchar
                                            as isize)
                                        as ::core::ffi::c_int
                                        & 0x1 as ::core::ffi::c_int
                                        != 0
                                        && getDigits(
                                            z.offset(12 as isize)
                                                as *const ::core::ffi::c_char,
                                            b"20c:20e\0" as *const u8 as *const ::core::ffi::c_char,
                                            &raw mut h,
                                            &raw mut m,
                                        ) == 2 as ::core::ffi::c_int
                                    {
                                        z2 = z.offset(12 as isize)
                                            as *const ::core::ffi::c_char;
                                        n = 2 as ::core::ffi::c_int;
                                        current_block_175 = 6897179874198677617;
                                    } else {
                                        current_block_175 = 2413388577390654262;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    current_block_175 = 6897179874198677617;
                }
                match current_block_175 {
                    2413388577390654262 => {}
                    _ => {
                        if *z2.offset(n as isize) as ::core::ffi::c_int == ':' as i32 {
                            let mut tx: DateTime = { ::core::mem::zeroed() };
                            let mut day: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
                            if *(&raw const crate::src::src::global::sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                                .offset(*z2 as ::core::ffi::c_uchar as isize)
                                as ::core::ffi::c_int
                                & 0x4 as ::core::ffi::c_int
                                == 0
                            {
                                z2 = z2.offset(1);
                            }
                            if !(parseHhMmSs(z2, &raw mut tx) != 0) {
                                computeJD(&raw mut tx);
                                tx.iJD -= 43200000 as crate::src::headers::sqlite3_h::sqlite3_int64;
                                day = tx.iJD / 86400000 as crate::src::headers::sqlite3_h::sqlite3_int64;
                                tx.iJD -= day * 86400000 as crate::src::headers::sqlite3_h::sqlite3_int64;
                                if z0 as ::core::ffi::c_int == '-' as i32 {
                                    tx.iJD = -tx.iJD;
                                }
                                computeJD(p);
                                clearYMD_HMS_TZ(p);
                                (*p).iJD += tx.iJD;
                                rc = 0 as ::core::ffi::c_int;
                            }
                        } else {
                            z = z.offset(n as isize);
                            while *(&raw const crate::src::src::global::sqlite3CtypeMap as *const ::core::ffi::c_uchar)
                                .offset(*z as ::core::ffi::c_uchar as isize)
                                as ::core::ffi::c_int
                                & 0x1 as ::core::ffi::c_int
                                != 0
                            {
                                z = z.offset(1);
                            }
                            n = crate::src::src::util::sqlite3Strlen30(z);
                            if !(n < 3 as ::core::ffi::c_int || n > 10 as ::core::ffi::c_int) {
                                if *(&raw const crate::src::src::global::sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                                    .offset(*z.offset((n - 1 as ::core::ffi::c_int) as isize)
                                        as crate::src::ext::rtree::rtree::u8_0
                                        as isize)
                                    as ::core::ffi::c_int
                                    == 's' as i32
                                {
                                    n -= 1;
                                }
                                computeJD(p);
                                rRounder = if r < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                                    -0.5f64
                                } else {
                                    0.5f64
                                };
                                (*p).nFloor = 0 as ::core::ffi::c_char;
                                i = 0 as ::core::ffi::c_int;
                                while i
                                    < (::core::mem::size_of::<[C2RustUnnamed; 6]>() as usize)
                                        .wrapping_div(
                                            ::core::mem::size_of::<C2RustUnnamed>() as usize
                                        )
                                        as ::core::ffi::c_int
                                {
                                    if aXformType[i as usize].nName as ::core::ffi::c_int == n
                                        && crate::src::src::util::sqlite3_strnicmp(
                                            &raw const (*(&raw const aXformType
                                                as *const C2RustUnnamed)
                                                .offset(i as isize))
                                            .zName
                                                as *const ::core::ffi::c_char,
                                            z,
                                            n,
                                        ) == 0 as ::core::ffi::c_int
                                        && r > -aXformType[i as usize].rLimit
                                            as ::core::ffi::c_double
                                        && r < aXformType[i as usize].rLimit
                                            as ::core::ffi::c_double
                                    {
                                        match i {
                                            4 => {
                                                computeYMD_HMS(p);
                                                let __p_ref = { &mut *p };
                                                __p_ref.M += r as ::core::ffi::c_int;
                                                x = if __p_ref.M > 0 as ::core::ffi::c_int {
                                                    (__p_ref.M - 1 as ::core::ffi::c_int)
                                                        / 12 as ::core::ffi::c_int
                                                } else {
                                                    (__p_ref.M - 12 as ::core::ffi::c_int)
                                                        / 12 as ::core::ffi::c_int
                                                };
                                                __p_ref.Y += x;
                                                __p_ref.M -= x * 12 as ::core::ffi::c_int;
                                                computeFloor(p);
                                                __p_ref.validJD = 0 as ::core::ffi::c_char;
                                                r -= r as ::core::ffi::c_int
                                                    as ::core::ffi::c_double;
                                            }
                                            5 => {
                                                let mut y: ::core::ffi::c_int =
                                                    r as ::core::ffi::c_int;
                                                computeYMD_HMS(p);
                                                (*p).Y += y;
                                                computeFloor(p);
                                                (*p).validJD = 0 as ::core::ffi::c_char;
                                                r -= r as ::core::ffi::c_int
                                                    as ::core::ffi::c_double;
                                            }
                                            _ => {}
                                        }
                                        computeJD(p);
                                        (*p).iJD += (r
                                            * 1000.0f64
                                            * aXformType[i as usize].rXform
                                                as ::core::ffi::c_double
                                            + rRounder)
                                            as crate::src::headers::sqlite3_h::sqlite3_int64;
                                        rc = 0 as ::core::ffi::c_int;
                                        break;
                                    } else {
                                        i += 1;
                                    }
                                }
                                clearYMD_HMS_TZ(p);
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    rc
}

unsafe extern "C" fn isDate(
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    mut p: *mut DateTime,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut z: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut eType: ::core::ffi::c_int = 0;
    ::libc::memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<DateTime>() as crate::__stddef_size_t_h::size_t,
    );
    if argc == 0 as ::core::ffi::c_int {
        if crate::src::src::vdbeaux::sqlite3NotPureFunc(context) == 0 {
            return 1 as ::core::ffi::c_int;
        }
        return setDateTimeToCurrent(context, p);
    }
    eType = crate::src::src::vdbeapi::sqlite3_value_type(*argv.offset(0 as isize));
    if eType == crate::src::headers::sqlite3_h::SQLITE_FLOAT || eType == crate::src::headers::sqlite3_h::SQLITE_INTEGER {
        setRawDateNumber(
            p,
            crate::src::src::vdbeapi::sqlite3_value_double(*argv.offset(0 as isize)),
        );
    } else {
        z = crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(0 as isize));
        if z.is_null() || parseDateOrTime(context, z as *mut ::core::ffi::c_char, p) != 0 {
            return 1 as ::core::ffi::c_int;
        }
    }
    i = 1 as ::core::ffi::c_int;
    while i < argc {
        z = crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(i as isize));
        n = crate::src::src::vdbeapi::sqlite3_value_bytes(*argv.offset(i as isize));
        if z.is_null() || parseModifier(context, z as *mut ::core::ffi::c_char, n, p, i) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    computeJD(p);
    let __p_ref = { &mut *p };
    if __p_ref.isError() as ::core::ffi::c_int != 0 || validJulianDay(__p_ref.iJD) == 0 {
        return 1 as ::core::ffi::c_int;
    }
    if argc == 1 as ::core::ffi::c_int
        && __p_ref.validYMD as ::core::ffi::c_int != 0
        && __p_ref.D > 28 as ::core::ffi::c_int
    {
        __p_ref.validYMD = 0 as ::core::ffi::c_char;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn juliandayFunc(
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let mut x: DateTime = DateTime {
        iJD: 0,
        Y: 0,
        M: 0,
        D: 0,
        h: 0,
        m: 0,
        tz: 0,
        s: 0.,
        validJD: 0,
        validYMD: 0,
        validHMS: 0,
        nFloor: 0,
        rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
        c2rust_padding: [0; 3],
    };
    if isDate(context, argc, argv, &raw mut x) == 0 as ::core::ffi::c_int {
        computeJD(&raw mut x);
        crate::src::src::vdbeapi::sqlite3_result_double(context, x.iJD as ::core::ffi::c_double / 86400000.0f64);
    }
}

unsafe extern "C" fn unixepochFunc(
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let mut x: DateTime = DateTime {
        iJD: 0,
        Y: 0,
        M: 0,
        D: 0,
        h: 0,
        m: 0,
        tz: 0,
        s: 0.,
        validJD: 0,
        validYMD: 0,
        validHMS: 0,
        nFloor: 0,
        rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
        c2rust_padding: [0; 3],
    };
    if isDate(context, argc, argv, &raw mut x) == 0 as ::core::ffi::c_int {
        computeJD(&raw mut x);
        if x.useSubsec() != 0 {
            crate::src::src::vdbeapi::sqlite3_result_double(
                context,
                (x.iJD as crate::src::ext::rtree::rtree::i64_0 - 21086676 as crate::src::ext::rtree::rtree::i64_0 * 10000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0)
                    as ::core::ffi::c_double
                    / 1000.0f64,
            );
        } else {
            crate::src::src::vdbeapi::sqlite3_result_int64(
                context,
                x.iJD / 1000 as crate::src::headers::sqlite3_h::sqlite3_int64
                    - 21086676 as crate::src::headers::sqlite3_h::sqlite3_int64 * 10000 as ::core::ffi::c_int as crate::src::headers::sqlite3_h::sqlite3_int64,
            );
        }
    }
}

unsafe extern "C" fn datetimeFunc(
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let mut x: DateTime = DateTime {
        iJD: 0,
        Y: 0,
        M: 0,
        D: 0,
        h: 0,
        m: 0,
        tz: 0,
        s: 0.,
        validJD: 0,
        validYMD: 0,
        validHMS: 0,
        nFloor: 0,
        rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
        c2rust_padding: [0; 3],
    };
    if isDate(context, argc, argv, &raw mut x) == 0 as ::core::ffi::c_int {
        let mut Y: ::core::ffi::c_int = 0;
        let mut s: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 32] = [0; 32];
        computeYMD_HMS(&raw mut x);
        Y = x.Y;
        if Y < 0 as ::core::ffi::c_int {
            Y = -Y;
        }
        zBuf[1 as ::core::ffi::c_int as usize] = ('0' as i32
            + Y / 1000 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        zBuf[2 as ::core::ffi::c_int as usize] = ('0' as i32
            + Y / 100 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        zBuf[3 as ::core::ffi::c_int as usize] = ('0' as i32
            + Y / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        zBuf[4 as ::core::ffi::c_int as usize] =
            ('0' as i32 + Y % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
        zBuf[5 as ::core::ffi::c_int as usize] = '-' as i32 as ::core::ffi::c_char;
        zBuf[6 as ::core::ffi::c_int as usize] = ('0' as i32
            + x.M / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        zBuf[7 as ::core::ffi::c_int as usize] =
            ('0' as i32 + x.M % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
        zBuf[8 as ::core::ffi::c_int as usize] = '-' as i32 as ::core::ffi::c_char;
        zBuf[9 as ::core::ffi::c_int as usize] = ('0' as i32
            + x.D / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        zBuf[10 as ::core::ffi::c_int as usize] =
            ('0' as i32 + x.D % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
        zBuf[11 as ::core::ffi::c_int as usize] = ' ' as i32 as ::core::ffi::c_char;
        zBuf[12 as ::core::ffi::c_int as usize] = ('0' as i32
            + x.h / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        zBuf[13 as ::core::ffi::c_int as usize] =
            ('0' as i32 + x.h % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
        zBuf[14 as ::core::ffi::c_int as usize] = ':' as i32 as ::core::ffi::c_char;
        zBuf[15 as ::core::ffi::c_int as usize] = ('0' as i32
            + x.m / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        zBuf[16 as ::core::ffi::c_int as usize] =
            ('0' as i32 + x.m % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
        zBuf[17 as ::core::ffi::c_int as usize] = ':' as i32 as ::core::ffi::c_char;
        if x.useSubsec() != 0 {
            s = (1000.0f64 * x.s + 0.5f64) as ::core::ffi::c_int;
            zBuf[18 as ::core::ffi::c_int as usize] = ('0' as i32
                + s / 10000 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[19 as ::core::ffi::c_int as usize] = ('0' as i32
                + s / 1000 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[20 as ::core::ffi::c_int as usize] = '.' as i32 as ::core::ffi::c_char;
            zBuf[21 as ::core::ffi::c_int as usize] = ('0' as i32
                + s / 100 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[22 as ::core::ffi::c_int as usize] = ('0' as i32
                + s / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[23 as ::core::ffi::c_int as usize] =
                ('0' as i32 + s % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
            zBuf[24 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            n = 24 as ::core::ffi::c_int;
        } else {
            s = x.s as ::core::ffi::c_int;
            zBuf[18 as ::core::ffi::c_int as usize] = ('0' as i32
                + s / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[19 as ::core::ffi::c_int as usize] =
                ('0' as i32 + s % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
            zBuf[20 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            n = 20 as ::core::ffi::c_int;
        }
        if x.Y < 0 as ::core::ffi::c_int {
            zBuf[0 as ::core::ffi::c_int as usize] = '-' as i32 as ::core::ffi::c_char;
            crate::src::src::vdbeapi::sqlite3_result_text(
                context,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                n,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
            );
        } else {
            crate::src::src::vdbeapi::sqlite3_result_text(
                context,
                (&raw mut zBuf as *mut ::core::ffi::c_char).offset(1 as isize)
                    as *mut ::core::ffi::c_char,
                n - 1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
            );
        }
    }
}

unsafe extern "C" fn timeFunc(
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let mut x: DateTime = DateTime {
        iJD: 0,
        Y: 0,
        M: 0,
        D: 0,
        h: 0,
        m: 0,
        tz: 0,
        s: 0.,
        validJD: 0,
        validYMD: 0,
        validHMS: 0,
        nFloor: 0,
        rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
        c2rust_padding: [0; 3],
    };
    if isDate(context, argc, argv, &raw mut x) == 0 as ::core::ffi::c_int {
        let mut s: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 16] = [0; 16];
        computeHMS(&raw mut x);
        zBuf[0 as ::core::ffi::c_int as usize] = ('0' as i32
            + x.h / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        zBuf[1 as ::core::ffi::c_int as usize] =
            ('0' as i32 + x.h % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
        zBuf[2 as ::core::ffi::c_int as usize] = ':' as i32 as ::core::ffi::c_char;
        zBuf[3 as ::core::ffi::c_int as usize] = ('0' as i32
            + x.m / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        zBuf[4 as ::core::ffi::c_int as usize] =
            ('0' as i32 + x.m % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
        zBuf[5 as ::core::ffi::c_int as usize] = ':' as i32 as ::core::ffi::c_char;
        if x.useSubsec() != 0 {
            s = (1000.0f64 * x.s + 0.5f64) as ::core::ffi::c_int;
            zBuf[6 as ::core::ffi::c_int as usize] = ('0' as i32
                + s / 10000 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[7 as ::core::ffi::c_int as usize] = ('0' as i32
                + s / 1000 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[8 as ::core::ffi::c_int as usize] = '.' as i32 as ::core::ffi::c_char;
            zBuf[9 as ::core::ffi::c_int as usize] = ('0' as i32
                + s / 100 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[10 as ::core::ffi::c_int as usize] = ('0' as i32
                + s / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[11 as ::core::ffi::c_int as usize] =
                ('0' as i32 + s % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
            zBuf[12 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            n = 12 as ::core::ffi::c_int;
        } else {
            s = x.s as ::core::ffi::c_int;
            zBuf[6 as ::core::ffi::c_int as usize] = ('0' as i32
                + s / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            zBuf[7 as ::core::ffi::c_int as usize] =
                ('0' as i32 + s % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
            zBuf[8 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            n = 8 as ::core::ffi::c_int;
        }
        crate::src::src::vdbeapi::sqlite3_result_text(
            context,
            &raw mut zBuf as *mut ::core::ffi::c_char,
            n,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
        );
    }
}

unsafe extern "C" fn dateFunc(
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let mut x: DateTime = DateTime {
        iJD: 0,
        Y: 0,
        M: 0,
        D: 0,
        h: 0,
        m: 0,
        tz: 0,
        s: 0.,
        validJD: 0,
        validYMD: 0,
        validHMS: 0,
        nFloor: 0,
        rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
        c2rust_padding: [0; 3],
    };
    if isDate(context, argc, argv, &raw mut x) == 0 as ::core::ffi::c_int {
        let mut Y: ::core::ffi::c_int = 0;
        let mut zBuf: [::core::ffi::c_char; 16] = [0; 16];
        computeYMD(&raw mut x);
        Y = x.Y;
        if Y < 0 as ::core::ffi::c_int {
            Y = -Y;
        }
        zBuf[1 as ::core::ffi::c_int as usize] = ('0' as i32
            + Y / 1000 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        zBuf[2 as ::core::ffi::c_int as usize] = ('0' as i32
            + Y / 100 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        zBuf[3 as ::core::ffi::c_int as usize] = ('0' as i32
            + Y / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        zBuf[4 as ::core::ffi::c_int as usize] =
            ('0' as i32 + Y % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
        zBuf[5 as ::core::ffi::c_int as usize] = '-' as i32 as ::core::ffi::c_char;
        zBuf[6 as ::core::ffi::c_int as usize] = ('0' as i32
            + x.M / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        zBuf[7 as ::core::ffi::c_int as usize] =
            ('0' as i32 + x.M % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
        zBuf[8 as ::core::ffi::c_int as usize] = '-' as i32 as ::core::ffi::c_char;
        zBuf[9 as ::core::ffi::c_int as usize] = ('0' as i32
            + x.D / 10 as ::core::ffi::c_int % 10 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        zBuf[10 as ::core::ffi::c_int as usize] =
            ('0' as i32 + x.D % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
        zBuf[11 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        if x.Y < 0 as ::core::ffi::c_int {
            zBuf[0 as ::core::ffi::c_int as usize] = '-' as i32 as ::core::ffi::c_char;
            crate::src::src::vdbeapi::sqlite3_result_text(
                context,
                &raw mut zBuf as *mut ::core::ffi::c_char,
                11 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
            );
        } else {
            crate::src::src::vdbeapi::sqlite3_result_text(
                context,
                (&raw mut zBuf as *mut ::core::ffi::c_char).offset(1 as isize)
                    as *mut ::core::ffi::c_char,
                10 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
            );
        }
    }
}

unsafe extern "C" fn daysAfterJan01(mut pDate: *mut DateTime) -> ::core::ffi::c_int {
    let mut jan01: DateTime = *pDate;
    jan01.validJD = 0 as ::core::ffi::c_char;
    jan01.M = 1 as ::core::ffi::c_int;
    jan01.D = 1 as ::core::ffi::c_int;
    computeJD(&raw mut jan01);
    (((*pDate).iJD - jan01.iJD + 43200000 as crate::src::headers::sqlite3_h::sqlite3_int64) / 86400000 as crate::src::headers::sqlite3_h::sqlite3_int64)
        as ::core::ffi::c_int
}

unsafe extern "C" fn daysAfterMonday(mut pDate: *mut DateTime) -> ::core::ffi::c_int {
    (((*pDate).iJD + 43200000 as crate::src::headers::sqlite3_h::sqlite3_int64) / 86400000 as crate::src::headers::sqlite3_h::sqlite3_int64)
        as ::core::ffi::c_int
        % 7 as ::core::ffi::c_int
}

unsafe extern "C" fn daysAfterSunday(mut pDate: *mut DateTime) -> ::core::ffi::c_int {
    (((*pDate).iJD + 129600000 as crate::src::headers::sqlite3_h::sqlite3_int64) / 86400000 as crate::src::headers::sqlite3_h::sqlite3_int64)
        as ::core::ffi::c_int
        % 7 as ::core::ffi::c_int
}

unsafe extern "C" fn strftimeFunc(
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let mut x: DateTime = DateTime {
        iJD: 0,
        Y: 0,
        M: 0,
        D: 0,
        h: 0,
        m: 0,
        tz: 0,
        s: 0.,
        validJD: 0,
        validYMD: 0,
        validHMS: 0,
        nFloor: 0,
        rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
        c2rust_padding: [0; 3],
    };
    let mut i: crate::__stddef_size_t_h::size_t = 0;
    let mut j: crate::__stddef_size_t_h::size_t = 0;
    let mut db: *mut crate::src::headers::sqliteInt_h::sqlite3 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>();
    let mut zFmt: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut sRes: crate::src::headers::sqliteInt_h::sqlite3_str = crate::src::headers::sqliteInt_h::sqlite3_str {
    db:  ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>(),
    zText:  ::core::ptr::null_mut::<::core::ffi::c_char>(),
    nAlloc:  0,
    mxAlloc:  0,
    nChar:  0,
    accError:  0,
    printfFlags:  0,
};
    if argc == 0 as ::core::ffi::c_int {
        return;
    }
    zFmt = crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(0 as isize))
        as *const ::core::ffi::c_char;
    if zFmt.is_null()
        || isDate(
            context,
            argc - 1 as ::core::ffi::c_int,
            argv.offset(1 as isize),
            &raw mut x,
        ) != 0
    {
        return;
    }
    db =  crate::src::src::vdbeapi::sqlite3_context_db_handle(context) as
    *mut crate::src::headers::sqliteInt_h::sqlite3;
    crate::src::src::printf::sqlite3StrAccumInit(
        
        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
        
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>() as
    *mut crate::src::headers::sqliteInt_h::sqlite3,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        (*db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_LENGTH as usize],
    );
    computeJD(&raw mut x);
    computeYMD_HMS(&raw mut x);
    j = 0 as crate::__stddef_size_t_h::size_t;
    i = j;
    while *zFmt.offset(i as isize) != 0 {
        let mut cf: ::core::ffi::c_char = 0;
        if !(*zFmt.offset(i as isize) as ::core::ffi::c_int != '%' as i32) {
            if j < i {
                crate::src::src::printf::sqlite3_str_append(
                    
                    &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                    zFmt.offset(j as isize),
                    i.wrapping_sub(j) as ::core::ffi::c_int,
                );
            }
            i = i.wrapping_add(1);
            j = i.wrapping_add(1 as crate::__stddef_size_t_h::size_t);
            cf = *zFmt.offset(i as isize);
            match cf as ::core::ffi::c_int {
                100 | 101 => {
                    if cf as ::core::ffi::c_int == 'd' as i32 {
                        crate::src::src::printf::sqlite3_str_vappendf2(
                            &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                            "%02d",
                            printf_args!(x.D),
                        );
                    } else {
                        crate::src::src::printf::sqlite3_str_vappendf2(
                            &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                            "%2d",
                            printf_args!(x.D),
                        );
                    }
                }
                102 => {
                    let mut s: ::core::ffi::c_double = x.s;
                    if s > 59.999f64 {
                        s = 59.999f64;
                    }
                    crate::src::src::printf::sqlite3_str_vappendf2(
                        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                        "%06.3f",
                        printf_args!(s),
                    );
                }
                70 => {
                    crate::src::src::printf::sqlite3_str_vappendf2(
                        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                        "%04d-%02d-%02d",
                        printf_args!(x.Y, x.M, x.D),
                    );
                }
                71 | 103 => {
                    let mut y: DateTime = x;
                    y.iJD += ((3 as ::core::ffi::c_int - daysAfterMonday(&raw mut x))
                        * 86400000 as ::core::ffi::c_int)
                        as crate::src::headers::sqlite3_h::sqlite3_int64;
                    y.validYMD = 0 as ::core::ffi::c_char;
                    computeYMD(&raw mut y);
                    if cf as ::core::ffi::c_int == 'g' as i32 {
                        crate::src::src::printf::sqlite3_str_vappendf2(
                            &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                            "%02d",
                            printf_args!(y.Y % 100 as ::core::ffi::c_int),
                        );
                    } else {
                        crate::src::src::printf::sqlite3_str_vappendf2(
                            &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                            "%04d",
                            printf_args!(y.Y),
                        );
                    }
                }
                72 | 107 => {
                    if cf as ::core::ffi::c_int == 'H' as i32 {
                        crate::src::src::printf::sqlite3_str_vappendf2(
                            &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                            "%02d",
                            printf_args!(x.h),
                        );
                    } else {
                        crate::src::src::printf::sqlite3_str_vappendf2(
                            &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                            "%2d",
                            printf_args!(x.h),
                        );
                    }
                }
                73 | 108 => {
                    let mut h: ::core::ffi::c_int = x.h;
                    if h > 12 as ::core::ffi::c_int {
                        h -= 12 as ::core::ffi::c_int;
                    }
                    if h == 0 as ::core::ffi::c_int {
                        h = 12 as ::core::ffi::c_int;
                    }
                    if cf as ::core::ffi::c_int == 'I' as i32 {
                        crate::src::src::printf::sqlite3_str_vappendf2(
                            &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                            "%02d",
                            printf_args!(h),
                        );
                    } else {
                        crate::src::src::printf::sqlite3_str_vappendf2(
                            &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                            "%2d",
                            printf_args!(h),
                        );
                    }
                }
                106 => {
                    crate::src::src::printf::sqlite3_str_vappendf2(
                        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                        "%03d",
                        printf_args!(daysAfterJan01(&raw mut x) + 1 as ::core::ffi::c_int),
                    );
                }
                74 => {
                    crate::src::src::printf::sqlite3_str_vappendf2(
                        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                        "%.16g",
                        printf_args!(x.iJD as ::core::ffi::c_double / 86400000.0f64),
                    );
                }
                109 => {
                    crate::src::src::printf::sqlite3_str_vappendf2(
                        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                        "%02d",
                        printf_args!(x.M),
                    );
                }
                77 => {
                    crate::src::src::printf::sqlite3_str_vappendf2(
                        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                        "%02d",
                        printf_args!(x.m),
                    );
                }
                112 | 80 => {
                    if x.h >= 12 as ::core::ffi::c_int {
                        crate::src::src::printf::sqlite3_str_append(
                            
                            &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                            if cf as ::core::ffi::c_int == 'p' as i32 {
                                b"PM\0" as *const u8 as *const ::core::ffi::c_char
                            } else {
                                b"pm\0" as *const u8 as *const ::core::ffi::c_char
                            },
                            2 as ::core::ffi::c_int,
                        );
                    } else {
                        crate::src::src::printf::sqlite3_str_append(
                            
                            &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                            if cf as ::core::ffi::c_int == 'p' as i32 {
                                b"AM\0" as *const u8 as *const ::core::ffi::c_char
                            } else {
                                b"am\0" as *const u8 as *const ::core::ffi::c_char
                            },
                            2 as ::core::ffi::c_int,
                        );
                    }
                }
                82 => {
                    crate::src::src::printf::sqlite3_str_vappendf2(
                        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                        "%02d:%02d",
                        printf_args!(x.h, x.m),
                    );
                }
                115 => {
                    if x.useSubsec() != 0 {
                        crate::src::src::printf::sqlite3_str_vappendf2(
                            &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                            "%.3f",
                            printf_args!(
                                (x.iJD as crate::src::ext::rtree::rtree::i64_0
                                    - 21086676 as crate::src::ext::rtree::rtree::i64_0 * 10000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0)
                                    as ::core::ffi::c_double
                                    / 1000.0f64
                            ),
                        );
                    } else {
                        let mut iS: crate::src::ext::rtree::rtree::i64_0 = x.iJD as crate::src::ext::rtree::rtree::i64_0 / 1000 as crate::src::ext::rtree::rtree::i64_0
                            - 21086676 as crate::src::ext::rtree::rtree::i64_0 * 10000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0;
                        crate::src::src::printf::sqlite3_str_vappendf2(
                            &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                            "%lld",
                            printf_args!(iS),
                        );
                    }
                }
                83 => {
                    crate::src::src::printf::sqlite3_str_vappendf2(
                        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                        "%02d",
                        printf_args!(x.s as ::core::ffi::c_int),
                    );
                }
                84 => {
                    crate::src::src::printf::sqlite3_str_vappendf2(
                        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                        "%02d:%02d:%02d",
                        printf_args!(x.h, x.m, x.s as ::core::ffi::c_int),
                    );
                }
                117 | 119 => {
                    let mut c: ::core::ffi::c_char =
                        (daysAfterSunday(&raw mut x) as ::core::ffi::c_char as ::core::ffi::c_int
                            + '0' as i32) as ::core::ffi::c_char;
                    if c as ::core::ffi::c_int == '0' as i32
                        && cf as ::core::ffi::c_int == 'u' as i32
                    {
                        c = '7' as i32 as ::core::ffi::c_char;
                    }
                    crate::src::src::printf::sqlite3_str_appendchar(&raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str, 1 as ::core::ffi::c_int, c);
                }
                85 => {
                    crate::src::src::printf::sqlite3_str_vappendf2(
                        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                        "%02d",
                        printf_args!(
                            (daysAfterJan01(&raw mut x) - daysAfterSunday(&raw mut x)
                                + 7 as ::core::ffi::c_int)
                                / 7 as ::core::ffi::c_int
                        ),
                    );
                }
                86 => {
                    let mut y_0: DateTime = x;
                    y_0.iJD += ((3 as ::core::ffi::c_int - daysAfterMonday(&raw mut x))
                        * 86400000 as ::core::ffi::c_int)
                        as crate::src::headers::sqlite3_h::sqlite3_int64;
                    y_0.validYMD = 0 as ::core::ffi::c_char;
                    computeYMD(&raw mut y_0);
                    crate::src::src::printf::sqlite3_str_vappendf2(
                        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                        "%02d",
                        printf_args!(
                            daysAfterJan01(&raw mut y_0) / 7 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                        ),
                    );
                }
                87 => {
                    crate::src::src::printf::sqlite3_str_vappendf2(
                        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                        "%02d",
                        printf_args!(
                            (daysAfterJan01(&raw mut x) - daysAfterMonday(&raw mut x)
                                + 7 as ::core::ffi::c_int)
                                / 7 as ::core::ffi::c_int
                        ),
                    );
                }
                89 => {
                    crate::src::src::printf::sqlite3_str_vappendf2(
                        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                        "%04d",
                        printf_args!(x.Y),
                    );
                }
                37 => {
                    crate::src::src::printf::sqlite3_str_appendchar(
                        
                        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                        1 as ::core::ffi::c_int,
                        '%' as i32 as ::core::ffi::c_char,
                    );
                }
                _ => {
                    crate::src::src::printf::sqlite3_str_reset(&raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str);
                    return;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if j < i {
        crate::src::src::printf::sqlite3_str_append(
            
            &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
            zFmt.offset(j as isize),
            i.wrapping_sub(j) as ::core::ffi::c_int,
        );
    }
    crate::src::src::printf::sqlite3ResultStrAccum(context,  &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str);
}

unsafe extern "C" fn ctimeFunc(
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut _NotUsed: ::core::ffi::c_int,
    mut _NotUsed2: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    timeFunc(
        context,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
}

unsafe extern "C" fn cdateFunc(
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut _NotUsed: ::core::ffi::c_int,
    mut _NotUsed2: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    dateFunc(
        context,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
}

unsafe extern "C" fn timediffFunc(
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut _NotUsed1: ::core::ffi::c_int,
    mut argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let mut sign: ::core::ffi::c_char = 0;
    let mut Y: ::core::ffi::c_int = 0;
    let mut M: ::core::ffi::c_int = 0;
    let mut d1: DateTime = DateTime {
        iJD: 0,
        Y: 0,
        M: 0,
        D: 0,
        h: 0,
        m: 0,
        tz: 0,
        s: 0.,
        validJD: 0,
        validYMD: 0,
        validHMS: 0,
        nFloor: 0,
        rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
        c2rust_padding: [0; 3],
    };
    let mut d2: DateTime = DateTime {
        iJD: 0,
        Y: 0,
        M: 0,
        D: 0,
        h: 0,
        m: 0,
        tz: 0,
        s: 0.,
        validJD: 0,
        validYMD: 0,
        validHMS: 0,
        nFloor: 0,
        rawS_isError_useSubsec_isUtc_isLocal: [0; 1],
        c2rust_padding: [0; 3],
    };
    let mut sRes: crate::src::headers::sqliteInt_h::sqlite3_str = crate::src::headers::sqliteInt_h::sqlite3_str {
    db:  ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>(),
    zText:  ::core::ptr::null_mut::<::core::ffi::c_char>(),
    nAlloc:  0,
    mxAlloc:  0,
    nChar:  0,
    accError:  0,
    printfFlags:  0,
};
    if isDate(
        context,
        1 as ::core::ffi::c_int,
        argv.offset(0 as isize) as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        &raw mut d1,
    ) != 0
    {
        return;
    }
    if isDate(
        context,
        1 as ::core::ffi::c_int,
        argv.offset(1 as isize) as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        &raw mut d2,
    ) != 0
    {
        return;
    }
    computeYMD_HMS(&raw mut d1);
    computeYMD_HMS(&raw mut d2);
    if d1.iJD >= d2.iJD {
        sign = '+' as i32 as ::core::ffi::c_char;
        Y = d1.Y - d2.Y;
        if Y != 0 {
            d2.Y = d1.Y;
            d2.validJD = 0 as ::core::ffi::c_char;
            computeJD(&raw mut d2);
        }
        M = d1.M - d2.M;
        if M < 0 as ::core::ffi::c_int {
            Y -= 1;
            M += 12 as ::core::ffi::c_int;
        }
        if M != 0 as ::core::ffi::c_int {
            d2.M = d1.M;
            d2.validJD = 0 as ::core::ffi::c_char;
            computeJD(&raw mut d2);
        }
        while d1.iJD < d2.iJD {
            M -= 1;
            if M < 0 as ::core::ffi::c_int {
                M = 11 as ::core::ffi::c_int;
                Y -= 1;
            }
            d2.M -= 1;
            if d2.M < 1 as ::core::ffi::c_int {
                d2.M = 12 as ::core::ffi::c_int;
                d2.Y -= 1;
            }
            d2.validJD = 0 as ::core::ffi::c_char;
            computeJD(&raw mut d2);
        }
        d1.iJD -= d2.iJD;
        d1.iJD = (d1.iJD as crate::src::ext::rtree::rtree::u64_0).wrapping_add(
            (1486995408 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0)
                .wrapping_mul(100000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0),
        ) as crate::src::headers::sqlite3_h::sqlite3_int64 as crate::src::headers::sqlite3_h::sqlite3_int64;
    } else {
        sign = '-' as i32 as ::core::ffi::c_char;
        Y = d2.Y - d1.Y;
        if Y != 0 {
            d2.Y = d1.Y;
            d2.validJD = 0 as ::core::ffi::c_char;
            computeJD(&raw mut d2);
        }
        M = d2.M - d1.M;
        if M < 0 as ::core::ffi::c_int {
            Y -= 1;
            M += 12 as ::core::ffi::c_int;
        }
        if M != 0 as ::core::ffi::c_int {
            d2.M = d1.M;
            d2.validJD = 0 as ::core::ffi::c_char;
            computeJD(&raw mut d2);
        }
        while d1.iJD > d2.iJD {
            M -= 1;
            if M < 0 as ::core::ffi::c_int {
                M = 11 as ::core::ffi::c_int;
                Y -= 1;
            }
            d2.M += 1;
            if d2.M > 12 as ::core::ffi::c_int {
                d2.M = 1 as ::core::ffi::c_int;
                d2.Y += 1;
            }
            d2.validJD = 0 as ::core::ffi::c_char;
            computeJD(&raw mut d2);
        }
        d1.iJD = d2.iJD - d1.iJD;
        d1.iJD = (d1.iJD as crate::src::ext::rtree::rtree::u64_0).wrapping_add(
            (1486995408 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0)
                .wrapping_mul(100000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0),
        ) as crate::src::headers::sqlite3_h::sqlite3_int64 as crate::src::headers::sqlite3_h::sqlite3_int64;
    }
    clearYMD_HMS_TZ(&raw mut d1);
    computeYMD_HMS(&raw mut d1);
    crate::src::src::printf::sqlite3StrAccumInit(
        
        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
        
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>() as
    *mut crate::src::headers::sqliteInt_h::sqlite3,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        100 as ::core::ffi::c_int,
    );
    crate::src::src::printf::sqlite3_str_vappendf2(
        &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
        "%c%04d-%02d-%02d %02d:%02d:%06.3f",
        printf_args!(
            sign as ::core::ffi::c_int,
            Y,
            M,
            d1.D - 1 as ::core::ffi::c_int,
            d1.h,
            d1.m,
            d1.s
        ),
    );
    crate::src::src::printf::sqlite3ResultStrAccum(context,  &raw mut sRes as *mut _ as *mut crate::src::headers::sqliteInt_h::sqlite3_str);
}

unsafe extern "C" fn ctimestampFunc(
    mut context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut _NotUsed: ::core::ffi::c_int,
    mut _NotUsed2: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    datetimeFunc(
        context,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<*mut crate::src::headers::vdbeInt_h::sqlite3_value>(),
    );
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3RegisterDateTimeFunctions() {
    static mut aDateTimeFuncs: [crate::src::headers::sqliteInt_h::FuncDef; 10] = {
        [
            crate::src::headers::sqliteInt_h::FuncDef {
    nArg:  -(1 as ::core::ffi::c_int) as crate::src::fts5::i16_0,
    funcFlags:  (crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_SLOCHNG
                    | crate::src::headers::sqlite3_h::SQLITE_UTF8
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT) as crate::src::ext::rtree::rtree::u32_0,
    pUserData:  &raw const crate::src::src::global::sqlite3Config as *mut crate::src::headers::sqliteInt_h::Sqlite3Config
                    as *mut ::core::ffi::c_void,
    pNext:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
    xSFunc:  Some(
                    juliandayFunc
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
    xFinalize:  None,
    xValue:  None,
    xInverse:  None,
    zName:  b"julianday\0" as *const u8 as *const ::core::ffi::c_char,
    u:  crate::src::headers::sqliteInt_h::__anon_union_2 {
    pHash:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
},
},
            crate::src::headers::sqliteInt_h::FuncDef {
    nArg:  -(1 as ::core::ffi::c_int) as crate::src::fts5::i16_0,
    funcFlags:  (crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_SLOCHNG
                    | crate::src::headers::sqlite3_h::SQLITE_UTF8
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT) as crate::src::ext::rtree::rtree::u32_0,
    pUserData:  &raw const crate::src::src::global::sqlite3Config as *mut crate::src::headers::sqliteInt_h::Sqlite3Config
                    as *mut ::core::ffi::c_void,
    pNext:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
    xSFunc:  Some(
                    unixepochFunc
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
    xFinalize:  None,
    xValue:  None,
    xInverse:  None,
    zName:  b"unixepoch\0" as *const u8 as *const ::core::ffi::c_char,
    u:  crate::src::headers::sqliteInt_h::__anon_union_2 {
    pHash:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
},
},
            crate::src::headers::sqliteInt_h::FuncDef {
    nArg:  -(1 as ::core::ffi::c_int) as crate::src::fts5::i16_0,
    funcFlags:  (crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_SLOCHNG
                    | crate::src::headers::sqlite3_h::SQLITE_UTF8
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT) as crate::src::ext::rtree::rtree::u32_0,
    pUserData:  &raw const crate::src::src::global::sqlite3Config as *mut crate::src::headers::sqliteInt_h::Sqlite3Config
                    as *mut ::core::ffi::c_void,
    pNext:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
    xSFunc:  Some(
                    dateFunc
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
    xFinalize:  None,
    xValue:  None,
    xInverse:  None,
    zName:  b"date\0" as *const u8 as *const ::core::ffi::c_char,
    u:  crate::src::headers::sqliteInt_h::__anon_union_2 {
    pHash:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
},
},
            crate::src::headers::sqliteInt_h::FuncDef {
    nArg:  -(1 as ::core::ffi::c_int) as crate::src::fts5::i16_0,
    funcFlags:  (crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_SLOCHNG
                    | crate::src::headers::sqlite3_h::SQLITE_UTF8
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT) as crate::src::ext::rtree::rtree::u32_0,
    pUserData:  &raw const crate::src::src::global::sqlite3Config as *mut crate::src::headers::sqliteInt_h::Sqlite3Config
                    as *mut ::core::ffi::c_void,
    pNext:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
    xSFunc:  Some(
                    timeFunc
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
    xFinalize:  None,
    xValue:  None,
    xInverse:  None,
    zName:  b"time\0" as *const u8 as *const ::core::ffi::c_char,
    u:  crate::src::headers::sqliteInt_h::__anon_union_2 {
    pHash:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
},
},
            crate::src::headers::sqliteInt_h::FuncDef {
    nArg:  -(1 as ::core::ffi::c_int) as crate::src::fts5::i16_0,
    funcFlags:  (crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_SLOCHNG
                    | crate::src::headers::sqlite3_h::SQLITE_UTF8
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT) as crate::src::ext::rtree::rtree::u32_0,
    pUserData:  &raw const crate::src::src::global::sqlite3Config as *mut crate::src::headers::sqliteInt_h::Sqlite3Config
                    as *mut ::core::ffi::c_void,
    pNext:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
    xSFunc:  Some(
                    datetimeFunc
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
    xFinalize:  None,
    xValue:  None,
    xInverse:  None,
    zName:  b"datetime\0" as *const u8 as *const ::core::ffi::c_char,
    u:  crate::src::headers::sqliteInt_h::__anon_union_2 {
    pHash:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
},
},
            crate::src::headers::sqliteInt_h::FuncDef {
    nArg:  -(1 as ::core::ffi::c_int) as crate::src::fts5::i16_0,
    funcFlags:  (crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_SLOCHNG
                    | crate::src::headers::sqlite3_h::SQLITE_UTF8
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT) as crate::src::ext::rtree::rtree::u32_0,
    pUserData:  &raw const crate::src::src::global::sqlite3Config as *mut crate::src::headers::sqliteInt_h::Sqlite3Config
                    as *mut ::core::ffi::c_void,
    pNext:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
    xSFunc:  Some(
                    strftimeFunc
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
    xFinalize:  None,
    xValue:  None,
    xInverse:  None,
    zName:  b"strftime\0" as *const u8 as *const ::core::ffi::c_char,
    u:  crate::src::headers::sqliteInt_h::__anon_union_2 {
    pHash:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
},
},
            crate::src::headers::sqliteInt_h::FuncDef {
    nArg:  2 as crate::src::fts5::i16_0,
    funcFlags:  (crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_SLOCHNG
                    | crate::src::headers::sqlite3_h::SQLITE_UTF8
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT) as crate::src::ext::rtree::rtree::u32_0,
    pUserData:  &raw const crate::src::src::global::sqlite3Config as *mut crate::src::headers::sqliteInt_h::Sqlite3Config
                    as *mut ::core::ffi::c_void,
    pNext:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
    xSFunc:  Some(
                    timediffFunc
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
    xFinalize:  None,
    xValue:  None,
    xInverse:  None,
    zName:  b"timediff\0" as *const u8 as *const ::core::ffi::c_char,
    u:  crate::src::headers::sqliteInt_h::__anon_union_2 {
    pHash:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
},
},
            crate::src::headers::sqliteInt_h::FuncDef {
    nArg:  0 as crate::src::fts5::i16_0,
    funcFlags:  (crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN | crate::src::headers::sqliteInt_h::SQLITE_FUNC_SLOCHNG | crate::src::headers::sqlite3_h::SQLITE_UTF8) as crate::src::ext::rtree::rtree::u32_0,
    pUserData:  ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    pNext:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
    xSFunc:  Some(
                    ctimeFunc
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
    xFinalize:  None,
    xValue:  None,
    xInverse:  None,
    zName:  b"current_time\0" as *const u8 as *const ::core::ffi::c_char,
    u:  crate::src::headers::sqliteInt_h::__anon_union_2 {
    pHash:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
},
},
            crate::src::headers::sqliteInt_h::FuncDef {
    nArg:  0 as crate::src::fts5::i16_0,
    funcFlags:  (crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN | crate::src::headers::sqliteInt_h::SQLITE_FUNC_SLOCHNG | crate::src::headers::sqlite3_h::SQLITE_UTF8) as crate::src::ext::rtree::rtree::u32_0,
    pUserData:  ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    pNext:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
    xSFunc:  Some(
                    ctimestampFunc
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
    xFinalize:  None,
    xValue:  None,
    xInverse:  None,
    zName:  b"current_timestamp\0" as *const u8 as *const ::core::ffi::c_char,
    u:  crate::src::headers::sqliteInt_h::__anon_union_2 {
    pHash:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
},
},
            crate::src::headers::sqliteInt_h::FuncDef {
    nArg:  0 as crate::src::fts5::i16_0,
    funcFlags:  (crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN | crate::src::headers::sqliteInt_h::SQLITE_FUNC_SLOCHNG | crate::src::headers::sqlite3_h::SQLITE_UTF8) as crate::src::ext::rtree::rtree::u32_0,
    pUserData:  ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    pNext:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
    xSFunc:  Some(
                    cdateFunc
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
    xFinalize:  None,
    xValue:  None,
    xInverse:  None,
    zName:  b"current_date\0" as *const u8 as *const ::core::ffi::c_char,
    u:  crate::src::headers::sqliteInt_h::__anon_union_2 {
    pHash:  ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>() as *mut crate::src::headers::sqliteInt_h::FuncDef,
},
},
        ]
    };
    crate::src::src::callback::sqlite3InsertBuiltinFuncs(
        
        &raw mut aDateTimeFuncs as *mut crate::src::headers::sqliteInt_h::FuncDef as *mut crate::src::headers::sqliteInt_h::FuncDef,
        (::core::mem::size_of::<[crate::src::headers::sqliteInt_h::FuncDef; 10]>() as usize)
            .wrapping_div(::core::mem::size_of::<crate::src::headers::sqliteInt_h::FuncDef>() as usize) as ::core::ffi::c_int,
    );
}

// Re-export variadic functions from printf_c_variadic module
pub use crate::src::printf_c_variadic::getDigits;
