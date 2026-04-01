
















pub use crate::src::headers::stdlib::va_list;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::src::printf_c_variadic::execSqlF;


pub use crate::src::src::btree::sqlite3BtreeBeginTrans;pub use crate::src::src::btree::sqlite3BtreeClose;pub use crate::src::src::btree::sqlite3BtreeCommit;pub use crate::src::src::backup::sqlite3BtreeCopyFile;pub use crate::src::src::btree::sqlite3BtreeGetAutoVacuum;pub use crate::src::src::btree::sqlite3BtreeGetMeta;pub use crate::src::src::btree::sqlite3BtreeGetPageSize;pub use crate::src::src::btree::sqlite3BtreeGetRequestedReserve;pub use crate::src::src::btree::sqlite3BtreePager;pub use crate::src::src::btree::sqlite3BtreeSetAutoVacuum;pub use crate::src::src::btree::sqlite3BtreeSetCacheSize;pub use crate::src::src::btree::sqlite3BtreeSetPageSize;pub use crate::src::src::btree::sqlite3BtreeSetPagerFlags;pub use crate::src::src::btree::sqlite3BtreeSetSpillSize;pub use crate::src::src::btree::sqlite3BtreeUpdateMeta;pub use crate::src::headers::btreeInt_h::BtCursor;pub use crate::src::headers::btreeInt_h::Btree;pub use crate::src::src::btree::BTREE_APPLICATION_ID;pub use crate::src::src::btree::BTREE_DEFAULT_CACHE_SIZE;pub use crate::src::src::btree::BTREE_SCHEMA_VERSION;pub use crate::src::src::btree::BTREE_TEXT_ENCODING;pub use crate::src::src::btree::BTREE_USER_VERSION;pub use crate::src::src::hash::Hash;pub use crate::src::src::hash::HashElem;pub use crate::src::src::hash::_ht;pub use crate::internal::__builtin_va_list;pub use crate::internal::__va_list_tag;
pub use crate::src::headers::opcodes_h::OP_Vacuum;





