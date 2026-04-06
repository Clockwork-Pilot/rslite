pub const SQLITE_FILE_HEADER: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"SQLite format 3\0") };

pub const PTF_INTKEY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const PTF_ZERODATA: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const PTF_LEAFDATA: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const PTF_LEAF: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct MemPage {
    pub isInit: crate::src::ext::rtree::rtree::u8_0,
    pub intKey: crate::src::ext::rtree::rtree::u8_0,
    pub intKeyLeaf: crate::src::ext::rtree::rtree::u8_0,
    pub pgno: crate::src::src::pager::Pgno,
    pub leaf: crate::src::ext::rtree::rtree::u8_0,
    pub hdrOffset: crate::src::ext::rtree::rtree::u8_0,
    pub childPtrSize: crate::src::ext::rtree::rtree::u8_0,
    pub max1bytePayload: crate::src::ext::rtree::rtree::u8_0,
    pub nOverflow: crate::src::ext::rtree::rtree::u8_0,
    pub maxLocal: crate::src::fts5::u16_0,
    pub minLocal: crate::src::fts5::u16_0,
    pub cellOffset: crate::src::fts5::u16_0,
    pub nFree: ::core::ffi::c_int,
    pub nCell: crate::src::fts5::u16_0,
    pub maskPage: crate::src::fts5::u16_0,
    pub aiOvfl: [crate::src::fts5::u16_0; 4],
    pub apOvfl: [*mut crate::src::ext::rtree::rtree::u8_0; 4],
    pub pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    pub aData: *mut crate::src::ext::rtree::rtree::u8_0,
    pub aDataEnd: *mut crate::src::ext::rtree::rtree::u8_0,
    pub aCellIdx: *mut crate::src::ext::rtree::rtree::u8_0,
    pub aDataOfst: *mut crate::src::ext::rtree::rtree::u8_0,
    pub pDbPage: *mut crate::src::src::pager::DbPage,
    pub xCellSize: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::btreeInt_h::MemPage,
            *mut crate::src::ext::rtree::rtree::u8_0,
        ) -> crate::src::fts5::u16_0,
    >,
    pub xParseCell: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::btreeInt_h::MemPage,
            *mut crate::src::ext::rtree::rtree::u8_0,
            *mut crate::src::headers::btreeInt_h::CellInfo,
        ) -> (),
    >,
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct BtLock {
    pub pBtree: *mut crate::src::headers::btreeInt_h::Btree,
    pub iTable: crate::src::src::pager::Pgno,
    pub eLock: crate::src::ext::rtree::rtree::u8_0,
    pub pNext: *mut crate::src::headers::btreeInt_h::BtLock,
}

pub const READ_LOCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const WRITE_LOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct Btree {
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    pub inTrans: crate::src::ext::rtree::rtree::u8_0,
    pub sharable: crate::src::ext::rtree::rtree::u8_0,
    pub locked: crate::src::ext::rtree::rtree::u8_0,
    pub hasIncrblobCur: crate::src::ext::rtree::rtree::u8_0,
    pub wantToLock: ::core::ffi::c_int,
    pub nBackup: ::core::ffi::c_int,
    pub iBDataVersion: crate::src::ext::rtree::rtree::u32_0,
    pub pNext: *mut crate::src::headers::btreeInt_h::Btree,
    pub pPrev: *mut crate::src::headers::btreeInt_h::Btree,
    pub lock: crate::src::headers::btreeInt_h::BtLock,
}

pub const TRANS_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const TRANS_READ: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const TRANS_WRITE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct BtShared {
    pub pPager: *mut crate::src::src::pager::Pager,
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub pCursor: *mut crate::src::headers::btreeInt_h::BtCursor,
    pub pPage1: *mut crate::src::headers::btreeInt_h::MemPage,
    pub openFlags: crate::src::ext::rtree::rtree::u8_0,
    pub autoVacuum: crate::src::ext::rtree::rtree::u8_0,
    pub incrVacuum: crate::src::ext::rtree::rtree::u8_0,
    pub bDoTruncate: crate::src::ext::rtree::rtree::u8_0,
    pub inTransaction: crate::src::ext::rtree::rtree::u8_0,
    pub max1bytePayload: crate::src::ext::rtree::rtree::u8_0,
    pub nReserveWanted: crate::src::ext::rtree::rtree::u8_0,
    pub btsFlags: crate::src::fts5::u16_0,
    pub maxLocal: crate::src::fts5::u16_0,
    pub minLocal: crate::src::fts5::u16_0,
    pub maxLeaf: crate::src::fts5::u16_0,
    pub minLeaf: crate::src::fts5::u16_0,
    pub pageSize: crate::src::ext::rtree::rtree::u32_0,
    pub usableSize: crate::src::ext::rtree::rtree::u32_0,
    pub nTransaction: ::core::ffi::c_int,
    pub nPage: crate::src::ext::rtree::rtree::u32_0,
    pub pSchema: *mut ::core::ffi::c_void,
    pub xFreeSchema: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub mutex: *mut crate::src::src::mutex_unix::sqlite3_mutex,
    pub pHasContent: *mut crate::src::src::bitvec::Bitvec,
    pub nRef: ::core::ffi::c_int,
    pub pNext: *mut crate::src::headers::btreeInt_h::BtShared,
    pub pLock: *mut crate::src::headers::btreeInt_h::BtLock,
    pub pWriter: *mut crate::src::headers::btreeInt_h::Btree,
    pub pTmpSpace: *mut crate::src::ext::rtree::rtree::u8_0,
    pub nPreformatSize: ::core::ffi::c_int,
}

