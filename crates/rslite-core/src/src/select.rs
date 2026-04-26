pub use crate::__stddef_size_t_h::SizeT;

pub use crate::src::headers::btreeInt_h::Btree;
pub use crate::src::headers::opcodes_h::OP_AddImm;
pub use crate::src::headers::opcodes_h::OP_AggFinal;
pub use crate::src::headers::opcodes_h::OP_AggStep;
pub use crate::src::headers::opcodes_h::OP_Close;
pub use crate::src::headers::opcodes_h::OP_CollSeq;
pub use crate::src::headers::opcodes_h::OP_Column;
pub use crate::src::headers::opcodes_h::OP_Compare;
pub use crate::src::headers::opcodes_h::OP_Copy;
pub use crate::src::headers::opcodes_h::OP_Count;
pub use crate::src::headers::opcodes_h::OP_DecrJumpZero;
pub use crate::src::headers::opcodes_h::OP_Delete;
pub use crate::src::headers::opcodes_h::OP_Eq;
pub use crate::src::headers::opcodes_h::OP_Explain;
pub use crate::src::headers::opcodes_h::OP_FilterAdd;
pub use crate::src::headers::opcodes_h::OP_Found;
pub use crate::src::headers::opcodes_h::OP_GetSubtype;
pub use crate::src::headers::opcodes_h::OP_Gosub;
pub use crate::src::headers::opcodes_h::OP_Goto;
pub use crate::src::headers::opcodes_h::OP_IdxDelete;
pub use crate::src::headers::opcodes_h::OP_IdxInsert;
pub use crate::src::headers::opcodes_h::OP_IdxLE;
pub use crate::src::headers::opcodes_h::OP_If;
pub use crate::src::headers::opcodes_h::OP_IfEmpty;
pub use crate::src::headers::opcodes_h::OP_IfNot;
pub use crate::src::headers::opcodes_h::OP_IfNotZero;
pub use crate::src::headers::opcodes_h::OP_IfPos;
pub use crate::src::headers::opcodes_h::OP_InitCoroutine;
pub use crate::src::headers::opcodes_h::OP_Insert;
pub use crate::src::headers::opcodes_h::OP_Integer;
pub use crate::src::headers::opcodes_h::OP_IsNull;
pub use crate::src::headers::opcodes_h::OP_Jump;
pub use crate::src::headers::opcodes_h::OP_Last;
pub use crate::src::headers::opcodes_h::OP_MakeRecord;
pub use crate::src::headers::opcodes_h::OP_MustBeInt;
pub use crate::src::headers::opcodes_h::OP_Ne;
pub use crate::src::headers::opcodes_h::OP_NewRowid;
pub use crate::src::headers::opcodes_h::OP_Next;
pub use crate::src::headers::opcodes_h::OP_NotFound;
pub use crate::src::headers::opcodes_h::OP_Null;
pub use crate::src::headers::opcodes_h::OP_NullRow;
pub use crate::src::headers::opcodes_h::OP_OffsetLimit;
pub use crate::src::headers::opcodes_h::OP_Once;
pub use crate::src::headers::opcodes_h::OP_OpenDup;
pub use crate::src::headers::opcodes_h::OP_OpenEphemeral;
pub use crate::src::headers::opcodes_h::OP_OpenPseudo;
pub use crate::src::headers::opcodes_h::OP_OpenRead;
pub use crate::src::headers::opcodes_h::OP_Permutation;
pub use crate::src::headers::opcodes_h::OP_ResetSorter;
pub use crate::src::headers::opcodes_h::OP_ResultRow;
pub use crate::src::headers::opcodes_h::OP_Return;
pub use crate::src::headers::opcodes_h::OP_Rewind;
pub use crate::src::headers::opcodes_h::OP_RowData;
pub use crate::src::headers::opcodes_h::OP_SCopy;
pub use crate::src::headers::opcodes_h::OP_Sequence;
pub use crate::src::headers::opcodes_h::OP_SequenceTest;
pub use crate::src::headers::opcodes_h::OP_SetSubtype;
pub use crate::src::headers::opcodes_h::OP_Sort;
pub use crate::src::headers::opcodes_h::OP_SorterData;
pub use crate::src::headers::opcodes_h::OP_SorterInsert;
pub use crate::src::headers::opcodes_h::OP_SorterNext;
pub use crate::src::headers::opcodes_h::OP_SorterOpen;
pub use crate::src::headers::opcodes_h::OP_SorterSort;
pub use crate::src::headers::opcodes_h::OP_Yield;
pub use crate::src::src::btree::BTREE_UNORDERED;
pub use crate::src::src::hash::_ht;
pub use crate::src::src::hash::Hash;
pub use crate::src::src::hash::HashElem;
pub use crate::src::src::hash::sqlite3HashClear;
pub use crate::src::src::hash::sqlite3HashFind;
pub use crate::src::src::hash::sqlite3HashInit;
pub use crate::src::src::hash::sqlite3HashInsert;
pub use crate::src::src::pager::Pgno;

pub use crate::src::ext::rtree::rtree::I64_0;
pub use crate::src::ext::rtree::rtree::U8_0;
pub use crate::src::ext::rtree::rtree::U32_0;
pub use crate::src::ext::rtree::rtree::U64_0;
pub use crate::src::fts5::I16_0;
pub use crate::src::fts5::U16_0;
pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;
pub use crate::src::headers::sqlite3_h::SQLITE_LIMIT_COLUMN;
pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SQLITE_READ;
pub use crate::src::headers::sqlite3_h::SQLITE_RECURSIVE;
pub use crate::src::headers::sqlite3_h::SQLITE_SELECT;
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
pub use crate::src::headers::sqliteInt_h::BMS;
pub use crate::src::headers::sqliteInt_h::Bitmask;
pub use crate::src::headers::sqliteInt_h::BusyHandler;
pub use crate::src::headers::sqliteInt_h::CCurHint;
pub use crate::src::headers::sqliteInt_h::COLFLAG_HASCOLL;
pub use crate::src::headers::sqliteInt_h::COLFLAG_HASTYPE;
pub use crate::src::headers::sqliteInt_h::COLFLAG_HIDDEN;
pub use crate::src::headers::sqliteInt_h::COLFLAG_NOEXPAND;
pub use crate::src::headers::sqliteInt_h::COLFLAG_NOINSERT;
pub use crate::src::headers::sqliteInt_h::CollSeq;
pub use crate::src::headers::sqliteInt_h::Column;
pub use crate::src::headers::sqliteInt_h::CoveringIndexCheck;
pub use crate::src::headers::sqliteInt_h::Cte;
pub use crate::src::headers::sqliteInt_h::CteUse;
pub use crate::src::headers::sqliteInt_h::Db;
pub use crate::src::headers::sqliteInt_h::DbClientData;
pub use crate::src::headers::sqliteInt_h::DbFixer;
pub use crate::src::headers::sqliteInt_h::ENAME_NAME;
pub use crate::src::headers::sqliteInt_h::ENAME_ROWID;
pub use crate::src::headers::sqliteInt_h::ENAME_TAB;
pub use crate::src::headers::sqliteInt_h::EP_Collate;
pub use crate::src::headers::sqliteInt_h::EP_HasFunc;
pub use crate::src::headers::sqliteInt_h::EP_IfNullRow;
pub use crate::src::headers::sqliteInt_h::EP_InnerON;
pub use crate::src::headers::sqliteInt_h::EP_IntValue;
pub use crate::src::headers::sqliteInt_h::EP_IsFalse;
pub use crate::src::headers::sqliteInt_h::EP_OuterON;
pub use crate::src::headers::sqliteInt_h::EP_Subquery;
pub use crate::src::headers::sqliteInt_h::EP_xIsSelect;
pub use crate::src::headers::sqliteInt_h::Expr;
pub use crate::src::headers::sqliteInt_h::ExprList;
pub use crate::src::headers::sqliteInt_h::ExprList_item;
pub use crate::src::headers::sqliteInt_h::FKey;
pub use crate::src::headers::sqliteInt_h::FuncDef;
pub use crate::src::headers::sqliteInt_h::FuncDestructor;
pub use crate::src::headers::sqliteInt_h::IdList;
pub use crate::src::headers::sqliteInt_h::IdList_item;
pub use crate::src::headers::sqliteInt_h::IdxCover;
pub use crate::src::headers::sqliteInt_h::Index;
pub use crate::src::headers::sqliteInt_h::IndexedExpr;
pub use crate::src::headers::sqliteInt_h::JT_CROSS;
pub use crate::src::headers::sqliteInt_h::JT_ERROR;
pub use crate::src::headers::sqliteInt_h::JT_INNER;
pub use crate::src::headers::sqliteInt_h::JT_LEFT;
pub use crate::src::headers::sqliteInt_h::JT_LTORJ;
pub use crate::src::headers::sqliteInt_h::JT_NATURAL;
pub use crate::src::headers::sqliteInt_h::JT_OUTER;
pub use crate::src::headers::sqliteInt_h::JT_RIGHT;
pub use crate::src::headers::sqliteInt_h::KEYINFO_ORDER_BIGNULL;
pub use crate::src::headers::sqliteInt_h::KEYINFO_ORDER_DESC;
pub use crate::src::headers::sqliteInt_h::KeyInfo;
pub use crate::src::headers::sqliteInt_h::LogEst;
pub use crate::src::headers::sqliteInt_h::Lookaside;
pub use crate::src::headers::sqliteInt_h::LookasideSlot;
pub use crate::src::headers::sqliteInt_h::M10d_No;
pub use crate::src::headers::sqliteInt_h::M10d_Yes;
pub use crate::src::headers::sqliteInt_h::Module;
pub use crate::src::headers::sqliteInt_h::NC_InAggFunc;
pub use crate::src::headers::sqliteInt_h::NameContext;
pub use crate::src::headers::sqliteInt_h::OPFLAG_APPEND;
pub use crate::src::headers::sqliteInt_h::OPFLAG_PERMUTE;
pub use crate::src::headers::sqliteInt_h::OPFLAG_USESEEKRESULT;
pub use crate::src::headers::sqliteInt_h::OnOrUsing;
pub use crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME;
pub use crate::src::headers::sqliteInt_h::Parse;
pub use crate::src::headers::sqliteInt_h::ParseCleanup;
pub use crate::src::headers::sqliteInt_h::RefSrcList;
pub use crate::src::headers::sqliteInt_h::RenameCtx;
pub use crate::src::headers::sqliteInt_h::RenameToken;
pub use crate::src::headers::sqliteInt_h::Returning;
pub use crate::src::headers::sqliteInt_h::SF_Aggregate;
pub use crate::src::headers::sqliteInt_h::SF_ComplexResult;
pub use crate::src::headers::sqliteInt_h::SF_Compound;
pub use crate::src::headers::sqliteInt_h::SF_Converted;
pub use crate::src::headers::sqliteInt_h::SF_CopyCte;
pub use crate::src::headers::sqliteInt_h::SF_Distinct;
pub use crate::src::headers::sqliteInt_h::SF_Expanded;
pub use crate::src::headers::sqliteInt_h::SF_FixedLimit;
pub use crate::src::headers::sqliteInt_h::SF_HasTypeInfo;
pub use crate::src::headers::sqliteInt_h::SF_IncludeHidden;
pub use crate::src::headers::sqliteInt_h::SF_MultiPart;
pub use crate::src::headers::sqliteInt_h::SF_MultiValue;
pub use crate::src::headers::sqliteInt_h::SF_NestedFrom;
pub use crate::src::headers::sqliteInt_h::SF_NoopOrderBy;
pub use crate::src::headers::sqliteInt_h::SF_OnToWhere;
pub use crate::src::headers::sqliteInt_h::SF_OrderByReqd;
pub use crate::src::headers::sqliteInt_h::SF_PushDown;
pub use crate::src::headers::sqliteInt_h::SF_Recursive;
pub use crate::src::headers::sqliteInt_h::SF_UFSrcCheck;
pub use crate::src::headers::sqliteInt_h::SF_UpdateFrom;
pub use crate::src::headers::sqliteInt_h::SF_UsesEphemeral;
pub use crate::src::headers::sqliteInt_h::SF_Values;
pub use crate::src::headers::sqliteInt_h::SF_View;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_DEFER;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_FLEXNUM;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_NONE;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC;
pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT;
pub use crate::src::headers::sqliteInt_h::SQLITE_ECEL_DUP;
pub use crate::src::headers::sqliteInt_h::SQLITE_ECEL_OMITREF;
pub use crate::src::headers::sqliteInt_h::SQLITE_ECEL_REF;
pub use crate::src::headers::sqliteInt_h::SQLITE_EnableView;
pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_COUNT;
pub use crate::src::headers::sqliteInt_h::SQLITE_FUNC_NEEDCOLL;
pub use crate::src::headers::sqliteInt_h::SQLITE_FullColNames;
pub use crate::src::headers::sqliteInt_h::SQLITE_IDXTYPE_PRIMARYKEY;
pub use crate::src::headers::sqliteInt_h::SQLITE_JUMPIFNULL;
pub use crate::src::headers::sqliteInt_h::SQLITE_N_STDTYPE;
pub use crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
pub use crate::src::headers::sqliteInt_h::SQLITE_NULLEQ;
pub use crate::src::headers::sqliteInt_h::SQLITE_ShortColNames;
pub use crate::src::headers::sqliteInt_h::SQLITE_TrustedSchema;
pub use crate::src::headers::sqliteInt_h::SRT_Coroutine;
pub use crate::src::headers::sqliteInt_h::SRT_DistFifo;
pub use crate::src::headers::sqliteInt_h::SRT_DistQueue;
pub use crate::src::headers::sqliteInt_h::SRT_EphemTab_1;
pub use crate::src::headers::sqliteInt_h::SRT_Except;
pub use crate::src::headers::sqliteInt_h::SRT_Exists_1;
pub use crate::src::headers::sqliteInt_h::SRT_Fifo;
pub use crate::src::headers::sqliteInt_h::SRT_Mem_1;
pub use crate::src::headers::sqliteInt_h::SRT_Output;
pub use crate::src::headers::sqliteInt_h::SRT_Queue;
pub use crate::src::headers::sqliteInt_h::SRT_Set;
pub use crate::src::headers::sqliteInt_h::SRT_Table;
pub use crate::src::headers::sqliteInt_h::SRT_Union;
pub use crate::src::headers::sqliteInt_h::SRT_Upfrom;
pub use crate::src::headers::sqliteInt_h::SZ_SRCLIST_1;
pub use crate::src::headers::sqliteInt_h::Savepoint;
pub use crate::src::headers::sqliteInt_h::Schema;
pub use crate::src::headers::sqliteInt_h::Select;
pub use crate::src::headers::sqliteInt_h::SelectDest;
pub use crate::src::headers::sqliteInt_h::SrcItem;
pub use crate::src::headers::sqliteInt_h::SrcList;
pub use crate::src::headers::sqliteInt_h::Subquery;
pub use crate::src::headers::sqliteInt_h::TABTYP_NORM;
pub use crate::src::headers::sqliteInt_h::TABTYP_VIEW;
pub use crate::src::headers::sqliteInt_h::TABTYP_VTAB;
pub use crate::src::headers::sqliteInt_h::TF_Ephemeral;
pub use crate::src::headers::sqliteInt_h::TF_NoVisibleRowid;
pub use crate::src::headers::sqliteInt_h::TF_WithoutRowid;
pub use crate::src::headers::sqliteInt_h::TOPBIT;
pub use crate::src::headers::sqliteInt_h::Table;
pub use crate::src::headers::sqliteInt_h::TableLock;
pub use crate::src::headers::sqliteInt_h::Token;
pub use crate::src::headers::sqliteInt_h::Trigger;
pub use crate::src::headers::sqliteInt_h::TriggerPrg;
pub use crate::src::headers::sqliteInt_h::TriggerStep;
pub use crate::src::headers::sqliteInt_h::Upsert;
pub use crate::src::headers::sqliteInt_h::VList;
pub use crate::src::headers::sqliteInt_h::VTable;
pub use crate::src::headers::sqliteInt_h::ViewCanHaveRowid;
pub use crate::src::headers::sqliteInt_h::VtabCtx;
pub use crate::src::headers::sqliteInt_h::WHERE_AGG_DISTINCT;
pub use crate::src::headers::sqliteInt_h::WHERE_DISTINCT_NOOP;
pub use crate::src::headers::sqliteInt_h::WHERE_DISTINCT_ORDERED;
pub use crate::src::headers::sqliteInt_h::WHERE_DISTINCT_UNIQUE;
pub use crate::src::headers::sqliteInt_h::WHERE_DISTINCT_UNORDERED;
pub use crate::src::headers::sqliteInt_h::WHERE_DISTINCTBY;
pub use crate::src::headers::sqliteInt_h::WHERE_GROUPBY;
pub use crate::src::headers::sqliteInt_h::WHERE_ORDERBY_MAX;
pub use crate::src::headers::sqliteInt_h::WHERE_ORDERBY_MIN;
pub use crate::src::headers::sqliteInt_h::WHERE_ORDERBY_NORMAL;
pub use crate::src::headers::sqliteInt_h::WHERE_SORTBYGROUP;
pub use crate::src::headers::sqliteInt_h::WHERE_WANT_DISTINCT;
pub use crate::src::headers::sqliteInt_h::WRC_Abort;
pub use crate::src::headers::sqliteInt_h::WRC_Continue;
pub use crate::src::headers::sqliteInt_h::WRC_Prune;
pub use crate::src::headers::sqliteInt_h::Walker;
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
pub use crate::src::headers::whereInt_h::WhereInfo;
pub use crate::src::parse::TK_AGG_COLUMN;
pub use crate::src::parse::TK_AGG_FUNCTION;
pub use crate::src::parse::TK_ALL;
pub use crate::src::parse::TK_AND;
pub use crate::src::parse::TK_ASTERISK;
pub use crate::src::parse::TK_CAST;
pub use crate::src::parse::TK_COLLATE;
pub use crate::src::parse::TK_COLUMN_1;
pub use crate::src::parse::TK_DOT;
pub use crate::src::parse::TK_EQ;
pub use crate::src::parse::TK_EXCEPT;
pub use crate::src::parse::TK_EXISTS;
pub use crate::src::parse::TK_FUNCTION;
pub use crate::src::parse::TK_GE;
pub use crate::src::parse::TK_ID;
pub use crate::src::parse::TK_IF_NULL_ROW_1;
pub use crate::src::parse::TK_INTEGER;
pub use crate::src::parse::TK_INTERSECT;
pub use crate::src::parse::TK_IS;
pub use crate::src::parse::TK_NULL;
pub use crate::src::parse::TK_PLUS;
pub use crate::src::parse::TK_REGISTER;
pub use crate::src::parse::TK_SELECT_1;
pub use crate::src::parse::TK_TRUEFALSE;
pub use crate::src::parse::TK_UNION;
pub use crate::src::src::alter::sqlite3RenameTokenRemap;
pub use crate::src::src::auth::sqlite3AuthCheck;
pub use crate::src::src::build::sqlite3AffinityType;
pub use crate::src::src::build::sqlite3CodeVerifySchema;
pub use crate::src::src::build::sqlite3ColumnSetColl;
pub use crate::src::src::build::sqlite3DeleteTable;
pub use crate::src::src::build::sqlite3DeleteTableGeneric;
pub use crate::src::src::build::sqlite3IdListAppend;
pub use crate::src::src::build::sqlite3IdListDelete;
pub use crate::src::src::build::sqlite3IdListIndex;
pub use crate::src::src::build::sqlite3KeyInfoOfIndex;
pub use crate::src::src::build::sqlite3LocateTableItem;
pub use crate::src::src::build::sqlite3PrimaryKeyIndex;
pub use crate::src::src::build::sqlite3SrcItemAttachSubquery;
pub use crate::src::src::build::sqlite3SrcListAppendFromTerm;
pub use crate::src::src::build::sqlite3SrcListAppendList;
pub use crate::src::src::build::sqlite3SrcListAssignCursors;
pub use crate::src::src::build::sqlite3SrcListDelete;
pub use crate::src::src::build::sqlite3SrcListEnlarge;
pub use crate::src::src::build::sqlite3SubqueryDetach;
pub use crate::src::src::build::sqlite3TableLock;
pub use crate::src::src::build::sqlite3ViewGetColumnNames;
pub use crate::src::src::build::sqlite3WithDelete;
pub use crate::src::src::build::sqlite3WithDeleteGeneric;
pub use crate::src::src::expr::sqlite3AggInfoPersistWalkerInit;
pub use crate::src::src::expr::sqlite3ClearTempRegCache;
pub use crate::src::src::expr::sqlite3Expr;
pub use crate::src::src::expr::sqlite3ExprAddCollateString;
pub use crate::src::src::expr::sqlite3ExprAffinity;
pub use crate::src::src::expr::sqlite3ExprAnalyzeAggList;
pub use crate::src::src::expr::sqlite3ExprAnalyzeAggregates;
pub use crate::src::src::expr::sqlite3ExprAnd;
pub use crate::src::src::expr::sqlite3ExprCanBeNull;
pub use crate::src::src::expr::sqlite3ExprCode;
pub use crate::src::src::expr::sqlite3ExprCodeExprList;
pub use crate::src::src::expr::sqlite3ExprCodeMove;
pub use crate::src::src::expr::sqlite3ExprCollSeq;
pub use crate::src::src::expr::sqlite3ExprCompareCollSeq;
pub use crate::src::src::expr::sqlite3ExprDataType;
pub use crate::src::src::expr::sqlite3ExprDelete;
pub use crate::src::src::expr::sqlite3ExprDup;
pub use crate::src::src::expr::sqlite3ExprFunction;
pub use crate::src::src::expr::sqlite3ExprIfFalse;
pub use crate::src::src::expr::sqlite3ExprImpliesNonNullRow;
pub use crate::src::src::expr::sqlite3ExprIsConstant;
pub use crate::src::src::expr::sqlite3ExprIsConstantOrGroupBy;
pub use crate::src::src::expr::sqlite3ExprIsInteger;
pub use crate::src::src::expr::sqlite3ExprIsSingleTableConstraint;
pub use crate::src::src::expr::sqlite3ExprIsVector;
pub use crate::src::src::expr::sqlite3ExprListAppend;
pub use crate::src::src::expr::sqlite3ExprListCompare;
pub use crate::src::src::expr::sqlite3ExprListDelete;
pub use crate::src::src::expr::sqlite3ExprListDeleteGeneric;
pub use crate::src::src::expr::sqlite3ExprListDup;
pub use crate::src::src::expr::sqlite3ExprNNCollSeq;
pub use crate::src::src::expr::sqlite3ExprNullRegisterRange;
pub use crate::src::src::expr::sqlite3ExprSetErrorOffset;
pub use crate::src::src::expr::sqlite3ExprSkipCollateAndLikely;
pub use crate::src::src::expr::sqlite3ExprToRegister;
pub use crate::src::src::expr::sqlite3ExprTruthValue;
pub use crate::src::src::expr::sqlite3GetTempRange;
pub use crate::src::src::expr::sqlite3GetTempReg;
pub use crate::src::src::expr::sqlite3IsTrueOrFalse;
pub use crate::src::src::expr::sqlite3PExpr;
pub use crate::src::src::expr::sqlite3PExprAddSelect;
pub use crate::src::src::expr::sqlite3ReleaseTempRange;
pub use crate::src::src::expr::sqlite3ReleaseTempReg;
pub use crate::src::src::expr::sqlite3RowidAlias;
pub use crate::src::src::expr::sqlite3SelectDup;
pub use crate::src::src::expr::sqlite3SelectExprHeight;
pub use crate::src::src::expr::sqlite3VectorErrorMsg;
pub use crate::src::src::global::sqlite3CtypeMap;
pub use crate::src::src::global::sqlite3StdType;
pub use crate::src::src::global::sqlite3StdTypeAffinity;
pub use crate::src::src::main::sqlite3IsBinary;
pub use crate::src::src::malloc::sqlite3DbFree;
pub use crate::src::src::malloc::sqlite3DbFreeNN;
pub use crate::src::src::malloc::sqlite3DbMallocRawNN;
pub use crate::src::src::malloc::sqlite3DbMallocZero;
pub use crate::src::src::malloc::sqlite3DbNNFreeNN;
pub use crate::src::src::malloc::sqlite3DbReallocOrFree;
pub use crate::src::src::malloc::sqlite3DbStrDup;
pub use crate::src::src::malloc::sqlite3OomFault;
pub use crate::src::src::mutex_unix::sqlite3_mutex;
pub use crate::src::src::prepare::sqlite3ParserAddCleanup;
pub use crate::src::src::prepare::sqlite3SchemaToIndex;
pub use crate::src::src::random::sqlite3_randomness;
pub use crate::src::src::resolve::sqlite3CreateColumnExpr;
pub use crate::src::src::resolve::sqlite3ExprColUsed;
pub use crate::src::src::resolve::sqlite3MatchEName;
pub use crate::src::src::resolve::sqlite3ResolveOrderGroupBy;
pub use crate::src::src::resolve::sqlite3ResolveSelectNames;
pub use crate::src::src::rowset::sqlite3RowSetClear;
pub use crate::src::src::util::sqlite3_stricmp;
pub use crate::src::src::util::sqlite3_strnicmp;
pub use crate::src::src::util::sqlite3ColumnType;
pub use crate::src::src::util::sqlite3LogEst;
pub use crate::src::src::util::sqlite3LogEstAdd;
pub use crate::src::src::util::sqlite3ProgressCheck;
pub use crate::src::src::util::sqlite3StrICmp;
pub use crate::src::src::util::sqlite3StrIHash;
pub use crate::src::src::util::sqlite3Strlen30;
pub use crate::src::src::walker::sqlite3ExprWalkNoop;
pub use crate::src::src::walker::sqlite3SelectWalkNoop;
pub use crate::src::src::walker::sqlite3WalkExpr;
pub use crate::src::src::walker::sqlite3WalkExprNN;
pub use crate::src::src::walker::sqlite3WalkSelect;
pub use crate::src::src::r#where::sqlite3WhereBegin;
pub use crate::src::src::r#where::sqlite3WhereBreakLabel;
pub use crate::src::src::r#where::sqlite3WhereContinueLabel;
pub use crate::src::src::r#where::sqlite3WhereEnd;
pub use crate::src::src::r#where::sqlite3WhereIsDistinct;
pub use crate::src::src::r#where::sqlite3WhereIsOrdered;
pub use crate::src::src::r#where::sqlite3WhereIsSorted;
pub use crate::src::src::r#where::sqlite3WhereMinMaxOptEarlyOut;
pub use crate::src::src::r#where::sqlite3WhereOrderByLimitOptLabel;
pub use crate::src::src::r#where::sqlite3WhereOutputRowCount;
pub use crate::src::src::window::sqlite3WindowCodeInit;
pub use crate::src::src::window::sqlite3WindowCodeStep;
pub use crate::src::src::window::sqlite3WindowListDelete;
pub use crate::src::src::window::sqlite3WindowRewrite;
pub use crate::src::src::window::sqlite3WindowUnlinkFromSelect;

pub use crate::src::headers::stdlib::Uint8T;
pub use crate::src::headers::stdlib::Uint16T;
pub use crate::src::headers::stdlib::Uint32T;
pub use crate::src::headers::vdbeInt_h::Vdbe;
pub use crate::src::src::vdbe::COLNAME_DECLTYPE;
pub use crate::src::src::vdbe::COLNAME_NAME;
pub use crate::src::src::vdbe::Mem;
pub use crate::src::src::vdbe::P4_COLLSEQ;
pub use crate::src::src::vdbe::P4_FUNCDEF;
pub use crate::src::src::vdbe::P4_INTARRAY;
pub use crate::src::src::vdbe::P4_KEYINFO;
pub use crate::src::src::vdbe::SubProgram;
pub use crate::src::src::vdbe::SubrtnSig;
pub use crate::src::src::vdbe::VdbeOp;
pub use crate::src::src::vdbe::p4union;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp0;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp1;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp2;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp3;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp4;
pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int;
pub use crate::src::src::vdbeaux::sqlite3VdbeAppendP4;
pub use crate::src::src::vdbeaux::sqlite3VdbeChangeOpcode;
pub use crate::src::src::vdbeaux::sqlite3VdbeChangeP2;
pub use crate::src::src::vdbeaux::sqlite3VdbeChangeP4;
pub use crate::src::src::vdbeaux::sqlite3VdbeChangeP5;
pub use crate::src::src::vdbeaux::sqlite3VdbeChangeToNoop;
pub use crate::src::src::vdbeaux::sqlite3VdbeCreate;
pub use crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr;
pub use crate::src::src::vdbeaux::sqlite3VdbeEndCoroutine;
pub use crate::src::src::vdbeaux::sqlite3VdbeExplainPop;
pub use crate::src::src::vdbeaux::sqlite3VdbeGetOp;
pub use crate::src::src::vdbeaux::sqlite3VdbeGoto;
pub use crate::src::src::vdbeaux::sqlite3VdbeJumpHere;
pub use crate::src::src::vdbeaux::sqlite3VdbeJumpHereOrPopInst;
pub use crate::src::src::vdbeaux::sqlite3VdbeMakeLabel;
pub use crate::src::src::vdbeaux::sqlite3VdbeResolveLabel;
pub use crate::src::src::vdbeaux::sqlite3VdbeSetColName;
pub use crate::src::src::vdbeaux::sqlite3VdbeSetNumCols;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CheckOnCtx {
    pub pSrc: *mut crate::src::headers::sqliteInt_h::SrcList,
    pub iJoin: ::core::ffi::c_int,
    pub pParent: *mut CheckOnCtx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WhereConst {
    pub pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pub pOomFault: *mut crate::src::ext::rtree::rtree::U8_0,
    pub nConst: ::core::ffi::c_int,
    pub nChng: ::core::ffi::c_int,
    pub bHasAffBlob: ::core::ffi::c_int,
    pub mExcludeOn: crate::src::ext::rtree::rtree::U32_0,
    pub apExpr: *mut *mut crate::src::headers::sqliteInt_h::Expr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SortCtx {
    pub pOrderBy: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub nOBSat: ::core::ffi::c_int,
    pub iECursor: ::core::ffi::c_int,
    pub regReturn: ::core::ffi::c_int,
    pub labelBkOut: ::core::ffi::c_int,
    pub addrSortIndex: ::core::ffi::c_int,
    pub labelDone: ::core::ffi::c_int,
    pub labelOBLopt: ::core::ffi::c_int,
    pub sortFlags: crate::src::ext::rtree::rtree::U8_0,
    pub pDeferredRowLoad: *mut RowLoadInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RowLoadInfo {
    pub regResult: ::core::ffi::c_int,
    pub ecelFlags: crate::src::ext::rtree::rtree::U8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DistinctCtx {
    pub isTnct: crate::src::ext::rtree::rtree::U8_0,
    pub eTnctType: crate::src::ext::rtree::rtree::U8_0,
    pub tabTnct: ::core::ffi::c_int,
    pub addrTnct: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubstContext {
    pub pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pub iTable: ::core::ffi::c_int,
    pub iNewTable: ::core::ffi::c_int,
    pub isOuterJoin: ::core::ffi::c_int,
    pub nSelDepth: ::core::ffi::c_int,
    pub pEList: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub pCList: *mut crate::src::headers::sqliteInt_h::ExprList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub i: crate::src::ext::rtree::rtree::U8_0,
    pub nChar: crate::src::ext::rtree::rtree::U8_0,
    pub code: crate::src::ext::rtree::rtree::U8_0,
}

pub const SORTFLAG_UseSorter: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

unsafe extern "C" fn clearSelect(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut p: *mut crate::src::headers::sqliteInt_h::Select,
    mut bFree: ::core::ffi::c_int,
) {
    while !p.is_null() {
        let pPrior: *mut crate::src::headers::sqliteInt_h::Select = (*p).pPrior;
        crate::src::src::expr::sqlite3ExprListDelete(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*p).pEList as *mut crate::src::headers::sqliteInt_h::ExprList,
        );
        crate::src::src::build::sqlite3SrcListDelete(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*p).pSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
        );
        crate::src::src::expr::sqlite3ExprDelete(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*p).pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
        );
        crate::src::src::expr::sqlite3ExprListDelete(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*p).pGroupBy as *mut crate::src::headers::sqliteInt_h::ExprList,
        );
        crate::src::src::expr::sqlite3ExprDelete(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*p).pHaving as *mut crate::src::headers::sqliteInt_h::Expr,
        );
        crate::src::src::expr::sqlite3ExprListDelete(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*p).pOrderBy as *mut crate::src::headers::sqliteInt_h::ExprList,
        );
        crate::src::src::expr::sqlite3ExprDelete(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (*p).pLimit as *mut crate::src::headers::sqliteInt_h::Expr,
        );
        if !(*p).pWith.is_null() {
            crate::src::src::build::sqlite3WithDelete(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*p).pWith as *mut crate::src::headers::sqliteInt_h::With,
            );
        }
        if !(*p).pWinDefn.is_null() {
            crate::src::src::window::sqlite3WindowListDelete(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*p).pWinDefn as *mut crate::src::headers::sqliteInt_h::Window,
            );
        }
        while !(*p).pWin.is_null() {
            crate::src::src::window::sqlite3WindowUnlinkFromSelect(
                (*p).pWin as *mut crate::src::headers::sqliteInt_h::Window,
            );
        }
        if bFree != 0 {
            crate::src::src::malloc::sqlite3DbNNFreeNN(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                p as *mut ::core::ffi::c_void,
            );
        }
        p = pPrior;
        bFree = 1 as ::core::ffi::c_int;
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SelectDestInit(
    pDest: *mut crate::src::headers::sqliteInt_h::SelectDest,
    eDest: ::core::ffi::c_int,
    iParm: ::core::ffi::c_int,
) {
    let __pDest_ref = unsafe { &mut *pDest };
    __pDest_ref.eDest = eDest as u8;
    __pDest_ref.iSDParm = iParm;
    __pDest_ref.iSDParm2 = 0 as ::core::ffi::c_int;
    __pDest_ref.zAffSdst = ::core::ptr::null_mut::<::core::ffi::c_char>();
    __pDest_ref.iSdst = 0 as ::core::ffi::c_int;
    __pDest_ref.nSdst = 0 as ::core::ffi::c_int;
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SelectNew(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut pEList: *mut crate::src::headers::sqliteInt_h::ExprList,
    mut pSrc: *mut crate::src::headers::sqliteInt_h::SrcList,
    pWhere: *mut crate::src::headers::sqliteInt_h::Expr,
    pGroupBy: *mut crate::src::headers::sqliteInt_h::ExprList,
    pHaving: *mut crate::src::headers::sqliteInt_h::Expr,
    pOrderBy: *mut crate::src::headers::sqliteInt_h::ExprList,
    selFlags: crate::src::ext::rtree::rtree::U32_0,
    pLimit: *mut crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::Select {
    let mut pNew: *mut crate::src::headers::sqliteInt_h::Select;
    let mut pAllocated: *mut crate::src::headers::sqliteInt_h::Select;
    let mut standin: crate::src::headers::sqliteInt_h::Select = unsafe { ::core::mem::zeroed() };
    let __pParse_ref = unsafe { &mut *pParse };
    pNew = crate::src::src::malloc::sqlite3DbMallocRawNN(
        __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Select>()
            as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::headers::sqliteInt_h::Select;
    pAllocated = pNew;
    if pNew.is_null() {
        pNew = &raw mut standin;
    }
    if pEList.is_null() {
        pEList = crate::src::src::expr::sqlite3ExprListAppend(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                as *mut crate::src::headers::sqliteInt_h::ExprList,
            crate::src::src::expr::sqlite3Expr(
                __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                crate::src::parse::TK_ASTERISK,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ) as *mut crate::src::headers::sqliteInt_h::Expr
                as *mut crate::src::headers::sqliteInt_h::Expr,
        ) as *mut crate::src::headers::sqliteInt_h::ExprList;
    }
    (*pNew).pEList = pEList;
    (*pNew).op = crate::src::parse::TK_SELECT_1 as u8;
    (*pNew).selFlags = selFlags;
    (*pNew).iLimit = 0 as ::core::ffi::c_int;
    (*pNew).iOffset = 0 as ::core::ffi::c_int;
    __pParse_ref.nSelect += 1;
    (*pNew).selId = __pParse_ref.nSelect as crate::src::ext::rtree::rtree::U32_0;
    (*pNew).addrOpenEphm[0 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
    (*pNew).addrOpenEphm[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
    (*pNew).nSelectRow = 0 as crate::src::headers::sqliteInt_h::LogEst;
    if pSrc.is_null() {
        pSrc = crate::src::src::malloc::sqlite3DbMallocZero(
            __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            crate::src::headers::sqliteInt_h::SZ_SRCLIST_1 as crate::src::ext::rtree::rtree::U64_0,
        ) as *mut crate::src::headers::sqliteInt_h::SrcList;
    }
    (*pNew).pSrc = pSrc;
    (*pNew).pWhere = pWhere;
    (*pNew).pGroupBy = pGroupBy;
    (*pNew).pHaving = pHaving;
    (*pNew).pOrderBy = pOrderBy;
    (*pNew).pPrior = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    (*pNew).pNext = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    (*pNew).pLimit = pLimit;
    (*pNew).pWith = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::With>();
    (*pNew).pWin = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Window>();
    (*pNew).pWinDefn = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Window>();
    if (*__pParse_ref.db).mallocFailed != 0 {
        clearSelect(
            __pParse_ref.db,
            pNew,
            (pNew != &raw mut standin) as ::core::ffi::c_int,
        );
        pAllocated = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    }
    pAllocated
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SelectDelete(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *mut crate::src::headers::sqliteInt_h::Select,
) {
    if !p.is_null() {
        clearSelect(db, p, 1 as ::core::ffi::c_int);
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SelectDeleteGeneric(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    p: *mut ::core::ffi::c_void,
) {
    if !p.is_null() {
        clearSelect(
            db,
            p as *mut crate::src::headers::sqliteInt_h::Select,
            1 as ::core::ffi::c_int,
        );
    }
}

unsafe extern "C" fn findRightmost(
    mut p: *mut crate::src::headers::sqliteInt_h::Select,
) -> *mut crate::src::headers::sqliteInt_h::Select {
    while !(*p).pNext.is_null() {
        p = (*p).pNext;
    }
    p
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3JoinType(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pA: *mut crate::src::headers::sqliteInt_h::Token,
    pB: *mut crate::src::headers::sqliteInt_h::Token,
    pC: *mut crate::src::headers::sqliteInt_h::Token,
) -> ::core::ffi::c_int {
    let mut jointype: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut apAll: [*mut crate::src::headers::sqliteInt_h::Token; 3] =
        [::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>(); 3];
    let mut p: *mut crate::src::headers::sqliteInt_h::Token;
    static mut zKeyText: [::core::ffi::c_char; 34] = unsafe {
        ::core::mem::transmute::<[u8; 34], [::core::ffi::c_char; 34]>(
            *b"naturaleftouterightfullinnercross\0",
        )
    };
    static mut aKeyword: [C2RustUnnamed; 7] = [
        C2RustUnnamed {
            i: 0_u8,
            nChar: 7_u8,
            code: crate::src::headers::sqliteInt_h::JT_NATURAL as u8,
        },
        C2RustUnnamed {
            i: 6_u8,
            nChar: 4_u8,
            code: (crate::src::headers::sqliteInt_h::JT_LEFT
                | crate::src::headers::sqliteInt_h::JT_OUTER) as u8,
        },
        C2RustUnnamed {
            i: 10_u8,
            nChar: 5_u8,
            code: crate::src::headers::sqliteInt_h::JT_OUTER as u8,
        },
        C2RustUnnamed {
            i: 14_u8,
            nChar: 5_u8,
            code: (crate::src::headers::sqliteInt_h::JT_RIGHT
                | crate::src::headers::sqliteInt_h::JT_OUTER) as u8,
        },
        C2RustUnnamed {
            i: 19_u8,
            nChar: 4_u8,
            code: (crate::src::headers::sqliteInt_h::JT_LEFT
                | crate::src::headers::sqliteInt_h::JT_RIGHT
                | crate::src::headers::sqliteInt_h::JT_OUTER) as u8,
        },
        C2RustUnnamed {
            i: 23_u8,
            nChar: 5_u8,
            code: crate::src::headers::sqliteInt_h::JT_INNER as u8,
        },
        C2RustUnnamed {
            i: 28_u8,
            nChar: 5_u8,
            code: (crate::src::headers::sqliteInt_h::JT_INNER
                | crate::src::headers::sqliteInt_h::JT_CROSS) as u8,
        },
    ];
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    apAll[0 as ::core::ffi::c_int as usize] = pA;
    apAll[1 as ::core::ffi::c_int as usize] = pB;
    apAll[2 as ::core::ffi::c_int as usize] = pC;
    i = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int && !apAll[i as usize].is_null() {
        p = apAll[i as usize];
        j = 0 as ::core::ffi::c_int;
        while j
            < (::core::mem::size_of::<[C2RustUnnamed; 7]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2RustUnnamed>() as usize)
                as ::core::ffi::c_int
        {
            let __p_ref = unsafe { &*p };
            if __p_ref.n == aKeyword[j as usize].nChar as ::core::ffi::c_uint
                && crate::src::src::util::sqlite3_strnicmp(
                    __p_ref.z as *mut ::core::ffi::c_char,
                    (&raw const zKeyText as *const ::core::ffi::c_char).offset(
                        (*(&raw const aKeyword as *const C2RustUnnamed).offset(j as isize)).i
                            as isize,
                    ) as *const ::core::ffi::c_char,
                    __p_ref.n as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
            {
                jointype |= aKeyword[j as usize].code as ::core::ffi::c_int;
                break;
            } else {
                j += 1;
            }
        }
        if j >= (::core::mem::size_of::<[C2RustUnnamed; 7]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed>() as usize)
            as ::core::ffi::c_int
        {
            jointype |= crate::src::headers::sqliteInt_h::JT_ERROR;
            break;
        } else {
            i += 1;
        }
    }
    if jointype
        & (crate::src::headers::sqliteInt_h::JT_INNER | crate::src::headers::sqliteInt_h::JT_OUTER)
        == crate::src::headers::sqliteInt_h::JT_INNER | crate::src::headers::sqliteInt_h::JT_OUTER
        || jointype & crate::src::headers::sqliteInt_h::JT_ERROR != 0 as ::core::ffi::c_int
        || jointype
            & (crate::src::headers::sqliteInt_h::JT_OUTER
                | crate::src::headers::sqliteInt_h::JT_LEFT
                | crate::src::headers::sqliteInt_h::JT_RIGHT)
            == crate::src::headers::sqliteInt_h::JT_OUTER
    {
        let mut zSp1: *const ::core::ffi::c_char =
            b" \0" as *const u8 as *const ::core::ffi::c_char;
        let mut zSp2: *const ::core::ffi::c_char =
            b" \0" as *const u8 as *const ::core::ffi::c_char;
        if pB.is_null() {
            zSp1 = zSp1.offset(1);
        }
        if pC.is_null() {
            zSp2 = zSp2.offset(1);
        }
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"unknown join type: %T%s%T%s%T\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::src::src::printf::PrintfArg::Token(pA),
                crate::src::src::printf::PrintfArg::Str(zSp1 as *mut ::core::ffi::c_char),
                crate::src::src::printf::PrintfArg::Token(pB),
                crate::src::src::printf::PrintfArg::Str(zSp2 as *mut ::core::ffi::c_char),
                crate::src::src::printf::PrintfArg::Token(pC),
            ],
        );
        jointype = crate::src::headers::sqliteInt_h::JT_INNER;
    }
    jointype
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ColumnIndex(
    pTab: *mut crate::src::headers::sqliteInt_h::Table,
    zCol: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    
    
    
    let h: crate::src::ext::rtree::rtree::U8_0 = crate::src::src::util::sqlite3StrIHash(zCol);
    let __pTab_ref = unsafe { &*pTab };
    let aCol: *const crate::src::headers::sqliteInt_h::Column = __pTab_ref.aCol;
    let nCol: ::core::ffi::c_int = __pTab_ref.nCol as ::core::ffi::c_int;
    i = __pTab_ref.aHx[(h as usize)
        .wrapping_rem(::core::mem::size_of::<[crate::src::ext::rtree::rtree::U8_0; 16]>() as usize)
        as usize] as ::core::ffi::c_int;
    if (*aCol.offset(i as isize)).hName as ::core::ffi::c_int == h as ::core::ffi::c_int
        && crate::src::src::util::sqlite3StrICmp((*aCol.offset(i as isize)).zCnName, zCol)
            == 0 as ::core::ffi::c_int
    {
        return i;
    }
    i = 0 as ::core::ffi::c_int;
    loop {
        if (*aCol.offset(i as isize)).hName as ::core::ffi::c_int == h as ::core::ffi::c_int
            && crate::src::src::util::sqlite3StrICmp((*aCol.offset(i as isize)).zCnName, zCol)
                == 0 as ::core::ffi::c_int
        {
            return i;
        }
        i += 1;
        if i >= nCol {
            break;
        }
    }
    -(1 as ::core::ffi::c_int)
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SrcItemColumnUsed(
    pItem: *mut crate::src::headers::sqliteInt_h::SrcItem,
    iCol: ::core::ffi::c_int,
) {
    if (*pItem).fg.isNestedFrom() != 0 {
        
        let pResults: *mut crate::src::headers::sqliteInt_h::ExprList = (*(*(*pItem).u4.pSubq).pSelect).pEList;
        let fresh5 = &mut (*(&raw mut (*pResults).a
            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(iCol as isize))
        .fg;
        (*fresh5).set_bUsed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}

unsafe extern "C" fn tableAndColumnIndex(
    pSrc: *mut crate::src::headers::sqliteInt_h::SrcList,
    iStart: ::core::ffi::c_int,
    iEnd: ::core::ffi::c_int,
    zCol: *const ::core::ffi::c_char,
    piTab: *mut ::core::ffi::c_int,
    piCol: *mut ::core::ffi::c_int,
    bIgnoreHidden: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let mut iCol: ::core::ffi::c_int;
    i = iStart;
    while i <= iEnd {
        iCol = sqlite3ColumnIndex(
            (*(&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                .offset(i as isize))
            .pSTab,
            zCol,
        );
        if iCol >= 0 as ::core::ffi::c_int
            && (bIgnoreHidden == 0 as ::core::ffi::c_int
                || ((*(*(*(&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(i as isize))
                .pSTab)
                    .aCol
                    .offset(iCol as isize))
                .colFlags as ::core::ffi::c_int
                    & crate::src::headers::sqliteInt_h::COLFLAG_HIDDEN
                    != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int)
        {
            if !piTab.is_null() {
                sqlite3SrcItemColumnUsed(
                    (&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                        .offset(i as isize)
                        as *mut crate::src::headers::sqliteInt_h::SrcItem,
                    iCol,
                );
                *piTab = i;
                *piCol = iCol;
            }
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    0 as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SetJoinExpr(
    mut p: *mut crate::src::headers::sqliteInt_h::Expr,
    iTable: ::core::ffi::c_int,
    joinFlag: crate::src::ext::rtree::rtree::U32_0,
) {
    while !p.is_null() {
        (*p).flags |= joinFlag;
        (*p).w.iJoin = iTable;
        if (*p).flags
            & crate::src::headers::sqliteInt_h::EP_xIsSelect as crate::src::ext::rtree::rtree::U32_0
            == 0 as crate::src::ext::rtree::rtree::U32_0
            && !(*p).x.pList.is_null() {
                let mut i: ::core::ffi::c_int;
                i = 0 as ::core::ffi::c_int;
                while i < (*(*p).x.pList).nExpr {
                    sqlite3SetJoinExpr(
                        (*(&raw mut (*(*p).x.pList).a
                            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                            .offset(i as isize))
                        .pExpr,
                        iTable,
                        joinFlag,
                    );
                    i += 1;
                }
            }
        sqlite3SetJoinExpr((*p).pLeft, iTable, joinFlag);
        p = (*p).pRight;
    }
}

unsafe extern "C" fn unsetJoinExpr(
    mut p: *mut crate::src::headers::sqliteInt_h::Expr,
    iTable: ::core::ffi::c_int,
    nullable: ::core::ffi::c_int,
) {
    while !p.is_null() {
        if iTable < 0 as ::core::ffi::c_int
            || (*p).flags & 0x1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
                && (*p).w.iJoin == iTable
        {
            (*p).flags &= !((0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int)
                as crate::src::ext::rtree::rtree::U32_0);
            if iTable >= 0 as ::core::ffi::c_int {
                (*p).flags |= 0x2 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
            }
        }
        if (*p).op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN_1
            && (*p).iTable == iTable
            && nullable == 0
        {
            (*p).flags &= !(0x200000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0);
        }
        if (*p).op as ::core::ffi::c_int == crate::src::parse::TK_FUNCTION
            && !(*p).x.pList.is_null() {
                let mut i: ::core::ffi::c_int;
                i = 0 as ::core::ffi::c_int;
                while i < (*(*p).x.pList).nExpr {
                    unsetJoinExpr(
                        (*(&raw mut (*(*p).x.pList).a
                            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                            .offset(i as isize))
                        .pExpr,
                        iTable,
                        nullable,
                    );
                    i += 1;
                }
            }
        unsetJoinExpr((*p).pLeft, iTable, nullable);
        p = (*p).pRight;
    }
}

unsafe extern "C" fn sqlite3ProcessJoin(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    let mut pLeft: *mut crate::src::headers::sqliteInt_h::SrcItem;
    let mut pRight: *mut crate::src::headers::sqliteInt_h::SrcItem;
    let pSrc: *mut crate::src::headers::sqliteInt_h::SrcList = (*p).pSrc;
    pLeft = (&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
        .offset(0_isize) as *mut crate::src::headers::sqliteInt_h::SrcItem;
    pRight = pLeft.offset(1_isize) as *mut crate::src::headers::sqliteInt_h::SrcItem;
    i = 0 as ::core::ffi::c_int;
    while i < (*pSrc).nSrc - 1 as ::core::ffi::c_int {
        let pRightTab: *mut crate::src::headers::sqliteInt_h::Table = (*pRight).pSTab;
        let joinType: crate::src::ext::rtree::rtree::U32_0;
        if !((*pLeft).pSTab.is_null() || pRightTab.is_null()) {
            let __pRight_ref = unsafe { &mut *pRight };
            joinType = (if __pRight_ref.fg.jointype as ::core::ffi::c_int
                & crate::src::headers::sqliteInt_h::JT_OUTER
                != 0 as ::core::ffi::c_int
            {
                crate::src::headers::sqliteInt_h::EP_OuterON
            } else {
                crate::src::headers::sqliteInt_h::EP_InnerON
            }) as crate::src::ext::rtree::rtree::U32_0;
            if __pRight_ref.fg.jointype as ::core::ffi::c_int
                & crate::src::headers::sqliteInt_h::JT_NATURAL
                != 0
            {
                let mut pUsing: *mut crate::src::headers::sqliteInt_h::IdList =
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::IdList>();
                if __pRight_ref.fg.isUsing() as ::core::ffi::c_int != 0
                    || !__pRight_ref.u3.pOn.is_null()
                {
                    crate::src::src::util::sqlite3ErrorMsg_args(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        b"a NATURAL join may not have an ON or USING clause\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                    return 1 as ::core::ffi::c_int;
                }
                j = 0 as ::core::ffi::c_int;
                while j < (*pRightTab).nCol as ::core::ffi::c_int {
                    let zName: *mut ::core::ffi::c_char;
                    if ((*(*pRightTab).aCol.offset(j as isize)).colFlags as ::core::ffi::c_int
                        & crate::src::headers::sqliteInt_h::COLFLAG_HIDDEN == 0 as ::core::ffi::c_int)
                    {
                        zName = (*(*pRightTab).aCol.offset(j as isize)).zCnName;
                        if tableAndColumnIndex(
                            pSrc,
                            0 as ::core::ffi::c_int,
                            i,
                            zName,
                            ::core::ptr::null_mut::<::core::ffi::c_int>(),
                            ::core::ptr::null_mut::<::core::ffi::c_int>(),
                            1 as ::core::ffi::c_int,
                        ) != 0
                        {
                            pUsing = crate::src::src::build::sqlite3IdListAppend(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                pUsing as *mut crate::src::headers::sqliteInt_h::IdList,
                                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                                    as *mut crate::src::headers::sqliteInt_h::Token,
                            )
                                as *mut crate::src::headers::sqliteInt_h::IdList;
                            if !pUsing.is_null() {
                                let fresh4 = &mut (*(&raw mut (*pUsing).a
                                    as *mut crate::src::headers::sqliteInt_h::IdList_item)
                                    .offset(((*pUsing).nId - 1 as ::core::ffi::c_int) as isize))
                                .zName;
                                *fresh4 = crate::src::src::malloc::sqlite3DbStrDup(
                                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                    zName,
                                );
                            }
                        }
                    }
                    j += 1;
                }
                if !pUsing.is_null() {
                    (*pRight)
                        .fg
                        .set_isUsing(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*pRight)
                        .fg
                        .set_isSynthUsing(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    __pRight_ref.u3.pUsing = pUsing;
                }
                if (*pParse).nErr != 0 {
                    return 1 as ::core::ffi::c_int;
                }
            }
            if __pRight_ref.fg.isUsing() != 0 {
                let pList: *mut crate::src::headers::sqliteInt_h::IdList =
                    __pRight_ref.u3.pUsing;
                let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
                j = 0 as ::core::ffi::c_int;
                while j < (*pList).nId {
                    
                    let mut iLeft: ::core::ffi::c_int = 0;
                    let mut iLeftCol: ::core::ffi::c_int = 0;
                    
                    let mut pE1: *mut crate::src::headers::sqliteInt_h::Expr;
                    
                    
                    let zName_0: *mut ::core::ffi::c_char = (*(&raw mut (*pList).a
                        as *mut crate::src::headers::sqliteInt_h::IdList_item)
                        .offset(j as isize))
                    .zName;
                    let iRightCol: ::core::ffi::c_int = sqlite3ColumnIndex(pRightTab, zName_0);
                    if iRightCol < 0 as ::core::ffi::c_int
                        || tableAndColumnIndex(
                            pSrc,
                            0 as ::core::ffi::c_int,
                            i,
                            zName_0,
                            &raw mut iLeft,
                            &raw mut iLeftCol,
                            __pRight_ref.fg.isSynthUsing() as ::core::ffi::c_int,
                        ) == 0 as ::core::ffi::c_int
                    {
                        crate::src::src::util::sqlite3ErrorMsg_args(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            b"cannot join using column %s - column not present in both tables\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            &[crate::src::src::printf::PrintfArg::Str(
                                zName_0 as *mut ::core::ffi::c_char,
                            )],
                        );
                        return 1 as ::core::ffi::c_int;
                    }
                    pE1 = crate::src::src::resolve::sqlite3CreateColumnExpr(
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        pSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
                        iLeft,
                        iLeftCol,
                    ) as *mut crate::src::headers::sqliteInt_h::Expr;
                    let __pSrc_ref = unsafe { &mut *pSrc };
                    sqlite3SrcItemColumnUsed(
                        (&raw mut __pSrc_ref.a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                            .offset(iLeft as isize)
                            as *mut crate::src::headers::sqliteInt_h::SrcItem,
                        iLeftCol,
                    );
                    if (*(&raw mut __pSrc_ref.a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                        .offset(0_isize))
                    .fg
                    .jointype as ::core::ffi::c_int
                        & crate::src::headers::sqliteInt_h::JT_LTORJ
                        != 0 as ::core::ffi::c_int
                        && (*pParse).nErr == 0 as ::core::ffi::c_int
                    {
                        let mut pFuncArgs: *mut crate::src::headers::sqliteInt_h::ExprList =
                            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
                        static mut tkCoalesce: crate::src::headers::sqliteInt_h::Token =
                            crate::src::headers::sqliteInt_h::Token {
                                z: b"coalesce\0" as *const u8 as *const ::core::ffi::c_char,
                                n: 8 as ::core::ffi::c_uint,
                            };
                        (*pE1).flags |=
                            0x200000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
                        while tableAndColumnIndex(
                            pSrc,
                            iLeft + 1 as ::core::ffi::c_int,
                            i,
                            zName_0,
                            &raw mut iLeft,
                            &raw mut iLeftCol,
                            __pRight_ref.fg.isSynthUsing() as ::core::ffi::c_int,
                        ) != 0 as ::core::ffi::c_int
                        {
                            if (*(&raw mut __pSrc_ref.a
                                as *mut crate::src::headers::sqliteInt_h::SrcItem)
                                .offset(iLeft as isize))
                            .fg
                            .isUsing() as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                                || crate::src::src::build::sqlite3IdListIndex(
                                    (*(&raw mut __pSrc_ref.a
                                        as *mut crate::src::headers::sqliteInt_h::SrcItem)
                                        .offset(iLeft as isize))
                                    .u3
                                    .pUsing
                                        as *mut crate::src::headers::sqliteInt_h::IdList,
                                    zName_0,
                                ) < 0 as ::core::ffi::c_int
                            {
                                crate::src::src::util::sqlite3ErrorMsg_args(
                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                    b"ambiguous reference to %s in USING()\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    &[crate::src::src::printf::PrintfArg::Str(
                                        zName_0 as *mut ::core::ffi::c_char,
                                    )],
                                );
                                break;
                            } else {
                                pFuncArgs = crate::src::src::expr::sqlite3ExprListAppend(
                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                    pFuncArgs as *mut crate::src::headers::sqliteInt_h::ExprList,
                                    pE1 as *mut crate::src::headers::sqliteInt_h::Expr,
                                )
                                    as *mut crate::src::headers::sqliteInt_h::ExprList;
                                pE1 = crate::src::src::resolve::sqlite3CreateColumnExpr(
                                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                    pSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
                                    iLeft,
                                    iLeftCol,
                                )
                                    as *mut crate::src::headers::sqliteInt_h::Expr;
                                sqlite3SrcItemColumnUsed(
                                    (&raw mut __pSrc_ref.a
                                        as *mut crate::src::headers::sqliteInt_h::SrcItem)
                                        .offset(iLeft as isize)
                                        as *mut crate::src::headers::sqliteInt_h::SrcItem,
                                    iLeftCol,
                                );
                            }
                        }
                        if !pFuncArgs.is_null() {
                            pFuncArgs = crate::src::src::expr::sqlite3ExprListAppend(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                pFuncArgs as *mut crate::src::headers::sqliteInt_h::ExprList,
                                pE1 as *mut crate::src::headers::sqliteInt_h::Expr,
                            )
                                as *mut crate::src::headers::sqliteInt_h::ExprList;
                            pE1 = crate::src::src::expr::sqlite3ExprFunction(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                pFuncArgs as *mut crate::src::headers::sqliteInt_h::ExprList,
                                &raw const tkCoalesce as *const _
                                    as *const crate::src::headers::sqliteInt_h::Token,
                                0 as ::core::ffi::c_int,
                            )
                                as *mut crate::src::headers::sqliteInt_h::Expr;
                            if !pE1.is_null() {
                                (*pE1).affExpr = crate::src::headers::sqliteInt_h::SQLITE_AFF_DEFER
                                    as ::core::ffi::c_char;
                            }
                        }
                    } else if (*(&raw mut __pSrc_ref.a
                        as *mut crate::src::headers::sqliteInt_h::SrcItem)
                        .offset((i + 1 as ::core::ffi::c_int) as isize))
                    .fg
                    .jointype as ::core::ffi::c_int
                        & crate::src::headers::sqliteInt_h::JT_LEFT
                        != 0 as ::core::ffi::c_int
                        && (*pParse).nErr == 0 as ::core::ffi::c_int
                    {
                        (*pE1).flags |=
                            0x200000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
                    }
                    let pE2: *mut crate::src::headers::sqliteInt_h::Expr = crate::src::src::resolve::sqlite3CreateColumnExpr(
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        pSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
                        i + 1 as ::core::ffi::c_int,
                        iRightCol,
                    ) as *mut crate::src::headers::sqliteInt_h::Expr;
                    sqlite3SrcItemColumnUsed(pRight, iRightCol);
                    let pEq: *mut crate::src::headers::sqliteInt_h::Expr = crate::src::src::expr::sqlite3PExpr(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        crate::src::parse::TK_EQ,
                        pE1 as *mut crate::src::headers::sqliteInt_h::Expr,
                        pE2 as *mut crate::src::headers::sqliteInt_h::Expr,
                    ) as *mut crate::src::headers::sqliteInt_h::Expr;
                    if !pEq.is_null() {
                        (*pEq).flags |= joinType;
                        (*pEq).w.iJoin = (*pE2).iTable;
                    }
                    (*p).pWhere = crate::src::src::expr::sqlite3ExprAnd(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        (*p).pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
                        pEq as *mut crate::src::headers::sqliteInt_h::Expr,
                    )
                        as *mut crate::src::headers::sqliteInt_h::Expr;
                    j += 1;
                }
            } else if !__pRight_ref.u3.pOn.is_null() {
                sqlite3SetJoinExpr(__pRight_ref.u3.pOn, __pRight_ref.iCursor, joinType);
                let __p_ref = unsafe { &mut *p };
                __p_ref.pWhere = crate::src::src::expr::sqlite3ExprAnd(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    __p_ref.pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
                    __pRight_ref.u3.pOn as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::Expr;
                __pRight_ref.u3.pOn =
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
                (*pRight)
                    .fg
                    .set_isOn(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                __p_ref.selFlags |= crate::src::headers::sqliteInt_h::SF_OnToWhere
                    as crate::src::ext::rtree::rtree::U32_0;
            }
        }
        i += 1;
        pRight = pRight.offset(1);
        pLeft = pLeft.offset(1);
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn innerLoopLoadRow(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pSelect: *mut crate::src::headers::sqliteInt_h::Select,
    pInfo: *mut RowLoadInfo,
) {
    crate::src::src::expr::sqlite3ExprCodeExprList(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        (*pSelect).pEList as *mut crate::src::headers::sqliteInt_h::ExprList,
        (*pInfo).regResult,
        0 as ::core::ffi::c_int,
        (*pInfo).ecelFlags,
    );
}

unsafe extern "C" fn makeSorterRecord(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pSort: *mut SortCtx,
    pSelect: *mut crate::src::headers::sqliteInt_h::Select,
    regBase: ::core::ffi::c_int,
    nBase: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let nOBSat: ::core::ffi::c_int = (*pSort).nOBSat;
    let __pParse_ref = unsafe { &mut *pParse };
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = __pParse_ref.pVdbe;
    __pParse_ref.nMem += 1;
    let regOut: ::core::ffi::c_int = __pParse_ref.nMem;
    if !(*pSort).pDeferredRowLoad.is_null() {
        innerLoopLoadRow(
            pParse,
            pSelect,
            (*pSort).pDeferredRowLoad as *mut RowLoadInfo,
        );
    }
    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
        v,
        crate::src::headers::opcodes_h::OP_MakeRecord,
        regBase + nOBSat,
        nBase - nOBSat,
        regOut,
    );
    regOut
}

unsafe extern "C" fn pushOntoSorter(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pSort: *mut SortCtx,
    pSelect: *mut crate::src::headers::sqliteInt_h::Select,
    regData: ::core::ffi::c_int,
    regOrigData: ::core::ffi::c_int,
    nData: ::core::ffi::c_int,
    nPrefixReg: ::core::ffi::c_int,
) {
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    let __pSort_ref = unsafe { &mut *pSort };
    let bSeq: ::core::ffi::c_int = (__pSort_ref.sortFlags as ::core::ffi::c_int
        & SORTFLAG_UseSorter
        == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let nExpr: ::core::ffi::c_int = (*__pSort_ref.pOrderBy).nExpr;
    let nBase: ::core::ffi::c_int = nExpr + bSeq + nData;
    let regBase: ::core::ffi::c_int;
    let mut regRecord: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let nOBSat: ::core::ffi::c_int = __pSort_ref.nOBSat;
    let op: ::core::ffi::c_int;
    
    let mut iSkip: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if nPrefixReg != 0 {
        regBase = regData - nPrefixReg;
    } else {
        regBase = (*pParse).nMem + 1 as ::core::ffi::c_int;
        (*pParse).nMem += nBase;
    }
    let iLimit: ::core::ffi::c_int = if (*pSelect).iOffset != 0 {
        (*pSelect).iOffset + 1 as ::core::ffi::c_int
    } else {
        (*pSelect).iLimit
    };
    __pSort_ref.labelDone = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
    );
    crate::src::src::expr::sqlite3ExprCodeExprList(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        __pSort_ref.pOrderBy as *mut crate::src::headers::sqliteInt_h::ExprList,
        regBase,
        regOrigData,
        (crate::src::headers::sqliteInt_h::SQLITE_ECEL_DUP
            | (if regOrigData != 0 {
                crate::src::headers::sqliteInt_h::SQLITE_ECEL_REF
            } else {
                0 as ::core::ffi::c_int
            })) as u8,
    );
    if bSeq != 0 {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Sequence,
            __pSort_ref.iECursor,
            regBase + nExpr,
        );
    }
    if nPrefixReg == 0 as ::core::ffi::c_int && nData > 0 as ::core::ffi::c_int {
        crate::src::src::expr::sqlite3ExprCodeMove(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            regData,
            regBase + nExpr + bSeq,
            nData,
        );
    }
    if nOBSat > 0 as ::core::ffi::c_int {
        
        let addrFirst: ::core::ffi::c_int;
        
        
        
        
        regRecord = makeSorterRecord(pParse, pSort, pSelect, regBase, nBase);
        let __pParse_ref = unsafe { &mut *pParse };
        let regPrevKey: ::core::ffi::c_int = __pParse_ref.nMem + 1 as ::core::ffi::c_int;
        __pParse_ref.nMem += __pSort_ref.nOBSat;
        let nKey: ::core::ffi::c_int = nExpr - __pSort_ref.nOBSat + bSeq;
        if bSeq != 0 {
            addrFirst = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                v,
                crate::src::headers::opcodes_h::OP_IfNot,
                regBase + nExpr,
            );
        } else {
            addrFirst = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                v,
                crate::src::headers::opcodes_h::OP_SequenceTest,
                __pSort_ref.iECursor,
            );
        }
        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
            v,
            crate::src::headers::opcodes_h::OP_Compare,
            regPrevKey,
            regBase,
            __pSort_ref.nOBSat,
        );
        let pOp: *mut crate::src::src::vdbe::VdbeOp = crate::src::src::vdbeaux::sqlite3VdbeGetOp(v, __pSort_ref.addrSortIndex)
            as *mut crate::src::src::vdbe::VdbeOp;
        if (*__pParse_ref.db).mallocFailed != 0 {
            return;
        }
        (*pOp).p2 = nKey + nData;
        let pKI: *mut crate::src::headers::sqliteInt_h::KeyInfo = (*pOp).p4.pKeyInfo;
        ::libc::memset(
            (*pKI).aSortFlags as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (*pKI).nKeyField as crate::__stddef_size_t_h::SizeT,
        );
        crate::src::src::vdbeaux::sqlite3VdbeChangeP4(
            v,
            -(1 as ::core::ffi::c_int),
            pKI as *mut ::core::ffi::c_char,
            crate::src::src::vdbe::P4_KEYINFO,
        );
        (*pOp).p4.pKeyInfo = sqlite3KeyInfoFromExprList(
            pParse,
            __pSort_ref.pOrderBy,
            nOBSat,
            (*pKI).nAllField as ::core::ffi::c_int
                - (*pKI).nKeyField as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
        );
        let addrJmp: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v);
        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
            v,
            crate::src::headers::opcodes_h::OP_Jump,
            addrJmp + 1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            addrJmp + 1 as ::core::ffi::c_int,
        );
        __pSort_ref.labelBkOut = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        );
        __pParse_ref.nMem += 1;
        __pSort_ref.regReturn = __pParse_ref.nMem;
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Gosub,
            __pSort_ref.regReturn,
            __pSort_ref.labelBkOut,
        );
        crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
            v,
            crate::src::headers::opcodes_h::OP_ResetSorter,
            __pSort_ref.iECursor,
        );
        if iLimit != 0 {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_IfNot,
                iLimit,
                __pSort_ref.labelDone,
            );
        }
        crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrFirst);
        crate::src::src::expr::sqlite3ExprCodeMove(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            regBase,
            regPrevKey,
            __pSort_ref.nOBSat,
        );
        crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrJmp);
    }
    if iLimit != 0 {
        let iCsr: ::core::ffi::c_int = __pSort_ref.iECursor;
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_IfNotZero,
            iLimit,
            crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v) + 4 as ::core::ffi::c_int,
        );
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Last,
            iCsr,
            0 as ::core::ffi::c_int,
        );
        iSkip = crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
            v,
            crate::src::headers::opcodes_h::OP_IdxLE,
            iCsr,
            0 as ::core::ffi::c_int,
            regBase + nOBSat,
            nExpr - nOBSat,
        );
        crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
            v,
            crate::src::headers::opcodes_h::OP_Delete,
            iCsr,
        );
    }
    if regRecord == 0 as ::core::ffi::c_int {
        regRecord = makeSorterRecord(pParse, pSort, pSelect, regBase, nBase);
    }
    if __pSort_ref.sortFlags as ::core::ffi::c_int & SORTFLAG_UseSorter != 0 {
        op = crate::src::headers::opcodes_h::OP_SorterInsert;
    } else {
        op = crate::src::headers::opcodes_h::OP_IdxInsert;
    }
    crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
        v,
        op,
        __pSort_ref.iECursor,
        regRecord,
        regBase + nOBSat,
        nBase - nOBSat,
    );
    if iSkip != 0 {
        crate::src::src::vdbeaux::sqlite3VdbeChangeP2(
            v,
            iSkip,
            if __pSort_ref.labelOBLopt != 0 {
                __pSort_ref.labelOBLopt
            } else {
                crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v)
            },
        );
    }
}

unsafe extern "C" fn codeOffset(
    v: *mut crate::src::headers::vdbeInt_h::Vdbe,
    iOffset: ::core::ffi::c_int,
    iContinue: ::core::ffi::c_int,
) {
    if iOffset > 0 as ::core::ffi::c_int {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
            v,
            crate::src::headers::opcodes_h::OP_IfPos,
            iOffset,
            iContinue,
            1 as ::core::ffi::c_int,
        );
    }
}

unsafe extern "C" fn codeDistinct(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    eTnctType: ::core::ffi::c_int,
    iTab: ::core::ffi::c_int,
    addrRepeat: ::core::ffi::c_int,
    pEList: *mut crate::src::headers::sqliteInt_h::ExprList,
    regElem: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut iRet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let nResultCol: ::core::ffi::c_int = (*pEList).nExpr;
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    match eTnctType {
        crate::src::headers::sqliteInt_h::WHERE_DISTINCT_ORDERED => {
            let mut i: ::core::ffi::c_int;
            
            
            let regPrev: ::core::ffi::c_int = (*pParse).nMem + 1 as ::core::ffi::c_int;
            iRet = regPrev;
            (*pParse).nMem += nResultCol;
            let iJump: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v) + nResultCol;
            i = 0 as ::core::ffi::c_int;
            while i < nResultCol {
                let pColl: *mut crate::src::headers::sqliteInt_h::CollSeq =
                    crate::src::src::expr::sqlite3ExprCollSeq(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        (*(&raw mut (*pEList).a
                            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                            .offset(i as isize))
                        .pExpr
                            as *const crate::src::headers::sqliteInt_h::Expr,
                    ) as *mut crate::src::headers::sqliteInt_h::CollSeq;
                if i < nResultCol - 1 as ::core::ffi::c_int {
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                        v,
                        crate::src::headers::opcodes_h::OP_Ne,
                        regElem + i,
                        iJump,
                        regPrev + i,
                    );
                } else {
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                        v,
                        crate::src::headers::opcodes_h::OP_Eq,
                        regElem + i,
                        addrRepeat,
                        regPrev + i,
                    );
                }
                crate::src::src::vdbeaux::sqlite3VdbeChangeP4(
                    v,
                    -(1 as ::core::ffi::c_int),
                    pColl as *const ::core::ffi::c_char,
                    crate::src::src::vdbe::P4_COLLSEQ,
                );
                crate::src::src::vdbeaux::sqlite3VdbeChangeP5(
                    v,
                    crate::src::headers::sqliteInt_h::SQLITE_NULLEQ as crate::src::fts5::U16_0,
                );
                i += 1;
            }
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_Copy,
                regElem,
                regPrev,
                nResultCol - 1 as ::core::ffi::c_int,
            );
        }
        crate::src::headers::sqliteInt_h::WHERE_DISTINCT_UNIQUE => {}
        _ => {
            let r1: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempReg(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                v,
                crate::src::headers::opcodes_h::OP_Found,
                iTab,
                addrRepeat,
                regElem,
                nResultCol,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_MakeRecord,
                regElem,
                nResultCol,
                r1,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                v,
                crate::src::headers::opcodes_h::OP_IdxInsert,
                iTab,
                r1,
                regElem,
                nResultCol,
            );
            crate::src::src::vdbeaux::sqlite3VdbeChangeP5(
                v,
                crate::src::headers::sqliteInt_h::OPFLAG_USESEEKRESULT as crate::src::fts5::U16_0,
            );
            crate::src::src::expr::sqlite3ReleaseTempReg(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                r1,
            );
            iRet = iTab;
        }
    }
    iRet
}

unsafe extern "C" fn fixDistinctOpenEph(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    eTnctType: ::core::ffi::c_int,
    iVal: ::core::ffi::c_int,
    iOpenEphAddr: ::core::ffi::c_int,
) {
    if (*pParse).nErr == 0 as ::core::ffi::c_int
        && (eTnctType == crate::src::headers::sqliteInt_h::WHERE_DISTINCT_UNIQUE
            || eTnctType == crate::src::headers::sqliteInt_h::WHERE_DISTINCT_ORDERED)
    {
        let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
        crate::src::src::vdbeaux::sqlite3VdbeChangeToNoop(v, iOpenEphAddr);
        if (*(crate::src::src::vdbeaux::sqlite3VdbeGetOp(v, iOpenEphAddr + 1 as ::core::ffi::c_int)
            as *mut crate::src::src::vdbe::VdbeOp))
            .opcode as ::core::ffi::c_int
            == crate::src::headers::opcodes_h::OP_Explain
        {
            crate::src::src::vdbeaux::sqlite3VdbeChangeToNoop(
                v,
                iOpenEphAddr + 1 as ::core::ffi::c_int,
            );
        }
        if eTnctType == crate::src::headers::sqliteInt_h::WHERE_DISTINCT_ORDERED {
            let pOp: *mut crate::src::src::vdbe::VdbeOp =
                crate::src::src::vdbeaux::sqlite3VdbeGetOp(v, iOpenEphAddr)
                    as *mut crate::src::src::vdbe::VdbeOp;
            let __pOp_ref = unsafe { &mut *pOp };
            __pOp_ref.opcode = crate::src::headers::opcodes_h::OP_Null as u8;
            __pOp_ref.p1 = 1 as ::core::ffi::c_int;
            __pOp_ref.p2 = iVal;
        }
    }
}

unsafe extern "C" fn selectInnerLoop(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
    srcTab: ::core::ffi::c_int,
    mut pSort: *mut SortCtx,
    pDistinct: *mut DistinctCtx,
    pDest: *mut crate::src::headers::sqliteInt_h::SelectDest,
    iContinue: ::core::ffi::c_int,
    iBreak: ::core::ffi::c_int,
) {
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    let mut i: ::core::ffi::c_int;
    
    let __pDest_ref = unsafe { &mut *pDest };
    let eDest: ::core::ffi::c_int = __pDest_ref.eDest as ::core::ffi::c_int;
    let iParm: ::core::ffi::c_int = __pDest_ref.iSDParm;
    let mut nResultCol: ::core::ffi::c_int;
    let mut nPrefixReg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sRowLoadInfo: RowLoadInfo = unsafe { ::core::mem::zeroed() };
    
    let mut regOrig: ::core::ffi::c_int;
    let hasDistinct: ::core::ffi::c_int = if !pDistinct.is_null() {
        (*pDistinct).eTnctType as ::core::ffi::c_int
    } else {
        crate::src::headers::sqliteInt_h::WHERE_DISTINCT_NOOP
    };
    if !pSort.is_null() && (*pSort).pOrderBy.is_null() {
        pSort = ::core::ptr::null_mut::<SortCtx>();
    }
    if pSort.is_null() && hasDistinct == 0 {
        codeOffset(v, (*p).iOffset, iContinue);
    }
    nResultCol = (*(*p).pEList).nExpr;
    if __pDest_ref.iSdst == 0 as ::core::ffi::c_int {
        if !pSort.is_null() {
            nPrefixReg = (*(*pSort).pOrderBy).nExpr;
            if (*pSort).sortFlags as ::core::ffi::c_int & SORTFLAG_UseSorter == 0 {
                nPrefixReg += 1;
            }
            (*pParse).nMem += nPrefixReg;
        }
        __pDest_ref.iSdst = (*pParse).nMem + 1 as ::core::ffi::c_int;
        (*pParse).nMem += nResultCol;
    } else if __pDest_ref.iSdst + nResultCol > (*pParse).nMem {
        (*pParse).nMem += nResultCol;
    }
    __pDest_ref.nSdst = nResultCol;
    let regResult: ::core::ffi::c_int = __pDest_ref.iSdst;
    regOrig = regResult;
    if srcTab >= 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < nResultCol {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_Column,
                srcTab,
                i,
                regResult + i,
            );
            i += 1;
        }
    } else if eDest != crate::src::headers::sqliteInt_h::SRT_Exists_1 {
        let mut ecelFlags: crate::src::ext::rtree::rtree::U8_0;
        let pEList: *mut crate::src::headers::sqliteInt_h::ExprList;
        if eDest == crate::src::headers::sqliteInt_h::SRT_Mem_1
            || eDest == crate::src::headers::sqliteInt_h::SRT_Output
            || eDest == crate::src::headers::sqliteInt_h::SRT_Coroutine
        {
            ecelFlags = crate::src::headers::sqliteInt_h::SQLITE_ECEL_DUP as u8;
        } else {
            ecelFlags = 0_u8;
        }
        if !pSort.is_null()
            && hasDistinct == 0 as ::core::ffi::c_int
            && eDest != crate::src::headers::sqliteInt_h::SRT_EphemTab_1
            && eDest != crate::src::headers::sqliteInt_h::SRT_Table
        {
            ecelFlags = (ecelFlags as ::core::ffi::c_int
                | (crate::src::headers::sqliteInt_h::SQLITE_ECEL_OMITREF
                    | crate::src::headers::sqliteInt_h::SQLITE_ECEL_REF))
                as u8;
            i = (*pSort).nOBSat;
            while i < (*(*pSort).pOrderBy).nExpr {
                
                let j: ::core::ffi::c_int = (*(&raw mut (*(*pSort).pOrderBy).a
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(i as isize))
                .u
                .x
                .iOrderByCol as ::core::ffi::c_int;
                if j > 0 as ::core::ffi::c_int {
                    (*(&raw mut (*(*p).pEList).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset((j - 1 as ::core::ffi::c_int) as isize))
                    .u
                    .x
                    .iOrderByCol =
                        (i + 1 as ::core::ffi::c_int - (*pSort).nOBSat) as crate::src::fts5::U16_0;
                }
                i += 1;
            }
            pEList = (*p).pEList;
            i = 0 as ::core::ffi::c_int;
            while i < (*pEList).nExpr {
                if (*(&raw mut (*pEList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(i as isize))
                .u
                .x
                .iOrderByCol as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    nResultCol -= 1;
                    regOrig = 0 as ::core::ffi::c_int;
                }
                i += 1;
            }
        }
        sRowLoadInfo.regResult = regResult;
        sRowLoadInfo.ecelFlags = ecelFlags;
        if (*p).iLimit != 0
            && ecelFlags as ::core::ffi::c_int
                & crate::src::headers::sqliteInt_h::SQLITE_ECEL_OMITREF
                != 0 as ::core::ffi::c_int
            && nPrefixReg > 0 as ::core::ffi::c_int
        {
            (*pSort).pDeferredRowLoad = &raw mut sRowLoadInfo as *mut RowLoadInfo;
            regOrig = 0 as ::core::ffi::c_int;
        } else {
            innerLoopLoadRow(pParse, p, &raw mut sRowLoadInfo);
        }
    }
    if hasDistinct != 0 {
        let __pDistinct_ref = unsafe { &*pDistinct };
        let eType: ::core::ffi::c_int = __pDistinct_ref.eTnctType as ::core::ffi::c_int;
        let mut iTab: ::core::ffi::c_int = __pDistinct_ref.tabTnct;
        iTab = codeDistinct(pParse, eType, iTab, iContinue, (*p).pEList, regResult);
        fixDistinctOpenEph(pParse, eType, iTab, __pDistinct_ref.addrTnct);
        if pSort.is_null() {
            codeOffset(v, (*p).iOffset, iContinue);
        }
    }
    match eDest {
        crate::src::headers::sqliteInt_h::SRT_Union => {
            
            let r1: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempReg(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_MakeRecord,
                regResult,
                nResultCol,
                r1,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                v,
                crate::src::headers::opcodes_h::OP_IdxInsert,
                iParm,
                r1,
                regResult,
                nResultCol,
            );
            crate::src::src::expr::sqlite3ReleaseTempReg(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                r1,
            );
        }
        crate::src::headers::sqliteInt_h::SRT_Except => {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_IdxDelete,
                iParm,
                regResult,
                nResultCol,
            );
        }
        crate::src::headers::sqliteInt_h::SRT_Fifo
        | crate::src::headers::sqliteInt_h::SRT_DistFifo
        | crate::src::headers::sqliteInt_h::SRT_Table
        | crate::src::headers::sqliteInt_h::SRT_EphemTab_1 => {
            let r1_0: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempRange(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                nPrefixReg + 1 as ::core::ffi::c_int,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_MakeRecord,
                regResult,
                nResultCol,
                r1_0 + nPrefixReg,
            );
            if eDest == crate::src::headers::sqliteInt_h::SRT_DistFifo {
                let addr: ::core::ffi::c_int =
                    crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v) + 4 as ::core::ffi::c_int;
                crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                    v,
                    crate::src::headers::opcodes_h::OP_Found,
                    iParm + 1 as ::core::ffi::c_int,
                    addr,
                    r1_0,
                    0 as ::core::ffi::c_int,
                );
                crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                    v,
                    crate::src::headers::opcodes_h::OP_IdxInsert,
                    iParm + 1 as ::core::ffi::c_int,
                    r1_0,
                    regResult,
                    nResultCol,
                );
            }
            if !pSort.is_null() {
                pushOntoSorter(
                    pParse,
                    pSort,
                    p,
                    r1_0 + nPrefixReg,
                    regOrig,
                    1 as ::core::ffi::c_int,
                    nPrefixReg,
                );
            } else {
                let r2: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempReg(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                );
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_NewRowid,
                    iParm,
                    r2,
                );
                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                    v,
                    crate::src::headers::opcodes_h::OP_Insert,
                    iParm,
                    r1_0,
                    r2,
                );
                crate::src::src::vdbeaux::sqlite3VdbeChangeP5(
                    v,
                    crate::src::headers::sqliteInt_h::OPFLAG_APPEND as crate::src::fts5::U16_0,
                );
                crate::src::src::expr::sqlite3ReleaseTempReg(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    r2,
                );
            }
            crate::src::src::expr::sqlite3ReleaseTempRange(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                r1_0,
                nPrefixReg + 1 as ::core::ffi::c_int,
            );
        }
        crate::src::headers::sqliteInt_h::SRT_Upfrom => {
            if !pSort.is_null() {
                pushOntoSorter(pParse, pSort, p, regResult, regOrig, nResultCol, nPrefixReg);
            } else {
                let i2: ::core::ffi::c_int = __pDest_ref.iSDParm2;
                let r1_1: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempReg(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                );
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_IsNull,
                    regResult,
                    iBreak,
                );
                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                    v,
                    crate::src::headers::opcodes_h::OP_MakeRecord,
                    regResult + (i2 < 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    nResultCol - (i2 < 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    r1_1,
                );
                if i2 < 0 as ::core::ffi::c_int {
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                        v,
                        crate::src::headers::opcodes_h::OP_Insert,
                        iParm,
                        r1_1,
                        regResult,
                    );
                } else {
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                        v,
                        crate::src::headers::opcodes_h::OP_IdxInsert,
                        iParm,
                        r1_1,
                        regResult,
                        i2,
                    );
                }
            }
        }
        crate::src::headers::sqliteInt_h::SRT_Set => {
            if !pSort.is_null() {
                pushOntoSorter(pParse, pSort, p, regResult, regOrig, nResultCol, nPrefixReg);
                __pDest_ref.iSDParm2 = 0 as ::core::ffi::c_int;
            } else {
                let r1_2: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempReg(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                );
                crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                    v,
                    crate::src::headers::opcodes_h::OP_MakeRecord,
                    regResult,
                    nResultCol,
                    r1_2,
                    __pDest_ref.zAffSdst,
                    nResultCol,
                );
                crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                    v,
                    crate::src::headers::opcodes_h::OP_IdxInsert,
                    iParm,
                    r1_2,
                    regResult,
                    nResultCol,
                );
                if __pDest_ref.iSDParm2 != 0 {
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                        v,
                        crate::src::headers::opcodes_h::OP_FilterAdd,
                        __pDest_ref.iSDParm2,
                        0 as ::core::ffi::c_int,
                        regResult,
                        nResultCol,
                    );
                    crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        0_u8,
                        b"CREATE BLOOM FILTER\0" as *const u8 as *const ::core::ffi::c_char,
                        &[],
                    );
                }
                crate::src::src::expr::sqlite3ReleaseTempReg(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    r1_2,
                );
            }
        }
        crate::src::headers::sqliteInt_h::SRT_Exists_1 => {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_Integer,
                1 as ::core::ffi::c_int,
                iParm,
            );
        }
        crate::src::headers::sqliteInt_h::SRT_Mem_1 => {
            if !pSort.is_null() {
                pushOntoSorter(pParse, pSort, p, regResult, regOrig, nResultCol, nPrefixReg);
                __pDest_ref.iSDParm = regResult;
            } else if regResult != iParm {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                    v,
                    crate::src::headers::opcodes_h::OP_Copy,
                    regResult,
                    iParm,
                    nResultCol - 1 as ::core::ffi::c_int,
                );
            }
        }
        crate::src::headers::sqliteInt_h::SRT_Coroutine
        | crate::src::headers::sqliteInt_h::SRT_Output => {
            if !pSort.is_null() {
                pushOntoSorter(pParse, pSort, p, regResult, regOrig, nResultCol, nPrefixReg);
            } else if eDest == crate::src::headers::sqliteInt_h::SRT_Coroutine {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                    v,
                    crate::src::headers::opcodes_h::OP_Yield,
                    __pDest_ref.iSDParm,
                );
            } else {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_ResultRow,
                    regResult,
                    nResultCol,
                );
            }
        }
        crate::src::headers::sqliteInt_h::SRT_DistQueue
        | crate::src::headers::sqliteInt_h::SRT_Queue => {
            
            
            
            
            let mut addrTest: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            
            let pSO: *mut crate::src::headers::sqliteInt_h::ExprList = __pDest_ref.pOrderBy;
            let nKey: ::core::ffi::c_int = (*pSO).nExpr;
            let r1_3: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempReg(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            );
            let r2_0: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempRange(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                nKey + 2 as ::core::ffi::c_int,
            );
            let r3: ::core::ffi::c_int = r2_0 + nKey + 1 as ::core::ffi::c_int;
            if eDest == crate::src::headers::sqliteInt_h::SRT_DistQueue {
                addrTest = crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                    v,
                    crate::src::headers::opcodes_h::OP_Found,
                    iParm + 1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    regResult,
                    nResultCol,
                );
            }
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_MakeRecord,
                regResult,
                nResultCol,
                r3,
            );
            if eDest == crate::src::headers::sqliteInt_h::SRT_DistQueue {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_IdxInsert,
                    iParm + 1 as ::core::ffi::c_int,
                    r3,
                );
                crate::src::src::vdbeaux::sqlite3VdbeChangeP5(
                    v,
                    crate::src::headers::sqliteInt_h::OPFLAG_USESEEKRESULT
                        as crate::src::fts5::U16_0,
                );
            }
            i = 0 as ::core::ffi::c_int;
            while i < nKey {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_SCopy,
                    regResult
                        + (*(&raw mut (*pSO).a
                            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                            .offset(i as isize))
                        .u
                        .x
                        .iOrderByCol as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int,
                    r2_0 + i,
                );
                i += 1;
            }
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_Sequence,
                iParm,
                r2_0 + nKey,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_MakeRecord,
                r2_0,
                nKey + 2 as ::core::ffi::c_int,
                r1_3,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                v,
                crate::src::headers::opcodes_h::OP_IdxInsert,
                iParm,
                r1_3,
                r2_0,
                nKey + 2 as ::core::ffi::c_int,
            );
            if addrTest != 0 {
                crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrTest);
            }
            crate::src::src::expr::sqlite3ReleaseTempReg(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                r1_3,
            );
            crate::src::src::expr::sqlite3ReleaseTempRange(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                r2_0,
                nKey + 2 as ::core::ffi::c_int,
            );
        }
        _ => {}
    }
    if pSort.is_null() && (*p).iLimit != 0 {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_DecrJumpZero,
            (*p).iLimit,
            iBreak,
        );
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3KeyInfoAlloc(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    N: ::core::ffi::c_int,
    X: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqliteInt_h::KeyInfo {
    let nExtra: ::core::ffi::c_int = ((N + X) as usize).wrapping_mul(
        (::core::mem::size_of::<*mut crate::src::headers::sqliteInt_h::CollSeq>() as usize)
            .wrapping_add(1_usize),
    ) as ::core::ffi::c_int;
    
    if N + X > 0xffff as ::core::ffi::c_int {
        return crate::src::src::malloc::sqlite3OomFault(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        ) as *mut crate::src::headers::sqliteInt_h::KeyInfo;
    }
    let p: *mut crate::src::headers::sqliteInt_h::KeyInfo = crate::src::src::malloc::sqlite3DbMallocRawNN(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        32_usize
            .wrapping_add(0_usize.wrapping_mul(::core::mem::size_of::<
                *mut crate::src::headers::sqliteInt_h::CollSeq,
            >() as usize))
            .wrapping_add(nExtra as usize) as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::headers::sqliteInt_h::KeyInfo;
    if !p.is_null() {
        let __p_ref = unsafe { &mut *p };
        __p_ref.aSortFlags = (&raw mut __p_ref.aColl
            as *mut *mut crate::src::headers::sqliteInt_h::CollSeq)
            .offset((N + X) as isize)
            as *mut *mut crate::src::headers::sqliteInt_h::CollSeq
            as *mut crate::src::ext::rtree::rtree::U8_0;
        __p_ref.nKeyField = N as crate::src::fts5::U16_0;
        __p_ref.nAllField = (N + X) as crate::src::fts5::U16_0;
        __p_ref.enc = (*db).enc;
        __p_ref.db = db;
        __p_ref.nRef = 1 as crate::src::ext::rtree::rtree::U32_0;
        ::libc::memset(
            &raw mut __p_ref.aColl as *mut *mut crate::src::headers::sqliteInt_h::CollSeq
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            nExtra as crate::__stddef_size_t_h::SizeT,
        );
    } else {
        return crate::src::src::malloc::sqlite3OomFault(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        ) as *mut crate::src::headers::sqliteInt_h::KeyInfo;
    }
    p
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3KeyInfoUnref(
    p: *mut crate::src::headers::sqliteInt_h::KeyInfo,
) {
    if !p.is_null() {
        let __p_ref = unsafe { &mut *p };
        __p_ref.nRef = __p_ref.nRef.wrapping_sub(1);
        if __p_ref.nRef == 0 as crate::src::ext::rtree::rtree::U32_0 {
            crate::src::src::malloc::sqlite3DbNNFreeNN(
                __p_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                p as *mut ::core::ffi::c_void,
            );
        }
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3KeyInfoRef(
    p: *mut crate::src::headers::sqliteInt_h::KeyInfo,
) -> *mut crate::src::headers::sqliteInt_h::KeyInfo {
    if !p.is_null() {
        (*p).nRef = (*p).nRef.wrapping_add(1);
    }
    p
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3KeyInfoFromExprList(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pList: *mut crate::src::headers::sqliteInt_h::ExprList,
    iStart: ::core::ffi::c_int,
    nExtra: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqliteInt_h::KeyInfo {
    
    
    let mut pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item;
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    let mut i: ::core::ffi::c_int;
    let nExpr: ::core::ffi::c_int = (*pList).nExpr;
    let pInfo: *mut crate::src::headers::sqliteInt_h::KeyInfo = sqlite3KeyInfoAlloc(db, nExpr - iStart, nExtra + 1 as ::core::ffi::c_int);
    if !pInfo.is_null() {
        i = iStart;
        pItem = (&raw mut (*pList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(iStart as isize)
            as *mut crate::src::headers::sqliteInt_h::ExprList_item;
        while i < nExpr {
            let fresh15 = &mut *(&raw mut (*pInfo).aColl
                as *mut *mut crate::src::headers::sqliteInt_h::CollSeq)
                .offset((i - iStart) as isize);
            *fresh15 = crate::src::src::expr::sqlite3ExprNNCollSeq(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*pItem).pExpr as *const crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::CollSeq;
            *(*pInfo).aSortFlags.offset((i - iStart) as isize) = (*pItem).fg.sortFlags;
            i += 1;
            pItem = pItem.offset(1);
        }
    }
    pInfo
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SelectOpName(
    id: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let z: *mut ::core::ffi::c_char;
    match id {
        crate::src::parse::TK_ALL => {
            z = b"UNION ALL\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        crate::src::parse::TK_INTERSECT => {
            z = b"INTERSECT\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        crate::src::parse::TK_EXCEPT => {
            z = b"EXCEPT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        _ => {
            z = b"UNION\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
    }
    z
}

unsafe extern "C" fn explainTempTable(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    zUsage: *const ::core::ffi::c_char,
) {
    crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        0_u8,
        b"USE TEMP B-TREE FOR %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::src::src::printf::PrintfArg::Str(
            zUsage as *mut ::core::ffi::c_char,
        )],
    );
}

unsafe extern "C" fn generateSortTail(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
    pSort: *mut SortCtx,
    mut nColumn: ::core::ffi::c_int,
    pDest: *mut crate::src::headers::sqliteInt_h::SelectDest,
) {
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    let __pSort_ref = unsafe { &*pSort };
    let addrBreak: ::core::ffi::c_int = __pSort_ref.labelDone;
    let addrContinue: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
    );
    let addr: ::core::ffi::c_int;
    let mut addrOnce: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    let pOrderBy: *mut crate::src::headers::sqliteInt_h::ExprList = __pSort_ref.pOrderBy;
    let eDest: ::core::ffi::c_int = (*pDest).eDest as ::core::ffi::c_int;
    let iParm: ::core::ffi::c_int = (*pDest).iSDParm;
    let regRow: ::core::ffi::c_int;
    let regRowid: ::core::ffi::c_int;
    let mut iCol: ::core::ffi::c_int;
    
    let iSortTab: ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    let bSeq: ::core::ffi::c_int;
    let nRefKey: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let aOutEx: *mut crate::src::headers::sqliteInt_h::ExprList_item =
        &raw mut (*(*p).pEList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item;
    let nKey: ::core::ffi::c_int = (*pOrderBy).nExpr - __pSort_ref.nOBSat;
    if __pSort_ref.nOBSat == 0 as ::core::ffi::c_int || nKey == 1 as ::core::ffi::c_int {
        crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            0_u8,
            b"USE TEMP B-TREE FOR %sORDER BY\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Str(
                if __pSort_ref.nOBSat != 0 {
                    b"LAST TERM OF \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                } as *mut ::core::ffi::c_char,
            )],
        );
    } else {
        crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            0_u8,
            b"USE TEMP B-TREE FOR LAST %d TERMS OF ORDER BY\0" as *const u8
                as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Int(
                nKey as crate::src::ext::rtree::rtree::I64_0,
            )],
        );
    }
    if __pSort_ref.labelBkOut != 0 {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Gosub,
            __pSort_ref.regReturn,
            __pSort_ref.labelBkOut,
        );
        crate::src::src::vdbeaux::sqlite3VdbeGoto(v, addrBreak);
        crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, __pSort_ref.labelBkOut);
    }
    let iTab: ::core::ffi::c_int = __pSort_ref.iECursor;
    if eDest == crate::src::headers::sqliteInt_h::SRT_Output
        || eDest == crate::src::headers::sqliteInt_h::SRT_Coroutine
        || eDest == crate::src::headers::sqliteInt_h::SRT_Mem_1
    {
        if eDest == crate::src::headers::sqliteInt_h::SRT_Mem_1 && (*p).iOffset != 0 {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_Null,
                0 as ::core::ffi::c_int,
                (*pDest).iSdst,
            );
        }
        regRowid = 0 as ::core::ffi::c_int;
        regRow = (*pDest).iSdst;
    } else {
        regRowid = crate::src::src::expr::sqlite3GetTempReg(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        );
        if eDest == crate::src::headers::sqliteInt_h::SRT_EphemTab_1
            || eDest == crate::src::headers::sqliteInt_h::SRT_Table
        {
            regRow = crate::src::src::expr::sqlite3GetTempReg(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            );
            nColumn = 0 as ::core::ffi::c_int;
        } else {
            regRow = crate::src::src::expr::sqlite3GetTempRange(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                nColumn,
            );
        }
    }
    if __pSort_ref.sortFlags as ::core::ffi::c_int & SORTFLAG_UseSorter != 0 {
        let __pParse_ref = unsafe { &mut *pParse };
        __pParse_ref.nMem += 1;
        let regSortOut: ::core::ffi::c_int = __pParse_ref.nMem;
        let fresh13 = __pParse_ref.nTab;
        __pParse_ref.nTab += 1;
        iSortTab = fresh13;
        if __pSort_ref.labelBkOut != 0 {
            addrOnce = crate::src::src::vdbeaux::sqlite3VdbeAddOp0(
                v,
                crate::src::headers::opcodes_h::OP_Once,
            );
        }
        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
            v,
            crate::src::headers::opcodes_h::OP_OpenPseudo,
            iSortTab,
            regSortOut,
            nKey + 1 as ::core::ffi::c_int + nColumn + nRefKey,
        );
        if addrOnce != 0 {
            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrOnce);
        }
        addr = 1 as ::core::ffi::c_int
            + crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_SorterSort,
                iTab,
                addrBreak,
            );
        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
            v,
            crate::src::headers::opcodes_h::OP_SorterData,
            iTab,
            regSortOut,
            iSortTab,
        );
        bSeq = 0 as ::core::ffi::c_int;
    } else {
        addr = 1 as ::core::ffi::c_int
            + crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_Sort,
                iTab,
                addrBreak,
            );
        codeOffset(v, (*p).iOffset, addrContinue);
        iSortTab = iTab;
        bSeq = 1 as ::core::ffi::c_int;
        if (*p).iOffset > 0 as ::core::ffi::c_int {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_AddImm,
                (*p).iLimit,
                -(1 as ::core::ffi::c_int),
            );
        }
    }
    i = 0 as ::core::ffi::c_int;
    iCol = nKey + bSeq - 1 as ::core::ffi::c_int;
    while i < nColumn {
        if (*aOutEx.offset(i as isize)).u.x.iOrderByCol as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            iCol += 1;
        }
        i += 1;
    }
    i = nColumn - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let iRead: ::core::ffi::c_int;
        if (*aOutEx.offset(i as isize)).u.x.iOrderByCol != 0 {
            iRead = (*aOutEx.offset(i as isize)).u.x.iOrderByCol as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int;
        } else {
            let fresh14 = iCol;
            iCol -= 1;
            iRead = fresh14;
        }
        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
            v,
            crate::src::headers::opcodes_h::OP_Column,
            iSortTab,
            iRead,
            regRow + i,
        );
        i -= 1;
    }
    match eDest {
        crate::src::headers::sqliteInt_h::SRT_Table
        | crate::src::headers::sqliteInt_h::SRT_EphemTab_1 => {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_Column,
                iSortTab,
                nKey + bSeq,
                regRow,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_NewRowid,
                iParm,
                regRowid,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_Insert,
                iParm,
                regRow,
                regRowid,
            );
            crate::src::src::vdbeaux::sqlite3VdbeChangeP5(
                v,
                crate::src::headers::sqliteInt_h::OPFLAG_APPEND as crate::src::fts5::U16_0,
            );
        }
        crate::src::headers::sqliteInt_h::SRT_Set => {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                v,
                crate::src::headers::opcodes_h::OP_MakeRecord,
                regRow,
                nColumn,
                regRowid,
                (*pDest).zAffSdst,
                nColumn,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                v,
                crate::src::headers::opcodes_h::OP_IdxInsert,
                iParm,
                regRowid,
                regRow,
                nColumn,
            );
        }
        crate::src::headers::sqliteInt_h::SRT_Mem_1 => {}
        crate::src::headers::sqliteInt_h::SRT_Upfrom => {
            let i2: ::core::ffi::c_int = (*pDest).iSDParm2;
            let r1: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempReg(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_MakeRecord,
                regRow + (i2 < 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                nColumn - (i2 < 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                r1,
            );
            if i2 < 0 as ::core::ffi::c_int {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                    v,
                    crate::src::headers::opcodes_h::OP_Insert,
                    iParm,
                    r1,
                    regRow,
                );
            } else {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                    v,
                    crate::src::headers::opcodes_h::OP_IdxInsert,
                    iParm,
                    r1,
                    regRow,
                    i2,
                );
            }
        }
        _ => {
            if eDest == crate::src::headers::sqliteInt_h::SRT_Output {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_ResultRow,
                    (*pDest).iSdst,
                    nColumn,
                );
            } else {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                    v,
                    crate::src::headers::opcodes_h::OP_Yield,
                    (*pDest).iSDParm,
                );
            }
        }
    }
    if regRowid != 0 {
        if eDest == crate::src::headers::sqliteInt_h::SRT_Set {
            crate::src::src::expr::sqlite3ReleaseTempRange(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                regRow,
                nColumn,
            );
        } else {
            crate::src::src::expr::sqlite3ReleaseTempReg(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                regRow,
            );
        }
        crate::src::src::expr::sqlite3ReleaseTempReg(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            regRowid,
        );
    }
    crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, addrContinue);
    if __pSort_ref.sortFlags as ::core::ffi::c_int & SORTFLAG_UseSorter != 0 {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_SorterNext,
            iTab,
            addr,
        );
    } else {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Next,
            iTab,
            addr,
        );
    }
    if __pSort_ref.regReturn != 0 {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
            v,
            crate::src::headers::opcodes_h::OP_Return,
            __pSort_ref.regReturn,
        );
    }
    crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, addrBreak);
}

unsafe extern "C" fn columnTypeImpl(
    mut pNC: *mut crate::src::headers::sqliteInt_h::NameContext,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> *const ::core::ffi::c_char {
    let mut zType: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut j: ::core::ffi::c_int;
    match (*pExpr).op as ::core::ffi::c_int {
        crate::src::parse::TK_COLUMN_1 => {
            let mut pTab: *mut crate::src::headers::sqliteInt_h::Table =
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
            let mut pS: *mut crate::src::headers::sqliteInt_h::Select =
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
            let iCol: ::core::ffi::c_int = (*pExpr).iColumn as ::core::ffi::c_int;
            while !pNC.is_null() && pTab.is_null() {
                let pTabList: *mut crate::src::headers::sqliteInt_h::SrcList = (*pNC).pSrcList;
                j = 0 as ::core::ffi::c_int;
                let __pTabList_ref = unsafe { &mut *pTabList };
                while j < __pTabList_ref.nSrc
                    && (*(&raw mut __pTabList_ref.a
                        as *mut crate::src::headers::sqliteInt_h::SrcItem)
                        .offset(j as isize))
                    .iCursor
                        != (*pExpr).iTable
                {
                    j += 1;
                }
                if j < __pTabList_ref.nSrc {
                    pTab = (*(&raw mut __pTabList_ref.a
                        as *mut crate::src::headers::sqliteInt_h::SrcItem)
                        .offset(j as isize))
                    .pSTab;
                    if (*(&raw mut __pTabList_ref.a
                        as *mut crate::src::headers::sqliteInt_h::SrcItem)
                        .offset(j as isize))
                    .fg
                    .isSubquery()
                        != 0
                    {
                        pS = (*(*(&raw mut __pTabList_ref.a
                            as *mut crate::src::headers::sqliteInt_h::SrcItem)
                            .offset(j as isize))
                        .u4
                        .pSubq)
                            .pSelect;
                    } else {
                        pS = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
                    }
                } else {
                    pNC = (*pNC).pNext;
                }
            }
            if !pTab.is_null() {
                if !pS.is_null() {
                    if iCol < (*(*pS).pEList).nExpr
                        && (crate::src::headers::sqliteInt_h::ViewCanHaveRowid == 0
                            || iCol >= 0 as ::core::ffi::c_int)
                    {
                        let mut sNC: crate::src::headers::sqliteInt_h::NameContext =
                            unsafe { ::core::mem::zeroed() };
                        let p: *mut crate::src::headers::sqliteInt_h::Expr =
                            (*(&raw mut (*(*pS).pEList).a
                                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                .offset(iCol as isize))
                            .pExpr;
                        sNC.pSrcList = (*pS).pSrc;
                        sNC.pNext = pNC;
                        sNC.pParse = (*pNC).pParse;
                        zType = columnTypeImpl(&raw mut sNC, p);
                    }
                } else if iCol < 0 as ::core::ffi::c_int {
                    zType = b"INTEGER\0" as *const u8 as *const ::core::ffi::c_char;
                } else {
                    zType = crate::src::src::util::sqlite3ColumnType(
                        (*pTab).aCol.offset(iCol as isize)
                            as *mut crate::src::headers::sqliteInt_h::Column
                            as *mut crate::src::headers::sqliteInt_h::Column,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                }
            }
        }
        crate::src::parse::TK_SELECT_1 => {
            let mut sNC_0: crate::src::headers::sqliteInt_h::NameContext =
                unsafe { ::core::mem::zeroed() };
            
            
            let pS_0: *mut crate::src::headers::sqliteInt_h::Select = (*pExpr).x.pSelect;
            let p_0: *mut crate::src::headers::sqliteInt_h::Expr = (*(&raw mut (*(*pS_0).pEList).a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(0_isize))
            .pExpr;
            sNC_0.pSrcList = (*pS_0).pSrc;
            sNC_0.pNext = pNC;
            sNC_0.pParse = (*pNC).pParse;
            zType = columnTypeImpl(&raw mut sNC_0, p_0);
        }
        _ => {}
    }
    zType
}

unsafe extern "C" fn generateColumnTypes(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pTabList: *mut crate::src::headers::sqliteInt_h::SrcList,
    pEList: *mut crate::src::headers::sqliteInt_h::ExprList,
) {
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    let mut i: ::core::ffi::c_int;
    let mut sNC: crate::src::headers::sqliteInt_h::NameContext = unsafe { ::core::mem::zeroed() };
    sNC.pSrcList = pTabList;
    sNC.pParse = pParse;
    sNC.pNext = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::NameContext>();
    i = 0 as ::core::ffi::c_int;
    while i < (*pEList).nExpr {
        let p: *mut crate::src::headers::sqliteInt_h::Expr = (*(&raw mut (*pEList).a
            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(i as isize))
        .pExpr;
        
        let zType: *const ::core::ffi::c_char = columnTypeImpl(&raw mut sNC, p);
        crate::src::src::vdbeaux::sqlite3VdbeSetColName(
            v,
            i,
            crate::src::src::vdbe::COLNAME_DECLTYPE,
            zType,
            ::core::mem::transmute::<
                crate::src::headers::stdlib::IntptrT,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
        );
        i += 1;
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3GenerateColumnNames(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut pSelect: *mut crate::src::headers::sqliteInt_h::Select,
) {
    let __pParse_ref = unsafe { &mut *pParse };
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = __pParse_ref.pVdbe;
    let mut i: ::core::ffi::c_int;
    let mut pTab: *mut crate::src::headers::sqliteInt_h::Table;
    
    
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
    
    
    if __pParse_ref.colNamesSet() != 0 {
        return;
    }
    while !(*pSelect).pPrior.is_null() {
        pSelect = (*pSelect).pPrior;
    }
    let pTabList: *mut crate::src::headers::sqliteInt_h::SrcList = (*pSelect).pSrc;
    let pEList: *mut crate::src::headers::sqliteInt_h::ExprList = (*pSelect).pEList;
    __pParse_ref.set_colNamesSet(
        1 as crate::src::headers::sqliteInt_h::Bft as crate::src::headers::sqliteInt_h::Bft,
    );
    let fullName: ::core::ffi::c_int = ((*db).flags
        & crate::src::headers::sqliteInt_h::SQLITE_FullColNames
            as crate::src::ext::rtree::rtree::U64_0
        != 0 as crate::src::ext::rtree::rtree::U64_0) as ::core::ffi::c_int;
    let srcName: ::core::ffi::c_int = ((*db).flags
        & crate::src::headers::sqliteInt_h::SQLITE_ShortColNames
            as crate::src::ext::rtree::rtree::U64_0
        != 0 as crate::src::ext::rtree::rtree::U64_0
        || fullName != 0) as ::core::ffi::c_int;
    crate::src::src::vdbeaux::sqlite3VdbeSetNumCols(v, (*pEList).nExpr);
    i = 0 as ::core::ffi::c_int;
    while i < (*pEList).nExpr {
        let __pEList_ref = unsafe { &mut *pEList };
        let p: *mut crate::src::headers::sqliteInt_h::Expr = (*(&raw mut __pEList_ref.a
            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(i as isize))
        .pExpr;
        if !(*(&raw mut __pEList_ref.a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(i as isize))
        .zEName
        .is_null()
            && (*(&raw mut __pEList_ref.a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(i as isize))
            .fg
            .eEName() as ::core::ffi::c_int
                == crate::src::headers::sqliteInt_h::ENAME_NAME
        {
            let zName: *mut ::core::ffi::c_char = (*(&raw mut __pEList_ref.a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(i as isize))
            .zEName;
            crate::src::src::vdbeaux::sqlite3VdbeSetColName(
                v,
                i,
                crate::src::src::vdbe::COLNAME_NAME,
                zName,
                ::core::mem::transmute::<
                    crate::src::headers::stdlib::IntptrT,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
            );
        } else if srcName != 0 && (*p).op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN_1 {
            let zCol: *mut ::core::ffi::c_char;
            let mut iCol: ::core::ffi::c_int = (*p).iColumn as ::core::ffi::c_int;
            pTab = (*p).y.pTab;
            if iCol < 0 as ::core::ffi::c_int {
                iCol = (*pTab).iPKey as ::core::ffi::c_int;
            }
            if iCol < 0 as ::core::ffi::c_int {
                zCol = b"rowid\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            } else {
                zCol = (*(*pTab).aCol.offset(iCol as isize)).zCnName;
            }
            if fullName != 0 {
                
                let zName_0: *mut ::core::ffi::c_char = crate::src::src::printf::sqlite3MPrintf_args(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    b"%s.%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::src::src::printf::PrintfArg::Str(
                            (*pTab).zName as *mut ::core::ffi::c_char,
                        ),
                        crate::src::src::printf::PrintfArg::Str(zCol as *mut ::core::ffi::c_char),
                    ],
                );
                crate::src::src::vdbeaux::sqlite3VdbeSetColName(
                    v,
                    i,
                    crate::src::src::vdbe::COLNAME_NAME,
                    zName_0,
                    Some(
                        crate::src::src::rowset::sqlite3RowSetClear
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                );
            } else {
                crate::src::src::vdbeaux::sqlite3VdbeSetColName(
                    v,
                    i,
                    crate::src::src::vdbe::COLNAME_NAME,
                    zCol,
                    ::core::mem::transmute::<
                        crate::src::headers::stdlib::IntptrT,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
                );
            }
        } else {
            let mut z: *const ::core::ffi::c_char = (*(&raw mut __pEList_ref.a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(i as isize))
            .zEName;
            z = if z.is_null() {
                crate::src::src::printf::sqlite3MPrintf_args(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    b"column%d\0" as *const u8 as *const ::core::ffi::c_char,
                    &[crate::src::src::printf::PrintfArg::Int(
                        (i + 1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::I64_0,
                    )],
                )
            } else {
                crate::src::src::malloc::sqlite3DbStrDup(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    z,
                )
            };
            crate::src::src::vdbeaux::sqlite3VdbeSetColName(
                v,
                i,
                crate::src::src::vdbe::COLNAME_NAME,
                z,
                Some(
                    crate::src::src::rowset::sqlite3RowSetClear
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
        }
        i += 1;
    }
    generateColumnTypes(pParse, pTabList, pEList);
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ColumnsFromExprList(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pEList: *mut crate::src::headers::sqliteInt_h::ExprList,
    pnCol: *mut crate::src::fts5::I16_0,
    paCol: *mut *mut crate::src::headers::sqliteInt_h::Column,
) -> ::core::ffi::c_int {
    let __pParse_ref = unsafe { &*pParse };
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    let mut cnt: crate::src::ext::rtree::rtree::U32_0;
    let aCol: *mut crate::src::headers::sqliteInt_h::Column;
    let mut pCol: *mut crate::src::headers::sqliteInt_h::Column;
    let mut nCol: ::core::ffi::c_int;
    let mut zName: *mut ::core::ffi::c_char;
    let mut nName: ::core::ffi::c_int;
    let mut ht: crate::src::src::hash::Hash = unsafe { ::core::mem::zeroed() };
    let mut pTab: *mut crate::src::headers::sqliteInt_h::Table;
    crate::src::src::hash::sqlite3HashInit(
        &raw mut ht as *mut _ as *mut crate::src::src::hash::Hash,
    );
    if !pEList.is_null() {
        nCol = (*pEList).nExpr;
        aCol = crate::src::src::malloc::sqlite3DbMallocZero(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (::core::mem::size_of::<crate::src::headers::sqliteInt_h::Column>() as usize)
                .wrapping_mul(nCol as usize) as crate::src::ext::rtree::rtree::U64_0,
        ) as *mut crate::src::headers::sqliteInt_h::Column;
        if nCol > 32767 as ::core::ffi::c_int {
            nCol = 32767 as ::core::ffi::c_int;
        }
    } else {
        nCol = 0 as ::core::ffi::c_int;
        aCol = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Column>();
    }
    *pnCol = nCol as crate::src::fts5::I16_0;
    *paCol = aCol;
    i = 0 as ::core::ffi::c_int;
    pCol = aCol;
    while i < nCol && __pParse_ref.nErr == 0 {
        let pX: *mut crate::src::headers::sqliteInt_h::ExprList_item = (&raw mut (*pEList).a
            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(i as isize)
            as *mut crate::src::headers::sqliteInt_h::ExprList_item;
        let mut pCollide: *mut crate::src::headers::sqliteInt_h::ExprList_item;
        let __pX_ref = unsafe { &mut *pX };
        zName = __pX_ref.zEName;
        if !(!zName.is_null()
            && __pX_ref.fg.eEName() as ::core::ffi::c_int
                == crate::src::headers::sqliteInt_h::ENAME_NAME)
        {
            let mut pColExpr: *mut crate::src::headers::sqliteInt_h::Expr =
                crate::src::src::expr::sqlite3ExprSkipCollateAndLikely(
                    __pX_ref.pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::Expr;
            while !pColExpr.is_null()
                && (*pColExpr).op as ::core::ffi::c_int == crate::src::parse::TK_DOT
            {
                pColExpr = (*pColExpr).pRight;
            }
            if (*pColExpr).op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN_1
                && (*pColExpr).flags
                    & (0x1000000 as ::core::ffi::c_int | 0x2000000 as ::core::ffi::c_int)
                        as crate::src::ext::rtree::rtree::U32_0
                    == 0 as crate::src::ext::rtree::rtree::U32_0
                && !(*pColExpr).y.pTab.is_null()
            {
                let mut iCol: ::core::ffi::c_int = (*pColExpr).iColumn as ::core::ffi::c_int;
                pTab = (*pColExpr).y.pTab;
                if iCol < 0 as ::core::ffi::c_int {
                    iCol = (*pTab).iPKey as ::core::ffi::c_int;
                }
                zName = (if iCol >= 0 as ::core::ffi::c_int {
                    (*(*pTab).aCol.offset(iCol as isize)).zCnName as *const ::core::ffi::c_char
                } else {
                    b"rowid\0" as *const u8 as *const ::core::ffi::c_char
                }) as *mut ::core::ffi::c_char;
            } else if (*pColExpr).op as ::core::ffi::c_int == crate::src::parse::TK_ID {
                zName = (*pColExpr).u.zToken;
            }
        }
        if !zName.is_null() && crate::src::src::expr::sqlite3IsTrueOrFalse(zName) == 0 {
            zName = crate::src::src::malloc::sqlite3DbStrDup(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                zName,
            );
        } else {
            zName = crate::src::src::printf::sqlite3MPrintf_args(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                b"column%d\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Int(
                    (i + 1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::I64_0,
                )],
            );
        }
        cnt = 0 as crate::src::ext::rtree::rtree::U32_0;
        while !zName.is_null() && {
            pCollide = crate::src::src::hash::sqlite3HashFind(
                &raw mut ht as *mut _ as *const crate::src::src::hash::Hash,
                zName,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList_item;
            !pCollide.is_null()
        } {
            if (*pCollide).fg.bUsingTerm() != 0 {
                (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int
                    | crate::src::headers::sqliteInt_h::COLFLAG_NOEXPAND)
                    as crate::src::fts5::U16_0;
            }
            nName = crate::src::src::util::sqlite3Strlen30(zName);
            if nName > 0 as ::core::ffi::c_int {
                j = nName - 1 as ::core::ffi::c_int;
                while j > 0 as ::core::ffi::c_int
                    && *(&raw const crate::src::src::global::sqlite3CtypeMap
                        as *const ::core::ffi::c_uchar)
                        .offset(*zName.offset(j as isize) as ::core::ffi::c_uchar as isize)
                        as ::core::ffi::c_int
                        & 0x4 as ::core::ffi::c_int
                        != 0
                {
                    j -= 1;
                }
                if *zName.offset(j as isize) as ::core::ffi::c_int == ':' as i32 {
                    nName = j;
                }
            }
            cnt = cnt.wrapping_add(1);
            zName = crate::src::src::printf::sqlite3MPrintf_args(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                b"%.*z:%u\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::src::src::printf::PrintfArg::Int(
                        nName as crate::src::ext::rtree::rtree::I64_0,
                    ),
                    crate::src::src::printf::PrintfArg::Str(zName as *mut ::core::ffi::c_char),
                    crate::src::src::printf::PrintfArg::UInt(
                        cnt as crate::src::headers::sqlite3_h::SqliteUint64,
                    ),
                ],
            );
            crate::src::src::util::sqlite3ProgressCheck(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            );
            if cnt > 3 as crate::src::ext::rtree::rtree::U32_0 {
                crate::src::src::random::sqlite3_randomness(
                    ::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>()
                        as ::core::ffi::c_int,
                    &raw mut cnt as *mut ::core::ffi::c_void,
                );
            }
        }
        (*pCol).zCnName = zName;
        (*pCol).hName = crate::src::src::util::sqlite3StrIHash(zName);
        if __pX_ref.fg.bNoExpand() != 0 {
            (*pCol).colFlags = ((*pCol).colFlags as ::core::ffi::c_int
                | crate::src::headers::sqliteInt_h::COLFLAG_NOEXPAND)
                as crate::src::fts5::U16_0;
        }
        if !zName.is_null()
            && crate::src::src::hash::sqlite3HashInsert(
                &raw mut ht as *mut _ as *mut crate::src::src::hash::Hash,
                zName,
                pX as *mut ::core::ffi::c_void,
            ) == pX as *mut ::core::ffi::c_void
        {
            crate::src::src::malloc::sqlite3OomFault(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            );
        }
        i += 1;
        pCol = pCol.offset(1);
    }
    crate::src::src::hash::sqlite3HashClear(
        &raw mut ht as *mut _ as *mut crate::src::src::hash::Hash,
    );
    if __pParse_ref.nErr != 0 {
        j = 0 as ::core::ffi::c_int;
        while j < i {
            crate::src::src::malloc::sqlite3DbFree(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*aCol.offset(j as isize)).zCnName as *mut ::core::ffi::c_void,
            );
            j += 1;
        }
        crate::src::src::malloc::sqlite3DbFree(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            aCol as *mut ::core::ffi::c_void,
        );
        *paCol = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Column>();
        *pnCol = 0 as crate::src::fts5::I16_0;
        return __pParse_ref.rc;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SubqueryColumnTypes(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pTab: *mut crate::src::headers::sqliteInt_h::Table,
    mut pSelect: *mut crate::src::headers::sqliteInt_h::Select,
    aff: ::core::ffi::c_char,
) {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    let mut pCol: *mut crate::src::headers::sqliteInt_h::Column;
    let mut pColl: *mut crate::src::headers::sqliteInt_h::CollSeq;
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    let mut p: *mut crate::src::headers::sqliteInt_h::Expr;
    
    let mut sNC: crate::src::headers::sqliteInt_h::NameContext = unsafe { ::core::mem::zeroed() };
    if (*db).mallocFailed as ::core::ffi::c_int != 0
        || (*pParse).eParseMode as ::core::ffi::c_int
            >= crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME
    {
        return;
    }
    while !(*pSelect).pPrior.is_null() {
        pSelect = (*pSelect).pPrior;
    }
    let a: *mut crate::src::headers::sqliteInt_h::ExprList_item = &raw mut (*(*pSelect).pEList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item
        as *mut crate::src::headers::sqliteInt_h::ExprList_item;
    sNC.pSrcList = (*pSelect).pSrc;
    i = 0 as ::core::ffi::c_int;
    let __pTab_ref = unsafe { &mut *pTab };
    pCol = __pTab_ref.aCol;
    while i < __pTab_ref.nCol as ::core::ffi::c_int {
        let mut zType: *const ::core::ffi::c_char;
        let n: crate::src::ext::rtree::rtree::I64_0;
        let mut m: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pS2: *mut crate::src::headers::sqliteInt_h::Select = pSelect;
        __pTab_ref.tabFlags |= ((*pCol).colFlags as ::core::ffi::c_int
            & crate::src::headers::sqliteInt_h::COLFLAG_NOINSERT)
            as crate::src::ext::rtree::rtree::U32_0;
        p = (*a.offset(i as isize)).pExpr;
        (*pCol).affinity = crate::src::src::expr::sqlite3ExprAffinity(
            p as *const crate::src::headers::sqliteInt_h::Expr,
        );
        while (*pCol).affinity as ::core::ffi::c_int
            <= crate::src::headers::sqliteInt_h::SQLITE_AFF_NONE
            && !(*pS2).pNext.is_null()
        {
            m |= crate::src::src::expr::sqlite3ExprDataType(
                (*(&raw mut (*(*pS2).pEList).a
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(i as isize))
                .pExpr as *const crate::src::headers::sqliteInt_h::Expr,
            );
            pS2 = (*pS2).pNext;
            (*pCol).affinity = crate::src::src::expr::sqlite3ExprAffinity(
                (*(&raw mut (*(*pS2).pEList).a
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(i as isize))
                .pExpr as *const crate::src::headers::sqliteInt_h::Expr,
            );
        }
        if (*pCol).affinity as ::core::ffi::c_int
            <= crate::src::headers::sqliteInt_h::SQLITE_AFF_NONE
        {
            (*pCol).affinity = aff;
        }
        if (*pCol).affinity as ::core::ffi::c_int
            >= crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT
            && (!(*pS2).pNext.is_null() || pS2 != pSelect)
        {
            pS2 = (*pS2).pNext;
            while !pS2.is_null() {
                m |= crate::src::src::expr::sqlite3ExprDataType(
                    (*(&raw mut (*(*pS2).pEList).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(i as isize))
                    .pExpr as *const crate::src::headers::sqliteInt_h::Expr,
                );
                pS2 = (*pS2).pNext;
            }
            let __pCol_ref = unsafe { &mut *pCol };
            if __pCol_ref.affinity as ::core::ffi::c_int
                == crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT
                && m & 0x1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                || __pCol_ref.affinity as ::core::ffi::c_int
                    >= crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC
                    && m & 0x2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            {
                __pCol_ref.affinity =
                    crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB as ::core::ffi::c_char;
            }
            if __pCol_ref.affinity as ::core::ffi::c_int
                >= crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC
                && (*p).op as ::core::ffi::c_int == crate::src::parse::TK_CAST
            {
                __pCol_ref.affinity =
                    crate::src::headers::sqliteInt_h::SQLITE_AFF_FLEXNUM as ::core::ffi::c_char;
            }
        }
        zType = columnTypeImpl(&raw mut sNC, p);
        if zType.is_null()
            || (*pCol).affinity as ::core::ffi::c_int
                != crate::src::src::build::sqlite3AffinityType(
                    zType,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Column>()
                        as *mut crate::src::headers::sqliteInt_h::Column,
                ) as ::core::ffi::c_int
        {
            if (*pCol).affinity as ::core::ffi::c_int
                == crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC
                || (*pCol).affinity as ::core::ffi::c_int
                    == crate::src::headers::sqliteInt_h::SQLITE_AFF_FLEXNUM
            {
                zType = b"NUM\0" as *const u8 as *const ::core::ffi::c_char;
            } else {
                zType = ::core::ptr::null::<::core::ffi::c_char>();
                j = 1 as ::core::ffi::c_int;
                while j < crate::src::headers::sqliteInt_h::SQLITE_N_STDTYPE {
                    if *(&raw const crate::src::src::global::sqlite3StdTypeAffinity
                        as *const ::core::ffi::c_char)
                        .offset(j as isize) as ::core::ffi::c_int
                        == (*pCol).affinity as ::core::ffi::c_int
                    {
                        zType = *(&raw mut crate::src::src::global::sqlite3StdType
                            as *mut *const ::core::ffi::c_char)
                            .offset(j as isize);
                        break;
                    } else {
                        j += 1;
                    }
                }
            }
        }
        if !zType.is_null() {
            let k: crate::src::ext::rtree::rtree::I64_0 =
                crate::src::src::util::sqlite3Strlen30(zType)
                    as crate::src::ext::rtree::rtree::I64_0;
            let __pCol_ref = unsafe { &mut *pCol };
            n = crate::src::src::util::sqlite3Strlen30(__pCol_ref.zCnName)
                as crate::src::ext::rtree::rtree::I64_0;
            __pCol_ref.zCnName = crate::src::src::malloc::sqlite3DbReallocOrFree(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                __pCol_ref.zCnName as *mut ::core::ffi::c_void,
                (n + k + 2 as crate::src::ext::rtree::rtree::I64_0)
                    as crate::src::ext::rtree::rtree::U64_0,
            ) as *mut ::core::ffi::c_char;
            __pCol_ref.colFlags = (__pCol_ref.colFlags as ::core::ffi::c_int
                & !(crate::src::headers::sqliteInt_h::COLFLAG_HASTYPE
                    | crate::src::headers::sqliteInt_h::COLFLAG_HASCOLL))
                as crate::src::fts5::U16_0;
            if !__pCol_ref.zCnName.is_null() {
                ::core::ptr::copy_nonoverlapping(
                    zType as *const u8,
                    __pCol_ref
                        .zCnName
                        .offset((n + 1 as crate::src::ext::rtree::rtree::I64_0) as isize)
                        as *mut ::core::ffi::c_char as *mut u8,
                    (k + 1 as crate::src::ext::rtree::rtree::I64_0) as usize,
                );
                __pCol_ref.colFlags = (__pCol_ref.colFlags as ::core::ffi::c_int
                    | crate::src::headers::sqliteInt_h::COLFLAG_HASTYPE)
                    as crate::src::fts5::U16_0;
            }
        }
        pColl = crate::src::src::expr::sqlite3ExprCollSeq(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            p as *const crate::src::headers::sqliteInt_h::Expr,
        ) as *mut crate::src::headers::sqliteInt_h::CollSeq;
        if !pColl.is_null() {
            crate::src::src::build::sqlite3ColumnSetColl(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                pCol as *mut crate::src::headers::sqliteInt_h::Column,
                (*pColl).zName,
            );
        }
        i += 1;
        pCol = pCol.offset(1);
    }
    __pTab_ref.szTabRow = 1 as crate::src::headers::sqliteInt_h::LogEst;
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ResultSetOfSelect(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut pSelect: *mut crate::src::headers::sqliteInt_h::Select,
    aff: ::core::ffi::c_char,
) -> *mut crate::src::headers::sqliteInt_h::Table {
    
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    
    let __db_ref = unsafe { &mut *db };
    let savedFlags: crate::src::ext::rtree::rtree::U64_0 = __db_ref.flags;
    __db_ref.flags &= !(crate::src::headers::sqliteInt_h::SQLITE_FullColNames
        as crate::src::ext::rtree::rtree::U64_0);
    __db_ref.flags |= crate::src::headers::sqliteInt_h::SQLITE_ShortColNames
        as crate::src::ext::rtree::rtree::U64_0;
    sqlite3SelectPrep(
        pParse,
        pSelect,
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::NameContext>(),
    );
    __db_ref.flags = savedFlags;
    if (*pParse).nErr != 0 {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
    }
    while !(*pSelect).pPrior.is_null() {
        pSelect = (*pSelect).pPrior;
    }
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = crate::src::src::malloc::sqlite3DbMallocZero(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Table>()
            as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::headers::sqliteInt_h::Table;
    if pTab.is_null() {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
    }
    (*pTab).nTabRef = 1 as crate::src::ext::rtree::rtree::U32_0;
    (*pTab).zName = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*pTab).nRowLogEst = 200 as crate::src::headers::sqliteInt_h::LogEst;
    sqlite3ColumnsFromExprList(
        pParse,
        (*pSelect).pEList,
        &raw mut (*pTab).nCol,
        &raw mut (*pTab).aCol,
    );
    sqlite3SubqueryColumnTypes(pParse, pTab, pSelect, aff);
    (*pTab).iPKey = -(1 as ::core::ffi::c_int) as crate::src::fts5::I16_0;
    if __db_ref.mallocFailed != 0 {
        crate::src::src::build::sqlite3DeleteTable(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pTab as *mut crate::src::headers::sqliteInt_h::Table,
        );
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
    }
    pTab
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3GetVdbe(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
) -> *mut crate::src::headers::vdbeInt_h::Vdbe {
    let __pParse_ref = unsafe { &mut *pParse };
    if !__pParse_ref.pVdbe.is_null() {
        return __pParse_ref.pVdbe;
    }
    if __pParse_ref.pToplevel.is_null()
        && (*__pParse_ref.db).dbOptFlags & 0x8 as crate::src::ext::rtree::rtree::U32_0
            == 0 as crate::src::ext::rtree::rtree::U32_0
    {
        __pParse_ref.set_okConstFactor(
            1 as crate::src::headers::sqliteInt_h::Bft as crate::src::headers::sqliteInt_h::Bft,
        );
    }
    crate::src::src::vdbeaux::sqlite3VdbeCreate(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
    )
}

unsafe extern "C" fn computeLimitRegisters(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
    iBreak: ::core::ffi::c_int,
) {
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe;
    let iLimit: ::core::ffi::c_int;
    let iOffset: ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = 0;
    let pLimit: *mut crate::src::headers::sqliteInt_h::Expr = (*p).pLimit;
    if (*p).iLimit != 0 {
        return;
    }
    if !pLimit.is_null() {
        (*pParse).nMem += 1;
        iLimit = (*pParse).nMem;
        (*p).iLimit = iLimit;
        v = sqlite3GetVdbe(pParse);
        if crate::src::src::expr::sqlite3ExprIsInteger(
            (*pLimit).pLeft as *const crate::src::headers::sqliteInt_h::Expr,
            &raw mut n,
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        ) != 0
        {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_Integer,
                n,
                iLimit,
            );
            if n == 0 as ::core::ffi::c_int {
                crate::src::src::vdbeaux::sqlite3VdbeGoto(v, iBreak);
            } else if n >= 0 as ::core::ffi::c_int
                && (*p).nSelectRow as ::core::ffi::c_int
                    > crate::src::src::util::sqlite3LogEst(
                        n as crate::src::ext::rtree::rtree::U64_0,
                    ) as ::core::ffi::c_int
            {
                (*p).nSelectRow =
                    crate::src::src::util::sqlite3LogEst(n as crate::src::ext::rtree::rtree::U64_0);
                (*p).selFlags |= crate::src::headers::sqliteInt_h::SF_FixedLimit
                    as crate::src::ext::rtree::rtree::U32_0;
            }
        } else {
            crate::src::src::expr::sqlite3ExprCode(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*pLimit).pLeft as *mut crate::src::headers::sqliteInt_h::Expr,
                iLimit,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                v,
                crate::src::headers::opcodes_h::OP_MustBeInt,
                iLimit,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_IfNot,
                iLimit,
                iBreak,
            );
        }
        if !(*pLimit).pRight.is_null() {
            let __pParse_ref = unsafe { &mut *pParse };
            __pParse_ref.nMem += 1;
            iOffset = __pParse_ref.nMem;
            (*p).iOffset = iOffset;
            __pParse_ref.nMem += 1;
            crate::src::src::expr::sqlite3ExprCode(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*pLimit).pRight as *mut crate::src::headers::sqliteInt_h::Expr,
                iOffset,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                v,
                crate::src::headers::opcodes_h::OP_MustBeInt,
                iOffset,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_OffsetLimit,
                iLimit,
                iOffset + 1 as ::core::ffi::c_int,
                iOffset,
            );
        }
    }
}

unsafe extern "C" fn multiSelectCollSeq(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
    iCol: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqliteInt_h::CollSeq {
    let mut pRet: *mut crate::src::headers::sqliteInt_h::CollSeq;
    if !(*p).pPrior.is_null() {
        pRet = multiSelectCollSeq(pParse, (*p).pPrior, iCol);
    } else {
        pRet = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::CollSeq>();
    }
    if pRet.is_null() && iCol < (*(*p).pEList).nExpr {
        pRet = crate::src::src::expr::sqlite3ExprCollSeq(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            (*(&raw mut (*(*p).pEList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(iCol as isize))
            .pExpr as *const crate::src::headers::sqliteInt_h::Expr,
        ) as *mut crate::src::headers::sqliteInt_h::CollSeq;
    }
    pRet
}

unsafe extern "C" fn multiSelectOrderByKeyInfo(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
    nExtra: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqliteInt_h::KeyInfo {
    let pOrderBy: *mut crate::src::headers::sqliteInt_h::ExprList = (*p).pOrderBy;
    let nOrderBy: ::core::ffi::c_int = if !pOrderBy.is_null() {
        (*pOrderBy).nExpr
    } else {
        0 as ::core::ffi::c_int
    };
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    let pRet: *mut crate::src::headers::sqliteInt_h::KeyInfo =
        sqlite3KeyInfoAlloc(db, nOrderBy + nExtra, 1 as ::core::ffi::c_int);
    if !pRet.is_null() {
        let mut i: ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < nOrderBy {
            let pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item =
                (&raw mut (*pOrderBy).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(i as isize)
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item;
            let pTerm: *mut crate::src::headers::sqliteInt_h::Expr = (*pItem).pExpr;
            let mut pColl: *mut crate::src::headers::sqliteInt_h::CollSeq;
            if (*pTerm).flags
                & crate::src::headers::sqliteInt_h::EP_Collate
                    as crate::src::ext::rtree::rtree::U32_0
                != 0
            {
                pColl = crate::src::src::expr::sqlite3ExprCollSeq(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    pTerm as *const crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::CollSeq;
            } else {
                pColl = multiSelectCollSeq(
                    pParse,
                    p,
                    (*pItem).u.x.iOrderByCol as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                );
                if pColl.is_null() {
                    pColl = (*db).pDfltColl;
                }
                let fresh29 = &mut (*(&raw mut (*pOrderBy).a
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(i as isize))
                .pExpr;
                *fresh29 = crate::src::src::expr::sqlite3ExprAddCollateString(
                    pParse as *const crate::src::headers::sqliteInt_h::Parse,
                    pTerm as *mut crate::src::headers::sqliteInt_h::Expr,
                    (*pColl).zName,
                ) as *mut crate::src::headers::sqliteInt_h::Expr;
            }
            let fresh30 = &mut *(&raw mut (*pRet).aColl
                as *mut *mut crate::src::headers::sqliteInt_h::CollSeq)
                .offset(i as isize);
            *fresh30 = pColl;
            *(*pRet).aSortFlags.offset(i as isize) = (*(&raw mut (*pOrderBy).a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(i as isize))
            .fg
            .sortFlags;
            i += 1;
        }
    }
    pRet
}

unsafe extern "C" fn generateWithRecursiveQuery(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
    pDest: *mut crate::src::headers::sqliteInt_h::SelectDest,
) {
    let current_block: u64;
    let __p_ref = unsafe { &mut *p };
    let pSrc: *mut crate::src::headers::sqliteInt_h::SrcList = __p_ref.pSrc;
    let nCol: ::core::ffi::c_int = (*__p_ref.pEList).nExpr;
    let __pParse_ref = unsafe { &mut *pParse };
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = __pParse_ref.pVdbe;
    let pSetup: *mut crate::src::headers::sqliteInt_h::Select;
    let mut pFirstRec: *mut crate::src::headers::sqliteInt_h::Select;
    let addrTop: ::core::ffi::c_int;
    let addrCont: ::core::ffi::c_int;
    
    let mut iCurrent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    let mut iDistinct: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let eDest: ::core::ffi::c_int;
    let mut destQueue: crate::src::headers::sqliteInt_h::SelectDest =
        unsafe { ::core::mem::zeroed() };
    let mut i: ::core::ffi::c_int;
    let rc: ::core::ffi::c_int;
    
    
    
    
    if !__p_ref.pWin.is_null() {
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"cannot use window functions in recursive queries\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        );
        return;
    }
    if crate::src::src::auth::sqlite3AuthCheck(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        crate::src::headers::sqlite3_h::SQLITE_RECURSIVE,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ) != 0
    {
        return;
    }
    let addrBreak: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
    );
    __p_ref.nSelectRow = 320 as crate::src::headers::sqliteInt_h::LogEst;
    computeLimitRegisters(pParse, p, addrBreak);
    let pLimit: *mut crate::src::headers::sqliteInt_h::Expr = __p_ref.pLimit;
    let regLimit: ::core::ffi::c_int = __p_ref.iLimit;
    let regOffset: ::core::ffi::c_int = __p_ref.iOffset;
    __p_ref.pLimit = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
    __p_ref.iOffset = 0 as ::core::ffi::c_int;
    __p_ref.iLimit = __p_ref.iOffset;
    let pOrderBy: *mut crate::src::headers::sqliteInt_h::ExprList = __p_ref.pOrderBy;
    i = 0 as ::core::ffi::c_int;
    while i < (*pSrc).nSrc {
        if (*(&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
            .offset(i as isize))
        .fg
        .isRecursive()
            != 0
        {
            iCurrent = (*(&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                .offset(i as isize))
            .iCursor;
            break;
        } else {
            i += 1;
        }
    }
    let fresh31 = __pParse_ref.nTab;
    __pParse_ref.nTab += 1;
    let iQueue: ::core::ffi::c_int = fresh31;
    if __p_ref.op as ::core::ffi::c_int == crate::src::parse::TK_UNION {
        eDest = if !pOrderBy.is_null() {
            crate::src::headers::sqliteInt_h::SRT_DistQueue
        } else {
            crate::src::headers::sqliteInt_h::SRT_DistFifo
        };
        let fresh32 = __pParse_ref.nTab;
        __pParse_ref.nTab += 1;
        iDistinct = fresh32;
    } else {
        eDest = if !pOrderBy.is_null() {
            crate::src::headers::sqliteInt_h::SRT_Queue
        } else {
            crate::src::headers::sqliteInt_h::SRT_Fifo
        };
    }
    sqlite3SelectDestInit(&raw mut destQueue, eDest, iQueue);
    __pParse_ref.nMem += 1;
    let regCurrent: ::core::ffi::c_int = __pParse_ref.nMem;
    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
        v,
        crate::src::headers::opcodes_h::OP_OpenPseudo,
        iCurrent,
        regCurrent,
        nCol,
    );
    if !pOrderBy.is_null() {
        let pKeyInfo: *mut crate::src::headers::sqliteInt_h::KeyInfo =
            multiSelectOrderByKeyInfo(pParse, p, 1 as ::core::ffi::c_int);
        crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
            v,
            crate::src::headers::opcodes_h::OP_OpenEphemeral,
            iQueue,
            (*pOrderBy).nExpr + 2 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            pKeyInfo as *mut ::core::ffi::c_char,
            crate::src::src::vdbe::P4_KEYINFO,
        );
        destQueue.pOrderBy = pOrderBy;
    } else {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_OpenEphemeral,
            iQueue,
            nCol,
        );
    }
    if iDistinct != 0 {
        __p_ref.addrOpenEphm[0 as ::core::ffi::c_int as usize] =
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_OpenEphemeral,
                iDistinct,
                0 as ::core::ffi::c_int,
            );
        __p_ref.selFlags |= crate::src::headers::sqliteInt_h::SF_UsesEphemeral
            as crate::src::ext::rtree::rtree::U32_0;
    }
    __p_ref.pOrderBy = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
    pFirstRec = p;
    loop {
        if pFirstRec.is_null() {
            current_block = 790185930182612747;
            break;
        }
        if (*pFirstRec).selFlags
            & crate::src::headers::sqliteInt_h::SF_Aggregate as crate::src::ext::rtree::rtree::U32_0
            != 0
        {
            crate::src::src::util::sqlite3ErrorMsg_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                b"recursive aggregate queries not supported\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
            current_block = 8834331691167872155;
            break;
        } else {
            (*pFirstRec).op = crate::src::parse::TK_ALL as u8;
            if (*(*pFirstRec).pPrior).selFlags
                & crate::src::headers::sqliteInt_h::SF_Recursive
                    as crate::src::ext::rtree::rtree::U32_0
                == 0 as crate::src::ext::rtree::rtree::U32_0
            {
                current_block = 790185930182612747;
                break;
            }
            pFirstRec = (*pFirstRec).pPrior;
        }
    }
    if current_block == 790185930182612747 {
        pSetup = (*pFirstRec).pPrior;
        (*pSetup).pNext = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
        crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            1_u8,
            b"SETUP\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
        rc = sqlite3Select(pParse, pSetup, &raw mut destQueue);
        (*pSetup).pNext = p;
        if (rc == 0) {
            addrTop = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_Rewind,
                iQueue,
                addrBreak,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                v,
                crate::src::headers::opcodes_h::OP_NullRow,
                iCurrent,
            );
            if !pOrderBy.is_null() {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                    v,
                    crate::src::headers::opcodes_h::OP_Column,
                    iQueue,
                    (*pOrderBy).nExpr + 1 as ::core::ffi::c_int,
                    regCurrent,
                );
            } else {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_RowData,
                    iQueue,
                    regCurrent,
                );
            }
            crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                v,
                crate::src::headers::opcodes_h::OP_Delete,
                iQueue,
            );
            addrCont = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            );
            codeOffset(v, regOffset, addrCont);
            selectInnerLoop(
                pParse,
                p,
                iCurrent,
                ::core::ptr::null_mut::<SortCtx>(),
                ::core::ptr::null_mut::<DistinctCtx>(),
                pDest,
                addrCont,
                addrBreak,
            );
            if regLimit != 0 {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_DecrJumpZero,
                    regLimit,
                    addrBreak,
                );
            }
            crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, addrCont);
            (*pFirstRec).pPrior =
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
            crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                1_u8,
                b"RECURSIVE STEP\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
            sqlite3Select(pParse, p, &raw mut destQueue);
            (*pFirstRec).pPrior = pSetup;
            crate::src::src::vdbeaux::sqlite3VdbeGoto(v, addrTop);
            crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, addrBreak);
        }
    }
    crate::src::src::expr::sqlite3ExprListDelete(
        __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __p_ref.pOrderBy as *mut crate::src::headers::sqliteInt_h::ExprList,
    );
    __p_ref.pOrderBy = pOrderBy;
    __p_ref.pLimit = pLimit;
}

unsafe extern "C" fn multiSelectValues(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut p: *mut crate::src::headers::sqliteInt_h::Select,
    pDest: *mut crate::src::headers::sqliteInt_h::SelectDest,
) -> ::core::ffi::c_int {
    let mut nRow: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let bShowAll: ::core::ffi::c_int = (*p).pLimit.is_null()
        as ::core::ffi::c_int;
    loop {
        if !(*p).pWin.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        if (*p).pPrior.is_null() {
            break;
        }
        p = (*p).pPrior;
        nRow += bShowAll;
    }
    crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        0_u8,
        b"SCAN %d CONSTANT ROW%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[
            crate::src::src::printf::PrintfArg::Int(nRow as crate::src::ext::rtree::rtree::I64_0),
            crate::src::src::printf::PrintfArg::Str(if nRow == 1 as ::core::ffi::c_int {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"S\0" as *const u8 as *const ::core::ffi::c_char
            } as *mut ::core::ffi::c_char),
        ],
    );
    while !p.is_null() {
        selectInnerLoop(
            pParse,
            p,
            -(1 as ::core::ffi::c_int),
            ::core::ptr::null_mut::<SortCtx>(),
            ::core::ptr::null_mut::<DistinctCtx>(),
            pDest,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        if bShowAll == 0 {
            break;
        }
        (*p).nSelectRow = nRow as crate::src::headers::sqliteInt_h::LogEst;
        p = (*p).pNext;
    }
    rc
}

unsafe extern "C" fn hasAnchor(
    mut p: *mut crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    while !p.is_null()
        && (*p).selFlags
            & crate::src::headers::sqliteInt_h::SF_Recursive as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        p = (*p).pPrior;
    }
    !p.is_null() as ::core::ffi::c_int
}

unsafe extern "C" fn multiSelect(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
    pDest: *mut crate::src::headers::sqliteInt_h::SelectDest,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    
    
    let mut dest: crate::src::headers::sqliteInt_h::SelectDest;
    let mut pDelete: *mut crate::src::headers::sqliteInt_h::Select =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    let pPrior: *mut crate::src::headers::sqliteInt_h::Select = (*p).pPrior;
    dest = *pDest;
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = sqlite3GetVdbe(pParse);
    if dest.eDest as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::SRT_EphemTab_1 {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_OpenEphemeral,
            dest.iSDParm,
            (*(*p).pEList).nExpr,
        );
        dest.eDest = crate::src::headers::sqliteInt_h::SRT_Table as u8;
    }
    if (*p).selFlags
        & crate::src::headers::sqliteInt_h::SF_MultiValue as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        rc = multiSelectValues(pParse, p, &raw mut dest);
        if rc >= 0 as ::core::ffi::c_int {
            current_block = 4843618750404471500;
        } else {
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
            current_block = 7149356873433890176;
        }
    } else {
        current_block = 7149356873433890176;
    }
    if current_block == 7149356873433890176 {
        if (*p).selFlags
            & crate::src::headers::sqliteInt_h::SF_Recursive
                as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
            && hasAnchor(p) != 0
        {
            generateWithRecursiveQuery(pParse, p, &raw mut dest);
            current_block = 4235089732467486934;
        } else if !(*p).pOrderBy.is_null() {
            return multiSelectOrderBy(pParse, p, pDest);
        } else {
            if (*pPrior).pPrior.is_null() {
                crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    1_u8,
                    b"COMPOUND QUERY\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
                crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    1_u8,
                    b"LEFT-MOST SUBQUERY\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            match (*p).op as ::core::ffi::c_int {
                crate::src::parse::TK_ALL => {
                    let mut addr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut nLimit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let __p_ref = unsafe { &mut *p };
                    let __pPrior_ref = unsafe { &mut *pPrior };
                    __pPrior_ref.iLimit = __p_ref.iLimit;
                    __pPrior_ref.iOffset = __p_ref.iOffset;
                    __pPrior_ref.pLimit = __p_ref.pLimit;
                    rc = sqlite3Select(pParse, pPrior, &raw mut dest);
                    __pPrior_ref.pLimit =
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
                    if rc != 0 {
                        current_block = 4843618750404471500;
                    } else {
                        __p_ref.pPrior =
                            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
                        __p_ref.iLimit = __pPrior_ref.iLimit;
                        __p_ref.iOffset = __pPrior_ref.iOffset;
                        if __p_ref.iLimit != 0 {
                            addr = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                                v,
                                crate::src::headers::opcodes_h::OP_IfNot,
                                __p_ref.iLimit,
                            );
                            if __p_ref.iOffset != 0 {
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                    v,
                                    crate::src::headers::opcodes_h::OP_OffsetLimit,
                                    __p_ref.iLimit,
                                    __p_ref.iOffset + 1 as ::core::ffi::c_int,
                                    __p_ref.iOffset,
                                );
                            }
                        }
                        crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            1_u8,
                            b"UNION ALL\0" as *const u8 as *const ::core::ffi::c_char,
                            &[],
                        );
                        rc = sqlite3Select(pParse, p, &raw mut dest);
                        pDelete = __p_ref.pPrior;
                        __p_ref.pPrior = pPrior;
                        __p_ref.nSelectRow = crate::src::src::util::sqlite3LogEstAdd(
                            __p_ref.nSelectRow,
                            __pPrior_ref.nSelectRow,
                        );
                        if !__p_ref.pLimit.is_null()
                            && crate::src::src::expr::sqlite3ExprIsInteger(
                                (*__p_ref.pLimit).pLeft
                                    as *const crate::src::headers::sqliteInt_h::Expr,
                                &raw mut nLimit,
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            ) != 0
                            && nLimit > 0 as ::core::ffi::c_int
                            && __p_ref.nSelectRow as ::core::ffi::c_int
                                > crate::src::src::util::sqlite3LogEst(
                                    nLimit as crate::src::ext::rtree::rtree::U64_0,
                                ) as ::core::ffi::c_int
                        {
                            __p_ref.nSelectRow = crate::src::src::util::sqlite3LogEst(
                                nLimit as crate::src::ext::rtree::rtree::U64_0,
                            );
                        }
                        if addr != 0 {
                            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addr);
                        }
                        current_block = 14579489411542934868;
                    }
                }
                crate::src::parse::TK_EXCEPT | crate::src::parse::TK_UNION => {
                    let unionTab: ::core::ffi::c_int;
                    let op: crate::src::ext::rtree::rtree::U8_0;
                    
                    let pLimit: *mut crate::src::headers::sqliteInt_h::Expr;
                    let addr_0: ::core::ffi::c_int;
                    let mut emptyBypass: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut uniondest: crate::src::headers::sqliteInt_h::SelectDest =
                        unsafe { ::core::mem::zeroed() };
                    let priorOp: ::core::ffi::c_int = crate::src::headers::sqliteInt_h::SRT_Union;
                    if dest.eDest as ::core::ffi::c_int == priorOp {
                        unionTab = dest.iSDParm;
                    } else {
                        let __pParse_ref = unsafe { &mut *pParse };
                        let fresh24 = __pParse_ref.nTab;
                        __pParse_ref.nTab += 1;
                        unionTab = fresh24;
                        addr_0 = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                            v,
                            crate::src::headers::opcodes_h::OP_OpenEphemeral,
                            unionTab,
                            0 as ::core::ffi::c_int,
                        );
                        (*p).addrOpenEphm[0 as ::core::ffi::c_int as usize] = addr_0;
                        (*findRightmost(p)).selFlags |=
                            crate::src::headers::sqliteInt_h::SF_UsesEphemeral
                                as crate::src::ext::rtree::rtree::U32_0;
                    }
                    sqlite3SelectDestInit(&raw mut uniondest, priorOp, unionTab);
                    rc = sqlite3Select(pParse, pPrior, &raw mut uniondest);
                    if rc != 0 {
                        current_block = 4843618750404471500;
                    } else {
                        let __p_ref = unsafe { &mut *p };
                        if __p_ref.op as ::core::ffi::c_int == crate::src::parse::TK_EXCEPT {
                            op = crate::src::headers::sqliteInt_h::SRT_Except as u8;
                            emptyBypass = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                                v,
                                crate::src::headers::opcodes_h::OP_IfEmpty,
                                unionTab,
                            );
                        } else {
                            op = crate::src::headers::sqliteInt_h::SRT_Union as u8;
                        }
                        __p_ref.pPrior =
                            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
                        pLimit = __p_ref.pLimit;
                        __p_ref.pLimit =
                            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
                        uniondest.eDest = op;
                        crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            1_u8,
                            b"%s USING TEMP B-TREE\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[crate::src::src::printf::PrintfArg::Str(
                                sqlite3SelectOpName(__p_ref.op as ::core::ffi::c_int)
                                    as *mut ::core::ffi::c_char,
                            )],
                        );
                        rc = sqlite3Select(pParse, p, &raw mut uniondest);
                        pDelete = __p_ref.pPrior;
                        __p_ref.pPrior = pPrior;
                        __p_ref.pOrderBy = ::core::ptr::null_mut::<
                            crate::src::headers::sqliteInt_h::ExprList,
                        >();
                        if __p_ref.op as ::core::ffi::c_int == crate::src::parse::TK_UNION {
                            __p_ref.nSelectRow = crate::src::src::util::sqlite3LogEstAdd(
                                __p_ref.nSelectRow,
                                (*pPrior).nSelectRow,
                            );
                        }
                        if emptyBypass != 0 {
                            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, emptyBypass);
                        }
                        crate::src::src::expr::sqlite3ExprDelete(
                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                            __p_ref.pLimit as *mut crate::src::headers::sqliteInt_h::Expr,
                        );
                        __p_ref.pLimit = pLimit;
                        __p_ref.iLimit = 0 as ::core::ffi::c_int;
                        __p_ref.iOffset = 0 as ::core::ffi::c_int;
                        if dest.eDest as ::core::ffi::c_int != priorOp
                            && (*db).mallocFailed as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        {
                            
                            
                            
                            let iBreak: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            );
                            let iCont: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            );
                            computeLimitRegisters(pParse, p, iBreak);
                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                v,
                                crate::src::headers::opcodes_h::OP_Rewind,
                                unionTab,
                                iBreak,
                            );
                            let iStart: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v);
                            selectInnerLoop(
                                pParse,
                                p,
                                unionTab,
                                ::core::ptr::null_mut::<SortCtx>(),
                                ::core::ptr::null_mut::<DistinctCtx>(),
                                &raw mut dest,
                                iCont,
                                iBreak,
                            );
                            crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, iCont);
                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                v,
                                crate::src::headers::opcodes_h::OP_Next,
                                unionTab,
                                iStart,
                            );
                            crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, iBreak);
                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                v,
                                crate::src::headers::opcodes_h::OP_Close,
                                unionTab,
                                0 as ::core::ffi::c_int,
                            );
                        }
                        current_block = 14579489411542934868;
                    }
                }
                _ => {
                    
                    
                    let iCont_0: ::core::ffi::c_int;
                    let iBreak_0: ::core::ffi::c_int;
                    let iStart_0: ::core::ffi::c_int;
                    let pLimit_0: *mut crate::src::headers::sqliteInt_h::Expr;
                    let mut addr_1: ::core::ffi::c_int;
                    let iLimit: ::core::ffi::c_int;
                    let iOffset: ::core::ffi::c_int;
                    let mut intersectdest: crate::src::headers::sqliteInt_h::SelectDest =
                        unsafe { ::core::mem::zeroed() };
                    let r1: ::core::ffi::c_int;
                    let emptyBypass_0: ::core::ffi::c_int;
                    let __pParse_ref = unsafe { &mut *pParse };
                    let fresh25 = __pParse_ref.nTab;
                    __pParse_ref.nTab += 1;
                    let tab1: ::core::ffi::c_int = fresh25;
                    let fresh26 = __pParse_ref.nTab;
                    __pParse_ref.nTab += 1;
                    let tab2: ::core::ffi::c_int = fresh26;
                    addr_1 = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                        v,
                        crate::src::headers::opcodes_h::OP_OpenEphemeral,
                        tab1,
                        0 as ::core::ffi::c_int,
                    );
                    (*p).addrOpenEphm[0 as ::core::ffi::c_int as usize] = addr_1;
                    (*findRightmost(p)).selFlags |=
                        crate::src::headers::sqliteInt_h::SF_UsesEphemeral
                            as crate::src::ext::rtree::rtree::U32_0;
                    sqlite3SelectDestInit(
                        &raw mut intersectdest,
                        crate::src::headers::sqliteInt_h::SRT_Union,
                        tab1,
                    );
                    rc = sqlite3Select(pParse, pPrior, &raw mut intersectdest);
                    if rc != 0 {
                        current_block = 4843618750404471500;
                    } else {
                        iBreak_0 = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        );
                        computeLimitRegisters(pParse, p, iBreak_0);
                        emptyBypass_0 = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                            v,
                            crate::src::headers::opcodes_h::OP_IfEmpty,
                            tab1,
                        );
                        addr_1 = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                            v,
                            crate::src::headers::opcodes_h::OP_OpenEphemeral,
                            tab2,
                            0 as ::core::ffi::c_int,
                        );
                        let __p_ref = unsafe { &mut *p };
                        __p_ref.addrOpenEphm[1 as ::core::ffi::c_int as usize] = addr_1;
                        pLimit_0 = __p_ref.pLimit;
                        iLimit = __p_ref.iLimit;
                        iOffset = __p_ref.iOffset;
                        __p_ref.pPrior =
                            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
                        __p_ref.pLimit =
                            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
                        __p_ref.iLimit = 0 as ::core::ffi::c_int;
                        __p_ref.iOffset = 0 as ::core::ffi::c_int;
                        intersectdest.iSDParm = tab2;
                        crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            1_u8,
                            b"%s USING TEMP B-TREE\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[crate::src::src::printf::PrintfArg::Str(
                                sqlite3SelectOpName(__p_ref.op as ::core::ffi::c_int)
                                    as *mut ::core::ffi::c_char,
                            )],
                        );
                        rc = sqlite3Select(pParse, p, &raw mut intersectdest);
                        pDelete = __p_ref.pPrior;
                        __p_ref.pPrior = pPrior;
                        if __p_ref.nSelectRow as ::core::ffi::c_int
                            > (*pPrior).nSelectRow as ::core::ffi::c_int
                        {
                            __p_ref.nSelectRow = (*pPrior).nSelectRow;
                        }
                        crate::src::src::expr::sqlite3ExprDelete(
                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                            __p_ref.pLimit as *mut crate::src::headers::sqliteInt_h::Expr,
                        );
                        __p_ref.pLimit = pLimit_0;
                        __p_ref.iLimit = iLimit;
                        __p_ref.iOffset = iOffset;
                        if rc != 0 {
                            current_block = 14579489411542934868;
                        } else {
                            crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                                v,
                                crate::src::headers::opcodes_h::OP_Rewind,
                                tab1,
                            );
                            r1 = crate::src::src::expr::sqlite3GetTempReg(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            );
                            iStart_0 = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                v,
                                crate::src::headers::opcodes_h::OP_RowData,
                                tab1,
                                r1,
                            );
                            iCont_0 = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            );
                            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                                v,
                                crate::src::headers::opcodes_h::OP_NotFound,
                                tab2,
                                iCont_0,
                                r1,
                                0 as ::core::ffi::c_int,
                            );
                            crate::src::src::expr::sqlite3ReleaseTempReg(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                r1,
                            );
                            selectInnerLoop(
                                pParse,
                                p,
                                tab1,
                                ::core::ptr::null_mut::<SortCtx>(),
                                ::core::ptr::null_mut::<DistinctCtx>(),
                                &raw mut dest,
                                iCont_0,
                                iBreak_0,
                            );
                            crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, iCont_0);
                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                v,
                                crate::src::headers::opcodes_h::OP_Next,
                                tab1,
                                iStart_0,
                            );
                            crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, iBreak_0);
                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                v,
                                crate::src::headers::opcodes_h::OP_Close,
                                tab2,
                                0 as ::core::ffi::c_int,
                            );
                            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, emptyBypass_0);
                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                v,
                                crate::src::headers::opcodes_h::OP_Close,
                                tab1,
                                0 as ::core::ffi::c_int,
                            );
                            current_block = 14579489411542934868;
                        }
                    }
                }
            }
            match current_block {
                4843618750404471500 => {}
                _ => {
                    if (*p).pNext.is_null() {
                        crate::src::src::vdbeaux::sqlite3VdbeExplainPop(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        );
                    }
                    current_block = 4235089732467486934;
                }
            }
        }
        match current_block {
            4843618750404471500 => {}
            _ => {
                if ((*pParse).nErr == 0)
                    && (*p).selFlags
                        & crate::src::headers::sqliteInt_h::SF_UsesEphemeral
                            as crate::src::ext::rtree::rtree::U32_0
                        != 0
                    {
                        let mut i: ::core::ffi::c_int;
                        
                        let mut pLoop: *mut crate::src::headers::sqliteInt_h::Select;
                        let mut apColl: *mut *mut crate::src::headers::sqliteInt_h::CollSeq;
                        
                        let nCol: ::core::ffi::c_int = (*(*p).pEList).nExpr;
                        let pKeyInfo: *mut crate::src::headers::sqliteInt_h::KeyInfo = sqlite3KeyInfoAlloc(db, nCol, 1 as ::core::ffi::c_int);
                        if pKeyInfo.is_null() {
                            rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
                        } else {
                            i = 0 as ::core::ffi::c_int;
                            apColl = &raw mut (*pKeyInfo).aColl
                                as *mut *mut crate::src::headers::sqliteInt_h::CollSeq;
                            while i < nCol {
                                *apColl = multiSelectCollSeq(pParse, p, i);
                                if (*apColl).is_null() {
                                    *apColl = (*db).pDfltColl;
                                }
                                i += 1;
                                apColl = apColl.offset(1);
                            }
                            pLoop = p;
                            while !pLoop.is_null() {
                                i = 0 as ::core::ffi::c_int;
                                while i < 2 as ::core::ffi::c_int {
                                    let addr_2: ::core::ffi::c_int =
                                        (*pLoop).addrOpenEphm[i as usize];
                                    if addr_2 < 0 as ::core::ffi::c_int {
                                        break;
                                    }
                                    crate::src::src::vdbeaux::sqlite3VdbeChangeP2(
                                        v, addr_2, nCol,
                                    );
                                    crate::src::src::vdbeaux::sqlite3VdbeChangeP4(
                                        v,
                                        addr_2,
                                        sqlite3KeyInfoRef(pKeyInfo) as *mut ::core::ffi::c_char,
                                        crate::src::src::vdbe::P4_KEYINFO,
                                    );
                                    (*pLoop).addrOpenEphm[i as usize] =
                                        -(1 as ::core::ffi::c_int);
                                    i += 1;
                                }
                                pLoop = (*pLoop).pPrior;
                            }
                            sqlite3KeyInfoUnref(pKeyInfo);
                        }
                    }
            }
        }
    }
    let __pDest_ref = unsafe { &mut *pDest };
    __pDest_ref.iSdst = dest.iSdst;
    __pDest_ref.nSdst = dest.nSdst;
    __pDest_ref.iSDParm2 = dest.iSDParm2;
    if !pDelete.is_null() {
        crate::src::src::prepare::sqlite3ParserAddCleanup(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            ::core::mem::transmute(Some(
                sqlite3SelectDeleteGeneric
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqliteInt_h::sqlite3,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            )),
            pDelete as *mut ::core::ffi::c_void,
        );
    }
    rc
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SelectWrongNumTermsError(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
) {
    if (*p).selFlags
        & crate::src::headers::sqliteInt_h::SF_Values as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"all VALUES must have the same number of terms\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        );
    } else {
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"SELECTs to the left and right of %s do not have the same number of result columns\0"
                as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Str(
                sqlite3SelectOpName((*p).op as ::core::ffi::c_int) as *mut ::core::ffi::c_char,
            )],
        );
    };
}

unsafe extern "C" fn generateOutputSubroutine(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
    pIn: *mut crate::src::headers::sqliteInt_h::SelectDest,
    pDest: *mut crate::src::headers::sqliteInt_h::SelectDest,
    regReturn: ::core::ffi::c_int,
    regPrev: ::core::ffi::c_int,
    pKeyInfo: *mut crate::src::headers::sqliteInt_h::KeyInfo,
    iBreak: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    
    
    let addr: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v);
    let iContinue: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
    );
    if regPrev != 0 {
        
        
        let addr1: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
            v,
            crate::src::headers::opcodes_h::OP_IfNot,
            regPrev,
        );
        let __pIn_ref = unsafe { &*pIn };
        let addr2: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
            v,
            crate::src::headers::opcodes_h::OP_Compare,
            __pIn_ref.iSdst,
            regPrev + 1 as ::core::ffi::c_int,
            __pIn_ref.nSdst,
            sqlite3KeyInfoRef(pKeyInfo) as *mut ::core::ffi::c_char,
            crate::src::src::vdbe::P4_KEYINFO,
        );
        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
            v,
            crate::src::headers::opcodes_h::OP_Jump,
            addr2 + 2 as ::core::ffi::c_int,
            iContinue,
            addr2 + 2 as ::core::ffi::c_int,
        );
        crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addr1);
        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
            v,
            crate::src::headers::opcodes_h::OP_Copy,
            __pIn_ref.iSdst,
            regPrev + 1 as ::core::ffi::c_int,
            __pIn_ref.nSdst - 1 as ::core::ffi::c_int,
        );
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Integer,
            1 as ::core::ffi::c_int,
            regPrev,
        );
    }
    if (*(*pParse).db).mallocFailed != 0 {
        return 0 as ::core::ffi::c_int;
    }
    codeOffset(v, (*p).iOffset, iContinue);
    match (*pDest).eDest as ::core::ffi::c_int {
        crate::src::headers::sqliteInt_h::SRT_EphemTab_1 => {
            let r1: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempReg(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            );
            let r2: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempReg(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_MakeRecord,
                (*pIn).iSdst,
                (*pIn).nSdst,
                r1,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_NewRowid,
                (*pDest).iSDParm,
                r2,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_Insert,
                (*pDest).iSDParm,
                r1,
                r2,
            );
            crate::src::src::vdbeaux::sqlite3VdbeChangeP5(
                v,
                crate::src::headers::sqliteInt_h::OPFLAG_APPEND as crate::src::fts5::U16_0,
            );
            crate::src::src::expr::sqlite3ReleaseTempReg(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                r2,
            );
            crate::src::src::expr::sqlite3ReleaseTempReg(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                r1,
            );
        }
        crate::src::headers::sqliteInt_h::SRT_Set => {
            
            let r1_0: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempReg(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            );
            let __pDest_ref = unsafe { &*pDest };
            let __pIn_ref = unsafe { &*pIn };
            crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                v,
                crate::src::headers::opcodes_h::OP_MakeRecord,
                __pIn_ref.iSdst,
                __pIn_ref.nSdst,
                r1_0,
                __pDest_ref.zAffSdst,
                __pIn_ref.nSdst,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                v,
                crate::src::headers::opcodes_h::OP_IdxInsert,
                __pDest_ref.iSDParm,
                r1_0,
                __pIn_ref.iSdst,
                __pIn_ref.nSdst,
            );
            if __pDest_ref.iSDParm2 > 0 as ::core::ffi::c_int {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                    v,
                    crate::src::headers::opcodes_h::OP_FilterAdd,
                    __pDest_ref.iSDParm2,
                    0 as ::core::ffi::c_int,
                    __pIn_ref.iSdst,
                    __pIn_ref.nSdst,
                );
                crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    0_u8,
                    b"CREATE BLOOM FILTER\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            crate::src::src::expr::sqlite3ReleaseTempReg(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                r1_0,
            );
        }
        crate::src::headers::sqliteInt_h::SRT_Mem_1 => {
            crate::src::src::expr::sqlite3ExprCodeMove(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*pIn).iSdst,
                (*pDest).iSDParm,
                (*pIn).nSdst,
            );
        }
        crate::src::headers::sqliteInt_h::SRT_Coroutine => {
            let __pDest_ref = unsafe { &mut *pDest };
            if __pDest_ref.iSdst == 0 as ::core::ffi::c_int {
                __pDest_ref.iSdst = crate::src::src::expr::sqlite3GetTempRange(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*pIn).nSdst,
                );
                __pDest_ref.nSdst = (*pIn).nSdst;
            }
            crate::src::src::expr::sqlite3ExprCodeMove(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*pIn).iSdst,
                __pDest_ref.iSdst,
                (*pIn).nSdst,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                v,
                crate::src::headers::opcodes_h::OP_Yield,
                __pDest_ref.iSDParm,
            );
        }
        _ => {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_ResultRow,
                (*pIn).iSdst,
                (*pIn).nSdst,
            );
        }
    }
    if (*p).iLimit != 0 {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_DecrJumpZero,
            (*p).iLimit,
            iBreak,
        );
    }
    crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, iContinue);
    crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
        v,
        crate::src::headers::opcodes_h::OP_Return,
        regReturn,
    );
    addr
}

unsafe extern "C" fn multiSelectOrderBy(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
    pDest: *mut crate::src::headers::sqliteInt_h::SelectDest,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    
    let mut pSplit: *mut crate::src::headers::sqliteInt_h::Select;
    let mut nSelect: ::core::ffi::c_int;
    
    let mut destA: crate::src::headers::sqliteInt_h::SelectDest = unsafe { ::core::mem::zeroed() };
    let mut destB: crate::src::headers::sqliteInt_h::SelectDest = unsafe { ::core::mem::zeroed() };
    
    
    
    
    
    
    
    let mut addrOutB: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let addrEofA: ::core::ffi::c_int;
    let addrEofA_noB: ::core::ffi::c_int;
    let addrEofB: ::core::ffi::c_int;
    let mut addrAltB: ::core::ffi::c_int;
    let addrAeqB: ::core::ffi::c_int;
    
    let regLimitA: ::core::ffi::c_int;
    let regLimitB: ::core::ffi::c_int;
    let regPrev: ::core::ffi::c_int;
    
    
    
    
    let mut addr1: ::core::ffi::c_int;
    
    let mut pKeyDup: *mut crate::src::headers::sqliteInt_h::KeyInfo =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::KeyInfo>();
    let pKeyMerge: *mut crate::src::headers::sqliteInt_h::KeyInfo;
    
    let mut pOrderBy: *mut crate::src::headers::sqliteInt_h::ExprList;
    let mut nOrderBy: ::core::ffi::c_int;
    
    let __pParse_ref = unsafe { &mut *pParse };
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = __pParse_ref.pVdbe;
    let labelEnd: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
    );
    let labelCmpr: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
    );
    let __p_ref = unsafe { &mut *p };
    let op: ::core::ffi::c_int = __p_ref.op as ::core::ffi::c_int;
    pOrderBy = __p_ref.pOrderBy;
    nOrderBy = (*pOrderBy).nExpr;
    if op != crate::src::parse::TK_ALL {
        i = 1 as ::core::ffi::c_int;
        while (*db).mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && i <= (*__p_ref.pEList).nExpr
        {
            let mut pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item;
            j = 0 as ::core::ffi::c_int;
            pItem = &raw mut (*pOrderBy).a as *mut crate::src::headers::sqliteInt_h::ExprList_item
                as *mut crate::src::headers::sqliteInt_h::ExprList_item;
            while j < nOrderBy {
                if (*pItem).u.x.iOrderByCol as ::core::ffi::c_int == i {
                    break;
                }
                j += 1;
                pItem = pItem.offset(1);
            }
            if j == nOrderBy {
                let pNew: *mut crate::src::headers::sqliteInt_h::Expr =
                    crate::src::src::expr::sqlite3Expr(
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        crate::src::parse::TK_INTEGER,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    ) as *mut crate::src::headers::sqliteInt_h::Expr;
                if pNew.is_null() {
                    return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
                }
                (*pNew).flags |= crate::src::headers::sqliteInt_h::EP_IntValue
                    as crate::src::ext::rtree::rtree::U32_0;
                (*pNew).u.iValue = i;
                pOrderBy = crate::src::src::expr::sqlite3ExprListAppend(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    pOrderBy as *mut crate::src::headers::sqliteInt_h::ExprList,
                    pNew as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::ExprList;
                __p_ref.pOrderBy = pOrderBy;
                if !pOrderBy.is_null() {
                    let fresh27 = nOrderBy;
                    nOrderBy += 1;
                    (*(&raw mut (*pOrderBy).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(fresh27 as isize))
                    .u
                    .x
                    .iOrderByCol = i as crate::src::fts5::U16_0;
                }
            }
            i += 1;
        }
    }
    let aPermute: *mut crate::src::ext::rtree::rtree::U32_0 = crate::src::src::malloc::sqlite3DbMallocRawNN(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        (::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>() as usize)
            .wrapping_mul((nOrderBy + 1 as ::core::ffi::c_int) as usize)
            as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::ext::rtree::rtree::U32_0;
    if !aPermute.is_null() {
        let mut pItem_0: *mut crate::src::headers::sqliteInt_h::ExprList_item;
        *aPermute.offset(0_isize) = nOrderBy as crate::src::ext::rtree::rtree::U32_0;
        i = 1 as ::core::ffi::c_int;
        pItem_0 = &raw mut (*pOrderBy).a as *mut crate::src::headers::sqliteInt_h::ExprList_item
            as *mut crate::src::headers::sqliteInt_h::ExprList_item;
        while i <= nOrderBy {
            *aPermute.offset(i as isize) = ((*pItem_0).u.x.iOrderByCol as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as crate::src::ext::rtree::rtree::U32_0;
            i += 1;
            pItem_0 = pItem_0.offset(1);
        }
        pKeyMerge = multiSelectOrderByKeyInfo(pParse, p, 1 as ::core::ffi::c_int);
    } else {
        pKeyMerge = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::KeyInfo>();
    }
    if op == crate::src::parse::TK_ALL {
        regPrev = 0 as ::core::ffi::c_int;
    } else {
        let nExpr: ::core::ffi::c_int = (*__p_ref.pEList).nExpr;
        regPrev = __pParse_ref.nMem + 1 as ::core::ffi::c_int;
        __pParse_ref.nMem += nExpr + 1 as ::core::ffi::c_int;
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Integer,
            0 as ::core::ffi::c_int,
            regPrev,
        );
        pKeyDup = sqlite3KeyInfoAlloc(db, nExpr, 1 as ::core::ffi::c_int);
        if !pKeyDup.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < nExpr {
                let fresh28 = &mut *(&raw mut (*pKeyDup).aColl
                    as *mut *mut crate::src::headers::sqliteInt_h::CollSeq)
                    .offset(i as isize);
                *fresh28 = multiSelectCollSeq(pParse, p, i);
                *(*pKeyDup).aSortFlags.offset(i as isize) = 0_u8;
                i += 1;
            }
        }
    }
    nSelect = 1 as ::core::ffi::c_int;
    if (op == crate::src::parse::TK_ALL || op == crate::src::parse::TK_UNION)
        && (*db).dbOptFlags & 0x200000 as crate::src::ext::rtree::rtree::U32_0
            == 0 as crate::src::ext::rtree::rtree::U32_0
    {
        pSplit = p;
        while !(*pSplit).pPrior.is_null() && (*pSplit).op as ::core::ffi::c_int == op {
            nSelect += 1;
            pSplit = (*pSplit).pPrior;
        }
    }
    if nSelect <= 3 as ::core::ffi::c_int {
        pSplit = p;
    } else {
        pSplit = p;
        i = 2 as ::core::ffi::c_int;
        while i < nSelect {
            pSplit = (*pSplit).pPrior;
            i += 2 as ::core::ffi::c_int;
        }
    }
    let pPrior: *mut crate::src::headers::sqliteInt_h::Select = (*pSplit).pPrior;
    (*pSplit).pPrior = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    (*pPrior).pNext = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    (*pPrior).pOrderBy = crate::src::src::expr::sqlite3ExprListDup(
        __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        pOrderBy as *const crate::src::headers::sqliteInt_h::ExprList,
        0 as ::core::ffi::c_int,
    ) as *mut crate::src::headers::sqliteInt_h::ExprList;
    crate::src::src::resolve::sqlite3ResolveOrderGroupBy(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        p as *mut crate::src::headers::sqliteInt_h::Select,
        __p_ref.pOrderBy as *mut crate::src::headers::sqliteInt_h::ExprList,
        b"ORDER\0" as *const u8 as *const ::core::ffi::c_char,
    );
    crate::src::src::resolve::sqlite3ResolveOrderGroupBy(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        pPrior as *mut crate::src::headers::sqliteInt_h::Select,
        (*pPrior).pOrderBy as *mut crate::src::headers::sqliteInt_h::ExprList,
        b"ORDER\0" as *const u8 as *const ::core::ffi::c_char,
    );
    computeLimitRegisters(pParse, p, labelEnd);
    if __p_ref.iLimit != 0 && op == crate::src::parse::TK_ALL {
        __pParse_ref.nMem += 1;
        regLimitA = __pParse_ref.nMem;
        __pParse_ref.nMem += 1;
        regLimitB = __pParse_ref.nMem;
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Copy,
            if __p_ref.iOffset != 0 {
                __p_ref.iOffset + 1 as ::core::ffi::c_int
            } else {
                __p_ref.iLimit
            },
            regLimitA,
        );
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Copy,
            regLimitA,
            regLimitB,
        );
    } else {
        regLimitB = 0 as ::core::ffi::c_int;
        regLimitA = regLimitB;
    }
    crate::src::src::expr::sqlite3ExprDelete(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __p_ref.pLimit as *mut crate::src::headers::sqliteInt_h::Expr,
    );
    __p_ref.pLimit = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
    __pParse_ref.nMem += 1;
    let regAddrA: ::core::ffi::c_int = __pParse_ref.nMem;
    __pParse_ref.nMem += 1;
    let regAddrB: ::core::ffi::c_int = __pParse_ref.nMem;
    __pParse_ref.nMem += 1;
    let regOutA: ::core::ffi::c_int = __pParse_ref.nMem;
    __pParse_ref.nMem += 1;
    let regOutB: ::core::ffi::c_int = __pParse_ref.nMem;
    sqlite3SelectDestInit(
        &raw mut destA,
        crate::src::headers::sqliteInt_h::SRT_Coroutine,
        regAddrA,
    );
    sqlite3SelectDestInit(
        &raw mut destB,
        crate::src::headers::sqliteInt_h::SRT_Coroutine,
        regAddrB,
    );
    crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        1_u8,
        b"MERGE (%s)\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::src::src::printf::PrintfArg::Str(
            sqlite3SelectOpName(__p_ref.op as ::core::ffi::c_int) as *mut ::core::ffi::c_char,
        )],
    );
    let addrSelectA: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v) + 1 as ::core::ffi::c_int;
    addr1 = crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
        v,
        crate::src::headers::opcodes_h::OP_InitCoroutine,
        regAddrA,
        0 as ::core::ffi::c_int,
        addrSelectA,
    );
    (*pPrior).iLimit = regLimitA;
    crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        1_u8,
        b"LEFT\0" as *const u8 as *const ::core::ffi::c_char,
        &[],
    );
    sqlite3Select(pParse, pPrior, &raw mut destA);
    crate::src::src::vdbeaux::sqlite3VdbeEndCoroutine(v, regAddrA);
    crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addr1);
    let addrSelectB: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v) + 1 as ::core::ffi::c_int;
    addr1 = crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
        v,
        crate::src::headers::opcodes_h::OP_InitCoroutine,
        regAddrB,
        0 as ::core::ffi::c_int,
        addrSelectB,
    );
    let savedLimit: ::core::ffi::c_int = __p_ref.iLimit;
    let savedOffset: ::core::ffi::c_int = __p_ref.iOffset;
    __p_ref.iLimit = regLimitB;
    __p_ref.iOffset = 0 as ::core::ffi::c_int;
    crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        1_u8,
        b"RIGHT\0" as *const u8 as *const ::core::ffi::c_char,
        &[],
    );
    sqlite3Select(pParse, p, &raw mut destB);
    __p_ref.iLimit = savedLimit;
    __p_ref.iOffset = savedOffset;
    crate::src::src::vdbeaux::sqlite3VdbeEndCoroutine(v, regAddrB);
    let addrOutA: ::core::ffi::c_int = generateOutputSubroutine(
        pParse,
        p,
        &raw mut destA,
        pDest,
        regOutA,
        regPrev,
        pKeyDup,
        labelEnd,
    );
    if op == crate::src::parse::TK_ALL || op == crate::src::parse::TK_UNION {
        addrOutB = generateOutputSubroutine(
            pParse,
            p,
            &raw mut destB,
            pDest,
            regOutB,
            regPrev,
            pKeyDup,
            labelEnd,
        );
    }
    sqlite3KeyInfoUnref(pKeyDup);
    if op == crate::src::parse::TK_EXCEPT || op == crate::src::parse::TK_INTERSECT {
        addrEofA = labelEnd;
        addrEofA_noB = addrEofA;
    } else {
        addrEofA = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Gosub,
            regOutB,
            addrOutB,
        );
        addrEofA_noB = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Yield,
            regAddrB,
            labelEnd,
        );
        crate::src::src::vdbeaux::sqlite3VdbeGoto(v, addrEofA);
        __p_ref.nSelectRow =
            crate::src::src::util::sqlite3LogEstAdd(__p_ref.nSelectRow, (*pPrior).nSelectRow);
    }
    if op == crate::src::parse::TK_INTERSECT {
        addrEofB = addrEofA;
        if __p_ref.nSelectRow as ::core::ffi::c_int > (*pPrior).nSelectRow as ::core::ffi::c_int {
            __p_ref.nSelectRow = (*pPrior).nSelectRow;
        }
    } else {
        addrEofB = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Gosub,
            regOutA,
            addrOutA,
        );
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Yield,
            regAddrA,
            labelEnd,
        );
        crate::src::src::vdbeaux::sqlite3VdbeGoto(v, addrEofB);
    }
    addrAltB = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
        v,
        crate::src::headers::opcodes_h::OP_Gosub,
        regOutA,
        addrOutA,
    );
    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
        v,
        crate::src::headers::opcodes_h::OP_Yield,
        regAddrA,
        addrEofA,
    );
    crate::src::src::vdbeaux::sqlite3VdbeGoto(v, labelCmpr);
    if op == crate::src::parse::TK_ALL {
        addrAeqB = addrAltB;
    } else if op == crate::src::parse::TK_INTERSECT {
        addrAeqB = addrAltB;
        addrAltB += 1;
    } else {
        addrAeqB = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Yield,
            regAddrA,
            addrEofA,
        );
        crate::src::src::vdbeaux::sqlite3VdbeGoto(v, labelCmpr);
    }
    let addrAgtB: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v);
    if op == crate::src::parse::TK_ALL || op == crate::src::parse::TK_UNION {
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_Gosub,
            regOutB,
            addrOutB,
        );
    }
    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
        v,
        crate::src::headers::opcodes_h::OP_Yield,
        regAddrB,
        addrEofB,
    );
    crate::src::src::vdbeaux::sqlite3VdbeGoto(v, labelCmpr);
    crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addr1);
    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
        v,
        crate::src::headers::opcodes_h::OP_Yield,
        regAddrA,
        addrEofA_noB,
    );
    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
        v,
        crate::src::headers::opcodes_h::OP_Yield,
        regAddrB,
        addrEofB,
    );
    crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, labelCmpr);
    crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
        v,
        crate::src::headers::opcodes_h::OP_Permutation,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        aPermute as *mut ::core::ffi::c_char,
        crate::src::src::vdbe::P4_INTARRAY,
    );
    crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
        v,
        crate::src::headers::opcodes_h::OP_Compare,
        destA.iSdst,
        destB.iSdst,
        nOrderBy,
        pKeyMerge as *mut ::core::ffi::c_char,
        crate::src::src::vdbe::P4_KEYINFO,
    );
    crate::src::src::vdbeaux::sqlite3VdbeChangeP5(
        v,
        crate::src::headers::sqliteInt_h::OPFLAG_PERMUTE as crate::src::fts5::U16_0,
    );
    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
        v,
        crate::src::headers::opcodes_h::OP_Jump,
        addrAltB,
        addrAeqB,
        addrAgtB,
    );
    crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, labelEnd);
    if !(*pSplit).pPrior.is_null() {
        crate::src::src::prepare::sqlite3ParserAddCleanup(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            ::core::mem::transmute(Some(
                sqlite3SelectDeleteGeneric
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqliteInt_h::sqlite3,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            )),
            (*pSplit).pPrior as *mut ::core::ffi::c_void,
        );
    }
    (*pSplit).pPrior = pPrior;
    (*pPrior).pNext = pSplit;
    crate::src::src::expr::sqlite3ExprListDelete(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        (*pPrior).pOrderBy as *mut crate::src::headers::sqliteInt_h::ExprList,
    );
    (*pPrior).pOrderBy = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
    crate::src::src::vdbeaux::sqlite3VdbeExplainPop(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
    );
    (__pParse_ref.nErr != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
}

unsafe extern "C" fn substExpr(
    pSubst: *mut SubstContext,
    mut pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    if pExpr.is_null() {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
    }
    if (*pExpr).flags
        & (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
        && (*pExpr).w.iJoin == (*pSubst).iTable
    {
        (*pExpr).w.iJoin = (*pSubst).iNewTable;
    }
    if (*pExpr).op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN_1
        && (*pExpr).iTable == (*pSubst).iTable
        && ((*pExpr).flags & 0x20 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
    {
        let mut pNew: *mut crate::src::headers::sqliteInt_h::Expr;
        
        let mut pCopy: *mut crate::src::headers::sqliteInt_h::Expr;
        let mut ifNullRow: crate::src::headers::sqliteInt_h::Expr =
            unsafe { ::core::mem::zeroed() };
        let iColumn: ::core::ffi::c_int = (*pExpr).iColumn as ::core::ffi::c_int;
        pCopy = (*(&raw mut (*(*pSubst).pEList).a
            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(iColumn as isize))
        .pExpr;
        if crate::src::src::expr::sqlite3ExprIsVector(
            pCopy as *const crate::src::headers::sqliteInt_h::Expr,
        ) != 0
        {
            crate::src::src::expr::sqlite3VectorErrorMsg(
                (*pSubst).pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                pCopy as *mut crate::src::headers::sqliteInt_h::Expr,
            );
        } else {
            let __pSubst_ref = unsafe { &mut *pSubst };
            let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*__pSubst_ref.pParse).db;
            if __pSubst_ref.isOuterJoin != 0
                && ((*pCopy).op as ::core::ffi::c_int != crate::src::parse::TK_COLUMN_1
                    || (*pCopy).iTable != __pSubst_ref.iNewTable)
            {
                ifNullRow.op = crate::src::parse::TK_IF_NULL_ROW_1 as u8;
                ifNullRow.pLeft = pCopy;
                ifNullRow.iTable = __pSubst_ref.iNewTable;
                ifNullRow.iColumn =
                    -(99 as ::core::ffi::c_int) as crate::src::headers::sqliteInt_h::YnVar;
                ifNullRow.flags = crate::src::headers::sqliteInt_h::EP_IfNullRow
                    as crate::src::ext::rtree::rtree::U32_0;
                pCopy = &raw mut ifNullRow;
            }
            pNew = crate::src::src::expr::sqlite3ExprDup(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                pCopy as *const crate::src::headers::sqliteInt_h::Expr,
                0 as ::core::ffi::c_int,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            if (*db).mallocFailed != 0 {
                crate::src::src::expr::sqlite3ExprDelete(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    pNew as *mut crate::src::headers::sqliteInt_h::Expr,
                );
                return pExpr;
            }
            if __pSubst_ref.isOuterJoin != 0 {
                (*pNew).flags |=
                    0x200000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
            }
            if (*pNew).op as ::core::ffi::c_int == crate::src::parse::TK_TRUEFALSE {
                (*pNew).u.iValue = crate::src::src::expr::sqlite3ExprTruthValue(
                    pNew as *const crate::src::headers::sqliteInt_h::Expr,
                );
                (*pNew).op = crate::src::parse::TK_INTEGER as u8;
                (*pNew).flags |=
                    0x800 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
            }
            let pNat: *mut crate::src::headers::sqliteInt_h::CollSeq =
                crate::src::src::expr::sqlite3ExprCollSeq(
                    __pSubst_ref.pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    pNew as *const crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::CollSeq;
            let pColl: *mut crate::src::headers::sqliteInt_h::CollSeq =
                crate::src::src::expr::sqlite3ExprCollSeq(
                    __pSubst_ref.pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*(&raw mut (*__pSubst_ref.pCList).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(iColumn as isize))
                    .pExpr as *const crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::CollSeq;
            if pNat != pColl
                || (*pNew).op as ::core::ffi::c_int != crate::src::parse::TK_COLUMN_1
                    && (*pNew).op as ::core::ffi::c_int != crate::src::parse::TK_COLLATE
            {
                pNew = crate::src::src::expr::sqlite3ExprAddCollateString(
                    __pSubst_ref.pParse as *const crate::src::headers::sqliteInt_h::Parse,
                    pNew as *mut crate::src::headers::sqliteInt_h::Expr,
                    if !pColl.is_null() {
                        (*pColl).zName as *const ::core::ffi::c_char
                    } else {
                        b"BINARY\0" as *const u8 as *const ::core::ffi::c_char
                    },
                ) as *mut crate::src::headers::sqliteInt_h::Expr;
            }
            (*pNew).flags &= !(0x200 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0);
            if (*pExpr).flags
                & (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int)
                    as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
            {
                sqlite3SetJoinExpr(
                    pNew,
                    (*pExpr).w.iJoin,
                    (*pExpr).flags
                        & (crate::src::headers::sqliteInt_h::EP_OuterON
                            | crate::src::headers::sqliteInt_h::EP_InnerON)
                            as crate::src::ext::rtree::rtree::U32_0,
                );
            }
            crate::src::src::expr::sqlite3ExprDelete(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
            );
            pExpr = pNew;
        }
    } else {
        let __pExpr_ref = unsafe { &mut *pExpr };
        if __pExpr_ref.op as ::core::ffi::c_int == crate::src::parse::TK_IF_NULL_ROW_1
            && __pExpr_ref.iTable == (*pSubst).iTable
        {
            __pExpr_ref.iTable = (*pSubst).iNewTable;
        }
        if __pExpr_ref.op as ::core::ffi::c_int == crate::src::parse::TK_AGG_FUNCTION
            && __pExpr_ref.op2 as ::core::ffi::c_int >= (*pSubst).nSelDepth
        {
            __pExpr_ref.op2 = __pExpr_ref.op2.wrapping_sub(1);
        }
        __pExpr_ref.pLeft = substExpr(pSubst, __pExpr_ref.pLeft);
        __pExpr_ref.pRight = substExpr(pSubst, __pExpr_ref.pRight);
        if __pExpr_ref.flags
            & crate::src::headers::sqliteInt_h::EP_xIsSelect as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
        {
            substSelect(pSubst, __pExpr_ref.x.pSelect, 1 as ::core::ffi::c_int);
        } else {
            substExprList(pSubst, __pExpr_ref.x.pList);
        }
        if __pExpr_ref.flags
            & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
        {
            let pWin: *mut crate::src::headers::sqliteInt_h::Window = __pExpr_ref.y.pWin;
            (*pWin).pFilter = substExpr(pSubst, (*pWin).pFilter);
            substExprList(pSubst, (*pWin).pPartition);
            substExprList(pSubst, (*pWin).pOrderBy);
        }
    }
    pExpr
}

unsafe extern "C" fn substExprList(
    pSubst: *mut SubstContext,
    pList: *mut crate::src::headers::sqliteInt_h::ExprList,
) {
    let mut i: ::core::ffi::c_int;
    if pList.is_null() {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*pList).nExpr {
        let fresh17 = &mut (*(&raw mut (*pList).a
            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(i as isize))
        .pExpr;
        *fresh17 = substExpr(
            pSubst,
            (*(&raw mut (*pList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(i as isize))
            .pExpr,
        );
        i += 1;
    }
}

unsafe extern "C" fn substSelect(
    pSubst: *mut SubstContext,
    mut p: *mut crate::src::headers::sqliteInt_h::Select,
    doPrior: ::core::ffi::c_int,
) {
    let mut pSrc: *mut crate::src::headers::sqliteInt_h::SrcList;
    let mut pItem: *mut crate::src::headers::sqliteInt_h::SrcItem;
    let mut i: ::core::ffi::c_int;
    if p.is_null() {
        return;
    }
    (*pSubst).nSelDepth += 1;
    loop {
        substExprList(pSubst, (*p).pEList);
        substExprList(pSubst, (*p).pGroupBy);
        substExprList(pSubst, (*p).pOrderBy);
        (*p).pHaving = substExpr(pSubst, (*p).pHaving);
        (*p).pWhere = substExpr(pSubst, (*p).pWhere);
        pSrc = (*p).pSrc;
        i = (*pSrc).nSrc;
        pItem = &raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem;
        while i > 0 as ::core::ffi::c_int {
            if (*pItem).fg.isSubquery() != 0 {
                substSelect(
                    pSubst,
                    (*(*pItem).u4.pSubq).pSelect,
                    1 as ::core::ffi::c_int,
                );
            }
            if (*pItem).fg.isTabFunc() != 0 {
                substExprList(pSubst, (*pItem).u1.pFuncArg);
            }
            i -= 1;
            pItem = pItem.offset(1);
        }
        if !(doPrior != 0 && {
            p = (*p).pPrior;
            !p.is_null()
        }) {
            break;
        }
    }
    (*pSubst).nSelDepth -= 1;
}

unsafe extern "C" fn recomputeColumnsUsedExpr(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    
    let __pExpr_ref = unsafe { &*pExpr };
    if __pExpr_ref.op as ::core::ffi::c_int != crate::src::parse::TK_COLUMN_1 {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    let pItem: *mut crate::src::headers::sqliteInt_h::SrcItem = (*pWalker).u.pSrcItem;
    if (*pItem).iCursor != __pExpr_ref.iTable {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    if (__pExpr_ref.iColumn as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    (*pItem).colUsed |= crate::src::src::resolve::sqlite3ExprColUsed(
        pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
    );
    crate::src::headers::sqliteInt_h::WRC_Continue
}

unsafe extern "C" fn recomputeColumnsUsed(
    pSelect: *mut crate::src::headers::sqliteInt_h::Select,
    pSrcItem: *mut crate::src::headers::sqliteInt_h::SrcItem,
) {
    let mut w: crate::src::headers::sqliteInt_h::Walker = unsafe { ::core::mem::zeroed() };
    if (*pSrcItem).pSTab.is_null() {
        return;
    }
    w.xExprCallback = Some(
        recomputeColumnsUsedExpr
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
    w.u.pSrcItem = pSrcItem;
    (*pSrcItem).colUsed = 0 as crate::src::headers::sqliteInt_h::Bitmask;
    crate::src::src::walker::sqlite3WalkSelect(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        pSelect as *mut crate::src::headers::sqliteInt_h::Select,
    );
}

unsafe extern "C" fn srclistRenumberCursors(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    aCsrMap: *mut ::core::ffi::c_int,
    pSrc: *mut crate::src::headers::sqliteInt_h::SrcList,
    iExcept: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int;
    let mut pItem: *mut crate::src::headers::sqliteInt_h::SrcItem;
    i = 0 as ::core::ffi::c_int;
    pItem = &raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem;
    while i < (*pSrc).nSrc {
        if i != iExcept {
            let mut p: *mut crate::src::headers::sqliteInt_h::Select;
            let __pItem_ref = unsafe { &mut *pItem };
            if __pItem_ref.fg.isRecursive() == 0
                || *aCsrMap.offset((__pItem_ref.iCursor + 1 as ::core::ffi::c_int) as isize)
                    == 0 as ::core::ffi::c_int
            {
                let __pParse_ref = unsafe { &mut *pParse };
                let fresh23 = __pParse_ref.nTab;
                __pParse_ref.nTab += 1;
                *aCsrMap.offset((__pItem_ref.iCursor + 1 as ::core::ffi::c_int) as isize) = fresh23;
            }
            __pItem_ref.iCursor =
                *aCsrMap.offset((__pItem_ref.iCursor + 1 as ::core::ffi::c_int) as isize);
            if __pItem_ref.fg.isSubquery() != 0 {
                p = (*__pItem_ref.u4.pSubq).pSelect;
                while !p.is_null() {
                    srclistRenumberCursors(pParse, aCsrMap, (*p).pSrc, -(1 as ::core::ffi::c_int));
                    p = (*p).pPrior;
                }
            }
        }
        i += 1;
        pItem = pItem.offset(1);
    }
}

unsafe extern "C" fn renumberCursorDoMapping(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    piCursor: *mut ::core::ffi::c_int,
) {
    let aCsrMap: *mut ::core::ffi::c_int = (*pWalker).u.aiCol;
    let iCsr: ::core::ffi::c_int = *piCursor;
    if iCsr < *aCsrMap.offset(0_isize)
        && *aCsrMap.offset((iCsr + 1 as ::core::ffi::c_int) as isize) > 0 as ::core::ffi::c_int
    {
        *piCursor = *aCsrMap.offset((iCsr + 1 as ::core::ffi::c_int) as isize);
    }
}

unsafe extern "C" fn renumberCursorsCb(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let op: ::core::ffi::c_int = (*pExpr).op as ::core::ffi::c_int;
    if op == crate::src::parse::TK_COLUMN_1 || op == crate::src::parse::TK_IF_NULL_ROW_1 {
        renumberCursorDoMapping(pWalker, &raw mut (*pExpr).iTable);
    }
    if (*pExpr).flags & 0x1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        renumberCursorDoMapping(pWalker, &raw mut (*pExpr).w.iJoin);
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}

unsafe extern "C" fn renumberCursors(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
    iExcept: ::core::ffi::c_int,
    aCsrMap: *mut ::core::ffi::c_int,
) {
    let mut w: crate::src::headers::sqliteInt_h::Walker = unsafe { ::core::mem::zeroed() };
    srclistRenumberCursors(pParse, aCsrMap, (*p).pSrc, iExcept);
    w.u.aiCol = aCsrMap;
    w.xExprCallback = Some(
        renumberCursorsCb
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
    crate::src::src::walker::sqlite3WalkSelect(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        p as *mut crate::src::headers::sqliteInt_h::Select,
    );
}

unsafe extern "C" fn findLeftmostExprlist(
    mut pSel: *mut crate::src::headers::sqliteInt_h::Select,
) -> *mut crate::src::headers::sqliteInt_h::ExprList {
    while !(*pSel).pPrior.is_null() {
        pSel = (*pSel).pPrior;
    }
    (*pSel).pEList
}

unsafe extern "C" fn compoundHasDifferentAffinities(
    p: *mut crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    let mut ii: ::core::ffi::c_int;
    
    let pList: *mut crate::src::headers::sqliteInt_h::ExprList = (*p).pEList;
    ii = 0 as ::core::ffi::c_int;
    while ii < (*pList).nExpr {
        
        let mut pSub1: *mut crate::src::headers::sqliteInt_h::Select;
        let aff: ::core::ffi::c_char = crate::src::src::expr::sqlite3ExprAffinity(
            (*(&raw mut (*pList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(ii as isize))
            .pExpr as *const crate::src::headers::sqliteInt_h::Expr,
        );
        pSub1 = (*p).pPrior;
        while !pSub1.is_null() {
            if crate::src::src::expr::sqlite3ExprAffinity(
                (*(&raw mut (*(*pSub1).pEList).a
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(ii as isize))
                .pExpr as *const crate::src::headers::sqliteInt_h::Expr,
            ) as ::core::ffi::c_int
                != aff as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            pSub1 = (*pSub1).pPrior;
        }
        ii += 1;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn flattenSubquery(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
    iFrom: ::core::ffi::c_int,
    isAgg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pParse_ref = unsafe { &mut *pParse };
    let zSavedAuthContext: *const ::core::ffi::c_char = __pParse_ref.zAuthContext;
    let mut pParent: *mut crate::src::headers::sqliteInt_h::Select;
    let mut pSub: *mut crate::src::headers::sqliteInt_h::Select;
    let mut pSub1: *mut crate::src::headers::sqliteInt_h::Select;
    let mut pSrc: *mut crate::src::headers::sqliteInt_h::SrcList;
    let mut pSubSrc: *mut crate::src::headers::sqliteInt_h::SrcList;
    
    let mut iNewParent: ::core::ffi::c_int;
    let mut isOuterJoin: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    let mut pWhere: *mut crate::src::headers::sqliteInt_h::Expr;
    let mut pSubitem: *mut crate::src::headers::sqliteInt_h::SrcItem;
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
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
    let mut aCsrMap: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    if (*db).dbOptFlags & 0x1 as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return 0 as ::core::ffi::c_int;
    }
    let __p_ref = unsafe { &mut *p };
    pSrc = __p_ref.pSrc;
    pSubitem = (&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
        .offset(iFrom as isize) as *mut crate::src::headers::sqliteInt_h::SrcItem;
    let iParent: ::core::ffi::c_int = (*pSubitem).iCursor;
    pSub = (*(*pSubitem).u4.pSubq).pSelect;
    if !__p_ref.pWin.is_null() || !(*pSub).pWin.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    pSubSrc = (*pSub).pSrc;
    if !(*pSub).pLimit.is_null() && !__p_ref.pLimit.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !(*pSub).pLimit.is_null() && !(*(*pSub).pLimit).pRight.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if __p_ref.selFlags
        & crate::src::headers::sqliteInt_h::SF_Compound as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
        && !(*pSub).pLimit.is_null()
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*pSubSrc).nSrc == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*pSub).selFlags
        & crate::src::headers::sqliteInt_h::SF_Distinct as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !(*pSub).pLimit.is_null() && ((*pSrc).nSrc > 1 as ::core::ffi::c_int || isAgg != 0) {
        return 0 as ::core::ffi::c_int;
    }
    if !__p_ref.pOrderBy.is_null() && !(*pSub).pOrderBy.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if isAgg != 0 && !(*pSub).pOrderBy.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !(*pSub).pLimit.is_null() && !__p_ref.pWhere.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !(*pSub).pLimit.is_null()
        && __p_ref.selFlags
            & crate::src::headers::sqliteInt_h::SF_Distinct as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*pSub).selFlags & 0x2000 as crate::src::ext::rtree::rtree::U32_0 != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*pSubitem).fg.jointype as ::core::ffi::c_int
        & (crate::src::headers::sqliteInt_h::JT_OUTER | crate::src::headers::sqliteInt_h::JT_LTORJ)
        != 0 as ::core::ffi::c_int
    {
        if (*pSubSrc).nSrc > 1 as ::core::ffi::c_int
            || __p_ref.selFlags
                & crate::src::headers::sqliteInt_h::SF_Distinct
                    as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
            || (*pSubitem).fg.jointype as ::core::ffi::c_int
                & crate::src::headers::sqliteInt_h::JT_RIGHT
                != 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        isOuterJoin = 1 as ::core::ffi::c_int;
    }
    if iFrom > 0 as ::core::ffi::c_int
        && (*(&raw mut (*pSubSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
            .offset(0_isize))
        .fg
        .jointype as ::core::ffi::c_int
            & crate::src::headers::sqliteInt_h::JT_LTORJ
            != 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if !(*pSub).pPrior.is_null() {
        let mut ii: ::core::ffi::c_int;
        if !(*pSub).pOrderBy.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if isAgg != 0
            || __p_ref.selFlags
                & crate::src::headers::sqliteInt_h::SF_Distinct
                    as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
            || isOuterJoin > 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        pSub1 = pSub;
        while !pSub1.is_null() {
            if (*pSub1).selFlags
                & (crate::src::headers::sqliteInt_h::SF_Distinct
                    | crate::src::headers::sqliteInt_h::SF_Aggregate)
                    as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
                || !(*pSub1).pPrior.is_null()
                    && (*pSub1).op as ::core::ffi::c_int != crate::src::parse::TK_ALL
                || (*(*pSub1).pSrc).nSrc < 1 as ::core::ffi::c_int
                || !(*pSub1).pWin.is_null()
            {
                return 0 as ::core::ffi::c_int;
            }
            if iFrom > 0 as ::core::ffi::c_int
                && (*(&raw mut (*(*pSub1).pSrc).a
                    as *mut crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(0_isize))
                .fg
                .jointype as ::core::ffi::c_int
                    & crate::src::headers::sqliteInt_h::JT_LTORJ
                    != 0 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            pSub1 = (*pSub1).pPrior;
        }
        if !__p_ref.pOrderBy.is_null() {
            ii = 0 as ::core::ffi::c_int;
            while ii < (*__p_ref.pOrderBy).nExpr {
                if (*(&raw mut (*__p_ref.pOrderBy).a
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(ii as isize))
                .u
                .x
                .iOrderByCol as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                }
                ii += 1;
            }
        }
        if __p_ref.selFlags
            & crate::src::headers::sqliteInt_h::SF_Recursive as crate::src::ext::rtree::rtree::U32_0
            != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if compoundHasDifferentAffinities(pSub) != 0 {
            return 0 as ::core::ffi::c_int;
        }
        if (*pSrc).nSrc > 1 as ::core::ffi::c_int {
            if __pParse_ref.nSelect > 500 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            if (*db).dbOptFlags & 0x800000 as crate::src::ext::rtree::rtree::U32_0
                != 0 as crate::src::ext::rtree::rtree::U32_0
            {
                return 0 as ::core::ffi::c_int;
            }
            aCsrMap = crate::src::src::malloc::sqlite3DbMallocZero(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                ((__pParse_ref.nTab as crate::src::ext::rtree::rtree::I64_0
                    + 1 as crate::src::ext::rtree::rtree::I64_0)
                    as crate::src::ext::rtree::rtree::U64_0)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()
                        as crate::src::ext::rtree::rtree::U64_0),
            ) as *mut ::core::ffi::c_int;
            if !aCsrMap.is_null() {
                *aCsrMap.offset(0_isize) = __pParse_ref.nTab;
            }
        }
    }
    __pParse_ref.zAuthContext = (*pSubitem).zName;
    crate::src::src::auth::sqlite3AuthCheck(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        crate::src::headers::sqlite3_h::SQLITE_SELECT,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    __pParse_ref.zAuthContext = zSavedAuthContext;
    if (*pSubitem).fg.isSubquery() != 0 {
        pSub1 = crate::src::src::build::sqlite3SubqueryDetach(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pSubitem as *mut crate::src::headers::sqliteInt_h::SrcItem,
        ) as *mut crate::src::headers::sqliteInt_h::Select;
    } else {
        pSub1 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    }
    crate::src::src::malloc::sqlite3DbFree(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        (*pSubitem).zName as *mut ::core::ffi::c_void,
    );
    crate::src::src::malloc::sqlite3DbFree(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        (*pSubitem).zAlias as *mut ::core::ffi::c_void,
    );
    (*pSubitem).zName = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*pSubitem).zAlias = ::core::ptr::null_mut::<::core::ffi::c_char>();
    pSub = (*pSub).pPrior;
    while !pSub.is_null() {
        
        let pOrderBy: *mut crate::src::headers::sqliteInt_h::ExprList = __p_ref.pOrderBy;
        let pLimit: *mut crate::src::headers::sqliteInt_h::Expr = __p_ref.pLimit;
        let pPrior: *mut crate::src::headers::sqliteInt_h::Select = __p_ref.pPrior;
        let __pSubitem_ref = unsafe { &mut *pSubitem };
        let pItemTab: *mut crate::src::headers::sqliteInt_h::Table = __pSubitem_ref.pSTab;
        __pSubitem_ref.pSTab = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
        __p_ref.pOrderBy = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
        __p_ref.pPrior = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
        __p_ref.pLimit = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
        let pNew: *mut crate::src::headers::sqliteInt_h::Select = crate::src::src::expr::sqlite3SelectDup(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            p as *const crate::src::headers::sqliteInt_h::Select,
            0 as ::core::ffi::c_int,
        ) as *mut crate::src::headers::sqliteInt_h::Select;
        __p_ref.pLimit = pLimit;
        __p_ref.pOrderBy = pOrderBy;
        __p_ref.op = crate::src::parse::TK_ALL as u8;
        __pSubitem_ref.pSTab = pItemTab;
        if pNew.is_null() {
            __p_ref.pPrior = pPrior;
        } else {
            __pParse_ref.nSelect += 1;
            (*pNew).selId = __pParse_ref.nSelect as crate::src::ext::rtree::rtree::U32_0;
            if !aCsrMap.is_null()
                && (*db).mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                renumberCursors(pParse, pNew, iFrom, aCsrMap);
            }
            (*pNew).pPrior = pPrior;
            if !pPrior.is_null() {
                (*pPrior).pNext = pNew;
            }
            (*pNew).pNext = p;
            __p_ref.pPrior = pNew;
        }
        pSub = (*pSub).pPrior;
    }
    crate::src::src::malloc::sqlite3DbFree(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        aCsrMap as *mut ::core::ffi::c_void,
    );
    if (*db).mallocFailed != 0 {
        crate::src::src::build::sqlite3SrcItemAttachSubquery(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            pSubitem as *mut crate::src::headers::sqliteInt_h::SrcItem,
            pSub1 as *mut crate::src::headers::sqliteInt_h::Select,
            0 as ::core::ffi::c_int,
        );
        return 1 as ::core::ffi::c_int;
    }
    if !(*pSubitem).pSTab.is_null() {
        let pTabToDel: *mut crate::src::headers::sqliteInt_h::Table = (*pSubitem).pSTab;
        if (*pTabToDel).nTabRef == 1 as crate::src::ext::rtree::rtree::U32_0 {
            let pToplevel: *mut crate::src::headers::sqliteInt_h::Parse =
                if !__pParse_ref.pToplevel.is_null() {
                    __pParse_ref.pToplevel
                } else {
                    pParse
                };
            crate::src::src::prepare::sqlite3ParserAddCleanup(
                pToplevel as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::mem::transmute(Some(
                    crate::src::src::build::sqlite3DeleteTableGeneric
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqliteInt_h::sqlite3,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                )),
                pTabToDel as *mut ::core::ffi::c_void,
            );
        } else {
            (*pTabToDel).nTabRef = (*pTabToDel).nTabRef.wrapping_sub(1);
        }
        (*pSubitem).pSTab = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
    }
    pSub = pSub1;
    pParent = p;
    while !pParent.is_null() {
        
        let jointype: crate::src::ext::rtree::rtree::U8_0 = (*pSubitem).fg.jointype;
        pSubSrc = (*pSub).pSrc;
        let nSubSrc: ::core::ffi::c_int = (*pSubSrc).nSrc;
        pSrc = (*pParent).pSrc;
        if nSubSrc > 1 as ::core::ffi::c_int {
            pSrc = crate::src::src::build::sqlite3SrcListEnlarge(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                pSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
                nSubSrc - 1 as ::core::ffi::c_int,
                iFrom + 1 as ::core::ffi::c_int,
            ) as *mut crate::src::headers::sqliteInt_h::SrcList;
            if pSrc.is_null() {
                break;
            }
            (*pParent).pSrc = pSrc;
            pSubitem = (&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                .offset(iFrom as isize)
                as *mut crate::src::headers::sqliteInt_h::SrcItem;
        }
        iNewParent = (*(&raw mut (*pSubSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
            .offset(0_isize))
        .iCursor;
        i = 0 as ::core::ffi::c_int;
        while i < nSubSrc {
            let pItem: *mut crate::src::headers::sqliteInt_h::SrcItem = (&raw mut (*pSrc).a
                as *mut crate::src::headers::sqliteInt_h::SrcItem)
                .offset((i + iFrom) as isize)
                as *mut crate::src::headers::sqliteInt_h::SrcItem;
            let __pItem_ref = unsafe { &mut *pItem };
            if __pItem_ref.fg.isUsing() != 0 {
                crate::src::src::build::sqlite3IdListDelete(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    __pItem_ref.u3.pUsing as *mut crate::src::headers::sqliteInt_h::IdList,
                );
            }
            *pItem = *(&raw mut (*pSubSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                .offset(i as isize);
            __pItem_ref.fg.jointype = (__pItem_ref.fg.jointype as ::core::ffi::c_int
                | jointype as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::JT_LTORJ)
                as u8;
            ::libc::memset(
                (&raw mut (*pSubSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(i as isize)
                    as *mut crate::src::headers::sqliteInt_h::SrcItem
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<crate::src::headers::sqliteInt_h::SrcItem>()
                    as crate::__stddef_size_t_h::SizeT,
            );
            i += 1;
        }
        (*pSubitem).fg.jointype =
            ((*pSubitem).fg.jointype as ::core::ffi::c_int | jointype as ::core::ffi::c_int) as u8;
        if !(*pSub).pOrderBy.is_null()
            && (*pParent).selFlags
                & crate::src::headers::sqliteInt_h::SF_NoopOrderBy
                    as crate::src::ext::rtree::rtree::U32_0
                == 0 as crate::src::ext::rtree::rtree::U32_0
        {
            let pOrderBy_0: *mut crate::src::headers::sqliteInt_h::ExprList = (*pSub).pOrderBy;
            i = 0 as ::core::ffi::c_int;
            while i < (*pOrderBy_0).nExpr {
                (*(&raw mut (*pOrderBy_0).a
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(i as isize))
                .u
                .x
                .iOrderByCol = 0 as crate::src::fts5::U16_0;
                i += 1;
            }
            (*pParent).pOrderBy = pOrderBy_0;
            (*pSub).pOrderBy =
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
        }
        pWhere = (*pSub).pWhere;
        (*pSub).pWhere = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
        if isOuterJoin > 0 as ::core::ffi::c_int {
            sqlite3SetJoinExpr(
                pWhere,
                iNewParent,
                crate::src::headers::sqliteInt_h::EP_OuterON
                    as crate::src::ext::rtree::rtree::U32_0,
            );
        }
        if !pWhere.is_null() {
            if !(*pParent).pWhere.is_null() {
                (*pParent).pWhere = crate::src::src::expr::sqlite3PExpr(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    crate::src::parse::TK_AND,
                    pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
                    (*pParent).pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
                )
                    as *mut crate::src::headers::sqliteInt_h::Expr;
            } else {
                (*pParent).pWhere = pWhere;
            }
        }
        if (*db).mallocFailed as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut x: SubstContext = unsafe { ::core::mem::zeroed() };
            x.pParse = pParse;
            x.iTable = iParent;
            x.iNewTable = iNewParent;
            x.isOuterJoin = isOuterJoin;
            x.nSelDepth = 0 as ::core::ffi::c_int;
            x.pEList = (*pSub).pEList;
            x.pCList = findLeftmostExprlist(pSub);
            substSelect(&raw mut x, pParent, 0 as ::core::ffi::c_int);
        }
        (*pParent).selFlags |= (*pSub).selFlags
            & crate::src::headers::sqliteInt_h::SF_Compound as crate::src::ext::rtree::rtree::U32_0;
        if !(*pSub).pLimit.is_null() {
            (*pParent).pLimit = (*pSub).pLimit;
            (*pSub).pLimit = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
        }
        i = 0 as ::core::ffi::c_int;
        while i < nSubSrc {
            recomputeColumnsUsed(
                pParent,
                (&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                    .offset((i + iFrom) as isize)
                    as *mut crate::src::headers::sqliteInt_h::SrcItem,
            );
            i += 1;
        }
        pParent = (*pParent).pPrior;
        pSub = (*pSub).pPrior;
    }
    crate::src::src::expr::sqlite3AggInfoPersistWalkerInit(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
    );
    crate::src::src::walker::sqlite3WalkSelect(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        pSub1 as *mut crate::src::headers::sqliteInt_h::Select,
    );
    sqlite3SelectDelete(db, pSub1);
    1 as ::core::ffi::c_int
}

unsafe extern "C" fn constInsert(
    pConst: *mut WhereConst,
    pColumn: *mut crate::src::headers::sqliteInt_h::Expr,
    pValue: *mut crate::src::headers::sqliteInt_h::Expr,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) {
    let mut i: ::core::ffi::c_int;
    if (*pColumn).flags & 0x20 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return;
    }
    if crate::src::src::expr::sqlite3ExprAffinity(
        pValue as *const crate::src::headers::sqliteInt_h::Expr,
    ) as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        return;
    }
    let __pConst_ref = unsafe { &mut *pConst };
    if crate::src::src::main::sqlite3IsBinary(crate::src::src::expr::sqlite3ExprCompareCollSeq(
        __pConst_ref.pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        pExpr as *const crate::src::headers::sqliteInt_h::Expr,
    )
        as *mut crate::src::headers::sqliteInt_h::CollSeq
        as *const crate::src::headers::sqliteInt_h::CollSeq)
        == 0
    {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < __pConst_ref.nConst {
        let pE2: *const crate::src::headers::sqliteInt_h::Expr = *(*pConst)
            .apExpr
            .offset((i * 2 as ::core::ffi::c_int) as isize);
        if (*pE2).iTable == (*pColumn).iTable
            && (*pE2).iColumn as ::core::ffi::c_int == (*pColumn).iColumn as ::core::ffi::c_int
        {
            return;
        }
        i += 1;
    }
    if crate::src::src::expr::sqlite3ExprAffinity(
        pColumn as *const crate::src::headers::sqliteInt_h::Expr,
    ) as ::core::ffi::c_int
        <= crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB
    {
        __pConst_ref.bHasAffBlob = 1 as ::core::ffi::c_int;
    }
    __pConst_ref.nConst += 1;
    __pConst_ref.apExpr = crate::src::src::malloc::sqlite3DbReallocOrFree(
        (*__pConst_ref.pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __pConst_ref.apExpr as *mut ::core::ffi::c_void,
        ((__pConst_ref.nConst * 2 as ::core::ffi::c_int) as usize).wrapping_mul(
            ::core::mem::size_of::<*mut crate::src::headers::sqliteInt_h::Expr>() as usize,
        ) as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut *mut crate::src::headers::sqliteInt_h::Expr;
    if __pConst_ref.apExpr.is_null() {
        __pConst_ref.nConst = 0 as ::core::ffi::c_int;
    } else {
        let fresh19 = &mut *__pConst_ref.apExpr.offset(
            (__pConst_ref.nConst * 2 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize,
        );
        *fresh19 = pColumn;
        let fresh20 = &mut *__pConst_ref.apExpr.offset(
            (__pConst_ref.nConst * 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
        );
        *fresh20 = pValue;
    };
}

unsafe extern "C" fn findConstInWhere(
    pConst: *mut WhereConst,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) {
    
    
    if pExpr.is_null() {
        return;
    }
    let __pConst_ref = unsafe { &*pConst };
    let __pExpr_ref = unsafe { &*pExpr };
    if __pExpr_ref.flags & __pConst_ref.mExcludeOn != 0 as crate::src::ext::rtree::rtree::U32_0 {
        return;
    }
    if __pExpr_ref.op as ::core::ffi::c_int == crate::src::parse::TK_AND {
        findConstInWhere(pConst, __pExpr_ref.pRight);
        findConstInWhere(pConst, __pExpr_ref.pLeft);
        return;
    }
    if __pExpr_ref.op as ::core::ffi::c_int != crate::src::parse::TK_EQ {
        return;
    }
    let pRight: *mut crate::src::headers::sqliteInt_h::Expr = __pExpr_ref.pRight;
    let pLeft: *mut crate::src::headers::sqliteInt_h::Expr = __pExpr_ref.pLeft;
    if (*pRight).op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN_1
        && crate::src::src::expr::sqlite3ExprIsConstant(
            __pConst_ref.pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            pLeft as *mut crate::src::headers::sqliteInt_h::Expr,
        ) != 0
    {
        constInsert(pConst, pRight, pLeft, pExpr);
    }
    if (*pLeft).op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN_1
        && crate::src::src::expr::sqlite3ExprIsConstant(
            __pConst_ref.pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            pRight as *mut crate::src::headers::sqliteInt_h::Expr,
        ) != 0
    {
        constInsert(pConst, pLeft, pRight, pExpr);
    }
}

unsafe extern "C" fn propagateConstantExprRewriteOne(
    pConst: *mut WhereConst,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
    bIgnoreAffBlob: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let __pConst_ref = unsafe { &mut *pConst };
    if *__pConst_ref.pOomFault.offset(0_isize) != 0 {
        return crate::src::headers::sqliteInt_h::WRC_Prune;
    }
    if (*pExpr).op as ::core::ffi::c_int != crate::src::parse::TK_COLUMN_1 {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    if (*pExpr).flags & (0x20 as crate::src::ext::rtree::rtree::U32_0 | __pConst_ref.mExcludeOn)
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    i = 0 as ::core::ffi::c_int;
    while i < __pConst_ref.nConst {
        let pColumn: *mut crate::src::headers::sqliteInt_h::Expr = *(*pConst)
            .apExpr
            .offset((i * 2 as ::core::ffi::c_int) as isize);
        if (pColumn != pExpr)
            && ((*pColumn).iTable == (*pExpr).iTable)
                && ((*pColumn).iColumn as ::core::ffi::c_int == (*pExpr).iColumn as ::core::ffi::c_int)
                {
                    if bIgnoreAffBlob != 0
                        && crate::src::src::expr::sqlite3ExprAffinity(
                            pColumn as *const crate::src::headers::sqliteInt_h::Expr,
                        ) as ::core::ffi::c_int
                            <= crate::src::headers::sqliteInt_h::SQLITE_AFF_BLOB
                    {
                        break;
                    }
                    __pConst_ref.nChng += 1;
                    let __pExpr_ref = unsafe { &mut *pExpr };
                    __pExpr_ref.flags &=
                        !(0x800000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0);
                    __pExpr_ref.flags |=
                        0x20 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
                    __pExpr_ref.pLeft = crate::src::src::expr::sqlite3ExprDup(
                        (*__pConst_ref.pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        *__pConst_ref.apExpr.offset(
                            (i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                        ) as *const crate::src::headers::sqliteInt_h::Expr,
                        0 as ::core::ffi::c_int,
                    )
                        as *mut crate::src::headers::sqliteInt_h::Expr;
                    if (*(*__pConst_ref.pParse).db).mallocFailed != 0 {
                        return crate::src::headers::sqliteInt_h::WRC_Prune;
                    }
                    break;
                }
        i += 1;
    }
    crate::src::headers::sqliteInt_h::WRC_Prune
}

unsafe extern "C" fn propagateConstantExprRewrite(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let pConst: *mut WhereConst = (*pWalker).u.pConst as *mut WhereConst;
    if (*pConst).bHasAffBlob != 0 {
        let __pExpr_ref = unsafe { &mut *pExpr };
        if __pExpr_ref.op as ::core::ffi::c_int >= crate::src::parse::TK_EQ
            && __pExpr_ref.op as ::core::ffi::c_int <= crate::src::parse::TK_GE
            || __pExpr_ref.op as ::core::ffi::c_int == crate::src::parse::TK_IS
        {
            propagateConstantExprRewriteOne(pConst, __pExpr_ref.pLeft, 0 as ::core::ffi::c_int);
            if *(*pConst).pOomFault.offset(0_isize) != 0 {
                return crate::src::headers::sqliteInt_h::WRC_Prune;
            }
            if crate::src::src::expr::sqlite3ExprAffinity(
                __pExpr_ref.pLeft as *const crate::src::headers::sqliteInt_h::Expr,
            ) as ::core::ffi::c_int
                != crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT
            {
                propagateConstantExprRewriteOne(
                    pConst,
                    __pExpr_ref.pRight,
                    0 as ::core::ffi::c_int,
                );
            }
        }
    }
    propagateConstantExprRewriteOne(pConst, pExpr, (*pConst).bHasAffBlob)
}

unsafe extern "C" fn propagateConstants(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    let mut x: WhereConst = unsafe { ::core::mem::zeroed() };
    let mut w: crate::src::headers::sqliteInt_h::Walker = unsafe { ::core::mem::zeroed() };
    let mut nChng: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    x.pParse = pParse;
    x.pOomFault = &raw mut (*(*pParse).db).mallocFailed;
    loop {
        x.nConst = 0 as ::core::ffi::c_int;
        x.nChng = 0 as ::core::ffi::c_int;
        x.apExpr = ::core::ptr::null_mut::<*mut crate::src::headers::sqliteInt_h::Expr>();
        x.bHasAffBlob = 0 as ::core::ffi::c_int;
        let __p_ref = unsafe { &mut *p };
        if !__p_ref.pSrc.is_null()
            && (*__p_ref.pSrc).nSrc > 0 as ::core::ffi::c_int
            && (*(&raw mut (*__p_ref.pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                .offset(0_isize))
            .fg
            .jointype as ::core::ffi::c_int
                & crate::src::headers::sqliteInt_h::JT_LTORJ
                != 0 as ::core::ffi::c_int
        {
            x.mExcludeOn = (crate::src::headers::sqliteInt_h::EP_InnerON
                | crate::src::headers::sqliteInt_h::EP_OuterON)
                as crate::src::ext::rtree::rtree::U32_0;
        } else {
            x.mExcludeOn = crate::src::headers::sqliteInt_h::EP_OuterON
                as crate::src::ext::rtree::rtree::U32_0;
        }
        findConstInWhere(&raw mut x, __p_ref.pWhere);
        if x.nConst != 0 {
            w.pParse = pParse;
            w.xExprCallback = Some(
                propagateConstantExprRewrite
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
            w.xSelectCallback2 = None;
            w.walkerDepth = 0 as ::core::ffi::c_int;
            w.u.pConst = &raw mut x as *mut WhereConst;
            crate::src::src::walker::sqlite3WalkExpr(
                &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
                __p_ref.pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
            );
            crate::src::src::malloc::sqlite3DbFree(
                (*x.pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                x.apExpr as *mut ::core::ffi::c_void,
            );
            nChng += x.nChng;
        }
        if (x.nChng == 0) {
            break;
        }
    }
    nChng
}

unsafe extern "C" fn pushDownWindowCheck(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pSubq: *mut crate::src::headers::sqliteInt_h::Select,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    crate::src::src::expr::sqlite3ExprIsConstantOrGroupBy(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
        (*(*pSubq).pWin).pPartition as *mut crate::src::headers::sqliteInt_h::ExprList,
    )
}

unsafe extern "C" fn pushDownWhereTerms(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut pSubq: *mut crate::src::headers::sqliteInt_h::Select,
    mut pWhere: *mut crate::src::headers::sqliteInt_h::Expr,
    pSrcList: *mut crate::src::headers::sqliteInt_h::SrcList,
    iSrc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pNew: *mut crate::src::headers::sqliteInt_h::Expr;
    
    let mut nChng: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let pSrc: *mut crate::src::headers::sqliteInt_h::SrcItem = (&raw mut (*pSrcList).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
        .offset(iSrc as isize) as *mut crate::src::headers::sqliteInt_h::SrcItem;
    if pWhere.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*pSubq).selFlags
        & (crate::src::headers::sqliteInt_h::SF_Recursive
            | crate::src::headers::sqliteInt_h::SF_MultiPart)
            as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*pSrc).fg.jointype as ::core::ffi::c_int
        & (crate::src::headers::sqliteInt_h::JT_LTORJ | crate::src::headers::sqliteInt_h::JT_RIGHT)
        != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !(*pSubq).pPrior.is_null() {
        let mut pSel: *mut crate::src::headers::sqliteInt_h::Select;
        let mut notUnionAll: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        pSel = pSubq;
        while !pSel.is_null() {
            let op: crate::src::ext::rtree::rtree::U8_0 = (*pSel).op;
            if op as ::core::ffi::c_int != crate::src::parse::TK_ALL
                && op as ::core::ffi::c_int != crate::src::parse::TK_SELECT_1
            {
                notUnionAll = 1 as ::core::ffi::c_int;
            }
            if !(*pSel).pWin.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            pSel = (*pSel).pPrior;
        }
        if notUnionAll != 0 {
            pSel = pSubq;
            while !pSel.is_null() {
                let mut ii: ::core::ffi::c_int;
                let pList: *const crate::src::headers::sqliteInt_h::ExprList = (*pSel).pEList;
                ii = 0 as ::core::ffi::c_int;
                while ii < (*pList).nExpr {
                    let pColl: *mut crate::src::headers::sqliteInt_h::CollSeq =
                        crate::src::src::expr::sqlite3ExprCollSeq(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            (*(&raw const (*pList).a
                                as *const crate::src::headers::sqliteInt_h::ExprList_item)
                                .offset(ii as isize))
                            .pExpr
                                as *const crate::src::headers::sqliteInt_h::Expr,
                        ) as *mut crate::src::headers::sqliteInt_h::CollSeq;
                    if crate::src::src::main::sqlite3IsBinary(
                        pColl as *const crate::src::headers::sqliteInt_h::CollSeq,
                    ) == 0
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                    ii += 1;
                }
                pSel = (*pSel).pPrior;
            }
        }
    } else if !(*pSubq).pWin.is_null() && (*(*pSubq).pWin).pPartition.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !(*pSubq).pLimit.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    while (*pWhere).op as ::core::ffi::c_int == crate::src::parse::TK_AND {
        nChng += pushDownWhereTerms(pParse, pSubq, (*pWhere).pRight, pSrcList, iSrc);
        pWhere = (*pWhere).pLeft;
    }
    if crate::src::src::expr::sqlite3ExprIsSingleTableConstraint(
        pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
        pSrcList as *const crate::src::headers::sqliteInt_h::SrcList,
        iSrc,
        1 as ::core::ffi::c_int,
    ) != 0
    {
        nChng += 1;
        (*pSubq).selFlags |=
            crate::src::headers::sqliteInt_h::SF_PushDown as crate::src::ext::rtree::rtree::U32_0;
        while !pSubq.is_null() {
            let mut x: SubstContext = unsafe { ::core::mem::zeroed() };
            pNew = crate::src::src::expr::sqlite3ExprDup(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                pWhere as *const crate::src::headers::sqliteInt_h::Expr,
                0 as ::core::ffi::c_int,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            unsetJoinExpr(pNew, -(1 as ::core::ffi::c_int), 1 as ::core::ffi::c_int);
            x.pParse = pParse;
            x.iTable = (*pSrc).iCursor;
            x.iNewTable = (*pSrc).iCursor;
            x.isOuterJoin = 0 as ::core::ffi::c_int;
            x.nSelDepth = 0 as ::core::ffi::c_int;
            x.pEList = (*pSubq).pEList;
            x.pCList = findLeftmostExprlist(pSubq);
            pNew = substExpr(&raw mut x, pNew);
            if !(*pSubq).pWin.is_null()
                && 0 as ::core::ffi::c_int == pushDownWindowCheck(pParse, pSubq, pNew)
            {
                crate::src::src::expr::sqlite3ExprDelete(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    pNew as *mut crate::src::headers::sqliteInt_h::Expr,
                );
                nChng -= 1;
                break;
            } else {
                if (*pSubq).selFlags
                    & crate::src::headers::sqliteInt_h::SF_Aggregate
                        as crate::src::ext::rtree::rtree::U32_0
                    != 0
                {
                    (*pSubq).pHaving = crate::src::src::expr::sqlite3ExprAnd(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        (*pSubq).pHaving as *mut crate::src::headers::sqliteInt_h::Expr,
                        pNew as *mut crate::src::headers::sqliteInt_h::Expr,
                    )
                        as *mut crate::src::headers::sqliteInt_h::Expr;
                } else {
                    (*pSubq).pWhere = crate::src::src::expr::sqlite3ExprAnd(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        (*pSubq).pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
                        pNew as *mut crate::src::headers::sqliteInt_h::Expr,
                    )
                        as *mut crate::src::headers::sqliteInt_h::Expr;
                }
                pSubq = (*pSubq).pPrior;
            }
        }
    }
    nChng
}

unsafe extern "C" fn disableUnusedSubqueryResultColumns(
    pItem: *mut crate::src::headers::sqliteInt_h::SrcItem,
) -> ::core::ffi::c_int {
    
    
    let mut pX: *mut crate::src::headers::sqliteInt_h::Select;
    
    let mut j: ::core::ffi::c_int;
    let mut nChng: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut colUsed: crate::src::headers::sqliteInt_h::Bitmask;
    let __pItem_ref = unsafe { &mut *pItem };
    if __pItem_ref.fg.isCorrelated() as ::core::ffi::c_int != 0
        || __pItem_ref.fg.isCte() as ::core::ffi::c_int != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = __pItem_ref.pSTab;
    let pSub: *mut crate::src::headers::sqliteInt_h::Select = (*__pItem_ref.u4.pSubq).pSelect;
    pX = pSub;
    while !pX.is_null() {
        if (*pX).selFlags
            & (crate::src::headers::sqliteInt_h::SF_Distinct
                | crate::src::headers::sqliteInt_h::SF_Aggregate)
                as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
        {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pX).pPrior.is_null() && (*pX).op as ::core::ffi::c_int != crate::src::parse::TK_ALL {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pX).pWin.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        pX = (*pX).pPrior;
    }
    colUsed = __pItem_ref.colUsed;
    if !(*pSub).pOrderBy.is_null() {
        let pList: *mut crate::src::headers::sqliteInt_h::ExprList = (*pSub).pOrderBy;
        j = 0 as ::core::ffi::c_int;
        while j < (*pList).nExpr {
            let mut iCol: crate::src::fts5::U16_0 = (*(&raw mut (*pList).a
                as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(j as isize))
            .u
            .x
            .iOrderByCol;
            if iCol as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                iCol = iCol.wrapping_sub(1);
                colUsed |= (1 as ::core::ffi::c_int as crate::src::headers::sqliteInt_h::Bitmask)
                    << (if iCol as ::core::ffi::c_int >= crate::src::headers::sqliteInt_h::BMS {
                        crate::src::headers::sqliteInt_h::BMS - 1 as ::core::ffi::c_int
                    } else {
                        iCol as ::core::ffi::c_int
                    });
            }
            j += 1;
        }
    }
    let nCol: ::core::ffi::c_int = (*pTab).nCol as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < nCol {
        let m: crate::src::headers::sqliteInt_h::Bitmask =
            if j < crate::src::headers::sqliteInt_h::BMS - 1 as ::core::ffi::c_int {
                (1 as ::core::ffi::c_int as crate::src::headers::sqliteInt_h::Bitmask) << j
            } else {
                crate::src::headers::sqliteInt_h::TOPBIT
            };
        if (m & colUsed == 0 as crate::src::headers::sqliteInt_h::Bitmask) {
            pX = pSub;
            while !pX.is_null() {
                let pY: *mut crate::src::headers::sqliteInt_h::Expr =
                    (*(&raw mut (*(*pX).pEList).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(j as isize))
                    .pExpr;
                if ((*pY).op as ::core::ffi::c_int != crate::src::parse::TK_NULL) {
                    (*pY).op = crate::src::parse::TK_NULL as u8;
                    (*pY).flags &= !((0x2000 as ::core::ffi::c_int | 0x80000 as ::core::ffi::c_int)
                        as crate::src::ext::rtree::rtree::U32_0);
                    (*pX).selFlags |= crate::src::headers::sqliteInt_h::SF_PushDown
                        as crate::src::ext::rtree::rtree::U32_0;
                    nChng += 1;
                }
                pX = (*pX).pPrior;
            }
        }
        j += 1;
    }
    nChng
}

unsafe extern "C" fn minMaxQuery(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pFunc: *mut crate::src::headers::sqliteInt_h::Expr,
    ppMinMax: *mut *mut crate::src::headers::sqliteInt_h::ExprList,
) -> crate::src::ext::rtree::rtree::U8_0 {
    let mut eRet: ::core::ffi::c_int = crate::src::headers::sqliteInt_h::WHERE_ORDERBY_NORMAL;
    
    
    
    let mut sortFlags: crate::src::ext::rtree::rtree::U8_0 = 0_u8;
    let __pFunc_ref = unsafe { &*pFunc };
    let pEList: *mut crate::src::headers::sqliteInt_h::ExprList = __pFunc_ref.x.pList;
    if pEList.is_null()
        || (*pEList).nExpr != 1 as ::core::ffi::c_int
        || __pFunc_ref.flags
            & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
        || (*db).dbOptFlags & 0x10000 as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return eRet as u8;
    }
    let zFunc: *const ::core::ffi::c_char = __pFunc_ref.u.zToken;
    if crate::src::src::util::sqlite3StrICmp(
        zFunc,
        b"min\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        eRet = crate::src::headers::sqliteInt_h::WHERE_ORDERBY_MIN;
        if crate::src::src::expr::sqlite3ExprCanBeNull(
            (*(&raw mut (*pEList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                .offset(0_isize))
            .pExpr as *const crate::src::headers::sqliteInt_h::Expr,
        ) != 0
        {
            sortFlags = crate::src::headers::sqliteInt_h::KEYINFO_ORDER_BIGNULL as u8;
        }
    } else if crate::src::src::util::sqlite3StrICmp(
        zFunc,
        b"max\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        eRet = crate::src::headers::sqliteInt_h::WHERE_ORDERBY_MAX;
        sortFlags = crate::src::headers::sqliteInt_h::KEYINFO_ORDER_DESC as u8;
    } else {
        return eRet as u8;
    }
    let pOrderBy: *mut crate::src::headers::sqliteInt_h::ExprList = crate::src::src::expr::sqlite3ExprListDup(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        pEList as *const crate::src::headers::sqliteInt_h::ExprList,
        0 as ::core::ffi::c_int,
    ) as *mut crate::src::headers::sqliteInt_h::ExprList;
    *ppMinMax = pOrderBy;
    if !pOrderBy.is_null() {
        (*(&raw mut (*pOrderBy).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(0_isize))
        .fg
        .sortFlags = sortFlags;
    }
    eRet as u8
}

unsafe extern "C" fn isSimpleCount(
    p: *mut crate::src::headers::sqliteInt_h::Select,
    pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo,
) -> *mut crate::src::headers::sqliteInt_h::Table {
    
    
    let __p_ref = unsafe { &mut *p };
    if !__p_ref.pWhere.is_null()
        || (*__p_ref.pEList).nExpr != 1 as ::core::ffi::c_int
        || (*__p_ref.pSrc).nSrc != 1 as ::core::ffi::c_int
        || (*(&raw mut (*__p_ref.pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
            .offset(0_isize))
        .fg
        .isSubquery() as ::core::ffi::c_int
            != 0
        || (*pAggInfo).nFunc != 1 as ::core::ffi::c_int
        || !__p_ref.pHaving.is_null()
    {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
    }
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = (*(&raw mut (*__p_ref.pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
        .offset(0_isize))
    .pSTab;
    if ((*pTab).eTabType as ::core::ffi::c_int != crate::src::headers::sqliteInt_h::TABTYP_NORM) {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
    }
    let pExpr: *mut crate::src::headers::sqliteInt_h::Expr = (*(&raw mut (*__p_ref.pEList).a
        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
        .offset(0_isize))
    .pExpr;
    if (*pExpr).op as ::core::ffi::c_int != crate::src::parse::TK_AGG_FUNCTION {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
    }
    if (*pExpr).pAggInfo != pAggInfo {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
    }
    if (*(*(*pAggInfo).aFunc.offset(0_isize)).pFunc).funcFlags
        & crate::src::headers::sqliteInt_h::SQLITE_FUNC_COUNT
            as crate::src::ext::rtree::rtree::U32_0
        == 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
    }
    if (*pExpr).flags
        & (0x4 as ::core::ffi::c_int | 0x1000000 as ::core::ffi::c_int)
            as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
    }
    pTab
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3IndexedByLookup(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pFrom: *mut crate::src::headers::sqliteInt_h::SrcItem,
) -> ::core::ffi::c_int {
    let __pFrom_ref = unsafe { &mut *pFrom };
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = __pFrom_ref.pSTab;
    let zIndexedBy: *mut ::core::ffi::c_char = __pFrom_ref.u1.zIndexedBy;
    let mut pIdx: *mut crate::src::headers::sqliteInt_h::Index;
    pIdx = (*pTab).pIndex;
    while !pIdx.is_null() && crate::src::src::util::sqlite3StrICmp((*pIdx).zName, zIndexedBy) != 0 {
        pIdx = (*pIdx).pNext;
    }
    if pIdx.is_null() {
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"no such index: %s\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Str(
                zIndexedBy as *mut ::core::ffi::c_char,
            )],
        );
        (*pParse).set_checkSchema(
            1 as crate::src::headers::sqliteInt_h::Bft as crate::src::headers::sqliteInt_h::Bft,
        );
        return crate::src::headers::sqlite3_h::SQLITE_ERROR;
    }
    __pFrom_ref.u2.pIBIndex = pIdx;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn convertCompoundSelectToSubquery(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    p: *mut crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    
    let mut pX: *mut crate::src::headers::sqliteInt_h::Select;
    
    
    
    
    let mut dummy: crate::src::headers::sqliteInt_h::Token = unsafe { ::core::mem::zeroed() };
    let __p_ref = unsafe { &mut *p };
    if __p_ref.pPrior.is_null() {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    if __p_ref.pOrderBy.is_null() {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    pX = p;
    while !pX.is_null()
        && ((*pX).op as ::core::ffi::c_int == crate::src::parse::TK_ALL
            || (*pX).op as ::core::ffi::c_int == crate::src::parse::TK_SELECT_1)
    {
        pX = (*pX).pPrior;
    }
    if pX.is_null() {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    let a: *mut crate::src::headers::sqliteInt_h::ExprList_item = &raw mut (*__p_ref.pOrderBy).a as *mut crate::src::headers::sqliteInt_h::ExprList_item
        as *mut crate::src::headers::sqliteInt_h::ExprList_item;
    if (*a.offset(0_isize)).u.x.iOrderByCol != 0 {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    i = (*__p_ref.pOrderBy).nExpr - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        if (*(*a.offset(i as isize)).pExpr).flags
            & crate::src::headers::sqliteInt_h::EP_Collate as crate::src::ext::rtree::rtree::U32_0
            != 0
        {
            break;
        }
        i -= 1;
    }
    if i < 0 as ::core::ffi::c_int {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    let pParse: *mut crate::src::headers::sqliteInt_h::Parse = (*pWalker).pParse;
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    let pNew: *mut crate::src::headers::sqliteInt_h::Select = crate::src::src::malloc::sqlite3DbMallocZero(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Select>()
            as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::headers::sqliteInt_h::Select;
    if pNew.is_null() {
        return crate::src::headers::sqliteInt_h::WRC_Abort;
    }
    let pNewSrc: *mut crate::src::headers::sqliteInt_h::SrcList = crate::src::src::build::sqlite3SrcListAppendFromTerm(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>()
            as *mut crate::src::headers::sqliteInt_h::SrcList,
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
            as *mut crate::src::headers::sqliteInt_h::Token,
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
            as *mut crate::src::headers::sqliteInt_h::Token,
        &raw mut dummy as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
        pNew as *mut crate::src::headers::sqliteInt_h::Select,
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::OnOrUsing>()
            as *mut crate::src::headers::sqliteInt_h::OnOrUsing,
    ) as *mut crate::src::headers::sqliteInt_h::SrcList;
    if (*pParse).nErr != 0 {
        crate::src::src::build::sqlite3SrcListDelete(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pNewSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
        );
        return crate::src::headers::sqliteInt_h::WRC_Abort;
    }
    *pNew = *p;
    __p_ref.pSrc = pNewSrc;
    __p_ref.pEList = crate::src::src::expr::sqlite3ExprListAppend(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
            as *mut crate::src::headers::sqliteInt_h::ExprList,
        crate::src::src::expr::sqlite3Expr(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            crate::src::parse::TK_ASTERISK,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ) as *mut crate::src::headers::sqliteInt_h::Expr
            as *mut crate::src::headers::sqliteInt_h::Expr,
    ) as *mut crate::src::headers::sqliteInt_h::ExprList;
    __p_ref.op = crate::src::parse::TK_SELECT_1 as u8;
    __p_ref.pWhere = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
    (*pNew).pGroupBy = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
    (*pNew).pHaving = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
    (*pNew).pOrderBy = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
    __p_ref.pPrior = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    __p_ref.pNext = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
    __p_ref.pWith = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::With>();
    __p_ref.pWinDefn = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Window>();
    __p_ref.selFlags &=
        !(crate::src::headers::sqliteInt_h::SF_Compound as crate::src::ext::rtree::rtree::U32_0);
    __p_ref.selFlags |=
        crate::src::headers::sqliteInt_h::SF_Converted as crate::src::ext::rtree::rtree::U32_0;
    (*(*pNew).pPrior).pNext = pNew;
    (*pNew).pLimit = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
    crate::src::headers::sqliteInt_h::WRC_Continue
}

unsafe extern "C" fn cannotBeFunction(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pFrom: *mut crate::src::headers::sqliteInt_h::SrcItem,
) -> ::core::ffi::c_int {
    if (*pFrom).fg.isTabFunc() != 0 {
        crate::src::src::util::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"'%s' is not a function\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Str(
                (*pFrom).zName as *mut ::core::ffi::c_char,
            )],
        );
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn searchWith(
    pWith: *mut crate::src::headers::sqliteInt_h::With,
    pItem: *mut crate::src::headers::sqliteInt_h::SrcItem,
    ppContext: *mut *mut crate::src::headers::sqliteInt_h::With,
) -> *mut crate::src::headers::sqliteInt_h::Cte {
    let zName: *const ::core::ffi::c_char = (*pItem).zName;
    let mut p: *mut crate::src::headers::sqliteInt_h::With;
    p = pWith;
    while !p.is_null() {
        let mut i: ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).nCte {
            if crate::src::src::util::sqlite3StrICmp(
                zName,
                (*(&raw mut (*p).a as *mut crate::src::headers::sqliteInt_h::Cte)
                    .offset(i as isize))
                .zName,
            ) == 0 as ::core::ffi::c_int
            {
                *ppContext = p;
                return (&raw mut (*p).a as *mut crate::src::headers::sqliteInt_h::Cte)
                    .offset(i as isize)
                    as *mut crate::src::headers::sqliteInt_h::Cte;
            }
            i += 1;
        }
        if (*p).bView != 0 {
            break;
        }
        p = (*p).pOuter;
    }
    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Cte>()
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3WithPush(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut pWith: *mut crate::src::headers::sqliteInt_h::With,
    bFree: crate::src::ext::rtree::rtree::U8_0,
) -> *mut crate::src::headers::sqliteInt_h::With {
    if !pWith.is_null() {
        if bFree != 0 {
            pWith = crate::src::src::prepare::sqlite3ParserAddCleanup(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::mem::transmute(Some(
                    crate::src::src::build::sqlite3WithDeleteGeneric
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqliteInt_h::sqlite3,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                )),
                pWith as *mut ::core::ffi::c_void,
            ) as *mut crate::src::headers::sqliteInt_h::With;
            if pWith.is_null() {
                return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::With>();
            }
        }
        if (*pParse).nErr == 0 as ::core::ffi::c_int {
            (*pWith).pOuter = (*pParse).pWith;
            (*pParse).pWith = pWith;
        }
    }
    pWith
}

unsafe extern "C" fn resolveFromTermToCte(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pFrom: *mut crate::src::headers::sqliteInt_h::SrcItem,
) -> ::core::ffi::c_int {
    
    let mut pWith: *mut crate::src::headers::sqliteInt_h::With =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::With>();
    let __pParse_ref = unsafe { &mut *pParse };
    if __pParse_ref.pWith.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if __pParse_ref.nErr != 0 {
        return 0 as ::core::ffi::c_int;
    }
    let __pFrom_ref = unsafe { &mut *pFrom };
    if __pFrom_ref.fg.fixedSchema() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && !__pFrom_ref.u4.zDatabase.is_null()
    {
        return 0 as ::core::ffi::c_int;
    }
    if __pFrom_ref.fg.notCte() != 0 {
        return 0 as ::core::ffi::c_int;
    }
    let pCte: *mut crate::src::headers::sqliteInt_h::Cte = searchWith(__pParse_ref.pWith, pFrom, &raw mut pWith)
        as *mut crate::src::headers::sqliteInt_h::Cte;
    if !pCte.is_null() {
        let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
        
        let mut pEList: *mut crate::src::headers::sqliteInt_h::ExprList;
        
        let mut pLeft: *mut crate::src::headers::sqliteInt_h::Select;
        let mut pRecTerm: *mut crate::src::headers::sqliteInt_h::Select;
        
        
        let mut iRecTab: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut pCteUse: *mut crate::src::headers::sqliteInt_h::CteUse;
        let __pCte_ref = unsafe { &mut *pCte };
        if !__pCte_ref.zCteErr.is_null() {
            crate::src::src::util::sqlite3ErrorMsg_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                __pCte_ref.zCteErr,
                &[crate::src::src::printf::PrintfArg::Str(
                    __pCte_ref.zName as *mut ::core::ffi::c_char,
                )],
            );
            return 2 as ::core::ffi::c_int;
        }
        if cannotBeFunction(pParse, pFrom) != 0 {
            return 2 as ::core::ffi::c_int;
        }
        let pTab: *mut crate::src::headers::sqliteInt_h::Table = crate::src::src::malloc::sqlite3DbMallocZero(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Table>()
                as crate::src::ext::rtree::rtree::U64_0,
        ) as *mut crate::src::headers::sqliteInt_h::Table;
        if pTab.is_null() {
            return 2 as ::core::ffi::c_int;
        }
        pCteUse = __pCte_ref.pUse;
        if pCteUse.is_null() {
            pCteUse = crate::src::src::malloc::sqlite3DbMallocZero(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                ::core::mem::size_of::<crate::src::headers::sqliteInt_h::CteUse>()
                    as crate::src::ext::rtree::rtree::U64_0,
            ) as *mut crate::src::headers::sqliteInt_h::CteUse;
            __pCte_ref.pUse = pCteUse;
            if pCteUse.is_null()
                || crate::src::src::prepare::sqlite3ParserAddCleanup(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    ::core::mem::transmute(Some(
                        crate::src::src::malloc::sqlite3DbFree
                            as unsafe extern "C" fn(
                                *mut crate::src::headers::sqliteInt_h::sqlite3,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    )),
                    pCteUse as *mut ::core::ffi::c_void,
                )
                .is_null()
            {
                crate::src::src::malloc::sqlite3DbFree(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    pTab as *mut ::core::ffi::c_void,
                );
                return 2 as ::core::ffi::c_int;
            }
            (*pCteUse).eM10d = __pCte_ref.eM10d;
        }
        __pFrom_ref.pSTab = pTab;
        (*pTab).nTabRef = 1 as crate::src::ext::rtree::rtree::U32_0;
        (*pTab).zName = crate::src::src::malloc::sqlite3DbStrDup(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            __pCte_ref.zName,
        );
        (*pTab).iPKey = -(1 as ::core::ffi::c_int) as crate::src::fts5::I16_0;
        (*pTab).nRowLogEst = 200 as crate::src::headers::sqliteInt_h::LogEst;
        (*pTab).tabFlags |= (crate::src::headers::sqliteInt_h::TF_Ephemeral
            | crate::src::headers::sqliteInt_h::TF_NoVisibleRowid)
            as crate::src::ext::rtree::rtree::U32_0;
        crate::src::src::build::sqlite3SrcItemAttachSubquery(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            pFrom as *mut crate::src::headers::sqliteInt_h::SrcItem,
            __pCte_ref.pSelect as *mut crate::src::headers::sqliteInt_h::Select,
            1 as ::core::ffi::c_int,
        );
        if (*db).mallocFailed != 0 {
            return 2 as ::core::ffi::c_int;
        }
        let pSel: *mut crate::src::headers::sqliteInt_h::Select = (*__pFrom_ref.u4.pSubq).pSelect;
        (*pSel).selFlags |=
            crate::src::headers::sqliteInt_h::SF_CopyCte as crate::src::ext::rtree::rtree::U32_0;
        if __pFrom_ref.fg.isIndexedBy() != 0 {
            crate::src::src::util::sqlite3ErrorMsg_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                b"no such index: \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Str(
                    __pFrom_ref.u1.zIndexedBy as *mut ::core::ffi::c_char,
                )],
            );
            return 2 as ::core::ffi::c_int;
        }
        (*pFrom)
            .fg
            .set_isCte(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        __pFrom_ref.u2.pCteUse = pCteUse;
        (*pCteUse).nUse += 1;
        pRecTerm = pSel;
        let bMayRecursive: ::core::ffi::c_int = ((*pSel).op as ::core::ffi::c_int == crate::src::parse::TK_ALL
            || (*pSel).op as ::core::ffi::c_int == crate::src::parse::TK_UNION)
            as ::core::ffi::c_int;
        while bMayRecursive != 0
            && (*pRecTerm).op as ::core::ffi::c_int == (*pSel).op as ::core::ffi::c_int
        {
            let mut i: ::core::ffi::c_int;
            let pSrc: *mut crate::src::headers::sqliteInt_h::SrcList = (*pRecTerm).pSrc;
            i = 0 as ::core::ffi::c_int;
            while i < (*pSrc).nSrc {
                let pItem: *mut crate::src::headers::sqliteInt_h::SrcItem = (&raw mut (*pSrc).a
                    as *mut crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(i as isize)
                    as *mut crate::src::headers::sqliteInt_h::SrcItem;
                let __pItem_ref = unsafe { &mut *pItem };
                if !__pItem_ref.zName.is_null()
                    && __pItem_ref.fg.hadSchema() == 0
                    && __pItem_ref.fg.isSubquery() == 0
                    && (__pItem_ref.fg.fixedSchema() as ::core::ffi::c_int != 0
                        || __pItem_ref.u4.zDatabase.is_null())
                    && 0 as ::core::ffi::c_int
                        == crate::src::src::util::sqlite3StrICmp(
                            __pItem_ref.zName,
                            __pCte_ref.zName,
                        )
                {
                    __pItem_ref.pSTab = pTab;
                    (*pTab).nTabRef = (*pTab).nTabRef.wrapping_add(1);
                    (*pItem)
                        .fg
                        .set_isRecursive(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    if (*pRecTerm).selFlags
                        & crate::src::headers::sqliteInt_h::SF_Recursive
                            as crate::src::ext::rtree::rtree::U32_0
                        != 0
                    {
                        crate::src::src::util::sqlite3ErrorMsg_args(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            b"multiple references to recursive table: %s\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[crate::src::src::printf::PrintfArg::Str(
                                __pCte_ref.zName as *mut ::core::ffi::c_char,
                            )],
                        );
                        return 2 as ::core::ffi::c_int;
                    }
                    (*pRecTerm).selFlags |= crate::src::headers::sqliteInt_h::SF_Recursive
                        as crate::src::ext::rtree::rtree::U32_0;
                    if iRecTab < 0 as ::core::ffi::c_int {
                        let fresh6 = __pParse_ref.nTab;
                        __pParse_ref.nTab += 1;
                        iRecTab = fresh6;
                    }
                    __pItem_ref.iCursor = iRecTab;
                }
                i += 1;
            }
            if (*pRecTerm).selFlags
                & crate::src::headers::sqliteInt_h::SF_Recursive
                    as crate::src::ext::rtree::rtree::U32_0
                == 0 as crate::src::ext::rtree::rtree::U32_0
            {
                break;
            }
            pRecTerm = (*pRecTerm).pPrior;
        }
        __pCte_ref.zCteErr = b"circular reference: %s\0" as *const u8 as *const ::core::ffi::c_char;
        let pSavedWith: *mut crate::src::headers::sqliteInt_h::With = __pParse_ref.pWith;
        __pParse_ref.pWith = pWith;
        if (*pSel).selFlags
            & crate::src::headers::sqliteInt_h::SF_Recursive as crate::src::ext::rtree::rtree::U32_0
            != 0
        {
            
            (*pRecTerm).pWith = (*pSel).pWith;
            let rc: ::core::ffi::c_int = crate::src::src::walker::sqlite3WalkSelect(
                pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
                pRecTerm as *mut crate::src::headers::sqliteInt_h::Select,
            );
            (*pRecTerm).pWith = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::With>();
            if rc != 0 {
                __pParse_ref.pWith = pSavedWith;
                return 2 as ::core::ffi::c_int;
            }
        } else if crate::src::src::walker::sqlite3WalkSelect(
            pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
            pSel as *mut crate::src::headers::sqliteInt_h::Select,
        ) != 0
        {
            __pParse_ref.pWith = pSavedWith;
            return 2 as ::core::ffi::c_int;
        }
        __pParse_ref.pWith = pWith;
        pLeft = pSel;
        while !(*pLeft).pPrior.is_null() {
            pLeft = (*pLeft).pPrior;
        }
        pEList = (*pLeft).pEList;
        if !__pCte_ref.pCols.is_null() {
            if !pEList.is_null() && (*pEList).nExpr != (*__pCte_ref.pCols).nExpr {
                crate::src::src::util::sqlite3ErrorMsg_args(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    b"table %s has %d values for %d columns\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[
                        crate::src::src::printf::PrintfArg::Str(
                            __pCte_ref.zName as *mut ::core::ffi::c_char,
                        ),
                        crate::src::src::printf::PrintfArg::Int(
                            (*pEList).nExpr as crate::src::ext::rtree::rtree::I64_0,
                        ),
                        crate::src::src::printf::PrintfArg::Int(
                            (*__pCte_ref.pCols).nExpr as crate::src::ext::rtree::rtree::I64_0,
                        ),
                    ],
                );
                __pParse_ref.pWith = pSavedWith;
                return 2 as ::core::ffi::c_int;
            }
            pEList = __pCte_ref.pCols;
        }
        sqlite3ColumnsFromExprList(pParse, pEList, &raw mut (*pTab).nCol, &raw mut (*pTab).aCol);
        if bMayRecursive != 0 {
            if (*pSel).selFlags
                & crate::src::headers::sqliteInt_h::SF_Recursive
                    as crate::src::ext::rtree::rtree::U32_0
                != 0
            {
                __pCte_ref.zCteErr = b"multiple recursive references: %s\0" as *const u8
                    as *const ::core::ffi::c_char;
            } else {
                __pCte_ref.zCteErr = b"recursive reference in a subquery: %s\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            crate::src::src::walker::sqlite3WalkSelect(
                pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
                pSel as *mut crate::src::headers::sqliteInt_h::Select,
            );
        }
        __pCte_ref.zCteErr = ::core::ptr::null::<::core::ffi::c_char>();
        __pParse_ref.pWith = pSavedWith;
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SelectPopWith(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    p: *mut crate::src::headers::sqliteInt_h::Select,
) {
    let pParse: *mut crate::src::headers::sqliteInt_h::Parse = (*pWalker).pParse;
    if !(*pParse).pWith.is_null() && (*p).pPrior.is_null() {
        let pWith: *mut crate::src::headers::sqliteInt_h::With = (*findRightmost(p)).pWith;
        if !pWith.is_null() {
            (*pParse).pWith = (*pWith).pOuter;
        }
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3ExpandSubquery(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pFrom: *mut crate::src::headers::sqliteInt_h::SrcItem,
) -> ::core::ffi::c_int {
    let mut pSel: *mut crate::src::headers::sqliteInt_h::Select;
    
    let __pFrom_ref = unsafe { &mut *pFrom };
    pSel = (*__pFrom_ref.u4.pSubq).pSelect;
    let pTab: *mut crate::src::headers::sqliteInt_h::Table = crate::src::src::malloc::sqlite3DbMallocZero(
        (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Table>()
            as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::headers::sqliteInt_h::Table;
    __pFrom_ref.pSTab = pTab;
    if pTab.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    (*pTab).nTabRef = 1 as crate::src::ext::rtree::rtree::U32_0;
    if !__pFrom_ref.zAlias.is_null() {
        (*pTab).zName = crate::src::src::malloc::sqlite3DbStrDup(
            (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            __pFrom_ref.zAlias,
        );
    } else {
        (*pTab).zName = crate::src::src::printf::sqlite3MPrintf_args(
            (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            b"%!S\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::SrcItem(
                pFrom as *mut crate::src::headers::sqliteInt_h::SrcItem,
            )],
        );
    }
    while !(*pSel).pPrior.is_null() {
        pSel = (*pSel).pPrior;
    }
    sqlite3ColumnsFromExprList(
        pParse,
        (*pSel).pEList,
        &raw mut (*pTab).nCol,
        &raw mut (*pTab).aCol,
    );
    (*pTab).iPKey = -(1 as ::core::ffi::c_int) as crate::src::fts5::I16_0;
    (*pTab).eTabType = crate::src::headers::sqliteInt_h::TABTYP_VIEW as u8;
    (*pTab).nRowLogEst = 200 as crate::src::headers::sqliteInt_h::LogEst;
    (*pTab).tabFlags |= (crate::src::headers::sqliteInt_h::TF_Ephemeral
        | crate::src::headers::sqliteInt_h::TF_NoVisibleRowid)
        as crate::src::ext::rtree::rtree::U32_0;
    if (*pParse).nErr != 0 {
        crate::src::headers::sqlite3_h::SQLITE_ERROR
    } else {
        crate::src::headers::sqlite3_h::SQLITE_OK
    }
}

unsafe extern "C" fn inAnyUsingClause(
    zName: *const ::core::ffi::c_char,
    mut pBase: *mut crate::src::headers::sqliteInt_h::SrcItem,
    mut N: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    while N > 0 as ::core::ffi::c_int {
        N -= 1;
        pBase = pBase.offset(1);
        if (*pBase).fg.isUsing() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            continue;
        }
        if (*pBase).u3.pUsing.is_null() {
            continue;
        }
        if crate::src::src::build::sqlite3IdListIndex(
            (*pBase).u3.pUsing as *mut crate::src::headers::sqliteInt_h::IdList,
            zName,
        ) >= 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn selectExpander(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    p: *mut crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    let pParse: *mut crate::src::headers::sqliteInt_h::Parse = (*pWalker).pParse;
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    let mut k: ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int;
    
    
    let mut pFrom: *mut crate::src::headers::sqliteInt_h::SrcItem;
    let __pParse_ref = unsafe { &mut *pParse };
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
    let mut pE: *mut crate::src::headers::sqliteInt_h::Expr;
    let mut pRight: *mut crate::src::headers::sqliteInt_h::Expr;
    let mut pExpr: *mut crate::src::headers::sqliteInt_h::Expr;
    let __p_ref = unsafe { &mut *p };
    let selFlags: crate::src::fts5::U16_0 = __p_ref.selFlags as crate::src::fts5::U16_0;
    let mut elistFlags: crate::src::ext::rtree::rtree::U32_0 =
        0 as crate::src::ext::rtree::rtree::U32_0;
    __p_ref.selFlags |=
        crate::src::headers::sqliteInt_h::SF_Expanded as crate::src::ext::rtree::rtree::U32_0;
    if (*db).mallocFailed != 0 {
        return crate::src::headers::sqliteInt_h::WRC_Abort;
    }
    if selFlags as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::SF_Expanded
        != 0 as ::core::ffi::c_int
    {
        return crate::src::headers::sqliteInt_h::WRC_Prune;
    }
    if (*pWalker).eCode != 0 {
        __pParse_ref.nSelect += 1;
        __p_ref.selId = __pParse_ref.nSelect as crate::src::ext::rtree::rtree::U32_0;
    }
    let pTabList: *mut crate::src::headers::sqliteInt_h::SrcList = __p_ref.pSrc;
    let pEList: *mut crate::src::headers::sqliteInt_h::ExprList = __p_ref.pEList;
    if !__pParse_ref.pWith.is_null()
        && __p_ref.selFlags
            & crate::src::headers::sqliteInt_h::SF_View as crate::src::ext::rtree::rtree::U32_0
            != 0
    {
        if __p_ref.pWith.is_null() {
            __p_ref.pWith = crate::src::src::malloc::sqlite3DbMallocZero(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                16_usize.wrapping_add(1_usize.wrapping_mul(::core::mem::size_of::<
                    crate::src::headers::sqliteInt_h::Cte,
                >() as usize)) as crate::src::ext::rtree::rtree::U64_0,
            ) as *mut crate::src::headers::sqliteInt_h::With;
            if __p_ref.pWith.is_null() {
                return crate::src::headers::sqliteInt_h::WRC_Abort;
            }
        }
        (*__p_ref.pWith).bView = 1 as ::core::ffi::c_int;
    }
    sqlite3WithPush(pParse, __p_ref.pWith, 0_u8);
    crate::src::src::build::sqlite3SrcListAssignCursors(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        pTabList as *mut crate::src::headers::sqliteInt_h::SrcList,
    );
    i = 0 as ::core::ffi::c_int;
    pFrom = &raw mut (*pTabList).a as *mut crate::src::headers::sqliteInt_h::SrcItem;
    while i < (*pTabList).nSrc {
        let pTab: *mut crate::src::headers::sqliteInt_h::Table;
        if (*pFrom).pSTab.is_null() {
            if (*pFrom).zName.is_null() {
                
                let pSel: *mut crate::src::headers::sqliteInt_h::Select = (*(*pFrom).u4.pSubq).pSelect;
                if crate::src::src::walker::sqlite3WalkSelect(
                    pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
                    pSel as *mut crate::src::headers::sqliteInt_h::Select,
                ) != 0
                {
                    return crate::src::headers::sqliteInt_h::WRC_Abort;
                }
                if sqlite3ExpandSubquery(pParse, pFrom) != 0 {
                    return crate::src::headers::sqliteInt_h::WRC_Abort;
                }
            } else {
                rc = resolveFromTermToCte(pParse, pWalker, pFrom);
                if rc != 0 as ::core::ffi::c_int {
                    if rc > 1 as ::core::ffi::c_int {
                        return crate::src::headers::sqliteInt_h::WRC_Abort;
                    }
                } else {
                    pTab = crate::src::src::build::sqlite3LocateTableItem(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        0 as crate::src::ext::rtree::rtree::U32_0,
                        pFrom as *mut crate::src::headers::sqliteInt_h::SrcItem,
                    ) as *mut crate::src::headers::sqliteInt_h::Table;
                    (*pFrom).pSTab = pTab;
                    if pTab.is_null() {
                        return crate::src::headers::sqliteInt_h::WRC_Abort;
                    }
                    if (*pTab).nTabRef >= 0xffff as crate::src::ext::rtree::rtree::U32_0 {
                        crate::src::src::util::sqlite3ErrorMsg_args(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            b"too many references to \"%s\": max 65535\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[crate::src::src::printf::PrintfArg::Str(
                                (*pTab).zName as *mut ::core::ffi::c_char,
                            )],
                        );
                        (*pFrom).pSTab =
                            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
                        return crate::src::headers::sqliteInt_h::WRC_Abort;
                    }
                    (*pTab).nTabRef = (*pTab).nTabRef.wrapping_add(1);
                    if ((*pTab).eTabType as ::core::ffi::c_int != crate::src::headers::sqliteInt_h::TABTYP_VTAB)
                        && cannotBeFunction(pParse, pFrom) != 0
                    {
                        return crate::src::headers::sqliteInt_h::WRC_Abort;
                    }
                    if ((*pTab).eTabType as ::core::ffi::c_int != crate::src::headers::sqliteInt_h::TABTYP_NORM)
                    {
                        
                        let __pWalker_ref = unsafe { &mut *pWalker };
                        let eCodeOrig: crate::src::ext::rtree::rtree::U8_0 =
                            __pWalker_ref.eCode as u8;
                        if crate::src::src::build::sqlite3ViewGetColumnNames(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            pTab as *mut crate::src::headers::sqliteInt_h::Table,
                        ) != 0
                        {
                            return crate::src::headers::sqliteInt_h::WRC_Abort;
                        }
                        let __pTab_ref = unsafe { &mut *pTab };
                        if __pTab_ref.eTabType as ::core::ffi::c_int
                            == crate::src::headers::sqliteInt_h::TABTYP_VIEW
                        {
                            if (*db).flags
                                & crate::src::headers::sqliteInt_h::SQLITE_EnableView
                                    as crate::src::ext::rtree::rtree::U64_0
                                == 0 as crate::src::ext::rtree::rtree::U64_0
                                && __pTab_ref.pSchema != (*(*db).aDb.offset(1_isize)).pSchema
                            {
                                crate::src::src::util::sqlite3ErrorMsg_args(
                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                    b"access to view \"%s\" prohibited\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    &[crate::src::src::printf::PrintfArg::Str(
                                        __pTab_ref.zName as *mut ::core::ffi::c_char,
                                    )],
                                );
                            }
                            crate::src::src::build::sqlite3SrcItemAttachSubquery(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                pFrom as *mut crate::src::headers::sqliteInt_h::SrcItem,
                                __pTab_ref.u.view.pSelect
                                    as *mut crate::src::headers::sqliteInt_h::Select,
                                1 as ::core::ffi::c_int,
                            );
                        } else if __pTab_ref.eTabType as ::core::ffi::c_int
                            == 1 as ::core::ffi::c_int
                            && (*pFrom).fg.fromDDL() as ::core::ffi::c_int != 0
                            && !__pTab_ref.u.vtab.p.is_null()
                            && (*__pTab_ref.u.vtab.p).eVtabRisk as ::core::ffi::c_int
                                > ((*db).flags
                                    & crate::src::headers::sqliteInt_h::SQLITE_TrustedSchema
                                        as crate::src::ext::rtree::rtree::U64_0
                                    != 0 as crate::src::ext::rtree::rtree::U64_0)
                                    as ::core::ffi::c_int
                        {
                            crate::src::src::util::sqlite3ErrorMsg_args(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                b"unsafe use of virtual table \"%s\"\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[crate::src::src::printf::PrintfArg::Str(
                                    __pTab_ref.zName as *mut ::core::ffi::c_char,
                                )],
                            );
                        }
                        let nCol: crate::src::fts5::I16_0 = __pTab_ref.nCol;
                        __pTab_ref.nCol = -(1 as ::core::ffi::c_int) as crate::src::fts5::I16_0;
                        __pWalker_ref.eCode = 1 as crate::src::fts5::U16_0;
                        if (*pFrom).fg.isSubquery() != 0 {
                            crate::src::src::walker::sqlite3WalkSelect(
                                pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
                                (*(*pFrom).u4.pSubq).pSelect
                                    as *mut crate::src::headers::sqliteInt_h::Select,
                            );
                        }
                        __pWalker_ref.eCode = eCodeOrig as crate::src::fts5::U16_0;
                        __pTab_ref.nCol = nCol;
                    }
                }
            }
            if (*pFrom).fg.isIndexedBy() as ::core::ffi::c_int != 0
                && sqlite3IndexedByLookup(pParse, pFrom) != 0
            {
                return crate::src::headers::sqliteInt_h::WRC_Abort;
            }
        }
        i += 1;
        pFrom = pFrom.offset(1);
    }
    if __pParse_ref.nErr != 0 || sqlite3ProcessJoin(pParse, p) != 0 {
        return crate::src::headers::sqliteInt_h::WRC_Abort;
    }
    k = 0 as ::core::ffi::c_int;
    while k < (*pEList).nExpr {
        pE = (*(&raw mut (*pEList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
            .offset(k as isize))
        .pExpr;
        if (*pE).op as ::core::ffi::c_int == crate::src::parse::TK_ASTERISK {
            break;
        }
        if (*pE).op as ::core::ffi::c_int == crate::src::parse::TK_DOT
            && (*(*pE).pRight).op as ::core::ffi::c_int == crate::src::parse::TK_ASTERISK
        {
            break;
        }
        elistFlags |= (*pE).flags;
        k += 1;
    }
    if k < (*pEList).nExpr {
        let a: *mut crate::src::headers::sqliteInt_h::ExprList_item =
            &raw mut (*pEList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item;
        let mut pNew: *mut crate::src::headers::sqliteInt_h::ExprList =
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
        let flags: ::core::ffi::c_int = (*__pParse_ref.db).flags as ::core::ffi::c_int;
        let longNames: ::core::ffi::c_int =
            (flags & crate::src::headers::sqliteInt_h::SQLITE_FullColNames
                != 0 as ::core::ffi::c_int
                && flags & crate::src::headers::sqliteInt_h::SQLITE_ShortColNames
                    == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        k = 0 as ::core::ffi::c_int;
        while k < (*pEList).nExpr {
            pE = (*a.offset(k as isize)).pExpr;
            elistFlags |= (*pE).flags;
            pRight = (*pE).pRight;
            if (*pE).op as ::core::ffi::c_int != crate::src::parse::TK_ASTERISK
                && ((*pE).op as ::core::ffi::c_int != crate::src::parse::TK_DOT
                    || (*pRight).op as ::core::ffi::c_int != crate::src::parse::TK_ASTERISK)
            {
                pNew = crate::src::src::expr::sqlite3ExprListAppend(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    pNew as *mut crate::src::headers::sqliteInt_h::ExprList,
                    (*a.offset(k as isize)).pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::ExprList;
                if !pNew.is_null() {
                    let __pNew_ref = unsafe { &mut *pNew };
                    let fresh0 = &mut (*(&raw mut __pNew_ref.a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset((__pNew_ref.nExpr - 1 as ::core::ffi::c_int) as isize))
                    .zEName;
                    *fresh0 = (*a.offset(k as isize)).zEName;
                    let fresh1 = &mut (*(&raw mut __pNew_ref.a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset((__pNew_ref.nExpr - 1 as ::core::ffi::c_int) as isize))
                    .fg;
                    (*fresh1)
                        .set_eEName((*a.offset(k as isize)).fg.eEName() as ::core::ffi::c_uint);
                    let fresh2 = &mut (*a.offset(k as isize)).zEName;
                    *fresh2 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                let fresh3 = &mut (*a.offset(k as isize)).pExpr;
                *fresh3 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
            } else {
                let mut tableSeen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut zTName: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let iErrOfst: ::core::ffi::c_int;
                if (*pE).op as ::core::ffi::c_int == crate::src::parse::TK_DOT {
                    zTName = (*(*pE).pLeft).u.zToken;
                    iErrOfst = (*(*pE).pRight).w.iOfst;
                } else {
                    iErrOfst = (*pE).w.iOfst;
                }
                let mut current_block_197: u64;
                i = 0 as ::core::ffi::c_int;
                pFrom = &raw mut (*pTabList).a as *mut crate::src::headers::sqliteInt_h::SrcItem;
                while i < (*pTabList).nSrc {
                    let mut nAdd: ::core::ffi::c_int;
                    let pTab_0: *mut crate::src::headers::sqliteInt_h::Table = (*pFrom).pSTab;
                    let mut pNestedFrom: *mut crate::src::headers::sqliteInt_h::ExprList =
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
                    let mut zTabName: *mut ::core::ffi::c_char;
                    let mut zSchemaName: *const ::core::ffi::c_char =
                        ::core::ptr::null::<::core::ffi::c_char>();
                    let iDb: ::core::ffi::c_int;
                    let pUsing: *mut crate::src::headers::sqliteInt_h::IdList;
                    zTabName = (*pFrom).zAlias;
                    if zTabName.is_null() {
                        zTabName = (*pTab_0).zName;
                    }
                    if (*db).mallocFailed != 0 {
                        break;
                    }
                    if (*pFrom).fg.isNestedFrom() != 0 {
                        pNestedFrom = (*(*(*pFrom).u4.pSubq).pSelect).pEList;
                        current_block_197 = 17623951255125923504;
                    } else if !zTName.is_null()
                        && crate::src::src::util::sqlite3StrICmp(zTName, zTabName)
                            != 0 as ::core::ffi::c_int
                    {
                        current_block_197 = 1428307939028130064;
                    } else {
                        pNestedFrom =
                            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
                        iDb = crate::src::src::prepare::sqlite3SchemaToIndex(
                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                            (*pTab_0).pSchema as *mut crate::src::headers::sqliteInt_h::Schema,
                        );
                        zSchemaName = if iDb >= 0 as ::core::ffi::c_int {
                            (*(*db).aDb.offset(iDb as isize)).zDbSName as *const ::core::ffi::c_char
                        } else {
                            b"*\0" as *const u8 as *const ::core::ffi::c_char
                        };
                        current_block_197 = 17623951255125923504;
                    }
                    if current_block_197 == 17623951255125923504 {
                        if (i + 1 as ::core::ffi::c_int) < (*pTabList).nSrc
                            && (*pFrom.offset(1_isize)).fg.isUsing() as ::core::ffi::c_int
                                != 0
                            && selFlags as ::core::ffi::c_int
                                & crate::src::headers::sqliteInt_h::SF_NestedFrom
                                != 0 as ::core::ffi::c_int
                        {
                            let mut ii: ::core::ffi::c_int;
                            pUsing = (*pFrom.offset(1_isize)).u3.pUsing;
                            ii = 0 as ::core::ffi::c_int;
                            while ii < (*pUsing).nId {
                                let zUName: *const ::core::ffi::c_char =
                                    (*(&raw mut (*pUsing).a
                                        as *mut crate::src::headers::sqliteInt_h::IdList_item)
                                        .offset(ii as isize))
                                    .zName;
                                pRight = crate::src::src::expr::sqlite3Expr(
                                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                    crate::src::parse::TK_ID,
                                    zUName,
                                )
                                    as *mut crate::src::headers::sqliteInt_h::Expr;
                                crate::src::src::expr::sqlite3ExprSetErrorOffset(
                                    pRight as *mut crate::src::headers::sqliteInt_h::Expr,
                                    iErrOfst,
                                );
                                pNew = crate::src::src::expr::sqlite3ExprListAppend(
                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                    pNew as *mut crate::src::headers::sqliteInt_h::ExprList,
                                    pRight as *mut crate::src::headers::sqliteInt_h::Expr,
                                )
                                    as *mut crate::src::headers::sqliteInt_h::ExprList;
                                if !pNew.is_null() {
                                    let pX: *mut crate::src::headers::sqliteInt_h::ExprList_item =
                                        (&raw mut (*pNew).a as *mut crate::src::headers::sqliteInt_h::ExprList_item).offset(
                                            ((*pNew).nExpr - 1 as ::core::ffi::c_int) as isize,
                                        )
                                            as *mut crate::src::headers::sqliteInt_h::ExprList_item;
                                    let __pX_ref = unsafe { &mut *pX };
                                    __pX_ref.zEName = crate::src::src::printf::sqlite3MPrintf_args(
                                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                        b"..%s\0" as *const u8 as *const ::core::ffi::c_char,
                                        &[crate::src::src::printf::PrintfArg::Str(zUName as *mut ::core::ffi::c_char)],
                                    );
                                    __pX_ref.fg.set_eEName(
                                        crate::src::headers::sqliteInt_h::ENAME_TAB
                                            as ::core::ffi::c_uint
                                            as ::core::ffi::c_uint,
                                    );
                                    __pX_ref.fg.set_bUsingTerm(
                                        1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                    );
                                }
                                ii += 1;
                            }
                        } else {
                            pUsing = ::core::ptr::null_mut::<
                                crate::src::headers::sqliteInt_h::IdList,
                            >();
                        }
                        nAdd = (*pTab_0).nCol as ::core::ffi::c_int;
                        if (*pTab_0).tabFlags
                            & crate::src::headers::sqliteInt_h::TF_NoVisibleRowid
                                as crate::src::ext::rtree::rtree::U32_0
                            == 0 as crate::src::ext::rtree::rtree::U32_0
                            && selFlags as ::core::ffi::c_int
                                & crate::src::headers::sqliteInt_h::SF_NestedFrom
                                != 0 as ::core::ffi::c_int
                        {
                            nAdd += 1;
                        }
                        let mut current_block_196: u64;
                        j = 0 as ::core::ffi::c_int;
                        while j < nAdd {
                            let zName: *const ::core::ffi::c_char;
                            let pX_0: *mut crate::src::headers::sqliteInt_h::ExprList_item;
                            if j == (*pTab_0).nCol as ::core::ffi::c_int {
                                zName = crate::src::src::expr::sqlite3RowidAlias(
                                    pTab_0 as *mut crate::src::headers::sqliteInt_h::Table,
                                );
                                if zName.is_null() {
                                    current_block_196 = 8507773468922410051;
                                } else {
                                    current_block_196 = 4338462691184853296;
                                }
                            } else {
                                let __pTab_0_ref = unsafe { &mut *pTab_0 };
                                zName = (*__pTab_0_ref.aCol.offset(j as isize)).zCnName;
                                if !pNestedFrom.is_null()
                                    && (*(&raw mut (*pNestedFrom).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                        .offset(j as isize))
                                    .fg
                                    .eEName()
                                        as ::core::ffi::c_int
                                        == crate::src::headers::sqliteInt_h::ENAME_ROWID
                                    || !zTName.is_null()
                                        && !pNestedFrom.is_null()
                                        && crate::src::src::resolve::sqlite3MatchEName(
                                            (&raw mut (*pNestedFrom).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                .offset(j as isize)
                                                as *mut crate::src::headers::sqliteInt_h::ExprList_item
                                                as *const crate::src::headers::sqliteInt_h::ExprList_item,
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            zTName,
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                        ) == 0 as ::core::ffi::c_int
                                    || __p_ref.selFlags & crate::src::headers::sqliteInt_h::SF_IncludeHidden as crate::src::ext::rtree::rtree::U32_0
                                        == 0 as crate::src::ext::rtree::rtree::U32_0
                                        && (*__pTab_0_ref.aCol.offset(j as isize)).colFlags
                                            as ::core::ffi::c_int
                                            & crate::src::headers::sqliteInt_h::COLFLAG_HIDDEN
                                            != 0 as ::core::ffi::c_int
                                    || (*__pTab_0_ref.aCol.offset(j as isize)).colFlags
                                        as ::core::ffi::c_int
                                        & crate::src::headers::sqliteInt_h::COLFLAG_NOEXPAND
                                        != 0 as ::core::ffi::c_int
                                        && zTName.is_null()
                                        && selFlags as ::core::ffi::c_int
                                            & 0x800 as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                {
                                    current_block_196 = 8507773468922410051;
                                } else {
                                    current_block_196 = 4338462691184853296;
                                }
                            }
                            if current_block_196 == 4338462691184853296 {
                                                                tableSeen = 1 as ::core::ffi::c_int;
                                                                if i > 0 as ::core::ffi::c_int
                                                                    && zTName.is_null()
                                                                    && selFlags as ::core::ffi::c_int
                                                                        & crate::src::headers::sqliteInt_h::SF_NestedFrom
                                                                        == 0 as ::core::ffi::c_int
                                                                {
                                                                    if (*pFrom).fg.isUsing() as ::core::ffi::c_int != 0
                                                                        && crate::src::src::build::sqlite3IdListIndex((*pFrom).u3.pUsing as *mut crate::src::headers::sqliteInt_h::IdList, zName)
                                                                            >= 0 as ::core::ffi::c_int
                                                                    {
                                                                        current_block_196 = 8507773468922410051;
                                                                    } else {
                                                                        current_block_196 = 17727836384662615028;
                                                                    }
                                                                } else {
                                                                    current_block_196 = 17727836384662615028;
                                                                }
                                                                match current_block_196 {
                                                                    8507773468922410051 => {}
                                                                    _ => {
                                                                        pRight =  crate::src::src::expr::sqlite3Expr(db as *mut crate::src::headers::sqliteInt_h::sqlite3, crate::src::parse::TK_ID, zName) as *mut crate::src::headers::sqliteInt_h::Expr;
                                                                        if (*pTabList).nSrc > 1 as ::core::ffi::c_int
                                                                            && ((*pFrom).fg.jointype as ::core::ffi::c_int
                                                                                & crate::src::headers::sqliteInt_h::JT_LTORJ
                                                                                == 0 as ::core::ffi::c_int
                                                                                || selFlags as ::core::ffi::c_int
                                                                                    & crate::src::headers::sqliteInt_h::SF_NestedFrom
                                                                                    != 0 as ::core::ffi::c_int
                                                                                || inAnyUsingClause(
                                                                                    zName,
                                                                                    pFrom,
                                                                                    (*pTabList).nSrc
                                                                                        - i
                                                                                        - 1 as ::core::ffi::c_int,
                                                                                ) == 0)
                                                                            || __pParse_ref.eParseMode as ::core::ffi::c_int
                                                                                >= crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME
                                                                        {
                                                                            let mut pLeft: *mut crate::src::headers::sqliteInt_h::Expr;
                                                                            pLeft =  crate::src::src::expr::sqlite3Expr(db as *mut crate::src::headers::sqliteInt_h::sqlite3, crate::src::parse::TK_ID, zTabName) as *mut crate::src::headers::sqliteInt_h::Expr;
                                                                            pExpr =
                                                                                
                                                                                crate::src::src::expr::sqlite3PExpr(pParse as *mut crate::src::headers::sqliteInt_h::Parse, crate::src::parse::TK_DOT,  pLeft as *mut crate::src::headers::sqliteInt_h::Expr,  pRight as *mut crate::src::headers::sqliteInt_h::Expr) as
                            *mut crate::src::headers::sqliteInt_h::Expr;
                                                                            if __pParse_ref.eParseMode as ::core::ffi::c_int
                                                                                >= crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME
                                                                                && !(*pE).pLeft.is_null()
                                                                            {
                                                                                crate::src::src::alter::sqlite3RenameTokenRemap(
                                                                                    
                                                                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                                    pLeft as *const ::core::ffi::c_void,
                                                                                    (*pE).pLeft
                                                                                        as *const ::core::ffi::c_void,
                                                                                );
                                                                            }
                                                                            if !zSchemaName.is_null() {
                                                                                pLeft =  crate::src::src::expr::sqlite3Expr(db as *mut crate::src::headers::sqliteInt_h::sqlite3, crate::src::parse::TK_ID, zSchemaName) as
                            *mut crate::src::headers::sqliteInt_h::Expr;
                                                                                pExpr =  crate::src::src::expr::sqlite3PExpr(
                                                                                    
                                                                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse, crate::src::parse::TK_DOT,  pLeft as *mut crate::src::headers::sqliteInt_h::Expr,  pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
                                                                                ) as
                            *mut crate::src::headers::sqliteInt_h::Expr;
                                                                            }
                                                                        } else {
                                                                            pExpr = pRight;
                                                                        }
                                                                        crate::src::src::expr::sqlite3ExprSetErrorOffset(pExpr as *mut crate::src::headers::sqliteInt_h::Expr, iErrOfst);
                                                                        pNew =  crate::src::src::expr::sqlite3ExprListAppend(pParse as *mut crate::src::headers::sqliteInt_h::Parse,  pNew as *mut crate::src::headers::sqliteInt_h::ExprList,  pExpr as *mut crate::src::headers::sqliteInt_h::Expr) as
                            *mut crate::src::headers::sqliteInt_h::ExprList;
                                                                        if pNew.is_null() {
                                                                            break;
                                                                        }
                                                                        pX_0 = (&raw mut (*pNew).a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                                            .offset(
                                                                                ((*pNew).nExpr - 1 as ::core::ffi::c_int)
                                                                                    as isize,
                                                                            )
                                                                            as *mut crate::src::headers::sqliteInt_h::ExprList_item
                                                                            as *mut crate::src::headers::sqliteInt_h::ExprList_item;
                                                                        if selFlags as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::SF_NestedFrom
                                                                            != 0 as ::core::ffi::c_int
                                                                            && ((__pParse_ref.eParseMode as ::core::ffi::c_int) < crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME)
                                                                        {
                                                                            if !pNestedFrom.is_null()
                                                                                && (crate::src::headers::sqliteInt_h::ViewCanHaveRowid == 0
                                                                                    || j < (*pNestedFrom).nExpr)
                                                                            {
                                                                                (*pX_0).zEName = crate::src::src::malloc::sqlite3DbStrDup(
                                                                                    
                                                                                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                                                                    (*(&raw mut (*pNestedFrom).a
                                                                                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                                                        .offset(j as isize))
                                                                                    .zEName,
                                                                                );
                                                                            } else {
                                                                                (*pX_0).zEName = crate::src::src::printf::sqlite3MPrintf_args(
                                                                                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                                                                    b"%s.%s.%s\0" as *const u8
                                                                                        as *const ::core::ffi::c_char,
                                                                                    &[crate::src::src::printf::PrintfArg::Str(zSchemaName as *mut ::core::ffi::c_char), crate::src::src::printf::PrintfArg::Str(zTabName as *mut ::core::ffi::c_char), crate::src::src::printf::PrintfArg::Str(zName as *mut ::core::ffi::c_char)],
                                                                                );
                                                                            }
                                                                            let __pTab_0_ref = unsafe { &mut *pTab_0 };
                                                                            (*pX_0).fg.set_eEName(
                                                                                (if j
                                                                                    == __pTab_0_ref.nCol as ::core::ffi::c_int
                                                                                {
                                                                                    crate::src::headers::sqliteInt_h::ENAME_ROWID
                                                                                } else {
                                                                                    crate::src::headers::sqliteInt_h::ENAME_TAB
                                                                                })
                                                                                    as ::core::ffi::c_uint
                                                                                    as ::core::ffi::c_uint,
                                                                            );
                                                                            if (*pFrom).fg.isUsing() as ::core::ffi::c_int
                                                                                != 0
                                                                                && crate::src::src::build::sqlite3IdListIndex(
                                                                                    
                                                                                    (*pFrom).u3.pUsing as *mut crate::src::headers::sqliteInt_h::IdList,
                                                                                    zName,
                                                                                ) >= 0 as ::core::ffi::c_int
                                                                                || !pUsing.is_null()
                                                                                    && crate::src::src::build::sqlite3IdListIndex(pUsing as *mut crate::src::headers::sqliteInt_h::IdList, zName)
                                                                                        >= 0 as ::core::ffi::c_int
                                                                                || j < __pTab_0_ref.nCol as ::core::ffi::c_int
                                                                                    && (*__pTab_0_ref.aCol.offset(j as isize))
                                                                                        .colFlags
                                                                                        as ::core::ffi::c_int
                                                                                        & crate::src::headers::sqliteInt_h::COLFLAG_NOEXPAND
                                                                                        != 0
                                                                            {
                                                                                (*pX_0).fg.set_bNoExpand(
                                                                                    1 as ::core::ffi::c_uint
                                                                                        as ::core::ffi::c_uint,
                                                                                );
                                                                            }
                                                                        } else if longNames != 0 {
                                                                            (*pX_0).zEName = crate::src::src::printf::sqlite3MPrintf_args(
                                                                                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                                                                b"%s.%s\0" as *const u8
                                                                                    as *const ::core::ffi::c_char,
                                                                                &[crate::src::src::printf::PrintfArg::Str(zTabName as *mut ::core::ffi::c_char), crate::src::src::printf::PrintfArg::Str(zName as *mut ::core::ffi::c_char)],
                                                                            );
                                                                            (*pX_0).fg.set_eEName(
                                                                                crate::src::headers::sqliteInt_h::ENAME_NAME as ::core::ffi::c_uint
                                                                                    as ::core::ffi::c_uint,
                                                                            );
                                                                        } else {
                                                                            (*pX_0).zEName = crate::src::src::malloc::sqlite3DbStrDup(db as *mut crate::src::headers::sqliteInt_h::sqlite3, zName);
                                                                            (*pX_0).fg.set_eEName(
                                                                                crate::src::headers::sqliteInt_h::ENAME_NAME as ::core::ffi::c_uint
                                                                                    as ::core::ffi::c_uint,
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                            }
                            j += 1;
                        }
                    }
                    i += 1;
                    pFrom = pFrom.offset(1);
                }
                if tableSeen == 0 {
                    if !zTName.is_null() {
                        crate::src::src::util::sqlite3ErrorMsg_args(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            b"no such table: %s\0" as *const u8 as *const ::core::ffi::c_char,
                            &[crate::src::src::printf::PrintfArg::Str(
                                zTName as *mut ::core::ffi::c_char,
                            )],
                        );
                    } else {
                        crate::src::src::util::sqlite3ErrorMsg_args(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            b"no tables specified\0" as *const u8 as *const ::core::ffi::c_char,
                            &[],
                        );
                    }
                }
            }
            k += 1;
        }
        crate::src::src::expr::sqlite3ExprListDelete(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pEList as *mut crate::src::headers::sqliteInt_h::ExprList,
        );
        __p_ref.pEList = pNew;
    }
    if !__p_ref.pEList.is_null() {
        if (*__p_ref.pEList).nExpr
            > (*db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_COLUMN as usize]
        {
            crate::src::src::util::sqlite3ErrorMsg_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                b"too many columns in result set\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
            return crate::src::headers::sqliteInt_h::WRC_Abort;
        }
        if elistFlags
            & (crate::src::headers::sqliteInt_h::EP_HasFunc
                | crate::src::headers::sqliteInt_h::EP_Subquery)
                as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
        {
            __p_ref.selFlags |= crate::src::headers::sqliteInt_h::SF_ComplexResult
                as crate::src::ext::rtree::rtree::U32_0;
        }
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}

unsafe extern "C" fn sqlite3SelectExpand(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pSelect: *mut crate::src::headers::sqliteInt_h::Select,
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
        crate::src::src::walker::sqlite3ExprWalkNoop
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
    w.pParse = pParse;
    if (*pParse).hasCompound != 0 {
        w.xSelectCallback = Some(
            convertCompoundSelectToSubquery
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
        crate::src::src::walker::sqlite3WalkSelect(
            &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
            pSelect as *mut crate::src::headers::sqliteInt_h::Select,
        );
    }
    w.xSelectCallback = Some(
        selectExpander
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
        sqlite3SelectPopWith
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
    w.eCode = 0 as crate::src::fts5::U16_0;
    crate::src::src::walker::sqlite3WalkSelect(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        pSelect as *mut crate::src::headers::sqliteInt_h::Select,
    );
}

unsafe extern "C" fn selectAddSubqueryTypeInfo(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    p: *mut crate::src::headers::sqliteInt_h::Select,
) {
    
    let mut i: ::core::ffi::c_int;
    
    let mut pFrom: *mut crate::src::headers::sqliteInt_h::SrcItem;
    let __p_ref = unsafe { &mut *p };
    if __p_ref.selFlags
        & crate::src::headers::sqliteInt_h::SF_HasTypeInfo as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        return;
    }
    __p_ref.selFlags |=
        crate::src::headers::sqliteInt_h::SF_HasTypeInfo as crate::src::ext::rtree::rtree::U32_0;
    let pParse: *mut crate::src::headers::sqliteInt_h::Parse = (*pWalker).pParse;
    let pTabList: *mut crate::src::headers::sqliteInt_h::SrcList = __p_ref.pSrc;
    i = 0 as ::core::ffi::c_int;
    pFrom = &raw mut (*pTabList).a as *mut crate::src::headers::sqliteInt_h::SrcItem;
    while i < (*pTabList).nSrc {
        let pTab: *mut crate::src::headers::sqliteInt_h::Table = (*pFrom).pSTab;
        if (*pTab).tabFlags
            & crate::src::headers::sqliteInt_h::TF_Ephemeral as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
            && (*pFrom).fg.isSubquery() as ::core::ffi::c_int != 0
        {
            let pSel: *mut crate::src::headers::sqliteInt_h::Select =
                (*(*pFrom).u4.pSubq).pSelect;
            sqlite3SubqueryColumnTypes(
                pParse,
                pTab,
                pSel,
                crate::src::headers::sqliteInt_h::SQLITE_AFF_NONE as ::core::ffi::c_char,
            );
        }
        i += 1;
        pFrom = pFrom.offset(1);
    }
}

unsafe extern "C" fn sqlite3SelectAddTypeInfo(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pSelect: *mut crate::src::headers::sqliteInt_h::Select,
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
    w.xSelectCallback = Some(
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
    w.xSelectCallback2 = Some(
        selectAddSubqueryTypeInfo
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
    w.xExprCallback = Some(
        crate::src::src::walker::sqlite3ExprWalkNoop
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
    w.pParse = pParse;
    crate::src::src::walker::sqlite3WalkSelect(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        pSelect as *mut crate::src::headers::sqliteInt_h::Select,
    );
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3SelectPrep(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
    pOuterNC: *mut crate::src::headers::sqliteInt_h::NameContext,
) {
    let __pParse_ref = unsafe { &*pParse };
    if (*__pParse_ref.db).mallocFailed != 0 {
        return;
    }
    if (*p).selFlags
        & crate::src::headers::sqliteInt_h::SF_HasTypeInfo as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        return;
    }
    sqlite3SelectExpand(pParse, p);
    if __pParse_ref.nErr != 0 {
        return;
    }
    crate::src::src::resolve::sqlite3ResolveSelectNames(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        p as *mut crate::src::headers::sqliteInt_h::Select,
        pOuterNC as *mut crate::src::headers::sqliteInt_h::NameContext,
    );
    if __pParse_ref.nErr != 0 {
        return;
    }
    sqlite3SelectAddTypeInfo(pParse, p);
}

unsafe extern "C" fn analyzeAggFuncArgs(
    pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo,
    pNC: *mut crate::src::headers::sqliteInt_h::NameContext,
) {
    let mut i: ::core::ffi::c_int;
    (*pNC).ncFlags |= crate::src::headers::sqliteInt_h::NC_InAggFunc;
    i = 0 as ::core::ffi::c_int;
    while i < (*pAggInfo).nFunc {
        let pExpr: *mut crate::src::headers::sqliteInt_h::Expr =
            (*(*pAggInfo).aFunc.offset(i as isize)).pFExpr;
        let __pExpr_ref = unsafe { &mut *pExpr };
        crate::src::src::expr::sqlite3ExprAnalyzeAggList(
            pNC as *mut crate::src::headers::sqliteInt_h::NameContext,
            __pExpr_ref.x.pList as *mut crate::src::headers::sqliteInt_h::ExprList,
        );
        if !__pExpr_ref.pLeft.is_null() {
            crate::src::src::expr::sqlite3ExprAnalyzeAggList(
                pNC as *mut crate::src::headers::sqliteInt_h::NameContext,
                (*__pExpr_ref.pLeft).x.pList as *mut crate::src::headers::sqliteInt_h::ExprList,
            );
        }
        if __pExpr_ref.flags
            & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
        {
            crate::src::src::expr::sqlite3ExprAnalyzeAggregates(
                pNC as *mut crate::src::headers::sqliteInt_h::NameContext,
                (*__pExpr_ref.y.pWin).pFilter as *mut crate::src::headers::sqliteInt_h::Expr,
            );
        }
        i += 1;
    }
    (*pNC).ncFlags &= !crate::src::headers::sqliteInt_h::NC_InAggFunc;
}

unsafe extern "C" fn optimizeAggregateUseOfIndexedExpr(
    mut _pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pSelect: *mut crate::src::headers::sqliteInt_h::Select,
    pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo,
    pNC: *mut crate::src::headers::sqliteInt_h::NameContext,
) {
    let __pAggInfo_ref = unsafe { &mut *pAggInfo };
    __pAggInfo_ref.nColumn = __pAggInfo_ref.nAccumulator;
    if __pAggInfo_ref.nSortingColumn > 0 as crate::src::ext::rtree::rtree::U32_0 {
        let mut mx: ::core::ffi::c_int = (*(*pSelect).pGroupBy).nExpr - 1 as ::core::ffi::c_int;
        let mut j: ::core::ffi::c_int;
        let mut k: ::core::ffi::c_int;
        j = 0 as ::core::ffi::c_int;
        while j < __pAggInfo_ref.nColumn {
            k = (*__pAggInfo_ref.aCol.offset(j as isize)).iSorterColumn;
            if k > mx {
                mx = k;
            }
            j += 1;
        }
        __pAggInfo_ref.nSortingColumn =
            (mx + 1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::U32_0;
    }
    analyzeAggFuncArgs(pAggInfo, pNC);
}

unsafe extern "C" fn aggregateIdxEprRefToColCallback(
    mut _pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    
    
    let __pExpr_ref = unsafe { &mut *pExpr };
    if __pExpr_ref.pAggInfo.is_null() {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    if __pExpr_ref.op as ::core::ffi::c_int == crate::src::parse::TK_AGG_COLUMN {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    if __pExpr_ref.op as ::core::ffi::c_int == crate::src::parse::TK_AGG_FUNCTION {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    if __pExpr_ref.op as ::core::ffi::c_int == crate::src::parse::TK_IF_NULL_ROW_1 {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    let pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo = __pExpr_ref.pAggInfo;
    if __pExpr_ref.iAgg as ::core::ffi::c_int >= (*pAggInfo).nColumn {
        return crate::src::headers::sqliteInt_h::WRC_Continue;
    }
    let pCol: *mut crate::src::headers::sqliteInt_h::AggInfo_col = (*pAggInfo).aCol.offset(__pExpr_ref.iAgg as isize)
        as *mut crate::src::headers::sqliteInt_h::AggInfo_col
        as *mut crate::src::headers::sqliteInt_h::AggInfo_col;
    __pExpr_ref.op = crate::src::parse::TK_AGG_COLUMN as u8;
    __pExpr_ref.iTable = (*pCol).iTable;
    __pExpr_ref.iColumn = (*pCol).iColumn as crate::src::headers::sqliteInt_h::YnVar;
    __pExpr_ref.flags &= !((0x2000 as ::core::ffi::c_int
        | 0x200 as ::core::ffi::c_int
        | 0x80000 as ::core::ffi::c_int)
        as crate::src::ext::rtree::rtree::U32_0);
    crate::src::headers::sqliteInt_h::WRC_Prune
}

unsafe extern "C" fn aggregateConvertIndexedExprRefToColumn(
    pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo,
) {
    let mut i: ::core::ffi::c_int;
    let mut w: crate::src::headers::sqliteInt_h::Walker = unsafe { ::core::mem::zeroed() };
    w.xExprCallback = Some(
        aggregateIdxEprRefToColCallback
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
    i = 0 as ::core::ffi::c_int;
    while i < (*pAggInfo).nFunc {
        crate::src::src::walker::sqlite3WalkExpr(
            &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
            (*(*pAggInfo).aFunc.offset(i as isize)).pFExpr
                as *mut crate::src::headers::sqliteInt_h::Expr,
        );
        i += 1;
    }
}

unsafe extern "C" fn assignAggregateRegisters(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo,
) {
    let __pAggInfo_ref = unsafe { &mut *pAggInfo };
    __pAggInfo_ref.iFirstReg = (*pParse).nMem + 1 as ::core::ffi::c_int;
    (*pParse).nMem += __pAggInfo_ref.nColumn + __pAggInfo_ref.nFunc;
}

unsafe extern "C" fn resetAccumulator(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo,
) {
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    let mut i: ::core::ffi::c_int;
    let mut pFunc: *mut crate::src::headers::sqliteInt_h::AggInfo_func;
    let __pAggInfo_ref = unsafe { &*pAggInfo };
    let nReg: ::core::ffi::c_int = __pAggInfo_ref.nFunc + __pAggInfo_ref.nColumn;
    if nReg == 0 as ::core::ffi::c_int {
        return;
    }
    if (*pParse).nErr != 0 {
        return;
    }
    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
        v,
        crate::src::headers::opcodes_h::OP_Null,
        0 as ::core::ffi::c_int,
        __pAggInfo_ref.iFirstReg,
        __pAggInfo_ref.iFirstReg + nReg - 1 as ::core::ffi::c_int,
    );
    pFunc = __pAggInfo_ref.aFunc as *mut crate::src::headers::sqliteInt_h::AggInfo_func;
    i = 0 as ::core::ffi::c_int;
    while i < __pAggInfo_ref.nFunc {
        if (*pFunc).iDistinct >= 0 as ::core::ffi::c_int {
            let pE: *mut crate::src::headers::sqliteInt_h::Expr = (*pFunc).pFExpr;
            if (*pE).x.pList.is_null() || (*(*pE).x.pList).nExpr != 1 as ::core::ffi::c_int {
                crate::src::src::util::sqlite3ErrorMsg_args(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    b"DISTINCT aggregates must have exactly one argument\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
                (*pFunc).iDistinct = -(1 as ::core::ffi::c_int);
            } else {
                let pKeyInfo: *mut crate::src::headers::sqliteInt_h::KeyInfo =
                    sqlite3KeyInfoFromExprList(
                        pParse,
                        (*pE).x.pList,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                let __pFunc_ref = unsafe { &mut *pFunc };
                __pFunc_ref.iDistAddr = crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                    v,
                    crate::src::headers::opcodes_h::OP_OpenEphemeral,
                    __pFunc_ref.iDistinct,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    pKeyInfo as *mut ::core::ffi::c_char,
                    crate::src::src::vdbe::P4_KEYINFO,
                );
                crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    0_u8,
                    b"USE TEMP B-TREE FOR %s(DISTINCT)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[crate::src::src::printf::PrintfArg::Str(
                        (*__pFunc_ref.pFunc).zName as *mut ::core::ffi::c_char,
                    )],
                );
            }
        }
        if (*pFunc).iOBTab >= 0 as ::core::ffi::c_int {
            
            
            let mut nExtra: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let __pFunc_ref = unsafe { &*pFunc };
            let pOBList: *mut crate::src::headers::sqliteInt_h::ExprList = (*(*__pFunc_ref.pFExpr).pLeft).x.pList;
            if __pFunc_ref.bOBUnique == 0 {
                nExtra += 1;
            }
            if __pFunc_ref.bOBPayload != 0 {
                nExtra += (*(*__pFunc_ref.pFExpr).x.pList).nExpr;
            }
            if __pFunc_ref.bUseSubtype != 0 {
                nExtra += (*(*__pFunc_ref.pFExpr).x.pList).nExpr;
            }
            let pKeyInfo_0: *mut crate::src::headers::sqliteInt_h::KeyInfo = sqlite3KeyInfoFromExprList(pParse, pOBList, 0 as ::core::ffi::c_int, nExtra);
            if __pFunc_ref.bOBUnique == 0 && (*pParse).nErr == 0 as ::core::ffi::c_int {
                (*pKeyInfo_0).nKeyField = (*pKeyInfo_0).nKeyField.wrapping_add(1);
            }
            crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                v,
                crate::src::headers::opcodes_h::OP_OpenEphemeral,
                __pFunc_ref.iOBTab,
                (*pOBList).nExpr + nExtra,
                0 as ::core::ffi::c_int,
                pKeyInfo_0 as *mut ::core::ffi::c_char,
                crate::src::src::vdbe::P4_KEYINFO,
            );
            crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                0_u8,
                b"USE TEMP B-TREE FOR %s(ORDER BY)\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::src::src::printf::PrintfArg::Str(
                    (*__pFunc_ref.pFunc).zName as *mut ::core::ffi::c_char,
                )],
            );
        }
        i += 1;
        pFunc = pFunc.offset(1);
    }
}

unsafe extern "C" fn finalizeAggFunctions(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo,
) {
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    let mut i: ::core::ffi::c_int;
    let mut pF: *mut crate::src::headers::sqliteInt_h::AggInfo_func;
    i = 0 as ::core::ffi::c_int;
    pF = (*pAggInfo).aFunc as *mut crate::src::headers::sqliteInt_h::AggInfo_func;
    while i < (*pAggInfo).nFunc {
        
        if (*pParse).nErr != 0 {
            return;
        }
        let pList: *mut crate::src::headers::sqliteInt_h::ExprList = (*(*pF).pFExpr).x.pList;
        if (*pF).iOBTab >= 0 as ::core::ffi::c_int {
            
            
            let mut nKey: ::core::ffi::c_int;
            
            let mut j: ::core::ffi::c_int;
            let nArg: ::core::ffi::c_int = (*pList).nExpr;
            let regAgg: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempRange(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                nArg,
            );
            let __pF_ref = unsafe { &*pF };
            if __pF_ref.bOBPayload as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                nKey = 0 as ::core::ffi::c_int;
            } else {
                nKey = (*(*(*__pF_ref.pFExpr).pLeft).x.pList).nExpr;
                if __pF_ref.bOBUnique == 0 {
                    nKey += 1;
                }
            }
            let iTop: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                v,
                crate::src::headers::opcodes_h::OP_Rewind,
                __pF_ref.iOBTab,
            );
            j = nArg - 1 as ::core::ffi::c_int;
            while j >= 0 as ::core::ffi::c_int {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                    v,
                    crate::src::headers::opcodes_h::OP_Column,
                    __pF_ref.iOBTab,
                    nKey + j,
                    regAgg + j,
                );
                j -= 1;
            }
            if __pF_ref.bUseSubtype != 0 {
                let regSubtype: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempReg(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                );
                let iBaseCol: ::core::ffi::c_int = nKey
                    + nArg
                    + (__pF_ref.bOBPayload as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && __pF_ref.bOBUnique as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                j = nArg - 1 as ::core::ffi::c_int;
                while j >= 0 as ::core::ffi::c_int {
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                        v,
                        crate::src::headers::opcodes_h::OP_Column,
                        __pF_ref.iOBTab,
                        iBaseCol + j,
                        regSubtype,
                    );
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                        v,
                        crate::src::headers::opcodes_h::OP_SetSubtype,
                        regSubtype,
                        regAgg + j,
                    );
                    j -= 1;
                }
                crate::src::src::expr::sqlite3ReleaseTempReg(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    regSubtype,
                );
            }
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_AggStep,
                0 as ::core::ffi::c_int,
                regAgg,
                (*pAggInfo).iFirstReg + (*pAggInfo).nColumn + i,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAppendP4(
                v,
                __pF_ref.pFunc as *mut ::core::ffi::c_void,
                crate::src::src::vdbe::P4_FUNCDEF,
            );
            crate::src::src::vdbeaux::sqlite3VdbeChangeP5(v, nArg as crate::src::fts5::U16_0);
            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                v,
                crate::src::headers::opcodes_h::OP_Next,
                __pF_ref.iOBTab,
                iTop + 1 as ::core::ffi::c_int,
            );
            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, iTop);
            crate::src::src::expr::sqlite3ReleaseTempRange(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                regAgg,
                nArg,
            );
        }
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::src::headers::opcodes_h::OP_AggFinal,
            (*pAggInfo).iFirstReg + (*pAggInfo).nColumn + i,
            if !pList.is_null() {
                (*pList).nExpr
            } else {
                0 as ::core::ffi::c_int
            },
        );
        crate::src::src::vdbeaux::sqlite3VdbeAppendP4(
            v,
            (*pF).pFunc as *mut ::core::ffi::c_void,
            crate::src::src::vdbe::P4_FUNCDEF,
        );
        i += 1;
        pF = pF.offset(1);
    }
}

unsafe extern "C" fn updateAccumulator(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    regAcc: ::core::ffi::c_int,
    pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo,
    eDistinctType: ::core::ffi::c_int,
) {
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = (*pParse).pVdbe;
    let mut i: ::core::ffi::c_int;
    let mut regHit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut addrHitTest: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pF: *mut crate::src::headers::sqliteInt_h::AggInfo_func;
    let mut pC: *mut crate::src::headers::sqliteInt_h::AggInfo_col;
    if (*pParse).nErr != 0 {
        return;
    }
    let __pAggInfo_ref = unsafe { &mut *pAggInfo };
    __pAggInfo_ref.directMode = 1_u8;
    i = 0 as ::core::ffi::c_int;
    pF = __pAggInfo_ref.aFunc as *mut crate::src::headers::sqliteInt_h::AggInfo_func;
    while i < __pAggInfo_ref.nFunc {
        let nArg: ::core::ffi::c_int;
        let mut addrNext: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let regAgg: ::core::ffi::c_int;
        let mut regAggSz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut regDistinct: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        
        let pList: *mut crate::src::headers::sqliteInt_h::ExprList = (*(*pF).pFExpr).x.pList;
        if (*(*pF).pFExpr).flags
            & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
        {
            let pFilter: *mut crate::src::headers::sqliteInt_h::Expr =
                (*(*(*pF).pFExpr).y.pWin).pFilter;
            if __pAggInfo_ref.nAccumulator != 0
                && (*(*pF).pFunc).funcFlags
                    & crate::src::headers::sqliteInt_h::SQLITE_FUNC_NEEDCOLL
                        as crate::src::ext::rtree::rtree::U32_0
                    != 0
                && regAcc != 0
            {
                if regHit == 0 as ::core::ffi::c_int {
                    (*pParse).nMem += 1;
                    regHit = (*pParse).nMem;
                }
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Copy,
                    regAcc,
                    regHit,
                );
            }
            addrNext = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            );
            crate::src::src::expr::sqlite3ExprIfFalse(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                pFilter as *mut crate::src::headers::sqliteInt_h::Expr,
                addrNext,
                crate::src::headers::sqliteInt_h::SQLITE_JUMPIFNULL,
            );
        }
        if (*pF).iOBTab >= 0 as ::core::ffi::c_int {
            let mut jj: ::core::ffi::c_int;
            
            nArg = (*pList).nExpr;
            let __pF_ref = unsafe { &*pF };
            let pOBList: *mut crate::src::headers::sqliteInt_h::ExprList = (*(*__pF_ref.pFExpr).pLeft).x.pList;
            regAggSz = (*pOBList).nExpr;
            if __pF_ref.bOBUnique == 0 {
                regAggSz += 1;
            }
            if __pF_ref.bOBPayload != 0 {
                regAggSz += nArg;
            }
            if __pF_ref.bUseSubtype != 0 {
                regAggSz += nArg;
            }
            regAggSz += 1;
            regAgg = crate::src::src::expr::sqlite3GetTempRange(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                regAggSz,
            );
            regDistinct = regAgg;
            crate::src::src::expr::sqlite3ExprCodeExprList(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                pOBList as *mut crate::src::headers::sqliteInt_h::ExprList,
                regAgg,
                0 as ::core::ffi::c_int,
                crate::src::headers::sqliteInt_h::SQLITE_ECEL_DUP as u8,
            );
            jj = (*pOBList).nExpr;
            if __pF_ref.bOBUnique == 0 {
                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                    v,
                    crate::src::headers::opcodes_h::OP_Sequence,
                    __pF_ref.iOBTab,
                    regAgg + jj,
                );
                jj += 1;
            }
            if __pF_ref.bOBPayload != 0 {
                regDistinct = regAgg + jj;
                crate::src::src::expr::sqlite3ExprCodeExprList(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    pList as *mut crate::src::headers::sqliteInt_h::ExprList,
                    regDistinct,
                    0 as ::core::ffi::c_int,
                    crate::src::headers::sqliteInt_h::SQLITE_ECEL_DUP as u8,
                );
                jj += nArg;
            }
            if __pF_ref.bUseSubtype != 0 {
                let mut kk: ::core::ffi::c_int;
                let regBase: ::core::ffi::c_int =
                    if __pF_ref.bOBPayload as ::core::ffi::c_int != 0 {
                        regDistinct
                    } else {
                        regAgg
                    };
                kk = 0 as ::core::ffi::c_int;
                while kk < nArg {
                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                        v,
                        crate::src::headers::opcodes_h::OP_GetSubtype,
                        regBase + kk,
                        regAgg + jj,
                    );
                    kk += 1;
                    jj += 1;
                }
            }
        } else if !pList.is_null() {
            nArg = (*pList).nExpr;
            regAgg = crate::src::src::expr::sqlite3GetTempRange(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                nArg,
            );
            regDistinct = regAgg;
            crate::src::src::expr::sqlite3ExprCodeExprList(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                pList as *mut crate::src::headers::sqliteInt_h::ExprList,
                regAgg,
                0 as ::core::ffi::c_int,
                crate::src::headers::sqliteInt_h::SQLITE_ECEL_DUP as u8,
            );
        } else {
            nArg = 0 as ::core::ffi::c_int;
            regAgg = 0 as ::core::ffi::c_int;
        }
        if (*pF).iDistinct >= 0 as ::core::ffi::c_int && !pList.is_null() {
            if addrNext == 0 as ::core::ffi::c_int {
                addrNext = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                );
            }
            (*pF).iDistinct = codeDistinct(
                pParse,
                eDistinctType,
                (*pF).iDistinct,
                addrNext,
                pList,
                regDistinct,
            );
        }
        if (*pF).iOBTab >= 0 as ::core::ffi::c_int {
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_MakeRecord,
                regAgg,
                regAggSz - 1 as ::core::ffi::c_int,
                regAgg + regAggSz - 1 as ::core::ffi::c_int,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                v,
                crate::src::headers::opcodes_h::OP_IdxInsert,
                (*pF).iOBTab,
                regAgg + regAggSz - 1 as ::core::ffi::c_int,
                regAgg,
                regAggSz - 1 as ::core::ffi::c_int,
            );
            crate::src::src::expr::sqlite3ReleaseTempRange(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                regAgg,
                regAggSz,
            );
        } else {
            if (*(*pF).pFunc).funcFlags
                & crate::src::headers::sqliteInt_h::SQLITE_FUNC_NEEDCOLL
                    as crate::src::ext::rtree::rtree::U32_0
                != 0
            {
                let mut pColl: *mut crate::src::headers::sqliteInt_h::CollSeq =
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::CollSeq>();
                let mut pItem: *mut crate::src::headers::sqliteInt_h::ExprList_item;
                let mut j: ::core::ffi::c_int;
                j = 0 as ::core::ffi::c_int;
                pItem = &raw mut (*pList).a as *mut crate::src::headers::sqliteInt_h::ExprList_item
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item;
                while pColl.is_null() && j < nArg {
                    pColl = crate::src::src::expr::sqlite3ExprCollSeq(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        (*pItem).pExpr as *const crate::src::headers::sqliteInt_h::Expr,
                    ) as *mut crate::src::headers::sqliteInt_h::CollSeq;
                    j += 1;
                    pItem = pItem.offset(1);
                }
                if pColl.is_null() {
                    pColl = (*(*pParse).db).pDfltColl;
                }
                if regHit == 0 as ::core::ffi::c_int && __pAggInfo_ref.nAccumulator != 0 {
                    (*pParse).nMem += 1;
                    regHit = (*pParse).nMem;
                }
                crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                    v,
                    crate::src::headers::opcodes_h::OP_CollSeq,
                    regHit,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    pColl as *mut ::core::ffi::c_char,
                    crate::src::src::vdbe::P4_COLLSEQ,
                );
            }
            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                v,
                crate::src::headers::opcodes_h::OP_AggStep,
                0 as ::core::ffi::c_int,
                regAgg,
                __pAggInfo_ref.iFirstReg + __pAggInfo_ref.nColumn + i,
            );
            crate::src::src::vdbeaux::sqlite3VdbeAppendP4(
                v,
                (*pF).pFunc as *mut ::core::ffi::c_void,
                crate::src::src::vdbe::P4_FUNCDEF,
            );
            crate::src::src::vdbeaux::sqlite3VdbeChangeP5(v, nArg as crate::src::fts5::U16_0);
            crate::src::src::expr::sqlite3ReleaseTempRange(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                regAgg,
                nArg,
            );
        }
        if addrNext != 0 {
            crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, addrNext);
        }
        if (*pParse).nErr != 0 {
            return;
        }
        i += 1;
        pF = pF.offset(1);
    }
    if regHit == 0 as ::core::ffi::c_int && __pAggInfo_ref.nAccumulator != 0 {
        regHit = regAcc;
    }
    if regHit != 0 {
        addrHitTest = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
            v,
            crate::src::headers::opcodes_h::OP_If,
            regHit,
        );
    }
    i = 0 as ::core::ffi::c_int;
    pC = __pAggInfo_ref.aCol as *mut crate::src::headers::sqliteInt_h::AggInfo_col;
    while i < __pAggInfo_ref.nAccumulator {
        crate::src::src::expr::sqlite3ExprCode(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            (*pC).pCExpr as *mut crate::src::headers::sqliteInt_h::Expr,
            __pAggInfo_ref.iFirstReg + i,
        );
        if (*pParse).nErr != 0 {
            return;
        }
        i += 1;
        pC = pC.offset(1);
    }
    __pAggInfo_ref.directMode = 0_u8;
    if addrHitTest != 0 {
        crate::src::src::vdbeaux::sqlite3VdbeJumpHereOrPopInst(v, addrHitTest);
    }
}

unsafe extern "C" fn explainSimpleCount(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pTab: *mut crate::src::headers::sqliteInt_h::Table,
    pIdx: *mut crate::src::headers::sqliteInt_h::Index,
) {
    if (*pParse).explain as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        let bCover: ::core::ffi::c_int = (!pIdx.is_null()
            && ((*pTab).tabFlags
                & crate::src::headers::sqliteInt_h::TF_WithoutRowid
                    as crate::src::ext::rtree::rtree::U32_0
                == 0 as crate::src::ext::rtree::rtree::U32_0
                || ((*pIdx).idxType() as ::core::ffi::c_int != crate::src::headers::sqliteInt_h::SQLITE_IDXTYPE_PRIMARYKEY)))
            as ::core::ffi::c_int;
        crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            0_u8,
            b"SCAN %s%s%s\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::src::src::printf::PrintfArg::Str((*pTab).zName as *mut ::core::ffi::c_char),
                crate::src::src::printf::PrintfArg::Str(if bCover != 0 {
                    b" USING COVERING INDEX \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                }
                    as *mut ::core::ffi::c_char),
                crate::src::src::printf::PrintfArg::Str(if bCover != 0 {
                    (*pIdx).zName as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                }
                    as *mut ::core::ffi::c_char),
            ],
        );
    }
}

unsafe extern "C" fn havingToWhereExprCb(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    if (*pExpr).op as ::core::ffi::c_int != crate::src::parse::TK_AND {
        let pS: *mut crate::src::headers::sqliteInt_h::Select = (*pWalker).u.pSelect;
        if crate::src::src::expr::sqlite3ExprIsConstantOrGroupBy(
            (*pWalker).pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
            (*pS).pGroupBy as *mut crate::src::headers::sqliteInt_h::ExprList,
        ) != 0
            && ((*pExpr).flags
                & (crate::src::headers::sqliteInt_h::EP_OuterON
                    | crate::src::headers::sqliteInt_h::EP_IsFalse)
                    as crate::src::ext::rtree::rtree::U32_0
                == crate::src::headers::sqliteInt_h::EP_IsFalse
                    as crate::src::ext::rtree::rtree::U32_0) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            && (*pExpr).pAggInfo.is_null()
        {
            let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*(*pWalker).pParse).db;
            let mut pNew: *mut crate::src::headers::sqliteInt_h::Expr =
                crate::src::src::expr::sqlite3Expr(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    crate::src::parse::TK_INTEGER,
                    b"1\0" as *const u8 as *const ::core::ffi::c_char,
                ) as *mut crate::src::headers::sqliteInt_h::Expr;
            if !pNew.is_null() {
                let pWhere: *mut crate::src::headers::sqliteInt_h::Expr = (*pS).pWhere;
                core::ptr::swap(pNew, pExpr);
                pNew = crate::src::src::expr::sqlite3ExprAnd(
                    (*pWalker).pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
                    pNew as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::Expr;
                (*pS).pWhere = pNew;
                (*pWalker).eCode = 1 as crate::src::fts5::U16_0;
            }
        }
        return crate::src::headers::sqliteInt_h::WRC_Prune;
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}

unsafe extern "C" fn havingToWhere(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
) {
    let mut sWalker: crate::src::headers::sqliteInt_h::Walker = unsafe { ::core::mem::zeroed() };
    sWalker.pParse = pParse;
    sWalker.xExprCallback = Some(
        havingToWhereExprCb
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
    sWalker.u.pSelect = p;
    crate::src::src::walker::sqlite3WalkExpr(
        &raw mut sWalker as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        (*p).pHaving as *mut crate::src::headers::sqliteInt_h::Expr,
    );
}

unsafe extern "C" fn isSelfJoinView(
    pTabList: *mut crate::src::headers::sqliteInt_h::SrcList,
    pThis: *mut crate::src::headers::sqliteInt_h::SrcItem,
    mut iFirst: ::core::ffi::c_int,
    iEnd: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqliteInt_h::SrcItem {
    let mut pItem: *mut crate::src::headers::sqliteInt_h::SrcItem;
    
    let pSel: *mut crate::src::headers::sqliteInt_h::Select = (*(*pThis).u4.pSubq).pSelect;
    if (*pSel).selFlags
        & crate::src::headers::sqliteInt_h::SF_PushDown as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        return ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcItem>();
    }
    while iFirst < iEnd {
        
        let fresh16 = iFirst;
        iFirst += 1;
        pItem = (&raw mut (*pTabList).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
            .offset(fresh16 as isize)
            as *mut crate::src::headers::sqliteInt_h::SrcItem;
        if (*pItem).fg.isSubquery() == 0 {
            continue;
        }
        if (*pItem).fg.viaCoroutine() != 0 {
            continue;
        }
        if (*pItem).zName.is_null() {
            continue;
        }
        let __pSTab_ref = &*(*pItem).pSTab;
        if __pSTab_ref.pSchema != (*(*pThis).pSTab).pSchema {
            continue;
        }
        if crate::src::src::util::sqlite3_stricmp((*pItem).zName, (*pThis).zName)
            != 0 as ::core::ffi::c_int
        {
            continue;
        }
        let pS1: *mut crate::src::headers::sqliteInt_h::Select = (*(*pItem).u4.pSubq).pSelect;
        if __pSTab_ref.pSchema.is_null() && (*pSel).selId != (*pS1).selId {
            continue;
        }
        if (*pS1).selFlags
            & crate::src::headers::sqliteInt_h::SF_PushDown as crate::src::ext::rtree::rtree::U32_0
            != 0
        {
            continue;
        }
        return pItem;
    }
    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcItem>()
}

unsafe extern "C" fn agginfoFree(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pArg: *mut ::core::ffi::c_void,
) {
    let p: *mut crate::src::headers::sqliteInt_h::AggInfo =
        pArg as *mut crate::src::headers::sqliteInt_h::AggInfo;
    crate::src::src::malloc::sqlite3DbFree(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        (*p).aCol as *mut ::core::ffi::c_void,
    );
    crate::src::src::malloc::sqlite3DbFree(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        (*p).aFunc as *mut ::core::ffi::c_void,
    );
    crate::src::src::malloc::sqlite3DbFreeNN(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        p as *mut ::core::ffi::c_void,
    );
}

unsafe extern "C" fn countOfViewOptimization(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    let mut pSub: *mut crate::src::headers::sqliteInt_h::Select;
    let mut pPrior: *mut crate::src::headers::sqliteInt_h::Select;
    let mut pExpr: *mut crate::src::headers::sqliteInt_h::Expr;
    
    
    
    let __p_ref = unsafe { &mut *p };
    if __p_ref.selFlags
        & crate::src::headers::sqliteInt_h::SF_Aggregate as crate::src::ext::rtree::rtree::U32_0
        == 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*__p_ref.pEList).nExpr != 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if !__p_ref.pWhere.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !__p_ref.pHaving.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !__p_ref.pGroupBy.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !__p_ref.pOrderBy.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    pExpr = (*(&raw mut (*__p_ref.pEList).a
        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
        .offset(0_isize))
    .pExpr;
    if (*pExpr).op as ::core::ffi::c_int != crate::src::parse::TK_AGG_FUNCTION {
        return 0 as ::core::ffi::c_int;
    }
    if crate::src::src::util::sqlite3_stricmp(
        (*pExpr).u.zToken,
        b"count\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !(*pExpr).x.pList.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*__p_ref.pSrc).nSrc != 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*pExpr).flags & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return 0 as ::core::ffi::c_int;
    }
    let pFrom: *mut crate::src::headers::sqliteInt_h::SrcItem = &raw mut (*__p_ref.pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem;
    if (*pFrom).fg.isSubquery() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    pSub = (*(*pFrom).u4.pSubq).pSelect;
    if (*pSub).pPrior.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*pSub).selFlags
        & crate::src::headers::sqliteInt_h::SF_CopyCte as crate::src::ext::rtree::rtree::U32_0
        != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        if (*pSub).op as ::core::ffi::c_int != crate::src::parse::TK_ALL
            && !(*pSub).pPrior.is_null()
        {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pSub).pWhere.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if !(*pSub).pLimit.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        if (*pSub).selFlags
            & (crate::src::headers::sqliteInt_h::SF_Aggregate
                | crate::src::headers::sqliteInt_h::SF_Distinct)
                as crate::src::ext::rtree::rtree::U32_0
            != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        pSub = (*pSub).pPrior;
        if pSub.is_null() {
            break;
        }
    }
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    let pCount: *mut crate::src::headers::sqliteInt_h::Expr = pExpr;
    pExpr = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
    pSub = crate::src::src::build::sqlite3SubqueryDetach(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        pFrom as *mut crate::src::headers::sqliteInt_h::SrcItem,
    ) as *mut crate::src::headers::sqliteInt_h::Select;
    crate::src::src::build::sqlite3SrcListDelete(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        __p_ref.pSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
    );
    __p_ref.pSrc = crate::src::src::malloc::sqlite3DbMallocZero(
        (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        crate::src::headers::sqliteInt_h::SZ_SRCLIST_1 as crate::src::ext::rtree::rtree::U64_0,
    ) as *mut crate::src::headers::sqliteInt_h::SrcList;
    while !pSub.is_null() {
        let mut pTerm: *mut crate::src::headers::sqliteInt_h::Expr;
        pPrior = (*pSub).pPrior;
        (*pSub).pPrior = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
        (*pSub).pNext = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
        (*pSub).selFlags |=
            crate::src::headers::sqliteInt_h::SF_Aggregate as crate::src::ext::rtree::rtree::U32_0;
        (*pSub).selFlags &= !(crate::src::headers::sqliteInt_h::SF_Compound
            as crate::src::ext::rtree::rtree::U32_0);
        (*pSub).nSelectRow = 0 as crate::src::headers::sqliteInt_h::LogEst;
        crate::src::src::prepare::sqlite3ParserAddCleanup(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            ::core::mem::transmute(Some(
                crate::src::src::expr::sqlite3ExprListDeleteGeneric
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqliteInt_h::sqlite3,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            )),
            (*pSub).pEList as *mut ::core::ffi::c_void,
        );
        pTerm = if !pPrior.is_null() {
            crate::src::src::expr::sqlite3ExprDup(
                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                pCount as *const crate::src::headers::sqliteInt_h::Expr,
                0 as ::core::ffi::c_int,
            ) as *mut crate::src::headers::sqliteInt_h::Expr
        } else {
            pCount
        };
        (*pSub).pEList = crate::src::src::expr::sqlite3ExprListAppend(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                as *mut crate::src::headers::sqliteInt_h::ExprList,
            pTerm as *mut crate::src::headers::sqliteInt_h::Expr,
        ) as *mut crate::src::headers::sqliteInt_h::ExprList;
        pTerm = crate::src::src::expr::sqlite3PExpr(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            crate::src::parse::TK_SELECT_1,
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                as *mut crate::src::headers::sqliteInt_h::Expr,
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                as *mut crate::src::headers::sqliteInt_h::Expr,
        ) as *mut crate::src::headers::sqliteInt_h::Expr;
        crate::src::src::expr::sqlite3PExprAddSelect(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            pTerm as *mut crate::src::headers::sqliteInt_h::Expr,
            pSub as *mut crate::src::headers::sqliteInt_h::Select,
        );
        if pExpr.is_null() {
            pExpr = pTerm;
        } else {
            pExpr = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_PLUS,
                pTerm as *mut crate::src::headers::sqliteInt_h::Expr,
                pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
        }
        pSub = pPrior;
    }
    let fresh18 = &mut (*(&raw mut (*__p_ref.pEList).a
        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
        .offset(0_isize))
    .pExpr;
    *fresh18 = pExpr;
    __p_ref.selFlags &=
        !(crate::src::headers::sqliteInt_h::SF_Aggregate as crate::src::ext::rtree::rtree::U32_0);
    1 as ::core::ffi::c_int
}

unsafe extern "C" fn sameSrcAlias(
    p0: *mut crate::src::headers::sqliteInt_h::SrcItem,
    pSrc: *mut crate::src::headers::sqliteInt_h::SrcList,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*pSrc).nSrc {
        let p1: *mut crate::src::headers::sqliteInt_h::SrcItem =
            (&raw mut (*pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                .offset(i as isize) as *mut crate::src::headers::sqliteInt_h::SrcItem;
        if (p1 != p0) {
            let __p1_ref = unsafe { &mut *p1 };
            if (*p0).pSTab == __p1_ref.pSTab
                && 0 as ::core::ffi::c_int
                    == crate::src::src::util::sqlite3_stricmp((*p0).zAlias, __p1_ref.zAlias)
            {
                return 1 as ::core::ffi::c_int;
            }
            if __p1_ref.fg.isSubquery() as ::core::ffi::c_int != 0
                && (*(*__p1_ref.u4.pSubq).pSelect).selFlags
                    & crate::src::headers::sqliteInt_h::SF_NestedFrom
                        as crate::src::ext::rtree::rtree::U32_0
                    != 0 as crate::src::ext::rtree::rtree::U32_0
                && sameSrcAlias(p0, (*(*__p1_ref.u4.pSubq).pSelect).pSrc) != 0
            {
                return 1 as ::core::ffi::c_int;
            }
        }
        i += 1;
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn fromClauseTermCanBeCoroutine(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pTabList: *mut crate::src::headers::sqliteInt_h::SrcList,
    mut i: ::core::ffi::c_int,
    selFlags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pTabList_ref = unsafe { &mut *pTabList };
    let mut pItem: *mut crate::src::headers::sqliteInt_h::SrcItem =
        (&raw mut __pTabList_ref.a as *mut crate::src::headers::sqliteInt_h::SrcItem)
            .offset(i as isize) as *mut crate::src::headers::sqliteInt_h::SrcItem;
    if (*pItem).fg.isCte() != 0 {
        let pCteUse: *const crate::src::headers::sqliteInt_h::CteUse = (*pItem).u2.pCteUse;
        let __pCteUse_ref = unsafe { &*pCteUse };
        if __pCteUse_ref.eM10d as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::M10d_Yes {
            return 0 as ::core::ffi::c_int;
        }
        if __pCteUse_ref.nUse >= 2 as ::core::ffi::c_int
            && __pCteUse_ref.eM10d as ::core::ffi::c_int
                != crate::src::headers::sqliteInt_h::M10d_No
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*(&raw mut __pTabList_ref.a as *mut crate::src::headers::sqliteInt_h::SrcItem)
        .offset(0_isize))
    .fg
    .jointype as ::core::ffi::c_int
        & crate::src::headers::sqliteInt_h::JT_LTORJ
        != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*(*pParse).db).dbOptFlags & 0x2000000 as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !isSelfJoinView(
        pTabList,
        pItem,
        i + 1 as ::core::ffi::c_int,
        __pTabList_ref.nSrc,
    )
    .is_null()
    {
        return 0 as ::core::ffi::c_int;
    }
    if i == 0 as ::core::ffi::c_int {
        if __pTabList_ref.nSrc == 1 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        if (*(&raw mut __pTabList_ref.a as *mut crate::src::headers::sqliteInt_h::SrcItem)
            .offset(1_isize))
        .fg
        .jointype as ::core::ffi::c_int
            & crate::src::headers::sqliteInt_h::JT_CROSS
            != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        if selFlags & crate::src::headers::sqliteInt_h::SF_UpdateFrom != 0 {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
    if selFlags & crate::src::headers::sqliteInt_h::SF_UpdateFrom != 0 {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        if (*pItem).fg.jointype as ::core::ffi::c_int
            & (crate::src::headers::sqliteInt_h::JT_OUTER
                | crate::src::headers::sqliteInt_h::JT_CROSS)
            != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if i == 0 as ::core::ffi::c_int {
            break;
        }
        i -= 1;
        pItem = pItem.offset(-1);
        if (*pItem).fg.isSubquery() != 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    1 as ::core::ffi::c_int
}
#[inline(never)]
unsafe extern "C" fn existsToJoin(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
    pWhere: *mut crate::src::headers::sqliteInt_h::Expr,
) {
    if (*pParse).nErr == 0 as ::core::ffi::c_int
        && !pWhere.is_null()
        && ((*pWhere).flags
            & (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int)
                as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
        && !(*p).pSrc.is_null()
        && (*(*p).pSrc).nSrc < crate::src::headers::sqliteInt_h::BMS
    {
        if (*pWhere).op as ::core::ffi::c_int == crate::src::parse::TK_AND {
            let pRight: *mut crate::src::headers::sqliteInt_h::Expr = (*pWhere).pRight;
            existsToJoin(pParse, p, (*pWhere).pLeft);
            existsToJoin(pParse, p, pRight);
        } else if (*pWhere).op as ::core::ffi::c_int == crate::src::parse::TK_EXISTS {
            let pSub: *mut crate::src::headers::sqliteInt_h::Select = (*pWhere).x.pSelect;
            let __pSub_ref = unsafe { &mut *pSub };
            let pSubWhere: *mut crate::src::headers::sqliteInt_h::Expr = __pSub_ref.pWhere;
            if (*__pSub_ref.pSrc).nSrc == 1 as ::core::ffi::c_int
                && __pSub_ref.selFlags
                    & crate::src::headers::sqliteInt_h::SF_Aggregate
                        as crate::src::ext::rtree::rtree::U32_0
                    == 0 as crate::src::ext::rtree::rtree::U32_0
                && (*(&raw mut (*__pSub_ref.pSrc).a
                    as *mut crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(0_isize))
                .fg
                .isSubquery()
                    == 0
                && __pSub_ref.pLimit.is_null()
                && __pSub_ref.pPrior.is_null()
            {
                let __pParse_ref = unsafe { &mut *pParse };
                let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
                let aCsrMap: *mut ::core::ffi::c_int =
                    crate::src::src::malloc::sqlite3DbMallocZero(
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        ((__pParse_ref.nTab + 2 as ::core::ffi::c_int) as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                            as crate::src::ext::rtree::rtree::U64_0,
                    ) as *mut ::core::ffi::c_int;
                if aCsrMap.is_null() {
                    return;
                }
                *aCsrMap.offset(0_isize) = __pParse_ref.nTab + 1 as ::core::ffi::c_int;
                renumberCursors(pParse, pSub, -(1 as ::core::ffi::c_int), aCsrMap);
                crate::src::src::malloc::sqlite3DbFree(
                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    aCsrMap as *mut ::core::ffi::c_void,
                );
                ::libc::memset(
                    pWhere as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Expr>()
                        as crate::__stddef_size_t_h::SizeT,
                );
                let __pWhere_ref = unsafe { &mut *pWhere };
                __pWhere_ref.op = crate::src::parse::TK_INTEGER as u8;
                __pWhere_ref.u.iValue = 1 as ::core::ffi::c_int;
                __pWhere_ref.flags |=
                    0x800 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0;
                let fresh21 = &mut (*(&raw mut (*__pSub_ref.pSrc).a
                    as *mut crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(0_isize))
                .fg;
                (*fresh21).set_fromExists(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                let fresh22 = &mut (*(&raw mut (*__pSub_ref.pSrc).a
                    as *mut crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(0_isize))
                .fg
                .jointype;
                *fresh22 = (*fresh22 as ::core::ffi::c_int
                    | crate::src::headers::sqliteInt_h::JT_CROSS) as u8;
                (*p).pSrc = crate::src::src::build::sqlite3SrcListAppendList(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*p).pSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
                    __pSub_ref.pSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
                ) as *mut crate::src::headers::sqliteInt_h::SrcList;
                if !pSubWhere.is_null() {
                    (*p).pWhere = crate::src::src::expr::sqlite3PExpr(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        crate::src::parse::TK_AND,
                        (*p).pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
                        pSubWhere as *mut crate::src::headers::sqliteInt_h::Expr,
                    )
                        as *mut crate::src::headers::sqliteInt_h::Expr;
                    __pSub_ref.pWhere =
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
                }
                __pSub_ref.pSrc =
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>();
                crate::src::src::prepare::sqlite3ParserAddCleanup(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    ::core::mem::transmute(Some(
                        sqlite3SelectDeleteGeneric
                            as unsafe extern "C" fn(
                                *mut crate::src::headers::sqliteInt_h::sqlite3,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    )),
                    pSub as *mut ::core::ffi::c_void,
                );
                existsToJoin(pParse, p, pSubWhere);
            }
        }
    }
}

unsafe extern "C" fn selectCheckOnClausesExpr(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
) -> ::core::ffi::c_int {
    let mut pCtx: *mut CheckOnCtx = (*pWalker).u.pCheckOnCtx as *mut CheckOnCtx;
    let __pExpr_ref = unsafe { &*pExpr };
    if (__pExpr_ref.flags & 0x1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
        != 0 as crate::src::ext::rtree::rtree::U32_0
        || __pExpr_ref.flags & 0x2 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0
            != 0 as crate::src::ext::rtree::rtree::U32_0
            && (*(&raw mut (*(*pCtx).pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                .offset(0_isize))
            .fg
            .jointype as ::core::ffi::c_int
                & crate::src::headers::sqliteInt_h::JT_LTORJ
                != 0 as ::core::ffi::c_int)
        && (*pCtx).iJoin == 0 as ::core::ffi::c_int {
            (*pCtx).iJoin = __pExpr_ref.w.iJoin;
            crate::src::src::walker::sqlite3WalkExprNN(
                pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
                pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
            );
            (*pCtx).iJoin = 0 as ::core::ffi::c_int;
            return crate::src::headers::sqliteInt_h::WRC_Prune;
        }
    if __pExpr_ref.op as ::core::ffi::c_int == crate::src::parse::TK_COLUMN_1 {
        loop {
            let pSrc: *mut crate::src::headers::sqliteInt_h::SrcList = (*pCtx).pSrc;
            let iTab: ::core::ffi::c_int = __pExpr_ref.iTable;
            let __pSrc_ref = unsafe { &mut *pSrc };
            if iTab
                >= (*(&raw mut __pSrc_ref.a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(0_isize))
                .iCursor
                && iTab
                    <= (*(&raw mut __pSrc_ref.a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                        .offset((__pSrc_ref.nSrc - 1 as ::core::ffi::c_int) as isize))
                    .iCursor
            {
                if (*pCtx).iJoin != 0 && iTab > (*pCtx).iJoin {
                    crate::src::src::util::sqlite3ErrorMsg_args(
                        (*pWalker).pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        b"ON clause references tables to its right\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                    return crate::src::headers::sqliteInt_h::WRC_Abort;
                }
                break;
            } else {
                pCtx = (*pCtx).pParent;
                if pCtx.is_null() {
                    break;
                }
            }
        }
    }
    crate::src::headers::sqliteInt_h::WRC_Continue
}

unsafe extern "C" fn selectCheckOnClausesSelect(
    pWalker: *mut crate::src::headers::sqliteInt_h::Walker,
    pSelect: *mut crate::src::headers::sqliteInt_h::Select,
) -> ::core::ffi::c_int {
    let pCtx: *mut CheckOnCtx = (*pWalker).u.pCheckOnCtx as *mut CheckOnCtx;
    if (*pSelect).pSrc == (*pCtx).pSrc || (*(*pSelect).pSrc).nSrc == 0 as ::core::ffi::c_int {
        crate::src::headers::sqliteInt_h::WRC_Continue
    } else {
        let mut sCtx: CheckOnCtx = unsafe { ::core::mem::zeroed() };
        sCtx.pSrc = (*pSelect).pSrc;
        sCtx.pParent = pCtx;
        (*pWalker).u.pCheckOnCtx = &raw mut sCtx as *mut CheckOnCtx;
        crate::src::src::walker::sqlite3WalkSelect(
            pWalker as *mut crate::src::headers::sqliteInt_h::Walker,
            pSelect as *mut crate::src::headers::sqliteInt_h::Select,
        );
        (*pWalker).u.pCheckOnCtx = pCtx as *mut CheckOnCtx;
        (*pSelect).selFlags &=
            !crate::src::headers::sqliteInt_h::SF_OnToWhere as crate::src::ext::rtree::rtree::U32_0;
        crate::src::headers::sqliteInt_h::WRC_Prune
    }
}

unsafe extern "C" fn selectCheckOnClauses(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pSelect: *mut crate::src::headers::sqliteInt_h::Select,
) {
    let mut w: crate::src::headers::sqliteInt_h::Walker = unsafe { ::core::mem::zeroed() };
    let mut sCtx: CheckOnCtx = unsafe { ::core::mem::zeroed() };
    w.pParse = pParse;
    w.xExprCallback = Some(
        selectCheckOnClausesExpr
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
        selectCheckOnClausesSelect
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
    w.u.pCheckOnCtx = &raw mut sCtx as *mut CheckOnCtx;
    let __pSelect_ref = unsafe { &mut *pSelect };
    crate::src::src::walker::sqlite3WalkExprNN(
        &raw mut w as *mut _ as *mut crate::src::headers::sqliteInt_h::Walker,
        __pSelect_ref.pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
    );
    __pSelect_ref.selFlags &=
        !crate::src::headers::sqliteInt_h::SF_OnToWhere as crate::src::ext::rtree::rtree::U32_0;
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3Select(
    pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    p: *mut crate::src::headers::sqliteInt_h::Select,
    pDest: *mut crate::src::headers::sqliteInt_h::SelectDest,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    let pWInfo: *mut crate::src::headers::whereInt_h::WhereInfo;
    
    let isAgg: ::core::ffi::c_int;
    let pEList: *mut crate::src::headers::sqliteInt_h::ExprList;
    let mut pTabList: *mut crate::src::headers::sqliteInt_h::SrcList;
    let mut pWhere: *mut crate::src::headers::sqliteInt_h::Expr;
    let mut pGroupBy: *mut crate::src::headers::sqliteInt_h::ExprList;
    let pHaving: *mut crate::src::headers::sqliteInt_h::Expr;
    let pAggInfo: *mut crate::src::headers::sqliteInt_h::AggInfo;
    let mut rc: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut sDistinct: DistinctCtx = unsafe { ::core::mem::zeroed() };
    let mut sSort: SortCtx = unsafe { ::core::mem::zeroed() };
    let iEnd: ::core::ffi::c_int;
    
    let mut pMinMaxOrderBy: *mut crate::src::headers::sqliteInt_h::ExprList =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
    let minMaxFlag: crate::src::ext::rtree::rtree::U8_0;
    let __pParse_ref = unsafe { &mut *pParse };
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
    let v: *mut crate::src::headers::vdbeInt_h::Vdbe = sqlite3GetVdbe(pParse);
    if p.is_null() || __pParse_ref.nErr != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if crate::src::src::auth::sqlite3AuthCheck(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        crate::src::headers::sqlite3_h::SQLITE_SELECT,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ) != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    if (*pDest).eDest as ::core::ffi::c_int <= crate::src::headers::sqliteInt_h::SRT_DistQueue {
        let __p_ref = unsafe { &mut *p };
        if !__p_ref.pOrderBy.is_null() {
            crate::src::src::prepare::sqlite3ParserAddCleanup(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::mem::transmute(Some(
                    crate::src::src::expr::sqlite3ExprListDeleteGeneric
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqliteInt_h::sqlite3,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                )),
                __p_ref.pOrderBy as *mut ::core::ffi::c_void,
            );
            __p_ref.pOrderBy =
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
        }
        __p_ref.selFlags &= !(crate::src::headers::sqliteInt_h::SF_Distinct
            as crate::src::ext::rtree::rtree::U32_0);
        __p_ref.selFlags |= crate::src::headers::sqliteInt_h::SF_NoopOrderBy
            as crate::src::ext::rtree::rtree::U32_0;
    }
    sqlite3SelectPrep(
        pParse,
        p,
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::NameContext>(),
    );
    if (__pParse_ref.nErr == 0) {
        if (*p).selFlags
            & crate::src::headers::sqliteInt_h::SF_OnToWhere as crate::src::ext::rtree::rtree::U32_0
            != 0
        {
            selectCheckOnClauses(pParse, p);
            if __pParse_ref.nErr != 0 {
                current_block = 9427106068226466434;
            } else {
                current_block = 1608152415753874203;
            }
        } else {
            current_block = 1608152415753874203;
        }
        match current_block {
            9427106068226466434 => {}
            _ => {
                if (*p).selFlags
                    & crate::src::headers::sqliteInt_h::SF_UFSrcCheck
                        as crate::src::ext::rtree::rtree::U32_0
                    != 0
                {
                    let p0: *mut crate::src::headers::sqliteInt_h::SrcItem =
                        (&raw mut (*(*p).pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                            .offset(0_isize)
                            as *mut crate::src::headers::sqliteInt_h::SrcItem;
                    if sameSrcAlias(p0, (*p).pSrc) != 0 {
                        crate::src::src::util::sqlite3ErrorMsg_args(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            b"target object/alias may not appear in FROM clause: %s\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[crate::src::src::printf::PrintfArg::Str(
                                if !(*p0).zAlias.is_null() {
                                    (*p0).zAlias
                                } else {
                                    (*(*p0).pSTab).zName
                                } as *mut ::core::ffi::c_char,
                            )],
                        );
                        current_block = 9427106068226466434;
                    } else {
                        (*p).selFlags &= !(crate::src::headers::sqliteInt_h::SF_UFSrcCheck
                            as crate::src::ext::rtree::rtree::U32_0);
                        current_block = 11913429853522160501;
                    }
                } else {
                    current_block = 11913429853522160501;
                }
                match current_block {
                    9427106068226466434 => {}
                    _ => {
                        if (*pDest).eDest as ::core::ffi::c_int
                            == crate::src::headers::sqliteInt_h::SRT_Output
                        {
                            sqlite3GenerateColumnNames(pParse, p);
                        }
                        if (crate::src::src::window::sqlite3WindowRewrite(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            p as *mut crate::src::headers::sqliteInt_h::Select,
                        ) == 0)
                        {
                            let __p_ref = unsafe { &mut *p };
                            pTabList = __p_ref.pSrc;
                            isAgg = (__p_ref.selFlags
                                & crate::src::headers::sqliteInt_h::SF_Aggregate
                                    as crate::src::ext::rtree::rtree::U32_0
                                != 0 as crate::src::ext::rtree::rtree::U32_0)
                                as ::core::ffi::c_int;
                            sSort.pOrderBy = __p_ref.pOrderBy;
                            i = 0 as ::core::ffi::c_int;
                            loop {
                                if !(__p_ref.pPrior.is_null() && i < (*pTabList).nSrc) {
                                    current_block = 1852451392920375136;
                                    break;
                                }
                                let pItem: *mut crate::src::headers::sqliteInt_h::SrcItem =
                                    (&raw mut (*pTabList).a
                                        as *mut crate::src::headers::sqliteInt_h::SrcItem)
                                        .offset(i as isize)
                                        as *mut crate::src::headers::sqliteInt_h::SrcItem;
                                let __pItem_ref = unsafe { &mut *pItem };
                                let pSub: *mut crate::src::headers::sqliteInt_h::Select =
                                    if __pItem_ref.fg.isSubquery() as ::core::ffi::c_int != 0 {
                                        (*__pItem_ref.u4.pSubq).pSelect
                                    } else {
                                        ::core::ptr::null_mut::<
                                            crate::src::headers::sqliteInt_h::Select,
                                        >()
                                    };
                                let pTab: *mut crate::src::headers::sqliteInt_h::Table =
                                    __pItem_ref.pSTab;
                                if __pItem_ref.fg.jointype as ::core::ffi::c_int
                                    & (crate::src::headers::sqliteInt_h::JT_LEFT
                                        | crate::src::headers::sqliteInt_h::JT_LTORJ)
                                    != 0 as ::core::ffi::c_int
                                    && crate::src::src::expr::sqlite3ExprImpliesNonNullRow(
                                        __p_ref.pWhere
                                            as *mut crate::src::headers::sqliteInt_h::Expr,
                                        __pItem_ref.iCursor,
                                        __pItem_ref.fg.jointype as ::core::ffi::c_int
                                            & crate::src::headers::sqliteInt_h::JT_LTORJ,
                                    ) != 0
                                    && (*db).dbOptFlags
                                        & 0x2000 as crate::src::ext::rtree::rtree::U32_0
                                        == 0 as crate::src::ext::rtree::rtree::U32_0
                                {
                                    if __pItem_ref.fg.jointype as ::core::ffi::c_int
                                        & crate::src::headers::sqliteInt_h::JT_LEFT
                                        != 0
                                    {
                                        if __pItem_ref.fg.jointype as ::core::ffi::c_int
                                            & crate::src::headers::sqliteInt_h::JT_RIGHT
                                            != 0
                                        {
                                            __pItem_ref.fg.jointype = (__pItem_ref.fg.jointype
                                                as ::core::ffi::c_int
                                                & !crate::src::headers::sqliteInt_h::JT_LEFT)
                                                as u8;
                                        } else {
                                            __pItem_ref.fg.jointype = (__pItem_ref.fg.jointype
                                                as ::core::ffi::c_int
                                                & !(crate::src::headers::sqliteInt_h::JT_LEFT
                                                    | crate::src::headers::sqliteInt_h::JT_OUTER))
                                                as u8;
                                            unsetJoinExpr(
                                                __p_ref.pWhere,
                                                __pItem_ref.iCursor,
                                                0 as ::core::ffi::c_int,
                                            );
                                        }
                                    }
                                    if __pItem_ref.fg.jointype as ::core::ffi::c_int
                                        & crate::src::headers::sqliteInt_h::JT_LTORJ
                                        != 0
                                    {
                                        j = i + 1 as ::core::ffi::c_int;
                                        while j < (*pTabList).nSrc {
                                            let pI2: *mut crate::src::headers::sqliteInt_h::SrcItem = (&raw mut (*pTabList).a
                                                as *mut crate::src::headers::sqliteInt_h::SrcItem)
                                                .offset(j as isize)
                                                as *mut crate::src::headers::sqliteInt_h::SrcItem;
                                            if (*pI2).fg.jointype as ::core::ffi::c_int
                                                & crate::src::headers::sqliteInt_h::JT_RIGHT
                                                != 0
                                            {
                                                if (*pI2).fg.jointype as ::core::ffi::c_int
                                                    & crate::src::headers::sqliteInt_h::JT_LEFT
                                                    != 0
                                                {
                                                    (*pI2).fg.jointype = ((*pI2).fg.jointype
                                                        as ::core::ffi::c_int
                                                        & !crate::src::headers::sqliteInt_h::JT_RIGHT)
                                                        as u8;
                                                } else {
                                                    let __pI2_ref = unsafe { &mut *pI2 };
                                                    __pI2_ref.fg.jointype = (__pI2_ref.fg.jointype
                                                        as ::core::ffi::c_int
                                                        & !(crate::src::headers::sqliteInt_h::JT_RIGHT | crate::src::headers::sqliteInt_h::JT_OUTER))
                                                        as u8;
                                                    unsetJoinExpr(
                                                        __p_ref.pWhere,
                                                        __pI2_ref.iCursor,
                                                        1 as ::core::ffi::c_int,
                                                    );
                                                }
                                            }
                                            j += 1;
                                        }
                                        j = (*pTabList).nSrc - 1 as ::core::ffi::c_int;
                                        while j >= 0 as ::core::ffi::c_int {
                                            let fresh7 = &mut (*(&raw mut (*pTabList).a
                                                as *mut crate::src::headers::sqliteInt_h::SrcItem)
                                                .offset(j as isize))
                                            .fg
                                            .jointype;
                                            *fresh7 = (*fresh7 as ::core::ffi::c_int
                                                & !crate::src::headers::sqliteInt_h::JT_LTORJ)
                                                as u8;
                                            if (*(&raw mut (*pTabList).a
                                                as *mut crate::src::headers::sqliteInt_h::SrcItem)
                                                .offset(j as isize))
                                            .fg
                                            .jointype
                                                as ::core::ffi::c_int
                                                & crate::src::headers::sqliteInt_h::JT_RIGHT
                                                != 0
                                            {
                                                break;
                                            }
                                            j -= 1;
                                        }
                                    }
                                }
                                if !pSub.is_null() {
                                    if (*pTab).nCol as ::core::ffi::c_int != (*(*pSub).pEList).nExpr
                                    {
                                        crate::src::src::util::sqlite3ErrorMsg_args(
                                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                            b"expected %d columns for '%s' but got %d\0"
                                                as *const u8
                                                as *const ::core::ffi::c_char,
                                            &[
                                                crate::src::src::printf::PrintfArg::Int(
                                                    (*pTab).nCol
                                                        as crate::src::ext::rtree::rtree::I64_0,
                                                ),
                                                crate::src::src::printf::PrintfArg::Str(
                                                    (*pTab).zName as *mut ::core::ffi::c_char,
                                                ),
                                                crate::src::src::printf::PrintfArg::Int(
                                                    (*(*pSub).pEList).nExpr
                                                        as crate::src::ext::rtree::rtree::I64_0,
                                                ),
                                            ],
                                        );
                                        current_block = 9427106068226466434;
                                        break;
                                    } else if !(__pItem_ref.fg.isCte() as ::core::ffi::c_int != 0
                                        && (*__pItem_ref.u2.pCteUse).eM10d as ::core::ffi::c_int
                                            == crate::src::headers::sqliteInt_h::M10d_Yes)
                                        && ((*pSub).selFlags
                                            & crate::src::headers::sqliteInt_h::SF_Aggregate
                                                as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
                                        {
                                            let __pSub_ref = unsafe { &mut *pSub };
                                            if !__pSub_ref.pOrderBy.is_null()
                                                && (!__p_ref.pOrderBy.is_null()
                                                    || (*pTabList).nSrc > 1 as ::core::ffi::c_int)
                                                && __pSub_ref.pLimit.is_null()
                                                && __pSub_ref.selFlags
                                                    & (crate::src::headers::sqliteInt_h::SF_OrderByReqd | crate::src::headers::sqliteInt_h::SF_Recursive) as crate::src::ext::rtree::rtree::U32_0
                                                    == 0 as crate::src::ext::rtree::rtree::U32_0
                                                && __p_ref.selFlags & crate::src::headers::sqliteInt_h::SF_OrderByReqd as crate::src::ext::rtree::rtree::U32_0
                                                    == 0 as crate::src::ext::rtree::rtree::U32_0
                                                && (*db).dbOptFlags & 0x40000 as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0
                                            {
                                                crate::src::src::prepare::sqlite3ParserAddCleanup(
                                                    
                                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                    ::core::mem::transmute(
                                                        
                                                    Some(
                                                        crate::src::src::expr::sqlite3ExprListDeleteGeneric
                                                            as unsafe extern "C" fn(
                                                                *mut crate::src::headers::sqliteInt_h::sqlite3,
                                                                *mut ::core::ffi::c_void,
                                                            )
                                                                -> (),
                                                    ),
                                                    ),
                                                    __pSub_ref.pOrderBy as *mut ::core::ffi::c_void,
                                                );
                                                __pSub_ref.pOrderBy =
                                                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
                                            }
                                            if !(!__pSub_ref.pOrderBy.is_null()
                                                && i == 0 as ::core::ffi::c_int
                                                && __p_ref.selFlags & crate::src::headers::sqliteInt_h::SF_ComplexResult as crate::src::ext::rtree::rtree::U32_0
                                                    != 0 as crate::src::ext::rtree::rtree::U32_0
                                                && ((*pTabList).nSrc == 1 as ::core::ffi::c_int
                                                    || (*(&raw mut (*pTabList).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                                                        .offset(1_isize))
                                                    .fg
                                                    .jointype
                                                        as ::core::ffi::c_int
                                                        & (crate::src::headers::sqliteInt_h::JT_OUTER | crate::src::headers::sqliteInt_h::JT_CROSS)
                                                        != 0 as ::core::ffi::c_int))
                                            {
                                                if flattenSubquery(pParse, p, i, isAgg) != 0 {
                                                    if __pParse_ref.nErr != 0 {
                                                        current_block = 9427106068226466434;
                                                        break;
                                                    }
                                                    i = -(1 as ::core::ffi::c_int);
                                                }
                                                pTabList = __p_ref.pSrc;
                                                if (*db).mallocFailed != 0 {
                                                    current_block = 9427106068226466434;
                                                    break;
                                                }
                                                if ((*pDest).eDest as ::core::ffi::c_int > crate::src::headers::sqliteInt_h::SRT_Fifo)
                                                {
                                                    sSort.pOrderBy = __p_ref.pOrderBy;
                                                }
                                            }
                                        }
                                }
                                i += 1;
                            }
                            match current_block {
                                9427106068226466434 => {}
                                _ => {
                                    if !__p_ref.pPrior.is_null() {
                                        rc = multiSelect(pParse, p, pDest);
                                        if __p_ref.pNext.is_null() {
                                            crate::src::src::vdbeaux::sqlite3VdbeExplainPop(
                                                pParse
                                                    as *mut crate::src::headers::sqliteInt_h::Parse,
                                            );
                                        }
                                        return rc;
                                    }
                                    let __db_ref = unsafe { &mut *db };
                                    if __pParse_ref.bHasExists as ::core::ffi::c_int != 0
                                        && __db_ref.dbOptFlags
                                            & 0x40000000 as crate::src::ext::rtree::rtree::U32_0
                                            == 0 as crate::src::ext::rtree::rtree::U32_0
                                    {
                                        existsToJoin(pParse, p, __p_ref.pWhere);
                                        pTabList = __p_ref.pSrc;
                                    }
                                    if !__p_ref.pWhere.is_null()
                                        && (*__p_ref.pWhere).op as ::core::ffi::c_int
                                            == crate::src::parse::TK_AND && __db_ref.dbOptFlags
                                            & 0x8000 as crate::src::ext::rtree::rtree::U32_0
                                            == 0 as crate::src::ext::rtree::rtree::U32_0 { propagateConstants(pParse, p) != 0; }
                                    if __db_ref.dbOptFlags
                                        & (0x1 as ::core::ffi::c_int | 0x200 as ::core::ffi::c_int)
                                            as crate::src::ext::rtree::rtree::U32_0
                                        == 0 as crate::src::ext::rtree::rtree::U32_0
                                        && countOfViewOptimization(pParse, p) != 0
                                    {
                                        if __db_ref.mallocFailed != 0 {
                                            current_block = 9427106068226466434;
                                        } else {
                                            pTabList = __p_ref.pSrc;
                                            current_block = 2220405792722996547;
                                        }
                                    } else {
                                        current_block = 2220405792722996547;
                                    }
                                    match current_block {
                                        9427106068226466434 => {}
                                        _ => {
                                            i = 0 as ::core::ffi::c_int;
                                            loop {
                                                if (i >= (*pTabList).nSrc) {
                                                    current_block = 10996290961880923853;
                                                    break;
                                                }
                                                let pItem_0: *mut crate::src::headers::sqliteInt_h::SrcItem =
                                                    (&raw mut (*pTabList).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                                                        .offset(i as isize)
                                                        as *mut crate::src::headers::sqliteInt_h::SrcItem;
                                                let pPrior: *mut crate::src::headers::sqliteInt_h::SrcItem;
                                                let mut dest: crate::src::headers::sqliteInt_h::SelectDest = unsafe { ::core::mem::zeroed() };
                                                let pSubq: *mut crate::src::headers::sqliteInt_h::Subquery;
                                                let pSub_0: *mut crate::src::headers::sqliteInt_h::Select;
                                                let zSavedAuthContext: *const ::core::ffi::c_char;
                                                let __pItem_0_ref = unsafe { &mut *pItem_0 };
                                                if __pItem_0_ref.colUsed
                                                    == 0
                                                        as crate::src::headers::sqliteInt_h::Bitmask
                                                    && !__pItem_0_ref.zName.is_null()
                                                {
                                                    let zDb: *const ::core::ffi::c_char;
                                                    if __pItem_0_ref.fg.fixedSchema() != 0 {
                                                        let iDb: ::core::ffi::c_int =
                                                            crate::src::src::prepare::sqlite3SchemaToIndex(
                                                                
                                                                __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                                                
                                                                __pItem_0_ref.u4.pSchema as *mut crate::src::headers::sqliteInt_h::Schema,
                                                            );
                                                        zDb = (*__db_ref.aDb.offset(iDb as isize))
                                                            .zDbSName;
                                                    } else if __pItem_0_ref.fg.isSubquery() != 0 {
                                                        zDb = ::core::ptr::null::<
                                                            ::core::ffi::c_char,
                                                        >(
                                                        );
                                                    } else {
                                                        zDb = __pItem_0_ref.u4.zDatabase;
                                                    }
                                                    crate::src::src::auth::sqlite3AuthCheck(
                                                        
                                                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                        crate::src::headers::sqlite3_h::SQLITE_READ,
                                                        __pItem_0_ref.zName,
                                                        b"\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        zDb,
                                                    );
                                                }
                                                if (__pItem_0_ref.fg.isSubquery()
                                                    as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                                                {
                                                    pSubq = __pItem_0_ref.u4.pSubq;
                                                    pSub_0 = (*pSubq).pSelect;
                                                    if ((*pSubq).addrFillSub == 0 as ::core::ffi::c_int)
                                                    {
                                                        __pParse_ref.nHeight +=
                                                            crate::src::src::expr::sqlite3SelectExprHeight(p as *const crate::src::headers::sqliteInt_h::Select);
                                                        if __db_ref.dbOptFlags & 0x1000 as crate::src::ext::rtree::rtree::U32_0
                                                            == 0 as crate::src::ext::rtree::rtree::U32_0 && (__pItem_0_ref.fg.isCte()
                                                                as ::core::ffi::c_int
                                                                == 0 as ::core::ffi::c_int
                                                                || (*__pItem_0_ref.u2.pCteUse).eM10d
                                                                    as ::core::ffi::c_int
                                                                    != crate::src::headers::sqliteInt_h::M10d_Yes
                                                                    && (*__pItem_0_ref.u2.pCteUse)
                                                                        .nUse
                                                                        < 2 as ::core::ffi::c_int) { pushDownWhereTerms(
                                                                pParse,
                                                                pSub_0,
                                                                __p_ref.pWhere,
                                                                pTabList,
                                                                i,
                                                            ) != 0; }
                                                        if __db_ref.dbOptFlags & 0x4000000 as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0 { disableUnusedSubqueryResultColumns(
                                                                pItem_0,
                                                            ) != 0; }
                                                        zSavedAuthContext =
                                                            __pParse_ref.zAuthContext;
                                                        __pParse_ref.zAuthContext =
                                                            __pItem_0_ref.zName;
                                                        if fromClauseTermCanBeCoroutine(
                                                            pParse,
                                                            pTabList,
                                                            i,
                                                            __p_ref.selFlags as ::core::ffi::c_int,
                                                        ) != 0
                                                        {
                                                            let addrTop: ::core::ffi::c_int =
                                                                crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v)
                                                                    + 1 as ::core::ffi::c_int;
                                                            __pParse_ref.nMem += 1;
                                                            let __pSubq_ref =
                                                                unsafe { &mut *pSubq };
                                                            __pSubq_ref.regReturn =
                                                                __pParse_ref.nMem;
                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                v,
                                                                crate::src::headers::opcodes_h::OP_InitCoroutine,
                                                                __pSubq_ref.regReturn,
                                                                0 as ::core::ffi::c_int,
                                                                addrTop,
                                                            );
                                                            __pSubq_ref.addrFillSub = addrTop;
                                                            sqlite3SelectDestInit(
                                                                &raw mut dest,
                                                                crate::src::headers::sqliteInt_h::SRT_Coroutine,
                                                                __pSubq_ref.regReturn,
                                                            );
                                                            crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                                                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                1_u8,
                                                                b"CO-ROUTINE %!S\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                &[crate::src::src::printf::PrintfArg::SrcItem(pItem_0 as *mut crate::src::headers::sqliteInt_h::SrcItem)],
                                                            );
                                                            sqlite3Select(
                                                                pParse,
                                                                pSub_0,
                                                                &raw mut dest,
                                                            );
                                                            (*__pItem_0_ref.pSTab).nRowLogEst =
                                                                (*pSub_0).nSelectRow;
                                                            __pItem_0_ref.fg.set_viaCoroutine(
                                                                1 as ::core::ffi::c_uint
                                                                    as ::core::ffi::c_uint,
                                                            );
                                                            __pSubq_ref.regResult = dest.iSdst;
                                                            crate::src::src::vdbeaux::sqlite3VdbeEndCoroutine(
                                                                v,
                                                                __pSubq_ref.regReturn,
                                                            );
                                                            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(
                                                                v,
                                                                addrTop - 1 as ::core::ffi::c_int,
                                                            );
                                                            crate::src::src::expr::sqlite3ClearTempRegCache(pParse as *mut crate::src::headers::sqliteInt_h::Parse);
                                                        } else if __pItem_0_ref.fg.isCte()
                                                            as ::core::ffi::c_int
                                                            != 0
                                                            && (*__pItem_0_ref.u2.pCteUse).addrM9e
                                                                > 0 as ::core::ffi::c_int
                                                        {
                                                            let pCteUse: *mut crate::src::headers::sqliteInt_h::CteUse =
                                                                __pItem_0_ref.u2.pCteUse;
                                                            let __pCteUse_ref =
                                                                unsafe { &*pCteUse };
                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                v,
                                                                crate::src::headers::opcodes_h::OP_Gosub,
                                                                __pCteUse_ref.regRtn,
                                                                __pCteUse_ref.addrM9e,
                                                            );
                                                            if __pItem_0_ref.iCursor
                                                                != __pCteUse_ref.iCur
                                                            {
                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                    v,
                                                                    crate::src::headers::opcodes_h::OP_OpenDup,
                                                                    __pItem_0_ref.iCursor,
                                                                    __pCteUse_ref.iCur,
                                                                );
                                                            }
                                                            (*pSub_0).nSelectRow =
                                                                __pCteUse_ref.nRowEst;
                                                        } else {
                                                            pPrior = isSelfJoinView(
                                                                pTabList,
                                                                pItem_0,
                                                                0 as ::core::ffi::c_int,
                                                                i,
                                                            );
                                                            if !pPrior.is_null() {
                                                                
                                                                let pPriorSubq: *mut crate::src::headers::sqliteInt_h::Subquery = (*pPrior).u4.pSubq;
                                                                if (*pPriorSubq).addrFillSub != 0 {
                                                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                        v,
                                                                        crate::src::headers::opcodes_h::OP_Gosub,
                                                                        (*pPriorSubq).regReturn,
                                                                        (*pPriorSubq).addrFillSub,
                                                                    );
                                                                }
                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                    v,
                                                                    crate::src::headers::opcodes_h::OP_OpenDup,
                                                                    __pItem_0_ref.iCursor,
                                                                    (*pPrior).iCursor,
                                                                );
                                                                (*pSub_0).nSelectRow =
                                                                    (*(*pPriorSubq).pSelect)
                                                                        .nSelectRow;
                                                            } else {
                                                                
                                                                let mut onceAddr: ::core::ffi::c_int = 0
                                                                    as ::core::ffi::c_int;
                                                                __pParse_ref.nMem += 1;
                                                                let __pSubq_ref =
                                                                    unsafe { &mut *pSubq };
                                                                __pSubq_ref.regReturn =
                                                                    __pParse_ref.nMem;
                                                                let topAddr: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeAddOp0(v, crate::src::headers::opcodes_h::OP_Goto);
                                                                __pSubq_ref.addrFillSub = topAddr
                                                                    + 1 as ::core::ffi::c_int;
                                                                __pItem_0_ref
                                                                    .fg
                                                                    .set_isMaterialized(
                                                                        1 as ::core::ffi::c_uint
                                                                            as ::core::ffi::c_uint,
                                                                    );
                                                                if __pItem_0_ref.fg.isCorrelated()
                                                                    as ::core::ffi::c_int
                                                                    == 0 as ::core::ffi::c_int
                                                                {
                                                                    onceAddr = crate::src::src::vdbeaux::sqlite3VdbeAddOp0(
                                                                        v, crate::src::headers::opcodes_h::OP_Once,
                                                                    );
                                                                }
                                                                sqlite3SelectDestInit(
                                                                    &raw mut dest,
                                                                    crate::src::headers::sqliteInt_h::SRT_EphemTab_1,
                                                                    __pItem_0_ref.iCursor,
                                                                );
                                                                crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                                                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                    1_u8,
                                                                    b"MATERIALIZE %!S\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                    &[crate::src::src::printf::PrintfArg::SrcItem(pItem_0 as *mut crate::src::headers::sqliteInt_h::SrcItem)],
                                                                );
                                                                sqlite3Select(
                                                                    pParse,
                                                                    pSub_0,
                                                                    &raw mut dest,
                                                                );
                                                                (*__pItem_0_ref.pSTab).nRowLogEst =
                                                                    (*pSub_0).nSelectRow;
                                                                if onceAddr != 0 {
                                                                    crate::src::src::vdbeaux::sqlite3VdbeJumpHere(
                                                                        v, onceAddr,
                                                                    );
                                                                }
                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                    v,
                                                                    crate::src::headers::opcodes_h::OP_Return,
                                                                    __pSubq_ref.regReturn,
                                                                    topAddr
                                                                        + 1 as ::core::ffi::c_int,
                                                                );
                                                                crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, topAddr);
                                                                crate::src::src::expr::sqlite3ClearTempRegCache(pParse as *mut crate::src::headers::sqliteInt_h::Parse);
                                                                if __pItem_0_ref.fg.isCte()
                                                                    as ::core::ffi::c_int
                                                                    != 0
                                                                    && __pItem_0_ref
                                                                        .fg
                                                                        .isCorrelated()
                                                                        as ::core::ffi::c_int
                                                                        == 0 as ::core::ffi::c_int
                                                                {
                                                                    let pCteUse_0: *mut crate::src::headers::sqliteInt_h::CteUse =
                                                                        __pItem_0_ref.u2.pCteUse;
                                                                    let __pCteUse_0_ref =
                                                                        unsafe { &mut *pCteUse_0 };
                                                                    __pCteUse_0_ref.addrM9e =
                                                                        __pSubq_ref.addrFillSub;
                                                                    __pCteUse_0_ref.regRtn =
                                                                        __pSubq_ref.regReturn;
                                                                    __pCteUse_0_ref.iCur =
                                                                        __pItem_0_ref.iCursor;
                                                                    __pCteUse_0_ref.nRowEst =
                                                                        (*pSub_0).nSelectRow;
                                                                }
                                                            }
                                                        }
                                                        if __db_ref.mallocFailed != 0 {
                                                            current_block = 9427106068226466434;
                                                            break;
                                                        }
                                                        __pParse_ref.nHeight -=
                                                            crate::src::src::expr::sqlite3SelectExprHeight(p as *const crate::src::headers::sqliteInt_h::Select);
                                                        __pParse_ref.zAuthContext =
                                                            zSavedAuthContext;
                                                    }
                                                }
                                                i += 1;
                                            }
                                            match current_block {
                                                9427106068226466434 => {}
                                                _ => {
                                                    pEList = __p_ref.pEList;
                                                    pWhere = __p_ref.pWhere;
                                                    pGroupBy = __p_ref.pGroupBy;
                                                    pHaving = __p_ref.pHaving;
                                                    sDistinct.isTnct = (__p_ref.selFlags
                                                        & crate::src::headers::sqliteInt_h::SF_Distinct as crate::src::ext::rtree::rtree::U32_0
                                                        != 0 as crate::src::ext::rtree::rtree::U32_0)
                                                        as ::core::ffi::c_int
                                                        as u8;
                                                    if __p_ref.selFlags
                                                        & (crate::src::headers::sqliteInt_h::SF_Distinct | crate::src::headers::sqliteInt_h::SF_Aggregate) as crate::src::ext::rtree::rtree::U32_0
                                                        == crate::src::headers::sqliteInt_h::SF_Distinct as crate::src::ext::rtree::rtree::U32_0
                                                        && crate::src::src::expr::sqlite3ExprListCompare(
                                                            
                                                            sSort.pOrderBy as *const crate::src::headers::sqliteInt_h::ExprList,
                                                            
                                                            pEList as *const crate::src::headers::sqliteInt_h::ExprList,
                                                            -(1 as ::core::ffi::c_int),
                                                        ) == 0 as ::core::ffi::c_int
                                                        && __db_ref.dbOptFlags & 0x4 as crate::src::ext::rtree::rtree::U32_0
                                                            == 0 as crate::src::ext::rtree::rtree::U32_0
                                                        && __p_ref.pWin.is_null()
                                                    {
                                                        __p_ref.selFlags &= !(crate::src::headers::sqliteInt_h::SF_Distinct as crate::src::ext::rtree::rtree::U32_0);
                                                        __p_ref.pGroupBy =  crate::src::src::expr::sqlite3ExprListDup(
                                                            
                                                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                                            
                                                            pEList as *const crate::src::headers::sqliteInt_h::ExprList,
                                                            0 as ::core::ffi::c_int,
                                                        ) as *mut crate::src::headers::sqliteInt_h::ExprList;
                                                        pGroupBy = __p_ref.pGroupBy;
                                                        if !pGroupBy.is_null() {
                                                            i = 0 as ::core::ffi::c_int;
                                                            while i < (*pGroupBy).nExpr {
                                                                (*(&raw mut (*pGroupBy).a
                                                                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                                    .offset(i as isize))
                                                                .u
                                                                .x
                                                                .iOrderByCol = (i + 1
                                                                    as ::core::ffi::c_int)
                                                                    as crate::src::fts5::U16_0;
                                                                i += 1;
                                                            }
                                                        }
                                                        __p_ref.selFlags |= crate::src::headers::sqliteInt_h::SF_Aggregate as crate::src::ext::rtree::rtree::U32_0;
                                                        sDistinct.isTnct = 2_u8;
                                                    }
                                                    if !sSort.pOrderBy.is_null() {
                                                        
                                                        let pKeyInfo: *mut crate::src::headers::sqliteInt_h::KeyInfo = sqlite3KeyInfoFromExprList(
                                                            pParse,
                                                            sSort.pOrderBy,
                                                            0 as ::core::ffi::c_int,
                                                            (*pEList).nExpr,
                                                        );
                                                        let fresh8 = __pParse_ref.nTab;
                                                        __pParse_ref.nTab += 1;
                                                        sSort.iECursor = fresh8;
                                                        sSort.addrSortIndex = crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                            v,
                                                            crate::src::headers::opcodes_h::OP_OpenEphemeral,
                                                            sSort.iECursor,
                                                            (*sSort.pOrderBy).nExpr
                                                                + 1 as ::core::ffi::c_int
                                                                + (*pEList).nExpr,
                                                            0 as ::core::ffi::c_int,
                                                            pKeyInfo as *mut ::core::ffi::c_char,
                                                            crate::src::src::vdbe::P4_KEYINFO,
                                                        );
                                                    } else {
                                                        sSort.addrSortIndex =
                                                            -(1 as ::core::ffi::c_int);
                                                    }
                                                    if (*pDest).eDest as ::core::ffi::c_int
                                                        == crate::src::headers::sqliteInt_h::SRT_EphemTab_1
                                                    {
                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                            v,
                                                            crate::src::headers::opcodes_h::OP_OpenEphemeral,
                                                            (*pDest).iSDParm,
                                                            (*pEList).nExpr,
                                                        );
                                                        if __p_ref.selFlags & crate::src::headers::sqliteInt_h::SF_NestedFrom as crate::src::ext::rtree::rtree::U32_0
                                                            != 0
                                                        {
                                                            let mut ii: ::core::ffi::c_int;
                                                            let __pEList_ref = unsafe { &mut *pEList };
                                                            ii = __pEList_ref.nExpr
                                                                - 1 as ::core::ffi::c_int;
                                                            while ii > 0 as ::core::ffi::c_int
                                                                && (*(&raw mut __pEList_ref.a
                                                                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                                    .offset(ii as isize))
                                                                .fg
                                                                .bUsed()
                                                                    as ::core::ffi::c_int
                                                                    == 0 as ::core::ffi::c_int
                                                            {
                                                                crate::src::src::expr::sqlite3ExprDelete(
                                                                    
                                                                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                                                    
                                                                    (*(&raw mut __pEList_ref.a
                                                                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                                        .offset(ii as isize))
                                                                    .pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
                                                                );
                                                                crate::src::src::malloc::sqlite3DbFree(
                                                                    
                                                                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                                                    (*(&raw mut __pEList_ref.a
                                                                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                                        .offset(ii as isize))
                                                                    .zEName
                                                                        as *mut ::core::ffi::c_void,
                                                                );
                                                                __pEList_ref.nExpr -= 1;
                                                                ii -= 1;
                                                            }
                                                            ii = 0 as ::core::ffi::c_int;
                                                            while ii < __pEList_ref.nExpr {
                                                                if (*(&raw mut __pEList_ref.a
                                                                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                                    .offset(ii as isize))
                                                                .fg
                                                                .bUsed()
                                                                    as ::core::ffi::c_int
                                                                    == 0 as ::core::ffi::c_int
                                                                {
                                                                    (*(*(&raw mut __pEList_ref.a
                                                                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                                        .offset(ii as isize))
                                                                    .pExpr)
                                                                        .op = crate::src::parse::TK_NULL as u8;
                                                                }
                                                                ii += 1;
                                                            }
                                                        }
                                                    }
                                                    iEnd = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(pParse as *mut crate::src::headers::sqliteInt_h::Parse);
                                                    if __p_ref.selFlags & crate::src::headers::sqliteInt_h::SF_FixedLimit as crate::src::ext::rtree::rtree::U32_0
                                                        == 0 as crate::src::ext::rtree::rtree::U32_0
                                                    {
                                                        __p_ref.nSelectRow = 320 as crate::src::headers::sqliteInt_h::LogEst;
                                                    }
                                                    if !__p_ref.pLimit.is_null() {
                                                        computeLimitRegisters(pParse, p, iEnd);
                                                    }
                                                    if __p_ref.iLimit == 0 as ::core::ffi::c_int
                                                        && sSort.addrSortIndex
                                                            >= 0 as ::core::ffi::c_int
                                                    {
                                                        crate::src::src::vdbeaux::sqlite3VdbeChangeOpcode(
                                                            v,
                                                            sSort.addrSortIndex,
                                                            crate::src::headers::opcodes_h::OP_SorterOpen as u8,
                                                        );
                                                        sSort.sortFlags = (sSort.sortFlags
                                                            as ::core::ffi::c_int
                                                            | SORTFLAG_UseSorter)
                                                            as u8;
                                                    }
                                                    if __p_ref.selFlags & crate::src::headers::sqliteInt_h::SF_Distinct as crate::src::ext::rtree::rtree::U32_0 != 0 {
                                                        let fresh9 = __pParse_ref.nTab;
                                                        __pParse_ref.nTab += 1;
                                                        sDistinct.tabTnct = fresh9;
                                                        sDistinct.addrTnct = crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                            v,
                                                            crate::src::headers::opcodes_h::OP_OpenEphemeral,
                                                            sDistinct.tabTnct,
                                                            0 as ::core::ffi::c_int,
                                                            0 as ::core::ffi::c_int,
                                                            sqlite3KeyInfoFromExprList(
                                                                pParse,
                                                                __p_ref.pEList,
                                                                0 as ::core::ffi::c_int,
                                                                0 as ::core::ffi::c_int,
                                                            )
                                                                as *mut ::core::ffi::c_char,
                                                            crate::src::src::vdbe::P4_KEYINFO,
                                                        );
                                                        crate::src::src::vdbeaux::sqlite3VdbeChangeP5(
                                                            v,
                                                            crate::src::src::btree::BTREE_UNORDERED as crate::src::fts5::U16_0,
                                                        );
                                                        sDistinct.eTnctType =
                                                            crate::src::headers::sqliteInt_h::WHERE_DISTINCT_UNORDERED as u8;
                                                    } else {
                                                        sDistinct.eTnctType =
                                                            crate::src::headers::sqliteInt_h::WHERE_DISTINCT_NOOP as u8;
                                                    }
                                                    if isAgg == 0 && pGroupBy.is_null() {
                                                        let wctrlFlags: crate::src::fts5::U16_0 = ((if sDistinct
                                                            .isTnct
                                                            as ::core::ffi::c_int
                                                            != 0
                                                        {
                                                            crate::src::headers::sqliteInt_h::WHERE_WANT_DISTINCT
                                                        } else {
                                                            0 as ::core::ffi::c_int
                                                        })
                                                            as crate::src::ext::rtree::rtree::U32_0
                                                            | __p_ref.selFlags
                                                                & crate::src::headers::sqliteInt_h::SF_FixedLimit as crate::src::ext::rtree::rtree::U32_0)
                                                            as crate::src::fts5::U16_0;
                                                        let pWin: *mut crate::src::headers::sqliteInt_h::Window = __p_ref.pWin;
                                                        if !pWin.is_null() {
                                                            crate::src::src::window::sqlite3WindowCodeInit(pParse as *mut crate::src::headers::sqliteInt_h::Parse,  p as *mut crate::src::headers::sqliteInt_h::Select);
                                                        }
                                                        pWInfo = crate::src::src::r#where::sqlite3WhereBegin(
                                                            
                                                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                            
                                                            pTabList as *mut crate::src::headers::sqliteInt_h::SrcList,
                                                            
                                                            pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
                                                            
                                                            sSort.pOrderBy as *mut crate::src::headers::sqliteInt_h::ExprList,
                                                            
                                                            __p_ref.pEList as *mut crate::src::headers::sqliteInt_h::ExprList,
                                                            
                                                            p as *mut crate::src::headers::sqliteInt_h::Select,
                                                            wctrlFlags,
                                                            __p_ref.nSelectRow as ::core::ffi::c_int,
                                                        );
                                                        if pWInfo.is_null() {
                                                            current_block = 9427106068226466434;
                                                        } else {
                                                            if (crate::src::src::r#where::sqlite3WhereOutputRowCount(pWInfo)
                                                                as ::core::ffi::c_int)
                                                                < __p_ref.nSelectRow
                                                                    as ::core::ffi::c_int
                                                            {
                                                                __p_ref.nSelectRow =
                                                                    crate::src::src::r#where::sqlite3WhereOutputRowCount(
                                                                        pWInfo,
                                                                    );
                                                                if (*pDest).eDest
                                                                    as ::core::ffi::c_int
                                                                    <= crate::src::headers::sqliteInt_h::SRT_DistQueue
                                                                    && (*pDest).eDest
                                                                        as ::core::ffi::c_int
                                                                        >= crate::src::headers::sqliteInt_h::SRT_DistFifo
                                                                {
                                                                    __p_ref.nSelectRow = ((*p)
                                                                        .nSelectRow
                                                                        as ::core::ffi::c_int
                                                                        - 30 as ::core::ffi::c_int)
                                                                        as crate::src::headers::sqliteInt_h::LogEst;
                                                                }
                                                            }
                                                            if sDistinct.isTnct
                                                                as ::core::ffi::c_int
                                                                != 0
                                                                && crate::src::src::r#where::sqlite3WhereIsDistinct(pWInfo)
                                                                    != 0
                                                            {
                                                                sDistinct.eTnctType =
                                                                    crate::src::src::r#where::sqlite3WhereIsDistinct(pWInfo)
                                                                        as u8;
                                                            }
                                                            if !sSort.pOrderBy.is_null() {
                                                                sSort.nOBSat =
                                                                    crate::src::src::r#where::sqlite3WhereIsOrdered(pWInfo);
                                                                sSort.labelOBLopt = crate::src::src::r#where::sqlite3WhereOrderByLimitOptLabel(
                                                                    pWInfo,
                                                                );
                                                                if sSort.nOBSat
                                                                    == (*sSort.pOrderBy).nExpr
                                                                {
                                                                    sSort.pOrderBy =
                                                                        ::core::ptr::null_mut::<
                                                                            crate::src::headers::sqliteInt_h::ExprList,
                                                                        >(
                                                                        );
                                                                }
                                                            }
                                                            if sSort.addrSortIndex
                                                                >= 0 as ::core::ffi::c_int
                                                                && sSort.pOrderBy.is_null()
                                                            {
                                                                crate::src::src::vdbeaux::sqlite3VdbeChangeToNoop(
                                                                    v,
                                                                    sSort.addrSortIndex,
                                                                );
                                                            }
                                                            if !pWin.is_null() {
                                                                let addrGosub: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                                                                    
                                                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                );
                                                                let iCont: ::core::ffi::c_int =
                                                                    crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(pParse as *mut crate::src::headers::sqliteInt_h::Parse);
                                                                let iBreak: ::core::ffi::c_int =
                                                                    crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(pParse as *mut crate::src::headers::sqliteInt_h::Parse);
                                                                __pParse_ref.nMem += 1;
                                                                let regGosub: ::core::ffi::c_int = __pParse_ref.nMem;
                                                                crate::src::src::window::sqlite3WindowCodeStep(
                                                                    
                                                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,  p as *mut crate::src::headers::sqliteInt_h::Select, pWInfo, regGosub,
                                                                    addrGosub,
                                                                );
                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                    v,
                                                                    crate::src::headers::opcodes_h::OP_Goto,
                                                                    0 as ::core::ffi::c_int,
                                                                    iBreak,
                                                                );
                                                                crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(
                                                                    v, addrGosub,
                                                                );
                                                                sSort.labelOBLopt =
                                                                    0 as ::core::ffi::c_int;
                                                                selectInnerLoop(
                                                                    pParse,
                                                                    p,
                                                                    -(1 as ::core::ffi::c_int),
                                                                    &raw mut sSort,
                                                                    &raw mut sDistinct,
                                                                    pDest,
                                                                    iCont,
                                                                    iBreak,
                                                                );
                                                                crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, iCont);
                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                                                                    v, crate::src::headers::opcodes_h::OP_Return, regGosub,
                                                                );
                                                                crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, iBreak);
                                                            } else {
                                                                selectInnerLoop(
                                                                    pParse,
                                                                    p,
                                                                    -(1 as ::core::ffi::c_int),
                                                                    &raw mut sSort,
                                                                    &raw mut sDistinct,
                                                                    pDest,
                                                                    crate::src::src::r#where::sqlite3WhereContinueLabel(
                                                                        pWInfo,
                                                                    ),
                                                                    crate::src::src::r#where::sqlite3WhereBreakLabel(pWInfo),
                                                                );
                                                                crate::src::src::r#where::sqlite3WhereEnd(pWInfo);
                                                            }
                                                            current_block = 6198430992841073810;
                                                        }
                                                    } else {
                                                        let mut sNC: crate::src::headers::sqliteInt_h::NameContext = unsafe { ::core::mem::zeroed() };
                                                        let iAMem: ::core::ffi::c_int;
                                                        let iBMem: ::core::ffi::c_int;
                                                        let iUseFlag: ::core::ffi::c_int;
                                                        let iAbortFlag: ::core::ffi::c_int;
                                                        let groupBySort: ::core::ffi::c_int;
                                                        
                                                        let mut sortPTab: ::core::ffi::c_int =
                                                            0 as ::core::ffi::c_int;
                                                        let mut sortOut: ::core::ffi::c_int =
                                                            0 as ::core::ffi::c_int;
                                                        let mut orderByGrp: ::core::ffi::c_int =
                                                            0 as ::core::ffi::c_int;
                                                        if !pGroupBy.is_null() {
                                                            let mut k: ::core::ffi::c_int;
                                                            let mut pItem_1: *mut crate::src::headers::sqliteInt_h::ExprList_item;
                                                            k = (*__p_ref.pEList).nExpr;
                                                            pItem_1 = &raw mut (*__p_ref.pEList).a
                                                                as *mut crate::src::headers::sqliteInt_h::ExprList_item
                                                                as *mut crate::src::headers::sqliteInt_h::ExprList_item;
                                                            while k > 0 as ::core::ffi::c_int {
                                                                (*pItem_1).u.x.iAlias =
                                                                    0 as crate::src::fts5::U16_0;
                                                                k -= 1;
                                                                pItem_1 = pItem_1.offset(1);
                                                            }
                                                            let __pGroupBy_ref =
                                                                unsafe { &mut *pGroupBy };
                                                            k = __pGroupBy_ref.nExpr;
                                                            pItem_1 = &raw mut __pGroupBy_ref.a
                                                                as *mut crate::src::headers::sqliteInt_h::ExprList_item
                                                                as *mut crate::src::headers::sqliteInt_h::ExprList_item;
                                                            while k > 0 as ::core::ffi::c_int {
                                                                (*pItem_1).u.x.iAlias =
                                                                    0 as crate::src::fts5::U16_0;
                                                                k -= 1;
                                                                pItem_1 = pItem_1.offset(1);
                                                            }
                                                            if __p_ref.nSelectRow
                                                                as ::core::ffi::c_int
                                                                > 66 as ::core::ffi::c_int
                                                            {
                                                                __p_ref.nSelectRow = 66 as crate::src::headers::sqliteInt_h::LogEst;
                                                            }
                                                            if !sSort.pOrderBy.is_null()
                                                                && __pGroupBy_ref.nExpr
                                                                    == (*sSort.pOrderBy).nExpr
                                                            {
                                                                let mut ii_0: ::core::ffi::c_int;
                                                                ii_0 = 0 as ::core::ffi::c_int;
                                                                while ii_0 < __pGroupBy_ref.nExpr {
                                                                    
                                                                    let sortFlags: crate::src::ext::rtree::rtree::U8_0 = ((*(&raw mut (*sSort
                                                                            .pOrderBy)
                                                                            .a
                                                                            as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                                            .offset(ii_0 as isize))
                                                                        .fg
                                                                        .sortFlags
                                                                            as ::core::ffi::c_int
                                                                            & crate::src::headers::sqliteInt_h::KEYINFO_ORDER_DESC)
                                                                            as u8;
                                                                    (*(&raw mut __pGroupBy_ref.a
                                                                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                                        .offset(ii_0 as isize))
                                                                    .fg
                                                                    .sortFlags = sortFlags;
                                                                    ii_0 += 1;
                                                                }
                                                                if crate::src::src::expr::sqlite3ExprListCompare(
                                                                    
                                                                    pGroupBy as *const crate::src::headers::sqliteInt_h::ExprList,
                                                                    
                                                                    sSort.pOrderBy as *const crate::src::headers::sqliteInt_h::ExprList,
                                                                    -(1 as ::core::ffi::c_int),
                                                                ) == 0 as ::core::ffi::c_int
                                                                {
                                                                    orderByGrp =
                                                                        1 as ::core::ffi::c_int;
                                                                }
                                                            }
                                                        } else {
                                                            __p_ref.nSelectRow = 0 as crate::src::headers::sqliteInt_h::LogEst;
                                                        }
                                                        let addrEnd: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(pParse as *mut crate::src::headers::sqliteInt_h::Parse);
                                                        pAggInfo = crate::src::src::malloc::sqlite3DbMallocZero(
                                                            
                                                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                                            ::core::mem::size_of::<crate::src::headers::sqliteInt_h::AggInfo>()
                                                                as crate::src::ext::rtree::rtree::U64_0,
                                                        )
                                                            as *mut crate::src::headers::sqliteInt_h::AggInfo;
                                                        if !pAggInfo.is_null() {
                                                            crate::src::src::prepare::sqlite3ParserAddCleanup(
                                                                
                                                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                ::core::mem::transmute(
                                                                    
                                                                Some(
                                                                    agginfoFree
                                                                        as unsafe extern "C" fn(
                                                                            *mut crate::src::headers::sqliteInt_h::sqlite3,
                                                                            *mut ::core::ffi::c_void,
                                                                        )
                                                                            -> (
                                                                        ),
                                                                ),
                                                                ),
                                                                pAggInfo
                                                                    as *mut ::core::ffi::c_void,
                                                            );
                                                        }
                                                        if __db_ref.mallocFailed != 0 {
                                                            current_block = 9427106068226466434;
                                                        } else {
                                                            let __pAggInfo_ref =
                                                                unsafe { &mut *pAggInfo };
                                                            __pAggInfo_ref.selId = __p_ref.selId;
                                                            sNC.pParse = pParse;
                                                            sNC.pSrcList = pTabList;
                                                            sNC.uNC.pAggInfo = pAggInfo;
                                                            __pAggInfo_ref.nSortingColumn =
                                                                (if !pGroupBy.is_null() {
                                                                    (*pGroupBy).nExpr
                                                                } else {
                                                                    0 as ::core::ffi::c_int
                                                                })
                                                                    as crate::src::ext::rtree::rtree::U32_0;
                                                            __pAggInfo_ref.pGroupBy = pGroupBy;
                                                            crate::src::src::expr::sqlite3ExprAnalyzeAggList(
                                                                
                                                                &raw mut sNC as *mut _ as *mut crate::src::headers::sqliteInt_h::NameContext,
                                                                
                                                                pEList as *mut crate::src::headers::sqliteInt_h::ExprList,
                                                            );
                                                            crate::src::src::expr::sqlite3ExprAnalyzeAggList(
                                                                
                                                                &raw mut sNC as *mut _ as *mut crate::src::headers::sqliteInt_h::NameContext,
                                                                
                                                                sSort.pOrderBy as *mut crate::src::headers::sqliteInt_h::ExprList,
                                                            );
                                                            if !pHaving.is_null() {
                                                                if !pGroupBy.is_null() {
                                                                    havingToWhere(pParse, p);
                                                                    pWhere = __p_ref.pWhere;
                                                                }
                                                                crate::src::src::expr::sqlite3ExprAnalyzeAggregates(
                                                                    
                                                                    &raw mut sNC as *mut _ as *mut crate::src::headers::sqliteInt_h::NameContext,
                                                                    
                                                                    pHaving as *mut crate::src::headers::sqliteInt_h::Expr,
                                                                );
                                                            }
                                                            __pAggInfo_ref.nAccumulator =
                                                                __pAggInfo_ref.nColumn;
                                                            if __p_ref.pGroupBy.is_null()
                                                                && __p_ref.pHaving.is_null()
                                                                && __pAggInfo_ref.nFunc
                                                                    == 1 as ::core::ffi::c_int
                                                            {
                                                                minMaxFlag = minMaxQuery(
                                                                    db,
                                                                    (*__pAggInfo_ref
                                                                        .aFunc
                                                                        .offset(0_isize))
                                                                    .pFExpr,
                                                                    &raw mut pMinMaxOrderBy,
                                                                );
                                                            } else {
                                                                minMaxFlag =
                                                                    crate::src::headers::sqliteInt_h::WHERE_ORDERBY_NORMAL as u8;
                                                            }
                                                            analyzeAggFuncArgs(
                                                                pAggInfo,
                                                                &raw mut sNC,
                                                            );
                                                            if __db_ref.mallocFailed != 0 {
                                                                current_block = 9427106068226466434;
                                                            } else {
                                                                if !pGroupBy.is_null() {
                                                                    
                                                                    let addr1: ::core::ffi::c_int;
                                                                    let mut addrOutputRow: ::core::ffi::c_int;
                                                                    
                                                                    let addrSetAbort: ::core::ffi::c_int;
                                                                    let addrTopOfLoop: ::core::ffi::c_int;
                                                                    
                                                                    
                                                                    
                                                                    let mut pDistinct: *mut crate::src::headers::sqliteInt_h::ExprList = ::core::ptr::null_mut::<
                                                                        crate::src::headers::sqliteInt_h::ExprList,
                                                                    >();
                                                                    let mut distFlag: crate::src::fts5::U16_0 =
                                                                        0 as crate::src::fts5::U16_0;
                                                                    let eDist: ::core::ffi::c_int;
                                                                    let __pAggInfo_ref =
                                                                        unsafe { &mut *pAggInfo };
                                                                    if __pAggInfo_ref.nFunc == 1 as ::core::ffi::c_int
                                                                        && (*(*pAggInfo)
                                                                            .aFunc
                                                                            .offset(0_isize))
                                                                            .iDistinct >= 0 as ::core::ffi::c_int
                                                                        && !(*(*pAggInfo)
                                                                            .aFunc
                                                                            .offset(0_isize))
                                                                            .pFExpr
                                                                            .is_null()
                                                                        && (*(*(*pAggInfo)
                                                                            .aFunc
                                                                            .offset(0_isize))
                                                                            .pFExpr)
                                                                            .flags & 0x1000 as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0
                                                                        && !(*(*(*pAggInfo)
                                                                            .aFunc
                                                                            .offset(0_isize))
                                                                            .pFExpr)
                                                                            .x
                                                                            .pList
                                                                            .is_null()
                                                                    {
                                                                        let mut pExpr: *mut crate::src::headers::sqliteInt_h::Expr = (*(&raw mut (*(*(*(*pAggInfo)
                                                                            .aFunc
                                                                            .offset(0_isize))
                                                                            .pFExpr)
                                                                            .x
                                                                            .pList)
                                                                            .a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                                            .offset(0_isize))
                                                                            .pExpr;
                                                                        pExpr =  crate::src::src::expr::sqlite3ExprDup(db as *mut crate::src::headers::sqliteInt_h::sqlite3,  pExpr as *const crate::src::headers::sqliteInt_h::Expr, 0 as ::core::ffi::c_int) as
    *mut crate::src::headers::sqliteInt_h::Expr;
                                                                        pDistinct =  crate::src::src::expr::sqlite3ExprListDup(
                                                                            
                                                                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                                                            
                                                                            pGroupBy as *const crate::src::headers::sqliteInt_h::ExprList,
                                                                            0 as ::core::ffi::c_int,
                                                                        ) as *mut crate::src::headers::sqliteInt_h::ExprList;
                                                                        pDistinct =  crate::src::src::expr::sqlite3ExprListAppend(pParse as *mut crate::src::headers::sqliteInt_h::Parse,  pDistinct as *mut crate::src::headers::sqliteInt_h::ExprList,  pExpr as *mut crate::src::headers::sqliteInt_h::Expr) as
    *mut crate::src::headers::sqliteInt_h::ExprList;
                                                                        distFlag = (if !pDistinct.is_null() {
                                                                            crate::src::headers::sqliteInt_h::WHERE_WANT_DISTINCT | crate::src::headers::sqliteInt_h::WHERE_AGG_DISTINCT
                                                                        } else {
                                                                            0 as ::core::ffi::c_int
                                                                        }) as crate::src::fts5::U16_0;
                                                                    }
                                                                    let fresh10 = __pParse_ref.nTab;
                                                                    __pParse_ref.nTab += 1;
                                                                    __pAggInfo_ref.sortingIdx =
                                                                        fresh10;
                                                                    let pKeyInfo_0: *mut crate::src::headers::sqliteInt_h::KeyInfo = sqlite3KeyInfoFromExprList(
                                                                            pParse,
                                                                            pGroupBy,
                                                                            0 as ::core::ffi::c_int,
                                                                            __pAggInfo_ref.nColumn,
                                                                        );
                                                                    let addrSortingIdx: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                                        v,
                                                                        crate::src::headers::opcodes_h::OP_SorterOpen,
                                                                        __pAggInfo_ref.sortingIdx,
                                                                        __pAggInfo_ref.nSortingColumn as ::core::ffi::c_int,
                                                                        0 as ::core::ffi::c_int,
                                                                        pKeyInfo_0 as *mut ::core::ffi::c_char,
                                                                        crate::src::src::vdbe::P4_KEYINFO,
                                                                    );
                                                                    __pParse_ref.nMem += 1;
                                                                    iUseFlag = __pParse_ref.nMem;
                                                                    __pParse_ref.nMem += 1;
                                                                    iAbortFlag = __pParse_ref.nMem;
                                                                    __pParse_ref.nMem += 1;
                                                                    let regOutputRow: ::core::ffi::c_int = __pParse_ref.nMem;
                                                                    addrOutputRow =
                                                                        crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                                                                            
                                                                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                        );
                                                                    __pParse_ref.nMem += 1;
                                                                    let regReset: ::core::ffi::c_int = __pParse_ref.nMem;
                                                                    let addrReset: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                                                                            
                                                                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                        );
                                                                    iAMem = __pParse_ref.nMem
                                                                        + 1 as ::core::ffi::c_int;
                                                                    let __pGroupBy_ref =
                                                                        unsafe { &mut *pGroupBy };
                                                                    __pParse_ref.nMem +=
                                                                        __pGroupBy_ref.nExpr;
                                                                    iBMem = __pParse_ref.nMem
                                                                        + 1 as ::core::ffi::c_int;
                                                                    __pParse_ref.nMem +=
                                                                        __pGroupBy_ref.nExpr;
                                                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                        v,
                                                                        crate::src::headers::opcodes_h::OP_Integer,
                                                                        0 as ::core::ffi::c_int,
                                                                        iAbortFlag,
                                                                    );
                                                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                        v,
                                                                        crate::src::headers::opcodes_h::OP_Null,
                                                                        0 as ::core::ffi::c_int,
                                                                        iAMem,
                                                                        iAMem + __pGroupBy_ref.nExpr - 1 as ::core::ffi::c_int,
                                                                    );
                                                                    crate::src::src::expr::sqlite3ExprNullRegisterRange(
                                                                        
                                                                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                        iAMem,
                                                                        __pGroupBy_ref.nExpr,
                                                                    );
                                                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                        v, crate::src::headers::opcodes_h::OP_Gosub, regReset,
                                                                        addrReset,
                                                                    );
                                                                    pWInfo = crate::src::src::r#where::sqlite3WhereBegin(
                                                                        
                                                                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                        
                                                                        pTabList as *mut crate::src::headers::sqliteInt_h::SrcList,
                                                                        
                                                                        pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
                                                                        
                                                                        pGroupBy as *mut crate::src::headers::sqliteInt_h::ExprList,
                                                                        
                                                                        pDistinct as *mut crate::src::headers::sqliteInt_h::ExprList,
                                                                        
                                                                        p as *mut crate::src::headers::sqliteInt_h::Select,
                                                                        ((if sDistinct.isTnct as ::core::ffi::c_int
                                                                            == 2 as ::core::ffi::c_int
                                                                        {
                                                                            crate::src::headers::sqliteInt_h::WHERE_DISTINCTBY
                                                                        } else {
                                                                            crate::src::headers::sqliteInt_h::WHERE_GROUPBY
                                                                        })
                                                                            | (if orderByGrp != 0 {
                                                                                crate::src::headers::sqliteInt_h::WHERE_SORTBYGROUP
                                                                            } else {
                                                                                0 as ::core::ffi::c_int
                                                                            }) | distFlag as ::core::ffi::c_int) as crate::src::fts5::U16_0,
                                                                        0 as ::core::ffi::c_int,
                                                                    );
                                                                    if pWInfo.is_null() {
                                                                        crate::src::src::expr::sqlite3ExprListDelete(
                                                                            
                                                                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,  pDistinct as *mut crate::src::headers::sqliteInt_h::ExprList,
                                                                        );
                                                                        current_block =
                                                                            9427106068226466434;
                                                                    } else {
                                                                        if !(*pParse)
                                                                            .pIdxEpr
                                                                            .is_null()
                                                                        {
                                                                            optimizeAggregateUseOfIndexedExpr(
                                                                                pParse,
                                                                                p,
                                                                                pAggInfo,
                                                                                &raw mut sNC,
                                                                            );
                                                                        }
                                                                        assignAggregateRegisters(
                                                                            pParse, pAggInfo,
                                                                        );
                                                                        eDist =
                                                                            crate::src::src::r#where::sqlite3WhereIsDistinct(
                                                                                pWInfo,
                                                                            );
                                                                        if crate::src::src::r#where::sqlite3WhereIsOrdered(
                                                                            pWInfo,
                                                                        ) == __pGroupBy_ref.nExpr
                                                                        {
                                                                            groupBySort = 0 as ::core::ffi::c_int;
                                                                        } else {
                                                                            
                                                                            
                                                                            let mut nCol: ::core::ffi::c_int;
                                                                            
                                                                            crate::src::src::vdbeaux::sqlite3VdbeExplain_args(
                                                                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                                0_u8,
                                                                                b"USE TEMP B-TREE FOR %s\0" as *const u8
                                                                                    as *const ::core::ffi::c_char,
                                                                                &[crate::src::src::printf::PrintfArg::Str(if sDistinct.isTnct as ::core::ffi::c_int != 0
                                                                                    && __p_ref.selFlags & 0x1 as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0
                                                                                {
                                                                                    b"DISTINCT\0" as *const u8 as *const ::core::ffi::c_char
                                                                                } else {
                                                                                    b"GROUP BY\0" as *const u8 as *const ::core::ffi::c_char
                                                                                } as *mut ::core::ffi::c_char)],
                                                                            );
                                                                            groupBySort = 1 as ::core::ffi::c_int;
                                                                            let nGroupBy: ::core::ffi::c_int = __pGroupBy_ref.nExpr;
                                                                            nCol = nGroupBy;
                                                                            j = nGroupBy;
                                                                            i = 0 as ::core::ffi::c_int;
                                                                            while i
                                                                                < (*pAggInfo)
                                                                                    .nColumn
                                                                            {
                                                                                if (*(*pAggInfo)
                                                                                    .aCol
                                                                                    .offset(
                                                                                        i as isize,
                                                                                    ))
                                                                                .iSorterColumn
                                                                                    >= j
                                                                                {
                                                                                    nCol += 1;
                                                                                    j += 1;
                                                                                }
                                                                                i += 1;
                                                                            }
                                                                            let regBase: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempRange(
                                                                                    
                                                                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse, nCol,
                                                                                );
                                                                            crate::src::src::expr::sqlite3ExprCodeExprList(
                                                                                
                                                                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                                
                                                                                pGroupBy as *mut crate::src::headers::sqliteInt_h::ExprList,
                                                                                regBase,
                                                                                0 as ::core::ffi::c_int,
                                                                                0_u8,
                                                                            );
                                                                            j = nGroupBy;
                                                                            (*pAggInfo)
                                                                                .directMode =
                                                                                1_u8;
                                                                            i = 0 as ::core::ffi::c_int;
                                                                            while i
                                                                                < (*pAggInfo)
                                                                                    .nColumn
                                                                            {
                                                                                let pCol: *mut crate::src::headers::sqliteInt_h::AggInfo_col = (*pAggInfo)
                                                                                    .aCol
                                                                                    .offset(i as isize) as *mut crate::src::headers::sqliteInt_h::AggInfo_col;
                                                                                if (*pCol)
                                                                                    .iSorterColumn
                                                                                    >= j
                                                                                {
                                                                                    crate::src::src::expr::sqlite3ExprCode(
                                                                                        
                                                                                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                                        
                                                                                        (*pCol)
                                                                                            .pCExpr as *mut crate::src::headers::sqliteInt_h::Expr,
                                                                                        j + regBase,
                                                                                    );
                                                                                    j += 1;
                                                                                }
                                                                                i += 1;
                                                                            }
                                                                            (*pAggInfo)
                                                                                .directMode =
                                                                                0_u8;
                                                                            let regRecord: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempReg(
                                                                                    
                                                                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                                );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                                v,
                                                                                crate::src::headers::opcodes_h::OP_MakeRecord,
                                                                                regBase,
                                                                                nCol,
                                                                                regRecord,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                                v,
                                                                                crate::src::headers::opcodes_h::OP_SorterInsert,
                                                                                (*pAggInfo)
                                                                                    .sortingIdx,
                                                                                regRecord,
                                                                            );
                                                                            crate::src::src::expr::sqlite3ReleaseTempReg(
                                                                                
                                                                                pParse as *mut crate::src::headers::sqliteInt_h::Parse, regRecord,
                                                                            );
                                                                            crate::src::src::expr::sqlite3ReleaseTempRange(
                                                                                
                                                                                pParse as *mut crate::src::headers::sqliteInt_h::Parse, regBase,
                                                                                nCol,
                                                                            );
                                                                            crate::src::src::r#where::sqlite3WhereEnd(pWInfo);
                                                                            let fresh11 =
                                                                                __pParse_ref.nTab;
                                                                            __pParse_ref.nTab += 1;
                                                                            sortPTab = fresh11;
                                                                            (*pAggInfo)
                                                                                .sortingIdxPTab =
                                                                                sortPTab;
                                                                            sortOut =
                                                                                crate::src::src::expr::sqlite3GetTempReg(
                                                                                    
                                                                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                                );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                                v,
                                                                                crate::src::headers::opcodes_h::OP_OpenPseudo,
                                                                                sortPTab,
                                                                                sortOut,
                                                                                nCol,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                                v,
                                                                                crate::src::headers::opcodes_h::OP_SorterSort,
                                                                                (*pAggInfo)
                                                                                    .sortingIdx,
                                                                                addrEnd,
                                                                            );
                                                                            (*pAggInfo)
                                                                                .useSortingIdx =
                                                                                1_u8;
                                                                        }
                                                                        if !(*pParse)
                                                                            .pIdxEpr
                                                                            .is_null()
                                                                        {
                                                                            aggregateConvertIndexedExprRefToColumn(pAggInfo);
                                                                        }
                                                                        if orderByGrp != 0
                                                                            && __db_ref.dbOptFlags & 0x4 as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0
                                                                            && (groupBySort != 0 || crate::src::src::r#where::sqlite3WhereIsSorted(pWInfo) != 0)
                                                                        {
                                                                            sSort.pOrderBy = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
                                                                            crate::src::src::vdbeaux::sqlite3VdbeChangeToNoop(v, sSort.addrSortIndex);
                                                                        }
                                                                        addrTopOfLoop =
                                                                            crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(
                                                                                v,
                                                                            );
                                                                        if groupBySort != 0 {
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                                v,
                                                                                crate::src::headers::opcodes_h::OP_SorterData,
                                                                                (*pAggInfo)
                                                                                    .sortingIdx,
                                                                                sortOut,
                                                                                sortPTab,
                                                                            );
                                                                        }
                                                                        j = 0 as ::core::ffi::c_int;
                                                                        while j < __pGroupBy_ref
                                                                            .nExpr
                                                                        {
                                                                            let iOrderByCol: ::core::ffi::c_int = (*(&raw mut (*pGroupBy)
                                                                                .a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                                                .offset(j as isize))
                                                                                .u
                                                                                .x
                                                                                .iOrderByCol as ::core::ffi::c_int;
                                                                            if groupBySort != 0 {
                                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                                    v,
                                                                                    crate::src::headers::opcodes_h::OP_Column,
                                                                                    sortPTab,
                                                                                    j,
                                                                                    iBMem + j,
                                                                                );
                                                                            } else {
                                                                                (*pAggInfo)
                                                                                    .directMode =
                                                                                    1_u8;
                                                                                crate::src::src::expr::sqlite3ExprCode(
                                                                                    
                                                                                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                                    
                                                                                    (*(&raw mut __pGroupBy_ref.a as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                                                        .offset(j as isize))
                                                                                        .pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
                                                                                    iBMem + j,
                                                                                );
                                                                            }
                                                                            if iOrderByCol != 0 {
                                                                                let mut pX: *mut crate::src::headers::sqliteInt_h::Expr = (*(&raw mut (*__p_ref.pEList).a
                                                                                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                                                                                    .offset((iOrderByCol - 1 as ::core::ffi::c_int) as isize))
                                                                                    .pExpr;
                                                                                let mut pBase: *mut crate::src::headers::sqliteInt_h::Expr =  crate::src::src::expr::sqlite3ExprSkipCollateAndLikely(
                                                                                    
                                                                                    pX as *mut crate::src::headers::sqliteInt_h::Expr,
                                                                                ) as *mut crate::src::headers::sqliteInt_h::Expr;
                                                                                while !pBase.is_null()
                                                                                    && (*pBase).op as ::core::ffi::c_int == crate::src::parse::TK_IF_NULL_ROW_1
                                                                                {
                                                                                    pX = (*pBase).pLeft;
                                                                                    pBase =  crate::src::src::expr::sqlite3ExprSkipCollateAndLikely(pX as *mut crate::src::headers::sqliteInt_h::Expr) as *mut crate::src::headers::sqliteInt_h::Expr;
                                                                                }
                                                                                if !pBase.is_null()
                                                                                    && (*pBase).op as ::core::ffi::c_int != crate::src::parse::TK_AGG_COLUMN
                                                                                    && (*pBase).op as ::core::ffi::c_int != crate::src::parse::TK_REGISTER
                                                                                {
                                                                                    crate::src::src::expr::sqlite3ExprToRegister(pX as *mut crate::src::headers::sqliteInt_h::Expr, iAMem + j);
                                                                                }
                                                                            }
                                                                            j += 1;
                                                                        }
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                                            v,
                                                                            crate::src::headers::opcodes_h::OP_Compare,
                                                                            iAMem,
                                                                            iBMem,
                                                                            __pGroupBy_ref.nExpr,
                                                                            sqlite3KeyInfoRef(pKeyInfo_0) as *mut ::core::ffi::c_char,
                                                                            crate::src::src::vdbe::P4_KEYINFO,
                                                                        );
                                                                        addr1 =
                                                                            crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(
                                                                                v,
                                                                            );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                            v,
                                                                            crate::src::headers::opcodes_h::OP_Jump,
                                                                            addr1 + 1 as ::core::ffi::c_int,
                                                                            0 as ::core::ffi::c_int,
                                                                            addr1 + 1 as ::core::ffi::c_int,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                            v,
                                                                            crate::src::headers::opcodes_h::OP_Gosub,
                                                                            regOutputRow,
                                                                            addrOutputRow,
                                                                        );
                                                                        crate::src::src::expr::sqlite3ExprCodeMove(
                                                                            
                                                                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                            iBMem,
                                                                            iAMem,
                                                                            __pGroupBy_ref.nExpr,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                            v, crate::src::headers::opcodes_h::OP_IfPos,
                                                                            iAbortFlag, addrEnd,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                            v, crate::src::headers::opcodes_h::OP_Gosub, regReset,
                                                                            addrReset,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeJumpHere(
                                                                            v, addr1,
                                                                        );
                                                                        updateAccumulator(
                                                                            pParse, iUseFlag,
                                                                            pAggInfo, eDist,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                            v,
                                                                            crate::src::headers::opcodes_h::OP_Integer,
                                                                            1 as ::core::ffi::c_int,
                                                                            iUseFlag,
                                                                        );
                                                                        if groupBySort != 0 {
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                                v,
                                                                                crate::src::headers::opcodes_h::OP_SorterNext,
                                                                                (*pAggInfo)
                                                                                    .sortingIdx,
                                                                                addrTopOfLoop,
                                                                            );
                                                                        } else {
                                                                            crate::src::src::r#where::sqlite3WhereEnd(pWInfo);
                                                                            crate::src::src::vdbeaux::sqlite3VdbeChangeToNoop(
                                                                                v,
                                                                                addrSortingIdx,
                                                                            );
                                                                        }
                                                                        crate::src::src::expr::sqlite3ExprListDelete(
                                                                            
                                                                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,  pDistinct as *mut crate::src::headers::sqliteInt_h::ExprList,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                            v,
                                                                            crate::src::headers::opcodes_h::OP_Gosub,
                                                                            regOutputRow,
                                                                            addrOutputRow,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeGoto(v, addrEnd);
                                                                        addrSetAbort =
                                                                            crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(
                                                                                v,
                                                                            );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                            v,
                                                                            crate::src::headers::opcodes_h::OP_Integer,
                                                                            1 as ::core::ffi::c_int,
                                                                            iAbortFlag,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                                                                            v,
                                                                            crate::src::headers::opcodes_h::OP_Return,
                                                                            regOutputRow,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(
                                                                            v,
                                                                            addrOutputRow,
                                                                        );
                                                                        addrOutputRow =
                                                                            crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(
                                                                                v,
                                                                            );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                            v,
                                                                            crate::src::headers::opcodes_h::OP_IfPos,
                                                                            iUseFlag,
                                                                            addrOutputRow + 2 as ::core::ffi::c_int,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                                                                            v,
                                                                            crate::src::headers::opcodes_h::OP_Return,
                                                                            regOutputRow,
                                                                        );
                                                                        finalizeAggFunctions(
                                                                            pParse, pAggInfo,
                                                                        );
                                                                        crate::src::src::expr::sqlite3ExprIfFalse(
                                                                            
                                                                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                            
                                                                            pHaving as *mut crate::src::headers::sqliteInt_h::Expr,
                                                                            addrOutputRow + 1 as ::core::ffi::c_int,
                                                                            crate::src::headers::sqliteInt_h::SQLITE_JUMPIFNULL,
                                                                        );
                                                                        selectInnerLoop(
                                                                            pParse,
                                                                            p,
                                                                            -(1 as ::core::ffi::c_int),
                                                                            &raw mut sSort,
                                                                            &raw mut sDistinct,
                                                                            pDest,
                                                                            addrOutputRow + 1 as ::core::ffi::c_int,
                                                                            addrSetAbort,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                                                                            v,
                                                                            crate::src::headers::opcodes_h::OP_Return,
                                                                            regOutputRow,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(
                                                                            v, addrReset,
                                                                        );
                                                                        resetAccumulator(
                                                                            pParse, pAggInfo,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                            v,
                                                                            crate::src::headers::opcodes_h::OP_Integer,
                                                                            0 as ::core::ffi::c_int,
                                                                            iUseFlag,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                                                                            v, crate::src::headers::opcodes_h::OP_Return, regReset,
                                                                        );
                                                                        if distFlag as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                                                            && eDist != crate::src::headers::sqliteInt_h::WHERE_DISTINCT_NOOP
                                                                        {
                                                                            let pF: *mut crate::src::headers::sqliteInt_h::AggInfo_func = (*pAggInfo)
                                                                                .aFunc
                                                                                .offset(0_isize)
                                                                                as *mut crate::src::headers::sqliteInt_h::AggInfo_func;
                                                                            fixDistinctOpenEph(
                                                                                pParse,
                                                                                eDist,
                                                                                (*pF).iDistinct,
                                                                                (*pF).iDistAddr,
                                                                            );
                                                                        }
                                                                        current_block =
                                                                            2969779015960460938;
                                                                    }
                                                                } else {
                                                                    
                                                                    let pTab_0: *mut crate::src::headers::sqliteInt_h::Table = isSimpleCount(p, pAggInfo);
                                                                    if !pTab_0.is_null() {
                                                                        let __pTab_0_ref =
                                                                            unsafe { &*pTab_0 };
                                                                        let iDb_0: ::core::ffi::c_int = crate::src::src::prepare::sqlite3SchemaToIndex(
                                                                            
                                                                            __pParse_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                                                            
                                                                            __pTab_0_ref.pSchema as *mut crate::src::headers::sqliteInt_h::Schema,
                                                                        ) as ::core::ffi::c_int;
                                                                        let fresh12 =
                                                                            __pParse_ref.nTab;
                                                                        __pParse_ref.nTab += 1;
                                                                        let iCsr: ::core::ffi::c_int = fresh12;
                                                                        let mut pIdx: *mut crate::src::headers::sqliteInt_h::Index;
                                                                        let mut pKeyInfo_1: *mut crate::src::headers::sqliteInt_h::KeyInfo = ::core::ptr::null_mut::<
                                                                            crate::src::headers::sqliteInt_h::KeyInfo,
                                                                        >();
                                                                        let mut pBest: *mut crate::src::headers::sqliteInt_h::Index =
                                                                            ::core::ptr::null_mut::<
                                                                                crate::src::headers::sqliteInt_h::Index,
                                                                            >(
                                                                            );
                                                                        let mut iRoot: crate::src::src::pager::Pgno =
                                                                            __pTab_0_ref.tnum;
                                                                        crate::src::src::build::sqlite3CodeVerifySchema(
                                                                            
                                                                            pParse as *mut crate::src::headers::sqliteInt_h::Parse, iDb_0,
                                                                        );
                                                                        crate::src::src::build::sqlite3TableLock(
                                                                            
                                                                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                            iDb_0,
                                                                            __pTab_0_ref.tnum,
                                                                            0_u8,
                                                                            __pTab_0_ref.zName,
                                                                        );
                                                                        if (__pTab_0_ref.tabFlags
                                                                            & crate::src::headers::sqliteInt_h::TF_WithoutRowid
                                                                                as crate::src::ext::rtree::rtree::U32_0 != 0 as crate::src::ext::rtree::rtree::U32_0)
                                                                        {
                                                                            pBest =  crate::src::src::build::sqlite3PrimaryKeyIndex(pTab_0 as *mut crate::src::headers::sqliteInt_h::Table) as *mut crate::src::headers::sqliteInt_h::Index;
                                                                        }
                                                                        if (*(&raw mut (*__p_ref.pSrc).a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                                                                            .offset(0_isize))
                                                                            .fg
                                                                            .notIndexed() == 0
                                                                        {
                                                                            pIdx = __pTab_0_ref.pIndex;
                                                                            while !pIdx.is_null() {
                                                                                if (*pIdx).bUnordered() as ::core::ffi::c_int
                                                                                    == 0 as ::core::ffi::c_int
                                                                                    && ((*pIdx).szIdxRow as ::core::ffi::c_int)
                                                                                        < __pTab_0_ref.szTabRow as ::core::ffi::c_int
                                                                                    && (*pIdx).pPartIdxWhere.is_null()
                                                                                    && (pBest.is_null()
                                                                                        || ((*pIdx).szIdxRow as ::core::ffi::c_int)
                                                                                            < (*pBest).szIdxRow as ::core::ffi::c_int)
                                                                                {
                                                                                    pBest = pIdx;
                                                                                }
                                                                                pIdx = (*pIdx).pNext;
                                                                            }
                                                                        }
                                                                        if !pBest.is_null() {
                                                                            iRoot = (*pBest).tnum;
                                                                            pKeyInfo_1 =  crate::src::src::build::sqlite3KeyInfoOfIndex(pParse as *mut crate::src::headers::sqliteInt_h::Parse,  pBest as *mut crate::src::headers::sqliteInt_h::Index) as
    *mut crate::src::headers::sqliteInt_h::KeyInfo;
                                                                        }
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                                                                            v,
                                                                            crate::src::headers::opcodes_h::OP_OpenRead,
                                                                            iCsr,
                                                                            iRoot as ::core::ffi::c_int,
                                                                            iDb_0,
                                                                            1 as ::core::ffi::c_int,
                                                                        );
                                                                        if !pKeyInfo_1.is_null() {
                                                                            crate::src::src::vdbeaux::sqlite3VdbeChangeP4(
                                                                                v,
                                                                                -(1 as ::core::ffi::c_int),
                                                                                pKeyInfo_1 as *mut ::core::ffi::c_char,
                                                                                crate::src::src::vdbe::P4_KEYINFO,
                                                                            );
                                                                        }
                                                                        assignAggregateRegisters(
                                                                            pParse, pAggInfo,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                            v,
                                                                            crate::src::headers::opcodes_h::OP_Count,
                                                                            iCsr,
                                                                            __pAggInfo_ref.iFirstReg + __pAggInfo_ref.nColumn
                                                                                + 0 as ::core::ffi::c_int,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                                                                            v, crate::src::headers::opcodes_h::OP_Close, iCsr,
                                                                        );
                                                                        explainSimpleCount(
                                                                            pParse, pTab_0, pBest,
                                                                        );
                                                                        current_block =
                                                                            14953953536506160536;
                                                                    } else {
                                                                        let mut regAcc: ::core::ffi::c_int = 0
                                                                            as ::core::ffi::c_int;
                                                                        let mut pDistinct_0: *mut crate::src::headers::sqliteInt_h::ExprList = ::core::ptr::null_mut::<
                                                                            crate::src::headers::sqliteInt_h::ExprList,
                                                                        >();
                                                                        let mut distFlag_0: crate::src::fts5::U16_0 =
                                                                            0 as crate::src::fts5::U16_0;
                                                                        let eDist_0: ::core::ffi::c_int;
                                                                        if __pAggInfo_ref.nAccumulator != 0 {
                                                                            i = 0 as ::core::ffi::c_int;
                                                                            while i < __pAggInfo_ref.nFunc {
                                                                                if ((*(*__pAggInfo_ref.aFunc.offset(i as isize)).pFExpr).flags
                                                                                    & 0x1000000 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0 == 0 as crate::src::ext::rtree::rtree::U32_0)
                                                                                    && (*(*__pAggInfo_ref.aFunc.offset(i as isize)).pFunc)
                                                                                        .funcFlags & crate::src::headers::sqliteInt_h::SQLITE_FUNC_NEEDCOLL as crate::src::ext::rtree::rtree::U32_0 != 0
                                                                                    {
                                                                                        break;
                                                                                    }
                                                                                i += 1;
                                                                            }
                                                                            if i == __pAggInfo_ref.nFunc {
                                                                                __pParse_ref.nMem += 1;
                                                                                regAcc = __pParse_ref.nMem;
                                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                                    v,
                                                                                    crate::src::headers::opcodes_h::OP_Integer,
                                                                                    0 as ::core::ffi::c_int,
                                                                                    regAcc,
                                                                                );
                                                                            }
                                                                        } else if __pAggInfo_ref.nFunc == 1 as ::core::ffi::c_int
                                                                            && (*(*pAggInfo)
                                                                                .aFunc
                                                                                .offset(0_isize))
                                                                                .iDistinct >= 0 as ::core::ffi::c_int
                                                                        {
                                                                            pDistinct_0 = (*(*(*pAggInfo)
                                                                                .aFunc
                                                                                .offset(0_isize))
                                                                                .pFExpr)
                                                                                .x
                                                                                .pList;
                                                                            distFlag_0 = (if !pDistinct_0.is_null() {
                                                                                crate::src::headers::sqliteInt_h::WHERE_WANT_DISTINCT | crate::src::headers::sqliteInt_h::WHERE_AGG_DISTINCT
                                                                            } else {
                                                                                0 as ::core::ffi::c_int
                                                                            }) as crate::src::fts5::U16_0;
                                                                        }
                                                                        assignAggregateRegisters(
                                                                            pParse, pAggInfo,
                                                                        );
                                                                        resetAccumulator(
                                                                            pParse, pAggInfo,
                                                                        );
                                                                        pWInfo = crate::src::src::r#where::sqlite3WhereBegin(
                                                                            
                                                                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                            
                                                                            pTabList as *mut crate::src::headers::sqliteInt_h::SrcList,
                                                                            
                                                                            pWhere as *mut crate::src::headers::sqliteInt_h::Expr,
                                                                            
                                                                            pMinMaxOrderBy as *mut crate::src::headers::sqliteInt_h::ExprList,
                                                                            
                                                                            pDistinct_0 as *mut crate::src::headers::sqliteInt_h::ExprList,
                                                                            
                                                                            p as *mut crate::src::headers::sqliteInt_h::Select,
                                                                            (minMaxFlag as ::core::ffi::c_int
                                                                                | distFlag_0 as ::core::ffi::c_int) as crate::src::fts5::U16_0,
                                                                            0 as ::core::ffi::c_int,
                                                                        );
                                                                        if pWInfo.is_null() {
                                                                            current_block =
                                                                                9427106068226466434;
                                                                        } else {
                                                                            eDist_0 = crate::src::src::r#where::sqlite3WhereIsDistinct(pWInfo);
                                                                            updateAccumulator(
                                                                                pParse, regAcc,
                                                                                pAggInfo, eDist_0,
                                                                            );
                                                                            if eDist_0 != crate::src::headers::sqliteInt_h::WHERE_DISTINCT_NOOP {
                                                                                let pF_0: *mut crate::src::headers::sqliteInt_h::AggInfo_func = __pAggInfo_ref.aFunc
                                                                                    as *mut crate::src::headers::sqliteInt_h::AggInfo_func;
                                                                                if !pF_0.is_null() {
                                                                                    fixDistinctOpenEph(
                                                                                        pParse,
                                                                                        eDist_0,
                                                                                        (*pF_0).iDistinct,
                                                                                        (*pF_0).iDistAddr,
                                                                                    );
                                                                                }
                                                                            }
                                                                            if regAcc != 0 {
                                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                                    v,
                                                                                    crate::src::headers::opcodes_h::OP_Integer,
                                                                                    1 as ::core::ffi::c_int,
                                                                                    regAcc,
                                                                                );
                                                                            }
                                                                            if minMaxFlag != 0 {
                                                                                crate::src::src::r#where::sqlite3WhereMinMaxOptEarlyOut(v, pWInfo);
                                                                            }
                                                                            crate::src::src::r#where::sqlite3WhereEnd(pWInfo);
                                                                            finalizeAggFunctions(
                                                                                pParse, pAggInfo,
                                                                            );
                                                                            current_block = 14953953536506160536;
                                                                        }
                                                                    }
                                                                    match current_block {
                                                                        9427106068226466434 => {}
                                                                        _ => {
                                                                            sSort.pOrderBy = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
                                                                            crate::src::src::expr::sqlite3ExprIfFalse(
                                                                                
                                                                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                                                                
                                                                                pHaving as *mut crate::src::headers::sqliteInt_h::Expr,
                                                                                addrEnd,
                                                                                crate::src::headers::sqliteInt_h::SQLITE_JUMPIFNULL,
                                                                            );
                                                                            selectInnerLoop(
                                                                                pParse,
                                                                                p,
                                                                                -(1 as ::core::ffi::c_int),
                                                                                ::core::ptr::null_mut::<SortCtx>(),
                                                                                ::core::ptr::null_mut::<DistinctCtx>(),
                                                                                pDest,
                                                                                addrEnd,
                                                                                addrEnd,
                                                                            );
                                                                            current_block =
                                                                                2969779015960460938;
                                                                        }
                                                                    }
                                                                }
                                                                match current_block {
                                                                    9427106068226466434 => {}
                                                                    _ => {
                                                                        crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(
                                                                            v, addrEnd,
                                                                        );
                                                                        current_block =
                                                                            6198430992841073810;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    match current_block {
                                                        9427106068226466434 => {}
                                                        _ => {
                                                            if sDistinct.eTnctType
                                                                as ::core::ffi::c_int
                                                                == crate::src::headers::sqliteInt_h::WHERE_DISTINCT_UNORDERED
                                                            {
                                                                explainTempTable(
                                                                    pParse,
                                                                    b"DISTINCT\0" as *const u8 as *const ::core::ffi::c_char,
                                                                );
                                                            }
                                                            if !sSort.pOrderBy.is_null() {
                                                                generateSortTail(
                                                                    pParse,
                                                                    p,
                                                                    &raw mut sSort,
                                                                    (*pEList).nExpr,
                                                                    pDest,
                                                                );
                                                            }
                                                            crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, iEnd);
                                                            rc = (__pParse_ref.nErr
                                                                > 0 as ::core::ffi::c_int)
                                                                as ::core::ffi::c_int;
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
    crate::src::src::expr::sqlite3ExprListDelete(
        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
        pMinMaxOrderBy as *mut crate::src::headers::sqliteInt_h::ExprList,
    );
    crate::src::src::vdbeaux::sqlite3VdbeExplainPop(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
    );
    rc
}
