










pub use crate::src::src::hash::Hash;pub use crate::src::src::hash::HashElem;pub use crate::src::src::hash::_ht;
pub use crate::src::src::pager::Pgno;

pub use crate::src::headers::sqlite3_h::sqlite3_callback;pub use crate::src::src::vdbeapi::sqlite3_column_count;pub use crate::src::src::vdbeapi::sqlite3_column_name;pub use crate::src::src::vdbeapi::sqlite3_column_text;pub use crate::src::src::vdbeapi::sqlite3_column_type;pub use crate::src::headers::vdbeInt_h::sqlite3_context;pub use crate::src::src::main::sqlite3_errmsg;pub use crate::src::headers::sqlite3_h::sqlite3_file;pub use crate::src::headers::sqlite3_h::sqlite3_filename;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;pub use crate::src::headers::sqlite3_h::sqlite3_index_info;pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;pub use crate::src::headers::sqlite3_h::sqlite3_int64;pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;pub use crate::src::headers::sqlite3_h::sqlite3_module;pub use crate::src::src::mutex_unix::sqlite3_mutex;pub use crate::src::src::mutex::sqlite3_mutex_enter;pub use crate::src::src::mutex::sqlite3_mutex_leave;pub use crate::src::src::prepare::sqlite3_prepare_v2;pub use crate::src::src::vdbeapi::sqlite3_step;pub use crate::src::headers::sqlite3_h::sqlite3_stmt;pub use crate::src::headers::sqlite3_h::sqlite3_syscall_ptr;pub use crate::src::headers::sqlite3_h::sqlite3_uint64;pub use crate::src::headers::vdbeInt_h::sqlite3_value;pub use crate::src::headers::sqlite3_h::sqlite3_vfs;pub use crate::src::headers::sqlite3_h::sqlite3_vtab;pub use crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;pub use crate::src::headers::sqlite3_h::sqlite_int64;pub use crate::src::headers::sqlite3_h::sqlite_uint64;pub use crate::src::headers::sqlite3_h::SQLITE_ABORT;pub use crate::src::headers::sqlite3_h::SQLITE_DONE;pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;pub use crate::src::headers::sqlite3_h::SQLITE_NULL;pub use crate::src::headers::sqlite3_h::SQLITE_OK;pub use crate::src::headers::sqlite3_h::SQLITE_ROW;pub use crate::src::headers::sqliteInt_h::__anon_struct_0;pub use crate::src::headers::sqliteInt_h::__anon_struct_1;pub use crate::src::headers::sqliteInt_h::__anon_struct_2;pub use crate::src::headers::sqliteInt_h::__anon_struct_3;pub use crate::src::headers::sqliteInt_h::__anon_struct_4;pub use crate::src::headers::sqliteInt_h::__anon_struct_5;pub use crate::src::headers::sqliteInt_h::__anon_struct_6;pub use crate::src::headers::sqliteInt_h::__anon_struct_7;pub use crate::src::headers::sqliteInt_h::__anon_struct_8;pub use crate::src::headers::sqliteInt_h::__anon_union_0;pub use crate::src::headers::sqliteInt_h::__anon_union_1;pub use crate::src::headers::sqliteInt_h::__anon_union_10;pub use crate::src::headers::sqliteInt_h::__anon_union_11;pub use crate::src::headers::sqliteInt_h::__anon_union_12;pub use crate::src::headers::sqliteInt_h::__anon_union_13;pub use crate::src::headers::sqliteInt_h::__anon_union_15;pub use crate::src::headers::sqliteInt_h::__anon_union_2;pub use crate::src::headers::sqliteInt_h::__anon_union_3;pub use crate::src::headers::sqliteInt_h::__anon_union_5;pub use crate::src::headers::sqliteInt_h::__anon_union_6;pub use crate::src::headers::sqliteInt_h::__anon_union_7;pub use crate::src::headers::sqliteInt_h::__anon_union_8;pub use crate::src::headers::sqliteInt_h::__anon_union_9;pub use crate::src::headers::sqliteInt_h::bft;pub use crate::src::fts5::i16_0;pub use crate::src::ext::rtree::rtree::i64_0;pub use crate::src::headers::sqliteInt_h::sColMap;pub use crate::src::headers::sqliteInt_h::sqlite3;pub use crate::src::src::malloc::sqlite3ApiExit;pub use crate::src::src::global::sqlite3CtypeMap;pub use crate::src::src::malloc::sqlite3DbFree;pub use crate::src::src::malloc::sqlite3DbMallocRaw;pub use crate::src::src::malloc::sqlite3DbStrDup;pub use crate::src::src::util::sqlite3Error;pub use crate::src::headers::sqliteInt_h::sqlite3InitInfo;pub use crate::src::src::main::sqlite3MisuseError;pub use crate::src::src::malloc::sqlite3OomFault;pub use crate::src::src::util::sqlite3SafetyCheckOk;pub use crate::src::headers::sqliteInt_h::sqlite3_xauth;pub use crate::src::fts5::u16_0;pub use crate::src::ext::rtree::rtree::u32_0;pub use crate::src::ext::rtree::rtree::u64_0;pub use crate::src::ext::rtree::rtree::u8_0;pub use crate::src::headers::sqliteInt_h::yDbMask;pub use crate::src::headers::sqliteInt_h::ynVar;pub use crate::src::headers::sqliteInt_h::AggInfo;pub use crate::src::headers::sqliteInt_h::AggInfo_col;pub use crate::src::headers::sqliteInt_h::AggInfo_func;pub use crate::src::headers::sqliteInt_h::AutoincInfo;pub use crate::src::headers::sqliteInt_h::Bitmask;pub use crate::src::headers::sqliteInt_h::BusyHandler;pub use crate::src::headers::sqliteInt_h::CollSeq;pub use crate::src::headers::sqliteInt_h::Column;pub use crate::src::headers::sqliteInt_h::Cte;pub use crate::src::headers::sqliteInt_h::CteUse;pub use crate::src::headers::sqliteInt_h::Db;pub use crate::src::headers::sqliteInt_h::DbClientData;pub use crate::src::headers::sqliteInt_h::Expr;pub use crate::src::headers::sqliteInt_h::ExprList;pub use crate::src::headers::sqliteInt_h::ExprList_item;pub use crate::src::headers::sqliteInt_h::FKey;pub use crate::src::headers::sqliteInt_h::FuncDef;pub use crate::src::headers::sqliteInt_h::FuncDestructor;pub use crate::src::headers::sqliteInt_h::IdList;pub use crate::src::headers::sqliteInt_h::IdList_item;pub use crate::src::headers::sqliteInt_h::Index;pub use crate::src::headers::sqliteInt_h::IndexedExpr;pub use crate::src::headers::sqliteInt_h::KeyInfo;pub use crate::src::headers::sqliteInt_h::LogEst;pub use crate::src::headers::sqliteInt_h::Lookaside;pub use crate::src::headers::sqliteInt_h::LookasideSlot;pub use crate::src::headers::sqliteInt_h::Module;pub use crate::src::headers::sqliteInt_h::Parse;pub use crate::src::headers::sqliteInt_h::ParseCleanup;pub use crate::src::headers::vdbeInt_h::PreUpdate;pub use crate::src::headers::sqliteInt_h::RenameToken;pub use crate::src::headers::sqliteInt_h::Returning;pub use crate::src::headers::sqliteInt_h::SQLITE_NullCallback;pub use crate::src::headers::sqliteInt_h::Savepoint;pub use crate::src::headers::sqliteInt_h::Schema;pub use crate::src::headers::sqliteInt_h::Select;pub use crate::src::headers::sqliteInt_h::SrcItem;pub use crate::src::headers::sqliteInt_h::SrcList;pub use crate::src::headers::sqliteInt_h::Subquery;pub use crate::src::headers::sqliteInt_h::Table;pub use crate::src::headers::sqliteInt_h::TableLock;pub use crate::src::headers::sqliteInt_h::Token;pub use crate::src::headers::sqliteInt_h::Trigger;pub use crate::src::headers::sqliteInt_h::TriggerPrg;pub use crate::src::headers::sqliteInt_h::TriggerStep;pub use crate::src::headers::sqliteInt_h::Upsert;pub use crate::src::headers::sqliteInt_h::VList;pub use crate::src::headers::sqliteInt_h::VTable;pub use crate::src::headers::sqliteInt_h::VtabCtx;pub use crate::src::headers::sqliteInt_h::Window;pub use crate::src::headers::sqliteInt_h::With;pub use crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
pub use crate::src::headers::stdlib::int16_t;


