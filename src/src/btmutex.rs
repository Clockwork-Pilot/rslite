



















pub use crate::btreeInt_h::BtCursor;pub use crate::btreeInt_h::BtLock;pub use crate::btreeInt_h::BtShared;pub use crate::btreeInt_h::Btree;pub use crate::btreeInt_h::CellInfo;pub use crate::btreeInt_h::MemPage;pub use crate::src::src::hash::Hash;pub use crate::src::src::hash::HashElem;pub use crate::src::src::hash::_ht;pub use crate::src::src::pager::DbPage;pub use crate::src::src::pager::Pager;pub use crate::src::src::pager::Pgno;pub use crate::pcache_h::PCache;pub use crate::src::src::pcache::PgHdr;pub use crate::vdbeInt_h::sqlite3_context;pub use crate::sqlite3_h::sqlite3_file;pub use crate::sqlite3_h::sqlite3_filename;pub use crate::sqlite3_h::sqlite3_index_constraint;pub use crate::sqlite3_h::sqlite3_index_constraint_usage;pub use crate::sqlite3_h::sqlite3_index_info;pub use crate::sqlite3_h::sqlite3_index_orderby;pub use crate::sqlite3_h::sqlite3_int64;pub use crate::sqlite3_h::sqlite3_io_methods;pub use crate::sqlite3_h::sqlite3_module;pub use crate::src::src::mutex_unix::sqlite3_mutex;pub use crate::src::src::mutex::sqlite3_mutex_enter;pub use crate::src::src::mutex::sqlite3_mutex_leave;pub use crate::src::src::mutex::sqlite3_mutex_try;pub use crate::sqlite3_h::sqlite3_pcache_page;pub use crate::sqlite3_h::sqlite3_syscall_ptr;pub use crate::sqlite3_h::sqlite3_uint64;pub use crate::vdbeInt_h::sqlite3_value;pub use crate::sqlite3_h::sqlite3_vfs;pub use crate::sqlite3_h::sqlite3_vtab;pub use crate::sqlite3_h::sqlite3_vtab_cursor;pub use crate::sqlite3_h::sqlite_int64;pub use crate::sqlite3_h::sqlite_uint64;pub use crate::sqlite3_h::SQLITE_OK;pub use crate::sqliteInt_h::bft;pub use crate::src::fts5::i16_0;pub use crate::src::ext::rtree::rtree::i64_0;pub use crate::sqliteInt_h::i8_0;pub use crate::sqliteInt_h::sColMap;pub use crate::sqliteInt_h::sqlite3;pub use crate::sqliteInt_h::sqlite3InitInfo;pub use crate::sqliteInt_h::sqlite3_xauth;pub use crate::src::fts5::u16_0;pub use crate::src::ext::rtree::rtree::u32_0;pub use crate::src::ext::rtree::rtree::u64_0;pub use crate::src::ext::rtree::rtree::u8_0;pub use crate::sqliteInt_h::yDbMask;pub use crate::sqliteInt_h::ynVar;pub use crate::sqliteInt_h::AggInfo;pub use crate::sqliteInt_h::AggInfo_col;pub use crate::sqliteInt_h::AggInfo_func;pub use crate::sqliteInt_h::AutoincInfo;pub use crate::sqliteInt_h::Bitmask;pub use crate::src::src::bitvec::Bitvec;pub use crate::sqliteInt_h::BusyHandler;pub use crate::sqliteInt_h::CollSeq;pub use crate::sqliteInt_h::Column;pub use crate::sqliteInt_h::Cte;pub use crate::sqliteInt_h::CteUse;pub use crate::sqliteInt_h::Db;pub use crate::sqliteInt_h::DbClientData;pub use crate::sqliteInt_h::Expr;pub use crate::sqliteInt_h::ExprList;pub use crate::sqliteInt_h::ExprList_item;pub use crate::sqliteInt_h::FKey;pub use crate::sqliteInt_h::FuncDef;pub use crate::sqliteInt_h::FuncDestructor;pub use crate::sqliteInt_h::IdList;pub use crate::sqliteInt_h::IdList_item;pub use crate::sqliteInt_h::Index;pub use crate::sqliteInt_h::IndexedExpr;pub use crate::sqliteInt_h::KeyInfo;pub use crate::sqliteInt_h::LogEst;pub use crate::sqliteInt_h::Lookaside;pub use crate::sqliteInt_h::LookasideSlot;pub use crate::sqliteInt_h::Module;pub use crate::sqliteInt_h::Parse;pub use crate::sqliteInt_h::ParseCleanup;pub use crate::vdbeInt_h::PreUpdate;pub use crate::sqliteInt_h::RenameToken;pub use crate::sqliteInt_h::Returning;pub use crate::sqliteInt_h::Savepoint;pub use crate::sqliteInt_h::Schema;pub use crate::sqliteInt_h::Select;pub use crate::sqliteInt_h::SrcItem;pub use crate::sqliteInt_h::SrcList;pub use crate::sqliteInt_h::Subquery;pub use crate::sqliteInt_h::Table;pub use crate::sqliteInt_h::TableLock;pub use crate::sqliteInt_h::Token;pub use crate::sqliteInt_h::Trigger;pub use crate::sqliteInt_h::TriggerPrg;pub use crate::sqliteInt_h::TriggerStep;pub use crate::sqliteInt_h::Upsert;pub use crate::sqliteInt_h::VList;pub use crate::sqliteInt_h::VTable;pub use crate::sqliteInt_h::VtabCtx;pub use crate::sqliteInt_h::Window;pub use crate::sqliteInt_h::With;pub use crate::sqliteInt_h::__anon_struct_0;pub use crate::sqliteInt_h::__anon_struct_1;pub use crate::sqliteInt_h::__anon_struct_2;pub use crate::sqliteInt_h::__anon_struct_3;pub use crate::sqliteInt_h::__anon_struct_4;pub use crate::sqliteInt_h::__anon_struct_5;pub use crate::sqliteInt_h::__anon_struct_6;pub use crate::sqliteInt_h::__anon_struct_7;pub use crate::sqliteInt_h::__anon_struct_8;pub use crate::sqliteInt_h::__anon_union_0;pub use crate::sqliteInt_h::__anon_union_1;pub use crate::sqliteInt_h::__anon_union_10;pub use crate::sqliteInt_h::__anon_union_11;pub use crate::sqliteInt_h::__anon_union_12;pub use crate::sqliteInt_h::__anon_union_13;pub use crate::sqliteInt_h::__anon_union_15;pub use crate::sqliteInt_h::__anon_union_2;pub use crate::sqliteInt_h::__anon_union_3;pub use crate::sqliteInt_h::__anon_union_5;pub use crate::sqliteInt_h::__anon_union_6;pub use crate::sqliteInt_h::__anon_union_7;pub use crate::sqliteInt_h::__anon_union_8;pub use crate::sqliteInt_h::__anon_union_9;pub use crate::stdlib::int16_t;pub use crate::stdlib::int8_t;pub use crate::stdlib::uint16_t;pub use crate::stdlib::uint32_t;pub use crate::stdlib::uint8_t;pub use crate::stdlib::__int16_t;pub use crate::stdlib::__int8_t;pub use crate::stdlib::__uint16_t;pub use crate::stdlib::__uint32_t;pub use crate::stdlib::__uint8_t;pub use crate::src::src::vdbe::p4union;pub use crate::src::src::vdbe::Mem;pub use crate::src::src::vdbe::SubProgram;pub use crate::src::src::vdbe::SubrtnSig;pub use crate::vdbeInt_h::Vdbe;pub use crate::src::src::vdbe::VdbeOp;