pub use crate::src::src::pager::sqlite3PagerFile;pub use crate::src::src::pager::sqlite3PagerGetJournalMode;pub use crate::src::src::pager::sqlite3PagerIsMemdb;pub use crate::src::src::pager::Pager;pub use crate::src::src::pager::Pgno;pub use crate::src::src::pager::PAGER_CACHESPILL;pub use crate::src::src::pager::PAGER_FLAGS_MASK;pub use crate::src::src::pager::PAGER_JOURNALMODE_WAL;pub use crate::src::src::pager::PAGER_SYNCHRONOUS_OFF;pub use crate::src::src::vdbeapi::sqlite3_column_text;pub use crate::src::src::main::sqlite3_errmsg;pub use crate::src::headers::sqlite3_h::sqlite3_file;pub use crate::src::headers::sqlite3_h::sqlite3_filename;pub use crate::src::src::vdbeapi::sqlite3_finalize;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;pub use crate::src::headers::sqlite3_h::sqlite3_index_info;pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;pub use crate::src::headers::sqlite3_h::sqlite3_int64;pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;pub use crate::src::headers::sqlite3_h::sqlite3_module;pub use crate::src::src::mutex_unix::sqlite3_mutex;pub use crate::src::src::prepare::sqlite3_prepare_v2;pub use crate::src::src::random::sqlite3_randomness;pub use crate::src::src::vdbeapi::sqlite3_step;pub use crate::src::headers::sqlite3_h::sqlite3_stmt;pub use crate::src::headers::sqlite3_h::sqlite3_syscall_ptr;pub use crate::src::headers::sqlite3_h::sqlite3_uint64;pub use crate::src::src::vdbeapi::sqlite3_value_text;pub use crate::src::src::vdbeapi::sqlite3_value_type;pub use crate::src::headers::sqlite3_h::sqlite3_vfs;pub use crate::src::headers::sqlite3_h::sqlite3_vtab;pub use crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;pub use crate::src::headers::sqlite3_h::sqlite_int64;pub use crate::src::headers::sqlite3_h::sqlite_uint64;pub use crate::src::headers::sqlite3_h::SQLITE_DONE;pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;pub use crate::src::headers::sqlite3_h::SQLITE_OK;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_CREATE;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_READONLY;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_READWRITE;pub use crate::src::headers::sqlite3_h::SQLITE_ROW;pub use crate::src::headers::sqlite3_h::SQLITE_TEXT;pub use crate::src::headers::sqliteInt_h::bft;pub use crate::src::fts5::i16_0;pub use crate::src::ext::rtree::rtree::i64_0;pub use crate::src::headers::sqliteInt_h::i8_0;pub use crate::src::headers::sqliteInt_h::sColMap;pub use crate::src::headers::sqliteInt_h::sqlite3;pub use crate::src::src::malloc::sqlite3DbFree;pub use crate::src::src::expr::sqlite3ExprCode;pub use crate::src::src::expr::sqlite3ExprDelete;pub use crate::src::src::select::sqlite3GetVdbe;pub use crate::src::headers::sqliteInt_h::sqlite3InitInfo;pub use crate::src::src::build::sqlite3ResetAllSchemasOfConnection;pub use crate::src::src::resolve::sqlite3ResolveSelfReference;pub use crate::src::src::malloc::sqlite3SetString;pub use crate::src::src::build::sqlite3TwoPartName;pub use crate::src::src::printf::sqlite3VMPrintf;pub use crate::src::headers::sqliteInt_h::sqlite3_xauth;pub use crate::src::fts5::u16_0;pub use crate::src::ext::rtree::rtree::u32_0;pub use crate::src::ext::rtree::rtree::u64_0;pub use crate::src::ext::rtree::rtree::u8_0;pub use crate::src::headers::sqliteInt_h::yDbMask;pub use crate::src::headers::sqliteInt_h::ynVar;pub use crate::src::headers::sqliteInt_h::AggInfo;pub use crate::src::headers::sqliteInt_h::AggInfo_col;pub use crate::src::headers::sqliteInt_h::AggInfo_func;pub use crate::src::headers::sqliteInt_h::AutoincInfo;pub use crate::src::headers::sqliteInt_h::Bitmask;pub use crate::src::headers::sqliteInt_h::BusyHandler;pub use crate::src::headers::sqliteInt_h::CollSeq;pub use crate::src::headers::sqliteInt_h::Column;pub use crate::src::headers::sqliteInt_h::Cte;pub use crate::src::headers::sqliteInt_h::CteUse;pub use crate::src::headers::sqliteInt_h::DBFLAG_PreferBuiltin;pub use crate::src::headers::sqliteInt_h::DBFLAG_Vacuum;pub use crate::src::headers::sqliteInt_h::DBFLAG_VacuumInto;pub use crate::src::headers::sqliteInt_h::Db;pub use crate::src::headers::sqliteInt_h::DbClientData;pub use crate::src::headers::sqliteInt_h::Expr;pub use crate::src::headers::sqliteInt_h::ExprList;pub use crate::src::headers::sqliteInt_h::ExprList_item;pub use crate::src::headers::sqliteInt_h::FKey;pub use crate::src::headers::sqliteInt_h::FuncDef;pub use crate::src::headers::sqliteInt_h::FuncDestructor;pub use crate::src::headers::sqliteInt_h::IdList;pub use crate::src::headers::sqliteInt_h::IdList_item;pub use crate::src::headers::sqliteInt_h::Index;pub use crate::src::headers::sqliteInt_h::IndexedExpr;pub use crate::src::headers::sqliteInt_h::KeyInfo;pub use crate::src::headers::sqliteInt_h::LogEst;pub use crate::src::headers::sqliteInt_h::Lookaside;pub use crate::src::headers::sqliteInt_h::LookasideSlot;pub use crate::src::headers::sqliteInt_h::Module;pub use crate::src::headers::sqliteInt_h::Parse;pub use crate::src::headers::sqliteInt_h::ParseCleanup;pub use crate::src::headers::sqliteInt_h::RenameToken;pub use crate::src::headers::sqliteInt_h::Returning;pub use crate::src::headers::sqliteInt_h::SQLITE_AttachCreate;pub use crate::src::headers::sqliteInt_h::SQLITE_AttachWrite;pub use crate::src::headers::sqliteInt_h::SQLITE_Comments;pub use crate::src::headers::sqliteInt_h::SQLITE_CountRows;pub use crate::src::headers::sqliteInt_h::SQLITE_Defensive;pub use crate::src::headers::sqliteInt_h::SQLITE_ForeignKeys;pub use crate::src::headers::sqliteInt_h::SQLITE_IgnoreChecks;pub use crate::src::headers::sqliteInt_h::SQLITE_ReverseOrder;pub use crate::src::headers::sqliteInt_h::SQLITE_WriteSchema;pub use crate::src::headers::sqliteInt_h::Savepoint;pub use crate::src::headers::sqliteInt_h::Schema;pub use crate::src::headers::sqliteInt_h::Select;pub use crate::src::headers::sqliteInt_h::SrcItem;pub use crate::src::headers::sqliteInt_h::SrcList;pub use crate::src::headers::sqliteInt_h::Subquery;pub use crate::src::headers::sqliteInt_h::Table;pub use crate::src::headers::sqliteInt_h::TableLock;pub use crate::src::headers::sqliteInt_h::Token;pub use crate::src::headers::sqliteInt_h::Trigger;pub use crate::src::headers::sqliteInt_h::TriggerPrg;pub use crate::src::headers::sqliteInt_h::TriggerStep;pub use crate::src::headers::sqliteInt_h::UnpackedRecord;pub use crate::src::headers::sqliteInt_h::Upsert;pub use crate::src::headers::sqliteInt_h::VList;pub use crate::src::headers::sqliteInt_h::VTable;pub use crate::src::headers::sqliteInt_h::VtabCtx;pub use crate::src::headers::sqliteInt_h::Window;pub use crate::src::headers::sqliteInt_h::With;pub use crate::src::headers::sqliteInt_h::__anon_struct_0;pub use crate::src::headers::sqliteInt_h::__anon_struct_1;pub use crate::src::headers::sqliteInt_h::__anon_struct_2;pub use crate::src::headers::sqliteInt_h::__anon_struct_3;pub use crate::src::headers::sqliteInt_h::__anon_struct_4;pub use crate::src::headers::sqliteInt_h::__anon_struct_5;pub use crate::src::headers::sqliteInt_h::__anon_struct_6;pub use crate::src::headers::sqliteInt_h::__anon_struct_7;pub use crate::src::headers::sqliteInt_h::__anon_struct_8;pub use crate::src::headers::sqliteInt_h::__anon_union_0;pub use crate::src::headers::sqliteInt_h::__anon_union_1;pub use crate::src::headers::sqliteInt_h::__anon_union_10;pub use crate::src::headers::sqliteInt_h::__anon_union_11;pub use crate::src::headers::sqliteInt_h::__anon_union_12;pub use crate::src::headers::sqliteInt_h::__anon_union_13;pub use crate::src::headers::sqliteInt_h::__anon_union_15;pub use crate::src::headers::sqliteInt_h::__anon_union_2;pub use crate::src::headers::sqliteInt_h::__anon_union_3;pub use crate::src::headers::sqliteInt_h::__anon_union_4;pub use crate::src::headers::sqliteInt_h::__anon_union_5;pub use crate::src::headers::sqliteInt_h::__anon_union_6;pub use crate::src::headers::sqliteInt_h::__anon_union_7;pub use crate::src::headers::sqliteInt_h::__anon_union_8;pub use crate::src::headers::sqliteInt_h::__anon_union_9;pub use crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;pub use crate::src::headers::stdlib::int16_t;pub use crate::src::headers::stdlib::int8_t;pub use crate::src::headers::stdlib::uint16_t;pub use crate::src::headers::stdlib::uint32_t;pub use crate::src::headers::stdlib::uint8_t;



