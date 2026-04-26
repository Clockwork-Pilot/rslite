use crate::src::ext::rtree::rtree::U32_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereMemBlock {
    pub pNext: *mut crate::src::headers::whereInt_h::WhereMemBlock,
    pub sz: crate::src::ext::rtree::rtree::U64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereRightJoin {
    pub iMatch: ::core::ffi::c_int,
    pub regBloom: ::core::ffi::c_int,
    pub regReturn: ::core::ffi::c_int,
    pub addrSubrtn: ::core::ffi::c_int,
    pub endSubrtn: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereLevel {
    pub iLeftJoin: ::core::ffi::c_int,
    pub iTabCur: ::core::ffi::c_int,
    pub iIdxCur: ::core::ffi::c_int,
    pub addrBrk: ::core::ffi::c_int,
    pub addrHalt: ::core::ffi::c_int,
    pub addrNxt: ::core::ffi::c_int,
    pub addrSkip: ::core::ffi::c_int,
    pub addrCont: ::core::ffi::c_int,
    pub addrFirst: ::core::ffi::c_int,
    pub addrBody: ::core::ffi::c_int,
    pub regBignull: ::core::ffi::c_int,
    pub addrBignull: ::core::ffi::c_int,
    pub iLikeRepCntr: crate::src::ext::rtree::rtree::U32_0,
    pub addrLikeRep: ::core::ffi::c_int,
    pub regFilter: ::core::ffi::c_int,
    pub pRJ: *mut crate::src::headers::whereInt_h::WhereRightJoin,
    pub iFrom: crate::src::ext::rtree::rtree::U8_0,
    pub op: crate::src::ext::rtree::rtree::U8_0,
    pub p3: crate::src::ext::rtree::rtree::U8_0,
    pub p5: crate::src::ext::rtree::rtree::U8_0,
    pub p1: ::core::ffi::c_int,
    pub p2: ::core::ffi::c_int,
    pub u: crate::src::headers::whereInt_h::__anon_union_19,
    pub pWLoop: *mut crate::src::headers::whereInt_h::WhereLoop,
    pub notReady: crate::src::headers::sqliteInt_h::Bitmask,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_19 {
    pub in_0: crate::src::headers::whereInt_h::__anon_struct_11,
    pub pCoveringIdx: *mut crate::src::headers::sqliteInt_h::Index,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anon_struct_11 {
    pub nIn: ::core::ffi::c_int,
    pub aInLoop: *mut crate::src::headers::whereInt_h::InLoop,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct InLoop {
    pub iCur: ::core::ffi::c_int,
    pub addrInTop: ::core::ffi::c_int,
    pub iBase: ::core::ffi::c_int,
    pub nPrefix: ::core::ffi::c_int,
    pub eEndLoopOp: crate::src::ext::rtree::rtree::U8_0,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereLoop {
    pub prereq: crate::src::headers::sqliteInt_h::Bitmask,
    pub maskSelf: crate::src::headers::sqliteInt_h::Bitmask,
    pub iTab: crate::src::ext::rtree::rtree::U8_0,
    pub iSortIdx: crate::src::ext::rtree::rtree::U8_0,
    pub rSetup: crate::src::headers::sqliteInt_h::LogEst,
    pub rRun: crate::src::headers::sqliteInt_h::LogEst,
    pub nOut: crate::src::headers::sqliteInt_h::LogEst,
    pub u: crate::src::headers::whereInt_h::__anon_union_20,
    pub wsFlags: crate::src::ext::rtree::rtree::U32_0,
    pub nLTerm: crate::src::fts5::U16_0,
    pub nSkip: crate::src::fts5::U16_0,
    pub nLSlot: crate::src::fts5::U16_0,
    pub aLTerm: *mut *mut crate::src::headers::whereInt_h::WhereTerm,
    pub pNextLoop: *mut crate::src::headers::whereInt_h::WhereLoop,
    pub aLTermSpace: [*mut crate::src::headers::whereInt_h::WhereTerm; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_20 {
    pub btree: crate::src::headers::whereInt_h::__anon_struct_12,
    pub vtab: crate::src::headers::whereInt_h::__anon_struct_13,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anon_struct_12 {
    pub nEq: crate::src::fts5::U16_0,
    pub nBtm: crate::src::fts5::U16_0,
    pub nTop: crate::src::fts5::U16_0,
    pub nDistinctCol: crate::src::fts5::U16_0,
    pub pIndex: *mut crate::src::headers::sqliteInt_h::Index,
    pub pOrderBy: *mut crate::src::headers::sqliteInt_h::ExprList,
}

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __anon_struct_13 {
    pub idxNum: ::core::ffi::c_int,
    #[bitfield(name = "needFree", ty = "U32_0", bits = "0..=0")]
    #[bitfield(name = "bOmitOffset", ty = "U32_0", bits = "1..=1")]
    #[bitfield(name = "bIdxNumHex", ty = "U32_0", bits = "2..=2")]
    pub needFree_bOmitOffset_bIdxNumHex: [u8; 1],
    pub isOrdered: crate::src::headers::sqliteInt_h::I8_0,
    pub omitMask: crate::src::fts5::U16_0,
    pub idxStr: *mut ::core::ffi::c_char,
    pub mHandleIn: crate::src::ext::rtree::rtree::U32_0,
}

pub const WHERE_LOOP_XFER_SZ: ::core::ffi::c_ulong = 56 as ::core::ffi::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereOrCost {
    pub prereq: crate::src::headers::sqliteInt_h::Bitmask,
    pub rRun: crate::src::headers::sqliteInt_h::LogEst,
    pub nOut: crate::src::headers::sqliteInt_h::LogEst,
}

pub const N_OR_COST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereOrSet {
    pub n: crate::src::fts5::U16_0,
    pub a: [crate::src::headers::whereInt_h::WhereOrCost; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WherePath {
    pub maskLoop: crate::src::headers::sqliteInt_h::Bitmask,
    pub revLoop: crate::src::headers::sqliteInt_h::Bitmask,
    pub nRow: crate::src::headers::sqliteInt_h::LogEst,
    pub rCost: crate::src::headers::sqliteInt_h::LogEst,
    pub rUnsort: crate::src::headers::sqliteInt_h::LogEst,
    pub isOrdered: crate::src::headers::sqliteInt_h::I8_0,
    pub aLoop: *mut *mut crate::src::headers::whereInt_h::WhereLoop,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereTerm {
    pub pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    pub pWC: *mut crate::src::headers::whereInt_h::WhereClause,
    pub truthProb: crate::src::headers::sqliteInt_h::LogEst,
    pub wtFlags: crate::src::fts5::U16_0,
    pub eOperator: crate::src::fts5::U16_0,
    pub nChild: crate::src::ext::rtree::rtree::U8_0,
    pub eMatchOp: crate::src::ext::rtree::rtree::U8_0,
    pub iParent: ::core::ffi::c_int,
    pub leftCursor: ::core::ffi::c_int,
    pub u: crate::src::headers::whereInt_h::__anon_union_21,
    pub prereqRight: crate::src::headers::sqliteInt_h::Bitmask,
    pub prereqAll: crate::src::headers::sqliteInt_h::Bitmask,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __anon_union_21 {
    pub x: crate::src::headers::whereInt_h::__anon_struct_14,
    pub pOrInfo: *mut crate::src::headers::whereInt_h::WhereOrInfo,
    pub pAndInfo: *mut crate::src::headers::whereInt_h::WhereAndInfo,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anon_struct_14 {
    pub leftColumn: ::core::ffi::c_int,
    pub iField: ::core::ffi::c_int,
}

pub const TERM_DYNAMIC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const TERM_VIRTUAL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const TERM_CODED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const TERM_COPIED: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const TERM_ORINFO: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const TERM_ANDINFO: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const TERM_OK: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const TERM_VNULL: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const TERM_LIKEOPT: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;

pub const TERM_LIKECOND: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;

pub const TERM_LIKE: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;

pub const TERM_IS: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;

pub const TERM_VARSELECT: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;

pub const TERM_HEURTRUTH: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;

pub const TERM_HIGHTRUTH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const TERM_SLICE: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereScan {
    pub pOrigWC: *mut crate::src::headers::whereInt_h::WhereClause,
    pub pWC: *mut crate::src::headers::whereInt_h::WhereClause,
    pub zCollName: *const ::core::ffi::c_char,
    pub pIdxExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    pub k: ::core::ffi::c_int,
    pub opMask: crate::src::ext::rtree::rtree::U32_0,
    pub idxaff: ::core::ffi::c_char,
    pub iEquiv: ::core::ffi::c_uchar,
    pub nEquiv: ::core::ffi::c_uchar,
    pub aiCur: [::core::ffi::c_int; 11],
    pub aiColumn: [crate::src::fts5::I16_0; 11],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereClause {
    pub pWInfo: *mut crate::src::headers::whereInt_h::WhereInfo,
    pub pOuter: *mut crate::src::headers::whereInt_h::WhereClause,
    pub op: crate::src::ext::rtree::rtree::U8_0,
    pub hasOr: crate::src::ext::rtree::rtree::U8_0,
    pub nTerm: ::core::ffi::c_int,
    pub nSlot: ::core::ffi::c_int,
    pub nBase: ::core::ffi::c_int,
    pub a: *mut crate::src::headers::whereInt_h::WhereTerm,
    pub aStatic: [crate::src::headers::whereInt_h::WhereTerm; 8],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereOrInfo {
    pub wc: crate::src::headers::whereInt_h::WhereClause,
    pub indexable: crate::src::headers::sqliteInt_h::Bitmask,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereAndInfo {
    pub wc: crate::src::headers::whereInt_h::WhereClause,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereMaskSet {
    pub bVarSelect: ::core::ffi::c_int,
    pub n: ::core::ffi::c_int,
    pub ix: [::core::ffi::c_int; 64],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereLoopBuilder {
    pub pWInfo: *mut crate::src::headers::whereInt_h::WhereInfo,
    pub pWC: *mut crate::src::headers::whereInt_h::WhereClause,
    pub pNew: *mut crate::src::headers::whereInt_h::WhereLoop,
    pub pOrSet: *mut crate::src::headers::whereInt_h::WhereOrSet,
    pub bldFlags1: ::core::ffi::c_uchar,
    pub bldFlags2: ::core::ffi::c_uchar,
    pub iPlanLimit: ::core::ffi::c_uint,
}

pub const SQLITE_BLDF1_INDEXED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const SQLITE_BLDF1_UNIQUE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const SQLITE_QUERY_PLANNER_LIMIT: ::core::ffi::c_int = 20000 as ::core::ffi::c_int;

pub const SQLITE_QUERY_PLANNER_LIMIT_INCR: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct WhereInfo {
    pub pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pub pTabList: *mut crate::src::headers::sqliteInt_h::SrcList,
    pub pOrderBy: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub pResultSet: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub pSelect: *mut crate::src::headers::sqliteInt_h::Select,
    pub aiCurOnePass: [::core::ffi::c_int; 2],
    pub iContinue: ::core::ffi::c_int,
    pub iBreak: ::core::ffi::c_int,
    pub savedNQueryLoop: ::core::ffi::c_int,
    pub wctrlFlags: crate::src::fts5::U16_0,
    pub iLimit: crate::src::headers::sqliteInt_h::LogEst,
    pub nLevel: crate::src::ext::rtree::rtree::U8_0,
    pub nOBSat: crate::src::headers::sqliteInt_h::I8_0,
    pub eOnePass: crate::src::ext::rtree::rtree::U8_0,
    pub eDistinct: crate::src::ext::rtree::rtree::U8_0,
    #[bitfield(name = "bDeferredSeek", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "untestedTerms", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "bOrderedInnerLoop", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "sorted", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "bStarDone", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "bStarUsed", ty = "::core::ffi::c_uint", bits = "5..=5")]
    pub bDeferredSeek_untestedTerms_bOrderedInnerLoop_sorted_bStarDone_bStarUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub nRowOut: crate::src::headers::sqliteInt_h::LogEst,
    pub iTop: ::core::ffi::c_int,
    pub iEndWhere: ::core::ffi::c_int,
    pub pLoops: *mut crate::src::headers::whereInt_h::WhereLoop,
    pub pMemToFree: *mut crate::src::headers::whereInt_h::WhereMemBlock,
    pub revMask: crate::src::headers::sqliteInt_h::Bitmask,
    pub sWC: crate::src::headers::whereInt_h::WhereClause,
    pub sMaskSet: crate::src::headers::whereInt_h::WhereMaskSet,
    pub a: [crate::src::headers::whereInt_h::WhereLevel; 0],
}

pub const WO_IN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const WO_EQ: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const WO_LT: ::core::ffi::c_int =
    crate::src::headers::whereInt_h::WO_EQ << (crate::src::parse::TK_LT_1 - crate::src::parse::TK_EQ);

pub const WO_LE: ::core::ffi::c_int =
    crate::src::headers::whereInt_h::WO_EQ << (crate::src::parse::TK_LE - crate::src::parse::TK_EQ);

pub const WO_GT: ::core::ffi::c_int =
    crate::src::headers::whereInt_h::WO_EQ << (crate::src::parse::TK_GT_1 - crate::src::parse::TK_EQ);

pub const WO_GE: ::core::ffi::c_int =
    crate::src::headers::whereInt_h::WO_EQ << (crate::src::parse::TK_GE - crate::src::parse::TK_EQ);

pub const WO_AUX: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const WO_IS: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const WO_ISNULL: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;

pub const WO_OR: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;

pub const WO_AND: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;

pub const WO_EQUIV: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;

pub const WO_ROWVAL: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;

pub const WO_ALL: ::core::ffi::c_int = 0x3fff as ::core::ffi::c_int;

pub const WO_SINGLE: ::core::ffi::c_int = 0x1ff as ::core::ffi::c_int;

pub const WHERE_COLUMN_EQ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const WHERE_COLUMN_RANGE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const WHERE_COLUMN_IN: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const WHERE_COLUMN_NULL: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const WHERE_CONSTRAINT: ::core::ffi::c_int = 0xf as ::core::ffi::c_int;

pub const WHERE_TOP_LIMIT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const WHERE_BTM_LIMIT: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const WHERE_BOTH_LIMIT: ::core::ffi::c_int = 0x30 as ::core::ffi::c_int;

pub const WHERE_IDX_ONLY: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const WHERE_IPK: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;

pub const WHERE_INDEXED: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;

pub const WHERE_VIRTUALTABLE: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;

pub const WHERE_IN_ABLE: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;

pub const WHERE_ONEROW: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;

pub const WHERE_MULTI_OR: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;

pub const WHERE_AUTO_INDEX: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;

pub const WHERE_SKIPSCAN: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;

pub const WHERE_UNQ_WANTED: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;

pub const WHERE_PARTIALIDX: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;

pub const WHERE_IN_EARLYOUT: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;

pub const WHERE_BIGNULL_SORT: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;

pub const WHERE_IN_SEEKSCAN: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;

pub const WHERE_TRANSCONS: ::core::ffi::c_int = 0x200000 as ::core::ffi::c_int;

pub const WHERE_BLOOMFILTER: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;

pub const WHERE_SELFCULL: ::core::ffi::c_int = 0x800000 as ::core::ffi::c_int;

pub const WHERE_COROUTINE: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;

pub const WHERE_EXPRIDX: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
