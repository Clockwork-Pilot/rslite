pub use crate::__stddef_size_t_h::SizeT;

pub use crate::src::headers::opcodes_h::OP_AddImm;
pub use crate::src::headers::opcodes_h::OP_Affinity;
pub use crate::src::headers::opcodes_h::OP_And;
pub use crate::src::headers::opcodes_h::OP_BeginSubrtn;
pub use crate::src::headers::opcodes_h::OP_BitAnd;
pub use crate::src::headers::opcodes_h::OP_Blob;
pub use crate::src::headers::opcodes_h::OP_Cast;
pub use crate::src::headers::opcodes_h::OP_ClrSubtype;
pub use crate::src::headers::opcodes_h::OP_CollSeq;
pub use crate::src::headers::opcodes_h::OP_Column;
pub use crate::src::headers::opcodes_h::OP_Copy;
pub use crate::src::headers::opcodes_h::OP_ElseEq;
pub use crate::src::headers::opcodes_h::OP_Eq;
pub use crate::src::headers::opcodes_h::OP_Filter;
pub use crate::src::headers::opcodes_h::OP_Found;
pub use crate::src::headers::opcodes_h::OP_Gosub;
pub use crate::src::headers::opcodes_h::OP_Goto;
pub use crate::src::headers::opcodes_h::OP_Halt;
pub use crate::src::headers::opcodes_h::OP_IdxInsert;
pub use crate::src::headers::opcodes_h::OP_If;
pub use crate::src::headers::opcodes_h::OP_IfNot;
pub use crate::src::headers::opcodes_h::OP_IfNullRow;
pub use crate::src::headers::opcodes_h::OP_Int64;
pub use crate::src::headers::opcodes_h::OP_Integer;
pub use crate::src::headers::opcodes_h::OP_IsNull;
pub use crate::src::headers::opcodes_h::OP_IsTrue;
pub use crate::src::headers::opcodes_h::OP_MakeRecord;
pub use crate::src::headers::opcodes_h::OP_Move;
pub use crate::src::headers::opcodes_h::OP_Ne;
pub use crate::src::headers::opcodes_h::OP_Next;
pub use crate::src::headers::opcodes_h::OP_Not;
pub use crate::src::headers::opcodes_h::OP_NotFound;
pub use crate::src::headers::opcodes_h::OP_NotNull;
pub use crate::src::headers::opcodes_h::OP_Null;
pub use crate::src::headers::opcodes_h::OP_NullRow;
pub use crate::src::headers::opcodes_h::OP_Once;
pub use crate::src::headers::opcodes_h::OP_OpenDup;
pub use crate::src::headers::opcodes_h::OP_OpenEphemeral;
pub use crate::src::headers::opcodes_h::OP_OpenRead;
pub use crate::src::headers::opcodes_h::OP_Or;
pub use crate::src::headers::opcodes_h::OP_Param;
pub use crate::src::headers::opcodes_h::OP_Real;
pub use crate::src::headers::opcodes_h::OP_RealAffinity;
pub use crate::src::headers::opcodes_h::OP_Return;
pub use crate::src::headers::opcodes_h::OP_Rewind;
pub use crate::src::headers::opcodes_h::OP_Rowid;
pub use crate::src::headers::opcodes_h::OP_SCopy;
pub use crate::src::headers::opcodes_h::OP_SeekRowid;
pub use crate::src::headers::opcodes_h::OP_Subtract;
pub use crate::src::headers::opcodes_h::OP_TypeCheck;
pub use crate::src::headers::opcodes_h::OP_VColumn;
pub use crate::src::headers::opcodes_h::OP_Variable;
pub use crate::src::headers::opcodes_h::OP_ZeroOrNull;
pub use crate::src::src::hash::_ht;
pub use crate::src::src::hash::Hash;
pub use crate::src::src::hash::HashElem;
pub use crate::src::src::pager::Pgno;

pub use crate::fts3Int_h::LARGEST_INT64;
pub use crate::fts3Int_h::SMALLEST_INT64;
pub use crate::src::ext::rtree::rtree::I64_0;
pub use crate::src::ext::rtree::rtree::U8_0;
pub use crate::src::ext::rtree::rtree::U32_0;
pub use crate::src::ext::rtree::rtree::U64_0;
pub use crate::src::fts5::I16_0;
pub use crate::src::fts5::U16_0;
pub use crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT;
pub use crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT_TRIGGER;
pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;
pub use crate::src::headers::sqlite3_h::SQLITE_INTEGER;
pub use crate::src::headers::sqlite3_h::SQLITE_LIMIT_COLUMN;
pub use crate::src::headers::sqlite3_h::SQLITE_LIMIT_EXPR_DEPTH;
pub use crate::src::headers::sqlite3_h::SQLITE_LIMIT_FUNCTION_ARG;
pub use crate::src::headers::sqlite3_h::SQLITE_LIMIT_VARIABLE_NUMBER;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SQLITE_RESULT_SUBTYPE;
pub use crate::src::headers::sqlite3_h::SQLITE_SUBTYPE;
pub use crate::src::headers::sqlite3_h::SQLITE_TEXT;
pub use crate::src::headers::sqlite3_h::SQLITE_UTF8;
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
pub use crate::src::headers::sqliteInt_h::__anon_union_14;
pub use crate::src::headers::sqliteInt_h::__anon_union_15;
pub use crate::src::headers::sqliteInt_h::__anon_union_16;
pub use crate::src::headers::sqliteInt_h::AggInfo;
pub use crate::src::headers::sqliteInt_h::AggInfo_col;
pub use crate::src::headers::sqliteInt_h::AggInfo_func;
pub use crate::src::headers::sqliteInt_h::AutoincInfo;
pub use crate::src::headers::sqliteInt_h::BMS;
pub use crate::src::headers::sqliteInt_h::Bitmask;
pub use crate::src::headers::sqliteInt_h::BusyHandler;
pub use crate::src::headers::sqliteInt_h::CCurHint;
pub use crate::src::headers::sqliteInt_h::COLFLAG_BUSY;
pub use crate::src::headers::sqliteInt_h::COLFLAG_GENERATED;
pub use crate::src::headers::sqliteInt_h::COLFLAG_NOTAVAIL;
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
pub use crate::src::headers::sqliteInt_h::ENAME_SPAN;
pub use crate::src::headers::sqliteInt_h::EP_Collate;
pub use crate::src::headers::sqliteInt_h::EP_Commuted;
pub use crate::src::headers::sqliteInt_h::EP_Distinct;
pub use crate::src::headers::sqliteInt_h::EP_FixedCol;
pub use crate::src::headers::sqliteInt_h::EP_HasFunc;
pub use crate::src::headers::sqliteInt_h::EP_InnerON;
pub use crate::src::headers::sqliteInt_h::EP_IntValue;
pub use crate::src::headers::sqliteInt_h::EP_IsFalse;
pub use crate::src::headers::sqliteInt_h::EP_IsTrue;
pub use crate::src::headers::sqliteInt_h::EP_Leaf;
pub use crate::src::headers::sqliteInt_h::EP_OuterON;
pub use crate::src::headers::sqliteInt_h::EP_Propagate;
pub use crate::src::headers::sqliteInt_h::EP_Reduced;
pub use crate::src::headers::sqliteInt_h::EP_Skip;
pub use crate::src::headers::sqliteInt_h::EP_Static;
pub use crate::src::headers::sqliteInt_h::EP_Subquery;
pub use crate::src::headers::sqliteInt_h::EP_TokenOnly;
pub use crate::src::headers::sqliteInt_h::EP_xIsSelect;
pub use crate::src::headers::sqliteInt_h::EXPR_FULLSIZE;
pub use crate::src::headers::sqliteInt_h::EXPR_REDUCEDSIZE;
pub use crate::src::headers::sqliteInt_h::EXPR_TOKENONLYSIZE;
pub use crate::src::headers::sqliteInt_h::EXPRDUP_REDUCE;
pub use crate::src::headers::sqliteInt_h::Expr;
pub use crate::src::headers::sqliteInt_h::ExprList;
pub use crate::src::headers::sqliteInt_h::ExprList_item;
pub use crate::src::headers::sqliteInt_h::FKey;
pub use crate::src::headers::sqliteInt_h::FuncDef;
pub use crate::src::headers::sqliteInt_h::FuncDestructor;
pub use crate::src::headers::sqliteInt_h::IN_INDEX_EPH;
pub use crate::src::headers::sqliteInt_h::IN_INDEX_INDEX_ASC;
pub use crate::src::headers::sqliteInt_h::IN_INDEX_INDEX_DESC;
pub use crate::src::headers::sqliteInt_h::IN_INDEX_LOOP;
pub use crate::src::headers::sqliteInt_h::IN_INDEX_MEMBERSHIP;
pub use crate::src::headers::sqliteInt_h::IN_INDEX_NOOP;
pub use crate::src::headers::sqliteInt_h::IN_INDEX_NOOP_OK;
pub use crate::src::headers::sqliteInt_h::IN_INDEX_ROWID;
pub use crate::src::headers::sqliteInt_h::INLINEFUNC_affinity;
pub use crate::src::headers::sqliteInt_h::INLINEFUNC_coalesce;
pub use crate::src::headers::sqliteInt_h::INLINEFUNC_expr_compare;
pub use crate::src::headers::sqliteInt_h::INLINEFUNC_expr_implies_expr;
pub use crate::src::headers::sqliteInt_h::INLINEFUNC_iif;
pub use crate::src::headers::sqliteInt_h::INLINEFUNC_implies_nonnull_row;
pub use crate::src::headers::sqliteInt_h::IdList;
pub use crate::src::headers::sqliteInt_h::IdList_item;
pub use crate::src::headers::sqliteInt_h::Index;
pub use crate::src::headers::sqliteInt_h::IndexedExpr;
pub use crate::src::headers::sqliteInt_h::JT_LEFT;
pub use crate::src::headers::sqliteInt_h::JT_LTORJ;
pub use crate::src::headers::sqliteInt_h::KEYINFO_ORDER_BIGNULL;
pub use crate::src::headers::sqliteInt_h::KeyInfo;
pub use crate::src::headers::sqliteInt_h::LogEst;
pub use crate::src::headers::sqliteInt_h::Lookaside;
pub use crate::src::headers::sqliteInt_h::LookasideSlot;
pub use crate::src::headers::sqliteInt_h::Module;
pub use crate::src::headers::sqliteInt_h::NC_InAggFunc;
pub use crate::src::headers::sqliteInt_h::NameContext;
pub use crate::src::headers::sqliteInt_h::OE_Abort;
pub use crate::src::headers::sqliteInt_h::OE_Ignore;
pub use crate::src::headers::sqliteInt_h::OE_None;
pub use crate::src::headers::sqliteInt_h::OPFLAG_BYTELENARG;
pub use crate::src::headers::sqliteInt_h::OPFLAG_NOCHNG;
pub use crate::src::headers::sqliteInt_h::OPFLAG_TYPEOFARG;
pub use crate::src::headers::sqliteInt_h::OnOrUsing;
pub use crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME;
pub use crate::src::headers::sqliteInt_h::Parse;
pub use crate::src::headers::sqliteInt_h::ParseCleanup;
pub use crate::src::headers::sqliteInt_h::RenameCtx;
pub use crate::src::headers::sqliteInt_h::RenameToken;
pub use crate::src::headers::sqliteInt_h::Returning;
pub use crate::src::headers::sqliteInt_h::SF_Aggregate;
pub use crate::src::headers::sqliteInt_h::SF_All;
pub use crate::src::headers::sqliteInt_h::SF_Correlated;
pub use crate::src::headers::sqliteInt_h::SF_Distinct;
pub use crate::src::headers::sqliteInt_h::SF_MultiValue;
pub use crate::src::headers::sqliteInt_h::SF_UsesEphemeral;
pub use crate::src::headers::sqliteInt_h::SF_Values;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_DEFER;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_INTEGER;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_NONE;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_REAL;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT;
pub use crate::src::headers::sqliteInt_h::SQLITE_ECEL_DUP;
pub use crate::src::headers::sqliteInt_h::SQLITE_ECEL_FACTOR;
pub use crate::src::headers::sqliteInt_h::SQLITE_ECEL_OMITREF;
pub use crate::src::headers::sqliteInt_h::SQLITE_ECEL_REF;
pub use crate::src::headers::sqliteInt_h::SQLITE_EnableQPSG;
pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT;
pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_DIRECT;
pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_INLINE;
pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_LENGTH;
pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_NEEDCOLL;
pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_SLOCHNG;
pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_TYPEOF;
pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_UNSAFE;
pub use crate::src::headers::sqliteInt_h::SQLITE_JUMPIFNULL;
pub use crate::src::headers::sqliteInt_h::SQLITE_NULLEQ;
pub use crate::src::headers::sqliteInt_h::SQLITE_SO_ASC;
pub use crate::src::headers::sqliteInt_h::SQLITE_SO_UNDEFINED;
pub use crate::src::headers::sqliteInt_h::SQLITE_TrustedSchema;
pub use crate::src::headers::sqliteInt_h::SRT_Exists;
pub use crate::src::headers::sqliteInt_h::SRT_Mem;
pub use crate::src::headers::sqliteInt_h::SRT_Set;
pub use crate::src::headers::sqliteInt_h::Savepoint;
pub use crate::src::headers::sqliteInt_h::Schema;
pub use crate::src::headers::sqliteInt_h::Select;
pub use crate::src::headers::sqliteInt_h::SelectDest;
pub use crate::src::headers::sqliteInt_h::SrcItem;
pub use crate::src::headers::sqliteInt_h::SrcList;
pub use crate::src::headers::sqliteInt_h::Subquery;
pub use crate::src::headers::sqliteInt_h::TABTYP_VTAB;
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
pub use crate::src::headers::sqliteInt_h::XN_EXPR;
pub use crate::src::headers::sqliteInt_h::Bft;
pub use crate::src::headers::sqliteInt_h::sColMap;
pub use crate::src::headers::sqliteInt_h::sqlite3;
pub use crate::src::headers::sqliteInt_h::Sqlite3Xauth;
pub use crate::src::headers::sqliteInt_h::sqlite3InitInfo;
pub use crate::src::headers::sqliteInt_h::YDbMask;
pub use crate::src::headers::sqliteInt_h::YnVar;
pub use crate::src::headers::stdlib::Int16T;
pub use crate::src::headers::stdlib::IntptrT;
pub use crate::src::headers::vdbeInt_h::PreUpdate;
pub use crate::src::headers::vdbeInt_h::sqlite3_context;
pub use crate::src::headers::vdbeInt_h::sqlite3_value;
pub use crate::src::parse::TK_AGG_COLUMN;
pub use crate::src::parse::TK_AGG_FUNCTION;
pub use crate::src::parse::TK_ALL;
pub use crate::src::parse::TK_AND;
pub use crate::src::parse::TK_BETWEEN_1;
pub use crate::src::parse::TK_BITAND;
pub use crate::src::parse::TK_BITNOT;
pub use crate::src::parse::TK_BITOR;
pub use crate::src::parse::TK_BLOB;
pub use crate::src::parse::TK_CASE_1;
pub use crate::src::parse::TK_CAST;
pub use crate::src::parse::TK_COLLATE;
pub use crate::src::parse::TK_COLUMN;
pub use crate::src::parse::TK_CONCAT;
pub use crate::src::parse::TK_DOT_1;
pub use crate::src::parse::TK_EQ_1;
pub use crate::src::parse::TK_ERROR;
pub use crate::src::parse::TK_EXISTS_1;
pub use crate::src::parse::TK_FILTER;
pub use crate::src::parse::TK_FLOAT;
pub use crate::src::parse::TK_FUNCTION;
pub use crate::src::parse::TK_GE;
pub use crate::src::parse::TK_GT;
pub use crate::src::parse::TK_ID_1;
pub use crate::src::parse::TK_IF_NULL_ROW;
pub use crate::src::parse::TK_IN;
pub use crate::src::parse::TK_INTEGER;
pub use crate::src::parse::TK_IS;
pub use crate::src::parse::TK_ISNOT_1;
pub use crate::src::parse::TK_ISNULL_1;
pub use crate::src::parse::TK_LE;
pub use crate::src::parse::TK_LIMIT;
pub use crate::src::parse::TK_LSHIFT;
pub use crate::src::parse::TK_LT;
pub use crate::src::parse::TK_MINUS;
pub use crate::src::parse::TK_NE;
pub use crate::src::parse::TK_NOT_1;
pub use crate::src::parse::TK_NOTNULL_1;
pub use crate::src::parse::TK_NULL;
pub use crate::src::parse::TK_NULLS;
pub use crate::src::parse::TK_OR;
pub use crate::src::parse::TK_ORDER;
pub use crate::src::parse::TK_PLUS_1;
pub use crate::src::parse::TK_RAISE;
pub use crate::src::parse::TK_REGISTER;
pub use crate::src::parse::TK_REM;
pub use crate::src::parse::TK_RSHIFT;
pub use crate::src::parse::TK_SELECT;
pub use crate::src::parse::TK_SELECT_COLUMN;
pub use crate::src::parse::TK_SLASH;
pub use crate::src::parse::TK_SPAN_1;
pub use crate::src::parse::TK_STAR;
pub use crate::src::parse::TK_STRING;
pub use crate::src::parse::TK_TRIGGER;
pub use crate::src::parse::TK_TRUEFALSE;
pub use crate::src::parse::TK_TRUTH;
pub use crate::src::parse::TK_UMINUS;
pub use crate::src::parse::TK_UPLUS;
pub use crate::src::parse::TK_VARIABLE;
pub use crate::src::parse::TK_VECTOR;
pub use crate::src::src::alter::sqlite3RenameExprUnmap;
pub use crate::src::src::alter::sqlite3RenameTokenMap;
pub use crate::src::src::build::sqlite3AffinityType;
pub use crate::src::src::build::sqlite3ArrayAllocate;
pub use crate::src::src::build::sqlite3CodeVerifySchema;
pub use crate::src::src::build::sqlite3ColumnColl;
pub use crate::src::src::build::sqlite3ColumnExpr;
pub use crate::src::src::build::sqlite3IdListDelete;
pub use crate::src::src::build::sqlite3MayAbort;
pub use crate::src::src::build::sqlite3PrimaryKeyIndex;
pub use crate::src::src::build::sqlite3TableColumnToIndex;
pub use crate::src::src::build::sqlite3TableColumnToStorage;
pub use crate::src::src::build::sqlite3TableLock;
pub use crate::src::src::callback::sqlite3CheckCollSeq;
pub use crate::src::src::callback::sqlite3FindCollSeq;
pub use crate::src::src::callback::sqlite3FindFunction;
pub use crate::src::src::callback::sqlite3GetCollSeq;
pub use crate::src::src::global::sqlite3CtypeMap;
pub use crate::src::src::insert::sqlite3OpenTable;
pub use crate::src::src::main::sqlite3IsBinary;
pub use crate::src::src::malloc::sqlite3DbFree;
pub use crate::src::src::malloc::sqlite3DbMallocRaw;
pub use crate::src::src::malloc::sqlite3DbMallocRawNN;
pub use crate::src::src::malloc::sqlite3DbMallocSize;
pub use crate::src::src::malloc::sqlite3DbMallocZero;
pub use crate::src::src::malloc::sqlite3DbNNFreeNN;
pub use crate::src::src::malloc::sqlite3DbRealloc;
pub use crate::src::src::malloc::sqlite3DbSpanDup;
pub use crate::src::src::malloc::sqlite3DbStrDup;
pub use crate::src::src::malloc::sqlite3DbStrNDup;
pub use crate::src::src::mutex_unix::sqlite3_mutex;
pub use crate::src::src::prepare::sqlite3ParserAddCleanup;
pub use crate::src::src::prepare::sqlite3SchemaToIndex;
pub use crate::src::src::printf::sqlite3RecordErrorOffsetOfExpr;
pub use crate::src::src::select::sqlite3ColumnIndex;
pub use crate::src::src::select::sqlite3GetVdbe;
pub use crate::src::src::select::sqlite3KeyInfoAlloc;
pub use crate::src::src::select::sqlite3KeyInfoUnref;
pub use crate::src::src::select::sqlite3Select;
pub use crate::src::src::select::sqlite3SelectDelete;
pub use crate::src::src::select::sqlite3SelectDestInit;
pub use crate::src::src::select::sqlite3SelectNew;
pub use crate::src::src::update::sqlite3ColumnDefault;
pub use crate::src::src::util::sqlite3_stricmp;
pub use crate::src::src::util::sqlite3_strnicmp;
pub use crate::src::src::util::sqlite3AtoF;
pub use crate::src::src::util::sqlite3Atoi64;
pub use crate::src::src::util::sqlite3DecOrHexToI64;
pub use crate::src::src::util::sqlite3Dequote;
pub use crate::src::src::util::sqlite3DequoteExpr;
pub use crate::src::src::util::sqlite3GetInt32;
pub use crate::src::src::util::sqlite3HexToBlob;
pub use crate::src::src::util::sqlite3StrICmp;
pub use crate::src::src::util::sqlite3Strlen30;
pub use crate::src::src::util::sqlite3TokenInit;
pub use crate::src::src::util::sqlite3VListAdd;
pub use crate::src::src::util::sqlite3VListNameToNum;
pub use crate::src::src::util::sqlite3VListNumToName;
pub use crate::src::src::vdbeapi::sqlite3_value_int64;
pub use crate::src::src::vdbeapi::sqlite3_value_text;
pub use crate::src::src::vdbeapi::sqlite3_value_type;
pub use crate::src::src::vdbemem::sqlite3ValueFree;
pub use crate::src::src::vdbemem::sqlite3ValueFromExpr;
pub use crate::src::src::vtab::sqlite3VtabOverloadFunction;
pub use crate::src::src::walker::sqlite3SelectWalkNoop;
pub use crate::src::src::walker::sqlite3WalkExpr;
pub use crate::src::src::walker::sqlite3WalkExprList;
pub use crate::src::src::walker::sqlite3WalkSelect;
pub use crate::src::src::walker::sqlite3WalkerDepthDecrease;
pub use crate::src::src::walker::sqlite3WalkerDepthIncrease;
pub use crate::src::src::window::sqlite3WindowCompare;
pub use crate::src::src::window::sqlite3WindowDelete;
pub use crate::src::src::window::sqlite3WindowDup;
pub use crate::src::src::window::sqlite3WindowLink;
pub use crate::src::src::window::sqlite3WindowListDup;

pub use crate::src::headers::stdlib::Uint8T;
pub use crate::src::headers::stdlib::Uint16T;
pub use crate::src::headers::stdlib::Uint32T;
pub use crate::src::headers::vdbeInt_h::Vdbe;
pub use crate::src::src::vdbe::Mem;
pub use crate::src::src::vdbe::P4_COLLSEQ;
pub use crate::src::src::vdbe::P4_DYNAMIC;
pub use crate::src::src::vdbe::P4_INT64;
pub use crate::src::src::vdbe::P4_KEYINFO;
pub use crate::src::src::vdbe::P4_REAL;
pub use crate::src::src::vdbe::P4_STATIC;
pub use crate::src::src::vdbe::P4_SUBRTNSIG;
pub use crate::src::src::vdbe::P4_TABLE;
pub use crate::src::src::vdbe::SubProgram;
pub use crate::src::src::vdbe::SubrtnSig;
pub use crate::src::src::vdbe::VdbeOp;
pub use crate::src::src::vdbe::p4union;
pub use crate::src::src::vdbeaux::sqlite3MemCompare;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddFunctionCall;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp0;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp1;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp2;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp3;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp4;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp4Dup8;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int;
pub use crate::src::src::vdbeaux::sqlite3VdbeChangeP2;
pub use crate::src::src::vdbeaux::sqlite3VdbeChangeP3;
pub use crate::src::src::vdbeaux::sqlite3VdbeChangeP4;
pub use crate::src::src::vdbeaux::sqlite3VdbeChangeP5;
pub use crate::src::src::vdbeaux::sqlite3VdbeChangeToNoop;
pub use crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr;
pub use crate::src::src::vdbeaux::sqlite3VdbeDb;
pub use crate::src::src::vdbeaux::sqlite3VdbeGetBoundValue;
pub use crate::src::src::vdbeaux::sqlite3VdbeGetLastOp;
pub use crate::src::src::vdbeaux::sqlite3VdbeGetOp;
pub use crate::src::src::vdbeaux::sqlite3VdbeGoto;
pub use crate::src::src::vdbeaux::sqlite3VdbeJumpHere;
pub use crate::src::src::vdbeaux::sqlite3VdbeLoadString;
pub use crate::src::src::vdbeaux::sqlite3VdbeMakeLabel;
pub use crate::src::src::vdbeaux::sqlite3VdbeParser;
pub use crate::src::src::vdbeaux::sqlite3VdbeResolveLabel;
pub use crate::src::src::vdbeaux::sqlite3VdbeSetP4KeyInfo;
pub use crate::src::src::vdbeaux::sqlite3VdbeSetVarmask;
pub use crate::src::src::vdbeaux::sqlite3VdbeTypeofColumn;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdxCover {
    pub pIdx: *mut crate::src::headers::sqliteInt_h::Index,
    pub iCur: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefSrcList {
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub pRef: *mut crate::src::headers::sqliteInt_h::SrcList,
    pub nExclude: crate::src::ext::rtree::rtree::I64_0,
    pub aiExclude: *mut ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdupBuf {
    pub zAlloc: *mut crate::src::ext::rtree::rtree::U8_0,
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3TableColumnAffinity(
    pTab: *const crate::src::headers::sqliteInt_h::Table,
    iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_char {
    if iCol < 0 as ::core::ffi::c_int || iCol >= (*pTab).nCol as ::core::ffi::c_int {
        return crate::src::headers::sqliteInt_h::SQLITE_AFF_INTEGER as ::core::ffi::c_char;
    }
    (*(*pTab).aCol.offset(iCol as isize)).affinity
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprAffinity(
    mut pExpr: *const crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_char {
    let mut op: ::core::ffi::c_int;
    op = (*pExpr).op as ::core::ffi::c_int;
    loop {
        if op == crate::src::parse::TK_COLUMN
            || op == crate::src::parse::TK_AGG_COLUMN && !(*pExpr).y.pTab.is_null()
        {
            return sqlite3TableColumnAffinity(
                (*pExpr).y.pTab,
                (*pExpr).iColumn as ::core::ffi::c_int,
            );
        }
        if op == crate::src::parse::TK_SELECT {
            return sqlite3ExprAffinity(
                (*(&raw mut (*(*(*pExpr).x.pSelect).pEList).a
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(0_isize))
                .pExpr,
            );
        }
        if op == crate::src::parse::TK_CAST {
            return crate::src::src::build::sqlite3AffinityType(
                (*pExpr).u.zToken,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Column>()
                    as *mut crate::src::headers::sqliteInt_h::Column,
            );
        }
        if op == crate::src::parse::TK_SELECT_COLUMN {
            return sqlite3ExprAffinity(
                (*(&raw mut (*(*(*(*pExpr).pLeft).x.pSelect).pEList).a
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset((*pExpr).iColumn as isize))
                .pExpr,
            );
        }
        if op == crate::src::parse::TK_VECTOR
            || op == crate::src::parse::TK_FUNCTION
                && (*pExpr).affExpr as ::core::ffi::c_int
                    == crate::src::headers::sqliteInt_h::SQLITE_AFF_DEFER
        {
            return sqlite3ExprAffinity(
                (*(&raw mut (*(*pExpr).x.pList).a
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(0_isize))
                .pExpr,
            );
        }
        if (*pExpr).flags
            & (0x2000 as ::core::ffi::c_int | 0x40000 as ::core::ffi::c_int)
                as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
        {
            pExpr = (*pExpr).pLeft;
            op = (*pExpr).op as ::core::ffi::c_int;
        } else {
            if op != crate::src::parse::TK_REGISTER {
                break;
            }
            op = (*pExpr).op2 as ::core::ffi::c_int;
            if op == 176 as ::core::ffi::c_int {
                break;
            }
        }
    }
    (*pExpr).affExpr
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprDataType(
    mut pExpr: *const crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    while !pExpr.is_null() {
        match (*pExpr).op as ::core::ffi::c_int {
            crate::src::parse::TK_COLLATE
            | crate::src::parse::TK_IF_NULL_ROW
            | crate::src::parse::TK_UPLUS => {
                pExpr = (*pExpr).pLeft;
            }
            crate::src::parse::TK_NULL => {
                pExpr = ::core::ptr::null::<crate::src::headers::sqliteInt_h::Expr>();
            }
            crate::src::parse::TK_STRING => return 0x2 as ::core::ffi::c_int,
            crate::src::parse::TK_BLOB => return 0x4 as ::core::ffi::c_int,
            crate::src::parse::TK_CONCAT => return 0x6 as ::core::ffi::c_int,
            crate::src::parse::TK_VARIABLE
            | crate::src::parse::TK_AGG_FUNCTION
            | crate::src::parse::TK_FUNCTION => {
                return 0x7 as ::core::ffi::c_int;
            }
            crate::src::parse::TK_COLUMN
            | crate::src::parse::TK_AGG_COLUMN
            | crate::src::parse::TK_SELECT
            | crate::src::parse::TK_CAST
            | crate::src::parse::TK_SELECT_COLUMN
            | crate::src::parse::TK_VECTOR => {
                let aff: ::core::ffi::c_int = sqlite3ExprAffinity(pExpr) as ::core::ffi::c_int;
                if aff >= crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC {
                    return 0x5 as ::core::ffi::c_int;
                }
                if aff == crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT {
                    return 0x6 as ::core::ffi::c_int;
                }
                return 0x7 as ::core::ffi::c_int;
            }
            crate::src::parse::TK_CASE_1 => {
                let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut ii: ::core::ffi::c_int;
                let pList: *mut crate::src::headers::sqliteInt_h::ExprList = (*pExpr).x.pList;
                ii = 1 as ::core::ffi::c_int;
                while ii < (*pList).nExpr {
                    res |= sqlite3ExprDataType(
                        (*(&raw mut (*pList).a
                            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                            .offset(ii as isize))
                        .pExpr,
                    );
                    ii += 2 as ::core::ffi::c_int;
                }
                if (*pList).nExpr % 2 as ::core::ffi::c_int != 0 {
                    res |= sqlite3ExprDataType(
                        (*(&raw mut (*pList).a
                            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                            .offset(((*pList).nExpr - 1 as ::core::ffi::c_int) as isize))
                        .pExpr,
                    );
                }
                return res;
            }
            _ => return 0x1 as ::core::ffi::c_int,
        }
    }
    0 as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprAddCollateToken(
    pParse: *const crate::src::headers::sqliteInt_h::Parse,
    mut pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    pCollName: *const crate::src::headers::sqliteInt_h::Token,
    dequote: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    if (*pCollName).n > 0 as ::core::ffi::c_uint {
        let pNew: *mut crate::src::headers::sqliteInt_h::Expr = sqlite3ExprAlloc(
            (*pParse).db,
            crate::src::parse::TK_COLLATE,
            pCollName,
            dequote,
        );
        if !pNew.is_null() {
            (*pNew).pLeft = pExpr;
            (*pNew).flags |= (crate::src::headers::sqliteInt_h::EP_Collate
                | crate::src::headers::sqliteInt_h::EP_Skip)
                as crate::src::ext::rtree::rtree::U32_0;
            pExpr = pNew;
        }
    }
    pExpr
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprAddCollateString(
    pParse: *const crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    zC: *const ::core::ffi::c_char,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    let mut s: crate::src::headers::sqliteInt_h::Token = unsafe { ::core::mem::zeroed() };
    crate::src::src::util::sqlite3TokenInit(
        &raw mut s as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
        zC as *mut ::core::ffi::c_char,
    );
    sqlite3ExprAddCollateToken(pParse, pExpr, &raw mut s, 0 as ::core::ffi::c_int)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprSkipCollate(
    mut pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    while !pExpr.is_null()
        && (*pExpr).flags & 0x2000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        pExpr = (*pExpr).pLeft;
    }
    pExpr
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprSkipCollateAndLikely(
    mut pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    while !pExpr.is_null()
        && (*pExpr).flags
            & (0x2000 as ::core::ffi::c_int | 0x80000 as ::core::ffi::c_int)
                as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        if (*pExpr).flags & 0x80000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
        {
            pExpr = (*(&raw mut (*(*pExpr).x.pList).a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(0_isize))
            .pExpr;
        } else {
            if ((*pExpr).op as ::core::ffi::c_int != crate::src::parse::TK_COLLATE) {
                break;
            }
            pExpr = (*pExpr).pLeft;
        }
    }
    pExpr
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCollSeq(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *const crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::CollSeq {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    let mut pColl: *mut crate::src::headers::sqliteInt_h::CollSeq =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::CollSeq>();
    let mut p: *const crate::src::headers::sqliteInt_h::Expr = pExpr;
    while !p.is_null() {
        let mut op: ::core::ffi::c_int = (*p).op as ::core::ffi::c_int;
        if op == crate::src::parse::TK_REGISTER {
            op = (*p).op2 as ::core::ffi::c_int;
        }
        if op == crate::src::parse::TK_AGG_COLUMN && !(*p).y.pTab.is_null()
            || op == crate::src::parse::TK_COLUMN
            || op == crate::src::parse::TK_TRIGGER
        {
            
            let j: ::core::ffi::c_int = (*p).iColumn as ::core::ffi::c_int;
            if j >= 0 as ::core::ffi::c_int {
                let zColl: *const ::core::ffi::c_char =
                    crate::src::src::build::sqlite3ColumnColl(
                        (*(*p).y.pTab).aCol.offset(j as isize)
                            as *mut crate::src::headers::sqliteInt_h::Column
                            as *mut crate::src::headers::sqliteInt_h::Column,
                    );
                pColl = crate::src::src::callback::sqlite3FindCollSeq(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    (*db).enc,
                    zColl,
                    0 as ::core::ffi::c_int,
                ) as *mut crate::src::headers::sqliteInt_h::CollSeq;
            }
            break;
        } else if op == crate::src::parse::TK_CAST || op == crate::src::parse::TK_UPLUS {
            p = (*p).pLeft;
        } else if op == crate::src::parse::TK_VECTOR
            || op == crate::src::parse::TK_FUNCTION
                && (*p).affExpr as ::core::ffi::c_int
                    == crate::src::headers::sqliteInt_h::SQLITE_AFF_DEFER
        {
            p = (*(&raw mut (*(*p).x.pList).a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(0_isize))
            .pExpr;
        } else if op == crate::src::parse::TK_COLLATE {
            pColl = crate::src::src::callback::sqlite3GetCollSeq(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*db).enc,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::CollSeq>()
                    as *mut crate::src::headers::sqliteInt_h::CollSeq,
                (*p).u.zToken,
            ) as *mut crate::src::headers::sqliteInt_h::CollSeq;
            break;
        } else {
            if ((*p).flags
                & crate::src::headers::sqliteInt_h::EP_Collate
                    as crate::src::ext::rtree::rtree::U32_0 == 0)
            {
                break;
            }
            if !(*p).pLeft.is_null()
                && (*(*p).pLeft).flags
                    & crate::src::headers::sqliteInt_h::EP_Collate
                        as crate::src::ext::rtree::rtree::U32_0
                    != 0 as crate::src::ext::rtree::rtree::U32_0
            {
                p = (*p).pLeft;
            } else {
                let mut pNext: *mut crate::src::headers::sqliteInt_h::Expr = (*p).pRight;
                if (*p).flags
                    & crate::src::headers::sqliteInt_h::EP_xIsSelect
                        as crate::src::ext::rtree::rtree::U32_0
                    == 0 as crate::src::ext::rtree::rtree::U32_0
                    && !(*p).x.pList.is_null()
                    && (*db).mallocFailed == 0
                {
                    let mut i: ::core::ffi::c_int;
                    i = 0 as ::core::ffi::c_int;
                    while i < (*(*p).x.pList).nExpr {
                        if (*(*(&raw mut (*(*p).x.pList).a
                            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                            .offset(i as isize))
                        .pExpr)
                            .flags
                            & 0x200 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                            != 0 as crate::src::ext::rtree::rtree::U32_0
                        {
                            pNext = (*(&raw mut (*(*p).x.pList).a
                                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                .offset(i as isize))
                            .pExpr;
                            break;
                        } else {
                            i += 1;
                        }
                    }
                }
                p = pNext;
            }
        }
    }
    if crate::src::src::callback::sqlite3CheckCollSeq(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        pColl as *mut crate::src::headers::sqliteInt_h::CollSeq,
    ) != 0
    {
        pColl = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::CollSeq>();
    }
    pColl
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprNNCollSeq(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *const crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::CollSeq {
    let mut p: *mut crate::src::headers::sqliteInt_h::CollSeq = sqlite3ExprCollSeq(pParse, pExpr);
    if p.is_null() {
        p = (*(*pParse).db).pDfltColl;
    }
    p
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCollSeqMatch(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pE1: *const crate::src::headers::sqliteInt_h::Expr,
    pE2: *const crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let pColl1: *mut crate::src::headers::sqliteInt_h::CollSeq =
        sqlite3ExprNNCollSeq(pParse, pE1);
    let pColl2: *mut crate::src::headers::sqliteInt_h::CollSeq =
        sqlite3ExprNNCollSeq(pParse, pE2);
    (crate::src::src::util::sqlite3StrICmp((*pColl1).zName, (*pColl2).zName)
        == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3CompareAffinity(
    pExpr: *const crate::src::headers::sqliteInt_h::Expr,
    aff2: ::core::ffi::c_char,
) -> ::core::ffi::c_char {
    let aff1: ::core::ffi::c_char = sqlite3ExprAffinity(pExpr);
    if aff1 as ::core::ffi::c_int > crate::src::headers::sqliteInt_h::SQLITE_AFF_NONE
        && aff2 as ::core::ffi::c_int > crate::src::headers::sqliteInt_h::SQLITE_AFF_NONE
    {
        if aff1 as ::core::ffi::c_int >= crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC
            || aff2 as ::core::ffi::c_int >= crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC
        {
            crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC as ::core::ffi::c_char
        } else {
            crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB as ::core::ffi::c_char
        }
    } else {
        (((if aff1 as ::core::ffi::c_int <= crate::src::headers::sqliteInt_h::SQLITE_AFF_NONE
        {
            aff2 as ::core::ffi::c_int
        } else {
            aff1 as ::core::ffi::c_int
        }) | crate::src::headers::sqliteInt_h::SQLITE_AFF_NONE)
            as ::core::ffi::c_char)
    }
}

unsafe extern "C" fn comparisonAffinity(
    pExpr: *const crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_char {
    let mut aff: ::core::ffi::c_char;
    aff = sqlite3ExprAffinity((*pExpr).pLeft);
    if !(*pExpr).pRight.is_null() {
        aff = sqlite3CompareAffinity((*pExpr).pRight, aff);
    } else if (*pExpr).flags
        & crate::src::headers::sqliteInt_h::EP_xIsSelect as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        aff = sqlite3CompareAffinity(
            (*(&raw mut (*(*(*pExpr).x.pSelect).pEList).a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(0_isize))
            .pExpr,
            aff,
        );
    } else if aff as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        aff = crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB as ::core::ffi::c_char;
    }
    aff
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3IndexAffinityOk(
    pExpr: *const crate::src::headers::sqliteInt_h::Expr,
    idx_affinity: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let aff: ::core::ffi::c_char = comparisonAffinity(pExpr);
    if (aff as ::core::ffi::c_int) < crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT {
        return 1 as ::core::ffi::c_int;
    }
    if aff as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT {
        return (idx_affinity as ::core::ffi::c_int
            == crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT)
            as ::core::ffi::c_int;
    }
    (idx_affinity as ::core::ffi::c_int >= crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC)
        as ::core::ffi::c_int
}

unsafe extern "C" fn binaryCompareP5(
    pExpr1: *const crate::src::headers::sqliteInt_h::Expr,
    pExpr2: *const crate::src::headers::sqliteInt_h::Expr,
    jumpIfNull: ::core::ffi::c_int,
) -> crate::src::ext::rtree::rtree::U8_0 {
    let mut aff: crate::src::ext::rtree::rtree::U8_0 =
        sqlite3ExprAffinity(pExpr2) as crate::src::ext::rtree::rtree::U8_0;
    aff = (sqlite3CompareAffinity(pExpr1, aff as ::core::ffi::c_char)
        as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_int
        | jumpIfNull as crate::src::ext::rtree::rtree::U8_0 as ::core::ffi::c_int)
        as crate::src::ext::rtree::rtree::U8_0;
    aff
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BinaryCompareCollSeq(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pLeft: *const crate::src::headers::sqliteInt_h::Expr,
    pRight: *const crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::CollSeq {
    let mut pColl: *mut crate::src::headers::sqliteInt_h::CollSeq;
    if (*pLeft).flags
        & crate::src::headers::sqliteInt_h::EP_Collate as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        pColl = sqlite3ExprCollSeq(pParse, pLeft);
    } else if !pRight.is_null()
        && (*pRight).flags
            & crate::src::headers::sqliteInt_h::EP_Collate as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        pColl = sqlite3ExprCollSeq(pParse, pRight);
    } else {
        pColl = sqlite3ExprCollSeq(pParse, pLeft);
        if pColl.is_null() {
            pColl = sqlite3ExprCollSeq(pParse, pRight);
        }
    }
    pColl
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCompareCollSeq(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *const crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::CollSeq {
    if (*p).flags & 0x400 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        sqlite3BinaryCompareCollSeq(pParse, (*p).pRight, (*p).pLeft)
    } else {
        sqlite3BinaryCompareCollSeq(pParse, (*p).pLeft, (*p).pRight)
    }
}

unsafe extern "C" fn codeCompare(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pLeft: *mut crate::src::headers::sqliteInt_h::Expr,
    pRight: *mut crate::src::headers::sqliteInt_h::Expr,
    opcode: ::core::ffi::c_int,
    in1: ::core::ffi::c_int,
    in2: ::core::ffi::c_int,
    dest: ::core::ffi::c_int,
    jumpIfNull: ::core::ffi::c_int,
    isCommuted: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    
    let p4: *mut crate::src::headers::sqliteInt_h::CollSeq;
    let __pParse_ref = unsafe { &*pParse };
    if __pParse_ref.nErr != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if isCommuted != 0 {
        p4 = sqlite3BinaryCompareCollSeq(pParse, pRight, pLeft);
    } else {
        p4 = sqlite3BinaryCompareCollSeq(pParse, pLeft, pRight);
    }
    let p5: ::core::ffi::c_int = binaryCompareP5(pLeft, pRight, jumpIfNull) as ::core::ffi::c_int;
    let addr: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
        __pParse_ref.pVdbe,
        opcode,
        in2,
        dest,
        in1,
        p4 as *mut ::core::ffi::c_void as *const ::core::ffi::c_char,
        crate::src::src::vdbe::P4_COLLSEQ,
    );
    crate::src::src::vdbeaux::sqlite3VdbeChangeP5(
        __pParse_ref.pVdbe,
        p5 as crate::src::fts5::U16_0,
    );
    addr
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprIsVector(
    pExpr: *const crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    (sqlite3ExprVectorSize(pExpr) > 1 as ::core::ffi::c_int) as ::core::ffi::c_int
}
pub unsafe extern "C" fn sqlite3ExprVectorSize(
    pExpr: *const crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let mut op: crate::src::ext::rtree::rtree::U8_0 = (*pExpr).op;
    if op as ::core::ffi::c_int == crate::src::parse::TK_REGISTER {
        op = (*pExpr).op2;
    }
    if op as ::core::ffi::c_int == crate::src::parse::TK_VECTOR {
        (*(*pExpr).x.pList).nExpr
    } else if op as ::core::ffi::c_int == crate::src::parse::TK_SELECT {
        (*(*(*pExpr).x.pSelect).pEList).nExpr
    } else {
        1 as ::core::ffi::c_int
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VectorFieldSubexpr(
    pVector: *mut crate::src::headers::sqliteInt_h::Expr,
    i: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    if sqlite3ExprIsVector(pVector) != 0 {
        if (*pVector).op as ::core::ffi::c_int == crate::src::parse::TK_SELECT
            || (*pVector).op2 as ::core::ffi::c_int == crate::src::parse::TK_SELECT
        {
            return (*(&raw mut (*(*(*pVector).x.pSelect).pEList).a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(i as isize))
            .pExpr;
        } else {
            return (*(&raw mut (*(*pVector).x.pList).a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(i as isize))
            .pExpr;
        }
    }
    pVector
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprForVectorField(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut pVector: *mut crate::src::headers::sqliteInt_h::Expr,
    iField: ::core::ffi::c_int,
    nField: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    let pRet: *mut crate::src::headers::sqliteInt_h::Expr;
    if (*pVector).op as ::core::ffi::c_int == crate::src::parse::TK_SELECT {
        pRet = sqlite3PExpr(
            pParse,
            crate::src::parse::TK_SELECT_COLUMN,
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>(),
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>(),
        );
        if !pRet.is_null() {
            let __pRet_ref = unsafe { &mut *pRet };
            __pRet_ref.flags |=
                0x20000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
            __pRet_ref.iTable = nField;
            __pRet_ref.iColumn = iField as crate::src::headers::sqliteInt_h::YnVar;
            __pRet_ref.pLeft = pVector;
        }
    } else {
        if (*pVector).op as ::core::ffi::c_int == crate::src::parse::TK_VECTOR {
            
            let ppVector: *mut *mut crate::src::headers::sqliteInt_h::Expr = &raw mut (*(&raw mut (*(*pVector).x.pList).a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(iField as isize))
            .pExpr;
            pVector = *ppVector;
            if (*pParse).eParseMode as ::core::ffi::c_int
                >= crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME
            {
                *ppVector = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
                return pVector;
            }
        }
        pRet = sqlite3ExprDup((*pParse).db, pVector, 0 as ::core::ffi::c_int);
    }
    pRet
}

unsafe extern "C" fn exprCodeSubselect(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let mut reg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_SELECT {
        reg = sqlite3CodeSubselect(pParse, pExpr);
    }
    reg
}

unsafe extern "C" fn exprVectorRegister(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pVector: *mut crate::src::headers::sqliteInt_h::Expr,
    iField: ::core::ffi::c_int,
    regSelect: ::core::ffi::c_int,
    ppExpr: *mut *mut crate::src::headers::sqliteInt_h::Expr,
    pRegFree: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let op: crate::src::ext::rtree::rtree::U8_0 = (*pVector).op;
    if op as ::core::ffi::c_int == crate::src::parse::TK_REGISTER {
        *ppExpr = sqlite3VectorFieldSubexpr(pVector, iField);
        return (*pVector).iTable + iField;
    }
    if op as ::core::ffi::c_int == crate::src::parse::TK_SELECT {
        *ppExpr = (*(&raw mut (*(*(*pVector).x.pSelect).pEList).a
            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(iField as isize))
        .pExpr;
        return regSelect + iField;
    }
    if op as ::core::ffi::c_int == crate::src::parse::TK_VECTOR {
        *ppExpr = (*(&raw mut (*(*pVector).x.pList).a
            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(iField as isize))
        .pExpr;
        return sqlite3ExprCodeTemp(pParse, *ppExpr, pRegFree);
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn codeVectorCompare(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    dest: ::core::ffi::c_int,
    op: crate::src::ext::rtree::rtree::U8_0,
    p5: crate::src::ext::rtree::rtree::U8_0,
) {
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    let __pExpr_ref = unsafe { &*pExpr };
    let pLeft: *mut crate::src::headers::sqliteInt_h::Expr = __pExpr_ref.pLeft;
    let pRight: *mut crate::src::headers::sqliteInt_h::Expr = __pExpr_ref.pRight;
    let nLeft: ::core::ffi::c_int = sqlite3ExprVectorSize(pLeft);
    let mut i: ::core::ffi::c_int;
    
    
    let mut opx: crate::src::ext::rtree::rtree::U8_0 = op;
    let mut addrCmp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let addrDone: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
    );
    let isCommuted: ::core::ffi::c_int =
        (__pExpr_ref.flags & 0x400 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0) as ::core::ffi::c_int;
    if (*pParse).nErr != 0 {
        return;
    }
    if nLeft != sqlite3ExprVectorSize(pRight) {
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"row value misused\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
        return;
    }
    if op as ::core::ffi::c_int == crate::src::parse::TK_LE {
        opx = crate::src::parse::TK_LT as crate::src::ext::rtree::rtree::U8_0;
    }
    if op as ::core::ffi::c_int == crate::src::parse::TK_GE {
        opx = crate::src::parse::TK_GT as crate::src::ext::rtree::rtree::U8_0;
    }
    if op as ::core::ffi::c_int == crate::src::parse::TK_NE {
        opx = crate::src::parse::TK_EQ_1 as crate::src::ext::rtree::rtree::U8_0;
    }
    let regLeft: ::core::ffi::c_int = exprCodeSubselect(pParse, pLeft);
    let regRight: ::core::ffi::c_int = exprCodeSubselect(pParse, pRight);
    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
        v,
        crate::src::headers::opcodes_h::OP_Integer,
        1 as ::core::ffi::c_int,
        dest,
    );
    i = 0 as ::core::ffi::c_int;
    loop {
        let mut regFree1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regFree2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pL: *mut crate::src::headers::sqliteInt_h::Expr =
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
        let mut pR: *mut crate::src::headers::sqliteInt_h::Expr =
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
        
        
        if addrCmp != 0 {
            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrCmp);
        }
        let r1: ::core::ffi::c_int = exprVectorRegister(pParse, pLeft, i, regLeft, &raw mut pL, &raw mut regFree1);
        let r2: ::core::ffi::c_int = exprVectorRegister(pParse, pRight, i, regRight, &raw mut pR, &raw mut regFree2);
        addrCmp = crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v);
        codeCompare(
            pParse,
            pL,
            pR,
            opx as ::core::ffi::c_int,
            r1,
            r2,
            addrDone,
            p5 as ::core::ffi::c_int,
            isCommuted,
        );
        sqlite3ReleaseTempReg(pParse, regFree1);
        sqlite3ReleaseTempReg(pParse, regFree2);
        if (opx as ::core::ffi::c_int == crate::src::parse::TK_LT
            || opx as ::core::ffi::c_int == crate::src::parse::TK_GT)
            && i < nLeft - 1 as ::core::ffi::c_int
        {
            addrCmp = crate::src::src::vdbeaux::sqlite3VdbeAddOp0(
                v,
                crate::src::headers::opcodes_h::OP_ElseEq,
            );
        }
        if p5 as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::SQLITE_NULLEQ {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_Integer,
                0 as ::core::ffi::c_int,
                dest,
            );
        } else {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_ZeroOrNull,
                r1,
                dest,
                r2,
            );
        }
        if i == nLeft - 1 as ::core::ffi::c_int {
            break;
        }
        if opx as ::core::ffi::c_int == crate::src::parse::TK_EQ_1 {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_NotNull,
                dest,
                addrDone,
            );
        } else {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_Goto,
                0 as ::core::ffi::c_int,
                addrDone,
            );
            if i == nLeft - 2 as ::core::ffi::c_int {
                opx = op;
            }
        }
        i += 1;
    }
    crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrCmp);
    crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, addrDone);
    if op as ::core::ffi::c_int == crate::src::parse::TK_NE {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Not,
            dest,
            dest,
        );
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCheckHeight(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    nHeight: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mxHeight: ::core::ffi::c_int =
        (*(*pParse).db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_EXPR_DEPTH as usize];
    if nHeight > mxHeight {
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"Expression tree is too large (maximum depth %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Int(
                mxHeight as crate::src::ext::rtree::rtree::I64_0,
            )],
        );
        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    rc
}

unsafe extern "C" fn heightOfExpr(
    p: *const crate::src::headers::sqliteInt_h::Expr,
    pnHeight: *mut ::core::ffi::c_int,
) {
    if !p.is_null()
        && (*p).nHeight > *pnHeight {
            *pnHeight = (*p).nHeight;
        }
}

unsafe extern "C" fn heightOfExprList(
    p: *const crate::src::headers::sqliteInt_h::ExprList,
    pnHeight: *mut ::core::ffi::c_int,
) {
    if !p.is_null() {
        let mut i: ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nExpr {
            heightOfExpr(
                (*(&raw const (*p).a as *const crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(i as isize))
                .pExpr,
                pnHeight,
            );
            i += 1;
        }
    }
}

unsafe extern "C" fn heightOfSelect(
    pSelect: *const crate::src::headers::sqliteInt_h::Select,
    pnHeight: *mut ::core::ffi::c_int,
) {
    let mut p: *const crate::src::headers::sqliteInt_h::Select;
    p = pSelect;
    while !p.is_null() {
        heightOfExpr((*p).pWhere, pnHeight);
        heightOfExpr((*p).pHaving, pnHeight);
        heightOfExpr((*p).pLimit, pnHeight);
        heightOfExprList((*p).pEList, pnHeight);
        heightOfExprList((*p).pGroupBy, pnHeight);
        heightOfExprList((*p).pOrderBy, pnHeight);
        p = (*p).pPrior;
    }
}

unsafe extern "C" fn exprSetHeight(p: *mut crate::src::headers::sqliteInt_h::Expr) {
    let __p_ref = unsafe { &mut *p };
    let mut nHeight: ::core::ffi::c_int = if !__p_ref.pLeft.is_null() {
        (*__p_ref.pLeft).nHeight
    } else {
        0 as ::core::ffi::c_int
    };
    if !__p_ref.pRight.is_null() && (*__p_ref.pRight).nHeight > nHeight {
        nHeight = (*__p_ref.pRight).nHeight;
    }
    if __p_ref.flags
        & crate::src::headers::sqliteInt_h::EP_xIsSelect as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        heightOfSelect(__p_ref.x.pSelect, &raw mut nHeight);
    } else if !__p_ref.x.pList.is_null() {
        heightOfExprList(__p_ref.x.pList, &raw mut nHeight);
        __p_ref.flags |= crate::src::headers::sqliteInt_h::EP_Propagate
            as crate::src::ext::rtree::rtree::U32_0
            & sqlite3ExprListFlags(__p_ref.x.pList);
    }
    __p_ref.nHeight = nHeight + 1 as ::core::ffi::c_int;
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprSetHeightAndFlags(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Expr,
) {
    if (*pParse).nErr != 0 {
        return;
    }
    exprSetHeight(p);
    sqlite3ExprCheckHeight(pParse, (*p).nHeight);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SelectExprHeight(
    p: *const crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    let mut nHeight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    heightOfSelect(p, &raw mut nHeight);
    nHeight
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprSetErrorOffset(
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    iOfst: ::core::ffi::c_int,
) {
    if pExpr.is_null() {
        return;
    }
    if (*pExpr).flags
        & (0x2 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return;
    }
    (*pExpr).w.iOfst = iOfst;
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprAlloc(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    op: ::core::ffi::c_int,
    pToken: *const crate::src::headers::sqliteInt_h::Token,
    dequote: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    
    let mut nExtra: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iValue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !pToken.is_null()
        && (op != crate::src::parse::TK_INTEGER
            || (*pToken).z.is_null()
            || crate::src::src::util::sqlite3GetInt32((*pToken).z, &raw mut iValue)
                == 0 as ::core::ffi::c_int)
        {
            nExtra = (*pToken).n.wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        }
    let pNew: *mut crate::src::headers::sqliteInt_h::Expr = crate::src::src::malloc::sqlite3DbMallocRawNN(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        (::core::mem::size_of::<crate::src::headers::sqliteInt_h::Expr>() as usize)
            .wrapping_add(nExtra as usize) as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::headers::sqliteInt_h::Expr;
    if !pNew.is_null() {
        ::libc::memset(
            pNew as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Expr>()
                as crate::__stddef_size_t_h::SizeT,
        );
        let __pNew_ref = unsafe { &mut *pNew };
        __pNew_ref.op = op as crate::src::ext::rtree::rtree::U8_0;
        __pNew_ref.iAgg = -(1 as ::core::ffi::c_int) as crate::src::fts5::I16_0;
        if !pToken.is_null() {
            if nExtra == 0 as ::core::ffi::c_int {
                __pNew_ref.flags |= (crate::src::headers::sqliteInt_h::EP_IntValue
                    | crate::src::headers::sqliteInt_h::EP_Leaf
                    | (if iValue != 0 {
                        crate::src::headers::sqliteInt_h::EP_IsTrue
                    } else {
                        crate::src::headers::sqliteInt_h::EP_IsFalse
                    })) as crate::src::ext::rtree::rtree::U32_0;
                __pNew_ref.u.iValue = iValue;
            } else {
                __pNew_ref.u.zToken = pNew.offset(1_isize)
                    as *mut crate::src::headers::sqliteInt_h::Expr
                    as *mut ::core::ffi::c_char;
                if (*pToken).n != 0 {
                    ::core::ptr::copy_nonoverlapping(
                        (*pToken).z as *const u8,
                        __pNew_ref.u.zToken as *mut u8,
                        (*pToken).n as usize,
                    );
                }
                *__pNew_ref.u.zToken.offset((*pToken).n as isize) = 0 as ::core::ffi::c_char;
                if dequote != 0
                    && *(&raw const crate::src::src::global::sqlite3CtypeMap
                        as *const ::core::ffi::c_uchar)
                        .offset(
                            *__pNew_ref.u.zToken.offset(0_isize) as ::core::ffi::c_uchar
                                as isize,
                        ) as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        != 0
                {
                    crate::src::src::util::sqlite3DequoteExpr(
                        pNew as *mut crate::src::headers::sqliteInt_h::Expr,
                    );
                }
            }
        }
        __pNew_ref.nHeight = 1 as ::core::ffi::c_int;
    }
    pNew
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Expr(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    op: ::core::ffi::c_int,
    zToken: *const ::core::ffi::c_char,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    let mut x: crate::src::headers::sqliteInt_h::Token = unsafe { ::core::mem::zeroed() };
    x.z = zToken;
    x.n = crate::src::src::util::sqlite3Strlen30(zToken) as ::core::ffi::c_uint;
    sqlite3ExprAlloc(db, op, &raw mut x, 0 as ::core::ffi::c_int)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprAttachSubtrees(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pRoot: *mut crate::src::headers::sqliteInt_h::Expr,
    pLeft: *mut crate::src::headers::sqliteInt_h::Expr,
    pRight: *mut crate::src::headers::sqliteInt_h::Expr,
) {
    if pRoot.is_null() {
        sqlite3ExprDelete(db, pLeft);
        sqlite3ExprDelete(db, pRight);
    } else {
        if !pRight.is_null() {
            let __pRoot_ref = unsafe { &mut *pRoot };
            __pRoot_ref.pRight = pRight;
            __pRoot_ref.flags |= crate::src::headers::sqliteInt_h::EP_Propagate
                as crate::src::ext::rtree::rtree::U32_0
                & (*pRight).flags;
            __pRoot_ref.nHeight = (*pRight).nHeight + 1 as ::core::ffi::c_int;
        } else {
            (*pRoot).nHeight = 1 as ::core::ffi::c_int;
        }
        if !pLeft.is_null() {
            let __pRoot_ref = unsafe { &mut *pRoot };
            __pRoot_ref.pLeft = pLeft;
            __pRoot_ref.flags |= crate::src::headers::sqliteInt_h::EP_Propagate
                as crate::src::ext::rtree::rtree::U32_0
                & (*pLeft).flags;
            if (*pLeft).nHeight >= __pRoot_ref.nHeight {
                __pRoot_ref.nHeight = (*pLeft).nHeight + 1 as ::core::ffi::c_int;
            }
        }
    };
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3PExpr(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    op: ::core::ffi::c_int,
    pLeft: *mut crate::src::headers::sqliteInt_h::Expr,
    pRight: *mut crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    
    let p: *mut crate::src::headers::sqliteInt_h::Expr = crate::src::src::malloc::sqlite3DbMallocRawNN(
        (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Expr>()
            as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::headers::sqliteInt_h::Expr;
    if !p.is_null() {
        ::libc::memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Expr>()
                as crate::__stddef_size_t_h::SizeT,
        );
        let __p_ref = unsafe { &mut *p };
        __p_ref.op = (op & 0xff as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U8_0;
        __p_ref.iAgg = -(1 as ::core::ffi::c_int) as crate::src::fts5::I16_0;
        sqlite3ExprAttachSubtrees((*pParse).db, p, pLeft, pRight);
        sqlite3ExprCheckHeight(pParse, __p_ref.nHeight);
    } else {
        sqlite3ExprDelete((*pParse).db, pLeft);
        sqlite3ExprDelete((*pParse).db, pRight);
    }
    p
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3PExprAddSelect(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    pSelect: *mut crate::src::headers::sqliteInt_h::Select,
) {
    if !pExpr.is_null() {
        (*pExpr).x.pSelect = pSelect;
        (*pExpr).flags |= (0x1000 as ::core::ffi::c_int | 0x400000 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0;
        sqlite3ExprSetHeightAndFlags(pParse, pExpr);
    } else {
        crate::src::src::select::sqlite3SelectDelete(
            (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pSelect as *mut crate::src::headers::sqliteInt_h::Select,
        );
    };
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprListToValues(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    nElem: ::core::ffi::c_int,
    pEList: *mut crate::src::headers::sqliteInt_h::ExprList,
) -> *mut crate::src::headers::sqliteInt_h::Select {
    let mut ii: ::core::ffi::c_int;
    let mut pRet: *mut crate::src::headers::sqliteInt_h::Select =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    ii = 0 as ::core::ffi::c_int;
    while ii < (*pEList).nExpr {
        let pSel: *mut crate::src::headers::sqliteInt_h::Select;
        let pExpr: *mut crate::src::headers::sqliteInt_h::Expr = (*(&raw mut (*pEList).a
            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(ii as isize))
        .pExpr;
        let nExprElem: ::core::ffi::c_int;
        if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_VECTOR {
            nExprElem = (*(*pExpr).x.pList).nExpr;
        } else {
            nExprElem = 1 as ::core::ffi::c_int;
        }
        if nExprElem != nElem {
            crate::src::src::util::sqlite3ErrorMsg_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                b"IN(...) element has %d term%s - expected %d\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[
                    crate::src::src::printf::PrintfArg::Int(
                        nExprElem as crate::src::ext::rtree::rtree::I64_0,
                    ),
                    crate::src::src::printf::PrintfArg::Str(
                        if nExprElem > 1 as ::core::ffi::c_int {
                            b"s\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        } as *mut ::core::ffi::c_char,
                    ),
                    crate::src::src::printf::PrintfArg::Int(
                        nElem as crate::src::ext::rtree::rtree::I64_0,
                    ),
                ],
            );
            break;
        } else {
            pSel = crate::src::src::select::sqlite3SelectNew(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*pExpr).x.pList as *mut crate::src::headers::sqliteInt_h::ExprList,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>()
                    as *mut crate::src::headers::sqliteInt_h::SrcList,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                crate::src::headers::sqliteInt_h::SF_Values as crate::src::ext::rtree::rtree::U32_0,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Select;
            (*pExpr).x.pList =
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
            if !pSel.is_null() {
                if !pRet.is_null() {
                    (*pSel).op = crate::src::parse::TK_ALL as crate::src::ext::rtree::rtree::U8_0;
                    (*pSel).pPrior = pRet;
                }
                pRet = pSel;
            }
            ii += 1;
        }
    }
    if !pRet.is_null() && !(*pRet).pPrior.is_null() {
        (*pRet).selFlags |=
            crate::src::headers::sqliteInt_h::SF_MultiValue as crate::src::ext::rtree::rtree::U32_0;
    }
    sqlite3ExprListDelete((*pParse).db, pEList);
    pRet
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprAnd(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pLeft: *mut crate::src::headers::sqliteInt_h::Expr,
    pRight: *mut crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    if pLeft.is_null() {
        pRight
    } else if pRight.is_null() {
        pLeft
    } else {
        let f: crate::src::ext::rtree::rtree::U32_0 = (*pLeft).flags | (*pRight).flags;
        if f & (crate::src::headers::sqliteInt_h::EP_OuterON
            | crate::src::headers::sqliteInt_h::EP_InnerON
            | crate::src::headers::sqliteInt_h::EP_IsFalse
            | crate::src::headers::sqliteInt_h::EP_HasFunc)
            as crate::src::ext::rtree::rtree::U32_0
            == crate::src::headers::sqliteInt_h::EP_IsFalse as crate::src::ext::rtree::rtree::U32_0
            && (((*pParse).eParseMode as ::core::ffi::c_int) < crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME)
        {
            sqlite3ExprDeferredDelete(pParse, pLeft);
            sqlite3ExprDeferredDelete(pParse, pRight);
            sqlite3Expr(
                db,
                crate::src::parse::TK_INTEGER,
                b"0\0" as *const u8 as *const ::core::ffi::c_char,
            )
        } else {
            sqlite3PExpr(pParse, crate::src::parse::TK_AND, pLeft, pRight)
        }
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprFunction(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pList: *mut crate::src::headers::sqliteInt_h::ExprList,
    pToken: *const crate::src::headers::sqliteInt_h::Token,
    eDistinct: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    
    let __pParse_ref = unsafe { &*pParse };
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
    let pNew: *mut crate::src::headers::sqliteInt_h::Expr = sqlite3ExprAlloc(
        db,
        crate::src::parse::TK_FUNCTION,
        pToken,
        1 as ::core::ffi::c_int,
    );
    if pNew.is_null() {
        sqlite3ExprListDelete(db, pList);
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
    }
    (*pNew).w.iOfst =
        (*pToken).z.offset_from(__pParse_ref.zTail) as ::core::ffi::c_long as ::core::ffi::c_int;
    if !pList.is_null()
        && (*pList).nExpr
            > (*__pParse_ref.db).aLimit
                [crate::src::headers::sqlite3_h::SQLITE_LIMIT_FUNCTION_ARG as usize]
        && __pParse_ref.nested == 0
    {
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"too many arguments on function %T\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Token(
                pToken as *mut crate::src::headers::sqliteInt_h::Token,
            )],
        );
    }
    (*pNew).x.pList = pList;
    (*pNew).flags |= 0x8 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
    sqlite3ExprSetHeightAndFlags(pParse, pNew);
    if eDistinct == crate::src::headers::sqliteInt_h::SF_Distinct {
        (*pNew).flags |= 0x4 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
    }
    pNew
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprOrderByAggregateError(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Expr,
) {
    crate::src::src::util::sqlite3ErrorMsg_args(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        b"ORDER BY may not be used with non-aggregate %#T()\0" as *const u8
            as *const ::core::ffi::c_char,
        &[crate::src::src::printf::PrintfArg::Expr(p)],
    );
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprAddFunctionOrderBy(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    pOrderBy: *mut crate::src::headers::sqliteInt_h::ExprList,
) {
    
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    if pOrderBy.is_null() {
        return;
    }
    if pExpr.is_null() {
        sqlite3ExprListDelete(db, pOrderBy);
        return;
    }
    let __pExpr_ref = unsafe { &mut *pExpr };
    if __pExpr_ref.x.pList.is_null() || (*__pExpr_ref.x.pList).nExpr == 0 as ::core::ffi::c_int {
        crate::src::src::prepare::sqlite3ParserAddCleanup(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            ::core::mem::transmute(Some(
                sqlite3ExprListDeleteGeneric
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqliteInt_h::sqlite3,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            )),
            pOrderBy as *mut ::core::ffi::c_void,
        );
        return;
    }
    if __pExpr_ref.flags & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
        && (*__pExpr_ref.y.pWin).eFrmType as ::core::ffi::c_int != crate::src::parse::TK_FILTER
    {
        sqlite3ExprOrderByAggregateError(pParse, pExpr);
        sqlite3ExprListDelete(db, pOrderBy);
        return;
    }
    if (*pOrderBy).nExpr
        > (*db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_COLUMN as usize]
    {
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"too many terms in ORDER BY clause\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
        sqlite3ExprListDelete(db, pOrderBy);
        return;
    }
    let pOB: *mut crate::src::headers::sqliteInt_h::Expr = sqlite3ExprAlloc(
        db,
        crate::src::parse::TK_ORDER,
        ::core::ptr::null::<crate::src::headers::sqliteInt_h::Token>(),
        0 as ::core::ffi::c_int,
    );
    if pOB.is_null() {
        sqlite3ExprListDelete(db, pOrderBy);
        return;
    }
    (*pOB).x.pList = pOrderBy;
    __pExpr_ref.pLeft = pOB;
    (*pOB).flags |= 0x20000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprFunctionUsable(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *const crate::src::headers::sqliteInt_h::Expr,
    pDef: *const crate::src::headers::sqliteInt_h::FuncDef,
) {
    if (*pExpr).flags & 0x40000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
        && ((*pDef).funcFlags
            & crate::src::headers::sqliteInt_h::SQLITE_FUNC_DIRECT
                as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
            || (*(*pParse).db).flags
                & crate::src::headers::sqliteInt_h::SQLITE_TrustedSchema
                    as crate::src::ext::rtree::rtree::U64_0
                == 0 as crate::src::ext::rtree::rtree::U64_0)
        {
            crate::src::src::util::sqlite3ErrorMsg_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                b"unsafe use of %#T()\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Expr(
                    pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
                )],
            );
        }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprAssignVarNumber(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    n: crate::src::ext::rtree::rtree::U32_0,
) {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    
    let mut x: crate::src::headers::sqliteInt_h::YnVar;
    if pExpr.is_null() {
        return;
    }
    let z: *const ::core::ffi::c_char = (*pExpr).u.zToken;
    if *z.offset(1_isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*pParse).nVar += 1;
        x = (*pParse).nVar;
    } else {
        let mut doAdd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if *z.offset(0_isize) as ::core::ffi::c_int == '?' as i32 {
            let mut i: crate::src::ext::rtree::rtree::I64_0 = 0;
            let bOk: ::core::ffi::c_int;
            if n == 2 as crate::src::ext::rtree::rtree::U32_0 {
                i = (*z.offset(1_isize) as ::core::ffi::c_int - '0' as i32)
                    as crate::src::ext::rtree::rtree::I64_0;
                bOk = 1 as ::core::ffi::c_int;
            } else {
                bOk = (0 as ::core::ffi::c_int
                    == crate::src::src::util::sqlite3Atoi64(
                        z.offset(1_isize) as *const ::core::ffi::c_char,
                        &raw mut i,
                        n.wrapping_sub(1 as crate::src::ext::rtree::rtree::U32_0)
                            as ::core::ffi::c_int,
                        crate::src::headers::sqlite3_h::SQLITE_UTF8
                            as crate::src::ext::rtree::rtree::U8_0,
                    )) as ::core::ffi::c_int;
            }
            if bOk == 0 as ::core::ffi::c_int
                || i < 1 as crate::src::ext::rtree::rtree::I64_0
                || i > (*db).aLimit
                    [crate::src::headers::sqlite3_h::SQLITE_LIMIT_VARIABLE_NUMBER as usize]
                    as crate::src::ext::rtree::rtree::I64_0
            {
                crate::src::src::util::sqlite3ErrorMsg_args(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    b"variable number must be between ?1 and ?%d\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[crate::src::src::printf::PrintfArg::Int(
                        (*db).aLimit
                            [crate::src::headers::sqlite3_h::SQLITE_LIMIT_VARIABLE_NUMBER as usize]
                            as crate::src::ext::rtree::rtree::I64_0,
                    )],
                );
                crate::src::src::printf::sqlite3RecordErrorOffsetOfExpr(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    pExpr as *const crate::src::headers::sqliteInt_h::Expr,
                );
                return;
            }
            x = i as crate::src::headers::sqliteInt_h::YnVar;
            if x as ::core::ffi::c_int > (*pParse).nVar as ::core::ffi::c_int {
                (*pParse).nVar = x as ::core::ffi::c_int as crate::src::headers::sqliteInt_h::YnVar;
                doAdd = 1 as ::core::ffi::c_int;
            } else if crate::src::src::util::sqlite3VListNumToName(
                (*pParse).pVList,
                x as ::core::ffi::c_int,
            )
            .is_null()
            {
                doAdd = 1 as ::core::ffi::c_int;
            }
        } else {
            x = crate::src::src::util::sqlite3VListNameToNum(
                (*pParse).pVList,
                z,
                n as ::core::ffi::c_int,
            ) as crate::src::headers::sqliteInt_h::YnVar;
            if x as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*pParse).nVar += 1;
                x = (*pParse).nVar;
                doAdd = 1 as ::core::ffi::c_int;
            }
        }
        if doAdd != 0 {
            (*pParse).pVList = crate::src::src::util::sqlite3VListAdd(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*pParse).pVList,
                z,
                n as ::core::ffi::c_int,
                x as ::core::ffi::c_int,
            );
        }
    }
    (*pExpr).iColumn = x;
    if x as ::core::ffi::c_int
        > (*db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_VARIABLE_NUMBER as usize]
    {
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"too many SQL variables\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
        crate::src::src::printf::sqlite3RecordErrorOffsetOfExpr(
            (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pExpr as *const crate::src::headers::sqliteInt_h::Expr,
        );
    }
}
#[inline(never)]
unsafe extern "C" fn sqlite3ExprDeleteNN(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut p: *mut crate::src::headers::sqliteInt_h::Expr,
) {
    while ((*p).flags
        & (0x10000 as ::core::ffi::c_int | 0x800000 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
    {
        if !(*p).pRight.is_null() {
            sqlite3ExprDeleteNN(db, (*p).pRight);
        } else if (*p).flags
            & crate::src::headers::sqliteInt_h::EP_xIsSelect as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
        {
            crate::src::src::select::sqlite3SelectDelete(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*p).x.pSelect as *mut crate::src::headers::sqliteInt_h::Select,
            );
        } else {
            sqlite3ExprListDelete(db, (*p).x.pList);
            if (*p).flags & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
            {
                crate::src::src::window::sqlite3WindowDelete(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    (*p).y.pWin as *mut crate::src::headers::sqliteInt_h::Window,
                );
            }
        }
        if !(!(*p).pLeft.is_null()
            && (*p).op as ::core::ffi::c_int != crate::src::parse::TK_SELECT_COLUMN)
        {
            break;
        }
        let pLeft: *mut crate::src::headers::sqliteInt_h::Expr = (*p).pLeft;
        if ((*p).flags & 0x8000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
            && ((*pLeft).flags
                & 0x8000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
        {
            crate::src::src::malloc::sqlite3DbNNFreeNN(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                p as *mut ::core::ffi::c_void,
            );
            p = pLeft;
        } else {
            sqlite3ExprDeleteNN(db, pLeft);
            break;
        }
    }
    if ((*p).flags & 0x8000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
    {
        crate::src::src::malloc::sqlite3DbNNFreeNN(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            p as *mut ::core::ffi::c_void,
        );
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprDelete(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *mut crate::src::headers::sqliteInt_h::Expr,
) {
    if !p.is_null() {
        sqlite3ExprDeleteNN(db, p);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprDeleteGeneric(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *mut ::core::ffi::c_void,
) {
    if !p.is_null() {
        sqlite3ExprDeleteNN(db, p as *mut crate::src::headers::sqliteInt_h::Expr);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ClearOnOrUsing(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *mut crate::src::headers::sqliteInt_h::OnOrUsing,
) {
    if !p.is_null() {
        if !(*p).pOn.is_null() {
            sqlite3ExprDeleteNN(db, (*p).pOn);
        } else if !(*p).pUsing.is_null() {
            crate::src::src::build::sqlite3IdListDelete(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*p).pUsing as *mut crate::src::headers::sqliteInt_h::IdList,
            );
        }
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprDeferredDelete(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    (::core::ptr::null_mut::<::core::ffi::c_void>()
        == crate::src::src::prepare::sqlite3ParserAddCleanup(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            ::core::mem::transmute(Some(
                sqlite3ExprDeleteGeneric
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqliteInt_h::sqlite3,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            )),
            pExpr as *mut ::core::ffi::c_void,
        )) as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprUnmapAndDelete(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Expr,
) {
    if !p.is_null() {
        if (*pParse).eParseMode as ::core::ffi::c_int
            >= crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME
        {
            crate::src::src::alter::sqlite3RenameExprUnmap(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                p as *mut crate::src::headers::sqliteInt_h::Expr,
            );
        }
        sqlite3ExprDeleteNN((*pParse).db, p);
    }
}

unsafe extern "C" fn exprStructSize(
    p: *const crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    if (*p).flags & 0x10000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return crate::src::headers::sqliteInt_h::EXPR_TOKENONLYSIZE as ::core::ffi::c_int;
    }
    if (*p).flags & 0x4000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return crate::src::headers::sqliteInt_h::EXPR_REDUCEDSIZE as ::core::ffi::c_int;
    }
    crate::src::headers::sqliteInt_h::EXPR_FULLSIZE as ::core::ffi::c_int
}

unsafe extern "C" fn dupedExprStructSize(
    p: *const crate::src::headers::sqliteInt_h::Expr,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let nSize: ::core::ffi::c_int;
    if 0 as ::core::ffi::c_int == flags
        || (*p).flags & 0x20000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        nSize = crate::src::headers::sqliteInt_h::EXPR_FULLSIZE as ::core::ffi::c_int;
    } else if !(*p).pLeft.is_null() || !(*p).x.pList.is_null() {
        nSize = (crate::src::headers::sqliteInt_h::EXPR_REDUCEDSIZE
            | crate::src::headers::sqliteInt_h::EP_Reduced as ::core::ffi::c_ulong)
            as ::core::ffi::c_int;
    } else {
        nSize = (crate::src::headers::sqliteInt_h::EXPR_TOKENONLYSIZE
            | crate::src::headers::sqliteInt_h::EP_TokenOnly as ::core::ffi::c_ulong)
            as ::core::ffi::c_int;
    }
    nSize
}

unsafe extern "C" fn dupedExprNodeSize(
    p: *const crate::src::headers::sqliteInt_h::Expr,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nByte: ::core::ffi::c_int = dupedExprStructSize(p, flags) & 0xfff as ::core::ffi::c_int;
    if ((*p).flags & 0x800 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
        && !(*p).u.zToken.is_null()
    {
        nByte = (nByte as crate::__stddef_size_t_h::SizeT).wrapping_add(
            (::libc::strlen((*p).u.zToken)
                & 0x3fffffff as ::core::ffi::c_int as crate::__stddef_size_t_h::SizeT)
                .wrapping_add(1 as crate::__stddef_size_t_h::SizeT),
        ) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    (nByte + 7 as ::core::ffi::c_int) & !(7 as ::core::ffi::c_int)
}

unsafe extern "C" fn dupedExprSize(
    p: *const crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let mut nByte: ::core::ffi::c_int;
    nByte = dupedExprNodeSize(p, crate::src::headers::sqliteInt_h::EXPRDUP_REDUCE);
    if !(*p).pLeft.is_null() {
        nByte += dupedExprSize((*p).pLeft);
    }
    if !(*p).pRight.is_null() {
        nByte += dupedExprSize((*p).pRight);
    }
    nByte
}

unsafe extern "C" fn exprDup(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *const crate::src::headers::sqliteInt_h::Expr,
    dupFlags: ::core::ffi::c_int,
    pEdupBuf: *mut EdupBuf,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    
    let mut sEdupBuf: EdupBuf = EdupBuf {
        zAlloc: ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U8_0>(),
    };
    let staticFlag: crate::src::ext::rtree::rtree::U32_0;
    let mut nToken: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if !pEdupBuf.is_null() {
        sEdupBuf.zAlloc = (*pEdupBuf).zAlloc;
        staticFlag =
            crate::src::headers::sqliteInt_h::EP_Static as crate::src::ext::rtree::rtree::U32_0;
    } else {
        let nAlloc: ::core::ffi::c_int;
        if dupFlags != 0 {
            nAlloc = dupedExprSize(p);
        } else if ((*p).flags
            & 0x800 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
            && !(*p).u.zToken.is_null()
        {
            nToken = (::libc::strlen((*p).u.zToken)
                & 0x3fffffff as ::core::ffi::c_int as crate::__stddef_size_t_h::SizeT)
                .wrapping_add(1 as crate::__stddef_size_t_h::SizeT)
                as ::core::ffi::c_int;
            nAlloc = ((::core::mem::size_of::<crate::src::headers::sqliteInt_h::Expr>() as usize)
                .wrapping_add(nToken as usize)
                .wrapping_add(7_usize)
                & !(7 as ::core::ffi::c_int) as usize) as ::core::ffi::c_int;
        } else {
            nToken = 0 as ::core::ffi::c_int;
            nAlloc = ((::core::mem::size_of::<crate::src::headers::sqliteInt_h::Expr>() as usize)
                .wrapping_add(7_usize)
                & !(7 as ::core::ffi::c_int) as usize) as ::core::ffi::c_int;
        }
        sEdupBuf.zAlloc = crate::src::src::malloc::sqlite3DbMallocRawNN(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            nAlloc as crate::src::ext::rtree::rtree::U64_0,
        ) as *mut crate::src::ext::rtree::rtree::U8_0;
        staticFlag = 0 as crate::src::ext::rtree::rtree::U32_0;
    }
    let pNew: *mut crate::src::headers::sqliteInt_h::Expr = sEdupBuf.zAlloc as *mut crate::src::headers::sqliteInt_h::Expr;
    if !pNew.is_null() {
        let nStructSize: ::core::ffi::c_uint =
            dupedExprStructSize(p, dupFlags) as ::core::ffi::c_uint;
        let mut nNewSize: ::core::ffi::c_int =
            (nStructSize & 0xfff as ::core::ffi::c_uint) as ::core::ffi::c_int;
        if nToken < 0 as ::core::ffi::c_int {
            if ((*p).flags & 0x800 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
                && !(*p).u.zToken.is_null()
            {
                nToken =
                    crate::src::src::util::sqlite3Strlen30((*p).u.zToken) + 1 as ::core::ffi::c_int;
            } else {
                nToken = 0 as ::core::ffi::c_int;
            }
        }
        if dupFlags != 0 {
            ::core::ptr::copy_nonoverlapping(
                p as *const u8,
                sEdupBuf.zAlloc as *mut u8,
                nNewSize as usize,
            );
        } else {
            let nSize: crate::src::ext::rtree::rtree::U32_0 =
                exprStructSize(p) as crate::src::ext::rtree::rtree::U32_0;
            ::core::ptr::copy_nonoverlapping(
                p as *const u8,
                sEdupBuf.zAlloc as *mut u8,
                nSize as usize,
            );
            if (nSize as usize) < crate::src::headers::sqliteInt_h::EXPR_FULLSIZE {
                ::libc::memset(
                    sEdupBuf.zAlloc.offset(nSize as isize)
                        as *mut crate::src::ext::rtree::rtree::U8_0
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    crate::src::headers::sqliteInt_h::EXPR_FULLSIZE
                        .wrapping_sub(nSize as crate::__stddef_size_t_h::SizeT),
                );
            }
            nNewSize = crate::src::headers::sqliteInt_h::EXPR_FULLSIZE as ::core::ffi::c_int;
        }
        let __pNew_ref = unsafe { &mut *pNew };
        __pNew_ref.flags &= !(crate::src::headers::sqliteInt_h::EP_Reduced
            | crate::src::headers::sqliteInt_h::EP_TokenOnly
            | crate::src::headers::sqliteInt_h::EP_Static)
            as crate::src::ext::rtree::rtree::U32_0;
        __pNew_ref.flags = (__pNew_ref.flags as ::core::ffi::c_uint
            | nStructSize
                & (crate::src::headers::sqliteInt_h::EP_Reduced
                    | crate::src::headers::sqliteInt_h::EP_TokenOnly)
                    as ::core::ffi::c_uint)
            as crate::src::ext::rtree::rtree::U32_0;
        __pNew_ref.flags |= staticFlag;
        dupFlags != 0;
        if nToken > 0 as ::core::ffi::c_int {
            __pNew_ref.u.zToken = sEdupBuf.zAlloc.offset(nNewSize as isize)
                as *mut crate::src::ext::rtree::rtree::U8_0
                as *mut ::core::ffi::c_char;
            let zToken: *mut ::core::ffi::c_char = __pNew_ref.u.zToken;
            ::core::ptr::copy_nonoverlapping(
                (*p).u.zToken as *const u8,
                zToken as *mut u8,
                nToken as usize,
            );
            nNewSize += nToken;
        }
        sEdupBuf.zAlloc = sEdupBuf
            .zAlloc
            .offset(((nNewSize + 7 as ::core::ffi::c_int) & !(7 as ::core::ffi::c_int)) as isize);
        if ((*p).flags | __pNew_ref.flags)
            & (crate::src::headers::sqliteInt_h::EP_TokenOnly
                | crate::src::headers::sqliteInt_h::EP_Leaf)
                as crate::src::ext::rtree::rtree::U32_0
            == 0 as crate::src::ext::rtree::rtree::U32_0
        {
            if (*p).flags
                & crate::src::headers::sqliteInt_h::EP_xIsSelect
                    as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
            {
                __pNew_ref.x.pSelect = sqlite3SelectDup(db, (*p).x.pSelect, dupFlags);
            } else {
                __pNew_ref.x.pList = sqlite3ExprListDup(
                    db,
                    (*p).x.pList,
                    if (*p).op as ::core::ffi::c_int != crate::src::parse::TK_ORDER {
                        dupFlags
                    } else {
                        0 as ::core::ffi::c_int
                    },
                );
            }
            if (*p).flags & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
            {
                __pNew_ref.y.pWin = crate::src::src::window::sqlite3WindowDup(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    pNew as *mut crate::src::headers::sqliteInt_h::Expr,
                    (*p).y.pWin as *mut crate::src::headers::sqliteInt_h::Window,
                )
                    as *mut crate::src::headers::sqliteInt_h::Window;
            }
            if dupFlags != 0 {
                if (*p).op as ::core::ffi::c_int == crate::src::parse::TK_SELECT_COLUMN {
                    __pNew_ref.pLeft = (*p).pLeft;
                } else {
                    __pNew_ref.pLeft = if !(*p).pLeft.is_null() {
                        exprDup(
                            db,
                            (*p).pLeft,
                            crate::src::headers::sqliteInt_h::EXPRDUP_REDUCE,
                            &raw mut sEdupBuf,
                        )
                    } else {
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    };
                }
                __pNew_ref.pRight = if !(*p).pRight.is_null() {
                    exprDup(
                        db,
                        (*p).pRight,
                        crate::src::headers::sqliteInt_h::EXPRDUP_REDUCE,
                        &raw mut sEdupBuf,
                    )
                } else {
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                };
            } else {
                if (*p).op as ::core::ffi::c_int == crate::src::parse::TK_SELECT_COLUMN {
                    __pNew_ref.pLeft = (*p).pLeft;
                } else {
                    __pNew_ref.pLeft = sqlite3ExprDup(db, (*p).pLeft, 0 as ::core::ffi::c_int);
                }
                __pNew_ref.pRight = sqlite3ExprDup(db, (*p).pRight, 0 as ::core::ffi::c_int);
            }
        }
    }
    if !pEdupBuf.is_null() {
        ::core::ptr::copy_nonoverlapping(
            &raw mut sEdupBuf as *const u8,
            pEdupBuf as *mut u8,
            ::core::mem::size_of::<EdupBuf>() as usize,
        );
    }
    pNew
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3WithDup(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *mut crate::src::headers::sqliteInt_h::With,
) -> *mut crate::src::headers::sqliteInt_h::With {
    let mut pRet: *mut crate::src::headers::sqliteInt_h::With =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::With>();
    if !p.is_null() {
        let nByte: crate::src::headers::sqlite3_h::Sqlite3Int64 =
            16_usize.wrapping_add(((*p).nCte as usize).wrapping_mul(::core::mem::size_of::<
                crate::src::headers::sqliteInt_h::Cte,
            >() as usize)) as crate::src::headers::sqlite3_h::Sqlite3Int64;
        pRet = crate::src::src::malloc::sqlite3DbMallocZero(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            nByte as crate::src::ext::rtree::rtree::U64_0,
        ) as *mut crate::src::headers::sqliteInt_h::With;
        if !pRet.is_null() {
            let mut i: ::core::ffi::c_int;
            (*pRet).nCte = (*p).nCte;
            i = 0 as ::core::ffi::c_int;
            while i < (*p).nCte {
                let __pRet_ref = unsafe { &mut *pRet };
                let fresh5 = &mut (*(&raw mut __pRet_ref.a
                    as *mut crate::src::headers::sqliteInt_h::Cte)
                    .offset(i as isize))
                .pSelect;
                let __p_ref = unsafe { &mut *p };
                *fresh5 = sqlite3SelectDup(
                    db,
                    (*(&raw mut __p_ref.a as *mut crate::src::headers::sqliteInt_h::Cte)
                        .offset(i as isize))
                    .pSelect,
                    0 as ::core::ffi::c_int,
                );
                let fresh6 = &mut (*(&raw mut __pRet_ref.a
                    as *mut crate::src::headers::sqliteInt_h::Cte)
                    .offset(i as isize))
                .pCols;
                *fresh6 = sqlite3ExprListDup(
                    db,
                    (*(&raw mut __p_ref.a as *mut crate::src::headers::sqliteInt_h::Cte)
                        .offset(i as isize))
                    .pCols,
                    0 as ::core::ffi::c_int,
                );
                let fresh7 = &mut (*(&raw mut __pRet_ref.a
                    as *mut crate::src::headers::sqliteInt_h::Cte)
                    .offset(i as isize))
                .zName;
                *fresh7 = crate::src::src::malloc::sqlite3DbStrDup(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    (*(&raw mut __p_ref.a as *mut crate::src::headers::sqliteInt_h::Cte)
                        .offset(i as isize))
                    .zName,
                );
                (*(&raw mut __pRet_ref.a as *mut crate::src::headers::sqliteInt_h::Cte)
                    .offset(i as isize))
                .eM10d = (*(&raw mut __p_ref.a as *mut crate::src::headers::sqliteInt_h::Cte)
                    .offset(i as isize))
                .eM10d;
                i += 1;
            }
        }
    }
    pRet
}

unsafe extern "C" fn gatherSelectWindowsCallback(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_FUNCTION
        && (*pExpr).flags & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        let pSelect: *mut crate::src::headers::sqliteInt_h::Select = (*pWalker).u.pSelect;
        let pWin: *mut crate::src::headers::sqliteInt_h::Window = (*pExpr).y.pWin;
        crate::src::src::window::sqlite3WindowLink(
            pSelect as *mut crate::src::headers::sqliteInt_h::Select,
            pWin as *mut crate::src::headers::sqliteInt_h::Window,
        );
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}

unsafe extern "C" fn gatherSelectWindowsSelectCallback(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    p: *mut crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    if p == (*pWalker).u.pSelect {
        crate::src::headers::sqliteInt_h::WRC_Continue
    } else {
        crate::src::headers::sqliteInt_h::WRC_Prune
    }
}

unsafe extern "C" fn gatherSelectWindows(p: *mut crate::src::headers::sqliteInt_h::Select) {
    let mut w: crate::src::headers::sqliteInt_h::Walker =
        crate::src::headers::sqliteInt_h::Walker {
            pParse: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Parse>(),
            xExprCallback: None,
            xSelectCallback: None,
            xSelectCallback2: None,
            walkerDepth: 0,
            eCode: 0,
            mWFlags: 0,
            u: crate::src::headers::sqliteInt_h::__anon_union_16 {
                pNC: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::NameContext>(),
            },
        };
    w.xExprCallback = Some(
        gatherSelectWindowsCallback
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
    w.xSelectCallback = Some(
        gatherSelectWindowsSelectCallback
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
    w.xSelectCallback2 = None;
    w.pParse = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Parse>();
    w.u.pSelect = p;
    crate::src::src::walker::sqlite3WalkSelect(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        p as *mut crate::src::headers::sqliteInt_h::Select,
    );
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprDup(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *const crate::src::headers::sqliteInt_h::Expr,
    flags: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    if !p.is_null() {
        exprDup(db, p, flags, ::core::ptr::null_mut::<EdupBuf>())
    } else {
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprListDup(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *const crate::src::headers::sqliteInt_h::ExprList,
    flags: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqliteInt_h::ExprList {
    
    let mut pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item;
    let mut pOldItem: *const crate::src::headers::sqliteInt_h::ExprList_item;
    let mut i: ::core::ffi::c_int;
    let mut pPriorSelectColOld: *mut crate::src::headers::sqliteInt_h::Expr =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
    let mut pPriorSelectColNew: *mut crate::src::headers::sqliteInt_h::Expr =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
    if p.is_null() {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
    }
    let pNew: *mut crate::src::headers::sqliteInt_h::ExprList = crate::src::src::malloc::sqlite3DbMallocRawNN(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        crate::src::src::malloc::sqlite3DbMallocSize(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            p as *const ::core::ffi::c_void,
        ) as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::headers::sqliteInt_h::ExprList;
    if pNew.is_null() {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
    }
    let __p_ref = unsafe { &*p };
    (*pNew).nExpr = __p_ref.nExpr;
    (*pNew).nAlloc = __p_ref.nAlloc;
    pItem = &raw mut (*pNew).a as *mut crate::src::headers::sqliteInt_h::ExprList_item
        as *mut crate::src::headers::sqliteInt_h::ExprList_item;
    pOldItem = &raw const __p_ref.a as *const crate::src::headers::sqliteInt_h::ExprList_item
        as *const crate::src::headers::sqliteInt_h::ExprList_item;
    i = 0 as ::core::ffi::c_int;
    while i < __p_ref.nExpr {
        let pOldExpr: *mut crate::src::headers::sqliteInt_h::Expr = (*pOldItem).pExpr;
        let pNewExpr: *mut crate::src::headers::sqliteInt_h::Expr;
        (*pItem).pExpr = sqlite3ExprDup(db, pOldExpr, flags);
        if !pOldExpr.is_null()
            && (*pOldExpr).op as ::core::ffi::c_int == crate::src::parse::TK_SELECT_COLUMN
            && {
                pNewExpr = (*pItem).pExpr;
                !pNewExpr.is_null()
            }
        {
            if !(*pNewExpr).pRight.is_null() {
                pPriorSelectColOld = (*pOldExpr).pRight;
                let __pNewExpr_ref = unsafe { &mut *pNewExpr };
                pPriorSelectColNew = __pNewExpr_ref.pRight;
                __pNewExpr_ref.pLeft = __pNewExpr_ref.pRight;
            } else {
                if (*pOldExpr).pLeft != pPriorSelectColOld {
                    pPriorSelectColOld = (*pOldExpr).pLeft;
                    pPriorSelectColNew = sqlite3ExprDup(db, pPriorSelectColOld, flags);
                    (*pNewExpr).pRight = pPriorSelectColNew;
                }
                (*pNewExpr).pLeft = pPriorSelectColNew;
            }
        }
        (*pItem).zEName = crate::src::src::malloc::sqlite3DbStrDup(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*pOldItem).zEName,
        );
        (*pItem).fg = (*pOldItem).fg;
        (*pItem).u = (*pOldItem).u;
        i += 1;
        pItem = pItem.offset(1);
        pOldItem = pOldItem.offset(1);
    }
    pNew
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SrcListDup(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *const crate::src::headers::sqliteInt_h::SrcList,
    flags: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqliteInt_h::SrcList {
    
    let mut i: ::core::ffi::c_int;
    if p.is_null() {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>();
    }
    let pNew: *mut crate::src::headers::sqliteInt_h::SrcList = crate::src::src::malloc::sqlite3DbMallocRawNN(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        8_usize.wrapping_add(((*p).nSrc as usize).wrapping_mul(::core::mem::size_of::<
            crate::src::headers::sqliteInt_h::SrcItem,
        >() as usize)) as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::headers::sqliteInt_h::SrcList;
    if pNew.is_null() {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>();
    }
    (*pNew).nAlloc = (*p).nSrc as crate::src::ext::rtree::rtree::U32_0;
    (*pNew).nSrc = (*pNew).nAlloc as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*p).nSrc {
        let pNewItem: *mut crate::src::headers::sqliteInt_h::SrcItem =
            (&raw mut (*pNew).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                .offset(i as isize) as *mut crate::src::headers::sqliteInt_h::SrcItem;
        let pOldItem: *const crate::src::headers::sqliteInt_h::SrcItem =
            (&raw const (*p).a as *const crate::src::headers::sqliteInt_h::SrcItem)
                .offset(i as isize) as *const crate::src::headers::sqliteInt_h::SrcItem;
        
        (*pNewItem).fg = (*pOldItem).fg;
        if (*pOldItem).fg.isSubquery() != 0 {
            let mut pNewSubq: *mut crate::src::headers::sqliteInt_h::Subquery =
                crate::src::src::malloc::sqlite3DbMallocRaw(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Subquery>()
                        as crate::src::ext::rtree::rtree::U64_0,
                ) as *mut crate::src::headers::sqliteInt_h::Subquery;
            if pNewSubq.is_null() {
                (*pNewItem)
                    .fg
                    .set_isSubquery(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else {
                ::core::ptr::copy_nonoverlapping(
                    (*pOldItem).u4.pSubq as *const u8,
                    pNewSubq as *mut u8,
                    ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Subquery>() as usize,
                );
                (*pNewSubq).pSelect = sqlite3SelectDup(db, (*pNewSubq).pSelect, flags);
                if (*pNewSubq).pSelect.is_null() {
                    crate::src::src::malloc::sqlite3DbFree(
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        pNewSubq as *mut ::core::ffi::c_void,
                    );
                    pNewSubq =
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Subquery>();
                    (*pNewItem)
                        .fg
                        .set_isSubquery(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
            }
            (*pNewItem).u4.pSubq = pNewSubq;
        } else if (*pOldItem).fg.fixedSchema() != 0 {
            (*pNewItem).u4.pSchema = (*pOldItem).u4.pSchema;
        } else {
            (*pNewItem).u4.zDatabase = crate::src::src::malloc::sqlite3DbStrDup(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*pOldItem).u4.zDatabase,
            );
        }
        (*pNewItem).zName = crate::src::src::malloc::sqlite3DbStrDup(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*pOldItem).zName,
        );
        (*pNewItem).zAlias = crate::src::src::malloc::sqlite3DbStrDup(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*pOldItem).zAlias,
        );
        (*pNewItem).iCursor = (*pOldItem).iCursor;
        if (*pNewItem).fg.isIndexedBy() != 0 {
            (*pNewItem).u1.zIndexedBy = crate::src::src::malloc::sqlite3DbStrDup(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*pOldItem).u1.zIndexedBy,
            );
        } else if (*pNewItem).fg.isTabFunc() != 0 {
            (*pNewItem).u1.pFuncArg = sqlite3ExprListDup(db, (*pOldItem).u1.pFuncArg, flags);
        } else {
            (*pNewItem).u1.nRow = (*pOldItem).u1.nRow;
        }
        (*pNewItem).u2 = (*pOldItem).u2;
        if (*pNewItem).fg.isCte() != 0 {
            (*(*pNewItem).u2.pCteUse).nUse += 1;
        }
        (*pNewItem).pSTab = (*pOldItem).pSTab;
        let pTab: *mut crate::src::headers::sqliteInt_h::Table = (*pNewItem).pSTab;
        if !pTab.is_null() {
            (*pTab).nTabRef = (*pTab).nTabRef.wrapping_add(1);
        }
        if (*pOldItem).fg.isUsing() != 0 {
            (*pNewItem).u3.pUsing = sqlite3IdListDup(db, (*pOldItem).u3.pUsing);
        } else {
            (*pNewItem).u3.pOn = sqlite3ExprDup(db, (*pOldItem).u3.pOn, flags);
        }
        (*pNewItem).colUsed = (*pOldItem).colUsed;
        i += 1;
    }
    pNew
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3IdListDup(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *const crate::src::headers::sqliteInt_h::IdList,
) -> *mut crate::src::headers::sqliteInt_h::IdList {
    
    let mut i: ::core::ffi::c_int;
    if p.is_null() {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::IdList>();
    }
    let pNew: *mut crate::src::headers::sqliteInt_h::IdList = crate::src::src::malloc::sqlite3DbMallocRawNN(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        8_usize.wrapping_add(((*p).nId as usize).wrapping_mul(::core::mem::size_of::<
            crate::src::headers::sqliteInt_h::IdList_item,
        >() as usize)) as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::headers::sqliteInt_h::IdList;
    if pNew.is_null() {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::IdList>();
    }
    (*pNew).nId = (*p).nId;
    i = 0 as ::core::ffi::c_int;
    while i < (*p).nId {
        let pNewItem: *mut crate::src::headers::sqliteInt_h::IdList_item = (&raw mut (*pNew).a
            as *mut crate::src::headers::sqliteInt_h::IdList_item)
            .offset(i as isize)
            as *mut crate::src::headers::sqliteInt_h::IdList_item;
        let pOldItem: *const crate::src::headers::sqliteInt_h::IdList_item = (&raw const (*p).a
            as *const crate::src::headers::sqliteInt_h::IdList_item)
            .offset(i as isize)
            as *const crate::src::headers::sqliteInt_h::IdList_item;
        (*pNewItem).zName = crate::src::src::malloc::sqlite3DbStrDup(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*pOldItem).zName,
        );
        i += 1;
    }
    pNew
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SelectDup(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pDup: *const crate::src::headers::sqliteInt_h::Select,
    flags: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqliteInt_h::Select {
    let mut pRet: *mut crate::src::headers::sqliteInt_h::Select =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    let mut pNext: *mut crate::src::headers::sqliteInt_h::Select =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    let mut pp: *mut *mut crate::src::headers::sqliteInt_h::Select = &raw mut pRet;
    let mut p: *const crate::src::headers::sqliteInt_h::Select;
    p = pDup;
    while !p.is_null() {
        let pNew: *mut crate::src::headers::sqliteInt_h::Select =
            crate::src::src::malloc::sqlite3DbMallocRawNN(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Select>()
                    as crate::src::ext::rtree::rtree::U64_0,
            ) as *mut crate::src::headers::sqliteInt_h::Select;
        if pNew.is_null() {
            break;
        }
        let __pNew_ref = unsafe { &mut *pNew };
        __pNew_ref.pEList = sqlite3ExprListDup(db, (*p).pEList, flags);
        __pNew_ref.pSrc = sqlite3SrcListDup(db, (*p).pSrc, flags);
        __pNew_ref.pWhere = sqlite3ExprDup(db, (*p).pWhere, flags);
        __pNew_ref.pGroupBy = sqlite3ExprListDup(db, (*p).pGroupBy, flags);
        __pNew_ref.pHaving = sqlite3ExprDup(db, (*p).pHaving, flags);
        __pNew_ref.pOrderBy = sqlite3ExprListDup(db, (*p).pOrderBy, flags);
        __pNew_ref.op = (*p).op;
        __pNew_ref.pNext = pNext;
        __pNew_ref.pPrior = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
        __pNew_ref.pLimit = sqlite3ExprDup(db, (*p).pLimit, flags);
        __pNew_ref.iLimit = 0 as ::core::ffi::c_int;
        __pNew_ref.iOffset = 0 as ::core::ffi::c_int;
        __pNew_ref.selFlags = (*p).selFlags
            & !(crate::src::headers::sqliteInt_h::SF_UsesEphemeral
                as crate::src::ext::rtree::rtree::U32_0);
        __pNew_ref.addrOpenEphm[0 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
        __pNew_ref.addrOpenEphm[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
        __pNew_ref.nSelectRow = (*p).nSelectRow;
        __pNew_ref.pWith = sqlite3WithDup(db, (*p).pWith);
        __pNew_ref.pWin = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Window>();
        __pNew_ref.pWinDefn = crate::src::src::window::sqlite3WindowListDup(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*p).pWinDefn as *mut crate::src::headers::sqliteInt_h::Window,
        ) as *mut crate::src::headers::sqliteInt_h::Window;
        if !(*p).pWin.is_null()
            && (*db).mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            gatherSelectWindows(pNew);
        }
        __pNew_ref.selId = (*p).selId;
        if (*db).mallocFailed != 0 {
            __pNew_ref.pNext = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
            crate::src::src::select::sqlite3SelectDelete(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                pNew as *mut crate::src::headers::sqliteInt_h::Select,
            );
            break;
        } else {
            *pp = pNew;
            pp = &raw mut __pNew_ref.pPrior;
            pNext = pNew;
            p = (*p).pPrior;
        }
    }
    pRet
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]

static mut zeroItem: crate::src::headers::sqliteInt_h::ExprList_item =
    crate::src::headers::sqliteInt_h::ExprList_item {
        pExpr: ::core::ptr::null::<crate::src::headers::sqliteInt_h::Expr>()
            as *mut crate::src::headers::sqliteInt_h::Expr,
        zEName: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        fg: crate::src::headers::sqliteInt_h::__anon_struct_4 {
            sortFlags: 0,
            eEName_done_reusable_bSorterRef_bNulls_bUsed_bUsingTerm_bNoExpand: [0; 2],
            c2rust_padding: [0; 1],
        },
        u: crate::src::headers::sqliteInt_h::__anon_union_9 {
            x: crate::src::headers::sqliteInt_h::__anon_struct_5 {
                iOrderByCol: 0,
                iAlias: 0,
            },
        },
    };
#[inline(never)]
pub unsafe extern "C" fn sqlite3ExprListAppendNew(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::ExprList {
    
    
    let pList: *mut crate::src::headers::sqliteInt_h::ExprList = crate::src::src::malloc::sqlite3DbMallocRawNN(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        8_usize.wrapping_add(4_usize.wrapping_mul(::core::mem::size_of::<
            crate::src::headers::sqliteInt_h::ExprList_item,
        >() as usize)) as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::headers::sqliteInt_h::ExprList;
    if pList.is_null() {
        sqlite3ExprDelete(db, pExpr);
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
    }
    (*pList).nAlloc = 4 as ::core::ffi::c_int;
    (*pList).nExpr = 1 as ::core::ffi::c_int;
    let pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item = (&raw mut (*pList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
        .offset(0_isize) as *mut crate::src::headers::sqliteInt_h::ExprList_item
        as *mut crate::src::headers::sqliteInt_h::ExprList_item;
    *pItem = zeroItem;
    (*pItem).pExpr = pExpr;
    pList
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]
#[inline(never)]
pub unsafe extern "C" fn sqlite3ExprListAppendGrow(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut pList: *mut crate::src::headers::sqliteInt_h::ExprList,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::ExprList {
    
    
    (*pList).nAlloc *= 2 as ::core::ffi::c_int;
    let pNew: *mut crate::src::headers::sqliteInt_h::ExprList = crate::src::src::malloc::sqlite3DbRealloc(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pList as *mut ::core::ffi::c_void,
            8_usize.wrapping_add(((*pList).nAlloc as usize).wrapping_mul(
                ::core::mem::size_of::<crate::src::headers::sqliteInt_h::ExprList_item>() as usize,
            )) as crate::src::ext::rtree::rtree::U64_0,
        ) as *mut crate::src::headers::sqliteInt_h::ExprList;
    if pNew.is_null() {
        sqlite3ExprListDelete(db, pList);
        sqlite3ExprDelete(db, pExpr);
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
    } else {
        pList = pNew;
    }
    let fresh2 = (*pList).nExpr;
    (*pList).nExpr += 1;
    let pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item = (&raw mut (*pList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
        .offset(fresh2 as isize) as *mut crate::src::headers::sqliteInt_h::ExprList_item
        as *mut crate::src::headers::sqliteInt_h::ExprList_item;
    *pItem = zeroItem;
    (*pItem).pExpr = pExpr;
    pList
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprListAppend(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pList: *mut crate::src::headers::sqliteInt_h::ExprList,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::ExprList {
    
    if pList.is_null() {
        return sqlite3ExprListAppendNew((*pParse).db, pExpr);
    }
    let __pList_ref = unsafe { &mut *pList };
    if __pList_ref.nAlloc < __pList_ref.nExpr + 1 as ::core::ffi::c_int {
        return sqlite3ExprListAppendGrow((*pParse).db, pList, pExpr);
    }
    let fresh1 = __pList_ref.nExpr;
    __pList_ref.nExpr += 1;
    let pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item = (&raw mut __pList_ref.a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
        .offset(fresh1 as isize) as *mut crate::src::headers::sqliteInt_h::ExprList_item
        as *mut crate::src::headers::sqliteInt_h::ExprList_item;
    *pItem = zeroItem;
    (*pItem).pExpr = pExpr;
    pList
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprListAppendVector(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut pList: *mut crate::src::headers::sqliteInt_h::ExprList,
    pColumns: *mut crate::src::headers::sqliteInt_h::IdList,
    mut pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::ExprList {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    let n: ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    let iFirst: ::core::ffi::c_int = if !pList.is_null() {
        (*pList).nExpr
    } else {
        0 as ::core::ffi::c_int
    };
    if !pColumns.is_null()
        && !pExpr.is_null() {
            if (*pExpr).op as ::core::ffi::c_int != crate::src::parse::TK_SELECT && {
                n = sqlite3ExprVectorSize(pExpr);
                (*pColumns).nId != n
            } {
                crate::src::src::util::sqlite3ErrorMsg_args(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    b"%d columns assigned %d values\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::src::src::printf::PrintfArg::Int(
                            (*pColumns).nId as crate::src::ext::rtree::rtree::I64_0,
                        ),
                        crate::src::src::printf::PrintfArg::Int(
                            n as crate::src::ext::rtree::rtree::I64_0,
                        ),
                    ],
                );
            } else {
                i = 0 as ::core::ffi::c_int;
                while i < (*pColumns).nId {
                    let pSubExpr: *mut crate::src::headers::sqliteInt_h::Expr =
                        sqlite3ExprForVectorField(pParse, pExpr, i, (*pColumns).nId);
                    if !pSubExpr.is_null() {
                        pList = sqlite3ExprListAppend(pParse, pList, pSubExpr);
                        if !pList.is_null() {
                            let fresh3 = &mut (*(&raw mut (*pList).a
                                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                .offset(((*pList).nExpr - 1 as ::core::ffi::c_int) as isize))
                            .zEName;
                            *fresh3 = (*(&raw mut (*pColumns).a
                                as *mut crate::src::headers::sqliteInt_h::IdList_item)
                                .offset(i as isize))
                            .zName;
                            let fresh4 = &mut (*(&raw mut (*pColumns).a
                                as *mut crate::src::headers::sqliteInt_h::IdList_item)
                                .offset(i as isize))
                            .zName;
                            *fresh4 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                    }
                    i += 1;
                }
                if (*db).mallocFailed == 0
                    && (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_SELECT
                    && !pList.is_null()
                {
                    let pFirst: *mut crate::src::headers::sqliteInt_h::Expr =
                        (*(&raw mut (*pList).a
                            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                            .offset(iFirst as isize))
                        .pExpr;
                    (*pFirst).pRight = pExpr;
                    pExpr = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
                    (*pFirst).iTable = (*pColumns).nId;
                }
            }
        }
    sqlite3ExprUnmapAndDelete(pParse, pExpr);
    crate::src::src::build::sqlite3IdListDelete(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        pColumns as *mut crate::src::headers::sqliteInt_h::IdList,
    );
    pList
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprListSetSortOrder(
    p: *mut crate::src::headers::sqliteInt_h::ExprList,
    mut iSortOrder: ::core::ffi::c_int,
    eNulls: ::core::ffi::c_int,
) {
    
    if p.is_null() {
        return;
    }
    let pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item = (&raw mut (*p).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
        .offset(((*p).nExpr - 1 as ::core::ffi::c_int) as isize)
        as *mut crate::src::headers::sqliteInt_h::ExprList_item
        as *mut crate::src::headers::sqliteInt_h::ExprList_item;
    if iSortOrder == crate::src::headers::sqliteInt_h::SQLITE_SO_UNDEFINED {
        iSortOrder = crate::src::headers::sqliteInt_h::SQLITE_SO_ASC;
    }
    (*pItem).fg.sortFlags = iSortOrder as crate::src::ext::rtree::rtree::U8_0;
    if eNulls != crate::src::headers::sqliteInt_h::SQLITE_SO_UNDEFINED {
        (*pItem)
            .fg
            .set_bNulls(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if iSortOrder != eNulls {
            (*pItem).fg.sortFlags = ((*pItem).fg.sortFlags as ::core::ffi::c_int
                | crate::src::headers::sqliteInt_h::KEYINFO_ORDER_BIGNULL)
                as crate::src::ext::rtree::rtree::U8_0;
        }
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprListSetName(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pList: *mut crate::src::headers::sqliteInt_h::ExprList,
    pName: *const crate::src::headers::sqliteInt_h::Token,
    dequote: ::core::ffi::c_int,
) {
    if !pList.is_null() {
        
        let pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item = (&raw mut (*pList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(((*pList).nExpr - 1 as ::core::ffi::c_int) as isize)
            as *mut crate::src::headers::sqliteInt_h::ExprList_item
            as *mut crate::src::headers::sqliteInt_h::ExprList_item;
        (*pItem).zEName = crate::src::src::malloc::sqlite3DbStrNDup(
            (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*pName).z,
            (*pName).n as crate::src::ext::rtree::rtree::U64_0,
        );
        if dequote != 0 {
            crate::src::src::util::sqlite3Dequote((*pItem).zEName);
            if (*pParse).eParseMode as ::core::ffi::c_int
                >= crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME
            {
                crate::src::src::alter::sqlite3RenameTokenMap(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*pItem).zEName as *const ::core::ffi::c_void,
                    pName as *const crate::src::headers::sqliteInt_h::Token,
                );
            }
        }
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprListSetSpan(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pList: *mut crate::src::headers::sqliteInt_h::ExprList,
    zStart: *const ::core::ffi::c_char,
    zEnd: *const ::core::ffi::c_char,
) {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    if !pList.is_null() {
        let pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item = (&raw mut (*pList).a
            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(((*pList).nExpr - 1 as ::core::ffi::c_int) as isize)
            as *mut crate::src::headers::sqliteInt_h::ExprList_item;
        if (*pItem).zEName.is_null() {
            (*pItem).zEName = crate::src::src::malloc::sqlite3DbSpanDup(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                zStart,
                zEnd,
            );
            (*pItem).fg.set_eEName(
                crate::src::headers::sqliteInt_h::ENAME_SPAN as ::core::ffi::c_uint
                    as ::core::ffi::c_uint,
            );
        }
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprListCheckLength(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pEList: *mut crate::src::headers::sqliteInt_h::ExprList,
    zObject: *const ::core::ffi::c_char,
) {
    let mx: ::core::ffi::c_int =
        (*(*pParse).db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_COLUMN as usize];
    if !pEList.is_null() && (*pEList).nExpr > mx {
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"too many columns in %s\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Str(
                zObject as *mut ::core::ffi::c_char,
            )],
        );
    }
}
#[inline(never)]
unsafe extern "C" fn exprListDeleteNN(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pList: *mut crate::src::headers::sqliteInt_h::ExprList,
) {
    let mut i: ::core::ffi::c_int = (*pList).nExpr;
    let mut pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item =
        &raw mut (*pList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item;
    loop {
        sqlite3ExprDelete(db, (*pItem).pExpr);
        if !(*pItem).zEName.is_null() {
            crate::src::src::malloc::sqlite3DbNNFreeNN(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*pItem).zEName as *mut ::core::ffi::c_void,
            );
        }
        pItem = pItem.offset(1);
        i -= 1;
        if (i <= 0 as ::core::ffi::c_int) {
            break;
        }
    }
    crate::src::src::malloc::sqlite3DbNNFreeNN(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        pList as *mut ::core::ffi::c_void,
    );
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprListDelete(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pList: *mut crate::src::headers::sqliteInt_h::ExprList,
) {
    if !pList.is_null() {
        exprListDeleteNN(db, pList);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprListDeleteGeneric(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pList: *mut ::core::ffi::c_void,
) {
    if !pList.is_null() {
        exprListDeleteNN(db, pList as *mut crate::src::headers::sqliteInt_h::ExprList);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprListFlags(
    pList: *const crate::src::headers::sqliteInt_h::ExprList,
) -> crate::src::ext::rtree::rtree::U32_0 {
    let mut i: ::core::ffi::c_int;
    let mut m: crate::src::ext::rtree::rtree::U32_0 = 0 as crate::src::ext::rtree::rtree::U32_0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pList).nExpr {
        let pExpr: *mut crate::src::headers::sqliteInt_h::Expr = (*(&raw const (*pList).a
            as *const crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(i as isize))
        .pExpr;
        m |= (*pExpr).flags;
        i += 1;
    }
    m
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SelectWalkFail(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    mut _NotUsed: *mut crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    (*pWalker).eCode = 0 as crate::src::fts5::U16_0;
    crate::src::headers::sqliteInt_h::WRC_Abort
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3IsTrueOrFalse(
    zIn: *const ::core::ffi::c_char,
) -> crate::src::ext::rtree::rtree::U32_0 {
    if crate::src::src::util::sqlite3StrICmp(
        zIn,
        b"true\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return crate::src::headers::sqliteInt_h::EP_IsTrue as crate::src::ext::rtree::rtree::U32_0;
    }
    if crate::src::src::util::sqlite3StrICmp(
        zIn,
        b"false\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return crate::src::headers::sqliteInt_h::EP_IsFalse
            as crate::src::ext::rtree::rtree::U32_0;
    }
    0 as crate::src::ext::rtree::rtree::U32_0
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprIdToTrueFalse(
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let v: crate::src::ext::rtree::rtree::U32_0;
    if ((*pExpr).flags
        & (0x4000000 as ::core::ffi::c_int | 0x800 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
        && {
            v = sqlite3IsTrueOrFalse((*pExpr).u.zToken);
            v != 0 as crate::src::ext::rtree::rtree::U32_0
        }
    {
        (*pExpr).op = crate::src::parse::TK_TRUEFALSE as crate::src::ext::rtree::rtree::U8_0;
        (*pExpr).flags |= v;
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprTruthValue(
    mut pExpr: *const crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    pExpr = sqlite3ExprSkipCollateAndLikely(pExpr as *mut crate::src::headers::sqliteInt_h::Expr);
    (*(*pExpr).u.zToken.offset(4_isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprSimplifiedAndOr(
    mut pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_AND
        || (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_OR
    {
        let pRight: *mut crate::src::headers::sqliteInt_h::Expr =
            sqlite3ExprSimplifiedAndOr((*pExpr).pRight);
        let pLeft: *mut crate::src::headers::sqliteInt_h::Expr =
            sqlite3ExprSimplifiedAndOr((*pExpr).pLeft);
        if (*pLeft).flags
            & (crate::src::headers::sqliteInt_h::EP_OuterON
                | crate::src::headers::sqliteInt_h::EP_IsTrue)
                as crate::src::ext::rtree::rtree::U32_0
            == crate::src::headers::sqliteInt_h::EP_IsTrue as crate::src::ext::rtree::rtree::U32_0
            || (*pRight).flags
                & (crate::src::headers::sqliteInt_h::EP_OuterON
                    | crate::src::headers::sqliteInt_h::EP_IsFalse)
                    as crate::src::ext::rtree::rtree::U32_0
                == crate::src::headers::sqliteInt_h::EP_IsFalse
                    as crate::src::ext::rtree::rtree::U32_0
        {
            pExpr = if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_AND {
                pRight
            } else {
                pLeft
            };
        } else if (*pRight).flags
            & (crate::src::headers::sqliteInt_h::EP_OuterON
                | crate::src::headers::sqliteInt_h::EP_IsTrue)
                as crate::src::ext::rtree::rtree::U32_0
            == crate::src::headers::sqliteInt_h::EP_IsTrue as crate::src::ext::rtree::rtree::U32_0
            || (*pLeft).flags
                & (crate::src::headers::sqliteInt_h::EP_OuterON
                    | crate::src::headers::sqliteInt_h::EP_IsFalse)
                    as crate::src::ext::rtree::rtree::U32_0
                == crate::src::headers::sqliteInt_h::EP_IsFalse
                    as crate::src::ext::rtree::rtree::U32_0
        {
            pExpr = if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_AND {
                pLeft
            } else {
                pRight
            };
        }
    }
    pExpr
}

unsafe extern "C" fn exprEvalRhsFirst(
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    if (*(*pExpr).pLeft).flags
        & 0x400000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
        && ((*(*pExpr).pRight).flags
            & 0x400000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }
}

unsafe extern "C" fn exprComputeOperands(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    pR1: *mut ::core::ffi::c_int,
    pR2: *mut ::core::ffi::c_int,
    pFree1: *mut ::core::ffi::c_int,
    pFree2: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut addrIsNull: ::core::ffi::c_int;
    
    let mut r2: ::core::ffi::c_int;
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    if exprEvalRhsFirst(pExpr) != 0 && sqlite3ExprCanBeNull((*pExpr).pRight) != 0 {
        r2 = sqlite3ExprCodeTemp(pParse, (*pExpr).pRight, pFree2);
        addrIsNull = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
            v,
            crate::src::headers::opcodes_h::OP_IsNull,
            r2,
        );
    } else {
        r2 = 0 as ::core::ffi::c_int;
        addrIsNull = 0 as ::core::ffi::c_int;
    }
    let r1: ::core::ffi::c_int = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, pFree1);
    if addrIsNull == 0 as ::core::ffi::c_int {
        let __pExpr_ref = unsafe { &*pExpr };
        if (*__pExpr_ref.pRight).flags
            & 0x400000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
            && sqlite3ExprCanBeNull(__pExpr_ref.pLeft) != 0
        {
            addrIsNull = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                v,
                crate::src::headers::opcodes_h::OP_IsNull,
                r1,
            );
        }
        r2 = sqlite3ExprCodeTemp(pParse, __pExpr_ref.pRight, pFree2);
    }
    *pR1 = r1;
    *pR2 = r2;
    addrIsNull
}
#[inline(never)]
unsafe extern "C" fn exprNodeIsConstantFunction(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let n: ::core::ffi::c_int;
    let pList: *mut crate::src::headers::sqliteInt_h::ExprList;
    
    
    let __pExpr_ref = unsafe { &*pExpr };
    if __pExpr_ref.flags & 0x10000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
        || {
            pList = __pExpr_ref.x.pList;
            pList.is_null()
        }
    {
        n = 0 as ::core::ffi::c_int;
    } else {
        n = (*pList).nExpr;
        crate::src::src::walker::sqlite3WalkExprList(
            pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
            pList as *mut crate::src::headers::sqliteInt_h::ExprList,
        );
        if (*pWalker).eCode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return crate::src::headers::sqliteInt_h::WRC_Abort;
        }
    }
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*(*pWalker).pParse).db;
    let pDef: *mut crate::src::headers::sqliteInt_h::FuncDef = crate::src::src::callback::sqlite3FindFunction(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __pExpr_ref.u.zToken,
        n,
        (*db).enc,
        0 as crate::src::ext::rtree::rtree::U8_0,
    ) as *mut crate::src::headers::sqliteInt_h::FuncDef;
    if pDef.is_null()
        || (*pDef).xFinalize.is_some()
        || (*pDef).funcFlags
            & (crate::src::headers::sqliteInt_h::SQLITE_FUNC_CONSTANT
                | crate::src::headers::sqliteInt_h::SQLITE_FUNC_SLOCHNG)
                as crate::src::ext::rtree::rtree::U32_0
            == 0 as crate::src::ext::rtree::rtree::U32_0
        || __pExpr_ref.flags
            & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        (*pWalker).eCode = 0 as crate::src::fts5::U16_0;
        return crate::src::headers::sqliteInt_h::WRC_Abort;
    }
    crate::src::headers::sqliteInt_h::WRC_Prune
}

unsafe extern "C" fn exprNodeIsConstant(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    if (*pWalker).eCode as ::core::ffi::c_int == 2 as ::core::ffi::c_int
        && (*pExpr).flags & 0x1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        (*pWalker).eCode = 0 as crate::src::fts5::U16_0;
        return crate::src::headers::sqliteInt_h::WRC_Abort;
    }
    let current_block_47: u64;
    match (*pExpr).op as ::core::ffi::c_int {
        crate::src::parse::TK_FUNCTION => {
            if ((*pWalker).eCode as ::core::ffi::c_int >= 4 as ::core::ffi::c_int
                || (*pExpr).flags
                    & 0x100000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                    != 0 as crate::src::ext::rtree::rtree::U32_0)
                && ((*pExpr).flags
                    & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
            {
                if (*pWalker).eCode as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
                    (*pExpr).flags |=
                        0x40000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
                }
                return crate::src::headers::sqliteInt_h::WRC_Continue;
            } else if !(*pWalker).pParse.is_null() {
                return exprNodeIsConstantFunction(pWalker, pExpr);
            } else {
                (*pWalker).eCode = 0 as crate::src::fts5::U16_0;
                return crate::src::headers::sqliteInt_h::WRC_Abort;
            }
        }
        crate::src::parse::TK_ID_1 => {
            if sqlite3ExprIdToTrueFalse(pExpr) != 0 {
                return crate::src::headers::sqliteInt_h::WRC_Prune;
            }
            current_block_47 = 8457315219000651999;
        }
        crate::src::parse::TK_COLUMN
        | crate::src::parse::TK_AGG_FUNCTION
        | crate::src::parse::TK_AGG_COLUMN => {
            current_block_47 = 8457315219000651999;
        }
        crate::src::parse::TK_IF_NULL_ROW
        | crate::src::parse::TK_REGISTER
        | crate::src::parse::TK_DOT_1
        | crate::src::parse::TK_RAISE => {
            current_block_47 = 4068382217303356765;
        }
        crate::src::parse::TK_VARIABLE => {
            if (*pWalker).eCode as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
                (*pExpr).op = crate::src::parse::TK_NULL as crate::src::ext::rtree::rtree::U8_0;
            } else if (*pWalker).eCode as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
                (*pWalker).eCode = 0 as crate::src::fts5::U16_0;
                return crate::src::headers::sqliteInt_h::WRC_Abort;
            }
            current_block_47 = 7245201122033322888;
        }
        _ => {
            current_block_47 = 7245201122033322888;
        }
    }
    match current_block_47 {
        8457315219000651999 => {
            let __pWalker_ref = unsafe { &*pWalker };
            if (*pExpr).flags & 0x20 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
                && __pWalker_ref.eCode as ::core::ffi::c_int != 2 as ::core::ffi::c_int
            {
                return crate::src::headers::sqliteInt_h::WRC_Continue;
            }
            if __pWalker_ref.eCode as ::core::ffi::c_int == 3 as ::core::ffi::c_int
                && (*pExpr).iTable == __pWalker_ref.u.iCur
            {
                return crate::src::headers::sqliteInt_h::WRC_Continue;
            }
        }
        7245201122033322888 => return crate::src::headers::sqliteInt_h::WRC_Continue,
        _ => {}
    }
    (*pWalker).eCode = 0 as crate::src::fts5::U16_0;
    crate::src::headers::sqliteInt_h::WRC_Abort
}

unsafe extern "C" fn exprIsConst(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Expr,
    initFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut w: crate::src::headers::sqliteInt_h::Walker =
        crate::src::headers::sqliteInt_h::Walker {
            pParse: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Parse>(),
            xExprCallback: None,
            xSelectCallback: None,
            xSelectCallback2: None,
            walkerDepth: 0,
            eCode: 0,
            mWFlags: 0,
            u: crate::src::headers::sqliteInt_h::__anon_union_16 {
                pNC: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::NameContext>(),
            },
        };
    w.eCode = initFlag as crate::src::fts5::U16_0;
    w.pParse = pParse;
    w.xExprCallback = Some(
        exprNodeIsConstant
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
    w.xSelectCallback = Some(
        sqlite3SelectWalkFail
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
    crate::src::src::walker::sqlite3WalkExpr(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        p as *mut crate::src::headers::sqliteInt_h::Expr,
    );
    w.eCode as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprIsConstant(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    exprIsConst(pParse, p, 1 as ::core::ffi::c_int)
}

unsafe extern "C" fn sqlite3ExprIsConstantNotJoin(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    exprIsConst(pParse, p, 2 as ::core::ffi::c_int)
}

unsafe extern "C" fn exprSelectWalkTableConstant(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pSelect: *mut crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    if (*pSelect).selFlags
        & crate::src::headers::sqliteInt_h::SF_Correlated as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        (*pWalker).eCode = 0 as crate::src::fts5::U16_0;
        return crate::src::headers::sqliteInt_h::WRC_Abort;
    }
    crate::src::headers::sqliteInt_h::WRC_Prune
}

unsafe extern "C" fn sqlite3ExprIsTableConstant(
    p: *mut crate::src::headers::sqliteInt_h::Expr,
    iCur: ::core::ffi::c_int,
    bAllowSubq: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut w: crate::src::headers::sqliteInt_h::Walker =
        crate::src::headers::sqliteInt_h::Walker {
            pParse: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Parse>(),
            xExprCallback: None,
            xSelectCallback: None,
            xSelectCallback2: None,
            walkerDepth: 0,
            eCode: 0,
            mWFlags: 0,
            u: crate::src::headers::sqliteInt_h::__anon_union_16 {
                pNC: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::NameContext>(),
            },
        };
    w.eCode = 3 as crate::src::fts5::U16_0;
    w.pParse = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Parse>();
    w.xExprCallback = Some(
        exprNodeIsConstant
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
    if bAllowSubq != 0 {
        w.xSelectCallback = Some(
            exprSelectWalkTableConstant
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
    } else {
        w.xSelectCallback = Some(
            sqlite3SelectWalkFail
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
    }
    w.u.iCur = iCur;
    crate::src::src::walker::sqlite3WalkExpr(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        p as *mut crate::src::headers::sqliteInt_h::Expr,
    );
    w.eCode as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprIsSingleTableConstraint(
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    pSrcList: *const crate::src::headers::sqliteInt_h::SrcList,
    iSrc: ::core::ffi::c_int,
    bAllowSubq: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pSrc: *const crate::src::headers::sqliteInt_h::SrcItem =
        (&raw const (*pSrcList).a as *const crate::src::headers::sqliteInt_h::SrcItem)
            .offset(iSrc as isize) as *const crate::src::headers::sqliteInt_h::SrcItem;
    let __pSrc_ref = unsafe { &*pSrc };
    if __pSrc_ref.fg.jointype as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::JT_LTORJ
        != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if __pSrc_ref.fg.jointype as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::JT_LEFT != 0
    {
        if ((*pExpr).flags & 0x1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*pExpr).w.iJoin != __pSrc_ref.iCursor {
            return 0 as ::core::ffi::c_int;
        }
    } else if (*pExpr).flags & 0x1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*pExpr).flags
        & (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
        && (*(&raw const (*pSrcList).a as *const crate::src::headers::sqliteInt_h::SrcItem)
            .offset(0_isize))
        .fg
        .jointype as ::core::ffi::c_int
            & crate::src::headers::sqliteInt_h::JT_LTORJ
            != 0 as ::core::ffi::c_int
    {
        let mut jj: ::core::ffi::c_int;
        jj = 0 as ::core::ffi::c_int;
        while jj < iSrc {
            if (*pExpr).w.iJoin
                == (*(&raw const (*pSrcList).a as *const crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(jj as isize))
                .iCursor
            {
                if (*(&raw const (*pSrcList).a as *const crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(jj as isize))
                .fg
                .jointype as ::core::ffi::c_int
                    & crate::src::headers::sqliteInt_h::JT_LTORJ
                    != 0 as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                }
                break;
            } else {
                jj += 1;
            }
        }
    }
    sqlite3ExprIsTableConstant(pExpr, __pSrc_ref.iCursor, bAllowSubq)
}

unsafe extern "C" fn exprNodeIsConstantOrGroupBy(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let pGroupBy: *mut crate::src::headers::sqliteInt_h::ExprList = (*pWalker).u.pGroupBy;
    let mut i: ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*pGroupBy).nExpr {
        let p: *mut crate::src::headers::sqliteInt_h::Expr = (*(&raw mut (*pGroupBy).a
            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(i as isize))
        .pExpr;
        if sqlite3ExprCompare(
            ::core::ptr::null::<crate::src::headers::sqliteInt_h::Parse>(),
            pExpr,
            p,
            -(1 as ::core::ffi::c_int),
        ) < 2 as ::core::ffi::c_int
        {
            let pColl: *mut crate::src::headers::sqliteInt_h::CollSeq =
                sqlite3ExprNNCollSeq((*pWalker).pParse, p);
            if crate::src::src::main::sqlite3IsBinary(
                pColl as *const crate::src::headers::sqliteInt_h::CollSeq,
            ) != 0
            {
                return crate::src::headers::sqliteInt_h::WRC_Prune;
            }
        }
        i += 1;
    }
    if (*pExpr).flags
        & crate::src::headers::sqliteInt_h::EP_xIsSelect as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        (*pWalker).eCode = 0 as crate::src::fts5::U16_0;
        return crate::src::headers::sqliteInt_h::WRC_Abort;
    }
    exprNodeIsConstant(pWalker, pExpr)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprIsConstantOrGroupBy(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Expr,
    pGroupBy: *mut crate::src::headers::sqliteInt_h::ExprList,
) -> ::core::ffi::c_int {
    let mut w: crate::src::headers::sqliteInt_h::Walker =
        crate::src::headers::sqliteInt_h::Walker {
            pParse: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Parse>(),
            xExprCallback: None,
            xSelectCallback: None,
            xSelectCallback2: None,
            walkerDepth: 0,
            eCode: 0,
            mWFlags: 0,
            u: crate::src::headers::sqliteInt_h::__anon_union_16 {
                pNC: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::NameContext>(),
            },
        };
    w.eCode = 1 as crate::src::fts5::U16_0;
    w.xExprCallback = Some(
        exprNodeIsConstantOrGroupBy
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
    w.xSelectCallback = None;
    w.u.pGroupBy = pGroupBy;
    w.pParse = pParse;
    crate::src::src::walker::sqlite3WalkExpr(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        p as *mut crate::src::headers::sqliteInt_h::Expr,
    );
    w.eCode as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprIsConstantOrFunction(
    p: *mut crate::src::headers::sqliteInt_h::Expr,
    isInit: crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    exprIsConst(
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Parse>(),
        p,
        4 as ::core::ffi::c_int + isInit as ::core::ffi::c_int,
    )
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprIsInteger(
    p: *const crate::src::headers::sqliteInt_h::Expr,
    pValue: *mut ::core::ffi::c_int,
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if p.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*p).flags
        & crate::src::headers::sqliteInt_h::EP_IntValue as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        *pValue = (*p).u.iValue;
        return 1 as ::core::ffi::c_int;
    }
    match (*p).op as ::core::ffi::c_int {
        crate::src::parse::TK_UPLUS => {
            rc = sqlite3ExprIsInteger(
                (*p).pLeft,
                pValue,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Parse>(),
            );
        }
        crate::src::parse::TK_UMINUS => {
            let mut v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if sqlite3ExprIsInteger(
                (*p).pLeft,
                &raw mut v,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Parse>(),
            ) != 0
            {
                *pValue = -v;
                rc = 1 as ::core::ffi::c_int;
            }
        }
        crate::src::parse::TK_VARIABLE => {
            let pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value;
            if !pParse.is_null()
                && !(*pParse).pVdbe.is_null()
                    && ((*(*pParse).db).flags
                        & crate::src::headers::sqliteInt_h::SQLITE_EnableQPSG
                            as crate::src::ext::rtree::rtree::U64_0 == 0 as crate::src::ext::rtree::rtree::U64_0)
                    {
                        crate::src::src::vdbeaux::sqlite3VdbeSetVarmask(
                            (*pParse).pVdbe,
                            (*p).iColumn as ::core::ffi::c_int,
                        );
                        pVal = crate::src::src::vdbeaux::sqlite3VdbeGetBoundValue(
                            (*pParse).pReprepare,
                            (*p).iColumn as ::core::ffi::c_int,
                            crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB
                                as crate::src::ext::rtree::rtree::U8_0,
                        );
                        if !pVal.is_null() {
                            if crate::src::src::vdbeapi::sqlite3_value_type(pVal)
                                == crate::src::headers::sqlite3_h::SQLITE_INTEGER
                            {
                                let vv: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                                    crate::src::src::vdbeapi::sqlite3_value_int64(pVal);
                                if vv
                                    == vv
                                        & 0x7fffffff
                                            as crate::src::headers::sqlite3_h::Sqlite3Int64
                                {
                                    *pValue = vv as ::core::ffi::c_int;
                                    rc = 1 as ::core::ffi::c_int;
                                }
                            }
                            crate::src::src::vdbemem::sqlite3ValueFree(pVal);
                        }
                    }
        }
        _ => {}
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCanBeNull(
    mut p: *const crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let mut op: crate::src::ext::rtree::rtree::U8_0;
    while (*p).op as ::core::ffi::c_int == crate::src::parse::TK_UPLUS
        || (*p).op as ::core::ffi::c_int == crate::src::parse::TK_UMINUS
    {
        p = (*p).pLeft;
    }
    op = (*p).op;
    if op as ::core::ffi::c_int == crate::src::parse::TK_REGISTER {
        op = (*p).op2;
    }
    match op as ::core::ffi::c_int {
        crate::src::parse::TK_INTEGER
        | crate::src::parse::TK_STRING
        | crate::src::parse::TK_FLOAT
        | crate::src::parse::TK_BLOB => 0 as ::core::ffi::c_int,
        crate::src::parse::TK_COLUMN => {
            ((*p).flags
                & 0x200000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
                || (*p).y.pTab.is_null()
                || (*p).iColumn as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                    && !(*(*p).y.pTab).aCol.is_null()
                    && ((*p).iColumn as ::core::ffi::c_int)
                        < (*(*p).y.pTab).nCol as ::core::ffi::c_int
                    && (*(*(*p).y.pTab).aCol.offset((*p).iColumn as isize)).notNull()
                        as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        }
        _ => 1 as ::core::ffi::c_int,
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprNeedsNoAffinityChange(
    mut p: *const crate::src::headers::sqliteInt_h::Expr,
    aff: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut op: crate::src::ext::rtree::rtree::U8_0;
    let mut unaryMinus: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if aff as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB {
        return 1 as ::core::ffi::c_int;
    }
    while (*p).op as ::core::ffi::c_int == crate::src::parse::TK_UPLUS
        || (*p).op as ::core::ffi::c_int == crate::src::parse::TK_UMINUS
    {
        if (*p).op as ::core::ffi::c_int == crate::src::parse::TK_UMINUS {
            unaryMinus = 1 as ::core::ffi::c_int;
        }
        p = (*p).pLeft;
    }
    op = (*p).op;
    if op as ::core::ffi::c_int == crate::src::parse::TK_REGISTER {
        op = (*p).op2;
    }
    match op as ::core::ffi::c_int {
        crate::src::parse::TK_INTEGER => {
            (aff as ::core::ffi::c_int
                >= crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC)
                as ::core::ffi::c_int
        }
        crate::src::parse::TK_FLOAT => {
            (aff as ::core::ffi::c_int
                >= crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC)
                as ::core::ffi::c_int
        }
        crate::src::parse::TK_STRING => {
            (unaryMinus == 0
                && aff as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT)
                as ::core::ffi::c_int
        }
        crate::src::parse::TK_BLOB => (unaryMinus == 0) as ::core::ffi::c_int,
        crate::src::parse::TK_COLUMN => {
            (aff as ::core::ffi::c_int
                >= crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC
                && ((*p).iColumn as ::core::ffi::c_int) < 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int
        }
        _ => 0 as ::core::ffi::c_int,
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3IsRowid(z: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if crate::src::src::util::sqlite3StrICmp(
        z,
        b"_ROWID_\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    if crate::src::src::util::sqlite3StrICmp(
        z,
        b"ROWID\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    if crate::src::src::util::sqlite3StrICmp(z, b"OID\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3RowidAlias(
    pTab: *mut crate::src::headers::sqliteInt_h::Table,
) -> *const ::core::ffi::c_char {
    let azOpt: [*const ::core::ffi::c_char; 3] = [
        b"_ROWID_\0" as *const u8 as *const ::core::ffi::c_char,
        b"ROWID\0" as *const u8 as *const ::core::ffi::c_char,
        b"OID\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut ii: ::core::ffi::c_int;
    ii = 0 as ::core::ffi::c_int;
    while ii
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
            as ::core::ffi::c_int
    {
        if crate::src::src::select::sqlite3ColumnIndex(
            pTab as *mut crate::src::headers::sqliteInt_h::Table,
            azOpt[ii as usize],
        ) < 0 as ::core::ffi::c_int
        {
            return azOpt[ii as usize];
        }
        ii += 1;
    }
    ::core::ptr::null::<::core::ffi::c_char>()
}

unsafe extern "C" fn isCandidateForInOpt(
    pX: *const crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::Select {
    
    
    
    
    let mut i: ::core::ffi::c_int;
    let __pX_ref = unsafe { &*pX };
    if (__pX_ref.flags
        & crate::src::headers::sqliteInt_h::EP_xIsSelect as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
    {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    }
    if __pX_ref.flags & 0x40 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    }
    let p: *mut crate::src::headers::sqliteInt_h::Select = __pX_ref.x.pSelect;
    if !(*p).pPrior.is_null() {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    }
    if (*p).selFlags
        & (crate::src::headers::sqliteInt_h::SF_Distinct
            | crate::src::headers::sqliteInt_h::SF_Aggregate)
            as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    }
    if !(*p).pLimit.is_null() {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    }
    if !(*p).pWhere.is_null() {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    }
    let pSrc: *mut crate::src::headers::sqliteInt_h::SrcList = (*p).pSrc;
    if (*pSrc).nSrc != 1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    }
    if (*(&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem).offset(0_isize))
        .fg
        .isSubquery()
        != 0
    {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    }
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = (*(&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
        .offset(0_isize))
    .pSTab;
    if (*pTab).eTabType as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::TABTYP_VTAB {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    }
    let pEList: *mut crate::src::headers::sqliteInt_h::ExprList = (*p).pEList;
    i = 0 as ::core::ffi::c_int;
    while i < (*pEList).nExpr {
        let pRes: *mut crate::src::headers::sqliteInt_h::Expr = (*(&raw mut (*pEList).a
            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(i as isize))
        .pExpr;
        if (*pRes).op as ::core::ffi::c_int != crate::src::parse::TK_COLUMN {
            return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
        }
        i += 1;
    }
    p
}

unsafe extern "C" fn sqlite3SetHasNullFlag(
    v: *mut crate::src::headers::vdbeInt_h::Vdbe,
    iCur: ::core::ffi::c_int,
    regHasNull: ::core::ffi::c_int,
) {
    
    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
        v,
        crate::src::headers::opcodes_h::OP_Integer,
        0 as ::core::ffi::c_int,
        regHasNull,
    );
    let addr1: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
        v,
        crate::src::headers::opcodes_h::OP_Rewind,
        iCur,
    );
    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
        v,
        crate::src::headers::opcodes_h::OP_Column,
        iCur,
        0 as ::core::ffi::c_int,
        regHasNull,
    );
    crate::src::src::vdbeaux::sqlite3VdbeChangeP5(
        v,
        crate::src::headers::sqliteInt_h::OPFLAG_TYPEOFARG as crate::src::fts5::U16_0,
    );
    crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addr1);
}

unsafe extern "C" fn sqlite3InRhsIsConstant(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pIn: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    
    
    let __pIn_ref = unsafe { &mut *pIn };
    let pLHS: *mut crate::src::headers::sqliteInt_h::Expr = __pIn_ref.pLeft;
    __pIn_ref.pLeft = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
    let res: ::core::ffi::c_int = sqlite3ExprIsConstant(pParse, pIn);
    __pIn_ref.pLeft = pLHS;
    res
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3FindInIndex(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pX: *mut crate::src::headers::sqliteInt_h::Expr,
    inFlags: crate::src::ext::rtree::rtree::U32_0,
    mut prRhsHasNull: *mut ::core::ffi::c_int,
    aiMap: *mut ::core::ffi::c_int,
    piTab: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let p: *mut crate::src::headers::sqliteInt_h::Select;
    let mut eType: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iTab: ::core::ffi::c_int;
    
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = crate::src::src::select::sqlite3GetVdbe(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
    );
    let mustBeUnique: ::core::ffi::c_int = (inFlags
        & crate::src::headers::sqliteInt_h::IN_INDEX_LOOP as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0) as ::core::ffi::c_int;
    let __pParse_ref = unsafe { &mut *pParse };
    let fresh8 = __pParse_ref.nTab;
    __pParse_ref.nTab += 1;
    iTab = fresh8;
    let __pX_ref = unsafe { &*pX };
    if !prRhsHasNull.is_null()
        && __pX_ref.flags
            & crate::src::headers::sqliteInt_h::EP_xIsSelect as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        let mut i: ::core::ffi::c_int;
        let pEList: *mut crate::src::headers::sqliteInt_h::ExprList =
            (*__pX_ref.x.pSelect).pEList;
        i = 0 as ::core::ffi::c_int;
        while i < (*pEList).nExpr {
            if sqlite3ExprCanBeNull(
                (*(&raw mut (*pEList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(i as isize))
                .pExpr,
            ) != 0
            {
                break;
            }
            i += 1;
        }
        if i == (*pEList).nExpr {
            prRhsHasNull = ::core::ptr::null_mut::<::core::ffi::c_int>();
        }
    }
    if __pParse_ref.nErr == 0 as ::core::ffi::c_int && {
        p = isCandidateForInOpt(pX);
        !p.is_null()
    } {
        let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
        
        
        let pEList_0: *mut crate::src::headers::sqliteInt_h::ExprList = (*p).pEList;
        let nExpr: ::core::ffi::c_int = (*pEList_0).nExpr;
        let pTab: *mut crate::src::headers::sqliteInt_h::Table = (*(&raw mut (*(*p).pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
            .offset(0_isize))
        .pSTab;
        let iDb: ::core::ffi::c_int = crate::src::src::prepare::sqlite3SchemaToIndex(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*pTab).pSchema as *mut crate::src::headers::sqliteInt_h::Schema,
        );
        crate::src::src::build::sqlite3CodeVerifySchema(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            iDb,
        );
        crate::src::src::build::sqlite3TableLock(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            iDb,
            (*pTab).tnum,
            0 as crate::src::ext::rtree::rtree::U8_0,
            (*pTab).zName,
        );
        if nExpr == 1 as ::core::ffi::c_int
            && ((*(*(&raw mut (*pEList_0).a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(0_isize))
            .pExpr)
                .iColumn as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
        {
            let iAddr: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeAddOp0(
                v,
                crate::src::headers::opcodes_h::OP_Once,
            );
            crate::src::src::insert::sqlite3OpenTable(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                iTab,
                iDb,
                pTab as *mut crate::src::headers::sqliteInt_h::Table,
                crate::src::headers::opcodes_h::OP_OpenRead,
            );
            eType = crate::src::headers::sqliteInt_h::IN_INDEX_ROWID;
            crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                0 as crate::src::ext::rtree::rtree::U8_0,
                b"USING ROWID SEARCH ON TABLE %s FOR IN-OPERATOR\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Str(
                    (*pTab).zName as *mut ::core::ffi::c_char,
                )],
            );
            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, iAddr);
        } else {
            let mut pIdx: *mut crate::src::headers::sqliteInt_h::Index;
            let mut affinity_ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut i_0: ::core::ffi::c_int;
            i_0 = 0 as ::core::ffi::c_int;
            while i_0 < nExpr && affinity_ok != 0 {
                let pLhs: *mut crate::src::headers::sqliteInt_h::Expr =
                    sqlite3VectorFieldSubexpr(__pX_ref.pLeft, i_0);
                let iCol: ::core::ffi::c_int = (*(*(&raw mut (*pEList_0).a
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(i_0 as isize))
                .pExpr)
                    .iColumn
                    as ::core::ffi::c_int;
                let idxaff: ::core::ffi::c_char = sqlite3TableColumnAffinity(pTab, iCol);
                let cmpaff: ::core::ffi::c_char = sqlite3CompareAffinity(pLhs, idxaff);
                match cmpaff as ::core::ffi::c_int {
                    crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB
                    | crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT => {}
                    _ => {
                        affinity_ok = (idxaff as ::core::ffi::c_int
                            >= crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC)
                            as ::core::ffi::c_int;
                    }
                }
                i_0 += 1;
            }
            if affinity_ok != 0 {
                let mut current_block_63: u64;
                pIdx = (*pTab).pIndex;
                while !pIdx.is_null() && eType == 0 as ::core::ffi::c_int {
                    let mut colUsed: crate::src::headers::sqliteInt_h::Bitmask;
                    let mut mCol: crate::src::headers::sqliteInt_h::Bitmask;
                    if (((*pIdx).nColumn as ::core::ffi::c_int) >= nExpr)
                        && (*pIdx).pPartIdxWhere.is_null()
                            && (((*pIdx).nColumn as ::core::ffi::c_int) < crate::src::headers::sqliteInt_h::BMS - 1 as ::core::ffi::c_int)
                            {
                                if mustBeUnique != 0 {
                                    let __pIdx_ref = unsafe { &*pIdx };
                                    if __pIdx_ref.nKeyCol as ::core::ffi::c_int > nExpr
                                        || __pIdx_ref.nColumn as ::core::ffi::c_int > nExpr
                                            && (__pIdx_ref.onError as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::OE_None)
                                    {
                                        current_block_63 = 7659304154607701039;
                                    } else {
                                        current_block_63 = 10399321362245223758;
                                    }
                                } else {
                                    current_block_63 = 10399321362245223758;
                                }
                                match current_block_63 {
                                    7659304154607701039 => {}
                                    _ => {
                                        colUsed = 0 as crate::src::headers::sqliteInt_h::Bitmask;
                                        i_0 = 0 as ::core::ffi::c_int;
                                        while i_0 < nExpr {
                                            let pLhs_0: *mut crate::src::headers::sqliteInt_h::Expr =
                                                sqlite3VectorFieldSubexpr(__pX_ref.pLeft, i_0);
                                            let pRhs: *mut crate::src::headers::sqliteInt_h::Expr = (*(&raw mut (*pEList_0).a
                                                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                .offset(i_0 as isize))
                                            .pExpr;
                                            let pReq: *mut crate::src::headers::sqliteInt_h::CollSeq =
                                                sqlite3BinaryCompareCollSeq(pParse, pLhs_0, pRhs);
                                            let mut j: ::core::ffi::c_int;
                                            j = 0 as ::core::ffi::c_int;
                                            while j < nExpr {
                                                if (*(*pIdx).aiColumn.offset(j as isize)
                                                    as ::core::ffi::c_int == (*pRhs).iColumn as ::core::ffi::c_int)
                                                    && !(!pReq.is_null()
                                                        && crate::src::src::util::sqlite3StrICmp(
                                                            (*pReq).zName,
                                                            *(*pIdx).azColl.offset(j as isize),
                                                        ) != 0 as ::core::ffi::c_int)
                                                    {
                                                        break;
                                                    }
                                                j += 1;
                                            }
                                            if j == nExpr {
                                                break;
                                            }
                                            mCol = (1 as ::core::ffi::c_int
                                                as crate::src::headers::sqliteInt_h::Bitmask)
                                                << j;
                                            if mCol & colUsed != 0 {
                                                break;
                                            }
                                            colUsed |= mCol;
                                            if !aiMap.is_null() {
                                                *aiMap.offset(i_0 as isize) = j;
                                            }
                                            i_0 += 1;
                                        }
                                        if colUsed
                                            == ((1 as ::core::ffi::c_int
                                                as crate::src::headers::sqliteInt_h::Bitmask)
                                                << nExpr)
                                                .wrapping_sub(
                                                    1 as crate::src::headers::sqliteInt_h::Bitmask,
                                                )
                                        {
                                            let iAddr_0: ::core::ffi::c_int =
                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp0(
                                                    v,
                                                    crate::src::headers::opcodes_h::OP_Once,
                                                );
                                            let __pIdx_ref = unsafe { &mut *pIdx };
                                            crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                                                pParse
                                                    as *mut crate::src::headers::sqliteInt_h::Parse,
                                                0 as crate::src::ext::rtree::rtree::U8_0,
                                                b"USING INDEX %s FOR IN-OPERATOR\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                &[crate::src::src::printf::PrintfArg::Str(
                                                    __pIdx_ref.zName as *mut ::core::ffi::c_char,
                                                )],
                                            );
                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                v,
                                                crate::src::headers::opcodes_h::OP_OpenRead,
                                                iTab,
                                                __pIdx_ref.tnum as ::core::ffi::c_int,
                                                iDb,
                                            );
                                            crate::src::src::vdbeaux::sqlite3VdbeSetP4KeyInfo(
                                                pParse
                                                    as *mut crate::src::headers::sqliteInt_h::Parse,
                                                pIdx
                                                    as *mut crate::src::headers::sqliteInt_h::Index,
                                            );
                                            eType =
                                                crate::src::headers::sqliteInt_h::IN_INDEX_INDEX_ASC
                                                    + *(*pIdx).aSortOrder.offset(0_isize)
                                                        as ::core::ffi::c_int;
                                            if !prRhsHasNull.is_null() {
                                                __pParse_ref.nMem += 1;
                                                *prRhsHasNull = __pParse_ref.nMem;
                                                if nExpr == 1 as ::core::ffi::c_int {
                                                    sqlite3SetHasNullFlag(v, iTab, *prRhsHasNull);
                                                }
                                            }
                                            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(
                                                v, iAddr_0,
                                            );
                                        }
                                    }
                                }
                            }
                    pIdx = (*pIdx).pNext;
                }
            }
        }
    }
    if eType == 0 as ::core::ffi::c_int
        && inFlags
            & crate::src::headers::sqliteInt_h::IN_INDEX_NOOP_OK
                as crate::src::ext::rtree::rtree::U32_0
            != 0
        && __pX_ref.flags
            & crate::src::headers::sqliteInt_h::EP_xIsSelect as crate::src::ext::rtree::rtree::U32_0
            == 0 as crate::src::ext::rtree::rtree::U32_0
        && (sqlite3InRhsIsConstant(pParse, pX) == 0
            || (*__pX_ref.x.pList).nExpr <= 2 as ::core::ffi::c_int)
    {
        __pParse_ref.nTab -= 1;
        iTab = -(1 as ::core::ffi::c_int);
        eType = crate::src::headers::sqliteInt_h::IN_INDEX_NOOP;
    }
    if eType == 0 as ::core::ffi::c_int {
        let savedNQueryLoop: crate::src::ext::rtree::rtree::U32_0 =
            __pParse_ref.nQueryLoop as crate::src::ext::rtree::rtree::U32_0;
        let mut rMayHaveNull: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        eType = crate::src::headers::sqliteInt_h::IN_INDEX_EPH;
        if inFlags
            & crate::src::headers::sqliteInt_h::IN_INDEX_LOOP
                as crate::src::ext::rtree::rtree::U32_0
            != 0
        {
            __pParse_ref.nQueryLoop = 0 as crate::src::headers::sqliteInt_h::LogEst;
        } else if !prRhsHasNull.is_null() {
            __pParse_ref.nMem += 1;
            rMayHaveNull = __pParse_ref.nMem;
            *prRhsHasNull = rMayHaveNull;
        }
        sqlite3CodeRhsOfIN(pParse, pX, iTab);
        if rMayHaveNull != 0 {
            sqlite3SetHasNullFlag(v, iTab, rMayHaveNull);
        }
        __pParse_ref.nQueryLoop = savedNQueryLoop as crate::src::headers::sqliteInt_h::LogEst;
    }
    if !aiMap.is_null()
        && eType != crate::src::headers::sqliteInt_h::IN_INDEX_INDEX_ASC
        && eType != crate::src::headers::sqliteInt_h::IN_INDEX_INDEX_DESC
    {
        let mut i_1: ::core::ffi::c_int;
        
        let n: ::core::ffi::c_int = sqlite3ExprVectorSize(__pX_ref.pLeft);
        i_1 = 0 as ::core::ffi::c_int;
        while i_1 < n {
            *aiMap.offset(i_1 as isize) = i_1;
            i_1 += 1;
        }
    }
    *piTab = iTab;
    eType
}

unsafe extern "C" fn exprINAffinity(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *const crate::src::headers::sqliteInt_h::Expr,
) -> *mut ::core::ffi::c_char {
    let pLeft: *mut crate::src::headers::sqliteInt_h::Expr = (*pExpr).pLeft;
    let nVal: ::core::ffi::c_int = sqlite3ExprVectorSize(pLeft);
    let pSelect: *mut crate::src::headers::sqliteInt_h::Select = if (*pExpr).flags
        & crate::src::headers::sqliteInt_h::EP_xIsSelect as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        (*pExpr).x.pSelect
    } else {
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>()
    };
    
    let zRet: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3DbMallocRaw(
        (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        (1 as crate::src::ext::rtree::rtree::I64_0 + nVal as crate::src::ext::rtree::rtree::I64_0)
            as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut ::core::ffi::c_char;
    if !zRet.is_null() {
        let mut i: ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < nVal {
            let pA: *mut crate::src::headers::sqliteInt_h::Expr =
                sqlite3VectorFieldSubexpr(pLeft, i);
            let a: ::core::ffi::c_char = sqlite3ExprAffinity(pA);
            if !pSelect.is_null() {
                *zRet.offset(i as isize) = sqlite3CompareAffinity(
                    (*(&raw mut (*(*pSelect).pEList).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(i as isize))
                    .pExpr,
                    a,
                );
            } else {
                *zRet.offset(i as isize) = a;
            }
            i += 1;
        }
        *zRet.offset(nVal as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    zRet
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SubselectError(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    nActual: ::core::ffi::c_int,
    nExpect: ::core::ffi::c_int,
) {
    if (*pParse).nErr == 0 as ::core::ffi::c_int {
        let zFmt: *const ::core::ffi::c_char = b"sub-select returns %d columns - expected %d\0"
            as *const u8
            as *const ::core::ffi::c_char;
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            zFmt,
            &[
                crate::src::src::printf::PrintfArg::Int(
                    nActual as crate::src::ext::rtree::rtree::I64_0,
                ),
                crate::src::src::printf::PrintfArg::Int(
                    nExpect as crate::src::ext::rtree::rtree::I64_0,
                ),
            ],
        );
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3VectorErrorMsg(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) {
    if (*pExpr).flags
        & crate::src::headers::sqliteInt_h::EP_xIsSelect as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        sqlite3SubselectError(
            pParse,
            (*(*(*pExpr).x.pSelect).pEList).nExpr,
            1 as ::core::ffi::c_int,
        );
    } else {
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"row value misused\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    };
}

unsafe extern "C" fn findCompatibleInRhsSubrtn(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    pNewSig: *mut crate::src::src::vdbe::SubrtnSig,
) -> ::core::ffi::c_int {
    let mut pOp: *mut crate::src::src::vdbe::VdbeOp;
    
    let mut pSig: *mut crate::src::src::vdbe::SubrtnSig;
    
    if pNewSig.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*pParse).mSubrtnSig as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << ((*pNewSig).selId & 7 as ::core::ffi::c_int)
        == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    pOp = crate::src::src::vdbeaux::sqlite3VdbeGetOp(v, 1 as ::core::ffi::c_int)
        as *mut crate::src::src::vdbe::VdbeOp;
    let pEnd: *mut crate::src::src::vdbe::VdbeOp = crate::src::src::vdbeaux::sqlite3VdbeGetLastOp(v) as *mut crate::src::src::vdbe::VdbeOp;
    while pOp < pEnd {
        if ((*pOp).p4type as ::core::ffi::c_int == crate::src::src::vdbe::P4_SUBRTNSIG) {
            pSig = (*pOp).p4.pSubrtnSig;
            if ((*pSig).bComplete != 0)
                && ((*pNewSig).selId == (*pSig).selId)
                    && (::libc::strcmp((*pNewSig).zAff, (*pSig).zAff) == 0 as ::core::ffi::c_int) {
                        let __pSig_ref = unsafe { &*pSig };
                        let __pExpr_ref = unsafe { &mut *pExpr };
                        __pExpr_ref.y.sub.iAddr = __pSig_ref.iAddr;
                        __pExpr_ref.y.sub.regReturn = __pSig_ref.regReturn;
                        __pExpr_ref.iTable = __pSig_ref.iTable;
                        __pExpr_ref.flags |=
                            0x2000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
                        return 1 as ::core::ffi::c_int;
                    }
        }
        pOp = pOp.offset(1);
    }
    0 as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3CodeRhsOfIN(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    iTab: ::core::ffi::c_int,
) {
    let mut addrOnce: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    
    
    
    let mut pSig: *mut crate::src::src::vdbe::SubrtnSig =
        ::core::ptr::null_mut::<crate::src::src::vdbe::SubrtnSig>();
    let __pParse_ref = unsafe { &mut *pParse };
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = __pParse_ref.pVdbe;
    let __pExpr_ref = unsafe { &mut *pExpr };
    if (__pExpr_ref.flags & 0x40 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
        && __pParse_ref.iSelfTab == 0 as ::core::ffi::c_int
    {
        if __pExpr_ref.flags
            & crate::src::headers::sqliteInt_h::EP_xIsSelect as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
            && (*__pExpr_ref.x.pSelect).selFlags
                & crate::src::headers::sqliteInt_h::SF_All as crate::src::ext::rtree::rtree::U32_0
                == 0 as crate::src::ext::rtree::rtree::U32_0
        {
            pSig = crate::src::src::malloc::sqlite3DbMallocRawNN(
                __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                ::core::mem::size_of::<crate::src::src::vdbe::SubrtnSig>()
                    as crate::src::ext::rtree::rtree::U64_0,
            ) as *mut crate::src::src::vdbe::SubrtnSig;
            if !pSig.is_null() {
                (*pSig).selId = (*__pExpr_ref.x.pSelect).selId as ::core::ffi::c_int;
                (*pSig).zAff = exprINAffinity(pParse, pExpr);
            }
        }
        if __pExpr_ref.flags
            & 0x2000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
            || findCompatibleInRhsSubrtn(pParse, pExpr, pSig) != 0
        {
            addrOnce = crate::src::src::vdbeaux::sqlite3VdbeAddOp0(
                v,
                crate::src::headers::opcodes_h::OP_Once,
            );
            if __pExpr_ref.flags
                & crate::src::headers::sqliteInt_h::EP_xIsSelect
                    as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
            {
                crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    0 as crate::src::ext::rtree::rtree::U8_0,
                    b"REUSE LIST SUBQUERY %d\0" as *const u8 as *const ::core::ffi::c_char,
                    &[crate::src::src::printf::PrintfArg::Int(
                        (*__pExpr_ref.x.pSelect).selId as crate::src::ext::rtree::rtree::I64_0,
                    )],
                );
            }
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_Gosub,
                __pExpr_ref.y.sub.regReturn,
                __pExpr_ref.y.sub.iAddr,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_OpenDup,
                iTab,
                __pExpr_ref.iTable,
            );
            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrOnce);
            if !pSig.is_null() {
                crate::src::src::malloc::sqlite3DbFree(
                    __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    (*pSig).zAff as *mut ::core::ffi::c_void,
                );
                crate::src::src::malloc::sqlite3DbFree(
                    __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    pSig as *mut ::core::ffi::c_void,
                );
            }
            return;
        }
        __pExpr_ref.flags |=
            0x2000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
        __pParse_ref.nMem += 1;
        __pExpr_ref.y.sub.regReturn = __pParse_ref.nMem;
        __pExpr_ref.y.sub.iAddr = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_BeginSubrtn,
            0 as ::core::ffi::c_int,
            __pExpr_ref.y.sub.regReturn,
        ) + 1 as ::core::ffi::c_int;
        if !pSig.is_null() {
            let __pSig_ref = unsafe { &mut *pSig };
            __pSig_ref.bComplete = 0 as crate::src::ext::rtree::rtree::U8_0;
            __pSig_ref.iAddr = __pExpr_ref.y.sub.iAddr;
            __pSig_ref.regReturn = __pExpr_ref.y.sub.regReturn;
            __pSig_ref.iTable = iTab;
            __pParse_ref.mSubrtnSig = ((1 as ::core::ffi::c_int)
                << (__pSig_ref.selId & 7 as ::core::ffi::c_int))
                as crate::src::ext::rtree::rtree::U8_0;
            crate::src::src::vdbeaux::sqlite3VdbeChangeP4(
                v,
                -(1 as ::core::ffi::c_int),
                pSig as *const ::core::ffi::c_char,
                crate::src::src::vdbe::P4_SUBRTNSIG,
            );
        }
        addrOnce =
            crate::src::src::vdbeaux::sqlite3VdbeAddOp0(v, crate::src::headers::opcodes_h::OP_Once);
    }
    let pLeft: *mut crate::src::headers::sqliteInt_h::Expr = __pExpr_ref.pLeft;
    let nVal: ::core::ffi::c_int = sqlite3ExprVectorSize(pLeft);
    __pExpr_ref.iTable = iTab;
    let addr: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
        v,
        crate::src::headers::opcodes_h::OP_OpenEphemeral,
        __pExpr_ref.iTable,
        nVal,
    );
    let pKeyInfo: *mut crate::src::headers::sqliteInt_h::KeyInfo = crate::src::src::select::sqlite3KeyInfoAlloc(
        __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        nVal,
        1 as ::core::ffi::c_int,
    ) as *mut crate::src::headers::sqliteInt_h::KeyInfo;
    if __pExpr_ref.flags
        & crate::src::headers::sqliteInt_h::EP_xIsSelect as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        let pSelect: *mut crate::src::headers::sqliteInt_h::Select = __pExpr_ref.x.pSelect;
        let pEList: *mut crate::src::headers::sqliteInt_h::ExprList = (*pSelect).pEList;
        crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            1 as crate::src::ext::rtree::rtree::U8_0,
            b"%sLIST SUBQUERY %d\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::src::src::printf::PrintfArg::Str(if addrOnce != 0 {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"CORRELATED \0" as *const u8 as *const ::core::ffi::c_char
                }
                    as *mut ::core::ffi::c_char),
                crate::src::src::printf::PrintfArg::Int(
                    (*pSelect).selId as crate::src::ext::rtree::rtree::I64_0,
                ),
            ],
        );
        if (*pEList).nExpr == nVal {
            
            let mut dest: crate::src::headers::sqliteInt_h::SelectDest =
                unsafe { ::core::mem::zeroed() };
            let mut i: ::core::ffi::c_int;
            
            let mut addrBloom: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            crate::src::src::select::sqlite3SelectDestInit(
                &raw mut dest as *mut _ as *mut crate::src::headers::sqliteInt_h::SelectDest,
                crate::src::headers::sqliteInt_h::SRT_Set,
                iTab,
            );
            dest.zAffSdst = exprINAffinity(pParse, pExpr);
            (*pSelect).iLimit = 0 as ::core::ffi::c_int;
            if addrOnce != 0
                && (*__pParse_ref.db).dbOptFlags & 0x80000 as crate::src::ext::rtree::rtree::U32_0
                    == 0 as crate::src::ext::rtree::rtree::U32_0
            {
                __pParse_ref.nMem += 1;
                let regBloom: ::core::ffi::c_int = __pParse_ref.nMem;
                addrBloom = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Blob,
                    10000 as ::core::ffi::c_int,
                    regBloom,
                );
                dest.iSDParm2 = regBloom;
            }
            let pCopy: *mut crate::src::headers::sqliteInt_h::Select = sqlite3SelectDup(__pParse_ref.db, pSelect, 0 as ::core::ffi::c_int);
            let rc: ::core::ffi::c_int = if (*__pParse_ref.db).mallocFailed as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                crate::src::src::select::sqlite3Select(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    pCopy as *mut crate::src::headers::sqliteInt_h::Select,
                    &raw mut dest as *mut _ as *mut crate::src::headers::sqliteInt_h::SelectDest,
                )
            };
            crate::src::src::select::sqlite3SelectDelete(
                __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                pCopy as *mut crate::src::headers::sqliteInt_h::Select,
            );
            crate::src::src::malloc::sqlite3DbFree(
                __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                dest.zAffSdst as *mut ::core::ffi::c_void,
            );
            if addrBloom != 0 {
                (*(crate::src::src::vdbeaux::sqlite3VdbeGetOp(v, addrOnce)
                    as *mut crate::src::src::vdbe::VdbeOp))
                    .p3 = dest.iSDParm2;
                if dest.iSDParm2 == 0 as ::core::ffi::c_int {
                    (*(crate::src::src::vdbeaux::sqlite3VdbeGetOp(v, addrBloom)
                        as *mut crate::src::src::vdbe::VdbeOp))
                        .p1 = 10 as ::core::ffi::c_int;
                }
            }
            if rc != 0 {
                crate::src::src::select::sqlite3KeyInfoUnref(
                    pKeyInfo as *mut crate::src::headers::sqliteInt_h::KeyInfo,
                );
                return;
            }
            i = 0 as ::core::ffi::c_int;
            while i < nVal {
                let p: *mut crate::src::headers::sqliteInt_h::Expr =
                    sqlite3VectorFieldSubexpr(pLeft, i);
                let fresh9 = &mut *(&raw mut (*pKeyInfo).aColl
                    as *mut *mut crate::src::headers::sqliteInt_h::CollSeq)
                    .offset(i as isize);
                *fresh9 = sqlite3BinaryCompareCollSeq(
                    pParse,
                    p,
                    (*(&raw mut (*pEList).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(i as isize))
                    .pExpr,
                );
                i += 1;
            }
        }
    } else if !__pExpr_ref.x.pList.is_null() {
        let mut affinity: ::core::ffi::c_char;
        let mut i_0: ::core::ffi::c_int;
        let pList: *mut crate::src::headers::sqliteInt_h::ExprList = __pExpr_ref.x.pList;
        let mut pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item;
        
        
        affinity = sqlite3ExprAffinity(pLeft);
        if affinity as ::core::ffi::c_int <= crate::src::headers::sqliteInt_h::SQLITE_AFF_NONE {
            affinity = crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB as ::core::ffi::c_char;
        } else if affinity as ::core::ffi::c_int
            == crate::src::headers::sqliteInt_h::SQLITE_AFF_REAL
        {
            affinity = crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC as ::core::ffi::c_char;
        }
        if !pKeyInfo.is_null() {
            let fresh10 = &mut *(&raw mut (*pKeyInfo).aColl
                as *mut *mut crate::src::headers::sqliteInt_h::CollSeq)
                .offset(0_isize);
            *fresh10 = sqlite3ExprCollSeq(pParse, __pExpr_ref.pLeft);
        }
        let r1: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
        let r2: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
        i_0 = (*pList).nExpr;
        pItem = &raw mut (*pList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item
            as *mut crate::src::headers::sqliteInt_h::ExprList_item;
        while i_0 > 0 as ::core::ffi::c_int {
            let pE2: *mut crate::src::headers::sqliteInt_h::Expr = (*pItem).pExpr;
            if addrOnce != 0 && sqlite3ExprIsConstant(pParse, pE2) == 0 {
                crate::src::src::vdbeaux::sqlite3VdbeChangeToNoop(
                    v,
                    addrOnce - 1 as ::core::ffi::c_int,
                );
                crate::src::src::vdbeaux::sqlite3VdbeChangeToNoop(v, addrOnce);
                __pExpr_ref.flags &=
                    !(0x2000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0);
                addrOnce = 0 as ::core::ffi::c_int;
            }
            sqlite3ExprCode(pParse, pE2, r1);
            crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                v,
                crate::src::headers::opcodes_h::OP_MakeRecord,
                r1,
                1 as ::core::ffi::c_int,
                r2,
                &raw mut affinity,
                1 as ::core::ffi::c_int,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                v,
                crate::src::headers::opcodes_h::OP_IdxInsert,
                iTab,
                r2,
                r1,
                1 as ::core::ffi::c_int,
            );
            i_0 -= 1;
            pItem = pItem.offset(1);
        }
        sqlite3ReleaseTempReg(pParse, r1);
        sqlite3ReleaseTempReg(pParse, r2);
    }
    if !pSig.is_null() {
        (*pSig).bComplete = 1 as crate::src::ext::rtree::rtree::U8_0;
    }
    if !pKeyInfo.is_null() {
        crate::src::src::vdbeaux::sqlite3VdbeChangeP4(
            v,
            addr,
            pKeyInfo as *mut ::core::ffi::c_void as *const ::core::ffi::c_char,
            crate::src::src::vdbe::P4_KEYINFO,
        );
    }
    if addrOnce != 0 {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
            v,
            crate::src::headers::opcodes_h::OP_NullRow,
            iTab,
        );
        crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrOnce);
        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
            v,
            crate::src::headers::opcodes_h::OP_Return,
            __pExpr_ref.y.sub.regReturn,
            __pExpr_ref.y.sub.iAddr,
            1 as ::core::ffi::c_int,
        );
        sqlite3ClearTempRegCache(pParse);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3CodeSubselect(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let mut addrOnce: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    let mut dest: crate::src::headers::sqliteInt_h::SelectDest = unsafe { ::core::mem::zeroed() };
    
    let mut pLimit: *mut crate::src::headers::sqliteInt_h::Expr;
    let __pParse_ref = unsafe { &mut *pParse };
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = __pParse_ref.pVdbe;
    if __pParse_ref.nErr != 0 {
        return 0 as ::core::ffi::c_int;
    }
    let __pExpr_ref = unsafe { &mut *pExpr };
    let pSel: *mut crate::src::headers::sqliteInt_h::Select = __pExpr_ref.x.pSelect;
    if __pExpr_ref.flags & 0x2000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            0 as crate::src::ext::rtree::rtree::U8_0,
            b"REUSE SUBQUERY %d\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Int(
                (*pSel).selId as crate::src::ext::rtree::rtree::I64_0,
            )],
        );
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Gosub,
            __pExpr_ref.y.sub.regReturn,
            __pExpr_ref.y.sub.iAddr,
        );
        return __pExpr_ref.iTable;
    }
    __pExpr_ref.flags |= 0x2000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
    __pParse_ref.nMem += 1;
    __pExpr_ref.y.sub.regReturn = __pParse_ref.nMem;
    __pExpr_ref.y.sub.iAddr = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
        v,
        crate::src::headers::opcodes_h::OP_BeginSubrtn,
        0 as ::core::ffi::c_int,
        __pExpr_ref.y.sub.regReturn,
    ) + 1 as ::core::ffi::c_int;
    if (__pExpr_ref.flags & 0x40 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
    {
        addrOnce =
            crate::src::src::vdbeaux::sqlite3VdbeAddOp0(v, crate::src::headers::opcodes_h::OP_Once);
    }
    crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        1 as crate::src::ext::rtree::rtree::U8_0,
        b"%sSCALAR SUBQUERY %d\0" as *const u8 as *const ::core::ffi::c_char,
        &[
            crate::src::src::printf::PrintfArg::Str(if addrOnce != 0 {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"CORRELATED \0" as *const u8 as *const ::core::ffi::c_char
            } as *mut ::core::ffi::c_char),
            crate::src::src::printf::PrintfArg::Int(
                (*pSel).selId as crate::src::ext::rtree::rtree::I64_0,
            ),
        ],
    );
    let nReg: ::core::ffi::c_int = if __pExpr_ref.op as ::core::ffi::c_int == crate::src::parse::TK_SELECT {
        (*(*pSel).pEList).nExpr
    } else {
        1 as ::core::ffi::c_int
    };
    crate::src::src::select::sqlite3SelectDestInit(
        &raw mut dest as *mut _ as *mut crate::src::headers::sqliteInt_h::SelectDest,
        0 as ::core::ffi::c_int,
        __pParse_ref.nMem + 1 as ::core::ffi::c_int,
    );
    __pParse_ref.nMem += nReg;
    if __pExpr_ref.op as ::core::ffi::c_int == crate::src::parse::TK_SELECT {
        dest.eDest =
            crate::src::headers::sqliteInt_h::SRT_Mem as crate::src::ext::rtree::rtree::U8_0;
        let __pSel_ref = unsafe { &mut *pSel };
        if __pSel_ref.selFlags
            & crate::src::headers::sqliteInt_h::SF_Distinct as crate::src::ext::rtree::rtree::U32_0
            != 0
            && !__pSel_ref.pLimit.is_null()
            && !(*__pSel_ref.pLimit).pRight.is_null()
        {
            dest.iSdst = __pParse_ref.nMem + 1 as ::core::ffi::c_int;
            __pParse_ref.nMem += nReg;
        } else {
            dest.iSdst = dest.iSDParm;
        }
        dest.nSdst = nReg;
        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
            v,
            crate::src::headers::opcodes_h::OP_Null,
            0 as ::core::ffi::c_int,
            dest.iSDParm,
            __pParse_ref.nMem,
        );
    } else {
        dest.eDest =
            crate::src::headers::sqliteInt_h::SRT_Exists as crate::src::ext::rtree::rtree::U8_0;
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Integer,
            0 as ::core::ffi::c_int,
            dest.iSDParm,
        );
    }
    if !(*pSel).pLimit.is_null() {
        let pLeft: *mut crate::src::headers::sqliteInt_h::Expr = (*(*pSel).pLimit).pLeft;
        let __pLeft_ref = unsafe { &*pLeft };
        if (__pLeft_ref.flags & 0x800 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || __pLeft_ref.u.iValue != 1 as ::core::ffi::c_int
                && __pLeft_ref.u.iValue != 0 as ::core::ffi::c_int
        {
            let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
            pLimit = sqlite3Expr(
                db,
                crate::src::parse::TK_INTEGER,
                b"0\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !pLimit.is_null() {
                (*pLimit).affExpr =
                    crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC as ::core::ffi::c_char;
                pLimit = sqlite3PExpr(
                    pParse,
                    crate::src::parse::TK_NE,
                    sqlite3ExprDup(db, pLeft, 0 as ::core::ffi::c_int),
                    pLimit,
                );
            }
            sqlite3ExprDeferredDelete(pParse, pLeft);
            (*(*pSel).pLimit).pLeft = pLimit;
        }
    } else {
        pLimit = sqlite3Expr(
            __pParse_ref.db,
            crate::src::parse::TK_INTEGER,
            b"1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*pSel).pLimit = sqlite3PExpr(
            pParse,
            crate::src::parse::TK_LIMIT,
            pLimit,
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>(),
        );
    }
    (*pSel).iLimit = 0 as ::core::ffi::c_int;
    if crate::src::src::select::sqlite3Select(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        pSel as *mut crate::src::headers::sqliteInt_h::Select,
        &raw mut dest as *mut _ as *mut crate::src::headers::sqliteInt_h::SelectDest,
    ) != 0
    {
        __pExpr_ref.op2 = __pExpr_ref.op;
        __pExpr_ref.op = crate::src::parse::TK_ERROR as crate::src::ext::rtree::rtree::U8_0;
        return 0 as ::core::ffi::c_int;
    }
    let rReg: ::core::ffi::c_int = dest.iSDParm;
    __pExpr_ref.iTable = rReg;
    if addrOnce != 0 {
        crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrOnce);
    }
    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
        v,
        crate::src::headers::opcodes_h::OP_Return,
        __pExpr_ref.y.sub.regReturn,
        __pExpr_ref.y.sub.iAddr,
        1 as ::core::ffi::c_int,
    );
    sqlite3ClearTempRegCache(pParse);
    rReg
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCheckIN(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pIn: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let nVector: ::core::ffi::c_int = sqlite3ExprVectorSize((*pIn).pLeft);
    if (*pIn).flags
        & crate::src::headers::sqliteInt_h::EP_xIsSelect as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
        && (*(*pParse).db).mallocFailed == 0
    {
        if nVector != (*(*(*pIn).x.pSelect).pEList).nExpr {
            sqlite3SubselectError(pParse, (*(*(*pIn).x.pSelect).pEList).nExpr, nVector);
            return 1 as ::core::ffi::c_int;
        }
    } else if nVector != 1 as ::core::ffi::c_int {
        sqlite3VectorErrorMsg(pParse, (*pIn).pLeft);
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn sqlite3ExprCodeIN(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    destIfFalse: ::core::ffi::c_int,
    destIfNull: ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut rRhsHasNull: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let eType: ::core::ffi::c_int;
    let mut rLhs: ::core::ffi::c_int;
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe;
    
    
    
    let mut iDummy: ::core::ffi::c_int = 0;
    
    let mut i: ::core::ffi::c_int;
    let destStep2: ::core::ffi::c_int;
    let mut destStep6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut addrTruthOp: ::core::ffi::c_int = 0;
    let destNotNull: ::core::ffi::c_int;
    let addrTop: ::core::ffi::c_int;
    let mut iTab: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let __pParse_ref = unsafe { &mut *pParse };
    let okConstFactor: crate::src::ext::rtree::rtree::U8_0 =
        __pParse_ref.okConstFactor() as crate::src::ext::rtree::rtree::U8_0;
    let pLeft: *mut crate::src::headers::sqliteInt_h::Expr = (*pExpr).pLeft;
    if sqlite3ExprCheckIN(pParse, pExpr) != 0 {
        return;
    }
    let zAff: *mut ::core::ffi::c_char = exprINAffinity(pParse, pExpr);
    let nVector: ::core::ffi::c_int = sqlite3ExprVectorSize((*pExpr).pLeft);
    let aiMap: *mut ::core::ffi::c_int = crate::src::src::malloc::sqlite3DbMallocZero(
        __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        (nVector as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
            as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut ::core::ffi::c_int;
    if ((*__pParse_ref.db).mallocFailed == 0) {
        v = __pParse_ref.pVdbe;
        eType = sqlite3FindInIndex(
            pParse,
            pExpr,
            (crate::src::headers::sqliteInt_h::IN_INDEX_MEMBERSHIP
                | crate::src::headers::sqliteInt_h::IN_INDEX_NOOP_OK)
                as crate::src::ext::rtree::rtree::U32_0,
            if destIfFalse == destIfNull {
                ::core::ptr::null_mut::<::core::ffi::c_int>()
            } else {
                &raw mut rRhsHasNull
            },
            aiMap,
            &raw mut iTab,
        );
        __pParse_ref.set_okConstFactor(
            0 as crate::src::headers::sqliteInt_h::Bft as crate::src::headers::sqliteInt_h::Bft,
        );
        rLhs = exprCodeVector(pParse, pLeft, &raw mut iDummy);
        __pParse_ref.set_okConstFactor(
            okConstFactor as crate::src::headers::sqliteInt_h::Bft
                as crate::src::headers::sqliteInt_h::Bft,
        );
        if eType == crate::src::headers::sqliteInt_h::IN_INDEX_NOOP {
            
            
            let labelOk: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            );
            let mut r2: ::core::ffi::c_int;
            let mut regToFree: ::core::ffi::c_int = 0;
            let mut regCkNull: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut ii: ::core::ffi::c_int;
            let pList: *mut crate::src::headers::sqliteInt_h::ExprList = (*pExpr).x.pList;
            let pColl: *mut crate::src::headers::sqliteInt_h::CollSeq = sqlite3ExprCollSeq(pParse, (*pExpr).pLeft);
            if destIfNull != destIfFalse {
                regCkNull = sqlite3GetTempReg(pParse);
                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                    v,
                    crate::src::headers::opcodes_h::OP_BitAnd,
                    rLhs,
                    rLhs,
                    regCkNull,
                );
            }
            ii = 0 as ::core::ffi::c_int;
            while ii < (*pList).nExpr {
                let __pList_ref = unsafe { &mut *pList };
                r2 = sqlite3ExprCodeTemp(
                    pParse,
                    (*(&raw mut __pList_ref.a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(ii as isize))
                    .pExpr,
                    &raw mut regToFree,
                );
                if regCkNull != 0
                    && sqlite3ExprCanBeNull(
                        (*(&raw mut __pList_ref.a
                            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                            .offset(ii as isize))
                        .pExpr,
                    ) != 0
                {
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                        v,
                        crate::src::headers::opcodes_h::OP_BitAnd,
                        regCkNull,
                        r2,
                        regCkNull,
                    );
                }
                sqlite3ReleaseTempReg(pParse, regToFree);
                if ii < __pList_ref.nExpr - 1 as ::core::ffi::c_int || destIfNull != destIfFalse {
                    let op: ::core::ffi::c_int = if rLhs != r2 {
                        crate::src::headers::opcodes_h::OP_Eq
                    } else {
                        crate::src::headers::opcodes_h::OP_NotNull
                    };
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                        v,
                        op,
                        rLhs,
                        labelOk,
                        r2,
                        pColl as *mut ::core::ffi::c_void as *const ::core::ffi::c_char,
                        crate::src::src::vdbe::P4_COLLSEQ,
                    );
                    crate::src::src::vdbeaux::sqlite3VdbeChangeP5(
                        v,
                        *zAff.offset(0_isize) as crate::src::fts5::U16_0,
                    );
                } else {
                    let op_0: ::core::ffi::c_int = if rLhs != r2 {
                        crate::src::headers::opcodes_h::OP_Ne
                    } else {
                        crate::src::headers::opcodes_h::OP_IsNull
                    };
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                        v,
                        op_0,
                        rLhs,
                        destIfFalse,
                        r2,
                        pColl as *mut ::core::ffi::c_void as *const ::core::ffi::c_char,
                        crate::src::src::vdbe::P4_COLLSEQ,
                    );
                    crate::src::src::vdbeaux::sqlite3VdbeChangeP5(
                        v,
                        (*zAff.offset(0_isize) as ::core::ffi::c_int
                            | crate::src::headers::sqliteInt_h::SQLITE_JUMPIFNULL)
                            as crate::src::fts5::U16_0,
                    );
                }
                ii += 1;
            }
            if regCkNull != 0 {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_IsNull,
                    regCkNull,
                    destIfNull,
                );
                crate::src::src::vdbeaux::sqlite3VdbeGoto(v, destIfFalse);
            }
            crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, labelOk);
            sqlite3ReleaseTempReg(pParse, regCkNull);
        } else {
            if eType != crate::src::headers::sqliteInt_h::IN_INDEX_ROWID {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                    v,
                    crate::src::headers::opcodes_h::OP_Affinity,
                    rLhs,
                    nVector,
                    0 as ::core::ffi::c_int,
                    zAff,
                    nVector,
                );
                i = 0 as ::core::ffi::c_int;
                while i < nVector && *aiMap.offset(i as isize) == i {
                    i += 1;
                }
                if i != nVector {
                    let rLhsOrig: ::core::ffi::c_int = rLhs;
                    rLhs = sqlite3GetTempRange(pParse, nVector);
                    i = 0 as ::core::ffi::c_int;
                    while i < nVector {
                        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                            v,
                            crate::src::headers::opcodes_h::OP_Copy,
                            rLhsOrig + i,
                            rLhs + *aiMap.offset(i as isize),
                            0 as ::core::ffi::c_int,
                        );
                        i += 1;
                    }
                    sqlite3ReleaseTempReg(pParse, rLhsOrig);
                }
            }
            if destIfNull == destIfFalse {
                destStep2 = destIfFalse;
            } else {
                destStep6 = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                );
                destStep2 = destStep6;
            }
            i = 0 as ::core::ffi::c_int;
            loop {
                if (i >= nVector) {
                    current_block = 7178192492338286402;
                    break;
                }
                let p: *mut crate::src::headers::sqliteInt_h::Expr =
                    sqlite3VectorFieldSubexpr((*pExpr).pLeft, i);
                if __pParse_ref.nErr != 0 {
                    current_block = 13839221997671892562;
                    break;
                }
                if sqlite3ExprCanBeNull(p) != 0 {
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                        v,
                        crate::src::headers::opcodes_h::OP_IsNull,
                        rLhs + i,
                        destStep2,
                    );
                }
                i += 1;
            }
            match current_block {
                13839221997671892562 => {}
                _ => {
                    if eType == crate::src::headers::sqliteInt_h::IN_INDEX_ROWID {
                        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                            v,
                            crate::src::headers::opcodes_h::OP_SeekRowid,
                            iTab,
                            destIfFalse,
                            rLhs,
                        );
                        addrTruthOp = crate::src::src::vdbeaux::sqlite3VdbeAddOp0(
                            v,
                            crate::src::headers::opcodes_h::OP_Goto,
                        );
                        current_block = 9199578309995299736;
                    } else if destIfFalse == destIfNull {
                        if (*pExpr).flags
                            & 0x2000000 as ::core::ffi::c_int
                                as crate::src::ext::rtree::rtree::U32_0
                            != 0 as crate::src::ext::rtree::rtree::U32_0
                        {
                            let pOp: *const crate::src::src::vdbe::VdbeOp =
                                crate::src::src::vdbeaux::sqlite3VdbeGetOp(v, (*pExpr).y.sub.iAddr)
                                    as *mut crate::src::src::vdbe::VdbeOp;
                            if (*pOp).opcode as ::core::ffi::c_int
                                == crate::src::headers::opcodes_h::OP_Once
                                && (*pOp).p3 > 0 as ::core::ffi::c_int
                            {
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                                    v,
                                    crate::src::headers::opcodes_h::OP_Filter,
                                    (*pOp).p3,
                                    destIfFalse,
                                    rLhs,
                                    nVector,
                                );
                            }
                        }
                        crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                            v,
                            crate::src::headers::opcodes_h::OP_NotFound,
                            iTab,
                            destIfFalse,
                            rLhs,
                            nVector,
                        );
                        current_block = 13839221997671892562;
                    } else {
                        addrTruthOp = crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                            v,
                            crate::src::headers::opcodes_h::OP_Found,
                            iTab,
                            0 as ::core::ffi::c_int,
                            rLhs,
                            nVector,
                        );
                        current_block = 9199578309995299736;
                    }
                    match current_block {
                        13839221997671892562 => {}
                        _ => {
                            if rRhsHasNull != 0 && nVector == 1 as ::core::ffi::c_int {
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                    v,
                                    crate::src::headers::opcodes_h::OP_NotNull,
                                    rRhsHasNull,
                                    destIfFalse,
                                );
                            }
                            if destIfFalse == destIfNull {
                                crate::src::src::vdbeaux::sqlite3VdbeGoto(v, destIfFalse);
                            }
                            if destStep6 != 0 {
                                crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, destStep6);
                            }
                            addrTop = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                v,
                                crate::src::headers::opcodes_h::OP_Rewind,
                                iTab,
                                destIfFalse,
                            );
                            if nVector > 1 as ::core::ffi::c_int {
                                destNotNull = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                );
                            } else {
                                destNotNull = destIfFalse;
                            }
                            i = 0 as ::core::ffi::c_int;
                            while i < nVector {
                                
                                
                                let r3: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
                                let p_0: *mut crate::src::headers::sqliteInt_h::Expr = sqlite3VectorFieldSubexpr(pLeft, i);
                                let pColl_0: *mut crate::src::headers::sqliteInt_h::CollSeq = sqlite3ExprCollSeq(pParse, p_0);
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                    v,
                                    crate::src::headers::opcodes_h::OP_Column,
                                    iTab,
                                    i,
                                    r3,
                                );
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                    v,
                                    crate::src::headers::opcodes_h::OP_Ne,
                                    rLhs + i,
                                    destNotNull,
                                    r3,
                                    pColl_0 as *mut ::core::ffi::c_void
                                        as *const ::core::ffi::c_char,
                                    crate::src::src::vdbe::P4_COLLSEQ,
                                );
                                sqlite3ReleaseTempReg(pParse, r3);
                                i += 1;
                            }
                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                v,
                                crate::src::headers::opcodes_h::OP_Goto,
                                0 as ::core::ffi::c_int,
                                destIfNull,
                            );
                            if nVector > 1 as ::core::ffi::c_int {
                                crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, destNotNull);
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                    v,
                                    crate::src::headers::opcodes_h::OP_Next,
                                    iTab,
                                    addrTop + 1 as ::core::ffi::c_int,
                                );
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                    v,
                                    crate::src::headers::opcodes_h::OP_Goto,
                                    0 as ::core::ffi::c_int,
                                    destIfFalse,
                                );
                            }
                            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrTruthOp);
                        }
                    }
                }
            }
        }
    }
    crate::src::src::malloc::sqlite3DbFree(
        __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        aiMap as *mut ::core::ffi::c_void,
    );
    crate::src::src::malloc::sqlite3DbFree(
        __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        zAff as *mut ::core::ffi::c_void,
    );
}

unsafe extern "C" fn codeReal(
    v: *mut crate::src::headers::vdbeInt_h::Vdbe,
    z: *const ::core::ffi::c_char,
    negateFlag: ::core::ffi::c_int,
    iMem: ::core::ffi::c_int,
) {
    if !z.is_null() {
        let mut value: ::core::ffi::c_double = 0.;
        crate::src::src::util::sqlite3AtoF(
            z,
            &raw mut value,
            crate::src::src::util::sqlite3Strlen30(z),
            crate::src::headers::sqlite3_h::SQLITE_UTF8 as crate::src::ext::rtree::rtree::U8_0,
        );
        if negateFlag != 0 {
            value = -value;
        }
        crate::src::src::vdbeaux::sqlite3VdbeAddOp4Dup8(
            v,
            crate::src::headers::opcodes_h::OP_Real,
            0 as ::core::ffi::c_int,
            iMem,
            0 as ::core::ffi::c_int,
            &raw mut value as *mut crate::src::ext::rtree::rtree::U8_0,
            crate::src::src::vdbe::P4_REAL,
        );
    }
}

unsafe extern "C" fn codeInteger(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    negFlag: ::core::ffi::c_int,
    iMem: ::core::ffi::c_int,
) {
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    if (*pExpr).flags
        & crate::src::headers::sqliteInt_h::EP_IntValue as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        let mut i: ::core::ffi::c_int = (*pExpr).u.iValue;
        if negFlag != 0 {
            i = -i;
        }
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Integer,
            i,
            iMem,
        );
    } else {
        
        let mut value: crate::src::ext::rtree::rtree::I64_0 = 0;
        let z: *const ::core::ffi::c_char = (*pExpr).u.zToken;
        let c: ::core::ffi::c_int = crate::src::src::util::sqlite3DecOrHexToI64(z, &raw mut value);
        if c == 3 as ::core::ffi::c_int && negFlag == 0
            || c == 2 as ::core::ffi::c_int
            || negFlag != 0 && value == crate::fts3Int_h::SMALLEST_INT64
        {
            if crate::src::src::util::sqlite3_strnicmp(
                z,
                b"0x\0" as *const u8 as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
            ) == 0 as ::core::ffi::c_int
            {
                crate::src::src::util::sqlite3ErrorMsg_args(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    b"hex literal too big: %s%#T\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::src::src::printf::PrintfArg::Str(if negFlag != 0 {
                            b"-\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        }
                            as *mut ::core::ffi::c_char),
                        crate::src::src::printf::PrintfArg::Expr(pExpr),
                    ],
                );
            } else {
                codeReal(v, z, negFlag, iMem);
            }
        } else {
            if negFlag != 0 {
                value = if c == 3 as ::core::ffi::c_int {
                    crate::fts3Int_h::SMALLEST_INT64
                } else {
                    -value
                };
            }
            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Dup8(
                v,
                crate::src::headers::opcodes_h::OP_Int64,
                0 as ::core::ffi::c_int,
                iMem,
                0 as ::core::ffi::c_int,
                &raw mut value as *mut crate::src::ext::rtree::rtree::U8_0,
                crate::src::src::vdbe::P4_INT64,
            );
        }
    };
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCodeLoadIndexColumn(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pIdx: *mut crate::src::headers::sqliteInt_h::Index,
    iTabCur: ::core::ffi::c_int,
    iIdxCol: ::core::ffi::c_int,
    regOut: ::core::ffi::c_int,
) {
    let iTabCol: crate::src::fts5::I16_0 = *(*pIdx).aiColumn.offset(iIdxCol as isize);
    if iTabCol as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::XN_EXPR {
        (*pParse).iSelfTab = iTabCur + 1 as ::core::ffi::c_int;
        sqlite3ExprCodeCopy(
            pParse,
            (*(&raw mut (*(*pIdx).aColExpr).a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(iIdxCol as isize))
            .pExpr,
            regOut,
        );
        (*pParse).iSelfTab = 0 as ::core::ffi::c_int;
    } else {
        sqlite3ExprCodeGetColumnOfTable(
            (*pParse).pVdbe,
            (*pIdx).pTable,
            iTabCur,
            iTabCol as ::core::ffi::c_int,
            regOut,
        );
    };
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCodeGeneratedColumn(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pTab: *mut crate::src::headers::sqliteInt_h::Table,
    pCol: *mut crate::src::headers::sqliteInt_h::Column,
    regOut: ::core::ffi::c_int,
) {
    let iAddr: ::core::ffi::c_int;
    let __pParse_ref = unsafe { &mut *pParse };
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = __pParse_ref.pVdbe;
    let nErr: ::core::ffi::c_int = __pParse_ref.nErr;
    if __pParse_ref.iSelfTab > 0 as ::core::ffi::c_int {
        iAddr = crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
            v,
            crate::src::headers::opcodes_h::OP_IfNullRow,
            __pParse_ref.iSelfTab - 1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            regOut,
        );
    } else {
        iAddr = 0 as ::core::ffi::c_int;
    }
    sqlite3ExprCodeCopy(
        pParse,
        crate::src::src::build::sqlite3ColumnExpr(
            pTab as *mut crate::src::headers::sqliteInt_h::Table,
            pCol as *mut crate::src::headers::sqliteInt_h::Column,
        ) as *mut crate::src::headers::sqliteInt_h::Expr,
        regOut,
    );
    if (*pCol).colFlags as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::COLFLAG_VIRTUAL
        != 0 as ::core::ffi::c_int
        && (*pTab).tabFlags
            & crate::src::headers::sqliteInt_h::TF_Strict as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        let p3: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + pCol.offset_from((*pTab).aCol) as ::core::ffi::c_long as ::core::ffi::c_int;
        crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
            v,
            crate::src::headers::opcodes_h::OP_TypeCheck,
            regOut,
            1 as ::core::ffi::c_int,
            p3,
            pTab as *mut ::core::ffi::c_char,
            crate::src::src::vdbe::P4_TABLE,
        );
    } else if (*pCol).affinity as ::core::ffi::c_int
        >= crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT
    {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
            v,
            crate::src::headers::opcodes_h::OP_Affinity,
            regOut,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw mut (*pCol).affinity,
            1 as ::core::ffi::c_int,
        );
    }
    if iAddr != 0 {
        crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, iAddr);
    }
    if __pParse_ref.nErr > nErr {
        (*__pParse_ref.db).errByteOffset = -(1 as ::core::ffi::c_int);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCodeGetColumnOfTable(
    v: *mut crate::src::headers::vdbeInt_h::Vdbe,
    pTab: *mut crate::src::headers::sqliteInt_h::Table,
    iTabCur: ::core::ffi::c_int,
    iCol: ::core::ffi::c_int,
    regOut: ::core::ffi::c_int,
) {
    let pCol: *mut crate::src::headers::sqliteInt_h::Column;
    if iCol < 0 as ::core::ffi::c_int || iCol == (*pTab).iPKey as ::core::ffi::c_int {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Rowid,
            iTabCur,
            regOut,
        );
    } else {
        let op: ::core::ffi::c_int;
        let x: ::core::ffi::c_int;
        if (*pTab).eTabType as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::TABTYP_VTAB {
            op = crate::src::headers::opcodes_h::OP_VColumn;
            x = iCol;
        } else {
            pCol =
                (*pTab).aCol.offset(iCol as isize) as *mut crate::src::headers::sqliteInt_h::Column;
            if (*pCol).colFlags as ::core::ffi::c_int
                & crate::src::headers::sqliteInt_h::COLFLAG_VIRTUAL
                != 0
            {
                let pParse: *mut crate::src::headers::sqliteInt_h::Parse =
                    crate::src::src::vdbeaux::sqlite3VdbeParser(v)
                        as *mut crate::src::headers::sqliteInt_h::Parse;
                if (*pCol).colFlags as ::core::ffi::c_int
                    & crate::src::headers::sqliteInt_h::COLFLAG_BUSY
                    != 0
                {
                    crate::src::src::util::sqlite3ErrorMsg_args(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        b"generated column loop on \"%s\"\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[crate::src::src::printf::PrintfArg::Str(
                            (*pCol).zCnName as *mut ::core::ffi::c_char,
                        )],
                    );
                } else {
                    let __pParse_ref = unsafe { &mut *pParse };
                    let savedSelfTab: ::core::ffi::c_int = __pParse_ref.iSelfTab;
                    let __pCol_ref = unsafe { &mut *pCol };
                    __pCol_ref.colFlags = (__pCol_ref.colFlags as ::core::ffi::c_int
                        | crate::src::headers::sqliteInt_h::COLFLAG_BUSY)
                        as crate::src::fts5::U16_0;
                    __pParse_ref.iSelfTab = iTabCur + 1 as ::core::ffi::c_int;
                    sqlite3ExprCodeGeneratedColumn(pParse, pTab, pCol, regOut);
                    __pParse_ref.iSelfTab = savedSelfTab;
                    __pCol_ref.colFlags = (__pCol_ref.colFlags as ::core::ffi::c_int
                        & !crate::src::headers::sqliteInt_h::COLFLAG_BUSY)
                        as crate::src::fts5::U16_0;
                }
                return;
            } else if ((*pTab).tabFlags
                & crate::src::headers::sqliteInt_h::TF_WithoutRowid
                    as crate::src::ext::rtree::rtree::U32_0 != 0 as crate::src::ext::rtree::rtree::U32_0)
            {
                x = crate::src::src::build::sqlite3TableColumnToIndex(
                    crate::src::src::build::sqlite3PrimaryKeyIndex(
                        pTab as *mut crate::src::headers::sqliteInt_h::Table,
                    ) as *mut crate::src::headers::sqliteInt_h::Index
                        as *mut crate::src::headers::sqliteInt_h::Index,
                    iCol,
                );
                op = crate::src::headers::opcodes_h::OP_Column;
            } else {
                x = crate::src::src::build::sqlite3TableColumnToStorage(
                    pTab as *mut crate::src::headers::sqliteInt_h::Table,
                    iCol as crate::src::fts5::I16_0,
                ) as ::core::ffi::c_int;
                op = crate::src::headers::opcodes_h::OP_Column;
            }
        }
        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(v, op, iTabCur, x, regOut);
        crate::src::src::update::sqlite3ColumnDefault(
            v,
            pTab as *mut crate::src::headers::sqliteInt_h::Table,
            iCol,
            regOut,
        );
    };
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCodeGetColumn(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pTab: *mut crate::src::headers::sqliteInt_h::Table,
    iColumn: ::core::ffi::c_int,
    iTable: ::core::ffi::c_int,
    iReg: ::core::ffi::c_int,
    p5: crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    sqlite3ExprCodeGetColumnOfTable((*pParse).pVdbe, pTab, iTable, iColumn, iReg);
    if p5 != 0 {
        let pOp: *mut crate::src::src::vdbe::VdbeOp =
            crate::src::src::vdbeaux::sqlite3VdbeGetLastOp((*pParse).pVdbe)
                as *mut crate::src::src::vdbe::VdbeOp;
        if (*pOp).opcode as ::core::ffi::c_int == crate::src::headers::opcodes_h::OP_Column {
            (*pOp).p5 = p5 as crate::src::fts5::U16_0;
        }
        if (*pOp).opcode as ::core::ffi::c_int == crate::src::headers::opcodes_h::OP_VColumn {
            (*pOp).p5 = (p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::OPFLAG_NOCHNG)
                as crate::src::fts5::U16_0;
        }
    }
    iReg
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCodeMove(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    iFrom: ::core::ffi::c_int,
    iTo: ::core::ffi::c_int,
    nReg: ::core::ffi::c_int,
) {
    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
        (*pParse).pVdbe,
        crate::src::headers::opcodes_h::OP_Move,
        iFrom,
        iTo,
        nReg,
    );
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprToRegister(
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    iReg: ::core::ffi::c_int,
) {
    let p: *mut crate::src::headers::sqliteInt_h::Expr = sqlite3ExprSkipCollateAndLikely(pExpr);
    if p.is_null() {
        return;
    }
    if ((*p).op as ::core::ffi::c_int != crate::src::parse::TK_REGISTER) {
        let __p_ref = unsafe { &mut *p };
        __p_ref.op2 = __p_ref.op;
        __p_ref.op = crate::src::parse::TK_REGISTER as crate::src::ext::rtree::rtree::U8_0;
        __p_ref.iTable = iReg;
        __p_ref.flags &= !(0x2000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0);
    }
}

unsafe extern "C" fn exprCodeVector(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Expr,
    piFreeable: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let iResult: ::core::ffi::c_int;
    let nResult: ::core::ffi::c_int = sqlite3ExprVectorSize(p);
    if nResult == 1 as ::core::ffi::c_int {
        iResult = sqlite3ExprCodeTemp(pParse, p, piFreeable);
    } else {
        *piFreeable = 0 as ::core::ffi::c_int;
        if (*p).op as ::core::ffi::c_int == crate::src::parse::TK_SELECT {
            iResult = sqlite3CodeSubselect(pParse, p);
        } else {
            let mut i: ::core::ffi::c_int;
            iResult = (*pParse).nMem + 1 as ::core::ffi::c_int;
            (*pParse).nMem += nResult;
            i = 0 as ::core::ffi::c_int;
            while i < nResult {
                sqlite3ExprCodeFactorable(
                    pParse,
                    (*(&raw mut (*(*p).x.pList).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(i as isize))
                    .pExpr,
                    i + iResult,
                );
                i += 1;
            }
        }
    }
    iResult
}

unsafe extern "C" fn setDoNotMergeFlagOnCopy(v: *mut crate::src::headers::vdbeInt_h::Vdbe) {
    if (*(crate::src::src::vdbeaux::sqlite3VdbeGetLastOp(v) as *mut crate::src::src::vdbe::VdbeOp))
        .opcode as ::core::ffi::c_int
        == crate::src::headers::opcodes_h::OP_Copy
    {
        crate::src::src::vdbeaux::sqlite3VdbeChangeP5(v, 1 as crate::src::fts5::U16_0);
    }
}

unsafe extern "C" fn exprCodeInlineFunction(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pFarg: *mut crate::src::headers::sqliteInt_h::ExprList,
    iFuncId: ::core::ffi::c_int,
    mut target: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    let nFarg: ::core::ffi::c_int = (*pFarg).nExpr;
    match iFuncId {
        crate::src::headers::sqliteInt_h::INLINEFUNC_coalesce => {
            let endCoalesce: ::core::ffi::c_int =
                crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                );
            let mut i: ::core::ffi::c_int;
            sqlite3ExprCode(
                pParse,
                (*(&raw mut (*pFarg).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(0_isize))
                .pExpr,
                target,
            );
            i = 1 as ::core::ffi::c_int;
            while i < nFarg {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_NotNull,
                    target,
                    endCoalesce,
                );
                sqlite3ExprCode(
                    pParse,
                    (*(&raw mut (*pFarg).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(i as isize))
                    .pExpr,
                    target,
                );
                i += 1;
            }
            setDoNotMergeFlagOnCopy(v);
            crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, endCoalesce);
        }
        crate::src::headers::sqliteInt_h::INLINEFUNC_iif => {
            let mut caseExpr: crate::src::headers::sqliteInt_h::Expr =
                unsafe { ::core::mem::zeroed() };
            caseExpr.op = crate::src::parse::TK_CASE_1 as crate::src::ext::rtree::rtree::U8_0;
            caseExpr.x.pList = pFarg;
            return sqlite3ExprCodeTarget(pParse, &raw mut caseExpr, target);
        }
        crate::src::headers::sqliteInt_h::INLINEFUNC_expr_compare => {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_Integer,
                sqlite3ExprCompare(
                    ::core::ptr::null::<crate::src::headers::sqliteInt_h::Parse>(),
                    (*(&raw mut (*pFarg).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(0_isize))
                    .pExpr,
                    (*(&raw mut (*pFarg).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(1_isize))
                    .pExpr,
                    -(1 as ::core::ffi::c_int),
                ),
                target,
            );
        }
        crate::src::headers::sqliteInt_h::INLINEFUNC_expr_implies_expr => {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_Integer,
                sqlite3ExprImpliesExpr(
                    pParse,
                    (*(&raw mut (*pFarg).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(0_isize))
                    .pExpr,
                    (*(&raw mut (*pFarg).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(1_isize))
                    .pExpr,
                    -(1 as ::core::ffi::c_int),
                ),
                target,
            );
        }
        crate::src::headers::sqliteInt_h::INLINEFUNC_implies_nonnull_row => {
            
            let pA1: *mut crate::src::headers::sqliteInt_h::Expr = (*(&raw mut (*pFarg).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(1_isize))
            .pExpr;
            if (*pA1).op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Integer,
                    sqlite3ExprImpliesNonNullRow(
                        (*(&raw mut (*pFarg).a
                            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                            .offset(0_isize))
                        .pExpr,
                        (*pA1).iTable,
                        1 as ::core::ffi::c_int,
                    ),
                    target,
                );
            } else {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Null,
                    0 as ::core::ffi::c_int,
                    target,
                );
            }
        }
        crate::src::headers::sqliteInt_h::INLINEFUNC_affinity => {
            let azAff: [*const ::core::ffi::c_char; 6] = [
                b"blob\0" as *const u8 as *const ::core::ffi::c_char,
                b"text\0" as *const u8 as *const ::core::ffi::c_char,
                b"numeric\0" as *const u8 as *const ::core::ffi::c_char,
                b"integer\0" as *const u8 as *const ::core::ffi::c_char,
                b"real\0" as *const u8 as *const ::core::ffi::c_char,
                b"flexnum\0" as *const u8 as *const ::core::ffi::c_char,
            ];
            
            let aff: ::core::ffi::c_char = sqlite3ExprAffinity(
                (*(&raw mut (*pFarg).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(0_isize))
                .pExpr,
            );
            crate::src::src::vdbeaux::sqlite3VdbeLoadString(
                v,
                target,
                if aff as ::core::ffi::c_int <= crate::src::headers::sqliteInt_h::SQLITE_AFF_NONE {
                    b"none\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    azAff[(aff as ::core::ffi::c_int
                        - crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB)
                        as usize]
                },
            );
        }
        _ => {
            target = sqlite3ExprCodeTarget(
                pParse,
                (*(&raw mut (*pFarg).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(0_isize))
                .pExpr,
                target,
            );
        }
    }
    target
}

unsafe extern "C" fn exprNodeCanReturnSubtype(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    
    
    
    let __pExpr_ref = unsafe { &mut *pExpr };
    if __pExpr_ref.op as ::core::ffi::c_int != crate::src::parse::TK_FUNCTION {
        return crate::src::headers::sqliteInt_h::WRC_Prune;
    }
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*(*pWalker).pParse).db;
    let n: ::core::ffi::c_int = if !__pExpr_ref.x.pList.is_null() {
        (*__pExpr_ref.x.pList).nExpr
    } else {
        0 as ::core::ffi::c_int
    };
    let pDef: *mut crate::src::headers::sqliteInt_h::FuncDef = crate::src::src::callback::sqlite3FindFunction(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __pExpr_ref.u.zToken,
        n,
        (*db).enc,
        0 as crate::src::ext::rtree::rtree::U8_0,
    ) as *mut crate::src::headers::sqliteInt_h::FuncDef;
    if pDef.is_null()
        || (*pDef).funcFlags
            & crate::src::headers::sqlite3_h::SQLITE_RESULT_SUBTYPE
                as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        (*pWalker).eCode = 1 as crate::src::fts5::U16_0;
        return crate::src::headers::sqliteInt_h::WRC_Prune;
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}

unsafe extern "C" fn sqlite3ExprCanReturnSubtype(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let mut w: crate::src::headers::sqliteInt_h::Walker = unsafe { ::core::mem::zeroed() };
    w.pParse = pParse;
    w.xExprCallback = Some(
        exprNodeCanReturnSubtype
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
    crate::src::src::walker::sqlite3WalkExpr(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
    );
    w.eCode as ::core::ffi::c_int
}
#[inline(never)]
unsafe extern "C" fn sqlite3IndexedExprLookup(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    target: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut crate::src::headers::sqliteInt_h::IndexedExpr;
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe;
    let mut current_block_21: u64;
    p = (*pParse).pIdxEpr;
    while !p.is_null() {
        let exprAff: crate::src::ext::rtree::rtree::U8_0;
        let mut iDataCur: ::core::ffi::c_int = (*p).iDataCur;
        if (iDataCur >= 0 as ::core::ffi::c_int) {
            if (*pParse).iSelfTab != 0 {
                if (*p).iDataCur != (*pParse).iSelfTab - 1 as ::core::ffi::c_int {
                    current_block_21 = 16668937799742929182;
                } else {
                    iDataCur = -(1 as ::core::ffi::c_int);
                    current_block_21 = 11006700562992250127;
                }
            } else {
                current_block_21 = 11006700562992250127;
            }
            match current_block_21 {
                16668937799742929182 => {}
                _ => {
                    if (sqlite3ExprCompare(
                        ::core::ptr::null::<crate::src::headers::sqliteInt_h::Parse>(),
                        pExpr,
                        (*p).pExpr,
                        iDataCur,
                    ) == 0 as ::core::ffi::c_int)
                    {
                        exprAff = sqlite3ExprAffinity(pExpr) as crate::src::ext::rtree::rtree::U8_0;
                        if !(exprAff as ::core::ffi::c_int
                            <= crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB
                            && (*p).aff as ::core::ffi::c_int
                                != crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB
                            || exprAff as ::core::ffi::c_int
                                == crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT
                                && (*p).aff as ::core::ffi::c_int
                                    != crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT
                            || exprAff as ::core::ffi::c_int
                                >= crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC
                                && (*p).aff as ::core::ffi::c_int
                                    != crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC)
                            && !((*pExpr).flags
                                & 0x80000000 as ::core::ffi::c_uint
                                    as crate::src::ext::rtree::rtree::U32_0
                                != 0 as crate::src::ext::rtree::rtree::U32_0
                                && sqlite3ExprCanReturnSubtype(pParse, pExpr) != 0)
                            {
                                v = (*pParse).pVdbe;
                                if (*p).bMaybeNullRow != 0 {
                                    let addr: ::core::ffi::c_int =
                                        crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v);
                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                        v,
                                        crate::src::headers::opcodes_h::OP_IfNullRow,
                                        (*p).iIdxCur,
                                        addr + 3 as ::core::ffi::c_int,
                                        target,
                                    );
                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                        v,
                                        crate::src::headers::opcodes_h::OP_Column,
                                        (*p).iIdxCur,
                                        (*p).iIdxCol,
                                        target,
                                    );
                                    crate::src::src::vdbeaux::sqlite3VdbeGoto(
                                        v,
                                        0 as ::core::ffi::c_int,
                                    );
                                    let __pParse_ref = unsafe { &mut *pParse };
                                    p = __pParse_ref.pIdxEpr;
                                    __pParse_ref.pIdxEpr = ::core::ptr::null_mut::<
                                        crate::src::headers::sqliteInt_h::IndexedExpr,
                                    >();
                                    sqlite3ExprCode(pParse, pExpr, target);
                                    __pParse_ref.pIdxEpr = p;
                                    crate::src::src::vdbeaux::sqlite3VdbeJumpHere(
                                        v,
                                        addr + 2 as ::core::ffi::c_int,
                                    );
                                } else {
                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                        v,
                                        crate::src::headers::opcodes_h::OP_Column,
                                        (*p).iIdxCur,
                                        (*p).iIdxCol,
                                        target,
                                    );
                                }
                                return target;
                            }
                    }
                }
            }
        }
        p = (*p).pIENext;
    }
    -(1 as ::core::ffi::c_int)
}

unsafe extern "C" fn exprPartidxExprLookup(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    iTarget: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut crate::src::headers::sqliteInt_h::IndexedExpr;
    p = (*pParse).pIdxPartExpr;
    while !p.is_null() {
        if (*pExpr).iColumn as ::core::ffi::c_int == (*p).iIdxCol
            && (*pExpr).iTable == (*p).iDataCur
        {
            let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
            let mut addr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            
            let __p_ref = unsafe { &mut *p };
            if __p_ref.bMaybeNullRow != 0 {
                addr = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                    v,
                    crate::src::headers::opcodes_h::OP_IfNullRow,
                    __p_ref.iIdxCur,
                );
            }
            let ret: ::core::ffi::c_int = sqlite3ExprCodeTarget(pParse, __p_ref.pExpr, iTarget);
            crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                (*pParse).pVdbe,
                crate::src::headers::opcodes_h::OP_Affinity,
                ret,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                &raw mut __p_ref.aff as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
            if addr != 0 {
                crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addr);
                crate::src::src::vdbeaux::sqlite3VdbeChangeP3(v, addr, ret);
            }
            return ret;
        }
        p = (*p).pIENext;
    }
    0 as ::core::ffi::c_int
}
#[inline(never)]
unsafe extern "C" fn exprCodeTargetAndOr(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    target: ::core::ffi::c_int,
    pTmpReg: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    
    let addrSkip: ::core::ffi::c_int;
    let regSS: ::core::ffi::c_int;
    let r1: ::core::ffi::c_int;
    let r2: ::core::ffi::c_int;
    
    
    let op: ::core::ffi::c_int = (*pExpr).op as ::core::ffi::c_int;
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    let pAlt: *mut crate::src::headers::sqliteInt_h::Expr = sqlite3ExprSimplifiedAndOr(pExpr);
    if pAlt != pExpr {
        r1 = sqlite3ExprCodeTarget(pParse, pAlt, target);
        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
            v,
            crate::src::headers::opcodes_h::OP_And,
            r1,
            r1,
            target,
        );
        return target;
    }
    let skipOp: ::core::ffi::c_int = if op == crate::src::parse::TK_AND {
        crate::src::headers::opcodes_h::OP_IfNot
    } else {
        crate::src::headers::opcodes_h::OP_If
    };
    if exprEvalRhsFirst(pExpr) != 0 {
        regSS = sqlite3ExprCodeTarget(pParse, (*pExpr).pRight, target);
        r2 = regSS;
        addrSkip = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(v, skipOp, r2);
        r1 = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, pTmpReg);
    } else {
        let __pExpr_ref = unsafe { &*pExpr };
        r1 = sqlite3ExprCodeTarget(pParse, __pExpr_ref.pLeft, target);
        if (*__pExpr_ref.pRight).flags
            & 0x400000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
        {
            regSS = r1;
            addrSkip = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(v, skipOp, r1);
        } else {
            regSS = 0 as ::core::ffi::c_int;
            addrSkip = regSS;
        }
        r2 = sqlite3ExprCodeTemp(pParse, __pExpr_ref.pRight, pTmpReg);
    }
    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(v, op, r2, r1, target);
    if addrSkip != 0 {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Goto,
            0 as ::core::ffi::c_int,
            crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
        );
        crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrSkip);
        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
            v,
            crate::src::headers::opcodes_h::OP_Or,
            regSS,
            regSS,
            target,
        );
    }
    target
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCodeTarget(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    target: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pFarg: *mut crate::src::headers::sqliteInt_h::ExprList =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
    let mut nFarg: ::core::ffi::c_int = 0;
    let mut pDef: *mut crate::src::headers::sqliteInt_h::FuncDef =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::FuncDef>();
    let zId: *const ::core::ffi::c_char;
    let mut constMask: crate::src::ext::rtree::rtree::U32_0 = 0;
    let mut i: ::core::ffi::c_int;
    let mut db: *mut crate::src::headers::sqliteInt_h::sqlite3 =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>();
    let enc: crate::src::ext::rtree::rtree::U8_0;
    let mut pColl: *mut crate::src::headers::sqliteInt_h::CollSeq =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::CollSeq>();
    let addrINR: ::core::ffi::c_int;
    let mut okConstFactor: crate::src::ext::rtree::rtree::U8_0 = 0;
    let mut pAggInfo_0: *mut crate::src::headers::sqliteInt_h::AggInfo =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::AggInfo>();
    let mut endLabel: ::core::ffi::c_int = 0;
    let mut nextCase: ::core::ffi::c_int;
    let mut nExpr: ::core::ffi::c_int = 0;
    let mut i_0: ::core::ffi::c_int;
    let mut pEList: *mut crate::src::headers::sqliteInt_h::ExprList =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
    let mut aListelem: *mut crate::src::headers::sqliteInt_h::ExprList_item =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList_item>();
    let mut opCompare: crate::src::headers::sqliteInt_h::Expr = unsafe { ::core::mem::zeroed() };
    let mut pX: *mut crate::src::headers::sqliteInt_h::Expr =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
    let mut pTest: *mut crate::src::headers::sqliteInt_h::Expr =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
    let mut pDel: *mut crate::src::headers::sqliteInt_h::Expr =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
    let mut db_0: *mut crate::src::headers::sqliteInt_h::sqlite3 =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>();
    let mut pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::AggInfo>();
    let pCol: *mut crate::src::headers::sqliteInt_h::AggInfo_col;
    let mut current_block: u64;
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    let mut op: ::core::ffi::c_int;
    let mut inReg: ::core::ffi::c_int = target;
    let mut regFree1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut regFree2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut r1: ::core::ffi::c_int = 0;
    let mut r2: ::core::ffi::c_int = 0;
    let mut tempX: crate::src::headers::sqliteInt_h::Expr = unsafe { ::core::mem::zeroed() };
    let mut p5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        if pExpr.is_null() {
            op = crate::src::parse::TK_NULL;
        } else if !(*pParse).pIdxEpr.is_null()
            && ((*pExpr).flags
                & 0x800000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
            && {
                r1 = sqlite3IndexedExprLookup(pParse, pExpr, target);
                r1 >= 0 as ::core::ffi::c_int
            }
        {
            return r1;
        } else {
            op = (*pExpr).op as ::core::ffi::c_int;
        }
        match op {
            crate::src::parse::TK_AGG_COLUMN => {
                pAggInfo = (*pExpr).pAggInfo;
                if (*pExpr).iAgg as ::core::ffi::c_int >= (*pAggInfo).nColumn {
                    current_block = 9606288038608642794;
                    break;
                } else {
                    current_block = 5143058163439228106;
                    break;
                }
            }
            crate::src::parse::TK_COLUMN => {
                current_block = 133000372198906578;
                break;
            }
            crate::src::parse::TK_INTEGER => {
                codeInteger(pParse, pExpr, 0 as ::core::ffi::c_int, target);
                return target;
            }
            crate::src::parse::TK_TRUEFALSE => {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Integer,
                    sqlite3ExprTruthValue(pExpr),
                    target,
                );
                return target;
            }
            crate::src::parse::TK_FLOAT => {
                codeReal(v, (*pExpr).u.zToken, 0 as ::core::ffi::c_int, target);
                return target;
            }
            crate::src::parse::TK_STRING => {
                crate::src::src::vdbeaux::sqlite3VdbeLoadString(v, target, (*pExpr).u.zToken);
                return target;
            }
            crate::src::parse::TK_NULLS => {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                    v,
                    crate::src::headers::opcodes_h::OP_Null,
                    0 as ::core::ffi::c_int,
                    target,
                    target + (*pExpr).y.nReg - 1 as ::core::ffi::c_int,
                );
                return target;
            }
            crate::src::parse::TK_BLOB => {
                
                
                
                let z: *const ::core::ffi::c_char = (*pExpr).u.zToken.offset(2_isize) as *mut ::core::ffi::c_char;
                let n: ::core::ffi::c_int = crate::src::src::util::sqlite3Strlen30(z) - 1 as ::core::ffi::c_int;
                let zBlob: *mut ::core::ffi::c_char = crate::src::src::util::sqlite3HexToBlob(
                    crate::src::src::vdbeaux::sqlite3VdbeDb(v)
                        as *mut crate::src::headers::sqliteInt_h::sqlite3
                        as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    z,
                    n,
                ) as *mut ::core::ffi::c_char;
                crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                    v,
                    crate::src::headers::opcodes_h::OP_Blob,
                    n / 2 as ::core::ffi::c_int,
                    target,
                    0 as ::core::ffi::c_int,
                    zBlob,
                    crate::src::src::vdbe::P4_DYNAMIC,
                );
                return target;
            }
            crate::src::parse::TK_VARIABLE => {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Variable,
                    (*pExpr).iColumn as ::core::ffi::c_int,
                    target,
                );
                return target;
            }
            crate::src::parse::TK_REGISTER => return (*pExpr).iTable,
            crate::src::parse::TK_CAST => {
                sqlite3ExprCode(pParse, (*pExpr).pLeft, target);
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Cast,
                    target,
                    crate::src::src::build::sqlite3AffinityType(
                        (*pExpr).u.zToken,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Column>()
                            as *mut crate::src::headers::sqliteInt_h::Column,
                    ) as ::core::ffi::c_int,
                );
                return inReg;
            }
            crate::src::parse::TK_IS | crate::src::parse::TK_ISNOT_1 => {
                op = if op == crate::src::parse::TK_IS {
                    crate::src::parse::TK_EQ_1
                } else {
                    crate::src::parse::TK_NE
                };
                p5 = crate::src::headers::sqliteInt_h::SQLITE_NULLEQ;
                current_block = 5409161009579131794;
                break;
            }
            crate::src::parse::TK_LT
            | crate::src::parse::TK_LE
            | crate::src::parse::TK_GT
            | crate::src::parse::TK_GE
            | crate::src::parse::TK_NE
            | crate::src::parse::TK_EQ_1 => {
                current_block = 5409161009579131794;
                break;
            }
            crate::src::parse::TK_AND | crate::src::parse::TK_OR => {
                inReg = exprCodeTargetAndOr(pParse, pExpr, target, &raw mut regFree1);
                current_block = 8004310806444026423;
                break;
            }
            crate::src::parse::TK_PLUS_1
            | crate::src::parse::TK_STAR
            | crate::src::parse::TK_MINUS
            | crate::src::parse::TK_REM
            | crate::src::parse::TK_BITAND
            | crate::src::parse::TK_BITOR
            | crate::src::parse::TK_SLASH
            | crate::src::parse::TK_LSHIFT
            | crate::src::parse::TK_RSHIFT
            | crate::src::parse::TK_CONCAT => {
                let addrIsNull_0: ::core::ffi::c_int;
                if (*pExpr).flags
                    & 0x400000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                    != 0 as crate::src::ext::rtree::rtree::U32_0
                {
                    addrIsNull_0 = exprComputeOperands(
                        pParse,
                        pExpr,
                        &raw mut r1,
                        &raw mut r2,
                        &raw mut regFree1,
                        &raw mut regFree2,
                    );
                } else {
                    r1 = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, &raw mut regFree1);
                    r2 = sqlite3ExprCodeTemp(pParse, (*pExpr).pRight, &raw mut regFree2);
                    addrIsNull_0 = 0 as ::core::ffi::c_int;
                }
                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(v, op, r2, r1, target);
                if addrIsNull_0 != 0 {
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                        v,
                        crate::src::headers::opcodes_h::OP_Goto,
                        0 as ::core::ffi::c_int,
                        crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v)
                            + 2 as ::core::ffi::c_int,
                    );
                    crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrIsNull_0);
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                        v,
                        crate::src::headers::opcodes_h::OP_Null,
                        0 as ::core::ffi::c_int,
                        target,
                    );
                }
                current_block = 8004310806444026423;
                break;
            }
            crate::src::parse::TK_UMINUS => {
                let pLeft_0: *mut crate::src::headers::sqliteInt_h::Expr = (*pExpr).pLeft;
                if (*pLeft_0).op as ::core::ffi::c_int == crate::src::parse::TK_INTEGER {
                    codeInteger(pParse, pLeft_0, 1 as ::core::ffi::c_int, target);
                    return target;
                } else if (*pLeft_0).op as ::core::ffi::c_int == crate::src::parse::TK_FLOAT {
                    codeReal(v, (*pLeft_0).u.zToken, 1 as ::core::ffi::c_int, target);
                    return target;
                } else {
                    tempX.op = crate::src::parse::TK_INTEGER as crate::src::ext::rtree::rtree::U8_0;
                    tempX.flags = (crate::src::headers::sqliteInt_h::EP_IntValue
                        | crate::src::headers::sqliteInt_h::EP_TokenOnly)
                        as crate::src::ext::rtree::rtree::U32_0;
                    tempX.u.iValue = 0 as ::core::ffi::c_int;
                    r1 = sqlite3ExprCodeTemp(pParse, &raw mut tempX, &raw mut regFree1);
                    r2 = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, &raw mut regFree2);
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                        v,
                        crate::src::headers::opcodes_h::OP_Subtract,
                        r2,
                        r1,
                        target,
                    );
                }
                current_block = 8004310806444026423;
                break;
            }
            crate::src::parse::TK_BITNOT | crate::src::parse::TK_NOT_1 => {
                r1 = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, &raw mut regFree1);
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(v, op, r1, inReg);
                current_block = 8004310806444026423;
                break;
            }
            crate::src::parse::TK_TRUTH => {
                
                
                let __pExpr_ref = unsafe { &*pExpr };
                r1 = sqlite3ExprCodeTemp(pParse, __pExpr_ref.pLeft, &raw mut regFree1);
                let isTrue: ::core::ffi::c_int = sqlite3ExprTruthValue(__pExpr_ref.pRight);
                let bNormal: ::core::ffi::c_int = (__pExpr_ref.op2 as ::core::ffi::c_int == crate::src::parse::TK_IS)
                    as ::core::ffi::c_int;
                crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                    v,
                    crate::src::headers::opcodes_h::OP_IsTrue,
                    r1,
                    inReg,
                    (isTrue == 0) as ::core::ffi::c_int,
                    isTrue ^ bNormal,
                );
                current_block = 8004310806444026423;
                break;
            }
            crate::src::parse::TK_ISNULL_1 | crate::src::parse::TK_NOTNULL_1 => {
                
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Integer,
                    1 as ::core::ffi::c_int,
                    target,
                );
                r1 = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, &raw mut regFree1);
                let addr: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(v, op, r1);
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Integer,
                    0 as ::core::ffi::c_int,
                    target,
                );
                crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addr);
                current_block = 8004310806444026423;
                break;
            }
            crate::src::parse::TK_AGG_FUNCTION => {
                let __pExpr_ref = unsafe { &*pExpr };
                let pInfo: *mut crate::src::headers::sqliteInt_h::AggInfo =
                    __pExpr_ref.pAggInfo;
                if pInfo.is_null()
                    || (__pExpr_ref.iAgg as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                    || __pExpr_ref.iAgg as ::core::ffi::c_int >= (*pInfo).nFunc
                {
                    crate::src::src::util::sqlite3ErrorMsg_args(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        b"misuse of aggregate: %#T()\0" as *const u8 as *const ::core::ffi::c_char,
                        &[crate::src::src::printf::PrintfArg::Expr(pExpr)],
                    );
                } else {
                    return (*pInfo).iFirstReg
                        + (*pInfo).nColumn
                        + __pExpr_ref.iAgg as ::core::ffi::c_int;
                }
                current_block = 8004310806444026423;
                break;
            }
            crate::src::parse::TK_FUNCTION => {
                constMask = 0 as crate::src::ext::rtree::rtree::U32_0;
                db = (*pParse).db;
                enc = (*db).enc;
                pColl = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::CollSeq>();
                let __pExpr_ref = unsafe { &*pExpr };
                if __pExpr_ref.flags
                    & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                    != 0 as crate::src::ext::rtree::rtree::U32_0
                {
                    return (*__pExpr_ref.y.pWin).regResult;
                }
                if (*pParse).okConstFactor() as ::core::ffi::c_int != 0
                    && sqlite3ExprIsConstantNotJoin(pParse, pExpr) != 0
                {
                    return sqlite3ExprCodeRunJustOnce(pParse, pExpr, -(1 as ::core::ffi::c_int));
                }
                pFarg = __pExpr_ref.x.pList;
                nFarg = if !pFarg.is_null() {
                    (*pFarg).nExpr
                } else {
                    0 as ::core::ffi::c_int
                };
                zId = __pExpr_ref.u.zToken;
                pDef = crate::src::src::callback::sqlite3FindFunction(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    zId,
                    nFarg,
                    enc,
                    0 as crate::src::ext::rtree::rtree::U8_0,
                ) as *mut crate::src::headers::sqliteInt_h::FuncDef;
                if pDef.is_null() || (*pDef).xFinalize.is_some() {
                    current_block = 5577234879133443219;
                    break;
                } else {
                    current_block = 1995330570110937187;
                    break;
                }
            }
            crate::src::parse::TK_EXISTS_1 | crate::src::parse::TK_SELECT => {
                let nCol: ::core::ffi::c_int;
                if (*(*pParse).db).mallocFailed != 0 {
                    return 0 as ::core::ffi::c_int;
                } else if op == crate::src::parse::TK_SELECT
                    && (*pExpr).flags & 0x1000 as crate::src::ext::rtree::rtree::U32_0
                        != 0 as crate::src::ext::rtree::rtree::U32_0
                    && {
                        nCol = (*(*(*pExpr).x.pSelect).pEList).nExpr;
                        nCol != 1 as ::core::ffi::c_int
                    }
                {
                    sqlite3SubselectError(pParse, nCol, 1 as ::core::ffi::c_int);
                } else {
                    return sqlite3CodeSubselect(pParse, pExpr);
                }
                current_block = 8004310806444026423;
                break;
            }
            crate::src::parse::TK_SELECT_COLUMN => {
                
                let __pExpr_ref = unsafe { &*pExpr };
                let pLeft_1: *mut crate::src::headers::sqliteInt_h::Expr = __pExpr_ref.pLeft;
                let __pLeft_1_ref = unsafe { &mut *pLeft_1 };
                if __pLeft_1_ref.iTable == 0 as ::core::ffi::c_int
                    || (*pParse).withinRJSubrtn as ::core::ffi::c_int
                        > __pLeft_1_ref.op2 as ::core::ffi::c_int
                {
                    __pLeft_1_ref.iTable = sqlite3CodeSubselect(pParse, pLeft_1);
                    __pLeft_1_ref.op2 = (*pParse).withinRJSubrtn;
                }
                let n_0: ::core::ffi::c_int = sqlite3ExprVectorSize(pLeft_1);
                if __pExpr_ref.iTable != n_0 {
                    crate::src::src::util::sqlite3ErrorMsg_args(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        b"%d columns assigned %d values\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            crate::src::src::printf::PrintfArg::Int(
                                __pExpr_ref.iTable as crate::src::ext::rtree::rtree::I64_0,
                            ),
                            crate::src::src::printf::PrintfArg::Int(
                                n_0 as crate::src::ext::rtree::rtree::I64_0,
                            ),
                        ],
                    );
                }
                return __pLeft_1_ref.iTable + __pExpr_ref.iColumn as ::core::ffi::c_int;
            }
            crate::src::parse::TK_IN => {
                let destIfFalse: ::core::ffi::c_int =
                    crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    );
                let destIfNull: ::core::ffi::c_int =
                    crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    );
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Null,
                    0 as ::core::ffi::c_int,
                    target,
                );
                sqlite3ExprCodeIN(pParse, pExpr, destIfFalse, destIfNull);
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Integer,
                    1 as ::core::ffi::c_int,
                    target,
                );
                crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, destIfFalse);
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_AddImm,
                    target,
                    0 as ::core::ffi::c_int,
                );
                crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, destIfNull);
                return target;
            }
            crate::src::parse::TK_BETWEEN_1 => {
                exprCodeBetween(pParse, pExpr, target, None, 0 as ::core::ffi::c_int);
                return target;
            }
            crate::src::parse::TK_COLLATE => {
                if ((*pExpr).flags
                    & 0x200 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
                {
                    sqlite3ExprCode(pParse, (*pExpr).pLeft, target);
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                        v,
                        crate::src::headers::opcodes_h::OP_ClrSubtype,
                        target,
                    );
                    return target;
                } else {
                    pExpr = (*pExpr).pLeft;
                }
            }
            crate::src::parse::TK_SPAN_1 | crate::src::parse::TK_UPLUS => {
                pExpr = (*pExpr).pLeft;
            }
            crate::src::parse::TK_TRIGGER => {
                
                
                
                let __pExpr_ref = unsafe { &*pExpr };
                let pTab_1: *mut crate::src::headers::sqliteInt_h::Table = __pExpr_ref.y.pTab;
                let iCol_0: ::core::ffi::c_int = __pExpr_ref.iColumn as ::core::ffi::c_int;
                let p1: ::core::ffi::c_int = __pExpr_ref.iTable
                    * ((*pTab_1).nCol as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int
                    + crate::src::src::build::sqlite3TableColumnToStorage(
                        pTab_1 as *mut crate::src::headers::sqliteInt_h::Table,
                        iCol_0 as crate::src::fts5::I16_0,
                    ) as ::core::ffi::c_int;
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Param,
                    p1,
                    target,
                );
                if iCol_0 >= 0 as ::core::ffi::c_int
                    && (*(*pTab_1).aCol.offset(iCol_0 as isize)).affinity as ::core::ffi::c_int
                        == crate::src::headers::sqliteInt_h::SQLITE_AFF_REAL
                {
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                        v,
                        crate::src::headers::opcodes_h::OP_RealAffinity,
                        target,
                    );
                }
                current_block = 8004310806444026423;
                break;
            }
            crate::src::parse::TK_VECTOR => {
                crate::src::src::util::sqlite3ErrorMsg_args(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    b"row value misused\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
                current_block = 8004310806444026423;
                break;
            }
            crate::src::parse::TK_IF_NULL_ROW => {
                okConstFactor = (*pParse).okConstFactor() as crate::src::ext::rtree::rtree::U8_0;
                pAggInfo_0 = (*pExpr).pAggInfo;
                if !pAggInfo_0.is_null() {
                    current_block = 1003765866876145076;
                    break;
                } else {
                    current_block = 3842749721932025293;
                    break;
                }
            }
            crate::src::parse::TK_CASE_1 => {
                let _ = 
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList_item>();
                opCompare = crate::src::headers::sqliteInt_h::Expr {
                    op: 0,
                    affExpr: 0,
                    op2: 0,
                    flags: 0,
                    u: crate::src::headers::sqliteInt_h::__anon_union_5 {
                        zToken: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    },
                    pLeft: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>(),
                    pRight: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>(),
                    x: crate::src::headers::sqliteInt_h::__anon_union_6 {
                        pList: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>(
                        ),
                    },
                    nHeight: 0,
                    iTable: 0,
                    iColumn: 0,
                    iAgg: 0,
                    w: crate::src::headers::sqliteInt_h::__anon_union_7 { iJoin: 0 },
                    pAggInfo: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::AggInfo>(),
                    y: crate::src::headers::sqliteInt_h::__anon_union_8 {
                        pTab: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>(),
                    },
                };
                pTest = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
                pDel = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
                db_0 = (*pParse).db;
                pEList = (*pExpr).x.pList;
                aListelem = &raw mut (*pEList).a
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item;
                nExpr = (*pEList).nExpr;
                endLabel = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                );
                pX = (*pExpr).pLeft;
                if !pX.is_null() {
                    current_block = 16280550096250383041;
                    break;
                } else {
                    current_block = 4841263011268485695;
                    break;
                }
            }
            crate::src::parse::TK_RAISE => {
                if (*pParse).pTriggerTab.is_null() && (*pParse).nested == 0 {
                    crate::src::src::util::sqlite3ErrorMsg_args(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        b"RAISE() may only be used within a trigger-program\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                    return 0 as ::core::ffi::c_int;
                }
                if (*pExpr).affExpr as ::core::ffi::c_int
                    == crate::src::headers::sqliteInt_h::OE_Abort
                {
                    crate::src::src::build::sqlite3MayAbort(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    );
                }
                if (*pExpr).affExpr as ::core::ffi::c_int
                    == crate::src::headers::sqliteInt_h::OE_Ignore
                {
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                        v,
                        crate::src::headers::opcodes_h::OP_Halt,
                        crate::src::headers::sqlite3_h::SQLITE_OK,
                        crate::src::headers::sqliteInt_h::OE_Ignore,
                    );
                } else {
                    r1 = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, &raw mut regFree1);
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                        v,
                        crate::src::headers::opcodes_h::OP_Halt,
                        if !(*pParse).pTriggerTab.is_null() {
                            crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT_TRIGGER
                        } else {
                            crate::src::headers::sqlite3_h::SQLITE_ERROR
                        },
                        (*pExpr).affExpr as ::core::ffi::c_int,
                        r1,
                    );
                }
                current_block = 8004310806444026423;
                break;
            }
            _ => {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Null,
                    0 as ::core::ffi::c_int,
                    target,
                );
                return target;
            }
        }
    }
    match current_block {
        1995330570110937187 => {
            if (*pDef).funcFlags
                & crate::src::headers::sqliteInt_h::SQLITE_FUNC_INLINE
                    as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
                && !pFarg.is_null()
            {
                return exprCodeInlineFunction(
                    pParse,
                    pFarg,
                    (*pDef).pUserData as crate::src::headers::stdlib::IntptrT
                        as ::core::ffi::c_int,
                    target,
                );
            } else if (*pDef).funcFlags
                & (crate::src::headers::sqliteInt_h::SQLITE_FUNC_DIRECT
                    | crate::src::headers::sqliteInt_h::SQLITE_FUNC_UNSAFE)
                    as crate::src::ext::rtree::rtree::U32_0
                != 0
            {
                sqlite3ExprFunctionUsable(pParse, pExpr, pDef);
            }
            i = 0 as ::core::ffi::c_int;
            while i < nFarg {
                if i < 32 as ::core::ffi::c_int
                    && sqlite3ExprIsConstant(
                        pParse,
                        (*(&raw mut (*pFarg).a
                            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                            .offset(i as isize))
                        .pExpr,
                    ) != 0
                {
                    constMask = (constMask as ::core::ffi::c_uint
                        | (1 as ::core::ffi::c_int as ::core::ffi::c_uint) << i)
                        as crate::src::ext::rtree::rtree::U32_0;
                }
                if (*pDef).funcFlags
                    & crate::src::headers::sqliteInt_h::SQLITE_FUNC_NEEDCOLL
                        as crate::src::ext::rtree::rtree::U32_0
                    != 0 as crate::src::ext::rtree::rtree::U32_0
                    && pColl.is_null()
                {
                    pColl = sqlite3ExprCollSeq(
                        pParse,
                        (*(&raw mut (*pFarg).a
                            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                            .offset(i as isize))
                        .pExpr,
                    );
                }
                i += 1;
            }
            if !pFarg.is_null() {
                if constMask != 0 {
                    r1 = (*pParse).nMem + 1 as ::core::ffi::c_int;
                    (*pParse).nMem += nFarg;
                } else {
                    r1 = sqlite3GetTempRange(pParse, nFarg);
                }
                if (*pDef).funcFlags
                    & (crate::src::headers::sqliteInt_h::SQLITE_FUNC_LENGTH
                        | crate::src::headers::sqliteInt_h::SQLITE_FUNC_TYPEOF)
                        as crate::src::ext::rtree::rtree::U32_0
                    != 0 as crate::src::ext::rtree::rtree::U32_0
                {
                    
                    let exprOp: crate::src::ext::rtree::rtree::U8_0 = (*(*(&raw mut (*pFarg).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(0_isize))
                    .pExpr)
                        .op;
                    if exprOp as ::core::ffi::c_int == crate::src::parse::TK_COLUMN
                        || exprOp as ::core::ffi::c_int == crate::src::parse::TK_AGG_COLUMN
                    {
                        (*(*(&raw mut (*pFarg).a
                            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                            .offset(0_isize))
                        .pExpr)
                            .op2 = ((*pDef).funcFlags
                            & crate::src::headers::sqliteInt_h::OPFLAG_BYTELENARG
                                as crate::src::ext::rtree::rtree::U32_0)
                            as crate::src::ext::rtree::rtree::U8_0;
                    }
                }
                sqlite3ExprCodeExprList(
                    pParse,
                    pFarg,
                    r1,
                    0 as ::core::ffi::c_int,
                    crate::src::headers::sqliteInt_h::SQLITE_ECEL_FACTOR
                        as crate::src::ext::rtree::rtree::U8_0,
                );
            } else {
                r1 = 0 as ::core::ffi::c_int;
            }
            if nFarg >= 2 as ::core::ffi::c_int
                && (*pExpr).flags
                    & 0x100 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                    != 0 as crate::src::ext::rtree::rtree::U32_0
            {
                pDef = crate::src::src::vtab::sqlite3VtabOverloadFunction(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    pDef as *mut crate::src::headers::sqliteInt_h::FuncDef,
                    nFarg,
                    (*(&raw mut (*pFarg).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(1_isize))
                    .pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::FuncDef;
            } else if nFarg > 0 as ::core::ffi::c_int {
                pDef = crate::src::src::vtab::sqlite3VtabOverloadFunction(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    pDef as *mut crate::src::headers::sqliteInt_h::FuncDef,
                    nFarg,
                    (*(&raw mut (*pFarg).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(0_isize))
                    .pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::FuncDef;
            }
            if (*pDef).funcFlags
                & crate::src::headers::sqliteInt_h::SQLITE_FUNC_NEEDCOLL
                    as crate::src::ext::rtree::rtree::U32_0
                != 0
            {
                if pColl.is_null() {
                    pColl = (*db).pDfltColl;
                }
                crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                    v,
                    crate::src::headers::opcodes_h::OP_CollSeq,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    pColl as *mut ::core::ffi::c_char,
                    crate::src::src::vdbe::P4_COLLSEQ,
                );
            }
            crate::src::src::vdbeaux::sqlite3VdbeAddFunctionCall(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                constMask as ::core::ffi::c_int,
                r1,
                target,
                nFarg,
                pDef as *const crate::src::headers::sqliteInt_h::FuncDef,
                (*pExpr).op2 as ::core::ffi::c_int,
            );
            if nFarg != 0
                && constMask == 0 as crate::src::ext::rtree::rtree::U32_0 {
                    sqlite3ReleaseTempRange(pParse, r1, nFarg);
                }
            return target;
        }
        1003765866876145076 => {
            if (*pAggInfo_0).directMode == 0 {
                inReg = (*pAggInfo_0).iFirstReg + (*pExpr).iAgg as ::core::ffi::c_int;
                current_block = 8004310806444026423;
            } else if (*(*pExpr).pAggInfo).useSortingIdx != 0 {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                    v,
                    crate::src::headers::opcodes_h::OP_Column,
                    (*pAggInfo_0).sortingIdxPTab,
                    (*(*pAggInfo_0).aCol.offset((*pExpr).iAgg as isize)).iSorterColumn,
                    target,
                );
                inReg = target;
                current_block = 8004310806444026423;
            } else {
                current_block = 3842749721932025293;
            }
        }
        5409161009579131794 => {
            let pLeft: *mut crate::src::headers::sqliteInt_h::Expr = (*pExpr).pLeft;
            let mut addrIsNull: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if sqlite3ExprIsVector(pLeft) != 0 {
                codeVectorCompare(
                    pParse,
                    pExpr,
                    target,
                    op as crate::src::ext::rtree::rtree::U8_0,
                    p5 as crate::src::ext::rtree::rtree::U8_0,
                );
            } else {
                let __pExpr_ref = unsafe { &*pExpr };
                if __pExpr_ref.flags
                    & 0x400000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                    != 0 as crate::src::ext::rtree::rtree::U32_0
                    && p5 != crate::src::headers::sqliteInt_h::SQLITE_NULLEQ
                {
                    addrIsNull = exprComputeOperands(
                        pParse,
                        pExpr,
                        &raw mut r1,
                        &raw mut r2,
                        &raw mut regFree1,
                        &raw mut regFree2,
                    );
                } else {
                    r1 = sqlite3ExprCodeTemp(pParse, __pExpr_ref.pLeft, &raw mut regFree1);
                    r2 = sqlite3ExprCodeTemp(pParse, __pExpr_ref.pRight, &raw mut regFree2);
                }
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Integer,
                    1 as ::core::ffi::c_int,
                    inReg,
                );
                codeCompare(
                    pParse,
                    pLeft,
                    __pExpr_ref.pRight,
                    op,
                    r1,
                    r2,
                    crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
                    p5,
                    (__pExpr_ref.flags
                        & 0x400 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                        != 0 as crate::src::ext::rtree::rtree::U32_0)
                        as ::core::ffi::c_int,
                );
                if p5 == crate::src::headers::sqliteInt_h::SQLITE_NULLEQ {
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                        v,
                        crate::src::headers::opcodes_h::OP_Integer,
                        0 as ::core::ffi::c_int,
                        inReg,
                    );
                } else {
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                        v,
                        crate::src::headers::opcodes_h::OP_ZeroOrNull,
                        r1,
                        inReg,
                        r2,
                    );
                    if addrIsNull != 0 {
                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                            v,
                            crate::src::headers::opcodes_h::OP_Goto,
                            0 as ::core::ffi::c_int,
                            crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v)
                                + 2 as ::core::ffi::c_int,
                        );
                        crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrIsNull);
                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                            v,
                            crate::src::headers::opcodes_h::OP_Null,
                            0 as ::core::ffi::c_int,
                            inReg,
                        );
                    }
                }
            }
            current_block = 8004310806444026423;
        }
        16280550096250383041 => {
            pDel = sqlite3ExprDup(db_0, pX, 0 as ::core::ffi::c_int);
            if (*db_0).mallocFailed != 0 {
                sqlite3ExprDelete(db_0, pDel);
                current_block = 8004310806444026423;
            } else {
                sqlite3ExprToRegister(pDel, exprCodeVector(pParse, pDel, &raw mut regFree1));
                opCompare.op = crate::src::parse::TK_EQ_1 as crate::src::ext::rtree::rtree::U8_0;
                opCompare.pLeft = pDel;
                pTest = &raw mut opCompare;
                regFree1 = 0 as ::core::ffi::c_int;
                current_block = 4841263011268485695;
            }
        }
        5143058163439228106 => {
            let __pAggInfo_ref = unsafe { &mut *pAggInfo };
            pCol = __pAggInfo_ref.aCol.offset((*pExpr).iAgg as isize)
                as *mut crate::src::headers::sqliteInt_h::AggInfo_col
                as *mut crate::src::headers::sqliteInt_h::AggInfo_col;
            if __pAggInfo_ref.directMode == 0 {
                return __pAggInfo_ref.iFirstReg + (*pExpr).iAgg as ::core::ffi::c_int;
            } else if __pAggInfo_ref.useSortingIdx != 0 {
                let pTab: *mut crate::src::headers::sqliteInt_h::Table = (*pCol).pTab;
                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                    v,
                    crate::src::headers::opcodes_h::OP_Column,
                    __pAggInfo_ref.sortingIdxPTab,
                    (*pCol).iSorterColumn,
                    target,
                );
                if !pTab.is_null()
                    && ((*pCol).iColumn >= 0 as ::core::ffi::c_int)
                        && (*(*pTab).aCol.offset((*pCol).iColumn as isize)).affinity
                            as ::core::ffi::c_int
                            == crate::src::headers::sqliteInt_h::SQLITE_AFF_REAL
                        {
                            crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                                v,
                                crate::src::headers::opcodes_h::OP_RealAffinity,
                                target,
                            );
                        }
                return target;
            } else if (*pExpr).y.pTab.is_null() {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                    v,
                    crate::src::headers::opcodes_h::OP_Column,
                    (*pExpr).iTable,
                    (*pExpr).iColumn as ::core::ffi::c_int,
                    target,
                );
                return target;
            }
            current_block = 133000372198906578;
        }
        9606288038608642794 => {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_Null,
                0 as ::core::ffi::c_int,
                target,
            );
            current_block = 8004310806444026423;
        }
        5577234879133443219 => {
            crate::src::src::util::sqlite3ErrorMsg_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                b"unknown function: %#T()\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Expr(pExpr)],
            );
            current_block = 8004310806444026423;
        }
        _ => {}
    }
    match current_block {
        3842749721932025293 => {
            addrINR = crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_IfNullRow,
                (*pExpr).iTable,
                0 as ::core::ffi::c_int,
                target,
            );
            (*pParse).set_okConstFactor(
                0 as crate::src::headers::sqliteInt_h::Bft as crate::src::headers::sqliteInt_h::Bft,
            );
            sqlite3ExprCode(pParse, (*pExpr).pLeft, target);
            (*pParse).set_okConstFactor(
                okConstFactor as crate::src::headers::sqliteInt_h::Bft
                    as crate::src::headers::sqliteInt_h::Bft,
            );
            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrINR);
        }
        4841263011268485695 => {
            i_0 = 0 as ::core::ffi::c_int;
            while i_0 < nExpr - 1 as ::core::ffi::c_int {
                if !pX.is_null() {
                } else {
                    pTest = (*aListelem.offset(i_0 as isize)).pExpr;
                }
                nextCase = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                );
                sqlite3ExprIfFalse(
                    pParse,
                    pTest,
                    nextCase,
                    crate::src::headers::sqliteInt_h::SQLITE_JUMPIFNULL,
                );
                sqlite3ExprCode(
                    pParse,
                    (*aListelem.offset((i_0 + 1 as ::core::ffi::c_int) as isize)).pExpr,
                    target,
                );
                crate::src::src::vdbeaux::sqlite3VdbeGoto(v, endLabel);
                crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, nextCase);
                i_0 += 2 as ::core::ffi::c_int;
            }
            if nExpr & 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                sqlite3ExprCode(
                    pParse,
                    (*(&raw mut (*pEList).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset((nExpr - 1 as ::core::ffi::c_int) as isize))
                    .pExpr,
                    target,
                );
            } else {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Null,
                    0 as ::core::ffi::c_int,
                    target,
                );
            }
            sqlite3ExprDelete(db_0, pDel);
            setDoNotMergeFlagOnCopy(v);
            crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, endLabel);
        }
        133000372198906578 => {
            let __pExpr_ref = unsafe { &*pExpr };
            let mut iTab: ::core::ffi::c_int = __pExpr_ref.iTable;
            let iReg: ::core::ffi::c_int;
            if __pExpr_ref.flags
                & 0x20 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
            {
                
                iReg = sqlite3ExprCodeTarget(pParse, __pExpr_ref.pLeft, target);
                let aff: ::core::ffi::c_int = sqlite3TableColumnAffinity(
                    __pExpr_ref.y.pTab,
                    __pExpr_ref.iColumn as ::core::ffi::c_int,
                ) as ::core::ffi::c_int;
                if aff > crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB {
                    static mut zAff: [::core::ffi::c_char; 10] = unsafe {
                        ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(
                            *b"B\0C\0D\0E\0F\0",
                        )
                    };
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                        v,
                        crate::src::headers::opcodes_h::OP_Affinity,
                        iReg,
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        (&raw const zAff as *const ::core::ffi::c_char)
                            .offset(((aff - 'B' as i32) * 2 as ::core::ffi::c_int) as isize)
                            as *const ::core::ffi::c_char,
                        crate::src::src::vdbe::P4_STATIC,
                    );
                }
                return iReg;
            }
            if iTab < 0 as ::core::ffi::c_int {
                if (*pParse).iSelfTab < 0 as ::core::ffi::c_int {
                    
                    
                    
                    let iCol: ::core::ffi::c_int = __pExpr_ref.iColumn as ::core::ffi::c_int;
                    let pTab_0: *mut crate::src::headers::sqliteInt_h::Table = __pExpr_ref.y.pTab;
                    if iCol < 0 as ::core::ffi::c_int {
                        return -(1 as ::core::ffi::c_int) - (*pParse).iSelfTab;
                    }
                    let pCol_0: *mut crate::src::headers::sqliteInt_h::Column = (*pTab_0).aCol.offset(iCol as isize);
                    let iSrc: ::core::ffi::c_int = crate::src::src::build::sqlite3TableColumnToStorage(
                        pTab_0 as *mut crate::src::headers::sqliteInt_h::Table,
                        iCol as crate::src::fts5::I16_0,
                    ) as ::core::ffi::c_int
                        - (*pParse).iSelfTab;
                    if (*pCol_0).colFlags as ::core::ffi::c_int
                        & crate::src::headers::sqliteInt_h::COLFLAG_GENERATED
                        != 0
                    {
                        let __pCol_0_ref = unsafe { &mut *pCol_0 };
                        if __pCol_0_ref.colFlags as ::core::ffi::c_int
                            & crate::src::headers::sqliteInt_h::COLFLAG_BUSY
                            != 0
                        {
                            crate::src::src::util::sqlite3ErrorMsg_args(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                b"generated column loop on \"%s\"\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[crate::src::src::printf::PrintfArg::Str(
                                    __pCol_0_ref.zCnName as *mut ::core::ffi::c_char,
                                )],
                            );
                            return 0 as ::core::ffi::c_int;
                        }
                        __pCol_0_ref.colFlags = (__pCol_0_ref.colFlags as ::core::ffi::c_int
                            | crate::src::headers::sqliteInt_h::COLFLAG_BUSY)
                            as crate::src::fts5::U16_0;
                        if __pCol_0_ref.colFlags as ::core::ffi::c_int
                            & crate::src::headers::sqliteInt_h::COLFLAG_NOTAVAIL
                            != 0
                        {
                            sqlite3ExprCodeGeneratedColumn(pParse, pTab_0, pCol_0, iSrc);
                        }
                        __pCol_0_ref.colFlags = (__pCol_0_ref.colFlags as ::core::ffi::c_int
                            & !(crate::src::headers::sqliteInt_h::COLFLAG_BUSY
                                | crate::src::headers::sqliteInt_h::COLFLAG_NOTAVAIL))
                            as crate::src::fts5::U16_0;
                        return iSrc;
                    } else if (*pCol_0).affinity as ::core::ffi::c_int
                        == crate::src::headers::sqliteInt_h::SQLITE_AFF_REAL
                    {
                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                            v,
                            crate::src::headers::opcodes_h::OP_SCopy,
                            iSrc,
                            target,
                        );
                        crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                            v,
                            crate::src::headers::opcodes_h::OP_RealAffinity,
                            target,
                        );
                        return target;
                    } else {
                        return iSrc;
                    }
                } else {
                    iTab = (*pParse).iSelfTab - 1 as ::core::ffi::c_int;
                }
            } else if !(*pParse).pIdxPartExpr.is_null() && {
                r1 = exprPartidxExprLookup(pParse, pExpr, target);
                0 as ::core::ffi::c_int != r1
            } {
                return r1;
            }
            iReg = sqlite3ExprCodeGetColumn(
                pParse,
                __pExpr_ref.y.pTab,
                __pExpr_ref.iColumn as ::core::ffi::c_int,
                iTab,
                target,
                __pExpr_ref.op2,
            );
            return iReg;
        }
        _ => {}
    }
    sqlite3ReleaseTempReg(pParse, regFree1);
    sqlite3ReleaseTempReg(pParse, regFree2);
    inReg
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCodeRunJustOnce(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    mut regDest: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut crate::src::headers::sqliteInt_h::ExprList;
    p = (*pParse).pConstExpr;
    if regDest < 0 as ::core::ffi::c_int && !p.is_null() {
        let mut pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item;
        let mut i: ::core::ffi::c_int;
        pItem = &raw mut (*p).a as *mut crate::src::headers::sqliteInt_h::ExprList_item
            as *mut crate::src::headers::sqliteInt_h::ExprList_item;
        i = (*p).nExpr;
        while i > 0 as ::core::ffi::c_int {
            if (*pItem).fg.reusable() as ::core::ffi::c_int != 0
                && sqlite3ExprCompare(
                    ::core::ptr::null::<crate::src::headers::sqliteInt_h::Parse>(),
                    (*pItem).pExpr,
                    pExpr,
                    -(1 as ::core::ffi::c_int),
                ) == 0 as ::core::ffi::c_int
            {
                return (*pItem).u.iConstExprReg;
            }
            pItem = pItem.offset(1);
            i -= 1;
        }
    }
    pExpr = sqlite3ExprDup((*pParse).db, pExpr, 0 as ::core::ffi::c_int);
    if !pExpr.is_null()
        && (*pExpr).flags & 0x8 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        let __pParse_ref = unsafe { &mut *pParse };
        let v: *mut crate::src::headers::vdbeInt_h::Vdbe = __pParse_ref.pVdbe;
        
        let addr: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeAddOp0(v, crate::src::headers::opcodes_h::OP_Once);
        __pParse_ref.set_okConstFactor(
            0 as crate::src::headers::sqliteInt_h::Bft as crate::src::headers::sqliteInt_h::Bft,
        );
        if (*__pParse_ref.db).mallocFailed == 0 {
            if regDest < 0 as ::core::ffi::c_int {
                __pParse_ref.nMem += 1;
                regDest = __pParse_ref.nMem;
            }
            sqlite3ExprCode(pParse, pExpr, regDest);
        }
        __pParse_ref.set_okConstFactor(
            1 as crate::src::headers::sqliteInt_h::Bft as crate::src::headers::sqliteInt_h::Bft,
        );
        sqlite3ExprDelete(__pParse_ref.db, pExpr);
        crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addr);
    } else {
        p = sqlite3ExprListAppend(pParse, p, pExpr);
        if !p.is_null() {
            let pItem_0: *mut crate::src::headers::sqliteInt_h::ExprList_item = (&raw mut (*p).a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(((*p).nExpr - 1 as ::core::ffi::c_int) as isize)
                as *mut crate::src::headers::sqliteInt_h::ExprList_item;
            (*pItem_0).fg.set_reusable(
                (regDest < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint
                    as ::core::ffi::c_uint,
            );
            if regDest < 0 as ::core::ffi::c_int {
                (*pParse).nMem += 1;
                regDest = (*pParse).nMem;
            }
            (*pItem_0).u.iConstExprReg = regDest;
        }
        (*pParse).pConstExpr = p;
    }
    regDest
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]
#[inline(never)]
pub unsafe extern "C" fn sqlite3ExprNullRegisterRange(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    iReg: ::core::ffi::c_int,
    nReg: ::core::ffi::c_int,
) {
    let __pParse_ref = unsafe { &mut *pParse };
    let okConstFactor: crate::src::ext::rtree::rtree::U8_0 =
        __pParse_ref.okConstFactor() as crate::src::ext::rtree::rtree::U8_0;
    let mut t: crate::src::headers::sqliteInt_h::Expr = unsafe { ::core::mem::zeroed() };
    t.op = crate::src::parse::TK_NULLS as crate::src::ext::rtree::rtree::U8_0;
    t.y.nReg = nReg;
    __pParse_ref.set_okConstFactor(
        1 as crate::src::headers::sqliteInt_h::Bft as crate::src::headers::sqliteInt_h::Bft,
    );
    sqlite3ExprCodeRunJustOnce(pParse, &raw mut t, iReg);
    __pParse_ref.set_okConstFactor(
        okConstFactor as crate::src::headers::sqliteInt_h::Bft
            as crate::src::headers::sqliteInt_h::Bft,
    );
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCodeTemp(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    pReg: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let r2: ::core::ffi::c_int;
    pExpr = sqlite3ExprSkipCollateAndLikely(pExpr);
    if (*pParse).okConstFactor() as ::core::ffi::c_int != 0
        && !pExpr.is_null()
        && (*pExpr).op as ::core::ffi::c_int != crate::src::parse::TK_REGISTER
        && sqlite3ExprIsConstantNotJoin(pParse, pExpr) != 0
    {
        *pReg = 0 as ::core::ffi::c_int;
        r2 = sqlite3ExprCodeRunJustOnce(pParse, pExpr, -(1 as ::core::ffi::c_int));
    } else {
        let r1: ::core::ffi::c_int = sqlite3GetTempReg(pParse);
        r2 = sqlite3ExprCodeTarget(pParse, pExpr, r1);
        if r2 == r1 {
            *pReg = r1;
        } else {
            sqlite3ReleaseTempReg(pParse, r1);
            *pReg = 0 as ::core::ffi::c_int;
        }
    }
    r2
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCode(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    target: ::core::ffi::c_int,
) {
    
    if (*pParse).pVdbe.is_null() {
        return;
    }
    let inReg: ::core::ffi::c_int = sqlite3ExprCodeTarget(pParse, pExpr, target);
    if inReg != target {
        let op: crate::src::ext::rtree::rtree::U8_0;
        let pX: *mut crate::src::headers::sqliteInt_h::Expr =
            sqlite3ExprSkipCollateAndLikely(pExpr);
        if !pX.is_null()
            && ((*pX).flags
                & 0x400000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
                || (*pX).op as ::core::ffi::c_int == crate::src::parse::TK_REGISTER)
        {
            op = crate::src::headers::opcodes_h::OP_Copy as crate::src::ext::rtree::rtree::U8_0;
        } else {
            op = crate::src::headers::opcodes_h::OP_SCopy as crate::src::ext::rtree::rtree::U8_0;
        }
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            (*pParse).pVdbe,
            op as ::core::ffi::c_int,
            inReg,
            target,
        );
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCodeCopy(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    target: ::core::ffi::c_int,
) {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    pExpr = sqlite3ExprDup(db, pExpr, 0 as ::core::ffi::c_int);
    if (*db).mallocFailed == 0 {
        sqlite3ExprCode(pParse, pExpr, target);
    }
    sqlite3ExprDelete(db, pExpr);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCodeFactorable(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    target: ::core::ffi::c_int,
) {
    if (*pParse).okConstFactor() as ::core::ffi::c_int != 0
        && sqlite3ExprIsConstantNotJoin(pParse, pExpr) != 0
    {
        sqlite3ExprCodeRunJustOnce(pParse, pExpr, target);
    } else {
        sqlite3ExprCodeCopy(pParse, pExpr, target);
    };
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCodeExprList(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pList: *mut crate::src::headers::sqliteInt_h::ExprList,
    target: ::core::ffi::c_int,
    srcReg: ::core::ffi::c_int,
    mut flags: crate::src::ext::rtree::rtree::U8_0,
) -> ::core::ffi::c_int {
    let mut pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item;
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int;
    let copyOp: crate::src::ext::rtree::rtree::U8_0 =
        (if flags as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::SQLITE_ECEL_DUP != 0 {
            crate::src::headers::opcodes_h::OP_Copy
        } else {
            crate::src::headers::opcodes_h::OP_SCopy
        }) as crate::src::ext::rtree::rtree::U8_0;
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    n = (*pList).nExpr;
    if (*pParse).okConstFactor() == 0 {
        flags = (flags as ::core::ffi::c_int
            & !crate::src::headers::sqliteInt_h::SQLITE_ECEL_FACTOR)
            as crate::src::ext::rtree::rtree::U8_0;
    }
    pItem = &raw mut (*pList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item
        as *mut crate::src::headers::sqliteInt_h::ExprList_item;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        let pExpr: *mut crate::src::headers::sqliteInt_h::Expr = (*pItem).pExpr;
        if flags as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::SQLITE_ECEL_REF
            != 0 as ::core::ffi::c_int
            && {
                j = (*pItem).u.x.iOrderByCol as ::core::ffi::c_int;
                j > 0 as ::core::ffi::c_int
            }
        {
            if flags as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::SQLITE_ECEL_OMITREF
                != 0
            {
                i -= 1;
                n -= 1;
            } else {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    copyOp as ::core::ffi::c_int,
                    j + srcReg - 1 as ::core::ffi::c_int,
                    target + i,
                );
            }
        } else if flags as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::SQLITE_ECEL_FACTOR
            != 0 as ::core::ffi::c_int
            && sqlite3ExprIsConstantNotJoin(pParse, pExpr) != 0
        {
            sqlite3ExprCodeRunJustOnce(pParse, pExpr, target + i);
        } else {
            let inReg: ::core::ffi::c_int = sqlite3ExprCodeTarget(pParse, pExpr, target + i);
            if inReg != target + i {
                let pOp: *mut crate::src::src::vdbe::VdbeOp;
                if copyOp as ::core::ffi::c_int == crate::src::headers::opcodes_h::OP_Copy
                    && {
                        pOp = crate::src::src::vdbeaux::sqlite3VdbeGetLastOp(v)
                            as *mut crate::src::src::vdbe::VdbeOp;
                        (*pOp).opcode as ::core::ffi::c_int
                            == crate::src::headers::opcodes_h::OP_Copy
                    }
                    && (*pOp).p1 + (*pOp).p3 + 1 as ::core::ffi::c_int == inReg
                    && (*pOp).p2 + (*pOp).p3 + 1 as ::core::ffi::c_int == target + i
                    && (*pOp).p5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    (*pOp).p3 += 1;
                } else {
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                        v,
                        copyOp as ::core::ffi::c_int,
                        inReg,
                        target + i,
                    );
                }
            }
        }
        i += 1;
        pItem = pItem.offset(1);
    }
    n
}

unsafe extern "C" fn exprCodeBetween(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    dest: ::core::ffi::c_int,
    xJump: Option<
        unsafe extern "C" fn(
            *mut crate::src::headers::sqliteInt_h::Parse,
            *mut crate::src::headers::sqliteInt_h::Expr,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    jumpIfNull: ::core::ffi::c_int,
) {
    let mut exprAnd: crate::src::headers::sqliteInt_h::Expr = unsafe { ::core::mem::zeroed() };
    let mut compLeft: crate::src::headers::sqliteInt_h::Expr = unsafe { ::core::mem::zeroed() };
    let mut compRight: crate::src::headers::sqliteInt_h::Expr = unsafe { ::core::mem::zeroed() };
    let mut regFree1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    let pDel: *mut crate::src::headers::sqliteInt_h::Expr = sqlite3ExprDup(db, (*pExpr).pLeft, 0 as ::core::ffi::c_int);
    if (*db).mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        exprAnd.op = crate::src::parse::TK_AND as crate::src::ext::rtree::rtree::U8_0;
        exprAnd.pLeft = &raw mut compLeft;
        exprAnd.pRight = &raw mut compRight;
        compLeft.pRight = (*(&raw mut (*(*pExpr).x.pList).a
            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(0_isize))
        .pExpr;
        let _ = compLeft.pRight;
        compRight.pRight = (*(&raw mut (*(*pExpr).x.pList).a
            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(1_isize))
        .pExpr;
        let _ = compRight.pRight;
        sqlite3ExprToRegister(pDel, exprCodeVector(pParse, pDel, &raw mut regFree1));
        if xJump.is_some() {
            xJump.expect("non-null function pointer")(pParse, &raw mut exprAnd, dest, jumpIfNull);
        } else {
            (*pDel).flags |= crate::src::headers::sqliteInt_h::EP_OuterON
                as crate::src::ext::rtree::rtree::U32_0;
            sqlite3ExprCodeTarget(pParse, &raw mut exprAnd, dest);
        }
        sqlite3ReleaseTempReg(pParse, regFree1);
    }
    sqlite3ExprDelete(db, pDel);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprIfTrue(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    dest: ::core::ffi::c_int,
    mut jumpIfNull: ::core::ffi::c_int,
) {
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    let mut op: ::core::ffi::c_int;
    let mut regFree1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut regFree2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut r1: ::core::ffi::c_int = 0;
    let mut r2: ::core::ffi::c_int = 0;
    if v.is_null() {
        return;
    }
    if pExpr.is_null() {
        return;
    }
    op = (*pExpr).op as ::core::ffi::c_int;
    let mut current_block_108: u64;
    match op {
        crate::src::parse::TK_AND | crate::src::parse::TK_OR => {
            let pAlt: *mut crate::src::headers::sqliteInt_h::Expr =
                sqlite3ExprSimplifiedAndOr(pExpr);
            if pAlt != pExpr {
                sqlite3ExprIfTrue(pParse, pAlt, dest, jumpIfNull);
            } else {
                let pFirst: *mut crate::src::headers::sqliteInt_h::Expr;
                let pSecond: *mut crate::src::headers::sqliteInt_h::Expr;
                if exprEvalRhsFirst(pExpr) != 0 {
                    pFirst = (*pExpr).pRight;
                    pSecond = (*pExpr).pLeft;
                } else {
                    pFirst = (*pExpr).pLeft;
                    pSecond = (*pExpr).pRight;
                }
                if op == crate::src::parse::TK_AND {
                    let d2: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    );
                    sqlite3ExprIfFalse(
                        pParse,
                        pFirst,
                        d2,
                        jumpIfNull ^ crate::src::headers::sqliteInt_h::SQLITE_JUMPIFNULL,
                    );
                    sqlite3ExprIfTrue(pParse, pSecond, dest, jumpIfNull);
                    crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, d2);
                } else {
                    sqlite3ExprIfTrue(pParse, pFirst, dest, jumpIfNull);
                    sqlite3ExprIfTrue(pParse, pSecond, dest, jumpIfNull);
                }
            }
            current_block_108 = 10369920510435091891;
        }
        crate::src::parse::TK_NOT_1 => {
            sqlite3ExprIfFalse(pParse, (*pExpr).pLeft, dest, jumpIfNull);
            current_block_108 = 10369920510435091891;
        }
        crate::src::parse::TK_TRUTH => {
            
            
            let isNot: ::core::ffi::c_int = ((*pExpr).op2 as ::core::ffi::c_int == crate::src::parse::TK_ISNOT_1)
                as ::core::ffi::c_int;
            let isTrue: ::core::ffi::c_int = sqlite3ExprTruthValue((*pExpr).pRight);
            if isTrue ^ isNot != 0 {
                sqlite3ExprIfTrue(
                    pParse,
                    (*pExpr).pLeft,
                    dest,
                    if isNot != 0 {
                        crate::src::headers::sqliteInt_h::SQLITE_JUMPIFNULL
                    } else {
                        0 as ::core::ffi::c_int
                    },
                );
            } else {
                sqlite3ExprIfFalse(
                    pParse,
                    (*pExpr).pLeft,
                    dest,
                    if isNot != 0 {
                        crate::src::headers::sqliteInt_h::SQLITE_JUMPIFNULL
                    } else {
                        0 as ::core::ffi::c_int
                    },
                );
            }
            current_block_108 = 10369920510435091891;
        }
        crate::src::parse::TK_IS | crate::src::parse::TK_ISNOT_1 => {
            op = if op == crate::src::parse::TK_IS {
                crate::src::parse::TK_EQ_1
            } else {
                crate::src::parse::TK_NE
            };
            jumpIfNull = crate::src::headers::sqliteInt_h::SQLITE_NULLEQ;
            current_block_108 = 15090052786889560393;
        }
        crate::src::parse::TK_LT
        | crate::src::parse::TK_LE
        | crate::src::parse::TK_GT
        | crate::src::parse::TK_GE
        | crate::src::parse::TK_NE
        | crate::src::parse::TK_EQ_1 => {
            current_block_108 = 15090052786889560393;
        }
        crate::src::parse::TK_ISNULL_1 | crate::src::parse::TK_NOTNULL_1 => {
            r1 = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, &raw mut regFree1);
            if regFree1 != 0 {
                crate::src::src::vdbeaux::sqlite3VdbeTypeofColumn(v, r1);
            }
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(v, op, r1, dest);
            current_block_108 = 10369920510435091891;
        }
        crate::src::parse::TK_BETWEEN_1 => {
            exprCodeBetween(
                pParse,
                pExpr,
                dest,
                Some(
                    sqlite3ExprIfTrue
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqliteInt_h::Parse,
                            *mut crate::src::headers::sqliteInt_h::Expr,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
                jumpIfNull,
            );
            current_block_108 = 10369920510435091891;
        }
        crate::src::parse::TK_IN => {
            let destIfFalse: ::core::ffi::c_int =
                crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                );
            let destIfNull: ::core::ffi::c_int =
                if jumpIfNull != 0 { dest } else { destIfFalse };
            sqlite3ExprCodeIN(pParse, pExpr, destIfFalse, destIfNull);
            crate::src::src::vdbeaux::sqlite3VdbeGoto(v, dest);
            crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, destIfFalse);
            current_block_108 = 10369920510435091891;
        }
        _ => {
            current_block_108 = 12729351628741572443;
        }
    }
    if current_block_108 == 15090052786889560393 {
        let addrIsNull: ::core::ffi::c_int;
        if sqlite3ExprIsVector((*pExpr).pLeft) != 0 {
            current_block_108 = 12729351628741572443;
        } else {
            let __pExpr_ref = unsafe { &*pExpr };
            if __pExpr_ref.flags
                & 0x400000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
                && jumpIfNull != crate::src::headers::sqliteInt_h::SQLITE_NULLEQ
            {
                addrIsNull = exprComputeOperands(
                    pParse,
                    pExpr,
                    &raw mut r1,
                    &raw mut r2,
                    &raw mut regFree1,
                    &raw mut regFree2,
                );
            } else {
                r1 = sqlite3ExprCodeTemp(pParse, __pExpr_ref.pLeft, &raw mut regFree1);
                r2 = sqlite3ExprCodeTemp(pParse, __pExpr_ref.pRight, &raw mut regFree2);
                addrIsNull = 0 as ::core::ffi::c_int;
            }
            codeCompare(
                pParse,
                __pExpr_ref.pLeft,
                __pExpr_ref.pRight,
                op,
                r1,
                r2,
                dest,
                jumpIfNull,
                (__pExpr_ref.flags
                    & 0x400 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                    != 0 as crate::src::ext::rtree::rtree::U32_0)
                    as ::core::ffi::c_int,
            );
            if addrIsNull != 0 {
                if jumpIfNull != 0 {
                    crate::src::src::vdbeaux::sqlite3VdbeChangeP2(v, addrIsNull, dest);
                } else {
                    crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrIsNull);
                }
            }
            current_block_108 = 10369920510435091891;
        }
    }
    if current_block_108 == 12729351628741572443 {
        if (*pExpr).flags
            & (crate::src::headers::sqliteInt_h::EP_OuterON
                | crate::src::headers::sqliteInt_h::EP_IsTrue)
                as crate::src::ext::rtree::rtree::U32_0
            == crate::src::headers::sqliteInt_h::EP_IsTrue
                as crate::src::ext::rtree::rtree::U32_0
        {
            crate::src::src::vdbeaux::sqlite3VdbeGoto(v, dest);
        } else if ((*pExpr).flags
            & (crate::src::headers::sqliteInt_h::EP_OuterON
                | crate::src::headers::sqliteInt_h::EP_IsFalse)
                as crate::src::ext::rtree::rtree::U32_0 != crate::src::headers::sqliteInt_h::EP_IsFalse
                as crate::src::ext::rtree::rtree::U32_0)
        {
            r1 = sqlite3ExprCodeTemp(pParse, pExpr, &raw mut regFree1);
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_If,
                r1,
                dest,
                (jumpIfNull != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            );
        }
    }
    sqlite3ReleaseTempReg(pParse, regFree1);
    sqlite3ReleaseTempReg(pParse, regFree2);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprIfFalse(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    dest: ::core::ffi::c_int,
    mut jumpIfNull: ::core::ffi::c_int,
) {
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    let mut op: ::core::ffi::c_int;
    let mut regFree1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut regFree2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut r1: ::core::ffi::c_int = 0;
    let mut r2: ::core::ffi::c_int = 0;
    if v.is_null() {
        return;
    }
    if pExpr.is_null() {
        return;
    }
    op = (((*pExpr).op as ::core::ffi::c_int
        + (crate::src::parse::TK_ISNULL_1 & 1 as ::core::ffi::c_int)) ^ 1 as ::core::ffi::c_int)
        - (crate::src::parse::TK_ISNULL_1 & 1 as ::core::ffi::c_int);
    let mut current_block_117: u64;
    match (*pExpr).op as ::core::ffi::c_int {
        crate::src::parse::TK_AND | crate::src::parse::TK_OR => {
            let pAlt: *mut crate::src::headers::sqliteInt_h::Expr =
                sqlite3ExprSimplifiedAndOr(pExpr);
            if pAlt != pExpr {
                sqlite3ExprIfFalse(pParse, pAlt, dest, jumpIfNull);
            } else {
                let pFirst: *mut crate::src::headers::sqliteInt_h::Expr;
                let pSecond: *mut crate::src::headers::sqliteInt_h::Expr;
                if exprEvalRhsFirst(pExpr) != 0 {
                    pFirst = (*pExpr).pRight;
                    pSecond = (*pExpr).pLeft;
                } else {
                    pFirst = (*pExpr).pLeft;
                    pSecond = (*pExpr).pRight;
                }
                if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_AND {
                    sqlite3ExprIfFalse(pParse, pFirst, dest, jumpIfNull);
                    sqlite3ExprIfFalse(pParse, pSecond, dest, jumpIfNull);
                } else {
                    let d2: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    );
                    sqlite3ExprIfTrue(
                        pParse,
                        pFirst,
                        d2,
                        jumpIfNull ^ crate::src::headers::sqliteInt_h::SQLITE_JUMPIFNULL,
                    );
                    sqlite3ExprIfFalse(pParse, pSecond, dest, jumpIfNull);
                    crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, d2);
                }
            }
            current_block_117 = 12369290732426379360;
        }
        crate::src::parse::TK_NOT_1 => {
            sqlite3ExprIfTrue(pParse, (*pExpr).pLeft, dest, jumpIfNull);
            current_block_117 = 12369290732426379360;
        }
        crate::src::parse::TK_TRUTH => {
            
            
            let isNot: ::core::ffi::c_int = ((*pExpr).op2 as ::core::ffi::c_int == crate::src::parse::TK_ISNOT_1)
                as ::core::ffi::c_int;
            let isTrue: ::core::ffi::c_int = sqlite3ExprTruthValue((*pExpr).pRight);
            if isTrue ^ isNot != 0 {
                sqlite3ExprIfFalse(
                    pParse,
                    (*pExpr).pLeft,
                    dest,
                    if isNot != 0 {
                        0 as ::core::ffi::c_int
                    } else {
                        crate::src::headers::sqliteInt_h::SQLITE_JUMPIFNULL
                    },
                );
            } else {
                sqlite3ExprIfTrue(
                    pParse,
                    (*pExpr).pLeft,
                    dest,
                    if isNot != 0 {
                        0 as ::core::ffi::c_int
                    } else {
                        crate::src::headers::sqliteInt_h::SQLITE_JUMPIFNULL
                    },
                );
            }
            current_block_117 = 12369290732426379360;
        }
        crate::src::parse::TK_IS | crate::src::parse::TK_ISNOT_1 => {
            op = if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_IS {
                crate::src::parse::TK_NE
            } else {
                crate::src::parse::TK_EQ_1
            };
            jumpIfNull = crate::src::headers::sqliteInt_h::SQLITE_NULLEQ;
            current_block_117 = 15004371738079956865;
        }
        crate::src::parse::TK_LT
        | crate::src::parse::TK_LE
        | crate::src::parse::TK_GT
        | crate::src::parse::TK_GE
        | crate::src::parse::TK_NE
        | crate::src::parse::TK_EQ_1 => {
            current_block_117 = 15004371738079956865;
        }
        crate::src::parse::TK_ISNULL_1 | crate::src::parse::TK_NOTNULL_1 => {
            r1 = sqlite3ExprCodeTemp(pParse, (*pExpr).pLeft, &raw mut regFree1);
            if regFree1 != 0 {
                crate::src::src::vdbeaux::sqlite3VdbeTypeofColumn(v, r1);
            }
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(v, op, r1, dest);
            current_block_117 = 12369290732426379360;
        }
        crate::src::parse::TK_BETWEEN_1 => {
            exprCodeBetween(
                pParse,
                pExpr,
                dest,
                Some(
                    sqlite3ExprIfFalse
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqliteInt_h::Parse,
                            *mut crate::src::headers::sqliteInt_h::Expr,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
                jumpIfNull,
            );
            current_block_117 = 12369290732426379360;
        }
        crate::src::parse::TK_IN => {
            if jumpIfNull != 0 {
                sqlite3ExprCodeIN(pParse, pExpr, dest, dest);
            } else {
                let destIfNull: ::core::ffi::c_int =
                    crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    );
                sqlite3ExprCodeIN(pParse, pExpr, dest, destIfNull);
                crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, destIfNull);
            }
            current_block_117 = 12369290732426379360;
        }
        _ => {
            current_block_117 = 12367036871764426053;
        }
    }
    if current_block_117 == 15004371738079956865 {
        let addrIsNull: ::core::ffi::c_int;
        if sqlite3ExprIsVector((*pExpr).pLeft) != 0 {
            current_block_117 = 12367036871764426053;
        } else {
            let __pExpr_ref = unsafe { &*pExpr };
            if __pExpr_ref.flags
                & 0x400000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
                && jumpIfNull != crate::src::headers::sqliteInt_h::SQLITE_NULLEQ
            {
                addrIsNull = exprComputeOperands(
                    pParse,
                    pExpr,
                    &raw mut r1,
                    &raw mut r2,
                    &raw mut regFree1,
                    &raw mut regFree2,
                );
            } else {
                r1 = sqlite3ExprCodeTemp(pParse, __pExpr_ref.pLeft, &raw mut regFree1);
                r2 = sqlite3ExprCodeTemp(pParse, __pExpr_ref.pRight, &raw mut regFree2);
                addrIsNull = 0 as ::core::ffi::c_int;
            }
            codeCompare(
                pParse,
                __pExpr_ref.pLeft,
                __pExpr_ref.pRight,
                op,
                r1,
                r2,
                dest,
                jumpIfNull,
                (__pExpr_ref.flags
                    & 0x400 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                    != 0 as crate::src::ext::rtree::rtree::U32_0)
                    as ::core::ffi::c_int,
            );
            if addrIsNull != 0 {
                if jumpIfNull != 0 {
                    crate::src::src::vdbeaux::sqlite3VdbeChangeP2(v, addrIsNull, dest);
                } else {
                    crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrIsNull);
                }
            }
            current_block_117 = 12369290732426379360;
        }
    }
    if current_block_117 == 12367036871764426053 {
        if (*pExpr).flags
            & (crate::src::headers::sqliteInt_h::EP_OuterON
                | crate::src::headers::sqliteInt_h::EP_IsFalse)
                as crate::src::ext::rtree::rtree::U32_0
            == crate::src::headers::sqliteInt_h::EP_IsFalse
                as crate::src::ext::rtree::rtree::U32_0
        {
            crate::src::src::vdbeaux::sqlite3VdbeGoto(v, dest);
        } else if ((*pExpr).flags
            & (crate::src::headers::sqliteInt_h::EP_OuterON
                | crate::src::headers::sqliteInt_h::EP_IsTrue)
                as crate::src::ext::rtree::rtree::U32_0 != crate::src::headers::sqliteInt_h::EP_IsTrue
                as crate::src::ext::rtree::rtree::U32_0)
        {
            r1 = sqlite3ExprCodeTemp(pParse, pExpr, &raw mut regFree1);
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_IfNot,
                r1,
                dest,
                (jumpIfNull != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            );
        }
    }
    sqlite3ReleaseTempReg(pParse, regFree1);
    sqlite3ReleaseTempReg(pParse, regFree2);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprIfFalseDup(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    dest: ::core::ffi::c_int,
    jumpIfNull: ::core::ffi::c_int,
) {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    let pCopy: *mut crate::src::headers::sqliteInt_h::Expr =
        sqlite3ExprDup(db, pExpr, 0 as ::core::ffi::c_int);
    if (*db).mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        sqlite3ExprIfFalse(pParse, pCopy, dest, jumpIfNull);
    }
    sqlite3ExprDelete(db, pCopy);
}
#[inline(never)]
unsafe extern "C" fn exprCompareVariable(
    pParse: *const crate::src::headers::sqliteInt_h::Parse,
    pVar: *const crate::src::headers::sqliteInt_h::Expr,
    pExpr: *const crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    let iVar: ::core::ffi::c_int;
    let pL: *mut crate::src::headers::vdbeInt_h::sqlite3_value;
    let mut pR: *mut crate::src::headers::vdbeInt_h::sqlite3_value =
        ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>();
    if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_VARIABLE
        && (*pVar).iColumn as ::core::ffi::c_int == (*pExpr).iColumn as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*(*pParse).db).flags
        & crate::src::headers::sqliteInt_h::SQLITE_EnableQPSG
            as crate::src::ext::rtree::rtree::U64_0
        != 0 as crate::src::ext::rtree::rtree::U64_0
    {
        return 2 as ::core::ffi::c_int;
    }
    crate::src::src::vdbemem::sqlite3ValueFromExpr(
        (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        pExpr as *const crate::src::headers::sqliteInt_h::Expr,
        crate::src::headers::sqlite3_h::SQLITE_UTF8 as crate::src::ext::rtree::rtree::U8_0,
        crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB as crate::src::ext::rtree::rtree::U8_0,
        &raw mut pR,
    );
    if !pR.is_null() {
        iVar = (*pVar).iColumn as ::core::ffi::c_int;
        crate::src::src::vdbeaux::sqlite3VdbeSetVarmask((*pParse).pVdbe, iVar);
        pL = crate::src::src::vdbeaux::sqlite3VdbeGetBoundValue(
            (*pParse).pReprepare,
            iVar,
            crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB
                as crate::src::ext::rtree::rtree::U8_0,
        );
        if !pL.is_null() {
            if crate::src::src::vdbeapi::sqlite3_value_type(pL)
                == crate::src::headers::sqlite3_h::SQLITE_TEXT
            {
                crate::src::src::vdbeapi::sqlite3_value_text(pL);
            }
            res = if crate::src::src::vdbeaux::sqlite3MemCompare(
                pL,
                pR,
                ::core::ptr::null::<crate::src::headers::sqliteInt_h::CollSeq>()
                    as *const crate::src::headers::sqliteInt_h::CollSeq,
            ) != 0
            {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
        }
        crate::src::src::vdbemem::sqlite3ValueFree(pR);
        crate::src::src::vdbemem::sqlite3ValueFree(pL);
    }
    res
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCompare(
    pParse: *const crate::src::headers::sqliteInt_h::Parse,
    pA: *const crate::src::headers::sqliteInt_h::Expr,
    pB: *const crate::src::headers::sqliteInt_h::Expr,
    iTab: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    if pA.is_null() || pB.is_null() {
        return if pB == pA {
            0 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        };
    }
    if !pParse.is_null() && (*pA).op as ::core::ffi::c_int == crate::src::parse::TK_VARIABLE {
        return exprCompareVariable(pParse, pA, pB);
    }
    let combinedFlags: crate::src::ext::rtree::rtree::U32_0 = (*pA).flags | (*pB).flags;
    if combinedFlags
        & crate::src::headers::sqliteInt_h::EP_IntValue as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        if (*pA).flags
            & (*pB).flags
            & crate::src::headers::sqliteInt_h::EP_IntValue as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
            && (*pA).u.iValue == (*pB).u.iValue
        {
            return 0 as ::core::ffi::c_int;
        }
        return 2 as ::core::ffi::c_int;
    }
    if (*pA).op as ::core::ffi::c_int != (*pB).op as ::core::ffi::c_int
        || (*pA).op as ::core::ffi::c_int == crate::src::parse::TK_RAISE
    {
        if (*pA).op as ::core::ffi::c_int == crate::src::parse::TK_COLLATE
            && sqlite3ExprCompare(pParse, (*pA).pLeft, pB, iTab) < 2 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        if (*pB).op as ::core::ffi::c_int == crate::src::parse::TK_COLLATE
            && sqlite3ExprCompare(pParse, pA, (*pB).pLeft, iTab) < 2 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        if (*pA).op as ::core::ffi::c_int == crate::src::parse::TK_AGG_COLUMN
            && (*pB).op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN
            && (*pB).iTable < 0 as ::core::ffi::c_int
            && (*pA).iTable == iTab
        {
        } else {
            return 2 as ::core::ffi::c_int;
        }
    }
    if !(*pA).u.zToken.is_null() {
        if (*pA).op as ::core::ffi::c_int == crate::src::parse::TK_FUNCTION
            || (*pA).op as ::core::ffi::c_int == crate::src::parse::TK_AGG_FUNCTION
        {
            if crate::src::src::util::sqlite3StrICmp((*pA).u.zToken, (*pB).u.zToken)
                != 0 as ::core::ffi::c_int
            {
                return 2 as ::core::ffi::c_int;
            }
            if ((*pA).flags
                & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0) as ::core::ffi::c_int
                != ((*pB).flags
                    & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                    != 0 as crate::src::ext::rtree::rtree::U32_0)
                    as ::core::ffi::c_int
            {
                return 2 as ::core::ffi::c_int;
            }
            if (*pA).flags & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
                && crate::src::src::window::sqlite3WindowCompare(
                    pParse as *const crate::src::headers::sqliteInt_h::Parse,
                    (*pA).y.pWin as *const crate::src::headers::sqliteInt_h::Window,
                    (*pB).y.pWin as *const crate::src::headers::sqliteInt_h::Window,
                    1 as ::core::ffi::c_int,
                ) != 0 as ::core::ffi::c_int
                {
                    return 2 as ::core::ffi::c_int;
                }
        } else if (*pA).op as ::core::ffi::c_int == crate::src::parse::TK_NULL {
            return 0 as ::core::ffi::c_int;
        } else if (*pA).op as ::core::ffi::c_int == crate::src::parse::TK_COLLATE {
            if crate::src::src::util::sqlite3_stricmp((*pA).u.zToken, (*pB).u.zToken)
                != 0 as ::core::ffi::c_int
            {
                return 2 as ::core::ffi::c_int;
            }
        } else if !(*pB).u.zToken.is_null()
            && (*pA).op as ::core::ffi::c_int != crate::src::parse::TK_COLUMN
            && (*pA).op as ::core::ffi::c_int != crate::src::parse::TK_AGG_COLUMN
            && ::libc::strcmp((*pA).u.zToken, (*pB).u.zToken) != 0 as ::core::ffi::c_int
        {
            return 2 as ::core::ffi::c_int;
        }
    }
    if (*pA).flags
        & (crate::src::headers::sqliteInt_h::EP_Distinct
            | crate::src::headers::sqliteInt_h::EP_Commuted)
            as crate::src::ext::rtree::rtree::U32_0
        != (*pB).flags
            & (crate::src::headers::sqliteInt_h::EP_Distinct
                | crate::src::headers::sqliteInt_h::EP_Commuted)
                as crate::src::ext::rtree::rtree::U32_0
    {
        return 2 as ::core::ffi::c_int;
    }
    if combinedFlags & 0x10000 as crate::src::ext::rtree::rtree::U32_0
        == 0 as crate::src::ext::rtree::rtree::U32_0
    {
        if combinedFlags
            & crate::src::headers::sqliteInt_h::EP_xIsSelect as crate::src::ext::rtree::rtree::U32_0
            != 0
        {
            return 2 as ::core::ffi::c_int;
        }
        if combinedFlags
            & crate::src::headers::sqliteInt_h::EP_FixedCol as crate::src::ext::rtree::rtree::U32_0
            == 0 as crate::src::ext::rtree::rtree::U32_0
            && sqlite3ExprCompare(pParse, (*pA).pLeft, (*pB).pLeft, iTab) != 0
        {
            return 2 as ::core::ffi::c_int;
        }
        if sqlite3ExprCompare(pParse, (*pA).pRight, (*pB).pRight, iTab) != 0 {
            return 2 as ::core::ffi::c_int;
        }
        if sqlite3ExprListCompare((*pA).x.pList, (*pB).x.pList, iTab) != 0 {
            return 2 as ::core::ffi::c_int;
        }
        if (*pA).op as ::core::ffi::c_int != crate::src::parse::TK_STRING
            && (*pA).op as ::core::ffi::c_int != crate::src::parse::TK_TRUEFALSE
            && combinedFlags & 0x4000 as crate::src::ext::rtree::rtree::U32_0
                == 0 as crate::src::ext::rtree::rtree::U32_0
        {
            if (*pA).iColumn as ::core::ffi::c_int != (*pB).iColumn as ::core::ffi::c_int {
                return 2 as ::core::ffi::c_int;
            }
            if (*pA).op2 as ::core::ffi::c_int != (*pB).op2 as ::core::ffi::c_int
                && (*pA).op as ::core::ffi::c_int == crate::src::parse::TK_TRUTH
            {
                return 2 as ::core::ffi::c_int;
            }
            if (*pA).op as ::core::ffi::c_int != crate::src::parse::TK_IN
                && (*pA).iTable != (*pB).iTable
                && (*pA).iTable != iTab
            {
                return 2 as ::core::ffi::c_int;
            }
        }
    }
    0 as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprListCompare(
    pA: *const crate::src::headers::sqliteInt_h::ExprList,
    pB: *const crate::src::headers::sqliteInt_h::ExprList,
    iTab: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    if pA.is_null() && pB.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if pA.is_null() || pB.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    if (*pA).nExpr != (*pB).nExpr {
        return 1 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*pA).nExpr {
        
        let pExprA: *mut crate::src::headers::sqliteInt_h::Expr = (*(&raw const (*pA).a
            as *const crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(i as isize))
        .pExpr;
        let pExprB: *mut crate::src::headers::sqliteInt_h::Expr = (*(&raw const (*pB).a
            as *const crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(i as isize))
        .pExpr;
        if (*(&raw const (*pA).a as *const crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(i as isize))
        .fg
        .sortFlags as ::core::ffi::c_int
            != (*(&raw const (*pB).a as *const crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(i as isize))
            .fg
            .sortFlags as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        let res: ::core::ffi::c_int = sqlite3ExprCompare(
            ::core::ptr::null::<crate::src::headers::sqliteInt_h::Parse>(),
            pExprA,
            pExprB,
            iTab,
        );
        if res != 0 {
            return res;
        }
        i += 1;
    }
    0 as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCompareSkip(
    pA: *mut crate::src::headers::sqliteInt_h::Expr,
    pB: *mut crate::src::headers::sqliteInt_h::Expr,
    iTab: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    sqlite3ExprCompare(
        ::core::ptr::null::<crate::src::headers::sqliteInt_h::Parse>(),
        sqlite3ExprSkipCollate(pA),
        sqlite3ExprSkipCollate(pB),
        iTab,
    )
}

unsafe extern "C" fn exprImpliesNotNull(
    pParse: *const crate::src::headers::sqliteInt_h::Parse,
    p: *const crate::src::headers::sqliteInt_h::Expr,
    pNN: *const crate::src::headers::sqliteInt_h::Expr,
    iTab: ::core::ffi::c_int,
    mut seenNot: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sqlite3ExprCompare(pParse, p, pNN, iTab) == 0 as ::core::ffi::c_int {
        return ((*pNN).op as ::core::ffi::c_int != crate::src::parse::TK_NULL)
            as ::core::ffi::c_int;
    }
    's_167: {
        let current_block_36: u64;
        match (*p).op as ::core::ffi::c_int {
            crate::src::parse::TK_IN => {
                if seenNot != 0
                    && (*p).flags
                        & 0x1000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                        != 0 as crate::src::ext::rtree::rtree::U32_0
                {
                    return 0 as ::core::ffi::c_int;
                }
                return exprImpliesNotNull(pParse, (*p).pLeft, pNN, iTab, 1 as ::core::ffi::c_int);
            }
            crate::src::parse::TK_BETWEEN_1 => {
                
                let pList: *mut crate::src::headers::sqliteInt_h::ExprList = (*p).x.pList;
                if seenNot != 0 {
                    return 0 as ::core::ffi::c_int;
                }
                if exprImpliesNotNull(
                    pParse,
                    (*(&raw mut (*pList).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(0_isize))
                    .pExpr,
                    pNN,
                    iTab,
                    1 as ::core::ffi::c_int,
                ) != 0
                    || exprImpliesNotNull(
                        pParse,
                        (*(&raw mut (*pList).a
                            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                            .offset(1_isize))
                        .pExpr,
                        pNN,
                        iTab,
                        1 as ::core::ffi::c_int,
                    ) != 0
                {
                    return 1 as ::core::ffi::c_int;
                }
                return exprImpliesNotNull(pParse, (*p).pLeft, pNN, iTab, 1 as ::core::ffi::c_int);
            }
            crate::src::parse::TK_EQ_1
            | crate::src::parse::TK_NE
            | crate::src::parse::TK_LT
            | crate::src::parse::TK_LE
            | crate::src::parse::TK_GT
            | crate::src::parse::TK_GE
            | crate::src::parse::TK_PLUS_1
            | crate::src::parse::TK_MINUS
            | crate::src::parse::TK_BITOR
            | crate::src::parse::TK_LSHIFT
            | crate::src::parse::TK_RSHIFT
            | crate::src::parse::TK_CONCAT => {
                seenNot = 1 as ::core::ffi::c_int;
                current_block_36 = 17964801197539127570;
            }
            crate::src::parse::TK_STAR
            | crate::src::parse::TK_REM
            | crate::src::parse::TK_BITAND
            | crate::src::parse::TK_SLASH => {
                current_block_36 = 17964801197539127570;
            }
            crate::src::parse::TK_SPAN_1
            | crate::src::parse::TK_COLLATE
            | crate::src::parse::TK_UPLUS
            | crate::src::parse::TK_UMINUS => {
                current_block_36 = 3712350157689843324;
            }
            crate::src::parse::TK_TRUTH => {
                if seenNot != 0 {
                    return 0 as ::core::ffi::c_int;
                }
                if (*p).op2 as ::core::ffi::c_int != crate::src::parse::TK_IS {
                    return 0 as ::core::ffi::c_int;
                }
                return exprImpliesNotNull(pParse, (*p).pLeft, pNN, iTab, 1 as ::core::ffi::c_int);
            }
            crate::src::parse::TK_BITNOT | crate::src::parse::TK_NOT_1 => {
                return exprImpliesNotNull(pParse, (*p).pLeft, pNN, iTab, 1 as ::core::ffi::c_int);
            }
            _ => {
                break 's_167;
            }
        }
        match current_block_36 {
            17964801197539127570
                if exprImpliesNotNull(pParse, (*p).pRight, pNN, iTab, seenNot) != 0 => {
                    return 1 as ::core::ffi::c_int;
                }
            _ => {}
        }
        return exprImpliesNotNull(pParse, (*p).pLeft, pNN, iTab, seenNot);
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn sqlite3ExprIsNotTrue(
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let mut v: ::core::ffi::c_int;
    if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_NULL {
        return 1 as ::core::ffi::c_int;
    }
    if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_TRUEFALSE
        && sqlite3ExprTruthValue(pExpr) == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    v = 1 as ::core::ffi::c_int;
    if sqlite3ExprIsInteger(
        pExpr,
        &raw mut v,
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Parse>(),
    ) != 0
        && v == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn sqlite3ExprIsIIF(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pExpr: *const crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    
    if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_FUNCTION {
        let z: *const ::core::ffi::c_char = (*pExpr).u.zToken;
        
        if *z.offset(0_isize) as ::core::ffi::c_int != 'i' as i32
            && *z.offset(0_isize) as ::core::ffi::c_int != 'I' as i32
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*pExpr).x.pList.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        let pDef: *mut crate::src::headers::sqliteInt_h::FuncDef = crate::src::src::callback::sqlite3FindFunction(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            z,
            (*(*pExpr).x.pList).nExpr,
            (*db).enc,
            0 as crate::src::ext::rtree::rtree::U8_0,
        ) as *mut crate::src::headers::sqliteInt_h::FuncDef;
        if pDef.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*pDef).funcFlags
            & crate::src::headers::sqliteInt_h::SQLITE_FUNC_INLINE
                as crate::src::ext::rtree::rtree::U32_0
            == 0 as crate::src::ext::rtree::rtree::U32_0
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*pDef).pUserData as crate::src::headers::stdlib::IntptrT as ::core::ffi::c_int
            != crate::src::headers::sqliteInt_h::INLINEFUNC_iif
        {
            return 0 as ::core::ffi::c_int;
        }
    } else if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_CASE_1 {
        if !(*pExpr).pLeft.is_null() {
            return 0 as ::core::ffi::c_int;
        }
    } else {
        return 0 as ::core::ffi::c_int;
    }
    let pList: *mut crate::src::headers::sqliteInt_h::ExprList = (*pExpr).x.pList;
    if (*pList).nExpr == 2 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if (*pList).nExpr == 3 as ::core::ffi::c_int
        && sqlite3ExprIsNotTrue(
            (*(&raw mut (*pList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(2_isize))
            .pExpr,
        ) != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprImpliesExpr(
    pParse: *const crate::src::headers::sqliteInt_h::Parse,
    pE1: *const crate::src::headers::sqliteInt_h::Expr,
    pE2: *const crate::src::headers::sqliteInt_h::Expr,
    iTab: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if sqlite3ExprCompare(pParse, pE1, pE2, iTab) == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    let __pE2_ref = unsafe { &*pE2 };
    if __pE2_ref.op as ::core::ffi::c_int == crate::src::parse::TK_OR
        && (sqlite3ExprImpliesExpr(pParse, pE1, __pE2_ref.pLeft, iTab) != 0
            || sqlite3ExprImpliesExpr(pParse, pE1, __pE2_ref.pRight, iTab) != 0)
    {
        return 1 as ::core::ffi::c_int;
    }
    if __pE2_ref.op as ::core::ffi::c_int == crate::src::parse::TK_NOTNULL_1
        && exprImpliesNotNull(pParse, pE1, __pE2_ref.pLeft, iTab, 0 as ::core::ffi::c_int) != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    if sqlite3ExprIsIIF((*pParse).db, pE1) != 0 {
        return sqlite3ExprImpliesExpr(
            pParse,
            (*(&raw mut (*(*pE1).x.pList).a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(0_isize))
            .pExpr,
            pE2,
            iTab,
        );
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn bothImplyNotNullRow(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pE1: *mut crate::src::headers::sqliteInt_h::Expr,
    pE2: *mut crate::src::headers::sqliteInt_h::Expr,
) {
    if (*pWalker).eCode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        crate::src::src::walker::sqlite3WalkExpr(
            pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
            pE1 as *mut crate::src::headers::sqliteInt_h::Expr,
        );
        if (*pWalker).eCode != 0 {
            (*pWalker).eCode = 0 as crate::src::fts5::U16_0;
            crate::src::src::walker::sqlite3WalkExpr(
                pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
                pE2 as *mut crate::src::headers::sqliteInt_h::Expr,
            );
        }
    }
}

unsafe extern "C" fn impliesNotNullRow(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let __pExpr_ref = unsafe { &mut *pExpr };
    if __pExpr_ref.flags & 0x1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return crate::src::headers::sqliteInt_h::WRC_Prune;
    }
    if __pExpr_ref.flags & 0x2 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
        && (*pWalker).mWFlags as ::core::ffi::c_int != 0
    {
        return crate::src::headers::sqliteInt_h::WRC_Prune;
    }
    match __pExpr_ref.op as ::core::ffi::c_int {
        crate::src::parse::TK_ISNOT_1
        | crate::src::parse::TK_ISNULL_1
        | crate::src::parse::TK_NOTNULL_1
        | crate::src::parse::TK_IS
        | crate::src::parse::TK_VECTOR
        | crate::src::parse::TK_FUNCTION
        | crate::src::parse::TK_TRUTH
        | crate::src::parse::TK_CASE_1 => return crate::src::headers::sqliteInt_h::WRC_Prune,
        crate::src::parse::TK_COLUMN => {
            if (*pWalker).u.iCur == __pExpr_ref.iTable {
                (*pWalker).eCode = 1 as crate::src::fts5::U16_0;
                return crate::src::headers::sqliteInt_h::WRC_Abort;
            }
            return crate::src::headers::sqliteInt_h::WRC_Prune;
        }
        crate::src::parse::TK_OR | crate::src::parse::TK_AND => {
            bothImplyNotNullRow(pWalker, __pExpr_ref.pLeft, __pExpr_ref.pRight);
            return crate::src::headers::sqliteInt_h::WRC_Prune;
        }
        crate::src::parse::TK_IN => {
            if __pExpr_ref.flags
                & crate::src::headers::sqliteInt_h::EP_xIsSelect
                    as crate::src::ext::rtree::rtree::U32_0
                == 0 as crate::src::ext::rtree::rtree::U32_0
                && (*__pExpr_ref.x.pList).nExpr > 0 as ::core::ffi::c_int
            {
                crate::src::src::walker::sqlite3WalkExpr(
                    pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
                    __pExpr_ref.pLeft as *mut crate::src::headers::sqliteInt_h::Expr,
                );
            }
            return crate::src::headers::sqliteInt_h::WRC_Prune;
        }
        crate::src::parse::TK_BETWEEN_1 => {
            crate::src::src::walker::sqlite3WalkExpr(
                pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
                __pExpr_ref.pLeft as *mut crate::src::headers::sqliteInt_h::Expr,
            );
            bothImplyNotNullRow(
                pWalker,
                (*(&raw mut (*__pExpr_ref.x.pList).a
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(0_isize))
                .pExpr,
                (*(&raw mut (*__pExpr_ref.x.pList).a
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(1_isize))
                .pExpr,
            );
            return crate::src::headers::sqliteInt_h::WRC_Prune;
        }
        crate::src::parse::TK_EQ_1
        | crate::src::parse::TK_NE
        | crate::src::parse::TK_LT
        | crate::src::parse::TK_LE
        | crate::src::parse::TK_GT
        | crate::src::parse::TK_GE => {
            let pLeft: *mut crate::src::headers::sqliteInt_h::Expr = __pExpr_ref.pLeft;
            let pRight: *mut crate::src::headers::sqliteInt_h::Expr = __pExpr_ref.pRight;
            let __pRight_ref = unsafe { &mut *pRight };
            let __pLeft_ref = unsafe { &mut *pLeft };
            if __pLeft_ref.op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN
                && !__pLeft_ref.y.pTab.is_null()
                && (*__pLeft_ref.y.pTab).eTabType as ::core::ffi::c_int
                    == crate::src::headers::sqliteInt_h::TABTYP_VTAB
                || __pRight_ref.op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN
                    && !__pRight_ref.y.pTab.is_null()
                    && (*__pRight_ref.y.pTab).eTabType as ::core::ffi::c_int
                        == crate::src::headers::sqliteInt_h::TABTYP_VTAB
            {
                return crate::src::headers::sqliteInt_h::WRC_Prune;
            }
        }
        _ => {}
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprImpliesNonNullRow(
    mut p: *mut crate::src::headers::sqliteInt_h::Expr,
    iTab: ::core::ffi::c_int,
    isRJ: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut w: crate::src::headers::sqliteInt_h::Walker =
        crate::src::headers::sqliteInt_h::Walker {
            pParse: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Parse>(),
            xExprCallback: None,
            xSelectCallback: None,
            xSelectCallback2: None,
            walkerDepth: 0,
            eCode: 0,
            mWFlags: 0,
            u: crate::src::headers::sqliteInt_h::__anon_union_16 {
                pNC: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::NameContext>(),
            },
        };
    p = sqlite3ExprSkipCollateAndLikely(p);
    if p.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*p).op as ::core::ffi::c_int == crate::src::parse::TK_NOTNULL_1 {
        p = (*p).pLeft;
    } else {
        while (*p).op as ::core::ffi::c_int == crate::src::parse::TK_AND {
            if sqlite3ExprImpliesNonNullRow((*p).pLeft, iTab, isRJ) != 0 {
                return 1 as ::core::ffi::c_int;
            }
            p = (*p).pRight;
        }
    }
    w.xExprCallback = Some(
        impliesNotNullRow
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
    w.xSelectCallback = None;
    w.xSelectCallback2 = None;
    w.eCode = 0 as crate::src::fts5::U16_0;
    w.mWFlags = (isRJ != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as crate::src::fts5::U16_0;
    w.u.iCur = iTab;
    crate::src::src::walker::sqlite3WalkExpr(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        p as *mut crate::src::headers::sqliteInt_h::Expr,
    );
    w.eCode as ::core::ffi::c_int
}

unsafe extern "C" fn exprIdxCover(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let __pExpr_ref = unsafe { &*pExpr };
    if __pExpr_ref.op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN
        && __pExpr_ref.iTable == (*(*pWalker).u.pIdxCover).iCur
        && crate::src::src::build::sqlite3TableColumnToIndex(
            (*(*pWalker).u.pIdxCover).pIdx as *mut crate::src::headers::sqliteInt_h::Index,
            __pExpr_ref.iColumn as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
    {
        (*pWalker).eCode = 1 as crate::src::fts5::U16_0;
        return crate::src::headers::sqliteInt_h::WRC_Abort;
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprCoveredByIndex(
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    iCur: ::core::ffi::c_int,
    pIdx: *mut crate::src::headers::sqliteInt_h::Index,
) -> ::core::ffi::c_int {
    let mut w: crate::src::headers::sqliteInt_h::Walker = unsafe { ::core::mem::zeroed() };
    let mut xcov: IdxCover = unsafe { ::core::mem::zeroed() };
    xcov.iCur = iCur;
    xcov.pIdx = pIdx;
    w.xExprCallback = Some(
        exprIdxCover
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
    w.u.pIdxCover = &raw mut xcov as *mut IdxCover;
    crate::src::src::walker::sqlite3WalkExpr(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
    );
    (w.eCode == 0) as ::core::ffi::c_int
}

unsafe extern "C" fn selectRefEnter(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pSelect: *mut crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    let p: *mut RefSrcList = (*pWalker).u.pRefSrcList as *mut RefSrcList;
    let pSrc: *mut crate::src::headers::sqliteInt_h::SrcList = (*pSelect).pSrc;
    let mut i: crate::src::ext::rtree::rtree::I64_0;
    let mut j: crate::src::ext::rtree::rtree::I64_0;
    
    let __pSrc_ref = unsafe { &mut *pSrc };
    if __pSrc_ref.nSrc == 0 as ::core::ffi::c_int {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    let __p_ref = unsafe { &mut *p };
    j = __p_ref.nExclude;
    __p_ref.nExclude += __pSrc_ref.nSrc as crate::src::ext::rtree::rtree::I64_0;
    let piNew: *mut ::core::ffi::c_int = crate::src::src::malloc::sqlite3DbRealloc(
        __p_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __p_ref.aiExclude as *mut ::core::ffi::c_void,
        (__p_ref.nExclude as crate::src::ext::rtree::rtree::U64_0)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()
                as crate::src::ext::rtree::rtree::U64_0),
    ) as *mut ::core::ffi::c_int;
    if piNew.is_null() {
        __p_ref.nExclude = 0 as crate::src::ext::rtree::rtree::I64_0;
        return crate::src::headers::sqliteInt_h::WRC_Abort;
    } else {
        __p_ref.aiExclude = piNew;
    }
    i = 0 as crate::src::ext::rtree::rtree::I64_0;
    while i < __pSrc_ref.nSrc as crate::src::ext::rtree::rtree::I64_0 {
        *__p_ref.aiExclude.offset(j as isize) = (*(&raw mut __pSrc_ref.a
            as *mut crate::src::headers::sqliteInt_h::SrcItem)
            .offset(i as isize))
        .iCursor;
        i += 1;
        j += 1;
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}

unsafe extern "C" fn selectRefLeave(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pSelect: *mut crate::src::headers::sqliteInt_h::Select,
) {
    let p: *mut RefSrcList = (*pWalker).u.pRefSrcList as *mut RefSrcList;
    let pSrc: *mut crate::src::headers::sqliteInt_h::SrcList = (*pSelect).pSrc;
    if (*p).nExclude != 0 {
        (*p).nExclude -= (*pSrc).nSrc as crate::src::ext::rtree::rtree::I64_0;
    }
}

unsafe extern "C" fn exprRefToSrcList(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN
        || (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_AGG_COLUMN
    {
        let mut i: ::core::ffi::c_int;
        let p: *mut RefSrcList = (*pWalker).u.pRefSrcList as *mut RefSrcList;
        let __p_ref = unsafe { &mut *p };
        let pSrc: *mut crate::src::headers::sqliteInt_h::SrcList = __p_ref.pRef;
        let nSrc: ::core::ffi::c_int = if !pSrc.is_null() {
            (*pSrc).nSrc
        } else {
            0 as ::core::ffi::c_int
        };
        i = 0 as ::core::ffi::c_int;
        while i < nSrc {
            if (*pExpr).iTable
                == (*(&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(i as isize))
                .iCursor
            {
                (*pWalker).eCode = ((*pWalker).eCode as ::core::ffi::c_int
                    | 1 as ::core::ffi::c_int)
                    as crate::src::fts5::U16_0;
                return crate::src::headers::sqliteInt_h::WRC_Continue;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while (i as crate::src::ext::rtree::rtree::I64_0) < __p_ref.nExclude
            && *__p_ref.aiExclude.offset(i as isize) != (*pExpr).iTable
        {
            i += 1;
        }
        if i as crate::src::ext::rtree::rtree::I64_0 >= __p_ref.nExclude {
            (*pWalker).eCode = ((*pWalker).eCode as ::core::ffi::c_int | 2 as ::core::ffi::c_int)
                as crate::src::fts5::U16_0;
        }
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ReferencesSrcList(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    pSrcList: *mut crate::src::headers::sqliteInt_h::SrcList,
) -> ::core::ffi::c_int {
    let mut w: crate::src::headers::sqliteInt_h::Walker = unsafe { ::core::mem::zeroed() };
    let mut x: RefSrcList = unsafe { ::core::mem::zeroed() };
    w.xExprCallback = Some(
        exprRefToSrcList
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
    w.xSelectCallback = Some(
        selectRefEnter
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
    w.xSelectCallback2 = Some(
        selectRefLeave
            as unsafe extern "C" fn(
                *mut crate::src::headers::sqliteInt_h::Walker,
                *mut crate::src::headers::sqliteInt_h::Select,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut crate::src::headers::sqliteInt_h::Walker,
                *mut crate::src::headers::sqliteInt_h::Select,
            ) -> (),
        >;
    w.u.pRefSrcList = &raw mut x as *mut RefSrcList;
    x.db = (*pParse).db;
    x.pRef = pSrcList;
    let __pExpr_ref = unsafe { &mut *pExpr };
    crate::src::src::walker::sqlite3WalkExprList(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        __pExpr_ref.x.pList as *mut crate::src::headers::sqliteInt_h::ExprList,
    );
    if !__pExpr_ref.pLeft.is_null() {
        crate::src::src::walker::sqlite3WalkExprList(
            &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
            (*__pExpr_ref.pLeft).x.pList as *mut crate::src::headers::sqliteInt_h::ExprList,
        );
    }
    if __pExpr_ref.flags & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        crate::src::src::walker::sqlite3WalkExpr(
            &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
            (*__pExpr_ref.y.pWin).pFilter as *mut crate::src::headers::sqliteInt_h::Expr,
        );
    }
    if !x.aiExclude.is_null() {
        crate::src::src::malloc::sqlite3DbNNFreeNN(
            (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            x.aiExclude as *mut ::core::ffi::c_void,
        );
    }
    if w.eCode as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else if w.eCode != 0 {
        0 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    }
}

unsafe extern "C" fn agginfoPersistExprCb(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    mut pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    if ((*pExpr).flags
        & (0x10000 as ::core::ffi::c_int | 0x4000 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
        && !(*pExpr).pAggInfo.is_null()
    {
        let pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo = (*pExpr).pAggInfo;
        let iAgg: ::core::ffi::c_int = (*pExpr).iAgg as ::core::ffi::c_int;
        let pParse: *mut crate::src::headers::sqliteInt_h::Parse = (*pWalker).pParse;
        let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
        if (*pExpr).op as ::core::ffi::c_int != crate::src::parse::TK_AGG_FUNCTION {
            if iAgg < (*pAggInfo).nColumn
                && (*(*pAggInfo).aCol.offset(iAgg as isize)).pCExpr == pExpr
            {
                pExpr = sqlite3ExprDup(db, pExpr, 0 as ::core::ffi::c_int);
                if !pExpr.is_null() && sqlite3ExprDeferredDelete(pParse, pExpr) == 0 {
                    let fresh11 = &mut (*(*pAggInfo).aCol.offset(iAgg as isize)).pCExpr;
                    *fresh11 = pExpr;
                }
            }
        } else if iAgg < (*pAggInfo).nFunc
            && (*(*pAggInfo).aFunc.offset(iAgg as isize)).pFExpr == pExpr
        {
            pExpr = sqlite3ExprDup(db, pExpr, 0 as ::core::ffi::c_int);
            if !pExpr.is_null() && sqlite3ExprDeferredDelete(pParse, pExpr) == 0 {
                let fresh12 = &mut (*(*pAggInfo).aFunc.offset(iAgg as isize)).pFExpr;
                *fresh12 = pExpr;
            }
        }
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3AggInfoPersistWalkerInit(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
) {
    ::libc::memset(
        pWalker as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Walker>()
            as crate::__stddef_size_t_h::SizeT,
    );
    let __pWalker_ref = unsafe { &mut *pWalker };
    __pWalker_ref.pParse = pParse;
    __pWalker_ref.xExprCallback = Some(
        agginfoPersistExprCb
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
    __pWalker_ref.xSelectCallback = Some(
        crate::src::src::walker::sqlite3SelectWalkNoop
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
}

unsafe extern "C" fn addAggInfoColumn(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pInfo: *mut crate::src::headers::sqliteInt_h::AggInfo,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let __pInfo_ref = unsafe { &mut *pInfo };
    __pInfo_ref.aCol = crate::src::src::build::sqlite3ArrayAllocate(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __pInfo_ref.aCol as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<crate::src::headers::sqliteInt_h::AggInfo_col>()
            as ::core::ffi::c_int,
        &raw mut __pInfo_ref.nColumn,
        &raw mut i,
    ) as *mut crate::src::headers::sqliteInt_h::AggInfo_col;
    i
}

unsafe extern "C" fn addAggInfoFunc(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pInfo: *mut crate::src::headers::sqliteInt_h::AggInfo,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let __pInfo_ref = unsafe { &mut *pInfo };
    __pInfo_ref.aFunc = crate::src::src::build::sqlite3ArrayAllocate(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __pInfo_ref.aFunc as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<crate::src::headers::sqliteInt_h::AggInfo_func>()
            as ::core::ffi::c_int,
        &raw mut __pInfo_ref.nFunc,
        &raw mut i,
    ) as *mut crate::src::headers::sqliteInt_h::AggInfo_func;
    i
}

unsafe extern "C" fn findOrCreateAggInfoColumn(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) {
    let current_block: u64;
    let mut pCol: *mut crate::src::headers::sqliteInt_h::AggInfo_col;
    let mut k: ::core::ffi::c_int;
    let mxTerm: ::core::ffi::c_int =
        (*(*pParse).db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_COLUMN as usize];
    pCol = (*pAggInfo).aCol as *mut crate::src::headers::sqliteInt_h::AggInfo_col;
    k = 0 as ::core::ffi::c_int;
    let __pExpr_ref = unsafe { &mut *pExpr };
    loop {
        if (k >= (*pAggInfo).nColumn) {
            current_block = 14523784380283086299;
            break;
        }
        if (*pCol).pCExpr == pExpr {
            return;
        }
        if (*pCol).iTable == __pExpr_ref.iTable
            && (*pCol).iColumn == __pExpr_ref.iColumn as ::core::ffi::c_int
            && __pExpr_ref.op as ::core::ffi::c_int != crate::src::parse::TK_IF_NULL_ROW
        {
            current_block = 17281240262373992796;
            break;
        }
        k += 1;
        pCol = pCol.offset(1);
    }
    if current_block == 14523784380283086299 {
        k = addAggInfoColumn((*pParse).db, pAggInfo);
        if k < 0 as ::core::ffi::c_int {
            return;
        }
        if k > mxTerm {
            crate::src::src::util::sqlite3ErrorMsg_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                b"more than %d aggregate terms\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Int(
                    mxTerm as crate::src::ext::rtree::rtree::I64_0,
                )],
            );
            k = mxTerm;
        }
        pCol = (*pAggInfo).aCol.offset(k as isize)
            as *mut crate::src::headers::sqliteInt_h::AggInfo_col
            as *mut crate::src::headers::sqliteInt_h::AggInfo_col;
        (*pCol).pTab = __pExpr_ref.y.pTab;
        (*pCol).iTable = __pExpr_ref.iTable;
        (*pCol).iColumn = __pExpr_ref.iColumn as ::core::ffi::c_int;
        (*pCol).iSorterColumn = -(1 as ::core::ffi::c_int);
        (*pCol).pCExpr = pExpr;
        if !(*pAggInfo).pGroupBy.is_null()
            && __pExpr_ref.op as ::core::ffi::c_int != crate::src::parse::TK_IF_NULL_ROW
        {
            let mut j: ::core::ffi::c_int;
            
            let pGB: *mut crate::src::headers::sqliteInt_h::ExprList = (*pAggInfo).pGroupBy;
            let mut pTerm: *mut crate::src::headers::sqliteInt_h::ExprList_item =
                &raw mut (*pGB).a as *mut crate::src::headers::sqliteInt_h::ExprList_item;
            let n: ::core::ffi::c_int = (*pGB).nExpr;
            j = 0 as ::core::ffi::c_int;
            while j < n {
                let pE: *mut crate::src::headers::sqliteInt_h::Expr = (*pTerm).pExpr;
                let __pE_ref = unsafe { &*pE };
                if __pE_ref.op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN
                    && __pE_ref.iTable == __pExpr_ref.iTable
                    && __pE_ref.iColumn as ::core::ffi::c_int
                        == __pExpr_ref.iColumn as ::core::ffi::c_int
                {
                    (*pCol).iSorterColumn = j;
                    break;
                } else {
                    j += 1;
                    pTerm = pTerm.offset(1);
                }
            }
        }
        if (*pCol).iSorterColumn < 0 as ::core::ffi::c_int {
            let __pAggInfo_ref = unsafe { &mut *pAggInfo };
            let fresh16 = __pAggInfo_ref.nSortingColumn;
            __pAggInfo_ref.nSortingColumn = __pAggInfo_ref.nSortingColumn.wrapping_add(1);
            (*pCol).iSorterColumn = fresh16 as ::core::ffi::c_int;
        }
    }
    __pExpr_ref.pAggInfo = pAggInfo;
    if __pExpr_ref.op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN {
        __pExpr_ref.op = crate::src::parse::TK_AGG_COLUMN as crate::src::ext::rtree::rtree::U8_0;
    }
    __pExpr_ref.iAgg = k as crate::src::fts5::I16_0;
}

unsafe extern "C" fn analyzeAggregate(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let pNC: *mut crate::src::headers::sqliteInt_h::NameContext = (*pWalker).u.pNC;
    let __pNC_ref = unsafe { &*pNC };
    let pParse: *mut crate::src::headers::sqliteInt_h::Parse = __pNC_ref.pParse;
    let pSrcList: *mut crate::src::headers::sqliteInt_h::SrcList = __pNC_ref.pSrcList;
    let pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo = __pNC_ref.uNC.pAggInfo;
    match (*pExpr).op as ::core::ffi::c_int {
        crate::src::parse::TK_IF_NULL_ROW
        | crate::src::parse::TK_AGG_COLUMN
        | crate::src::parse::TK_COLUMN => {
            if !pSrcList.is_null() {
                let mut pItem: *mut crate::src::headers::sqliteInt_h::SrcItem =
                    &raw mut (*pSrcList).a as *mut crate::src::headers::sqliteInt_h::SrcItem;
                i = 0 as ::core::ffi::c_int;
                while i < (*pSrcList).nSrc {
                    if (*pExpr).iTable == (*pItem).iCursor {
                        findOrCreateAggInfoColumn(pParse, pAggInfo, pExpr);
                        break;
                    } else {
                        i += 1;
                        pItem = pItem.offset(1);
                    }
                }
            }
            return crate::src::headers::sqliteInt_h::WRC_Continue;
        }
        crate::src::parse::TK_AGG_FUNCTION => {
            if __pNC_ref.ncFlags & crate::src::headers::sqliteInt_h::NC_InAggFunc
                == 0 as ::core::ffi::c_int
                && (*pWalker).walkerDepth == (*pExpr).op2 as ::core::ffi::c_int
                && (*pExpr).pAggInfo.is_null()
            {
                let __pAggInfo_ref = unsafe { &mut *pAggInfo };
                let mut pItem_0: *mut crate::src::headers::sqliteInt_h::AggInfo_func =
                    __pAggInfo_ref.aFunc as *mut crate::src::headers::sqliteInt_h::AggInfo_func;
                let mxTerm: ::core::ffi::c_int = (*(*pParse).db).aLimit
                    [crate::src::headers::sqlite3_h::SQLITE_LIMIT_COLUMN as usize];
                i = 0 as ::core::ffi::c_int;
                while i < __pAggInfo_ref.nFunc {
                    if (*pItem_0).pFExpr == pExpr {
                        break;
                    }
                    if sqlite3ExprCompare(
                        ::core::ptr::null::<crate::src::headers::sqliteInt_h::Parse>(),
                        (*pItem_0).pFExpr,
                        pExpr,
                        -(1 as ::core::ffi::c_int),
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                    i += 1;
                    pItem_0 = pItem_0.offset(1);
                }
                if i > mxTerm {
                    crate::src::src::util::sqlite3ErrorMsg_args(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        b"more than %d aggregate terms\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[crate::src::src::printf::PrintfArg::Int(
                            mxTerm as crate::src::ext::rtree::rtree::I64_0,
                        )],
                    );
                    i = mxTerm;
                } else if i >= __pAggInfo_ref.nFunc {
                    let enc: crate::src::ext::rtree::rtree::U8_0 = (*(*pParse).db).enc;
                    i = addAggInfoFunc((*pParse).db, pAggInfo);
                    if i >= 0 as ::core::ffi::c_int {
                        
                        pItem_0 = __pAggInfo_ref.aFunc.offset(i as isize)
                            as *mut crate::src::headers::sqliteInt_h::AggInfo_func
                            as *mut crate::src::headers::sqliteInt_h::AggInfo_func;
                        (*pItem_0).pFExpr = pExpr;
                        let __pExpr_ref = unsafe { &mut *pExpr };
                        let nArg: ::core::ffi::c_int = if !__pExpr_ref.x.pList.is_null() {
                            (*__pExpr_ref.x.pList).nExpr
                        } else {
                            0 as ::core::ffi::c_int
                        };
                        (*pItem_0).pFunc = crate::src::src::callback::sqlite3FindFunction(
                            (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                            __pExpr_ref.u.zToken,
                            nArg,
                            enc,
                            0 as crate::src::ext::rtree::rtree::U8_0,
                        )
                            as *mut crate::src::headers::sqliteInt_h::FuncDef;
                        if !__pExpr_ref.pLeft.is_null()
                            && (*(*pItem_0).pFunc).funcFlags
                                & crate::src::headers::sqliteInt_h::SQLITE_FUNC_NEEDCOLL
                                    as crate::src::ext::rtree::rtree::U32_0
                                == 0 as crate::src::ext::rtree::rtree::U32_0
                        {
                            
                            let __pParse_ref = unsafe { &mut *pParse };
                            let fresh14 = __pParse_ref.nTab;
                            __pParse_ref.nTab += 1;
                            let __pItem_0_ref = unsafe { &mut *pItem_0 };
                            __pItem_0_ref.iOBTab = fresh14;
                            let pOBList: *mut crate::src::headers::sqliteInt_h::ExprList = (*__pExpr_ref.pLeft).x.pList;
                            if (*pOBList).nExpr == 1 as ::core::ffi::c_int
                                && nArg == 1 as ::core::ffi::c_int
                                && sqlite3ExprCompare(
                                    ::core::ptr::null::<crate::src::headers::sqliteInt_h::Parse>(),
                                    (*(&raw mut (*pOBList).a
                                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                        .offset(0_isize))
                                    .pExpr,
                                    (*(&raw mut (*__pExpr_ref.x.pList).a
                                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                        .offset(0_isize))
                                    .pExpr,
                                    0 as ::core::ffi::c_int,
                                ) == 0 as ::core::ffi::c_int
                            {
                                __pItem_0_ref.bOBPayload = 0 as crate::src::ext::rtree::rtree::U8_0;
                                __pItem_0_ref.bOBUnique = (__pExpr_ref.flags
                                    & 0x4 as ::core::ffi::c_int
                                        as crate::src::ext::rtree::rtree::U32_0
                                    != 0 as crate::src::ext::rtree::rtree::U32_0)
                                    as ::core::ffi::c_int
                                    as crate::src::ext::rtree::rtree::U8_0;
                            } else {
                                __pItem_0_ref.bOBPayload = 1 as crate::src::ext::rtree::rtree::U8_0;
                            }
                            __pItem_0_ref.bUseSubtype = ((*__pItem_0_ref.pFunc).funcFlags
                                & crate::src::headers::sqlite3_h::SQLITE_SUBTYPE
                                    as crate::src::ext::rtree::rtree::U32_0
                                != 0 as crate::src::ext::rtree::rtree::U32_0)
                                as ::core::ffi::c_int
                                as crate::src::ext::rtree::rtree::U8_0;
                        } else {
                            (*pItem_0).iOBTab = -(1 as ::core::ffi::c_int);
                        }
                        if __pExpr_ref.flags
                            & 0x4 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                            != 0 as crate::src::ext::rtree::rtree::U32_0
                            && (*pItem_0).bOBUnique == 0
                        {
                            let __pParse_ref = unsafe { &mut *pParse };
                            let fresh15 = __pParse_ref.nTab;
                            __pParse_ref.nTab += 1;
                            (*pItem_0).iDistinct = fresh15;
                        } else {
                            (*pItem_0).iDistinct = -(1 as ::core::ffi::c_int);
                        }
                    }
                }
                (*pExpr).iAgg = i as crate::src::fts5::I16_0;
                (*pExpr).pAggInfo = pAggInfo;
                return crate::src::headers::sqliteInt_h::WRC_Prune;
            } else {
                return crate::src::headers::sqliteInt_h::WRC_Continue;
            }
        }
        _ => {
            let mut pIEpr: *mut crate::src::headers::sqliteInt_h::IndexedExpr;
            let mut tmp: crate::src::headers::sqliteInt_h::Expr = unsafe { ::core::mem::zeroed() };
            if (__pNC_ref.ncFlags & crate::src::headers::sqliteInt_h::NC_InAggFunc != 0 as ::core::ffi::c_int)
                && !(*pParse).pIdxEpr.is_null() {
                    pIEpr = (*pParse).pIdxEpr;
                    while !pIEpr.is_null() {
                        let iDataCur: ::core::ffi::c_int = (*pIEpr).iDataCur;
                        if (iDataCur >= 0 as ::core::ffi::c_int)
                            && sqlite3ExprCompare(
                                ::core::ptr::null::<crate::src::headers::sqliteInt_h::Parse>(),
                                pExpr,
                                (*pIEpr).pExpr,
                                iDataCur,
                            ) == 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                        pIEpr = (*pIEpr).pIENext;
                    }
                    if !pIEpr.is_null()
                        && (*pExpr).flags
                            & (0x1000000 as ::core::ffi::c_int | 0x2000000 as ::core::ffi::c_int)
                                as crate::src::ext::rtree::rtree::U32_0
                            == 0 as crate::src::ext::rtree::rtree::U32_0
                        {
                            i = 0 as ::core::ffi::c_int;
                            while i < (*pSrcList).nSrc {
                                if (*(&raw mut (*pSrcList).a
                                    as *mut crate::src::headers::sqliteInt_h::SrcItem)
                                    .offset(0_isize))
                                .iCursor
                                    == (*pIEpr).iDataCur
                                {
                                    break;
                                }
                                i += 1;
                            }
                            if (i < (*pSrcList).nSrc)
                                && (*pExpr).pAggInfo.is_null() {
                                    if (*pParse).nErr != 0 {
                                        return crate::src::headers::sqliteInt_h::WRC_Abort;
                                    }
                                    tmp.op = crate::src::parse::TK_AGG_COLUMN
                                        as crate::src::ext::rtree::rtree::U8_0;
                                    tmp.iTable = (*pIEpr).iIdxCur;
                                    tmp.iColumn =
                                        (*pIEpr).iIdxCol as crate::src::headers::sqliteInt_h::YnVar;
                                    findOrCreateAggInfoColumn(pParse, pAggInfo, &raw mut tmp);
                                    if (*pParse).nErr != 0 {
                                        return crate::src::headers::sqliteInt_h::WRC_Abort;
                                    }
                                    let fresh13 = &mut (*(*pAggInfo).aCol.offset(tmp.iAgg as isize)).pCExpr;
                                    *fresh13 = pExpr;
                                    (*pExpr).pAggInfo = pAggInfo;
                                    (*pExpr).iAgg = tmp.iAgg;
                                    return crate::src::headers::sqliteInt_h::WRC_Prune;
                                }
                        }
                }
        }
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprAnalyzeAggregates(
    pNC: *mut crate::src::headers::sqliteInt_h::NameContext,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) {
    let mut w: crate::src::headers::sqliteInt_h::Walker =
        crate::src::headers::sqliteInt_h::Walker {
            pParse: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Parse>(),
            xExprCallback: None,
            xSelectCallback: None,
            xSelectCallback2: None,
            walkerDepth: 0,
            eCode: 0,
            mWFlags: 0,
            u: crate::src::headers::sqliteInt_h::__anon_union_16 {
                pNC: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::NameContext>(),
            },
        };
    w.xExprCallback = Some(
        analyzeAggregate
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
    w.xSelectCallback = Some(
        crate::src::src::walker::sqlite3WalkerDepthIncrease
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
    w.xSelectCallback2 = Some(
        crate::src::src::walker::sqlite3WalkerDepthDecrease
            as unsafe extern "C" fn(
                *mut crate::src::headers::sqliteInt_h::Walker,
                *mut crate::src::headers::sqliteInt_h::Select,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut crate::src::headers::sqliteInt_h::Walker,
                *mut crate::src::headers::sqliteInt_h::Select,
            ) -> (),
        >;
    w.walkerDepth = 0 as ::core::ffi::c_int;
    w.u.pNC = pNC;
    w.pParse = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Parse>();
    crate::src::src::walker::sqlite3WalkExpr(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
    );
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExprAnalyzeAggList(
    pNC: *mut crate::src::headers::sqliteInt_h::NameContext,
    pList: *mut crate::src::headers::sqliteInt_h::ExprList,
) {
    let mut pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item;
    let mut i: ::core::ffi::c_int;
    if !pList.is_null() {
        pItem = &raw mut (*pList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item
            as *mut crate::src::headers::sqliteInt_h::ExprList_item;
        i = 0 as ::core::ffi::c_int;
        while i < (*pList).nExpr {
            sqlite3ExprAnalyzeAggregates(pNC, (*pItem).pExpr);
            i += 1;
            pItem = pItem.offset(1);
        }
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3GetTempReg(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
) -> ::core::ffi::c_int {
    let __pParse_ref = unsafe { &mut *pParse };
    if __pParse_ref.nTempReg as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        __pParse_ref.nMem += 1;
        return __pParse_ref.nMem;
    }
    __pParse_ref.nTempReg = __pParse_ref.nTempReg.wrapping_sub(1);
    __pParse_ref.aTempReg[__pParse_ref.nTempReg as usize]
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ReleaseTempReg(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    iReg: ::core::ffi::c_int,
) {
    if iReg != 0
        && ((*pParse).nTempReg as ::core::ffi::c_int)
            < (::core::mem::size_of::<[::core::ffi::c_int; 8]>() as usize)
                .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                as ::core::ffi::c_int
        {
            let __pParse_ref = unsafe { &mut *pParse };
            let fresh0 = __pParse_ref.nTempReg;
            __pParse_ref.nTempReg = __pParse_ref.nTempReg.wrapping_add(1);
            __pParse_ref.aTempReg[fresh0 as usize] = iReg;
        }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3GetTempRange(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    nReg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    
    if nReg == 1 as ::core::ffi::c_int {
        return sqlite3GetTempReg(pParse);
    }
    i = (*pParse).iRangeReg;
    let n: ::core::ffi::c_int = (*pParse).nRangeReg;
    if nReg <= n {
        (*pParse).iRangeReg += nReg;
        (*pParse).nRangeReg -= nReg;
    } else {
        i = (*pParse).nMem + 1 as ::core::ffi::c_int;
        (*pParse).nMem += nReg;
    }
    i
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ReleaseTempRange(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    iReg: ::core::ffi::c_int,
    nReg: ::core::ffi::c_int,
) {
    if nReg == 1 as ::core::ffi::c_int {
        sqlite3ReleaseTempReg(pParse, iReg);
        return;
    }
    if nReg > (*pParse).nRangeReg {
        (*pParse).nRangeReg = nReg;
        (*pParse).iRangeReg = iReg;
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ClearTempRegCache(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
) {
    (*pParse).nTempReg = 0 as crate::src::ext::rtree::rtree::U8_0;
    (*pParse).nRangeReg = 0 as ::core::ffi::c_int;
}
pub unsafe extern "C" fn sqlite3TouchRegister(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    iReg: ::core::ffi::c_int,
) {
    if (*pParse).nMem < iReg {
        (*pParse).nMem = iReg;
    }
}
