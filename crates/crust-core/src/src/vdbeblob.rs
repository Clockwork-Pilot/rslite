













pub use crate::src::src::hash::Hash;pub use crate::src::src::hash::HashElem;pub use crate::src::src::hash::_ht;pub use crate::src::headers::opcodes_h::OP_Column;pub use crate::src::headers::opcodes_h::OP_Halt;pub use crate::src::headers::opcodes_h::OP_NotExists;pub use crate::src::headers::opcodes_h::OP_OpenRead;pub use crate::src::headers::opcodes_h::OP_OpenWrite;pub use crate::src::headers::opcodes_h::OP_ResultRow;pub use crate::src::headers::opcodes_h::OP_TableLock;pub use crate::src::headers::opcodes_h::OP_Transaction;
pub use crate::src::src::pager::Pgno;






pub use crate::src::headers::sqlite3_h::sqlite3_blob;pub use crate::src::src::main::sqlite3_errmsg;pub use crate::src::headers::sqlite3_h::sqlite3_file;pub use crate::src::headers::sqlite3_h::sqlite3_filename;pub use crate::src::src::vdbeapi::sqlite3_finalize;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;pub use crate::src::headers::sqlite3_h::sqlite3_index_info;pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;pub use crate::src::headers::sqlite3_h::sqlite3_int64;pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;pub use crate::src::headers::sqlite3_h::sqlite3_module;pub use crate::src::src::mutex_unix::sqlite3_mutex;pub use crate::src::src::mutex::sqlite3_mutex_enter;pub use crate::src::src::mutex::sqlite3_mutex_leave;pub use crate::src::src::vdbeapi::sqlite3_step;pub use crate::src::headers::sqlite3_h::sqlite3_stmt;pub use crate::src::headers::sqlite3_h::sqlite3_syscall_ptr;pub use crate::src::headers::sqlite3_h::sqlite3_uint64;pub use crate::src::headers::sqlite3_h::sqlite3_vfs;pub use crate::src::headers::sqlite3_h::sqlite3_vtab;pub use crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;pub use crate::src::headers::sqlite3_h::sqlite_int64;pub use crate::src::headers::sqlite3_h::sqlite_uint64;pub use crate::src::headers::sqlite3_h::SQLITE_ABORT;pub use crate::src::headers::sqlite3_h::SQLITE_DELETE;pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;pub use crate::src::headers::sqlite3_h::SQLITE_OK;pub use crate::src::headers::sqlite3_h::SQLITE_ROW;pub use crate::src::headers::sqlite3_h::SQLITE_SCHEMA;pub use crate::src::headers::sqliteInt_h::__anon_struct_0;pub use crate::src::headers::sqliteInt_h::__anon_struct_1;pub use crate::src::headers::sqliteInt_h::__anon_struct_2;pub use crate::src::headers::sqliteInt_h::__anon_struct_3;pub use crate::src::headers::sqliteInt_h::__anon_struct_4;pub use crate::src::headers::sqliteInt_h::__anon_struct_5;pub use crate::src::headers::sqliteInt_h::__anon_struct_6;pub use crate::src::headers::sqliteInt_h::__anon_struct_7;pub use crate::src::headers::sqliteInt_h::__anon_struct_8;pub use crate::src::headers::sqliteInt_h::__anon_union_0;pub use crate::src::headers::sqliteInt_h::__anon_union_1;pub use crate::src::headers::sqliteInt_h::__anon_union_10;pub use crate::src::headers::sqliteInt_h::__anon_union_11;pub use crate::src::headers::sqliteInt_h::__anon_union_12;pub use crate::src::headers::sqliteInt_h::__anon_union_13;pub use crate::src::headers::sqliteInt_h::__anon_union_15;pub use crate::src::headers::sqliteInt_h::__anon_union_2;pub use crate::src::headers::sqliteInt_h::__anon_union_3;pub use crate::src::headers::sqliteInt_h::__anon_union_4;pub use crate::src::headers::sqliteInt_h::__anon_union_5;pub use crate::src::headers::sqliteInt_h::__anon_union_6;pub use crate::src::headers::sqliteInt_h::__anon_union_7;pub use crate::src::headers::sqliteInt_h::__anon_union_8;pub use crate::src::headers::sqliteInt_h::__anon_union_9;pub use crate::src::headers::sqliteInt_h::bft;pub use crate::src::fts5::i16_0;pub use crate::src::ext::rtree::rtree::i64_0;pub use crate::src::headers::sqliteInt_h::i8_0;pub use crate::src::headers::sqliteInt_h::sColMap;pub use crate::src::headers::sqliteInt_h::sqlite3;pub use crate::src::src::malloc::sqlite3ApiExit;pub use crate::src::src::select::sqlite3ColumnIndex;pub use crate::src::src::malloc::sqlite3DbFree;pub use crate::src::src::malloc::sqlite3DbMallocZero;pub use crate::src::src::util::sqlite3Error;pub use crate::src::headers::sqliteInt_h::sqlite3InitInfo;pub use crate::src::src::build::sqlite3LocateTable;pub use crate::src::src::main::sqlite3MisuseError;pub use crate::src::src::build::sqlite3OpenTempDatabase;pub use crate::src::src::prepare::sqlite3ParseObjectInit;pub use crate::src::src::prepare::sqlite3ParseObjectReset;pub use crate::src::src::prepare::sqlite3SchemaToIndex;pub use crate::src::headers::sqliteInt_h::sqlite3_xauth;pub use crate::src::fts5::u16_0;pub use crate::src::ext::rtree::rtree::u32_0;pub use crate::src::ext::rtree::rtree::u64_0;pub use crate::src::ext::rtree::rtree::u8_0;pub use crate::src::headers::sqliteInt_h::yDbMask;pub use crate::src::headers::sqliteInt_h::ynVar;pub use crate::src::headers::sqliteInt_h::AggInfo;pub use crate::src::headers::sqliteInt_h::AggInfo_col;pub use crate::src::headers::sqliteInt_h::AggInfo_func;pub use crate::src::headers::sqliteInt_h::AutoincInfo;pub use crate::src::headers::sqliteInt_h::Bitmask;pub use crate::src::headers::sqliteInt_h::BusyHandler;pub use crate::src::headers::sqliteInt_h::CollSeq;pub use crate::src::headers::sqliteInt_h::Column;pub use crate::src::headers::sqliteInt_h::Cte;pub use crate::src::headers::sqliteInt_h::CteUse;pub use crate::src::headers::sqliteInt_h::Db;pub use crate::src::headers::sqliteInt_h::DbClientData;pub use crate::src::headers::sqliteInt_h::Expr;pub use crate::src::headers::sqliteInt_h::ExprList;pub use crate::src::headers::sqliteInt_h::ExprList_item;pub use crate::src::headers::sqliteInt_h::FKey;pub use crate::src::headers::sqliteInt_h::FuncDef;pub use crate::src::headers::sqliteInt_h::FuncDestructor;pub use crate::src::headers::sqliteInt_h::IdList;pub use crate::src::headers::sqliteInt_h::IdList_item;pub use crate::src::headers::sqliteInt_h::Index;pub use crate::src::headers::sqliteInt_h::IndexedExpr;pub use crate::src::headers::sqliteInt_h::KeyInfo;pub use crate::src::headers::sqliteInt_h::LogEst;pub use crate::src::headers::sqliteInt_h::Lookaside;pub use crate::src::headers::sqliteInt_h::LookasideSlot;pub use crate::src::headers::sqliteInt_h::Module;pub use crate::src::headers::sqliteInt_h::Parse;pub use crate::src::headers::sqliteInt_h::ParseCleanup;pub use crate::src::headers::sqliteInt_h::RenameToken;pub use crate::src::headers::sqliteInt_h::Returning;pub use crate::src::headers::sqliteInt_h::SQLITE_ForeignKeys;pub use crate::src::headers::sqliteInt_h::Savepoint;pub use crate::src::headers::sqliteInt_h::Schema;pub use crate::src::headers::sqliteInt_h::Select;pub use crate::src::headers::sqliteInt_h::SrcItem;pub use crate::src::headers::sqliteInt_h::SrcList;pub use crate::src::headers::sqliteInt_h::Subquery;pub use crate::src::headers::sqliteInt_h::TF_HasGenerated;pub use crate::src::headers::sqliteInt_h::TF_WithoutRowid;pub use crate::src::headers::sqliteInt_h::Table;pub use crate::src::headers::sqliteInt_h::TableLock;pub use crate::src::headers::sqliteInt_h::Token;pub use crate::src::headers::sqliteInt_h::Trigger;pub use crate::src::headers::sqliteInt_h::TriggerPrg;pub use crate::src::headers::sqliteInt_h::TriggerStep;pub use crate::src::headers::sqliteInt_h::UnpackedRecord;pub use crate::src::headers::sqliteInt_h::Upsert;pub use crate::src::headers::sqliteInt_h::VList;pub use crate::src::headers::sqliteInt_h::VTable;pub use crate::src::headers::sqliteInt_h::VtabCtx;pub use crate::src::headers::sqliteInt_h::Window;pub use crate::src::headers::sqliteInt_h::With;pub use crate::src::headers::sqliteInt_h::TABTYP_VIEW;pub use crate::src::headers::sqliteInt_h::TABTYP_VTAB;pub use crate::src::headers::sqliteInt_h::XN_EXPR;pub use crate::src::headers::stdlib::int16_t;pub use crate::src::headers::stdlib::int8_t;pub use crate::src::headers::stdlib::uint16_t;pub use crate::src::headers::stdlib::uint32_t;pub use crate::src::headers::stdlib::uint8_t;pub use crate::src::headers::stdlib::__int16_t;pub use crate::src::headers::stdlib::__int8_t;pub use crate::src::headers::stdlib::__uint16_t;pub use crate::src::headers::stdlib::__uint32_t;pub use crate::src::headers::stdlib::__uint8_t;pub use crate::src::headers::vdbeInt_h::sqlite3_context;pub use crate::src::headers::vdbeInt_h::sqlite3_value;pub use crate::src::headers::vdbeInt_h::AuxData;pub use crate::src::headers::vdbeInt_h::Bool;pub use crate::src::headers::vdbeInt_h::MemValue;pub use crate::src::headers::vdbeInt_h::Op;pub use crate::src::headers::vdbeInt_h::PreUpdate;pub use crate::src::headers::vdbeInt_h::Vdbe;pub use crate::src::headers::vdbeInt_h::VdbeCursor;pub use crate::src::headers::vdbeInt_h::VdbeFrame;pub use crate::src::headers::vdbeInt_h::VdbeSorter;pub use crate::src::headers::vdbeInt_h::VdbeTxtBlbCache;pub use crate::src::headers::vdbeInt_h::__anon_struct_10;pub use crate::src::headers::vdbeInt_h::__anon_union_17;pub use crate::src::headers::vdbeInt_h::__anon_union_18;pub use crate::src::src::vdbe::sqlite3VdbeExec;pub use crate::src::src::vdbemem::sqlite3VdbeMemSetInt64;pub use crate::src::src::vdbeaux::sqlite3VdbePreUpdateHook;pub use crate::src::src::vdbeaux::sqlite3VdbeSerialTypeLen;pub use crate::src::headers::vdbeInt_h::SQLITE_MAX_SCHEMA_RETRY;pub use crate::src::src::vdbe::p4union;pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int;pub use crate::src::src::vdbeaux::sqlite3VdbeAddOpList;pub use crate::src::src::vdbeaux::sqlite3VdbeChangeP4;pub use crate::src::src::vdbeaux::sqlite3VdbeChangeP5;pub use crate::src::src::vdbeaux::sqlite3VdbeCreate;pub use crate::src::src::vdbeaux::sqlite3VdbeFinalize;pub use crate::src::src::vdbeaux::sqlite3VdbeMakeReady;pub use crate::src::src::vdbeaux::sqlite3VdbeUsesBtree;pub use crate::src::src::vdbe::Mem;pub use crate::src::src::vdbe::SubProgram;pub use crate::src::src::vdbe::SubrtnSig;pub use crate::src::src::vdbe::VdbeOp;pub use crate::src::src::vdbe::VdbeOpList;pub use crate::src::src::vdbe::P4_INT32;pub use crate::src::src::vdbe::P4_TRANSIENT;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct Incrblob {
    pub nByte: ::core::ffi::c_int,
    pub iOffset: ::core::ffi::c_int,
    pub iCol: crate::src::fts5::u16_0,
    pub pCsr: *mut crate::src::headers::btreeInt_h::BtCursor,
    pub pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt,
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub zDb: *mut ::core::ffi::c_char,
    pub pTab: *mut crate::src::headers::sqliteInt_h::Table,
}

