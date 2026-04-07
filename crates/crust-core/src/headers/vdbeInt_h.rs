pub use crate::src::src::vdbesort::VdbeSorter;
use crate::src::src::window::bft;

pub const SQLITE_MAX_SCHEMA_RETRY: ::core::ffi::c_int = 50 as ::core::ffi::c_int;

pub type Op = crate::src::src::vdbe::VdbeOp;

pub type Bool = ::core::ffi::c_uint;

pub const CURTYPE_BTREE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const CURTYPE_BTREE_1: ::core::ffi::c_int = 0;

pub const CURTYPE_SORTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const CURTYPE_SORTER_1: ::core::ffi::c_int = 1;

pub const CURTYPE_VTAB: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const CURTYPE_VTAB_1: ::core::ffi::c_int = 2;

pub const CURTYPE_PSEUDO: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]

pub struct VdbeCursor {
    pub eCurType: crate::src::ext::rtree::rtree::u8_0,
    pub iDb: crate::src::headers::sqliteInt_h::i8_0,
    pub nullRow: crate::src::ext::rtree::rtree::u8_0,
    pub deferredMoveto: crate::src::ext::rtree::rtree::u8_0,
    pub isTable: crate::src::ext::rtree::rtree::u8_0,
    #[bitfield(name = "isEphemeral", ty = "Bool", bits = "0..=0")]
    #[bitfield(name = "useRandomRowid", ty = "Bool", bits = "1..=1")]
    #[bitfield(name = "isOrdered", ty = "Bool", bits = "2..=2")]
    #[bitfield(name = "noReuse", ty = "Bool", bits = "3..=3")]
    #[bitfield(name = "colCache", ty = "Bool", bits = "4..=4")]
    pub isEphemeral_useRandomRowid_isOrdered_noReuse_colCache: [u8; 1],
    pub seekHit: crate::src::fts5::u16_0,
    pub ub: crate::src::headers::vdbeInt_h::__anon_union_17,
    pub seqCount: crate::src::ext::rtree::rtree::i64_0,
    pub cacheStatus: crate::src::ext::rtree::rtree::u32_0,
    pub seekResult: ::core::ffi::c_int,
    pub pAltCursor: *mut crate::src::headers::vdbeInt_h::VdbeCursor,
    pub uc: crate::src::headers::vdbeInt_h::__anon_union_18,
    pub pKeyInfo: *mut crate::src::headers::sqliteInt_h::KeyInfo,
    pub iHdrOffset: crate::src::ext::rtree::rtree::u32_0,
    pub pgnoRoot: crate::src::src::pager::Pgno,
    pub nField: crate::src::fts5::i16_0,
    pub nHdrParsed: crate::src::fts5::u16_0,
    pub movetoTarget: crate::src::ext::rtree::rtree::i64_0,
    pub aOffset: *mut crate::src::ext::rtree::rtree::u32_0,
    pub aRow: *const crate::src::ext::rtree::rtree::u8_0,
    pub payloadSize: crate::src::ext::rtree::rtree::u32_0,
    pub szRow: crate::src::ext::rtree::rtree::u32_0,
    pub pCache: *mut crate::src::headers::vdbeInt_h::VdbeTxtBlbCache,
    pub aType: [crate::src::ext::rtree::rtree::u32_0; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]

pub union __anon_union_17 {
    pub pBtx: *mut crate::src::headers::btreeInt_h::Btree,
    pub aAltMap: *mut crate::src::ext::rtree::rtree::u32_0,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub union __anon_union_18 {
    pub pCursor: *mut crate::src::headers::btreeInt_h::BtCursor,
    pub pVCur: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    pub pSorter: *mut crate::src::headers::vdbeInt_h::VdbeSorter,
}

pub const CACHE_STALE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct VdbeTxtBlbCache {
    pub pCValue: *mut ::core::ffi::c_char,
    pub iOffset: crate::src::ext::rtree::rtree::i64_0,
    pub iCol: ::core::ffi::c_int,
    pub cacheStatus: crate::src::ext::rtree::rtree::u32_0,
    pub colCacheCtr: crate::src::ext::rtree::rtree::u32_0,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct VdbeFrame {
    pub v: *mut crate::src::headers::vdbeInt_h::Vdbe,
    pub pParent: *mut crate::src::headers::vdbeInt_h::VdbeFrame,
    pub aOp: *mut crate::src::headers::vdbeInt_h::Op,
    pub aMem: *mut crate::src::src::vdbe::Mem,
    pub apCsr: *mut *mut crate::src::headers::vdbeInt_h::VdbeCursor,
    pub aOnce: *mut crate::src::ext::rtree::rtree::u8_0,
    pub token: *mut ::core::ffi::c_void,
    pub lastRowid: crate::src::ext::rtree::rtree::i64_0,
    pub pAuxData: *mut crate::src::headers::vdbeInt_h::AuxData,
    pub nCursor: ::core::ffi::c_int,
    pub pc: ::core::ffi::c_int,
    pub nOp: ::core::ffi::c_int,
    pub nMem: ::core::ffi::c_int,
    pub nChildMem: ::core::ffi::c_int,
    pub nChildCsr: ::core::ffi::c_int,
    pub nChange: crate::src::ext::rtree::rtree::i64_0,
    pub nDbChange: crate::src::ext::rtree::rtree::i64_0,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_value {
    pub u: crate::src::headers::vdbeInt_h::MemValue,
    pub z: *mut ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub flags: crate::src::fts5::u16_0,
    pub enc: crate::src::ext::rtree::rtree::u8_0,
    pub eSubtype: crate::src::ext::rtree::rtree::u8_0,
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub szMalloc: ::core::ffi::c_int,
    pub uTemp: crate::src::ext::rtree::rtree::u32_0,
    pub zMalloc: *mut ::core::ffi::c_char,
    pub xDel: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub union MemValue {
    pub r: ::core::ffi::c_double,
    pub i: crate::src::ext::rtree::rtree::i64_0,
    pub nZero: ::core::ffi::c_int,
    pub zPType: *const ::core::ffi::c_char,
    pub pDef: *mut crate::src::headers::sqliteInt_h::FuncDef,
}

pub const MEMCELLSIZE: ::core::ffi::c_ulong = 24 as ::core::ffi::c_ulong;

pub const MEM_Undefined: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const MEM_Null: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const MEM_Str: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const MEM_Int: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const MEM_Real: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const MEM_Blob: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const MEM_IntReal: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const MEM_AffMask: ::core::ffi::c_int = 0x3f as ::core::ffi::c_int;

pub const MEM_FromBind: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const MEM_Cleared: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;

pub const MEM_Term: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;

pub const MEM_Zero: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;

pub const MEM_Subtype: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;

pub const MEM_TypeMask: ::core::ffi::c_int = 0xdbf as ::core::ffi::c_int;

pub const MEM_Dyn: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;

pub const MEM_Static: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;

pub const MEM_Ephem: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;

pub const MEM_Agg: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct AuxData {
    pub iAuxOp: ::core::ffi::c_int,
    pub iAuxArg: ::core::ffi::c_int,
    pub pAux: *mut ::core::ffi::c_void,
    pub xDeleteAux: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub pNextAux: *mut crate::src::headers::vdbeInt_h::AuxData,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct sqlite3_context {
    pub pOut: *mut crate::src::src::vdbe::Mem,
    pub pFunc: *mut crate::src::headers::sqliteInt_h::FuncDef,
    pub pMem: *mut crate::src::src::vdbe::Mem,
    pub pVdbe: *mut crate::src::headers::vdbeInt_h::Vdbe,
    pub iOp: ::core::ffi::c_int,
    pub isError: ::core::ffi::c_int,
    pub enc: crate::src::ext::rtree::rtree::u8_0,
    pub skipFlag: crate::src::ext::rtree::rtree::u8_0,
    pub argc: crate::src::fts5::u16_0,
    pub argv: [*mut crate::src::headers::vdbeInt_h::sqlite3_value; 0],
}

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]

pub struct Vdbe {
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub ppVPrev: *mut *mut crate::src::headers::vdbeInt_h::Vdbe,
    pub pVNext: *mut crate::src::headers::vdbeInt_h::Vdbe,
    pub pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pub nVar: crate::src::headers::sqliteInt_h::ynVar,
    pub nMem: ::core::ffi::c_int,
    pub nCursor: ::core::ffi::c_int,
    pub cacheCtr: crate::src::ext::rtree::rtree::u32_0,
    pub pc: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub nChange: crate::src::ext::rtree::rtree::i64_0,
    pub iStatement: ::core::ffi::c_int,
    pub iCurrentTime: crate::src::ext::rtree::rtree::i64_0,
    pub nFkConstraint: crate::src::ext::rtree::rtree::i64_0,
    pub nStmtDefCons: crate::src::ext::rtree::rtree::i64_0,
    pub nStmtDefImmCons: crate::src::ext::rtree::rtree::i64_0,
    pub aMem: *mut crate::src::src::vdbe::Mem,
    pub apArg: *mut *mut crate::src::src::vdbe::Mem,
    pub apCsr: *mut *mut crate::src::headers::vdbeInt_h::VdbeCursor,
    pub aVar: *mut crate::src::src::vdbe::Mem,
    pub aOp: *mut crate::src::headers::vdbeInt_h::Op,
    pub nOp: ::core::ffi::c_int,
    pub nOpAlloc: ::core::ffi::c_int,
    pub aColName: *mut crate::src::src::vdbe::Mem,
    pub pResultRow: *mut crate::src::src::vdbe::Mem,
    pub zErrMsg: *mut ::core::ffi::c_char,
    pub pVList: *mut crate::src::headers::sqliteInt_h::VList,
    pub startTime: crate::src::ext::rtree::rtree::i64_0,
    pub nResColumn: crate::src::fts5::u16_0,
    pub nResAlloc: crate::src::fts5::u16_0,
    pub errorAction: crate::src::ext::rtree::rtree::u8_0,
    pub minWriteFileFormat: crate::src::ext::rtree::rtree::u8_0,
    pub prepFlags: crate::src::ext::rtree::rtree::u8_0,
    pub eVdbeState: crate::src::ext::rtree::rtree::u8_0,
    #[bitfield(name = "expired", ty = "bft", bits = "0..=1")]
    #[bitfield(name = "explain", ty = "bft", bits = "2..=3")]
    #[bitfield(name = "changeCntOn", ty = "bft", bits = "4..=4")]
    #[bitfield(name = "usesStmtJournal", ty = "bft", bits = "5..=5")]
    #[bitfield(name = "readOnly", ty = "bft", bits = "6..=6")]
    #[bitfield(name = "bIsReader", ty = "bft", bits = "7..=7")]
    #[bitfield(name = "haveEqpOps", ty = "bft", bits = "8..=8")]
    pub expired_explain_changeCntOn_usesStmtJournal_readOnly_bIsReader_haveEqpOps: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub btreeMask: crate::src::headers::sqliteInt_h::yDbMask,
    pub lockMask: crate::src::headers::sqliteInt_h::yDbMask,
    pub aCounter: [crate::src::ext::rtree::rtree::u32_0; 9],
    pub zSql: *mut ::core::ffi::c_char,
    pub pFree: *mut ::core::ffi::c_void,
    pub pFrame: *mut crate::src::headers::vdbeInt_h::VdbeFrame,
    pub pDelFrame: *mut crate::src::headers::vdbeInt_h::VdbeFrame,
    pub nFrame: ::core::ffi::c_int,
    pub expmask: crate::src::ext::rtree::rtree::u32_0,
    pub pProgram: *mut crate::src::src::vdbe::SubProgram,
    pub pAuxData: *mut crate::src::headers::vdbeInt_h::AuxData,
}

pub const VDBE_INIT_STATE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const VDBE_READY_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const VDBE_RUN_STATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const VDBE_HALT_STATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct PreUpdate {
    pub v: *mut crate::src::headers::vdbeInt_h::Vdbe,
    pub pCsr: *mut crate::src::headers::vdbeInt_h::VdbeCursor,
    pub op: ::core::ffi::c_int,
    pub aRecord: *mut crate::src::ext::rtree::rtree::u8_0,
    pub pKeyinfo: *mut crate::src::headers::sqliteInt_h::KeyInfo,
    pub pUnpacked: *mut crate::src::headers::sqliteInt_h::UnpackedRecord,
    pub pNewUnpacked: *mut crate::src::headers::sqliteInt_h::UnpackedRecord,
    pub iNewReg: ::core::ffi::c_int,
    pub iBlobWrite: ::core::ffi::c_int,
    pub iKey1: crate::src::ext::rtree::rtree::i64_0,
    pub iKey2: crate::src::ext::rtree::rtree::i64_0,
    pub oldipk: crate::src::src::vdbe::Mem,
    pub aNew: *mut crate::src::src::vdbe::Mem,
    pub pTab: *mut crate::src::headers::sqliteInt_h::Table,
    pub pPk: *mut crate::src::headers::sqliteInt_h::Index,
    pub apDflt: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pub uKey: crate::src::headers::vdbeInt_h::__anon_struct_10,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct __anon_struct_10 {
    pub keyinfoSpace: [crate::src::ext::rtree::rtree::u8_0; 32],
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct ValueList {
    pub pCsr: *mut crate::src::headers::btreeInt_h::BtCursor,
    pub pOut: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
}