pub use crate::src::headers::stdlib::__int16_t;pub use crate::src::headers::stdlib::__int8_t;pub use crate::src::headers::stdlib::__uint16_t;pub use crate::src::headers::stdlib::__uint32_t;pub use crate::src::headers::stdlib::__uint8_t;pub use crate::src::headers::vdbeInt_h::sqlite3_context;pub use crate::src::headers::vdbeInt_h::sqlite3_value;pub use crate::src::headers::vdbeInt_h::AuxData;pub use crate::src::headers::vdbeInt_h::Bool;pub use crate::src::headers::vdbeInt_h::MemValue;pub use crate::src::headers::vdbeInt_h::Op;pub use crate::src::headers::vdbeInt_h::PreUpdate;pub use crate::src::headers::vdbeInt_h::Vdbe;pub use crate::src::headers::vdbeInt_h::VdbeCursor;pub use crate::src::headers::vdbeInt_h::VdbeFrame;pub use crate::src::headers::vdbeInt_h::VdbeSorter;pub use crate::src::headers::vdbeInt_h::VdbeTxtBlbCache;pub use crate::src::headers::vdbeInt_h::__anon_struct_10;pub use crate::src::headers::vdbeInt_h::__anon_union_17;pub use crate::src::headers::vdbeInt_h::__anon_union_18;pub use crate::src::src::vdbe::p4union;pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp2;pub use crate::src::src::vdbeaux::sqlite3VdbeUsesBtree;pub use crate::src::src::vdbe::Mem;pub use crate::src::src::vdbe::SubProgram;pub use crate::src::src::vdbe::SubrtnSig;pub use crate::src::src::vdbe::VdbeOp;

