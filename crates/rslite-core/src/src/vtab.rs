pub use crate::__stddef_size_t_h::SizeT;
pub use crate::src::headers::stdlib::VaList;

pub use crate::internal::BuiltinVaList;
pub use crate::internal::VaListTag;
pub use crate::src::headers::opcodes_h::OP_Expire;
pub use crate::src::headers::opcodes_h::OP_VCreate_1;
pub use crate::src::src::hash::_ht;
pub use crate::src::src::hash::Hash;
pub use crate::src::src::hash::HashElem;
pub use crate::src::src::hash::sqlite3HashFind;
pub use crate::src::src::hash::sqlite3HashInsert;
pub use crate::src::src::pager::Pgno;

pub use crate::src::ext::rtree::rtree::I64_0;
pub use crate::src::ext::rtree::rtree::U8_0;
pub use crate::src::ext::rtree::rtree::U32_0;
pub use crate::src::ext::rtree::rtree::U64_0;
pub use crate::src::fts5::I16_0;
pub use crate::src::fts5::U16_0;
pub use crate::src::headers::sqlite3_h::SQLITE_ABORT;
pub use crate::src::headers::sqlite3_h::SQLITE_CREATE_VTABLE;
pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;
pub use crate::src::headers::sqlite3_h::SQLITE_FAIL;
pub use crate::src::headers::sqlite3_h::SQLITE_IGNORE;
pub use crate::src::headers::sqlite3_h::SQLITE_LIMIT_COLUMN;
pub use crate::src::headers::sqlite3_h::SQLITE_LOCKED;
pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SQLITE_REPLACE;
pub use crate::src::headers::sqlite3_h::SQLITE_ROLLBACK;
pub use crate::src::headers::sqlite3_h::SqliteInt64;
pub use crate::src::headers::sqlite3_h::SqliteUint64;
pub use crate::src::headers::sqlite3_h::sqlite3_file;
pub use crate::src::headers::sqlite3_h::Sqlite3Filename;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;
pub use crate::src::headers::sqlite3_h::sqlite3_index_info;
pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;
pub use crate::src::headers::sqlite3_h::Sqlite3Int64;
pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;
pub use crate::src::headers::sqlite3_h::sqlite3_module;
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
pub use crate::src::headers::sqliteInt_h::COLFLAG_HIDDEN;
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
pub use crate::src::headers::sqliteInt_h::PARSE_MODE_DECLARE_VTAB;
pub use crate::src::headers::sqliteInt_h::PARSE_MODE_NORMAL;
pub use crate::src::headers::sqliteInt_h::Parse;
pub use crate::src::headers::sqliteInt_h::ParseCleanup;
pub use crate::src::headers::sqliteInt_h::RenameToken;
pub use crate::src::headers::sqliteInt_h::Returning;
pub use crate::src::headers::sqliteInt_h::SAVEPOINT_BEGIN_1;
pub use crate::src::headers::sqliteInt_h::SAVEPOINT_ROLLBACK_1;
pub use crate::src::headers::sqliteInt_h::SQLITE_Defensive;
pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_EPHEM;
pub use crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
pub use crate::src::headers::sqliteInt_h::SQLITE_VTABRISK_High;
pub use crate::src::headers::sqliteInt_h::SQLITE_VTABRISK_Low;
pub use crate::src::headers::sqliteInt_h::SQLITE_VTABRISK_Normal;
pub use crate::src::headers::sqliteInt_h::Savepoint;
pub use crate::src::headers::sqliteInt_h::Schema;
pub use crate::src::headers::sqliteInt_h::Select;
pub use crate::src::headers::sqliteInt_h::SrcItem;
pub use crate::src::headers::sqliteInt_h::SrcList;
pub use crate::src::headers::sqliteInt_h::Subquery;
pub use crate::src::headers::sqliteInt_h::TABTYP_VTAB;
pub use crate::src::headers::sqliteInt_h::TF_Ephemeral;
pub use crate::src::headers::sqliteInt_h::TF_Eponymous;
pub use crate::src::headers::sqliteInt_h::TF_HasHidden;
pub use crate::src::headers::sqliteInt_h::TF_NoVisibleRowid;
pub use crate::src::headers::sqliteInt_h::TF_OOOHidden;
pub use crate::src::headers::sqliteInt_h::TF_WithoutRowid;
pub use crate::src::headers::sqliteInt_h::Table;
pub use crate::src::headers::sqliteInt_h::TableLock;
pub use crate::src::headers::sqliteInt_h::Token;
pub use crate::src::headers::sqliteInt_h::Trigger;
pub use crate::src::headers::sqliteInt_h::TriggerPrg;
pub use crate::src::headers::sqliteInt_h::TriggerStep;
pub use crate::src::headers::sqliteInt_h::Upsert;
pub use crate::src::headers::sqliteInt_h::VList;
pub use crate::src::headers::sqliteInt_h::VTable;
pub use crate::src::headers::sqliteInt_h::Window;
pub use crate::src::headers::sqliteInt_h::With;
pub use crate::src::headers::sqliteInt_h::Bft;
pub use crate::src::headers::sqliteInt_h::sColMap;
pub use crate::src::headers::sqliteInt_h::sqlite3;
pub use crate::src::headers::sqliteInt_h::Sqlite3Xauth;
pub use crate::src::headers::sqliteInt_h::sqlite3InitInfo;
pub use crate::src::headers::sqliteInt_h::YDbMask;
pub use crate::src::headers::sqliteInt_h::YnVar;
pub use crate::src::headers::stdlib::Int16T;
pub use crate::src::headers::vdbeInt_h::PreUpdate;
pub use crate::src::headers::vdbeInt_h::sqlite3_context;
pub use crate::src::headers::vdbeInt_h::sqlite3_value;
pub use crate::src::parse::TK_COLUMN;
pub use crate::src::parse::TK_COMMENT;
pub use crate::src::parse::TK_CREATE;
pub use crate::src::parse::TK_SPACE;
pub use crate::src::parse::TK_TABLE;
pub use crate::src::src::auth::sqlite3AuthCheck;
pub use crate::src::src::build::sqlite3ChangeCookie;
pub use crate::src::src::build::sqlite3DeleteTable;
pub use crate::src::src::build::sqlite3FindTable;
pub use crate::src::src::build::sqlite3MarkAllShadowTablesOf;
pub use crate::src::src::build::sqlite3MayAbort;
pub use crate::src::src::build::sqlite3NameFromToken;
pub use crate::src::src::build::sqlite3PrimaryKeyIndex;
pub use crate::src::src::build::sqlite3StartTable;
pub use crate::src::src::expr::sqlite3ExprListDelete;
pub use crate::src::src::main::sqlite3MisuseError;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3ApiExit;
pub use crate::src::src::malloc::sqlite3DbFree;
pub use crate::src::src::malloc::sqlite3DbMallocZero;
pub use crate::src::src::malloc::sqlite3DbRealloc;
pub use crate::src::src::malloc::sqlite3DbStrDup;
pub use crate::src::src::malloc::sqlite3DbStrNDup;
pub use crate::src::src::malloc::sqlite3Malloc;
pub use crate::src::src::malloc::sqlite3MallocZero;
pub use crate::src::src::malloc::sqlite3OomFault;
pub use crate::src::src::malloc::sqlite3Realloc;
pub use crate::src::src::mutex::sqlite3_mutex_enter;
pub use crate::src::src::mutex::sqlite3_mutex_leave;
pub use crate::src::src::mutex_unix::sqlite3_mutex;
pub use crate::src::src::prepare::sqlite3ParseObjectInit;
pub use crate::src::src::prepare::sqlite3ParseObjectReset;
pub use crate::src::src::prepare::sqlite3SchemaToIndex;
pub use crate::src::src::select::sqlite3GetVdbe;
pub use crate::src::src::tokenize::sqlite3GetToken;
pub use crate::src::src::tokenize::sqlite3RunParser;
pub use crate::src::src::util::sqlite3_strnicmp;
pub use crate::src::src::util::sqlite3ColumnType;
pub use crate::src::src::util::sqlite3Error;
pub use crate::src::src::util::sqlite3Strlen30;
pub use crate::src::src::vdbeaux::sqlite3VtabImportErrmsg;

