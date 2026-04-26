pub use crate::__stddef_size_t_h::SizeT;
pub use crate::src::headers::stdlib::VaList;
pub use crate::src::printf_c_variadic::sqlite3_log_args;
pub use crate::src::printf_c_variadic::sqlite3DebugPrintf_args;

pub use crate::internal::BuiltinVaList;
pub use crate::internal::VaListTag;
pub use crate::src::src::hash::_ht;
pub use crate::src::src::hash::Hash;
pub use crate::src::src::hash::HashElem;
pub use crate::src::src::pager::Pgno;

pub use crate::sqliteLimit_h::SQLITE_MAX_LENGTH;
pub use crate::src::ext::rtree::rtree::I64_0;
pub use crate::src::ext::rtree::rtree::U8_0;
pub use crate::src::ext::rtree::rtree::U32_0;
pub use crate::src::ext::rtree::rtree::U64_0;
pub use crate::src::fts5::I16_0;
pub use crate::src::fts5::U16_0;
pub use crate::src::headers::sqlite3_h::SQLITE_LIMIT_LENGTH;
pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_STATIC;
pub use crate::src::headers::sqlite3_h::SQLITE_TOOBIG;
pub use crate::src::headers::sqlite3_h::SqliteInt64;
pub use crate::src::headers::sqlite3_h::SqliteUint64;
pub use crate::src::headers::sqlite3_h::Sqlite3DestructorType;
pub use crate::src::headers::sqlite3_h::sqlite3_file;
pub use crate::src::headers::sqlite3_h::Sqlite3Filename;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;
pub use crate::src::headers::sqlite3_h::sqlite3_index_info;
pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;
pub use crate::src::headers::sqlite3_h::Sqlite3Int64;
pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;
pub use crate::src::headers::sqlite3_h::sqlite3_mem_methods;
pub use crate::src::headers::sqlite3_h::sqlite3_module;
pub use crate::src::headers::sqlite3_h::sqlite3_mutex_methods;
pub use crate::src::headers::sqlite3_h::Sqlite3Pcache;
pub use crate::src::headers::sqlite3_h::sqlite3_pcache_methods2;
pub use crate::src::headers::sqlite3_h::sqlite3_pcache_page;
pub use crate::src::headers::sqlite3_h::Sqlite3SyscallPtr;
pub use crate::src::headers::sqlite3_h::Sqlite3Uint64;
pub use crate::src::headers::sqlite3_h::sqlite3_vfs;
pub use crate::src::headers::sqlite3_h::sqlite3_vtab;
pub use crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;
pub use crate::src::headers::sqliteInt_h::__anon_struct_0;
pub use crate::src::headers::sqliteInt_h::__anon_struct_1;
pub use crate::src::headers::sqliteInt_h::__anon_struct_2;
pub use crate::src::headers::sqliteInt_h::__anon_struct_3;
pub use crate::src::headers::sqliteInt_h::__anon_struct_4;
pub use crate::src::headers::sqliteInt_h::__anon_struct_5;
pub use crate::src::headers::sqliteInt_h::__anon_struct_6;
pub use crate::src::headers::sqliteInt_h::__anon_struct_7;
pub use crate::src::headers::sqliteInt_h::__anon_struct_8;
pub use crate::src::headers::sqliteInt_h::__anon_union_0;
pub use crate::src::headers::sqliteInt_h::__anon_union_1;
pub use crate::src::headers::sqliteInt_h::__anon_union_2;
pub use crate::src::headers::sqliteInt_h::__anon_union_3;
pub use crate::src::headers::sqliteInt_h::__anon_union_5;
pub use crate::src::headers::sqliteInt_h::__anon_union_6;
pub use crate::src::headers::sqliteInt_h::__anon_union_7;
pub use crate::src::headers::sqliteInt_h::__anon_union_8;
pub use crate::src::headers::sqliteInt_h::__anon_union_9;
pub use crate::src::headers::sqliteInt_h::__anon_union_10;
pub use crate::src::headers::sqliteInt_h::__anon_union_11;
pub use crate::src::headers::sqliteInt_h::__anon_union_12;
pub use crate::src::headers::sqliteInt_h::__anon_union_13;
pub use crate::src::headers::sqliteInt_h::__anon_union_15;
pub use crate::src::headers::sqliteInt_h::AggInfo;
pub use crate::src::headers::sqliteInt_h::AggInfo_col;
pub use crate::src::headers::sqliteInt_h::AggInfo_func;
pub use crate::src::headers::sqliteInt_h::AutoincInfo;
pub use crate::src::headers::sqliteInt_h::Bitmask;
pub use crate::src::headers::sqliteInt_h::BusyHandler;
pub use crate::src::headers::sqliteInt_h::CollSeq;
pub use crate::src::headers::sqliteInt_h::Column;
pub use crate::src::headers::sqliteInt_h::Cte;
pub use crate::src::headers::sqliteInt_h::CteUse;
pub use crate::src::headers::sqliteInt_h::Db;
pub use crate::src::headers::sqliteInt_h::DbClientData;
pub use crate::src::headers::sqliteInt_h::Expr;
pub use crate::src::headers::sqliteInt_h::ExprList;
pub use crate::src::headers::sqliteInt_h::ExprList_item;
pub use crate::src::headers::sqliteInt_h::FKey;
pub use crate::src::headers::sqliteInt_h::FpDecode;
pub use crate::src::headers::sqliteInt_h::FuncDef;
pub use crate::src::headers::sqliteInt_h::FuncDestructor;
pub use crate::src::headers::sqliteInt_h::IdList;
pub use crate::src::headers::sqliteInt_h::IdList_item;
pub use crate::src::headers::sqliteInt_h::Index;
pub use crate::src::headers::sqliteInt_h::IndexedExpr;
pub use crate::src::headers::sqliteInt_h::KeyInfo;
pub use crate::src::headers::sqliteInt_h::LogEst;
pub use crate::src::headers::sqliteInt_h::Lookaside;
pub use crate::src::headers::sqliteInt_h::LookasideSlot;
pub use crate::src::headers::sqliteInt_h::Module;
pub use crate::src::headers::sqliteInt_h::Parse;
pub use crate::src::headers::sqliteInt_h::ParseCleanup;
pub use crate::src::headers::sqliteInt_h::PrintfArguments;
pub use crate::src::headers::sqliteInt_h::RCStr;
pub use crate::src::headers::sqliteInt_h::RenameToken;
pub use crate::src::headers::sqliteInt_h::Returning;
pub use crate::src::headers::sqliteInt_h::SF_MultiValue;
pub use crate::src::headers::sqliteInt_h::SF_NestedFrom;
pub use crate::src::headers::sqliteInt_h::SQLITE_PRINTF_INTERNAL;
pub use crate::src::headers::sqliteInt_h::SQLITE_PRINTF_MALLOCED;
pub use crate::src::headers::sqliteInt_h::SQLITE_PRINTF_SQLFUNC;
pub use crate::src::headers::sqliteInt_h::Savepoint;
pub use crate::src::headers::sqliteInt_h::Schema;
pub use crate::src::headers::sqliteInt_h::Select;
pub use crate::src::headers::sqliteInt_h::Sqlite3Config;
pub use crate::src::headers::sqliteInt_h::SrcItem;
pub use crate::src::headers::sqliteInt_h::SrcList;
pub use crate::src::headers::sqliteInt_h::StrAccum;
pub use crate::src::headers::sqliteInt_h::Subquery;
pub use crate::src::headers::sqliteInt_h::Table;
pub use crate::src::headers::sqliteInt_h::TableLock;
pub use crate::src::headers::sqliteInt_h::Token;
pub use crate::src::headers::sqliteInt_h::Trigger;
pub use crate::src::headers::sqliteInt_h::TriggerPrg;
pub use crate::src::headers::sqliteInt_h::TriggerStep;
pub use crate::src::headers::sqliteInt_h::Upsert;
pub use crate::src::headers::sqliteInt_h::VList;
pub use crate::src::headers::sqliteInt_h::VTable;
pub use crate::src::headers::sqliteInt_h::VtabCtx;
pub use crate::src::headers::sqliteInt_h::Window;
pub use crate::src::headers::sqliteInt_h::With;
pub use crate::src::headers::sqliteInt_h::Bft;
pub use crate::src::headers::sqliteInt_h::sColMap;
pub use crate::src::headers::sqliteInt_h::sqlite3;
pub use crate::src::headers::sqliteInt_h::sqlite3_str;
pub use crate::src::headers::sqliteInt_h::Sqlite3Xauth;
pub use crate::src::headers::sqliteInt_h::sqlite3InitInfo;
pub use crate::src::headers::sqliteInt_h::Uptr;
pub use crate::src::headers::sqliteInt_h::YDbMask;
pub use crate::src::headers::sqliteInt_h::YnVar;
pub use crate::src::headers::stdlib::Int16T;
pub use crate::src::headers::stdlib::UintptrT;
pub use crate::src::headers::vdbeInt_h::PreUpdate;
pub use crate::src::headers::vdbeInt_h::sqlite3_context;
pub use crate::src::headers::vdbeInt_h::sqlite3_value;
pub use crate::src::src::global::sqlite3Config;
pub use crate::src::src::main::sqlite3_initialize;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3_malloc64;
pub use crate::src::src::malloc::sqlite3_realloc64;
pub use crate::src::src::malloc::sqlite3DbFree;
pub use crate::src::src::malloc::sqlite3DbMallocRaw;
pub use crate::src::src::malloc::sqlite3DbMallocSize;
pub use crate::src::src::malloc::sqlite3DbRealloc;
pub use crate::src::src::malloc::sqlite3OomFault;
pub use crate::src::src::malloc::sqlite3Realloc;
pub use crate::src::src::mutex_unix::sqlite3_mutex;
pub use crate::src::src::rowset::sqlite3RowSetClear;
pub use crate::src::src::utf::sqlite3AppendOneUtf8Character;
pub use crate::src::src::util::sqlite3ErrorToParser;
pub use crate::src::src::util::sqlite3FpDecode;
pub use crate::src::src::util::sqlite3Strlen30;
pub use crate::src::src::vdbeapi::sqlite3_result_error_code;
pub use crate::src::src::vdbeapi::sqlite3_result_text;
pub use crate::src::src::vdbeapi::sqlite3_value_double;
pub use crate::src::src::vdbeapi::sqlite3_value_int64;
pub use crate::src::src::vdbeapi::sqlite3_value_text;

pub use crate::src::headers::stdlib::OffT;
pub use crate::src::headers::stdlib::Off64T;
pub use crate::src::headers::stdlib::Uint8T;
pub use crate::src::headers::stdlib::Uint16T;
pub use crate::src::headers::stdlib::Uint32T;
pub use crate::src::headers::stdlib::IoFile;
pub use crate::src::headers::stdlib::IoLockT;
pub use crate::src::headers::stdlib::FILE;
pub use crate::src::headers::vdbeInt_h::Vdbe;
pub use crate::src::src::vdbe::Mem;
pub use crate::src::src::vdbe::SubProgram;
pub use crate::src::src::vdbe::SubrtnSig;
pub use crate::src::src::vdbe::VdbeOp;
pub use crate::src::src::vdbe::p4union;

pub type EtByte = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct et_info {
    pub fmttype: ::core::ffi::c_char,
    pub base: EtByte,
    pub flags: EtByte,
    pub type_0: EtByte,
    pub charset: EtByte,
    pub prefix: EtByte,
    pub iNxt: ::core::ffi::c_char,
}

pub const etRADIX: ::core::ffi::c_int = 0;

pub const etFLOAT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const etEXP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const etGENERIC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const etSIZE: ::core::ffi::c_int = 4;

pub const etSTRING: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const etDYNSTRING: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub const etPERCENT: ::core::ffi::c_int = 7;

pub const etCHARX: ::core::ffi::c_int = 8;

pub const etESCAPE_q: ::core::ffi::c_int = 9 as ::core::ffi::c_int;

pub const etESCAPE_Q: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

pub const etTOKEN: ::core::ffi::c_int = 11;

pub const etSRCITEM: ::core::ffi::c_int = 12;

pub const etPOINTER: ::core::ffi::c_int = 13;

pub const etESCAPE_w: ::core::ffi::c_int = 14 as ::core::ffi::c_int;

pub const etORDINAL: ::core::ffi::c_int = 15 as ::core::ffi::c_int;

pub const etDECIMAL: ::core::ffi::c_int = 16;

pub const etINVALID: ::core::ffi::c_int = 17 as ::core::ffi::c_int;

pub const FLAG_SIGNED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

static mut aDigits: [::core::ffi::c_char; 33] = unsafe {
    ::core::mem::transmute::<[u8; 33], [::core::ffi::c_char; 33]>(
        *b"0123456789ABCDEF0123456789abcdef\0",
    )
};

static mut aPrefix: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"-x0\0X0\0") };