unsafe extern "C" fn lockBtreeMutex(mut p: *mut crate::btreeInt_h::Btree) {
    let __p_ref = unsafe { &mut *p };
    crate::src::src::mutex::sqlite3_mutex_enter((*__p_ref.pBt).mutex);
    (*__p_ref.pBt).db = __p_ref.db;
    __p_ref.locked = 1 as crate::src::ext::rtree::rtree::u8_0;
}
#[inline(never)]

unsafe extern "C" fn unlockBtreeMutex(mut p: *mut crate::btreeInt_h::Btree) {
    let mut pBt: *mut crate::btreeInt_h::BtShared = (*p).pBt;
    crate::src::src::mutex::sqlite3_mutex_leave((*pBt).mutex);
    (*p).locked = 0 as crate::src::ext::rtree::rtree::u8_0;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3BtreeEnter(mut p: *mut crate::btreeInt_h::Btree) {
    let __p_ref = unsafe { &mut *p };
    if __p_ref.sharable == 0 {
        return;
    }
    __p_ref.wantToLock += 1;
    if __p_ref.locked != 0 {
        return;
    }
    btreeLockCarefully(p);
}
#[inline(never)]

unsafe extern "C" fn btreeLockCarefully(mut p: *mut crate::btreeInt_h::Btree) {
    let mut pLater: *mut crate::btreeInt_h::Btree = ::core::ptr::null_mut::<crate::btreeInt_h::Btree>();
    let __p_ref = unsafe { &mut *p };
    if crate::src::src::mutex::sqlite3_mutex_try((*__p_ref.pBt).mutex) == crate::sqlite3_h::SQLITE_OK {
        (*__p_ref.pBt).db = __p_ref.db;
        __p_ref.locked = 1 as crate::src::ext::rtree::rtree::u8_0;
        return;
    }
    pLater = __p_ref.pNext;
    while !pLater.is_null() {
        if (*pLater).locked != 0 {
            unlockBtreeMutex(pLater);
        }
        pLater = (*pLater).pNext;
    }
    lockBtreeMutex(p);
    pLater = __p_ref.pNext;
    while !pLater.is_null() {
        if (*pLater).wantToLock != 0 {
            lockBtreeMutex(pLater);
        }
        pLater = (*pLater).pNext;
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3BtreeLeave(mut p: *mut crate::btreeInt_h::Btree) {
    if (*p).sharable != 0 {
        (*p).wantToLock -= 1;
        if (*p).wantToLock == 0 as ::core::ffi::c_int {
            unlockBtreeMutex(p);
        }
    }
}
#[inline(never)]

unsafe extern "C" fn btreeEnterAll(mut db: *mut crate::sqliteInt_h::sqlite3) {
    let mut i: ::core::ffi::c_int = 0;
    let mut skipOk: crate::src::ext::rtree::rtree::u8_0 = 1 as crate::src::ext::rtree::rtree::u8_0;
    let mut p: *mut crate::btreeInt_h::Btree = ::core::ptr::null_mut::<crate::btreeInt_h::Btree>();
    i = 0 as ::core::ffi::c_int;
    while i < (*db).nDb {
        p = (*(*db).aDb.offset(i as isize)).pBt;
        if !p.is_null() && (*p).sharable as ::core::ffi::c_int != 0 {
            sqlite3BtreeEnter(p);
            skipOk = 0 as crate::src::ext::rtree::rtree::u8_0;
        }
        i += 1;
    }
    (*db).noSharedCache = skipOk;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3BtreeEnterAll(mut db: *mut crate::sqliteInt_h::sqlite3) {
    if (*db).noSharedCache as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        btreeEnterAll(db);
    }
}
#[inline(never)]

unsafe extern "C" fn btreeLeaveAll(mut db: *mut crate::sqliteInt_h::sqlite3) {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *mut crate::btreeInt_h::Btree = ::core::ptr::null_mut::<crate::btreeInt_h::Btree>();
    i = 0 as ::core::ffi::c_int;
    while i < (*db).nDb {
        p = (*(*db).aDb.offset(i as isize)).pBt;
        if !p.is_null() {
            sqlite3BtreeLeave(p);
        }
        i += 1;
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3BtreeLeaveAll(mut db: *mut crate::sqliteInt_h::sqlite3) {
    if (*db).noSharedCache as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        btreeLeaveAll(db);
    }
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3BtreeEnterCursor(mut pCur: *mut crate::btreeInt_h::BtCursor) {
    sqlite3BtreeEnter((*pCur).pBtree);
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3BtreeLeaveCursor(mut pCur: *mut crate::btreeInt_h::BtCursor) {
    sqlite3BtreeLeave((*pCur).pBtree);
}