pub use crate::src::headers::stdlib::Uint8T;
pub use crate::src::headers::stdlib::Uint16T;
pub use crate::src::headers::stdlib::Uint32T;
pub use crate::src::headers::vdbeInt_h::Vdbe;
pub use crate::src::src::vdbe::Mem;
pub use crate::src::src::vdbe::SubProgram;
pub use crate::src::src::vdbe::SubrtnSig;
pub use crate::src::src::vdbe::VdbeOp;
pub use crate::src::src::vdbe::p4union;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp0;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp2;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddParseSchemaOp;
pub use crate::src::src::vdbeaux::sqlite3VdbeFinalize;
pub use crate::src::src::vdbeaux::sqlite3VdbeLoadString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VtabCtx {
    pub pVTable: *mut crate::src::headers::sqliteInt_h::VTable,
    pub pTab: *mut crate::src::headers::sqliteInt_h::Table,
    pub pPrior: *mut VtabCtx,
    pub bDeclared: ::core::ffi::c_int,
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabCreateModule(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zName: *const ::core::ffi::c_char,
    pModule: *const crate::src::headers::sqlite3_h::sqlite3_module,
    pAux: *mut ::core::ffi::c_void,
    xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> *mut crate::src::headers::sqliteInt_h::Module {
    let mut pMod: *mut crate::src::headers::sqliteInt_h::Module;
    
    let zCopy: *mut ::core::ffi::c_char;
    if pModule.is_null() {
        zCopy = zName as *mut ::core::ffi::c_char;
        pMod = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Module>();
    } else {
        let nName: ::core::ffi::c_int = crate::src::src::util::sqlite3Strlen30(zName);
        pMod = crate::src::src::malloc::sqlite3Malloc(
            (::core::mem::size_of::<crate::src::headers::sqliteInt_h::Module>() as usize)
                .wrapping_add(nName as usize)
                .wrapping_add(1_usize) as crate::src::ext::rtree::rtree::U64_0,
        ) as *mut crate::src::headers::sqliteInt_h::Module;
        if pMod.is_null() {
            crate::src::src::malloc::sqlite3OomFault(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            );
            return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Module>();
        }
        zCopy = pMod.offset(1_isize) as *mut crate::src::headers::sqliteInt_h::Module
            as *mut ::core::ffi::c_char;
        ::core::ptr::copy_nonoverlapping(
            zName as *const u8,
            zCopy as *mut u8,
            (nName + 1 as ::core::ffi::c_int) as usize,
        );
        (*pMod).zName = zCopy;
        (*pMod).pModule = pModule;
        (*pMod).pAux = pAux;
        (*pMod).xDestroy = xDestroy;
        (*pMod).pEpoTab = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
        (*pMod).nRefModule = 1 as ::core::ffi::c_int;
    }
    let pDel: *mut crate::src::headers::sqliteInt_h::Module = crate::src::src::hash::sqlite3HashInsert(
        &raw mut (*db).aModule as *mut _ as *mut crate::src::src::hash::Hash,
        zCopy,
        pMod as *mut ::core::ffi::c_void,
    ) as *mut crate::src::headers::sqliteInt_h::Module;
    if !pDel.is_null() {
        if pDel == pMod {
            crate::src::src::malloc::sqlite3OomFault(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            );
            crate::src::src::malloc::sqlite3DbFree(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                pDel as *mut ::core::ffi::c_void,
            );
            pMod = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Module>();
        } else {
            sqlite3VtabEponymousTableClear(db, pDel);
            sqlite3VtabModuleUnref(db, pDel);
        }
    }
    pMod
}

unsafe extern "C" fn createModule(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zName: *const ::core::ffi::c_char,
    pModule: *const crate::src::headers::sqlite3_h::sqlite3_module,
    pAux: *mut ::core::ffi::c_void,
    xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    crate::src::src::mutex::sqlite3_mutex_enter((*db).mutex);
    sqlite3VtabCreateModule(db, zName, pModule, pAux, xDestroy);
    rc = crate::src::src::malloc::sqlite3ApiExit(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        rc,
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK && xDestroy.is_some() {
        xDestroy.expect("non-null function pointer")(pAux);
    }
    crate::src::src::mutex::sqlite3_mutex_leave((*db).mutex);
    rc
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_create_module(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zName: *const ::core::ffi::c_char,
    pModule: *const crate::src::headers::sqlite3_h::sqlite3_module,
    pAux: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    createModule(db, zName, pModule, pAux, None)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_create_module_v2(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zName: *const ::core::ffi::c_char,
    pModule: *const crate::src::headers::sqlite3_h::sqlite3_module,
    pAux: *mut ::core::ffi::c_void,
    xDestroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) -> ::core::ffi::c_int {
    createModule(db, zName, pModule, pAux, xDestroy)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_drop_modules(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    azNames: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pThis: *mut crate::src::src::hash::HashElem;
    let mut pNext: *mut crate::src::src::hash::HashElem;
    let mut current_block_5: u64;
    pThis = (*db).aModule.first;
    while !pThis.is_null() {
        let pMod: *mut crate::src::headers::sqliteInt_h::Module =
            (*pThis).data as *mut crate::src::headers::sqliteInt_h::Module;
        pNext = (*pThis).next;
        if !azNames.is_null() {
            let mut ii: ::core::ffi::c_int;
            ii = 0 as ::core::ffi::c_int;
            while !(*azNames.offset(ii as isize)).is_null()
                && ::libc::strcmp(*azNames.offset(ii as isize), (*pMod).zName)
                    != 0 as ::core::ffi::c_int
            {
                ii += 1;
            }
            if !(*azNames.offset(ii as isize)).is_null() {
                current_block_5 = 11174649648027449784;
            } else {
                current_block_5 = 13513818773234778473;
            }
        } else {
            current_block_5 = 13513818773234778473;
        }
        match current_block_5 {
            13513818773234778473 => {
                createModule(
                    db,
                    (*pMod).zName,
                    ::core::ptr::null::<crate::src::headers::sqlite3_h::sqlite3_module>(),
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    None,
                );
            }
            _ => {}
        }
        pThis = pNext;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabModuleUnref(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pMod: *mut crate::src::headers::sqliteInt_h::Module,
) {
    (*pMod).nRefModule -= 1;
    if (*pMod).nRefModule == 0 as ::core::ffi::c_int {
        if (*pMod).xDestroy.is_some() {
            (*pMod).xDestroy.expect("non-null function pointer")((*pMod).pAux);
        }
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pMod as *mut ::core::ffi::c_void,
        );
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabLock(pVTab: *mut crate::src::headers::sqliteInt_h::VTable) {
    (*pVTab).nRef += 1;
}
pub unsafe extern "C" fn sqlite3GetVTable(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pTab: *mut crate::src::headers::sqliteInt_h::Table,
) -> *mut crate::src::headers::sqliteInt_h::VTable {
    let mut pVtab: *mut crate::src::headers::sqliteInt_h::VTable;
    pVtab = (*pTab).u.vtab.p;
    while !pVtab.is_null() && (*pVtab).db != db {
        pVtab = (*pVtab).pNext;
    }
    pVtab
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabUnlock(
    pVTab: *mut crate::src::headers::sqliteInt_h::VTable,
) {
    let __pVTab_ref = unsafe { &mut *pVTab };
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pVTab_ref.db;
    __pVTab_ref.nRef -= 1;
    if __pVTab_ref.nRef == 0 as ::core::ffi::c_int {
        let p: *mut crate::src::headers::sqlite3_h::sqlite3_vtab = __pVTab_ref.pVtab;
        if !p.is_null() {
            (*(*p).pModule)
                .xDisconnect
                .expect("non-null function pointer")(p);
        }
        sqlite3VtabModuleUnref(__pVTab_ref.db, __pVTab_ref.pMod);
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pVTab as *mut ::core::ffi::c_void,
        );
    }
}

unsafe extern "C" fn vtabDisconnectAll(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *mut crate::src::headers::sqliteInt_h::Table,
) -> *mut crate::src::headers::sqliteInt_h::VTable {
    let mut pRet: *mut crate::src::headers::sqliteInt_h::VTable =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::VTable>();
    let mut pVTable: *mut crate::src::headers::sqliteInt_h::VTable;
    pVTable = (*p).u.vtab.p;
    (*p).u.vtab.p = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::VTable>();
    while !pVTable.is_null() {
        let db2: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pVTable).db;
        let pNext: *mut crate::src::headers::sqliteInt_h::VTable = (*pVTable).pNext;
        if db2 == db {
            pRet = pVTable;
            (*p).u.vtab.p = pRet;
            (*pRet).pNext = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::VTable>();
        } else {
            (*pVTable).pNext = (*db2).pDisconnect;
            (*db2).pDisconnect = pVTable;
        }
        pVTable = pNext;
    }
    pRet
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabDisconnect(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *mut crate::src::headers::sqliteInt_h::Table,
) {
    let mut ppVTab: *mut *mut crate::src::headers::sqliteInt_h::VTable;
    ppVTab = &raw mut (*p).u.vtab.p;
    while !(*ppVTab).is_null() {
        if (**ppVTab).db == db {
            let pVTab: *mut crate::src::headers::sqliteInt_h::VTable = *ppVTab;
            *ppVTab = (*pVTab).pNext;
            sqlite3VtabUnlock(pVTab);
            break;
        } else {
            ppVTab = &raw mut (**ppVTab).pNext;
        }
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabUnlockList(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
) {
    let mut p: *mut crate::src::headers::sqliteInt_h::VTable = (*db).pDisconnect;
    if !p.is_null() {
        (*db).pDisconnect = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::VTable>();
        loop {
            let pNext: *mut crate::src::headers::sqliteInt_h::VTable = (*p).pNext;
            sqlite3VtabUnlock(p);
            p = pNext;
            if p.is_null() {
                break;
            }
        }
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabClear(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *mut crate::src::headers::sqliteInt_h::Table,
) {
    if (*db).pnBytesFreed.is_null() {
        vtabDisconnectAll(
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>(),
            p,
        );
    }
    if !(*p).u.vtab.azArg.is_null() {
        let mut i: ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).u.vtab.nArg {
            if i != 1 as ::core::ffi::c_int {
                crate::src::src::malloc::sqlite3DbFree(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    *(*p).u.vtab.azArg.offset(i as isize) as *mut ::core::ffi::c_void,
                );
            }
            i += 1;
        }
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*p).u.vtab.azArg as *mut ::core::ffi::c_void,
        );
    }
}

unsafe extern "C" fn addModuleArgument(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pTable: *mut crate::src::headers::sqliteInt_h::Table,
    zArg: *mut ::core::ffi::c_char,
) {
    
    
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    let __pTable_ref = unsafe { &mut *pTable };
    let nBytes: crate::src::headers::sqlite3_h::Sqlite3Int64 = (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
        .wrapping_mul((2 as ::core::ffi::c_int + __pTable_ref.u.vtab.nArg) as usize)
        as crate::src::headers::sqlite3_h::Sqlite3Int64;
    if __pTable_ref.u.vtab.nArg + 3 as ::core::ffi::c_int
        >= (*db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_COLUMN as usize]
    {
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"too many columns on %s\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Str(
                __pTable_ref.zName as *mut ::core::ffi::c_char,
            )],
        );
    }
    let azModuleArg: *mut *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3DbRealloc(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __pTable_ref.u.vtab.azArg as *mut ::core::ffi::c_void,
        nBytes as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut *mut ::core::ffi::c_char;
    if azModuleArg.is_null() {
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            zArg as *mut ::core::ffi::c_void,
        );
    } else {
        let fresh2 = __pTable_ref.u.vtab.nArg;
        __pTable_ref.u.vtab.nArg += 1;
        let i: ::core::ffi::c_int = fresh2;
        let fresh3 = &mut *azModuleArg.offset(i as isize);
        *fresh3 = zArg;
        let fresh4 = &mut *azModuleArg.offset((i + 1 as ::core::ffi::c_int) as isize);
        *fresh4 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        __pTable_ref.u.vtab.azArg = azModuleArg;
    };
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabBeginParse(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pName1: *mut crate::src::headers::sqliteInt_h::Token,
    pName2: *mut crate::src::headers::sqliteInt_h::Token,
    pModuleName: *mut crate::src::headers::sqliteInt_h::Token,
    ifNotExists: ::core::ffi::c_int,
) {
    
    
    crate::src::src::build::sqlite3StartTable(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        pName1 as *mut crate::src::headers::sqliteInt_h::Token,
        pName2 as *mut crate::src::headers::sqliteInt_h::Token,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        ifNotExists,
    );
    let __pParse_ref = unsafe { &mut *pParse };
    let pTable: *mut crate::src::headers::sqliteInt_h::Table = __pParse_ref.pNewTable;
    if pTable.is_null() {
        return;
    }
    (*pTable).eTabType =
        crate::src::headers::sqliteInt_h::TABTYP_VTAB as crate::src::ext::rtree::rtree::U8_0;
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
    addModuleArgument(
        pParse,
        pTable,
        crate::src::src::build::sqlite3NameFromToken(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pModuleName as *const crate::src::headers::sqliteInt_h::Token,
        ),
    );
    addModuleArgument(
        pParse,
        pTable,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
    addModuleArgument(
        pParse,
        pTable,
        crate::src::src::malloc::sqlite3DbStrDup(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*pTable).zName,
        ),
    );
    __pParse_ref.sNameToken.n =
        ((*pModuleName).z.offset((*pModuleName).n as isize) as *const ::core::ffi::c_char)
            .offset_from(__pParse_ref.sNameToken.z) as ::core::ffi::c_long
            as ::core::ffi::c_int as ::core::ffi::c_uint;
    if !(*pTable).u.vtab.azArg.is_null() {
        let __pTable_ref = unsafe { &mut *pTable };
        let iDb: ::core::ffi::c_int = crate::src::src::prepare::sqlite3SchemaToIndex(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            __pTable_ref.pSchema as *mut crate::src::headers::sqliteInt_h::Schema,
        );
        crate::src::src::auth::sqlite3AuthCheck(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            crate::src::headers::sqlite3_h::SQLITE_CREATE_VTABLE,
            __pTable_ref.zName,
            *(*pTable).u.vtab.azArg.offset(0_isize),
            (*(*__pParse_ref.db).aDb.offset(iDb as isize)).zDbSName,
        );
    }
}

unsafe extern "C" fn addArgumentToVtab(pParse: *mut crate::src::headers::sqliteInt_h::Parse) {
    if !(*pParse).sArg.z.is_null() && !(*pParse).pNewTable.is_null() {
        let __pParse_ref = unsafe { &*pParse };
        let z: *const ::core::ffi::c_char = __pParse_ref.sArg.z;
        let n: ::core::ffi::c_int = __pParse_ref.sArg.n as ::core::ffi::c_int;
        let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
        addModuleArgument(
            pParse,
            __pParse_ref.pNewTable,
            crate::src::src::malloc::sqlite3DbStrNDup(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                z,
                n as crate::src::ext::rtree::rtree::U64_0,
            ),
        );
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabFinishParse(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pEnd: *mut crate::src::headers::sqliteInt_h::Token,
) {
    let __pParse_ref = unsafe { &mut *pParse };
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = __pParse_ref.pNewTable;
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
    if pTab.is_null() {
        return;
    }
    addArgumentToVtab(pParse);
    __pParse_ref.sArg.z = ::core::ptr::null::<::core::ffi::c_char>();
    if (*pTab).u.vtab.nArg < 1 as ::core::ffi::c_int {
        return;
    }
    if (*db).init.busy == 0 {
        
        
        
        
        
        crate::src::src::build::sqlite3MayAbort(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        );
        if !pEnd.is_null() {
            __pParse_ref.sNameToken.n =
                ((*pEnd).z.offset_from(__pParse_ref.sNameToken.z) as ::core::ffi::c_long
                    as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_add((*pEnd).n);
        }
        let zStmt: *mut ::core::ffi::c_char = crate::src::src::printf::sqlite3MPrintf_args(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            b"CREATE VIRTUAL TABLE %T\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Token(
                &raw mut __pParse_ref.sNameToken,
            )],
        );
        let __pTab_ref = unsafe { &*pTab };
        let iDb: ::core::ffi::c_int = crate::src::src::prepare::sqlite3SchemaToIndex(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            __pTab_ref.pSchema as *mut crate::src::headers::sqliteInt_h::Schema,
        );
        crate::src::src::build::sqlite3NestedParse_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            "UPDATE %Q.sqlite_master SET type='table', name=%Q, tbl_name=%Q, rootpage=0, sql=%Q WHERE rowid=#%d",
            crate::printf_args_slice!(
                (*(*db).aDb.offset(iDb as isize)).zDbSName,
                __pTab_ref.zName,
                __pTab_ref.zName,
                zStmt,
                __pParse_ref.u1.cr.regRowid
            ),
        );
        let v: *mut crate::src::headers::vdbeInt_h::Vdbe = crate::src::src::select::sqlite3GetVdbe(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        );
        crate::src::src::build::sqlite3ChangeCookie(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            iDb,
        );
        crate::src::src::vdbeaux::sqlite3VdbeAddOp0(v, crate::src::headers::opcodes_h::OP_Expire);
        let zWhere: *mut ::core::ffi::c_char = crate::src::src::printf::sqlite3MPrintf_args(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            b"name=%Q AND sql=%Q\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::src::src::printf::PrintfArg::Str(
                    __pTab_ref.zName as *mut ::core::ffi::c_char,
                ),
                crate::src::src::printf::PrintfArg::Str(zStmt),
            ],
        );
        crate::src::src::vdbeaux::sqlite3VdbeAddParseSchemaOp(
            v,
            iDb,
            zWhere,
            0 as crate::src::fts5::U16_0,
        );
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            zStmt as *mut ::core::ffi::c_void,
        );
        __pParse_ref.nMem += 1;
        let iReg: ::core::ffi::c_int = __pParse_ref.nMem;
        crate::src::src::vdbeaux::sqlite3VdbeLoadString(v, iReg, __pTab_ref.zName);
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_VCreate_1,
            iDb,
            iReg,
        );
    } else {
        
        let pSchema: *mut crate::src::headers::sqliteInt_h::Schema = (*pTab).pSchema;
        let zName: *const ::core::ffi::c_char = (*pTab).zName;
        crate::src::src::build::sqlite3MarkAllShadowTablesOf(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pTab as *mut crate::src::headers::sqliteInt_h::Table,
        );
        let pOld: *mut crate::src::headers::sqliteInt_h::Table = crate::src::src::hash::sqlite3HashInsert(
            &raw mut (*pSchema).tblHash as *mut _ as *mut crate::src::src::hash::Hash,
            zName,
            pTab as *mut ::core::ffi::c_void,
        ) as *mut crate::src::headers::sqliteInt_h::Table;
        if !pOld.is_null() {
            crate::src::src::malloc::sqlite3OomFault(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            );
            return;
        }
        __pParse_ref.pNewTable = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
    };
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabArgInit(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
) {
    addArgumentToVtab(pParse);
    (*pParse).sArg.z = ::core::ptr::null::<::core::ffi::c_char>();
    (*pParse).sArg.n = 0 as ::core::ffi::c_uint;
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabArgExtend(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Token,
) {
    let pArg: *mut crate::src::headers::sqliteInt_h::Token = &raw mut (*pParse).sArg;
    if (*pArg).z.is_null() {
        (*pArg).z = (*p).z;
        (*pArg).n = (*p).n;
    } else {
        (*pArg).n = ((*p).z.offset((*p).n as isize) as *const ::core::ffi::c_char)
            .offset_from((*pArg).z) as ::core::ffi::c_long as ::core::ffi::c_int
            as ::core::ffi::c_uint;
    };
}

unsafe extern "C" fn vtabCallConstructor(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pTab: *mut crate::src::headers::sqliteInt_h::Table,
    pMod: *mut crate::src::headers::sqliteInt_h::Module,
    xConstruct: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut sCtx: VtabCtx = unsafe { ::core::mem::zeroed() };
    
    let mut rc: ::core::ffi::c_int;
    
    let __pTab_ref = unsafe { &mut *pTab };
    let nArg: ::core::ffi::c_int = __pTab_ref.u.vtab.nArg;
    let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    
    
    let mut pCtx: *mut VtabCtx;
    let azArg: *const *const ::core::ffi::c_char = __pTab_ref.u.vtab.azArg as *const *const ::core::ffi::c_char;
    let __db_ref = unsafe { &mut *db };
    pCtx = __db_ref.pVtabCtx;
    while !pCtx.is_null() {
        if (*pCtx).pTab == pTab {
            *pzErr = crate::src::src::printf::sqlite3MPrintf_args(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                b"vtable constructor called recursively: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Str(
                    __pTab_ref.zName as *mut ::core::ffi::c_char,
                )],
            );
            return crate::src::headers::sqlite3_h::SQLITE_LOCKED;
        }
        pCtx = (*pCtx).pPrior;
    }
    let zModuleName: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3DbStrDup(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __pTab_ref.zName,
    );
    if zModuleName.is_null() {
        return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
    }
    let pVTable: *mut crate::src::headers::sqliteInt_h::VTable = crate::src::src::malloc::sqlite3MallocZero(::core::mem::size_of::<
        crate::src::headers::sqliteInt_h::VTable,
    >()
        as crate::src::ext::rtree::rtree::U64_0)
        as *mut crate::src::headers::sqliteInt_h::VTable;
    if pVTable.is_null() {
        crate::src::src::malloc::sqlite3OomFault(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        );
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            zModuleName as *mut ::core::ffi::c_void,
        );
        return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
    }
    (*pVTable).db = db;
    (*pVTable).pMod = pMod;
    (*pVTable).eVtabRisk = crate::src::headers::sqliteInt_h::SQLITE_VTABRISK_Normal
        as crate::src::ext::rtree::rtree::U8_0;
    let iDb: ::core::ffi::c_int = crate::src::src::prepare::sqlite3SchemaToIndex(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __pTab_ref.pSchema as *mut crate::src::headers::sqliteInt_h::Schema,
    );
    let fresh0 = &mut *(*pTab).u.vtab.azArg.offset(1_isize);
    *fresh0 = (*__db_ref.aDb.offset(iDb as isize)).zDbSName;
    sCtx.pTab = pTab;
    sCtx.pVTable = pVTable;
    sCtx.pPrior = __db_ref.pVtabCtx;
    sCtx.bDeclared = 0 as ::core::ffi::c_int;
    __db_ref.pVtabCtx = &raw mut sCtx;
    __pTab_ref.nTabRef = __pTab_ref.nTabRef.wrapping_add(1);
    rc = xConstruct.expect("non-null function pointer")(
        db,
        (*pMod).pAux,
        nArg,
        azArg,
        &raw mut (*pVTable).pVtab,
        &raw mut zErr,
    );
    crate::src::src::build::sqlite3DeleteTable(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        pTab as *mut crate::src::headers::sqliteInt_h::Table,
    );
    __db_ref.pVtabCtx = sCtx.pPrior;
    if rc == crate::src::headers::sqlite3_h::SQLITE_NOMEM {
        crate::src::src::malloc::sqlite3OomFault(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        );
    }
    if crate::src::headers::sqlite3_h::SQLITE_OK != rc {
        if zErr.is_null() {
            *pzErr = crate::src::src::printf::sqlite3MPrintf_args(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                b"vtable constructor failed: %s\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Str(zModuleName)],
            );
        } else {
            *pzErr = crate::src::src::printf::sqlite3MPrintf_args(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Str(zErr)],
            );
            crate::src::src::malloc::sqlite3_free(zErr as *mut ::core::ffi::c_void);
        }
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pVTable as *mut ::core::ffi::c_void,
        );
    } else if !(*pVTable).pVtab.is_null() {
        let __pVTable_ref = unsafe { &mut *pVTable };
        ::libc::memset(
            __pVTable_ref.pVtab as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<crate::src::headers::sqlite3_h::sqlite3_vtab>()
                as crate::__stddef_size_t_h::SizeT,
        );
        (*__pVTable_ref.pVtab).pModule = (*pMod).pModule;
        (*pMod).nRefModule += 1;
        __pVTable_ref.nRef = 1 as ::core::ffi::c_int;
        if sCtx.bDeclared == 0 as ::core::ffi::c_int {
            let zFormat: *const ::core::ffi::c_char =
                b"vtable constructor did not declare schema: %s\0" as *const u8
                    as *const ::core::ffi::c_char;
            *pzErr = crate::src::src::printf::sqlite3MPrintf_args(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                zFormat,
                &[crate::src::src::printf::PrintfArg::Str(zModuleName)],
            );
            sqlite3VtabUnlock(pVTable);
            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
        } else {
            let mut iCol: ::core::ffi::c_int;
            let mut oooHidden: crate::src::fts5::U16_0 = 0 as crate::src::fts5::U16_0;
            __pVTable_ref.pNext = __pTab_ref.u.vtab.p;
            __pTab_ref.u.vtab.p = pVTable;
            iCol = 0 as ::core::ffi::c_int;
            while iCol < __pTab_ref.nCol as ::core::ffi::c_int {
                let zType: *mut ::core::ffi::c_char = crate::src::src::util::sqlite3ColumnType(
                    __pTab_ref.aCol.offset(iCol as isize)
                        as *mut crate::src::headers::sqliteInt_h::Column
                        as *mut crate::src::headers::sqliteInt_h::Column,
                    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                );
                
                let mut i: ::core::ffi::c_int;
                let nType: ::core::ffi::c_int = crate::src::src::util::sqlite3Strlen30(zType);
                i = 0 as ::core::ffi::c_int;
                while i < nType {
                    if 0 as ::core::ffi::c_int
                        == crate::src::src::util::sqlite3_strnicmp(
                            b"hidden\0" as *const u8 as *const ::core::ffi::c_char,
                            zType.offset(i as isize) as *mut ::core::ffi::c_char,
                            6 as ::core::ffi::c_int,
                        )
                        && (i == 0 as ::core::ffi::c_int
                            || *zType.offset((i - 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int
                                == ' ' as i32)
                        && (*zType.offset((i + 6 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == '\0' as i32
                            || *zType.offset((i + 6 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int
                                == ' ' as i32)
                    {
                        break;
                    }
                    i += 1;
                }
                if i < nType {
                    let mut j: ::core::ffi::c_int;
                    let nDel: ::core::ffi::c_int = 6 as ::core::ffi::c_int
                        + (if *zType.offset((i + 6 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        });
                    j = i;
                    while j + nDel <= nType {
                        *zType.offset(j as isize) = *zType.offset((j + nDel) as isize);
                        j += 1;
                    }
                    if *zType.offset(i as isize) as ::core::ffi::c_int == '\0' as i32
                        && i > 0 as ::core::ffi::c_int
                    {
                        *zType.offset((i - 1 as ::core::ffi::c_int) as isize) =
                            '\0' as i32 as ::core::ffi::c_char;
                    }
                    let fresh1 = &mut (*__pTab_ref.aCol.offset(iCol as isize)).colFlags;
                    *fresh1 = (*fresh1 as ::core::ffi::c_int
                        | crate::src::headers::sqliteInt_h::COLFLAG_HIDDEN)
                        as crate::src::fts5::U16_0;
                    __pTab_ref.tabFlags |= crate::src::headers::sqliteInt_h::TF_HasHidden
                        as crate::src::ext::rtree::rtree::U32_0;
                    oooHidden =
                        crate::src::headers::sqliteInt_h::TF_OOOHidden as crate::src::fts5::U16_0;
                } else {
                    __pTab_ref.tabFlags |= oooHidden as crate::src::ext::rtree::rtree::U32_0;
                }
                iCol += 1;
            }
        }
    }
    crate::src::src::malloc::sqlite3DbFree(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        zModuleName as *mut ::core::ffi::c_void,
    );
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabCallConnect(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pTab: *mut crate::src::headers::sqliteInt_h::Table,
) -> ::core::ffi::c_int {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    
    
    let rc: ::core::ffi::c_int;
    if !sqlite3GetVTable(db, pTab).is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    let zMod: *const ::core::ffi::c_char = *(*pTab).u.vtab.azArg.offset(0_isize);
    let pMod: *mut crate::src::headers::sqliteInt_h::Module = crate::src::src::hash::sqlite3HashFind(
        &raw mut (*db).aModule as *mut _ as *const crate::src::src::hash::Hash,
        zMod,
    ) as *mut crate::src::headers::sqliteInt_h::Module;
    if pMod.is_null() {
        let zModule: *const ::core::ffi::c_char = *(*pTab).u.vtab.azArg.offset(0_isize);
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"no such module: %s\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Str(
                zModule as *mut ::core::ffi::c_char,
            )],
        );
        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    } else {
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        rc = vtabCallConstructor(db, pTab, pMod, (*(*pMod).pModule).xConnect, &raw mut zErr);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::src::util::sqlite3ErrorMsg_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Str(zErr)],
            );
            (*pParse).rc = rc;
        }
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            zErr as *mut ::core::ffi::c_void,
        );
    }
    rc
}

unsafe extern "C" fn growVTrans(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
) -> ::core::ffi::c_int {
    let ARRAY_INCR: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    if (*db).nVTrans % ARRAY_INCR == 0 as ::core::ffi::c_int {
        
        let __db_ref = unsafe { &mut *db };
        let nBytes: crate::src::headers::sqlite3_h::Sqlite3Int64 =
            (::core::mem::size_of::<*mut crate::src::headers::sqlite3_h::sqlite3_vtab>()
                as ::core::ffi::c_ulonglong)
                .wrapping_mul(
                    (__db_ref.nVTrans as crate::src::headers::sqlite3_h::Sqlite3Int64
                        + ARRAY_INCR as crate::src::headers::sqlite3_h::Sqlite3Int64)
                        as ::core::ffi::c_ulonglong,
                ) as crate::src::headers::sqlite3_h::Sqlite3Int64;
        let aVTrans: *mut *mut crate::src::headers::sqliteInt_h::VTable = crate::src::src::malloc::sqlite3DbRealloc(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            __db_ref.aVTrans as *mut ::core::ffi::c_void,
            nBytes as crate::src::ext::rtree::rtree::U64_0,
        ) as *mut *mut crate::src::headers::sqliteInt_h::VTable;
        if aVTrans.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
        }
        ::libc::memset(
            aVTrans.offset(__db_ref.nVTrans as isize)
                as *mut *mut crate::src::headers::sqliteInt_h::VTable
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<*mut crate::src::headers::sqlite3_h::sqlite3_vtab>()
                as crate::__stddef_size_t_h::SizeT)
                .wrapping_mul(ARRAY_INCR as crate::__stddef_size_t_h::SizeT),
        );
        __db_ref.aVTrans = aVTrans;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn addToVTrans(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pVTab: *mut crate::src::headers::sqliteInt_h::VTable,
) {
    let __db_ref = unsafe { &mut *db };
    let fresh7 = __db_ref.nVTrans;
    __db_ref.nVTrans += 1;
    let fresh8 = &mut *__db_ref.aVTrans.offset(fresh7 as isize);
    *fresh8 = pVTab;
    sqlite3VtabLock(pVTab);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabCallCreate(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    iDb: ::core::ffi::c_int,
    zTab: *const ::core::ffi::c_char,
    pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    
    
    
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = crate::src::src::build::sqlite3FindTable(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        zTab,
        (*(*db).aDb.offset(iDb as isize)).zDbSName,
    ) as *mut crate::src::headers::sqliteInt_h::Table;
    let zMod: *const ::core::ffi::c_char = *(*pTab).u.vtab.azArg.offset(0_isize);
    let pMod: *mut crate::src::headers::sqliteInt_h::Module = crate::src::src::hash::sqlite3HashFind(
        &raw mut (*db).aModule as *mut _ as *const crate::src::src::hash::Hash,
        zMod,
    ) as *mut crate::src::headers::sqliteInt_h::Module;
    if pMod.is_null()
        || (*(*pMod).pModule).xCreate.is_none()
        || (*(*pMod).pModule).xDestroy.is_none()
    {
        *pzErr = crate::src::src::printf::sqlite3MPrintf_args(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            b"no such module: %s\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Str(
                zMod as *mut ::core::ffi::c_char,
            )],
        );
        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    } else {
        rc = vtabCallConstructor(db, pTab, pMod, (*(*pMod).pModule).xCreate, pzErr);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !sqlite3GetVTable(db, pTab).is_null() {
        rc = growVTrans(db);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            addToVTrans(db, sqlite3GetVTable(db, pTab));
        }
    }
    rc
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_declare_vtab(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zCreateTable: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    
    let mut sParse: crate::src::headers::sqliteInt_h::Parse = unsafe { ::core::mem::zeroed() };
    
    let mut i: ::core::ffi::c_int;
    let mut z: *const ::core::ffi::c_uchar;
    static mut aKeyword: [crate::src::ext::rtree::rtree::U8_0; 3] = [
        crate::src::parse::TK_CREATE as crate::src::ext::rtree::rtree::U8_0,
        crate::src::parse::TK_TABLE as crate::src::ext::rtree::rtree::U8_0,
        0 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U8_0,
    ];
    z = zCreateTable as *const ::core::ffi::c_uchar;
    i = 0 as ::core::ffi::c_int;
    while aKeyword[i as usize] != 0 {
        let mut tokenType: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            z = z
                .offset(crate::src::src::tokenize::sqlite3GetToken(z, &raw mut tokenType) as isize);
            if !(tokenType == crate::src::parse::TK_SPACE
                || tokenType == crate::src::parse::TK_COMMENT)
            {
                break;
            }
        }
        if tokenType != aKeyword[i as usize] as ::core::ffi::c_int {
            crate::src::src::util::sqlite3ErrorWithMsg_args(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                crate::src::headers::sqlite3_h::SQLITE_ERROR,
                b"syntax error\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
            return crate::src::headers::sqlite3_h::SQLITE_ERROR;
        }
        i += 1;
    }
    let __db_ref = unsafe { &mut *db };
    crate::src::src::mutex::sqlite3_mutex_enter(__db_ref.mutex);
    let pCtx: *mut VtabCtx = __db_ref.pVtabCtx;
    if pCtx.is_null() || (*pCtx).bDeclared != 0 {
        crate::src::src::util::sqlite3Error(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            crate::src::src::main::sqlite3MisuseError(846 as ::core::ffi::c_int),
        );
        crate::src::src::mutex::sqlite3_mutex_leave(__db_ref.mutex);
        return crate::src::src::main::sqlite3MisuseError(848 as ::core::ffi::c_int);
    }
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = (*pCtx).pTab;
    crate::src::src::prepare::sqlite3ParseObjectInit(
        &raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse,
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    );
    sParse.eParseMode = crate::src::headers::sqliteInt_h::PARSE_MODE_DECLARE_VTAB
        as crate::src::ext::rtree::rtree::U8_0;
    sParse.disableTriggers = 1 as crate::src::ext::rtree::rtree::U8_0;
    let initBusy: ::core::ffi::c_int = __db_ref.init.busy as ::core::ffi::c_int;
    __db_ref.init.busy = 0 as crate::src::ext::rtree::rtree::U8_0;
    sParse.nQueryLoop = 1 as crate::src::headers::sqliteInt_h::LogEst;
    if crate::src::headers::sqlite3_h::SQLITE_OK
        == crate::src::src::tokenize::sqlite3RunParser(
            &raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse,
            zCreateTable,
        )
    {
        if (*pTab).aCol.is_null() {
            let pNew: *mut crate::src::headers::sqliteInt_h::Table = sParse.pNewTable;
            
            let __pNew_ref = unsafe { &mut *pNew };
            let __pTab_ref = unsafe { &mut *pTab };
            __pTab_ref.aCol = __pNew_ref.aCol;
            crate::src::src::expr::sqlite3ExprListDelete(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                __pNew_ref.u.tab.pDfltList as *mut crate::src::headers::sqliteInt_h::ExprList,
            );
            __pTab_ref.nCol = __pNew_ref.nCol;
            __pTab_ref.nNVCol = __pTab_ref.nCol;
            __pTab_ref.tabFlags |= __pNew_ref.tabFlags
                & (crate::src::headers::sqliteInt_h::TF_WithoutRowid
                    | crate::src::headers::sqliteInt_h::TF_NoVisibleRowid)
                    as crate::src::ext::rtree::rtree::U32_0;
            __pNew_ref.nCol = 0 as crate::src::fts5::I16_0;
            __pNew_ref.aCol = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Column>();
            if (__pNew_ref.tabFlags
                & crate::src::headers::sqliteInt_h::TF_WithoutRowid
                    as crate::src::ext::rtree::rtree::U32_0 != 0 as crate::src::ext::rtree::rtree::U32_0)
                && (*(*(*(*pCtx).pVTable).pMod).pModule).xUpdate.is_some()
                && (*(crate::src::src::build::sqlite3PrimaryKeyIndex(
                    pNew as *mut crate::src::headers::sqliteInt_h::Table,
                ) as *mut crate::src::headers::sqliteInt_h::Index))
                    .nKeyCol as ::core::ffi::c_int
                    != 1 as ::core::ffi::c_int
            {
                rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
            }
            let pIdx: *mut crate::src::headers::sqliteInt_h::Index = __pNew_ref.pIndex;
            if !pIdx.is_null() {
                __pTab_ref.pIndex = pIdx;
                __pNew_ref.pIndex =
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Index>();
                (*pIdx).pTable = pTab;
            }
        }
        (*pCtx).bDeclared = 1 as ::core::ffi::c_int;
    } else {
        crate::src::src::util::sqlite3ErrorWithMsg_args(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            crate::src::headers::sqlite3_h::SQLITE_ERROR,
            if !sParse.zErrMsg.is_null() {
                b"%s\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            },
            &[crate::src::src::printf::PrintfArg::Str(sParse.zErrMsg)],
        );
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            sParse.zErrMsg as *mut ::core::ffi::c_void,
        );
        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    sParse.eParseMode =
        crate::src::headers::sqliteInt_h::PARSE_MODE_NORMAL as crate::src::ext::rtree::rtree::U8_0;
    if !sParse.pVdbe.is_null() {
        crate::src::src::vdbeaux::sqlite3VdbeFinalize(sParse.pVdbe);
    }
    crate::src::src::build::sqlite3DeleteTable(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        sParse.pNewTable as *mut crate::src::headers::sqliteInt_h::Table,
    );
    crate::src::src::prepare::sqlite3ParseObjectReset(
        &raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse,
    );
    __db_ref.init.busy = initBusy as crate::src::ext::rtree::rtree::U8_0;
    rc = crate::src::src::malloc::sqlite3ApiExit(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        rc,
    );
    crate::src::src::mutex::sqlite3_mutex_leave(__db_ref.mutex);
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabCallDestroy(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    iDb: ::core::ffi::c_int,
    zTab: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = crate::src::src::build::sqlite3FindTable(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        zTab,
        (*(*db).aDb.offset(iDb as isize)).zDbSName,
    ) as *mut crate::src::headers::sqliteInt_h::Table;
    if !pTab.is_null()
        && (*pTab).eTabType as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        && !(*pTab).u.vtab.p.is_null()
    {
        let mut p: *mut crate::src::headers::sqliteInt_h::VTable;
        let mut xDestroy: Option<
            unsafe extern "C" fn(
                *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
            ) -> ::core::ffi::c_int,
        >;
        let __pTab_ref = unsafe { &mut *pTab };
        p = __pTab_ref.u.vtab.p;
        while !p.is_null() {
            if (*(*p).pVtab).nRef > 0 as ::core::ffi::c_int {
                return crate::src::headers::sqlite3_h::SQLITE_LOCKED;
            }
            p = (*p).pNext;
        }
        p = vtabDisconnectAll(db, pTab);
        xDestroy = (*(*(*p).pMod).pModule).xDestroy;
        if xDestroy.is_none() {
            xDestroy = (*(*(*p).pMod).pModule).xDisconnect;
        }
        __pTab_ref.nTabRef = __pTab_ref.nTabRef.wrapping_add(1);
        rc = xDestroy.expect("non-null function pointer")((*p).pVtab);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            (*p).pVtab = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vtab>();
            __pTab_ref.u.vtab.p =
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::VTable>();
            sqlite3VtabUnlock(p);
        }
        crate::src::src::build::sqlite3DeleteTable(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pTab as *mut crate::src::headers::sqliteInt_h::Table,
        );
    }
    rc
}

unsafe extern "C" fn callFinaliser(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    offset: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int;
    if !(*db).aVTrans.is_null() {
        let __db_ref = unsafe { &mut *db };
        let aVTrans: *mut *mut crate::src::headers::sqliteInt_h::VTable = __db_ref.aVTrans;
        __db_ref.aVTrans = ::core::ptr::null_mut::<*mut crate::src::headers::sqliteInt_h::VTable>();
        i = 0 as ::core::ffi::c_int;
        while i < __db_ref.nVTrans {
            let pVTab: *mut crate::src::headers::sqliteInt_h::VTable =
                *aVTrans.offset(i as isize);
            let p: *mut crate::src::headers::sqlite3_h::sqlite3_vtab = (*pVTab).pVtab;
            if !p.is_null() {
                
                let x: Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    ) -> ::core::ffi::c_int,
                > = *(((*p).pModule as *mut ::core::ffi::c_char).offset(offset as isize)
                    as *mut Option<
                        unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                        ) -> ::core::ffi::c_int,
                    >);
                if x.is_some() {
                    x.expect("non-null function pointer")(p);
                }
            }
            (*pVTab).iSavepoint = 0 as ::core::ffi::c_int;
            sqlite3VtabUnlock(pVTab);
            i += 1;
        }
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            aVTrans as *mut ::core::ffi::c_void,
        );
        __db_ref.nVTrans = 0 as ::core::ffi::c_int;
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabSync(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *mut crate::src::headers::vdbeInt_h::Vdbe,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __db_ref = unsafe { &mut *db };
    let aVTrans: *mut *mut crate::src::headers::sqliteInt_h::VTable = __db_ref.aVTrans;
    __db_ref.aVTrans = ::core::ptr::null_mut::<*mut crate::src::headers::sqliteInt_h::VTable>();
    i = 0 as ::core::ffi::c_int;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < __db_ref.nVTrans {
        let x: Option<
            unsafe extern "C" fn(
                *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
            ) -> ::core::ffi::c_int,
        >;
        let pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab =
            (**aVTrans.offset(i as isize)).pVtab;
        if !pVtab.is_null() && {
            x = (*(*pVtab).pModule).xSync;
            x.is_some()
        } {
            rc = x.expect("non-null function pointer")(pVtab);
            crate::src::src::vdbeaux::sqlite3VtabImportErrmsg(
                p,
                pVtab as *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
            );
        }
        i += 1;
    }
    __db_ref.aVTrans = aVTrans;
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabRollback(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
) -> ::core::ffi::c_int {
    callFinaliser(db, 136 as ::core::ffi::c_ulong as ::core::ffi::c_int);
    crate::src::headers::sqlite3_h::SQLITE_OK
}
pub unsafe extern "C" fn sqlite3VtabCommit(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
) -> ::core::ffi::c_int {
    callFinaliser(db, 128 as ::core::ffi::c_ulong as ::core::ffi::c_int);
    crate::src::headers::sqlite3_h::SQLITE_OK
}
pub unsafe extern "C" fn sqlite3VtabBegin(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pVTab: *mut crate::src::headers::sqliteInt_h::VTable,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    
    if (*db).nVTrans > 0 as ::core::ffi::c_int && (*db).aVTrans.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_LOCKED;
    }
    if pVTab.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    let pModule: *const crate::src::headers::sqlite3_h::sqlite3_module = (*(*pVTab).pVtab).pModule;
    if (*pModule).xBegin.is_some() {
        let mut i: ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*db).nVTrans {
            if *(*db).aVTrans.offset(i as isize) == pVTab {
                return crate::src::headers::sqlite3_h::SQLITE_OK;
            }
            i += 1;
        }
        rc = growVTrans(db);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = (*pModule).xBegin.expect("non-null function pointer")((*pVTab).pVtab);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                let iSvpt: ::core::ffi::c_int = (*db).nStatement + (*db).nSavepoint;
                addToVTrans(db, pVTab);
                if iSvpt != 0 && (*pModule).xSavepoint.is_some() {
                    (*pVTab).iSavepoint = iSvpt;
                    rc = (*pModule).xSavepoint.expect("non-null function pointer")(
                        (*pVTab).pVtab,
                        iSvpt - 1 as ::core::ffi::c_int,
                    );
                }
            }
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabSavepoint(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    op: ::core::ffi::c_int,
    iSavepoint: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if !(*db).aVTrans.is_null() {
        let mut i: ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while rc == crate::src::headers::sqlite3_h::SQLITE_OK && i < (*db).nVTrans {
            let pVTab: *mut crate::src::headers::sqliteInt_h::VTable =
                *(*db).aVTrans.offset(i as isize);
            let pMod: *const crate::src::headers::sqlite3_h::sqlite3_module =
                (*(*pVTab).pMod).pModule;
            if !(*pVTab).pVtab.is_null() && (*pMod).iVersion >= 2 as ::core::ffi::c_int {
                let xMethod: Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >;
                sqlite3VtabLock(pVTab);
                match op {
                    crate::src::headers::sqliteInt_h::SAVEPOINT_BEGIN_1 => {
                        xMethod = (*pMod).xSavepoint;
                        (*pVTab).iSavepoint = iSavepoint + 1 as ::core::ffi::c_int;
                    }
                    crate::src::headers::sqliteInt_h::SAVEPOINT_ROLLBACK_1 => {
                        xMethod = (*pMod).xRollbackTo;
                    }
                    _ => {
                        xMethod = (*pMod).xRelease;
                    }
                }
                if xMethod.is_some() && (*pVTab).iSavepoint > iSavepoint {
                    let __db_ref = unsafe { &mut *db };
                    let savedFlags: crate::src::ext::rtree::rtree::U64_0 = __db_ref.flags
                        & crate::src::headers::sqliteInt_h::SQLITE_Defensive
                            as crate::src::ext::rtree::rtree::U64_0;
                    __db_ref.flags &= !(crate::src::headers::sqliteInt_h::SQLITE_Defensive
                        as crate::src::ext::rtree::rtree::U64_0);
                    rc = xMethod.expect("non-null function pointer")((*pVTab).pVtab, iSavepoint);
                    __db_ref.flags |= savedFlags;
                }
                sqlite3VtabUnlock(pVTab);
            }
            i += 1;
        }
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabOverloadFunction(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pDef: *mut crate::src::headers::sqliteInt_h::FuncDef,
    nArg: ::core::ffi::c_int,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::FuncDef {
    
    
    
    let mut xSFunc: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
            ::core::ffi::c_int,
            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
        ) -> (),
    > = None;
    let mut pArg: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    
    
    if pExpr.is_null() {
        return pDef;
    }
    if (*pExpr).op as ::core::ffi::c_int != crate::src::parse::TK_COLUMN {
        return pDef;
    }
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = (*pExpr).y.pTab;
    if pTab.is_null() {
        return pDef;
    }
    if ((*pTab).eTabType as ::core::ffi::c_int != crate::src::headers::sqliteInt_h::TABTYP_VTAB) {
        return pDef;
    }
    let pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab = (*sqlite3GetVTable(db, pTab)).pVtab;
    let pMod: *mut crate::src::headers::sqlite3_h::sqlite3_module = (*pVtab).pModule as *mut crate::src::headers::sqlite3_h::sqlite3_module;
    if (*pMod).xFindFunction.is_none() {
        return pDef;
    }
    let __pDef_ref = unsafe { &mut *pDef };
    let rc: ::core::ffi::c_int = (*pMod).xFindFunction.expect("non-null function pointer")(
        pVtab,
        nArg,
        __pDef_ref.zName,
        &raw mut xSFunc,
        &raw mut pArg,
    );
    if rc == 0 as ::core::ffi::c_int {
        return pDef;
    }
    let pNew: *mut crate::src::headers::sqliteInt_h::FuncDef = crate::src::src::malloc::sqlite3DbMallocZero(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        (::core::mem::size_of::<crate::src::headers::sqliteInt_h::FuncDef>() as usize)
            .wrapping_add(crate::src::src::util::sqlite3Strlen30(__pDef_ref.zName) as usize)
            .wrapping_add(1_usize) as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::headers::sqliteInt_h::FuncDef;
    if pNew.is_null() {
        return pDef;
    }
    *pNew = *pDef;
    (*pNew).zName = pNew.offset(1_isize) as *mut crate::src::headers::sqliteInt_h::FuncDef
        as *const ::core::ffi::c_char;
    ::core::ptr::copy_nonoverlapping(
        __pDef_ref.zName as *const u8,
        pNew.offset(1_isize) as *mut crate::src::headers::sqliteInt_h::FuncDef
            as *mut ::core::ffi::c_char as *mut u8,
        (crate::src::src::util::sqlite3Strlen30(__pDef_ref.zName) + 1 as ::core::ffi::c_int)
            as usize,
    );
    (*pNew).xSFunc = xSFunc;
    (*pNew).pUserData = pArg;
    (*pNew).funcFlags |=
        crate::src::headers::sqliteInt_h::SQLITE_FUNC_EPHEM as crate::src::ext::rtree::rtree::U32_0;
    pNew
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabMakeWritable(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pTab: *mut crate::src::headers::sqliteInt_h::Table,
) {
    let pToplevel: *mut crate::src::headers::sqliteInt_h::Parse =
        if !(*pParse).pToplevel.is_null() {
            (*pParse).pToplevel
        } else {
            pParse
        };
    let mut i: ::core::ffi::c_int;
    
    
    i = 0 as ::core::ffi::c_int;
    let __pToplevel_ref = unsafe { &mut *pToplevel };
    while i < __pToplevel_ref.nVtabLock {
        if pTab == *__pToplevel_ref.apVtabLock.offset(i as isize) {
            return;
        }
        i += 1;
    }
    let n: ::core::ffi::c_int = ((__pToplevel_ref.nVtabLock + 1 as ::core::ffi::c_int) as usize).wrapping_mul(
        ::core::mem::size_of::<*mut crate::src::headers::sqliteInt_h::Table>() as usize,
    ) as ::core::ffi::c_int;
    let apVtabLock: *mut *mut crate::src::headers::sqliteInt_h::Table = crate::src::src::malloc::sqlite3Realloc(
        __pToplevel_ref.apVtabLock as *mut ::core::ffi::c_void,
        n as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut *mut crate::src::headers::sqliteInt_h::Table;
    if !apVtabLock.is_null() {
        __pToplevel_ref.apVtabLock = apVtabLock;
        let fresh5 = __pToplevel_ref.nVtabLock;
        __pToplevel_ref.nVtabLock += 1;
        let fresh6 = &mut *__pToplevel_ref.apVtabLock.offset(fresh5 as isize);
        *fresh6 = pTab;
    } else {
        crate::src::src::malloc::sqlite3OomFault(
            __pToplevel_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        );
    };
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabEponymousTableInit(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pMod: *mut crate::src::headers::sqliteInt_h::Module,
) -> ::core::ffi::c_int {
    let __pMod_ref = unsafe { &mut *pMod };
    let pModule: *const crate::src::headers::sqlite3_h::sqlite3_module = __pMod_ref.pModule;
    
    let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    if !__pMod_ref.pEpoTab.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    if (*pModule).xCreate.is_some() && (*pModule).xCreate != (*pModule).xConnect {
        return 0 as ::core::ffi::c_int;
    }
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = crate::src::src::malloc::sqlite3DbMallocZero(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Table>()
            as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::headers::sqliteInt_h::Table;
    if pTab.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    (*pTab).zName = crate::src::src::malloc::sqlite3DbStrDup(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __pMod_ref.zName,
    );
    if (*pTab).zName.is_null() {
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pTab as *mut ::core::ffi::c_void,
        );
        return 0 as ::core::ffi::c_int;
    }
    __pMod_ref.pEpoTab = pTab;
    (*pTab).nTabRef = 1 as crate::src::ext::rtree::rtree::U32_0;
    (*pTab).eTabType =
        crate::src::headers::sqliteInt_h::TABTYP_VTAB as crate::src::ext::rtree::rtree::U8_0;
    let __db_ref = unsafe { &mut *db };
    (*pTab).pSchema = (*__db_ref.aDb.offset(0_isize)).pSchema;
    (*pTab).iPKey = -(1 as ::core::ffi::c_int) as crate::src::fts5::I16_0;
    (*pTab).tabFlags |=
        crate::src::headers::sqliteInt_h::TF_Eponymous as crate::src::ext::rtree::rtree::U32_0;
    addModuleArgument(
        pParse,
        pTab,
        crate::src::src::malloc::sqlite3DbStrDup(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*pTab).zName,
        ),
    );
    addModuleArgument(pParse, pTab, ::core::ptr::null_mut::<::core::ffi::c_char>());
    addModuleArgument(
        pParse,
        pTab,
        crate::src::src::malloc::sqlite3DbStrDup(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*pTab).zName,
        ),
    );
    __db_ref.nSchemaLock = __db_ref.nSchemaLock.wrapping_add(1);
    let rc: ::core::ffi::c_int = vtabCallConstructor(db, pTab, pMod, (*pModule).xConnect, &raw mut zErr);
    __db_ref.nSchemaLock = __db_ref.nSchemaLock.wrapping_sub(1);
    if rc != 0 {
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Str(zErr)],
        );
        (*pParse).rc = rc;
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            zErr as *mut ::core::ffi::c_void,
        );
        sqlite3VtabEponymousTableClear(db, pMod);
    }
    1 as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VtabEponymousTableClear(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pMod: *mut crate::src::headers::sqliteInt_h::Module,
) {
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = (*pMod).pEpoTab;
    if !pTab.is_null() {
        (*pTab).tabFlags |=
            crate::src::headers::sqliteInt_h::TF_Ephemeral as crate::src::ext::rtree::rtree::U32_0;
        crate::src::src::build::sqlite3DeleteTable(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pTab as *mut crate::src::headers::sqliteInt_h::Table,
        );
        (*pMod).pEpoTab = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_vtab_on_conflict(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
) -> ::core::ffi::c_int {
    static mut aMap: [::core::ffi::c_uchar; 5] = [
        crate::src::headers::sqlite3_h::SQLITE_ROLLBACK as ::core::ffi::c_uchar,
        crate::src::headers::sqlite3_h::SQLITE_ABORT as ::core::ffi::c_uchar,
        crate::src::headers::sqlite3_h::SQLITE_FAIL as ::core::ffi::c_uchar,
        crate::src::headers::sqlite3_h::SQLITE_IGNORE as ::core::ffi::c_uchar,
        crate::src::headers::sqlite3_h::SQLITE_REPLACE as ::core::ffi::c_uchar,
    ];
    aMap[((*db).vtabOnConflict as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int
}

// sqlite3_vtab_config — C wrapper is in c_code/vtab_config.c
pub use crate::src::printf_c_variadic::rs_vtab_config_dispatch as sqlite3_vtab_config_args;