unsafe extern "C" fn blobSeekToRow(
    mut p: *mut Incrblob,
    mut iRow: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*p).pStmt as *mut crate::src::headers::vdbeInt_h::Vdbe;
    crate::src::src::vdbemem::sqlite3VdbeMemSetInt64(
        
        (*v).aMem.offset(1 as isize) as *mut crate::src::src::vdbe::Mem as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        iRow as crate::src::ext::rtree::rtree::i64_0,
    );
    if (*v).pc > 4 as ::core::ffi::c_int {
        (*v).pc = 4 as ::core::ffi::c_int;
        rc = crate::src::src::vdbe::sqlite3VdbeExec(v as *mut crate::src::headers::vdbeInt_h::Vdbe);
    } else {
        rc = crate::src::src::vdbeapi::sqlite3_step((*p).pStmt);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_ROW {
        let mut pC: *mut crate::src::headers::vdbeInt_h::VdbeCursor = *(*v).apCsr.offset(0 as isize);
        let mut type_0: crate::src::ext::rtree::rtree::u32_0 = 0;
        type_0 = if (*pC).nHdrParsed as ::core::ffi::c_int > (*p).iCol as ::core::ffi::c_int {
            *(&raw mut (*pC).aType as *mut crate::src::ext::rtree::rtree::u32_0).offset((*p).iCol as isize)
        } else {
            0 as crate::src::ext::rtree::rtree::u32_0
        };
        if type_0 < 12 as crate::src::ext::rtree::rtree::u32_0 {
            let __p_ref = unsafe { &mut *p };
            zErr = crate::src::printf_c_variadic::sqlite3MPrintf_args(
                __p_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                b"cannot open value of type %s\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Str(
                    if type_0 == 0 as crate::src::ext::rtree::rtree::u32_0 {
                        b"null\0" as *const u8 as *const ::core::ffi::c_char
                    } else if type_0 == 7 as crate::src::ext::rtree::rtree::u32_0 {
                        b"real\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"integer\0" as *const u8 as *const ::core::ffi::c_char
                    } as *mut ::core::ffi::c_char,
                )],
            );
            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
            crate::src::src::vdbeapi::sqlite3_finalize(__p_ref.pStmt);
            __p_ref.pStmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
        } else {
            let __pC_ref = unsafe { &mut *pC };
            let __p_ref = unsafe { &mut *p };
            __p_ref.iOffset = *(&raw mut __pC_ref.aType as *mut crate::src::ext::rtree::rtree::u32_0).offset(
                (__p_ref.iCol as ::core::ffi::c_int + __pC_ref.nField as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int;
            __p_ref.nByte = crate::src::src::vdbeaux::sqlite3VdbeSerialTypeLen(type_0) as ::core::ffi::c_int;
            __p_ref.pCsr = __pC_ref.uc.pCursor;
            crate::src::src::btree::sqlite3BtreeIncrblobCursor(__p_ref.pCsr);
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_ROW {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    } else if !(*p).pStmt.is_null() {
        rc = crate::src::src::vdbeapi::sqlite3_finalize((*p).pStmt);
        (*p).pStmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            zErr = crate::src::printf_c_variadic::sqlite3MPrintf_args(
                (*p).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                b"no such rowid: %lld\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Int(iRow)],
            );
            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
        } else {
            zErr = crate::src::printf_c_variadic::sqlite3MPrintf_args(
                (*p).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Str(
                    crate::src::src::main::sqlite3_errmsg((*p).db as *mut crate::src::headers::sqliteInt_h::sqlite3) as *mut ::core::ffi::c_char,
                )],
            );
        }
    }
    *pzErr = zErr;
    rc
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3_blob_open(
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut zDb: *const ::core::ffi::c_char,
    mut zTable: *const ::core::ffi::c_char,
    mut zColumn: *const ::core::ffi::c_char,
    mut iRow: crate::src::headers::sqlite3_h::sqlite_int64,
    mut wrFlag: ::core::ffi::c_int,
    mut ppBlob: *mut *mut crate::src::headers::sqlite3_h::sqlite3_blob,
) -> ::core::ffi::c_int {
    let mut nAttempt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iCol: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pTab: *mut crate::src::headers::sqliteInt_h::Table = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
    let mut pBlob: *mut Incrblob = ::core::ptr::null_mut::<Incrblob>();
    let mut iDb: ::core::ffi::c_int = 0;
    let mut sParse: crate::src::headers::sqliteInt_h::Parse = unsafe { ::core::mem::zeroed() };
    *ppBlob = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_blob>();
    wrFlag = (wrFlag != 0) as ::core::ffi::c_int;
    let __db_ref = unsafe { &mut *db };
    crate::src::src::mutex::sqlite3_mutex_enter(__db_ref.mutex);
    pBlob = crate::src::src::malloc::sqlite3DbMallocZero(db as *mut crate::src::headers::sqliteInt_h::sqlite3, ::core::mem::size_of::<Incrblob>() as crate::src::ext::rtree::rtree::u64_0) as *mut Incrblob;
    loop {
        crate::src::src::prepare::sqlite3ParseObjectInit(&raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse,  db as *mut crate::src::headers::sqliteInt_h::sqlite3);
        if pBlob.is_null() {
            break;
        }
        crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, zErr as *mut ::core::ffi::c_void);
        zErr = ::core::ptr::null_mut::<::core::ffi::c_char>();
        crate::src::src::btmutex::sqlite3BtreeEnterAll(db as *mut crate::src::headers::sqliteInt_h::sqlite3);
        pTab =  crate::src::src::build::sqlite3LocateTable(&raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse, 0 as crate::src::ext::rtree::rtree::u32_0, zTable, zDb) as
    *mut crate::src::headers::sqliteInt_h::Table;
        if !pTab.is_null() && (*pTab).eTabType as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::TABTYP_VTAB {
            pTab = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
            crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                &raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse,
                b"cannot open virtual table: %s\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Str(zTable as *mut ::core::ffi::c_char)],
            );
        }
        if !pTab.is_null() && !((*pTab).tabFlags & crate::src::headers::sqliteInt_h::TF_WithoutRowid as crate::src::ext::rtree::rtree::u32_0 == 0 as crate::src::ext::rtree::rtree::u32_0) {
            pTab = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
            crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                &raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse,
                b"cannot open table without rowid: %s\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Str(zTable as *mut ::core::ffi::c_char)],
            );
        }
        if !pTab.is_null() && (*pTab).tabFlags & crate::src::headers::sqliteInt_h::TF_HasGenerated as crate::src::ext::rtree::rtree::u32_0 != 0 as crate::src::ext::rtree::rtree::u32_0 {
            pTab = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
            crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                &raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse,
                b"cannot open table with generated columns: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Str(zTable as *mut ::core::ffi::c_char)],
            );
        }
        if !pTab.is_null() && (*pTab).eTabType as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::TABTYP_VIEW {
            pTab = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
            crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                &raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse,
                b"cannot open view: %s\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Str(zTable as *mut ::core::ffi::c_char)],
            );
        }
        if pTab.is_null() || {
            iDb = crate::src::src::prepare::sqlite3SchemaToIndex(db as *mut crate::src::headers::sqliteInt_h::sqlite3,  (*pTab).pSchema as *mut crate::src::headers::sqliteInt_h::Schema);
            iDb == 1 as ::core::ffi::c_int && crate::src::src::build::sqlite3OpenTempDatabase(&raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse) != 0
        } {
            if !sParse.zErrMsg.is_null() {
                crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, zErr as *mut ::core::ffi::c_void);
                zErr = sParse.zErrMsg;
                sParse.zErrMsg = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
            crate::src::src::btmutex::sqlite3BtreeLeaveAll(db as *mut crate::src::headers::sqliteInt_h::sqlite3);
            break;
        } else {
            (*pBlob).pTab = pTab;
            (*pBlob).zDb = (*__db_ref.aDb.offset(iDb as isize)).zDbSName;
            iCol = crate::src::src::select::sqlite3ColumnIndex(pTab as *mut crate::src::headers::sqliteInt_h::Table, zColumn);
            if iCol < 0 as ::core::ffi::c_int {
                crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, zErr as *mut ::core::ffi::c_void);
                zErr = crate::src::printf_c_variadic::sqlite3MPrintf_args(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    b"no such column: \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                    &[crate::src::src::printf::PrintfArg::Str(zColumn as *mut ::core::ffi::c_char)],
                );
                rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                crate::src::src::btmutex::sqlite3BtreeLeaveAll(db as *mut crate::src::headers::sqliteInt_h::sqlite3);
                break;
            } else {
                if wrFlag != 0 {
                    let mut zFault: *const ::core::ffi::c_char =
                        ::core::ptr::null::<::core::ffi::c_char>();
                    let mut pIdx: *mut crate::src::headers::sqliteInt_h::Index = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Index>();
                    if __db_ref.flags & crate::src::headers::sqliteInt_h::SQLITE_ForeignKeys as crate::src::ext::rtree::rtree::u64_0 != 0 {
                        let mut pFKey: *mut crate::src::headers::sqliteInt_h::FKey = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::FKey>();
                        pFKey = (*pTab).u.tab.pFKey;
                        while !pFKey.is_null() {
                            let mut j: ::core::ffi::c_int = 0;
                            j = 0 as ::core::ffi::c_int;
                            while j < (*pFKey).nCol {
                                if (*(&raw mut (*pFKey).aCol as *mut crate::src::headers::sqliteInt_h::sColMap).offset(j as isize))
                                    .iFrom
                                    == iCol
                                {
                                    zFault =
                                        b"foreign key\0" as *const u8 as *const ::core::ffi::c_char;
                                }
                                j += 1;
                            }
                            pFKey = (*pFKey).pNextFrom;
                        }
                    }
                    pIdx = (*pTab).pIndex;
                    while !pIdx.is_null() {
                        let mut j_0: ::core::ffi::c_int = 0;
                        j_0 = 0 as ::core::ffi::c_int;
                        while j_0 < (*pIdx).nKeyCol as ::core::ffi::c_int {
                            if *(*pIdx).aiColumn.offset(j_0 as isize) as ::core::ffi::c_int == iCol
                                || *(*pIdx).aiColumn.offset(j_0 as isize) as ::core::ffi::c_int
                                    == crate::src::headers::sqliteInt_h::XN_EXPR
                            {
                                zFault = b"indexed\0" as *const u8 as *const ::core::ffi::c_char;
                            }
                            j_0 += 1;
                        }
                        pIdx = (*pIdx).pNext;
                    }
                    if !zFault.is_null() {
                        crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, zErr as *mut ::core::ffi::c_void);
                        zErr = crate::src::printf_c_variadic::sqlite3MPrintf_args(
                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                            b"cannot open %s column for writing\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[crate::src::src::printf::PrintfArg::Str(zFault as *mut ::core::ffi::c_char)],
                        );
                        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                        crate::src::src::btmutex::sqlite3BtreeLeaveAll(db as *mut crate::src::headers::sqliteInt_h::sqlite3);
                        break;
                    }
                }
                let __pBlob_ref = unsafe { &mut *pBlob };
                __pBlob_ref.pStmt =  crate::src::src::vdbeaux::sqlite3VdbeCreate(&raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse) as *mut crate::src::headers::vdbeInt_h::Vdbe as *mut crate::src::headers::sqlite3_h::sqlite3_stmt;
                if !__pBlob_ref.pStmt.is_null() {
                    static mut iLn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    static mut openBlob: [crate::src::src::vdbe::VdbeOpList; 6] = [
                        crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::src::headers::opcodes_h::OP_TableLock as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  0 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                        crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::src::headers::opcodes_h::OP_OpenRead as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  0 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                        crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::src::headers::opcodes_h::OP_NotExists as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  5 as ::core::ffi::c_schar,
    p3:  1 as ::core::ffi::c_schar,
},
                        crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::src::headers::opcodes_h::OP_Column as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  0 as ::core::ffi::c_schar,
    p3:  1 as ::core::ffi::c_schar,
},
                        crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::src::headers::opcodes_h::OP_ResultRow as crate::src::ext::rtree::rtree::u8_0,
    p1:  1 as ::core::ffi::c_schar,
    p2:  0 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                        crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::src::headers::opcodes_h::OP_Halt as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  0 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                    ];
                    let mut v: *mut crate::src::headers::vdbeInt_h::Vdbe = __pBlob_ref.pStmt as *mut crate::src::headers::vdbeInt_h::Vdbe;
                    let mut aOp: *mut crate::src::src::vdbe::VdbeOp = ::core::ptr::null_mut::<crate::src::src::vdbe::VdbeOp>();
                    let __pSchema_ref = &*(*pTab).pSchema;
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                        
                        v as *mut crate::src::headers::vdbeInt_h::Vdbe,
                        crate::src::headers::opcodes_h::OP_Transaction,
                        iDb,
                        wrFlag,
                        __pSchema_ref.schema_cookie,
                        __pSchema_ref.iGeneration,
                    );
                    crate::src::src::vdbeaux::sqlite3VdbeChangeP5(v as *mut crate::src::headers::vdbeInt_h::Vdbe, 1 as crate::src::fts5::u16_0);
                    aOp =  crate::src::src::vdbeaux::sqlite3VdbeAddOpList(
                        
                        v as *mut crate::src::headers::vdbeInt_h::Vdbe,
                        (::core::mem::size_of::<[crate::src::src::vdbe::VdbeOpList; 6]>() as usize)
                            .wrapping_div(::core::mem::size_of::<crate::src::src::vdbe::VdbeOpList>() as usize)
                            as ::core::ffi::c_int,
                        
                        &raw const openBlob as *const crate::src::src::vdbe::VdbeOpList as *const crate::src::src::vdbe::VdbeOpList,
                        iLn,
                    ) as
    *mut crate::src::src::vdbe::VdbeOp;
                    crate::src::src::vdbeaux::sqlite3VdbeUsesBtree(v as *mut crate::src::headers::vdbeInt_h::Vdbe, iDb);
                    if __db_ref.mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        (*aOp.offset(0 as isize)).p1 = iDb;
                        (*aOp.offset(0 as isize)).p2 =
                            (*pTab).tnum as ::core::ffi::c_int;
                        (*aOp.offset(0 as isize)).p3 = wrFlag;
                        crate::src::src::vdbeaux::sqlite3VdbeChangeP4(
                            
                            v as *mut crate::src::headers::vdbeInt_h::Vdbe,
                            2 as ::core::ffi::c_int,
                            (*pTab).zName,
                            crate::src::src::vdbe::P4_TRANSIENT,
                        );
                    }
                    if __db_ref.mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        if wrFlag != 0 {
                            (*aOp.offset(1 as isize)).opcode =
                                crate::src::headers::opcodes_h::OP_OpenWrite as crate::src::ext::rtree::rtree::u8_0;
                        }
                        let __pTab_ref = unsafe { &*pTab };
                        (*aOp.offset(1 as isize)).p2 =
                            __pTab_ref.tnum as ::core::ffi::c_int;
                        (*aOp.offset(1 as isize)).p3 = iDb;
                        (*aOp.offset(1 as isize)).p4type =
                            crate::src::src::vdbe::P4_INT32 as ::core::ffi::c_schar;
                        (*aOp.offset(1 as isize)).p4.i =
                            __pTab_ref.nCol as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                        (*aOp.offset(3 as isize)).p2 =
                            __pTab_ref.nCol as ::core::ffi::c_int;
                        sParse.nVar = 0 as crate::src::headers::sqliteInt_h::ynVar;
                        sParse.nMem = 1 as ::core::ffi::c_int;
                        sParse.nTab = 1 as ::core::ffi::c_int;
                        crate::src::src::vdbeaux::sqlite3VdbeMakeReady(v as *mut crate::src::headers::vdbeInt_h::Vdbe,  &raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse);
                    }
                }
                __pBlob_ref.iCol = iCol as crate::src::fts5::u16_0;
                __pBlob_ref.db = db;
                crate::src::src::btmutex::sqlite3BtreeLeaveAll(db as *mut crate::src::headers::sqliteInt_h::sqlite3);
                if __db_ref.mallocFailed != 0 {
                    break;
                }
                rc = blobSeekToRow(pBlob, iRow as crate::src::headers::sqlite3_h::sqlite3_int64, &raw mut zErr);
                nAttempt += 1;
                if nAttempt >= crate::src::headers::vdbeInt_h::SQLITE_MAX_SCHEMA_RETRY || rc != crate::src::headers::sqlite3_h::SQLITE_SCHEMA {
                    break;
                }
                crate::src::src::prepare::sqlite3ParseObjectReset(&raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse);
            }
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && __db_ref.mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        *ppBlob = pBlob as *mut crate::src::headers::sqlite3_h::sqlite3_blob;
    } else {
        if !pBlob.is_null() && !(*pBlob).pStmt.is_null() {
            crate::src::src::vdbeaux::sqlite3VdbeFinalize((*pBlob).pStmt as *mut crate::src::headers::vdbeInt_h::Vdbe as *mut crate::src::headers::vdbeInt_h::Vdbe);
        }
        crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, pBlob as *mut ::core::ffi::c_void);
    }
    crate::src::printf_c_variadic::sqlite3ErrorWithMsg_args(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        rc,
        if !zErr.is_null() {
            b"%s\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>() as *const ::core::ffi::c_char
        },
        &[crate::src::src::printf::PrintfArg::Str(zErr)],
    );
    crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, zErr as *mut ::core::ffi::c_void);
    crate::src::src::prepare::sqlite3ParseObjectReset(&raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse);
    rc = crate::src::src::malloc::sqlite3ApiExit(db as *mut crate::src::headers::sqliteInt_h::sqlite3, rc);
    crate::src::src::mutex::sqlite3_mutex_leave(__db_ref.mutex);
    rc
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3_blob_close(mut pBlob: *mut crate::src::headers::sqlite3_h::sqlite3_blob) -> ::core::ffi::c_int {
    let mut p: *mut Incrblob = pBlob as *mut Incrblob;
    let mut rc: ::core::ffi::c_int = 0;
    let mut db: *mut crate::src::headers::sqliteInt_h::sqlite3 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>();
    if !p.is_null() {
        let mut pStmt: *mut crate::src::headers::sqlite3_h::sqlite3_stmt = (*p).pStmt;
        db = (*p).db;
        crate::src::src::mutex::sqlite3_mutex_enter((*db).mutex);
        crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, p as *mut ::core::ffi::c_void);
        crate::src::src::mutex::sqlite3_mutex_leave((*db).mutex);
        rc = crate::src::src::vdbeapi::sqlite3_finalize(pStmt);
    } else {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    rc
}

