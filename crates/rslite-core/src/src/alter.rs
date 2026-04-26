pub use crate::__stddef_size_t_h::SizeT;

pub use crate::src::headers::btreeInt_h::Btree;
pub use crate::src::headers::opcodes_h::OP_AddImm;
pub use crate::src::headers::opcodes_h::OP_Column;
pub use crate::src::headers::opcodes_h::OP_IdxInsert;
pub use crate::src::headers::opcodes_h::OP_IfPos;
pub use crate::src::headers::opcodes_h::OP_Insert;
pub use crate::src::headers::opcodes_h::OP_MakeRecord;
pub use crate::src::headers::opcodes_h::OP_Next;
pub use crate::src::headers::opcodes_h::OP_Null;
pub use crate::src::headers::opcodes_h::OP_OpenWrite;
pub use crate::src::headers::opcodes_h::OP_ReadCookie;
pub use crate::src::headers::opcodes_h::OP_Rewind;
pub use crate::src::headers::opcodes_h::OP_Rowid;
pub use crate::src::headers::opcodes_h::OP_SetCookie;
pub use crate::src::headers::opcodes_h::OP_VRename;
pub use crate::src::src::btmutex::sqlite3BtreeEnterAll;
pub use crate::src::src::btmutex::sqlite3BtreeLeaveAll;
pub use crate::src::src::btree::BTREE_FILE_FORMAT;
pub use crate::src::src::hash::_ht;
pub use crate::src::src::hash::Hash;
pub use crate::src::src::hash::HashElem;
pub use crate::src::src::pager::Pgno;

pub use crate::src::ext::rtree::rtree::I64_0;
pub use crate::src::ext::rtree::rtree::U8_0;
pub use crate::src::ext::rtree::rtree::U32_0;
pub use crate::src::ext::rtree::rtree::U64_0;
pub use crate::src::fts5::I16_0;
pub use crate::src::fts5::U16_0;
pub use crate::src::headers::sqlite3_h::SQLITE_ALTER_TABLE;
pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;
pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SQLITE_UTF8;
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
pub use crate::src::headers::sqliteInt_h::__anon_union_14;
pub use crate::src::headers::sqliteInt_h::__anon_union_15;
pub use crate::src::headers::sqliteInt_h::__anon_union_16;
pub use crate::src::headers::sqliteInt_h::AggInfo;
pub use crate::src::headers::sqliteInt_h::AggInfo_col;
pub use crate::src::headers::sqliteInt_h::AggInfo_func;
pub use crate::src::headers::sqliteInt_h::AutoincInfo;
pub use crate::src::headers::sqliteInt_h::Bitmask;
pub use crate::src::headers::sqliteInt_h::BusyHandler;
pub use crate::src::headers::sqliteInt_h::CCurHint;
pub use crate::src::headers::sqliteInt_h::COLFLAG_GENERATED;
pub use crate::src::headers::sqliteInt_h::COLFLAG_PRIMKEY;
pub use crate::src::headers::sqliteInt_h::COLFLAG_STORED;
pub use crate::src::headers::sqliteInt_h::COLFLAG_UNIQUE;
pub use crate::src::headers::sqliteInt_h::COLFLAG_VIRTUAL;
pub use crate::src::headers::sqliteInt_h::CheckOnCtx;
pub use crate::src::headers::sqliteInt_h::CollSeq;
pub use crate::src::headers::sqliteInt_h::Column;
pub use crate::src::headers::sqliteInt_h::CoveringIndexCheck;
pub use crate::src::headers::sqliteInt_h::Cte;
pub use crate::src::headers::sqliteInt_h::CteUse;
pub use crate::src::headers::sqliteInt_h::Db;
pub use crate::src::headers::sqliteInt_h::DbClientData;
pub use crate::src::headers::sqliteInt_h::DbFixer;
pub use crate::src::headers::sqliteInt_h::ENAME_NAME;
pub use crate::src::headers::sqliteInt_h::ENAME_SPAN;
pub use crate::src::headers::sqliteInt_h::EP_DblQuoted;
pub use crate::src::headers::sqliteInt_h::EP_Subrtn;
pub use crate::src::headers::sqliteInt_h::EP_WinFunc;
pub use crate::src::headers::sqliteInt_h::Expr;
pub use crate::src::headers::sqliteInt_h::ExprList;
pub use crate::src::headers::sqliteInt_h::ExprList_item;
pub use crate::src::headers::sqliteInt_h::FKey;
pub use crate::src::headers::sqliteInt_h::FuncDef;
pub use crate::src::headers::sqliteInt_h::FuncDestructor;
pub use crate::src::headers::sqliteInt_h::INITFLAG_AlterAdd;
pub use crate::src::headers::sqliteInt_h::INITFLAG_AlterDrop;
pub use crate::src::headers::sqliteInt_h::INITFLAG_AlterRename;
pub use crate::src::headers::sqliteInt_h::IdList;
pub use crate::src::headers::sqliteInt_h::IdList_item;
pub use crate::src::headers::sqliteInt_h::IdxCover;
pub use crate::src::headers::sqliteInt_h::Index;
pub use crate::src::headers::sqliteInt_h::IndexedExpr;
pub use crate::src::headers::sqliteInt_h::KeyInfo;
pub use crate::src::headers::sqliteInt_h::LogEst;
pub use crate::src::headers::sqliteInt_h::Lookaside;
pub use crate::src::headers::sqliteInt_h::LookasideSlot;
pub use crate::src::headers::sqliteInt_h::Module;
pub use crate::src::headers::sqliteInt_h::NC_UUpsert;
pub use crate::src::headers::sqliteInt_h::NameContext;
pub use crate::src::headers::sqliteInt_h::OPFLAG_SAVEPOSITION;
pub use crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME;
pub use crate::src::headers::sqliteInt_h::PARSE_MODE_UNMAP;
pub use crate::src::headers::sqliteInt_h::Parse;
pub use crate::src::headers::sqliteInt_h::ParseCleanup;
pub use crate::src::headers::sqliteInt_h::RefSrcList;
pub use crate::src::headers::sqliteInt_h::Returning;
pub use crate::src::headers::sqliteInt_h::SF_CopyCte;
pub use crate::src::headers::sqliteInt_h::SF_Expanded;
pub use crate::src::headers::sqliteInt_h::SF_View;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_REAL;
pub use crate::src::headers::sqliteInt_h::SQLITE_Comments;
pub use crate::src::headers::sqliteInt_h::SQLITE_DqsDDL;
pub use crate::src::headers::sqliteInt_h::SQLITE_DqsDML;
pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN;
pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT;
pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_INTERNAL;
pub use crate::src::headers::sqliteInt_h::SQLITE_ForeignKeys;
pub use crate::src::headers::sqliteInt_h::SQLITE_LegacyAlter;
pub use crate::src::headers::sqliteInt_h::Savepoint;
pub use crate::src::headers::sqliteInt_h::Schema;
pub use crate::src::headers::sqliteInt_h::Select;
pub use crate::src::headers::sqliteInt_h::SrcItem;
pub use crate::src::headers::sqliteInt_h::SrcList;
pub use crate::src::headers::sqliteInt_h::Subquery;
pub use crate::src::headers::sqliteInt_h::TABTYP_NORM;
pub use crate::src::headers::sqliteInt_h::TABTYP_VIEW;
pub use crate::src::headers::sqliteInt_h::TABTYP_VTAB;
pub use crate::src::headers::sqliteInt_h::TF_Eponymous;
pub use crate::src::headers::sqliteInt_h::TF_Shadow;
pub use crate::src::headers::sqliteInt_h::TF_Strict;
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
pub use crate::src::headers::sqliteInt_h::VtabCtx;
pub use crate::src::headers::sqliteInt_h::WRC_Abort;
pub use crate::src::headers::sqliteInt_h::WRC_Continue;
pub use crate::src::headers::sqliteInt_h::WRC_Prune;
pub use crate::src::headers::sqliteInt_h::Walker;
pub use crate::src::headers::sqliteInt_h::WhereConst;
pub use crate::src::headers::sqliteInt_h::Window;
pub use crate::src::headers::sqliteInt_h::WindowRewrite;
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
pub use crate::src::parse::TK_NULL;
pub use crate::src::parse::TK_STRING;
pub use crate::src::parse::TK_TRIGGER;
pub use crate::src::src::auth::sqlite3AuthCheck;
pub use crate::src::src::build::sqlite3ChangeCookie;
pub use crate::src::src::build::sqlite3CheckObjectName;
pub use crate::src::src::build::sqlite3ColumnExpr;
pub use crate::src::src::build::sqlite3DeleteTable;
pub use crate::src::src::build::sqlite3FindDbName;
pub use crate::src::src::build::sqlite3FindIndex;
pub use crate::src::src::build::sqlite3FindTable;
pub use crate::src::src::build::sqlite3FreeIndex;
pub use crate::src::src::build::sqlite3IsShadowTableOf;
pub use crate::src::src::build::sqlite3LocateTable;
pub use crate::src::src::build::sqlite3LocateTableItem;
pub use crate::src::src::build::sqlite3MayAbort;
pub use crate::src::src::build::sqlite3NameFromToken;
pub use crate::src::src::build::sqlite3PrimaryKeyIndex;
pub use crate::src::src::build::sqlite3ReadOnlyShadowTables;
pub use crate::src::src::build::sqlite3SrcListDelete;
pub use crate::src::src::build::sqlite3TableColumnToIndex;
pub use crate::src::src::build::sqlite3ViewGetColumnNames;
pub use crate::src::src::build::sqlite3WritableSchema;
pub use crate::src::src::callback::sqlite3InsertBuiltinFuncs;
pub use crate::src::src::expr::sqlite3ExprCodeGetColumnOfTable;
pub use crate::src::src::expr::sqlite3ExprListDup;
pub use crate::src::src::expr::sqlite3GetTempReg;
pub use crate::src::src::expr::sqlite3ReleaseTempReg;
pub use crate::src::src::expr::sqlite3WithDup;
pub use crate::src::src::global::sqlite3CtypeMap;
pub use crate::src::src::insert::sqlite3OpenTable;
pub use crate::src::src::main::sqlite3CorruptError;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3DbFree;
pub use crate::src::src::malloc::sqlite3DbMallocZero;
pub use crate::src::src::malloc::sqlite3DbStrDup;
pub use crate::src::src::malloc::sqlite3DbStrNDup;
pub use crate::src::src::mutex_unix::sqlite3_mutex;
pub use crate::src::src::prepare::sqlite3ParseObjectInit;
pub use crate::src::src::prepare::sqlite3ParseObjectReset;
pub use crate::src::src::prepare::sqlite3SchemaToIndex;
pub use crate::src::src::resolve::sqlite3ResolveExprListNames;
pub use crate::src::src::resolve::sqlite3ResolveExprNames;
pub use crate::src::src::select::sqlite3ColumnIndex;
pub use crate::src::src::select::sqlite3GetVdbe;
pub use crate::src::src::select::sqlite3SelectDelete;
pub use crate::src::src::select::sqlite3SelectNew;
pub use crate::src::src::select::sqlite3SelectPrep;
pub use crate::src::src::select::sqlite3WithPush;
pub use crate::src::src::tokenize::sqlite3IsIdChar;
pub use crate::src::src::tokenize::sqlite3RunParser;
pub use crate::src::src::trigger::sqlite3DeleteTrigger;
pub use crate::src::src::trigger::sqlite3TriggerStepSrc;
pub use crate::src::src::utf::sqlite3Utf8CharLen;
pub use crate::src::src::util::sqlite3_stricmp;
pub use crate::src::src::util::sqlite3_strnicmp;
pub use crate::src::src::util::sqlite3Dequote;
pub use crate::src::src::util::sqlite3StrIHash;
pub use crate::src::src::util::sqlite3Strlen30;
pub use crate::src::src::vdbeapi::sqlite3_context_db_handle;
pub use crate::src::src::vdbeapi::sqlite3_result_error;
pub use crate::src::src::vdbeapi::sqlite3_result_error_code;
pub use crate::src::src::vdbeapi::sqlite3_result_int;
pub use crate::src::src::vdbeapi::sqlite3_result_text;
pub use crate::src::src::vdbeapi::sqlite3_result_value;
pub use crate::src::src::vdbeapi::sqlite3_value_int;
pub use crate::src::src::vdbeapi::sqlite3_value_text;
pub use crate::src::src::vdbemem::sqlite3ValueFree;
pub use crate::src::src::vdbemem::sqlite3ValueFromExpr;
pub use crate::src::src::vtab::sqlite3GetVTable;
pub use crate::src::src::walker::sqlite3WalkExpr;
pub use crate::src::src::walker::sqlite3WalkExprList;
pub use crate::src::src::walker::sqlite3WalkSelect;

pub use crate::src::headers::stdlib::Uint8T;
pub use crate::src::headers::stdlib::Uint16T;
pub use crate::src::headers::stdlib::Uint32T;
pub use crate::src::headers::vdbeInt_h::Vdbe;
pub use crate::src::src::vdbe::Mem;
pub use crate::src::src::vdbe::P4_VTAB;
pub use crate::src::src::vdbe::SubProgram;
pub use crate::src::src::vdbe::SubrtnSig;
pub use crate::src::src::vdbe::VdbeOp;
pub use crate::src::src::vdbe::p4union;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp1;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp2;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp3;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp4;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddParseSchemaOp;
pub use crate::src::src::vdbeaux::sqlite3VdbeChangeP5;
pub use crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr;
pub use crate::src::src::vdbeaux::sqlite3VdbeFinalize;
pub use crate::src::src::vdbeaux::sqlite3VdbeJumpHere;
pub use crate::src::src::vdbeaux::sqlite3VdbeLoadString;
pub use crate::src::src::vdbeaux::sqlite3VdbeUsesBtree;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenameToken {
    pub p: *const ::core::ffi::c_void,
    pub t: crate::src::headers::sqliteInt_h::Token,
    pub pNext: *mut RenameToken,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenameCtx {
    pub pList: *mut RenameToken,
    pub nList: ::core::ffi::c_int,
    pub iCol: ::core::ffi::c_int,
    pub pTab: *mut crate::src::headers::sqliteInt_h::Table,
    pub zOld: *const ::core::ffi::c_char,
}

unsafe extern "C" fn isAlterableTable(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pTab: *mut crate::src::headers::sqliteInt_h::Table,
) -> ::core::ffi::c_int {
    let __pTab_ref = { &*pTab };
    if 0 as ::core::ffi::c_int
        == crate::src::src::util::sqlite3_strnicmp(
            __pTab_ref.zName,
            b"sqlite_\0" as *const u8 as *const ::core::ffi::c_char,
            7 as ::core::ffi::c_int,
        )
        || __pTab_ref.tabFlags
            & crate::src::headers::sqliteInt_h::TF_Eponymous as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
        || __pTab_ref.tabFlags
            & crate::src::headers::sqliteInt_h::TF_Shadow as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
            && crate::src::src::build::sqlite3ReadOnlyShadowTables(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            ) != 0
    {
        crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"table %s may not be altered\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Str(
                __pTab_ref.zName as *mut ::core::ffi::c_char,
            )],
        );
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn renameTestSchema(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    zDb: *const ::core::ffi::c_char,
    bTemp: ::core::ffi::c_int,
    zWhen: *const ::core::ffi::c_char,
    bNoDQS: ::core::ffi::c_int,
) {
    (*pParse).set_colNamesSet(
        1 as crate::src::headers::sqliteInt_h::Bft as crate::src::headers::sqliteInt_h::Bft,
    );
    crate::src::src::build::sqlite3NestedParse_args(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        "SELECT 1 FROM \"%w\".sqlite_master WHERE name NOT LIKE 'sqliteX_%%' ESCAPE 'X' AND sql NOT LIKE 'create virtual%%' AND sqlite_rename_test(%Q, sql, type, name, %d, %Q, %d)=NULL ",
        crate::printf_args_slice!(zDb, zDb, bTemp, zWhen, bNoDQS),
    );
    if bTemp == 0 as ::core::ffi::c_int {
        crate::src::src::build::sqlite3NestedParse_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            "SELECT 1 FROM temp.sqlite_master WHERE name NOT LIKE 'sqliteX_%%' ESCAPE 'X' AND sql NOT LIKE 'create virtual%%' AND sqlite_rename_test(%Q, sql, type, name, 1, %Q, %d)=NULL ",
            crate::printf_args_slice!(zDb, zWhen, bNoDQS),
        );
    }
}

unsafe extern "C" fn renameFixQuotes(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    zDb: *const ::core::ffi::c_char,
    bTemp: ::core::ffi::c_int,
) {
    crate::src::src::build::sqlite3NestedParse_args(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        "UPDATE \"%w\".sqlite_master SET sql = sqlite_rename_quotefix(%Q, sql)WHERE name NOT LIKE 'sqliteX_%%' ESCAPE 'X' AND sql NOT LIKE 'create virtual%%'",
        crate::printf_args_slice!(zDb, zDb),
    );
    if bTemp == 0 as ::core::ffi::c_int {
        crate::src::src::build::sqlite3NestedParse_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            "UPDATE temp.sqlite_master SET sql = sqlite_rename_quotefix('temp', sql)WHERE name NOT LIKE 'sqliteX_%%' ESCAPE 'X' AND sql NOT LIKE 'create virtual%%'",
            crate::printf_args_slice!(),
        );
    }
}

