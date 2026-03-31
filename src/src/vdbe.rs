














// =============== BEGIN vdbe_h ================
pub type Mem =  crate::src::headers::vdbeInt_h::sqlite3_value;
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct SubrtnSig {
        pub selId: ::core::ffi::c_int,
        pub bComplete: crate::src::ext::rtree::rtree::u8_0,
        pub zAff: *mut ::core::ffi::c_char,
        pub iTable: ::core::ffi::c_int,
        pub iAddr: ::core::ffi::c_int,
        pub regReturn: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct VdbeOp {
        pub opcode: crate::src::ext::rtree::rtree::u8_0,
        pub p4type: ::core::ffi::c_schar,
        pub p5: crate::src::fts5::u16_0,
        pub p1: ::core::ffi::c_int,
        pub p2: ::core::ffi::c_int,
        pub p3: ::core::ffi::c_int,
        pub p4: crate::src::src::vdbe::p4union,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub union p4union {
        pub i: ::core::ffi::c_int,
        pub p: *mut ::core::ffi::c_void,
        pub z: *mut ::core::ffi::c_char,
        pub pI64: *mut crate::src::ext::rtree::rtree::i64_0,
        pub pReal: *mut ::core::ffi::c_double,
        pub pFunc: *mut crate::src::headers::sqliteInt_h::FuncDef,
        pub pCtx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
        pub pColl: *mut crate::src::headers::sqliteInt_h::CollSeq,
        pub pMem: *mut crate::src::src::vdbe::Mem,
        pub pVtab: *mut crate::src::headers::sqliteInt_h::VTable,
        pub pKeyInfo: *mut crate::src::headers::sqliteInt_h::KeyInfo,
        pub ai: *mut crate::src::ext::rtree::rtree::u32_0,
        pub pProgram: *mut crate::src::src::vdbe::SubProgram,
        pub pTab: *mut crate::src::headers::sqliteInt_h::Table,
        pub pSubrtnSig: *mut crate::src::src::vdbe::SubrtnSig,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct SubProgram {
        pub aOp: *mut crate::src::src::vdbe::VdbeOp,
        pub nOp: ::core::ffi::c_int,
        pub nMem: ::core::ffi::c_int,
        pub nCsr: ::core::ffi::c_int,
        pub aOnce: *mut crate::src::ext::rtree::rtree::u8_0,
        pub token: *mut ::core::ffi::c_void,
        pub pNext: *mut crate::src::src::vdbe::SubProgram,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct VdbeOpList {
        pub opcode: crate::src::ext::rtree::rtree::u8_0,
        pub p1: ::core::ffi::c_schar,
        pub p2: ::core::ffi::c_schar,
        pub p3: ::core::ffi::c_schar,
    }
    
    pub const P4_NOTUSED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    pub const P4_TRANSIENT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    pub const P4_STATIC: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    
    pub const P4_COLLSEQ: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
    
    pub const P4_COLLSEQ_1: ::core::ffi::c_int = -2;
    
    pub const P4_INT32: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
    
    pub const P4_SUBPROGRAM: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
    
    pub const P4_TABLE: ::core::ffi::c_int = -(5 as ::core::ffi::c_int);
    
    pub const P4_TABLE_1: ::core::ffi::c_int = -5;
    
    pub const P4_FREE_IF_LE: ::core::ffi::c_int = -(6 as ::core::ffi::c_int);
    
    pub const P4_DYNAMIC: ::core::ffi::c_int = -(6 as ::core::ffi::c_int);
    
    pub const P4_FUNCDEF: ::core::ffi::c_int = -(7 as ::core::ffi::c_int);
    
    pub const P4_FUNCDEF_1: ::core::ffi::c_int = -7;
    
    pub const P4_KEYINFO: ::core::ffi::c_int = -(8 as ::core::ffi::c_int);
    
    pub const P4_KEYINFO_1: ::core::ffi::c_int = -8;
    
    pub const P4_MEM: ::core::ffi::c_int = -(10 as ::core::ffi::c_int);
    
    pub const P4_MEM_1: ::core::ffi::c_int = -10;
    
    pub const P4_VTAB: ::core::ffi::c_int = -(11 as ::core::ffi::c_int);
    
    pub const P4_REAL: ::core::ffi::c_int = -(12 as ::core::ffi::c_int);
    
    pub const P4_REAL_1: ::core::ffi::c_int = -12;
    
    pub const P4_INT64: ::core::ffi::c_int = -(13 as ::core::ffi::c_int);
    
    pub const P4_INT64_1: ::core::ffi::c_int = -13;
    
    pub const P4_INTARRAY: ::core::ffi::c_int = -(14 as ::core::ffi::c_int);
    
    pub const P4_INTARRAY_1: ::core::ffi::c_int = -14;
    
    pub const P4_FUNCCTX: ::core::ffi::c_int = -(15 as ::core::ffi::c_int);
    
    pub const P4_FUNCCTX_1: ::core::ffi::c_int = -15;
    
    pub const P4_TABLEREF: ::core::ffi::c_int = -(16 as ::core::ffi::c_int);
    
    pub const P4_TABLEREF_1: ::core::ffi::c_int = -16;
    
    pub const P4_SUBRTNSIG: ::core::ffi::c_int = -(17 as ::core::ffi::c_int);
    
    pub const P4_SUBRTNSIG_1: ::core::ffi::c_int = -17;
    
    pub const P5_ConstraintNotNull: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const P5_ConstraintUnique: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    pub const P5_ConstraintCheck: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    pub const P5_ConstraintFK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    pub const COLNAME_NAME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    pub const COLNAME_DECLTYPE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    
    pub const COLNAME_N: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    
    pub const SQLITE_PREPARE_SAVESQL: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    
    pub const SQLITE_PREPARE_MASK: ::core::ffi::c_int = 0x1f as ::core::ffi::c_int;
    
    pub type RecordCompare = Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            *mut crate::src::headers::sqliteInt_h::UnpackedRecord,
        ) -> ::core::ffi::c_int,
    >;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::src::btree::sqlite3BtreeBeginStmt;pub use crate::src::src::btree::sqlite3BtreeBeginTrans;pub use crate::src::src::btree::sqlite3BtreeClearCursor;pub use crate::src::src::btree::sqlite3BtreeClearTable;pub use crate::src::src::btree::sqlite3BtreeClearTableOfCursor;pub use crate::src::src::btree::sqlite3BtreeClose;pub use crate::src::src::btree::sqlite3BtreeCount;pub use crate::src::src::btree::sqlite3BtreeCreateTable;pub use crate::src::src::btree::sqlite3BtreeCursor;pub use crate::src::src::btree::sqlite3BtreeCursorHasHint;pub use crate::src::src::btree::sqlite3BtreeCursorHasMoved;pub use crate::src::src::btree::sqlite3BtreeCursorHintFlags;pub use crate::src::src::btree::sqlite3BtreeCursorIsValidNN;pub use crate::src::src::btree::sqlite3BtreeCursorPin;pub use crate::src::src::btree::sqlite3BtreeCursorSize;pub use crate::src::src::btree::sqlite3BtreeCursorUnpin;pub use crate::src::src::btree::sqlite3BtreeCursorZero;pub use crate::src::src::btree::sqlite3BtreeDelete;pub use crate::src::src::btree::sqlite3BtreeDropTable;pub use crate::src::src::btree::sqlite3BtreeEof;pub use crate::src::src::btree::sqlite3BtreeFakeValidCursor;pub use crate::src::src::btree::sqlite3BtreeFirst;pub use crate::src::src::btree::sqlite3BtreeGetMeta;pub use crate::src::src::btree::sqlite3BtreeIncrVacuum;pub use crate::src::src::btree::sqlite3BtreeIndexMoveto;pub use crate::src::src::btree::sqlite3BtreeInsert;pub use crate::src::src::btree::sqlite3BtreeIntegerKey;pub use crate::src::src::btree::sqlite3BtreeIntegrityCheck;pub use crate::src::src::btree::sqlite3BtreeIsEmpty;pub use crate::src::src::btree::sqlite3BtreeLast;pub use crate::src::src::btree::sqlite3BtreeLastPage;pub use crate::src::src::btree::sqlite3BtreeLockTable;pub use crate::src::src::btree::sqlite3BtreeMaxPageCount;pub use crate::src::src::btree::sqlite3BtreeNext;pub use crate::src::src::btree::sqlite3BtreeOffset;pub use crate::src::src::btree::sqlite3BtreeOpen;pub use crate::src::src::btree::sqlite3BtreePager;pub use crate::src::src::btree::sqlite3BtreePayload;pub use crate::src::src::btree::sqlite3BtreePayloadFetch;pub use crate::src::src::btree::sqlite3BtreePayloadSize;pub use crate::src::src::btree::sqlite3BtreePrevious;pub use crate::src::src::btree::sqlite3BtreeRowCountEst;pub use crate::src::src::btree::sqlite3BtreeSavepoint;pub use crate::src::src::btree::sqlite3BtreeSetVersion;pub use crate::src::src::btree::sqlite3BtreeTableMoveto;pub use crate::src::src::btree::sqlite3BtreeTransferRow;pub use crate::src::src::btree::sqlite3BtreeTripAllCursors;pub use crate::src::src::btree::sqlite3BtreeUpdateMeta;pub use crate::src::headers::btreeInt_h::BtCursor;pub use crate::src::headers::btreeInt_h::Btree;pub use crate::src::src::btree::BtreePayload;pub use crate::src::src::btree::BTREE_AUXDELETE;pub use crate::src::src::btree::BTREE_BLOBKEY;pub use crate::src::src::btree::BTREE_FILE_FORMAT;pub use crate::src::src::btree::BTREE_OMIT_JOURNAL;pub use crate::src::src::btree::BTREE_SCHEMA_VERSION;pub use crate::src::src::btree::BTREE_SEEK_EQ;pub use crate::src::src::btree::BTREE_SINGLE;pub use crate::src::src::btree::BTREE_UNORDERED;pub use crate::src::src::btree::BTREE_WRCSR;pub use crate::src::src::hash::Hash;pub use crate::src::src::hash::HashElem;pub use crate::src::src::hash::_ht;
pub use crate::internal::__ATOMIC_RELAXED;









pub use crate::src::headers::opcodes_h::OP_Add;pub use crate::src::headers::opcodes_h::OP_AddImm_1;pub use crate::src::headers::opcodes_h::OP_Affinity_1;pub use crate::src::headers::opcodes_h::OP_AggFinal_1;pub use crate::src::headers::opcodes_h::OP_AggInverse;pub use crate::src::headers::opcodes_h::OP_AggStep_1;pub use crate::src::headers::opcodes_h::OP_AggStep1;pub use crate::src::headers::opcodes_h::OP_AggValue;pub use crate::src::headers::opcodes_h::OP_And;pub use crate::src::headers::opcodes_h::OP_AutoCommit_1;pub use crate::src::headers::opcodes_h::OP_BeginSubrtn_1;pub use crate::src::headers::opcodes_h::OP_BitAnd;pub use crate::src::headers::opcodes_h::OP_BitNot;pub use crate::src::headers::opcodes_h::OP_BitOr;pub use crate::src::headers::opcodes_h::OP_Blob_1;pub use crate::src::headers::opcodes_h::OP_Cast_1;pub use crate::src::headers::opcodes_h::OP_Checkpoint_1;pub use crate::src::headers::opcodes_h::OP_Clear_1;pub use crate::src::headers::opcodes_h::OP_Close_1;pub use crate::src::headers::opcodes_h::OP_ClrSubtype_1;pub use crate::src::headers::opcodes_h::OP_CollSeq_1;pub use crate::src::headers::opcodes_h::OP_Column_1;pub use crate::src::headers::opcodes_h::OP_Compare_1;pub use crate::src::headers::opcodes_h::OP_Concat_1;pub use crate::src::headers::opcodes_h::OP_Copy_1;pub use crate::src::headers::opcodes_h::OP_Count_1;pub use crate::src::headers::opcodes_h::OP_CreateBtree_1;pub use crate::src::headers::opcodes_h::OP_CursorLock_1;pub use crate::src::headers::opcodes_h::OP_CursorUnlock_1;pub use crate::src::headers::opcodes_h::OP_DecrJumpZero_1;pub use crate::src::headers::opcodes_h::OP_DeferredSeek;pub use crate::src::headers::opcodes_h::OP_Delete_1;pub use crate::src::headers::opcodes_h::OP_Destroy_1;pub use crate::src::headers::opcodes_h::OP_Divide;pub use crate::src::headers::opcodes_h::OP_DropIndex_1;pub use crate::src::headers::opcodes_h::OP_DropTable_1;pub use crate::src::headers::opcodes_h::OP_DropTrigger_1;pub use crate::src::headers::opcodes_h::OP_ElseEq_1;pub use crate::src::headers::opcodes_h::OP_EndCoroutine;pub use crate::src::headers::opcodes_h::OP_Eq_1;pub use crate::src::headers::opcodes_h::OP_Expire_1;pub use crate::src::headers::opcodes_h::OP_Filter_1;pub use crate::src::headers::opcodes_h::OP_FilterAdd_1;pub use crate::src::headers::opcodes_h::OP_FinishSeek_1;pub use crate::src::headers::opcodes_h::OP_FkCheck_1;pub use crate::src::headers::opcodes_h::OP_FkCounter_1;pub use crate::src::headers::opcodes_h::OP_FkIfZero_1;pub use crate::src::headers::opcodes_h::OP_Found;pub use crate::src::headers::opcodes_h::OP_Function;pub use crate::src::headers::opcodes_h::OP_Ge;pub use crate::src::headers::opcodes_h::OP_GetSubtype_1;pub use crate::src::headers::opcodes_h::OP_Gosub_1;pub use crate::src::headers::opcodes_h::OP_Goto_1;pub use crate::src::headers::opcodes_h::OP_Gt;pub use crate::src::headers::opcodes_h::OP_Halt_1;pub use crate::src::headers::opcodes_h::OP_HaltIfNull_1;pub use crate::src::headers::opcodes_h::OP_IdxDelete_1;pub use crate::src::headers::opcodes_h::OP_IdxGE;pub use crate::src::headers::opcodes_h::OP_IdxGT_1;pub use crate::src::headers::opcodes_h::OP_IdxInsert_1;pub use crate::src::headers::opcodes_h::OP_IdxLE_1;pub use crate::src::headers::opcodes_h::OP_IdxLT;pub use crate::src::headers::opcodes_h::OP_IdxRowid_1;pub use crate::src::headers::opcodes_h::OP_If_1;pub use crate::src::headers::opcodes_h::OP_IfEmpty_1;pub use crate::src::headers::opcodes_h::OP_IfNoHope;pub use crate::src::headers::opcodes_h::OP_IfNot_1;pub use crate::src::headers::opcodes_h::OP_IfNotOpen;pub use crate::src::headers::opcodes_h::OP_IfNotZero_1;pub use crate::src::headers::opcodes_h::OP_IfNullRow_1;pub use crate::src::headers::opcodes_h::OP_IfPos_1;pub use crate::src::headers::opcodes_h::OP_IfSizeBetween_1;pub use crate::src::headers::opcodes_h::OP_IncrVacuum_1;pub use crate::src::headers::opcodes_h::OP_Init;pub use crate::src::headers::opcodes_h::OP_InitCoroutine_1;pub use crate::src::headers::opcodes_h::OP_Insert_1;pub use crate::src::headers::opcodes_h::OP_Int64_1;pub use crate::src::headers::opcodes_h::OP_IntCopy_1;pub use crate::src::headers::opcodes_h::OP_Integer_1;pub use crate::src::headers::opcodes_h::OP_IntegrityCk_1;pub use crate::src::headers::opcodes_h::OP_IsNull_1;pub use crate::src::headers::opcodes_h::OP_IsTrue_1;pub use crate::src::headers::opcodes_h::OP_IsType_1;pub use crate::src::headers::opcodes_h::OP_JournalMode_1;pub use crate::src::headers::opcodes_h::OP_Jump_1;pub use crate::src::headers::opcodes_h::OP_Last_1;pub use crate::src::headers::opcodes_h::OP_Le_1;pub use crate::src::headers::opcodes_h::OP_LoadAnalysis_1;pub use crate::src::headers::opcodes_h::OP_Lt;pub use crate::src::headers::opcodes_h::OP_MakeRecord_1;pub use crate::src::headers::opcodes_h::OP_MaxPgcnt_1;pub use crate::src::headers::opcodes_h::OP_MemMax_1;pub use crate::src::headers::opcodes_h::OP_Move_1;pub use crate::src::headers::opcodes_h::OP_Multiply;pub use crate::src::headers::opcodes_h::OP_MustBeInt_1;pub use crate::src::headers::opcodes_h::OP_Ne_1;pub use crate::src::headers::opcodes_h::OP_NewRowid_1;pub use crate::src::headers::opcodes_h::OP_Next_1;pub use crate::src::headers::opcodes_h::OP_NoConflict;pub use crate::src::headers::opcodes_h::OP_Not_1;pub use crate::src::headers::opcodes_h::OP_NotExists_1;pub use crate::src::headers::opcodes_h::OP_NotFound_1;pub use crate::src::headers::opcodes_h::OP_NotNull_1;pub use crate::src::headers::opcodes_h::OP_Null_1;pub use crate::src::headers::opcodes_h::OP_NullRow_1;pub use crate::src::headers::opcodes_h::OP_OffsetLimit_1;pub use crate::src::headers::opcodes_h::OP_Once;pub use crate::src::headers::opcodes_h::OP_OpenAutoindex;pub use crate::src::headers::opcodes_h::OP_OpenDup_1;pub use crate::src::headers::opcodes_h::OP_OpenEphemeral_1;pub use crate::src::headers::opcodes_h::OP_OpenPseudo_1;pub use crate::src::headers::opcodes_h::OP_OpenRead_1;pub use crate::src::headers::opcodes_h::OP_OpenWrite;pub use crate::src::headers::opcodes_h::OP_Or_1;pub use crate::src::headers::opcodes_h::OP_Pagecount_1;pub use crate::src::headers::opcodes_h::OP_Param_1;pub use crate::src::headers::opcodes_h::OP_ParseSchema;pub use crate::src::headers::opcodes_h::OP_Permutation_1;pub use crate::src::headers::opcodes_h::OP_Prev;pub use crate::src::headers::opcodes_h::OP_Program_1;pub use crate::src::headers::opcodes_h::OP_PureFunc;pub use crate::src::headers::opcodes_h::OP_ReadCookie_1;pub use crate::src::headers::opcodes_h::OP_Real_1;pub use crate::src::headers::opcodes_h::OP_RealAffinity_1;pub use crate::src::headers::opcodes_h::OP_Remainder;pub use crate::src::headers::opcodes_h::OP_ReopenIdx;pub use crate::src::headers::opcodes_h::OP_ResetCount_1;pub use crate::src::headers::opcodes_h::OP_ResetSorter_1;pub use crate::src::headers::opcodes_h::OP_ResultRow_1;pub use crate::src::headers::opcodes_h::OP_Return_1;pub use crate::src::headers::opcodes_h::OP_Rewind_1;pub use crate::src::headers::opcodes_h::OP_RowCell_1;pub use crate::src::headers::opcodes_h::OP_RowData_1;pub use crate::src::headers::opcodes_h::OP_RowSetAdd_1;pub use crate::src::headers::opcodes_h::OP_RowSetRead_1;pub use crate::src::headers::opcodes_h::OP_RowSetTest;pub use crate::src::headers::opcodes_h::OP_Rowid_1;pub use crate::src::headers::opcodes_h::OP_SCopy_1;pub use crate::src::headers::opcodes_h::OP_Savepoint_1;pub use crate::src::headers::opcodes_h::OP_SeekEnd;pub use crate::src::headers::opcodes_h::OP_SeekGE;pub use crate::src::headers::opcodes_h::OP_SeekGT;pub use crate::src::headers::opcodes_h::OP_SeekHit;pub use crate::src::headers::opcodes_h::OP_SeekLE;pub use crate::src::headers::opcodes_h::OP_SeekLT;pub use crate::src::headers::opcodes_h::OP_SeekRowid_1;pub use crate::src::headers::opcodes_h::OP_SeekScan;pub use crate::src::headers::opcodes_h::OP_Sequence_1;pub use crate::src::headers::opcodes_h::OP_SequenceTest_1;pub use crate::src::headers::opcodes_h::OP_SetCookie_1;pub use crate::src::headers::opcodes_h::OP_SetSubtype_1;pub use crate::src::headers::opcodes_h::OP_ShiftLeft;pub use crate::src::headers::opcodes_h::OP_ShiftRight;pub use crate::src::headers::opcodes_h::OP_SoftNull_1;pub use crate::src::headers::opcodes_h::OP_Sort_1;pub use crate::src::headers::opcodes_h::OP_SorterCompare_1;pub use crate::src::headers::opcodes_h::OP_SorterData_1;pub use crate::src::headers::opcodes_h::OP_SorterInsert_1;pub use crate::src::headers::opcodes_h::OP_SorterNext_1;pub use crate::src::headers::opcodes_h::OP_SorterOpen_1;pub use crate::src::headers::opcodes_h::OP_SorterSort_1;pub use crate::src::headers::opcodes_h::OP_SqlExec_1;pub use crate::src::headers::opcodes_h::OP_String;pub use crate::src::headers::opcodes_h::OP_String8_1;pub use crate::src::headers::opcodes_h::OP_Subtract_1;pub use crate::src::headers::opcodes_h::OP_TableLock_1;pub use crate::src::headers::opcodes_h::OP_Trace;pub use crate::src::headers::opcodes_h::OP_Transaction_1;pub use crate::src::headers::opcodes_h::OP_TypeCheck_1;pub use crate::src::headers::opcodes_h::OP_VBegin_1;pub use crate::src::headers::opcodes_h::OP_VCheck_1;pub use crate::src::headers::opcodes_h::OP_VColumn_1;pub use crate::src::headers::opcodes_h::OP_VCreate;pub use crate::src::headers::opcodes_h::OP_VDestroy_1;pub use crate::src::headers::opcodes_h::OP_VFilter;pub use crate::src::headers::opcodes_h::OP_VInitIn;pub use crate::src::headers::opcodes_h::OP_VNext;pub use crate::src::headers::opcodes_h::OP_VOpen_1;pub use crate::src::headers::opcodes_h::OP_VRename_1;pub use crate::src::headers::opcodes_h::OP_VUpdate_1;pub use crate::src::headers::opcodes_h::OP_Vacuum_1;pub use crate::src::headers::opcodes_h::OP_Variable_1;pub use crate::src::headers::opcodes_h::OP_Yield_1;pub use crate::src::headers::opcodes_h::OP_ZeroOrNull_1;pub use crate::src::src::pager::sqlite3PagerCloseWal;pub use crate::src::src::pager::sqlite3PagerFilename;pub use crate::src::src::pager::sqlite3PagerGetJournalMode;pub use crate::src::src::pager::sqlite3PagerOkToChangeJournalMode;pub use crate::src::src::pager::sqlite3PagerSetJournalMode;pub use crate::src::src::pager::sqlite3PagerWalSupported;pub use crate::src::src::pager::Pager;pub use crate::src::src::pager::Pgno;pub use crate::src::src::pager::PAGER_JOURNALMODE_MEMORY;pub use crate::src::src::pager::PAGER_JOURNALMODE_OFF;pub use crate::src::src::pager::PAGER_JOURNALMODE_QUERY;pub use crate::src::src::pager::PAGER_JOURNALMODE_WAL;pub use crate::src::headers::sqlite3_h::sqlite3_destructor_type;pub use crate::src::src::legacy::sqlite3_exec;pub use crate::src::headers::sqlite3_h::sqlite3_file;pub use crate::src::headers::sqlite3_h::sqlite3_filename;pub use crate::src::src::malloc::sqlite3_free;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;pub use crate::src::headers::sqlite3_h::sqlite3_index_info;pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;pub use crate::src::headers::sqlite3_h::sqlite3_int64;pub use crate::src::src::main::sqlite3_interrupt;pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;pub use crate::src::src::printf::sqlite3_log;pub use crate::src::src::malloc::sqlite3_malloc64;pub use crate::src::headers::sqlite3_h::sqlite3_mem_methods;pub use crate::src::headers::sqlite3_h::sqlite3_module;pub use crate::src::src::mutex_unix::sqlite3_mutex;pub use crate::src::headers::sqlite3_h::sqlite3_mutex_methods;pub use crate::src::headers::sqlite3_h::sqlite3_pcache;pub use crate::src::headers::sqlite3_h::sqlite3_pcache_methods2;pub use crate::src::headers::sqlite3_h::sqlite3_pcache_page;pub use crate::src::src::random::sqlite3_randomness;pub use crate::src::src::printf::sqlite3_snprintf;pub use crate::src::headers::sqlite3_h::sqlite3_syscall_ptr;pub use crate::src::headers::sqlite3_h::sqlite3_uint64;pub use crate::src::src::vdbeapi::sqlite3_value_text;pub use crate::src::src::vdbeapi::sqlite3_value_type;pub use crate::src::headers::sqlite3_h::sqlite3_vfs;pub use crate::src::headers::sqlite3_h::sqlite3_vtab;pub use crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;pub use crate::src::headers::sqlite3_h::sqlite_int64;pub use crate::src::headers::sqlite3_h::sqlite_uint64;pub use crate::src::headers::sqlite3_h::SQLITE_ABORT;pub use crate::src::headers::sqlite3_h::SQLITE_ABORT_ROLLBACK_1;pub use crate::src::headers::sqlite3_h::SQLITE_BUSY;pub use crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT;pub use crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT_DATATYPE;pub use crate::src::headers::sqlite3_h::SQLITE_CORRUPT;pub use crate::src::headers::sqlite3_h::SQLITE_CORRUPT_INDEX;pub use crate::src::headers::sqlite3_h::SQLITE_DELETE;pub use crate::src::headers::sqlite3_h::SQLITE_DONE;pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;pub use crate::src::headers::sqlite3_h::SQLITE_FULL;pub use crate::src::headers::sqlite3_h::SQLITE_INSERT;pub use crate::src::headers::sqlite3_h::SQLITE_INTERRUPT;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_CORRUPTFS;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_NOMEM;pub use crate::src::headers::sqlite3_h::SQLITE_LIMIT_LENGTH;pub use crate::src::headers::sqlite3_h::SQLITE_LIMIT_TRIGGER_DEPTH;pub use crate::src::headers::sqlite3_h::SQLITE_LOCKED;pub use crate::src::headers::sqlite3_h::SQLITE_MISMATCH;pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;pub use crate::src::headers::sqlite3_h::SQLITE_OK;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_CREATE;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_DELETEONCLOSE;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_EXCLUSIVE;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_READWRITE;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_TRANSIENT_DB;pub use crate::src::headers::sqlite3_h::SQLITE_READONLY;pub use crate::src::headers::sqlite3_h::SQLITE_RESULT_SUBTYPE;pub use crate::src::headers::sqlite3_h::SQLITE_ROW;pub use crate::src::headers::sqlite3_h::SQLITE_SCHEMA;pub use crate::src::headers::sqlite3_h::SQLITE_STATIC;pub use crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_FILTER_HIT;pub use crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_FILTER_MISS;pub use crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_RUN;pub use crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_SORT;pub use crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_VM_STEP;pub use crate::src::headers::sqlite3_h::SQLITE_TEXT;pub use crate::src::headers::sqlite3_h::SQLITE_TOOBIG;pub use crate::src::headers::sqlite3_h::SQLITE_TRACE_ROW;pub use crate::src::headers::sqlite3_h::SQLITE_TRACE_STMT;pub use crate::src::headers::sqlite3_h::SQLITE_UPDATE;pub use crate::src::headers::sqlite3_h::SQLITE_UTF8;pub use crate::src::headers::sqliteInt_h::__anon_struct_0;pub use crate::src::headers::sqliteInt_h::__anon_struct_1;pub use crate::src::headers::sqliteInt_h::__anon_struct_2;pub use crate::src::headers::sqliteInt_h::__anon_struct_3;pub use crate::src::headers::sqliteInt_h::__anon_struct_4;pub use crate::src::headers::sqliteInt_h::__anon_struct_5;pub use crate::src::headers::sqliteInt_h::__anon_struct_6;pub use crate::src::headers::sqliteInt_h::__anon_struct_7;pub use crate::src::headers::sqliteInt_h::__anon_struct_8;pub use crate::src::headers::sqliteInt_h::__anon_struct_9;pub use crate::src::headers::sqliteInt_h::__anon_union_0;pub use crate::src::headers::sqliteInt_h::__anon_union_1;pub use crate::src::headers::sqliteInt_h::__anon_union_10;pub use crate::src::headers::sqliteInt_h::__anon_union_11;pub use crate::src::headers::sqliteInt_h::__anon_union_12;pub use crate::src::headers::sqliteInt_h::__anon_union_13;pub use crate::src::headers::sqliteInt_h::__anon_union_15;pub use crate::src::headers::sqliteInt_h::__anon_union_2;pub use crate::src::headers::sqliteInt_h::__anon_union_3;pub use crate::src::headers::sqliteInt_h::__anon_union_4;pub use crate::src::headers::sqliteInt_h::__anon_union_5;pub use crate::src::headers::sqliteInt_h::__anon_union_6;pub use crate::src::headers::sqliteInt_h::__anon_union_7;pub use crate::src::headers::sqliteInt_h::__anon_union_8;pub use crate::src::headers::sqliteInt_h::__anon_union_9;pub use crate::src::headers::sqliteInt_h::bft;pub use crate::src::fts5::i16_0;pub use crate::src::ext::rtree::rtree::i64_0;pub use crate::src::headers::sqliteInt_h::i8_0;pub use crate::src::headers::sqliteInt_h::sColMap;pub use crate::src::headers::sqliteInt_h::sqlite3;pub use crate::src::src::util::sqlite3AddInt64;pub use crate::src::src::analyze::sqlite3AnalysisLoad;pub use crate::src::src::util::sqlite3AtoF;pub use crate::src::src::util::sqlite3Atoi64;pub use crate::src::src::main::sqlite3Checkpoint;pub use crate::src::src::main::sqlite3CloseSavepoints;pub use crate::src::src::global::sqlite3Config;pub use crate::src::src::main::sqlite3CorruptError;pub use crate::src::src::global::sqlite3CtypeMap;pub use crate::src::src::malloc::sqlite3DbFree;pub use crate::src::src::malloc::sqlite3DbFreeNN;pub use crate::src::src::malloc::sqlite3DbMallocRaw;pub use crate::src::src::malloc::sqlite3DbMallocRawNN;pub use crate::src::src::malloc::sqlite3DbMallocZero;pub use crate::src::src::malloc::sqlite3DbStrDup;pub use crate::src::src::main::sqlite3ErrStr;pub use crate::src::src::vdbeaux::sqlite3ExpirePreparedStatements;pub use crate::src::src::fkey::sqlite3FkClearTriggerCache;pub use crate::src::src::util::sqlite3GetVarint32;pub use crate::src::src::prepare::sqlite3InitCallback;pub use crate::src::headers::sqliteInt_h::sqlite3InitInfo;pub use crate::src::src::prepare::sqlite3InitOne;pub use crate::src::src::util::sqlite3IsNaN;pub use crate::src::src::pragma::sqlite3JournalModename;pub use crate::src::src::util::sqlite3LogEst;pub use crate::src::src::printf::sqlite3MPrintf;pub use crate::src::src::util::sqlite3MulInt64;pub use crate::src::src::malloc::sqlite3OomFault;pub use crate::src::src::util::sqlite3PutVarint;pub use crate::src::src::printf::sqlite3RCStrNew;pub use crate::src::src::printf::sqlite3RCStrRef;pub use crate::src::src::printf::sqlite3RCStrUnref;pub use crate::src::src::vdbemem::sqlite3RealSameAsInt;pub use crate::src::src::vdbemem::sqlite3RealToI64;pub use crate::src::src::main::sqlite3ReportError;pub use crate::src::src::build::sqlite3ResetAllSchemasOfConnection;pub use crate::src::src::build::sqlite3ResetOneSchema;pub use crate::src::src::main::sqlite3RollbackAll;pub use crate::src::src::build::sqlite3RootPageMoved;pub use crate::src::src::rowset::sqlite3RowSetInsert;pub use crate::src::src::rowset::sqlite3RowSetNext;pub use crate::src::src::rowset::sqlite3RowSetTest;pub use crate::src::src::vacuum::sqlite3RunVacuum;pub use crate::src::src::callback::sqlite3SchemaClear;pub use crate::src::src::global::sqlite3StdType;pub use crate::src::src::util::sqlite3StrICmp;pub use crate::src::src::util::sqlite3Strlen30;pub use crate::src::src::util::sqlite3SubInt64;pub use crate::src::src::util::sqlite3SystemError;pub use crate::src::src::build::sqlite3UnlinkAndDeleteIndex;pub use crate::src::src::build::sqlite3UnlinkAndDeleteTable;pub use crate::src::src::trigger::sqlite3UnlinkAndDeleteTrigger;pub use crate::src::src::vdbemem::sqlite3ValueText;pub use crate::src::src::util::sqlite3VarintLen;pub use crate::src::src::vdbeaux::sqlite3VdbeSetChanges;pub use crate::src::src::vtab::sqlite3VtabBegin;pub use crate::src::src::vtab::sqlite3VtabCallCreate;pub use crate::src::src::vtab::sqlite3VtabCallDestroy;pub use crate::src::src::vdbeaux::sqlite3VtabImportErrmsg;pub use crate::src::src::vtab::sqlite3VtabLock;pub use crate::src::src::vtab::sqlite3VtabSavepoint;pub use crate::src::src::vtab::sqlite3VtabUnlock;pub use crate::src::src::build::sqlite3WritableSchema;pub use crate::src::headers::sqliteInt_h::sqlite3_xauth;pub use crate::src::src::global::sqlite3aEQb;pub use crate::src::src::global::sqlite3aGTb;pub use crate::src::src::global::sqlite3aLTb;pub use crate::src::fts5::u16_0;pub use crate::src::ext::rtree::rtree::u32_0;pub use crate::src::ext::rtree::rtree::u64_0;pub use crate::src::ext::rtree::rtree::u8_0;pub use crate::src::headers::sqliteInt_h::yDbMask;pub use crate::src::headers::sqliteInt_h::ynVar;pub use crate::src::headers::sqliteInt_h::AggInfo;pub use crate::src::headers::sqliteInt_h::AggInfo_col;pub use crate::src::headers::sqliteInt_h::AggInfo_func;pub use crate::src::headers::sqliteInt_h::AutoincInfo;pub use crate::src::headers::sqliteInt_h::Bitmask;pub use crate::src::headers::sqliteInt_h::BusyHandler;pub use crate::src::headers::sqliteInt_h::CollSeq;pub use crate::src::headers::sqliteInt_h::Column;pub use crate::src::headers::sqliteInt_h::Cte;pub use crate::src::headers::sqliteInt_h::CteUse;pub use crate::src::headers::sqliteInt_h::DBFLAG_SchemaChange;pub use crate::src::headers::sqliteInt_h::DBFLAG_SchemaKnownOk;pub use crate::src::headers::sqliteInt_h::Db;pub use crate::src::headers::sqliteInt_h::DbClientData;pub use crate::src::headers::sqliteInt_h::Expr;pub use crate::src::headers::sqliteInt_h::ExprList;pub use crate::src::headers::sqliteInt_h::ExprList_item;pub use crate::src::headers::sqliteInt_h::FKey;pub use crate::src::headers::sqliteInt_h::FuncDef;pub use crate::src::headers::sqliteInt_h::FuncDestructor;pub use crate::src::headers::sqliteInt_h::IdList;pub use crate::src::headers::sqliteInt_h::IdList_item;pub use crate::src::headers::sqliteInt_h::Index;pub use crate::src::headers::sqliteInt_h::IndexedExpr;pub use crate::src::headers::sqliteInt_h::InitData;pub use crate::src::headers::sqliteInt_h::KeyInfo;pub use crate::src::headers::sqliteInt_h::LogEst;pub use crate::src::headers::sqliteInt_h::Lookaside;pub use crate::src::headers::sqliteInt_h::LookasideSlot;pub use crate::src::headers::sqliteInt_h::Module;pub use crate::src::headers::sqliteInt_h::OE_Abort;pub use crate::src::headers::sqliteInt_h::OE_Ignore;pub use crate::src::headers::sqliteInt_h::OE_Replace;pub use crate::src::headers::sqliteInt_h::Parse;pub use crate::src::headers::sqliteInt_h::ParseCleanup;pub use crate::src::headers::sqliteInt_h::RenameToken;pub use crate::src::headers::sqliteInt_h::Returning;pub use crate::src::src::rowset::RowSet;pub use crate::src::headers::sqliteInt_h::SQLITE_CorruptRdOnly;pub use crate::src::headers::sqliteInt_h::SQLITE_DeferFKs;pub use crate::src::headers::sqliteInt_h::SQLITE_LegacyAlter;pub use crate::src::headers::sqliteInt_h::SQLITE_QueryOnly;pub use crate::src::headers::sqliteInt_h::SQLITE_ReadUncommit;pub use crate::src::headers::sqliteInt_h::Savepoint;pub use crate::src::headers::sqliteInt_h::Schema;pub use crate::src::headers::sqliteInt_h::Select;pub use crate::src::headers::sqliteInt_h::Sqlite3Config;pub use crate::src::headers::sqliteInt_h::SrcItem;pub use crate::src::headers::sqliteInt_h::SrcList;pub use crate::src::headers::sqliteInt_h::Subquery;pub use crate::src::headers::sqliteInt_h::TF_WithoutRowid;pub use crate::src::headers::sqliteInt_h::Table;pub use crate::src::headers::sqliteInt_h::TableLock;pub use crate::src::headers::sqliteInt_h::Token;pub use crate::src::headers::sqliteInt_h::Trigger;pub use crate::src::headers::sqliteInt_h::TriggerPrg;pub use crate::src::headers::sqliteInt_h::TriggerStep;pub use crate::src::headers::sqliteInt_h::UnpackedRecord;pub use crate::src::headers::sqliteInt_h::Upsert;pub use crate::src::headers::sqliteInt_h::VList;pub use crate::src::headers::sqliteInt_h::VTable;pub use crate::src::headers::sqliteInt_h::VtabCtx;pub use crate::src::headers::sqliteInt_h::Window;pub use crate::src::headers::sqliteInt_h::With;pub use crate::src::headers::sqliteInt_h::COLFLAG_GENERATED;pub use crate::src::headers::sqliteInt_h::COLFLAG_VIRTUAL;pub use crate::src::headers::sqliteInt_h::COLTYPE_BLOB;pub use crate::src::headers::sqliteInt_h::COLTYPE_INT;pub use crate::src::headers::sqliteInt_h::COLTYPE_INTEGER_1;pub use crate::src::headers::sqliteInt_h::COLTYPE_REAL;pub use crate::src::headers::sqliteInt_h::COLTYPE_TEXT;pub use crate::src::headers::sqliteInt_h::KEYINFO_ORDER_BIGNULL;pub use crate::src::headers::sqliteInt_h::KEYINFO_ORDER_DESC;pub use crate::fts3Int_h::LARGEST_INT64;pub use crate::src::headers::sqliteInt_h::LARGEST_UINT64;pub use crate::src::headers::sqliteInt_h::LEGACY_SCHEMA_TABLE;pub use crate::src::headers::sqliteInt_h::OPFLAG_APPEND;pub use crate::src::headers::sqliteInt_h::OPFLAG_BULKCSR;pub use crate::src::headers::sqliteInt_h::OPFLAG_BYTELENARG;pub use crate::src::headers::sqliteInt_h::OPFLAG_FORDELETE;pub use crate::src::headers::sqliteInt_h::OPFLAG_ISNOOP;pub use crate::src::headers::sqliteInt_h::OPFLAG_ISUPDATE;pub use crate::src::headers::sqliteInt_h::OPFLAG_LASTROWID;pub use crate::src::headers::sqliteInt_h::OPFLAG_NCHANGE;pub use crate::src::headers::sqliteInt_h::OPFLAG_NOCHNG;pub use crate::src::headers::sqliteInt_h::OPFLAG_P2ISREG;pub use crate::src::headers::sqliteInt_h::OPFLAG_PERMUTE;pub use crate::src::headers::sqliteInt_h::OPFLAG_PREFORMAT;pub use crate::src::headers::sqliteInt_h::OPFLAG_SAVEPOSITION;pub use crate::src::headers::sqliteInt_h::OPFLAG_SEEKEQ;pub use crate::src::headers::sqliteInt_h::OPFLAG_TYPEOFARG;pub use crate::src::headers::sqliteInt_h::OPFLAG_USESEEKRESULT;pub use crate::src::headers::sqliteInt_h::SAVEPOINT_BEGIN;pub use crate::src::headers::sqliteInt_h::SAVEPOINT_RELEASE;pub use crate::src::headers::sqliteInt_h::SAVEPOINT_ROLLBACK;pub use crate::src::headers::sqliteInt_h::SCHEMA_ROOT;pub use crate::fts3Int_h::SMALLEST_INT64;pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_MASK;pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC;pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_REAL;pub use crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT;pub use crate::src::headers::sqliteInt_h::SQLITE_JUMPIFNULL;pub use crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;pub use crate::src::headers::sqliteInt_h::SQLITE_NULLEQ;pub use crate::src::headers::sqliteInt_h::SQLITE_TRACE_LEGACY;pub use crate::src::headers::stdlib::int16_t;pub use crate::src::headers::stdlib::int8_t;pub use crate::src::headers::stdlib::uint16_t;pub use crate::src::headers::stdlib::uint32_t;pub use crate::src::headers::stdlib::uint8_t;pub use crate::src::headers::stdlib::__int16_t;pub use crate::src::headers::stdlib::__int8_t;pub use crate::src::headers::stdlib::__uint16_t;pub use crate::src::headers::stdlib::__uint32_t;pub use crate::src::headers::stdlib::__uint8_t;pub use crate::src::headers::vdbeInt_h::__anon_struct_10;pub use crate::src::headers::vdbeInt_h::__anon_union_17;pub use crate::src::headers::vdbeInt_h::__anon_union_18;pub use crate::src::src::vdbeaux::sqlite3IntFloatCompare;pub use crate::src::src::vdbeaux::sqlite3SmallTypeSizes;pub use crate::src::src::vdbemem::sqlite3VdbeBooleanValue;pub use crate::src::src::vdbemem::sqlite3VdbeChangeEncoding;pub use crate::src::src::vdbeaux::sqlite3VdbeCheckFkDeferred;pub use crate::src::src::vdbeaux::sqlite3VdbeCheckFkImmediate;pub use crate::src::src::vdbeaux::sqlite3VdbeCursorRestore;pub use crate::src::src::vdbeaux::sqlite3VdbeDeleteAuxData;pub use crate::src::src::vdbeaux::sqlite3VdbeEnter;pub use crate::src::src::vdbeaux::sqlite3VdbeError;pub use crate::src::src::vdbeaux::sqlite3VdbeFinishMoveto;pub use crate::src::src::vdbeaux::sqlite3VdbeFrameMemDel;pub use crate::src::src::vdbeaux::sqlite3VdbeFrameRestore;pub use crate::src::src::vdbeaux::sqlite3VdbeFreeCursor;pub use crate::src::src::vdbeaux::sqlite3VdbeFreeCursorNN;pub use crate::src::src::vdbeaux::sqlite3VdbeHalt;pub use crate::src::src::vdbeaux::sqlite3VdbeHandleMovedCursor;pub use crate::src::src::vdbeaux::sqlite3VdbeIdxKeyCompare;pub use crate::src::src::vdbeaux::sqlite3VdbeIdxRowid;pub use crate::src::src::vdbemem::sqlite3VdbeIntValue;pub use crate::src::src::vdbemem::sqlite3VdbeIntegerAffinity;pub use crate::src::src::vdbeaux::sqlite3VdbeLeave;pub use crate::src::src::vdbemem::sqlite3VdbeMemAggValue;pub use crate::src::src::vdbemem::sqlite3VdbeMemCast;pub use crate::src::src::vdbemem::sqlite3VdbeMemClearAndResize;pub use crate::src::src::vdbemem::sqlite3VdbeMemCopy;pub use crate::src::src::vdbemem::sqlite3VdbeMemExpandBlob;pub use crate::src::src::vdbemem::sqlite3VdbeMemFinalize;pub use crate::src::src::vdbemem::sqlite3VdbeMemFromBtree;pub use crate::src::src::vdbemem::sqlite3VdbeMemFromBtreeZeroOffset;pub use crate::src::src::vdbemem::sqlite3VdbeMemGrow;pub use crate::src::src::vdbemem::sqlite3VdbeMemInit;pub use crate::src::src::vdbemem::sqlite3VdbeMemIntegerify;pub use crate::src::src::vdbemem::sqlite3VdbeMemMakeWriteable;pub use crate::src::src::vdbemem::sqlite3VdbeMemMove;pub use crate::src::src::vdbemem::sqlite3VdbeMemRealify;pub use crate::src::src::vdbemem::sqlite3VdbeMemRelease;pub use crate::src::src::vdbemem::sqlite3VdbeMemReleaseMalloc;pub use crate::src::src::vdbemem::sqlite3VdbeMemSetInt64;pub use crate::src::src::vdbemem::sqlite3VdbeMemSetNull;pub use crate::src::src::vdbemem::sqlite3VdbeMemSetPointer;pub use crate::src::src::vdbemem::sqlite3VdbeMemSetRowSet;pub use crate::src::src::vdbemem::sqlite3VdbeMemSetStr;pub use crate::src::src::vdbemem::sqlite3VdbeMemSetZeroBlob;pub use crate::src::src::vdbemem::sqlite3VdbeMemShallowCopy;pub use crate::src::src::vdbemem::sqlite3VdbeMemStringify;pub use crate::src::src::vdbemem::sqlite3VdbeMemTooBig;pub use crate::src::src::vdbeaux::sqlite3VdbeOneByteSerialTypeLen;pub use crate::src::src::vdbeaux::sqlite3VdbePreUpdateHook;pub use crate::src::src::vdbemem::sqlite3VdbeRealValue;pub use crate::src::src::vdbeaux::sqlite3VdbeSerialGet;pub use crate::src::src::vdbeaux::sqlite3VdbeSerialTypeLen;pub use crate::src::src::vdbesort::sqlite3VdbeSorterCompare;pub use crate::src::src::vdbesort::sqlite3VdbeSorterInit;pub use crate::src::src::vdbesort::sqlite3VdbeSorterNext;pub use crate::src::src::vdbesort::sqlite3VdbeSorterReset;pub use crate::src::src::vdbesort::sqlite3VdbeSorterRewind;pub use crate::src::src::vdbesort::sqlite3VdbeSorterRowkey;pub use crate::src::src::vdbesort::sqlite3VdbeSorterWrite;pub use crate::src::src::vdbeapi::sqlite3VdbeValueListFree;pub use crate::src::headers::vdbeInt_h::sqlite3_context;pub use crate::src::headers::vdbeInt_h::sqlite3_value;pub use crate::src::headers::vdbeInt_h::AuxData;pub use crate::src::headers::vdbeInt_h::Bool;pub use crate::src::headers::vdbeInt_h::MEM_AffMask;pub use crate::src::headers::vdbeInt_h::MEM_Agg;pub use crate::src::headers::vdbeInt_h::MEM_Blob;pub use crate::src::headers::vdbeInt_h::MEM_Cleared;pub use crate::src::headers::vdbeInt_h::MEM_Dyn;pub use crate::src::headers::vdbeInt_h::MEM_Ephem;pub use crate::src::headers::vdbeInt_h::MEM_FromBind;pub use crate::src::headers::vdbeInt_h::MEM_Int;pub use crate::src::headers::vdbeInt_h::MEM_IntReal;pub use crate::src::headers::vdbeInt_h::MEM_Null;pub use crate::src::headers::vdbeInt_h::MEM_Real;pub use crate::src::headers::vdbeInt_h::MEM_Static;pub use crate::src::headers::vdbeInt_h::MEM_Str;pub use crate::src::headers::vdbeInt_h::MEM_Subtype;pub use crate::src::headers::vdbeInt_h::MEM_Term;pub use crate::src::headers::vdbeInt_h::MEM_TypeMask;pub use crate::src::headers::vdbeInt_h::MEM_Undefined;pub use crate::src::headers::vdbeInt_h::MEM_Zero;pub use crate::src::headers::vdbeInt_h::MemValue;pub use crate::src::headers::vdbeInt_h::Op;pub use crate::src::headers::vdbeInt_h::PreUpdate;pub use crate::src::headers::vdbeInt_h::ValueList;pub use crate::src::headers::vdbeInt_h::Vdbe;pub use crate::src::headers::vdbeInt_h::VdbeCursor;pub use crate::src::headers::vdbeInt_h::VdbeFrame;pub use crate::src::headers::vdbeInt_h::VdbeSorter;pub use crate::src::headers::vdbeInt_h::VdbeTxtBlbCache;pub use crate::src::headers::vdbeInt_h::CACHE_STALE;pub use crate::src::headers::vdbeInt_h::CURTYPE_BTREE;pub use crate::src::headers::vdbeInt_h::CURTYPE_PSEUDO;pub use crate::src::headers::vdbeInt_h::CURTYPE_SORTER;pub use crate::src::headers::vdbeInt_h::CURTYPE_VTAB;pub use crate::src::headers::vdbeInt_h::MEMCELLSIZE;pub use crate::src::headers::vdbeInt_h::VDBE_HALT_STATE;pub use crate::src::headers::vdbeInt_h::VDBE_RUN_STATE;pub use crate::src::src::vdbeaux::sqlite3MemCompare;pub use crate::src::src::vdbeaux::sqlite3VdbeAllocUnpackedRecord;pub use crate::src::src::vdbetrace::sqlite3VdbeExpandSql;pub use crate::src::src::vdbeaux::sqlite3VdbeRecordCompareWithSkip;pub use crate::src::src::vdbeaux::sqlite3VdbeRecordUnpack;
#[unsafe(no_mangle)]

pub static mut sqlite3_search_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]

pub static mut sqlite3_interrupt_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]

pub static mut sqlite3_sort_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]

pub static mut sqlite3_max_blobsize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

unsafe extern "C" fn updateMaxBlobsize(mut p: *mut crate::src::src::vdbe::Mem) {
    if (*p).flags as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Str | crate::src::headers::vdbeInt_h::MEM_Blob) != 0 as ::core::ffi::c_int
        && (*p).n > sqlite3_max_blobsize
    {
        sqlite3_max_blobsize = (*p).n;
    }
}
#[unsafe(no_mangle)]

pub static mut sqlite3_found_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

unsafe extern "C" fn allocateCursor(
    mut p: *mut crate::src::headers::vdbeInt_h::Vdbe,
    mut iCur: ::core::ffi::c_int,
    mut nField: ::core::ffi::c_int,
    mut eCurType: crate::src::ext::rtree::rtree::u8_0,
) -> *mut crate::src::headers::vdbeInt_h::VdbeCursor {
    let mut pMem: *mut crate::src::src::vdbe::Mem = if iCur > 0 as ::core::ffi::c_int {
        (*p).aMem.offset(((*p).nMem - iCur) as isize) as *mut crate::src::src::vdbe::Mem
    } else {
        (*p).aMem
    };
    let mut nByte: crate::src::ext::rtree::rtree::i64_0 = 0;
    let mut pCx: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
    nByte = ((112 as usize).wrapping_add(7 as usize) & !(7 as ::core::ffi::c_int) as usize)
        .wrapping_add(
            ((nField + 1 as ::core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<crate::src::ext::rtree::rtree::u64_0>() as usize),
        ) as crate::src::ext::rtree::rtree::i64_0;
    if eCurType as ::core::ffi::c_int == crate::src::headers::vdbeInt_h::CURTYPE_BTREE {
        nByte += crate::src::src::btree::sqlite3BtreeCursorSize() as crate::src::ext::rtree::rtree::i64_0;
    }
    if !(*(*p).apCsr.offset(iCur as isize)).is_null() {
        crate::src::src::vdbeaux::sqlite3VdbeFreeCursorNN(p as *mut crate::src::headers::vdbeInt_h::Vdbe,  *(*p).apCsr.offset(iCur as isize) as *mut crate::src::headers::vdbeInt_h::VdbeCursor);
        let ref mut fresh19 = *(*p).apCsr.offset(iCur as isize);
        *fresh19 = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
    }
    if ((*pMem).szMalloc as crate::src::ext::rtree::rtree::i64_0) < nByte {
        let __pMem_ref = unsafe { &mut *pMem };
        if __pMem_ref.szMalloc > 0 as ::core::ffi::c_int {
            crate::src::src::malloc::sqlite3DbFreeNN(__pMem_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3, __pMem_ref.zMalloc as *mut ::core::ffi::c_void);
        }
        __pMem_ref.zMalloc =
            crate::src::src::malloc::sqlite3DbMallocRaw(__pMem_ref.db as *mut crate::src::headers::sqliteInt_h::sqlite3, nByte as crate::src::ext::rtree::rtree::u64_0) as *mut ::core::ffi::c_char;
        __pMem_ref.z = __pMem_ref.zMalloc;
        if __pMem_ref.zMalloc.is_null() {
            __pMem_ref.szMalloc = 0 as ::core::ffi::c_int;
            return ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
        }
        __pMem_ref.szMalloc = nByte as ::core::ffi::c_int;
    }
    pCx = (*pMem).zMalloc as *mut crate::src::headers::vdbeInt_h::VdbeCursor;
    let ref mut fresh20 = *(*p).apCsr.offset(iCur as isize);
    *fresh20 = pCx;
    ::libc::memset(
        pCx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        32 as crate::__stddef_size_t_h::size_t,
    );
    (*pCx).eCurType = eCurType;
    (*pCx).nField = nField as crate::src::fts5::i16_0;
    (*pCx).aOffset = (&raw mut (*pCx).aType as *mut crate::src::ext::rtree::rtree::u32_0).offset(nField as isize) as *mut crate::src::ext::rtree::rtree::u32_0;
    if eCurType as ::core::ffi::c_int == crate::src::headers::vdbeInt_h::CURTYPE_BTREE {
        (*pCx).uc.pCursor = (*pMem).z.offset(
            ((112 as usize).wrapping_add(7 as usize) & !(7 as ::core::ffi::c_int) as usize)
                .wrapping_add(
                    ((nField + 1 as ::core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<crate::src::ext::rtree::rtree::u64_0>() as usize),
                ) as isize,
        ) as *mut ::core::ffi::c_char as *mut crate::src::headers::btreeInt_h::BtCursor;
        crate::src::src::btree::sqlite3BtreeCursorZero((*pCx).uc.pCursor);
    }
    pCx
}

unsafe extern "C" fn alsoAnInt(
    mut pRec: *mut crate::src::src::vdbe::Mem,
    mut rValue: ::core::ffi::c_double,
    mut piValue: *mut crate::src::ext::rtree::rtree::i64_0,
) -> ::core::ffi::c_int {
    let mut iValue: crate::src::ext::rtree::rtree::i64_0 = 0;
    iValue = crate::src::src::vdbemem::sqlite3RealToI64(rValue);
    if crate::src::src::vdbemem::sqlite3RealSameAsInt(rValue, iValue as crate::src::headers::sqlite3_h::sqlite3_int64) != 0 {
        *piValue = iValue;
        return 1 as ::core::ffi::c_int;
    }
    let __pRec_ref = unsafe { &*pRec };
    (0 as ::core::ffi::c_int == crate::src::src::util::sqlite3Atoi64(__pRec_ref.z, piValue, __pRec_ref.n, __pRec_ref.enc))
        as ::core::ffi::c_int
}

unsafe extern "C" fn applyNumericAffinity(mut pRec: *mut crate::src::src::vdbe::Mem, mut bTryForInt: ::core::ffi::c_int) {
    let mut rValue: ::core::ffi::c_double = 0.;
    let __pRec_ref = unsafe { &mut *pRec };
    let mut enc: crate::src::ext::rtree::rtree::u8_0 = __pRec_ref.enc;
    let mut rc: ::core::ffi::c_int = 0;
    rc = crate::src::src::util::sqlite3AtoF(__pRec_ref.z, &raw mut rValue, __pRec_ref.n, enc);
    if rc <= 0 as ::core::ffi::c_int {
        return;
    }
    if rc == 1 as ::core::ffi::c_int && alsoAnInt(pRec, rValue, &raw mut __pRec_ref.u.i) != 0 {
        __pRec_ref.flags = (__pRec_ref.flags as ::core::ffi::c_int | crate::src::headers::vdbeInt_h::MEM_Int) as crate::src::fts5::u16_0;
    } else {
        __pRec_ref.u.r = rValue;
        __pRec_ref.flags = (__pRec_ref.flags as ::core::ffi::c_int | crate::src::headers::vdbeInt_h::MEM_Real) as crate::src::fts5::u16_0;
        if bTryForInt != 0 {
            crate::src::src::vdbemem::sqlite3VdbeIntegerAffinity(pRec as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
        }
    }
    __pRec_ref.flags = (__pRec_ref.flags as ::core::ffi::c_int & !crate::src::headers::vdbeInt_h::MEM_Str) as crate::src::fts5::u16_0;
}

unsafe extern "C" fn applyAffinity(
    mut pRec: *mut crate::src::src::vdbe::Mem,
    mut affinity: ::core::ffi::c_char,
    mut enc: crate::src::ext::rtree::rtree::u8_0,
) {
    if affinity as ::core::ffi::c_int >= crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC {
        if (*pRec).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Int == 0 as ::core::ffi::c_int {
            if (*pRec).flags as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_IntReal)
                == 0 as ::core::ffi::c_int
            {
                if (*pRec).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Str != 0 {
                    applyNumericAffinity(pRec, 1 as ::core::ffi::c_int);
                }
            } else if affinity as ::core::ffi::c_int <= crate::src::headers::sqliteInt_h::SQLITE_AFF_REAL {
                crate::src::src::vdbemem::sqlite3VdbeIntegerAffinity(pRec as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
            }
        }
    } else if affinity as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT {
        let __pRec_ref = unsafe { &mut *pRec };
        if 0 as ::core::ffi::c_int == __pRec_ref.flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Str {
            if __pRec_ref.flags as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_IntReal) != 0 {
                crate::src::src::vdbemem::sqlite3VdbeMemStringify(pRec as *mut crate::src::headers::vdbeInt_h::sqlite3_value, enc, 1 as crate::src::ext::rtree::rtree::u8_0);
            }
        }
        __pRec_ref.flags =
            (__pRec_ref.flags as ::core::ffi::c_int & !(crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_IntReal)) as crate::src::fts5::u16_0;
    }
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3_value_numeric_type(
    mut pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let mut eType: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_value_type(pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
    if eType == crate::src::headers::sqlite3_h::SQLITE_TEXT {
        let mut pMem: *mut crate::src::src::vdbe::Mem = pVal as *mut crate::src::src::vdbe::Mem;
        applyNumericAffinity(pMem, 0 as ::core::ffi::c_int);
        eType = crate::src::src::vdbeapi::sqlite3_value_type(pVal as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
    }
    eType
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3ValueApplyAffinity(
    mut pVal: *mut crate::src::headers::vdbeInt_h::sqlite3_value,
    mut affinity: crate::src::ext::rtree::rtree::u8_0,
    mut enc: crate::src::ext::rtree::rtree::u8_0,
) {
    applyAffinity(pVal as *mut crate::src::src::vdbe::Mem, affinity as ::core::ffi::c_char, enc);
}
#[inline(never)]

unsafe extern "C" fn computeNumericType(mut pMem: *mut crate::src::src::vdbe::Mem) -> crate::src::fts5::u16_0 {
    let mut rc: ::core::ffi::c_int = 0;
    let mut ix: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
    let __pMem_ref = unsafe { &mut *pMem };
    if if __pMem_ref.flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Zero != 0 {
        crate::src::src::vdbemem::sqlite3VdbeMemExpandBlob(pMem as *mut crate::src::headers::vdbeInt_h::sqlite3_value)
    } else {
        0 as ::core::ffi::c_int
    } != 0
    {
        __pMem_ref.u.i = 0 as crate::src::ext::rtree::rtree::i64_0;
        return crate::src::headers::vdbeInt_h::MEM_Int as crate::src::fts5::u16_0;
    }
    rc = crate::src::src::util::sqlite3AtoF(__pMem_ref.z, &raw mut __pMem_ref.u.r, __pMem_ref.n, __pMem_ref.enc);
    if rc <= 0 as ::core::ffi::c_int {
        if rc == 0 as ::core::ffi::c_int
            && crate::src::src::util::sqlite3Atoi64(__pMem_ref.z, &raw mut ix, __pMem_ref.n, __pMem_ref.enc)
                <= 1 as ::core::ffi::c_int
        {
            __pMem_ref.u.i = ix as crate::src::ext::rtree::rtree::i64_0;
            return crate::src::headers::vdbeInt_h::MEM_Int as crate::src::fts5::u16_0;
        } else {
            return crate::src::headers::vdbeInt_h::MEM_Real as crate::src::fts5::u16_0;
        }
    } else if rc == 1 as ::core::ffi::c_int
        && crate::src::src::util::sqlite3Atoi64(__pMem_ref.z, &raw mut ix, __pMem_ref.n, __pMem_ref.enc) == 0 as ::core::ffi::c_int
    {
        __pMem_ref.u.i = ix as crate::src::ext::rtree::rtree::i64_0;
        return crate::src::headers::vdbeInt_h::MEM_Int as crate::src::fts5::u16_0;
    }
    crate::src::headers::vdbeInt_h::MEM_Real as crate::src::fts5::u16_0
}

unsafe extern "C" fn numericType(mut pMem: *mut crate::src::src::vdbe::Mem) -> crate::src::fts5::u16_0 {
    if (*pMem).flags as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_IntReal | crate::src::headers::vdbeInt_h::MEM_Null) != 0 {
        return ((*pMem).flags as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_IntReal | crate::src::headers::vdbeInt_h::MEM_Null))
            as crate::src::fts5::u16_0;
    }
    computeNumericType(pMem)
}
#[inline(never)]

unsafe extern "C" fn out2PrereleaseWithClear(mut pOut: *mut crate::src::src::vdbe::Mem) -> *mut crate::src::src::vdbe::Mem {
    crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
    (*pOut).flags = crate::src::headers::vdbeInt_h::MEM_Int as crate::src::fts5::u16_0;
    pOut
}

unsafe extern "C" fn out2Prerelease(mut p: *mut crate::src::headers::vdbeInt_h::Vdbe, mut pOp: *mut crate::src::src::vdbe::VdbeOp) -> *mut crate::src::src::vdbe::Mem {
    let mut pOut: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
    pOut = (*p).aMem.offset((*pOp).p2 as isize) as *mut crate::src::src::vdbe::Mem;
    if (*pOut).flags as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Agg | crate::src::headers::vdbeInt_h::MEM_Dyn) != 0 as ::core::ffi::c_int {
        return out2PrereleaseWithClear(pOut);
    } else {
        (*pOut).flags = crate::src::headers::vdbeInt_h::MEM_Int as crate::src::fts5::u16_0;
        return pOut;
    };
}

unsafe extern "C" fn filterHash(mut aMem: *const crate::src::src::vdbe::Mem, mut pOp: *const crate::src::headers::vdbeInt_h::Op) -> crate::src::ext::rtree::rtree::u64_0 {
    let mut i: ::core::ffi::c_int = 0;
    let mut mx: ::core::ffi::c_int = 0;
    let mut h: crate::src::ext::rtree::rtree::u64_0 = 0 as crate::src::ext::rtree::rtree::u64_0;
    i = (*pOp).p3;
    mx = i + (*pOp).p4.i;
    while i < mx {
        let mut p: *const crate::src::src::vdbe::Mem = aMem.offset(i as isize) as *const crate::src::src::vdbe::Mem;
        let __p_ref = unsafe { &*p };
        if __p_ref.flags as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_IntReal) != 0 {
            h = h.wrapping_add(__p_ref.u.i as crate::src::ext::rtree::rtree::u64_0);
        } else if __p_ref.flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Real != 0 {
            h = h.wrapping_add(crate::src::src::vdbemem::sqlite3VdbeIntValue(p as *const crate::src::headers::vdbeInt_h::sqlite3_value) as crate::src::ext::rtree::rtree::u64_0);
        } else if __p_ref.flags as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Str | crate::src::headers::vdbeInt_h::MEM_Blob) != 0 {
            h = h.wrapping_add(
                (4093 as ::core::ffi::c_int
                    + (__p_ref.flags as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Str | crate::src::headers::vdbeInt_h::MEM_Blob)))
                    as crate::src::ext::rtree::rtree::u64_0,
            );
        }
        i += 1;
    }
    h
}
#[inline(never)]

unsafe extern "C" fn vdbeColumnFromOverflow(
    mut pC: *mut crate::src::headers::vdbeInt_h::VdbeCursor,
    mut iCol: ::core::ffi::c_int,
    mut t: crate::src::ext::rtree::rtree::u32_0,
    mut iOffset: crate::src::ext::rtree::rtree::i64_0,
    mut cacheStatus: crate::src::ext::rtree::rtree::u32_0,
    mut colCacheCtr: crate::src::ext::rtree::rtree::u32_0,
    mut pDest: *mut crate::src::src::vdbe::Mem,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let __pDest_ref = unsafe { &mut *pDest };
    let mut db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pDest_ref.db;
    let mut encoding: ::core::ffi::c_int = __pDest_ref.enc as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeSerialTypeLen(t) as ::core::ffi::c_int;
    if len > (*db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_LENGTH as usize] {
        return crate::src::headers::sqlite3_h::SQLITE_TOOBIG;
    }
    if len > 4000 as ::core::ffi::c_int && (*pC).pKeyInfo.is_null() {
        let mut pCache: *mut crate::src::headers::vdbeInt_h::VdbeTxtBlbCache = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeTxtBlbCache>();
        let mut pBuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let __pC_ref = unsafe { &mut *pC };
        if __pC_ref.colCache() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            __pC_ref.pCache =
                crate::src::src::malloc::sqlite3DbMallocZero(db as *mut crate::src::headers::sqliteInt_h::sqlite3, ::core::mem::size_of::<crate::src::headers::vdbeInt_h::VdbeTxtBlbCache>() as crate::src::ext::rtree::rtree::u64_0)
                    as *mut crate::src::headers::vdbeInt_h::VdbeTxtBlbCache;
            if __pC_ref.pCache.is_null() {
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
            __pC_ref.set_colCache(1 as crate::src::headers::vdbeInt_h::Bool as crate::src::headers::vdbeInt_h::Bool);
        }
        pCache = __pC_ref.pCache;
        if (*pCache).pCValue.is_null()
            || (*pCache).iCol != iCol
            || (*pCache).cacheStatus != cacheStatus
            || (*pCache).colCacheCtr != colCacheCtr
            || (*pCache).iOffset != crate::src::src::btree::sqlite3BtreeOffset(__pC_ref.uc.pCursor)
        {
            let __pCache_ref = unsafe { &mut *pCache };
            if !__pCache_ref.pCValue.is_null() {
                crate::src::src::printf::sqlite3RCStrUnref(__pCache_ref.pCValue as *mut ::core::ffi::c_void);
            }
            __pCache_ref.pCValue = crate::src::src::printf::sqlite3RCStrNew((len + 3 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::u64_0);
            pBuf = __pCache_ref.pCValue;
            if pBuf.is_null() {
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
            rc = crate::src::src::btree::sqlite3BtreePayload(
                __pC_ref.uc.pCursor,
                iOffset as crate::src::ext::rtree::rtree::u32_0,
                len as crate::src::ext::rtree::rtree::u32_0,
                pBuf as *mut ::core::ffi::c_void,
            );
            if rc != 0 {
                return rc;
            }
            *pBuf.offset(len as isize) = 0 as ::core::ffi::c_char;
            *pBuf.offset((len + 1 as ::core::ffi::c_int) as isize) = 0 as ::core::ffi::c_char;
            *pBuf.offset((len + 2 as ::core::ffi::c_int) as isize) = 0 as ::core::ffi::c_char;
            __pCache_ref.iCol = iCol;
            __pCache_ref.cacheStatus = cacheStatus;
            __pCache_ref.colCacheCtr = colCacheCtr;
            __pCache_ref.iOffset = crate::src::src::btree::sqlite3BtreeOffset(__pC_ref.uc.pCursor);
        } else {
            pBuf = (*pCache).pCValue;
        }
        crate::src::src::printf::sqlite3RCStrRef(pBuf);
        if t & 1 as crate::src::ext::rtree::rtree::u32_0 != 0 {
            rc = crate::src::src::vdbemem::sqlite3VdbeMemSetStr(
                
                pDest as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                pBuf,
                len as crate::src::ext::rtree::rtree::i64_0,
                encoding as crate::src::ext::rtree::rtree::u8_0,
                Some(crate::src::src::printf::sqlite3RCStrUnref as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            );
            __pDest_ref.flags = (__pDest_ref.flags as ::core::ffi::c_int | crate::src::headers::vdbeInt_h::MEM_Term) as crate::src::fts5::u16_0;
        } else {
            rc = crate::src::src::vdbemem::sqlite3VdbeMemSetStr(
                
                pDest as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                pBuf,
                len as crate::src::ext::rtree::rtree::i64_0,
                0 as crate::src::ext::rtree::rtree::u8_0,
                Some(crate::src::src::printf::sqlite3RCStrUnref as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            );
        }
    } else {
        rc = crate::src::src::vdbemem::sqlite3VdbeMemFromBtree((*pC).uc.pCursor, iOffset as crate::src::ext::rtree::rtree::u32_0, len as crate::src::ext::rtree::rtree::u32_0,  pDest as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
        if rc != 0 {
            return rc;
        }
        crate::src::src::vdbeaux::sqlite3VdbeSerialGet(__pDest_ref.z as *const ::core::ffi::c_uchar, t,  pDest as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
        if t & 1 as crate::src::ext::rtree::rtree::u32_0 != 0 as crate::src::ext::rtree::rtree::u32_0 && encoding == crate::src::headers::sqlite3_h::SQLITE_UTF8 {
            *__pDest_ref.z.offset(len as isize) = 0 as ::core::ffi::c_char;
            __pDest_ref.flags = (__pDest_ref.flags as ::core::ffi::c_int | crate::src::headers::vdbeInt_h::MEM_Term) as crate::src::fts5::u16_0;
        }
    }
    __pDest_ref.flags = (__pDest_ref.flags as ::core::ffi::c_int & !crate::src::headers::vdbeInt_h::MEM_Ephem) as crate::src::fts5::u16_0;
    rc
}
#[inline(never)]

unsafe extern "C" fn sqlite3VdbeLogAbort(
    mut p: *mut crate::src::headers::vdbeInt_h::Vdbe,
    mut rc: ::core::ffi::c_int,
    mut pOp: *mut crate::src::headers::vdbeInt_h::Op,
    mut aOp: *mut crate::src::headers::vdbeInt_h::Op,
) {
    let __p_ref = unsafe { &mut *p };
    let mut zSql: *const ::core::ffi::c_char = __p_ref.zSql;
    let mut zPrefix: *const ::core::ffi::c_char = b"\0" as *const u8 as *const ::core::ffi::c_char;
    let mut pc: ::core::ffi::c_int = 0;
    let mut zXtra: [::core::ffi::c_char; 100] = [0; 100];
    if !__p_ref.pFrame.is_null() {
        if !(*aOp.offset(0 as isize))
            .p4
            .z
            .is_null()
        {
            crate::src::src::printf::sqlite3_snprintf(
                ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as ::core::ffi::c_int,
                &raw mut zXtra as *mut ::core::ffi::c_char,
                b"/* %s */ \0" as *const u8 as *const ::core::ffi::c_char,
                (*aOp.offset(0 as isize))
                    .p4
                    .z
                    .offset(3 as isize),
            );
            zPrefix = &raw mut zXtra as *mut ::core::ffi::c_char;
        } else {
            zPrefix = b"/* unknown trigger */ \0" as *const u8 as *const ::core::ffi::c_char;
        }
    }
    pc = pOp.offset_from(aOp) as ::core::ffi::c_long as ::core::ffi::c_int;
    crate::src::src::printf::sqlite3_log(
        rc,
        b"statement aborts at %d: %s; [%s%s]\0" as *const u8 as *const ::core::ffi::c_char,
        pc,
        __p_ref.zErrMsg,
        zPrefix,
        zSql,
    );
}

unsafe extern "C" fn vdbeMemTypeName(mut pMem: *mut crate::src::src::vdbe::Mem) -> *const ::core::ffi::c_char {
    static mut azTypes: [*const ::core::ffi::c_char; 5] = [
        b"INT\0" as *const u8 as *const ::core::ffi::c_char,
        b"REAL\0" as *const u8 as *const ::core::ffi::c_char,
        b"TEXT\0" as *const u8 as *const ::core::ffi::c_char,
        b"BLOB\0" as *const u8 as *const ::core::ffi::c_char,
        b"NULL\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    azTypes
        [(crate::src::src::vdbeapi::sqlite3_value_type(pMem as *mut crate::src::headers::vdbeInt_h::sqlite3_value as *mut crate::src::headers::vdbeInt_h::sqlite3_value) - 1 as ::core::ffi::c_int) as usize]
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3VdbeExec(mut p: *mut crate::src::headers::vdbeInt_h::Vdbe) -> ::core::ffi::c_int {
    let mut pC_21: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
    let mut pC_8: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
    let mut pCrsr_1: *mut crate::src::headers::btreeInt_h::BtCursor = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>();
    let mut res_2: ::core::ffi::c_int = 0;
    let mut iKey_0: crate::src::ext::rtree::rtree::u64_0 = 0;
    let mut nField_0: ::core::ffi::c_int = 0;
    let mut pKeyInfo_0: *mut crate::src::headers::sqliteInt_h::KeyInfo = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::KeyInfo>();
    let mut p2_2: crate::src::ext::rtree::rtree::u32_0 = 0;
    let mut iDb_0: ::core::ffi::c_int = 0;
    let mut wrFlag: ::core::ffi::c_int = 0;
    let mut pX: *mut crate::src::headers::btreeInt_h::Btree = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::Btree>();
    let mut pCur: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
    let mut pDb_1: *mut crate::src::headers::sqliteInt_h::Db = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Db>();
    let mut current_block: u64;
    let __p_ref = unsafe { &mut *p };
    let mut aOp: *mut crate::src::headers::vdbeInt_h::Op = __p_ref.aOp;
    let mut pOp: *mut crate::src::headers::vdbeInt_h::Op = aOp;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __p_ref.db;
    let mut resetSchemaOnFault: crate::src::ext::rtree::rtree::u8_0 = 0 as crate::src::ext::rtree::rtree::u8_0;
    let mut encoding: crate::src::ext::rtree::rtree::u8_0 = (*db).enc;
    let mut iCompare: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nVmStep: crate::src::ext::rtree::rtree::u64_0 = 0 as crate::src::ext::rtree::rtree::u64_0;
    let mut nProgressLimit: crate::src::ext::rtree::rtree::u64_0 = 0;
    let mut aMem: *mut crate::src::src::vdbe::Mem = __p_ref.aMem;
    let mut pIn1: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
    let mut pIn2: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
    let mut pIn3: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
    let mut pOut: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
    let mut colCacheCtr: crate::src::ext::rtree::rtree::u32_0 = 0 as crate::src::ext::rtree::rtree::u32_0;
    if __p_ref.lockMask != 0 as crate::src::headers::sqliteInt_h::yDbMask {
        crate::src::src::vdbeaux::sqlite3VdbeEnter(p as *mut crate::src::headers::vdbeInt_h::Vdbe);
    }
    if (*db).xProgress.is_some() {
        let mut iPrior: crate::src::ext::rtree::rtree::u32_0 = __p_ref.aCounter[crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_VM_STEP as usize];
        nProgressLimit = ((*db).nProgressOps as crate::src::ext::rtree::rtree::u32_0)
            .wrapping_sub(iPrior.wrapping_rem((*db).nProgressOps as crate::src::ext::rtree::rtree::u32_0))
            as crate::src::ext::rtree::rtree::u64_0;
    } else {
        nProgressLimit = crate::src::headers::sqliteInt_h::LARGEST_UINT64;
    }
    if __p_ref.rc == crate::src::headers::sqlite3_h::SQLITE_NOMEM {
        current_block = 2471432538116610086;
    } else {
        __p_ref.rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        __p_ref.iCurrentTime = 0 as crate::src::ext::rtree::rtree::i64_0;
        (*db).busyHandler.nBusy = 0 as ::core::ffi::c_int;
        if (*((&raw mut (*db).u1.isInterrupted) as *mut std::sync::atomic::AtomicI32)).load(std::sync::atomic::Ordering::Relaxed) != 0 {
            current_block = 9771092749923633615;
        } else {
            pOp = aOp.offset(__p_ref.pc as isize) as *mut crate::src::headers::vdbeInt_h::Op;
            's_109: loop {
                nVmStep = nVmStep.wrapping_add(1);
                if sqlite3_interrupt_count > 0 as ::core::ffi::c_int {
                    sqlite3_interrupt_count -= 1;
                    if sqlite3_interrupt_count == 0 as ::core::ffi::c_int {
                        crate::src::src::main::sqlite3_interrupt(db as *mut crate::src::headers::sqliteInt_h::sqlite3);
                    }
                }
                match  (*pOp).opcode as ::core::ffi::c_int {
    crate::src::headers::opcodes_h::OP_Goto_1 =>  {
                        current_block = 16391045501586817625;
                    }
    crate::src::headers::opcodes_h::OP_Gosub_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        (*pIn1).flags = crate::src::headers::vdbeInt_h::MEM_Int as crate::src::fts5::u16_0;
                        (*pIn1).u.i = pOp.offset_from(aOp) as ::core::ffi::c_long
                            as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0;
                        current_block = 16391045501586817625;
                    }
    crate::src::headers::opcodes_h::OP_Return_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pIn1).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Int != 0 {
                            (*pOp).p3 != 0;
                            pOp = aOp.offset((*pIn1).u.i as isize) as *mut crate::src::headers::vdbeInt_h::Op;
                        } else {
                            (*pOp).p3 != 0;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_InitCoroutine_1 =>  {
                        let __pOp_ref = unsafe { &*pOp };
                        pOut = aMem.offset(__pOp_ref.p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        (*pOut).u.i = (__pOp_ref.p3 - 1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0;
                        (*pOut).flags = crate::src::headers::vdbeInt_h::MEM_Int as crate::src::fts5::u16_0;
                        if __pOp_ref.p2 == 0 as ::core::ffi::c_int {
                            current_block = 5783071609795492627;
                        } else {
                            current_block = 9512719473022792396;
                        }
                    }
    crate::src::headers::opcodes_h::OP_EndCoroutine =>  {
                        let mut pCaller: *mut crate::src::src::vdbe::VdbeOp = ::core::ptr::null_mut::<crate::src::src::vdbe::VdbeOp>();
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pCaller = aOp.offset((*pIn1).u.i as isize) as *mut crate::src::headers::vdbeInt_h::Op as *mut crate::src::src::vdbe::VdbeOp;
                        (*pIn1).u.i =
                            (pOp.offset_from(__p_ref.aOp) as ::core::ffi::c_long as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0;
                        pOp = aOp.offset(((*pCaller).p2 - 1 as ::core::ffi::c_int) as isize)
                            as *mut crate::src::headers::vdbeInt_h::Op;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Yield_1 =>  {
                        let mut pcDest: ::core::ffi::c_int = 0;
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        (*pIn1).flags = crate::src::headers::vdbeInt_h::MEM_Int as crate::src::fts5::u16_0;
                        pcDest = (*pIn1).u.i as ::core::ffi::c_int;
                        (*pIn1).u.i = pOp.offset_from(aOp) as ::core::ffi::c_long
                            as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0;
                        pOp = aOp.offset(pcDest as isize) as *mut crate::src::headers::vdbeInt_h::Op;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_HaltIfNull_1 =>  {
                        pIn3 = aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pIn3).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Null == 0 as ::core::ffi::c_int
                        {
                            current_block = 5783071609795492627;
                        } else {
                            current_block = 10435735846551762309;
                        }
                    }
    crate::src::headers::opcodes_h::OP_Halt_1 =>  {
                        current_block = 10435735846551762309;
                    }
    crate::src::headers::opcodes_h::OP_Integer_1 =>  {
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        (*pOut).u.i = (*pOp).p1 as crate::src::ext::rtree::rtree::i64_0;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Int64_1 =>  {
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        (*pOut).u.i = *(*pOp).p4.pI64;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Real_1 =>  {
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        (*pOut).flags = crate::src::headers::vdbeInt_h::MEM_Real as crate::src::fts5::u16_0;
                        (*pOut).u.r = *(*pOp).p4.pReal;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_String8_1 =>  {
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        let __pOp_ref = unsafe { &mut *pOp };
                        __pOp_ref.p1 = crate::src::src::util::sqlite3Strlen30(__pOp_ref.p4.z);
                        if encoding as ::core::ffi::c_int != crate::src::headers::sqlite3_h::SQLITE_UTF8 {
                            rc = crate::src::src::vdbemem::sqlite3VdbeMemSetStr(
                                
                                pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                __pOp_ref.p4.z,
                                -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0,
                                crate::src::headers::sqlite3_h::SQLITE_UTF8 as crate::src::ext::rtree::rtree::u8_0,
                                crate::src::headers::sqlite3_h::SQLITE_STATIC,
                            );
                            if rc != 0 {
                                current_block = 7113757447962609068;
                                break;
                            }
                            if crate::src::headers::sqlite3_h::SQLITE_OK
                                != crate::src::src::vdbemem::sqlite3VdbeChangeEncoding(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value, encoding as ::core::ffi::c_int)
                            {
                                current_block = 2471432538116610086;
                                break;
                            }
                            let __pOut_ref = unsafe { &mut *pOut };
                            __pOut_ref.szMalloc = 0 as ::core::ffi::c_int;
                            __pOut_ref.flags =
                                (__pOut_ref.flags as ::core::ffi::c_int | crate::src::headers::vdbeInt_h::MEM_Static) as crate::src::fts5::u16_0;
                            if __pOp_ref.p4type as ::core::ffi::c_int == crate::src::src::vdbe::P4_DYNAMIC {
                                crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, __pOp_ref.p4.z as *mut ::core::ffi::c_void);
                            }
                            __pOp_ref.p4type = crate::src::src::vdbe::P4_DYNAMIC as ::core::ffi::c_schar;
                            __pOp_ref.p4.z = __pOut_ref.z;
                            __pOp_ref.p1 = __pOut_ref.n;
                        }
                        if __pOp_ref.p1 > (*db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_LENGTH as usize] {
                            current_block = 7113757447962609068;
                            break;
                        }
                        __pOp_ref.opcode = crate::src::headers::opcodes_h::OP_String as crate::src::ext::rtree::rtree::u8_0;
                        current_block = 16937825661756021828;
                    }
    crate::src::headers::opcodes_h::OP_String =>  {
                        current_block = 16937825661756021828;
                    }
    crate::src::headers::opcodes_h::OP_BeginSubrtn_1 | crate::src::headers::opcodes_h::OP_Null_1 =>  {
                        let mut cnt: ::core::ffi::c_int = 0;
                        let mut nullFlag: crate::src::fts5::u16_0 = 0;
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        let __pOp_ref = unsafe { &*pOp };
                        cnt = __pOp_ref.p3 - __pOp_ref.p2;
                        nullFlag = (if __pOp_ref.p1 != 0 {
                            crate::src::headers::vdbeInt_h::MEM_Null | crate::src::headers::vdbeInt_h::MEM_Cleared
                        } else {
                            crate::src::headers::vdbeInt_h::MEM_Null
                        }) as crate::src::fts5::u16_0;
                        (*pOut).flags = nullFlag;
                        (*pOut).n = 0 as ::core::ffi::c_int;
                        while cnt > 0 as ::core::ffi::c_int {
                            pOut = pOut.offset(1);
                            crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                            (*pOut).flags = nullFlag;
                            (*pOut).n = 0 as ::core::ffi::c_int;
                            cnt -= 1;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_SoftNull_1 =>  {
                        pOut = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                            & !(crate::src::headers::vdbeInt_h::MEM_Undefined | crate::src::headers::vdbeInt_h::MEM_AffMask)
                            | crate::src::headers::vdbeInt_h::MEM_Null) as crate::src::fts5::u16_0;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Blob_1 =>  {
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        if (*pOp).p4.z.is_null() {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetZeroBlob(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value, (*pOp).p1);
                            if crate::src::src::vdbemem::sqlite3VdbeMemExpandBlob(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value) != 0 {
                                current_block = 2471432538116610086;
                                break;
                            }
                        } else {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetStr(
                                
                                pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                (*pOp).p4.z,
                                (*pOp).p1 as crate::src::ext::rtree::rtree::i64_0,
                                0 as crate::src::ext::rtree::rtree::u8_0,
                                None,
                            );
                        }
                        (*pOut).enc = encoding;
                        updateMaxBlobsize(pOut);
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Variable_1 =>  {
                        let mut pVar: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        pVar = (*p)
                            .aVar
                            .offset(((*pOp).p1 - 1 as ::core::ffi::c_int) as isize)
                            as *mut crate::src::src::vdbe::Mem;
                        if crate::src::src::vdbemem::sqlite3VdbeMemTooBig(pVar as *mut crate::src::headers::vdbeInt_h::sqlite3_value) != 0 {
                            current_block = 7113757447962609068;
                            break;
                        }
                        pOut = aMem.offset((*pOp).p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pOut).flags as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Agg | crate::src::headers::vdbeInt_h::MEM_Dyn)
                            != 0 as ::core::ffi::c_int
                        {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        }
                        ::core::ptr::copy_nonoverlapping(
                    pVar as *const u8,
                    pOut as *mut u8,
                    crate::src::headers::vdbeInt_h::MEMCELLSIZE as usize,
                );
                        (*pOut).flags =
                            ((*pOut).flags as ::core::ffi::c_int & !(crate::src::headers::vdbeInt_h::MEM_Dyn | crate::src::headers::vdbeInt_h::MEM_Ephem)) as crate::src::fts5::u16_0;
                        (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                            | (crate::src::headers::vdbeInt_h::MEM_Static | crate::src::headers::vdbeInt_h::MEM_FromBind))
                            as crate::src::fts5::u16_0;
                        updateMaxBlobsize(pOut);
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Move_1 =>  {
                        let mut n: ::core::ffi::c_int = 0;
                        let mut p1: ::core::ffi::c_int = 0;
                        let mut p2: ::core::ffi::c_int = 0;
                        let __pOp_ref = unsafe { &*pOp };
                        n = __pOp_ref.p3;
                        p1 = __pOp_ref.p1;
                        p2 = __pOp_ref.p2;
                        pIn1 = aMem.offset(p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pOut = aMem.offset(p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        loop {
                            crate::src::src::vdbemem::sqlite3VdbeMemMove(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value,  pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                            if (*pOut).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Ephem
                                != 0 as ::core::ffi::c_int
                                && crate::src::src::vdbemem::sqlite3VdbeMemMakeWriteable(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value) != 0
                            {
                                current_block = 2471432538116610086;
                                break 's_109;
                            }
                            pIn1 = pIn1.offset(1);
                            pOut = pOut.offset(1);
                            n -= 1;
                            if !(n != 0) {
                                break;
                            }
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Copy_1 =>  {
                        let mut n_0: ::core::ffi::c_int = 0;
                        let __pOp_ref = unsafe { &*pOp };
                        n_0 = __pOp_ref.p3;
                        pIn1 = aMem.offset(__pOp_ref.p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pOut = aMem.offset(__pOp_ref.p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        loop {
                            crate::src::src::vdbemem::sqlite3VdbeMemShallowCopy(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value,  pIn1 as *const crate::src::headers::vdbeInt_h::sqlite3_value, crate::src::headers::vdbeInt_h::MEM_Ephem);
                            if (*pOut).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Ephem
                                != 0 as ::core::ffi::c_int
                                && crate::src::src::vdbemem::sqlite3VdbeMemMakeWriteable(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value) != 0
                            {
                                current_block = 2471432538116610086;
                                break 's_109;
                            }
                            if (*pOut).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Subtype
                                != 0 as ::core::ffi::c_int
                                && __pOp_ref.p5 as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int
                                    != 0 as ::core::ffi::c_int
                            {
                                (*pOut).flags =
                                    ((*pOut).flags as ::core::ffi::c_int & !crate::src::headers::vdbeInt_h::MEM_Subtype) as crate::src::fts5::u16_0;
                            }
                            let fresh0 = n_0;
                            n_0 -= 1;
                            if fresh0 == 0 as ::core::ffi::c_int {
                                break;
                            }
                            pOut = pOut.offset(1);
                            pIn1 = pIn1.offset(1);
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_SCopy_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pOut = aMem.offset((*pOp).p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        crate::src::src::vdbemem::sqlite3VdbeMemShallowCopy(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value,  pIn1 as *const crate::src::headers::vdbeInt_h::sqlite3_value, crate::src::headers::vdbeInt_h::MEM_Ephem);
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_IntCopy_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pOut = aMem.offset((*pOp).p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        crate::src::src::vdbemem::sqlite3VdbeMemSetInt64(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value, (*pIn1).u.i);
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_FkCheck_1 =>  {
                        rc = crate::src::src::vdbeaux::sqlite3VdbeCheckFkImmediate(p as *mut crate::src::headers::vdbeInt_h::Vdbe);
                        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                            current_block = 9863799879598557851;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_ResultRow_1 =>  {
                        __p_ref.cacheCtr = __p_ref.cacheCtr.wrapping_add(2 as crate::src::ext::rtree::rtree::u32_0) | 1 as crate::src::ext::rtree::rtree::u32_0;
                        __p_ref.pResultRow = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*db).mallocFailed != 0 {
                            current_block = 2471432538116610086;
                            break;
                        }
                        if (*db).mTrace as ::core::ffi::c_int & crate::src::headers::sqlite3_h::SQLITE_TRACE_ROW != 0 {
                            (*db).trace.xV2.expect("non-null function pointer")(
                                crate::src::headers::sqlite3_h::SQLITE_TRACE_ROW as crate::src::ext::rtree::rtree::u32_0,
                                (*db).pTraceArg,
                                p as *mut ::core::ffi::c_void,
                                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                            );
                        }
                        __p_ref.pc = pOp.offset_from(aOp) as ::core::ffi::c_long as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int;
                        rc = crate::src::headers::sqlite3_h::SQLITE_ROW;
                        current_block = 10408586827809538586;
                        break;
                    }
    crate::src::headers::opcodes_h::OP_Concat_1 =>  {
                        let mut nByte: crate::src::ext::rtree::rtree::i64_0 = 0;
                        let mut flags1: crate::src::fts5::u16_0 = 0;
                        let mut flags2: crate::src::fts5::u16_0 = 0;
                        let __pOp_ref = unsafe { &*pOp };
                        pIn1 = aMem.offset(__pOp_ref.p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pIn2 = aMem.offset(__pOp_ref.p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        pOut = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        flags1 = (*pIn1).flags;
                        if (flags1 as ::core::ffi::c_int | (*pIn2).flags as ::core::ffi::c_int)
                            & crate::src::headers::vdbeInt_h::MEM_Null
                            != 0
                        {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        } else {
                            let __pIn1_ref = unsafe { &mut *pIn1 };
                            if flags1 as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Str | crate::src::headers::vdbeInt_h::MEM_Blob)
                                == 0 as ::core::ffi::c_int
                            {
                                if crate::src::src::vdbemem::sqlite3VdbeMemStringify(pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value, encoding, 0 as crate::src::ext::rtree::rtree::u8_0) != 0 {
                                    current_block = 2471432538116610086;
                                    break;
                                }
                                flags1 = (__pIn1_ref.flags as ::core::ffi::c_int & !crate::src::headers::vdbeInt_h::MEM_Str) as crate::src::fts5::u16_0;
                            } else if flags1 as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Zero
                                != 0 as ::core::ffi::c_int
                            {
                                if crate::src::src::vdbemem::sqlite3VdbeMemExpandBlob(pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value) != 0 {
                                    current_block = 2471432538116610086;
                                    break;
                                }
                                flags1 = (__pIn1_ref.flags as ::core::ffi::c_int & !crate::src::headers::vdbeInt_h::MEM_Str) as crate::src::fts5::u16_0;
                            }
                            let __pIn2_ref = unsafe { &mut *pIn2 };
                            flags2 = __pIn2_ref.flags;
                            if flags2 as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Str | crate::src::headers::vdbeInt_h::MEM_Blob)
                                == 0 as ::core::ffi::c_int
                            {
                                if crate::src::src::vdbemem::sqlite3VdbeMemStringify(pIn2 as *mut crate::src::headers::vdbeInt_h::sqlite3_value, encoding, 0 as crate::src::ext::rtree::rtree::u8_0) != 0 {
                                    current_block = 2471432538116610086;
                                    break;
                                }
                                flags2 = (__pIn2_ref.flags as ::core::ffi::c_int & !crate::src::headers::vdbeInt_h::MEM_Str) as crate::src::fts5::u16_0;
                            } else if flags2 as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Zero
                                != 0 as ::core::ffi::c_int
                            {
                                if crate::src::src::vdbemem::sqlite3VdbeMemExpandBlob(pIn2 as *mut crate::src::headers::vdbeInt_h::sqlite3_value) != 0 {
                                    current_block = 2471432538116610086;
                                    break;
                                }
                                flags2 = (__pIn2_ref.flags as ::core::ffi::c_int & !crate::src::headers::vdbeInt_h::MEM_Str) as crate::src::fts5::u16_0;
                            }
                            nByte = __pIn1_ref.n as crate::src::ext::rtree::rtree::i64_0;
                            nByte += __pIn2_ref.n as crate::src::ext::rtree::rtree::i64_0;
                            if nByte > (*db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_LENGTH as usize] as crate::src::ext::rtree::rtree::i64_0 {
                                current_block = 7113757447962609068;
                                break;
                            }
                            if crate::src::src::vdbemem::sqlite3VdbeMemGrow(
                                
                                pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                nByte as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
                                (pOut == pIn2) as ::core::ffi::c_int,
                            ) != 0
                            {
                                current_block = 2471432538116610086;
                                break;
                            }
                            let __pOut_ref = unsafe { &mut *pOut };
                            __pOut_ref.flags = (__pOut_ref.flags as ::core::ffi::c_int
                                & !(crate::src::headers::vdbeInt_h::MEM_TypeMask | crate::src::headers::vdbeInt_h::MEM_Zero)
                                | 0x2 as ::core::ffi::c_int)
                                as crate::src::fts5::u16_0;
                            if pOut != pIn2 {
                                ::core::ptr::copy_nonoverlapping(
                    __pIn2_ref.z as *const u8,
                    __pOut_ref.z as *mut u8,
                    __pIn2_ref.n as usize,
                );
                                __pIn2_ref.flags = flags2;
                            }
                            ::core::ptr::copy_nonoverlapping(
                    __pIn1_ref.z as *const u8,
                    __pOut_ref.z.offset(__pIn2_ref.n as isize) as *mut ::core::ffi::c_char as *mut u8,
                    __pIn1_ref.n as usize,
                );
                            __pIn1_ref.flags = flags1;
                            if encoding as ::core::ffi::c_int > crate::src::headers::sqlite3_h::SQLITE_UTF8 {
                                nByte &= !(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0;
                            }
                            *__pOut_ref.z.offset(nByte as isize) = 0 as ::core::ffi::c_char;
                            *__pOut_ref.z.offset((nByte + 1 as crate::src::ext::rtree::rtree::i64_0) as isize) =
                                0 as ::core::ffi::c_char;
                            __pOut_ref.flags =
                                (__pOut_ref.flags as ::core::ffi::c_int | crate::src::headers::vdbeInt_h::MEM_Term) as crate::src::fts5::u16_0;
                            __pOut_ref.n = nByte as ::core::ffi::c_int;
                            __pOut_ref.enc = encoding;
                            updateMaxBlobsize(pOut);
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Add =>  {
                        current_block = 16448975459825856284;
                    }
    crate::src::headers::opcodes_h::OP_Subtract_1 =>  {
                        current_block = 16448975459825856284;
                    }
    crate::src::headers::opcodes_h::OP_Multiply =>  {
                        current_block = 5899965325122513769;
                    }
    crate::src::headers::opcodes_h::OP_Divide =>  {
                        current_block = 16916022866774761203;
                    }
    crate::src::headers::opcodes_h::OP_Remainder =>  {
                        current_block = 11475295656646479480;
                    }
    crate::src::headers::opcodes_h::OP_CollSeq_1 =>  {
                        if (*pOp).p1 != 0 {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetInt64(
                                
                                aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                0 as crate::src::ext::rtree::rtree::i64_0,
                            );
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_BitAnd =>  {
                        current_block = 18047165599808473703;
                    }
    crate::src::headers::opcodes_h::OP_BitOr =>  {
                        current_block = 18047165599808473703;
                    }
    crate::src::headers::opcodes_h::OP_ShiftLeft =>  {
                        current_block = 8472080195360146252;
                    }
    crate::src::headers::opcodes_h::OP_ShiftRight =>  {
                        current_block = 1895500680468589183;
                    }
    crate::src::headers::opcodes_h::OP_AddImm_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        crate::src::src::vdbemem::sqlite3VdbeMemIntegerify(pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        let ref mut fresh1 = *(&raw mut (*pIn1).u.i as *mut crate::src::ext::rtree::rtree::u64_0);
                        *fresh1 = (*fresh1).wrapping_add((*pOp).p2 as crate::src::ext::rtree::rtree::u64_0);
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_MustBeInt_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pIn1).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Int == 0 as ::core::ffi::c_int
                        {
                            applyAffinity(
                                pIn1,
                                crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC as ::core::ffi::c_char,
                                encoding,
                            );
                            if (*pIn1).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Int
                                == 0 as ::core::ffi::c_int
                            {
                                if (*pOp).p2 == 0 as ::core::ffi::c_int {
                                    rc = crate::src::headers::sqlite3_h::SQLITE_MISMATCH;
                                    current_block = 9863799879598557851;
                                    break;
                                } else {
                                    current_block = 9512719473022792396;
                                }
                            } else {
                                current_block = 7073745033006193998;
                            }
                        } else {
                            current_block = 7073745033006193998;
                        }
                        match current_block {
                            9512719473022792396 => {}
                            _ => {
                                (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                    & !(crate::src::headers::vdbeInt_h::MEM_TypeMask | crate::src::headers::vdbeInt_h::MEM_Zero)
                                    | 0x4 as ::core::ffi::c_int)
                                    as crate::src::fts5::u16_0;
                                current_block = 5783071609795492627;
                            }
                        }
                    }
    crate::src::headers::opcodes_h::OP_RealAffinity_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pIn1).flags as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_IntReal) != 0 {
                            crate::src::src::vdbemem::sqlite3VdbeMemRealify(pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Cast_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        rc = if (*pIn1).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Zero != 0 {
                            crate::src::src::vdbemem::sqlite3VdbeMemExpandBlob(pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value)
                        } else {
                            0 as ::core::ffi::c_int
                        };
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        rc = crate::src::src::vdbemem::sqlite3VdbeMemCast(pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value, (*pOp).p2 as crate::src::ext::rtree::rtree::u8_0, encoding);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        updateMaxBlobsize(pIn1);
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Eq_1 =>  {
                        current_block = 8915789739336007503;
                    }
    crate::src::headers::opcodes_h::OP_Ne_1 =>  {
                        current_block = 8915789739336007503;
                    }
    crate::src::headers::opcodes_h::OP_Lt =>  {
                        current_block = 9183375779473790601;
                    }
    crate::src::headers::opcodes_h::OP_Le_1 =>  {
                        current_block = 3502568586026288603;
                    }
    crate::src::headers::opcodes_h::OP_Gt =>  {
                        current_block = 7712297393629777289;
                    }
    crate::src::headers::opcodes_h::OP_Ge =>  {
                        current_block = 9598485154371622742;
                    }
    crate::src::headers::opcodes_h::OP_ElseEq_1 =>  {
                        if iCompare == 0 as ::core::ffi::c_int {
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_Compare_1 =>  {
                        let mut n_1: ::core::ffi::c_int = 0;
                        let mut i: ::core::ffi::c_int = 0;
                        let mut p1_0: ::core::ffi::c_int = 0;
                        let mut p2_0: ::core::ffi::c_int = 0;
                        let mut pKeyInfo: *const crate::src::headers::sqliteInt_h::KeyInfo = ::core::ptr::null::<crate::src::headers::sqliteInt_h::KeyInfo>();
                        let mut idx: crate::src::ext::rtree::rtree::u32_0 = 0;
                        let mut pColl: *mut crate::src::headers::sqliteInt_h::CollSeq = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::CollSeq>();
                        let mut bRev: ::core::ffi::c_int = 0;
                        let mut aPermute: *mut crate::src::ext::rtree::rtree::u32_0 = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u32_0>();
                        let __pOp_ref = unsafe { &*pOp };
                        if __pOp_ref.p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::OPFLAG_PERMUTE
                            == 0 as ::core::ffi::c_int
                        {
                            aPermute = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u32_0>();
                        } else {
                            aPermute = (*pOp.offset(-(1 as ::core::ffi::c_int) as isize))
                                .p4
                                .ai
                                .offset(1 as isize);
                        }
                        n_1 = __pOp_ref.p3;
                        pKeyInfo = __pOp_ref.p4.pKeyInfo;
                        p1_0 = __pOp_ref.p1;
                        p2_0 = __pOp_ref.p2;
                        i = 0 as ::core::ffi::c_int;
                        while i < n_1 {
                            idx = if !aPermute.is_null() {
                                *aPermute.offset(i as isize)
                            } else {
                                i as crate::src::ext::rtree::rtree::u32_0
                            };
                            pColl = *(&raw const (*pKeyInfo).aColl as *const *mut crate::src::headers::sqliteInt_h::CollSeq)
                                .offset(i as isize);
                            bRev = *(*pKeyInfo).aSortFlags.offset(i as isize) as ::core::ffi::c_int
                                & crate::src::headers::sqliteInt_h::KEYINFO_ORDER_DESC;
                            iCompare = crate::src::src::vdbeaux::sqlite3MemCompare(
                                
                                aMem.offset((p1_0 as crate::src::ext::rtree::rtree::u32_0).wrapping_add(idx) as isize) as *mut crate::src::src::vdbe::Mem as *const crate::src::headers::vdbeInt_h::sqlite3_value,
                                
                                aMem.offset((p2_0 as crate::src::ext::rtree::rtree::u32_0).wrapping_add(idx) as isize) as *mut crate::src::src::vdbe::Mem as *const crate::src::headers::vdbeInt_h::sqlite3_value,
                                
                                pColl as *const crate::src::headers::sqliteInt_h::CollSeq,
                            );
                            if iCompare != 0 {
                                if *(*pKeyInfo).aSortFlags.offset(i as isize) as ::core::ffi::c_int
                                    & crate::src::headers::sqliteInt_h::KEYINFO_ORDER_BIGNULL
                                    != 0
                                    && ((*aMem.offset((p1_0 as crate::src::ext::rtree::rtree::u32_0).wrapping_add(idx) as isize))
                                        .flags
                                        as ::core::ffi::c_int
                                        & crate::src::headers::vdbeInt_h::MEM_Null
                                        != 0
                                        || (*aMem
                                            .offset((p2_0 as crate::src::ext::rtree::rtree::u32_0).wrapping_add(idx) as isize))
                                        .flags
                                            as ::core::ffi::c_int
                                            & crate::src::headers::vdbeInt_h::MEM_Null
                                            != 0)
                                {
                                    iCompare = -iCompare;
                                }
                                if bRev != 0 {
                                    iCompare = -iCompare;
                                }
                                break;
                            } else {
                                i += 1;
                            }
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Jump_1 =>  {
                        if iCompare < 0 as ::core::ffi::c_int {
                            pOp = aOp.offset(((*pOp).p1 - 1 as ::core::ffi::c_int) as isize)
                                as *mut crate::src::headers::vdbeInt_h::Op;
                        } else if iCompare == 0 as ::core::ffi::c_int {
                            pOp = aOp.offset(((*pOp).p2 - 1 as ::core::ffi::c_int) as isize)
                                as *mut crate::src::headers::vdbeInt_h::Op;
                        } else {
                            pOp = aOp.offset(((*pOp).p3 - 1 as ::core::ffi::c_int) as isize)
                                as *mut crate::src::headers::vdbeInt_h::Op;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_And =>  {
                        current_block = 10856689599696957676;
                    }
    crate::src::headers::opcodes_h::OP_Or_1 =>  {
                        current_block = 10856689599696957676;
                    }
    crate::src::headers::opcodes_h::OP_IsTrue_1 =>  {
                        let __pOp_ref = unsafe { &*pOp };
                        crate::src::src::vdbemem::sqlite3VdbeMemSetInt64(
                            
                            aMem.offset(__pOp_ref.p2 as isize) as *mut crate::src::src::vdbe::Mem as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            (crate::src::src::vdbemem::sqlite3VdbeBooleanValue(
                                
                                aMem.offset(__pOp_ref.p1 as isize) as *mut crate::src::src::vdbe::Mem as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                __pOp_ref.p3,
                            ) ^ __pOp_ref.p4.i) as crate::src::ext::rtree::rtree::i64_0,
                        );
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Not_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pOut = aMem.offset((*pOp).p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pIn1).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Null == 0 as ::core::ffi::c_int
                        {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetInt64(
                                
                                pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                (crate::src::src::vdbemem::sqlite3VdbeBooleanValue(pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value, 0 as ::core::ffi::c_int) == 0)
                                    as ::core::ffi::c_int as crate::src::ext::rtree::rtree::i64_0,
                            );
                        } else {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_BitNot =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pOut = aMem.offset((*pOp).p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        if (*pIn1).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Null == 0 as ::core::ffi::c_int
                        {
                            (*pOut).flags = crate::src::headers::vdbeInt_h::MEM_Int as crate::src::fts5::u16_0;
                            (*pOut).u.i = !crate::src::src::vdbemem::sqlite3VdbeIntValue(pIn1 as *const crate::src::headers::vdbeInt_h::sqlite3_value);
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Once =>  {
                        let mut iAddr: crate::src::ext::rtree::rtree::u32_0 = 0;
                        if !__p_ref.pFrame.is_null() {
                            iAddr = pOp.offset_from(__p_ref.aOp) as ::core::ffi::c_long
                                as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u32_0;
                            if *(*__p_ref.pFrame)
                                .aOnce
                                .offset(iAddr.wrapping_div(8 as crate::src::ext::rtree::rtree::u32_0) as isize)
                                as ::core::ffi::c_int
                                & (1 as ::core::ffi::c_int) << (iAddr & 7 as crate::src::ext::rtree::rtree::u32_0)
                                != 0 as ::core::ffi::c_int
                            {
                                current_block = 9512719473022792396;
                            } else {
                                let ref mut fresh2 = *(*__p_ref.pFrame)
                                    .aOnce
                                    .offset(iAddr.wrapping_div(8 as crate::src::ext::rtree::rtree::u32_0) as isize);
                                *fresh2 = (*fresh2 as ::core::ffi::c_int
                                    | (1 as ::core::ffi::c_int) << (iAddr & 7 as crate::src::ext::rtree::rtree::u32_0))
                                    as crate::src::ext::rtree::rtree::u8_0;
                                current_block = 11642008854238505342;
                            }
                        } else if (*__p_ref.aOp.offset(0 as isize)).p1
                            == (*pOp).p1
                        {
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 11642008854238505342;
                        }
                        match current_block {
                            9512719473022792396 => {}
                            _ => {
                                (*pOp).p1 = (*__p_ref.aOp.offset(0 as isize)).p1;
                                current_block = 5783071609795492627;
                            }
                        }
                    }
    crate::src::headers::opcodes_h::OP_If_1 =>  {
                        let mut c: ::core::ffi::c_int = 0;
                        c = crate::src::src::vdbemem::sqlite3VdbeBooleanValue(
                            
                            aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            (*pOp).p3,
                        );
                        if c != 0 {
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_IfNot_1 =>  {
                        let mut c_0: ::core::ffi::c_int = 0;
                        c_0 = (crate::src::src::vdbemem::sqlite3VdbeBooleanValue(
                            
                            aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            ((*pOp).p3 == 0) as ::core::ffi::c_int,
                        ) == 0) as ::core::ffi::c_int;
                        if c_0 != 0 {
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_IsNull_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pIn1).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Null != 0 as ::core::ffi::c_int
                        {
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_IsType_1 =>  {
                        let mut pC: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut typeMask: crate::src::fts5::u16_0 = 0;
                        let mut serialType: crate::src::ext::rtree::rtree::u32_0 = 0;
                        if (*pOp).p1 >= 0 as ::core::ffi::c_int {
                            pC = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                            if (*pOp).p3 < (*pC).nHdrParsed as ::core::ffi::c_int {
                                serialType = *(&raw mut (*pC).aType as *mut crate::src::ext::rtree::rtree::u32_0)
                                    .offset((*pOp).p3 as isize);
                                if serialType >= 12 as crate::src::ext::rtree::rtree::u32_0 {
                                    if serialType & 1 as crate::src::ext::rtree::rtree::u32_0 != 0 {
                                        typeMask = 0x4 as crate::src::fts5::u16_0;
                                    } else {
                                        typeMask = 0x8 as crate::src::fts5::u16_0;
                                    }
                                } else {
                                    static mut aMask: [::core::ffi::c_uchar; 12] = [
                                        0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                        0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                        0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                        0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                    ];
                                    typeMask = aMask[serialType as usize] as crate::src::fts5::u16_0;
                                }
                            } else {
                                typeMask = ((1 as ::core::ffi::c_int)
                                    << (*pOp).p4.i - 1 as ::core::ffi::c_int)
                                    as crate::src::fts5::u16_0;
                            }
                        } else {
                            typeMask = ((1 as ::core::ffi::c_int)
                                << crate::src::src::vdbeapi::sqlite3_value_type(aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem as *mut  crate::src::headers::vdbeInt_h::sqlite3_value
                                    as *mut crate::src::headers::vdbeInt_h::sqlite3_value)
                                    - 1 as ::core::ffi::c_int)
                                as crate::src::fts5::u16_0;
                        }
                        if typeMask as ::core::ffi::c_int & (*pOp).p5 as ::core::ffi::c_int != 0 {
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_ZeroOrNull_1 =>  {
                        if (*aMem.offset((*pOp).p1 as isize)).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Null
                            != 0 as ::core::ffi::c_int
                            || (*aMem.offset((*pOp).p3 as isize)).flags as ::core::ffi::c_int
                                & crate::src::headers::vdbeInt_h::MEM_Null
                                != 0 as ::core::ffi::c_int
                        {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetNull(aMem.offset((*pOp).p2 as isize) as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        } else {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetInt64(aMem.offset((*pOp).p2 as isize) as *mut crate::src::headers::vdbeInt_h::sqlite3_value, 0 as crate::src::ext::rtree::rtree::i64_0);
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_NotNull_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pIn1).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Null == 0 as ::core::ffi::c_int
                        {
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_IfNullRow_1 =>  {
                        let mut pC_0: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        pC_0 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        if !pC_0.is_null() && (*pC_0).nullRow as ::core::ffi::c_int != 0 {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetNull(aMem.offset((*pOp).p3 as isize) as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_Column_1 =>  {
                        let mut p2_1: crate::src::ext::rtree::rtree::u32_0 = 0;
                        let mut pC_1: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut pCrsr: *mut crate::src::headers::btreeInt_h::BtCursor = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>();
                        let mut aOffset: *mut crate::src::ext::rtree::rtree::u32_0 = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u32_0>();
                        let mut len: ::core::ffi::c_int = 0;
                        let mut i_0: ::core::ffi::c_int = 0;
                        let mut pDest: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let mut sMem: crate::src::src::vdbe::Mem = unsafe { ::core::mem::zeroed() };
                        let mut zData: *const crate::src::ext::rtree::rtree::u8_0 = ::core::ptr::null::<crate::src::ext::rtree::rtree::u8_0>();
                        let mut zHdr: *const crate::src::ext::rtree::rtree::u8_0 = ::core::ptr::null::<crate::src::ext::rtree::rtree::u8_0>();
                        let mut zEndHdr: *const crate::src::ext::rtree::rtree::u8_0 = ::core::ptr::null::<crate::src::ext::rtree::rtree::u8_0>();
                        let mut offset64: crate::src::ext::rtree::rtree::u64_0 = 0;
                        let mut t: crate::src::ext::rtree::rtree::u32_0 = 0;
                        let mut pReg: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        pC_1 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        p2_1 = (*pOp).p2 as crate::src::ext::rtree::rtree::u32_0;
                        loop {
                            aOffset = (*pC_1).aOffset;
                            if (*pC_1).cacheStatus != __p_ref.cacheCtr {
                                if (*pC_1).nullRow != 0 {
                                    if (*pC_1).eCurType as ::core::ffi::c_int == crate::src::headers::vdbeInt_h::CURTYPE_PSEUDO
                                        && (*pC_1).seekResult > 0 as ::core::ffi::c_int
                                    {
                                        current_block = 9495722216958262053;
                                        break;
                                    } else {
                                        current_block = 1975004770106644911;
                                        break;
                                    }
                                } else {
                                    pCrsr = (*pC_1).uc.pCursor;
                                    if (*pC_1).deferredMoveto != 0 {
                                        let mut iMap: crate::src::ext::rtree::rtree::u32_0 = 0;
                                        if !(*pC_1).ub.aAltMap.is_null() && {
                                            iMap = *(*pC_1)
                                                .ub
                                                .aAltMap
                                                .offset((1 as crate::src::ext::rtree::rtree::u32_0).wrapping_add(p2_1) as isize);
                                            iMap > 0 as crate::src::ext::rtree::rtree::u32_0
                                        } {
                                            pC_1 = (*pC_1).pAltCursor;
                                            p2_1 = iMap.wrapping_sub(1 as crate::src::ext::rtree::rtree::u32_0);
                                        } else {
                                            rc = crate::src::src::vdbeaux::sqlite3VdbeFinishMoveto(pC_1 as *mut crate::src::headers::vdbeInt_h::VdbeCursor);
                                            if rc != 0 {
                                                current_block = 9863799879598557851;
                                                break 's_109;
                                            } else {
                                                current_block = 13246752163818097729;
                                                break;
                                            }
                                        }
                                    } else {
                                        if !(crate::src::src::btree::sqlite3BtreeCursorHasMoved(pCrsr) != 0) {
                                            current_block = 13246752163818097729;
                                            break;
                                        }
                                        rc = crate::src::src::vdbeaux::sqlite3VdbeHandleMovedCursor(pC_1 as *mut crate::src::headers::vdbeInt_h::VdbeCursor);
                                        if rc != 0 {
                                            current_block = 9863799879598557851;
                                            break 's_109;
                                        }
                                    }
                                }
                            } else {
                                if !(crate::src::src::btree::sqlite3BtreeCursorHasMoved((*pC_1).uc.pCursor) != 0) {
                                    current_block = 9147150541555199843;
                                    break;
                                }
                                rc = crate::src::src::vdbeaux::sqlite3VdbeHandleMovedCursor(pC_1 as *mut crate::src::headers::vdbeInt_h::VdbeCursor);
                                if rc != 0 {
                                    current_block = 9863799879598557851;
                                    break 's_109;
                                }
                            }
                        }
                        match current_block {
                            13246752163818097729 => {
                                let __pC_1_ref = unsafe { &mut *pC_1 };
                                __pC_1_ref.payloadSize = crate::src::src::btree::sqlite3BtreePayloadSize(pCrsr);
                                __pC_1_ref.aRow =
                                    crate::src::src::btree::sqlite3BtreePayloadFetch(pCrsr, &raw mut __pC_1_ref.szRow)
                                        as *const crate::src::ext::rtree::rtree::u8_0;
                                current_block = 4580464979391500237;
                            }
                            1975004770106644911 => {
                                pDest = aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem;
                                crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pDest as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                                current_block = 14389784149831352918;
                            }
                            9495722216958262053 => {
                                let __pC_1_ref = unsafe { &mut *pC_1 };
                                pReg = aMem.offset(__pC_1_ref.seekResult as isize) as *mut crate::src::src::vdbe::Mem;
                                __pC_1_ref.szRow = (*pReg).n as crate::src::ext::rtree::rtree::u32_0;
                                __pC_1_ref.payloadSize = __pC_1_ref.szRow;
                                __pC_1_ref.aRow = (*pReg).z as *mut crate::src::ext::rtree::rtree::u8_0;
                                current_block = 4580464979391500237;
                            }
                            _ => {}
                        }
                        match current_block {
                            4580464979391500237 => {
                                let __pC_1_ref = unsafe { &mut *pC_1 };
                                __pC_1_ref.cacheStatus = __p_ref.cacheCtr;
                                let ref mut fresh3 =
                                    *aOffset.offset(0 as isize);
                                *fresh3 =
                                    *__pC_1_ref.aRow.offset(0 as isize) as crate::src::ext::rtree::rtree::u32_0;
                                if *fresh3 < 0x80 as crate::src::ext::rtree::rtree::u32_0 {
                                    __pC_1_ref.iHdrOffset = 1 as crate::src::ext::rtree::rtree::u32_0;
                                } else {
                                    __pC_1_ref.iHdrOffset = crate::src::src::util::sqlite3GetVarint32(
                                        __pC_1_ref.aRow as *const ::core::ffi::c_uchar,
                                        aOffset,
                                    )
                                        as crate::src::ext::rtree::rtree::u32_0;
                                }
                                __pC_1_ref.nHdrParsed = 0 as crate::src::fts5::u16_0;
                                if __pC_1_ref.szRow < *aOffset.offset(0 as isize)
                                {
                                    __pC_1_ref.aRow = ::core::ptr::null::<crate::src::ext::rtree::rtree::u8_0>();
                                    __pC_1_ref.szRow = 0 as crate::src::ext::rtree::rtree::u32_0;
                                    if *aOffset.offset(0 as isize)
                                        > 98307 as crate::src::ext::rtree::rtree::u32_0
                                        || *aOffset.offset(0 as isize)
                                            > __pC_1_ref.payloadSize
                                    {
                                        current_block = 1447722815581200623;
                                    } else {
                                        current_block = 9147150541555199843;
                                    }
                                } else {
                                    zData = __pC_1_ref.aRow;
                                    current_block = 13485359645741033759;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            9147150541555199843 => {
                                if (*pC_1).nHdrParsed as crate::src::ext::rtree::rtree::u32_0 <= p2_1 {
                                    if (*pC_1).iHdrOffset
                                        < *aOffset.offset(0 as isize)
                                    {
                                        if (*pC_1).aRow.is_null() {
                                            rc = crate::src::src::vdbemem::sqlite3VdbeMemFromBtreeZeroOffset(
                                                (*pC_1).uc.pCursor,
                                                *aOffset.offset(0 as isize),
                                                
                                                &raw mut sMem as *mut _ as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                            );
                                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                                current_block = 9863799879598557851;
                                                break;
                                            }
                                            zData = sMem.z as *mut crate::src::ext::rtree::rtree::u8_0;
                                        } else {
                                            zData = (*pC_1).aRow;
                                        }
                                        current_block = 13485359645741033759;
                                    } else {
                                        t = 0 as crate::src::ext::rtree::rtree::u32_0;
                                        current_block = 13908727566713266484;
                                    }
                                } else {
                                    t = *(&raw mut (*pC_1).aType as *mut crate::src::ext::rtree::rtree::u32_0)
                                        .offset(p2_1 as isize);
                                    current_block = 2644446111302234773;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            13485359645741033759 => {
                                let __pC_1_ref = unsafe { &mut *pC_1 };
                                i_0 = __pC_1_ref.nHdrParsed as ::core::ffi::c_int;
                                offset64 = *aOffset.offset(i_0 as isize) as crate::src::ext::rtree::rtree::u64_0;
                                zHdr = zData.offset(__pC_1_ref.iHdrOffset as isize);
                                zEndHdr =
                                    zData
                                        .offset(*aOffset.offset(0 as isize)
                                            as isize);
                                loop {
                                    t = *zHdr.offset(0 as isize) as crate::src::ext::rtree::rtree::u32_0;
                                    let ref mut fresh4 = *(&raw mut __pC_1_ref.aType as *mut crate::src::ext::rtree::rtree::u32_0)
                                        .offset(i_0 as isize);
                                    *fresh4 = t;
                                    if *fresh4 < 0x80 as crate::src::ext::rtree::rtree::u32_0 {
                                        zHdr = zHdr.offset(1);
                                        offset64 = offset64.wrapping_add(
                                            crate::src::src::vdbeaux::sqlite3VdbeOneByteSerialTypeLen(t as crate::src::ext::rtree::rtree::u8_0) as crate::src::ext::rtree::rtree::u64_0,
                                        );
                                    } else {
                                        zHdr = zHdr.offset(crate::src::src::util::sqlite3GetVarint32(
                                            zHdr as *const ::core::ffi::c_uchar,
                                            &raw mut t,
                                        )
                                            as ::core::ffi::c_int
                                            as isize);
                                        *(&raw mut __pC_1_ref.aType as *mut crate::src::ext::rtree::rtree::u32_0)
                                            .offset(i_0 as isize) = t;
                                        offset64 = offset64
                                            .wrapping_add(crate::src::src::vdbeaux::sqlite3VdbeSerialTypeLen(t) as crate::src::ext::rtree::rtree::u64_0);
                                    }
                                    i_0 += 1;
                                    *aOffset.offset(i_0 as isize) =
                                        (offset64 & 0xffffffff as crate::src::ext::rtree::rtree::u64_0) as crate::src::ext::rtree::rtree::u32_0;
                                    if !(i_0 as crate::src::ext::rtree::rtree::u32_0 <= p2_1 && zHdr < zEndHdr) {
                                        break;
                                    }
                                }
                                if zHdr >= zEndHdr
                                    && (zHdr > zEndHdr || offset64 != __pC_1_ref.payloadSize as crate::src::ext::rtree::rtree::u64_0)
                                    || offset64 > __pC_1_ref.payloadSize as crate::src::ext::rtree::rtree::u64_0
                                {
                                    if *aOffset.offset(0 as isize)
                                        == 0 as crate::src::ext::rtree::rtree::u32_0
                                    {
                                        i_0 = 0 as ::core::ffi::c_int;
                                        zHdr = zEndHdr;
                                        current_block = 14847419611938267583;
                                    } else {
                                        if __pC_1_ref.aRow.is_null() {
                                            crate::src::src::vdbemem::sqlite3VdbeMemRelease(&raw mut sMem as *mut _ as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                                        }
                                        current_block = 1447722815581200623;
                                    }
                                } else {
                                    current_block = 14847419611938267583;
                                }
                                match current_block {
                                    1447722815581200623 => {}
                                    _ => {
                                        __pC_1_ref.nHdrParsed = i_0 as crate::src::fts5::u16_0;
                                        __pC_1_ref.iHdrOffset =
                                            zHdr.offset_from(zData) as ::core::ffi::c_long as crate::src::ext::rtree::rtree::u32_0;
                                        if __pC_1_ref.aRow.is_null() {
                                            crate::src::src::vdbemem::sqlite3VdbeMemRelease(&raw mut sMem as *mut _ as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                                        }
                                        current_block = 13908727566713266484;
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            13908727566713266484 => {
                                if (*pC_1).nHdrParsed as crate::src::ext::rtree::rtree::u32_0 <= p2_1 {
                                    pDest = aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem;
                                    if (*pOp).p4type as ::core::ffi::c_int == crate::src::src::vdbe::P4_MEM {
                                        crate::src::src::vdbemem::sqlite3VdbeMemShallowCopy(
                                            
                                            pDest as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                            
                                            (*pOp).p4.pMem as *const crate::src::headers::vdbeInt_h::sqlite3_value,
                                            crate::src::headers::vdbeInt_h::MEM_Static,
                                        );
                                    } else {
                                        crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pDest as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                                    }
                                    current_block = 14389784149831352918;
                                } else {
                                    current_block = 2644446111302234773;
                                }
                            }
                            1447722815581200623 => {
                                if (*aOp.offset(0 as isize)).p3
                                    > 0 as ::core::ffi::c_int
                                {
                                    pOp = aOp.offset(
                                        ((*aOp.offset(0 as isize)).p3
                                            - 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *mut crate::src::headers::vdbeInt_h::Op;
                                } else {
                                    rc = crate::src::src::main::sqlite3CorruptError(3263 as ::core::ffi::c_int);
                                    current_block = 9863799879598557851;
                                    break;
                                }
                                current_block = 5783071609795492627;
                            }
                            _ => {}
                        }
                        match current_block {
                            5783071609795492627 => {}
                            _ => {
                                match current_block {
                                    2644446111302234773 => {
                                        pDest = aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem;
                                        if (*pDest).flags as ::core::ffi::c_int
                                            & (crate::src::headers::vdbeInt_h::MEM_Agg | crate::src::headers::vdbeInt_h::MEM_Dyn)
                                            != 0 as ::core::ffi::c_int
                                        {
                                            crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pDest as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                                        }
                                        if (*pC_1).szRow
                                            >= *aOffset
                                                .offset(p2_1.wrapping_add(1 as crate::src::ext::rtree::rtree::u32_0) as isize)
                                        {
                                            zData = (*pC_1)
                                                .aRow
                                                .offset(*aOffset.offset(p2_1 as isize) as isize);
                                            if t < 12 as crate::src::ext::rtree::rtree::u32_0 {
                                                crate::src::src::vdbeaux::sqlite3VdbeSerialGet(
                                                    zData as *const ::core::ffi::c_uchar,
                                                    t,
                                                    
                                                    pDest as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                                );
                                            } else {
                                                static mut aFlag: [crate::src::fts5::u16_0; 2] = [
                                                    crate::src::headers::vdbeInt_h::MEM_Blob as crate::src::fts5::u16_0,
                                                    (crate::src::headers::vdbeInt_h::MEM_Str | crate::src::headers::vdbeInt_h::MEM_Term) as crate::src::fts5::u16_0,
                                                ];
                                                len = t
                                                    .wrapping_sub(12 as crate::src::ext::rtree::rtree::u32_0)
                                                    .wrapping_div(2 as crate::src::ext::rtree::rtree::u32_0)
                                                    as ::core::ffi::c_int;
                                                let __pDest_ref = unsafe { &mut *pDest };
                                                __pDest_ref.n = len;
                                                __pDest_ref.enc = encoding;
                                                if __pDest_ref.szMalloc < len + 2 as ::core::ffi::c_int
                                                {
                                                    if len
                                                        > (*db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_LENGTH as usize]
                                                    {
                                                        current_block = 7113757447962609068;
                                                        break;
                                                    }
                                                    __pDest_ref.flags = crate::src::headers::vdbeInt_h::MEM_Null as crate::src::fts5::u16_0;
                                                    if crate::src::src::vdbemem::sqlite3VdbeMemGrow(
                                                        
                                                        pDest as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                                        len + 2 as ::core::ffi::c_int,
                                                        0 as ::core::ffi::c_int,
                                                    ) != 0
                                                    {
                                                        current_block = 2471432538116610086;
                                                        break;
                                                    }
                                                } else {
                                                    __pDest_ref.z = __pDest_ref.zMalloc;
                                                }
                                                ::core::ptr::copy_nonoverlapping(
                    zData as *const u8,
                    __pDest_ref.z as *mut u8,
                    len as usize,
                );
                                                *__pDest_ref.z.offset(len as isize) =
                                                    0 as ::core::ffi::c_char;
                                                *__pDest_ref.z.offset(
                                                    (len + 1 as ::core::ffi::c_int) as isize,
                                                ) = 0 as ::core::ffi::c_char;
                                                __pDest_ref.flags = aFlag[(t & 1 as crate::src::ext::rtree::rtree::u32_0) as usize];
                                            }
                                        } else {
                                            let mut p5: crate::src::ext::rtree::rtree::u8_0 = 0;
                                            (*pDest).enc = encoding;
                                            p5 = ((*pOp).p5 as ::core::ffi::c_int
                                                & crate::src::headers::sqliteInt_h::OPFLAG_BYTELENARG)
                                                as crate::src::ext::rtree::rtree::u8_0;
                                            if p5 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                                && (p5 as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::OPFLAG_TYPEOFARG
                                                    || t >= 12 as crate::src::ext::rtree::rtree::u32_0
                                                        && (t & 1 as crate::src::ext::rtree::rtree::u32_0 == 0 as crate::src::ext::rtree::rtree::u32_0
                                                            || p5 as ::core::ffi::c_int
                                                                == crate::src::headers::sqliteInt_h::OPFLAG_BYTELENARG))
                                                || crate::src::src::vdbeaux::sqlite3VdbeSerialTypeLen(t) == 0 as crate::src::ext::rtree::rtree::u32_0
                                            {
                                                crate::src::src::vdbeaux::sqlite3VdbeSerialGet(
                                                    &raw const crate::src::src::global::sqlite3CtypeMap
                                                        as *const ::core::ffi::c_uchar
                                                        as *mut crate::src::ext::rtree::rtree::u8_0,
                                                    t,
                                                    
                                                    pDest as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                                );
                                            } else {
                                                rc = vdbeColumnFromOverflow(
                                                    pC_1,
                                                    p2_1 as ::core::ffi::c_int,
                                                    t,
                                                    *aOffset.offset(p2_1 as isize) as crate::src::ext::rtree::rtree::i64_0,
                                                    __p_ref.cacheCtr,
                                                    colCacheCtr,
                                                    pDest,
                                                );
                                                if rc != 0 {
                                                    if rc == crate::src::headers::sqlite3_h::SQLITE_NOMEM {
                                                        current_block = 2471432538116610086;
                                                        break;
                                                    }
                                                    if rc == crate::src::headers::sqlite3_h::SQLITE_TOOBIG {
                                                        current_block = 7113757447962609068;
                                                        break;
                                                    } else {
                                                        current_block = 9863799879598557851;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                updateMaxBlobsize(pDest);
                                current_block = 5783071609795492627;
                            }
                        }
                    }
    crate::src::headers::opcodes_h::OP_TypeCheck_1 =>  {
                        let mut pTab: *mut crate::src::headers::sqliteInt_h::Table = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
                        let mut aCol: *mut crate::src::headers::sqliteInt_h::Column = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Column>();
                        let mut i_1: ::core::ffi::c_int = 0;
                        let mut nCol: ::core::ffi::c_int = 0;
                        let __pOp_ref = unsafe { &*pOp };
                        pTab = __pOp_ref.p4.pTab;
                        aCol = (*pTab).aCol;
                        pIn1 = aMem.offset(__pOp_ref.p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        if __pOp_ref.p3 < 2 as ::core::ffi::c_int {
                            i_1 = 0 as ::core::ffi::c_int;
                            nCol = (*pTab).nCol as ::core::ffi::c_int;
                        } else {
                            i_1 = __pOp_ref.p3 - 2 as ::core::ffi::c_int;
                            nCol = i_1 + 1 as ::core::ffi::c_int;
                        }
                        loop {
                            if !(i_1 < nCol) {
                                current_block = 5783071609795492627;
                                break;
                            }
                            if (*aCol.offset(i_1 as isize)).colFlags as ::core::ffi::c_int
                                & crate::src::headers::sqliteInt_h::COLFLAG_GENERATED
                                != 0 as ::core::ffi::c_int
                                && __pOp_ref.p3 < 2 as ::core::ffi::c_int
                            {
                                if (*aCol.offset(i_1 as isize)).colFlags as ::core::ffi::c_int
                                    & crate::src::headers::sqliteInt_h::COLFLAG_VIRTUAL
                                    != 0 as ::core::ffi::c_int
                                {
                                    current_block = 9099392116323677207;
                                } else if __pOp_ref.p3 != 0 {
                                    pIn1 = pIn1.offset(1);
                                    current_block = 9099392116323677207;
                                } else {
                                    current_block = 14461663267880372962;
                                }
                            } else {
                                current_block = 14461663267880372962;
                            }
                            match current_block {
                                14461663267880372962 => {
                                    applyAffinity(
                                        pIn1,
                                        (*aCol.offset(i_1 as isize)).affinity,
                                        encoding,
                                    );
                                    if (*pIn1).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Null
                                        == 0 as ::core::ffi::c_int
                                    {
                                        match  (*aCol.offset(i_1 as isize)).eCType()
                                            as ::core::ffi::c_int {
    crate::src::headers::sqliteInt_h::COLTYPE_BLOB =>  {
                                                current_block = 18205226828399833612;
                                                match current_block {
                                                    3518619798157913413 => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Str
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                    18205226828399833612 => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Blob
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                    10371895734591966926 => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Int
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                    _ => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Int
                                                            != 0
                                                        {
                                                            if (*pIn1).u.i
                                                                <= 140737488355327
                                                                    as ::core::ffi::c_longlong
                                                                && (*pIn1).u.i
                                                                    >= -(140737488355328
                                                                        as ::core::ffi::c_longlong)
                                                            {
                                                                let __pIn1_ref = unsafe { &mut *pIn1 };
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    | crate::src::headers::vdbeInt_h::MEM_IntReal)
                                                                    as crate::src::fts5::u16_0;
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    & !crate::src::headers::vdbeInt_h::MEM_Int)
                                                                    as crate::src::fts5::u16_0;
                                                            } else {
                                                                let __pIn1_ref = unsafe { &mut *pIn1 };
                                                                __pIn1_ref.u.r = __pIn1_ref.u.i
                                                                    as ::core::ffi::c_double;
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    | crate::src::headers::vdbeInt_h::MEM_Real)
                                                                    as crate::src::fts5::u16_0;
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    & !crate::src::headers::vdbeInt_h::MEM_Int)
                                                                    as crate::src::fts5::u16_0;
                                                            }
                                                        } else if (*pIn1).flags
                                                            as ::core::ffi::c_int
                                                            & (crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_IntReal)
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
    crate::src::headers::sqliteInt_h::COLTYPE_INTEGER_1 | crate::src::headers::sqliteInt_h::COLTYPE_INT =>
         {
                                                current_block = 10371895734591966926;
                                                match current_block {
                                                    3518619798157913413 => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Str
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                    18205226828399833612 => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Blob
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                    10371895734591966926 => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Int
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                    _ => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Int
                                                            != 0
                                                        {
                                                            if (*pIn1).u.i
                                                                <= 140737488355327
                                                                    as ::core::ffi::c_longlong
                                                                && (*pIn1).u.i
                                                                    >= -(140737488355328
                                                                        as ::core::ffi::c_longlong)
                                                            {
                                                                let __pIn1_ref = unsafe { &mut *pIn1 };
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    | crate::src::headers::vdbeInt_h::MEM_IntReal)
                                                                    as crate::src::fts5::u16_0;
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    & !crate::src::headers::vdbeInt_h::MEM_Int)
                                                                    as crate::src::fts5::u16_0;
                                                            } else {
                                                                let __pIn1_ref = unsafe { &mut *pIn1 };
                                                                __pIn1_ref.u.r = __pIn1_ref.u.i
                                                                    as ::core::ffi::c_double;
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    | crate::src::headers::vdbeInt_h::MEM_Real)
                                                                    as crate::src::fts5::u16_0;
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    & !crate::src::headers::vdbeInt_h::MEM_Int)
                                                                    as crate::src::fts5::u16_0;
                                                            }
                                                        } else if (*pIn1).flags
                                                            as ::core::ffi::c_int
                                                            & (crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_IntReal)
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
    crate::src::headers::sqliteInt_h::COLTYPE_TEXT =>  {
                                                current_block = 3518619798157913413;
                                                match current_block {
                                                    3518619798157913413 => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Str
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                    18205226828399833612 => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Blob
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                    10371895734591966926 => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Int
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                    _ => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Int
                                                            != 0
                                                        {
                                                            if (*pIn1).u.i
                                                                <= 140737488355327
                                                                    as ::core::ffi::c_longlong
                                                                && (*pIn1).u.i
                                                                    >= -(140737488355328
                                                                        as ::core::ffi::c_longlong)
                                                            {
                                                                let __pIn1_ref = unsafe { &mut *pIn1 };
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    | crate::src::headers::vdbeInt_h::MEM_IntReal)
                                                                    as crate::src::fts5::u16_0;
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    & !crate::src::headers::vdbeInt_h::MEM_Int)
                                                                    as crate::src::fts5::u16_0;
                                                            } else {
                                                                let __pIn1_ref = unsafe { &mut *pIn1 };
                                                                __pIn1_ref.u.r = __pIn1_ref.u.i
                                                                    as ::core::ffi::c_double;
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    | crate::src::headers::vdbeInt_h::MEM_Real)
                                                                    as crate::src::fts5::u16_0;
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    & !crate::src::headers::vdbeInt_h::MEM_Int)
                                                                    as crate::src::fts5::u16_0;
                                                            }
                                                        } else if (*pIn1).flags
                                                            as ::core::ffi::c_int
                                                            & (crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_IntReal)
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
    crate::src::headers::sqliteInt_h::COLTYPE_REAL =>  {
                                                current_block = 6603035963279056686;
                                                match current_block {
                                                    3518619798157913413 => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Str
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                    18205226828399833612 => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Blob
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                    10371895734591966926 => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Int
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                    _ => {
                                                        if (*pIn1).flags as ::core::ffi::c_int
                                                            & crate::src::headers::vdbeInt_h::MEM_Int
                                                            != 0
                                                        {
                                                            if (*pIn1).u.i
                                                                <= 140737488355327
                                                                    as ::core::ffi::c_longlong
                                                                && (*pIn1).u.i
                                                                    >= -(140737488355328
                                                                        as ::core::ffi::c_longlong)
                                                            {
                                                                let __pIn1_ref = unsafe { &mut *pIn1 };
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    | crate::src::headers::vdbeInt_h::MEM_IntReal)
                                                                    as crate::src::fts5::u16_0;
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    & !crate::src::headers::vdbeInt_h::MEM_Int)
                                                                    as crate::src::fts5::u16_0;
                                                            } else {
                                                                let __pIn1_ref = unsafe { &mut *pIn1 };
                                                                __pIn1_ref.u.r = __pIn1_ref.u.i
                                                                    as ::core::ffi::c_double;
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    | crate::src::headers::vdbeInt_h::MEM_Real)
                                                                    as crate::src::fts5::u16_0;
                                                                __pIn1_ref.flags = (__pIn1_ref.flags
                                                                    as ::core::ffi::c_int
                                                                    & !crate::src::headers::vdbeInt_h::MEM_Int)
                                                                    as crate::src::fts5::u16_0;
                                                            }
                                                        } else if (*pIn1).flags
                                                            as ::core::ffi::c_int
                                                            & (crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_IntReal)
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            current_block = 8094297922442305621;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
    _ =>  {}
}
                                    }
                                    pIn1 = pIn1.offset(1);
                                }
                                _ => {}
                            }
                            i_1 += 1;
                        }
                        match current_block {
                            5783071609795492627 => {}
                            _ => {
                                crate::src::src::vdbeaux::sqlite3VdbeError(
                                    
                                    p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                    b"cannot store %s value in %s column %s.%s\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    vdbeMemTypeName(pIn1),
                                    *(&raw mut crate::src::src::global::sqlite3StdType as *mut *const ::core::ffi::c_char)
                                        .offset(
                                            ((*aCol.offset(i_1 as isize)).eCType()
                                                as ::core::ffi::c_int
                                                - 1 as ::core::ffi::c_int)
                                                as isize,
                                        ),
                                    (*pTab).zName,
                                    (*aCol.offset(i_1 as isize)).zCnName,
                                );
                                rc = crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT_DATATYPE;
                                current_block = 9863799879598557851;
                                break;
                            }
                        }
                    }
    crate::src::headers::opcodes_h::OP_Affinity_1 =>  {
                        let mut zAffinity: *const ::core::ffi::c_char =
                            ::core::ptr::null::<::core::ffi::c_char>();
                        zAffinity = (*pOp).p4.z;
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        loop {
                            applyAffinity(
                                pIn1,
                                *zAffinity.offset(0 as isize),
                                encoding,
                            );
                            if *zAffinity.offset(0 as isize)
                                as ::core::ffi::c_int
                                == crate::src::headers::sqliteInt_h::SQLITE_AFF_REAL
                                && (*pIn1).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Int
                                    != 0 as ::core::ffi::c_int
                            {
                                if (*pIn1).u.i <= 140737488355327 as ::core::ffi::c_longlong
                                    && (*pIn1).u.i >= -(140737488355328 as ::core::ffi::c_longlong)
                                {
                                    let __pIn1_ref = unsafe { &mut *pIn1 };
                                    __pIn1_ref.flags = (__pIn1_ref.flags as ::core::ffi::c_int
                                        | crate::src::headers::vdbeInt_h::MEM_IntReal)
                                        as crate::src::fts5::u16_0;
                                    __pIn1_ref.flags =
                                        (__pIn1_ref.flags as ::core::ffi::c_int & !crate::src::headers::vdbeInt_h::MEM_Int) as crate::src::fts5::u16_0;
                                } else {
                                    let __pIn1_ref = unsafe { &mut *pIn1 };
                                    __pIn1_ref.u.r = __pIn1_ref.u.i as ::core::ffi::c_double;
                                    __pIn1_ref.flags =
                                        (__pIn1_ref.flags as ::core::ffi::c_int | crate::src::headers::vdbeInt_h::MEM_Real) as crate::src::fts5::u16_0;
                                    __pIn1_ref.flags = (__pIn1_ref.flags as ::core::ffi::c_int
                                        & !(crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_Str))
                                        as crate::src::fts5::u16_0;
                                }
                            }
                            zAffinity = zAffinity.offset(1);
                            if *zAffinity.offset(0 as isize)
                                as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                            pIn1 = pIn1.offset(1);
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_MakeRecord_1 =>  {
                        let mut pRec: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let mut nData: crate::src::ext::rtree::rtree::u64_0 = 0;
                        let mut nHdr: ::core::ffi::c_int = 0;
                        let mut nByte_0: crate::src::ext::rtree::rtree::i64_0 = 0;
                        let mut nZero: crate::src::ext::rtree::rtree::i64_0 = 0;
                        let mut nVarint: ::core::ffi::c_int = 0;
                        let mut serial_type: crate::src::ext::rtree::rtree::u32_0 = 0;
                        let mut pData0: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let mut pLast: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let mut nField: ::core::ffi::c_int = 0;
                        let mut zAffinity_0: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        let mut len_0: crate::src::ext::rtree::rtree::u32_0 = 0;
                        let mut zHdr_0: *mut crate::src::ext::rtree::rtree::u8_0 = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u8_0>();
                        let mut zPayload: *mut crate::src::ext::rtree::rtree::u8_0 = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u8_0>();
                        nData = 0 as crate::src::ext::rtree::rtree::u64_0;
                        nHdr = 0 as ::core::ffi::c_int;
                        nZero = 0 as crate::src::ext::rtree::rtree::i64_0;
                        let __pOp_ref = unsafe { &*pOp };
                        nField = __pOp_ref.p1;
                        zAffinity_0 = __pOp_ref.p4.z;
                        pData0 = aMem.offset(nField as isize) as *mut crate::src::src::vdbe::Mem;
                        nField = __pOp_ref.p2;
                        pLast =
                            pData0.offset((nField - 1 as ::core::ffi::c_int) as isize) as *mut crate::src::src::vdbe::Mem;
                        pOut = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        if !zAffinity_0.is_null() {
                            pRec = pData0;
                            loop {
                                applyAffinity(
                                    pRec,
                                    *zAffinity_0.offset(0 as isize),
                                    encoding,
                                );
                                if *zAffinity_0.offset(0 as isize)
                                    as ::core::ffi::c_int
                                    == crate::src::headers::sqliteInt_h::SQLITE_AFF_REAL
                                    && (*pRec).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Int != 0
                                {
                                    let __pRec_ref = unsafe { &mut *pRec };
                                    __pRec_ref.flags = (__pRec_ref.flags as ::core::ffi::c_int
                                        | crate::src::headers::vdbeInt_h::MEM_IntReal)
                                        as crate::src::fts5::u16_0;
                                    __pRec_ref.flags = (__pRec_ref.flags as ::core::ffi::c_int
                                        & !(0x4 as ::core::ffi::c_int))
                                        as crate::src::fts5::u16_0;
                                }
                                zAffinity_0 = zAffinity_0.offset(1);
                                pRec = pRec.offset(1);
                                if !(*zAffinity_0.offset(0 as isize) != 0) {
                                    break;
                                }
                            }
                        }
                        pRec = pLast;
                        loop {
                            if (*pRec).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Null != 0 {
                                if (*pRec).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Zero != 0 {
                                    (*pRec).uTemp = 10 as crate::src::ext::rtree::rtree::u32_0;
                                } else {
                                    (*pRec).uTemp = 0 as crate::src::ext::rtree::rtree::u32_0;
                                }
                                nHdr += 1;
                            } else if (*pRec).flags as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_IntReal)
                                != 0
                            {
                                let mut i_2: crate::src::ext::rtree::rtree::i64_0 = (*pRec).u.i;
                                let mut uu: crate::src::ext::rtree::rtree::u64_0 = 0;
                                if i_2 < 0 as crate::src::ext::rtree::rtree::i64_0 {
                                    uu = !i_2 as crate::src::ext::rtree::rtree::u64_0;
                                } else {
                                    uu = i_2 as crate::src::ext::rtree::rtree::u64_0;
                                }
                                nHdr += 1;
                                if uu <= 127 as crate::src::ext::rtree::rtree::u64_0 {
                                    if i_2 & 1 as crate::src::ext::rtree::rtree::i64_0 == i_2
                                        && __p_ref.minWriteFileFormat as ::core::ffi::c_int
                                            >= 4 as ::core::ffi::c_int
                                    {
                                        (*pRec).uTemp = (8 as crate::src::ext::rtree::rtree::u32_0).wrapping_add(uu as crate::src::ext::rtree::rtree::u32_0);
                                    } else {
                                        nData = nData.wrapping_add(1);
                                        (*pRec).uTemp = 1 as crate::src::ext::rtree::rtree::u32_0;
                                    }
                                } else if uu <= 32767 as crate::src::ext::rtree::rtree::u64_0 {
                                    nData = nData.wrapping_add(2 as crate::src::ext::rtree::rtree::u64_0);
                                    (*pRec).uTemp = 2 as crate::src::ext::rtree::rtree::u32_0;
                                } else if uu <= 8388607 as crate::src::ext::rtree::rtree::u64_0 {
                                    nData = nData.wrapping_add(3 as crate::src::ext::rtree::rtree::u64_0);
                                    (*pRec).uTemp = 3 as crate::src::ext::rtree::rtree::u32_0;
                                } else if uu <= 2147483647 as crate::src::ext::rtree::rtree::u64_0 {
                                    nData = nData.wrapping_add(4 as crate::src::ext::rtree::rtree::u64_0);
                                    (*pRec).uTemp = 4 as crate::src::ext::rtree::rtree::u32_0;
                                } else if uu <= 140737488355327 as crate::src::ext::rtree::rtree::u64_0 {
                                    nData = nData.wrapping_add(6 as crate::src::ext::rtree::rtree::u64_0);
                                    (*pRec).uTemp = 5 as crate::src::ext::rtree::rtree::u32_0;
                                } else {
                                    nData = nData.wrapping_add(8 as crate::src::ext::rtree::rtree::u64_0);
                                    if (*pRec).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_IntReal != 0 {
                                        let __pRec_ref = unsafe { &mut *pRec };
                                        __pRec_ref.u.r = __pRec_ref.u.i as ::core::ffi::c_double;
                                        __pRec_ref.flags = (__pRec_ref.flags as ::core::ffi::c_int
                                            & !crate::src::headers::vdbeInt_h::MEM_IntReal)
                                            as crate::src::fts5::u16_0;
                                        __pRec_ref.flags = (__pRec_ref.flags as ::core::ffi::c_int
                                            | crate::src::headers::vdbeInt_h::MEM_Real)
                                            as crate::src::fts5::u16_0;
                                        __pRec_ref.uTemp = 7 as crate::src::ext::rtree::rtree::u32_0;
                                    } else {
                                        (*pRec).uTemp = 6 as crate::src::ext::rtree::rtree::u32_0;
                                    }
                                }
                            } else if (*pRec).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Real != 0 {
                                nHdr += 1;
                                nData = nData.wrapping_add(8 as crate::src::ext::rtree::rtree::u64_0);
                                (*pRec).uTemp = 7 as crate::src::ext::rtree::rtree::u32_0;
                            } else {
                                let __pRec_ref = unsafe { &mut *pRec };
                                len_0 = __pRec_ref.n as crate::src::ext::rtree::rtree::u32_0;
                                serial_type = len_0
                                    .wrapping_mul(2 as crate::src::ext::rtree::rtree::u32_0)
                                    .wrapping_add(12 as crate::src::ext::rtree::rtree::u32_0)
                                    .wrapping_add(
                                        (__pRec_ref.flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Str
                                            != 0 as ::core::ffi::c_int)
                                            as ::core::ffi::c_int
                                            as crate::src::ext::rtree::rtree::u32_0,
                                    );
                                if __pRec_ref.flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Zero != 0 {
                                    serial_type = serial_type.wrapping_add(
                                        (__pRec_ref.u.nZero as crate::src::ext::rtree::rtree::u32_0).wrapping_mul(2 as crate::src::ext::rtree::rtree::u32_0),
                                    );
                                    if nData != 0 {
                                        if crate::src::src::vdbemem::sqlite3VdbeMemExpandBlob(pRec as *mut crate::src::headers::vdbeInt_h::sqlite3_value) != 0 {
                                            current_block = 2471432538116610086;
                                            break 's_109;
                                        }
                                        len_0 = len_0.wrapping_add(__pRec_ref.u.nZero as crate::src::ext::rtree::rtree::u32_0);
                                    } else {
                                        nZero += __pRec_ref.u.nZero as crate::src::ext::rtree::rtree::i64_0;
                                    }
                                }
                                nData = nData.wrapping_add(len_0 as crate::src::ext::rtree::rtree::u64_0);
                                nHdr += crate::src::src::util::sqlite3VarintLen(serial_type as crate::src::ext::rtree::rtree::u64_0);
                                __pRec_ref.uTemp = serial_type;
                            }
                            if pRec == pData0 {
                                break;
                            }
                            pRec = pRec.offset(-1);
                        }
                        if nHdr <= 126 as ::core::ffi::c_int {
                            nHdr += 1 as ::core::ffi::c_int;
                        } else {
                            nVarint = crate::src::src::util::sqlite3VarintLen(nHdr as crate::src::ext::rtree::rtree::u64_0);
                            nHdr += nVarint;
                            if nVarint < crate::src::src::util::sqlite3VarintLen(nHdr as crate::src::ext::rtree::rtree::u64_0) {
                                nHdr += 1;
                            }
                        }
                        nByte_0 = (nHdr as crate::src::ext::rtree::rtree::u64_0).wrapping_add(nData) as crate::src::ext::rtree::rtree::i64_0;
                        if nByte_0 + nZero <= (*pOut).szMalloc as crate::src::ext::rtree::rtree::i64_0 {
                            (*pOut).z = (*pOut).zMalloc;
                        } else {
                            if nByte_0 + nZero > (*db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_LENGTH as usize] as crate::src::ext::rtree::rtree::i64_0
                            {
                                current_block = 7113757447962609068;
                                break;
                            }
                            if crate::src::src::vdbemem::sqlite3VdbeMemClearAndResize(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value, nByte_0 as ::core::ffi::c_int)
                                != 0
                            {
                                current_block = 2471432538116610086;
                                break;
                            }
                        }
                        (*pOut).n = nByte_0 as ::core::ffi::c_int;
                        (*pOut).flags = crate::src::headers::vdbeInt_h::MEM_Blob as crate::src::fts5::u16_0;
                        if nZero != 0 {
                            let __pOut_ref = unsafe { &mut *pOut };
                            __pOut_ref.u.nZero = nZero as ::core::ffi::c_int;
                            __pOut_ref.flags =
                                (__pOut_ref.flags as ::core::ffi::c_int | crate::src::headers::vdbeInt_h::MEM_Zero) as crate::src::fts5::u16_0;
                        }
                        updateMaxBlobsize(pOut);
                        zHdr_0 = (*pOut).z as *mut crate::src::ext::rtree::rtree::u8_0;
                        zPayload = zHdr_0.offset(nHdr as isize);
                        if nHdr < 0x80 as ::core::ffi::c_int {
                            let fresh5 = zHdr_0;
                            zHdr_0 = zHdr_0.offset(1);
                            *fresh5 = nHdr as crate::src::ext::rtree::rtree::u8_0;
                        } else {
                            zHdr_0 = zHdr_0.offset(crate::src::src::util::sqlite3PutVarint(
                                zHdr_0 as *mut ::core::ffi::c_uchar,
                                nHdr as crate::src::ext::rtree::rtree::u64_0,
                            ) as isize);
                        }
                        pRec = pData0;
                        loop {
                            serial_type = (*pRec).uTemp;
                            if serial_type <= 7 as crate::src::ext::rtree::rtree::u32_0 {
                                let fresh6 = zHdr_0;
                                zHdr_0 = zHdr_0.offset(1);
                                *fresh6 = serial_type as crate::src::ext::rtree::rtree::u8_0;
                                if !(serial_type == 0 as crate::src::ext::rtree::rtree::u32_0) {
                                    let mut v: crate::src::ext::rtree::rtree::u64_0 = 0;
                                    if serial_type == 7 as crate::src::ext::rtree::rtree::u32_0 {
                                        ::core::ptr::copy_nonoverlapping(
                    &raw mut (*pRec).u.r as *const u8,
                    &raw mut v as *mut u8,
                    ::core::mem::size_of::<crate::src::ext::rtree::rtree::u64_0>() as usize,
                );
                                    } else {
                                        v = (*pRec).u.i as crate::src::ext::rtree::rtree::u64_0;
                                    }
                                    len_0 = *(&raw const crate::src::src::vdbeaux::sqlite3SmallTypeSizes as *const crate::src::ext::rtree::rtree::u8_0)
                                        .offset(serial_type as isize)
                                        as crate::src::ext::rtree::rtree::u32_0;
                                    let mut current_block_959: u64;
                                    match len_0 {
                                        6 => {
                                            current_block_959 = 5984951798085141333;
                                        }
                                        4 => {
                                            current_block_959 = 367661657315018966;
                                        }
                                        3 => {
                                            current_block_959 = 17706572088613336346;
                                        }
                                        2 => {
                                            current_block_959 = 18141179283682394123;
                                        }
                                        1 => {
                                            current_block_959 = 249133735357546894;
                                        }
                                        _ => {
                                            *zPayload.offset(7 as isize) =
                                                (v & 0xff as crate::src::ext::rtree::rtree::u64_0) as crate::src::ext::rtree::rtree::u8_0;
                                            v >>= 8 as ::core::ffi::c_int;
                                            *zPayload.offset(6 as isize) =
                                                (v & 0xff as crate::src::ext::rtree::rtree::u64_0) as crate::src::ext::rtree::rtree::u8_0;
                                            v >>= 8 as ::core::ffi::c_int;
                                            current_block_959 = 5984951798085141333;
                                        }
                                    }
                                    match current_block_959 {
                                        5984951798085141333 => {
                                            *zPayload.offset(5 as isize) =
                                                (v & 0xff as crate::src::ext::rtree::rtree::u64_0) as crate::src::ext::rtree::rtree::u8_0;
                                            v >>= 8 as ::core::ffi::c_int;
                                            *zPayload.offset(4 as isize) =
                                                (v & 0xff as crate::src::ext::rtree::rtree::u64_0) as crate::src::ext::rtree::rtree::u8_0;
                                            v >>= 8 as ::core::ffi::c_int;
                                            current_block_959 = 367661657315018966;
                                        }
                                        _ => {}
                                    }
                                    match current_block_959 {
                                        367661657315018966 => {
                                            *zPayload.offset(3 as isize) =
                                                (v & 0xff as crate::src::ext::rtree::rtree::u64_0) as crate::src::ext::rtree::rtree::u8_0;
                                            v >>= 8 as ::core::ffi::c_int;
                                            current_block_959 = 17706572088613336346;
                                        }
                                        _ => {}
                                    }
                                    match current_block_959 {
                                        17706572088613336346 => {
                                            *zPayload.offset(2 as isize) =
                                                (v & 0xff as crate::src::ext::rtree::rtree::u64_0) as crate::src::ext::rtree::rtree::u8_0;
                                            v >>= 8 as ::core::ffi::c_int;
                                            current_block_959 = 18141179283682394123;
                                        }
                                        _ => {}
                                    }
                                    match current_block_959 {
                                        18141179283682394123 => {
                                            *zPayload.offset(1 as isize) =
                                                (v & 0xff as crate::src::ext::rtree::rtree::u64_0) as crate::src::ext::rtree::rtree::u8_0;
                                            v >>= 8 as ::core::ffi::c_int;
                                        }
                                        _ => {}
                                    }
                                    *zPayload.offset(0 as isize) =
                                        (v & 0xff as crate::src::ext::rtree::rtree::u64_0) as crate::src::ext::rtree::rtree::u8_0;
                                    zPayload = zPayload.offset(len_0 as isize);
                                }
                            } else if serial_type < 0x80 as crate::src::ext::rtree::rtree::u32_0 {
                                let fresh7 = zHdr_0;
                                zHdr_0 = zHdr_0.offset(1);
                                *fresh7 = serial_type as crate::src::ext::rtree::rtree::u8_0;
                                if serial_type >= 14 as crate::src::ext::rtree::rtree::u32_0 && (*pRec).n > 0 as ::core::ffi::c_int
                                {
                                    let __pRec_ref = unsafe { &*pRec };
                                    ::core::ptr::copy_nonoverlapping(
                    __pRec_ref.z as *const u8,
                    zPayload as *mut u8,
                    __pRec_ref.n as usize,
                );
                                    zPayload = zPayload.offset(__pRec_ref.n as isize);
                                }
                            } else {
                                zHdr_0 = zHdr_0.offset(crate::src::src::util::sqlite3PutVarint(
                                    zHdr_0 as *mut ::core::ffi::c_uchar,
                                    serial_type as crate::src::ext::rtree::rtree::u64_0,
                                ) as isize);
                                if (*pRec).n != 0 {
                                    let __pRec_ref = unsafe { &*pRec };
                                    ::core::ptr::copy_nonoverlapping(
                    __pRec_ref.z as *const u8,
                    zPayload as *mut u8,
                    __pRec_ref.n as usize,
                );
                                    zPayload = zPayload.offset(__pRec_ref.n as isize);
                                }
                            }
                            if pRec == pLast {
                                break;
                            }
                            pRec = pRec.offset(1);
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Count_1 =>  {
                        let mut nEntry: crate::src::ext::rtree::rtree::i64_0 = 0;
                        let mut pCrsr_0: *mut crate::src::headers::btreeInt_h::BtCursor = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>();
                        pCrsr_0 = (**__p_ref.apCsr.offset((*pOp).p1 as isize)).uc.pCursor;
                        if (*pOp).p3 != 0 {
                            nEntry = crate::src::src::btree::sqlite3BtreeRowCountEst(pCrsr_0);
                        } else {
                            nEntry = 0 as crate::src::ext::rtree::rtree::i64_0;
                            rc = crate::src::src::btree::sqlite3BtreeCount(db as *mut crate::src::headers::sqliteInt_h::sqlite3, pCrsr_0, &raw mut nEntry);
                            if rc != 0 {
                                current_block = 9863799879598557851;
                                break;
                            }
                        }
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        (*pOut).u.i = nEntry;
                        current_block = 20147595251170673;
                    }
    crate::src::headers::opcodes_h::OP_Savepoint_1 =>  {
                        let mut p1_1: ::core::ffi::c_int = 0;
                        let mut zName: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        let mut nName: ::core::ffi::c_int = 0;
                        let mut pNew: *mut crate::src::headers::sqliteInt_h::Savepoint = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Savepoint>();
                        let mut pSavepoint: *mut crate::src::headers::sqliteInt_h::Savepoint = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Savepoint>();
                        let mut pTmp: *mut crate::src::headers::sqliteInt_h::Savepoint = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Savepoint>();
                        let mut iSavepoint: ::core::ffi::c_int = 0;
                        let mut ii: ::core::ffi::c_int = 0;
                        p1_1 = (*pOp).p1;
                        zName = (*pOp).p4.z;
                        if p1_1 == crate::src::headers::sqliteInt_h::SAVEPOINT_BEGIN {
                            if (*db).nVdbeWrite > 0 as ::core::ffi::c_int {
                                crate::src::src::vdbeaux::sqlite3VdbeError(
                                    
                                    p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                    b"cannot open savepoint - SQL statements in progress\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
                            } else {
                                nName = crate::src::src::util::sqlite3Strlen30(zName);
                                rc = crate::src::src::vtab::sqlite3VtabSavepoint(
                                    
                                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                    crate::src::headers::sqliteInt_h::SAVEPOINT_BEGIN,
                                    (*db).nStatement + (*db).nSavepoint,
                                );
                                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                    current_block = 9863799879598557851;
                                    break;
                                }
                                pNew = crate::src::src::malloc::sqlite3DbMallocRawNN(
                                    
                                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                    (::core::mem::size_of::<crate::src::headers::sqliteInt_h::Savepoint>() as usize)
                                        .wrapping_add(nName as usize)
                                        .wrapping_add(1 as usize)
                                        as crate::src::ext::rtree::rtree::u64_0,
                                ) as *mut crate::src::headers::sqliteInt_h::Savepoint;
                                if !pNew.is_null() {
                                    let __pNew_ref = unsafe { &mut *pNew };
                                    __pNew_ref.zName = pNew.offset(1 as isize)
                                        as *mut crate::src::headers::sqliteInt_h::Savepoint
                                        as *mut ::core::ffi::c_char;
                                    ::core::ptr::copy_nonoverlapping(
                    zName as *const u8,
                    __pNew_ref.zName as *mut u8,
                    (nName + 1 as ::core::ffi::c_int) as usize,
                );
                                    let __db_ref = unsafe { &mut *db };
                                    if __db_ref.autoCommit != 0 {
                                        __db_ref.autoCommit = 0 as crate::src::ext::rtree::rtree::u8_0;
                                        __db_ref.isTransactionSavepoint = 1 as crate::src::ext::rtree::rtree::u8_0;
                                    } else {
                                        __db_ref.nSavepoint += 1;
                                    }
                                    __pNew_ref.pNext = __db_ref.pSavepoint;
                                    __db_ref.pSavepoint = pNew;
                                    __pNew_ref.nDeferredCons = __db_ref.nDeferredCons;
                                    __pNew_ref.nDeferredImmCons = __db_ref.nDeferredImmCons;
                                }
                            }
                        } else {
                            iSavepoint = 0 as ::core::ffi::c_int;
                            pSavepoint = (*db).pSavepoint;
                            while !pSavepoint.is_null()
                                && crate::src::src::util::sqlite3StrICmp((*pSavepoint).zName, zName) != 0
                            {
                                iSavepoint += 1;
                                pSavepoint = (*pSavepoint).pNext;
                            }
                            if pSavepoint.is_null() {
                                crate::src::src::vdbeaux::sqlite3VdbeError(
                                    
                                    p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                    b"no such savepoint: %s\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    zName,
                                );
                                rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                            } else if (*db).nVdbeWrite > 0 as ::core::ffi::c_int
                                && p1_1 == crate::src::headers::sqliteInt_h::SAVEPOINT_RELEASE
                            {
                                crate::src::src::vdbeaux::sqlite3VdbeError(
                                    
                                    p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                    b"cannot release savepoint - SQL statements in progress\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
                            } else {
                                let mut isTransaction: ::core::ffi::c_int =
                                    ((*pSavepoint).pNext.is_null()
                                        && (*db).isTransactionSavepoint as ::core::ffi::c_int != 0)
                                        as ::core::ffi::c_int;
                                if isTransaction != 0 && p1_1 == crate::src::headers::sqliteInt_h::SAVEPOINT_RELEASE {
                                    rc = crate::src::src::vdbeaux::sqlite3VdbeCheckFkDeferred(p as *mut crate::src::headers::vdbeInt_h::Vdbe);
                                    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                        current_block = 10408586827809538586;
                                        break;
                                    }
                                    (*db).autoCommit = 1 as crate::src::ext::rtree::rtree::u8_0;
                                    if crate::src::src::vdbeaux::sqlite3VdbeHalt(p as *mut crate::src::headers::vdbeInt_h::Vdbe) == crate::src::headers::sqlite3_h::SQLITE_BUSY {
                                        __p_ref.pc = pOp.offset_from(aOp) as ::core::ffi::c_long
                                            as ::core::ffi::c_int;
                                        (*db).autoCommit = 0 as crate::src::ext::rtree::rtree::u8_0;
                                        rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
                                        __p_ref.rc = rc;
                                        current_block = 10408586827809538586;
                                        break;
                                    } else {
                                        rc = __p_ref.rc;
                                        if rc != 0 {
                                            (*db).autoCommit = 0 as crate::src::ext::rtree::rtree::u8_0;
                                        } else {
                                            (*db).isTransactionSavepoint = 0 as crate::src::ext::rtree::rtree::u8_0;
                                        }
                                    }
                                } else {
                                    let mut isSchemaChange: ::core::ffi::c_int = 0;
                                    iSavepoint =
                                        (*db).nSavepoint - iSavepoint - 1 as ::core::ffi::c_int;
                                    if p1_1 == crate::src::headers::sqliteInt_h::SAVEPOINT_ROLLBACK {
                                        isSchemaChange = ((*db).mDbFlags
                                            & crate::src::headers::sqliteInt_h::DBFLAG_SchemaChange as crate::src::ext::rtree::rtree::u32_0
                                            != 0 as crate::src::ext::rtree::rtree::u32_0)
                                            as ::core::ffi::c_int;
                                        ii = 0 as ::core::ffi::c_int;
                                        while ii < (*db).nDb {
                                            rc = crate::src::src::btree::sqlite3BtreeTripAllCursors(
                                                (*(*db).aDb.offset(ii as isize)).pBt,
                                                crate::src::headers::sqlite3_h::SQLITE_ABORT_ROLLBACK_1,
                                                (isSchemaChange == 0 as ::core::ffi::c_int)
                                                    as ::core::ffi::c_int,
                                            );
                                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                                current_block = 9863799879598557851;
                                                break 's_109;
                                            }
                                            ii += 1;
                                        }
                                    } else {
                                        isSchemaChange = 0 as ::core::ffi::c_int;
                                    }
                                    ii = 0 as ::core::ffi::c_int;
                                    while ii < (*db).nDb {
                                        rc = crate::src::src::btree::sqlite3BtreeSavepoint(
                                            (*(*db).aDb.offset(ii as isize)).pBt,
                                            p1_1,
                                            iSavepoint,
                                        );
                                        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                            current_block = 9863799879598557851;
                                            break 's_109;
                                        }
                                        ii += 1;
                                    }
                                    if isSchemaChange != 0 {
                                        crate::src::src::vdbeaux::sqlite3ExpirePreparedStatements(
                                            
                                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                            0 as ::core::ffi::c_int,
                                        );
                                        crate::src::src::build::sqlite3ResetAllSchemasOfConnection(db as *mut crate::src::headers::sqliteInt_h::sqlite3);
                                        (*db).mDbFlags |= crate::src::headers::sqliteInt_h::DBFLAG_SchemaChange as crate::src::ext::rtree::rtree::u32_0;
                                    }
                                }
                                if rc != 0 {
                                    current_block = 9863799879598557851;
                                    break;
                                }
                                while (*db).pSavepoint != pSavepoint {
                                    let __db_ref = unsafe { &mut *db };
                                    pTmp = __db_ref.pSavepoint;
                                    __db_ref.pSavepoint = (*pTmp).pNext;
                                    crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, pTmp as *mut ::core::ffi::c_void);
                                    __db_ref.nSavepoint -= 1;
                                }
                                if p1_1 == crate::src::headers::sqliteInt_h::SAVEPOINT_RELEASE {
                                    (*db).pSavepoint = (*pSavepoint).pNext;
                                    crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, pSavepoint as *mut ::core::ffi::c_void);
                                    if isTransaction == 0 {
                                        (*db).nSavepoint -= 1;
                                    }
                                } else {
                                    (*db).nDeferredCons = (*pSavepoint).nDeferredCons;
                                    (*db).nDeferredImmCons = (*pSavepoint).nDeferredImmCons;
                                }
                                if isTransaction == 0 || p1_1 == crate::src::headers::sqliteInt_h::SAVEPOINT_ROLLBACK {
                                    rc = crate::src::src::vtab::sqlite3VtabSavepoint(db as *mut crate::src::headers::sqliteInt_h::sqlite3, p1_1, iSavepoint);
                                    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                        current_block = 9863799879598557851;
                                        break;
                                    }
                                }
                            }
                        }
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        if __p_ref.eVdbeState as ::core::ffi::c_int == crate::src::headers::vdbeInt_h::VDBE_HALT_STATE {
                            rc = crate::src::headers::sqlite3_h::SQLITE_DONE;
                            current_block = 10408586827809538586;
                            break;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_AutoCommit_1 =>  {
                        let mut desiredAutoCommit: ::core::ffi::c_int = 0;
                        let mut iRollback: ::core::ffi::c_int = 0;
                        desiredAutoCommit = (*pOp).p1;
                        iRollback = (*pOp).p2;
                        if desiredAutoCommit != (*db).autoCommit as ::core::ffi::c_int {
                            if iRollback != 0 {
                                crate::src::src::main::sqlite3RollbackAll(db as *mut crate::src::headers::sqliteInt_h::sqlite3, crate::src::headers::sqlite3_h::SQLITE_ABORT_ROLLBACK_1);
                                (*db).autoCommit = 1 as crate::src::ext::rtree::rtree::u8_0;
                            } else if desiredAutoCommit != 0
                                && (*db).nVdbeWrite > 0 as ::core::ffi::c_int
                            {
                                crate::src::src::vdbeaux::sqlite3VdbeError(
                                    
                                    p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                    b"cannot commit transaction - SQL statements in progress\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
                                current_block = 9863799879598557851;
                                break;
                            } else {
                                rc = crate::src::src::vdbeaux::sqlite3VdbeCheckFkDeferred(p as *mut crate::src::headers::vdbeInt_h::Vdbe);
                                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                    current_block = 10408586827809538586;
                                    break;
                                }
                                (*db).autoCommit = desiredAutoCommit as crate::src::ext::rtree::rtree::u8_0;
                            }
                            if crate::src::src::vdbeaux::sqlite3VdbeHalt(p as *mut crate::src::headers::vdbeInt_h::Vdbe) == crate::src::headers::sqlite3_h::SQLITE_BUSY {
                                __p_ref.pc = pOp.offset_from(aOp) as ::core::ffi::c_long
                                    as ::core::ffi::c_int;
                                (*db).autoCommit =
                                    (1 as ::core::ffi::c_int - desiredAutoCommit) as crate::src::ext::rtree::rtree::u8_0;
                                rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
                                __p_ref.rc = rc;
                                current_block = 10408586827809538586;
                                break;
                            } else {
                                crate::src::src::main::sqlite3CloseSavepoints(db as *mut crate::src::headers::sqliteInt_h::sqlite3);
                                if __p_ref.rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                    rc = crate::src::headers::sqlite3_h::SQLITE_DONE;
                                } else {
                                    rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                                }
                                current_block = 10408586827809538586;
                                break;
                            }
                        } else {
                            crate::src::src::vdbeaux::sqlite3VdbeError(
                                
                                p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                if desiredAutoCommit == 0 {
                                    b"cannot start a transaction within a transaction\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char
                                } else if iRollback != 0 {
                                    b"cannot rollback - no transaction is active\0" as *const u8
                                        as *const ::core::ffi::c_char
                                } else {
                                    b"cannot commit - no transaction is active\0" as *const u8
                                        as *const ::core::ffi::c_char
                                },
                            );
                            rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                            current_block = 9863799879598557851;
                            break;
                        }
                    }
    crate::src::headers::opcodes_h::OP_Transaction_1 =>  {
                        let mut pBt: *mut crate::src::headers::btreeInt_h::Btree = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::Btree>();
                        let mut pDb: *mut crate::src::headers::sqliteInt_h::Db = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Db>();
                        let mut iMeta: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        if (*pOp).p2 != 0
                            && (*db).flags & (crate::src::headers::sqliteInt_h::SQLITE_QueryOnly as crate::src::ext::rtree::rtree::u64_0 | crate::src::headers::sqliteInt_h::SQLITE_CorruptRdOnly)
                                != 0 as crate::src::ext::rtree::rtree::u64_0
                        {
                            if (*db).flags & crate::src::headers::sqliteInt_h::SQLITE_QueryOnly as crate::src::ext::rtree::rtree::u64_0 != 0 {
                                rc = crate::src::headers::sqlite3_h::SQLITE_READONLY;
                            } else {
                                rc = crate::src::headers::sqlite3_h::SQLITE_CORRUPT;
                            }
                            current_block = 9863799879598557851;
                            break;
                        } else {
                            let __pOp_ref = unsafe { &*pOp };
                            pDb = (*db).aDb.offset(__pOp_ref.p1 as isize) as *mut crate::src::headers::sqliteInt_h::Db;
                            pBt = (*pDb).pBt;
                            if !pBt.is_null() {
                                rc = crate::src::src::btree::sqlite3BtreeBeginTrans(pBt, __pOp_ref.p2, &raw mut iMeta);
                                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                    if !(rc & 0xff as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_BUSY) {
                                        current_block = 9863799879598557851;
                                        break;
                                    }
                                    __p_ref.pc = pOp.offset_from(aOp) as ::core::ffi::c_long
                                        as ::core::ffi::c_int;
                                    __p_ref.rc = rc;
                                    current_block = 10408586827809538586;
                                    break;
                                } else if __p_ref.usesStmtJournal() as ::core::ffi::c_int != 0
                                    && __pOp_ref.p2 != 0
                                    && ((*db).autoCommit as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int
                                        || (*db).nVdbeRead > 1 as ::core::ffi::c_int)
                                {
                                    if __p_ref.iStatement == 0 as ::core::ffi::c_int {
                                        let __db_ref = unsafe { &mut *db };
                                        __db_ref.nStatement += 1;
                                        __p_ref.iStatement = __db_ref.nSavepoint + __db_ref.nStatement;
                                    }
                                    rc = crate::src::src::vtab::sqlite3VtabSavepoint(
                                        
                                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                        crate::src::headers::sqliteInt_h::SAVEPOINT_BEGIN,
                                        __p_ref.iStatement - 1 as ::core::ffi::c_int,
                                    );
                                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                        rc = crate::src::src::btree::sqlite3BtreeBeginStmt(pBt, __p_ref.iStatement);
                                    }
                                    __p_ref.nStmtDefCons = (*db).nDeferredCons;
                                    __p_ref.nStmtDefImmCons = (*db).nDeferredImmCons;
                                }
                            }
                            if rc == crate::src::headers::sqlite3_h::SQLITE_OK
                                && __pOp_ref.p5 as ::core::ffi::c_int != 0
                                && (iMeta != __pOp_ref.p3
                                    || (*(*pDb).pSchema).iGeneration != __pOp_ref.p4.i)
                            {
                                crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, __p_ref.zErrMsg as *mut ::core::ffi::c_void);
                                __p_ref.zErrMsg = crate::src::src::malloc::sqlite3DbStrDup(
                                    
                                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                    b"database schema has changed\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                if (*(*(*db).aDb.offset(__pOp_ref.p1 as isize)).pSchema).schema_cookie
                                    != iMeta
                                {
                                    crate::src::src::build::sqlite3ResetOneSchema(db as *mut crate::src::headers::sqliteInt_h::sqlite3, __pOp_ref.p1);
                                }
                                __p_ref.set_expired(1 as crate::src::headers::sqliteInt_h::bft as crate::src::headers::sqliteInt_h::bft);
                                rc = crate::src::headers::sqlite3_h::SQLITE_SCHEMA;
                                __p_ref.set_changeCntOn(0 as crate::src::headers::sqliteInt_h::bft as crate::src::headers::sqliteInt_h::bft);
                            }
                            if rc != 0 {
                                current_block = 9863799879598557851;
                                break;
                            }
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_ReadCookie_1 =>  {
                        let mut iMeta_0: ::core::ffi::c_int = 0;
                        let mut iDb: ::core::ffi::c_int = 0;
                        let mut iCookie: ::core::ffi::c_int = 0;
                        iDb = (*pOp).p1;
                        iCookie = (*pOp).p3;
                        crate::src::src::btree::sqlite3BtreeGetMeta(
                            (*(*db).aDb.offset(iDb as isize)).pBt,
                            iCookie,
                            &raw mut iMeta_0 as *mut crate::src::ext::rtree::rtree::u32_0,
                        );
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        (*pOut).u.i = iMeta_0 as crate::src::ext::rtree::rtree::i64_0;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_SetCookie_1 =>  {
                        let mut pDb_0: *mut crate::src::headers::sqliteInt_h::Db = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Db>();
                        let __pOp_ref = unsafe { &mut *pOp };
                        pDb_0 = (*db).aDb.offset(__pOp_ref.p1 as isize) as *mut crate::src::headers::sqliteInt_h::Db;
                        rc = crate::src::src::btree::sqlite3BtreeUpdateMeta((*pDb_0).pBt, __pOp_ref.p2, __pOp_ref.p3 as crate::src::ext::rtree::rtree::u32_0);
                        if __pOp_ref.p2 == crate::src::src::btree::BTREE_SCHEMA_VERSION {
                            *(&raw mut (*(*pDb_0).pSchema).schema_cookie as *mut crate::src::ext::rtree::rtree::u32_0) =
                                (*(&raw mut __pOp_ref.p3 as *mut crate::src::ext::rtree::rtree::u32_0))
                                    .wrapping_sub(__pOp_ref.p5 as crate::src::ext::rtree::rtree::u32_0);
                            (*db).mDbFlags |= crate::src::headers::sqliteInt_h::DBFLAG_SchemaChange as crate::src::ext::rtree::rtree::u32_0;
                            crate::src::src::fkey::sqlite3FkClearTriggerCache(db as *mut crate::src::headers::sqliteInt_h::sqlite3, __pOp_ref.p1);
                        } else if __pOp_ref.p2 == crate::src::src::btree::BTREE_FILE_FORMAT {
                            (*(*pDb_0).pSchema).file_format = __pOp_ref.p3 as crate::src::ext::rtree::rtree::u8_0;
                        }
                        if __pOp_ref.p1 == 1 as ::core::ffi::c_int {
                            crate::src::src::vdbeaux::sqlite3ExpirePreparedStatements(db as *mut crate::src::headers::sqliteInt_h::sqlite3, 0 as ::core::ffi::c_int);
                            __p_ref.set_expired(0 as crate::src::headers::sqliteInt_h::bft as crate::src::headers::sqliteInt_h::bft);
                        }
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_ReopenIdx =>  {
                        nField_0 = 0;
                        pKeyInfo_0 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::KeyInfo>();
                        p2_2 = 0;
                        iDb_0 = 0;
                        wrFlag = 0;
                        pX = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::Btree>();
                        pCur = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        pDb_1 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Db>();
                        pCur = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        if !pCur.is_null() && (*pCur).pgnoRoot == (*pOp).p2 as crate::src::ext::rtree::rtree::u32_0 {
                            crate::src::src::btree::sqlite3BtreeClearCursor((*pCur).uc.pCursor);
                            current_block = 2376416726268577300;
                        } else {
                            current_block = 6098721512207810182;
                        }
                    }
    crate::src::headers::opcodes_h::OP_OpenRead_1 | crate::src::headers::opcodes_h::OP_OpenWrite =>  {
                        current_block = 6098721512207810182;
                    }
    crate::src::headers::opcodes_h::OP_OpenDup_1 =>  {
                        let mut pOrig: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut pCx: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        pOrig = *__p_ref.apCsr.offset((*pOp).p2 as isize);
                        pCx = allocateCursor(
                            p,
                            (*pOp).p1,
                            (*pOrig).nField as ::core::ffi::c_int,
                            crate::src::headers::vdbeInt_h::CURTYPE_BTREE as crate::src::ext::rtree::rtree::u8_0,
                        );
                        if pCx.is_null() {
                            current_block = 2471432538116610086;
                            break;
                        }
                        (*pCx).nullRow = 1 as crate::src::ext::rtree::rtree::u8_0;
                        (*pCx).set_isEphemeral(1 as crate::src::headers::vdbeInt_h::Bool as crate::src::headers::vdbeInt_h::Bool);
                        (*pCx).pKeyInfo = (*pOrig).pKeyInfo;
                        (*pCx).isTable = (*pOrig).isTable;
                        (*pCx).pgnoRoot = (*pOrig).pgnoRoot;
                        (*pCx).set_isOrdered((*pOrig).isOrdered() as crate::src::headers::vdbeInt_h::Bool);
                        (*pCx).ub.pBtx = (*pOrig).ub.pBtx;
                        (*pCx).set_noReuse(1 as crate::src::headers::vdbeInt_h::Bool as crate::src::headers::vdbeInt_h::Bool);
                        (*pOrig).set_noReuse(1 as crate::src::headers::vdbeInt_h::Bool as crate::src::headers::vdbeInt_h::Bool);
                        rc = crate::src::src::btree::sqlite3BtreeCursor(
                            (*pCx).ub.pBtx,
                            (*pCx).pgnoRoot,
                            crate::src::src::btree::BTREE_WRCSR,
                            
                            (*pCx).pKeyInfo as *mut crate::src::headers::sqliteInt_h::KeyInfo as *mut crate::src::headers::sqliteInt_h::KeyInfo,
                            (*pCx).uc.pCursor,
                        );
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_OpenAutoindex =>  {
                        current_block = 4855994676461051050;
                    }
    crate::src::headers::opcodes_h::OP_OpenEphemeral_1 =>  {
                        current_block = 4855994676461051050;
                    }
    crate::src::headers::opcodes_h::OP_SorterOpen_1 =>  {
                        let mut pCx_1: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let __pOp_ref = unsafe { &*pOp };
                        pCx_1 = allocateCursor(p, __pOp_ref.p1, __pOp_ref.p2, crate::src::headers::vdbeInt_h::CURTYPE_SORTER as crate::src::ext::rtree::rtree::u8_0);
                        if pCx_1.is_null() {
                            current_block = 2471432538116610086;
                            break;
                        }
                        (*pCx_1).pKeyInfo = __pOp_ref.p4.pKeyInfo;
                        rc = crate::src::src::vdbesort::sqlite3VdbeSorterInit(db as *mut crate::src::headers::sqliteInt_h::sqlite3, __pOp_ref.p3,  pCx_1 as *mut crate::src::headers::vdbeInt_h::VdbeCursor);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_SequenceTest_1 =>  {
                        let mut pC_2: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        pC_2 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        let fresh10 = (*pC_2).seqCount;
                        (*pC_2).seqCount += 1;
                        if fresh10 == 0 as crate::src::ext::rtree::rtree::i64_0 {
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_OpenPseudo_1 =>  {
                        let mut pCx_2: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let __pOp_ref = unsafe { &*pOp };
                        pCx_2 = allocateCursor(p, __pOp_ref.p1, __pOp_ref.p3, crate::src::headers::vdbeInt_h::CURTYPE_PSEUDO as crate::src::ext::rtree::rtree::u8_0);
                        if pCx_2.is_null() {
                            current_block = 2471432538116610086;
                            break;
                        }
                        (*pCx_2).nullRow = 1 as crate::src::ext::rtree::rtree::u8_0;
                        (*pCx_2).seekResult = __pOp_ref.p2;
                        (*pCx_2).isTable = 1 as crate::src::ext::rtree::rtree::u8_0;
                        (*pCx_2).uc.pCursor = crate::src::src::btree::sqlite3BtreeFakeValidCursor();
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Close_1 =>  {
                        crate::src::src::vdbeaux::sqlite3VdbeFreeCursor(p as *mut crate::src::headers::vdbeInt_h::Vdbe,  *__p_ref.apCsr.offset((*pOp).p1 as isize) as *mut crate::src::headers::vdbeInt_h::VdbeCursor);
                        let ref mut fresh11 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        *fresh11 = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_SeekLT =>  {
                        current_block = 2622337694718008658;
                    }
    crate::src::headers::opcodes_h::OP_SeekLE =>  {
                        current_block = 2622337694718008658;
                    }
    crate::src::headers::opcodes_h::OP_SeekGE =>  {
                        current_block = 17378708304186809879;
                    }
    crate::src::headers::opcodes_h::OP_SeekGT =>  {
                        current_block = 6051598589340159089;
                    }
    crate::src::headers::opcodes_h::OP_SeekScan =>  {
                        let mut pC_4: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut res_1: ::core::ffi::c_int = 0;
                        let mut nStep: ::core::ffi::c_int = 0;
                        let mut r_0: crate::src::headers::sqliteInt_h::UnpackedRecord = unsafe { ::core::mem::zeroed() };
                        pC_4 = *(*p)
                            .apCsr
                            .offset((*pOp.offset(1 as isize)).p1 as isize);
                        if crate::src::src::btree::sqlite3BtreeCursorIsValidNN((*pC_4).uc.pCursor) == 0 {
                            current_block = 5783071609795492627;
                        } else {
                            nStep = (*pOp).p1;
                            r_0.pKeyInfo = (*pC_4).pKeyInfo;
                            r_0.nField =
                                (*pOp.offset(1 as isize)).p4.i as crate::src::fts5::u16_0;
                            r_0.default_rc = 0 as crate::src::headers::sqliteInt_h::i8_0;
                            r_0.aMem = aMem
                                .offset((*pOp.offset(1 as isize)).p3 as isize)
                                as *mut crate::src::src::vdbe::Mem;
                            res_1 = 0 as ::core::ffi::c_int;
                            loop {
                                rc = crate::src::src::vdbeaux::sqlite3VdbeIdxKeyCompare(
                                    
                                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                    
                                    pC_4 as *mut crate::src::headers::vdbeInt_h::VdbeCursor,
                                    
                                    &raw mut r_0 as *mut _ as *mut crate::src::headers::sqliteInt_h::UnpackedRecord,
                                    &raw mut res_1,
                                );
                                if rc != 0 {
                                    current_block = 9863799879598557851;
                                    break 's_109;
                                }
                                if !(res_1 > 0 as ::core::ffi::c_int
                                    && (*pOp).p5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                                {
                                    if res_1 >= 0 as ::core::ffi::c_int {
                                        current_block = 9512719473022792396;
                                        break;
                                    } else {
                                        if nStep <= 0 as ::core::ffi::c_int {
                                            current_block = 5783071609795492627;
                                            break;
                                        }
                                        nStep -= 1;
                                        (*pC_4).cacheStatus = crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                                        rc = crate::src::src::btree::sqlite3BtreeNext(
                                            (*pC_4).uc.pCursor,
                                            0 as ::core::ffi::c_int,
                                        );
                                        if !(rc != 0) {
                                            continue;
                                        }
                                        if !(rc == crate::src::headers::sqlite3_h::SQLITE_DONE) {
                                            current_block = 9863799879598557851;
                                            break 's_109;
                                        }
                                        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                                    }
                                }
                                pOp = pOp.offset(1);
                                current_block = 9512719473022792396;
                                break;
                            }
                        }
                    }
    crate::src::headers::opcodes_h::OP_SeekHit =>  {
                        let mut pC_5: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let __pOp_ref = unsafe { &*pOp };
                        pC_5 = *__p_ref.apCsr.offset(__pOp_ref.p1 as isize);
                        if ((*pC_5).seekHit as ::core::ffi::c_int) < __pOp_ref.p2 {
                            (*pC_5).seekHit = __pOp_ref.p2 as crate::src::fts5::u16_0;
                        } else if (*pC_5).seekHit as ::core::ffi::c_int > __pOp_ref.p3 {
                            (*pC_5).seekHit = __pOp_ref.p3 as crate::src::fts5::u16_0;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_IfNotOpen =>  {
                        let mut pCur_0: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        pCur_0 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        if pCur_0.is_null() || (*pCur_0).nullRow as ::core::ffi::c_int != 0 {
                            current_block = 16391045501586817625;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_IfNoHope =>  {
                        let mut pC_6: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        pC_6 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        if (*pC_6).seekHit as ::core::ffi::c_int >= (*pOp).p4.i {
                            current_block = 5783071609795492627;
                        } else {
                            current_block = 8122660736604414621;
                        }
                    }
    crate::src::headers::opcodes_h::OP_NoConflict =>  {
                        current_block = 8122660736604414621;
                    }
    crate::src::headers::opcodes_h::OP_NotFound_1 =>  {
                        current_block = 10733086697120658930;
                    }
    crate::src::headers::opcodes_h::OP_Found =>  {
                        current_block = 18125716024132132232;
                    }
    crate::src::headers::opcodes_h::OP_SeekRowid_1 =>  {
                        pC_8 = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        pCrsr_1 = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>();
                        res_2 = 0;
                        iKey_0 = 0;
                        pIn3 = aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pIn3).flags as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_IntReal)
                            == 0 as ::core::ffi::c_int
                        {
                            let mut x: crate::src::src::vdbe::Mem = *pIn3.offset(0 as isize);
                            applyAffinity(
                                &raw mut x,
                                crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC as ::core::ffi::c_char,
                                encoding,
                            );
                            if x.flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Int == 0 as ::core::ffi::c_int {
                                current_block = 9512719473022792396;
                            } else {
                                iKey_0 = x.u.i as crate::src::ext::rtree::rtree::u64_0;
                                current_block = 8490332201604051012;
                            }
                        } else {
                            current_block = 15776502115119204583;
                        }
                    }
    crate::src::headers::opcodes_h::OP_NotExists_1 =>  {
                        current_block = 15776502115119204583;
                    }
    crate::src::headers::opcodes_h::OP_Sequence_1 =>  {
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        let ref mut fresh12 = (**__p_ref.apCsr.offset((*pOp).p1 as isize)).seqCount;
                        let fresh13 = *fresh12;
                        *fresh12 += 1;
                        (*pOut).u.i = fresh13;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_NewRowid_1 =>  {
                        let mut v_0: crate::src::ext::rtree::rtree::i64_0 = 0;
                        let mut pC_9: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut res_3: ::core::ffi::c_int = 0;
                        let mut cnt_0: ::core::ffi::c_int = 0;
                        let mut pMem: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let mut pFrame_0: *mut crate::src::headers::vdbeInt_h::VdbeFrame = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeFrame>();
                        v_0 = 0 as crate::src::ext::rtree::rtree::i64_0;
                        res_3 = 0 as ::core::ffi::c_int;
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        pC_9 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        if (*pC_9).useRandomRowid() == 0 {
                            rc = crate::src::src::btree::sqlite3BtreeLast((*pC_9).uc.pCursor, &raw mut res_3);
                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                current_block = 9863799879598557851;
                                break;
                            }
                            if res_3 != 0 {
                                v_0 = 1 as crate::src::ext::rtree::rtree::i64_0;
                            } else {
                                v_0 = crate::src::src::btree::sqlite3BtreeIntegerKey((*pC_9).uc.pCursor);
                                if v_0 >= MAX_ROWID {
                                    (*pC_9).set_useRandomRowid(1 as crate::src::headers::vdbeInt_h::Bool as crate::src::headers::vdbeInt_h::Bool);
                                } else {
                                    v_0 += 1;
                                }
                            }
                        }
                        if (*pOp).p3 != 0 {
                            if !__p_ref.pFrame.is_null() {
                                pFrame_0 = __p_ref.pFrame;
                                while !(*pFrame_0).pParent.is_null() {
                                    pFrame_0 = (*pFrame_0).pParent;
                                }
                                pMem = (*pFrame_0).aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem;
                            } else {
                                pMem = aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem;
                            }
                            crate::src::src::vdbemem::sqlite3VdbeMemIntegerify(pMem as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                            if (*pMem).u.i == MAX_ROWID
                                || (*pC_9).useRandomRowid() as ::core::ffi::c_int != 0
                            {
                                rc = crate::src::headers::sqlite3_h::SQLITE_FULL;
                                current_block = 9863799879598557851;
                                break;
                            } else {
                                if v_0 < (*pMem).u.i + 1 as crate::src::ext::rtree::rtree::i64_0 {
                                    v_0 = (*pMem).u.i + 1 as crate::src::ext::rtree::rtree::i64_0;
                                }
                                (*pMem).u.i = v_0;
                            }
                        }
                        if (*pC_9).useRandomRowid() != 0 {
                            cnt_0 = 0 as ::core::ffi::c_int;
                            loop {
                                crate::src::src::random::sqlite3_randomness(
                                    ::core::mem::size_of::<crate::src::ext::rtree::rtree::i64_0>() as ::core::ffi::c_int,
                                    &raw mut v_0 as *mut ::core::ffi::c_void,
                                );
                                v_0 &= MAX_ROWID >> 1 as ::core::ffi::c_int;
                                v_0 += 1;
                                rc = crate::src::src::btree::sqlite3BtreeTableMoveto(
                                    (*pC_9).uc.pCursor,
                                    v_0 as crate::src::ext::rtree::rtree::u64_0 as crate::src::ext::rtree::rtree::i64_0,
                                    0 as ::core::ffi::c_int,
                                    &raw mut res_3,
                                );
                                if !(rc == crate::src::headers::sqlite3_h::SQLITE_OK && res_3 == 0 as ::core::ffi::c_int && {
                                    cnt_0 += 1;
                                    cnt_0 < 100 as ::core::ffi::c_int
                                }) {
                                    break;
                                }
                            }
                            if rc != 0 {
                                current_block = 9863799879598557851;
                                break;
                            }
                            if res_3 == 0 as ::core::ffi::c_int {
                                rc = crate::src::headers::sqlite3_h::SQLITE_FULL;
                                current_block = 9863799879598557851;
                                break;
                            }
                        }
                        (*pC_9).deferredMoveto = 0 as crate::src::ext::rtree::rtree::u8_0;
                        (*pC_9).cacheStatus = crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                        (*pOut).u.i = v_0;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Insert_1 =>  {
                        let mut pData: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let mut pKey: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let mut pC_10: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut seekResult: ::core::ffi::c_int = 0;
                        let mut zDb: *const ::core::ffi::c_char =
                            ::core::ptr::null::<::core::ffi::c_char>();
                        let mut pTab_0: *mut crate::src::headers::sqliteInt_h::Table = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
                        let mut x_0: crate::src::src::btree::BtreePayload = unsafe { ::core::mem::zeroed() };
                        let __pOp_ref = unsafe { &*pOp };
                        pData = aMem.offset(__pOp_ref.p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        pC_10 = *__p_ref.apCsr.offset(__pOp_ref.p1 as isize);
                        pKey = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        x_0.nKey = (*pKey).u.i as crate::src::headers::sqlite3_h::sqlite3_int64;
                        if __pOp_ref.p4type as ::core::ffi::c_int == crate::src::src::vdbe::P4_TABLE
                            && ((*db).xPreUpdateCallback.is_some()
                                || (*db).xUpdateCallback.is_some())
                        {
                            zDb = (*(*db).aDb.offset((*pC_10).iDb as isize)).zDbSName;
                            pTab_0 = __pOp_ref.p4.pTab;
                        } else {
                            pTab_0 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
                            zDb = ::core::ptr::null::<::core::ffi::c_char>();
                        }
                        if !pTab_0.is_null() {
                            if (*db).xPreUpdateCallback.is_some()
                                && __pOp_ref.p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::OPFLAG_ISUPDATE == 0
                            {
                                crate::src::src::vdbeaux::sqlite3VdbePreUpdateHook(
                                    
                                    p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                    
                                    pC_10 as *mut crate::src::headers::vdbeInt_h::VdbeCursor,
                                    crate::src::headers::sqlite3_h::SQLITE_INSERT,
                                    zDb,
                                    
                                    pTab_0 as *mut crate::src::headers::sqliteInt_h::Table,
                                    x_0.nKey as crate::src::ext::rtree::rtree::i64_0,
                                    __pOp_ref.p2,
                                    -(1 as ::core::ffi::c_int),
                                );
                            }
                            if (*db).xUpdateCallback.is_none() || (*pTab_0).aCol.is_null() {
                                pTab_0 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
                            }
                        }
                        if __pOp_ref.p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::OPFLAG_ISNOOP != 0 {
                            current_block = 5783071609795492627;
                        } else {
                            if __pOp_ref.p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::OPFLAG_NCHANGE != 0 {
                                __p_ref.nChange += 1;
                                if __pOp_ref.p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::OPFLAG_LASTROWID != 0 {
                                    (*db).lastRowid = x_0.nKey as crate::src::ext::rtree::rtree::i64_0;
                                }
                            }
                            let __pData_ref = unsafe { &*pData };
                            x_0.pData = __pData_ref.z as *const ::core::ffi::c_void;
                            x_0.nData = __pData_ref.n;
                            let __pC_10_ref = unsafe { &mut *pC_10 };
                            seekResult =
                                if __pOp_ref.p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::OPFLAG_USESEEKRESULT != 0 {
                                    __pC_10_ref.seekResult
                                } else {
                                    0 as ::core::ffi::c_int
                                };
                            if __pData_ref.flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Zero != 0 {
                                x_0.nZero = __pData_ref.u.nZero;
                            } else {
                                x_0.nZero = 0 as ::core::ffi::c_int;
                            }
                            x_0.pKey = ::core::ptr::null::<::core::ffi::c_void>();
                            rc = crate::src::src::btree::sqlite3BtreeInsert(
                                __pC_10_ref.uc.pCursor,
                                
                                &raw mut x_0 as *mut _ as *const crate::src::src::btree::BtreePayload,
                                __pOp_ref.p5 as ::core::ffi::c_int
                                    & (crate::src::headers::sqliteInt_h::OPFLAG_APPEND | crate::src::headers::sqliteInt_h::OPFLAG_SAVEPOSITION | crate::src::headers::sqliteInt_h::OPFLAG_PREFORMAT),
                                seekResult,
                            );
                            __pC_10_ref.deferredMoveto = 0 as crate::src::ext::rtree::rtree::u8_0;
                            __pC_10_ref.cacheStatus = crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                            colCacheCtr = colCacheCtr.wrapping_add(1);
                            if rc != 0 {
                                current_block = 9863799879598557851;
                                break;
                            }
                            if !pTab_0.is_null() {
                                (*db).xUpdateCallback.expect("non-null function pointer")(
                                    (*db).pUpdateArg,
                                    if __pOp_ref.p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::OPFLAG_ISUPDATE != 0 {
                                        crate::src::headers::sqlite3_h::SQLITE_UPDATE
                                    } else {
                                        crate::src::headers::sqlite3_h::SQLITE_INSERT
                                    },
                                    zDb,
                                    (*pTab_0).zName,
                                    x_0.nKey as crate::src::headers::sqlite3_h::sqlite_int64,
                                );
                            }
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_RowCell_1 =>  {
                        let mut pDest_0: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut pSrc: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut iKey_1: crate::src::ext::rtree::rtree::i64_0 = 0;
                        let __pOp_ref = unsafe { &*pOp };
                        pDest_0 = *__p_ref.apCsr.offset(__pOp_ref.p1 as isize);
                        pSrc = *__p_ref.apCsr.offset(__pOp_ref.p2 as isize);
                        iKey_1 = if __pOp_ref.p3 != 0 {
                            (*aMem.offset(__pOp_ref.p3 as isize)).u.i
                        } else {
                            0 as crate::src::ext::rtree::rtree::i64_0
                        };
                        rc = crate::src::src::btree::sqlite3BtreeTransferRow(
                            (*pDest_0).uc.pCursor,
                            (*pSrc).uc.pCursor,
                            iKey_1,
                        );
                        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                            current_block = 9863799879598557851;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Delete_1 =>  {
                        let mut pC_11: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut zDb_0: *const ::core::ffi::c_char =
                            ::core::ptr::null::<::core::ffi::c_char>();
                        let mut pTab_1: *mut crate::src::headers::sqliteInt_h::Table = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
                        let mut opflags: ::core::ffi::c_int = 0;
                        let __pOp_ref = unsafe { &*pOp };
                        opflags = __pOp_ref.p2;
                        pC_11 = *__p_ref.apCsr.offset(__pOp_ref.p1 as isize);
                        let __db_ref = unsafe { &mut *db };
                        if __pOp_ref.p4type as ::core::ffi::c_int == crate::src::src::vdbe::P4_TABLE
                            && (__db_ref.xPreUpdateCallback.is_some()
                                || __db_ref.xUpdateCallback.is_some())
                        {
                            zDb_0 = (*__db_ref.aDb.offset((*pC_11).iDb as isize)).zDbSName;
                            pTab_1 = __pOp_ref.p4.pTab;
                            if __pOp_ref.p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::OPFLAG_SAVEPOSITION
                                != 0 as ::core::ffi::c_int
                                && (*pC_11).isTable as ::core::ffi::c_int != 0
                            {
                                (*pC_11).movetoTarget = crate::src::src::btree::sqlite3BtreeIntegerKey((*pC_11).uc.pCursor);
                            }
                        } else {
                            zDb_0 = ::core::ptr::null::<::core::ffi::c_char>();
                            pTab_1 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
                        }
                        if __db_ref.xPreUpdateCallback.is_some() && !pTab_1.is_null() {
                            crate::src::src::vdbeaux::sqlite3VdbePreUpdateHook(
                                
                                p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                
                                pC_11 as *mut crate::src::headers::vdbeInt_h::VdbeCursor,
                                if opflags & crate::src::headers::sqliteInt_h::OPFLAG_ISUPDATE != 0 {
                                    crate::src::headers::sqlite3_h::SQLITE_UPDATE
                                } else {
                                    crate::src::headers::sqlite3_h::SQLITE_DELETE
                                },
                                zDb_0,
                                
                                pTab_1 as *mut crate::src::headers::sqliteInt_h::Table,
                                (*pC_11).movetoTarget,
                                __pOp_ref.p3,
                                -(1 as ::core::ffi::c_int),
                            );
                        }
                        if opflags & crate::src::headers::sqliteInt_h::OPFLAG_ISNOOP != 0 {
                            current_block = 5783071609795492627;
                        } else {
                            let __pC_11_ref = unsafe { &mut *pC_11 };
                            rc = crate::src::src::btree::sqlite3BtreeDelete(__pC_11_ref.uc.pCursor, __pOp_ref.p5 as crate::src::ext::rtree::rtree::u8_0);
                            __pC_11_ref.cacheStatus = crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                            colCacheCtr = colCacheCtr.wrapping_add(1);
                            __pC_11_ref.seekResult = 0 as ::core::ffi::c_int;
                            if rc != 0 {
                                current_block = 9863799879598557851;
                                break;
                            }
                            if opflags & crate::src::headers::sqliteInt_h::OPFLAG_NCHANGE != 0 {
                                __p_ref.nChange += 1;
                                if __db_ref.xUpdateCallback.is_some()
                                    && !pTab_1.is_null()
                                    && (*pTab_1).tabFlags & crate::src::headers::sqliteInt_h::TF_WithoutRowid as crate::src::ext::rtree::rtree::u32_0 == 0 as crate::src::ext::rtree::rtree::u32_0
                                {
                                    __db_ref.xUpdateCallback.expect("non-null function pointer")(
                                        __db_ref.pUpdateArg,
                                        crate::src::headers::sqlite3_h::SQLITE_DELETE,
                                        zDb_0,
                                        (*pTab_1).zName,
                                        __pC_11_ref.movetoTarget as crate::src::headers::sqlite3_h::sqlite_int64,
                                    );
                                }
                            }
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_ResetCount_1 =>  {
                        crate::src::src::vdbeaux::sqlite3VdbeSetChanges(db as *mut crate::src::headers::sqliteInt_h::sqlite3, __p_ref.nChange);
                        __p_ref.nChange = 0 as crate::src::ext::rtree::rtree::i64_0;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_SorterCompare_1 =>  {
                        let mut pC_12: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut res_4: ::core::ffi::c_int = 0;
                        let mut nKeyCol: ::core::ffi::c_int = 0;
                        let __pOp_ref = unsafe { &*pOp };
                        pC_12 = *__p_ref.apCsr.offset(__pOp_ref.p1 as isize);
                        pIn3 = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        nKeyCol = __pOp_ref.p4.i;
                        res_4 = 0 as ::core::ffi::c_int;
                        rc = crate::src::src::vdbesort::sqlite3VdbeSorterCompare(pC_12 as *const crate::src::headers::vdbeInt_h::VdbeCursor,  pIn3 as *mut crate::src::headers::vdbeInt_h::sqlite3_value, nKeyCol, &raw mut res_4);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        if res_4 != 0 {
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_SorterData_1 =>  {
                        let mut pC_13: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let __pOp_ref = unsafe { &mut *pOp };
                        pOut = aMem.offset(__pOp_ref.p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        pC_13 = *__p_ref.apCsr.offset(__pOp_ref.p1 as isize);
                        rc = crate::src::src::vdbesort::sqlite3VdbeSorterRowkey(pC_13 as *const crate::src::headers::vdbeInt_h::VdbeCursor,  pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        (**__p_ref.apCsr.offset(__pOp_ref.p3 as isize)).cacheStatus =
                            crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_RowData_1 =>  {
                        let mut pC_14: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut pCrsr_2: *mut crate::src::headers::btreeInt_h::BtCursor = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>();
                        let mut n_2: crate::src::ext::rtree::rtree::u32_0 = 0;
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        pC_14 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        pCrsr_2 = (*pC_14).uc.pCursor;
                        n_2 = crate::src::src::btree::sqlite3BtreePayloadSize(pCrsr_2);
                        if n_2 > (*db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_LENGTH as usize] as crate::src::ext::rtree::rtree::u32_0 {
                            current_block = 7113757447962609068;
                            break;
                        }
                        rc = crate::src::src::vdbemem::sqlite3VdbeMemFromBtreeZeroOffset(pCrsr_2, n_2,  pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        if (*pOp).p3 == 0 {
                            if (*pOut).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Ephem
                                != 0 as ::core::ffi::c_int
                                && crate::src::src::vdbemem::sqlite3VdbeMemMakeWriteable(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value) != 0
                            {
                                current_block = 2471432538116610086;
                                break;
                            }
                        }
                        updateMaxBlobsize(pOut);
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Rowid_1 =>  {
                        let mut pC_15: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut v_1: crate::src::ext::rtree::rtree::i64_0 = 0;
                        let mut pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vtab>();
                        let mut pModule: *const crate::src::headers::sqlite3_h::sqlite3_module =
                            ::core::ptr::null::<crate::src::headers::sqlite3_h::sqlite3_module>();
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        pC_15 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        if (*pC_15).nullRow != 0 {
                            (*pOut).flags = crate::src::headers::vdbeInt_h::MEM_Null as crate::src::fts5::u16_0;
                        } else {
                            if (*pC_15).deferredMoveto != 0 {
                                v_1 = (*pC_15).movetoTarget;
                                current_block = 6117620705961481515;
                            } else if (*pC_15).eCurType as ::core::ffi::c_int == crate::src::headers::vdbeInt_h::CURTYPE_VTAB {
                                pVtab = (*(*pC_15).uc.pVCur).pVtab;
                                pModule = (*pVtab).pModule;
                                rc = (*pModule).xRowid.expect("non-null function pointer")(
                                    (*pC_15).uc.pVCur,
                                    &raw mut v_1,
                                );
                                crate::src::src::vdbeaux::sqlite3VtabImportErrmsg(p as *mut crate::src::headers::vdbeInt_h::Vdbe,  pVtab as *mut crate::src::headers::sqlite3_h::sqlite3_vtab);
                                if rc != 0 {
                                    current_block = 9863799879598557851;
                                    break;
                                }
                                current_block = 6117620705961481515;
                            } else {
                                rc = crate::src::src::vdbeaux::sqlite3VdbeCursorRestore(pC_15 as *mut crate::src::headers::vdbeInt_h::VdbeCursor);
                                if rc != 0 {
                                    current_block = 9863799879598557851;
                                    break;
                                }
                                if (*pC_15).nullRow != 0 {
                                    (*pOut).flags = crate::src::headers::vdbeInt_h::MEM_Null as crate::src::fts5::u16_0;
                                    current_block = 5783071609795492627;
                                } else {
                                    v_1 = crate::src::src::btree::sqlite3BtreeIntegerKey((*pC_15).uc.pCursor);
                                    current_block = 6117620705961481515;
                                }
                            }
                            match current_block {
                                5783071609795492627 => {}
                                _ => {
                                    (*pOut).u.i = v_1;
                                }
                            }
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_NullRow_1 =>  {
                        let mut pC_16: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        pC_16 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        if pC_16.is_null() {
                            pC_16 = allocateCursor(
                                p,
                                (*pOp).p1,
                                1 as ::core::ffi::c_int,
                                crate::src::headers::vdbeInt_h::CURTYPE_PSEUDO as crate::src::ext::rtree::rtree::u8_0,
                            );
                            if pC_16.is_null() {
                                current_block = 2471432538116610086;
                                break;
                            }
                            (*pC_16).seekResult = 0 as ::core::ffi::c_int;
                            (*pC_16).isTable = 1 as crate::src::ext::rtree::rtree::u8_0;
                            (*pC_16).set_noReuse(1 as crate::src::headers::vdbeInt_h::Bool as crate::src::headers::vdbeInt_h::Bool);
                            (*pC_16).uc.pCursor = crate::src::src::btree::sqlite3BtreeFakeValidCursor();
                        }
                        (*pC_16).nullRow = 1 as crate::src::ext::rtree::rtree::u8_0;
                        (*pC_16).cacheStatus = crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                        if (*pC_16).eCurType as ::core::ffi::c_int == crate::src::headers::vdbeInt_h::CURTYPE_BTREE {
                            crate::src::src::btree::sqlite3BtreeClearCursor((*pC_16).uc.pCursor);
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_SeekEnd =>  {
                        current_block = 3859792399008177139;
                    }
    crate::src::headers::opcodes_h::OP_Last_1 =>  {
                        current_block = 3859792399008177139;
                    }
    crate::src::headers::opcodes_h::OP_IfSizeBetween_1 =>  {
                        let mut pC_18: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut pCrsr_4: *mut crate::src::headers::btreeInt_h::BtCursor = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>();
                        let mut res_6: ::core::ffi::c_int = 0;
                        let mut sz: crate::src::ext::rtree::rtree::i64_0 = 0;
                        let __pOp_ref = unsafe { &*pOp };
                        pC_18 = *__p_ref.apCsr.offset(__pOp_ref.p1 as isize);
                        pCrsr_4 = (*pC_18).uc.pCursor;
                        rc = crate::src::src::btree::sqlite3BtreeFirst(pCrsr_4, &raw mut res_6);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        if res_6 != 0 as ::core::ffi::c_int {
                            sz = -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0;
                        } else {
                            sz = crate::src::src::btree::sqlite3BtreeRowCountEst(pCrsr_4);
                            sz = crate::src::src::util::sqlite3LogEst(sz as crate::src::ext::rtree::rtree::u64_0) as crate::src::ext::rtree::rtree::i64_0;
                        }
                        res_6 = (sz >= __pOp_ref.p3 as crate::src::ext::rtree::rtree::i64_0 && sz <= __pOp_ref.p4.i as crate::src::ext::rtree::rtree::i64_0)
                            as ::core::ffi::c_int;
                        if res_6 != 0 {
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_SorterSort_1 | crate::src::headers::opcodes_h::OP_Sort_1 =>  {
                        sqlite3_sort_count += 1;
                        sqlite3_search_count -= 1;
                        __p_ref.aCounter[crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_SORT as usize] =
                            __p_ref.aCounter[crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_SORT as usize].wrapping_add(1);
                        current_block = 3082952251283703534;
                    }
    crate::src::headers::opcodes_h::OP_Rewind_1 =>  {
                        current_block = 3082952251283703534;
                    }
    crate::src::headers::opcodes_h::OP_IfEmpty_1 =>  {
                        let mut pC_20: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut pCrsr_6: *mut crate::src::headers::btreeInt_h::BtCursor = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>();
                        let mut res_8: ::core::ffi::c_int = 0;
                        pC_20 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        pCrsr_6 = (*pC_20).uc.pCursor;
                        rc = crate::src::src::btree::sqlite3BtreeIsEmpty(pCrsr_6, &raw mut res_8);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        if res_8 != 0 {
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_SorterNext_1 =>  {
                        pC_21 = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        pC_21 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        rc = crate::src::src::vdbesort::sqlite3VdbeSorterNext(db as *mut crate::src::headers::sqliteInt_h::sqlite3,  pC_21 as *const crate::src::headers::vdbeInt_h::VdbeCursor);
                        current_block = 13527485019907976489;
                    }
    crate::src::headers::opcodes_h::OP_Prev =>  {
                        pC_21 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        rc = crate::src::src::btree::sqlite3BtreePrevious((*pC_21).uc.pCursor, (*pOp).p3);
                        current_block = 13527485019907976489;
                    }
    crate::src::headers::opcodes_h::OP_Next_1 =>  {
                        pC_21 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        rc = crate::src::src::btree::sqlite3BtreeNext((*pC_21).uc.pCursor, (*pOp).p3);
                        current_block = 13527485019907976489;
                    }
    crate::src::headers::opcodes_h::OP_IdxInsert_1 =>  {
                        let mut pC_22: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut x_1: crate::src::src::btree::BtreePayload = unsafe { ::core::mem::zeroed() };
                        let __pOp_ref = unsafe { &*pOp };
                        pC_22 = *__p_ref.apCsr.offset(__pOp_ref.p1 as isize);
                        pIn2 = aMem.offset(__pOp_ref.p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        if __pOp_ref.p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::OPFLAG_NCHANGE != 0 {
                            __p_ref.nChange += 1;
                        }
                        rc = if (*pIn2).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Zero != 0 {
                            crate::src::src::vdbemem::sqlite3VdbeMemExpandBlob(pIn2 as *mut crate::src::headers::vdbeInt_h::sqlite3_value)
                        } else {
                            0 as ::core::ffi::c_int
                        };
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        x_1.nKey = (*pIn2).n as crate::src::headers::sqlite3_h::sqlite3_int64;
                        x_1.pKey = (*pIn2).z as *const ::core::ffi::c_void;
                        x_1.aMem = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::headers::vdbeInt_h::sqlite3_value;
                        x_1.nMem = __pOp_ref.p4.i as crate::src::fts5::u16_0;
                        rc = crate::src::src::btree::sqlite3BtreeInsert(
                            (*pC_22).uc.pCursor,
                            
                            &raw mut x_1 as *mut _ as *const crate::src::src::btree::BtreePayload,
                            __pOp_ref.p5 as ::core::ffi::c_int
                                & (crate::src::headers::sqliteInt_h::OPFLAG_APPEND | crate::src::headers::sqliteInt_h::OPFLAG_SAVEPOSITION | crate::src::headers::sqliteInt_h::OPFLAG_PREFORMAT),
                            if __pOp_ref.p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::OPFLAG_USESEEKRESULT != 0 {
                                (*pC_22).seekResult
                            } else {
                                0 as ::core::ffi::c_int
                            },
                        );
                        (*pC_22).cacheStatus = crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_SorterInsert_1 =>  {
                        let mut pC_23: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        pC_23 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        pIn2 = aMem.offset((*pOp).p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        rc = if (*pIn2).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Zero != 0 {
                            crate::src::src::vdbemem::sqlite3VdbeMemExpandBlob(pIn2 as *mut crate::src::headers::vdbeInt_h::sqlite3_value)
                        } else {
                            0 as ::core::ffi::c_int
                        };
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        rc = crate::src::src::vdbesort::sqlite3VdbeSorterWrite(pC_23 as *const crate::src::headers::vdbeInt_h::VdbeCursor,  pIn2 as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_IdxDelete_1 =>  {
                        let mut pC_24: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut pCrsr_7: *mut crate::src::headers::btreeInt_h::BtCursor = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>();
                        let mut res_9: ::core::ffi::c_int = 0;
                        let mut r_2: crate::src::headers::sqliteInt_h::UnpackedRecord = unsafe { ::core::mem::zeroed() };
                        let __pOp_ref = unsafe { &*pOp };
                        pC_24 = *__p_ref.apCsr.offset(__pOp_ref.p1 as isize);
                        pCrsr_7 = (*pC_24).uc.pCursor;
                        r_2.pKeyInfo = (*pC_24).pKeyInfo;
                        r_2.nField = __pOp_ref.p3 as crate::src::fts5::u16_0;
                        r_2.default_rc = 0 as crate::src::headers::sqliteInt_h::i8_0;
                        r_2.aMem = aMem.offset(__pOp_ref.p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        rc = crate::src::src::btree::sqlite3BtreeIndexMoveto(pCrsr_7,  &raw mut r_2 as *mut _ as *mut crate::src::headers::sqliteInt_h::UnpackedRecord, &raw mut res_9);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        if res_9 == 0 as ::core::ffi::c_int {
                            rc = crate::src::src::btree::sqlite3BtreeDelete(pCrsr_7, crate::src::src::btree::BTREE_AUXDELETE as crate::src::ext::rtree::rtree::u8_0);
                            if rc != 0 {
                                current_block = 9863799879598557851;
                                break;
                            }
                        } else if __pOp_ref.p5 as ::core::ffi::c_int != 0
                            && crate::src::src::build::sqlite3WritableSchema(db as *mut crate::src::headers::sqliteInt_h::sqlite3) == 0
                        {
                            rc = crate::src::src::main::sqlite3ReportError(
                                crate::src::headers::sqlite3_h::SQLITE_CORRUPT_INDEX,
                                6663 as ::core::ffi::c_int,
                                b"index corruption\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                            current_block = 9863799879598557851;
                            break;
                        }
                        (*pC_24).cacheStatus = crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                        (*pC_24).seekResult = 0 as ::core::ffi::c_int;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_DeferredSeek =>  {
                        current_block = 2251003667842051127;
                    }
    crate::src::headers::opcodes_h::OP_IdxRowid_1 =>  {
                        current_block = 2251003667842051127;
                    }
    crate::src::headers::opcodes_h::OP_FinishSeek_1 =>  {
                        let mut pC_26: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        pC_26 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        if (*pC_26).deferredMoveto != 0 {
                            rc = crate::src::src::vdbeaux::sqlite3VdbeFinishMoveto(pC_26 as *mut crate::src::headers::vdbeInt_h::VdbeCursor);
                            if rc != 0 {
                                current_block = 9863799879598557851;
                                break;
                            }
                            current_block = 5783071609795492627;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_IdxLE_1 =>  {
                        current_block = 13800034716966053010;
                    }
    crate::src::headers::opcodes_h::OP_IdxGT_1 =>  {
                        current_block = 13800034716966053010;
                    }
    crate::src::headers::opcodes_h::OP_IdxLT =>  {
                        current_block = 970849332036924775;
                    }
    crate::src::headers::opcodes_h::OP_IdxGE =>  {
                        current_block = 6380214163151454549;
                    }
    crate::src::headers::opcodes_h::OP_Destroy_1 =>  {
                        let mut iMoved: ::core::ffi::c_int = 0;
                        let mut iDb_1: ::core::ffi::c_int = 0;
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        (*pOut).flags = crate::src::headers::vdbeInt_h::MEM_Null as crate::src::fts5::u16_0;
                        if (*db).nVdbeRead > (*db).nVDestroy + 1 as ::core::ffi::c_int {
                            rc = crate::src::headers::sqlite3_h::SQLITE_LOCKED;
                            __p_ref.errorAction = crate::src::headers::sqliteInt_h::OE_Abort as crate::src::ext::rtree::rtree::u8_0;
                            current_block = 9863799879598557851;
                            break;
                        } else {
                            iDb_1 = (*pOp).p3;
                            iMoved = 0 as ::core::ffi::c_int;
                            rc = crate::src::src::btree::sqlite3BtreeDropTable(
                                (*(*db).aDb.offset(iDb_1 as isize)).pBt,
                                (*pOp).p1,
                                &raw mut iMoved,
                            );
                            (*pOut).flags = crate::src::headers::vdbeInt_h::MEM_Int as crate::src::fts5::u16_0;
                            (*pOut).u.i = iMoved as crate::src::ext::rtree::rtree::i64_0;
                            if rc != 0 {
                                current_block = 9863799879598557851;
                                break;
                            }
                            if iMoved != 0 as ::core::ffi::c_int {
                                crate::src::src::build::sqlite3RootPageMoved(db as *mut crate::src::headers::sqliteInt_h::sqlite3, iDb_1, iMoved as crate::src::src::pager::Pgno, (*pOp).p1 as crate::src::src::pager::Pgno);
                                resetSchemaOnFault = (iDb_1 + 1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::u8_0;
                            }
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Clear_1 =>  {
                        let mut nChange: crate::src::ext::rtree::rtree::i64_0 = 0;
                        nChange = 0 as crate::src::ext::rtree::rtree::i64_0;
                        let __pOp_ref = unsafe { &mut *pOp };
                        rc = crate::src::src::btree::sqlite3BtreeClearTable(
                            (*(*db).aDb.offset(__pOp_ref.p2 as isize)).pBt,
                            __pOp_ref.p1 as crate::src::ext::rtree::rtree::u32_0 as ::core::ffi::c_int,
                            &raw mut nChange,
                        );
                        if __pOp_ref.p3 != 0 {
                            __p_ref.nChange += nChange;
                            if __pOp_ref.p3 > 0 as ::core::ffi::c_int {
                                (*aMem.offset(__pOp_ref.p3 as isize)).u.i += nChange;
                            }
                        }
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_ResetSorter_1 =>  {
                        let mut pC_28: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        pC_28 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        if (*pC_28).eCurType as ::core::ffi::c_int == crate::src::headers::vdbeInt_h::CURTYPE_SORTER {
                            crate::src::src::vdbesort::sqlite3VdbeSorterReset(db as *mut crate::src::headers::sqliteInt_h::sqlite3, (*pC_28).uc.pSorter);
                        } else {
                            rc = crate::src::src::btree::sqlite3BtreeClearTableOfCursor((*pC_28).uc.pCursor);
                            if rc != 0 {
                                current_block = 9863799879598557851;
                                break;
                            }
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_CreateBtree_1 =>  {
                        let mut pgno: crate::src::src::pager::Pgno = 0;
                        let mut pDb_2: *mut crate::src::headers::sqliteInt_h::Db = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Db>();
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        pgno = 0 as crate::src::src::pager::Pgno;
                        pDb_2 = (*db).aDb.offset((*pOp).p1 as isize) as *mut crate::src::headers::sqliteInt_h::Db;
                        rc = crate::src::src::btree::sqlite3BtreeCreateTable((*pDb_2).pBt, &raw mut pgno, (*pOp).p3);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        (*pOut).u.i = pgno as crate::src::ext::rtree::rtree::i64_0;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_SqlExec_1 =>  {
                        let mut zErr_0: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        let mut xAuth: crate::src::headers::sqliteInt_h::sqlite3_xauth = None;
                        let mut mTrace: crate::src::ext::rtree::rtree::u8_0 = 0;
                        let mut savedAnalysisLimit: ::core::ffi::c_int = 0;
                        let __db_ref = unsafe { &mut *db };
                        __db_ref.nSqlExec = __db_ref.nSqlExec.wrapping_add(1);
                        zErr_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        xAuth = __db_ref.xAuth;
                        mTrace = __db_ref.mTrace;
                        savedAnalysisLimit = __db_ref.nAnalysisLimit;
                        let __pOp_ref = unsafe { &*pOp };
                        if __pOp_ref.p1 & 0x1 as ::core::ffi::c_int != 0 {
                            __db_ref.xAuth = None;
                            __db_ref.mTrace = 0 as crate::src::ext::rtree::rtree::u8_0;
                        }
                        if __pOp_ref.p1 & 0x2 as ::core::ffi::c_int != 0 {
                            __db_ref.nAnalysisLimit = __pOp_ref.p2;
                        }
                        rc = crate::src::src::legacy::sqlite3_exec(
                            
                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                            __pOp_ref.p4.z,
                            None,
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                            &raw mut zErr_0,
                        );
                        __db_ref.nSqlExec = __db_ref.nSqlExec.wrapping_sub(1);
                        __db_ref.xAuth = xAuth;
                        __db_ref.mTrace = mTrace;
                        __db_ref.nAnalysisLimit = savedAnalysisLimit;
                        if !zErr_0.is_null() || rc != 0 {
                            crate::src::src::vdbeaux::sqlite3VdbeError(
                                
                                p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                zErr_0,
                            );
                            crate::src::src::malloc::sqlite3_free(zErr_0 as *mut ::core::ffi::c_void);
                            if rc == crate::src::headers::sqlite3_h::SQLITE_NOMEM {
                                current_block = 2471432538116610086;
                                break;
                            } else {
                                current_block = 9863799879598557851;
                                break;
                            }
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_ParseSchema =>  {
                        let mut iDb_2: ::core::ffi::c_int = 0;
                        let mut zSchema: *const ::core::ffi::c_char =
                            ::core::ptr::null::<::core::ffi::c_char>();
                        let mut zSql: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        let mut initData: crate::src::headers::sqliteInt_h::InitData = unsafe { ::core::mem::zeroed() };
                        iDb_2 = (*pOp).p1;
                        if (*pOp).p4.z.is_null() {
                            let __db_ref = unsafe { &mut *db };
                            crate::src::src::callback::sqlite3SchemaClear(
                                (*__db_ref.aDb.offset(iDb_2 as isize)).pSchema
                                    as *mut ::core::ffi::c_void,
                            );
                            __db_ref.mDbFlags &= !crate::src::headers::sqliteInt_h::DBFLAG_SchemaKnownOk as crate::src::ext::rtree::rtree::u32_0;
                            rc = crate::src::src::prepare::sqlite3InitOne(
                                
                                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                iDb_2,
                                &raw mut __p_ref.zErrMsg,
                                (*pOp).p5 as crate::src::ext::rtree::rtree::u32_0,
                            );
                            __db_ref.mDbFlags |= crate::src::headers::sqliteInt_h::DBFLAG_SchemaChange as crate::src::ext::rtree::rtree::u32_0;
                            __p_ref.set_expired(0 as crate::src::headers::sqliteInt_h::bft as crate::src::headers::sqliteInt_h::bft);
                        } else {
                            zSchema = crate::src::headers::sqliteInt_h::LEGACY_SCHEMA_TABLE.as_ptr();
                            initData.db = db;
                            initData.iDb = iDb_2;
                            initData.pzErrMsg = &raw mut __p_ref.zErrMsg;
                            initData.mInitFlags = 0 as crate::src::ext::rtree::rtree::u32_0;
                            initData.mxPage =
                                crate::src::src::btree::sqlite3BtreeLastPage((*(*db).aDb.offset(iDb_2 as isize)).pBt);
                            zSql = crate::src::src::printf::sqlite3MPrintf(
                                
                                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                b"SELECT*FROM\"%w\".%s WHERE %s ORDER BY rowid\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*(*db).aDb.offset(iDb_2 as isize)).zDbSName,
                                zSchema,
                                (*pOp).p4.z,
                            );
                            if zSql.is_null() {
                                rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
                            } else {
                                (*db).init.busy = 1 as crate::src::ext::rtree::rtree::u8_0;
                                initData.rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                                initData.nInitRow = 0 as crate::src::ext::rtree::rtree::u32_0;
                                rc = crate::src::src::legacy::sqlite3_exec(
                                    
                                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                    zSql,
                                    Some(
                                        crate::src::src::prepare::sqlite3InitCallback
                                            as unsafe extern "C" fn(
                                                *mut ::core::ffi::c_void,
                                                ::core::ffi::c_int,
                                                *mut *mut ::core::ffi::c_char,
                                                *mut *mut ::core::ffi::c_char,
                                            )
                                                -> ::core::ffi::c_int,
                                    ),
                                    &raw mut initData as *mut ::core::ffi::c_void,
                                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                );
                                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                    rc = initData.rc;
                                }
                                if rc == crate::src::headers::sqlite3_h::SQLITE_OK && initData.nInitRow == 0 as crate::src::ext::rtree::rtree::u32_0 {
                                    rc = crate::src::src::main::sqlite3CorruptError(7161 as ::core::ffi::c_int);
                                }
                                crate::src::src::malloc::sqlite3DbFreeNN(db as *mut crate::src::headers::sqliteInt_h::sqlite3, zSql as *mut ::core::ffi::c_void);
                                (*db).init.busy = 0 as crate::src::ext::rtree::rtree::u8_0;
                            }
                        }
                        if rc != 0 {
                            crate::src::src::build::sqlite3ResetAllSchemasOfConnection(db as *mut crate::src::headers::sqliteInt_h::sqlite3);
                            if rc == crate::src::headers::sqlite3_h::SQLITE_NOMEM {
                                current_block = 2471432538116610086;
                                break;
                            } else {
                                current_block = 9863799879598557851;
                                break;
                            }
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_LoadAnalysis_1 =>  {
                        rc = crate::src::src::analyze::sqlite3AnalysisLoad(db as *mut crate::src::headers::sqliteInt_h::sqlite3, (*pOp).p1);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_DropTable_1 =>  {
                        crate::src::src::build::sqlite3UnlinkAndDeleteTable(db as *mut crate::src::headers::sqliteInt_h::sqlite3, (*pOp).p1, (*pOp).p4.z);
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_DropIndex_1 =>  {
                        crate::src::src::build::sqlite3UnlinkAndDeleteIndex(db as *mut crate::src::headers::sqliteInt_h::sqlite3, (*pOp).p1, (*pOp).p4.z);
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_DropTrigger_1 =>  {
                        crate::src::src::trigger::sqlite3UnlinkAndDeleteTrigger(db as *mut crate::src::headers::sqliteInt_h::sqlite3, (*pOp).p1, (*pOp).p4.z);
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_IntegrityCk_1 =>  {
                        let mut nRoot: ::core::ffi::c_int = 0;
                        let mut aRoot: *mut crate::src::src::pager::Pgno = ::core::ptr::null_mut::<crate::src::src::pager::Pgno>();
                        let mut nErr: ::core::ffi::c_int = 0;
                        let mut z: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        let mut pnErr: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let __pOp_ref = unsafe { &*pOp };
                        nRoot = __pOp_ref.p2;
                        aRoot = __pOp_ref.p4.ai as *mut crate::src::src::pager::Pgno;
                        pnErr = aMem.offset(__pOp_ref.p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pIn1 =
                            aMem.offset((__pOp_ref.p1 + 1 as ::core::ffi::c_int) as isize) as *mut crate::src::src::vdbe::Mem;
                        rc = crate::src::src::btree::sqlite3BtreeIntegrityCheck(
                            
                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                            (*(*db).aDb.offset(__pOp_ref.p5 as isize)).pBt,
                            aRoot.offset(1 as isize) as *mut crate::src::src::pager::Pgno,
                            aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            nRoot,
                            (*pnErr).u.i as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                            &raw mut nErr,
                            &raw mut z,
                        );
                        crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        if !(nErr == 0 as ::core::ffi::c_int) {
                            if rc != 0 {
                                crate::src::src::malloc::sqlite3_free(z as *mut ::core::ffi::c_void);
                                current_block = 9863799879598557851;
                                break;
                            } else {
                                (*pnErr).u.i -= (nErr - 1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0;
                                crate::src::src::vdbemem::sqlite3VdbeMemSetStr(
                                    
                                    pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                    z,
                                    -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0,
                                    crate::src::headers::sqlite3_h::SQLITE_UTF8 as crate::src::ext::rtree::rtree::u8_0,
                                    Some(
                                        crate::src::src::malloc::sqlite3_free
                                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                                    ),
                                );
                            }
                        }
                        updateMaxBlobsize(pIn1);
                        crate::src::src::vdbemem::sqlite3VdbeChangeEncoding(pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value, encoding as ::core::ffi::c_int);
                        current_block = 20147595251170673;
                    }
    crate::src::headers::opcodes_h::OP_RowSetAdd_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pIn2 = aMem.offset((*pOp).p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pIn1).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Blob == 0 as ::core::ffi::c_int
                        {
                            if crate::src::src::vdbemem::sqlite3VdbeMemSetRowSet(pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value) != 0 {
                                current_block = 2471432538116610086;
                                break;
                            }
                        }
                        crate::src::src::rowset::sqlite3RowSetInsert((*pIn1).z as *mut crate::src::src::rowset::RowSet, (*pIn2).u.i);
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_RowSetRead_1 =>  {
                        let mut val: crate::src::ext::rtree::rtree::i64_0 = 0;
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pIn1).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Blob == 0 as ::core::ffi::c_int
                            || crate::src::src::rowset::sqlite3RowSetNext((*pIn1).z as *mut crate::src::src::rowset::RowSet, &raw mut val)
                                == 0 as ::core::ffi::c_int
                        {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                            current_block = 16391045501586817625;
                        } else {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetInt64(
                                
                                aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                val,
                            );
                            current_block = 20147595251170673;
                        }
                    }
    crate::src::headers::opcodes_h::OP_RowSetTest =>  {
                        let mut iSet: ::core::ffi::c_int = 0;
                        let mut exists: ::core::ffi::c_int = 0;
                        let __pOp_ref = unsafe { &*pOp };
                        pIn1 = aMem.offset(__pOp_ref.p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pIn3 = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        iSet = __pOp_ref.p4.i;
                        if (*pIn1).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Blob == 0 as ::core::ffi::c_int
                        {
                            if crate::src::src::vdbemem::sqlite3VdbeMemSetRowSet(pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value) != 0 {
                                current_block = 2471432538116610086;
                                break;
                            }
                        }
                        if iSet != 0 {
                            exists = crate::src::src::rowset::sqlite3RowSetTest((*pIn1).z as *mut crate::src::src::rowset::RowSet, iSet, (*pIn3).u.i);
                            if exists != 0 {
                                current_block = 9512719473022792396;
                            } else {
                                current_block = 13046928203416818118;
                            }
                        } else {
                            current_block = 13046928203416818118;
                        }
                        match current_block {
                            9512719473022792396 => {}
                            _ => {
                                if iSet >= 0 as ::core::ffi::c_int {
                                    crate::src::src::rowset::sqlite3RowSetInsert((*pIn1).z as *mut crate::src::src::rowset::RowSet, (*pIn3).u.i);
                                }
                                current_block = 5783071609795492627;
                            }
                        }
                    }
    crate::src::headers::opcodes_h::OP_Program_1 =>  {
                        let mut nMem: ::core::ffi::c_int = 0;
                        let mut nByte_1: crate::src::ext::rtree::rtree::i64_0 = 0;
                        let mut pRt: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let mut pMem_0: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let mut pEnd: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let mut pFrame_1: *mut crate::src::headers::vdbeInt_h::VdbeFrame = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeFrame>();
                        let mut pProgram: *mut crate::src::src::vdbe::SubProgram = ::core::ptr::null_mut::<crate::src::src::vdbe::SubProgram>();
                        let mut t_0: *mut ::core::ffi::c_void =
                            ::core::ptr::null_mut::<::core::ffi::c_void>();
                        pProgram = (*pOp).p4.pProgram;
                        pRt = aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pOp).p5 != 0 {
                            t_0 = (*pProgram).token;
                            pFrame_1 = __p_ref.pFrame;
                            while !pFrame_1.is_null() && (*pFrame_1).token != t_0 {
                                pFrame_1 = (*pFrame_1).pParent;
                            }
                            if !pFrame_1.is_null() {
                                current_block = 5783071609795492627;
                            } else {
                                current_block = 5564066760466108218;
                            }
                        } else {
                            current_block = 5564066760466108218;
                        }
                        match current_block {
                            5783071609795492627 => {}
                            _ => {
                                if __p_ref.nFrame >= (*db).aLimit[crate::src::headers::sqlite3_h::SQLITE_LIMIT_TRIGGER_DEPTH as usize]
                                {
                                    rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                                    crate::src::src::vdbeaux::sqlite3VdbeError(
                                        
                                        p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                        b"too many levels of trigger recursion\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    current_block = 9863799879598557851;
                                    break;
                                } else {
                                    let __pProgram_ref = unsafe { &mut *pProgram };
                                    if (*pRt).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Blob
                                        == 0 as ::core::ffi::c_int
                                    {
                                        nMem = __pProgram_ref.nMem + __pProgram_ref.nCsr;
                                        if __pProgram_ref.nCsr == 0 as ::core::ffi::c_int {
                                            nMem += 1;
                                        }
                                        nByte_1 = (((::core::mem::size_of::<crate::src::headers::vdbeInt_h::VdbeFrame>() as usize)
                                            .wrapping_add(7 as usize)
                                            & !(7 as ::core::ffi::c_int) as usize)
                                            .wrapping_add((nMem as usize).wrapping_mul(
                                                ::core::mem::size_of::<crate::src::src::vdbe::Mem>() as usize,
                                            ))
                                            .wrapping_add((__pProgram_ref.nCsr as usize).wrapping_mul(
                                                ::core::mem::size_of::<*mut crate::src::headers::vdbeInt_h::VdbeCursor>() as usize,
                                            ))
                                            as ::core::ffi::c_ulonglong)
                                            .wrapping_add(
                                                ((7 as crate::src::ext::rtree::rtree::i64_0 + __pProgram_ref.nOp as crate::src::ext::rtree::rtree::i64_0)
                                                    / 8 as crate::src::ext::rtree::rtree::i64_0)
                                                    as ::core::ffi::c_ulonglong,
                                            )
                                            as crate::src::ext::rtree::rtree::i64_0;
                                        pFrame_1 = crate::src::src::malloc::sqlite3DbMallocZero(db as *mut crate::src::headers::sqliteInt_h::sqlite3, nByte_1 as crate::src::ext::rtree::rtree::u64_0)
                                            as *mut crate::src::headers::vdbeInt_h::VdbeFrame;
                                        if pFrame_1.is_null() {
                                            current_block = 2471432538116610086;
                                            break;
                                        }
                                        crate::src::src::vdbemem::sqlite3VdbeMemRelease(pRt as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                                        let __pRt_ref = unsafe { &mut *pRt };
                                        __pRt_ref.flags = (crate::src::headers::vdbeInt_h::MEM_Blob | crate::src::headers::vdbeInt_h::MEM_Dyn) as crate::src::fts5::u16_0;
                                        __pRt_ref.z = pFrame_1 as *mut ::core::ffi::c_char;
                                        __pRt_ref.n = nByte_1 as ::core::ffi::c_int;
                                        __pRt_ref.xDel = Some(
                                            crate::src::src::vdbeaux::sqlite3VdbeFrameMemDel
                                                as unsafe extern "C" fn(
                                                    *mut ::core::ffi::c_void,
                                                )
                                                    -> (),
                                        )
                                            as Option<
                                                unsafe extern "C" fn(
                                                    *mut ::core::ffi::c_void,
                                                )
                                                    -> (),
                                            >;
                                        (*pFrame_1).v = p;
                                        (*pFrame_1).nChildMem = nMem;
                                        (*pFrame_1).nChildCsr = __pProgram_ref.nCsr;
                                        (*pFrame_1).pc = pOp.offset_from(aOp) as ::core::ffi::c_long
                                            as ::core::ffi::c_int;
                                        (*pFrame_1).aMem = __p_ref.aMem;
                                        (*pFrame_1).nMem = __p_ref.nMem;
                                        (*pFrame_1).apCsr = __p_ref.apCsr;
                                        (*pFrame_1).nCursor = __p_ref.nCursor;
                                        (*pFrame_1).aOp = __p_ref.aOp;
                                        (*pFrame_1).nOp = __p_ref.nOp;
                                        (*pFrame_1).token = __pProgram_ref.token;
                                        pEnd = ((pFrame_1 as *mut crate::src::ext::rtree::rtree::u8_0).offset(
                                            ((::core::mem::size_of::<crate::src::headers::vdbeInt_h::VdbeFrame>() as usize)
                                                .wrapping_add(7 as usize)
                                                & !(7 as ::core::ffi::c_int) as usize)
                                                as isize,
                                        )
                                            as *mut crate::src::ext::rtree::rtree::u8_0
                                            as *mut crate::src::src::vdbe::Mem)
                                            .offset((*pFrame_1).nChildMem as isize)
                                            as *mut crate::src::src::vdbe::Mem;
                                        pMem_0 = (pFrame_1 as *mut crate::src::ext::rtree::rtree::u8_0).offset(
                                            ((::core::mem::size_of::<crate::src::headers::vdbeInt_h::VdbeFrame>() as usize)
                                                .wrapping_add(7 as usize)
                                                & !(7 as ::core::ffi::c_int) as usize)
                                                as isize,
                                        )
                                            as *mut crate::src::ext::rtree::rtree::u8_0
                                            as *mut crate::src::src::vdbe::Mem;
                                        while pMem_0 != pEnd {
                                            (*pMem_0).flags = crate::src::headers::vdbeInt_h::MEM_Undefined as crate::src::fts5::u16_0;
                                            (*pMem_0).db = db;
                                            pMem_0 = pMem_0.offset(1);
                                        }
                                    } else {
                                        pFrame_1 = (*pRt).z as *mut crate::src::headers::vdbeInt_h::VdbeFrame;
                                    }
                                    __p_ref.nFrame += 1;
                                    (*pFrame_1).pParent = __p_ref.pFrame;
                                    (*pFrame_1).lastRowid = (*db).lastRowid;
                                    (*pFrame_1).nChange = __p_ref.nChange;
                                    (*pFrame_1).nDbChange = (*__p_ref.db).nChange;
                                    (*pFrame_1).pAuxData = __p_ref.pAuxData;
                                    __p_ref.pAuxData = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::AuxData>();
                                    __p_ref.nChange = 0 as crate::src::ext::rtree::rtree::i64_0;
                                    __p_ref.pFrame = pFrame_1;
                                    aMem = (pFrame_1 as *mut crate::src::ext::rtree::rtree::u8_0).offset(
                                        ((::core::mem::size_of::<crate::src::headers::vdbeInt_h::VdbeFrame>() as usize)
                                            .wrapping_add(7 as usize)
                                            & !(7 as ::core::ffi::c_int) as usize)
                                            as isize,
                                    ) as *mut crate::src::ext::rtree::rtree::u8_0
                                        as *mut crate::src::src::vdbe::Mem;
                                    __p_ref.aMem = aMem;
                                    __p_ref.nMem = (*pFrame_1).nChildMem;
                                    __p_ref.nCursor =
                                        (*pFrame_1).nChildCsr as crate::src::fts5::u16_0 as ::core::ffi::c_int;
                                    __p_ref.apCsr = aMem.offset(__p_ref.nMem as isize) as *mut crate::src::src::vdbe::Mem
                                        as *mut *mut crate::src::headers::vdbeInt_h::VdbeCursor;
                                    (*pFrame_1).aOnce = __p_ref.apCsr.offset(__pProgram_ref.nCsr as isize)
                                        as *mut *mut crate::src::headers::vdbeInt_h::VdbeCursor
                                        as *mut crate::src::ext::rtree::rtree::u8_0;
                                    ::libc::memset(
                                        (*pFrame_1).aOnce as *mut ::core::ffi::c_void,
                                        0 as ::core::ffi::c_int,
                                        ((__pProgram_ref.nOp + 7 as ::core::ffi::c_int)
                                            / 8 as ::core::ffi::c_int)
                                            as crate::__stddef_size_t_h::size_t,
                                    );
                                    aOp = __pProgram_ref.aOp as *mut crate::src::headers::vdbeInt_h::Op;
                                    __p_ref.aOp = aOp;
                                    __p_ref.nOp = __pProgram_ref.nOp;
                                    pOp =
                                        aOp.offset(-(1 as ::core::ffi::c_int) as isize) as *mut crate::src::headers::vdbeInt_h::Op;
                                }
                                current_block = 20147595251170673;
                            }
                        }
                    }
    crate::src::headers::opcodes_h::OP_Param_1 =>  {
                        let mut pFrame_2: *mut crate::src::headers::vdbeInt_h::VdbeFrame = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeFrame>();
                        let mut pIn: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        pFrame_2 = __p_ref.pFrame;
                        pIn = (*pFrame_2).aMem.offset(
                            ((*pOp).p1 + (*(*pFrame_2).aOp.offset((*pFrame_2).pc as isize)).p1)
                                as isize,
                        ) as *mut crate::src::src::vdbe::Mem;
                        crate::src::src::vdbemem::sqlite3VdbeMemShallowCopy(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value,  pIn as *const crate::src::headers::vdbeInt_h::sqlite3_value, crate::src::headers::vdbeInt_h::MEM_Ephem);
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_FkCounter_1 =>  {
                        if (*pOp).p1 != 0 {
                            (*db).nDeferredCons += (*pOp).p2 as crate::src::ext::rtree::rtree::i64_0;
                        } else if (*db).flags & crate::src::headers::sqliteInt_h::SQLITE_DeferFKs as crate::src::ext::rtree::rtree::u64_0 != 0 {
                            (*db).nDeferredImmCons += (*pOp).p2 as crate::src::ext::rtree::rtree::i64_0;
                        } else {
                            __p_ref.nFkConstraint += (*pOp).p2 as crate::src::ext::rtree::rtree::i64_0;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_FkIfZero_1 =>  {
                        if (*pOp).p1 != 0 {
                            if (*db).nDeferredCons == 0 as crate::src::ext::rtree::rtree::i64_0
                                && (*db).nDeferredImmCons == 0 as crate::src::ext::rtree::rtree::i64_0
                            {
                                current_block = 9512719473022792396;
                            } else {
                                current_block = 5783071609795492627;
                            }
                        } else if __p_ref.nFkConstraint == 0 as crate::src::ext::rtree::rtree::i64_0
                            && (*db).nDeferredImmCons == 0 as crate::src::ext::rtree::rtree::i64_0
                        {
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_MemMax_1 =>  {
                        let mut pFrame_3: *mut crate::src::headers::vdbeInt_h::VdbeFrame = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeFrame>();
                        if !__p_ref.pFrame.is_null() {
                            pFrame_3 = __p_ref.pFrame;
                            while !(*pFrame_3).pParent.is_null() {
                                pFrame_3 = (*pFrame_3).pParent;
                            }
                            pIn1 = (*pFrame_3).aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        } else {
                            pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        }
                        crate::src::src::vdbemem::sqlite3VdbeMemIntegerify(pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        pIn2 = aMem.offset((*pOp).p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        crate::src::src::vdbemem::sqlite3VdbeMemIntegerify(pIn2 as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        if (*pIn1).u.i < (*pIn2).u.i {
                            (*pIn1).u.i = (*pIn2).u.i;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_IfPos_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pIn1).u.i > 0 as crate::src::ext::rtree::rtree::i64_0 {
                            (*pIn1).u.i -= (*pOp).p3 as crate::src::ext::rtree::rtree::i64_0;
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_OffsetLimit_1 =>  {
                        let mut x_2: crate::src::ext::rtree::rtree::i64_0 = 0;
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pIn3 = aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        x_2 = (*pIn1).u.i;
                        if x_2 <= 0 as crate::src::ext::rtree::rtree::i64_0
                            || crate::src::src::util::sqlite3AddInt64(
                                &raw mut x_2,
                                if (*pIn3).u.i > 0 as crate::src::ext::rtree::rtree::i64_0 {
                                    (*pIn3).u.i
                                } else {
                                    0 as crate::src::ext::rtree::rtree::i64_0
                                },
                            ) != 0
                        {
                            (*pOut).u.i = -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0;
                        } else {
                            (*pOut).u.i = x_2;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_IfNotZero_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pIn1).u.i != 0 {
                            if (*pIn1).u.i > 0 as crate::src::ext::rtree::rtree::i64_0 {
                                (*pIn1).u.i -= 1;
                            }
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_DecrJumpZero_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pIn1).u.i > crate::fts3Int_h::SMALLEST_INT64 {
                            (*pIn1).u.i -= 1;
                        }
                        if (*pIn1).u.i == 0 as crate::src::ext::rtree::rtree::i64_0 {
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_AggInverse | crate::src::headers::opcodes_h::OP_AggStep_1 =>  {
                        let mut n_3: ::core::ffi::c_int = 0;
                        let mut pCtx: *mut crate::src::headers::vdbeInt_h::sqlite3_context =
                            ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_context>();
                        let mut nAlloc: crate::src::ext::rtree::rtree::u64_0 = 0;
                        let __pOp_ref = unsafe { &mut *pOp };
                        n_3 = __pOp_ref.p5 as ::core::ffi::c_int;
                        nAlloc = (48 as usize)
                            .wrapping_add((n_3 as usize).wrapping_mul(::core::mem::size_of::<
                            *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        >(
                        )
                            as usize)) as crate::src::ext::rtree::rtree::u64_0;
                        pCtx = crate::src::src::malloc::sqlite3DbMallocRawNN(
                            
                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                            nAlloc.wrapping_add(::core::mem::size_of::<crate::src::src::vdbe::Mem>() as crate::src::ext::rtree::rtree::u64_0),
                        ) as *mut crate::src::headers::vdbeInt_h::sqlite3_context;
                        if pCtx.is_null() {
                            current_block = 2471432538116610086;
                            break;
                        }
                        (*pCtx).pOut = (pCtx as *mut crate::src::ext::rtree::rtree::u8_0).offset(nAlloc as isize) as *mut crate::src::src::vdbe::Mem;
                        crate::src::src::vdbemem::sqlite3VdbeMemInit((*pCtx).pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value,  db as *mut crate::src::headers::sqliteInt_h::sqlite3, crate::src::headers::vdbeInt_h::MEM_Null as crate::src::fts5::u16_0);
                        (*pCtx).pMem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        (*pCtx).pFunc = __pOp_ref.p4.pFunc;
                        (*pCtx).iOp =
                            pOp.offset_from(aOp) as ::core::ffi::c_long as ::core::ffi::c_int;
                        (*pCtx).pVdbe = p;
                        (*pCtx).skipFlag = 0 as crate::src::ext::rtree::rtree::u8_0;
                        (*pCtx).isError = 0 as ::core::ffi::c_int;
                        (*pCtx).enc = encoding;
                        (*pCtx).argc = n_3 as crate::src::fts5::u16_0;
                        __pOp_ref.p4type = crate::src::src::vdbe::P4_FUNCCTX as ::core::ffi::c_schar;
                        __pOp_ref.p4.pCtx = pCtx;
                        __pOp_ref.opcode = crate::src::headers::opcodes_h::OP_AggStep1 as crate::src::ext::rtree::rtree::u8_0;
                        current_block = 1931008622201330627;
                    }
    crate::src::headers::opcodes_h::OP_AggStep1 =>  {
                        current_block = 1931008622201330627;
                    }
    crate::src::headers::opcodes_h::OP_AggValue | crate::src::headers::opcodes_h::OP_AggFinal_1 =>  {
                        let mut pMem_2: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        pMem_2 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pOp).p3 != 0 {
                            let __pOp_ref = unsafe { &*pOp };
                            rc = crate::src::src::vdbemem::sqlite3VdbeMemAggValue(
                                
                                pMem_2 as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                
                                aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                
                                __pOp_ref.p4.pFunc as *mut crate::src::headers::sqliteInt_h::FuncDef,
                            );
                            pMem_2 = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        } else {
                            rc = crate::src::src::vdbemem::sqlite3VdbeMemFinalize(pMem_2 as *mut crate::src::headers::vdbeInt_h::sqlite3_value,  (*pOp).p4.pFunc as *mut crate::src::headers::sqliteInt_h::FuncDef);
                        }
                        if rc != 0 {
                            crate::src::src::vdbeaux::sqlite3VdbeError(
                                
                                p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                crate::src::src::vdbeapi::sqlite3_value_text(pMem_2 as *mut crate::src::headers::vdbeInt_h::sqlite3_value as *mut crate::src::headers::vdbeInt_h::sqlite3_value),
                            );
                            current_block = 9863799879598557851;
                            break;
                        } else {
                            crate::src::src::vdbemem::sqlite3VdbeChangeEncoding(pMem_2 as *mut crate::src::headers::vdbeInt_h::sqlite3_value, encoding as ::core::ffi::c_int);
                            updateMaxBlobsize(pMem_2);
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Checkpoint_1 =>  {
                        let mut i_4: ::core::ffi::c_int = 0;
                        let mut aRes: [::core::ffi::c_int; 3] = [0; 3];
                        let mut pMem_3: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        aRes[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
                        aRes[2 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
                        aRes[1 as ::core::ffi::c_int as usize] =
                            aRes[2 as ::core::ffi::c_int as usize];
                        let __pOp_ref = unsafe { &*pOp };
                        rc = crate::src::src::main::sqlite3Checkpoint(
                            
                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                            __pOp_ref.p1,
                            __pOp_ref.p2,
                            (&raw mut aRes as *mut ::core::ffi::c_int)
                                .offset(1 as isize)
                                as *mut ::core::ffi::c_int,
                            (&raw mut aRes as *mut ::core::ffi::c_int)
                                .offset(2 as isize)
                                as *mut ::core::ffi::c_int,
                        );
                        if rc != 0 {
                            if rc != crate::src::headers::sqlite3_h::SQLITE_BUSY {
                                current_block = 9863799879598557851;
                                break;
                            }
                            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                            aRes[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
                        }
                        i_4 = 0 as ::core::ffi::c_int;
                        pMem_3 = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        while i_4 < 3 as ::core::ffi::c_int {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetInt64(pMem_3 as *mut crate::src::headers::vdbeInt_h::sqlite3_value, aRes[i_4 as usize] as crate::src::ext::rtree::rtree::i64_0);
                            i_4 += 1;
                            pMem_3 = pMem_3.offset(1);
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_JournalMode_1 =>  {
                        let mut pBt_0: *mut crate::src::headers::btreeInt_h::Btree = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::Btree>();
                        let mut pPager: *mut crate::src::src::pager::Pager = ::core::ptr::null_mut::<crate::src::src::pager::Pager>();
                        let mut eNew: ::core::ffi::c_int = 0;
                        let mut eOld: ::core::ffi::c_int = 0;
                        let mut zFilename: *const ::core::ffi::c_char =
                            ::core::ptr::null::<::core::ffi::c_char>();
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        eNew = (*pOp).p3;
                        pBt_0 = (*(*db).aDb.offset((*pOp).p1 as isize)).pBt;
                        pPager = crate::src::src::btree::sqlite3BtreePager(pBt_0) as *mut crate::src::src::pager::Pager;
                        eOld = crate::src::src::pager::sqlite3PagerGetJournalMode(pPager);
                        if eNew == crate::src::src::pager::PAGER_JOURNALMODE_QUERY {
                            eNew = eOld;
                        }
                        if crate::src::src::pager::sqlite3PagerOkToChangeJournalMode(pPager) == 0 {
                            eNew = eOld;
                        }
                        zFilename = crate::src::src::pager::sqlite3PagerFilename(pPager, 1 as ::core::ffi::c_int);
                        if eNew == crate::src::src::pager::PAGER_JOURNALMODE_WAL
                            && (crate::src::src::util::sqlite3Strlen30(zFilename) == 0 as ::core::ffi::c_int
                                || crate::src::src::pager::sqlite3PagerWalSupported(pPager) == 0)
                        {
                            eNew = eOld;
                        }
                        if eNew != eOld
                            && (eOld == crate::src::src::pager::PAGER_JOURNALMODE_WAL || eNew == crate::src::src::pager::PAGER_JOURNALMODE_WAL)
                        {
                            if (*db).autoCommit == 0 || (*db).nVdbeRead > 1 as ::core::ffi::c_int {
                                rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                                crate::src::src::vdbeaux::sqlite3VdbeError(
                                    
                                    p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                    b"cannot change %s wal mode from within a transaction\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    if eNew == crate::src::src::pager::PAGER_JOURNALMODE_WAL {
                                        b"into\0" as *const u8 as *const ::core::ffi::c_char
                                    } else {
                                        b"out of\0" as *const u8 as *const ::core::ffi::c_char
                                    },
                                );
                                current_block = 9863799879598557851;
                                break;
                            } else {
                                if eOld == crate::src::src::pager::PAGER_JOURNALMODE_WAL {
                                    rc = crate::src::src::pager::sqlite3PagerCloseWal(pPager,  db as *mut crate::src::headers::sqliteInt_h::sqlite3);
                                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                        crate::src::src::pager::sqlite3PagerSetJournalMode(pPager, eNew);
                                    }
                                } else if eOld == crate::src::src::pager::PAGER_JOURNALMODE_MEMORY {
                                    crate::src::src::pager::sqlite3PagerSetJournalMode(pPager, crate::src::src::pager::PAGER_JOURNALMODE_OFF);
                                }
                                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                    rc = crate::src::src::btree::sqlite3BtreeSetVersion(
                                        pBt_0,
                                        if eNew == crate::src::src::pager::PAGER_JOURNALMODE_WAL {
                                            2 as ::core::ffi::c_int
                                        } else {
                                            1 as ::core::ffi::c_int
                                        },
                                    );
                                }
                            }
                        }
                        if rc != 0 {
                            eNew = eOld;
                        }
                        eNew = crate::src::src::pager::sqlite3PagerSetJournalMode(pPager, eNew);
                        (*pOut).flags = (crate::src::headers::vdbeInt_h::MEM_Str | crate::src::headers::vdbeInt_h::MEM_Static | crate::src::headers::vdbeInt_h::MEM_Term) as crate::src::fts5::u16_0;
                        (*pOut).z = crate::src::src::pragma::sqlite3JournalModename(eNew) as *mut ::core::ffi::c_char;
                        (*pOut).n = crate::src::src::util::sqlite3Strlen30((*pOut).z);
                        (*pOut).enc = crate::src::headers::sqlite3_h::SQLITE_UTF8 as crate::src::ext::rtree::rtree::u8_0;
                        crate::src::src::vdbemem::sqlite3VdbeChangeEncoding(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value, encoding as ::core::ffi::c_int);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Vacuum_1 =>  {
                        rc = crate::src::src::vacuum::sqlite3RunVacuum(
                            &raw mut __p_ref.zErrMsg,
                            
                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                            (*pOp).p1,
                            
                            if (*pOp).p2 != 0 {
                                aMem.offset((*pOp).p2 as isize) as *mut crate::src::headers::vdbeInt_h::sqlite3_value
                            } else {
                                ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_value>()
                            }
    as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        );
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_IncrVacuum_1 =>  {
                        let mut pBt_1: *mut crate::src::headers::btreeInt_h::Btree = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::Btree>();
                        pBt_1 = (*(*db).aDb.offset((*pOp).p1 as isize)).pBt;
                        rc = crate::src::src::btree::sqlite3BtreeIncrVacuum(pBt_1);
                        if rc != 0 {
                            if rc != crate::src::headers::sqlite3_h::SQLITE_DONE {
                                current_block = 9863799879598557851;
                                break;
                            }
                            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_Expire_1 =>  {
                        if (*pOp).p1 == 0 {
                            crate::src::src::vdbeaux::sqlite3ExpirePreparedStatements(db as *mut crate::src::headers::sqliteInt_h::sqlite3, (*pOp).p2);
                        } else {
                            __p_ref.set_expired(((*pOp).p2 + 1 as ::core::ffi::c_int) as crate::src::headers::sqliteInt_h::bft as crate::src::headers::sqliteInt_h::bft);
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_CursorLock_1 =>  {
                        let mut pC_29: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        pC_29 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        crate::src::src::btree::sqlite3BtreeCursorPin((*pC_29).uc.pCursor);
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_CursorUnlock_1 =>  {
                        let mut pC_30: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        pC_30 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        crate::src::src::btree::sqlite3BtreeCursorUnpin((*pC_30).uc.pCursor);
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_TableLock_1 =>  {
                        let mut isWriteLock: crate::src::ext::rtree::rtree::u8_0 = (*pOp).p3 as crate::src::ext::rtree::rtree::u8_0;
                        if isWriteLock as ::core::ffi::c_int != 0
                            || 0 as crate::src::ext::rtree::rtree::u64_0 == (*db).flags & crate::src::headers::sqliteInt_h::SQLITE_ReadUncommit
                        {
                            let mut p1_2: ::core::ffi::c_int = (*pOp).p1;
                            rc = crate::src::src::btree::sqlite3BtreeLockTable(
                                (*(*db).aDb.offset(p1_2 as isize)).pBt,
                                (*pOp).p2,
                                isWriteLock,
                            );
                            if rc != 0 {
                                if rc & 0xff as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_LOCKED {
                                    let mut z_0: *const ::core::ffi::c_char = (*pOp).p4.z;
                                    crate::src::src::vdbeaux::sqlite3VdbeError(
                                        
                                        p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                        b"database table is locked: %s\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        z_0,
                                    );
                                }
                                current_block = 9863799879598557851;
                                break;
                            } else {
                                current_block = 5783071609795492627;
                            }
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_VBegin_1 =>  {
                        let mut pVTab: *mut crate::src::headers::sqliteInt_h::VTable = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::VTable>();
                        pVTab = (*pOp).p4.pVtab;
                        rc = crate::src::src::vtab::sqlite3VtabBegin(db as *mut crate::src::headers::sqliteInt_h::sqlite3,  pVTab as *mut crate::src::headers::sqliteInt_h::VTable);
                        if !pVTab.is_null() {
                            crate::src::src::vdbeaux::sqlite3VtabImportErrmsg(p as *mut crate::src::headers::vdbeInt_h::Vdbe,  (*pVTab).pVtab as *mut crate::src::headers::sqlite3_h::sqlite3_vtab);
                        }
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_VCreate =>  {
                        let mut sMem_0: crate::src::src::vdbe::Mem = unsafe { ::core::mem::zeroed() };
                        let mut zTab: *const ::core::ffi::c_char =
                            ::core::ptr::null::<::core::ffi::c_char>();
                        sMem_0.db = db;
                        rc = crate::src::src::vdbemem::sqlite3VdbeMemCopy(
                            
                            &raw mut sMem_0 as *mut _ as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            
                            aMem.offset((*pOp).p2 as isize) as *mut crate::src::src::vdbe::Mem as *const crate::src::headers::vdbeInt_h::sqlite3_value,
                        );
                        zTab = crate::src::src::vdbeapi::sqlite3_value_text(&raw mut sMem_0 as *mut _ as *mut crate::src::headers::vdbeInt_h::sqlite3_value) as *const ::core::ffi::c_char;
                        if !zTab.is_null() {
                            rc = crate::src::src::vtab::sqlite3VtabCallCreate(db as *mut crate::src::headers::sqliteInt_h::sqlite3, (*pOp).p1, zTab, &raw mut __p_ref.zErrMsg);
                        }
                        crate::src::src::vdbemem::sqlite3VdbeMemRelease(&raw mut sMem_0 as *mut _ as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_VDestroy_1 =>  {
                        (*db).nVDestroy += 1;
                        rc = crate::src::src::vtab::sqlite3VtabCallDestroy(db as *mut crate::src::headers::sqliteInt_h::sqlite3, (*pOp).p1, (*pOp).p4.z);
                        (*db).nVDestroy -= 1;
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_VOpen_1 =>  {
                        let mut pCur_2: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut pVCur: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor =
                            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vtab_cursor>();
                        let mut pVtab_0: *mut crate::src::headers::sqlite3_h::sqlite3_vtab =
                            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vtab>();
                        let mut pModule_0: *const crate::src::headers::sqlite3_h::sqlite3_module =
                            ::core::ptr::null::<crate::src::headers::sqlite3_h::sqlite3_module>();
                        pCur_2 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        if !(!pCur_2.is_null()
                            && (*pCur_2).eCurType as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                            && (*(*pCur_2).uc.pVCur).pVtab == (*(*pOp).p4.pVtab).pVtab)
                        {
                            pVCur = ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vtab_cursor>();
                            pVtab_0 = (*(*pOp).p4.pVtab).pVtab;
                            if pVtab_0.is_null() || (*pVtab_0).pModule.is_null() {
                                rc = crate::src::headers::sqlite3_h::SQLITE_LOCKED;
                                current_block = 9863799879598557851;
                                break;
                            } else {
                                pModule_0 = (*pVtab_0).pModule;
                                rc = (*pModule_0).xOpen.expect("non-null function pointer")(
                                    pVtab_0,
                                    &raw mut pVCur,
                                );
                                crate::src::src::vdbeaux::sqlite3VtabImportErrmsg(p as *mut crate::src::headers::vdbeInt_h::Vdbe,  pVtab_0 as *mut crate::src::headers::sqlite3_h::sqlite3_vtab);
                                if rc != 0 {
                                    current_block = 9863799879598557851;
                                    break;
                                }
                                (*pVCur).pVtab = pVtab_0;
                                pCur_2 = allocateCursor(
                                    p,
                                    (*pOp).p1,
                                    0 as ::core::ffi::c_int,
                                    crate::src::headers::vdbeInt_h::CURTYPE_VTAB as crate::src::ext::rtree::rtree::u8_0,
                                );
                                if !pCur_2.is_null() {
                                    (*pCur_2).uc.pVCur = pVCur;
                                    (*pVtab_0).nRef += 1;
                                } else {
                                    (*pModule_0).xClose.expect("non-null function pointer")(pVCur);
                                    current_block = 2471432538116610086;
                                    break;
                                }
                            }
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_VCheck_1 =>  {
                        let mut pTab_2: *mut crate::src::headers::sqliteInt_h::Table = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Table>();
                        let mut pVtab_1: *mut crate::src::headers::sqlite3_h::sqlite3_vtab =
                            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vtab>();
                        let mut pModule_1: *const crate::src::headers::sqlite3_h::sqlite3_module =
                            ::core::ptr::null::<crate::src::headers::sqlite3_h::sqlite3_module>();
                        let mut zErr_1: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        pOut = aMem.offset((*pOp).p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        pTab_2 = (*pOp).p4.pTab;
                        if (*pTab_2).u.vtab.p.is_null() {
                            current_block = 5783071609795492627;
                        } else {
                            let __pTab_2_ref = unsafe { &*pTab_2 };
                            pVtab_1 = (*__pTab_2_ref.u.vtab.p).pVtab;
                            pModule_1 = (*pVtab_1).pModule;
                            crate::src::src::vtab::sqlite3VtabLock(__pTab_2_ref.u.vtab.p as *mut crate::src::headers::sqliteInt_h::VTable);
                            rc = (*pModule_1).xIntegrity.expect("non-null function pointer")(
                                pVtab_1,
                                (*(*db).aDb.offset((*pOp).p1 as isize)).zDbSName,
                                __pTab_2_ref.zName,
                                (*pOp).p3,
                                &raw mut zErr_1,
                            );
                            crate::src::src::vtab::sqlite3VtabUnlock(__pTab_2_ref.u.vtab.p as *mut crate::src::headers::sqliteInt_h::VTable);
                            if rc != 0 {
                                crate::src::src::malloc::sqlite3_free(zErr_1 as *mut ::core::ffi::c_void);
                                current_block = 9863799879598557851;
                                break;
                            } else if !zErr_1.is_null() {
                                crate::src::src::vdbemem::sqlite3VdbeMemSetStr(
                                    
                                    pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                    zErr_1,
                                    -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0,
                                    crate::src::headers::sqlite3_h::SQLITE_UTF8 as crate::src::ext::rtree::rtree::u8_0,
                                    Some(
                                        crate::src::src::malloc::sqlite3_free
                                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                                    ),
                                );
                            }
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_VInitIn =>  {
                        let mut pC_31: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut pRhs: *mut crate::src::headers::vdbeInt_h::ValueList = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::ValueList>();
                        pC_31 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        pRhs =
                            crate::src::src::malloc::sqlite3_malloc64(::core::mem::size_of::<crate::src::headers::vdbeInt_h::ValueList>() as crate::src::headers::sqlite3_h::sqlite3_uint64)
                                as *mut crate::src::headers::vdbeInt_h::ValueList;
                        if pRhs.is_null() {
                            current_block = 2471432538116610086;
                            break;
                        }
                        (*pRhs).pCsr = (*pC_31).uc.pCursor;
                        (*pRhs).pOut =
                            aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem as *mut crate::src::headers::vdbeInt_h::sqlite3_value;
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        (*pOut).flags = crate::src::headers::vdbeInt_h::MEM_Null as crate::src::fts5::u16_0;
                        crate::src::src::vdbemem::sqlite3VdbeMemSetPointer(
                            
                            pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            pRhs as *mut ::core::ffi::c_void,
                            b"ValueList\0" as *const u8 as *const ::core::ffi::c_char,
                            Some(
                                crate::src::src::vdbeapi::sqlite3VdbeValueListFree
                                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                            ),
                        );
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_VFilter =>  {
                        let mut nArg: ::core::ffi::c_int = 0;
                        let mut iQuery: ::core::ffi::c_int = 0;
                        let mut pModule_2: *const crate::src::headers::sqlite3_h::sqlite3_module =
                            ::core::ptr::null::<crate::src::headers::sqlite3_h::sqlite3_module>();
                        let mut pQuery: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let mut pArgc: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let mut pVCur_0: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor =
                            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vtab_cursor>();
                        let mut pVtab_2: *mut crate::src::headers::sqlite3_h::sqlite3_vtab =
                            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vtab>();
                        let mut pCur_3: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut res_11: ::core::ffi::c_int = 0;
                        let mut i_5: ::core::ffi::c_int = 0;
                        let mut apArg: *mut *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<*mut crate::src::src::vdbe::Mem>();
                        let __pOp_ref = unsafe { &*pOp };
                        pQuery = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        pArgc = pQuery.offset(1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pCur_3 = *__p_ref.apCsr.offset(__pOp_ref.p1 as isize);
                        pVCur_0 = (*pCur_3).uc.pVCur;
                        pVtab_2 = (*pVCur_0).pVtab;
                        pModule_2 = (*pVtab_2).pModule;
                        nArg = (*pArgc).u.i as ::core::ffi::c_int;
                        iQuery = (*pQuery).u.i as ::core::ffi::c_int;
                        apArg = __p_ref.apArg;
                        i_5 = 0 as ::core::ffi::c_int;
                        while i_5 < nArg {
                            let ref mut fresh15 = *apArg.offset(i_5 as isize);
                            *fresh15 =
                                pArgc.offset((i_5 + 1 as ::core::ffi::c_int) as isize) as *mut crate::src::src::vdbe::Mem;
                            i_5 += 1;
                        }
                        rc = (*pModule_2).xFilter.expect("non-null function pointer")(
                            pVCur_0,
                            iQuery,
                            __pOp_ref.p4.z,
                            nArg,
                            apArg as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        );
                        crate::src::src::vdbeaux::sqlite3VtabImportErrmsg(p as *mut crate::src::headers::vdbeInt_h::Vdbe,  pVtab_2 as *mut crate::src::headers::sqlite3_h::sqlite3_vtab);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        res_11 = (*pModule_2).xEof.expect("non-null function pointer")(pVCur_0);
                        (*pCur_3).nullRow = 0 as crate::src::ext::rtree::rtree::u8_0;
                        if res_11 != 0 {
                            current_block = 9512719473022792396;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_VColumn_1 =>  {
                        let mut pVtab_3: *mut crate::src::headers::sqlite3_h::sqlite3_vtab =
                            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vtab>();
                        let mut pModule_3: *const crate::src::headers::sqlite3_h::sqlite3_module =
                            ::core::ptr::null::<crate::src::headers::sqlite3_h::sqlite3_module>();
                        let mut pDest_1: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let mut sContext: crate::src::headers::vdbeInt_h::sqlite3_context = unsafe { ::core::mem::zeroed() };
                        let mut nullFunc: crate::src::headers::sqliteInt_h::FuncDef = crate::src::headers::sqliteInt_h::FuncDef {
    nArg:  0,
    funcFlags:  0,
    pUserData:  ::core::ptr::null_mut::<::core::ffi::c_void>(),
    pNext:  ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::FuncDef>(),
    xSFunc:  None,
    xFinalize:  None,
    xValue:  None,
    xInverse:  None,
    zName:  ::core::ptr::null::<::core::ffi::c_char>(),
    u:  crate::src::headers::sqliteInt_h::__anon_union_2 {
    pHash:  ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::FuncDef>(),
},
};
                        let mut pCur_4: *mut crate::src::headers::vdbeInt_h::VdbeCursor = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        pDest_1 = aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pCur_4).nullRow != 0 {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pDest_1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        } else {
                            pVtab_3 = (*(*pCur_4).uc.pVCur).pVtab;
                            pModule_3 = (*pVtab_3).pModule;
                            sContext.pOut = pDest_1;
                            sContext.enc = encoding;
                            nullFunc.pUserData = ::core::ptr::null_mut::<::core::ffi::c_void>();
                            nullFunc.funcFlags = crate::src::headers::sqlite3_h::SQLITE_RESULT_SUBTYPE as crate::src::ext::rtree::rtree::u32_0;
                            sContext.pFunc = &raw mut nullFunc;
                            if (*pOp).p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::OPFLAG_NOCHNG != 0 {
                                crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pDest_1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                                (*pDest_1).flags = (crate::src::headers::vdbeInt_h::MEM_Null | crate::src::headers::vdbeInt_h::MEM_Zero) as crate::src::fts5::u16_0;
                                (*pDest_1).u.nZero = 0 as ::core::ffi::c_int;
                            } else {
                                (*pDest_1).flags = ((*pDest_1).flags as ::core::ffi::c_int
                                    & !(crate::src::headers::vdbeInt_h::MEM_TypeMask | crate::src::headers::vdbeInt_h::MEM_Zero)
                                    | 0x1 as ::core::ffi::c_int)
                                    as crate::src::fts5::u16_0;
                            }
                            rc = (*pModule_3).xColumn.expect("non-null function pointer")(
                                (*pCur_4).uc.pVCur,
                                &raw mut sContext,
                                (*pOp).p2,
                            );
                            crate::src::src::vdbeaux::sqlite3VtabImportErrmsg(p as *mut crate::src::headers::vdbeInt_h::Vdbe,  pVtab_3 as *mut crate::src::headers::sqlite3_h::sqlite3_vtab);
                            if sContext.isError > 0 as ::core::ffi::c_int {
                                crate::src::src::vdbeaux::sqlite3VdbeError(
                                    
                                    p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                    crate::src::src::vdbeapi::sqlite3_value_text(pDest_1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value as *mut crate::src::headers::vdbeInt_h::sqlite3_value),
                                );
                                rc = sContext.isError;
                            }
                            crate::src::src::vdbemem::sqlite3VdbeChangeEncoding(pDest_1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value, encoding as ::core::ffi::c_int);
                            updateMaxBlobsize(pDest_1);
                            if rc != 0 {
                                current_block = 9863799879598557851;
                                break;
                            }
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_VNext =>  {
                        let mut pVtab_4: *mut crate::src::headers::sqlite3_h::sqlite3_vtab =
                            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vtab>();
                        let mut pModule_4: *const crate::src::headers::sqlite3_h::sqlite3_module =
                            ::core::ptr::null::<crate::src::headers::sqlite3_h::sqlite3_module>();
                        let mut res_12: ::core::ffi::c_int = 0;
                        let mut pCur_5: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        pCur_5 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        if (*pCur_5).nullRow != 0 {
                            current_block = 5783071609795492627;
                        } else {
                            let __pCur_5_ref = unsafe { &*pCur_5 };
                            pVtab_4 = (*__pCur_5_ref.uc.pVCur).pVtab;
                            pModule_4 = (*pVtab_4).pModule;
                            rc = (*pModule_4).xNext.expect("non-null function pointer")(
                                __pCur_5_ref.uc.pVCur,
                            );
                            crate::src::src::vdbeaux::sqlite3VtabImportErrmsg(p as *mut crate::src::headers::vdbeInt_h::Vdbe,  pVtab_4 as *mut crate::src::headers::sqlite3_h::sqlite3_vtab);
                            if rc != 0 {
                                current_block = 9863799879598557851;
                                break;
                            }
                            res_12 = (*pModule_4).xEof.expect("non-null function pointer")(
                                __pCur_5_ref.uc.pVCur,
                            );
                            if res_12 == 0 {
                                current_block = 16391045501586817625;
                            } else {
                                current_block = 20147595251170673;
                            }
                        }
                    }
    crate::src::headers::opcodes_h::OP_VRename_1 =>  {
                        let mut pVtab_5: *mut crate::src::headers::sqlite3_h::sqlite3_vtab =
                            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vtab>();
                        let mut pName: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let mut isLegacy: ::core::ffi::c_int = 0;
                        isLegacy =
                            ((*db).flags & crate::src::headers::sqliteInt_h::SQLITE_LegacyAlter as crate::src::ext::rtree::rtree::u64_0) as ::core::ffi::c_int;
                        (*db).flags |= crate::src::headers::sqliteInt_h::SQLITE_LegacyAlter as crate::src::ext::rtree::rtree::u64_0;
                        pVtab_5 = (*(*pOp).p4.pVtab).pVtab;
                        pName = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        rc = crate::src::src::vdbemem::sqlite3VdbeChangeEncoding(pName as *mut crate::src::headers::vdbeInt_h::sqlite3_value, crate::src::headers::sqlite3_h::SQLITE_UTF8);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        rc = (*(*pVtab_5).pModule)
                            .xRename
                            .expect("non-null function pointer")(
                            pVtab_5, (*pName).z
                        );
                        if isLegacy == 0 as ::core::ffi::c_int {
                            (*db).flags &= !(crate::src::headers::sqliteInt_h::SQLITE_LegacyAlter as crate::src::ext::rtree::rtree::u64_0);
                        }
                        crate::src::src::vdbeaux::sqlite3VtabImportErrmsg(p as *mut crate::src::headers::vdbeInt_h::Vdbe,  pVtab_5 as *mut crate::src::headers::sqlite3_h::sqlite3_vtab);
                        __p_ref.set_expired(0 as crate::src::headers::sqliteInt_h::bft as crate::src::headers::sqliteInt_h::bft);
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_VUpdate_1 =>  {
                        let mut pVtab_6: *mut crate::src::headers::sqlite3_h::sqlite3_vtab =
                            ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vtab>();
                        let mut pModule_5: *const crate::src::headers::sqlite3_h::sqlite3_module =
                            ::core::ptr::null::<crate::src::headers::sqlite3_h::sqlite3_module>();
                        let mut nArg_0: ::core::ffi::c_int = 0;
                        let mut i_6: ::core::ffi::c_int = 0;
                        let mut rowid_0: crate::src::headers::sqlite3_h::sqlite_int64 = 0 as crate::src::headers::sqlite3_h::sqlite_int64;
                        let mut apArg_0: *mut *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<*mut crate::src::src::vdbe::Mem>();
                        let mut pX_0: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        if (*db).mallocFailed != 0 {
                            current_block = 2471432538116610086;
                            break;
                        }
                        pVtab_6 = (*(*pOp).p4.pVtab).pVtab;
                        if pVtab_6.is_null() || (*pVtab_6).pModule.is_null() {
                            rc = crate::src::headers::sqlite3_h::SQLITE_LOCKED;
                            current_block = 9863799879598557851;
                            break;
                        } else {
                            pModule_5 = (*pVtab_6).pModule;
                            nArg_0 = (*pOp).p2;
                            if (*pModule_5).xUpdate.is_some() {
                                let __db_ref = unsafe { &mut *db };
                                let mut vtabOnConflict: crate::src::ext::rtree::rtree::u8_0 = __db_ref.vtabOnConflict;
                                apArg_0 = __p_ref.apArg;
                                let __pOp_ref = unsafe { &*pOp };
                                pX_0 = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                                i_6 = 0 as ::core::ffi::c_int;
                                while i_6 < nArg_0 {
                                    let ref mut fresh16 = *apArg_0.offset(i_6 as isize);
                                    *fresh16 = pX_0;
                                    pX_0 = pX_0.offset(1);
                                    i_6 += 1;
                                }
                                __db_ref.vtabOnConflict = __pOp_ref.p5 as crate::src::ext::rtree::rtree::u8_0;
                                rc = (*pModule_5).xUpdate.expect("non-null function pointer")(
                                    pVtab_6,
                                    nArg_0,
                                    apArg_0 as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                    &raw mut rowid_0,
                                );
                                __db_ref.vtabOnConflict = vtabOnConflict;
                                crate::src::src::vdbeaux::sqlite3VtabImportErrmsg(p as *mut crate::src::headers::vdbeInt_h::Vdbe,  pVtab_6 as *mut crate::src::headers::sqlite3_h::sqlite3_vtab);
                                if rc == crate::src::headers::sqlite3_h::SQLITE_OK && __pOp_ref.p1 != 0 {
                                    __db_ref.lastRowid = rowid_0 as crate::src::ext::rtree::rtree::i64_0;
                                }
                                if rc & 0xff as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_CONSTRAINT
                                    && (*__pOp_ref.p4.pVtab).bConstraint as ::core::ffi::c_int != 0
                                {
                                    if __pOp_ref.p5 as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::OE_Ignore {
                                        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                                    } else {
                                        __p_ref.errorAction =
                                            (if __pOp_ref.p5 as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::OE_Replace {
                                                crate::src::headers::sqliteInt_h::OE_Abort
                                            } else {
                                                __pOp_ref.p5 as ::core::ffi::c_int
                                            }) as crate::src::ext::rtree::rtree::u8_0;
                                    }
                                } else {
                                    __p_ref.nChange += 1;
                                }
                                if rc != 0 {
                                    current_block = 9863799879598557851;
                                    break;
                                }
                            }
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Pagecount_1 =>  {
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        (*pOut).u.i =
                            crate::src::src::btree::sqlite3BtreeLastPage((*(*db).aDb.offset((*pOp).p1 as isize)).pBt)
                                as crate::src::ext::rtree::rtree::i64_0;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_MaxPgcnt_1 =>  {
                        let mut newMax: ::core::ffi::c_uint = 0;
                        let mut pBt_2: *mut crate::src::headers::btreeInt_h::Btree = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::Btree>();
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        pBt_2 = (*(*db).aDb.offset((*pOp).p1 as isize)).pBt;
                        newMax = 0 as ::core::ffi::c_uint;
                        if (*pOp).p3 != 0 {
                            newMax = crate::src::src::btree::sqlite3BtreeLastPage(pBt_2) as ::core::ffi::c_uint;
                            if newMax < (*pOp).p3 as ::core::ffi::c_uint {
                                newMax = (*pOp).p3 as ::core::ffi::c_uint;
                            }
                        }
                        (*pOut).u.i = crate::src::src::btree::sqlite3BtreeMaxPageCount(pBt_2, newMax as crate::src::src::pager::Pgno) as crate::src::ext::rtree::rtree::i64_0;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_PureFunc =>  {
                        current_block = 13540359916536463501;
                    }
    crate::src::headers::opcodes_h::OP_Function =>  {
                        current_block = 13540359916536463501;
                    }
    crate::src::headers::opcodes_h::OP_ClrSubtype_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        (*pIn1).flags =
                            ((*pIn1).flags as ::core::ffi::c_int & !crate::src::headers::vdbeInt_h::MEM_Subtype) as crate::src::fts5::u16_0;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_GetSubtype_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pOut = aMem.offset((*pOp).p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pIn1).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Subtype != 0 {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetInt64(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value, (*pIn1).eSubtype as crate::src::ext::rtree::rtree::i64_0);
                        } else {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_SetSubtype_1 =>  {
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pOut = aMem.offset((*pOp).p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pIn1).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Null != 0 {
                            (*pOut).flags =
                                ((*pOut).flags as ::core::ffi::c_int & !crate::src::headers::vdbeInt_h::MEM_Subtype) as crate::src::fts5::u16_0;
                        } else {
                            let __pOut_ref = unsafe { &mut *pOut };
                            __pOut_ref.flags =
                                (__pOut_ref.flags as ::core::ffi::c_int | crate::src::headers::vdbeInt_h::MEM_Subtype) as crate::src::fts5::u16_0;
                            __pOut_ref.eSubtype = ((*pIn1).u.i & 0xff as crate::src::ext::rtree::rtree::i64_0) as crate::src::ext::rtree::rtree::u8_0;
                        }
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_FilterAdd_1 =>  {
                        let mut h: crate::src::ext::rtree::rtree::u64_0 = 0;
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        h = filterHash(aMem, pOp);
                        h = h.wrapping_rem(((*pIn1).n * 8 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::u64_0);
                        let ref mut fresh18 =
                            *(*pIn1).z.offset(h.wrapping_div(8 as crate::src::ext::rtree::rtree::u64_0) as isize);
                        *fresh18 = (*fresh18 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << (h & 7 as crate::src::ext::rtree::rtree::u64_0))
                            as ::core::ffi::c_char;
                        current_block = 5783071609795492627;
                    }
    crate::src::headers::opcodes_h::OP_Filter_1 =>  {
                        let mut h_0: crate::src::ext::rtree::rtree::u64_0 = 0;
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        h_0 = filterHash(aMem, pOp);
                        h_0 = h_0.wrapping_rem(((*pIn1).n * 8 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::u64_0);
                        if *(*pIn1).z.offset(h_0.wrapping_div(8 as crate::src::ext::rtree::rtree::u64_0) as isize)
                            as ::core::ffi::c_int
                            & (1 as ::core::ffi::c_int) << (h_0 & 7 as crate::src::ext::rtree::rtree::u64_0)
                            == 0 as ::core::ffi::c_int
                        {
                            __p_ref.aCounter[crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_FILTER_HIT as usize] = __p_ref.aCounter
                                [crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_FILTER_HIT as usize]
                                .wrapping_add(1);
                            current_block = 9512719473022792396;
                        } else {
                            __p_ref.aCounter[crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_FILTER_MISS as usize] = __p_ref.aCounter
                                [crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_FILTER_MISS as usize]
                                .wrapping_add(1);
                            current_block = 5783071609795492627;
                        }
                    }
    crate::src::headers::opcodes_h::OP_Trace | crate::src::headers::opcodes_h::OP_Init =>  {
                        let mut i_8: ::core::ffi::c_int = 0;
                        let mut zTrace: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        if (*db).mTrace as ::core::ffi::c_int
                            & (crate::src::headers::sqlite3_h::SQLITE_TRACE_STMT | crate::src::headers::sqliteInt_h::SQLITE_TRACE_LEGACY)
                            != 0 as ::core::ffi::c_int
                            && __p_ref.minWriteFileFormat as ::core::ffi::c_int
                                != 254 as ::core::ffi::c_int
                            && {
                                zTrace = if !(*pOp).p4.z.is_null() {
                                    (*pOp).p4.z
                                } else {
                                    __p_ref.zSql
                                };
                                !zTrace.is_null()
                            }
                        {
                            if (*db).mTrace as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::SQLITE_TRACE_LEGACY != 0 {
                                let mut z_1: *mut ::core::ffi::c_char =
                                    crate::src::src::vdbetrace::sqlite3VdbeExpandSql(p as *mut crate::src::headers::vdbeInt_h::Vdbe, zTrace);
                                (*db).trace.xLegacy.expect("non-null function pointer")(
                                    (*db).pTraceArg,
                                    z_1,
                                );
                                crate::src::src::malloc::sqlite3_free(z_1 as *mut ::core::ffi::c_void);
                            } else if (*db).nVdbeExec > 1 as ::core::ffi::c_int {
                                let mut z_2: *mut ::core::ffi::c_char = crate::src::src::printf::sqlite3MPrintf(
                                    
                                    db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                    b"-- %s\0" as *const u8 as *const ::core::ffi::c_char,
                                    zTrace,
                                );
                                (*db).trace.xV2.expect("non-null function pointer")(
                                    crate::src::headers::sqlite3_h::SQLITE_TRACE_STMT as crate::src::ext::rtree::rtree::u32_0,
                                    (*db).pTraceArg,
                                    p as *mut ::core::ffi::c_void,
                                    z_2 as *mut ::core::ffi::c_void,
                                );
                                crate::src::src::malloc::sqlite3DbFree(db as *mut crate::src::headers::sqliteInt_h::sqlite3, z_2 as *mut ::core::ffi::c_void);
                            } else {
                                (*db).trace.xV2.expect("non-null function pointer")(
                                    crate::src::headers::sqlite3_h::SQLITE_TRACE_STMT as crate::src::ext::rtree::rtree::u32_0,
                                    (*db).pTraceArg,
                                    p as *mut ::core::ffi::c_void,
                                    zTrace as *mut ::core::ffi::c_void,
                                );
                            }
                        }
                        if (*pOp).p1 >= crate::src::src::global::sqlite3Config.iOnceResetThreshold {
                            if (*pOp).opcode as ::core::ffi::c_int == crate::src::headers::opcodes_h::OP_Trace {
                                current_block = 5783071609795492627;
                            } else {
                                i_8 = 1 as ::core::ffi::c_int;
                                while i_8 < __p_ref.nOp {
                                    if (*__p_ref.aOp.offset(i_8 as isize)).opcode as ::core::ffi::c_int
                                        == crate::src::headers::opcodes_h::OP_Once
                                    {
                                        (*__p_ref.aOp.offset(i_8 as isize)).p1 =
                                            0 as ::core::ffi::c_int;
                                    }
                                    i_8 += 1;
                                }
                                (*pOp).p1 = 0 as ::core::ffi::c_int;
                                current_block = 13408672579463531032;
                            }
                        } else {
                            current_block = 13408672579463531032;
                        }
                        match current_block {
                            5783071609795492627 => {}
                            _ => {
                                (*pOp).p1 += 1;
                                __p_ref.aCounter[crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_RUN as usize] =
                                    __p_ref.aCounter[crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_RUN as usize].wrapping_add(1);
                                current_block = 9512719473022792396;
                            }
                        }
                    }
    crate::src::headers::opcodes_h::OP_Permutation_1 | _ =>  {
                        current_block = 5783071609795492627;
                    }
}
                match current_block {
                    10435735846551762309 => {
                        let mut pFrame: *mut crate::src::headers::vdbeInt_h::VdbeFrame = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeFrame>();
                        let mut pcx: ::core::ffi::c_int = 0;
                        if !__p_ref.pFrame.is_null() && (*pOp).p1 == crate::src::headers::sqlite3_h::SQLITE_OK {
                            pFrame = __p_ref.pFrame;
                            __p_ref.pFrame = (*pFrame).pParent;
                            __p_ref.nFrame -= 1;
                            crate::src::src::vdbeaux::sqlite3VdbeSetChanges(db as *mut crate::src::headers::sqliteInt_h::sqlite3, __p_ref.nChange);
                            pcx = crate::src::src::vdbeaux::sqlite3VdbeFrameRestore(pFrame as *mut crate::src::headers::vdbeInt_h::VdbeFrame);
                            if (*pOp).p2 == crate::src::headers::sqliteInt_h::OE_Ignore {
                                pcx = (*__p_ref.aOp.offset(pcx as isize)).p2 - 1 as ::core::ffi::c_int;
                            }
                            aOp = __p_ref.aOp;
                            aMem = __p_ref.aMem;
                            pOp = aOp.offset(pcx as isize) as *mut crate::src::headers::vdbeInt_h::Op;
                        } else {
                            __p_ref.rc = (*pOp).p1;
                            __p_ref.errorAction = (*pOp).p2 as crate::src::ext::rtree::rtree::u8_0;
                            if __p_ref.rc != 0 {
                                let __pOp_ref = unsafe { &mut *pOp };
                                if __pOp_ref.p3 > 0 as ::core::ffi::c_int
                                    && __pOp_ref.p4type as ::core::ffi::c_int == crate::src::src::vdbe::P4_NOTUSED
                                {
                                    let mut zErr: *const ::core::ffi::c_char =
                                        ::core::ptr::null::<::core::ffi::c_char>();
                                    zErr = crate::src::src::vdbemem::sqlite3ValueText(
                                        
                                        aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::headers::vdbeInt_h::sqlite3_value as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                        crate::src::headers::sqlite3_h::SQLITE_UTF8 as crate::src::ext::rtree::rtree::u8_0,
                                    )
                                        as *const ::core::ffi::c_char;
                                    crate::src::src::vdbeaux::sqlite3VdbeError(
                                        
                                        p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                        zErr,
                                    );
                                } else if __pOp_ref.p5 != 0 {
                                    static mut azType: [*const ::core::ffi::c_char; 4] = [
                                        b"NOT NULL\0" as *const u8 as *const ::core::ffi::c_char,
                                        b"UNIQUE\0" as *const u8 as *const ::core::ffi::c_char,
                                        b"CHECK\0" as *const u8 as *const ::core::ffi::c_char,
                                        b"FOREIGN KEY\0" as *const u8 as *const ::core::ffi::c_char,
                                    ];
                                    crate::src::src::vdbeaux::sqlite3VdbeError(
                                        
                                        p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                        b"%s constraint failed\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        azType[(__pOp_ref.p5 as ::core::ffi::c_int
                                            - 1 as ::core::ffi::c_int)
                                            as usize],
                                    );
                                    if !__pOp_ref.p4.z.is_null() {
                                        __p_ref.zErrMsg = crate::src::src::printf::sqlite3MPrintf(
                                            
                                            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                            b"%z: %s\0" as *const u8 as *const ::core::ffi::c_char,
                                            __p_ref.zErrMsg,
                                            __pOp_ref.p4.z,
                                        );
                                    }
                                } else {
                                    crate::src::src::vdbeaux::sqlite3VdbeError(
                                        
                                        p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                        __pOp_ref.p4.z,
                                    );
                                }
                                sqlite3VdbeLogAbort(p, __pOp_ref.p1, pOp, aOp);
                            }
                            rc = crate::src::src::vdbeaux::sqlite3VdbeHalt(p as *mut crate::src::headers::vdbeInt_h::Vdbe);
                            if rc == crate::src::headers::sqlite3_h::SQLITE_BUSY {
                                __p_ref.rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
                            } else {
                                rc = if __p_ref.rc != 0 {
                                    crate::src::headers::sqlite3_h::SQLITE_ERROR
                                } else {
                                    crate::src::headers::sqlite3_h::SQLITE_DONE
                                };
                            }
                            current_block = 10408586827809538586;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
                    13540359916536463501 => {
                        let mut i_7: ::core::ffi::c_int = 0;
                        let mut pCtx_1: *mut crate::src::headers::vdbeInt_h::sqlite3_context =
                            ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_context>();
                        pCtx_1 = (*pOp).p4.pCtx;
                        pOut = aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pCtx_1).pOut != pOut {
                            let __pCtx_1_ref = unsafe { &mut *pCtx_1 };
                            __pCtx_1_ref.pVdbe = p;
                            __pCtx_1_ref.pOut = pOut;
                            __pCtx_1_ref.enc = encoding;
                            i_7 = __pCtx_1_ref.argc as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
                            while i_7 >= 0 as ::core::ffi::c_int {
                                let ref mut fresh17 = *(&raw mut __pCtx_1_ref.argv
                                    as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value)
                                    .offset(i_7 as isize);
                                *fresh17 = aMem.offset(((*pOp).p2 + i_7) as isize) as *mut crate::src::src::vdbe::Mem
                                    as *mut crate::src::headers::vdbeInt_h::sqlite3_value;
                                i_7 -= 1;
                            }
                        }
                        (*pOut).flags =
                            ((*pOut).flags as ::core::ffi::c_int & !(crate::src::headers::vdbeInt_h::MEM_TypeMask | crate::src::headers::vdbeInt_h::MEM_Zero)
                                | 0x1 as ::core::ffi::c_int) as crate::src::fts5::u16_0;
                        Some(
                            (*(*pCtx_1).pFunc)
                                .xSFunc
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            pCtx_1,
                            (*pCtx_1).argc as ::core::ffi::c_int,
                            &raw mut (*pCtx_1).argv as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        );
                        if (*pCtx_1).isError != 0 {
                            let __pCtx_1_ref = unsafe { &mut *pCtx_1 };
                            if __pCtx_1_ref.isError > 0 as ::core::ffi::c_int {
                                crate::src::src::vdbeaux::sqlite3VdbeError(
                                    
                                    p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                    crate::src::src::vdbeapi::sqlite3_value_text(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value as *mut crate::src::headers::vdbeInt_h::sqlite3_value),
                                );
                                rc = __pCtx_1_ref.isError;
                            }
                            crate::src::src::vdbeaux::sqlite3VdbeDeleteAuxData(
                                
                                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                
                                &raw mut __p_ref.pAuxData as *mut _ as *mut *mut crate::src::headers::vdbeInt_h::AuxData,
                                __pCtx_1_ref.iOp,
                                (*pOp).p1,
                            );
                            __pCtx_1_ref.isError = 0 as ::core::ffi::c_int;
                            if rc != 0 {
                                current_block = 9863799879598557851;
                                break;
                            }
                        }
                        updateMaxBlobsize(pOut);
                        current_block = 5783071609795492627;
                    }
                    1931008622201330627 => {
                        let mut i_3: ::core::ffi::c_int = 0;
                        let mut pCtx_0: *mut crate::src::headers::vdbeInt_h::sqlite3_context =
                            ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::sqlite3_context>();
                        let mut pMem_1: *mut crate::src::src::vdbe::Mem = ::core::ptr::null_mut::<crate::src::src::vdbe::Mem>();
                        let __pOp_ref = unsafe { &*pOp };
                        pCtx_0 = __pOp_ref.p4.pCtx;
                        pMem_1 = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        if (*pCtx_0).pMem != pMem_1 {
                            (*pCtx_0).pMem = pMem_1;
                            i_3 = (*pCtx_0).argc as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
                            while i_3 >= 0 as ::core::ffi::c_int {
                                let ref mut fresh14 = *(&raw mut (*pCtx_0).argv
                                    as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value)
                                    .offset(i_3 as isize);
                                *fresh14 = aMem.offset((__pOp_ref.p2 + i_3) as isize) as *mut crate::src::src::vdbe::Mem
                                    as *mut crate::src::headers::vdbeInt_h::sqlite3_value;
                                i_3 -= 1;
                            }
                        }
                        (*pMem_1).n += 1;
                        if __pOp_ref.p1 != 0 {
                            let __pCtx_0_ref = unsafe { &mut *pCtx_0 };
                            (*__pCtx_0_ref.pFunc)
                                .xInverse
                                .expect("non-null function pointer")(
                                pCtx_0,
                                __pCtx_0_ref.argc as ::core::ffi::c_int,
                                &raw mut __pCtx_0_ref.argv as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            );
                        } else {
                            let __pCtx_0_ref = unsafe { &mut *pCtx_0 };
                            (*__pCtx_0_ref.pFunc)
                                .xSFunc
                                .expect("non-null function pointer")(
                                pCtx_0,
                                __pCtx_0_ref.argc as ::core::ffi::c_int,
                                &raw mut __pCtx_0_ref.argv as *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            );
                        }
                        if (*pCtx_0).isError != 0 {
                            let __pCtx_0_ref = unsafe { &mut *pCtx_0 };
                            if __pCtx_0_ref.isError > 0 as ::core::ffi::c_int {
                                crate::src::src::vdbeaux::sqlite3VdbeError(
                                    
                                    p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                                    crate::src::src::vdbeapi::sqlite3_value_text(__pCtx_0_ref.pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value as *mut crate::src::headers::vdbeInt_h::sqlite3_value),
                                );
                                rc = __pCtx_0_ref.isError;
                            }
                            if __pCtx_0_ref.skipFlag != 0 {
                                i_3 = (*pOp.offset(-(1 as ::core::ffi::c_int) as isize)).p1;
                                if i_3 != 0 {
                                    crate::src::src::vdbemem::sqlite3VdbeMemSetInt64(
                                        
                                        aMem.offset(i_3 as isize) as *mut crate::src::src::vdbe::Mem as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                                        1 as crate::src::ext::rtree::rtree::i64_0,
                                    );
                                }
                                __pCtx_0_ref.skipFlag = 0 as crate::src::ext::rtree::rtree::u8_0;
                            }
                            crate::src::src::vdbemem::sqlite3VdbeMemRelease(__pCtx_0_ref.pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                            (*__pCtx_0_ref.pOut).flags = crate::src::headers::vdbeInt_h::MEM_Null as crate::src::fts5::u16_0;
                            __pCtx_0_ref.isError = 0 as ::core::ffi::c_int;
                            if rc != 0 {
                                current_block = 9863799879598557851;
                                break;
                            }
                            current_block = 5783071609795492627;
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
                    2251003667842051127 => {
                        let mut pC_25: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut pTabCur: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut rowid: crate::src::ext::rtree::rtree::i64_0 = 0;
                        pC_25 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        rc = crate::src::src::vdbeaux::sqlite3VdbeCursorRestore(pC_25 as *mut crate::src::headers::vdbeInt_h::VdbeCursor);
                        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                            current_block = 9863799879598557851;
                            break;
                        }
                        if (*pC_25).nullRow == 0 {
                            rowid = 0 as crate::src::ext::rtree::rtree::i64_0;
                            rc = crate::src::src::vdbeaux::sqlite3VdbeIdxRowid(db as *mut crate::src::headers::sqliteInt_h::sqlite3, (*pC_25).uc.pCursor, &raw mut rowid);
                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                current_block = 9863799879598557851;
                                break;
                            }
                            if (*pOp).opcode as ::core::ffi::c_int == crate::src::headers::opcodes_h::OP_DeferredSeek {
                                pTabCur = *__p_ref.apCsr.offset((*pOp).p3 as isize);
                                (*pTabCur).nullRow = 0 as crate::src::ext::rtree::rtree::u8_0;
                                (*pTabCur).movetoTarget = rowid;
                                (*pTabCur).deferredMoveto = 1 as crate::src::ext::rtree::rtree::u8_0;
                                (*pTabCur).cacheStatus = crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                                (*pTabCur).ub.aAltMap = (*pOp).p4.ai;
                                (*pTabCur).pAltCursor = pC_25;
                            } else {
                                pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                                (*pOut).u.i = rowid;
                            }
                        } else {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetNull(aMem.offset((*pOp).p2 as isize) as *mut crate::src::src::vdbe::Mem as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        }
                        current_block = 5783071609795492627;
                    }
                    13527485019907976489 => {
                        (*pC_21).cacheStatus = crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                            (*pC_21).nullRow = 0 as crate::src::ext::rtree::rtree::u8_0;
                            __p_ref.aCounter[(*pOp).p5 as usize] =
                                __p_ref.aCounter[(*pOp).p5 as usize].wrapping_add(1);
                            sqlite3_search_count += 1;
                            current_block = 16391045501586817625;
                        } else {
                            if rc != crate::src::headers::sqlite3_h::SQLITE_DONE {
                                current_block = 9863799879598557851;
                                break;
                            }
                            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                            (*pC_21).nullRow = 1 as crate::src::ext::rtree::rtree::u8_0;
                            current_block = 20147595251170673;
                        }
                    }
                    3082952251283703534 => {
                        let mut pC_19: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut pCrsr_5: *mut crate::src::headers::btreeInt_h::BtCursor = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>();
                        let mut res_7: ::core::ffi::c_int = 0;
                        pC_19 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        res_7 = 1 as ::core::ffi::c_int;
                        if (*pC_19).eCurType as ::core::ffi::c_int == crate::src::headers::vdbeInt_h::CURTYPE_SORTER {
                            rc = crate::src::src::vdbesort::sqlite3VdbeSorterRewind(pC_19 as *const crate::src::headers::vdbeInt_h::VdbeCursor, &raw mut res_7);
                        } else {
                            let __pC_19_ref = unsafe { &mut *pC_19 };
                            pCrsr_5 = __pC_19_ref.uc.pCursor;
                            rc = crate::src::src::btree::sqlite3BtreeFirst(pCrsr_5, &raw mut res_7);
                            __pC_19_ref.deferredMoveto = 0 as crate::src::ext::rtree::rtree::u8_0;
                            __pC_19_ref.cacheStatus = crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                        }
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        (*pC_19).nullRow = res_7 as crate::src::ext::rtree::rtree::u8_0;
                        if (*pOp).p2 > 0 as ::core::ffi::c_int {
                            if res_7 != 0 {
                                current_block = 9512719473022792396;
                            } else {
                                current_block = 5783071609795492627;
                            }
                        } else {
                            current_block = 5783071609795492627;
                        }
                    }
                    3859792399008177139 => {
                        let mut pC_17: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut pCrsr_3: *mut crate::src::headers::btreeInt_h::BtCursor = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>();
                        let mut res_5: ::core::ffi::c_int = 0;
                        pC_17 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        pCrsr_3 = (*pC_17).uc.pCursor;
                        res_5 = 0 as ::core::ffi::c_int;
                        if (*pOp).opcode as ::core::ffi::c_int == crate::src::headers::opcodes_h::OP_SeekEnd {
                            (*pC_17).seekResult = -(1 as ::core::ffi::c_int);
                            if crate::src::src::btree::sqlite3BtreeCursorIsValidNN(pCrsr_3) != 0 {
                                current_block = 5783071609795492627;
                            } else {
                                current_block = 5427770127893779035;
                            }
                        } else {
                            current_block = 5427770127893779035;
                        }
                        match current_block {
                            5783071609795492627 => {}
                            _ => {
                                rc = crate::src::src::btree::sqlite3BtreeLast(pCrsr_3, &raw mut res_5);
                                let __pC_17_ref = unsafe { &mut *pC_17 };
                                __pC_17_ref.nullRow = res_5 as crate::src::ext::rtree::rtree::u8_0;
                                __pC_17_ref.deferredMoveto = 0 as crate::src::ext::rtree::rtree::u8_0;
                                __pC_17_ref.cacheStatus = crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                                if rc != 0 {
                                    current_block = 9863799879598557851;
                                    break;
                                }
                                if (*pOp).p2 > 0 as ::core::ffi::c_int {
                                    if res_5 != 0 {
                                        current_block = 9512719473022792396;
                                    } else {
                                        current_block = 5783071609795492627;
                                    }
                                } else {
                                    current_block = 5783071609795492627;
                                }
                            }
                        }
                    }
                    15776502115119204583 => {
                        pIn3 = aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        iKey_0 = (*pIn3).u.i as crate::src::ext::rtree::rtree::u64_0;
                        current_block = 8490332201604051012;
                    }
                    4855994676461051050 => {
                        let mut pCx_0: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut pKeyInfo_1: *mut crate::src::headers::sqliteInt_h::KeyInfo = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::KeyInfo>();
                        static mut vfsFlags: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OPEN_READWRITE
                            | crate::src::headers::sqlite3_h::SQLITE_OPEN_CREATE
                            | crate::src::headers::sqlite3_h::SQLITE_OPEN_EXCLUSIVE
                            | crate::src::headers::sqlite3_h::SQLITE_OPEN_DELETEONCLOSE
                            | crate::src::headers::sqlite3_h::SQLITE_OPEN_TRANSIENT_DB;
                        let __pOp_ref = unsafe { &mut *pOp };
                        if __pOp_ref.p3 > 0 as ::core::ffi::c_int {
                            (*aMem.offset(__pOp_ref.p3 as isize)).n = 0 as ::core::ffi::c_int;
                            let ref mut fresh8 = (*aMem.offset(__pOp_ref.p3 as isize)).z;
                            *fresh8 = b"\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        pCx_0 = *__p_ref.apCsr.offset(__pOp_ref.p1 as isize);
                        if !pCx_0.is_null()
                            && (*pCx_0).noReuse() == 0
                            && __pOp_ref.p2 <= (*pCx_0).nField as ::core::ffi::c_int
                        {
                            let __pCx_0_ref = unsafe { &mut *pCx_0 };
                            __pCx_0_ref.seqCount = 0 as crate::src::ext::rtree::rtree::i64_0;
                            __pCx_0_ref.cacheStatus = crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                            rc = crate::src::src::btree::sqlite3BtreeClearTable(
                                __pCx_0_ref.ub.pBtx,
                                __pCx_0_ref.pgnoRoot as ::core::ffi::c_int,
                                ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::i64_0>(),
                            );
                        } else {
                            pCx_0 = allocateCursor(p, __pOp_ref.p1, __pOp_ref.p2, crate::src::headers::vdbeInt_h::CURTYPE_BTREE as crate::src::ext::rtree::rtree::u8_0);
                            if pCx_0.is_null() {
                                current_block = 2471432538116610086;
                                break;
                            }
                            (*pCx_0).set_isEphemeral(1 as crate::src::headers::vdbeInt_h::Bool as crate::src::headers::vdbeInt_h::Bool);
                            rc = crate::src::src::btree::sqlite3BtreeOpen(
                                
                                (*db).pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                
                                db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                                &raw mut (*pCx_0).ub.pBtx,
                                crate::src::src::btree::BTREE_OMIT_JOURNAL | crate::src::src::btree::BTREE_SINGLE | __pOp_ref.p5 as ::core::ffi::c_int,
                                vfsFlags,
                            );
                            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                rc = crate::src::src::btree::sqlite3BtreeBeginTrans(
                                    (*pCx_0).ub.pBtx,
                                    1 as ::core::ffi::c_int,
                                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                );
                                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                    pKeyInfo_1 = __pOp_ref.p4.pKeyInfo;
                                    (*pCx_0).pKeyInfo = pKeyInfo_1;
                                    if !(*pCx_0).pKeyInfo.is_null() {
                                        let __pCx_0_ref = unsafe { &mut *pCx_0 };
                                        rc = crate::src::src::btree::sqlite3BtreeCreateTable(
                                            __pCx_0_ref.ub.pBtx,
                                            &raw mut __pCx_0_ref.pgnoRoot,
                                            crate::src::src::btree::BTREE_BLOBKEY | __pOp_ref.p5 as ::core::ffi::c_int,
                                        );
                                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                            rc = crate::src::src::btree::sqlite3BtreeCursor(
                                                __pCx_0_ref.ub.pBtx,
                                                __pCx_0_ref.pgnoRoot,
                                                crate::src::src::btree::BTREE_WRCSR,
                                                
                                                pKeyInfo_1 as *mut crate::src::headers::sqliteInt_h::KeyInfo as *mut crate::src::headers::sqliteInt_h::KeyInfo,
                                                __pCx_0_ref.uc.pCursor,
                                            );
                                        }
                                        __pCx_0_ref.isTable = 0 as crate::src::ext::rtree::rtree::u8_0;
                                    } else {
                                        let __pCx_0_ref = unsafe { &mut *pCx_0 };
                                        __pCx_0_ref.pgnoRoot = crate::src::headers::sqliteInt_h::SCHEMA_ROOT as crate::src::src::pager::Pgno;
                                        rc = crate::src::src::btree::sqlite3BtreeCursor(
                                            __pCx_0_ref.ub.pBtx,
                                            crate::src::headers::sqliteInt_h::SCHEMA_ROOT as crate::src::src::pager::Pgno,
                                            crate::src::src::btree::BTREE_WRCSR,
                                            
                                            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::KeyInfo>() as
    *mut crate::src::headers::sqliteInt_h::KeyInfo,
                                            __pCx_0_ref.uc.pCursor,
                                        );
                                        __pCx_0_ref.isTable = 1 as crate::src::ext::rtree::rtree::u8_0;
                                    }
                                }
                                (*pCx_0).set_isOrdered(
                                    (__pOp_ref.p5 as ::core::ffi::c_int != crate::src::src::btree::BTREE_UNORDERED)
                                        as ::core::ffi::c_int
                                        as crate::src::headers::vdbeInt_h::Bool as crate::src::headers::vdbeInt_h::Bool,
                                );
                                if rc != 0 {
                                    crate::src::src::btree::sqlite3BtreeClose((*pCx_0).ub.pBtx);
                                    let ref mut fresh9 = *__p_ref.apCsr.offset(__pOp_ref.p1 as isize);
                                    *fresh9 = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                                }
                            }
                        }
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        (*pCx_0).nullRow = 1 as crate::src::ext::rtree::rtree::u8_0;
                        current_block = 5783071609795492627;
                    }
                    6098721512207810182 => {
                        if __p_ref.expired() as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                            rc = crate::src::headers::sqlite3_h::SQLITE_ABORT_ROLLBACK_1;
                            current_block = 9863799879598557851;
                            break;
                        } else {
                            nField_0 = 0 as ::core::ffi::c_int;
                            pKeyInfo_0 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::KeyInfo>();
                            let __pOp_ref = unsafe { &*pOp };
                            p2_2 = __pOp_ref.p2 as crate::src::ext::rtree::rtree::u32_0;
                            iDb_0 = __pOp_ref.p3;
                            pDb_1 = (*db).aDb.offset(iDb_0 as isize) as *mut crate::src::headers::sqliteInt_h::Db;
                            pX = (*pDb_1).pBt;
                            if __pOp_ref.opcode as ::core::ffi::c_int == crate::src::headers::opcodes_h::OP_OpenWrite {
                                wrFlag = crate::src::src::btree::BTREE_WRCSR
                                    | __pOp_ref.p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::OPFLAG_FORDELETE;
                                if ((*(*pDb_1).pSchema).file_format as ::core::ffi::c_int)
                                    < __p_ref.minWriteFileFormat as ::core::ffi::c_int
                                {
                                    __p_ref.minWriteFileFormat = (*(*pDb_1).pSchema).file_format;
                                }
                                if __pOp_ref.p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::OPFLAG_P2ISREG != 0 {
                                    pIn2 = aMem.offset(p2_2 as isize) as *mut crate::src::src::vdbe::Mem;
                                    crate::src::src::vdbemem::sqlite3VdbeMemIntegerify(pIn2 as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                                    p2_2 = (*pIn2).u.i as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u32_0;
                                }
                            } else {
                                wrFlag = 0 as ::core::ffi::c_int;
                            }
                            if __pOp_ref.p4type as ::core::ffi::c_int == crate::src::src::vdbe::P4_KEYINFO {
                                pKeyInfo_0 = __pOp_ref.p4.pKeyInfo;
                                nField_0 = (*pKeyInfo_0).nAllField as ::core::ffi::c_int;
                            } else if __pOp_ref.p4type as ::core::ffi::c_int == crate::src::src::vdbe::P4_INT32 {
                                nField_0 = __pOp_ref.p4.i;
                            }
                            pCur = allocateCursor(p, __pOp_ref.p1, nField_0, crate::src::headers::vdbeInt_h::CURTYPE_BTREE as crate::src::ext::rtree::rtree::u8_0);
                            if pCur.is_null() {
                                current_block = 2471432538116610086;
                                break;
                            }
                            (*pCur).iDb = iDb_0 as crate::src::headers::sqliteInt_h::i8_0;
                            (*pCur).nullRow = 1 as crate::src::ext::rtree::rtree::u8_0;
                            (*pCur).set_isOrdered(1 as crate::src::headers::vdbeInt_h::Bool as crate::src::headers::vdbeInt_h::Bool);
                            (*pCur).pgnoRoot = p2_2 as crate::src::src::pager::Pgno;
                            rc = crate::src::src::btree::sqlite3BtreeCursor(
                                pX,
                                p2_2 as crate::src::src::pager::Pgno,
                                wrFlag,
                                
                                pKeyInfo_0 as *mut crate::src::headers::sqliteInt_h::KeyInfo as *mut crate::src::headers::sqliteInt_h::KeyInfo,
                                (*pCur).uc.pCursor,
                            );
                            (*pCur).pKeyInfo = pKeyInfo_0;
                            (*pCur).isTable = (__pOp_ref.p4type as ::core::ffi::c_int != crate::src::src::vdbe::P4_KEYINFO)
                                as ::core::ffi::c_int
                                as crate::src::ext::rtree::rtree::u8_0;
                        }
                        current_block = 2376416726268577300;
                    }
                    10856689599696957676 => {
                        let mut v1: ::core::ffi::c_int = 0;
                        let mut v2: ::core::ffi::c_int = 0;
                        let __pOp_ref = unsafe { &*pOp };
                        v1 = crate::src::src::vdbemem::sqlite3VdbeBooleanValue(
                            
                            aMem.offset(__pOp_ref.p1 as isize) as *mut crate::src::src::vdbe::Mem as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            2 as ::core::ffi::c_int,
                        );
                        v2 = crate::src::src::vdbemem::sqlite3VdbeBooleanValue(
                            
                            aMem.offset(__pOp_ref.p2 as isize) as *mut crate::src::src::vdbe::Mem as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            2 as ::core::ffi::c_int,
                        );
                        if __pOp_ref.opcode as ::core::ffi::c_int == crate::src::headers::opcodes_h::OP_And {
                            static mut and_logic: [::core::ffi::c_uchar; 9] = [
                                0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                            ];
                            v1 = and_logic[(v1 * 3 as ::core::ffi::c_int + v2) as usize]
                                as ::core::ffi::c_int;
                        } else {
                            static mut or_logic: [::core::ffi::c_uchar; 9] = [
                                0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                            ];
                            v1 = or_logic[(v1 * 3 as ::core::ffi::c_int + v2) as usize]
                                as ::core::ffi::c_int;
                        }
                        pOut = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        if v1 == 2 as ::core::ffi::c_int {
                            (*pOut).flags = ((*pOut).flags as ::core::ffi::c_int
                                & !(crate::src::headers::vdbeInt_h::MEM_TypeMask | crate::src::headers::vdbeInt_h::MEM_Zero)
                                | 0x1 as ::core::ffi::c_int)
                                as crate::src::fts5::u16_0;
                        } else {
                            let __pOut_ref = unsafe { &mut *pOut };
                            __pOut_ref.u.i = v1 as crate::src::ext::rtree::rtree::i64_0;
                            __pOut_ref.flags = (__pOut_ref.flags as ::core::ffi::c_int
                                & !(crate::src::headers::vdbeInt_h::MEM_TypeMask | crate::src::headers::vdbeInt_h::MEM_Zero)
                                | 0x4 as ::core::ffi::c_int)
                                as crate::src::fts5::u16_0;
                        }
                        current_block = 5783071609795492627;
                    }
                    16937825661756021828 => {
                        pOut = out2Prerelease(p, pOp as *mut crate::src::src::vdbe::VdbeOp);
                        (*pOut).flags = (crate::src::headers::vdbeInt_h::MEM_Str | crate::src::headers::vdbeInt_h::MEM_Static | crate::src::headers::vdbeInt_h::MEM_Term) as crate::src::fts5::u16_0;
                        let __pOp_ref = unsafe { &*pOp };
                        (*pOut).z = __pOp_ref.p4.z;
                        (*pOut).n = __pOp_ref.p1;
                        (*pOut).enc = encoding;
                        updateMaxBlobsize(pOut);
                        if __pOp_ref.p3 > 0 as ::core::ffi::c_int {
                            pIn3 = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                            if (*pIn3).u.i == __pOp_ref.p5 as crate::src::ext::rtree::rtree::i64_0 {
                                (*pOut).flags = (crate::src::headers::vdbeInt_h::MEM_Blob | crate::src::headers::vdbeInt_h::MEM_Static | crate::src::headers::vdbeInt_h::MEM_Term) as crate::src::fts5::u16_0;
                            }
                        }
                        current_block = 5783071609795492627;
                    }
                    16448975459825856284 => {
                        current_block = 5899965325122513769;
                    }
                    18047165599808473703 => {
                        current_block = 8472080195360146252;
                    }
                    8915789739336007503 => {
                        current_block = 9183375779473790601;
                    }
                    2622337694718008658 => {
                        current_block = 17378708304186809879;
                    }
                    8122660736604414621 => {
                        current_block = 10733086697120658930;
                    }
                    13800034716966053010 => {
                        current_block = 970849332036924775;
                    }
                    _ => {}
                }
                match current_block {
                    8490332201604051012 => {
                        pC_8 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        pCrsr_1 = (*pC_8).uc.pCursor;
                        res_2 = 0 as ::core::ffi::c_int;
                        rc = crate::src::src::btree::sqlite3BtreeTableMoveto(
                            pCrsr_1,
                            iKey_0 as crate::src::ext::rtree::rtree::i64_0,
                            0 as ::core::ffi::c_int,
                            &raw mut res_2,
                        );
                        (*pC_8).movetoTarget = iKey_0 as crate::src::ext::rtree::rtree::i64_0;
                        (*pC_8).nullRow = 0 as crate::src::ext::rtree::rtree::u8_0;
                        (*pC_8).cacheStatus = crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                        (*pC_8).deferredMoveto = 0 as crate::src::ext::rtree::rtree::u8_0;
                        (*pC_8).seekResult = res_2;
                        if res_2 != 0 as ::core::ffi::c_int {
                            if (*pOp).p2 == 0 as ::core::ffi::c_int {
                                rc = crate::src::src::main::sqlite3CorruptError(5545 as ::core::ffi::c_int);
                                current_block = 13115802299706855851;
                            } else {
                                current_block = 9512719473022792396;
                            }
                        } else {
                            current_block = 13115802299706855851;
                        }
                        match current_block {
                            9512719473022792396 => {}
                            _ => {
                                if rc != 0 {
                                    current_block = 9863799879598557851;
                                    break;
                                }
                                current_block = 5783071609795492627;
                            }
                        }
                    }
                    2376416726268577300 => {
                        crate::src::src::btree::sqlite3BtreeCursorHintFlags(
                            (*pCur).uc.pCursor,
                            ((*pOp).p5 as ::core::ffi::c_int & (crate::src::headers::sqliteInt_h::OPFLAG_BULKCSR | crate::src::headers::sqliteInt_h::OPFLAG_SEEKEQ))
                                as ::core::ffi::c_uint,
                        );
                        if rc != 0 {
                            current_block = 9863799879598557851;
                            break;
                        }
                        current_block = 5783071609795492627;
                    }
                    5899965325122513769 => {
                        current_block = 16916022866774761203;
                    }
                    8472080195360146252 => {
                        current_block = 1895500680468589183;
                    }
                    9183375779473790601 => {
                        current_block = 3502568586026288603;
                    }
                    17378708304186809879 => {
                        current_block = 6051598589340159089;
                    }
                    10733086697120658930 => {
                        current_block = 18125716024132132232;
                    }
                    970849332036924775 => {
                        current_block = 6380214163151454549;
                    }
                    16391045501586817625 => {
                        pOp = aOp.offset(((*pOp).p2 - 1 as ::core::ffi::c_int) as isize) as *mut crate::src::headers::vdbeInt_h::Op;
                        current_block = 20147595251170673;
                    }
                    _ => {}
                }
                match current_block {
                    20147595251170673 => {
                        if (*((&raw mut (*db).u1.isInterrupted) as *mut std::sync::atomic::AtomicI32)).load(std::sync::atomic::Ordering::Relaxed)
                            != 0
                        {
                            current_block = 9771092749923633615;
                            break;
                        }
                        while nVmStep >= nProgressLimit && (*db).xProgress.is_some() {
                            let __db_ref = unsafe { &mut *db };
                            nProgressLimit =
                                nProgressLimit.wrapping_add(__db_ref.nProgressOps as crate::src::ext::rtree::rtree::u64_0);
                            if !(__db_ref.xProgress.expect("non-null function pointer")(
                                __db_ref.pProgressArg,
                            ) != 0)
                            {
                                continue;
                            }
                            nProgressLimit = crate::src::headers::sqliteInt_h::LARGEST_UINT64;
                            rc = crate::src::headers::sqlite3_h::SQLITE_INTERRUPT;
                            current_block = 9863799879598557851;
                            break 's_109;
                        }
                        current_block = 5783071609795492627;
                    }
                    6380214163151454549 => {
                        let mut pC_27: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut res_10: ::core::ffi::c_int = 0;
                        let mut r_3: crate::src::headers::sqliteInt_h::UnpackedRecord = unsafe { ::core::mem::zeroed() };
                        let __pOp_ref = unsafe { &*pOp };
                        pC_27 = *__p_ref.apCsr.offset(__pOp_ref.p1 as isize);
                        r_3.pKeyInfo = (*pC_27).pKeyInfo;
                        r_3.nField = __pOp_ref.p4.i as crate::src::fts5::u16_0;
                        if (__pOp_ref.opcode as ::core::ffi::c_int) < crate::src::headers::opcodes_h::OP_IdxLT {
                            r_3.default_rc = -(1 as ::core::ffi::c_int) as crate::src::headers::sqliteInt_h::i8_0;
                        } else {
                            r_3.default_rc = 0 as crate::src::headers::sqliteInt_h::i8_0;
                        }
                        r_3.aMem = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        let mut nCellKey: crate::src::ext::rtree::rtree::i64_0 = 0 as crate::src::ext::rtree::rtree::i64_0;
                        let mut pCur_1: *mut crate::src::headers::btreeInt_h::BtCursor = ::core::ptr::null_mut::<crate::src::headers::btreeInt_h::BtCursor>();
                        let mut m: crate::src::src::vdbe::Mem = crate::src::headers::vdbeInt_h::sqlite3_value {
    u:  crate::src::headers::vdbeInt_h::MemValue { r:  0. },
    z:  ::core::ptr::null_mut::<::core::ffi::c_char>(),
    n:  0,
    flags:  0,
    enc:  0,
    eSubtype:  0,
    db:  ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>(),
    szMalloc:  0,
    uTemp:  0,
    zMalloc:  ::core::ptr::null_mut::<::core::ffi::c_char>(),
    xDel:  None,
};
                        pCur_1 = (*pC_27).uc.pCursor;
                        nCellKey = crate::src::src::btree::sqlite3BtreePayloadSize(pCur_1) as crate::src::ext::rtree::rtree::i64_0;
                        if nCellKey <= 0 as crate::src::ext::rtree::rtree::i64_0 || nCellKey > 0x7fffffff as crate::src::ext::rtree::rtree::i64_0 {
                            rc = crate::src::src::main::sqlite3CorruptError(6868 as ::core::ffi::c_int);
                            current_block = 9863799879598557851;
                            break;
                        } else {
                            crate::src::src::vdbemem::sqlite3VdbeMemInit(&raw mut m as *mut _ as *mut crate::src::headers::vdbeInt_h::sqlite3_value,  db as *mut crate::src::headers::sqliteInt_h::sqlite3, 0 as crate::src::fts5::u16_0);
                            rc = crate::src::src::vdbemem::sqlite3VdbeMemFromBtreeZeroOffset(
                                pCur_1,
                                nCellKey as crate::src::ext::rtree::rtree::u32_0,
                                
                                &raw mut m as *mut _ as *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                            );
                            if rc != 0 {
                                current_block = 9863799879598557851;
                                break;
                            }
                            res_10 = crate::src::src::vdbeaux::sqlite3VdbeRecordCompareWithSkip(
                                m.n,
                                m.z as *const ::core::ffi::c_void,
                                
                                &raw mut r_3 as *mut _ as *mut crate::src::headers::sqliteInt_h::UnpackedRecord,
                                0 as ::core::ffi::c_int,
                            );
                            crate::src::src::vdbemem::sqlite3VdbeMemReleaseMalloc(&raw mut m as *mut _ as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                            if __pOp_ref.opcode as ::core::ffi::c_int & 1 as ::core::ffi::c_int
                                == crate::src::headers::opcodes_h::OP_IdxLT & 1 as ::core::ffi::c_int
                            {
                                res_10 = -res_10;
                            } else {
                                res_10 += 1;
                            }
                            if res_10 > 0 as ::core::ffi::c_int {
                                current_block = 9512719473022792396;
                            } else {
                                current_block = 5783071609795492627;
                            }
                        }
                    }
                    18125716024132132232 => {
                        let mut alreadyExists: ::core::ffi::c_int = 0;
                        let mut ii_0: ::core::ffi::c_int = 0;
                        let mut pC_7: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut pIdxKey: *mut crate::src::headers::sqliteInt_h::UnpackedRecord =
                            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::UnpackedRecord>();
                        let mut r_1: crate::src::headers::sqliteInt_h::UnpackedRecord = unsafe { ::core::mem::zeroed() };
                        let __pOp_ref = unsafe { &*pOp };
                        if __pOp_ref.opcode as ::core::ffi::c_int != crate::src::headers::opcodes_h::OP_NoConflict {
                            sqlite3_found_count += 1;
                        }
                        pC_7 = *__p_ref.apCsr.offset(__pOp_ref.p1 as isize);
                        r_1.aMem = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        r_1.nField = __pOp_ref.p4.i as crate::src::fts5::u16_0;
                        if r_1.nField as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                            let __pC_7_ref = unsafe { &mut *pC_7 };
                            r_1.pKeyInfo = __pC_7_ref.pKeyInfo;
                            r_1.default_rc = 0 as crate::src::headers::sqliteInt_h::i8_0;
                            rc = crate::src::src::btree::sqlite3BtreeIndexMoveto(
                                __pC_7_ref.uc.pCursor,
                                
                                &raw mut r_1 as *mut _ as *mut crate::src::headers::sqliteInt_h::UnpackedRecord,
                                &raw mut __pC_7_ref.seekResult,
                            );
                        } else {
                            rc = if (*r_1.aMem).flags as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Zero != 0 {
                                crate::src::src::vdbemem::sqlite3VdbeMemExpandBlob(r_1.aMem as *mut crate::src::headers::vdbeInt_h::sqlite3_value)
                            } else {
                                0 as ::core::ffi::c_int
                            };
                            if rc != 0 {
                                current_block = 2471432538116610086;
                                break;
                            }
                            let __pC_7_ref = unsafe { &mut *pC_7 };
                            pIdxKey =  crate::src::src::vdbeaux::sqlite3VdbeAllocUnpackedRecord(__pC_7_ref.pKeyInfo as *mut crate::src::headers::sqliteInt_h::KeyInfo) as
    *mut crate::src::headers::sqliteInt_h::UnpackedRecord;
                            if pIdxKey.is_null() {
                                current_block = 2471432538116610086;
                                break;
                            }
                            crate::src::src::vdbeaux::sqlite3VdbeRecordUnpack(
                                (*r_1.aMem).n,
                                (*r_1.aMem).z as *const ::core::ffi::c_void,
                                
                                pIdxKey as *mut crate::src::headers::sqliteInt_h::UnpackedRecord,
                            );
                            (*pIdxKey).default_rc = 0 as crate::src::headers::sqliteInt_h::i8_0;
                            rc = crate::src::src::btree::sqlite3BtreeIndexMoveto(
                                __pC_7_ref.uc.pCursor,
                                
                                pIdxKey as *mut crate::src::headers::sqliteInt_h::UnpackedRecord,
                                &raw mut __pC_7_ref.seekResult,
                            );
                            crate::src::src::malloc::sqlite3DbFreeNN(db as *mut crate::src::headers::sqliteInt_h::sqlite3, pIdxKey as *mut ::core::ffi::c_void);
                        }
                        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                            current_block = 9863799879598557851;
                            break;
                        }
                        alreadyExists =
                            ((*pC_7).seekResult == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                        (*pC_7).nullRow = (1 as ::core::ffi::c_int - alreadyExists) as crate::src::ext::rtree::rtree::u8_0;
                        (*pC_7).deferredMoveto = 0 as crate::src::ext::rtree::rtree::u8_0;
                        (*pC_7).cacheStatus = crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                        if __pOp_ref.opcode as ::core::ffi::c_int == crate::src::headers::opcodes_h::OP_Found {
                            if alreadyExists != 0 {
                                current_block = 9512719473022792396;
                            } else {
                                current_block = 5783071609795492627;
                            }
                        } else if alreadyExists == 0 {
                            current_block = 9512719473022792396;
                        } else {
                            if __pOp_ref.opcode as ::core::ffi::c_int == crate::src::headers::opcodes_h::OP_NoConflict {
                                ii_0 = 0 as ::core::ffi::c_int;
                                loop {
                                    if !(ii_0 < r_1.nField as ::core::ffi::c_int) {
                                        current_block = 17613180196767804613;
                                        break;
                                    }
                                    if (*r_1.aMem.offset(ii_0 as isize)).flags as ::core::ffi::c_int
                                        & crate::src::headers::vdbeInt_h::MEM_Null
                                        != 0
                                    {
                                        current_block = 9512719473022792396;
                                        break;
                                    }
                                    ii_0 += 1;
                                }
                            } else {
                                current_block = 17613180196767804613;
                            }
                            match current_block {
                                9512719473022792396 => {}
                                _ => {
                                    if __pOp_ref.opcode as ::core::ffi::c_int == crate::src::headers::opcodes_h::OP_IfNoHope {
                                        (*pC_7).seekHit = __pOp_ref.p4.i as crate::src::fts5::u16_0;
                                    }
                                    current_block = 5783071609795492627;
                                }
                            }
                        }
                    }
                    6051598589340159089 => {
                        let mut res_0: ::core::ffi::c_int = 0;
                        let mut oc: ::core::ffi::c_int = 0;
                        let mut pC_3: *mut crate::src::headers::vdbeInt_h::VdbeCursor = ::core::ptr::null_mut::<crate::src::headers::vdbeInt_h::VdbeCursor>();
                        let mut r: crate::src::headers::sqliteInt_h::UnpackedRecord = unsafe { ::core::mem::zeroed() };
                        let mut nField_1: ::core::ffi::c_int = 0;
                        let mut iKey: crate::src::ext::rtree::rtree::i64_0 = 0;
                        let mut eqOnly: ::core::ffi::c_int = 0;
                        pC_3 = *__p_ref.apCsr.offset((*pOp).p1 as isize);
                        oc = (*pOp).opcode as ::core::ffi::c_int;
                        eqOnly = 0 as ::core::ffi::c_int;
                        (*pC_3).nullRow = 0 as crate::src::ext::rtree::rtree::u8_0;
                        (*pC_3).deferredMoveto = 0 as crate::src::ext::rtree::rtree::u8_0;
                        (*pC_3).cacheStatus = crate::src::headers::vdbeInt_h::CACHE_STALE as crate::src::ext::rtree::rtree::u32_0;
                        if (*pC_3).isTable != 0 {
                            let mut flags3_0: crate::src::fts5::u16_0 = 0;
                            let mut newType: crate::src::fts5::u16_0 = 0;
                            pIn3 = aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem;
                            flags3_0 = (*pIn3).flags;
                            if flags3_0 as ::core::ffi::c_int
                                & (crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_IntReal | crate::src::headers::vdbeInt_h::MEM_Str)
                                == crate::src::headers::vdbeInt_h::MEM_Str
                            {
                                applyNumericAffinity(pIn3, 0 as ::core::ffi::c_int);
                            }
                            iKey = crate::src::src::vdbemem::sqlite3VdbeIntValue(pIn3 as *const crate::src::headers::vdbeInt_h::sqlite3_value);
                            newType = (*pIn3).flags;
                            (*pIn3).flags = flags3_0;
                            if newType as ::core::ffi::c_int & (crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_IntReal)
                                == 0 as ::core::ffi::c_int
                            {
                                let mut c_1: ::core::ffi::c_int = 0;
                                if newType as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Real
                                    == 0 as ::core::ffi::c_int
                                {
                                    if newType as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Null != 0
                                        || oc >= crate::src::headers::opcodes_h::OP_SeekGE
                                    {
                                        current_block = 9512719473022792396;
                                    } else {
                                        rc = crate::src::src::btree::sqlite3BtreeLast((*pC_3).uc.pCursor, &raw mut res_0);
                                        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                            current_block = 9863799879598557851;
                                            break;
                                        }
                                        current_block = 8651270701404509569;
                                    }
                                } else {
                                    c_1 = crate::src::src::vdbeaux::sqlite3IntFloatCompare(iKey, (*pIn3).u.r);
                                    if c_1 > 0 as ::core::ffi::c_int {
                                        if oc & 0x1 as ::core::ffi::c_int
                                            == crate::src::headers::opcodes_h::OP_SeekGT & 0x1 as ::core::ffi::c_int
                                        {
                                            oc -= 1;
                                        }
                                    } else if c_1 < 0 as ::core::ffi::c_int {
                                        if oc & 0x1 as ::core::ffi::c_int
                                            == crate::src::headers::opcodes_h::OP_SeekLT & 0x1 as ::core::ffi::c_int
                                        {
                                            oc += 1;
                                        }
                                    }
                                    current_block = 30898356577063691;
                                }
                            } else {
                                current_block = 30898356577063691;
                            }
                            match current_block {
                                8651270701404509569 => {}
                                9512719473022792396 => {}
                                _ => {
                                    rc = crate::src::src::btree::sqlite3BtreeTableMoveto(
                                        (*pC_3).uc.pCursor,
                                        iKey as crate::src::ext::rtree::rtree::u64_0 as crate::src::ext::rtree::rtree::i64_0,
                                        0 as ::core::ffi::c_int,
                                        &raw mut res_0,
                                    );
                                    (*pC_3).movetoTarget = iKey;
                                    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                        current_block = 9863799879598557851;
                                        break;
                                    }
                                    current_block = 1453880179885609999;
                                }
                            }
                        } else {
                            let __pC_3_ref = unsafe { &*pC_3 };
                            if crate::src::src::btree::sqlite3BtreeCursorHasHint(
                                __pC_3_ref.uc.pCursor,
                                crate::src::src::btree::BTREE_SEEK_EQ as ::core::ffi::c_uint,
                            ) != 0
                            {
                                eqOnly = 1 as ::core::ffi::c_int;
                            }
                            nField_1 = (*pOp).p4.i;
                            r.pKeyInfo = __pC_3_ref.pKeyInfo;
                            r.nField = nField_1 as crate::src::fts5::u16_0;
                            r.default_rc = (if 1 as ::core::ffi::c_int & oc - crate::src::headers::opcodes_h::OP_SeekLT != 0 {
                                -(1 as ::core::ffi::c_int)
                            } else {
                                1 as ::core::ffi::c_int
                            }) as crate::src::headers::sqliteInt_h::i8_0;
                            r.aMem = aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem;
                            r.eqSeen = 0 as crate::src::ext::rtree::rtree::u8_0;
                            rc = crate::src::src::btree::sqlite3BtreeIndexMoveto(
                                __pC_3_ref.uc.pCursor,
                                
                                &raw mut r as *mut _ as *mut crate::src::headers::sqliteInt_h::UnpackedRecord,
                                &raw mut res_0,
                            );
                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                current_block = 9863799879598557851;
                                break;
                            }
                            if eqOnly != 0
                                && r.eqSeen as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            {
                                current_block = 8651270701404509569;
                            } else {
                                current_block = 1453880179885609999;
                            }
                        }
                        match current_block {
                            9512719473022792396 => {}
                            _ => {
                                match current_block {
                                    1453880179885609999 => {
                                        sqlite3_search_count += 1;
                                        if oc >= crate::src::headers::opcodes_h::OP_SeekGE {
                                            if res_0 < 0 as ::core::ffi::c_int
                                                || res_0 == 0 as ::core::ffi::c_int
                                                    && oc == crate::src::headers::opcodes_h::OP_SeekGT
                                            {
                                                res_0 = 0 as ::core::ffi::c_int;
                                                rc = crate::src::src::btree::sqlite3BtreeNext(
                                                    (*pC_3).uc.pCursor,
                                                    0 as ::core::ffi::c_int,
                                                );
                                                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                                    if !(rc == crate::src::headers::sqlite3_h::SQLITE_DONE) {
                                                        current_block = 9863799879598557851;
                                                        break;
                                                    }
                                                    rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                                                    res_0 = 1 as ::core::ffi::c_int;
                                                }
                                            } else {
                                                res_0 = 0 as ::core::ffi::c_int;
                                            }
                                        } else if res_0 > 0 as ::core::ffi::c_int
                                            || res_0 == 0 as ::core::ffi::c_int && oc == crate::src::headers::opcodes_h::OP_SeekLT
                                        {
                                            res_0 = 0 as ::core::ffi::c_int;
                                            rc = crate::src::src::btree::sqlite3BtreePrevious(
                                                (*pC_3).uc.pCursor,
                                                0 as ::core::ffi::c_int,
                                            );
                                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                                if !(rc == crate::src::headers::sqlite3_h::SQLITE_DONE) {
                                                    current_block = 9863799879598557851;
                                                    break;
                                                }
                                                rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                                                res_0 = 1 as ::core::ffi::c_int;
                                            }
                                        } else {
                                            res_0 = crate::src::src::btree::sqlite3BtreeEof((*pC_3).uc.pCursor);
                                        }
                                    }
                                    _ => {}
                                }
                                if res_0 != 0 {
                                    current_block = 9512719473022792396;
                                } else {
                                    if eqOnly != 0 {
                                        pOp = pOp.offset(1);
                                    }
                                    current_block = 5783071609795492627;
                                }
                            }
                        }
                    }
                    1895500680468589183 => {
                        let mut iA_0: crate::src::ext::rtree::rtree::i64_0 = 0;
                        let mut uA: crate::src::ext::rtree::rtree::u64_0 = 0;
                        let mut iB_0: crate::src::ext::rtree::rtree::i64_0 = 0;
                        let mut op: crate::src::ext::rtree::rtree::u8_0 = 0;
                        let __pOp_ref = unsafe { &*pOp };
                        pIn1 = aMem.offset(__pOp_ref.p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pIn2 = aMem.offset(__pOp_ref.p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        pOut = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        if ((*pIn1).flags as ::core::ffi::c_int
                            | (*pIn2).flags as ::core::ffi::c_int)
                            & crate::src::headers::vdbeInt_h::MEM_Null
                            != 0
                        {
                            crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                        } else {
                            iA_0 = crate::src::src::vdbemem::sqlite3VdbeIntValue(pIn2 as *const crate::src::headers::vdbeInt_h::sqlite3_value);
                            iB_0 = crate::src::src::vdbemem::sqlite3VdbeIntValue(pIn1 as *const crate::src::headers::vdbeInt_h::sqlite3_value);
                            op = __pOp_ref.opcode;
                            if op as ::core::ffi::c_int == crate::src::headers::opcodes_h::OP_BitAnd {
                                iA_0 &= iB_0;
                            } else if op as ::core::ffi::c_int == crate::src::headers::opcodes_h::OP_BitOr {
                                iA_0 |= iB_0;
                            } else if iB_0 != 0 as crate::src::ext::rtree::rtree::i64_0 {
                                if iB_0 < 0 as crate::src::ext::rtree::rtree::i64_0 {
                                    op = (2 as ::core::ffi::c_int * crate::src::headers::opcodes_h::OP_ShiftLeft
                                        + 1 as ::core::ffi::c_int
                                        - op as ::core::ffi::c_int)
                                        as crate::src::ext::rtree::rtree::u8_0;
                                    iB_0 = if iB_0 > -(64 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0 {
                                        -iB_0
                                    } else {
                                        64 as crate::src::ext::rtree::rtree::i64_0
                                    };
                                }
                                if iB_0 >= 64 as crate::src::ext::rtree::rtree::i64_0 {
                                    iA_0 = (if iA_0 >= 0 as crate::src::ext::rtree::rtree::i64_0
                                        || op as ::core::ffi::c_int == crate::src::headers::opcodes_h::OP_ShiftLeft
                                    {
                                        0 as ::core::ffi::c_int
                                    } else {
                                        -(1 as ::core::ffi::c_int)
                                    }) as crate::src::ext::rtree::rtree::i64_0;
                                } else {
                                    ::core::ptr::copy_nonoverlapping(
                    &raw mut iA_0 as *const u8,
                    &raw mut uA as *mut u8,
                    ::core::mem::size_of::<crate::src::ext::rtree::rtree::u64_0>() as usize,
                );
                                    if op as ::core::ffi::c_int == crate::src::headers::opcodes_h::OP_ShiftLeft {
                                        uA <<= iB_0;
                                    } else {
                                        uA >>= iB_0;
                                        if iA_0 < 0 as crate::src::ext::rtree::rtree::i64_0 {
                                            uA |= ((0xffffffff as ::core::ffi::c_uint as crate::src::ext::rtree::rtree::u64_0)
                                                << 32 as ::core::ffi::c_int
                                                | 0xffffffff as crate::src::ext::rtree::rtree::u64_0)
                                                << 64 as crate::src::ext::rtree::rtree::i64_0 - iB_0;
                                        }
                                    }
                                    ::core::ptr::copy_nonoverlapping(
                    &raw mut uA as *const u8,
                    &raw mut iA_0 as *mut u8,
                    ::core::mem::size_of::<crate::src::ext::rtree::rtree::i64_0>() as usize,
                );
                                }
                            }
                            let __pOut_ref = unsafe { &mut *pOut };
                            __pOut_ref.u.i = iA_0;
                            __pOut_ref.flags = (__pOut_ref.flags as ::core::ffi::c_int
                                & !(crate::src::headers::vdbeInt_h::MEM_TypeMask | crate::src::headers::vdbeInt_h::MEM_Zero)
                                | 0x4 as ::core::ffi::c_int)
                                as crate::src::fts5::u16_0;
                        }
                        current_block = 5783071609795492627;
                    }
                    16916022866774761203 => {
                        current_block = 11475295656646479480;
                    }
                    3502568586026288603 => {
                        current_block = 7712297393629777289;
                    }
                    _ => {}
                }
                match current_block {
                    11475295656646479480 => {
                        let mut type1: crate::src::fts5::u16_0 = 0;
                        let mut type2: crate::src::fts5::u16_0 = 0;
                        let mut iA: crate::src::ext::rtree::rtree::i64_0 = 0;
                        let mut iB: crate::src::ext::rtree::rtree::i64_0 = 0;
                        let mut rA: ::core::ffi::c_double = 0.;
                        let mut rB: ::core::ffi::c_double = 0.;
                        let __pOp_ref = unsafe { &*pOp };
                        pIn1 = aMem.offset(__pOp_ref.p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        type1 = (*pIn1).flags;
                        pIn2 = aMem.offset(__pOp_ref.p2 as isize) as *mut crate::src::src::vdbe::Mem;
                        type2 = (*pIn2).flags;
                        pOut = aMem.offset(__pOp_ref.p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        if type1 as ::core::ffi::c_int & type2 as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Int
                            != 0 as ::core::ffi::c_int
                        {
                            current_block = 4899479028894616879;
                        } else if (type1 as ::core::ffi::c_int | type2 as ::core::ffi::c_int)
                            & crate::src::headers::vdbeInt_h::MEM_Null
                            != 0 as ::core::ffi::c_int
                        {
                            current_block = 11350077356682528340;
                        } else {
                            type1 = numericType(pIn1);
                            type2 = numericType(pIn2);
                            if type1 as ::core::ffi::c_int & type2 as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Int
                                != 0 as ::core::ffi::c_int
                            {
                                current_block = 4899479028894616879;
                            } else {
                                current_block = 16857357506424929049;
                            }
                        }
                        match current_block {
                            4899479028894616879 => {
                                iA = (*pIn1).u.i;
                                iB = (*pIn2).u.i;
                                match  __pOp_ref.opcode as ::core::ffi::c_int {
    crate::src::headers::opcodes_h::OP_Add =>  {
                                        if crate::src::src::util::sqlite3AddInt64(&raw mut iB, iA) != 0 {
                                            current_block = 16857357506424929049;
                                        } else {
                                            current_block = 14114736409816581360;
                                        }
                                    }
    crate::src::headers::opcodes_h::OP_Subtract_1 =>  {
                                        if crate::src::src::util::sqlite3SubInt64(&raw mut iB, iA) != 0 {
                                            current_block = 16857357506424929049;
                                        } else {
                                            current_block = 14114736409816581360;
                                        }
                                    }
    crate::src::headers::opcodes_h::OP_Multiply =>  {
                                        if crate::src::src::util::sqlite3MulInt64(&raw mut iB, iA) != 0 {
                                            current_block = 16857357506424929049;
                                        } else {
                                            current_block = 14114736409816581360;
                                        }
                                    }
    crate::src::headers::opcodes_h::OP_Divide =>  {
                                        if iA == 0 as crate::src::ext::rtree::rtree::i64_0 {
                                            current_block = 11350077356682528340;
                                        } else if iA == -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0
                                            && iB == crate::fts3Int_h::SMALLEST_INT64
                                        {
                                            current_block = 16857357506424929049;
                                        } else {
                                            iB /= iA;
                                            current_block = 14114736409816581360;
                                        }
                                    }
    _ =>  {
                                        if iA == 0 as crate::src::ext::rtree::rtree::i64_0 {
                                            current_block = 11350077356682528340;
                                        } else {
                                            if iA == -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0 {
                                                iA = 1 as crate::src::ext::rtree::rtree::i64_0;
                                            }
                                            iB %= iA;
                                            current_block = 14114736409816581360;
                                        }
                                    }
}
                                match current_block {
                                    11350077356682528340 => {}
                                    16857357506424929049 => {}
                                    _ => {
                                        let __pOut_ref = unsafe { &mut *pOut };
                                        __pOut_ref.u.i = iB;
                                        __pOut_ref.flags = (__pOut_ref.flags as ::core::ffi::c_int
                                            & !(crate::src::headers::vdbeInt_h::MEM_TypeMask | crate::src::headers::vdbeInt_h::MEM_Zero)
                                            | 0x4 as ::core::ffi::c_int)
                                            as crate::src::fts5::u16_0;
                                        current_block = 5783071609795492627;
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            5783071609795492627 => {}
                            _ => {
                                match current_block {
                                    16857357506424929049 => {
                                        rA = crate::src::src::vdbemem::sqlite3VdbeRealValue(pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                                        rB = crate::src::src::vdbemem::sqlite3VdbeRealValue(pIn2 as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                                        match  __pOp_ref.opcode as ::core::ffi::c_int {
    crate::src::headers::opcodes_h::OP_Add =>  {
                                                rB += rA;
                                                current_block = 7157913506381630964;
                                            }
    crate::src::headers::opcodes_h::OP_Subtract_1 =>  {
                                                rB -= rA;
                                                current_block = 7157913506381630964;
                                            }
    crate::src::headers::opcodes_h::OP_Multiply =>  {
                                                rB *= rA;
                                                current_block = 7157913506381630964;
                                            }
    crate::src::headers::opcodes_h::OP_Divide =>  {
                                                if rA
                                                    == 0 as ::core::ffi::c_int
                                                        as ::core::ffi::c_double
                                                {
                                                    current_block = 11350077356682528340;
                                                } else {
                                                    rB /= rA;
                                                    current_block = 7157913506381630964;
                                                }
                                            }
    _ =>  {
                                                iA = crate::src::src::vdbemem::sqlite3VdbeIntValue(pIn1 as *const crate::src::headers::vdbeInt_h::sqlite3_value);
                                                iB = crate::src::src::vdbemem::sqlite3VdbeIntValue(pIn2 as *const crate::src::headers::vdbeInt_h::sqlite3_value);
                                                if iA == 0 as crate::src::ext::rtree::rtree::i64_0 {
                                                    current_block = 11350077356682528340;
                                                } else {
                                                    if iA == -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0 {
                                                        iA = 1 as crate::src::ext::rtree::rtree::i64_0;
                                                    }
                                                    rB = (iB % iA) as ::core::ffi::c_double;
                                                    current_block = 7157913506381630964;
                                                }
                                            }
}
                                        match current_block {
                                            11350077356682528340 => {}
                                            _ => {
                                                if crate::src::src::util::sqlite3IsNaN(rB) != 0 {
                                                    current_block = 11350077356682528340;
                                                } else {
                                                    let __pOut_ref = unsafe { &mut *pOut };
                                                    __pOut_ref.u.r = rB;
                                                    __pOut_ref.flags = (__pOut_ref.flags
                                                        as ::core::ffi::c_int
                                                        & !(crate::src::headers::vdbeInt_h::MEM_TypeMask | crate::src::headers::vdbeInt_h::MEM_Zero)
                                                        | 0x8 as ::core::ffi::c_int)
                                                        as crate::src::fts5::u16_0;
                                                    current_block = 5783071609795492627;
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    5783071609795492627 => {}
                                    _ => {
                                        crate::src::src::vdbemem::sqlite3VdbeMemSetNull(pOut as *mut crate::src::headers::vdbeInt_h::sqlite3_value);
                                        current_block = 5783071609795492627;
                                    }
                                }
                            }
                        }
                    }
                    7712297393629777289 => {
                        current_block = 9598485154371622742;
                    }
                    _ => {}
                }
                match current_block {
                    9598485154371622742 => {
                        let mut res: ::core::ffi::c_int = 0;
                        let mut res2: ::core::ffi::c_int = 0;
                        let mut affinity: ::core::ffi::c_char = 0;
                        let mut flags1_0: crate::src::fts5::u16_0 = 0;
                        let mut flags3: crate::src::fts5::u16_0 = 0;
                        pIn1 = aMem.offset((*pOp).p1 as isize) as *mut crate::src::src::vdbe::Mem;
                        pIn3 = aMem.offset((*pOp).p3 as isize) as *mut crate::src::src::vdbe::Mem;
                        flags1_0 = (*pIn1).flags;
                        flags3 = (*pIn3).flags;
                        if flags1_0 as ::core::ffi::c_int & flags3 as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Int
                            != 0 as ::core::ffi::c_int
                        {
                            if (*pIn3).u.i > (*pIn1).u.i {
                                if *crate::src::src::global::sqlite3aGTb.offset((*pOp).opcode as isize) != 0 {
                                    current_block = 9512719473022792396;
                                } else {
                                    iCompare = 1 as ::core::ffi::c_int;
                                    current_block = 5783071609795492627;
                                }
                            } else if (*pIn3).u.i < (*pIn1).u.i {
                                if *crate::src::src::global::sqlite3aLTb.offset((*pOp).opcode as isize) != 0 {
                                    current_block = 9512719473022792396;
                                } else {
                                    iCompare = -(1 as ::core::ffi::c_int);
                                    current_block = 5783071609795492627;
                                }
                            } else if *crate::src::src::global::sqlite3aEQb.offset((*pOp).opcode as isize) != 0 {
                                current_block = 9512719473022792396;
                            } else {
                                iCompare = 0 as ::core::ffi::c_int;
                                current_block = 5783071609795492627;
                            }
                        } else {
                            if (flags1_0 as ::core::ffi::c_int | flags3 as ::core::ffi::c_int)
                                & crate::src::headers::vdbeInt_h::MEM_Null
                                != 0
                            {
                                if (*pOp).p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::SQLITE_NULLEQ != 0 {
                                    if flags1_0 as ::core::ffi::c_int
                                        & flags3 as ::core::ffi::c_int
                                        & crate::src::headers::vdbeInt_h::MEM_Null
                                        != 0 as ::core::ffi::c_int
                                        && flags3 as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Cleared
                                            == 0 as ::core::ffi::c_int
                                    {
                                        res = 0 as ::core::ffi::c_int;
                                    } else {
                                        res = if flags3 as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Null != 0 {
                                            -(1 as ::core::ffi::c_int)
                                        } else {
                                            1 as ::core::ffi::c_int
                                        };
                                    }
                                    current_block = 915695187752424879;
                                } else if (*pOp).p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::SQLITE_JUMPIFNULL != 0 {
                                    current_block = 9512719473022792396;
                                } else {
                                    iCompare = 1 as ::core::ffi::c_int;
                                    current_block = 5783071609795492627;
                                }
                            } else {
                                affinity = ((*pOp).p5 as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::SQLITE_AFF_MASK)
                                    as ::core::ffi::c_char;
                                if affinity as ::core::ffi::c_int >= crate::src::headers::sqliteInt_h::SQLITE_AFF_NUMERIC {
                                    if (flags1_0 as ::core::ffi::c_int
                                        | flags3 as ::core::ffi::c_int)
                                        & crate::src::headers::vdbeInt_h::MEM_Str
                                        != 0
                                    {
                                        if flags1_0 as ::core::ffi::c_int
                                            & (crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_IntReal | crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_Str)
                                            == crate::src::headers::vdbeInt_h::MEM_Str
                                        {
                                            applyNumericAffinity(pIn1, 0 as ::core::ffi::c_int);
                                            flags3 = (*pIn3).flags;
                                        }
                                        if flags3 as ::core::ffi::c_int
                                            & (crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_IntReal | crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_Str)
                                            == crate::src::headers::vdbeInt_h::MEM_Str
                                        {
                                            applyNumericAffinity(pIn3, 0 as ::core::ffi::c_int);
                                        }
                                    }
                                } else if affinity as ::core::ffi::c_int == crate::src::headers::sqliteInt_h::SQLITE_AFF_TEXT
                                    && (flags1_0 as ::core::ffi::c_int
                                        | flags3 as ::core::ffi::c_int)
                                        & crate::src::headers::vdbeInt_h::MEM_Str
                                        != 0 as ::core::ffi::c_int
                                {
                                    if flags1_0 as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Str
                                        != 0 as ::core::ffi::c_int
                                    {
                                        (*pIn1).flags = ((*pIn1).flags as ::core::ffi::c_int
                                            & !(crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_IntReal))
                                            as crate::src::fts5::u16_0;
                                    } else if flags1_0 as ::core::ffi::c_int
                                        & (crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_IntReal)
                                        != 0 as ::core::ffi::c_int
                                    {
                                        crate::src::src::vdbemem::sqlite3VdbeMemStringify(pIn1 as *mut crate::src::headers::vdbeInt_h::sqlite3_value, encoding, 1 as crate::src::ext::rtree::rtree::u8_0);
                                        flags1_0 = ((*pIn1).flags as ::core::ffi::c_int
                                            & !crate::src::headers::vdbeInt_h::MEM_TypeMask
                                            | flags1_0 as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_TypeMask)
                                            as crate::src::fts5::u16_0;
                                        if pIn1 == pIn3 {
                                            flags3 =
                                                (flags1_0 as ::core::ffi::c_int | crate::src::headers::vdbeInt_h::MEM_Str) as crate::src::fts5::u16_0;
                                        }
                                    }
                                    if flags3 as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_Str
                                        != 0 as ::core::ffi::c_int
                                    {
                                        (*pIn3).flags = ((*pIn3).flags as ::core::ffi::c_int
                                            & !(crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_IntReal))
                                            as crate::src::fts5::u16_0;
                                    } else if flags3 as ::core::ffi::c_int
                                        & (crate::src::headers::vdbeInt_h::MEM_Int | crate::src::headers::vdbeInt_h::MEM_Real | crate::src::headers::vdbeInt_h::MEM_IntReal)
                                        != 0 as ::core::ffi::c_int
                                    {
                                        crate::src::src::vdbemem::sqlite3VdbeMemStringify(pIn3 as *mut crate::src::headers::vdbeInt_h::sqlite3_value, encoding, 1 as crate::src::ext::rtree::rtree::u8_0);
                                        flags3 = ((*pIn3).flags as ::core::ffi::c_int
                                            & !crate::src::headers::vdbeInt_h::MEM_TypeMask
                                            | flags3 as ::core::ffi::c_int & crate::src::headers::vdbeInt_h::MEM_TypeMask)
                                            as crate::src::fts5::u16_0;
                                    }
                                }
                                res = crate::src::src::vdbeaux::sqlite3MemCompare(pIn3 as *const crate::src::headers::vdbeInt_h::sqlite3_value,  pIn1 as *const crate::src::headers::vdbeInt_h::sqlite3_value,  (*pOp).p4.pColl as *const crate::src::headers::sqliteInt_h::CollSeq);
                                current_block = 915695187752424879;
                            }
                            match current_block {
                                5783071609795492627 => {}
                                9512719473022792396 => {}
                                _ => {
                                    if res < 0 as ::core::ffi::c_int {
                                        res2 = *crate::src::src::global::sqlite3aLTb.offset((*pOp).opcode as isize)
                                            as ::core::ffi::c_int;
                                    } else if res == 0 as ::core::ffi::c_int {
                                        res2 = *crate::src::src::global::sqlite3aEQb.offset((*pOp).opcode as isize)
                                            as ::core::ffi::c_int;
                                    } else {
                                        res2 = *crate::src::src::global::sqlite3aGTb.offset((*pOp).opcode as isize)
                                            as ::core::ffi::c_int;
                                    }
                                    iCompare = res;
                                    (*pIn3).flags = flags3;
                                    (*pIn1).flags = flags1_0;
                                    if res2 != 0 {
                                        current_block = 9512719473022792396;
                                    } else {
                                        current_block = 5783071609795492627;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                match current_block {
                    9512719473022792396 => {
                        pOp = aOp.offset(((*pOp).p2 - 1 as ::core::ffi::c_int) as isize) as *mut crate::src::headers::vdbeInt_h::Op;
                    }
                    _ => {}
                }
                pOp = pOp.offset(1);
            }
            match current_block {
                9863799879598557851 => {}
                9771092749923633615 => {}
                10408586827809538586 => {}
                2471432538116610086 => {}
                _ => {
                    crate::src::src::vdbeaux::sqlite3VdbeError(
                        
                        p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                        b"string or blob too big\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    rc = crate::src::headers::sqlite3_h::SQLITE_TOOBIG;
                    current_block = 9863799879598557851;
                }
            }
        }
        match current_block {
            9863799879598557851 => {}
            10408586827809538586 => {}
            2471432538116610086 => {}
            _ => {
                rc = crate::src::headers::sqlite3_h::SQLITE_INTERRUPT;
                current_block = 9863799879598557851;
            }
        }
    }
    match current_block {
        2471432538116610086 => {
            crate::src::src::malloc::sqlite3OomFault(db as *mut crate::src::headers::sqliteInt_h::sqlite3);
            crate::src::src::vdbeaux::sqlite3VdbeError(
                
                p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
            );
            rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
            current_block = 9863799879598557851;
        }
        _ => {}
    }
    loop {
        match current_block {
            10408586827809538586 => {
                let __db_ref = unsafe { &mut *db };
                if !(nVmStep >= nProgressLimit && __db_ref.xProgress.is_some()) {
                    break;
                }
                nProgressLimit = nProgressLimit.wrapping_add(__db_ref.nProgressOps as crate::src::ext::rtree::rtree::u64_0);
                if !(__db_ref.xProgress.expect("non-null function pointer")(__db_ref.pProgressArg) != 0) {
                    current_block = 10408586827809538586;
                    continue;
                }
                nProgressLimit = crate::src::headers::sqliteInt_h::LARGEST_UINT64;
                rc = crate::src::headers::sqlite3_h::SQLITE_INTERRUPT;
                current_block = 9863799879598557851;
            }
            _ => {
                if (*db).mallocFailed != 0 {
                    rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
                } else if rc == crate::src::headers::sqlite3_h::SQLITE_IOERR_CORRUPTFS {
                    rc = crate::src::src::main::sqlite3CorruptError(9231 as ::core::ffi::c_int);
                }
                if __p_ref.zErrMsg.is_null() && rc != crate::src::headers::sqlite3_h::SQLITE_IOERR_NOMEM {
                    crate::src::src::vdbeaux::sqlite3VdbeError(
                        
                        p as *mut crate::src::headers::vdbeInt_h::Vdbe,
                        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                        crate::src::src::main::sqlite3ErrStr(rc),
                    );
                }
                __p_ref.rc = rc;
                crate::src::src::util::sqlite3SystemError(db as *mut crate::src::headers::sqliteInt_h::sqlite3, rc);
                sqlite3VdbeLogAbort(p, rc, pOp, aOp);
                if __p_ref.eVdbeState as ::core::ffi::c_int == crate::src::headers::vdbeInt_h::VDBE_RUN_STATE {
                    crate::src::src::vdbeaux::sqlite3VdbeHalt(p as *mut crate::src::headers::vdbeInt_h::Vdbe);
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_IOERR_NOMEM {
                    crate::src::src::malloc::sqlite3OomFault(db as *mut crate::src::headers::sqliteInt_h::sqlite3);
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_CORRUPT
                    && (*db).autoCommit as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    (*db).flags |= crate::src::headers::sqliteInt_h::SQLITE_CorruptRdOnly;
                }
                rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                if resetSchemaOnFault as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    crate::src::src::build::sqlite3ResetOneSchema(
                        
                        db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        resetSchemaOnFault as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
                    );
                }
                current_block = 10408586827809538586;
            }
        }
    }
    __p_ref.aCounter[crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_VM_STEP as usize] = __p_ref.aCounter
        [crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_VM_STEP as usize]
        .wrapping_add(nVmStep as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u32_0);
    if __p_ref.lockMask != 0 as crate::src::headers::sqliteInt_h::yDbMask {
        crate::src::src::vdbeaux::sqlite3VdbeLeave(p as *mut crate::src::headers::vdbeInt_h::Vdbe);
    }
    rc
}

pub const MAX_ROWID: crate::src::ext::rtree::rtree::i64_0 = ((0x7fffffff as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u64_0)
    << 32 as ::core::ffi::c_int
    | 0xffffffff as ::core::ffi::c_uint as crate::src::ext::rtree::rtree::u64_0) as crate::src::ext::rtree::rtree::i64_0;