pub unsafe extern "C" fn execSql(
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut zSql: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = crate::src::src::prepare::sqlite3_prepare_v2(
        
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        zSql,
        -(1 as ::core::ffi::c_int),
        &raw mut pStmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    loop {
        rc = crate::src::src::vdbeapi::sqlite3_step(pStmt);
        if !(crate::src::headers::sqlite3_h::SQLITE_ROW == rc) {
            break;
        }
        let mut zSubSql: *const ::core::ffi::c_char =
            crate::src::src::vdbeapi::sqlite3_column_text(pStmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !(!zSubSql.is_null()
            && (::libc::strncmp(
                zSubSql,
                b"CRE\0" as *const u8 as *const ::core::ffi::c_char,
                3 as crate::__stddef_size_t_h::size_t,
            ) == 0 as ::core::ffi::c_int
                || ::libc::strncmp(
                    zSubSql,
                    b"INS\0" as *const u8 as *const ::core::ffi::c_char,
                    3 as crate::__stddef_size_t_h::size_t,
                ) == 0 as ::core::ffi::c_int))
        {
            continue;
        }
        rc = execSql(db, pzErrMsg, zSubSql);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            break;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    if rc != 0 {
        crate::src::src::malloc::sqlite3SetString(pzErrMsg,  db as *mut crate::src::headers::sqliteInt_h::sqlite3, crate::src::src::main::sqlite3_errmsg(db as *mut crate::src::headers::sqliteInt_h::sqlite3));
    }
    crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
    rc
}

#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3Vacuum(
    mut pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut pNm: *mut crate::src::headers::sqliteInt_h::Token,
    mut pInto: *mut crate::src::headers::sqliteInt_h::Expr,
) {
    let mut current_block: u64;
    let mut v: *mut crate::src::headers::vdbeInt_h::Vdbe = crate::src::src::select::sqlite3GetVdbe(pParse as *mut crate::src::headers::sqliteInt_h::Parse);
    let mut iDb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !v.is_null() {
        if !((*pParse).nErr != 0) {
            if !pNm.is_null() {
                iDb = crate::src::src::build::sqlite3TwoPartName(pParse as *mut crate::src::headers::sqliteInt_h::Parse,  pNm as *mut crate::src::headers::sqliteInt_h::Token,  pNm as *mut crate::src::headers::sqliteInt_h::Token,  &raw mut pNm as *mut _ as *mut *mut crate::src::headers::sqliteInt_h::Token);
                if iDb < 0 as ::core::ffi::c_int {
                    current_block = 14120926627757568903;
                } else {
                    current_block = 15427931788582360902;
                }
            } else {
                current_block = 15427931788582360902;
            }
            match current_block {
                14120926627757568903 => {}
                _ => {
                    if iDb != 1 as ::core::ffi::c_int {
                        let mut iIntoReg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        if !pInto.is_null()
                            && crate::src::src::resolve::sqlite3ResolveSelfReference(
                                
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                
                                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>() as
    *mut crate::src::headers::sqliteInt_h::Table,
                                0 as ::core::ffi::c_int,
                                
                                pInto as *mut crate::src::headers::sqliteInt_h::Expr,
                                
                                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>() as
    *mut crate::src::headers::sqliteInt_h::ExprList,
                            ) == 0 as ::core::ffi::c_int
                        {
                            (*pParse).nMem += 1;
                            iIntoReg = (*pParse).nMem;
                            crate::src::src::expr::sqlite3ExprCode(pParse as *mut crate::src::headers::sqliteInt_h::Parse,  pInto as *mut crate::src::headers::sqliteInt_h::Expr, iIntoReg);
                        }
                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(v as *mut crate::src::headers::vdbeInt_h::Vdbe, crate::src::headers::opcodes_h::OP_Vacuum, iDb, iIntoReg);
                        crate::src::src::vdbeaux::sqlite3VdbeUsesBtree(v as *mut crate::src::headers::vdbeInt_h::Vdbe, iDb);
                    }
                }
            }
        }
    }
    crate::src::src::expr::sqlite3ExprDelete((*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,  pInto as *mut crate::src::headers::sqliteInt_h::Expr);
}
#[unsafe(no_mangle)]
#[inline(never)]

pub unsafe extern "C" fn sqlite3RunVacuum(
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut iDb: ::core::ffi::c_int,
    mut pOut: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pMain: *mut crate::src::headers::btreeInt_h::Btree = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::Btree>();
    let mut pTemp: *mut crate::src::headers::btreeInt_h::Btree = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::Btree>();
    let mut saved_mDbFlags: crate::src::ext::rtree::rtree::u32_0 = 0;
    let mut saved_flags: crate::src::ext::rtree::rtree::u64_0 = 0;
    let mut saved_nChange: crate::src::ext::rtree::rtree::i64_0 = 0;
    let mut saved_nTotalChange: crate::src::ext::rtree::rtree::i64_0 = 0;
    let mut saved_openFlags: crate::src::ext::rtree::rtree::u32_0 = 0;
    let mut saved_mTrace: crate::src::ext::rtree::rtree::u8_0 = 0;
    let mut pDb: *mut crate::src::headers::sqliteInt_h::Db = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Db>();
    let mut isMemDb: ::core::ffi::c_int = 0;
    let mut nRes: ::core::ffi::c_int = 0;
    let mut nDb: ::core::ffi::c_int = 0;
    let mut zDbMain: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut zOut: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut pgflags: crate::src::ext::rtree::rtree::u32_0 = crate::src::src::pager::PAGER_SYNCHRONOUS_OFF as crate::src::ext::rtree::rtree::u32_0;
    let mut iRandom: crate::src::ext::rtree::rtree::u64_0 = 0;
    let mut zDbVacuum: [::core::ffi::c_char; 42] = [0; 42];
    let __db_ref = unsafe { &mut *db };
    if __db_ref.autoCommit == 0 {
        crate::src::src::malloc::sqlite3SetString(
            pzErrMsg,
            
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            b"cannot VACUUM from within a transaction\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    if __db_ref.nVdbeActive > 1 as ::core::ffi::c_int {
        crate::src::src::malloc::sqlite3SetString(
            pzErrMsg,
            
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            b"cannot VACUUM - SQL statements in progress\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    saved_openFlags = __db_ref.openFlags as crate::src::ext::rtree::rtree::u32_0;
    if !pOut.is_null() {
        if crate::src::src::vdbeapi::sqlite3_value_type(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value) != crate::src::headers::sqlite3_h::SQLITE_TEXT {
            crate::src::src::malloc::sqlite3SetString(
                pzErrMsg,
                
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                b"non-text filename\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return crate::src::headers::sqlite3_h::SQLITE_ERROR;
        }
        zOut = crate::src::src::vdbeapi::sqlite3_value_text(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value) as *const ::core::ffi::c_char;
        __db_ref.openFlags &= !crate::src::headers::sqlite3_h::SQLITE_OPEN_READONLY as ::core::ffi::c_uint;
        __db_ref.openFlags |= (crate::src::headers::sqlite3_h::SQLITE_OPEN_CREATE | crate::src::headers::sqlite3_h::SQLITE_OPEN_READWRITE) as ::core::ffi::c_uint;
    } else {
        zOut = b"\0" as *const u8 as *const ::core::ffi::c_char;
    }
    saved_flags = __db_ref.flags;
    saved_mDbFlags = __db_ref.mDbFlags;
    saved_nChange = __db_ref.nChange;
    saved_nTotalChange = __db_ref.nTotalChange;
    saved_mTrace = __db_ref.mTrace;
    __db_ref.flags |= (crate::src::headers::sqliteInt_h::SQLITE_WriteSchema | crate::src::headers::sqliteInt_h::SQLITE_IgnoreChecks) as crate::src::ext::rtree::rtree::u64_0
        | crate::src::headers::sqliteInt_h::SQLITE_Comments
        | crate::src::headers::sqliteInt_h::SQLITE_AttachCreate
        | crate::src::headers::sqliteInt_h::SQLITE_AttachWrite;
    __db_ref.mDbFlags |= (crate::src::headers::sqliteInt_h::DBFLAG_PreferBuiltin | crate::src::headers::sqliteInt_h::DBFLAG_Vacuum) as crate::src::ext::rtree::rtree::u32_0;
    __db_ref.flags &= !((crate::src::headers::sqliteInt_h::SQLITE_ForeignKeys | crate::src::headers::sqliteInt_h::SQLITE_ReverseOrder | crate::src::headers::sqliteInt_h::SQLITE_Defensive) as crate::src::ext::rtree::rtree::u64_0
        | crate::src::headers::sqliteInt_h::SQLITE_CountRows);
    __db_ref.mTrace = 0 as crate::src::ext::rtree::rtree::u8_0;
    zDbMain = (*__db_ref.aDb.offset(iDb as isize)).zDbSName;
    pMain = (*__db_ref.aDb.offset(iDb as isize)).pBt;
    isMemDb = crate::src::src::pager::sqlite3PagerIsMemdb(crate::src::src::btree::sqlite3BtreePager(pMain) as *mut crate::src::src::pager::Pager);
    crate::src::src::random::sqlite3_randomness(
        ::core::mem::size_of::<crate::src::ext::rtree::rtree::u64_0>() as ::core::ffi::c_int,
        &raw mut iRandom as *mut ::core::ffi::c_void,
    );
    crate::sqlite_snprintf!(
        &raw mut zDbVacuum as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 42]>() as ::core::ffi::c_int,
        "vacuum_%016llx",
        iRandom,
    );
    nDb = __db_ref.nDb;
    rc = execSqlF(
        db,
        pzErrMsg,
        b"ATTACH %Q AS %s\0" as *const u8 as *const ::core::ffi::c_char,
        zOut,
        &raw mut zDbVacuum as *mut ::core::ffi::c_char,
    );
    __db_ref.openFlags = saved_openFlags as ::core::ffi::c_uint;
    if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
        pDb = __db_ref.aDb.offset(nDb as isize) as *mut crate::src::headers::sqliteInt_h::Db;
        pTemp = (*pDb).pBt;
        if !pOut.is_null() {
            let mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file =
                
                crate::src::src::pager::sqlite3PagerFile(crate::src::src::btree::sqlite3BtreePager(pTemp) as *mut crate::src::src::pager::Pager) as
    *mut crate::src::headers::sqlite3_h::sqlite3_file;
            let mut sz: crate::src::ext::rtree::rtree::i64_0 = 0 as crate::src::ext::rtree::rtree::i64_0;
            if !(*id).pMethods.is_null()
                && (crate::src::src::os::sqlite3OsFileSize(id as *mut crate::src::headers::sqlite3_h::sqlite3_file, &raw mut sz) != crate::src::headers::sqlite3_h::SQLITE_OK || sz > 0 as crate::src::ext::rtree::rtree::i64_0)
            {
                rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                crate::src::src::malloc::sqlite3SetString(
                    pzErrMsg,
                    
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    b"output file already exists\0" as *const u8 as *const ::core::ffi::c_char,
                );
                current_block = 3638292225213388937;
            } else {
                __db_ref.mDbFlags |= crate::src::headers::sqliteInt_h::DBFLAG_VacuumInto as crate::src::ext::rtree::rtree::u32_0;
                pgflags = ((*__db_ref.aDb.offset(iDb as isize)).safety_level as crate::src::ext::rtree::rtree::u64_0
                    | __db_ref.flags & crate::src::src::pager::PAGER_FLAGS_MASK as crate::src::ext::rtree::rtree::u64_0) as crate::src::ext::rtree::rtree::u32_0;
                current_block = 6450597802325118133;
            }
        } else {
            current_block = 6450597802325118133;
        }
        match current_block {
            3638292225213388937 => {}
            _ => {
                nRes = crate::src::src::btree::sqlite3BtreeGetRequestedReserve(pMain);
                crate::src::src::btree::sqlite3BtreeSetCacheSize(
                    pTemp,
                    (*(*__db_ref.aDb.offset(iDb as isize)).pSchema).cache_size,
                );
                crate::src::src::btree::sqlite3BtreeSetSpillSize(
                    pTemp,
                    crate::src::src::btree::sqlite3BtreeSetSpillSize(pMain, 0 as ::core::ffi::c_int),
                );
                crate::src::src::btree::sqlite3BtreeSetPagerFlags(
                    pTemp,
                    pgflags as ::core::ffi::c_uint | crate::src::src::pager::PAGER_CACHESPILL as ::core::ffi::c_uint,
                );
                rc = execSql(
                    db,
                    pzErrMsg,
                    b"BEGIN\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                    rc = crate::src::src::btree::sqlite3BtreeBeginTrans(
                        pMain,
                        if pOut.is_null() {
                            2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        },
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    );
                    if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                        if crate::src::src::pager::sqlite3PagerGetJournalMode(crate::src::src::btree::sqlite3BtreePager(pMain) as *mut crate::src::src::pager::Pager)
                            == crate::src::src::pager::PAGER_JOURNALMODE_WAL
                            && pOut.is_null()
                        {
                            __db_ref.nextPagesize = 0 as ::core::ffi::c_int;
                        }
                        if crate::src::src::btree::sqlite3BtreeSetPageSize(
                            pTemp,
                            crate::src::src::btree::sqlite3BtreeGetPageSize(pMain),
                            nRes,
                            0 as ::core::ffi::c_int,
                        ) != 0
                            || isMemDb == 0
                                && crate::src::src::btree::sqlite3BtreeSetPageSize(
                                    pTemp,
                                    __db_ref.nextPagesize,
                                    nRes,
                                    0 as ::core::ffi::c_int,
                                ) != 0
                            || __db_ref.mallocFailed as ::core::ffi::c_int != 0
                        {
                            rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
                        } else {
                            crate::src::src::btree::sqlite3BtreeSetAutoVacuum(
                                pTemp,
                                if __db_ref.nextAutovac as ::core::ffi::c_int
                                    >= 0 as ::core::ffi::c_int
                                {
                                    __db_ref.nextAutovac as ::core::ffi::c_int
                                } else {
                                    crate::src::src::btree::sqlite3BtreeGetAutoVacuum(pMain)
                                },
                            );
                            __db_ref.init.iDb = nDb as crate::src::ext::rtree::rtree::u8_0;
                            rc = execSqlF(
                                db,
                                pzErrMsg,
                                b"SELECT sql FROM \"%w\".sqlite_schema WHERE type='table'AND name<>'sqlite_sequence' AND coalesce(rootpage,1)>0\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                zDbMain,
                            );
                            if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                                rc = execSqlF(
                                    db,
                                    pzErrMsg,
                                    b"SELECT sql FROM \"%w\".sqlite_schema WHERE type='index'\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    zDbMain,
                                );
                                if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                                    __db_ref.init.iDb = 0 as crate::src::ext::rtree::rtree::u8_0;
                                    rc = execSqlF(
                                        db,
                                        pzErrMsg,
                                        b"SELECT'INSERT INTO %s.'||quote(name)||' SELECT*FROM\"%w\".'||quote(name)FROM %s.sqlite_schema WHERE type='table'AND coalesce(rootpage,1)>0\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                        &raw mut zDbVacuum as *mut ::core::ffi::c_char,
                                        zDbMain,
                                        &raw mut zDbVacuum as *mut ::core::ffi::c_char,
                                    );
                                    __db_ref.mDbFlags &= !crate::src::headers::sqliteInt_h::DBFLAG_Vacuum as crate::src::ext::rtree::rtree::u32_0;
                                    if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                                        rc = execSqlF(
                                            db,
                                            pzErrMsg,
                                            b"INSERT INTO %s.sqlite_schema SELECT*FROM \"%w\".sqlite_schema WHERE type IN('view','trigger') OR(type='table'AND rootpage=0)\0"
                                                as *const u8 as *const ::core::ffi::c_char,
                                            &raw mut zDbVacuum as *mut ::core::ffi::c_char,
                                            zDbMain,
                                        );
                                        if !(rc != 0) {
                                            let mut meta: crate::src::ext::rtree::rtree::u32_0 = 0;
                                            let mut i: ::core::ffi::c_int = 0;
                                            static mut aCopy: [::core::ffi::c_uchar; 10] = [
                                                crate::src::src::btree::BTREE_SCHEMA_VERSION as ::core::ffi::c_uchar,
                                                1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                crate::src::src::btree::BTREE_DEFAULT_CACHE_SIZE as ::core::ffi::c_uchar,
                                                0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                crate::src::src::btree::BTREE_TEXT_ENCODING as ::core::ffi::c_uchar,
                                                0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                crate::src::src::btree::BTREE_USER_VERSION as ::core::ffi::c_uchar,
                                                0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                crate::src::src::btree::BTREE_APPLICATION_ID as ::core::ffi::c_uchar,
                                                0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                            ];
                                            i = 0 as ::core::ffi::c_int;
                                            loop {
                                                if !(i
                                                    < (::core::mem::size_of::<
                                                        [::core::ffi::c_uchar; 10],
                                                    >(
                                                    )
                                                        as usize)
                                                        .wrapping_div(::core::mem::size_of::<
                                                            ::core::ffi::c_uchar,
                                                        >(
                                                        )
                                                            as usize)
                                                        as ::core::ffi::c_int)
                                                {
                                                    current_block = 12758904613967585247;
                                                    break;
                                                }
                                                crate::src::src::btree::sqlite3BtreeGetMeta(
                                                    pMain,
                                                    aCopy[i as usize] as ::core::ffi::c_int,
                                                    &raw mut meta,
                                                );
                                                rc = crate::src::src::btree::sqlite3BtreeUpdateMeta(
                                                    pTemp,
                                                    aCopy[i as usize] as ::core::ffi::c_int,
                                                    meta.wrapping_add(
                                                        aCopy
                                                            [(i + 1 as ::core::ffi::c_int) as usize]
                                                            as crate::src::ext::rtree::rtree::u32_0,
                                                    ),
                                                );
                                                if rc != 0 as ::core::ffi::c_int {
                                                    current_block = 3638292225213388937;
                                                    break;
                                                }
                                                i += 2 as ::core::ffi::c_int;
                                            }
                                            match current_block {
                                                3638292225213388937 => {}
                                                _ => {
                                                    if pOut.is_null() {
                                                        rc = crate::src::src::backup::sqlite3BtreeCopyFile(pMain, pTemp);
                                                    }
                                                    if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                                                        rc = crate::src::src::btree::sqlite3BtreeCommit(pTemp);
                                                        if !(rc != crate::src::headers::sqlite3_h::SQLITE_OK) {
                                                            if pOut.is_null() {
                                                                crate::src::src::btree::sqlite3BtreeSetAutoVacuum(
                                                                    pMain,
                                                                    crate::src::src::btree::sqlite3BtreeGetAutoVacuum(
                                                                        pTemp,
                                                                    ),
                                                                );
                                                            }
                                                            if pOut.is_null() {
                                                                nRes =
                                                                    crate::src::src::btree::sqlite3BtreeGetRequestedReserve(
                                                                        pTemp,
                                                                    );
                                                                rc = crate::src::src::btree::sqlite3BtreeSetPageSize(
                                                                    pMain,
                                                                    crate::src::src::btree::sqlite3BtreeGetPageSize(pTemp),
                                                                    nRes,
                                                                    1 as ::core::ffi::c_int,
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    __db_ref.init.iDb = 0 as crate::src::ext::rtree::rtree::u8_0;
    __db_ref.mDbFlags = saved_mDbFlags;
    __db_ref.flags = saved_flags;
    __db_ref.nChange = saved_nChange;
    __db_ref.nTotalChange = saved_nTotalChange;
    __db_ref.mTrace = saved_mTrace;
    crate::src::src::btree::sqlite3BtreeSetPageSize(
        pMain,
        -(1 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    __db_ref.autoCommit = 1 as crate::src::ext::rtree::rtree::u8_0;
    if !pDb.is_null() {
        let __pDb_ref = unsafe { &mut *pDb };
        crate::src::src::btree::sqlite3BtreeClose(__pDb_ref.pBt);
        __pDb_ref.pBt = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::Btree>();
        __pDb_ref.pSchema = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Schema>();
    }
    crate::src::src::build::sqlite3ResetAllSchemasOfConnection(db as *mut crate::src::headers::sqliteInt_h::sqlite3);
    rc
}