pub const BTS_READ_ONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const BTS_PAGESIZE_FIXED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const BTS_SECURE_DELETE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const BTS_FAST_SECURE: ::core::ffi::c_int = 0xc as ::core::ffi::c_int;

pub const BTS_INITIALLY_EMPTY: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const BTS_NO_WAL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const BTS_EXCLUSIVE: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const BTS_PENDING: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct CellInfo {
    pub nKey: crate::src::ext::rtree::rtree::i64_0,
    pub pPayload: *mut crate::src::ext::rtree::rtree::u8_0,
    pub nPayload: crate::src::ext::rtree::rtree::u32_0,
    pub nLocal: crate::src::fts5::u16_0,
    pub nSize: crate::src::fts5::u16_0,
}

pub const BTCURSOR_MAX_DEPTH: ::core::ffi::c_int = 20 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct BtCursor {
    pub eState: crate::src::ext::rtree::rtree::u8_0,
    pub curFlags: crate::src::ext::rtree::rtree::u8_0,
    pub curPagerFlags: crate::src::ext::rtree::rtree::u8_0,
    pub hints: crate::src::ext::rtree::rtree::u8_0,
    pub skipNext: ::core::ffi::c_int,
    pub pBtree: *mut crate::src::headers::btreeInt_h::Btree,
    pub aOverflow: *mut crate::src::src::pager::Pgno,
    pub pKey: *mut ::core::ffi::c_void,
    pub pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    pub pNext: *mut crate::src::headers::btreeInt_h::BtCursor,
    pub info: crate::src::headers::btreeInt_h::CellInfo,
    pub nKey: crate::src::ext::rtree::rtree::i64_0,
    pub pgnoRoot: crate::src::src::pager::Pgno,
    pub iPage: crate::src::headers::sqliteInt_h::i8_0,
    pub curIntKey: crate::src::ext::rtree::rtree::u8_0,
    pub ix: crate::src::fts5::u16_0,
    pub aiIdx: [crate::src::fts5::u16_0; 19],
    pub pKeyInfo: *mut crate::src::headers::sqliteInt_h::KeyInfo,
    pub pPage: *mut crate::src::headers::btreeInt_h::MemPage,
    pub apPage: [*mut crate::src::headers::btreeInt_h::MemPage; 19],
}

pub const BTCF_WriteFlag: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const BTCF_ValidNKey: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const BTCF_ValidOvfl: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const BTCF_AtLast: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const BTCF_Incrblob: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const BTCF_Multiple: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const BTCF_Pinned: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const CURSOR_VALID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const CURSOR_INVALID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const CURSOR_SKIPNEXT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const CURSOR_REQUIRESEEK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const CURSOR_FAULT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const PTRMAP_ROOTPAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const PTRMAP_FREEPAGE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const PTRMAP_OVERFLOW1: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const PTRMAP_OVERFLOW2: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const PTRMAP_BTREE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct IntegrityCk {
    pub pBt: *mut crate::src::headers::btreeInt_h::BtShared,
    pub pPager: *mut crate::src::src::pager::Pager,
    pub aPgRef: *mut crate::src::ext::rtree::rtree::u8_0,
    pub nCkPage: crate::src::src::pager::Pgno,
    pub mxErr: ::core::ffi::c_int,
    pub nErr: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub nStep: crate::src::ext::rtree::rtree::u32_0,
    pub zPfx: *const ::core::ffi::c_char,
    pub v0: crate::src::src::pager::Pgno,
    pub v1: crate::src::src::pager::Pgno,
    pub v2: ::core::ffi::c_int,
    pub errMsg: crate::src::headers::sqliteInt_h::StrAccum,
    pub heap: *mut crate::src::ext::rtree::rtree::u32_0,
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub nRow: crate::src::ext::rtree::rtree::i64_0,
}