unsafe extern "C" fn blobReadWrite(
    mut pBlob: *mut crate::src::headers::sqlite3_h::sqlite3_blob,
    mut z: *mut ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
    mut iOffset: ::core::ffi::c_int,
    mut xCall: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::btreeInt_h::BtCursor,
            crate::src::ext::rtree::rtree::u32_0,
            crate::src::ext::rtree::rtree::u32_0,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut p: *mut Incrblob = pBlob as *mut Incrblob;
    let mut v: *mut crate::src::headers::vdbeInt_h::Vdbe = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::Vdbe>();
    let mut db: *mut crate::src::headers::sqliteInt_h::sqlite3 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>();
    if p.is_null() {
        return crate::src::src::main::sqlite3MisuseError(393 as ::core::ffi::c_int);
    }
    let __p_ref = unsafe { &mut *p };
    db = __p_ref.db;
    crate::src::src::mutex::sqlite3_mutex_enter((*db).mutex);
    v = __p_ref.pStmt as *mut crate::src::headers::vdbeInt_h::Vdbe;
    if n < 0 as ::core::ffi::c_int
        || iOffset < 0 as ::core::ffi::c_int
        || iOffset as crate::src::headers::sqlite3_h::sqlite3_int64 + n as crate::src::headers::sqlite3_h::sqlite3_int64 > __p_ref.nByte as crate::src::headers::sqlite3_h::sqlite3_int64
    {
        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    } else if v.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_ABORT;
    } else {
        crate::src::src::btmutex::sqlite3BtreeEnterCursor(__p_ref.pCsr);
        if xCall
            == Some(
                crate::src::src::btree::sqlite3BtreePutData
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::btreeInt_h::BtCursor,
                        crate::src::ext::rtree::rtree::u32_0,
                        crate::src::ext::rtree::rtree::u32_0,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            )
            && (*db).xPreUpdateCallback.is_some()
        {
            if crate::src::src::btree::sqlite3BtreeCursorIsValidNN(__p_ref.pCsr) == 0 as ::core::ffi::c_int {
                let mut bDiff: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                rc = crate::src::src::btree::sqlite3BtreeCursorRestore(__p_ref.pCsr, &raw mut bDiff);
            }
            if crate::src::src::btree::sqlite3BtreeCursorIsValidNN(__p_ref.pCsr) != 0 {
                let mut iKey: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
                iKey = crate::src::src::btree::sqlite3BtreeIntegerKey(__p_ref.pCsr) as crate::src::headers::sqlite3_h::sqlite3_int64;
                crate::src::src::vdbeaux::sqlite3VdbePreUpdateHook(
                    
                    v as *mut crate::src::headers::vdbeInt_h::Vdbe,
                    
                    *(*v).apCsr.offset(0 as isize) as
    *mut crate::src::headers::vdbeInt_h::VdbeCursor,
                    crate::src::headers::sqlite3_h::SQLITE_DELETE,
                    __p_ref.zDb,
                    
                    __p_ref.pTab as *mut crate::src::headers::sqliteInt_h::Table,
                    iKey as crate::src::ext::rtree::rtree::i64_0,
                    -(1 as ::core::ffi::c_int),
                    __p_ref.iCol as ::core::ffi::c_int,
                );
            }
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = xCall.expect("non-null function pointer")(
                __p_ref.pCsr,
                (iOffset + __p_ref.iOffset) as crate::src::ext::rtree::rtree::u32_0,
                n as crate::src::ext::rtree::rtree::u32_0,
                z,
            );
        }
        crate::src::src::btmutex::sqlite3BtreeLeaveCursor(__p_ref.pCsr);
        if rc == crate::src::headers::sqlite3_h::SQLITE_ABORT {
            crate::src::src::vdbeaux::sqlite3VdbeFinalize(v as *mut crate::src::headers::vdbeInt_h::Vdbe);
            __p_ref.pStmt = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
        } else {
            (*v).rc = rc;
        }
    }
    crate::src::src::util::sqlite3Error(db as *mut crate::src::headers::sqliteInt_h::sqlite3, rc);
    rc = crate::src::src::malloc::sqlite3ApiExit(db as *mut crate::src::headers::sqliteInt_h::sqlite3, rc);
    crate::src::src::mutex::sqlite3_mutex_leave((*db).mutex);
    rc
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3_blob_read(
    mut pBlob: *mut crate::src::headers::sqlite3_h::sqlite3_blob,
    mut z: *mut ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
    mut iOffset: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    blobReadWrite(
        pBlob,
        z,
        n,
        iOffset,
        Some(
            crate::src::src::btree::sqlite3BtreePayloadChecked
                as unsafe extern "C" fn(
                    *mut crate::src::headers::btreeInt_h::BtCursor,
                    crate::src::ext::rtree::rtree::u32_0,
                    crate::src::ext::rtree::rtree::u32_0,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    )
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3_blob_write(
    mut pBlob: *mut crate::src::headers::sqlite3_h::sqlite3_blob,
    mut z: *const ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
    mut iOffset: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    blobReadWrite(
        pBlob,
        z as *mut ::core::ffi::c_void,
        n,
        iOffset,
        Some(
            crate::src::src::btree::sqlite3BtreePutData
                as unsafe extern "C" fn(
                    *mut crate::src::headers::btreeInt_h::BtCursor,
                    crate::src::ext::rtree::rtree::u32_0,
                    crate::src::ext::rtree::rtree::u32_0,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    )
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3_blob_bytes(mut pBlob: *mut crate::src::headers::sqlite3_h::sqlite3_blob) -> ::core::ffi::c_int {
    let mut p: *mut Incrblob = pBlob as *mut Incrblob;
    if !p.is_null() && !(*p).pStmt.is_null() {
        (*p).nByte
    } else {
        0 as ::core::ffi::c_int
    }
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3_blob_reopen(
    mut pBlob: *mut crate::src::headers::sqlite3_h::sqlite3_blob,
    mut iRow: crate::src::headers::sqlite3_h::sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut p: *mut Incrblob = pBlob as *mut Incrblob;
    let mut db: *mut crate::src::headers::sqliteInt_h::sqlite3 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>();
    if p.is_null() {
        return crate::src::src::main::sqlite3MisuseError(508 as ::core::ffi::c_int);
    }
    db = (*p).db;
    crate::src::src::mutex::sqlite3_mutex_enter((*db).mutex);
    if (*p).pStmt.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_ABORT;
    } else {
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*((*p).pStmt as *mut crate::src::headers::vdbeInt_h::Vdbe)).rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        rc = blobSeekToRow(p, iRow, &raw mut zErr);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::printf_c_variadic::sqlite3ErrorWithMsg_args(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                rc,
                if !zErr.is_null() {
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null_mut::<::core::ffi::c_char>() as *const ::core::ffi::c_char
                },
                &[crate::src::src::printf::PrintfArg::Str(zErr)],
            );
            crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, zErr as *mut ::core::ffi::c_void);
        }
    }
    rc = crate::src::src::malloc::sqlite3ApiExit(db as *mut crate::src::headers::sqliteInt_h::sqlite3, rc);
    crate::src::src::mutex::sqlite3_mutex_leave((*db).mutex);
    rc
}