static mut fmtinfo: [et_info; 23] = [
    et_info {
        fmttype: 's' as i32 as ::core::ffi::c_char,
        base: 0 as EtByte,
        flags: 4 as EtByte,
        type_0: etSTRING as EtByte,
        charset: 0 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 1 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'E' as i32 as ::core::ffi::c_char,
        base: 0 as EtByte,
        flags: 1 as EtByte,
        type_0: etEXP as EtByte,
        charset: 14 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'u' as i32 as ::core::ffi::c_char,
        base: 10 as EtByte,
        flags: 0 as EtByte,
        type_0: etDECIMAL as EtByte,
        charset: 0 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 3 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'G' as i32 as ::core::ffi::c_char,
        base: 0 as EtByte,
        flags: 1 as EtByte,
        type_0: etGENERIC as EtByte,
        charset: 14 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'w' as i32 as ::core::ffi::c_char,
        base: 0 as EtByte,
        flags: 4 as EtByte,
        type_0: etESCAPE_w as EtByte,
        charset: 0 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'x' as i32 as ::core::ffi::c_char,
        base: 16 as EtByte,
        flags: 0 as EtByte,
        type_0: etRADIX as EtByte,
        charset: 16 as EtByte,
        prefix: 1 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'c' as i32 as ::core::ffi::c_char,
        base: 0 as EtByte,
        flags: 0 as EtByte,
        type_0: etCHARX as EtByte,
        charset: 0 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'z' as i32 as ::core::ffi::c_char,
        base: 0 as EtByte,
        flags: 4 as EtByte,
        type_0: etDYNSTRING as EtByte,
        charset: 0 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 6 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'd' as i32 as ::core::ffi::c_char,
        base: 10 as EtByte,
        flags: 1 as EtByte,
        type_0: etDECIMAL as EtByte,
        charset: 0 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'e' as i32 as ::core::ffi::c_char,
        base: 0 as EtByte,
        flags: 1 as EtByte,
        type_0: etEXP as EtByte,
        charset: 30 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'f' as i32 as ::core::ffi::c_char,
        base: 0 as EtByte,
        flags: 1 as EtByte,
        type_0: etFLOAT as EtByte,
        charset: 0 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'g' as i32 as ::core::ffi::c_char,
        base: 0 as EtByte,
        flags: 1 as EtByte,
        type_0: etGENERIC as EtByte,
        charset: 30 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'Q' as i32 as ::core::ffi::c_char,
        base: 0 as EtByte,
        flags: 4 as EtByte,
        type_0: etESCAPE_Q as EtByte,
        charset: 0 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'i' as i32 as ::core::ffi::c_char,
        base: 10 as EtByte,
        flags: 1 as EtByte,
        type_0: etDECIMAL as EtByte,
        charset: 0 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: '%' as i32 as ::core::ffi::c_char,
        base: 0 as EtByte,
        flags: 0 as EtByte,
        type_0: etPERCENT as EtByte,
        charset: 0 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 16 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'T' as i32 as ::core::ffi::c_char,
        base: 0 as EtByte,
        flags: 0 as EtByte,
        type_0: etTOKEN as EtByte,
        charset: 0 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'S' as i32 as ::core::ffi::c_char,
        base: 0 as EtByte,
        flags: 0 as EtByte,
        type_0: etSRCITEM as EtByte,
        charset: 0 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'X' as i32 as ::core::ffi::c_char,
        base: 16 as EtByte,
        flags: 0 as EtByte,
        type_0: etRADIX as EtByte,
        charset: 0 as EtByte,
        prefix: 4 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'n' as i32 as ::core::ffi::c_char,
        base: 0 as EtByte,
        flags: 0 as EtByte,
        type_0: etSIZE as EtByte,
        charset: 0 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'o' as i32 as ::core::ffi::c_char,
        base: 8 as EtByte,
        flags: 0 as EtByte,
        type_0: etRADIX as EtByte,
        charset: 0 as EtByte,
        prefix: 2 as EtByte,
        iNxt: 17 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'p' as i32 as ::core::ffi::c_char,
        base: 16 as EtByte,
        flags: 0 as EtByte,
        type_0: etPOINTER as EtByte,
        charset: 0 as EtByte,
        prefix: 1 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'q' as i32 as ::core::ffi::c_char,
        base: 0 as EtByte,
        flags: 4 as EtByte,
        type_0: etESCAPE_q as EtByte,
        charset: 0 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
    et_info {
        fmttype: 'r' as i32 as ::core::ffi::c_char,
        base: 10 as EtByte,
        flags: 1 as EtByte,
        type_0: etORDINAL as EtByte,
        charset: 0 as EtByte,
        prefix: 0 as EtByte,
        iNxt: 0 as ::core::ffi::c_char,
    },
];
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3StrAccumSetError(
    p: *mut crate::src::headers::sqliteInt_h::StrAccum,
    eError: crate::src::ext::rtree::rtree::U8_0,
) {
    (*p).accError = eError;
    if (*p).mxAlloc != 0 {
        sqlite3_str_reset(p);
    }
    if eError as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_TOOBIG {
        crate::src::src::util::sqlite3ErrorToParser(
            (*p).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            eError as ::core::ffi::c_int,
        );
    }
}

pub unsafe extern "C" fn getIntArg(
    p: *mut crate::src::headers::sqliteInt_h::PrintfArguments,
) -> crate::src::headers::sqlite3_h::Sqlite3Int64 {
    let __p_ref = unsafe { &mut *p };
    if __p_ref.nArg <= __p_ref.nUsed {
        return 0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
    }
    let fresh42 = __p_ref.nUsed;
    __p_ref.nUsed += 1;
    crate::src::src::vdbeapi::sqlite3_value_int64(*__p_ref.apArg.offset(fresh42 as isize))
}

pub unsafe extern "C" fn getDoubleArg(
    p: *mut crate::src::headers::sqliteInt_h::PrintfArguments,
) -> ::core::ffi::c_double {
    let __p_ref = unsafe { &mut *p };
    if __p_ref.nArg <= __p_ref.nUsed {
        return 0.0f64;
    }
    let fresh41 = __p_ref.nUsed;
    __p_ref.nUsed += 1;
    crate::src::src::vdbeapi::sqlite3_value_double(*__p_ref.apArg.offset(fresh41 as isize))
}

pub unsafe extern "C" fn getTextArg(
    p: *mut crate::src::headers::sqliteInt_h::PrintfArguments,
) -> *mut ::core::ffi::c_char {
    let __p_ref = unsafe { &mut *p };
    if __p_ref.nArg <= __p_ref.nUsed {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let fresh40 = __p_ref.nUsed;
    __p_ref.nUsed += 1;
    crate::src::src::vdbeapi::sqlite3_value_text(*__p_ref.apArg.offset(fresh40 as isize))
        as *mut ::core::ffi::c_char
}

unsafe extern "C" fn printfTempBuf(
    pAccum: *mut crate::src::headers::sqliteInt_h::sqlite3_str,
    n: crate::src::headers::sqlite3_h::Sqlite3Int64,
) -> *mut ::core::ffi::c_char {
    
    let __pAccum_ref = unsafe { &*pAccum };
    if __pAccum_ref.accError != 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if n > __pAccum_ref.nAlloc as crate::src::headers::sqlite3_h::Sqlite3Int64
        && n > __pAccum_ref.mxAlloc as crate::src::headers::sqlite3_h::Sqlite3Int64
    {
        sqlite3StrAccumSetError(
            pAccum as *mut crate::src::headers::sqliteInt_h::StrAccum,
            crate::src::headers::sqlite3_h::SQLITE_TOOBIG as crate::src::ext::rtree::rtree::U8_0,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let z: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3DbMallocRaw(
        __pAccum_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        n as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut ::core::ffi::c_char;
    if z.is_null() {
        sqlite3StrAccumSetError(
            pAccum as *mut crate::src::headers::sqliteInt_h::StrAccum,
            crate::src::headers::sqlite3_h::SQLITE_NOMEM as crate::src::ext::rtree::rtree::U8_0,
        );
    }
    z
}

pub const SQLITE_PRINT_BUF_SIZE: ::core::ffi::c_int = 70 as ::core::ffi::c_int;

pub const etBUFSIZE: ::core::ffi::c_int = SQLITE_PRINT_BUF_SIZE;

pub const SQLITE_FP_PRECISION_LIMIT: ::core::ffi::c_int = 100000000 as ::core::ffi::c_int;

#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3RecordErrorByteOffset(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    z: *const ::core::ffi::c_char,
) {
    
    
    
    if db.is_null() {
        return;
    }
    if (*db).errByteOffset != -(2 as ::core::ffi::c_int) {
        return;
    }
    let pParse: *const crate::src::headers::sqliteInt_h::Parse = (*db).pParse;
    if pParse.is_null() {
        return;
    }
    let zText: *const ::core::ffi::c_char = (*pParse).zTail;
    if zText.is_null() {
        return;
    }
    let zEnd: *const ::core::ffi::c_char = zText.offset((::libc::strlen
        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> crate::__stddef_size_t_h::SizeT)(
        zText,
    ) as isize) as *const ::core::ffi::c_char;
    if z as crate::src::headers::sqliteInt_h::Uptr
        >= zText as crate::src::headers::sqliteInt_h::Uptr
        && (z as crate::src::headers::sqliteInt_h::Uptr)
            < zEnd as crate::src::headers::sqliteInt_h::Uptr
    {
        (*db).errByteOffset = z.offset_from(zText) as ::core::ffi::c_long as ::core::ffi::c_int;
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3RecordErrorOffsetOfExpr(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut pExpr: *const crate::src::headers::sqliteInt_h::Expr,
) {
    while !pExpr.is_null()
        && ((*pExpr).flags
            & (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int)
                as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
            || (*pExpr).w.iOfst <= 0 as ::core::ffi::c_int)
    {
        pExpr = (*pExpr).pLeft;
    }
    if pExpr.is_null() {
        return;
    }
    if (*pExpr).flags & 0x40000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return;
    }
    (*db).errByteOffset = (*pExpr).w.iOfst;
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3StrAccumEnlarge(
    p: *mut crate::src::headers::sqliteInt_h::StrAccum,
    N: crate::src::ext::rtree::rtree::I64_0,
) -> ::core::ffi::c_int {
    let zNew: *mut ::core::ffi::c_char;
    if (*p).accError != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*p).mxAlloc == 0 as crate::src::ext::rtree::rtree::U32_0 {
        sqlite3StrAccumSetError(
            p,
            crate::src::headers::sqlite3_h::SQLITE_TOOBIG as crate::src::ext::rtree::rtree::U8_0,
        );
        return (*p)
            .nAlloc
            .wrapping_sub((*p).nChar)
            .wrapping_sub(1 as crate::src::ext::rtree::rtree::U32_0)
            as ::core::ffi::c_int;
    } else {
        let __p_ref = unsafe { &mut *p };
        let zOld: *mut ::core::ffi::c_char = if __p_ref.printfFlags as ::core::ffi::c_int
            & crate::src::headers::sqliteInt_h::SQLITE_PRINTF_MALLOCED
            != 0 as ::core::ffi::c_int
        {
            __p_ref.zText
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        let mut szNew: crate::src::ext::rtree::rtree::I64_0 = __p_ref.nChar
            as crate::src::ext::rtree::rtree::I64_0
            + N
            + 1 as crate::src::ext::rtree::rtree::I64_0;
        if szNew + __p_ref.nChar as crate::src::ext::rtree::rtree::I64_0
            <= __p_ref.mxAlloc as crate::src::ext::rtree::rtree::I64_0
        {
            szNew += __p_ref.nChar as crate::src::ext::rtree::rtree::I64_0;
        }
        if szNew > __p_ref.mxAlloc as crate::src::ext::rtree::rtree::I64_0 {
            sqlite3_str_reset(p);
            sqlite3StrAccumSetError(
                p,
                crate::src::headers::sqlite3_h::SQLITE_TOOBIG
                    as crate::src::ext::rtree::rtree::U8_0,
            );
            return 0 as ::core::ffi::c_int;
        } else {
            __p_ref.nAlloc = szNew as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
        }
        if !__p_ref.db.is_null() {
            zNew = crate::src::src::malloc::sqlite3DbRealloc(
                __p_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                zOld as *mut ::core::ffi::c_void,
                __p_ref.nAlloc as crate::src::ext::rtree::rtree::U64_0,
            ) as *mut ::core::ffi::c_char;
        } else {
            zNew = crate::src::src::malloc::sqlite3Realloc(
                zOld as *mut ::core::ffi::c_void,
                __p_ref.nAlloc as crate::src::ext::rtree::rtree::U64_0,
            ) as *mut ::core::ffi::c_char;
        }
        if !zNew.is_null() {
            if (__p_ref.printfFlags as ::core::ffi::c_int
                & crate::src::headers::sqliteInt_h::SQLITE_PRINTF_MALLOCED == 0 as ::core::ffi::c_int)
                && __p_ref.nChar > 0 as crate::src::ext::rtree::rtree::U32_0
            {
                ::core::ptr::copy_nonoverlapping(
                    __p_ref.zText as *const u8,
                    zNew as *mut u8,
                    __p_ref.nChar as usize,
                );
            }
            __p_ref.zText = zNew;
            __p_ref.nAlloc = crate::src::src::malloc::sqlite3DbMallocSize(
                __p_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                zNew as *const ::core::ffi::c_void,
            ) as crate::src::ext::rtree::rtree::U32_0;
            __p_ref.printfFlags = (__p_ref.printfFlags as ::core::ffi::c_int
                | crate::src::headers::sqliteInt_h::SQLITE_PRINTF_MALLOCED)
                as crate::src::ext::rtree::rtree::U8_0;
        } else {
            sqlite3_str_reset(p);
            sqlite3StrAccumSetError(
                p,
                crate::src::headers::sqlite3_h::SQLITE_NOMEM as crate::src::ext::rtree::rtree::U8_0,
            );
            return 0 as ::core::ffi::c_int;
        }
    }
    N as ::core::ffi::c_int
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_str_appendchar(
    p: *mut crate::src::headers::sqliteInt_h::sqlite3_str,
    mut N: ::core::ffi::c_int,
    c: ::core::ffi::c_char,
) {
    if (*p).nChar as crate::src::ext::rtree::rtree::I64_0
        + N as crate::src::ext::rtree::rtree::I64_0
        >= (*p).nAlloc as crate::src::ext::rtree::rtree::I64_0
        && {
            N = sqlite3StrAccumEnlarge(
                p as *mut crate::src::headers::sqliteInt_h::StrAccum,
                N as crate::src::ext::rtree::rtree::I64_0,
            );
            N <= 0 as ::core::ffi::c_int
        }
    {
        return;
    }
    loop {
        let fresh38 = N;
        N -= 1;
        if (fresh38 <= 0 as ::core::ffi::c_int) {
            break;
        }
        let __p_ref = unsafe { &mut *p };
        let fresh39 = __p_ref.nChar;
        __p_ref.nChar = __p_ref.nChar.wrapping_add(1);
        *__p_ref.zText.offset(fresh39 as isize) = c;
    }
}
#[inline(never)]
unsafe extern "C" fn enlargeAndAppend(
    p: *mut crate::src::headers::sqliteInt_h::StrAccum,
    z: *const ::core::ffi::c_char,
    mut N: ::core::ffi::c_int,
) {
    N = sqlite3StrAccumEnlarge(p, N as crate::src::ext::rtree::rtree::I64_0);
    if N > 0 as ::core::ffi::c_int {
        let __p_ref = unsafe { &mut *p };
        ::core::ptr::copy_nonoverlapping(
            z as *const u8,
            __p_ref.zText.offset(__p_ref.nChar as isize) as *mut ::core::ffi::c_char as *mut u8,
            N as usize,
        );
        __p_ref.nChar = __p_ref
            .nChar
            .wrapping_add(N as crate::src::ext::rtree::rtree::U32_0);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_str_append(
    p: *mut crate::src::headers::sqliteInt_h::sqlite3_str,
    z: *const ::core::ffi::c_char,
    N: ::core::ffi::c_int,
) {
    if (*p)
        .nChar
        .wrapping_add(N as crate::src::ext::rtree::rtree::U32_0)
        >= (*p).nAlloc
    {
        enlargeAndAppend(p as *mut crate::src::headers::sqliteInt_h::StrAccum, z, N);
    } else if N != 0 {
        let __p_ref = unsafe { &mut *p };
        __p_ref.nChar = __p_ref
            .nChar
            .wrapping_add(N as crate::src::ext::rtree::rtree::U32_0);
        ::core::ptr::copy_nonoverlapping(
            z as *const u8,
            __p_ref.zText.offset(
                __p_ref
                    .nChar
                    .wrapping_sub(N as crate::src::ext::rtree::rtree::U32_0)
                    as isize,
            ) as *mut ::core::ffi::c_char as *mut u8,
            N as usize,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_str_appendall(
    p: *mut crate::src::headers::sqliteInt_h::sqlite3_str,
    z: *const ::core::ffi::c_char,
) {
    sqlite3_str_append(p, z, crate::src::src::util::sqlite3Strlen30(z));
}

/// Single-pass SQLFUNC formatter: reads arguments directly from PrintfArguments
/// as format specifiers are encountered, avoiding the dangling-pointer problem
/// caused by pre-extracting all sqlite3_value_text pointers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_str_vappendf_sqlfunc(
    pAccum: *mut crate::src::headers::sqliteInt_h::sqlite3_str,
    fmt_start: *const ::core::ffi::c_char,
    pArgList: *mut crate::src::headers::sqliteInt_h::PrintfArguments,
) {
    // Reset nUsed so we consume from the start
    if !pArgList.is_null() {
        (*pArgList).nUsed = 0;
    }
    let src = DirectArgs::new(pArgList);
    let fmt = ::core::ffi::CStr::from_ptr(fmt_start)
        .to_str()
        .unwrap_or("");
    sqlite3_str_vappendf2(pAccum, fmt, src);
}
/// C FFI entry point — thin wrapper that converts raw pointer+len to a slice.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_str_vappendf_packed(
    pAccum: *mut crate::src::headers::sqliteInt_h::sqlite3_str,
    fmt_start: *const ::core::ffi::c_char,
    args: *const u64,
    nArgs: ::core::ffi::c_int,
) {
    let packed: &[u64] = if args.is_null() || nArgs <= 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(args, nArgs as usize)
    };
    sqlite3_str_vappendf_from_packed(pAccum, fmt_start, packed);
}
/// Non-variadic entry point for the normal (non-SQLFUNC) path.
/// Called from C wrappers that pre-extract va_args into a uint64_t slice.
/// Rust parses the format string to determine types and interprets each u64 accordingly.
pub unsafe fn sqlite3_str_vappendf_from_packed(
    pAccum: *mut crate::src::headers::sqliteInt_h::sqlite3_str,
    fmt_start: *const ::core::ffi::c_char,
    packed: &[u64],
) {
    let a = unpack_printf_args(fmt_start, packed);
    sqlite3_str_vappendf_args(pAccum, fmt_start, &a);
}

/// Format and append to StrAccum using pre-extracted PrintfArg arguments.
/// This is the VaList-free entry point — the real decoupled formatter.
///
/// # Safety
/// `pAccum` and `fmt` must be valid. `args` must match the format string.
pub unsafe fn sqlite3_str_vappendf_args(
    pAccum: *mut crate::src::headers::sqliteInt_h::sqlite3_str,
    fmt_start: *const ::core::ffi::c_char,
    args: &[PrintfArg],
) {
    let src = ExtractedArgs::new(args);
    let fmt = ::core::ffi::CStr::from_ptr(fmt_start)
        .to_str()
        .unwrap_or("");
    sqlite3_str_vappendf2(pAccum, fmt, src);
}

// ─── PrintfArgs trait: abstract argument source for the formatter ─────────
//
// Two implementations:
//   1. ExtractedArgs: reads from a &[PrintfArg] slice (VaList pre-extraction path)
//   2. DirectArgs: reads from PrintfArguments* via getIntArg/getTextArg/getDoubleArg (SQLFUNC path)
//
// DirectArgs is critical for SQLFUNC because sqlite3_value_text() may invalidate
// pointers returned by prior sqlite3_value_text() calls (shared buffer reuse).
// Pre-extracting all pointers in advance causes dangling pointer bugs.

pub trait PrintfArgs {
    unsafe fn get_int(&mut self) -> crate::src::headers::sqlite3_h::Sqlite3Int64;
    unsafe fn get_uint(&mut self) -> crate::src::headers::sqlite3_h::SqliteUint64;
    unsafe fn get_double(&mut self) -> ::core::ffi::c_double;
    unsafe fn get_str(&mut self) -> *mut ::core::ffi::c_char;
    unsafe fn get_char(&mut self) -> ::core::ffi::c_uint;
    /// For %c in SQLFUNC mode: get the raw text pointer for multi-byte UTF-8.
    unsafe fn get_charx_text(&mut self) -> *mut ::core::ffi::c_char;
    unsafe fn get_token(&mut self) -> *mut crate::src::headers::sqliteInt_h::Token;
    unsafe fn get_expr(&mut self) -> *mut crate::src::headers::sqliteInt_h::Expr;
    unsafe fn get_srcitem(&mut self) -> *mut crate::src::headers::sqliteInt_h::SrcItem;
    unsafe fn get_nout_ptr(&mut self) -> *mut ::core::ffi::c_int;
    /// Whether this source is in SQLFUNC direct mode (reads from PrintfArguments).
    fn is_direct(&self) -> bool;
    /// Get next raw PrintfArg (only valid for ExtractedArgs; returns None for DirectArgs).
    fn next_arg(&mut self) -> PrintfArg {
        PrintfArg::None
    }
}

// ─── ExtractedArgs: reads from pre-extracted &[PrintfArg] ────────────────

pub struct ExtractedArgs<'a> {
    args: &'a [PrintfArg],
    pos: usize,
}

impl<'a> ExtractedArgs<'a> {
    pub fn new(args: &'a [PrintfArg]) -> Self {
        Self { args, pos: 0 }
    }

    fn next_arg(&mut self) -> &PrintfArg {
        if self.pos < self.args.len() {
            let a = &self.args[self.pos];
            self.pos += 1;
            a
        } else {
            &PrintfArg::None
        }
    }
}

impl From<crate::src::ext::rtree::rtree::U64_0> for PrintfArg {
    fn from(v: crate::src::ext::rtree::rtree::U64_0) -> Self {
        PrintfArg::UInt(v)
    }
}

impl From<crate::src::ext::rtree::rtree::I64_0> for PrintfArg {
    fn from(v: crate::src::ext::rtree::rtree::I64_0) -> Self {
        PrintfArg::Int(v)
    }
}

impl From<::core::ffi::c_double> for PrintfArg {
    fn from(v: ::core::ffi::c_double) -> Self {
        PrintfArg::Double(v)
    }
}

impl From<*mut ::core::ffi::c_char> for PrintfArg {
    fn from(v: *mut ::core::ffi::c_char) -> Self {
        PrintfArg::Str(v)
    }
}

impl From<*const ::core::ffi::c_char> for PrintfArg {
    fn from(v: *const ::core::ffi::c_char) -> Self {
        PrintfArg::Str(v as *mut ::core::ffi::c_char)
    }
}

impl From<*const ::core::ffi::c_uchar> for PrintfArg {
    fn from(v: *const ::core::ffi::c_uchar) -> Self {
        PrintfArg::Str(v as *mut ::core::ffi::c_char)
    }
}

impl From<::core::ffi::c_int> for PrintfArg {
    fn from(v: ::core::ffi::c_int) -> Self {
        PrintfArg::Int(v as crate::src::ext::rtree::rtree::I64_0)
    }
}

impl From<::core::ffi::c_uint> for PrintfArg {
    fn from(v: ::core::ffi::c_uint) -> Self {
        PrintfArg::UInt(v as crate::src::headers::sqlite3_h::SqliteUint64)
    }
}

impl From<*mut crate::src::headers::sqliteInt_h::SrcItem> for PrintfArg {
    fn from(v: *mut crate::src::headers::sqliteInt_h::SrcItem) -> Self {
        PrintfArg::SrcItem(v)
    }
}

/// Create an ExtractedArgs cursor from inline values.
/// Usage: `printf_args!(val)` — auto-converts via From trait
#[macro_export]
macro_rules! printf_args {
    ($($arg:expr),* $(,)?) => {
        $crate::src::src::printf::ExtractedArgs::new(
            &[$($crate::src::src::printf::PrintfArg::from($arg)),*]
        )
    };
}
/// Like printf_args! but returns &[PrintfArg] slice directly.
#[macro_export]
macro_rules! printf_args_slice {
    ($($arg:expr),* $(,)?) => {
        &[$($crate::src::src::printf::PrintfArg::from($arg)),*] as &[$crate::src::src::printf::PrintfArg]
    };
}
impl<'a> PrintfArgs for ExtractedArgs<'a> {
    fn next_arg(&mut self) -> PrintfArg {
        if self.pos < self.args.len() {
            let a = self.args[self.pos].clone();
            self.pos += 1;
            a
        } else {
            PrintfArg::None
        }
    }

    unsafe fn get_int(&mut self) -> crate::src::headers::sqlite3_h::Sqlite3Int64 {
        match self.next_arg() {
            PrintfArg::Int(v) => *v,
            PrintfArg::UInt(v) => *v as crate::src::headers::sqlite3_h::Sqlite3Int64,
            PrintfArg::Char(v) => *v as crate::src::headers::sqlite3_h::Sqlite3Int64,
            _ => 0,
        }
    }

    unsafe fn get_uint(&mut self) -> crate::src::headers::sqlite3_h::SqliteUint64 {
        match self.next_arg() {
            PrintfArg::UInt(v) => *v,
            PrintfArg::Int(v) => *v as crate::src::headers::sqlite3_h::SqliteUint64,
            PrintfArg::Char(v) => *v as crate::src::headers::sqlite3_h::SqliteUint64,
            _ => 0,
        }
    }

    unsafe fn get_double(&mut self) -> ::core::ffi::c_double {
        match self.next_arg() {
            PrintfArg::Double(v) => *v,
            PrintfArg::Int(v) => *v as ::core::ffi::c_double,
            _ => 0.0,
        }
    }

    unsafe fn get_str(&mut self) -> *mut ::core::ffi::c_char {
        match self.next_arg() {
            PrintfArg::Str(p) => *p,
            _ => ::core::ptr::null_mut(),
        }
    }

    unsafe fn get_char(&mut self) -> ::core::ffi::c_uint {
        match self.next_arg() {
            PrintfArg::Char(v) => *v,
            PrintfArg::Int(v) => *v as ::core::ffi::c_uint,
            _ => 0,
        }
    }

    unsafe fn get_charx_text(&mut self) -> *mut ::core::ffi::c_char {
        match self.next_arg() {
            PrintfArg::Str(p) => *p,
            _ => ::core::ptr::null_mut(),
        }
    }

    unsafe fn get_token(&mut self) -> *mut crate::src::headers::sqliteInt_h::Token {
        match self.next_arg() {
            PrintfArg::Token(p) => *p,
            _ => ::core::ptr::null_mut(),
        }
    }

    unsafe fn get_expr(&mut self) -> *mut crate::src::headers::sqliteInt_h::Expr {
        match self.next_arg() {
            PrintfArg::Expr(p) => *p,
            _ => ::core::ptr::null_mut(),
        }
    }

    unsafe fn get_srcitem(&mut self) -> *mut crate::src::headers::sqliteInt_h::SrcItem {
        match self.next_arg() {
            PrintfArg::SrcItem(p) => *p,
            _ => ::core::ptr::null_mut(),
        }
    }

    unsafe fn get_nout_ptr(&mut self) -> *mut ::core::ffi::c_int {
        match self.next_arg() {
            PrintfArg::NOut(p) => *p,
            _ => ::core::ptr::null_mut(),
        }
    }

    fn is_direct(&self) -> bool {
        false
    }
}

// ─── DirectArgs: reads from PrintfArguments* on demand (SQLFUNC path) ────

pub struct DirectArgs {
    pArgList: *mut crate::src::headers::sqliteInt_h::PrintfArguments,
}

impl DirectArgs {
    pub fn new(pArgList: *mut crate::src::headers::sqliteInt_h::PrintfArguments) -> Self {
        Self { pArgList }
    }
}

impl PrintfArgs for DirectArgs {
    unsafe fn get_int(&mut self) -> crate::src::headers::sqlite3_h::Sqlite3Int64 {
        getIntArg(self.pArgList)
    }

    unsafe fn get_uint(&mut self) -> crate::src::headers::sqlite3_h::SqliteUint64 {
        getIntArg(self.pArgList) as crate::src::headers::sqlite3_h::SqliteUint64
    }

    unsafe fn get_double(&mut self) -> ::core::ffi::c_double {
        getDoubleArg(self.pArgList)
    }

    unsafe fn get_str(&mut self) -> *mut ::core::ffi::c_char {
        getTextArg(self.pArgList)
    }

    unsafe fn get_char(&mut self) -> ::core::ffi::c_uint {
        let ptr = getTextArg(self.pArgList);
        if !ptr.is_null() {
            *ptr as u8 as ::core::ffi::c_uint
        } else {
            0
        }
    }

    unsafe fn get_charx_text(&mut self) -> *mut ::core::ffi::c_char {
        getTextArg(self.pArgList)
    }

    unsafe fn get_token(&mut self) -> *mut crate::src::headers::sqliteInt_h::Token {
        ::core::ptr::null_mut()
    }

    unsafe fn get_expr(&mut self) -> *mut crate::src::headers::sqliteInt_h::Expr {
        ::core::ptr::null_mut()
    }

    unsafe fn get_srcitem(&mut self) -> *mut crate::src::headers::sqliteInt_h::SrcItem {
        ::core::ptr::null_mut()
    }

    unsafe fn get_nout_ptr(&mut self) -> *mut ::core::ffi::c_int {
        ::core::ptr::null_mut()
    }

    fn is_direct(&self) -> bool {
        true
    }
}

// ─── PrintfArg: typed intermediate representation for extracted VaList args ───
//
// sqlite3_str_vappendf interleaves two concerns:
//   1. Extracting typed args from VaList (or PrintfArguments for SQLFUNC)
//   2. Formatting those args and appending to StrAccum
//
// PrintfArg decouples concern #1 from #2. The extraction phase walks the format
// string and consumes VaList into a Vec<PrintfArg>. The formatting phase
// (sqlite3_str_vappendf_args) reads from the Vec — no VaList needed.

/// A single extracted printf argument, typed to match what the format specifier expects.
#[derive(Clone)]
pub enum PrintfArg {
    /// Signed 64-bit integer (covers %d, %i, %r with flag_long=2 or i64 args)
    Int(crate::src::ext::rtree::rtree::I64_0),
    /// Unsigned 64-bit integer (covers %u, %x, %X, %o, %p with flag_long=2 or u64 args)
    UInt(crate::src::headers::sqlite3_h::SqliteUint64),
    /// Double-precision float (covers %f, %e, %g, %G)
    Double(::core::ffi::c_double),
    /// C string pointer (covers %s, %z, %q, %Q, %w). Caller keeps ownership unless %z.
    Str(*mut ::core::ffi::c_char),
    /// Character value (covers %c)
    Char(::core::ffi::c_uint),
    /// Token pointer (covers %T with INTERNAL flag, #T variant)
    Token(*mut crate::src::headers::sqliteInt_h::Token),
    /// Expr pointer (covers %T with INTERNAL flag, non-#T variant)
    Expr(*mut crate::src::headers::sqliteInt_h::Expr),
    /// SrcItem pointer (covers %S with INTERNAL flag)
    SrcItem(*mut crate::src::headers::sqliteInt_h::SrcItem),
    /// Write-back pointer for %n (stores characters written so far)
    NOut(*mut ::core::ffi::c_int),
    /// No argument consumed (covers %%)
    None,
}
/// Parsed format specifier — carries all flag/width/precision/type info needed
/// to both extract the right VaList arg and format it later.
#[derive(Clone)]
pub struct PrintfSpec {
    pub xtype: ::core::ffi::c_int,
    pub infop_idx: ::core::ffi::c_int,
    pub flag_leftjustify: bool,
    pub flag_prefix: ::core::ffi::c_char,
    pub flag_alternateform: bool,
    pub flag_altform2: bool,
    pub flag_zeropad: bool,
    pub flag_long: u8,
    pub cThousand: u8,
    pub width: ::core::ffi::c_int,
    pub precision: ::core::ffi::c_int,
    /// Byte offset into the format string where the literal text before this spec starts
    pub literal_start: usize,
    /// Byte offset where the literal text ends (== where '%' is)
    pub literal_end: usize,
}
// extract_printf_args removed — VaList extraction now happens in C (c_code/printf_c.c).
// Rust receives pre-packed uint64_t[] via unpack_printf_args.

/// Walk a printf format string, interpret each u64 from `packed` based on the specifier type.
/// Same logic as extract_printf_args but reads from a pre-extracted uint64_t array
/// (produced by C wrappers) instead of VaList.
pub unsafe fn unpack_printf_args(
    fmt_start: *const ::core::ffi::c_char,
    packed: &[u64],
) -> Vec<PrintfArg> {
    let mut args: Vec<PrintfArg> = Vec::new();
    let mut fmt = fmt_start;
    let mut pi = 0usize; // index into packed[]

    loop {
        let c = *fmt as ::core::ffi::c_int;
        if c == 0 {
            break;
        }
        if c != '%' as i32 {
            fmt = ::libc::strchr(fmt, '%' as i32);
            if fmt.is_null() {
                break;
            }
        }
        fmt = fmt.offset(1); // skip '%'
        let mut c = *fmt as ::core::ffi::c_int;
        if c == 0 {
            break;
        }

        let mut flag_alternateform = false;
        let flag_long: u8 = 0;
        let mut done = false;

        loop {
            match c as u8 {
                b'-' | b'+' | b' ' | b'!' | b'0' | b',' => {}
                b'#' => flag_alternateform = true,
                b'l' => {
                    fmt = fmt.offset(1);
                    c = *fmt as ::core::ffi::c_int;
                    if c == 'l' as i32 {
                        fmt = fmt.offset(1);
                        c = *fmt as ::core::ffi::c_int;
                    }
                    done = true;
                }
                b'1'..=b'9' => {
                    loop {
                        fmt = fmt.offset(1);
                        c = *fmt as ::core::ffi::c_int;
                        if !(c >= '0' as i32 && c <= '9' as i32) {
                            break;
                        }
                    }
                    if c != '.' as i32 && c != 'l' as i32 {
                        done = true;
                    } else {
                        fmt = fmt.offset(-1);
                    }
                }
                b'*' => {
                    // Width from args
                    if pi < packed.len() {
                        args.push(PrintfArg::Int(
                            packed[pi] as crate::src::ext::rtree::rtree::I64_0,
                        ));
                        pi += 1;
                    }
                    c = *fmt.offset(1) as ::core::ffi::c_int;
                    if c != '.' as i32 && c != 'l' as i32 {
                        fmt = fmt.offset(1);
                        c = *fmt as ::core::ffi::c_int;
                        done = true;
                    }
                }
                b'.' => {
                    fmt = fmt.offset(1);
                    c = *fmt as ::core::ffi::c_int;
                    if c == '*' as i32 {
                        if pi < packed.len() {
                            args.push(PrintfArg::Int(
                                packed[pi] as crate::src::ext::rtree::rtree::I64_0,
                            ));
                            pi += 1;
                        }
                        fmt = fmt.offset(1);
                        c = *fmt as ::core::ffi::c_int;
                    } else {
                        while c >= '0' as i32 && c <= '9' as i32 {
                            fmt = fmt.offset(1);
                            c = *fmt as ::core::ffi::c_int;
                        }
                    }
                    if c == 'l' as i32 {
                        fmt = fmt.offset(-1);
                    } else {
                        done = true;
                    }
                }
                _ => {
                    done = true;
                }
            }
            if done {
                break;
            }
            fmt = fmt.offset(1);
            c = *fmt as ::core::ffi::c_int;
            if c == 0 {
                break;
            }
        }

        // Lookup specifier in fmtinfo table
        let mut idx = (c as u32 % 23) as ::core::ffi::c_int;
        let (xtype, infop_idx) = if fmtinfo[idx as usize].fmttype as ::core::ffi::c_int == c || {
            idx = fmtinfo[idx as usize].iNxt as ::core::ffi::c_int;
            fmtinfo[idx as usize].fmttype as ::core::ffi::c_int == c
        } {
            (fmtinfo[idx as usize].type_0 as ::core::ffi::c_int, idx)
        } else {
            (etINVALID, 0)
        };

        let arg = if pi < packed.len() {
            let v = packed[pi];
            pi += 1;
            match xtype {
                etFLOAT | etEXP | etGENERIC => PrintfArg::Double(f64::from_bits(v)),
                etCHARX => PrintfArg::Char(v as u32),
                etSTRING | etDYNSTRING | etESCAPE_q | etESCAPE_Q | etESCAPE_w => {
                    PrintfArg::Str(v as usize as *mut ::core::ffi::c_char)
                }
                etTOKEN => {
                    if flag_alternateform {
                        PrintfArg::Expr(v as usize as *mut crate::src::headers::sqliteInt_h::Expr)
                    } else {
                        PrintfArg::Token(v as usize as *mut crate::src::headers::sqliteInt_h::Token)
                    }
                }
                etSRCITEM => {
                    PrintfArg::SrcItem(v as usize as *mut crate::src::headers::sqliteInt_h::SrcItem)
                }
                etSIZE => PrintfArg::NOut(v as usize as *mut ::core::ffi::c_int),
                etPOINTER => PrintfArg::UInt(v),
                etORDINAL | etRADIX | etDECIMAL => {
                    let signed =
                        fmtinfo[infop_idx as usize].flags as ::core::ffi::c_int & FLAG_SIGNED != 0;
                    if signed {
                        PrintfArg::Int(v as crate::src::ext::rtree::rtree::I64_0)
                    } else {
                        PrintfArg::UInt(v)
                    }
                }
                etPERCENT => {
                    pi -= 1; // %% consumes no arg
                    PrintfArg::None
                }
                _ => {
                    pi -= 1;
                    PrintfArg::None
                }
            }
        } else {
            PrintfArg::None
        };

        if !matches!(arg, PrintfArg::None) {
            args.push(arg);
        }

        if *fmt != 0 {
            fmt = fmt.offset(1);
        }
    }

    args
}

// Signed/unsigned integer extraction is inlined in extract_printf_args
// to avoid VaList borrow issues with helper functions.
#[inline(never)]
unsafe extern "C" fn strAccumFinishRealloc(
    p: *mut crate::src::headers::sqliteInt_h::StrAccum,
) -> *mut ::core::ffi::c_char {
    
    let __p_ref = unsafe { &mut *p };
    let zText: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3DbMallocRaw(
        __p_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        (1 as crate::src::ext::rtree::rtree::U64_0)
            .wrapping_add(__p_ref.nChar as crate::src::ext::rtree::rtree::U64_0),
    ) as *mut ::core::ffi::c_char;
    if !zText.is_null() {
        ::core::ptr::copy_nonoverlapping(
            __p_ref.zText as *const u8,
            zText as *mut u8,
            __p_ref
                .nChar
                .wrapping_add(1 as crate::src::ext::rtree::rtree::U32_0) as usize,
        );
        __p_ref.printfFlags = (__p_ref.printfFlags as ::core::ffi::c_int
            | crate::src::headers::sqliteInt_h::SQLITE_PRINTF_MALLOCED)
            as crate::src::ext::rtree::rtree::U8_0;
    } else {
        sqlite3StrAccumSetError(
            p,
            crate::src::headers::sqlite3_h::SQLITE_NOMEM as crate::src::ext::rtree::rtree::U8_0,
        );
    }
    __p_ref.zText = zText;
    zText
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3StrAccumFinish(
    p: *mut crate::src::headers::sqliteInt_h::StrAccum,
) -> *mut ::core::ffi::c_char {
    if !(*p).zText.is_null() {
        let __p_ref = unsafe { &mut *p };
        *__p_ref.zText.offset(__p_ref.nChar as isize) = 0 as ::core::ffi::c_char;
        if __p_ref.mxAlloc > 0 as crate::src::ext::rtree::rtree::U32_0
            && (__p_ref.printfFlags as ::core::ffi::c_int
                & crate::src::headers::sqliteInt_h::SQLITE_PRINTF_MALLOCED == 0 as ::core::ffi::c_int)
        {
            return strAccumFinishRealloc(p);
        }
    }
    (*p).zText
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ResultStrAccum(
    pCtx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    p: *mut crate::src::headers::sqliteInt_h::StrAccum,
) {
    if (*p).accError != 0 {
        crate::src::src::vdbeapi::sqlite3_result_error_code(
            pCtx,
            (*p).accError as ::core::ffi::c_int,
        );
        sqlite3_str_reset(p);
    } else if (*p).printfFlags as ::core::ffi::c_int
        & crate::src::headers::sqliteInt_h::SQLITE_PRINTF_MALLOCED
        != 0 as ::core::ffi::c_int
    {
        crate::src::src::vdbeapi::sqlite3_result_text(
            pCtx,
            (*p).zText,
            (*p).nChar as ::core::ffi::c_int,
            Some(
                crate::src::src::rowset::sqlite3RowSetClear
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
        );
    } else {
        crate::src::src::vdbeapi::sqlite3_result_text(
            pCtx,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            crate::src::headers::sqlite3_h::SQLITE_STATIC,
        );
        sqlite3_str_reset(p);
    };
}

static mut sqlite3OomStr: crate::src::headers::sqliteInt_h::sqlite3_str =
    crate::src::headers::sqliteInt_h::sqlite3_str {
        db: ::core::ptr::null::<crate::src::headers::sqliteInt_h::sqlite3>()
            as *mut crate::src::headers::sqliteInt_h::sqlite3,
        zText: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        nAlloc: 0 as crate::src::ext::rtree::rtree::U32_0,
        mxAlloc: 0 as crate::src::ext::rtree::rtree::U32_0,
        nChar: 0 as crate::src::ext::rtree::rtree::U32_0,
        accError: crate::src::headers::sqlite3_h::SQLITE_NOMEM
            as crate::src::ext::rtree::rtree::U8_0,
        printfFlags: 0 as crate::src::ext::rtree::rtree::U8_0,
    };
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_str_finish(
    p: *mut crate::src::headers::sqliteInt_h::sqlite3_str,
) -> *mut ::core::ffi::c_char {
    let z: *mut ::core::ffi::c_char;
    if !p.is_null() && p != &raw mut sqlite3OomStr {
        z = sqlite3StrAccumFinish(p as *mut crate::src::headers::sqliteInt_h::StrAccum);
        crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
    } else {
        z = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    z
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_str_errcode(
    p: *mut crate::src::headers::sqliteInt_h::sqlite3_str,
) -> ::core::ffi::c_int {
    if !p.is_null() {
        (*p).accError as ::core::ffi::c_int
    } else {
        crate::src::headers::sqlite3_h::SQLITE_NOMEM
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_str_length(
    p: *mut crate::src::headers::sqliteInt_h::sqlite3_str,
) -> ::core::ffi::c_int {
    (if !p.is_null() {
        (*p).nChar
    } else {
        0 as crate::src::ext::rtree::rtree::U32_0
    }) as ::core::ffi::c_int
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_str_value(
    p: *mut crate::src::headers::sqliteInt_h::sqlite3_str,
) -> *mut ::core::ffi::c_char {
    let __p_ref = unsafe { &mut *p };
    if p.is_null() || __p_ref.nChar == 0 as crate::src::ext::rtree::rtree::U32_0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *__p_ref.zText.offset(__p_ref.nChar as isize) = 0 as ::core::ffi::c_char;
    __p_ref.zText
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_str_reset(p: *mut crate::src::headers::sqliteInt_h::StrAccum) {
    let __p_ref = unsafe { &mut *p };
    if __p_ref.printfFlags as ::core::ffi::c_int
        & crate::src::headers::sqliteInt_h::SQLITE_PRINTF_MALLOCED
        != 0 as ::core::ffi::c_int
    {
        crate::src::src::malloc::sqlite3DbFree(
            __p_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            __p_ref.zText as *mut ::core::ffi::c_void,
        );
        __p_ref.printfFlags = (__p_ref.printfFlags as ::core::ffi::c_int
            & !crate::src::headers::sqliteInt_h::SQLITE_PRINTF_MALLOCED)
            as crate::src::ext::rtree::rtree::U8_0;
    }
    __p_ref.nAlloc = 0 as crate::src::ext::rtree::rtree::U32_0;
    __p_ref.nChar = 0 as crate::src::ext::rtree::rtree::U32_0;
    __p_ref.zText = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3StrAccumInit(
    p: *mut crate::src::headers::sqliteInt_h::StrAccum,
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zBase: *mut ::core::ffi::c_char,
    n: ::core::ffi::c_int,
    mx: ::core::ffi::c_int,
) {
    let __p_ref = unsafe { &mut *p };
    __p_ref.zText = zBase;
    __p_ref.db = db;
    __p_ref.nAlloc = n as crate::src::ext::rtree::rtree::U32_0;
    __p_ref.mxAlloc = mx as crate::src::ext::rtree::rtree::U32_0;
    __p_ref.nChar = 0 as crate::src::ext::rtree::rtree::U32_0;
    __p_ref.accError = 0 as crate::src::ext::rtree::rtree::U8_0;
    __p_ref.printfFlags = 0 as crate::src::ext::rtree::rtree::U8_0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_str_new(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
) -> *mut crate::src::headers::sqliteInt_h::sqlite3_str {
    let mut p: *mut crate::src::headers::sqliteInt_h::sqlite3_str =
        crate::src::src::malloc::sqlite3_malloc64(::core::mem::size_of::<
            crate::src::headers::sqliteInt_h::sqlite3_str,
        >()
            as crate::src::headers::sqlite3_h::Sqlite3Uint64)
            as *mut crate::src::headers::sqliteInt_h::sqlite3_str;
    if !p.is_null() {
        sqlite3StrAccumInit(
            p as *mut crate::src::headers::sqliteInt_h::StrAccum,
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            if !db.is_null() {
                (*db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_LENGTH as usize]
            } else {
                crate::sqliteLimit_h::SQLITE_MAX_LENGTH
            },
        );
    } else {
        p = &raw mut sqlite3OomStr;
    }
    p
}
/// Non-variadic sqlite3VMPrintf — takes pre-extracted PrintfArg slice.
pub unsafe fn sqlite3VMPrintf_args(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zFormat: *const ::core::ffi::c_char,
    args: &[PrintfArg],
) -> *mut ::core::ffi::c_char {
    let mut zBase: [::core::ffi::c_char; 70] = [0; 70];
    let mut acc: crate::src::headers::sqliteInt_h::StrAccum = unsafe { ::core::mem::zeroed() };
    sqlite3StrAccumInit(
        &raw mut acc,
        db,
        &raw mut zBase as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 70]>() as ::core::ffi::c_int,
        (*db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_LENGTH as usize],
    );
    acc.printfFlags = crate::src::headers::sqliteInt_h::SQLITE_PRINTF_INTERNAL
        as crate::src::ext::rtree::rtree::U8_0;
    sqlite3_str_vappendf_args(&raw mut acc, zFormat, args);
    let z = sqlite3StrAccumFinish(&raw mut acc);
    if acc.accError as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_NOMEM {
        crate::src::src::malloc::sqlite3OomFault(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        );
    }
    z
}

/// Non-variadic sqlite3_snprintf — takes pre-extracted PrintfArg slice.
pub unsafe fn sqlite3_snprintf_args(
    n: ::core::ffi::c_int,
    zBuf: *mut ::core::ffi::c_char,
    zFormat: *const ::core::ffi::c_char,
    args: &[PrintfArg],
) -> *mut ::core::ffi::c_char {
    let mut acc: crate::src::headers::sqliteInt_h::StrAccum = unsafe { ::core::mem::zeroed() };
    if n <= 0 as ::core::ffi::c_int {
        return zBuf;
    }
    sqlite3StrAccumInit(
        &raw mut acc,
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>(),
        zBuf,
        n,
        0 as ::core::ffi::c_int,
    );
    sqlite3_str_vappendf_args(
        &raw mut acc as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
        zFormat,
        args,
    );
    sqlite3StrAccumFinish(&raw mut acc);
    zBuf
}

// sqlite3MPrintf moved to printf_c_variadic.rs
pub use crate::src::printf_c_variadic::sqlite3MPrintf_args;

/// Non-variadic sqlite3_vmprintf — takes pre-extracted PrintfArg slice.
pub unsafe fn sqlite3_vmprintf_args(
    zFormat: *const ::core::ffi::c_char,
    args: &[PrintfArg],
) -> *mut ::core::ffi::c_char {
    let mut zBase: [::core::ffi::c_char; 70] = [0; 70];
    let mut acc: crate::src::headers::sqliteInt_h::StrAccum = unsafe { ::core::mem::zeroed() };
    if crate::src::src::main::sqlite3_initialize() != 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    sqlite3StrAccumInit(
        &raw mut acc,
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>(),
        &raw mut zBase as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 70]>() as ::core::ffi::c_int,
        crate::sqliteLimit_h::SQLITE_MAX_LENGTH,
    );
    sqlite3_str_vappendf_args(&raw mut acc, zFormat, args);
    sqlite3StrAccumFinish(&raw mut acc)
}

pub unsafe fn renderLogMsg_args(
    iErrCode: ::core::ffi::c_int,
    zFormat: *const ::core::ffi::c_char,
    args: &[PrintfArg],
) {
    let mut acc: crate::src::headers::sqliteInt_h::StrAccum = ::core::mem::zeroed();
    let mut zMsg: [::core::ffi::c_char; 700] = [0; 700];
    sqlite3StrAccumInit(
        &raw mut acc,
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>(),
        &raw mut zMsg as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 700]>() as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    sqlite3_str_vappendf_args(&raw mut acc, zFormat, args);
    crate::src::src::global::sqlite3Config
        .xLog
        .expect("non-null function pointer")(
        crate::src::src::global::sqlite3Config.pLogArg,
        iErrCode,
        sqlite3StrAccumFinish(&raw mut acc),
    );
}

pub unsafe extern "C" fn sqlite3RCStrRef(
    z: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut p: *mut crate::src::headers::sqliteInt_h::RCStr =
        z as *mut crate::src::headers::sqliteInt_h::RCStr;
    p = p.offset(-1);
    (*p).nRCRef = (*p).nRCRef.wrapping_add(1);
    z
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3RCStrUnref(z: *mut ::core::ffi::c_void) {
    let mut p: *mut crate::src::headers::sqliteInt_h::RCStr =
        z as *mut crate::src::headers::sqliteInt_h::RCStr;
    p = p.offset(-1);
    if (*p).nRCRef >= 2 as crate::src::ext::rtree::rtree::U64_0 {
        (*p).nRCRef = (*p).nRCRef.wrapping_sub(1);
    } else {
        crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
    };
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3RCStrNew(
    N: crate::src::ext::rtree::rtree::U64_0,
) -> *mut ::core::ffi::c_char {
    let p: *mut crate::src::headers::sqliteInt_h::RCStr =
        crate::src::src::malloc::sqlite3_malloc64(
            (N as crate::src::headers::sqlite3_h::Sqlite3Uint64)
                .wrapping_add(
                    ::core::mem::size_of::<crate::src::headers::sqliteInt_h::RCStr>()
                        as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                )
                .wrapping_add(1 as crate::src::headers::sqlite3_h::Sqlite3Uint64),
        ) as *mut crate::src::headers::sqliteInt_h::RCStr;
    if p.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    (*p).nRCRef = 1 as crate::src::ext::rtree::rtree::U64_0;
    p.offset(1_isize) as *mut crate::src::headers::sqliteInt_h::RCStr as *mut ::core::ffi::c_char
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3RCStrResize(
    z: *mut ::core::ffi::c_char,
    N: crate::src::ext::rtree::rtree::U64_0,
) -> *mut ::core::ffi::c_char {
    let mut p: *mut crate::src::headers::sqliteInt_h::RCStr =
        z as *mut crate::src::headers::sqliteInt_h::RCStr;
    
    p = p.offset(-1);
    let pNew: *mut crate::src::headers::sqliteInt_h::RCStr = crate::src::src::malloc::sqlite3_realloc64(
        p as *mut ::core::ffi::c_void,
        (N as crate::src::headers::sqlite3_h::Sqlite3Uint64)
            .wrapping_add(
                ::core::mem::size_of::<crate::src::headers::sqliteInt_h::RCStr>()
                    as crate::src::headers::sqlite3_h::Sqlite3Uint64,
            )
            .wrapping_add(1 as crate::src::headers::sqlite3_h::Sqlite3Uint64),
    ) as *mut crate::src::headers::sqliteInt_h::RCStr;
    if pNew.is_null() {
        crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    } else {
        pNew.offset(1_isize) as *mut crate::src::headers::sqliteInt_h::RCStr
            as *mut ::core::ffi::c_char
    }
}

pub unsafe fn sqlite3_str_vappendf2<T: PrintfArgs>(
    pAccum: *mut crate::src::headers::sqliteInt_h::sqlite3_str,
    fmt: &str,
    mut cursor: T,
) {
    let mut current_block: u64;
    let mut c: ::core::ffi::c_int;
    let mut bufpt: *mut ::core::ffi::c_char;
    let mut precision: ::core::ffi::c_int;
    let mut length: ::core::ffi::c_int = 0;
    let mut idx: ::core::ffi::c_int;
    let mut width: ::core::ffi::c_int;
    let mut flag_leftjustify: EtByte;
    let mut flag_prefix: EtByte;
    let mut flag_alternateform: EtByte;
    let mut flag_altform2: EtByte;
    let mut flag_zeropad: EtByte;
    let mut _flag_long: EtByte = 0;
    let mut done: EtByte;
    let mut cThousand: EtByte;
    let mut xtype: EtByte;
    let mut prefix: ::core::ffi::c_char;
    let mut longvalue: crate::src::headers::sqlite3_h::SqliteUint64;
    let mut realvalue: ::core::ffi::c_double;
    let mut infop: *const et_info;
    let mut zOut: *mut ::core::ffi::c_char;
    let mut nOut: ::core::ffi::c_int;
    let mut zExtra: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut exp: ::core::ffi::c_int;
    let mut e2: ::core::ffi::c_int;
    let mut flag_dp: EtByte;
    let mut flag_rtz: EtByte;
    let bArgList: bool = cursor.is_direct();
    let mut buf: [::core::ffi::c_char; 70] = [0; 70];
    bufpt = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fi: usize = 0;
    loop {
        let remaining = &fmt[fi..];
        if remaining.is_empty() {
            break;
        }
        let ch = remaining.chars().next().unwrap();
        if ch != '%' {
            // Find next '%' or end of string
            let literal_len = remaining.find('%').unwrap_or(remaining.len());
            sqlite3_str_append(
                pAccum,
                remaining.as_ptr() as *const ::core::ffi::c_char,
                literal_len as ::core::ffi::c_int,
            );
            fi += literal_len;
            if fi >= fmt.len() {
                break;
            }
            continue;
        }
        fi += 1;
        if fi >= fmt.len() {
            sqlite3_str_append(
                pAccum,
                b"%\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
            break;
        }
        c = fmt[fi..].chars().next().unwrap_or('\0') as ::core::ffi::c_int;
        if c == 0 as ::core::ffi::c_int {
            sqlite3_str_append(
                pAccum,
                b"%\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
            break;
        } else {
            flag_zeropad = 0 as EtByte;
            flag_altform2 = flag_zeropad;
            flag_alternateform = flag_altform2;
            cThousand = flag_alternateform;
            flag_prefix = cThousand;
            flag_leftjustify = flag_prefix;
            done = 0 as EtByte;
            width = 0 as ::core::ffi::c_int;
            _flag_long = 0 as EtByte;
            precision = -(1 as ::core::ffi::c_int);
            loop {
                match c as u8 {
                    b'-' => {
                        flag_leftjustify = 1 as EtByte;
                    }
                    b'+' => {
                        flag_prefix = '+' as i32 as EtByte;
                    }
                    b' ' => {
                        flag_prefix = ' ' as i32 as EtByte;
                    }
                    b'#' => {
                        flag_alternateform = 1 as EtByte;
                    }
                    b'!' => {
                        flag_altform2 = 1 as EtByte;
                    }
                    b'0' => {
                        flag_zeropad = 1 as EtByte;
                    }
                    b',' => {
                        cThousand = ',' as i32 as EtByte;
                    }
                    b'l' => {
                        _flag_long = 1 as EtByte;
                        fi += 1;
                        c = fmt[fi..].chars().next().unwrap_or('\0') as ::core::ffi::c_int;
                        if c == 'l' as i32 {
                            fi += 1;
                            c = fmt[fi..].chars().next().unwrap_or('\0') as ::core::ffi::c_int;
                            _flag_long = 2 as EtByte;
                        }
                        done = 1 as EtByte;
                    }
                    b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                        let mut wx: ::core::ffi::c_uint = (c - '0' as i32) as ::core::ffi::c_uint;
                        loop {
                            fi += 1;
                            c = fmt[fi..].chars().next().unwrap_or('\0') as ::core::ffi::c_int;
                            if !(c >= '0' as i32 && c <= '9' as i32) {
                                break;
                            }
                            wx = wx
                                .wrapping_mul(10 as ::core::ffi::c_uint)
                                .wrapping_add(c as ::core::ffi::c_uint)
                                .wrapping_sub('0' as i32 as ::core::ffi::c_uint);
                        }
                        width = (wx & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint)
                            as ::core::ffi::c_int;
                        if c != '.' as i32 && c != 'l' as i32 {
                            done = 1 as EtByte;
                        } else {
                            fi -= 1;
                        }
                    }
                    b'*' => {
                        width = cursor.get_int() as ::core::ffi::c_int;
                        if width < 0 as ::core::ffi::c_int {
                            flag_leftjustify = 1 as EtByte;
                            width = if width >= -(2147483647 as ::core::ffi::c_int) {
                                -width
                            } else {
                                0 as ::core::ffi::c_int
                            };
                        }
                        c = fmt[fi + 1..].chars().next().unwrap_or('\0') as ::core::ffi::c_int;
                        if c != '.' as i32 && c != 'l' as i32 {
                            fi += 1;
                            c = fmt[fi..].chars().next().unwrap_or('\0') as ::core::ffi::c_int;
                            done = 1 as EtByte;
                        }
                    }
                    b'.' => {
                        fi += 1;
                        c = fmt[fi..].chars().next().unwrap_or('\0') as ::core::ffi::c_int;
                        if c == '*' as i32 {
                            precision = cursor.get_int() as ::core::ffi::c_int;
                            if precision < 0 as ::core::ffi::c_int {
                                precision = if precision >= -(2147483647 as ::core::ffi::c_int) {
                                    -precision
                                } else {
                                    -(1 as ::core::ffi::c_int)
                                };
                            }
                            fi += 1;
                            c = fmt[fi..].chars().next().unwrap_or('\0') as ::core::ffi::c_int;
                        } else {
                            let mut px: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                            while c >= '0' as i32 && c <= '9' as i32 {
                                px = px
                                    .wrapping_mul(10 as ::core::ffi::c_uint)
                                    .wrapping_add(c as ::core::ffi::c_uint)
                                    .wrapping_sub('0' as i32 as ::core::ffi::c_uint);
                                fi += 1;
                                c = fmt[fi..].chars().next().unwrap_or('\0') as ::core::ffi::c_int;
                            }
                            precision = (px
                                & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint)
                                as ::core::ffi::c_int;
                        }
                        if c == 'l' as i32 {
                            fi -= 1;
                        } else {
                            done = 1 as EtByte;
                        }
                    }
                    _ => {
                        done = 1 as EtByte;
                    }
                }
                if !(done == 0 && {
                    fi += 1;
                    c = fmt[fi..].chars().next().unwrap_or('\0') as ::core::ffi::c_int;
                    c != 0 as ::core::ffi::c_int
                }) {
                    break;
                }
            }
            idx = (c as ::core::ffi::c_uint).wrapping_rem(23 as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
            if fmtinfo[idx as usize].fmttype as ::core::ffi::c_int == c || {
                idx = fmtinfo[idx as usize].iNxt as ::core::ffi::c_int;
                fmtinfo[idx as usize].fmttype as ::core::ffi::c_int == c
            } {
                infop =
                    (&raw const fmtinfo as *const et_info).offset(idx as isize) as *const et_info;
                xtype = (*infop).type_0;
            } else {
                infop = (&raw const fmtinfo as *const et_info).offset(0_isize) as *const et_info;
                xtype = etINVALID as EtByte;
            }
            match xtype as ::core::ffi::c_int {
                etPOINTER => {
                    _flag_long = (if ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize
                        == ::core::mem::size_of::<crate::src::ext::rtree::rtree::I64_0>() as usize
                    {
                        2 as ::core::ffi::c_int
                    } else if ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize
                        == ::core::mem::size_of::<::core::ffi::c_long>() as usize
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as EtByte;
                    current_block = 8988494922703612237;
                }
                etORDINAL | etRADIX => {
                    current_block = 8988494922703612237;
                }
                etDECIMAL => {
                    current_block = 9031895888649199432;
                }
                etFLOAT | etEXP | etGENERIC => {
                    let mut s: crate::src::headers::sqliteInt_h::FpDecode =
                        unsafe { ::core::mem::zeroed() };
                    let iRound: ::core::ffi::c_int;
                    let mut j: ::core::ffi::c_int;
                    realvalue = cursor.get_double();
                    if precision < 0 as ::core::ffi::c_int {
                        precision = 6 as ::core::ffi::c_int;
                    }
                    if precision > SQLITE_FP_PRECISION_LIMIT {
                        precision = SQLITE_FP_PRECISION_LIMIT;
                    }
                    if xtype as ::core::ffi::c_int == etFLOAT {
                        iRound = -precision;
                    } else if xtype as ::core::ffi::c_int == etGENERIC {
                        if precision == 0 as ::core::ffi::c_int {
                            precision = 1 as ::core::ffi::c_int;
                        }
                        iRound = precision;
                    } else {
                        iRound = precision + 1 as ::core::ffi::c_int;
                    }
                    crate::src::src::util::sqlite3FpDecode(
                        &raw mut s as *mut _ as *mut crate::src::headers::sqliteInt_h::FpDecode,
                        realvalue,
                        iRound,
                        if flag_altform2 as ::core::ffi::c_int != 0 {
                            26 as ::core::ffi::c_int
                        } else {
                            16 as ::core::ffi::c_int
                        },
                    );
                    if s.isSpecial != 0 {
                        if s.isSpecial as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                            bufpt = (if flag_zeropad as ::core::ffi::c_int != 0 {
                                b"null\0" as *const u8 as *const ::core::ffi::c_char
                            } else {
                                b"NaN\0" as *const u8 as *const ::core::ffi::c_char
                            }) as *mut ::core::ffi::c_char;
                            length = crate::src::src::util::sqlite3Strlen30(bufpt);
                            current_block = 11068715555910905014;
                        } else if flag_zeropad != 0 {
                            *s.z.offset(0_isize) = '9' as i32 as ::core::ffi::c_char;
                            s.iDP = 1000 as ::core::ffi::c_int;
                            s.n = 1 as ::core::ffi::c_int;
                            current_block = 12252098823794565961;
                        } else {
                            ::core::ptr::copy_nonoverlapping(
                                b"-Inf\0" as *const u8 as *const ::core::ffi::c_char,
                                &raw mut buf as *mut ::core::ffi::c_char,
                                5_usize,
                            );
                            bufpt = &raw mut buf as *mut ::core::ffi::c_char;
                            if (s.sign as ::core::ffi::c_int != '-' as i32) {
                                if flag_prefix != 0 {
                                    buf[0 as ::core::ffi::c_int as usize] =
                                        flag_prefix as ::core::ffi::c_char;
                                } else {
                                    bufpt = bufpt.offset(1);
                                }
                            }
                            length = crate::src::src::util::sqlite3Strlen30(bufpt);
                            current_block = 11068715555910905014;
                        }
                    } else {
                        current_block = 12252098823794565961;
                    }
                    match current_block {
                        11068715555910905014 => {}
                        _ => {
                            if s.sign as ::core::ffi::c_int == '-' as i32 {
                                if flag_alternateform as ::core::ffi::c_int != 0
                                    && flag_prefix == 0
                                    && xtype as ::core::ffi::c_int == etFLOAT
                                    && s.iDP <= iRound
                                {
                                    prefix = 0 as ::core::ffi::c_char;
                                } else {
                                    prefix = '-' as i32 as ::core::ffi::c_char;
                                }
                            } else {
                                prefix = flag_prefix as ::core::ffi::c_char;
                            }
                            exp = s.iDP - 1 as ::core::ffi::c_int;
                            if xtype as ::core::ffi::c_int == etGENERIC {
                                precision -= 1;
                                flag_rtz =
                                    (flag_alternateform == 0) as ::core::ffi::c_int as EtByte;
                                if exp < -(4 as ::core::ffi::c_int) || exp > precision {
                                    xtype = etEXP as EtByte;
                                } else {
                                    precision -= exp;
                                    xtype = etFLOAT as EtByte;
                                }
                            } else {
                                flag_rtz = flag_altform2;
                            }
                            if xtype as ::core::ffi::c_int == etEXP {
                                e2 = 0 as ::core::ffi::c_int;
                            } else {
                                e2 = s.iDP - 1 as ::core::ffi::c_int;
                            }
                            bufpt = &raw mut buf as *mut ::core::ffi::c_char;
                            let mut szBufNeeded: crate::src::ext::rtree::rtree::I64_0;
                            szBufNeeded = (if e2 > 0 as ::core::ffi::c_int {
                                e2
                            } else {
                                0 as ::core::ffi::c_int
                            })
                                as crate::src::ext::rtree::rtree::I64_0
                                + precision as crate::src::ext::rtree::rtree::I64_0
                                + width as crate::src::ext::rtree::rtree::I64_0
                                + 15 as crate::src::ext::rtree::rtree::I64_0;
                            if cThousand as ::core::ffi::c_int != 0 && e2 > 0 as ::core::ffi::c_int
                            {
                                szBufNeeded += ((e2 + 2 as ::core::ffi::c_int)
                                    / 3 as ::core::ffi::c_int)
                                    as crate::src::ext::rtree::rtree::I64_0;
                            }
                            if szBufNeeded > etBUFSIZE as crate::src::ext::rtree::rtree::I64_0 {
                                zExtra = printfTempBuf(
                                    pAccum,
                                    szBufNeeded as crate::src::headers::sqlite3_h::Sqlite3Int64,
                                );
                                bufpt = zExtra;
                                if bufpt.is_null() {
                                    return;
                                }
                            }
                            zOut = bufpt;
                            flag_dp = ((if precision > 0 as ::core::ffi::c_int {
                                1 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            }) | flag_alternateform as ::core::ffi::c_int
                                | flag_altform2 as ::core::ffi::c_int)
                                as EtByte;
                            if prefix != 0 {
                                let fresh0 = bufpt;
                                bufpt = bufpt.offset(1);
                                *fresh0 = prefix;
                            }
                            j = 0 as ::core::ffi::c_int;
                            if e2 < 0 as ::core::ffi::c_int {
                                let fresh1 = bufpt;
                                bufpt = bufpt.offset(1);
                                *fresh1 = '0' as i32 as ::core::ffi::c_char;
                            } else {
                                while e2 >= 0 as ::core::ffi::c_int {
                                    let fresh3 = bufpt;
                                    bufpt = bufpt.offset(1);
                                    *fresh3 = (if j < s.n {
                                        let fresh2 = j;
                                        j += 1;
                                        *s.z.offset(fresh2 as isize) as ::core::ffi::c_int
                                    } else {
                                        '0' as i32
                                    })
                                        as ::core::ffi::c_char;
                                    if cThousand as ::core::ffi::c_int != 0
                                        && e2 % 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                        && e2 > 1 as ::core::ffi::c_int
                                    {
                                        let fresh4 = bufpt;
                                        bufpt = bufpt.offset(1);
                                        *fresh4 = ',' as i32 as ::core::ffi::c_char;
                                    }
                                    e2 -= 1;
                                }
                            }
                            if flag_dp != 0 {
                                let fresh5 = bufpt;
                                bufpt = bufpt.offset(1);
                                *fresh5 = '.' as i32 as ::core::ffi::c_char;
                            }
                            e2 += 1;
                            while e2 < 0 as ::core::ffi::c_int
                                && precision > 0 as ::core::ffi::c_int
                            {
                                let fresh6 = bufpt;
                                bufpt = bufpt.offset(1);
                                *fresh6 = '0' as i32 as ::core::ffi::c_char;
                                precision -= 1;
                                e2 += 1;
                            }
                            loop {
                                let fresh7 = precision;
                                precision -= 1;
                                if (fresh7 <= 0 as ::core::ffi::c_int) {
                                    break;
                                }
                                let fresh9 = bufpt;
                                bufpt = bufpt.offset(1);
                                *fresh9 = (if j < s.n {
                                    let fresh8 = j;
                                    j += 1;
                                    *s.z.offset(fresh8 as isize) as ::core::ffi::c_int
                                } else {
                                    '0' as i32
                                }) as ::core::ffi::c_char;
                            }
                            if flag_rtz as ::core::ffi::c_int != 0
                                && flag_dp as ::core::ffi::c_int != 0
                            {
                                while *bufpt.offset(-(1 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    == '0' as i32
                                {
                                    bufpt = bufpt.offset(-1);
                                    *bufpt = 0 as ::core::ffi::c_char;
                                }
                                if *bufpt.offset(-(1 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    == '.' as i32
                                {
                                    if flag_altform2 != 0 {
                                        let fresh10 = bufpt;
                                        bufpt = bufpt.offset(1);
                                        *fresh10 = '0' as i32 as ::core::ffi::c_char;
                                    } else {
                                        bufpt = bufpt.offset(-1);
                                        *bufpt = 0 as ::core::ffi::c_char;
                                    }
                                }
                            }
                            if xtype as ::core::ffi::c_int == etEXP {
                                exp = s.iDP - 1 as ::core::ffi::c_int;
                                let fresh11 = bufpt;
                                bufpt = bufpt.offset(1);
                                *fresh11 = aDigits[(*infop).charset as usize];
                                if exp < 0 as ::core::ffi::c_int {
                                    let fresh12 = bufpt;
                                    bufpt = bufpt.offset(1);
                                    *fresh12 = '-' as i32 as ::core::ffi::c_char;
                                    exp = -exp;
                                } else {
                                    let fresh13 = bufpt;
                                    bufpt = bufpt.offset(1);
                                    *fresh13 = '+' as i32 as ::core::ffi::c_char;
                                }
                                if exp >= 100 as ::core::ffi::c_int {
                                    let fresh14 = bufpt;
                                    bufpt = bufpt.offset(1);
                                    *fresh14 = (exp / 100 as ::core::ffi::c_int + '0' as i32)
                                        as ::core::ffi::c_char;
                                    exp %= 100 as ::core::ffi::c_int;
                                }
                                let fresh15 = bufpt;
                                bufpt = bufpt.offset(1);
                                *fresh15 = (exp / 10 as ::core::ffi::c_int + '0' as i32)
                                    as ::core::ffi::c_char;
                                let fresh16 = bufpt;
                                bufpt = bufpt.offset(1);
                                *fresh16 = (exp % 10 as ::core::ffi::c_int + '0' as i32)
                                    as ::core::ffi::c_char;
                            }
                            *bufpt = 0 as ::core::ffi::c_char;
                            length = bufpt.offset_from(zOut) as ::core::ffi::c_long
                                as ::core::ffi::c_int;
                            bufpt = zOut;
                            if flag_zeropad as ::core::ffi::c_int != 0
                                && flag_leftjustify == 0
                                && length < width
                            {
                                let mut i: ::core::ffi::c_int;
                                let mut nPad: ::core::ffi::c_int = width - length;
                                i = width;
                                while i >= nPad {
                                    *bufpt.offset(i as isize) = *bufpt.offset((i - nPad) as isize);
                                    i -= 1;
                                }
                                i = (prefix as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int;
                                loop {
                                    let fresh17 = nPad;
                                    nPad -= 1;
                                    if (fresh17 == 0) {
                                        break;
                                    }
                                    let fresh18 = i;
                                    i += 1;
                                    *bufpt.offset(fresh18 as isize) =
                                        '0' as i32 as ::core::ffi::c_char;
                                }
                                length = width;
                            }
                            current_block = 11068715555910905014;
                        }
                    }
                }
                etSIZE => {
                    if !bArgList {
                        let nout = cursor.get_nout_ptr();
                        if !nout.is_null() {
                            *nout = (*pAccum).nChar as ::core::ffi::c_int;
                        }
                    }
                    width = 0 as ::core::ffi::c_int;
                    length = width;
                    current_block = 11068715555910905014;
                }
                etPERCENT => {
                    buf[0 as ::core::ffi::c_int as usize] = '%' as i32 as ::core::ffi::c_char;
                    bufpt = &raw mut buf as *mut ::core::ffi::c_char;
                    length = 1 as ::core::ffi::c_int;
                    current_block = 11068715555910905014;
                }
                etCHARX => {
                    if bArgList {
                        bufpt = cursor.get_charx_text();
                        length = 1 as ::core::ffi::c_int;
                        if !bufpt.is_null() {
                            let fresh19 = bufpt;
                            bufpt = bufpt.offset(1);
                            c = *fresh19 as ::core::ffi::c_int;
                            buf[0 as ::core::ffi::c_int as usize] = c as ::core::ffi::c_char;
                            if c & 0xc0 as ::core::ffi::c_int == 0xc0 as ::core::ffi::c_int {
                                while length < 4 as ::core::ffi::c_int
                                    && *bufpt.offset(0_isize) as ::core::ffi::c_int
                                        & 0xc0 as ::core::ffi::c_int
                                        == 0x80 as ::core::ffi::c_int
                                {
                                    let fresh20 = bufpt;
                                    bufpt = bufpt.offset(1);
                                    let fresh21 = length;
                                    length += 1;
                                    buf[fresh21 as usize] = *fresh20;
                                }
                            }
                        } else {
                            buf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
                        }
                    } else {
                        let ch: ::core::ffi::c_uint = cursor.get_char();
                        length = crate::src::src::utf::sqlite3AppendOneUtf8Character(
                            &raw mut buf as *mut ::core::ffi::c_char,
                            ch as crate::src::ext::rtree::rtree::U32_0,
                        );
                    }
                    if precision > 1 as ::core::ffi::c_int {
                        let mut nPrior: crate::src::ext::rtree::rtree::I64_0 =
                            1 as crate::src::ext::rtree::rtree::I64_0;
                        width -= precision - 1 as ::core::ffi::c_int;
                        if width > 1 as ::core::ffi::c_int && flag_leftjustify == 0 {
                            sqlite3_str_appendchar(
                                pAccum,
                                width - 1 as ::core::ffi::c_int,
                                ' ' as i32 as ::core::ffi::c_char,
                            );
                            width = 0 as ::core::ffi::c_int;
                        }
                        sqlite3_str_append(
                            pAccum,
                            &raw mut buf as *mut ::core::ffi::c_char,
                            length,
                        );
                        precision -= 1;
                        while precision > 1 as ::core::ffi::c_int {
                            
                            if nPrior
                                > (precision - 1 as ::core::ffi::c_int)
                                    as crate::src::ext::rtree::rtree::I64_0
                            {
                                nPrior = (precision - 1 as ::core::ffi::c_int)
                                    as crate::src::ext::rtree::rtree::I64_0;
                            }
                            let nCopyBytes: crate::src::ext::rtree::rtree::I64_0 = length as crate::src::ext::rtree::rtree::I64_0 * nPrior;
                            let __pAccum_ref = unsafe { &mut *pAccum };
                            if nCopyBytes
                                + __pAccum_ref.nChar as crate::src::ext::rtree::rtree::I64_0
                                >= __pAccum_ref.nAlloc as crate::src::ext::rtree::rtree::I64_0
                            {
                                sqlite3StrAccumEnlarge(
                                    pAccum as *mut crate::src::headers::sqliteInt_h::StrAccum,
                                    nCopyBytes,
                                );
                            }
                            if __pAccum_ref.accError != 0 {
                                break;
                            }
                            sqlite3_str_append(
                                pAccum,
                                (*pAccum).zText.offset(
                                    (__pAccum_ref.nChar as crate::src::ext::rtree::rtree::I64_0
                                        - nCopyBytes) as isize,
                                ) as *mut ::core::ffi::c_char,
                                nCopyBytes as ::core::ffi::c_int,
                            );
                            precision = (precision as crate::src::ext::rtree::rtree::I64_0 - nPrior)
                                as ::core::ffi::c_int;
                            nPrior *= 2 as crate::src::ext::rtree::rtree::I64_0;
                        }
                    }
                    bufpt = &raw mut buf as *mut ::core::ffi::c_char;
                    flag_altform2 = 1 as EtByte;
                    current_block = 11870784952300973314;
                }
                etSTRING | etDYNSTRING => {
                    bufpt = cursor.get_str();
                    if bArgList {
                        xtype = etSTRING as EtByte;
                    }
                    if bufpt.is_null() {
                        bufpt = b"\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        current_block = 7175397428936041062;
                    } else if xtype as ::core::ffi::c_int == etDYNSTRING {
                        let __pAccum_ref = unsafe { &mut *pAccum };
                        if __pAccum_ref.nChar == 0 as crate::src::ext::rtree::rtree::U32_0
                            && __pAccum_ref.mxAlloc != 0
                            && width == 0 as ::core::ffi::c_int
                            && precision < 0 as ::core::ffi::c_int
                            && __pAccum_ref.accError as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        {
                            __pAccum_ref.zText = bufpt;
                            __pAccum_ref.nAlloc = crate::src::src::malloc::sqlite3DbMallocSize(
                                __pAccum_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                bufpt as *const ::core::ffi::c_void,
                            )
                                as crate::src::ext::rtree::rtree::U32_0;
                            __pAccum_ref.nChar = (0x7fffffff as ::core::ffi::c_int
                                & ::libc::strlen(bufpt) as ::core::ffi::c_int)
                                as crate::src::ext::rtree::rtree::U32_0;
                            __pAccum_ref.printfFlags = (__pAccum_ref.printfFlags
                                as ::core::ffi::c_int
                                | crate::src::headers::sqliteInt_h::SQLITE_PRINTF_MALLOCED)
                                as crate::src::ext::rtree::rtree::U8_0;
                            length = 0 as ::core::ffi::c_int;
                            current_block = 11068715555910905014;
                        } else {
                            zExtra = bufpt;
                            current_block = 7175397428936041062;
                        }
                    } else {
                        current_block = 7175397428936041062;
                    }
                    match current_block {
                        11068715555910905014 => {}
                        _ => {
                            if precision >= 0 as ::core::ffi::c_int {
                                if flag_altform2 != 0 {
                                    let mut z: *mut ::core::ffi::c_uchar =
                                        bufpt as *mut ::core::ffi::c_uchar;
                                    loop {
                                        let fresh22 = precision;
                                        precision -= 1;
                                        if !(fresh22 > 0 as ::core::ffi::c_int
                                            && *z.offset(0_isize) as ::core::ffi::c_int != 0)
                                        {
                                            break;
                                        }
                                        let fresh23 = z;
                                        z = z.offset(1);
                                        if *fresh23 as ::core::ffi::c_int
                                            >= 0xc0 as ::core::ffi::c_int
                                        {
                                            while *z as ::core::ffi::c_int
                                                & 0xc0 as ::core::ffi::c_int
                                                == 0x80 as ::core::ffi::c_int
                                            {
                                                z = z.offset(1);
                                            }
                                        }
                                    }
                                    length = z.offset_from(bufpt as *mut ::core::ffi::c_uchar)
                                        as ::core::ffi::c_long
                                        as ::core::ffi::c_int;
                                } else {
                                    length = 0 as ::core::ffi::c_int;
                                    while length < precision
                                        && *bufpt.offset(length as isize) as ::core::ffi::c_int != 0
                                    {
                                        length += 1;
                                    }
                                }
                            } else {
                                length = 0x7fffffff as ::core::ffi::c_int
                                    & ::libc::strlen(bufpt) as ::core::ffi::c_int;
                            }
                            current_block = 11870784952300973314;
                        }
                    }
                }
                etESCAPE_q | etESCAPE_Q | etESCAPE_w => {
                    let mut i_0: crate::src::ext::rtree::rtree::I64_0;
                    let mut j_0: crate::src::ext::rtree::rtree::I64_0;
                    let mut k: crate::src::ext::rtree::rtree::I64_0;
                    let mut n_0: crate::src::ext::rtree::rtree::I64_0;
                    let mut needQuote: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut ch_0: ::core::ffi::c_char;
                    let mut escarg: *mut ::core::ffi::c_char;
                    let q: ::core::ffi::c_char;
                    escarg = cursor.get_str();
                    if escarg.is_null() {
                        escarg = (if xtype as ::core::ffi::c_int == etESCAPE_Q {
                            b"NULL\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"(NULL)\0" as *const u8 as *const ::core::ffi::c_char
                        }) as *mut ::core::ffi::c_char;
                    } else if xtype as ::core::ffi::c_int == etESCAPE_Q {
                        needQuote = 1 as ::core::ffi::c_int;
                    }
                    if xtype as ::core::ffi::c_int == etESCAPE_w {
                        q = '"' as i32 as ::core::ffi::c_char;
                        flag_alternateform = 0 as EtByte;
                    } else {
                        q = '\'' as i32 as ::core::ffi::c_char;
                    }
                    k = precision as crate::src::ext::rtree::rtree::I64_0;
                    n_0 = 0 as crate::src::ext::rtree::rtree::I64_0;
                    i_0 = n_0;
                    while k != 0 as crate::src::ext::rtree::rtree::I64_0 && {
                        ch_0 = *escarg.offset(i_0 as isize);
                        ch_0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    } {
                        if ch_0 as ::core::ffi::c_int == q as ::core::ffi::c_int {
                            n_0 += 1;
                        }
                        if flag_altform2 as ::core::ffi::c_int != 0
                            && ch_0 as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                                == 0xc0 as ::core::ffi::c_int
                        {
                            while *escarg
                                .offset((i_0 + 1 as crate::src::ext::rtree::rtree::I64_0) as isize)
                                as ::core::ffi::c_int
                                & 0xc0 as ::core::ffi::c_int
                                == 0x80 as ::core::ffi::c_int
                            {
                                i_0 += 1;
                            }
                        }
                        i_0 += 1;
                        k -= 1;
                    }
                    if flag_alternateform != 0 {
                        let mut nBack: crate::src::ext::rtree::rtree::U32_0 =
                            0 as crate::src::ext::rtree::rtree::U32_0;
                        let mut nCtrl: crate::src::ext::rtree::rtree::U32_0 =
                            0 as crate::src::ext::rtree::rtree::U32_0;
                        k = 0 as crate::src::ext::rtree::rtree::I64_0;
                        while k < i_0 {
                            if *escarg.offset(k as isize) as ::core::ffi::c_int == '\\' as i32 {
                                nBack = nBack.wrapping_add(1);
                            } else if *(escarg as *mut crate::src::ext::rtree::rtree::U8_0)
                                .offset(k as isize)
                                as ::core::ffi::c_int
                                <= 0x1f as ::core::ffi::c_int
                            {
                                nCtrl = nCtrl.wrapping_add(1);
                            }
                            k += 1;
                        }
                        if nCtrl != 0 || xtype as ::core::ffi::c_int == etESCAPE_q {
                            n_0 += nBack.wrapping_add(
                                (5 as crate::src::ext::rtree::rtree::U32_0).wrapping_mul(nCtrl),
                            )
                                as crate::src::ext::rtree::rtree::I64_0;
                            if xtype as ::core::ffi::c_int == etESCAPE_Q {
                                n_0 += 10 as crate::src::ext::rtree::rtree::I64_0;
                                needQuote = 2 as ::core::ffi::c_int;
                            }
                        } else {
                            flag_alternateform = 0 as EtByte;
                        }
                    }
                    n_0 += i_0 + 3 as crate::src::ext::rtree::rtree::I64_0;
                    if n_0 > etBUFSIZE as crate::src::ext::rtree::rtree::I64_0 {
                        zExtra = printfTempBuf(
                            pAccum,
                            n_0 as crate::src::headers::sqlite3_h::Sqlite3Int64,
                        );
                        bufpt = zExtra;
                        if bufpt.is_null() {
                            return;
                        }
                    } else {
                        bufpt = &raw mut buf as *mut ::core::ffi::c_char;
                    }
                    j_0 = 0 as crate::src::ext::rtree::rtree::I64_0;
                    if needQuote != 0 {
                        if needQuote == 2 as ::core::ffi::c_int {
                            ::core::ptr::copy_nonoverlapping(
                                b"unistr('\0" as *const u8 as *const ::core::ffi::c_char,
                                bufpt.offset(j_0 as isize) as *mut ::core::ffi::c_char,
                                8_usize,
                            );
                            j_0 += 8 as crate::src::ext::rtree::rtree::I64_0;
                        } else {
                            let fresh25 = j_0;
                            j_0 += 1;
                            *bufpt.offset(fresh25 as isize) = '\'' as i32 as ::core::ffi::c_char;
                        }
                    }
                    k = i_0;
                    if flag_alternateform != 0 {
                        i_0 = 0 as crate::src::ext::rtree::rtree::I64_0;
                        while i_0 < k {
                            ch_0 = *escarg.offset(i_0 as isize);
                            let fresh26 = j_0;
                            j_0 += 1;
                            *bufpt.offset(fresh26 as isize) = ch_0;
                            if ch_0 as ::core::ffi::c_int == q as ::core::ffi::c_int {
                                let fresh27 = j_0;
                                j_0 += 1;
                                *bufpt.offset(fresh27 as isize) = ch_0;
                            } else if ch_0 as ::core::ffi::c_int == '\\' as i32 {
                                let fresh28 = j_0;
                                j_0 += 1;
                                *bufpt.offset(fresh28 as isize) =
                                    '\\' as i32 as ::core::ffi::c_char;
                            } else if ch_0 as ::core::ffi::c_uchar as ::core::ffi::c_int
                                <= 0x1f as ::core::ffi::c_int
                            {
                                *bufpt.offset(
                                    (j_0 - 1 as crate::src::ext::rtree::rtree::I64_0) as isize,
                                ) = '\\' as i32 as ::core::ffi::c_char;
                                let fresh29 = j_0;
                                j_0 += 1;
                                *bufpt.offset(fresh29 as isize) = 'u' as i32 as ::core::ffi::c_char;
                                let fresh30 = j_0;
                                j_0 += 1;
                                *bufpt.offset(fresh30 as isize) = '0' as i32 as ::core::ffi::c_char;
                                let fresh31 = j_0;
                                j_0 += 1;
                                *bufpt.offset(fresh31 as isize) = '0' as i32 as ::core::ffi::c_char;
                                let fresh32 = j_0;
                                j_0 += 1;
                                *bufpt.offset(fresh32 as isize) =
                                    (if ch_0 as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int {
                                        '1' as i32
                                    } else {
                                        '0' as i32
                                    }) as ::core::ffi::c_char;
                                let fresh33 = j_0;
                                j_0 += 1;
                                *bufpt.offset(fresh33 as isize) =
                                    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(
                                        *b"0123456789abcdef\0",
                                    )[(ch_0
                                        as ::core::ffi::c_int
                                        & 0xf as ::core::ffi::c_int)
                                        as usize];
                            }
                            i_0 += 1;
                        }
                    } else {
                        i_0 = 0 as crate::src::ext::rtree::rtree::I64_0;
                        while i_0 < k {
                            ch_0 = *escarg.offset(i_0 as isize);
                            let fresh34 = j_0;
                            j_0 += 1;
                            *bufpt.offset(fresh34 as isize) = ch_0;
                            if ch_0 as ::core::ffi::c_int == q as ::core::ffi::c_int {
                                let fresh35 = j_0;
                                j_0 += 1;
                                *bufpt.offset(fresh35 as isize) = ch_0;
                            }
                            i_0 += 1;
                        }
                    }
                    if needQuote != 0 {
                        let fresh36 = j_0;
                        j_0 += 1;
                        *bufpt.offset(fresh36 as isize) = '\'' as i32 as ::core::ffi::c_char;
                        if needQuote == 2 as ::core::ffi::c_int {
                            let fresh37 = j_0;
                            j_0 += 1;
                            *bufpt.offset(fresh37 as isize) = ')' as i32 as ::core::ffi::c_char;
                        }
                    }
                    *bufpt.offset(j_0 as isize) = 0 as ::core::ffi::c_char;
                    length = j_0 as ::core::ffi::c_int;
                    current_block = 11870784952300973314;
                }
                etTOKEN => {
                    if (*pAccum).printfFlags as ::core::ffi::c_int
                        & crate::src::headers::sqliteInt_h::SQLITE_PRINTF_INTERNAL
                        == 0 as ::core::ffi::c_int
                    {
                        return;
                    }
                    if flag_alternateform != 0 {
                        let pExpr: *mut crate::src::headers::sqliteInt_h::Expr =
                            cursor.get_expr();
                        if !pExpr.is_null()
                            && ((*pExpr).flags
                                & 0x800 as ::core::ffi::c_int
                                    as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
                        {
                            sqlite3_str_appendall(
                                pAccum,
                                (*pExpr).u.zToken as *const ::core::ffi::c_char,
                            );
                            sqlite3RecordErrorOffsetOfExpr((*pAccum).db, pExpr);
                        }
                    } else {
                        let pToken: *mut crate::src::headers::sqliteInt_h::Token =
                            cursor.get_token();
                        if !pToken.is_null() && (*pToken).n != 0 {
                            let __pToken_ref = unsafe { &*pToken };
                            sqlite3_str_append(
                                pAccum,
                                __pToken_ref.z,
                                __pToken_ref.n as ::core::ffi::c_int,
                            );
                            sqlite3RecordErrorByteOffset((*pAccum).db, __pToken_ref.z);
                        }
                    }
                    width = 0 as ::core::ffi::c_int;
                    length = width;
                    current_block = 11068715555910905014;
                }
                etSRCITEM => {
                    
                    if (*pAccum).printfFlags as ::core::ffi::c_int
                        & crate::src::headers::sqliteInt_h::SQLITE_PRINTF_INTERNAL
                        == 0 as ::core::ffi::c_int
                    {
                        return;
                    }
                    let pItem: *mut crate::src::headers::sqliteInt_h::SrcItem = cursor.get_srcitem();
                    if !(*pItem).zAlias.is_null() && flag_altform2 == 0 {
                        sqlite3_str_appendall(pAccum, (*pItem).zAlias);
                    } else if !(*pItem).zName.is_null() {
                        let __pItem_ref = unsafe { &mut *pItem };
                        if __pItem_ref.fg.fixedSchema() as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                            && __pItem_ref.fg.isSubquery() as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            && !__pItem_ref.u4.zDatabase.is_null()
                        {
                            sqlite3_str_appendall(pAccum, __pItem_ref.u4.zDatabase);
                            sqlite3_str_append(
                                pAccum,
                                b".\0" as *const u8 as *const ::core::ffi::c_char,
                                1 as ::core::ffi::c_int,
                            );
                        }
                        sqlite3_str_appendall(pAccum, __pItem_ref.zName);
                    } else if !(*pItem).zAlias.is_null() {
                        sqlite3_str_appendall(pAccum, (*pItem).zAlias);
                    } else if (*pItem).fg.isSubquery() != 0 {
                        let pSel: *mut crate::src::headers::sqliteInt_h::Select =
                            (*(*pItem).u4.pSubq).pSelect;
                        if (*pSel).selFlags
                            & crate::src::headers::sqliteInt_h::SF_NestedFrom
                                as crate::src::ext::rtree::rtree::U32_0
                            != 0
                        {
                            sqlite3_str_vappendf2(
                                pAccum as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                                "(join-%u)",
                                printf_args!((*pSel).selId as u64),
                            );
                        } else if (*pSel).selFlags
                            & crate::src::headers::sqliteInt_h::SF_MultiValue
                                as crate::src::ext::rtree::rtree::U32_0
                            != 0
                        {
                            sqlite3_str_vappendf2(
                                pAccum as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                                "%u-ROW VALUES CLAUSE",
                                printf_args!((*pItem).u1.nRow as u64),
                            );
                        } else {
                            sqlite3_str_vappendf2(
                                pAccum as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
                                "(subquery-%u)",
                                printf_args!((*pSel).selId as u64),
                            );
                        }
                    }
                    width = 0 as ::core::ffi::c_int;
                    length = width;
                    current_block = 11068715555910905014;
                }
                _ => return,
            }
            match current_block {
                8988494922703612237 => {
                    cThousand = 0 as EtByte;
                    current_block = 9031895888649199432;
                }
                11870784952300973314 => {
                    if flag_altform2 as ::core::ffi::c_int != 0 && width > 0 as ::core::ffi::c_int {
                        let mut ii: ::core::ffi::c_int = length - 1 as ::core::ffi::c_int;
                        while ii >= 0 as ::core::ffi::c_int {
                            let fresh24 = ii;
                            ii -= 1;
                            if *bufpt.offset(fresh24 as isize) as ::core::ffi::c_int
                                & 0xc0 as ::core::ffi::c_int
                                == 0x80 as ::core::ffi::c_int
                            {
                                width += 1;
                            }
                        }
                    }
                    current_block = 11068715555910905014;
                }
                _ => {}
            }
            if current_block == 9031895888649199432 {
                let __infop_ref = unsafe { &*infop };
                if __infop_ref.flags as ::core::ffi::c_int & FLAG_SIGNED != 0 {
                    let v: crate::src::ext::rtree::rtree::I64_0 = cursor.get_int();
                    if v < 0 as crate::src::ext::rtree::rtree::I64_0 {
                        longvalue = !v as crate::src::headers::sqlite3_h::SqliteUint64;
                        longvalue = longvalue.wrapping_add(1);
                        prefix = '-' as i32 as ::core::ffi::c_char;
                    } else {
                        longvalue = v as crate::src::headers::sqlite3_h::SqliteUint64;
                        prefix = flag_prefix as ::core::ffi::c_char;
                    }
                } else {
                    longvalue = cursor.get_uint();
                    prefix = 0 as ::core::ffi::c_char;
                }
                if longvalue == 0 as crate::src::headers::sqlite3_h::SqliteUint64 {
                    flag_alternateform = 0 as EtByte;
                }
                if flag_zeropad as ::core::ffi::c_int != 0
                    && precision
                        < width
                            - (prefix as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                {
                    precision = width
                        - (prefix as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int;
                }
                if precision
                    < etBUFSIZE - 10 as ::core::ffi::c_int - etBUFSIZE / 3 as ::core::ffi::c_int
                {
                    nOut = etBUFSIZE;
                    zOut = &raw mut buf as *mut ::core::ffi::c_char;
                } else {
                    let mut n: crate::src::ext::rtree::rtree::U64_0;
                    n = (precision as crate::src::ext::rtree::rtree::U64_0)
                        .wrapping_add(10 as crate::src::ext::rtree::rtree::U64_0);
                    if cThousand != 0 {
                        n = n.wrapping_add(
                            (precision / 3 as ::core::ffi::c_int)
                                as crate::src::ext::rtree::rtree::U64_0,
                        );
                    }
                    zExtra = printfTempBuf(
                        pAccum,
                        n as crate::src::headers::sqlite3_h::Sqlite3Int64,
                    );
                    zOut = zExtra;
                    if zOut.is_null() {
                        return;
                    }
                    nOut = n as ::core::ffi::c_int;
                }
                bufpt = zOut.offset((nOut - 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char;
                if xtype as ::core::ffi::c_int == etORDINAL {
                    static mut zOrd: [::core::ffi::c_char; 9] = unsafe {
                        ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(
                            *b"thstndrd\0",
                        )
                    };
                    let mut x: ::core::ffi::c_int = longvalue
                        .wrapping_rem(10 as crate::src::headers::sqlite3_h::SqliteUint64)
                        as ::core::ffi::c_int;
                    if x >= 4 as ::core::ffi::c_int
                        || longvalue
                            .wrapping_div(10 as crate::src::headers::sqlite3_h::SqliteUint64)
                            .wrapping_rem(10 as crate::src::headers::sqlite3_h::SqliteUint64)
                            == 1 as crate::src::headers::sqlite3_h::SqliteUint64
                    {
                        x = 0 as ::core::ffi::c_int;
                    }
                    bufpt = bufpt.offset(-1);
                    *bufpt =
                        zOrd[(x * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize];
                    bufpt = bufpt.offset(-1);
                    *bufpt = zOrd[(x * 2 as ::core::ffi::c_int) as usize];
                }
                let cset: *const ::core::ffi::c_char = (&raw const aDigits
                    as *const ::core::ffi::c_char)
                    .offset(__infop_ref.charset as isize)
                    as *const ::core::ffi::c_char;
                let base: crate::src::ext::rtree::rtree::U8_0 =
                    __infop_ref.base as crate::src::ext::rtree::rtree::U8_0;
                loop {
                    bufpt = bufpt.offset(-1);
                    *bufpt = *cset.offset(
                        longvalue
                            .wrapping_rem(base as crate::src::headers::sqlite3_h::SqliteUint64)
                            as isize,
                    );
                    longvalue = longvalue
                        .wrapping_div(base as crate::src::headers::sqlite3_h::SqliteUint64);
                    if (longvalue <= 0 as crate::src::headers::sqlite3_h::SqliteUint64) {
                        break;
                    }
                }
                length = (zOut.offset((nOut - 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char)
                    .offset_from(bufpt) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
                while precision > length {
                    bufpt = bufpt.offset(-1);
                    *bufpt = '0' as i32 as ::core::ffi::c_char;
                    length += 1;
                }
                if cThousand != 0 {
                    let mut nn: ::core::ffi::c_int =
                        (length - 1 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int;
                    let mut ix: ::core::ffi::c_int = (length - 1 as ::core::ffi::c_int)
                        % 3 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int;
                    bufpt = bufpt.offset(-(nn as isize));
                    idx = 0 as ::core::ffi::c_int;
                    while nn > 0 as ::core::ffi::c_int {
                        *bufpt.offset(idx as isize) = *bufpt.offset((idx + nn) as isize);
                        ix -= 1;
                        if ix == 0 as ::core::ffi::c_int {
                            idx += 1;
                            *bufpt.offset(idx as isize) = cThousand as ::core::ffi::c_char;
                            nn -= 1;
                            ix = 3 as ::core::ffi::c_int;
                        }
                        idx += 1;
                    }
                }
                if prefix != 0 {
                    bufpt = bufpt.offset(-1);
                    *bufpt = prefix;
                }
                if flag_alternateform as ::core::ffi::c_int != 0
                    && __infop_ref.prefix as ::core::ffi::c_int != 0
                {
                    let mut pre: *const ::core::ffi::c_char;
                    let mut x_0: ::core::ffi::c_char;
                    pre = (&raw const aPrefix as *const ::core::ffi::c_char)
                        .offset(__infop_ref.prefix as isize)
                        as *const ::core::ffi::c_char;
                    loop {
                        x_0 = *pre;
                        if (x_0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
                            break;
                        }
                        bufpt = bufpt.offset(-1);
                        *bufpt = x_0;
                        pre = pre.offset(1);
                    }
                }
                length = (zOut.offset((nOut - 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char)
                    .offset_from(bufpt) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
            }
            width -= length;
            if width > 0 as ::core::ffi::c_int {
                if flag_leftjustify == 0 {
                    sqlite3_str_appendchar(pAccum, width, ' ' as i32 as ::core::ffi::c_char);
                }
                sqlite3_str_append(pAccum, bufpt, length);
                if flag_leftjustify != 0 {
                    sqlite3_str_appendchar(pAccum, width, ' ' as i32 as ::core::ffi::c_char);
                }
            } else {
                sqlite3_str_append(pAccum, bufpt, length);
            }
            if !zExtra.is_null() {
                crate::src::src::malloc::sqlite3DbFree(
                    (*pAccum).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    zExtra as *mut ::core::ffi::c_void,
                );
                zExtra = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            fi += 1;
        }
    }
}