unsafe extern "C" fn renameReloadSchema(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    iDb: ::core::ffi::c_int,
    p5: crate::src::fts5::U16_0,
) {
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    if !v.is_null() {
        crate::src::src::build::sqlite3ChangeCookie(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            iDb,
        );
        crate::src::src::vdbeaux::sqlite3VdbeAddParseSchemaOp(
            (*pParse).pVdbe,
            iDb,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            p5,
        );
        if iDb != 1 as ::core::ffi::c_int {
            crate::src::src::vdbeaux::sqlite3VdbeAddParseSchemaOp(
                (*pParse).pVdbe,
                1 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                p5,
            );
        }
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3AlterRenameTable(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pSrc: *mut crate::src::headers::sqliteInt_h::SrcList,
    pName: *mut crate::src::headers::sqliteInt_h::Token,
) {
    let iDb: ::core::ffi::c_int;
    let zDb: *mut ::core::ffi::c_char;
    let pTab: *mut crate::src::headers::sqliteInt_h::Table;
    let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    let nTabName: ::core::ffi::c_int;
    let zTabName: *const ::core::ffi::c_char;
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe;
    let mut pVTab: *mut crate::src::headers::sqliteInt_h::VTable =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::VTable>();
    if ((*db).mallocFailed == 0) {
        pTab = crate::src::src::build::sqlite3LocateTableItem(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            0 as crate::src::ext::rtree::rtree::U32_0,
            (&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                .offset(0_isize) as *mut crate::src::headers::sqliteInt_h::SrcItem
                as *mut crate::src::headers::sqliteInt_h::SrcItem,
        ) as *mut crate::src::headers::sqliteInt_h::Table;
        if !pTab.is_null() {
            iDb = crate::src::src::prepare::sqlite3SchemaToIndex(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*pTab).pSchema as *mut crate::src::headers::sqliteInt_h::Schema,
            );
            zDb = (*(*db).aDb.offset(iDb as isize)).zDbSName;
            zName = crate::src::src::build::sqlite3NameFromToken(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                pName as *const crate::src::headers::sqliteInt_h::Token,
            );
            if !zName.is_null() {
                if !(crate::src::src::build::sqlite3FindTable(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    zName,
                    zDb,
                ) as *mut crate::src::headers::sqliteInt_h::Table)
                    .is_null()
                    || !(crate::src::src::build::sqlite3FindIndex(
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        zName,
                        zDb,
                    ) as *mut crate::src::headers::sqliteInt_h::Index)
                        .is_null()
                    || crate::src::src::build::sqlite3IsShadowTableOf(
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        pTab as *mut crate::src::headers::sqliteInt_h::Table,
                        zName,
                    ) != 0
                {
                    crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        b"there is already another table or index with this name: %s\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[crate::src::src::printf::PrintfArg::Str(
                            zName as *mut ::core::ffi::c_char,
                        )],
                    );
                } else if (crate::src::headers::sqlite3_h::SQLITE_OK == isAlterableTable(pParse, pTab))
                    && (crate::src::headers::sqlite3_h::SQLITE_OK == crate::src::src::build::sqlite3CheckObjectName(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            zName,
                            b"table\0" as *const u8 as *const ::core::ffi::c_char,
                            zName,
                        ))
                    {
                        if (*pTab).eTabType as ::core::ffi::c_int
                            == crate::src::headers::sqliteInt_h::TABTYP_VIEW
                        {
                            crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                b"view %s may not be altered\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[crate::src::src::printf::PrintfArg::Str(
                                    (*pTab).zName as *mut ::core::ffi::c_char,
                                )],
                            );
                        } else if (crate::src::src::auth::sqlite3AuthCheck(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            crate::src::headers::sqlite3_h::SQLITE_ALTER_TABLE,
                            zDb,
                            (*pTab).zName,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        ) == 0)
                            && (crate::src::src::build::sqlite3ViewGetColumnNames(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                pTab as *mut crate::src::headers::sqliteInt_h::Table,
                            ) == 0)
                            {
                                if (*pTab).eTabType as ::core::ffi::c_int
                                    == crate::src::headers::sqliteInt_h::TABTYP_VTAB
                                {
                                    pVTab = crate::src::src::vtab::sqlite3GetVTable(
                                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                        pTab as *mut crate::src::headers::sqliteInt_h::Table,
                                    )
                                        as *mut crate::src::headers::sqliteInt_h::VTable;
                                    if (*(*(*pVTab).pVtab).pModule).xRename.is_none() {
                                        pVTab = ::core::ptr::null_mut::<
                                            crate::src::headers::sqliteInt_h::VTable,
                                        >();
                                    }
                                }
                                v = crate::src::src::select::sqlite3GetVdbe(
                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                );
                                if !v.is_null() {
                                    crate::src::src::build::sqlite3MayAbort(
                                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                    );
                                    zTabName = (*pTab).zName;
                                    nTabName = crate::src::src::utf::sqlite3Utf8CharLen(
                                        zTabName,
                                        -(1 as ::core::ffi::c_int),
                                    );
                                    crate::src::src::build::sqlite3NestedParse_args(
                                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                        "UPDATE \"%w\".sqlite_master SET sql = sqlite_rename_table(%Q, type, name, sql, %Q, %Q, %d) WHERE (type!='index' OR tbl_name=%Q COLLATE nocase)AND   name NOT LIKE 'sqliteX_%%' ESCAPE 'X'",
                                        crate::printf_args_slice!(
                                            zDb,
                                            zDb,
                                            zTabName,
                                            zName,
                                            (iDb == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                            zTabName
                                        ),
                                    );
                                    crate::src::src::build::sqlite3NestedParse_args(
                                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                        "UPDATE %Q.sqlite_master SET tbl_name = %Q, name = CASE WHEN type='table' THEN %Q WHEN name LIKE 'sqliteX_autoindex%%' ESCAPE 'X'      AND type='index' THEN 'sqlite_autoindex_' || %Q || substr(name,%d+18) ELSE name END WHERE tbl_name=%Q COLLATE nocase AND (type='table' OR type='index' OR type='trigger');",
                                        crate::printf_args_slice!(
                                            zDb, zName, zName, zName, nTabName, zTabName
                                        ),
                                    );
                                    if !(crate::src::src::build::sqlite3FindTable(
                                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                        b"sqlite_sequence\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        zDb,
                                    )
                                        as *mut crate::src::headers::sqliteInt_h::Table)
                                        .is_null()
                                    {
                                        crate::src::src::build::sqlite3NestedParse_args(
                                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                            "UPDATE \"%w\".sqlite_sequence set name = %Q WHERE name = %Q",
                                            crate::printf_args_slice!(zDb, zName, (*pTab).zName),
                                        );
                                    }
                                    if iDb != 1 as ::core::ffi::c_int {
                                        crate::src::src::build::sqlite3NestedParse_args(
                                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                            "UPDATE sqlite_temp_schema SET sql = sqlite_rename_table(%Q, type, name, sql, %Q, %Q, 1), tbl_name = CASE WHEN tbl_name=%Q COLLATE nocase AND   sqlite_rename_test(%Q, sql, type, name, 1, 'after rename', 0) THEN %Q ELSE tbl_name END WHERE type IN ('view', 'trigger')",
                                            crate::printf_args_slice!(
                                                zDb, zTabName, zName, zTabName, zDb, zName
                                            ),
                                        );
                                    }
                                    if !pVTab.is_null() {
                                        (*pParse).nMem += 1;
                                        let i: ::core::ffi::c_int = (*pParse).nMem;
                                        crate::src::src::vdbeaux::sqlite3VdbeLoadString(
                                            v, i, zName,
                                        );
                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                            v,
                                            crate::src::headers::opcodes_h::OP_VRename,
                                            i,
                                            0 as ::core::ffi::c_int,
                                            0 as ::core::ffi::c_int,
                                            pVTab as *const ::core::ffi::c_char,
                                            crate::src::src::vdbe::P4_VTAB,
                                        );
                                    }
                                    renameReloadSchema(
                                        pParse,
                                        iDb,
                                        crate::src::headers::sqliteInt_h::INITFLAG_AlterRename
                                            as crate::src::fts5::U16_0,
                                    );
                                    renameTestSchema(
                                        pParse,
                                        zDb,
                                        (iDb == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                        b"after rename\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        0 as ::core::ffi::c_int,
                                    );
                                }
                            }
                    }
            }
        }
    }
    crate::src::src::build::sqlite3SrcListDelete(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        pSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
    );
    crate::src::src::malloc::sqlite3DbFree(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        zName as *mut ::core::ffi::c_void,
    );
}