pub use crate::src::headers::stdlib::uint16_t;pub use crate::src::headers::stdlib::uint32_t;pub use crate::src::headers::stdlib::uint8_t;pub use crate::src::headers::stdlib::__int16_t;pub use crate::src::headers::stdlib::__uint16_t;pub use crate::src::headers::stdlib::__uint32_t;pub use crate::src::headers::stdlib::__uint8_t;pub use crate::src::src::vdbe::p4union;pub use crate::src::src::vdbeaux::sqlite3VdbeFinalize;pub use crate::src::src::vdbe::Mem;pub use crate::src::src::vdbe::SubProgram;pub use crate::src::src::vdbe::SubrtnSig;pub use crate::src::headers::vdbeInt_h::Vdbe;pub use crate::src::src::vdbe::VdbeOp;
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3_exec(
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut zSql: *const ::core::ffi::c_char,
    mut xCallback: crate::src::headers::sqlite3_h::sqlite3_callback,
    mut pArg: *mut ::core::ffi::c_void,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut zLeftover: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut azCols: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut callbackIsInit: ::core::ffi::c_int = 0;
    if crate::src::src::util::sqlite3SafetyCheckOk(db as *mut crate::src::headers::sqliteInt_h::sqlite3) == 0 {
        return crate::src::src::main::sqlite3MisuseError(43 as ::core::ffi::c_int);
    }
    if zSql.is_null() {
        zSql = b"\0" as *const u8 as *const ::core::ffi::c_char;
    }
    crate::src::src::mutex::sqlite3_mutex_enter((*db).mutex);
    crate::src::src::util::sqlite3Error(db as *mut crate::src::headers::sqliteInt_h::sqlite3, crate::src::headers::sqlite3_h::SQLITE_OK);
    's_32: while rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && *zSql.offset(0 as isize) as ::core::ffi::c_int != 0
    {
        let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut azVals: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        pStmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
        rc = crate::src::src::prepare::sqlite3_prepare_v2(
            
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            zSql,
            -(1 as ::core::ffi::c_int),
            &raw mut pStmt,
            &raw mut zLeftover,
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            continue;
        }
        if pStmt.is_null() {
            zSql = zLeftover;
        } else {
            callbackIsInit = 0 as ::core::ffi::c_int;
            loop {
                let mut i: ::core::ffi::c_int = 0;
                rc = crate::src::src::vdbeapi::sqlite3_step(pStmt);
                if xCallback.is_some()
                    && (crate::src::headers::sqlite3_h::SQLITE_ROW == rc
                        || crate::src::headers::sqlite3_h::SQLITE_DONE == rc
                            && callbackIsInit == 0
                            && (*db).flags & crate::src::headers::sqliteInt_h::SQLITE_NullCallback as crate::src::ext::rtree::rtree::u64_0 != 0)
                {
                    if callbackIsInit == 0 {
                        nCol = crate::src::src::vdbeapi::sqlite3_column_count(pStmt);
                        azCols = crate::src::src::malloc::sqlite3DbMallocRaw(
                            
                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                            ((2 as ::core::ffi::c_int * nCol + 1 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*const ::core::ffi::c_char>() as usize
                                ) as crate::src::ext::rtree::rtree::u64_0,
                        ) as *mut *mut ::core::ffi::c_char;
                        if azCols.is_null() {
                            break 's_32;
                        }
                        i = 0 as ::core::ffi::c_int;
                        while i < nCol {
                            let ref mut fresh0 = *azCols.offset(i as isize);
                            *fresh0 = crate::src::src::vdbeapi::sqlite3_column_name(pStmt, i) as *mut ::core::ffi::c_char;
                            i += 1;
                        }
                        callbackIsInit = 1 as ::core::ffi::c_int;
                    }
                    if rc == crate::src::headers::sqlite3_h::SQLITE_ROW {
                        azVals = azCols.offset(nCol as isize) as *mut *mut ::core::ffi::c_char;
                        i = 0 as ::core::ffi::c_int;
                        while i < nCol {
                            let ref mut fresh1 = *azVals.offset(i as isize);
                            *fresh1 = crate::src::src::vdbeapi::sqlite3_column_text(pStmt, i) as *mut ::core::ffi::c_char;
                            if (*azVals.offset(i as isize)).is_null()
                                && crate::src::src::vdbeapi::sqlite3_column_type(pStmt, i) != crate::src::headers::sqlite3_h::SQLITE_NULL
                            {
                                crate::src::src::malloc::sqlite3OomFault(db as *mut crate::src::headers::sqliteInt_h::sqlite3);
                                break 's_32;
                            } else {
                                i += 1;
                            }
                        }
                        let ref mut fresh2 = *azVals.offset(i as isize);
                        *fresh2 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    if xCallback.expect("non-null function pointer")(pArg, nCol, azVals, azCols)
                        != 0
                    {
                        rc = crate::src::headers::sqlite3_h::SQLITE_ABORT;
                        crate::src::src::vdbeaux::sqlite3VdbeFinalize(pStmt as *mut crate::src::headers::vdbeInt_h::Vdbe);
                        pStmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
                        crate::src::src::util::sqlite3Error(db as *mut crate::src::headers::sqliteInt_h::sqlite3, crate::src::headers::sqlite3_h::SQLITE_ABORT);
                        break 's_32;
                    }
                }
                if !(rc != crate::src::headers::sqlite3_h::SQLITE_ROW) {
                    continue;
                }
                rc = crate::src::src::vdbeaux::sqlite3VdbeFinalize(pStmt as *mut crate::src::headers::vdbeInt_h::Vdbe);
                pStmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
                zSql = zLeftover;
                while *(&raw const crate::src::src::global::sqlite3CtypeMap as *const ::core::ffi::c_uchar).offset(
                    *zSql.offset(0 as isize) as ::core::ffi::c_uchar as isize,
                ) as ::core::ffi::c_int
                    & 0x1 as ::core::ffi::c_int
                    != 0
                {
                    zSql = zSql.offset(1);
                }
                break;
            }
            crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, azCols as *mut ::core::ffi::c_void);
            azCols = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        }
    }
    if !pStmt.is_null() {
        crate::src::src::vdbeaux::sqlite3VdbeFinalize(pStmt as *mut crate::src::headers::vdbeInt_h::Vdbe);
    }
    crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, azCols as *mut ::core::ffi::c_void);
    rc = crate::src::src::malloc::sqlite3ApiExit(db as *mut crate::src::headers::sqliteInt_h::sqlite3, rc);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK && !pzErrMsg.is_null() {
        *pzErrMsg = crate::src::src::malloc::sqlite3DbStrDup(::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>() as
    *mut crate::src::headers::sqliteInt_h::sqlite3, crate::src::src::main::sqlite3_errmsg(db as *mut crate::src::headers::sqliteInt_h::sqlite3));
        if (*pzErrMsg).is_null() {
            rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
            crate::src::src::util::sqlite3Error(db as *mut crate::src::headers::sqliteInt_h::sqlite3, crate::src::headers::sqlite3_h::SQLITE_NOMEM);
        }
    } else if !pzErrMsg.is_null() {
        *pzErrMsg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    crate::src::src::mutex::sqlite3_mutex_leave((*db).mutex);
    rc
}