unsafe extern "C" fn sqlite3ErrorIfNotEmpty(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    zDb: *const ::core::ffi::c_char,
    zTab: *const ::core::ffi::c_char,
    zErr: *const ::core::ffi::c_char,
) {
    crate::src::src::build::sqlite3NestedParse_args(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        "SELECT raise(ABORT,%Q) FROM \"%w\".\"%w\"",
        crate::printf_args_slice!(zErr, zDb, zTab),
    );
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3AlterFinishAddColumn(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pColDef: *mut crate::src::headers::sqliteInt_h::Token,
) {
    
    
    
    
    
    
    
    let mut pDflt: *mut crate::src::headers::sqliteInt_h::Expr;
    
    
    let r1: ::core::ffi::c_int;
    let __pParse_ref = { &*pParse };
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
    if __pParse_ref.nErr != 0 {
        return;
    }
    let pNew: *mut crate::src::headers::sqliteInt_h::Table = __pParse_ref.pNewTable;
    let iDb: ::core::ffi::c_int = crate::src::src::prepare::sqlite3SchemaToIndex(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        (*pNew).pSchema as *mut crate::src::headers::sqliteInt_h::Schema,
    );
    let zDb: *const ::core::ffi::c_char = (*(*db).aDb.offset(iDb as isize)).zDbSName;
    let zTab: *const ::core::ffi::c_char = (*pNew).zName.offset(16_isize) as *mut ::core::ffi::c_char;
    let pCol: *mut crate::src::headers::sqliteInt_h::Column = (*pNew)
        .aCol
        .offset(((*pNew).nCol as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
        as *mut crate::src::headers::sqliteInt_h::Column;
    pDflt = crate::src::src::build::sqlite3ColumnExpr(
        pNew as *mut crate::src::headers::sqliteInt_h::Table,
        pCol as *mut crate::src::headers::sqliteInt_h::Column,
    ) as *mut crate::src::headers::sqliteInt_h::Expr;
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = crate::src::src::build::sqlite3FindTable(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        zTab,
        zDb,
    ) as *mut crate::src::headers::sqliteInt_h::Table;
    if crate::src::src::auth::sqlite3AuthCheck(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        crate::src::headers::sqlite3_h::SQLITE_ALTER_TABLE,
        zDb,
        (*pTab).zName,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ) != 0
    {
        return;
    }
    if (*pCol).colFlags as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::COLFLAG_PRIMKEY
        != 0
    {
        crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"Cannot add a PRIMARY KEY column\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
        return;
    }
    if !(*pNew).pIndex.is_null() {
        crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"Cannot add a UNIQUE column\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
        return;
    }
    if (*pCol).colFlags as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::COLFLAG_GENERATED
        == 0 as ::core::ffi::c_int
    {
        if !pDflt.is_null()
            && (*(*pDflt).pLeft).op as ::core::ffi::c_int == crate::src::parse::TK_NULL
        {
            pDflt = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
        }
        if (*db).flags
            & crate::src::headers::sqliteInt_h::SQLITE_ForeignKeys
                as crate::src::ext::rtree::rtree::U64_0
            != 0
            && !(*pNew).u.tab.pFKey.is_null()
            && !pDflt.is_null()
        {
            sqlite3ErrorIfNotEmpty(
                pParse,
                zDb,
                zTab,
                b"Cannot add a REFERENCES column with non-NULL default value\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if (*pCol).notNull() as ::core::ffi::c_int != 0 && pDflt.is_null() {
            sqlite3ErrorIfNotEmpty(
                pParse,
                zDb,
                zTab,
                b"Cannot add a NOT NULL column with default value NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if !pDflt.is_null() {
            let mut pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
                ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
            
            let rc: ::core::ffi::c_int = crate::src::src::vdbemem::sqlite3ValueFromExpr(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                pDflt as *const crate::src::headers::sqliteInt_h::Expr,
                crate::src::headers::sqlite3_h::SQLITE_UTF8 as crate::src::ext::rtree::rtree::U8_0,
                crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB
                    as crate::src::ext::rtree::rtree::U8_0,
                &raw mut pVal,
            );
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return;
            }
            if pVal.is_null() {
                sqlite3ErrorIfNotEmpty(
                    pParse,
                    zDb,
                    zTab,
                    b"Cannot add a column with non-constant default\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            crate::src::src::vdbemem::sqlite3ValueFree(pVal);
        }
    } else if (*pCol).colFlags as ::core::ffi::c_int
        & crate::src::headers::sqliteInt_h::COLFLAG_STORED
        != 0
    {
        sqlite3ErrorIfNotEmpty(
            pParse,
            zDb,
            zTab,
            b"cannot add a STORED column\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let zCol: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3DbStrNDup(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        (*pColDef).z as *mut ::core::ffi::c_char,
        (*pColDef).n as crate::src::ext::rtree::rtree::U64_0,
    );
    if !zCol.is_null() {
        let mut zEnd: *mut ::core::ffi::c_char = zCol
            .offset((*pColDef).n.wrapping_sub(1 as ::core::ffi::c_uint) as isize)
            as *mut ::core::ffi::c_char;
        while zEnd > zCol
            && (*zEnd as ::core::ffi::c_int == ';' as i32
                || *(&raw const crate::src::src::global::sqlite3CtypeMap
                    as *const ::core::ffi::c_uchar)
                    .offset(*zEnd as ::core::ffi::c_uchar as isize)
                    as ::core::ffi::c_int
                    & 0x1 as ::core::ffi::c_int
                    != 0)
        {
            let fresh1 = zEnd;
            zEnd = zEnd.offset(-1);
            *fresh1 = '\0' as i32 as ::core::ffi::c_char;
        }
        crate::src::src::build::sqlite3NestedParse_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            "UPDATE \"%w\".sqlite_master SET sql = printf('%%.%ds, ',sql) || %Q || substr(sql,1+length(printf('%%.%ds',sql))) WHERE type = 'table' AND name = %Q",
            crate::printf_args_slice!(
                zDb,
                (*pNew).u.tab.addColOffset,
                zCol,
                (*pNew).u.tab.addColOffset,
                zTab
            ),
        );
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            zCol as *mut ::core::ffi::c_void,
        );
    }
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = crate::src::src::select::sqlite3GetVdbe(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
    );
    if !v.is_null() {
        r1 = crate::src::src::expr::sqlite3GetTempReg(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        );
        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
            v,
            crate::src::headers::opcodes_h::OP_ReadCookie,
            iDb,
            r1,
            crate::src::src::btree::BTREE_FILE_FORMAT,
        );
        crate::src::src::vdbeaux::sqlite3VdbeUsesBtree(v, iDb);
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_AddImm,
            r1,
            -(2 as ::core::ffi::c_int),
        );
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_IfPos,
            r1,
            crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
        );
        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
            v,
            crate::src::headers::opcodes_h::OP_SetCookie,
            iDb,
            crate::src::src::btree::BTREE_FILE_FORMAT,
            3 as ::core::ffi::c_int,
        );
        crate::src::src::expr::sqlite3ReleaseTempReg(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            r1,
        );
        renameReloadSchema(
            pParse,
            iDb,
            crate::src::headers::sqliteInt_h::INITFLAG_AlterAdd as crate::src::fts5::U16_0,
        );
        if !(*pNew).pCheck.is_null()
            || (*pCol).notNull() as ::core::ffi::c_int != 0
                && (*pCol).colFlags as ::core::ffi::c_int
                    & crate::src::headers::sqliteInt_h::COLFLAG_GENERATED
                    != 0 as ::core::ffi::c_int
            || (*pTab).tabFlags
                & crate::src::headers::sqliteInt_h::TF_Strict
                    as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
        {
            crate::src::src::build::sqlite3NestedParse_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                "SELECT CASE WHEN quick_check GLOB 'CHECK*' THEN raise(ABORT,'CHECK constraint failed') WHEN quick_check GLOB 'non-* value in*' THEN raise(ABORT,'type mismatch on DEFAULT') ELSE raise(ABORT,'NOT NULL constraint failed') END  FROM pragma_quick_check(%Q,%Q) WHERE quick_check GLOB 'CHECK*' OR quick_check GLOB 'NULL*' OR quick_check GLOB 'non-* value in*'",
                crate::printf_args_slice!(zTab, zDb),
            );
        }
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3AlterBeginAddColumn(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pSrc: *mut crate::src::headers::sqliteInt_h::SrcList,
) {
    let pNew: *mut crate::src::headers::sqliteInt_h::Table;
    let pTab: *mut crate::src::headers::sqliteInt_h::Table;
    let iDb: ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    let nAlloc: ::core::ffi::c_int;
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    if ((*db).mallocFailed == 0) {
        pTab = crate::src::src::build::sqlite3LocateTableItem(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            0 as crate::src::ext::rtree::rtree::U32_0,
            (&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                .offset(0_isize) as *mut crate::src::headers::sqliteInt_h::SrcItem
                as *mut crate::src::headers::sqliteInt_h::SrcItem,
        ) as *mut crate::src::headers::sqliteInt_h::Table;
        if !pTab.is_null() {
            if (*pTab).eTabType as ::core::ffi::c_int
                == crate::src::headers::sqliteInt_h::TABTYP_VTAB
            {
                crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    b"virtual tables may not be altered\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            } else if (*pTab).eTabType as ::core::ffi::c_int
                == crate::src::headers::sqliteInt_h::TABTYP_VIEW
            {
                crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    b"Cannot add a column to a view\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            } else if (crate::src::headers::sqlite3_h::SQLITE_OK == isAlterableTable(pParse, pTab))
            {
                crate::src::src::build::sqlite3MayAbort(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                );
                iDb = crate::src::src::prepare::sqlite3SchemaToIndex(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    (*pTab).pSchema as *mut crate::src::headers::sqliteInt_h::Schema,
                );
                pNew = crate::src::src::malloc::sqlite3DbMallocZero(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Table>()
                        as crate::src::ext::rtree::rtree::U64_0,
                ) as *mut crate::src::headers::sqliteInt_h::Table;
                if !pNew.is_null() {
                    (*pParse).pNewTable = pNew;
                    let __pNew_ref = { &mut *pNew };
                    __pNew_ref.nTabRef = 1 as crate::src::ext::rtree::rtree::U32_0;
                    __pNew_ref.nCol = (*pTab).nCol;
                    nAlloc = (__pNew_ref.nCol as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        / 8 as ::core::ffi::c_int
                        * 8 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int;
                    __pNew_ref.aCol = crate::src::src::malloc::sqlite3DbMallocZero(
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        (::core::mem::size_of::<crate::src::headers::sqliteInt_h::Column>()
                            as usize)
                            .wrapping_mul(nAlloc as crate::src::ext::rtree::rtree::U32_0 as usize)
                            as crate::src::ext::rtree::rtree::U64_0,
                    )
                        as *mut crate::src::headers::sqliteInt_h::Column;
                    __pNew_ref.zName = crate::src::printf_c_variadic::sqlite3MPrintf_args(
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        b"sqlite_altertab_%s\0" as *const u8 as *const ::core::ffi::c_char,
                        &[crate::src::src::printf::PrintfArg::Str(
                            (*pTab).zName as *mut ::core::ffi::c_char,
                        )],
                    );
                    if !(__pNew_ref.aCol.is_null() || __pNew_ref.zName.is_null()) {
                        let __pTab_ref = { &*pTab };
                        ::core::ptr::copy_nonoverlapping(
                            __pTab_ref.aCol as *const u8,
                            __pNew_ref.aCol as *mut u8,
                            ((::core::mem::size_of::<crate::src::headers::sqliteInt_h::Column>()
                                as crate::__stddef_size_t_h::SizeT)
                                .wrapping_mul(__pNew_ref.nCol as crate::__stddef_size_t_h::SizeT))
                                as usize,
                        );
                        i = 0 as ::core::ffi::c_int;
                        while i < __pNew_ref.nCol as ::core::ffi::c_int {
                            let pCol: *mut crate::src::headers::sqliteInt_h::Column =
                                __pNew_ref.aCol.offset(i as isize)
                                    as *mut crate::src::headers::sqliteInt_h::Column;
                            let __pCol_ref = { &mut *pCol };
                            __pCol_ref.zCnName = crate::src::src::malloc::sqlite3DbStrDup(
                                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                __pCol_ref.zCnName,
                            );
                            __pCol_ref.hName =
                                crate::src::src::util::sqlite3StrIHash(__pCol_ref.zCnName);
                            i += 1;
                        }
                        __pNew_ref.u.tab.pDfltList = crate::src::src::expr::sqlite3ExprListDup(
                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                            __pTab_ref.u.tab.pDfltList
                                as *const crate::src::headers::sqliteInt_h::ExprList,
                            0 as ::core::ffi::c_int,
                        )
                            as *mut crate::src::headers::sqliteInt_h::ExprList;
                        __pNew_ref.pSchema = (*(*db).aDb.offset(iDb as isize)).pSchema;
                        __pNew_ref.u.tab.addColOffset = __pTab_ref.u.tab.addColOffset;
                    }
                }
            }
        }
    }
    crate::src::src::build::sqlite3SrcListDelete(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        pSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
    );
}

unsafe extern "C" fn isRealTable(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pTab: *mut crate::src::headers::sqliteInt_h::Table,
    bDrop: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut zType: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if (*pTab).eTabType as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::TABTYP_VIEW {
        zType = b"view\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if (*pTab).eTabType as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::TABTYP_VTAB {
        zType = b"virtual table\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if !zType.is_null() {
        crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"cannot %s %s \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::src::src::printf::PrintfArg::Str(if bDrop != 0 {
                    b"drop column from\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"rename columns of\0" as *const u8 as *const ::core::ffi::c_char
                }
                    as *mut ::core::ffi::c_char),
                crate::src::src::printf::PrintfArg::Str(zType as *mut ::core::ffi::c_char),
                crate::src::src::printf::PrintfArg::Str((*pTab).zName as *mut ::core::ffi::c_char),
            ],
        );
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3AlterRenameColumn(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pSrc: *mut crate::src::headers::sqliteInt_h::SrcList,
    pOld: *mut crate::src::headers::sqliteInt_h::Token,
    pNew: *mut crate::src::headers::sqliteInt_h::Token,
) {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    
    let iCol: ::core::ffi::c_int;
    let mut zOld: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let zDb: *const ::core::ffi::c_char;
    let iSchema: ::core::ffi::c_int;
    let bQuote: ::core::ffi::c_int;
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = crate::src::src::build::sqlite3LocateTableItem(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        0 as crate::src::ext::rtree::rtree::U32_0,
        (&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem).offset(0_isize)
            as *mut crate::src::headers::sqliteInt_h::SrcItem
            as *mut crate::src::headers::sqliteInt_h::SrcItem,
    ) as *mut crate::src::headers::sqliteInt_h::Table;
    if !pTab.is_null()
        && (crate::src::headers::sqlite3_h::SQLITE_OK == isAlterableTable(pParse, pTab))
            && (crate::src::headers::sqlite3_h::SQLITE_OK == isRealTable(pParse, pTab, 0 as ::core::ffi::c_int))
            {
                iSchema = crate::src::src::prepare::sqlite3SchemaToIndex(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    (*pTab).pSchema as *mut crate::src::headers::sqliteInt_h::Schema,
                );
                zDb = (*(*db).aDb.offset(iSchema as isize)).zDbSName;
                if (crate::src::src::auth::sqlite3AuthCheck(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    crate::src::headers::sqlite3_h::SQLITE_ALTER_TABLE,
                    zDb,
                    (*pTab).zName,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                ) == 0)
                {
                    zOld = crate::src::src::build::sqlite3NameFromToken(
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        pOld as *const crate::src::headers::sqliteInt_h::Token,
                    );
                    if !zOld.is_null() {
                        iCol = crate::src::src::select::sqlite3ColumnIndex(
                            pTab as *mut crate::src::headers::sqliteInt_h::Table,
                            zOld,
                        );
                        if iCol < 0 as ::core::ffi::c_int {
                            crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                b"no such column: \"%T\"\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[crate::src::src::printf::PrintfArg::Token(
                                    pOld as *mut crate::src::headers::sqliteInt_h::Token,
                                )],
                            );
                        } else {
                            renameTestSchema(
                                pParse,
                                zDb,
                                (iSchema == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                b"\0" as *const u8 as *const ::core::ffi::c_char,
                                0 as ::core::ffi::c_int,
                            );
                            renameFixQuotes(
                                pParse,
                                zDb,
                                (iSchema == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                            );
                            crate::src::src::build::sqlite3MayAbort(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            );
                            zNew = crate::src::src::build::sqlite3NameFromToken(
                                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                pNew as *const crate::src::headers::sqliteInt_h::Token,
                            );
                            if !zNew.is_null() {
                                bQuote = *(&raw const crate::src::src::global::sqlite3CtypeMap
                                    as *const ::core::ffi::c_uchar)
                                    .offset(*(*pNew).z.offset(0_isize) as ::core::ffi::c_uchar
                                        as isize)
                                    as ::core::ffi::c_int
                                    & 0x80 as ::core::ffi::c_int;
                                let __pTab_ref = { &*pTab };
                                crate::src::src::build::sqlite3NestedParse_args(
                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                    "UPDATE \"%w\".sqlite_master SET sql = sqlite_rename_column(sql, type, name, %Q, %Q, %d, %Q, %d, %d) WHERE name NOT LIKE 'sqliteX_%%' ESCAPE 'X'  AND (type != 'index' OR tbl_name = %Q)",
                                    crate::printf_args_slice!(
                                        zDb,
                                        zDb,
                                        __pTab_ref.zName,
                                        iCol,
                                        zNew,
                                        bQuote,
                                        (iSchema == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                        __pTab_ref.zName
                                    ),
                                );
                                crate::src::src::build::sqlite3NestedParse_args(
                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                    "UPDATE temp.sqlite_master SET sql = sqlite_rename_column(sql, type, name, %Q, %Q, %d, %Q, %d, 1) WHERE type IN ('trigger', 'view')",
                                    crate::printf_args_slice!(
                                        zDb,
                                        __pTab_ref.zName,
                                        iCol,
                                        zNew,
                                        bQuote
                                    ),
                                );
                                renameReloadSchema(
                                    pParse,
                                    iSchema,
                                    crate::src::headers::sqliteInt_h::INITFLAG_AlterRename
                                        as crate::src::fts5::U16_0,
                                );
                                renameTestSchema(
                                    pParse,
                                    zDb,
                                    (iSchema == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                    b"after rename\0" as *const u8 as *const ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int,
                                );
                            }
                        }
                    }
                }
            }
    crate::src::src::build::sqlite3SrcListDelete(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        pSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
    );
    crate::src::src::malloc::sqlite3DbFree(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        zOld as *mut ::core::ffi::c_void,
    );
    crate::src::src::malloc::sqlite3DbFree(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        zNew as *mut ::core::ffi::c_void,
    );
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3RenameTokenMap(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pPtr: *const ::core::ffi::c_void,
    pToken: *const crate::src::headers::sqliteInt_h::Token,
) -> *const ::core::ffi::c_void {
    let pNew: *mut RenameToken;
    if (*pParse).eParseMode as ::core::ffi::c_int != 3 as ::core::ffi::c_int {
        pNew = crate::src::src::malloc::sqlite3DbMallocZero(
            (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::mem::size_of::<RenameToken>() as crate::src::ext::rtree::rtree::U64_0,
        ) as *mut RenameToken;
        if !pNew.is_null() {
            let __pNew_ref = { &mut *pNew };
            __pNew_ref.p = pPtr;
            __pNew_ref.t = *pToken;
            __pNew_ref.pNext = (*pParse).pRename;
            (*pParse).pRename = pNew;
        }
    }
    pPtr
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3RenameTokenRemap(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pTo: *const ::core::ffi::c_void,
    pFrom: *const ::core::ffi::c_void,
) {
    let mut p: *mut RenameToken;
    p = (*pParse).pRename;
    while !p.is_null() {
        if (*p).p == pFrom {
            (*p).p = pTo;
            break;
        } else {
            p = (*p).pNext;
        }
    }
}

unsafe extern "C" fn renameUnmapExprCb(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let pParse: *mut crate::src::headers::sqliteInt_h::Parse = (*pWalker).pParse;
    sqlite3RenameTokenRemap(
        pParse,
        ::core::ptr::null::<::core::ffi::c_void>(),
        pExpr as *const ::core::ffi::c_void,
    );
    if (*pExpr).flags
        & (crate::src::headers::sqliteInt_h::EP_WinFunc
            | crate::src::headers::sqliteInt_h::EP_Subrtn)
            as crate::src::ext::rtree::rtree::U32_0
        == 0 as crate::src::ext::rtree::rtree::U32_0
    {
        sqlite3RenameTokenRemap(
            pParse,
            ::core::ptr::null::<::core::ffi::c_void>(),
            &raw mut (*pExpr).y.pTab as *const ::core::ffi::c_void,
        );
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}

unsafe extern "C" fn renameWalkWith(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pSelect: *mut crate::src::headers::sqliteInt_h::Select,
) {
    let pWith: *mut crate::src::headers::sqliteInt_h::With = (*pSelect).pWith;
    if !pWith.is_null() {
        let pParse: *mut crate::src::headers::sqliteInt_h::Parse = (*pWalker).pParse;
        let mut i: ::core::ffi::c_int;
        let mut pCopy: *mut crate::src::headers::sqliteInt_h::With =
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::With>();
        if (*(*(&raw mut (*pWith).a as *mut crate::src::headers::sqliteInt_h::Cte)
            .offset(0_isize))
        .pSelect)
            .selFlags
            & crate::src::headers::sqliteInt_h::SF_Expanded as crate::src::ext::rtree::rtree::U32_0
            == 0 as crate::src::ext::rtree::rtree::U32_0
        {
            pCopy = crate::src::src::expr::sqlite3WithDup(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                pWith as *mut crate::src::headers::sqliteInt_h::With,
            ) as *mut crate::src::headers::sqliteInt_h::With;
            pCopy = crate::src::src::select::sqlite3WithPush(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                pCopy as *mut crate::src::headers::sqliteInt_h::With,
                1 as crate::src::ext::rtree::rtree::U8_0,
            ) as *mut crate::src::headers::sqliteInt_h::With;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*pWith).nCte {
            let p: *mut crate::src::headers::sqliteInt_h::Select = (*(&raw mut (*pWith).a
                as *mut crate::src::headers::sqliteInt_h::Cte)
                .offset(i as isize))
            .pSelect;
            let mut sNC: crate::src::headers::sqliteInt_h::NameContext = { ::core::mem::zeroed() };
            sNC.pParse = pParse;
            if !pCopy.is_null() {
                crate::src::src::select::sqlite3SelectPrep(
                    sNC.pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    p as *mut crate::src::headers::sqliteInt_h::Select,
                    &raw mut sNC as *mut _ as *mut crate::src::headers::sqliteInt_h::NameContext,
                );
            }
            if (*(*sNC.pParse).db).mallocFailed != 0 {
                return;
            }
            crate::src::src::walker::sqlite3WalkSelect(
                pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
                p as *mut crate::src::headers::sqliteInt_h::Select,
            );
            sqlite3RenameExprlistUnmap(
                pParse,
                (*(&raw mut (*pWith).a as *mut crate::src::headers::sqliteInt_h::Cte)
                    .offset(i as isize))
                .pCols,
            );
            i += 1;
        }
        if !pCopy.is_null() && (*pParse).pWith == pCopy {
            (*pParse).pWith = (*pCopy).pOuter;
        }
    }
}

unsafe extern "C" fn unmapColumnIdlistNames(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pIdList: *const crate::src::headers::sqliteInt_h::IdList,
) {
    let mut ii: ::core::ffi::c_int;
    ii = 0 as ::core::ffi::c_int;
    while ii < (*pIdList).nId {
        sqlite3RenameTokenRemap(
            pParse,
            ::core::ptr::null::<::core::ffi::c_void>(),
            (*(&raw const (*pIdList).a as *const crate::src::headers::sqliteInt_h::IdList_item)
                .offset(ii as isize))
            .zName as *const ::core::ffi::c_void,
        );
        ii += 1;
    }
}

unsafe extern "C" fn renameUnmapSelectCb(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    p: *mut crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    let pParse: *mut crate::src::headers::sqliteInt_h::Parse = (*pWalker).pParse;
    let mut i: ::core::ffi::c_int;
    if (*pParse).nErr != 0 {
        return crate::src::headers::sqliteInt_h::WRC_Abort;
    }
    let __p_ref = { &mut *p };
    if __p_ref.selFlags
        & (crate::src::headers::sqliteInt_h::SF_View | crate::src::headers::sqliteInt_h::SF_CopyCte)
            as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        return crate::src::headers::sqliteInt_h::WRC_Prune;
    }
    if !__p_ref.pEList.is_null() {
        let pList: *mut crate::src::headers::sqliteInt_h::ExprList = __p_ref.pEList;
        i = 0 as ::core::ffi::c_int;
        while i < (*pList).nExpr {
            if !(*(&raw mut (*pList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(i as isize))
            .zEName
            .is_null()
                && (*(&raw mut (*pList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(i as isize))
                .fg
                .eEName() as ::core::ffi::c_int
                    == crate::src::headers::sqliteInt_h::ENAME_NAME
            {
                sqlite3RenameTokenRemap(
                    pParse,
                    ::core::ptr::null::<::core::ffi::c_void>(),
                    (*(&raw mut (*pList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(i as isize))
                    .zEName as *mut ::core::ffi::c_void,
                );
            }
            i += 1;
        }
    }
    if !__p_ref.pSrc.is_null() {
        let pSrc: *mut crate::src::headers::sqliteInt_h::SrcList = __p_ref.pSrc;
        i = 0 as ::core::ffi::c_int;
        while i < (*pSrc).nSrc {
            sqlite3RenameTokenRemap(
                pParse,
                ::core::ptr::null::<::core::ffi::c_void>(),
                (*(&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(i as isize))
                .zName as *mut ::core::ffi::c_void,
            );
            if (*(&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                .offset(i as isize))
            .fg
            .isUsing() as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                crate::src::src::walker::sqlite3WalkExpr(
                    pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
                    (*(&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                        .offset(i as isize))
                    .u3
                    .pOn as *mut crate::src::headers::sqliteInt_h::Expr,
                );
            } else {
                unmapColumnIdlistNames(
                    pParse,
                    (*(&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                        .offset(i as isize))
                    .u3
                    .pUsing,
                );
            }
            i += 1;
        }
    }
    renameWalkWith(pWalker, p);
    crate::src::headers::sqliteInt_h::WRC_Continue
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3RenameExprUnmap(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) {
    let __pParse_ref = { &mut *pParse };
    let eMode: crate::src::ext::rtree::rtree::U8_0 = __pParse_ref.eParseMode;
    let mut sWalker: crate::src::headers::sqliteInt_h::Walker = { ::core::mem::zeroed() };
    sWalker.pParse = pParse;
    sWalker.xExprCallback = Some(
        renameUnmapExprCb
            as unsafe extern "C" fn(
                *mut crate::src::headers::sqliteInt_h::Walker,
                *mut crate::src::headers::sqliteInt_h::Expr,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut crate::src::headers::sqliteInt_h::Walker,
                *mut crate::src::headers::sqliteInt_h::Expr,
            ) -> ::core::ffi::c_int,
        >;
    sWalker.xSelectCallback = Some(
        renameUnmapSelectCb
            as unsafe extern "C" fn(
                *mut crate::src::headers::sqliteInt_h::Walker,
                *mut crate::src::headers::sqliteInt_h::Select,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut crate::src::headers::sqliteInt_h::Walker,
                *mut crate::src::headers::sqliteInt_h::Select,
            ) -> ::core::ffi::c_int,
        >;
    __pParse_ref.eParseMode =
        crate::src::headers::sqliteInt_h::PARSE_MODE_UNMAP as crate::src::ext::rtree::rtree::U8_0;
    crate::src::src::walker::sqlite3WalkExpr(
        &raw mut sWalker as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
    );
    __pParse_ref.eParseMode = eMode;
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3RenameExprlistUnmap(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pEList: *mut crate::src::headers::sqliteInt_h::ExprList,
) {
    if !pEList.is_null() {
        let mut i: ::core::ffi::c_int;
        let mut sWalker: crate::src::headers::sqliteInt_h::Walker = { ::core::mem::zeroed() };
        sWalker.pParse = pParse;
        sWalker.xExprCallback = Some(
            renameUnmapExprCb
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqliteInt_h::Walker,
                    *mut crate::src::headers::sqliteInt_h::Expr,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::headers::sqliteInt_h::Walker,
                    *mut crate::src::headers::sqliteInt_h::Expr,
                ) -> ::core::ffi::c_int,
            >;
        crate::src::src::walker::sqlite3WalkExprList(
            &raw mut sWalker as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
            pEList as *mut crate::src::headers::sqliteInt_h::ExprList,
        );
        i = 0 as ::core::ffi::c_int;
        while i < (*pEList).nExpr {
            if (*(&raw mut (*pEList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(i as isize))
            .fg
            .eEName() as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                sqlite3RenameTokenRemap(
                    pParse,
                    ::core::ptr::null::<::core::ffi::c_void>(),
                    (*(&raw mut (*pEList).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(i as isize))
                    .zEName as *mut ::core::ffi::c_void,
                );
            }
            i += 1;
        }
    }
}

unsafe extern "C" fn renameTokenFree(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pToken: *mut RenameToken,
) {
    let mut pNext: *mut RenameToken;
    let mut p: *mut RenameToken;
    p = pToken;
    while !p.is_null() {
        pNext = (*p).pNext;
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            p as *mut ::core::ffi::c_void,
        );
        p = pNext;
    }
}

unsafe extern "C" fn renameTokenFind(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pCtx: *mut RenameCtx,
    pPtr: *const ::core::ffi::c_void,
) -> *mut RenameToken {
    let mut pp: *mut *mut RenameToken;
    if pPtr.is_null() {
        return ::core::ptr::null_mut::<RenameToken>();
    }
    pp = &raw mut (*pParse).pRename;
    while !(*pp).is_null() {
        if (**pp).p == pPtr {
            let pToken: *mut RenameToken = *pp;
            if !pCtx.is_null() {
                *pp = (*pToken).pNext;
                let __pCtx_ref = { &mut *pCtx };
                (*pToken).pNext = __pCtx_ref.pList;
                __pCtx_ref.pList = pToken;
                __pCtx_ref.nList += 1;
            }
            return pToken;
        }
        pp = &raw mut (**pp).pNext;
    }
    ::core::ptr::null_mut::<RenameToken>()
}

unsafe extern "C" fn renameColumnSelectCb(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    p: *mut crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    if (*p).selFlags
        & (crate::src::headers::sqliteInt_h::SF_View | crate::src::headers::sqliteInt_h::SF_CopyCte)
            as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        return crate::src::headers::sqliteInt_h::WRC_Prune;
    }
    renameWalkWith(pWalker, p);
    crate::src::headers::sqliteInt_h::WRC_Continue
}

unsafe extern "C" fn renameColumnExprCb(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let p: *mut RenameCtx = (*pWalker).u.pRename as *mut RenameCtx;
    let __p_ref = { &*p };
    let __pExpr_ref = { &*pExpr };
    if __pExpr_ref.op as ::core::ffi::c_int == crate::src::parse::TK_TRIGGER
        && __pExpr_ref.iColumn as ::core::ffi::c_int == __p_ref.iCol
        && (*(*pWalker).pParse).pTriggerTab == __p_ref.pTab
        || __pExpr_ref.op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN
            && __pExpr_ref.iColumn as ::core::ffi::c_int == __p_ref.iCol
            && __pExpr_ref.flags
                & (0x1000000 as ::core::ffi::c_int | 0x2000000 as ::core::ffi::c_int)
                    as crate::src::ext::rtree::rtree::U32_0
                == 0 as crate::src::ext::rtree::rtree::U32_0
            && __p_ref.pTab == __pExpr_ref.y.pTab
    {
        renameTokenFind(
            (*pWalker).pParse,
            p as *mut RenameCtx,
            pExpr as *mut ::core::ffi::c_void,
        );
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}

unsafe extern "C" fn renameColumnTokenNext(pCtx: *mut RenameCtx) -> *mut RenameToken {
    let mut pBest: *mut RenameToken = (*pCtx).pList;
    let mut pToken: *mut RenameToken;
    let mut pp: *mut *mut RenameToken;
    pToken = (*pBest).pNext;
    while !pToken.is_null() {
        if (*pToken).t.z > (*pBest).t.z {
            pBest = pToken;
        }
        pToken = (*pToken).pNext;
    }
    pp = &raw mut (*pCtx).pList;
    while *pp != pBest {
        pp = &raw mut (**pp).pNext;
    }
    *pp = (*pBest).pNext;
    pBest
}

unsafe extern "C" fn renameColumnParseError(
    pCtx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    zWhen: *const ::core::ffi::c_char,
    pType: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pObject: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
) {
    let zT: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(pType) as *const ::core::ffi::c_char;
    let zN: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(pObject) as *const ::core::ffi::c_char;
    
    let __pParse_ref = { &*pParse };
    let zErr: *mut ::core::ffi::c_char = crate::src::printf_c_variadic::sqlite3MPrintf_args(
        __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        b"error in %s %s%s%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[
            crate::src::src::printf::PrintfArg::Str(zT as *mut ::core::ffi::c_char),
            crate::src::src::printf::PrintfArg::Str(zN as *mut ::core::ffi::c_char),
            crate::src::src::printf::PrintfArg::Str(if *zWhen.offset(0_isize)
                as ::core::ffi::c_int
                != 0
            {
                b" \0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } as *mut ::core::ffi::c_char),
            crate::src::src::printf::PrintfArg::Str(zWhen as *mut ::core::ffi::c_char),
            crate::src::src::printf::PrintfArg::Str(
                __pParse_ref.zErrMsg as *mut ::core::ffi::c_char,
            ),
        ],
    );
    crate::src::src::vdbeapi::sqlite3_result_error(pCtx, zErr, -(1 as ::core::ffi::c_int));
    crate::src::src::malloc::sqlite3DbFree(
        __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        zErr as *mut ::core::ffi::c_void,
    );
}

unsafe extern "C" fn renameColumnElistNames(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pCtx: *mut RenameCtx,
    pEList: *const crate::src::headers::sqliteInt_h::ExprList,
    zOld: *const ::core::ffi::c_char,
) {
    if !pEList.is_null() {
        let mut i: ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*pEList).nExpr {
            let zName: *const ::core::ffi::c_char = (*(&raw const (*pEList).a
                as *const crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(i as isize))
            .zEName;
            if (*(&raw const (*pEList).a as *const crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(i as isize))
            .fg
            .eEName() as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && !zName.is_null()
                && 0 as ::core::ffi::c_int == crate::src::src::util::sqlite3_stricmp(zName, zOld)
            {
                renameTokenFind(
                    pParse,
                    pCtx as *mut RenameCtx,
                    zName as *const ::core::ffi::c_void,
                );
            }
            i += 1;
        }
    }
}

unsafe extern "C" fn renameColumnIdlistNames(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pCtx: *mut RenameCtx,
    pIdList: *const crate::src::headers::sqliteInt_h::IdList,
    zOld: *const ::core::ffi::c_char,
) {
    if !pIdList.is_null() {
        let mut i: ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIdList).nId {
            let zName: *const ::core::ffi::c_char = (*(&raw const (*pIdList).a
                as *const crate::src::headers::sqliteInt_h::IdList_item)
                .offset(i as isize))
            .zName;
            if 0 as ::core::ffi::c_int == crate::src::src::util::sqlite3_stricmp(zName, zOld) {
                renameTokenFind(
                    pParse,
                    pCtx as *mut RenameCtx,
                    zName as *const ::core::ffi::c_void,
                );
            }
            i += 1;
        }
    }
}

unsafe extern "C" fn renameParseSql(
    p: *mut crate::src::headers::sqliteInt_h::Parse,
    zDb: *const ::core::ffi::c_char,
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zSql: *const ::core::ffi::c_char,
    bTemp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    
    crate::src::src::prepare::sqlite3ParseObjectInit(
        p as *mut crate::src::headers::sqliteInt_h::Parse,
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    );
    if zSql.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    if crate::src::src::util::sqlite3_strnicmp(
        zSql,
        b"CREATE \0" as *const u8 as *const ::core::ffi::c_char,
        7 as ::core::ffi::c_int,
    ) != 0 as ::core::ffi::c_int
    {
        return crate::src::src::main::sqlite3CorruptError(1146 as ::core::ffi::c_int);
    }
    let __db_ref = { &mut *db };
    if bTemp != 0 {
        __db_ref.init.iDb = 1 as crate::src::ext::rtree::rtree::U8_0;
    } else {
        let iDb: ::core::ffi::c_int = crate::src::src::build::sqlite3FindDbName(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            zDb,
        );
        __db_ref.init.iDb = iDb as crate::src::ext::rtree::rtree::U8_0;
    }
    let __p_ref = { &mut *p };
    __p_ref.eParseMode =
        crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME as crate::src::ext::rtree::rtree::U8_0;
    __p_ref.db = db;
    __p_ref.nQueryLoop = 1 as crate::src::headers::sqliteInt_h::LogEst;
    let flags: crate::src::ext::rtree::rtree::U64_0 = __db_ref.flags;
    __db_ref.flags |= crate::src::headers::sqliteInt_h::SQLITE_Comments;
    rc = crate::src::src::tokenize::sqlite3RunParser(
        p as *mut crate::src::headers::sqliteInt_h::Parse,
        zSql,
    );
    __db_ref.flags = flags;
    if __db_ref.mallocFailed != 0 {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && (__p_ref.pNewTable.is_null()
            && __p_ref.pNewIndex.is_null()
            && __p_ref.pNewTrigger.is_null())
    {
        rc = crate::src::src::main::sqlite3CorruptError(1167 as ::core::ffi::c_int);
    }
    __db_ref.init.iDb = 0 as crate::src::ext::rtree::rtree::U8_0;
    rc
}

unsafe extern "C" fn renameEditSql(
    pCtx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    pRename: *mut RenameCtx,
    zSql: *const ::core::ffi::c_char,
    zNew: *const ::core::ffi::c_char,
    bQuote: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let nNew: crate::src::ext::rtree::rtree::I64_0 =
        crate::src::src::util::sqlite3Strlen30(zNew) as crate::src::ext::rtree::rtree::I64_0;
    let nSql: crate::src::ext::rtree::rtree::I64_0 =
        crate::src::src::util::sqlite3Strlen30(zSql) as crate::src::ext::rtree::rtree::I64_0;
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 =
        crate::src::src::vdbeapi::sqlite3_context_db_handle(pCtx)
            as *mut crate::src::headers::sqliteInt_h::sqlite3;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut zQuot: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let zOut: *mut ::core::ffi::c_char;
    let mut nQuot: crate::src::ext::rtree::rtree::I64_0 = 0 as crate::src::ext::rtree::rtree::I64_0;
    let mut zBuf1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zBuf2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !zNew.is_null() {
        zQuot = crate::src::printf_c_variadic::sqlite3MPrintf_args(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            b"\"%w\" \0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Str(
                zNew as *mut ::core::ffi::c_char,
            )],
        );
        if zQuot.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            nQuot = (crate::src::src::util::sqlite3Strlen30(zQuot) - 1 as ::core::ffi::c_int)
                as crate::src::ext::rtree::rtree::I64_0;
        }
        zOut = crate::src::src::malloc::sqlite3DbMallocZero(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (nSql as crate::src::ext::rtree::rtree::U64_0)
                .wrapping_add(
                    ((*pRename).nList as crate::src::ext::rtree::rtree::U64_0)
                        .wrapping_mul(nQuot as crate::src::ext::rtree::rtree::U64_0),
                )
                .wrapping_add(1 as crate::src::ext::rtree::rtree::U64_0),
        ) as *mut ::core::ffi::c_char;
    } else {
        zOut = crate::src::src::malloc::sqlite3DbMallocZero(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (2 as crate::src::ext::rtree::rtree::U64_0)
                .wrapping_mul(nSql as crate::src::ext::rtree::rtree::U64_0)
                .wrapping_add(1 as crate::src::ext::rtree::rtree::U64_0)
                .wrapping_mul(3 as crate::src::ext::rtree::rtree::U64_0),
        ) as *mut ::core::ffi::c_char;
        if !zOut.is_null() {
            zBuf1 = zOut.offset(
                (nSql * 2 as crate::src::ext::rtree::rtree::I64_0
                    + 1 as crate::src::ext::rtree::rtree::I64_0) as isize,
            ) as *mut ::core::ffi::c_char;
            zBuf2 = zOut.offset(
                (nSql * 4 as crate::src::ext::rtree::rtree::I64_0
                    + 2 as crate::src::ext::rtree::rtree::I64_0) as isize,
            ) as *mut ::core::ffi::c_char;
        }
    }
    if !zOut.is_null() {
        let mut nOut: crate::src::ext::rtree::rtree::I64_0 = nSql;
        ::core::ptr::copy_nonoverlapping(zSql as *const u8, zOut as *mut u8, nSql as usize);
        while !(*pRename).pList.is_null() {
            
            let mut nReplace: crate::src::ext::rtree::rtree::I64_0;
            let zReplace: *const ::core::ffi::c_char;
            let pBest: *mut RenameToken = renameColumnTokenNext(pRename);
            if !zNew.is_null() {
                if bQuote == 0 as ::core::ffi::c_int
                    && crate::src::src::tokenize::sqlite3IsIdChar(
                        *((*pBest).t.z as *mut crate::src::ext::rtree::rtree::U8_0),
                    ) != 0
                {
                    nReplace = nNew;
                    zReplace = zNew;
                } else {
                    nReplace = nQuot;
                    zReplace = zQuot;
                    if *(*pBest).t.z.offset((*pBest).t.n as isize) as ::core::ffi::c_int
                        == '"' as i32
                    {
                        nReplace += 1;
                    }
                }
            } else {
                let __pBest_ref = { &mut *pBest };
                ::core::ptr::copy_nonoverlapping(
                    __pBest_ref.t.z as *const u8,
                    zBuf1 as *mut u8,
                    __pBest_ref.t.n as usize,
                );
                *zBuf1.offset(__pBest_ref.t.n as isize) = 0 as ::core::ffi::c_char;
                crate::src::src::util::sqlite3Dequote(zBuf1);
                crate::sqlite_snprintf!(
                    zBuf2,
                    (nSql * 2 as crate::src::ext::rtree::rtree::I64_0) as ::core::ffi::c_int,
                    "%Q%s",
                    zBuf1,
                    if *__pBest_ref.t.z.offset(__pBest_ref.t.n as isize) as ::core::ffi::c_int
                        == '\'' as i32
                    {
                        b" \0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    },
                );
                zReplace = zBuf2;
                nReplace = crate::src::src::util::sqlite3Strlen30(zReplace)
                    as crate::src::ext::rtree::rtree::I64_0;
            }
            let iOff: ::core::ffi::c_int = (*pBest).t.z.offset_from(zSql) as ::core::ffi::c_long as ::core::ffi::c_int;
            if (*pBest).t.n as crate::src::ext::rtree::rtree::I64_0 != nReplace {
                let __pBest_ref = { &*pBest };
                ::core::ptr::copy(
                    zOut.offset((iOff as ::core::ffi::c_uint).wrapping_add(__pBest_ref.t.n) as isize)
                        as *const ::core::ffi::c_char,
                    zOut.offset((iOff as crate::src::ext::rtree::rtree::I64_0 + nReplace) as isize) as *mut ::core::ffi::c_char,
                    (nOut - (iOff as ::core::ffi::c_uint).wrapping_add(__pBest_ref.t.n) as crate::src::ext::rtree::rtree::I64_0) as usize,
                );
                nOut += nReplace - __pBest_ref.t.n as crate::src::ext::rtree::rtree::I64_0;
                *zOut.offset(nOut as isize) = '\0' as i32 as ::core::ffi::c_char;
            }
            ::core::ptr::copy_nonoverlapping(
                zReplace as *const u8,
                zOut.offset(iOff as isize) as *mut ::core::ffi::c_char as *mut u8,
                nReplace as usize,
            );
            crate::src::src::malloc::sqlite3DbFree(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                pBest as *mut ::core::ffi::c_void,
            );
        }
        crate::src::src::vdbeapi::sqlite3_result_text(
            pCtx,
            zOut,
            -(1 as ::core::ffi::c_int),
            ::core::mem::transmute::<
                crate::src::headers::stdlib::IntptrT,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
        );
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            zOut as *mut ::core::ffi::c_void,
        );
    } else {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    crate::src::src::malloc::sqlite3_free(zQuot as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn renameSetENames(
    pEList: *mut crate::src::headers::sqliteInt_h::ExprList,
    val: ::core::ffi::c_int,
) {
    if !pEList.is_null() {
        let mut i: ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*pEList).nExpr {
            let fresh0 = &mut (*(&raw mut (*pEList).a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(i as isize))
            .fg;
            (*fresh0).set_eEName(
                (val & 0x3 as ::core::ffi::c_int) as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
            i += 1;
        }
    }
}

unsafe extern "C" fn renameResolveTrigger(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
) -> ::core::ffi::c_int {
    let __pParse_ref = { &mut *pParse };
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
    let pNew: *mut crate::src::headers::sqliteInt_h::Trigger = __pParse_ref.pNewTrigger;
    let mut pStep: *mut crate::src::headers::sqliteInt_h::TriggerStep;
    let mut sNC: crate::src::headers::sqliteInt_h::NameContext = { ::core::mem::zeroed() };
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    sNC.pParse = pParse;
    let __pNew_ref = { &mut *pNew };
    __pParse_ref.pTriggerTab = crate::src::src::build::sqlite3FindTable(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __pNew_ref.table,
        (*(*db)
            .aDb
            .offset(crate::src::src::prepare::sqlite3SchemaToIndex(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                __pNew_ref.pTabSchema as *mut crate::src::headers::sqliteInt_h::Schema,
            ) as isize))
        .zDbSName,
    ) as *mut crate::src::headers::sqliteInt_h::Table;
    __pParse_ref.eTriggerOp = __pNew_ref.op;
    if !__pParse_ref.pTriggerTab.is_null() {
        rc = (crate::src::src::build::sqlite3ViewGetColumnNames(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            __pParse_ref.pTriggerTab as *mut crate::src::headers::sqliteInt_h::Table,
        ) != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !__pNew_ref.pWhen.is_null() {
        rc = crate::src::src::resolve::sqlite3ResolveExprNames(
            &raw mut sNC as *mut _ as *mut crate::src::headers::sqliteInt_h::NameContext,
            __pNew_ref.pWhen as *mut crate::src::headers::sqliteInt_h::Expr,
        );
    }
    pStep = __pNew_ref.step_list;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK && !pStep.is_null() {
        if !(*pStep).pSelect.is_null() {
            crate::src::src::select::sqlite3SelectPrep(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*pStep).pSelect as *mut crate::src::headers::sqliteInt_h::Select,
                &raw mut sNC as *mut _ as *mut crate::src::headers::sqliteInt_h::NameContext,
            );
            if __pParse_ref.nErr != 0 {
                rc = __pParse_ref.rc;
            }
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !(*pStep).zTarget.is_null() {
            let mut pSrc: *mut crate::src::headers::sqliteInt_h::SrcList =
                crate::src::src::trigger::sqlite3TriggerStepSrc(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    pStep as *mut crate::src::headers::sqliteInt_h::TriggerStep,
                ) as *mut crate::src::headers::sqliteInt_h::SrcList;
            if !pSrc.is_null() {
                let __pStep_ref = { &mut *pStep };
                let pSel: *mut crate::src::headers::sqliteInt_h::Select =
                    crate::src::src::select::sqlite3SelectNew(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        __pStep_ref.pExprList as *mut crate::src::headers::sqliteInt_h::ExprList,
                        pSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                            as *mut crate::src::headers::sqliteInt_h::ExprList,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                            as *mut crate::src::headers::sqliteInt_h::ExprList,
                        0 as crate::src::ext::rtree::rtree::U32_0,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                    ) as *mut crate::src::headers::sqliteInt_h::Select;
                if pSel.is_null() {
                    __pStep_ref.pExprList =
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
                    pSrc = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>();
                    rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                } else {
                    renameSetENames(
                        __pStep_ref.pExprList,
                        crate::src::headers::sqliteInt_h::ENAME_SPAN,
                    );
                    crate::src::src::select::sqlite3SelectPrep(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        pSel as *mut crate::src::headers::sqliteInt_h::Select,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::NameContext>()
                            as *mut crate::src::headers::sqliteInt_h::NameContext,
                    );
                    renameSetENames(
                        __pStep_ref.pExprList,
                        crate::src::headers::sqliteInt_h::ENAME_NAME,
                    );
                    rc = if __pParse_ref.nErr != 0 {
                        crate::src::headers::sqlite3_h::SQLITE_ERROR
                    } else {
                        crate::src::headers::sqlite3_h::SQLITE_OK
                    };
                    if !__pStep_ref.pExprList.is_null() {
                        (*pSel).pEList =
                            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
                    }
                    (*pSel).pSrc =
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>();
                    crate::src::src::select::sqlite3SelectDelete(
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        pSel as *mut crate::src::headers::sqliteInt_h::Select,
                    );
                }
                if !__pStep_ref.pFrom.is_null() {
                    let mut i: ::core::ffi::c_int;
                    i = 0 as ::core::ffi::c_int;
                    while i < (*__pStep_ref.pFrom).nSrc
                        && rc == crate::src::headers::sqlite3_h::SQLITE_OK
                    {
                        let p: *mut crate::src::headers::sqliteInt_h::SrcItem =
                            (&raw mut (*__pStep_ref.pFrom).a
                                as *mut crate::src::headers::sqliteInt_h::SrcItem)
                                .offset(i as isize)
                                as *mut crate::src::headers::sqliteInt_h::SrcItem;
                        if (*p).fg.isSubquery() != 0 {
                            crate::src::src::select::sqlite3SelectPrep(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                (*(*p).u4.pSubq).pSelect
                                    as *mut crate::src::headers::sqliteInt_h::Select,
                                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::NameContext>(
                                )
                                    as *mut crate::src::headers::sqliteInt_h::NameContext,
                            );
                        }
                        i += 1;
                    }
                }
                if (*db).mallocFailed != 0 {
                    rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                }
                sNC.pSrcList = pSrc;
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK && !__pStep_ref.pWhere.is_null()
                {
                    rc = crate::src::src::resolve::sqlite3ResolveExprNames(
                        &raw mut sNC as *mut _
                            as *mut crate::src::headers::sqliteInt_h::NameContext,
                        __pStep_ref.pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
                    );
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    rc = crate::src::src::resolve::sqlite3ResolveExprListNames(
                        &raw mut sNC as *mut _
                            as *mut crate::src::headers::sqliteInt_h::NameContext,
                        __pStep_ref.pExprList as *mut crate::src::headers::sqliteInt_h::ExprList,
                    );
                }
                if !__pStep_ref.pUpsert.is_null() && rc == crate::src::headers::sqlite3_h::SQLITE_OK
                {
                    let pUpsert: *mut crate::src::headers::sqliteInt_h::Upsert =
                        __pStep_ref.pUpsert;
                    (*pUpsert).pUpsertSrc = pSrc;
                    sNC.uNC.pUpsert = pUpsert;
                    sNC.ncFlags = crate::src::headers::sqliteInt_h::NC_UUpsert;
                    rc = crate::src::src::resolve::sqlite3ResolveExprListNames(
                        &raw mut sNC as *mut _
                            as *mut crate::src::headers::sqliteInt_h::NameContext,
                        (*pUpsert).pUpsertTarget as *mut crate::src::headers::sqliteInt_h::ExprList,
                    );
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        let pUpsertSet: *mut crate::src::headers::sqliteInt_h::ExprList =
                            (*pUpsert).pUpsertSet;
                        rc = crate::src::src::resolve::sqlite3ResolveExprListNames(
                            &raw mut sNC as *mut _
                                as *mut crate::src::headers::sqliteInt_h::NameContext,
                            pUpsertSet as *mut crate::src::headers::sqliteInt_h::ExprList,
                        );
                    }
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        rc = crate::src::src::resolve::sqlite3ResolveExprNames(
                            &raw mut sNC as *mut _
                                as *mut crate::src::headers::sqliteInt_h::NameContext,
                            (*pUpsert).pUpsertWhere as *mut crate::src::headers::sqliteInt_h::Expr,
                        );
                    }
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        rc = crate::src::src::resolve::sqlite3ResolveExprNames(
                            &raw mut sNC as *mut _
                                as *mut crate::src::headers::sqliteInt_h::NameContext,
                            (*pUpsert).pUpsertTargetWhere
                                as *mut crate::src::headers::sqliteInt_h::Expr,
                        );
                    }
                    sNC.ncFlags = 0 as ::core::ffi::c_int;
                }
                sNC.pSrcList = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>();
                crate::src::src::build::sqlite3SrcListDelete(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    pSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
                );
            } else {
                rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
        }
        pStep = (*pStep).pNext;
    }
    rc
}

unsafe extern "C" fn renameWalkTrigger(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pTrigger: *mut crate::src::headers::sqliteInt_h::Trigger,
) {
    let mut pStep: *mut crate::src::headers::sqliteInt_h::TriggerStep;
    crate::src::src::walker::sqlite3WalkExpr(
        pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
        (*pTrigger).pWhen as *mut crate::src::headers::sqliteInt_h::Expr,
    );
    pStep = (*pTrigger).step_list;
    while !pStep.is_null() {
        crate::src::src::walker::sqlite3WalkSelect(
            pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
            (*pStep).pSelect as *mut crate::src::headers::sqliteInt_h::Select,
        );
        crate::src::src::walker::sqlite3WalkExpr(
            pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
            (*pStep).pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
        );
        crate::src::src::walker::sqlite3WalkExprList(
            pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
            (*pStep).pExprList as *mut crate::src::headers::sqliteInt_h::ExprList,
        );
        if !(*pStep).pUpsert.is_null() {
            let pUpsert: *mut crate::src::headers::sqliteInt_h::Upsert = (*pStep).pUpsert;
            let __pUpsert_ref = { &*pUpsert };
            crate::src::src::walker::sqlite3WalkExprList(
                pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
                __pUpsert_ref.pUpsertTarget as *mut crate::src::headers::sqliteInt_h::ExprList,
            );
            crate::src::src::walker::sqlite3WalkExprList(
                pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
                __pUpsert_ref.pUpsertSet as *mut crate::src::headers::sqliteInt_h::ExprList,
            );
            crate::src::src::walker::sqlite3WalkExpr(
                pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
                __pUpsert_ref.pUpsertWhere as *mut crate::src::headers::sqliteInt_h::Expr,
            );
            crate::src::src::walker::sqlite3WalkExpr(
                pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
                __pUpsert_ref.pUpsertTargetWhere as *mut crate::src::headers::sqliteInt_h::Expr,
            );
        }
        if !(*pStep).pFrom.is_null() {
            let mut i: ::core::ffi::c_int;
            let pFrom: *mut crate::src::headers::sqliteInt_h::SrcList = (*pStep).pFrom;
            i = 0 as ::core::ffi::c_int;
            while i < (*pFrom).nSrc {
                if (*(&raw mut (*pFrom).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(i as isize))
                .fg
                .isSubquery()
                    != 0
                {
                    crate::src::src::walker::sqlite3WalkSelect(
                        pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
                        (*(*(&raw mut (*pFrom).a
                            as *mut crate::src::headers::sqliteInt_h::SrcItem)
                            .offset(i as isize))
                        .u4
                        .pSubq)
                            .pSelect
                            as *mut crate::src::headers::sqliteInt_h::Select,
                    );
                }
                i += 1;
            }
        }
        pStep = (*pStep).pNext;
    }
}

unsafe extern "C" fn renameParseCleanup(pParse: *mut crate::src::headers::sqliteInt_h::Parse) {
    let __pParse_ref = { &mut *pParse };
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
    let mut pIdx: *mut crate::src::headers::sqliteInt_h::Index;
    if !__pParse_ref.pVdbe.is_null() {
        crate::src::src::vdbeaux::sqlite3VdbeFinalize(__pParse_ref.pVdbe);
    }
    crate::src::src::build::sqlite3DeleteTable(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __pParse_ref.pNewTable as *mut crate::src::headers::sqliteInt_h::Table,
    );
    loop {
        pIdx = __pParse_ref.pNewIndex;
        if pIdx.is_null() {
            break;
        }
        __pParse_ref.pNewIndex = (*pIdx).pNext;
        crate::src::src::build::sqlite3FreeIndex(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pIdx as *mut crate::src::headers::sqliteInt_h::Index,
        );
    }
    crate::src::src::trigger::sqlite3DeleteTrigger(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __pParse_ref.pNewTrigger as *mut crate::src::headers::sqliteInt_h::Trigger,
    );
    crate::src::src::malloc::sqlite3DbFree(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __pParse_ref.zErrMsg as *mut ::core::ffi::c_void,
    );
    renameTokenFree(db, __pParse_ref.pRename);
    crate::src::src::prepare::sqlite3ParseObjectReset(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
    );
}

unsafe extern "C" fn renameColumnFunc(
    context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut _NotUsed: ::core::ffi::c_int,
    argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let current_block: u64;
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 =
        crate::src::src::vdbeapi::sqlite3_context_db_handle(context)
            as *mut crate::src::headers::sqliteInt_h::sqlite3;
    let mut sCtx: RenameCtx = { ::core::mem::zeroed() };
    let zSql: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(0_isize))
            as *const ::core::ffi::c_char;
    let zDb: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(3_isize))
            as *const ::core::ffi::c_char;
    let zTable: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(4_isize))
            as *const ::core::ffi::c_char;
    let iCol: ::core::ffi::c_int =
        crate::src::src::vdbeapi::sqlite3_value_int(*argv.offset(5_isize));
    let zNew: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(6_isize))
            as *const ::core::ffi::c_char;
    let bQuote: ::core::ffi::c_int =
        crate::src::src::vdbeapi::sqlite3_value_int(*argv.offset(7_isize));
    let bTemp: ::core::ffi::c_int =
        crate::src::src::vdbeapi::sqlite3_value_int(*argv.offset(8_isize));
    
    let mut rc: ::core::ffi::c_int;
    let mut sParse: crate::src::headers::sqliteInt_h::Parse = { ::core::mem::zeroed() };
    let mut sWalker: crate::src::headers::sqliteInt_h::Walker = { ::core::mem::zeroed() };
    let mut pIdx: *mut crate::src::headers::sqliteInt_h::Index;
    let mut i: ::core::ffi::c_int;
    
    let __db_ref = { &mut *db };
    let xAuth: crate::src::headers::sqliteInt_h::Sqlite3Xauth = __db_ref.xAuth;
    if zSql.is_null() {
        return;
    }
    if zTable.is_null() {
        return;
    }
    if zNew.is_null() {
        return;
    }
    if iCol < 0 as ::core::ffi::c_int {
        return;
    }
    crate::src::src::btmutex::sqlite3BtreeEnterAll(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    );
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = crate::src::src::build::sqlite3FindTable(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        zTable,
        zDb,
    ) as *mut crate::src::headers::sqliteInt_h::Table;
    if pTab.is_null() || iCol >= (*pTab).nCol as ::core::ffi::c_int {
        crate::src::src::btmutex::sqlite3BtreeLeaveAll(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        );
        return;
    }
    let zOld: *const ::core::ffi::c_char = (*(*pTab).aCol.offset(iCol as isize)).zCnName;
    sCtx.iCol = if iCol == (*pTab).iPKey as ::core::ffi::c_int {
        -(1 as ::core::ffi::c_int)
    } else {
        iCol
    };
    __db_ref.xAuth = None;
    rc = renameParseSql(&raw mut sParse, zDb, db, zSql, bTemp);
    sWalker.pParse = &raw mut sParse;
    sWalker.xExprCallback = Some(
        renameColumnExprCb
            as unsafe extern "C" fn(
                *mut crate::src::headers::sqliteInt_h::Walker,
                *mut crate::src::headers::sqliteInt_h::Expr,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut crate::src::headers::sqliteInt_h::Walker,
                *mut crate::src::headers::sqliteInt_h::Expr,
            ) -> ::core::ffi::c_int,
        >;
    sWalker.xSelectCallback = Some(
        renameColumnSelectCb
            as unsafe extern "C" fn(
                *mut crate::src::headers::sqliteInt_h::Walker,
                *mut crate::src::headers::sqliteInt_h::Select,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut crate::src::headers::sqliteInt_h::Walker,
                *mut crate::src::headers::sqliteInt_h::Select,
            ) -> ::core::ffi::c_int,
        >;
    sWalker.u.pRename = &raw mut sCtx as *mut RenameCtx;
    sCtx.pTab = pTab;
    if (rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
        if !sParse.pNewTable.is_null() {
            if (*sParse.pNewTable).eTabType as ::core::ffi::c_int
                == crate::src::headers::sqliteInt_h::TABTYP_VIEW
            {
                let pSelect: *mut crate::src::headers::sqliteInt_h::Select =
                    (*sParse.pNewTable).u.view.pSelect;
                (*pSelect).selFlags &= !(crate::src::headers::sqliteInt_h::SF_View
                    as crate::src::ext::rtree::rtree::U32_0);
                sParse.rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                crate::src::src::select::sqlite3SelectPrep(
                    &raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse,
                    pSelect as *mut crate::src::headers::sqliteInt_h::Select,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::NameContext>()
                        as *mut crate::src::headers::sqliteInt_h::NameContext,
                );
                rc = if __db_ref.mallocFailed as ::core::ffi::c_int != 0 {
                    crate::src::headers::sqlite3_h::SQLITE_NOMEM
                } else {
                    sParse.rc
                };
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    crate::src::src::walker::sqlite3WalkSelect(
                        &raw mut sWalker as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
                        pSelect as *mut crate::src::headers::sqliteInt_h::Select,
                    );
                }
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    current_block = 6633030875973123825;
                } else {
                    current_block = 17688141731389699982;
                }
            } else {
                if (*sParse.pNewTable).eTabType as ::core::ffi::c_int
                    == crate::src::headers::sqliteInt_h::TABTYP_NORM
                {
                    let bFKOnly: ::core::ffi::c_int =
                        crate::src::src::util::sqlite3_stricmp(zTable, (*sParse.pNewTable).zName);
                    let mut pFKey: *mut crate::src::headers::sqliteInt_h::FKey;
                    sCtx.pTab = sParse.pNewTable;
                    if bFKOnly == 0 as ::core::ffi::c_int {
                        if iCol < (*sParse.pNewTable).nCol as ::core::ffi::c_int {
                            renameTokenFind(
                                &raw mut sParse,
                                &raw mut sCtx,
                                (*(*sParse.pNewTable).aCol.offset(iCol as isize)).zCnName
                                    as *mut ::core::ffi::c_void,
                            );
                        }
                        if sCtx.iCol < 0 as ::core::ffi::c_int {
                            renameTokenFind(
                                &raw mut sParse,
                                &raw mut sCtx,
                                &raw mut (*sParse.pNewTable).iPKey as *mut ::core::ffi::c_void,
                            );
                        }
                        crate::src::src::walker::sqlite3WalkExprList(
                            &raw mut sWalker as *mut _
                                as *mut crate::src::headers::sqliteInt_h::Walker,
                            (*sParse.pNewTable).pCheck
                                as *mut crate::src::headers::sqliteInt_h::ExprList,
                        );
                        pIdx = (*sParse.pNewTable).pIndex;
                        while !pIdx.is_null() {
                            crate::src::src::walker::sqlite3WalkExprList(
                                &raw mut sWalker as *mut _
                                    as *mut crate::src::headers::sqliteInt_h::Walker,
                                (*pIdx).aColExpr as *mut crate::src::headers::sqliteInt_h::ExprList,
                            );
                            pIdx = (*pIdx).pNext;
                        }
                        pIdx = sParse.pNewIndex;
                        while !pIdx.is_null() {
                            crate::src::src::walker::sqlite3WalkExprList(
                                &raw mut sWalker as *mut _
                                    as *mut crate::src::headers::sqliteInt_h::Walker,
                                (*pIdx).aColExpr as *mut crate::src::headers::sqliteInt_h::ExprList,
                            );
                            pIdx = (*pIdx).pNext;
                        }
                        i = 0 as ::core::ffi::c_int;
                        while i < (*sParse.pNewTable).nCol as ::core::ffi::c_int {
                            let pExpr: *mut crate::src::headers::sqliteInt_h::Expr =
                                crate::src::src::build::sqlite3ColumnExpr(
                                    sParse.pNewTable
                                        as *mut crate::src::headers::sqliteInt_h::Table,
                                    (*sParse.pNewTable).aCol.offset(i as isize)
                                        as *mut crate::src::headers::sqliteInt_h::Column
                                        as *mut crate::src::headers::sqliteInt_h::Column,
                                )
                                    as *mut crate::src::headers::sqliteInt_h::Expr;
                            crate::src::src::walker::sqlite3WalkExpr(
                                &raw mut sWalker as *mut _
                                    as *mut crate::src::headers::sqliteInt_h::Walker,
                                pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
                            );
                            i += 1;
                        }
                    }
                    pFKey = (*sParse.pNewTable).u.tab.pFKey;
                    while !pFKey.is_null() {
                        i = 0 as ::core::ffi::c_int;
                        while i < (*pFKey).nCol {
                            let __pFKey_ref = { &mut *pFKey };
                            if bFKOnly == 0 as ::core::ffi::c_int
                                && (*(&raw mut __pFKey_ref.aCol
                                    as *mut crate::src::headers::sqliteInt_h::sColMap)
                                    .offset(i as isize))
                                .iFrom
                                    == iCol
                            {
                                renameTokenFind(
                                    &raw mut sParse,
                                    &raw mut sCtx,
                                    (&raw mut __pFKey_ref.aCol
                                        as *mut crate::src::headers::sqliteInt_h::sColMap)
                                        .offset(i as isize)
                                        as *mut crate::src::headers::sqliteInt_h::sColMap
                                        as *mut ::core::ffi::c_void,
                                );
                            }
                            if 0 as ::core::ffi::c_int
                                == crate::src::src::util::sqlite3_stricmp(__pFKey_ref.zTo, zTable)
                                && 0 as ::core::ffi::c_int
                                    == crate::src::src::util::sqlite3_stricmp(
                                        (*(&raw mut __pFKey_ref.aCol
                                            as *mut crate::src::headers::sqliteInt_h::sColMap)
                                            .offset(i as isize))
                                        .zCol,
                                        zOld,
                                    )
                            {
                                renameTokenFind(
                                    &raw mut sParse,
                                    &raw mut sCtx,
                                    (*(&raw mut __pFKey_ref.aCol
                                        as *mut crate::src::headers::sqliteInt_h::sColMap)
                                        .offset(i as isize))
                                    .zCol
                                        as *mut ::core::ffi::c_void,
                                );
                            }
                            i += 1;
                        }
                        pFKey = (*pFKey).pNextFrom;
                    }
                }
                current_block = 17688141731389699982;
            }
        } else if !sParse.pNewIndex.is_null() {
            crate::src::src::walker::sqlite3WalkExprList(
                &raw mut sWalker as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
                (*sParse.pNewIndex).aColExpr as *mut crate::src::headers::sqliteInt_h::ExprList,
            );
            crate::src::src::walker::sqlite3WalkExpr(
                &raw mut sWalker as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
                (*sParse.pNewIndex).pPartIdxWhere as *mut crate::src::headers::sqliteInt_h::Expr,
            );
            current_block = 17688141731389699982;
        } else {
            let mut pStep: *mut crate::src::headers::sqliteInt_h::TriggerStep;
            rc = renameResolveTrigger(&raw mut sParse);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                current_block = 6633030875973123825;
            } else {
                pStep = (*sParse.pNewTrigger).step_list;
                while !pStep.is_null() {
                    if !(*pStep).zTarget.is_null() {
                        let pTarget: *mut crate::src::headers::sqliteInt_h::Table =
                            crate::src::src::build::sqlite3LocateTable(
                                &raw mut sParse as *mut _
                                    as *mut crate::src::headers::sqliteInt_h::Parse,
                                0 as crate::src::ext::rtree::rtree::U32_0,
                                (*pStep).zTarget,
                                zDb,
                            )
                                as *mut crate::src::headers::sqliteInt_h::Table;
                        if pTarget == pTab {
                            let __pStep_ref = { &mut *pStep };
                            if !__pStep_ref.pUpsert.is_null() {
                                let pUpsertSet: *mut crate::src::headers::sqliteInt_h::ExprList = (*__pStep_ref.pUpsert).pUpsertSet;
                                renameColumnElistNames(
                                    &raw mut sParse,
                                    &raw mut sCtx,
                                    pUpsertSet,
                                    zOld,
                                );
                            }
                            renameColumnIdlistNames(
                                &raw mut sParse,
                                &raw mut sCtx,
                                __pStep_ref.pIdList,
                                zOld,
                            );
                            renameColumnElistNames(
                                &raw mut sParse,
                                &raw mut sCtx,
                                __pStep_ref.pExprList,
                                zOld,
                            );
                        }
                    }
                    pStep = (*pStep).pNext;
                }
                if sParse.pTriggerTab == pTab {
                    renameColumnIdlistNames(
                        &raw mut sParse,
                        &raw mut sCtx,
                        (*sParse.pNewTrigger).pColumns,
                        zOld,
                    );
                }
                renameWalkTrigger(&raw mut sWalker, sParse.pNewTrigger);
                current_block = 17688141731389699982;
            }
        }
        match current_block {
            6633030875973123825 => {}
            _ => {
                rc = renameEditSql(context, &raw mut sCtx, zSql, zNew, bQuote);
            }
        }
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        if rc == crate::src::headers::sqlite3_h::SQLITE_ERROR
            && crate::src::src::build::sqlite3WritableSchema(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            ) != 0
        {
            crate::src::src::vdbeapi::sqlite3_result_value(context, *argv.offset(0_isize));
        } else if !sParse.zErrMsg.is_null() {
            renameColumnParseError(
                context,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(1_isize),
                *argv.offset(2_isize),
                &raw mut sParse,
            );
        } else {
            crate::src::src::vdbeapi::sqlite3_result_error_code(context, rc);
        }
    }
    renameParseCleanup(&raw mut sParse);
    renameTokenFree(db, sCtx.pList);
    __db_ref.xAuth = xAuth;
    crate::src::src::btmutex::sqlite3BtreeLeaveAll(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    );
}

unsafe extern "C" fn renameTableExprCb(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let p: *mut RenameCtx = (*pWalker).u.pRename as *mut RenameCtx;
    let __pExpr_ref = { &mut *pExpr };
    if __pExpr_ref.op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN
        && __pExpr_ref.flags
            & (0x1000000 as ::core::ffi::c_int | 0x2000000 as ::core::ffi::c_int)
                as crate::src::ext::rtree::rtree::U32_0
            == 0 as crate::src::ext::rtree::rtree::U32_0
        && (*p).pTab == __pExpr_ref.y.pTab
    {
        renameTokenFind(
            (*pWalker).pParse,
            p as *mut RenameCtx,
            &raw mut __pExpr_ref.y.pTab as *mut ::core::ffi::c_void,
        );
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}

unsafe extern "C" fn renameTableSelectCb(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pSelect: *mut crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let p: *mut RenameCtx = (*pWalker).u.pRename as *mut RenameCtx;
    let pSrc: *mut crate::src::headers::sqliteInt_h::SrcList = (*pSelect).pSrc;
    if (*pSelect).selFlags
        & (crate::src::headers::sqliteInt_h::SF_View | crate::src::headers::sqliteInt_h::SF_CopyCte)
            as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        return crate::src::headers::sqliteInt_h::WRC_Prune;
    }
    if pSrc.is_null() {
        return crate::src::headers::sqliteInt_h::WRC_Abort;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*pSrc).nSrc {
        let pItem: *mut crate::src::headers::sqliteInt_h::SrcItem =
            (&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                .offset(i as isize) as *mut crate::src::headers::sqliteInt_h::SrcItem;
        if (*pItem).pSTab == (*p).pTab {
            renameTokenFind(
                (*pWalker).pParse,
                p as *mut RenameCtx,
                (*pItem).zName as *const ::core::ffi::c_void,
            );
        }
        i += 1;
    }
    renameWalkWith(pWalker, pSelect);
    crate::src::headers::sqliteInt_h::WRC_Continue
}

unsafe extern "C" fn renameTableFunc(
    context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut _NotUsed: ::core::ffi::c_int,
    argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 =
        crate::src::src::vdbeapi::sqlite3_context_db_handle(context)
            as *mut crate::src::headers::sqliteInt_h::sqlite3;
    let zDb: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(0_isize))
            as *const ::core::ffi::c_char;
    let zInput: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(3_isize))
            as *const ::core::ffi::c_char;
    let zOld: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(4_isize))
            as *const ::core::ffi::c_char;
    let zNew: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(5_isize))
            as *const ::core::ffi::c_char;
    let bTemp: ::core::ffi::c_int =
        crate::src::src::vdbeapi::sqlite3_value_int(*argv.offset(6_isize));
    if !zInput.is_null() && !zOld.is_null() && !zNew.is_null() {
        let mut sParse: crate::src::headers::sqliteInt_h::Parse = { ::core::mem::zeroed() };
        let mut rc: ::core::ffi::c_int;
        let bQuote: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut sCtx: RenameCtx = { ::core::mem::zeroed() };
        let mut sWalker: crate::src::headers::sqliteInt_h::Walker = { ::core::mem::zeroed() };
        let __db_ref = { &mut *db };
        let xAuth: crate::src::headers::sqliteInt_h::Sqlite3Xauth = __db_ref.xAuth;
        __db_ref.xAuth = None;
        crate::src::src::btmutex::sqlite3BtreeEnterAll(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        );
        sCtx.pTab = crate::src::src::build::sqlite3FindTable(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            zOld,
            zDb,
        ) as *mut crate::src::headers::sqliteInt_h::Table;
        sWalker.pParse = &raw mut sParse;
        sWalker.xExprCallback = Some(
            renameTableExprCb
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqliteInt_h::Walker,
                    *mut crate::src::headers::sqliteInt_h::Expr,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::headers::sqliteInt_h::Walker,
                    *mut crate::src::headers::sqliteInt_h::Expr,
                ) -> ::core::ffi::c_int,
            >;
        sWalker.xSelectCallback = Some(
            renameTableSelectCb
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqliteInt_h::Walker,
                    *mut crate::src::headers::sqliteInt_h::Select,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::headers::sqliteInt_h::Walker,
                    *mut crate::src::headers::sqliteInt_h::Select,
                ) -> ::core::ffi::c_int,
            >;
        sWalker.u.pRename = &raw mut sCtx as *mut RenameCtx;
        rc = renameParseSql(&raw mut sParse, zDb, db, zInput, bTemp);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let isLegacy: ::core::ffi::c_int = (__db_ref.flags
                & crate::src::headers::sqliteInt_h::SQLITE_LegacyAlter
                    as crate::src::ext::rtree::rtree::U64_0)
                as ::core::ffi::c_int;
            if !sParse.pNewTable.is_null() {
                let pTab: *mut crate::src::headers::sqliteInt_h::Table = sParse.pNewTable;
                if (*pTab).eTabType as ::core::ffi::c_int
                    == crate::src::headers::sqliteInt_h::TABTYP_VIEW
                {
                    if isLegacy == 0 as ::core::ffi::c_int {
                        let pSelect: *mut crate::src::headers::sqliteInt_h::Select =
                            (*pTab).u.view.pSelect;
                        let mut sNC: crate::src::headers::sqliteInt_h::NameContext =
                            { ::core::mem::zeroed() };
                        sNC.pParse = &raw mut sParse;
                        (*pSelect).selFlags &= !(crate::src::headers::sqliteInt_h::SF_View
                            as crate::src::ext::rtree::rtree::U32_0);
                        crate::src::src::select::sqlite3SelectPrep(
                            &raw mut sParse as *mut _
                                as *mut crate::src::headers::sqliteInt_h::Parse,
                            (*pTab).u.view.pSelect as *mut crate::src::headers::sqliteInt_h::Select,
                            &raw mut sNC as *mut _
                                as *mut crate::src::headers::sqliteInt_h::NameContext,
                        );
                        if sParse.nErr != 0 {
                            rc = sParse.rc;
                        } else {
                            crate::src::src::walker::sqlite3WalkSelect(
                                &raw mut sWalker as *mut _
                                    as *mut crate::src::headers::sqliteInt_h::Walker,
                                (*pTab).u.view.pSelect
                                    as *mut crate::src::headers::sqliteInt_h::Select,
                            );
                        }
                    }
                } else {
                    if (isLegacy == 0 as ::core::ffi::c_int
                        || __db_ref.flags
                            & crate::src::headers::sqliteInt_h::SQLITE_ForeignKeys
                                as crate::src::ext::rtree::rtree::U64_0
                            != 0)
                        && ((*pTab).eTabType as ::core::ffi::c_int != crate::src::headers::sqliteInt_h::TABTYP_VTAB)
                    {
                        let mut pFKey: *mut crate::src::headers::sqliteInt_h::FKey;
                        pFKey = (*pTab).u.tab.pFKey;
                        while !pFKey.is_null() {
                            if crate::src::src::util::sqlite3_stricmp((*pFKey).zTo, zOld)
                                == 0 as ::core::ffi::c_int
                            {
                                renameTokenFind(
                                    &raw mut sParse,
                                    &raw mut sCtx,
                                    (*pFKey).zTo as *mut ::core::ffi::c_void,
                                );
                            }
                            pFKey = (*pFKey).pNextFrom;
                        }
                    }
                    if crate::src::src::util::sqlite3_stricmp(zOld, (*pTab).zName)
                        == 0 as ::core::ffi::c_int
                    {
                        sCtx.pTab = pTab;
                        if isLegacy == 0 as ::core::ffi::c_int {
                            crate::src::src::walker::sqlite3WalkExprList(
                                &raw mut sWalker as *mut _
                                    as *mut crate::src::headers::sqliteInt_h::Walker,
                                (*pTab).pCheck as *mut crate::src::headers::sqliteInt_h::ExprList,
                            );
                        }
                        renameTokenFind(
                            &raw mut sParse,
                            &raw mut sCtx,
                            (*pTab).zName as *const ::core::ffi::c_void,
                        );
                    }
                }
            } else if !sParse.pNewIndex.is_null() {
                renameTokenFind(
                    &raw mut sParse,
                    &raw mut sCtx,
                    (*sParse.pNewIndex).zName as *const ::core::ffi::c_void,
                );
                if isLegacy == 0 as ::core::ffi::c_int {
                    crate::src::src::walker::sqlite3WalkExpr(
                        &raw mut sWalker as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
                        (*sParse.pNewIndex).pPartIdxWhere
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                    );
                }
            } else {
                let pTrigger: *mut crate::src::headers::sqliteInt_h::Trigger =
                    sParse.pNewTrigger;
                let mut pStep: *mut crate::src::headers::sqliteInt_h::TriggerStep;
                if 0 as ::core::ffi::c_int
                    == crate::src::src::util::sqlite3_stricmp((*sParse.pNewTrigger).table, zOld)
                    && (*sCtx.pTab).pSchema == (*pTrigger).pTabSchema
                {
                    renameTokenFind(
                        &raw mut sParse,
                        &raw mut sCtx,
                        (*sParse.pNewTrigger).table as *const ::core::ffi::c_void,
                    );
                }
                if isLegacy == 0 as ::core::ffi::c_int {
                    rc = renameResolveTrigger(&raw mut sParse);
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        renameWalkTrigger(&raw mut sWalker, pTrigger);
                        pStep = (*pTrigger).step_list;
                        while !pStep.is_null() {
                            if !(*pStep).zTarget.is_null()
                                && 0 as ::core::ffi::c_int
                                    == crate::src::src::util::sqlite3_stricmp(
                                        (*pStep).zTarget,
                                        zOld,
                                    )
                            {
                                renameTokenFind(
                                    &raw mut sParse,
                                    &raw mut sCtx,
                                    (*pStep).zTarget as *const ::core::ffi::c_void,
                                );
                            }
                            if !(*pStep).pFrom.is_null() {
                                let mut i: ::core::ffi::c_int;
                                i = 0 as ::core::ffi::c_int;
                                while i < (*(*pStep).pFrom).nSrc {
                                    let pItem: *mut crate::src::headers::sqliteInt_h::SrcItem =
                                        (&raw mut (*(*pStep).pFrom).a
                                            as *mut crate::src::headers::sqliteInt_h::SrcItem)
                                            .offset(i as isize)
                                            as *mut crate::src::headers::sqliteInt_h::SrcItem;
                                    if 0 as ::core::ffi::c_int
                                        == crate::src::src::util::sqlite3_stricmp(
                                            (*pItem).zName,
                                            zOld,
                                        )
                                    {
                                        renameTokenFind(
                                            &raw mut sParse,
                                            &raw mut sCtx,
                                            (*pItem).zName as *const ::core::ffi::c_void,
                                        );
                                    }
                                    i += 1;
                                }
                            }
                            pStep = (*pStep).pNext;
                        }
                    }
                }
            }
        }
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = renameEditSql(context, &raw mut sCtx, zInput, zNew, bQuote);
        }
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            if rc == crate::src::headers::sqlite3_h::SQLITE_ERROR
                && crate::src::src::build::sqlite3WritableSchema(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                ) != 0
            {
                crate::src::src::vdbeapi::sqlite3_result_value(context, *argv.offset(3_isize));
            } else if !sParse.zErrMsg.is_null() {
                renameColumnParseError(
                    context,
                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                    *argv.offset(1_isize),
                    *argv.offset(2_isize),
                    &raw mut sParse,
                );
            } else {
                crate::src::src::vdbeapi::sqlite3_result_error_code(context, rc);
            }
        }
        renameParseCleanup(&raw mut sParse);
        renameTokenFree(db, sCtx.pList);
        crate::src::src::btmutex::sqlite3BtreeLeaveAll(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        );
        __db_ref.xAuth = xAuth;
    }
}

unsafe extern "C" fn renameQuotefixExprCb(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_STRING
        && (*pExpr).flags
            & crate::src::headers::sqliteInt_h::EP_DblQuoted as crate::src::ext::rtree::rtree::U32_0
            != 0
    {
        renameTokenFind(
            (*pWalker).pParse,
            (*pWalker).u.pRename as *mut RenameCtx,
            pExpr as *const ::core::ffi::c_void,
        );
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}

unsafe extern "C" fn renameQuotefixFunc(
    context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut _NotUsed: ::core::ffi::c_int,
    argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 =
        crate::src::src::vdbeapi::sqlite3_context_db_handle(context)
            as *mut crate::src::headers::sqliteInt_h::sqlite3;
    let zDb: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(0_isize))
            as *const ::core::ffi::c_char;
    let zInput: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(1_isize))
            as *const ::core::ffi::c_char;
    let __db_ref = { &mut *db };
    let xAuth: crate::src::headers::sqliteInt_h::Sqlite3Xauth = __db_ref.xAuth;
    __db_ref.xAuth = None;
    crate::src::src::btmutex::sqlite3BtreeEnterAll(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    );
    if !zDb.is_null() && !zInput.is_null() {
        let mut rc: ::core::ffi::c_int;
        let mut sParse: crate::src::headers::sqliteInt_h::Parse = { ::core::mem::zeroed() };
        rc = renameParseSql(&raw mut sParse, zDb, db, zInput, 0 as ::core::ffi::c_int);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            let mut sCtx: RenameCtx = { ::core::mem::zeroed() };
            let mut sWalker: crate::src::headers::sqliteInt_h::Walker = { ::core::mem::zeroed() };
            sWalker.pParse = &raw mut sParse;
            sWalker.xExprCallback = Some(
                renameQuotefixExprCb
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqliteInt_h::Walker,
                        *mut crate::src::headers::sqliteInt_h::Expr,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::sqliteInt_h::Walker,
                        *mut crate::src::headers::sqliteInt_h::Expr,
                    ) -> ::core::ffi::c_int,
                >;
            sWalker.xSelectCallback = Some(
                renameColumnSelectCb
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqliteInt_h::Walker,
                        *mut crate::src::headers::sqliteInt_h::Select,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::headers::sqliteInt_h::Walker,
                        *mut crate::src::headers::sqliteInt_h::Select,
                    ) -> ::core::ffi::c_int,
                >;
            sWalker.u.pRename = &raw mut sCtx as *mut RenameCtx;
            if !sParse.pNewTable.is_null() {
                if (*sParse.pNewTable).eTabType as ::core::ffi::c_int
                    == crate::src::headers::sqliteInt_h::TABTYP_VIEW
                {
                    let pSelect: *mut crate::src::headers::sqliteInt_h::Select =
                        (*sParse.pNewTable).u.view.pSelect;
                    (*pSelect).selFlags &= !(crate::src::headers::sqliteInt_h::SF_View
                        as crate::src::ext::rtree::rtree::U32_0);
                    sParse.rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                    crate::src::src::select::sqlite3SelectPrep(
                        &raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse,
                        pSelect as *mut crate::src::headers::sqliteInt_h::Select,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::NameContext>()
                            as *mut crate::src::headers::sqliteInt_h::NameContext,
                    );
                    rc = if __db_ref.mallocFailed as ::core::ffi::c_int != 0 {
                        crate::src::headers::sqlite3_h::SQLITE_NOMEM
                    } else {
                        sParse.rc
                    };
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        crate::src::src::walker::sqlite3WalkSelect(
                            &raw mut sWalker as *mut _
                                as *mut crate::src::headers::sqliteInt_h::Walker,
                            pSelect as *mut crate::src::headers::sqliteInt_h::Select,
                        );
                    }
                } else {
                    let mut i: ::core::ffi::c_int;
                    crate::src::src::walker::sqlite3WalkExprList(
                        &raw mut sWalker as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
                        (*sParse.pNewTable).pCheck
                            as *mut crate::src::headers::sqliteInt_h::ExprList,
                    );
                    i = 0 as ::core::ffi::c_int;
                    while i < (*sParse.pNewTable).nCol as ::core::ffi::c_int {
                        crate::src::src::walker::sqlite3WalkExpr(
                            &raw mut sWalker as *mut _
                                as *mut crate::src::headers::sqliteInt_h::Walker,
                            crate::src::src::build::sqlite3ColumnExpr(
                                sParse.pNewTable as *mut crate::src::headers::sqliteInt_h::Table,
                                (*sParse.pNewTable).aCol.offset(i as isize)
                                    as *mut crate::src::headers::sqliteInt_h::Column
                                    as *mut crate::src::headers::sqliteInt_h::Column,
                            )
                                as *mut crate::src::headers::sqliteInt_h::Expr
                                as *mut crate::src::headers::sqliteInt_h::Expr,
                        );
                        i += 1;
                    }
                }
            } else if !sParse.pNewIndex.is_null() {
                crate::src::src::walker::sqlite3WalkExprList(
                    &raw mut sWalker as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
                    (*sParse.pNewIndex).aColExpr as *mut crate::src::headers::sqliteInt_h::ExprList,
                );
                crate::src::src::walker::sqlite3WalkExpr(
                    &raw mut sWalker as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
                    (*sParse.pNewIndex).pPartIdxWhere
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                );
            } else {
                rc = renameResolveTrigger(&raw mut sParse);
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    renameWalkTrigger(&raw mut sWalker, sParse.pNewTrigger);
                }
            }
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = renameEditSql(
                    context,
                    &raw mut sCtx,
                    zInput,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    0 as ::core::ffi::c_int,
                );
            }
            renameTokenFree(db, sCtx.pList);
        }
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            if crate::src::src::build::sqlite3WritableSchema(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            ) != 0
                && rc == crate::src::headers::sqlite3_h::SQLITE_ERROR
            {
                crate::src::src::vdbeapi::sqlite3_result_value(context, *argv.offset(1_isize));
            } else {
                crate::src::src::vdbeapi::sqlite3_result_error_code(context, rc);
            }
        }
        renameParseCleanup(&raw mut sParse);
    }
    __db_ref.xAuth = xAuth;
    crate::src::src::btmutex::sqlite3BtreeLeaveAll(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    );
}

unsafe extern "C" fn renameTableTest(
    context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut _NotUsed: ::core::ffi::c_int,
    argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 =
        crate::src::src::vdbeapi::sqlite3_context_db_handle(context)
            as *mut crate::src::headers::sqliteInt_h::sqlite3;
    let zDb: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(0_isize))
            as *const ::core::ffi::c_char;
    let zInput: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(1_isize))
            as *const ::core::ffi::c_char;
    let bTemp: ::core::ffi::c_int =
        crate::src::src::vdbeapi::sqlite3_value_int(*argv.offset(4_isize));
    let __db_ref = { &mut *db };
    let isLegacy: ::core::ffi::c_int = (__db_ref.flags
        & crate::src::headers::sqliteInt_h::SQLITE_LegacyAlter
            as crate::src::ext::rtree::rtree::U64_0)
        as ::core::ffi::c_int;
    let zWhen: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(5_isize))
            as *const ::core::ffi::c_char;
    let bNoDQS: ::core::ffi::c_int =
        crate::src::src::vdbeapi::sqlite3_value_int(*argv.offset(6_isize));
    let xAuth: crate::src::headers::sqliteInt_h::Sqlite3Xauth = __db_ref.xAuth;
    __db_ref.xAuth = None;
    if !zDb.is_null() && !zInput.is_null() {
        let mut rc: ::core::ffi::c_int;
        let mut sParse: crate::src::headers::sqliteInt_h::Parse = { ::core::mem::zeroed() };
        let flags: crate::src::ext::rtree::rtree::U64_0 = __db_ref.flags;
        if bNoDQS != 0 {
            __db_ref.flags &= !(crate::src::headers::sqliteInt_h::SQLITE_DqsDML
                | crate::src::headers::sqliteInt_h::SQLITE_DqsDDL)
                as crate::src::ext::rtree::rtree::U64_0;
        }
        rc = renameParseSql(&raw mut sParse, zDb, db, zInput, bTemp);
        __db_ref.flags = flags;
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            if isLegacy == 0 as ::core::ffi::c_int
                && !sParse.pNewTable.is_null()
                && (*sParse.pNewTable).eTabType as ::core::ffi::c_int
                    == crate::src::headers::sqliteInt_h::TABTYP_VIEW
            {
                let mut sNC: crate::src::headers::sqliteInt_h::NameContext =
                    { ::core::mem::zeroed() };
                sNC.pParse = &raw mut sParse;
                crate::src::src::select::sqlite3SelectPrep(
                    &raw mut sParse as *mut _ as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*sParse.pNewTable).u.view.pSelect
                        as *mut crate::src::headers::sqliteInt_h::Select,
                    &raw mut sNC as *mut _ as *mut crate::src::headers::sqliteInt_h::NameContext,
                );
                if sParse.nErr != 0 {
                    rc = sParse.rc;
                }
            } else if !sParse.pNewTrigger.is_null() {
                if isLegacy == 0 as ::core::ffi::c_int {
                    rc = renameResolveTrigger(&raw mut sParse);
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    let i1: ::core::ffi::c_int = crate::src::src::prepare::sqlite3SchemaToIndex(
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        (*sParse.pNewTrigger).pTabSchema
                            as *mut crate::src::headers::sqliteInt_h::Schema,
                    );
                    let i2: ::core::ffi::c_int = crate::src::src::build::sqlite3FindDbName(
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        zDb,
                    );
                    if i1 == i2 {
                        crate::src::src::vdbeapi::sqlite3_result_int(
                            context,
                            1 as ::core::ffi::c_int,
                        );
                    }
                }
            }
        }
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK
            && !zWhen.is_null()
            && crate::src::src::build::sqlite3WritableSchema(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            ) == 0
        {
            renameColumnParseError(
                context,
                zWhen,
                *argv.offset(2_isize),
                *argv.offset(3_isize),
                &raw mut sParse,
            );
        }
        renameParseCleanup(&raw mut sParse);
    }
    __db_ref.xAuth = xAuth;
}

unsafe extern "C" fn dropColumnFunc(
    context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut _NotUsed: ::core::ffi::c_int,
    argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 =
        crate::src::src::vdbeapi::sqlite3_context_db_handle(context)
            as *mut crate::src::headers::sqliteInt_h::sqlite3;
    let iSchema: ::core::ffi::c_int =
        crate::src::src::vdbeapi::sqlite3_value_int(*argv.offset(0_isize));
    let zSql: *const ::core::ffi::c_char =
        crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(1_isize))
            as *const ::core::ffi::c_char;
    let iCol: ::core::ffi::c_int =
        crate::src::src::vdbeapi::sqlite3_value_int(*argv.offset(2_isize));
    let __db_ref = { &mut *db };
    let zDb: *const ::core::ffi::c_char = (*__db_ref.aDb.offset(iSchema as isize)).zDbSName;
    let mut rc: ::core::ffi::c_int;
    let mut sParse: crate::src::headers::sqliteInt_h::Parse = { ::core::mem::zeroed() };
    let pCol: *mut RenameToken;
    let pTab: *mut crate::src::headers::sqliteInt_h::Table;
    let zEnd: *const ::core::ffi::c_char;
    let zNew: *mut ::core::ffi::c_char;
    let xAuth: crate::src::headers::sqliteInt_h::Sqlite3Xauth = __db_ref.xAuth;
    __db_ref.xAuth = None;
    rc = renameParseSql(
        &raw mut sParse,
        zDb,
        db,
        zSql,
        (iSchema == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    if (rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
        pTab = sParse.pNewTable;
        if pTab.is_null()
            || (*pTab).nCol as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            || iCol >= (*pTab).nCol as ::core::ffi::c_int
        {
            rc = crate::src::src::main::sqlite3CorruptError(2135 as ::core::ffi::c_int);
        } else {
            pCol = renameTokenFind(
                &raw mut sParse,
                ::core::ptr::null_mut::<RenameCtx>(),
                (*(*pTab).aCol.offset(iCol as isize)).zCnName as *mut ::core::ffi::c_void,
            );
            if iCol < (*pTab).nCol as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
                
                let pEnd: *mut RenameToken = renameTokenFind(
                    &raw mut sParse,
                    ::core::ptr::null_mut::<RenameCtx>(),
                    (*(*pTab)
                        .aCol
                        .offset((iCol + 1 as ::core::ffi::c_int) as isize))
                    .zCnName as *mut ::core::ffi::c_void,
                );
                zEnd = (*pEnd).t.z;
            } else {
                zEnd =
                    zSql.offset((*pTab).u.tab.addColOffset as isize) as *const ::core::ffi::c_char;
                while *(*pCol).t.z.offset(0_isize) as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                    && *(*pCol).t.z.offset(0_isize) as ::core::ffi::c_int != ',' as i32
                {
                    (*pCol).t.z = (*pCol).t.z.offset(-1);
                }
            }
            zNew = crate::src::printf_c_variadic::sqlite3MPrintf_args(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                b"%.*s%s\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::src::src::printf::PrintfArg::Int(
                        (*pCol).t.z.offset_from(zSql) as crate::src::ext::rtree::rtree::I64_0
                    ),
                    crate::src::src::printf::PrintfArg::Str(zSql as *mut ::core::ffi::c_char),
                    crate::src::src::printf::PrintfArg::Str(zEnd as *mut ::core::ffi::c_char),
                ],
            );
            crate::src::src::vdbeapi::sqlite3_result_text(
                context,
                zNew,
                -(1 as ::core::ffi::c_int),
                ::core::mem::transmute::<
                    crate::src::headers::stdlib::IntptrT,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
            );
            crate::src::src::malloc::sqlite3_free(zNew as *mut ::core::ffi::c_void);
        }
    }
    renameParseCleanup(&raw mut sParse);
    __db_ref.xAuth = xAuth;
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_result_error_code(context, rc);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3AlterDropColumn(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pSrc: *mut crate::src::headers::sqliteInt_h::SrcList,
    pName: *const crate::src::headers::sqliteInt_h::Token,
) {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    let pTab: *mut crate::src::headers::sqliteInt_h::Table;
    let iDb: ::core::ffi::c_int;
    let zDb: *const ::core::ffi::c_char;
    let mut zCol: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let iCol: ::core::ffi::c_int;
    if ((*db).mallocFailed == 0) {
        pTab = crate::src::src::build::sqlite3LocateTableItem(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            0 as crate::src::ext::rtree::rtree::U32_0,
            (&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                .offset(0_isize) as *mut crate::src::headers::sqliteInt_h::SrcItem
                as *mut crate::src::headers::sqliteInt_h::SrcItem,
        ) as *mut crate::src::headers::sqliteInt_h::Table;
        if !pTab.is_null()
            && (crate::src::headers::sqlite3_h::SQLITE_OK == isAlterableTable(pParse, pTab))
                && (crate::src::headers::sqlite3_h::SQLITE_OK == isRealTable(pParse, pTab, 1 as ::core::ffi::c_int))
                {
                    zCol = crate::src::src::build::sqlite3NameFromToken(
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        pName as *const crate::src::headers::sqliteInt_h::Token,
                    );
                    if !zCol.is_null() {
                        iCol = crate::src::src::select::sqlite3ColumnIndex(
                            pTab as *mut crate::src::headers::sqliteInt_h::Table,
                            zCol,
                        );
                        if iCol < 0 as ::core::ffi::c_int {
                            crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                b"no such column: \"%T\"\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[crate::src::src::printf::PrintfArg::Token(
                                    pName as *mut crate::src::headers::sqliteInt_h::Token,
                                )],
                            );
                        } else if (*(*pTab).aCol.offset(iCol as isize)).colFlags
                            as ::core::ffi::c_int
                            & (crate::src::headers::sqliteInt_h::COLFLAG_PRIMKEY
                                | crate::src::headers::sqliteInt_h::COLFLAG_UNIQUE)
                            != 0
                        {
                            crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                b"cannot drop %s column: \"%s\"\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[
                                    crate::src::src::printf::PrintfArg::Str(if (*(*pTab)
                                        .aCol
                                        .offset(iCol as isize))
                                    .colFlags
                                        as ::core::ffi::c_int
                                        & crate::src::headers::sqliteInt_h::COLFLAG_PRIMKEY
                                        != 0
                                    {
                                        b"PRIMARY KEY\0" as *const u8 as *const ::core::ffi::c_char
                                    } else {
                                        b"UNIQUE\0" as *const u8 as *const ::core::ffi::c_char
                                    }
                                        as *mut ::core::ffi::c_char),
                                    crate::src::src::printf::PrintfArg::Str(
                                        zCol as *mut ::core::ffi::c_char,
                                    ),
                                ],
                            );
                        } else if (*pTab).nCol as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
                            crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                b"cannot drop column \"%s\": no other columns exist\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[crate::src::src::printf::PrintfArg::Str(
                                    zCol as *mut ::core::ffi::c_char,
                                )],
                            );
                        } else {
                            iDb = crate::src::src::prepare::sqlite3SchemaToIndex(
                                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                (*pTab).pSchema as *mut crate::src::headers::sqliteInt_h::Schema,
                            );
                            zDb = (*(*db).aDb.offset(iDb as isize)).zDbSName;
                            if (crate::src::src::auth::sqlite3AuthCheck(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                crate::src::headers::sqlite3_h::SQLITE_ALTER_TABLE,
                                zDb,
                                (*pTab).zName,
                                zCol,
                            ) == 0)
                            {
                                renameTestSchema(
                                    pParse,
                                    zDb,
                                    (iDb == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                                    0 as ::core::ffi::c_int,
                                );
                                renameFixQuotes(
                                    pParse,
                                    zDb,
                                    (iDb == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                );
                                crate::src::src::build::sqlite3NestedParse_args(
                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                    "UPDATE \"%w\".sqlite_master SET sql = sqlite_drop_column(%d, sql, %d) WHERE (type=='table' AND tbl_name=%Q COLLATE nocase)",
                                    crate::printf_args_slice!(zDb, iDb, iCol, (*pTab).zName),
                                );
                                renameReloadSchema(
                                    pParse,
                                    iDb,
                                    crate::src::headers::sqliteInt_h::INITFLAG_AlterDrop
                                        as crate::src::fts5::U16_0,
                                );
                                renameTestSchema(
                                    pParse,
                                    zDb,
                                    (iDb == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                    b"after drop column\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int,
                                );
                                if (*pParse).nErr == 0 as ::core::ffi::c_int
                                    && (*(*pTab).aCol.offset(iCol as isize)).colFlags
                                        as ::core::ffi::c_int
                                        & crate::src::headers::sqliteInt_h::COLFLAG_VIRTUAL
                                        == 0 as ::core::ffi::c_int
                                {
                                    let mut i: ::core::ffi::c_int;
                                    
                                    
                                    
                                    let mut pPk: *mut crate::src::headers::sqliteInt_h::Index =
                                        ::core::ptr::null_mut::<
                                            crate::src::headers::sqliteInt_h::Index,
                                        >();
                                    let mut nField: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                    
                                    let v: *mut crate::src::headers::vdbeInt_h::Vdbe =
                                        crate::src::src::select::sqlite3GetVdbe(
                                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                        );
                                    let __pParse_ref = { &mut *pParse };
                                    let fresh2 = __pParse_ref.nTab;
                                    __pParse_ref.nTab += 1;
                                    let iCur: ::core::ffi::c_int = fresh2;
                                    crate::src::src::insert::sqlite3OpenTable(
                                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                        iCur,
                                        iDb,
                                        pTab as *mut crate::src::headers::sqliteInt_h::Table,
                                        crate::src::headers::opcodes_h::OP_OpenWrite,
                                    );
                                    let addr: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                                        v,
                                        crate::src::headers::opcodes_h::OP_Rewind,
                                        iCur,
                                    );
                                    __pParse_ref.nMem += 1;
                                    let reg: ::core::ffi::c_int = __pParse_ref.nMem;
                                    if (*pTab).tabFlags
                                        & crate::src::headers::sqliteInt_h::TF_WithoutRowid
                                            as crate::src::ext::rtree::rtree::U32_0
                                        == 0 as crate::src::ext::rtree::rtree::U32_0
                                    {
                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                            v,
                                            crate::src::headers::opcodes_h::OP_Rowid,
                                            iCur,
                                            reg,
                                        );
                                        __pParse_ref.nMem += (*pTab).nCol as ::core::ffi::c_int;
                                    } else {
                                        pPk = crate::src::src::build::sqlite3PrimaryKeyIndex(
                                            pTab as *mut crate::src::headers::sqliteInt_h::Table,
                                        )
                                            as *mut crate::src::headers::sqliteInt_h::Index;
                                        __pParse_ref.nMem += (*pPk).nColumn as ::core::ffi::c_int;
                                        i = 0 as ::core::ffi::c_int;
                                        while i < (*pPk).nKeyCol as ::core::ffi::c_int {
                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                v,
                                                crate::src::headers::opcodes_h::OP_Column,
                                                iCur,
                                                i,
                                                reg + i + 1 as ::core::ffi::c_int,
                                            );
                                            i += 1;
                                        }
                                        nField = (*pPk).nKeyCol as ::core::ffi::c_int;
                                    }
                                    __pParse_ref.nMem += 1;
                                    let regRec: ::core::ffi::c_int = __pParse_ref.nMem;
                                    let mut current_block_48: u64;
                                    i = 0 as ::core::ffi::c_int;
                                    while i < (*pTab).nCol as ::core::ffi::c_int {
                                        if i != iCol
                                            && (*(*pTab).aCol.offset(i as isize)).colFlags
                                                as ::core::ffi::c_int
                                                & crate::src::headers::sqliteInt_h::COLFLAG_VIRTUAL
                                                == 0 as ::core::ffi::c_int
                                        {
                                            let mut regOut: ::core::ffi::c_int = 0;
                                            if !pPk.is_null() {
                                                let iPos: ::core::ffi::c_int =
                                                    crate::src::src::build::sqlite3TableColumnToIndex(pPk as *mut crate::src::headers::sqliteInt_h::Index, i);
                                                let iColPos: ::core::ffi::c_int =
                                                    crate::src::src::build::sqlite3TableColumnToIndex(pPk as *mut crate::src::headers::sqliteInt_h::Index, iCol);
                                                if iPos < (*pPk).nKeyCol as ::core::ffi::c_int {
                                                    current_block_48 = 10891380440665537214;
                                                } else {
                                                    regOut = reg + 1 as ::core::ffi::c_int + iPos
                                                        - (iPos > iColPos) as ::core::ffi::c_int;
                                                    current_block_48 = 2989495919056355252;
                                                }
                                            } else {
                                                regOut = reg + 1 as ::core::ffi::c_int + nField;
                                                current_block_48 = 2989495919056355252;
                                            }
                                            match current_block_48 {
                                                10891380440665537214 => {}
                                                _ => {
                                                    if i == (*pTab).iPKey as ::core::ffi::c_int {
                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                            v,
                                                            crate::src::headers::opcodes_h::OP_Null,
                                                            0 as ::core::ffi::c_int,
                                                            regOut,
                                                        );
                                                    } else {
                                                        let aff: ::core::ffi::c_char =
                                                            (*(*pTab).aCol.offset(i as isize))
                                                                .affinity;
                                                        if aff as ::core::ffi::c_int
                                                            == crate::src::headers::sqliteInt_h::SQLITE_AFF_REAL
                                                        {
                                                            (*(*pTab).aCol.offset(i as isize))
                                                                .affinity = crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC
                                                                as ::core::ffi::c_char;
                                                        }
                                                        crate::src::src::expr::sqlite3ExprCodeGetColumnOfTable(
                                                            v,  pTab as *mut crate::src::headers::sqliteInt_h::Table, iCur, i, regOut,
                                                        );
                                                        (*(*pTab).aCol.offset(i as isize))
                                                            .affinity = aff;
                                                    }
                                                    nField += 1;
                                                }
                                            }
                                        }
                                        i += 1;
                                    }
                                    if nField == 0 as ::core::ffi::c_int {
                                        __pParse_ref.nMem += 1;
                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                            v,
                                            crate::src::headers::opcodes_h::OP_Null,
                                            0 as ::core::ffi::c_int,
                                            reg + 1 as ::core::ffi::c_int,
                                        );
                                        nField = 1 as ::core::ffi::c_int;
                                    }
                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                        v,
                                        crate::src::headers::opcodes_h::OP_MakeRecord,
                                        reg + 1 as ::core::ffi::c_int,
                                        nField,
                                        regRec,
                                    );
                                    if !pPk.is_null() {
                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                                            v,
                                            crate::src::headers::opcodes_h::OP_IdxInsert,
                                            iCur,
                                            regRec,
                                            reg + 1 as ::core::ffi::c_int,
                                            (*pPk).nKeyCol as ::core::ffi::c_int,
                                        );
                                    } else {
                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                            v,
                                            crate::src::headers::opcodes_h::OP_Insert,
                                            iCur,
                                            regRec,
                                            reg,
                                        );
                                    }
                                    crate::src::src::vdbeaux::sqlite3VdbeChangeP5(
                                        v,
                                        crate::src::headers::sqliteInt_h::OPFLAG_SAVEPOSITION
                                            as crate::src::fts5::U16_0,
                                    );
                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                        v,
                                        crate::src::headers::opcodes_h::OP_Next,
                                        iCur,
                                        addr + 1 as ::core::ffi::c_int,
                                    );
                                    crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addr);
                                }
                            }
                        }
                    }
                }
    }
    crate::src::src::malloc::sqlite3DbFree(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        zCol as *mut ::core::ffi::c_void,
    );
    crate::src::src::build::sqlite3SrcListDelete(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        pSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
    );
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3AlterFunctions() {
    static mut aAlterTableFuncs: [crate::src::headers::sqliteInt_h::FuncDef; 5] = {
        [
            crate::src::headers::sqliteInt_h::FuncDef {
                nArg: 9 as crate::src::fts5::I16_0,
                funcFlags: (crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_INTERNAL
                    | crate::src::headers::sqlite3_h::SQLITE_UTF8
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT)
                    as crate::src::ext::rtree::rtree::U32_0,
                pUserData: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
                pNext: ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>()
                    as *mut crate::src::headers::sqliteInt_h::FuncDef,
                xSFunc: Some(
                    renameColumnFunc
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
                xFinalize: None,
                xValue: None,
                xInverse: None,
                zName: b"sqlite_rename_column\0" as *const u8 as *const ::core::ffi::c_char,
                u: crate::src::headers::sqliteInt_h::__anon_union_2 {
                    pHash: ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>()
                        as *mut crate::src::headers::sqliteInt_h::FuncDef,
                },
            },
            crate::src::headers::sqliteInt_h::FuncDef {
                nArg: 7 as crate::src::fts5::I16_0,
                funcFlags: (crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_INTERNAL
                    | crate::src::headers::sqlite3_h::SQLITE_UTF8
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT)
                    as crate::src::ext::rtree::rtree::U32_0,
                pUserData: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
                pNext: ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>()
                    as *mut crate::src::headers::sqliteInt_h::FuncDef,
                xSFunc: Some(
                    renameTableFunc
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
                xFinalize: None,
                xValue: None,
                xInverse: None,
                zName: b"sqlite_rename_table\0" as *const u8 as *const ::core::ffi::c_char,
                u: crate::src::headers::sqliteInt_h::__anon_union_2 {
                    pHash: ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>()
                        as *mut crate::src::headers::sqliteInt_h::FuncDef,
                },
            },
            crate::src::headers::sqliteInt_h::FuncDef {
                nArg: 7 as crate::src::fts5::I16_0,
                funcFlags: (crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_INTERNAL
                    | crate::src::headers::sqlite3_h::SQLITE_UTF8
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT)
                    as crate::src::ext::rtree::rtree::U32_0,
                pUserData: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
                pNext: ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>()
                    as *mut crate::src::headers::sqliteInt_h::FuncDef,
                xSFunc: Some(
                    renameTableTest
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
                xFinalize: None,
                xValue: None,
                xInverse: None,
                zName: b"sqlite_rename_test\0" as *const u8 as *const ::core::ffi::c_char,
                u: crate::src::headers::sqliteInt_h::__anon_union_2 {
                    pHash: ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>()
                        as *mut crate::src::headers::sqliteInt_h::FuncDef,
                },
            },
            crate::src::headers::sqliteInt_h::FuncDef {
                nArg: 3 as crate::src::fts5::I16_0,
                funcFlags: (crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_INTERNAL
                    | crate::src::headers::sqlite3_h::SQLITE_UTF8
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT)
                    as crate::src::ext::rtree::rtree::U32_0,
                pUserData: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
                pNext: ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>()
                    as *mut crate::src::headers::sqliteInt_h::FuncDef,
                xSFunc: Some(
                    dropColumnFunc
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
                xFinalize: None,
                xValue: None,
                xInverse: None,
                zName: b"sqlite_drop_column\0" as *const u8 as *const ::core::ffi::c_char,
                u: crate::src::headers::sqliteInt_h::__anon_union_2 {
                    pHash: ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>()
                        as *mut crate::src::headers::sqliteInt_h::FuncDef,
                },
            },
            crate::src::headers::sqliteInt_h::FuncDef {
                nArg: 2 as crate::src::fts5::I16_0,
                funcFlags: (crate::src::headers::sqliteInt_h::SQLITE_FUNC_BUILTIN
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_INTERNAL
                    | crate::src::headers::sqlite3_h::SQLITE_UTF8
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT)
                    as crate::src::ext::rtree::rtree::U32_0,
                pUserData: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
                pNext: ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>()
                    as *mut crate::src::headers::sqliteInt_h::FuncDef,
                xSFunc: Some(
                    renameQuotefixFunc
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
                xFinalize: None,
                xValue: None,
                xInverse: None,
                zName: b"sqlite_rename_quotefix\0" as *const u8 as *const ::core::ffi::c_char,
                u: crate::src::headers::sqliteInt_h::__anon_union_2 {
                    pHash: ::core::ptr::null::<crate::src::headers::sqliteInt_h::FuncDef>()
                        as *mut crate::src::headers::sqliteInt_h::FuncDef,
                },
            },
        ]
    };
    crate::src::src::callback::sqlite3InsertBuiltinFuncs(
        &raw mut aAlterTableFuncs as *mut crate::src::headers::sqliteInt_h::FuncDef
            as *mut crate::src::headers::sqliteInt_h::FuncDef,
        (::core::mem::size_of::<[crate::src::headers::sqliteInt_h::FuncDef; 5]>() as usize)
            .wrapping_div(
                ::core::mem::size_of::<crate::src::headers::sqliteInt_h::FuncDef>() as usize,
            ) as ::core::ffi::c_int,
    );
}
