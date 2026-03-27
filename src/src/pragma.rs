












// =============== BEGIN pragma_h ================
pub const PragTyp_ANALYSIS_LIMIT:  ::core::ffi::c_int =  1;
    
    pub const PragTyp_HEADER_VALUE: ::core::ffi::c_int = 2;
    
    pub const PragTyp_AUTO_VACUUM: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    
    pub const PragTyp_FLAG: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    
    pub const PragTyp_BUSY_TIMEOUT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    
    pub const PragTyp_CACHE_SIZE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    
    pub const PragTyp_CACHE_SPILL: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    
    pub const PragTyp_CASE_SENSITIVE_LIKE: ::core::ffi::c_int = 8;
    
    pub const PragTyp_COLLATION_LIST: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
    
    pub const PragTyp_COMPILE_OPTIONS: ::core::ffi::c_int = 10;
    
    pub const PragTyp_DATABASE_LIST: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
    
    pub const PragTyp_DEFAULT_CACHE_SIZE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
    
    pub const PragTyp_ENCODING: ::core::ffi::c_int = 14;
    
    pub const PragTyp_FOREIGN_KEY_CHECK: ::core::ffi::c_int = 15;
    
    pub const PragTyp_FOREIGN_KEY_LIST: ::core::ffi::c_int = 16;
    
    pub const PragTyp_FUNCTION_LIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
    
    pub const PragTyp_HARD_HEAP_LIMIT: ::core::ffi::c_int = 18;
    
    pub const PragTyp_INCREMENTAL_VACUUM: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
    
    pub const PragTyp_INDEX_INFO: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
    
    pub const PragTyp_INDEX_LIST: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
    
    pub const PragTyp_INTEGRITY_CHECK: ::core::ffi::c_int = 22;
    
    pub const PragTyp_JOURNAL_MODE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
    
    pub const PragTyp_JOURNAL_SIZE_LIMIT: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
    
    pub const PragTyp_LOCKING_MODE: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
    
    pub const PragTyp_PAGE_COUNT: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
    
    pub const PragTyp_MMAP_SIZE: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
    
    pub const PragTyp_MODULE_LIST: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
    
    pub const PragTyp_OPTIMIZE: ::core::ffi::c_int = 30;
    
    pub const PragTyp_PAGE_SIZE: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
    
    pub const PragTyp_PRAGMA_LIST: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    
    pub const PragTyp_SECURE_DELETE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
    
    pub const PragTyp_SHRINK_MEMORY: ::core::ffi::c_int = 34;
    
    pub const PragTyp_SOFT_HEAP_LIMIT: ::core::ffi::c_int = 35;
    
    pub const PragTyp_SYNCHRONOUS: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
    
    pub const PragTyp_TABLE_INFO: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
    
    pub const PragTyp_TABLE_LIST: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
    
    pub const PragTyp_TEMP_STORE: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
    
    pub const PragTyp_TEMP_STORE_DIRECTORY: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
    
    pub const PragTyp_THREADS: ::core::ffi::c_int = 41;
    
    pub const PragTyp_WAL_AUTOCHECKPOINT: ::core::ffi::c_int = 42;
    
    pub const PragTyp_WAL_CHECKPOINT: ::core::ffi::c_int = 43;
    
    pub const PragTyp_LOCK_STATUS: ::core::ffi::c_int = 44;
    
    pub const PragFlg_NeedSchema: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    
    pub const PragFlg_NoColumns: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    
    pub const PragFlg_NoColumns1: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    
    pub const PragFlg_ReadOnly: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    
    pub const PragFlg_Result0: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    
    pub const PragFlg_Result1: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    
    pub const PragFlg_SchemaOpt: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
    
    pub const PragFlg_SchemaReq: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct PragmaName {
        pub zName: *const ::core::ffi::c_char,
        pub ePragTyp: crate::src::ext::rtree::rtree::u8_0,
        pub mPragFlg: crate::src::ext::rtree::rtree::u8_0,
        pub iPragCName: crate::src::ext::rtree::rtree::u8_0,
        pub nPragCName: crate::src::ext::rtree::rtree::u8_0,
        pub iArg: crate::src::ext::rtree::rtree::u64_0,
    }

pub mod pragma_h {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub static mut pragCName: [*const ::core::ffi::c_char; 57] = [
        b"id\0" as *const u8 as *const ::core::ffi::c_char,
        b"seq\0" as *const u8 as *const ::core::ffi::c_char,
        b"table\0" as *const u8 as *const ::core::ffi::c_char,
        b"from\0" as *const u8 as *const ::core::ffi::c_char,
        b"to\0" as *const u8 as *const ::core::ffi::c_char,
        b"on_update\0" as *const u8 as *const ::core::ffi::c_char,
        b"on_delete\0" as *const u8 as *const ::core::ffi::c_char,
        b"match\0" as *const u8 as *const ::core::ffi::c_char,
        b"cid\0" as *const u8 as *const ::core::ffi::c_char,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        b"type\0" as *const u8 as *const ::core::ffi::c_char,
        b"notnull\0" as *const u8 as *const ::core::ffi::c_char,
        b"dflt_value\0" as *const u8 as *const ::core::ffi::c_char,
        b"pk\0" as *const u8 as *const ::core::ffi::c_char,
        b"hidden\0" as *const u8 as *const ::core::ffi::c_char,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        b"builtin\0" as *const u8 as *const ::core::ffi::c_char,
        b"type\0" as *const u8 as *const ::core::ffi::c_char,
        b"enc\0" as *const u8 as *const ::core::ffi::c_char,
        b"narg\0" as *const u8 as *const ::core::ffi::c_char,
        b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        b"schema\0" as *const u8 as *const ::core::ffi::c_char,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        b"type\0" as *const u8 as *const ::core::ffi::c_char,
        b"ncol\0" as *const u8 as *const ::core::ffi::c_char,
        b"wr\0" as *const u8 as *const ::core::ffi::c_char,
        b"strict\0" as *const u8 as *const ::core::ffi::c_char,
        b"seqno\0" as *const u8 as *const ::core::ffi::c_char,
        b"cid\0" as *const u8 as *const ::core::ffi::c_char,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        b"desc\0" as *const u8 as *const ::core::ffi::c_char,
        b"coll\0" as *const u8 as *const ::core::ffi::c_char,
        b"key\0" as *const u8 as *const ::core::ffi::c_char,
        b"seq\0" as *const u8 as *const ::core::ffi::c_char,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        b"unique\0" as *const u8 as *const ::core::ffi::c_char,
        b"origin\0" as *const u8 as *const ::core::ffi::c_char,
        b"partial\0" as *const u8 as *const ::core::ffi::c_char,
        b"tbl\0" as *const u8 as *const ::core::ffi::c_char,
        b"idx\0" as *const u8 as *const ::core::ffi::c_char,
        b"wdth\0" as *const u8 as *const ::core::ffi::c_char,
        b"hght\0" as *const u8 as *const ::core::ffi::c_char,
        b"flgs\0" as *const u8 as *const ::core::ffi::c_char,
        b"table\0" as *const u8 as *const ::core::ffi::c_char,
        b"rowid\0" as *const u8 as *const ::core::ffi::c_char,
        b"parent\0" as *const u8 as *const ::core::ffi::c_char,
        b"fkid\0" as *const u8 as *const ::core::ffi::c_char,
        b"busy\0" as *const u8 as *const ::core::ffi::c_char,
        b"log\0" as *const u8 as *const ::core::ffi::c_char,
        b"checkpointed\0" as *const u8 as *const ::core::ffi::c_char,
        b"seq\0" as *const u8 as *const ::core::ffi::c_char,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        b"file\0" as *const u8 as *const ::core::ffi::c_char,
        b"database\0" as *const u8 as *const ::core::ffi::c_char,
        b"status\0" as *const u8 as *const ::core::ffi::c_char,
        b"cache_size\0" as *const u8 as *const ::core::ffi::c_char,
        b"timeout\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    
    pub static mut aPragmaName: [crate::src::src::pragma::PragmaName; 67] = [
        crate::src::src::pragma::PragmaName {
    zName:  b"analysis_limit\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_ANALYSIS_LIMIT as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_Result0 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"application_id\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_HEADER_VALUE as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NoColumns1 | crate::src::src::pragma::PragFlg_Result0) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::src::src::btree::BTREE_APPLICATION_ID as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"auto_vacuum\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_AUTO_VACUUM as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema
                | crate::src::src::pragma::PragFlg_Result0
                | crate::src::src::pragma::PragFlg_SchemaReq
                | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"automatic_index\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_AutoIndex as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"busy_timeout\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_BUSY_TIMEOUT as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_Result0 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  56 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  1 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"cache_size\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_CACHE_SIZE as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema
                | crate::src::src::pragma::PragFlg_Result0
                | crate::src::src::pragma::PragFlg_SchemaReq
                | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"cache_spill\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_CACHE_SPILL as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_SchemaReq | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"case_sensitive_like\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_CASE_SENSITIVE_LIKE as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_NoColumns as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"cell_size_check\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_CellSizeCk as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"checkpoint_fullfsync\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_CkptFullFSync as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"collation_list\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_COLLATION_LIST as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_Result0 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  33 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  2 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"compile_options\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_COMPILE_OPTIONS as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_Result0 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"count_changes\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_CountRows,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"data_version\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_HEADER_VALUE as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_ReadOnly | crate::src::src::pragma::PragFlg_Result0) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::src::src::btree::BTREE_DATA_VERSION as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"database_list\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_DATABASE_LIST as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_Result0 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  50 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  3 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"default_cache_size\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_DEFAULT_CACHE_SIZE as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema
                | crate::src::src::pragma::PragFlg_Result0
                | crate::src::src::pragma::PragFlg_SchemaReq
                | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  55 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  1 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"defer_foreign_keys\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_DeferFKs as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"empty_result_callbacks\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_NullCallback as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"encoding\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_ENCODING as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"foreign_key_check\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FOREIGN_KEY_CHECK as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema | crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_Result1 | crate::src::src::pragma::PragFlg_SchemaOpt)
                as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  43 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  4 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"foreign_key_list\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FOREIGN_KEY_LIST as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema | crate::src::src::pragma::PragFlg_Result1 | crate::src::src::pragma::PragFlg_SchemaOpt) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  8 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"foreign_keys\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_ForeignKeys as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"freelist_count\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_HEADER_VALUE as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_ReadOnly | crate::src::src::pragma::PragFlg_Result0) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::src::src::btree::BTREE_FREE_PAGE_COUNT as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"full_column_names\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_FullColNames as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"fullfsync\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_FullFSync as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"function_list\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FUNCTION_LIST as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_Result0 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  15 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  6 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"hard_heap_limit\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_HARD_HEAP_LIMIT as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_Result0 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"ignore_check_constraints\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_IgnoreChecks as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"incremental_vacuum\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_INCREMENTAL_VACUUM as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema | crate::src::src::pragma::PragFlg_NoColumns) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"index_info\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_INDEX_INFO as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema | crate::src::src::pragma::PragFlg_Result1 | crate::src::src::pragma::PragFlg_SchemaOpt) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  27 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  3 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"index_list\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_INDEX_LIST as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema | crate::src::src::pragma::PragFlg_Result1 | crate::src::src::pragma::PragFlg_SchemaOpt) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  33 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  5 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"index_xinfo\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_INDEX_INFO as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema | crate::src::src::pragma::PragFlg_Result1 | crate::src::src::pragma::PragFlg_SchemaOpt) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  27 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  6 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  1 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"integrity_check\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_INTEGRITY_CHECK as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema | crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_Result1 | crate::src::src::pragma::PragFlg_SchemaOpt)
                as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"journal_mode\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_JOURNAL_MODE as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema | crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_SchemaReq) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"journal_size_limit\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_JOURNAL_SIZE_LIMIT as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_SchemaReq) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"legacy_alter_table\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_LegacyAlter as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"lock_status\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_LOCK_STATUS as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_Result0 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  53 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  2 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"locking_mode\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_LOCKING_MODE as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_SchemaReq) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"max_page_count\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_PAGE_COUNT as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema | crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_SchemaReq) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"mmap_size\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_MMAP_SIZE as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  0 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"module_list\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_MODULE_LIST as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_Result0 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  9 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  1 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"optimize\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_OPTIMIZE as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result1 | crate::src::src::pragma::PragFlg_NeedSchema) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"page_count\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_PAGE_COUNT as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema | crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_SchemaReq) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"page_size\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_PAGE_SIZE as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_SchemaReq | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"pragma_list\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_PRAGMA_LIST as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_Result0 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  9 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  1 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"query_only\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_QueryOnly as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"quick_check\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_INTEGRITY_CHECK as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema | crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_Result1 | crate::src::src::pragma::PragFlg_SchemaOpt)
                as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"read_uncommitted\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_ReadUncommit,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"recursive_triggers\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_RecTriggers as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"reverse_unordered_selects\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_ReverseOrder as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"schema_version\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_HEADER_VALUE as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NoColumns1 | crate::src::src::pragma::PragFlg_Result0) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::src::src::btree::BTREE_SCHEMA_VERSION as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"secure_delete\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_SECURE_DELETE as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_Result0 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"short_column_names\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_ShortColNames as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"shrink_memory\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_SHRINK_MEMORY as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_NoColumns as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"soft_heap_limit\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_SOFT_HEAP_LIMIT as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_Result0 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"synchronous\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_SYNCHRONOUS as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema
                | crate::src::src::pragma::PragFlg_Result0
                | crate::src::src::pragma::PragFlg_SchemaReq
                | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"table_info\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_TABLE_INFO as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema | crate::src::src::pragma::PragFlg_Result1 | crate::src::src::pragma::PragFlg_SchemaOpt) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  8 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  6 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"table_list\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_TABLE_LIST as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema | crate::src::src::pragma::PragFlg_Result1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  21 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  6 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"table_xinfo\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_TABLE_INFO as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NeedSchema | crate::src::src::pragma::PragFlg_Result1 | crate::src::src::pragma::PragFlg_SchemaOpt) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  8 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  7 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  1 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"temp_store\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_TEMP_STORE as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"temp_store_directory\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_TEMP_STORE_DIRECTORY as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_NoColumns1 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"threads\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_THREADS as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_Result0 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"trusted_schema\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::sqliteInt_h::SQLITE_TrustedSchema as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"user_version\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_HEADER_VALUE as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_NoColumns1 | crate::src::src::pragma::PragFlg_Result0) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  crate::src::src::btree::BTREE_USER_VERSION as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"wal_autocheckpoint\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_WAL_AUTOCHECKPOINT as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  0 as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"wal_checkpoint\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_WAL_CHECKPOINT as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  crate::src::src::pragma::PragFlg_NeedSchema as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  47 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  3 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  0 as crate::src::ext::rtree::rtree::u64_0,
},
        crate::src::src::pragma::PragmaName {
    zName:  b"writable_schema\0" as *const u8 as *const ::core::ffi::c_char,
    ePragTyp:  crate::src::src::pragma::PragTyp_FLAG as crate::src::ext::rtree::rtree::u8_0,
    mPragFlg:  (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_NoColumns1) as crate::src::ext::rtree::rtree::u8_0,
    iPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    nPragCName:  0 as crate::src::ext::rtree::rtree::u8_0,
    iArg:  (crate::sqliteInt_h::SQLITE_WriteSchema | crate::sqliteInt_h::SQLITE_NoSchemaError) as crate::src::ext::rtree::rtree::u64_0,
},
    ];
    
    
}




pub use crate::__stddef_size_t_h::size_t;


pub use crate::src::src::btree::sqlite3BtreeClose;pub use crate::src::src::btree::sqlite3BtreeGetAutoVacuum;pub use crate::src::src::btree::sqlite3BtreeGetFilename;pub use crate::src::src::btree::sqlite3BtreeGetPageSize;pub use crate::src::src::btree::sqlite3BtreePager;pub use crate::src::src::btree::sqlite3BtreeSecureDelete;pub use crate::src::src::btree::sqlite3BtreeSetAutoVacuum;pub use crate::src::src::btree::sqlite3BtreeSetCacheSize;pub use crate::src::src::btree::sqlite3BtreeSetMmapLimit;pub use crate::src::src::btree::sqlite3BtreeSetPageSize;pub use crate::src::src::btree::sqlite3BtreeSetPagerFlags;pub use crate::src::src::btree::sqlite3BtreeSetSpillSize;pub use crate::src::src::btree::sqlite3BtreeTxnState;pub use crate::btreeInt_h::Btree;pub use crate::src::src::btree::BTREE_APPLICATION_ID;pub use crate::src::src::btree::BTREE_AUTOVACUUM_FULL;pub use crate::src::src::btree::BTREE_AUTOVACUUM_INCR;pub use crate::src::src::btree::BTREE_AUTOVACUUM_NONE;pub use crate::src::src::btree::BTREE_DATA_VERSION;pub use crate::src::src::btree::BTREE_DEFAULT_CACHE_SIZE;pub use crate::src::src::btree::BTREE_FREE_PAGE_COUNT;pub use crate::src::src::btree::BTREE_INCR_VACUUM;pub use crate::src::src::btree::BTREE_LARGEST_ROOT_PAGE;pub use crate::src::src::btree::BTREE_SCHEMA_VERSION;pub use crate::src::src::btree::BTREE_USER_VERSION;pub use crate::src::src::hash::Hash;pub use crate::src::src::hash::HashElem;pub use crate::src::src::hash::_ht;pub use crate::src::src::hash::sqlite3HashFind;pub use crate::opcodes_h::OP_AddImm;pub use crate::opcodes_h::OP_Affinity;pub use crate::opcodes_h::OP_Checkpoint;pub use crate::opcodes_h::OP_Column;pub use crate::opcodes_h::OP_Concat;pub use crate::opcodes_h::OP_Eq;pub use crate::opcodes_h::OP_Expire;pub use crate::opcodes_h::OP_Found;pub use crate::opcodes_h::OP_Goto;pub use crate::opcodes_h::OP_Halt;pub use crate::opcodes_h::OP_IdxGT;pub use crate::opcodes_h::OP_IdxRowid;pub use crate::opcodes_h::OP_If;pub use crate::opcodes_h::OP_IfNotZero;pub use crate::opcodes_h::OP_IfPos;pub use crate::opcodes_h::OP_IfSizeBetween;pub use crate::opcodes_h::OP_IncrVacuum;pub use crate::opcodes_h::OP_Int64;pub use crate::opcodes_h::OP_Integer;pub use crate::opcodes_h::OP_IntegrityCk;pub use crate::opcodes_h::OP_IsNull;pub use crate::opcodes_h::OP_IsType;pub use crate::opcodes_h::OP_JournalMode;pub use crate::opcodes_h::OP_MaxPgcnt;pub use crate::opcodes_h::OP_Ne;pub use crate::opcodes_h::OP_Next;pub use crate::opcodes_h::OP_Noop;pub use crate::opcodes_h::OP_NotNull;pub use crate::opcodes_h::OP_Null;pub use crate::opcodes_h::OP_OpenRead;pub use crate::opcodes_h::OP_Pagecount;pub use crate::opcodes_h::OP_ReadCookie;pub use crate::opcodes_h::OP_ResultRow;pub use crate::opcodes_h::OP_Rewind;pub use crate::opcodes_h::OP_Rowid;pub use crate::opcodes_h::OP_SeekRowid;pub use crate::opcodes_h::OP_SetCookie;pub use crate::opcodes_h::OP_SqlExec;pub use crate::opcodes_h::OP_String8;pub use crate::opcodes_h::OP_Subtract;pub use crate::opcodes_h::OP_Transaction;pub use crate::opcodes_h::OP_VCheck;





pub use crate::src::src::pager::sqlite3PagerJournalSizeLimit;pub use crate::src::src::pager::sqlite3PagerLockingMode;pub use crate::src::src::pager::Pager;pub use crate::src::src::pager::Pgno;pub use crate::src::src::pager::PAGER_FLAGS_MASK;pub use crate::src::src::pager::PAGER_JOURNALMODE_OFF;pub use crate::src::src::pager::PAGER_JOURNALMODE_QUERY;pub use crate::src::src::pager::PAGER_LOCKINGMODE_EXCLUSIVE;pub use crate::src::src::pager::PAGER_LOCKINGMODE_NORMAL;pub use crate::src::src::pager::PAGER_LOCKINGMODE_QUERY;pub use crate::src::src::pager::PAGER_SYNCHRONOUS_MASK;pub use crate::src::src::pragma::pragma_h::aPragmaName;pub use crate::src::src::pragma::pragma_h::pragCName;pub use crate::src::src::main::sqlite3_busy_timeout;pub use crate::src::src::vdbeapi::sqlite3_column_value;pub use crate::src::src::main::sqlite3_compileoption_get;pub use crate::vdbeInt_h::sqlite3_context;pub use crate::src::src::main::sqlite3_db_release_memory;pub use crate::src::src::vtab::sqlite3_declare_vtab;pub use crate::sqlite3_h::sqlite3_destructor_type;pub use crate::src::src::main::sqlite3_errmsg;pub use crate::sqlite3_h::sqlite3_file;pub use crate::src::src::main::sqlite3_file_control;pub use crate::sqlite3_h::sqlite3_filename;pub use crate::src::src::vdbeapi::sqlite3_finalize;pub use crate::src::src::malloc::sqlite3_free;pub use crate::src::src::malloc::sqlite3_hard_heap_limit64;pub use crate::sqlite3_h::sqlite3_index_constraint;pub use crate::sqlite3_h::sqlite3_index_constraint_usage;pub use crate::sqlite3_h::sqlite3_index_info;pub use crate::sqlite3_h::sqlite3_index_orderby;pub use crate::sqlite3_h::sqlite3_int64;pub use crate::sqlite3_h::sqlite3_io_methods;pub use crate::src::src::main::sqlite3_limit;pub use crate::src::src::malloc::sqlite3_malloc;pub use crate::sqlite3_h::sqlite3_mem_methods;pub use crate::sqlite3_h::sqlite3_module;pub use crate::src::src::printf::sqlite3_mprintf;pub use crate::src::src::mutex_unix::sqlite3_mutex;pub use crate::src::src::mutex::sqlite3_mutex_enter;pub use crate::src::src::mutex::sqlite3_mutex_leave;pub use crate::sqlite3_h::sqlite3_mutex_methods;pub use crate::sqlite3_h::sqlite3_pcache;pub use crate::sqlite3_h::sqlite3_pcache_methods2;pub use crate::sqlite3_h::sqlite3_pcache_page;pub use crate::src::src::prepare::sqlite3_prepare_v2;pub use crate::src::src::prepare::sqlite3_prepare_v3;pub use crate::src::src::vdbeapi::sqlite3_result_text;pub use crate::src::src::vdbeapi::sqlite3_result_value;pub use crate::src::src::malloc::sqlite3_soft_heap_limit64;pub use crate::src::src::vdbeapi::sqlite3_step;pub use crate::sqlite3_h::sqlite3_stmt;pub use crate::src::src::printf::sqlite3_str_append;pub use crate::src::src::printf::sqlite3_str_appendall;pub use crate::src::src::printf::sqlite3_str_appendf;pub use crate::src::src::util::sqlite3_stricmp;pub use crate::src::src::util::sqlite3_strnicmp;pub use crate::sqlite3_h::sqlite3_syscall_ptr;pub use crate::src::src::main::sqlite3_temp_directory;pub use crate::sqlite3_h::sqlite3_uint64;pub use crate::vdbeInt_h::sqlite3_value;pub use crate::src::src::vdbeapi::sqlite3_value_text;pub use crate::src::src::vdbeapi::sqlite3_value_type;pub use crate::sqlite3_h::sqlite3_vfs;pub use crate::sqlite3_h::sqlite3_vtab;pub use crate::sqlite3_h::sqlite3_vtab_cursor;pub use crate::src::src::main::sqlite3_wal_autocheckpoint;pub use crate::sqlite3_h::sqlite_int64;pub use crate::sqlite3_h::sqlite_uint64;pub use crate::sqlite3_h::SQLITE_ACCESS_READWRITE;pub use crate::sqlite3_h::SQLITE_CHECKPOINT_FULL;pub use crate::sqlite3_h::SQLITE_CHECKPOINT_NOOP;pub use crate::sqlite3_h::SQLITE_CHECKPOINT_PASSIVE;pub use crate::sqlite3_h::SQLITE_CHECKPOINT_RESTART;pub use crate::sqlite3_h::SQLITE_CHECKPOINT_TRUNCATE;pub use crate::sqlite3_h::SQLITE_CONSTRAINT;pub use crate::sqlite3_h::SQLITE_CORRUPT;pub use crate::sqlite3_h::SQLITE_DETERMINISTIC;pub use crate::sqlite3_h::SQLITE_DIRECTONLY;pub use crate::sqlite3_h::SQLITE_ERROR;pub use crate::sqlite3_h::SQLITE_FCNTL_LOCKSTATE_1;pub use crate::sqlite3_h::SQLITE_FCNTL_MMAP_SIZE_1;pub use crate::sqlite3_h::SQLITE_FCNTL_PRAGMA;pub use crate::sqlite3_h::SQLITE_INDEX_CONSTRAINT_EQ;pub use crate::sqlite3_h::SQLITE_INNOCUOUS;pub use crate::sqlite3_h::SQLITE_LIMIT_SQL_LENGTH;pub use crate::sqlite3_h::SQLITE_LIMIT_WORKER_THREADS;pub use crate::sqlite3_h::SQLITE_MUTEX_STATIC_VFS1;pub use crate::sqlite3_h::SQLITE_NOMEM;pub use crate::sqlite3_h::SQLITE_NOTFOUND;pub use crate::sqlite3_h::SQLITE_NULL;pub use crate::sqlite3_h::SQLITE_OK;pub use crate::sqlite3_h::SQLITE_PRAGMA;pub use crate::sqlite3_h::SQLITE_PREPARE_DONT_LOG;pub use crate::sqlite3_h::SQLITE_ROW;pub use crate::sqlite3_h::SQLITE_STATIC;pub use crate::sqlite3_h::SQLITE_SUBTYPE;pub use crate::sqlite3_h::SQLITE_TXN_NONE;pub use crate::sqlite3_h::SQLITE_UTF16BE;pub use crate::sqlite3_h::SQLITE_UTF16LE;pub use crate::sqlite3_h::SQLITE_UTF8;pub use crate::sqliteInt_h::__anon_struct_0;pub use crate::sqliteInt_h::__anon_struct_1;pub use crate::sqliteInt_h::__anon_struct_2;pub use crate::sqliteInt_h::__anon_struct_3;pub use crate::sqliteInt_h::__anon_struct_4;pub use crate::sqliteInt_h::__anon_struct_5;pub use crate::sqliteInt_h::__anon_struct_6;pub use crate::sqliteInt_h::__anon_struct_7;pub use crate::sqliteInt_h::__anon_struct_8;pub use crate::sqliteInt_h::__anon_union_0;pub use crate::sqliteInt_h::__anon_union_1;pub use crate::sqliteInt_h::__anon_union_10;pub use crate::sqliteInt_h::__anon_union_11;pub use crate::sqliteInt_h::__anon_union_12;pub use crate::sqliteInt_h::__anon_union_13;pub use crate::sqliteInt_h::__anon_union_15;pub use crate::sqliteInt_h::__anon_union_2;pub use crate::sqliteInt_h::__anon_union_3;pub use crate::sqliteInt_h::__anon_union_5;pub use crate::sqliteInt_h::__anon_union_6;pub use crate::sqliteInt_h::__anon_union_7;pub use crate::sqliteInt_h::__anon_union_8;pub use crate::sqliteInt_h::__anon_union_9;pub use crate::sqliteInt_h::bft;pub use crate::src::fts5::i16_0;pub use crate::src::ext::rtree::rtree::i64_0;pub use crate::sqliteInt_h::sColMap;pub use crate::sqliteInt_h::sqlite3;pub use crate::src::src::util::sqlite3AbsInt32;pub use crate::src::src::util::sqlite3Atoi;pub use crate::src::src::auth::sqlite3AuthCheck;pub use crate::src::src::build::sqlite3BeginWriteOperation;pub use crate::src::src::global::sqlite3BuiltinFunctions;pub use crate::src::src::expr::sqlite3ClearTempRegCache;pub use crate::src::src::build::sqlite3CodeVerifyNamedSchema;pub use crate::src::src::build::sqlite3CodeVerifySchema;pub use crate::src::src::update::sqlite3ColumnDefault;pub use crate::src::src::build::sqlite3ColumnExpr;pub use crate::src::src::util::sqlite3ColumnType;pub use crate::src::src::global::sqlite3Config;pub use crate::src::src::global::sqlite3CtypeMap;pub use crate::src::src::malloc::sqlite3DbFree;pub use crate::src::src::malloc::sqlite3DbMallocRawNN;pub use crate::src::src::util::sqlite3DecOrHexToI64;pub use crate::src::src::main::sqlite3ErrStr;pub use crate::src::src::util::sqlite3ErrorMsg;pub use crate::src::src::expr::sqlite3ExprCodeGetColumnOfTable;pub use crate::src::src::expr::sqlite3ExprCodeLoadIndexColumn;pub use crate::src::src::expr::sqlite3ExprIfFalse;pub use crate::src::src::expr::sqlite3ExprIfTrue;pub use crate::src::src::expr::sqlite3ExprListDelete;pub use crate::src::src::expr::sqlite3ExprListDup;pub use crate::src::src::build::sqlite3FindIndex;pub use crate::src::src::build::sqlite3FindTable;pub use crate::src::src::fkey::sqlite3FkLocateIndex;pub use crate::src::src::delete::sqlite3GenerateIndexKey;pub use crate::src::src::util::sqlite3GetInt32;pub use crate::src::src::expr::sqlite3GetTempRange;pub use crate::src::src::expr::sqlite3GetTempReg;pub use crate::src::src::select::sqlite3GetVdbe;pub use crate::src::src::insert::sqlite3IndexAffinityStr;pub use crate::sqliteInt_h::sqlite3InitInfo;pub use crate::src::src::build::sqlite3LocateTable;pub use crate::src::src::printf::sqlite3MPrintf;pub use crate::src::src::mutex::sqlite3MutexAlloc;pub use crate::src::src::build::sqlite3NameFromToken;pub use crate::src::src::malloc::sqlite3OomFault;pub use crate::src::src::insert::sqlite3OpenTable;pub use crate::src::src::insert::sqlite3OpenTableAndIndices;pub use crate::src::src::build::sqlite3OpenTempDatabase;pub use crate::src::src::build::sqlite3PreferredTableName;pub use crate::src::src::build::sqlite3PrimaryKeyIndex;pub use crate::src::src::prepare::sqlite3ReadSchema;pub use crate::src::src::func::sqlite3RegisterLikeFunctions;pub use crate::src::src::expr::sqlite3ReleaseTempRange;pub use crate::src::src::build::sqlite3ResetAllSchemasOfConnection;pub use crate::src::src::delete::sqlite3ResolvePartIdxLabel;pub use crate::src::src::prepare::sqlite3SchemaToIndex;pub use crate::src::src::callback::sqlite3SetTextEncoding;pub use crate::src::src::global::sqlite3StdType;pub use crate::src::src::printf::sqlite3StrAccumFinish;pub use crate::src::src::printf::sqlite3StrAccumInit;pub use crate::src::src::global::sqlite3StrBINARY;pub use crate::src::src::util::sqlite3StrICmp;pub use crate::src::src::util::sqlite3Strlen30;pub use crate::src::src::build::sqlite3TableColumnToIndex;pub use crate::src::src::build::sqlite3TableColumnToStorage;pub use crate::src::src::build::sqlite3TableLock;pub use crate::src::src::expr::sqlite3TouchRegister;pub use crate::src::src::build::sqlite3TwoPartName;pub use crate::src::src::global::sqlite3UpperToLower;pub use crate::src::src::vdbemem::sqlite3ValueFree;pub use crate::src::src::vdbemem::sqlite3ValueFromExpr;pub use crate::src::src::build::sqlite3ViewGetColumnNames;pub use crate::src::src::vtab::sqlite3VtabCreateModule;pub use crate::src::src::main::sqlite3WalDefaultHook;pub use crate::sqliteInt_h::sqlite3_str;pub use crate::sqliteInt_h::sqlite3_xauth;pub use crate::src::fts5::u16_0;pub use crate::src::ext::rtree::rtree::u32_0;pub use crate::src::ext::rtree::rtree::u64_0;pub use crate::src::ext::rtree::rtree::u8_0;pub use crate::sqliteInt_h::yDbMask;pub use crate::sqliteInt_h::ynVar;pub use crate::sqliteInt_h::AggInfo;pub use crate::sqliteInt_h::AggInfo_col;pub use crate::sqliteInt_h::AggInfo_func;pub use crate::sqliteInt_h::AutoincInfo;pub use crate::sqliteInt_h::Bitmask;pub use crate::sqliteInt_h::BusyHandler;pub use crate::sqliteInt_h::CollSeq;pub use crate::sqliteInt_h::Column;pub use crate::sqliteInt_h::Cte;pub use crate::sqliteInt_h::CteUse;pub use crate::sqliteInt_h::DBFLAG_EncodingFixed;pub use crate::sqliteInt_h::DBFLAG_InternalFunc;pub use crate::sqliteInt_h::Db;pub use crate::sqliteInt_h::DbClientData;pub use crate::sqliteInt_h::Expr;pub use crate::sqliteInt_h::ExprList;pub use crate::sqliteInt_h::ExprList_item;pub use crate::sqliteInt_h::FKey;pub use crate::sqliteInt_h::FuncDef;pub use crate::sqliteInt_h::FuncDefHash;pub use crate::sqliteInt_h::FuncDestructor;pub use crate::sqliteInt_h::IdList;pub use crate::sqliteInt_h::IdList_item;pub use crate::sqliteInt_h::Index;pub use crate::sqliteInt_h::IndexedExpr;pub use crate::sqliteInt_h::KeyInfo;pub use crate::sqliteInt_h::LogEst;pub use crate::sqliteInt_h::Lookaside;pub use crate::sqliteInt_h::LookasideSlot;pub use crate::sqliteInt_h::Module;pub use crate::sqliteInt_h::OE_Abort;pub use crate::sqliteInt_h::OE_Cascade_1;pub use crate::sqliteInt_h::OE_None;pub use crate::sqliteInt_h::OE_Restrict_1;pub use crate::sqliteInt_h::OE_SetDflt_1;pub use crate::sqliteInt_h::OE_SetNull_1;pub use crate::sqliteInt_h::Parse;pub use crate::sqliteInt_h::ParseCleanup;pub use crate::vdbeInt_h::PreUpdate;pub use crate::sqliteInt_h::RenameToken;pub use crate::sqliteInt_h::Returning;pub use crate::sqliteInt_h::SQLITE_AutoIndex;pub use crate::sqliteInt_h::SQLITE_CacheSpill;pub use crate::sqliteInt_h::SQLITE_CellSizeCk;pub use crate::sqliteInt_h::SQLITE_CkptFullFSync;pub use crate::sqliteInt_h::SQLITE_CountRows;pub use crate::sqliteInt_h::SQLITE_Defensive;pub use crate::sqliteInt_h::SQLITE_DeferFKs;pub use crate::sqliteInt_h::SQLITE_ForeignKeys;pub use crate::sqliteInt_h::SQLITE_FullColNames;pub use crate::sqliteInt_h::SQLITE_FullFSync;pub use crate::sqliteInt_h::SQLITE_IgnoreChecks;pub use crate::sqliteInt_h::SQLITE_LegacyAlter;pub use crate::sqliteInt_h::SQLITE_NoSchemaError;pub use crate::sqliteInt_h::SQLITE_NullCallback;pub use crate::sqliteInt_h::SQLITE_QueryOnly;pub use crate::sqliteInt_h::SQLITE_ReadUncommit;pub use crate::sqliteInt_h::SQLITE_RecTriggers;pub use crate::sqliteInt_h::SQLITE_ReverseOrder;pub use crate::sqliteInt_h::SQLITE_ShortColNames;pub use crate::sqliteInt_h::SQLITE_TrustedSchema;pub use crate::sqliteInt_h::SQLITE_WriteSchema;pub use crate::sqliteInt_h::Savepoint;pub use crate::sqliteInt_h::Schema;pub use crate::sqliteInt_h::Select;pub use crate::sqliteInt_h::Sqlite3Config;pub use crate::sqliteInt_h::SrcItem;pub use crate::sqliteInt_h::SrcList;pub use crate::sqliteInt_h::StrAccum;pub use crate::sqliteInt_h::Subquery;pub use crate::sqliteInt_h::TF_Imposter;pub use crate::sqliteInt_h::TF_MaybeReanalyze;pub use crate::sqliteInt_h::TF_Shadow;pub use crate::sqliteInt_h::TF_Strict;pub use crate::sqliteInt_h::TF_WithoutRowid;pub use crate::sqliteInt_h::Table;pub use crate::sqliteInt_h::TableLock;pub use crate::sqliteInt_h::Token;pub use crate::sqliteInt_h::Trigger;pub use crate::sqliteInt_h::TriggerPrg;pub use crate::sqliteInt_h::TriggerStep;pub use crate::sqliteInt_h::Upsert;pub use crate::sqliteInt_h::VList;pub use crate::sqliteInt_h::VTable;pub use crate::sqliteInt_h::VtabCtx;pub use crate::sqliteInt_h::Window;pub use crate::sqliteInt_h::With;pub use crate::sqliteInt_h::COLFLAG_NOINSERT;pub use crate::sqliteInt_h::COLFLAG_PRIMKEY;pub use crate::sqliteInt_h::COLFLAG_STORED;pub use crate::sqliteInt_h::COLFLAG_VIRTUAL;pub use crate::sqliteInt_h::COLTYPE_ANY;pub use crate::sqliteInt_h::LOCATE_NOERR;pub use crate::sqliteInt_h::OMIT_TEMPDB;pub use crate::sqliteInt_h::SQLITE_AFF_BLOB;pub use crate::sqliteInt_h::SQLITE_AFF_NUMERIC;pub use crate::sqliteInt_h::SQLITE_AFF_TEXT;pub use crate::sqliteInt_h::SQLITE_FUNC_ENCMASK;pub use crate::sqliteInt_h::SQLITE_FUNC_HASH_SZ;pub use crate::sqliteInt_h::SQLITE_FUNC_INTERNAL;pub use crate::sqliteInt_h::SQLITE_IDXTYPE_PRIMARYKEY;pub use crate::sqliteInt_h::SQLITE_JUMPIFNULL;pub use crate::sqliteInt_h::SQLITE_MAX_DB;pub use crate::sqliteInt_h::SQLITE_MUTEX_STATIC_TEMPDIR;pub use crate::sqliteInt_h::SQLITE_NOMEM_BKPT;pub use crate::sqliteInt_h::SQLITE_TEMP_STORE;pub use crate::sqliteInt_h::SQLITE_UTF16NATIVE;pub use crate::sqliteInt_h::TABTYP_NORM;pub use crate::sqliteInt_h::TABTYP_VIEW;pub use crate::sqliteInt_h::TABTYP_VTAB;pub use crate::sqliteLimit_h::SQLITE_DEFAULT_CACHE_SIZE;pub use crate::sqliteLimit_h::SQLITE_MAX_ATTACHED;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::uint16_t;pub use crate::stdlib::uint32_t;pub use crate::stdlib::uint8_t;


pub use crate::stdlib::__int16_t;pub use crate::stdlib::__uint16_t;pub use crate::stdlib::__uint32_t;pub use crate::stdlib::__uint8_t;pub use crate::src::src::vdbe::p4union;pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp0;pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp1;pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp2;pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp3;pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp4;pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp4Dup8;pub use crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int;pub use crate::src::src::vdbeaux::sqlite3VdbeAddOpList;pub use crate::src::src::vdbeaux::sqlite3VdbeAppendP4;pub use crate::src::src::vdbeaux::sqlite3VdbeChangeP3;pub use crate::src::src::vdbeaux::sqlite3VdbeChangeP5;pub use crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr;pub use crate::src::src::vdbeaux::sqlite3VdbeGetOp;pub use crate::src::src::vdbeaux::sqlite3VdbeGoto;pub use crate::src::src::vdbeaux::sqlite3VdbeJumpHere;pub use crate::src::src::vdbeaux::sqlite3VdbeLoadString;pub use crate::src::src::vdbeaux::sqlite3VdbeMakeLabel;pub use crate::src::src::vdbeaux::sqlite3VdbeMultiLoad;pub use crate::src::src::vdbeaux::sqlite3VdbeResolveLabel;pub use crate::src::src::vdbeaux::sqlite3VdbeReusable;pub use crate::src::src::vdbeaux::sqlite3VdbeRunOnlyOnce;pub use crate::src::src::vdbeaux::sqlite3VdbeSetColName;pub use crate::src::src::vdbeaux::sqlite3VdbeSetNumCols;pub use crate::src::src::vdbeaux::sqlite3VdbeSetP4KeyInfo;pub use crate::src::src::vdbeaux::sqlite3VdbeTypeofColumn;pub use crate::src::src::vdbeaux::sqlite3VdbeUsesBtree;pub use crate::src::src::vdbe::Mem;pub use crate::src::src::vdbe::SubProgram;pub use crate::src::src::vdbe::SubrtnSig;pub use crate::vdbeInt_h::Vdbe;pub use crate::src::src::vdbe::VdbeOp;pub use crate::src::src::vdbe::VdbeOpList;pub use crate::src::src::vdbe::COLNAME_NAME;pub use crate::src::src::vdbe::P4_DYNAMIC;pub use crate::src::src::vdbe::P4_INT64;pub use crate::src::src::vdbe::P4_INTARRAY;pub use crate::src::src::vdbe::P4_STATIC;pub use crate::src::src::vdbe::P4_TABLEREF;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct EncName {
    pub zName: *mut ::core::ffi::c_char,
    pub enc: crate::src::ext::rtree::rtree::u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct PragmaVtabCursor {
    pub base: crate::sqlite3_h::sqlite3_vtab_cursor,
    pub pPragma: *mut crate::sqlite3_h::sqlite3_stmt,
    pub iRowid: crate::sqlite3_h::sqlite_int64,
    pub azArg: [*mut ::core::ffi::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct PragmaVtab {
    pub base: crate::sqlite3_h::sqlite3_vtab,
    pub db: *mut crate::sqliteInt_h::sqlite3,
    pub pName: *const crate::src::src::pragma::PragmaName,
    pub nHidden: crate::src::ext::rtree::rtree::u8_0,
    pub iHidden: crate::src::ext::rtree::rtree::u8_0,
}

pub const SQLITE_DEFAULT_OPTIMIZE_LIMIT: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;

unsafe extern "C" fn getSafetyLevel(
    mut z: *const ::core::ffi::c_char,
    mut omitFull: ::core::ffi::c_int,
    mut dflt: crate::src::ext::rtree::rtree::u8_0,
) -> crate::src::ext::rtree::rtree::u8_0 {
    static mut zText: [::core::ffi::c_char; 25] = unsafe {
        ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(
            *b"onoffalseyestruextrafull\0",
        )
    };
    static mut iOffset: [crate::src::ext::rtree::rtree::u8_0; 8] = [
        0 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        2 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        4 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        9 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        12 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        15 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        20 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
    ];
    static mut iLength: [crate::src::ext::rtree::rtree::u8_0; 8] = [
        2 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        2 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        3 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        5 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        3 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        4 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        5 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        4 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
    ];
    static mut iValue: [crate::src::ext::rtree::rtree::u8_0; 8] = [
        1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        0 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        0 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        0 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        3 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
        2 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0,
    ];
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    if *(&raw const crate::src::src::global::sqlite3CtypeMap as *const ::core::ffi::c_uchar)
        .offset(*z as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & 0x4 as ::core::ffi::c_int
        != 0
    {
        return crate::src::src::util::sqlite3Atoi(z) as crate::src::ext::rtree::rtree::u8_0;
    }
    n = crate::src::src::util::sqlite3Strlen30(z);
    i = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[crate::src::ext::rtree::rtree::u8_0; 8]>() as usize)
            .wrapping_div(::core::mem::size_of::<crate::src::ext::rtree::rtree::u8_0>() as usize) as ::core::ffi::c_int
    {
        if iLength[i as usize] as ::core::ffi::c_int == n
            && crate::src::src::util::sqlite3_strnicmp(
                (&raw const zText as *const ::core::ffi::c_char)
                    .offset(*(&raw const iOffset as *const crate::src::ext::rtree::rtree::u8_0).offset(i as isize) as isize)
                    as *const ::core::ffi::c_char,
                z,
                n,
            ) == 0 as ::core::ffi::c_int
            && (omitFull == 0
                || iValue[i as usize] as ::core::ffi::c_int <= 1 as ::core::ffi::c_int)
        {
            return iValue[i as usize];
        }
        i += 1;
    }
    return dflt;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3GetBoolean(
    mut z: *const ::core::ffi::c_char,
    mut dflt: crate::src::ext::rtree::rtree::u8_0,
) -> crate::src::ext::rtree::rtree::u8_0 {
    return (getSafetyLevel(z, 1 as ::core::ffi::c_int, dflt) as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u8_0;
}

unsafe extern "C" fn getLockingMode(mut z: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if !z.is_null() {
        if 0 as ::core::ffi::c_int
            == crate::src::src::util::sqlite3StrICmp(z, b"exclusive\0" as *const u8 as *const ::core::ffi::c_char)
        {
            return crate::src::src::pager::PAGER_LOCKINGMODE_EXCLUSIVE;
        }
        if 0 as ::core::ffi::c_int
            == crate::src::src::util::sqlite3StrICmp(z, b"normal\0" as *const u8 as *const ::core::ffi::c_char)
        {
            return crate::src::src::pager::PAGER_LOCKINGMODE_NORMAL;
        }
    }
    return crate::src::src::pager::PAGER_LOCKINGMODE_QUERY;
}

unsafe extern "C" fn getAutoVacuum(mut z: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if 0 as ::core::ffi::c_int
        == crate::src::src::util::sqlite3StrICmp(z, b"none\0" as *const u8 as *const ::core::ffi::c_char)
    {
        return crate::src::src::btree::BTREE_AUTOVACUUM_NONE;
    }
    if 0 as ::core::ffi::c_int
        == crate::src::src::util::sqlite3StrICmp(z, b"full\0" as *const u8 as *const ::core::ffi::c_char)
    {
        return crate::src::src::btree::BTREE_AUTOVACUUM_FULL;
    }
    if 0 as ::core::ffi::c_int
        == crate::src::src::util::sqlite3StrICmp(
            z,
            b"incremental\0" as *const u8 as *const ::core::ffi::c_char,
        )
    {
        return crate::src::src::btree::BTREE_AUTOVACUUM_INCR;
    }
    i = crate::src::src::util::sqlite3Atoi(z);
    return (if i >= 0 as ::core::ffi::c_int && i <= 2 as ::core::ffi::c_int {
        i
    } else {
        0 as ::core::ffi::c_int
    }) as crate::src::ext::rtree::rtree::u8_0 as ::core::ffi::c_int;
}

unsafe extern "C" fn getTempStore(mut z: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if *z.offset(0 as isize) as ::core::ffi::c_int >= '0' as i32
        && *z.offset(0 as isize) as ::core::ffi::c_int <= '2' as i32
    {
        return *z.offset(0 as isize) as ::core::ffi::c_int - '0' as i32;
    } else if crate::src::src::util::sqlite3StrICmp(z, b"file\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    } else if crate::src::src::util::sqlite3StrICmp(z, b"memory\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return 2 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}

unsafe extern "C" fn invalidateTempStorage(mut pParse: *mut crate::sqliteInt_h::Parse) -> ::core::ffi::c_int {
    let mut db: *mut crate::sqliteInt_h::sqlite3 = (*pParse).db;
    if !(*(*db).aDb.offset(1 as isize))
        .pBt
        .is_null()
    {
        let __db_ref = unsafe { &mut *db };
        if __db_ref.autoCommit == 0
            || crate::src::src::btree::sqlite3BtreeTxnState((*__db_ref.aDb.offset(1 as isize)).pBt)
                != crate::sqlite3_h::SQLITE_TXN_NONE
        {
            crate::src::src::util::sqlite3ErrorMsg(
                
                pParse as *mut crate::sqliteInt_h::Parse,
                b"temporary storage cannot be changed from within a transaction\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return crate::sqlite3_h::SQLITE_ERROR;
        }
        crate::src::src::btree::sqlite3BtreeClose((*__db_ref.aDb.offset(1 as isize)).pBt);
        let ref mut fresh8 = (*__db_ref.aDb.offset(1 as isize)).pBt;
        *fresh8 = ::core::ptr::null_mut::<crate::btreeInt_h::Btree>();
        crate::src::src::build::sqlite3ResetAllSchemasOfConnection(db as *mut crate::sqliteInt_h::sqlite3);
    }
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn changeTempStorage(
    mut pParse: *mut crate::sqliteInt_h::Parse,
    mut zStorageType: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ts: ::core::ffi::c_int = getTempStore(zStorageType);
    let mut db: *mut crate::sqliteInt_h::sqlite3 = (*pParse).db;
    if (*db).temp_store as ::core::ffi::c_int == ts {
        return crate::sqlite3_h::SQLITE_OK;
    }
    if invalidateTempStorage(pParse) != crate::sqlite3_h::SQLITE_OK {
        return crate::sqlite3_h::SQLITE_ERROR;
    }
    (*db).temp_store = ts as crate::src::ext::rtree::rtree::u8_0;
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn setPragmaResultColumnNames(mut v: *mut crate::vdbeInt_h::Vdbe, mut pPragma: *const crate::src::src::pragma::PragmaName) {
    let mut n: crate::src::ext::rtree::rtree::u8_0 = (*pPragma).nPragCName;
    crate::src::src::vdbeaux::sqlite3VdbeSetNumCols(
        v,
        if n as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            n as ::core::ffi::c_int
        },
    );
    if n as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        crate::src::src::vdbeaux::sqlite3VdbeSetColName(
            v,
            0 as ::core::ffi::c_int,
            crate::src::src::vdbe::COLNAME_NAME,
            (*pPragma).zName,
            crate::sqlite3_h::SQLITE_STATIC,
        );
    } else {
        let mut i: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        j = (*pPragma).iPragCName as ::core::ffi::c_int;
        while i < n as ::core::ffi::c_int {
            crate::src::src::vdbeaux::sqlite3VdbeSetColName(v, i, crate::src::src::vdbe::COLNAME_NAME, pragCName[j as usize], crate::sqlite3_h::SQLITE_STATIC);
            i += 1;
            j += 1;
        }
    };
}

unsafe extern "C" fn returnSingleInt(mut v: *mut crate::vdbeInt_h::Vdbe, mut value: crate::src::ext::rtree::rtree::i64_0) {
    crate::src::src::vdbeaux::sqlite3VdbeAddOp4Dup8(
        v,
        crate::opcodes_h::OP_Int64,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        &raw mut value as *const crate::src::ext::rtree::rtree::u8_0,
        crate::src::src::vdbe::P4_INT64,
    );
    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
        v,
        crate::opcodes_h::OP_ResultRow,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn returnSingleText(mut v: *mut crate::vdbeInt_h::Vdbe, mut zValue: *const ::core::ffi::c_char) {
    if !zValue.is_null() {
        crate::src::src::vdbeaux::sqlite3VdbeLoadString(v, 1 as ::core::ffi::c_int, zValue);
        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
            v,
            crate::opcodes_h::OP_ResultRow,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    }
}

unsafe extern "C" fn setAllPagerFlags(mut db: *mut crate::sqliteInt_h::sqlite3) {
    if (*db).autoCommit != 0 {
        let mut pDb: *mut crate::sqliteInt_h::Db = (*db).aDb;
        let mut n: ::core::ffi::c_int = (*db).nDb;
        loop {
            let fresh7 = n;
            n = n - 1;
            if !(fresh7 > 0 as ::core::ffi::c_int) {
                break;
            }
            if !(*pDb).pBt.is_null() {
                crate::src::src::btree::sqlite3BtreeSetPagerFlags(
                    (*pDb).pBt,
                    ((*pDb).safety_level as crate::src::ext::rtree::rtree::u64_0 | (*db).flags & crate::src::src::pager::PAGER_FLAGS_MASK as crate::src::ext::rtree::rtree::u64_0)
                        as ::core::ffi::c_uint,
                );
            }
            pDb = pDb.offset(1);
        }
    }
}

unsafe extern "C" fn actionName(mut action: crate::src::ext::rtree::rtree::u8_0) -> *const ::core::ffi::c_char {
    let mut zName: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    match  action as ::core::ffi::c_int {
    crate::sqliteInt_h::OE_SetNull_1 =>  {
            zName = b"SET NULL\0" as *const u8 as *const ::core::ffi::c_char;
        }
    crate::sqliteInt_h::OE_SetDflt_1 =>  {
            zName = b"SET DEFAULT\0" as *const u8 as *const ::core::ffi::c_char;
        }
    crate::sqliteInt_h::OE_Cascade_1 =>  {
            zName = b"CASCADE\0" as *const u8 as *const ::core::ffi::c_char;
        }
    crate::sqliteInt_h::OE_Restrict_1 =>  {
            zName = b"RESTRICT\0" as *const u8 as *const ::core::ffi::c_char;
        }
    _ =>  {
            zName = b"NO ACTION\0" as *const u8 as *const ::core::ffi::c_char;
        }
}
    return zName;
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3JournalModename(
    mut eMode: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    static mut azModeName: [*mut ::core::ffi::c_char; 6] = [
        b"delete\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"persist\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"off\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"truncate\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"memory\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"wal\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ];
    if eMode
        == (::core::mem::size_of::<[*mut ::core::ffi::c_char; 6]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            as ::core::ffi::c_int
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return azModeName[eMode as usize];
}

unsafe extern "C" fn pragmaLocate(mut zName: *const ::core::ffi::c_char) -> *const crate::src::src::pragma::PragmaName {
    let mut upr: ::core::ffi::c_int = 0;
    let mut lwr: ::core::ffi::c_int = 0;
    let mut mid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = 0;
    lwr = 0 as ::core::ffi::c_int;
    upr = (::core::mem::size_of::<[crate::src::src::pragma::PragmaName; 67]>() as usize)
        .wrapping_div(::core::mem::size_of::<crate::src::src::pragma::PragmaName>() as usize)
        as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int;
    while lwr <= upr {
        mid = (lwr + upr) / 2 as ::core::ffi::c_int;
        rc = crate::src::src::util::sqlite3_stricmp(zName, aPragmaName[mid as usize].zName);
        if rc == 0 as ::core::ffi::c_int {
            break;
        }
        if rc < 0 as ::core::ffi::c_int {
            upr = mid - 1 as ::core::ffi::c_int;
        } else {
            lwr = mid + 1 as ::core::ffi::c_int;
        }
    }
    return if lwr > upr {
        ::core::ptr::null::<crate::src::src::pragma::PragmaName>()
    } else {
        (&raw const aPragmaName as *const crate::src::src::pragma::PragmaName).offset(mid as isize) as *const crate::src::src::pragma::PragmaName
    };
}

unsafe extern "C" fn pragmaFunclistLine(
    mut v: *mut crate::vdbeInt_h::Vdbe,
    mut p: *mut crate::sqliteInt_h::FuncDef,
    mut isBuiltin: ::core::ffi::c_int,
    mut showInternFuncs: ::core::ffi::c_int,
) {
    let mut mask: crate::src::ext::rtree::rtree::u32_0 = (crate::sqlite3_h::SQLITE_DETERMINISTIC
        | crate::sqlite3_h::SQLITE_DIRECTONLY
        | crate::sqlite3_h::SQLITE_SUBTYPE
        | crate::sqlite3_h::SQLITE_INNOCUOUS
        | crate::sqliteInt_h::SQLITE_FUNC_INTERNAL) as crate::src::ext::rtree::rtree::u32_0;
    if showInternFuncs != 0 {
        mask = 0xffffffff as ::core::ffi::c_uint as crate::src::ext::rtree::rtree::u32_0;
    }
    while !p.is_null() {
        let mut zType: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        static mut azEnc: [*const ::core::ffi::c_char; 4] = [
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"utf8\0" as *const u8 as *const ::core::ffi::c_char,
            b"utf16le\0" as *const u8 as *const ::core::ffi::c_char,
            b"utf16be\0" as *const u8 as *const ::core::ffi::c_char,
        ];
        if !(*p).xSFunc.is_none() {
            if !((*p).funcFlags & crate::sqliteInt_h::SQLITE_FUNC_INTERNAL as crate::src::ext::rtree::rtree::u32_0 != 0 as crate::src::ext::rtree::rtree::u32_0
                && showInternFuncs == 0 as ::core::ffi::c_int)
            {
                let __p_ref = unsafe { &mut *p };
                if __p_ref.xValue.is_some() {
                    zType = b"w\0" as *const u8 as *const ::core::ffi::c_char;
                } else if __p_ref.xFinalize.is_some() {
                    zType = b"a\0" as *const u8 as *const ::core::ffi::c_char;
                } else {
                    zType = b"s\0" as *const u8 as *const ::core::ffi::c_char;
                }
                crate::src::src::vdbeaux::sqlite3VdbeMultiLoad(
                    v,
                    1 as ::core::ffi::c_int,
                    b"sissii\0" as *const u8 as *const ::core::ffi::c_char,
                    __p_ref.zName,
                    isBuiltin,
                    zType,
                    azEnc[(__p_ref.funcFlags & crate::sqliteInt_h::SQLITE_FUNC_ENCMASK as crate::src::ext::rtree::rtree::u32_0) as usize],
                    __p_ref.nArg as ::core::ffi::c_int,
                    __p_ref.funcFlags & mask ^ crate::sqlite3_h::SQLITE_INNOCUOUS as crate::src::ext::rtree::rtree::u32_0,
                );
            }
        }
        p = (*p).pNext;
    }
}

unsafe extern "C" fn integrityCheckResultRow(mut v: *mut crate::vdbeInt_h::Vdbe) -> ::core::ffi::c_int {
    let mut addr: ::core::ffi::c_int = 0;
    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
        v,
        crate::opcodes_h::OP_ResultRow,
        3 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    addr = crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
        v,
        crate::opcodes_h::OP_IfPos,
        1 as ::core::ffi::c_int,
        crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    crate::src::src::vdbeaux::sqlite3VdbeAddOp0(v, crate::opcodes_h::OP_Halt);
    return addr;
}

unsafe extern "C" fn tableSkipIntegrityCheck(
    mut pTab: *const crate::sqliteInt_h::Table,
    mut pObjTab: *const crate::sqliteInt_h::Table,
) -> ::core::ffi::c_int {
    if !pObjTab.is_null() {
        return (pTab != pObjTab) as ::core::ffi::c_int;
    } else {
        return ((*pTab).tabFlags & crate::sqliteInt_h::TF_Imposter as crate::src::ext::rtree::rtree::u32_0 != 0 as crate::src::ext::rtree::rtree::u32_0) as ::core::ffi::c_int;
    };
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Pragma(
    mut pParse: *mut crate::sqliteInt_h::Parse,
    mut pId1: *mut crate::sqliteInt_h::Token,
    mut pId2: *mut crate::sqliteInt_h::Token,
    mut pValue: *mut crate::sqliteInt_h::Token,
    mut minusFlag: ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut zLeft: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zRight: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut pId: *mut crate::sqliteInt_h::Token = ::core::ptr::null_mut::<crate::sqliteInt_h::Token>();
    let mut aFcntl: [*mut ::core::ffi::c_char; 4] =
        [::core::ptr::null_mut::<::core::ffi::c_char>(); 4];
    let mut iDb: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0;
    let mut db: *mut crate::sqliteInt_h::sqlite3 = (*pParse).db;
    let mut pDb: *mut crate::sqliteInt_h::Db = ::core::ptr::null_mut::<crate::sqliteInt_h::Db>();
    let mut v: *mut crate::vdbeInt_h::Vdbe = crate::src::src::select::sqlite3GetVdbe(pParse as *mut crate::sqliteInt_h::Parse);
    let mut pPragma: *const crate::src::src::pragma::PragmaName = ::core::ptr::null::<crate::src::src::pragma::PragmaName>();
    if v.is_null() {
        return;
    }
    crate::src::src::vdbeaux::sqlite3VdbeRunOnlyOnce(v);
    (*pParse).nMem = 2 as ::core::ffi::c_int;
    iDb = crate::src::src::build::sqlite3TwoPartName(pParse as *mut crate::sqliteInt_h::Parse,  pId1 as *mut crate::sqliteInt_h::Token,  pId2 as *mut crate::sqliteInt_h::Token,  &raw mut pId as *mut _ as *mut *mut crate::sqliteInt_h::Token);
    if iDb < 0 as ::core::ffi::c_int {
        return;
    }
    pDb = (*db).aDb.offset(iDb as isize) as *mut crate::sqliteInt_h::Db;
    if iDb == 1 as ::core::ffi::c_int && crate::src::src::build::sqlite3OpenTempDatabase(pParse as *mut crate::sqliteInt_h::Parse) != 0 {
        return;
    }
    zLeft = crate::src::src::build::sqlite3NameFromToken(db as *mut crate::sqliteInt_h::sqlite3,  pId as *const crate::sqliteInt_h::Token);
    if zLeft.is_null() {
        return;
    }
    if minusFlag != 0 {
        zRight = crate::src::src::printf::sqlite3MPrintf(
            
            db as *mut crate::sqliteInt_h::sqlite3,
            b"-%T\0" as *const u8 as *const ::core::ffi::c_char,
            pValue,
        );
    } else {
        zRight = crate::src::src::build::sqlite3NameFromToken(db as *mut crate::sqliteInt_h::sqlite3,  pValue as *const crate::sqliteInt_h::Token);
    }
    zDb = if (*pId2).n > 0 as ::core::ffi::c_uint {
        (*pDb).zDbSName
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    if !(crate::src::src::auth::sqlite3AuthCheck(pParse as *mut crate::sqliteInt_h::Parse, crate::sqlite3_h::SQLITE_PRAGMA, zLeft, zRight, zDb) != 0) {
        aFcntl[0 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
        aFcntl[1 as ::core::ffi::c_int as usize] = zLeft;
        aFcntl[2 as ::core::ffi::c_int as usize] = zRight;
        aFcntl[3 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*db).busyHandler.nBusy = 0 as ::core::ffi::c_int;
        rc = crate::src::src::main::sqlite3_file_control(
            
            db as *mut crate::sqliteInt_h::sqlite3,
            zDb,
            crate::sqlite3_h::SQLITE_FCNTL_PRAGMA,
            &raw mut aFcntl as *mut *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        );
        if rc == crate::sqlite3_h::SQLITE_OK {
            crate::src::src::vdbeaux::sqlite3VdbeSetNumCols(v, 1 as ::core::ffi::c_int);
            crate::src::src::vdbeaux::sqlite3VdbeSetColName(
                v,
                0 as ::core::ffi::c_int,
                crate::src::src::vdbe::COLNAME_NAME,
                aFcntl[0 as ::core::ffi::c_int as usize],
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
            );
            returnSingleText(v, aFcntl[0 as ::core::ffi::c_int as usize]);
            crate::src::src::malloc::sqlite3_free(aFcntl[0 as ::core::ffi::c_int as usize] as *mut ::core::ffi::c_void);
        } else if rc != crate::sqlite3_h::SQLITE_NOTFOUND {
            if !aFcntl[0 as ::core::ffi::c_int as usize].is_null() {
                crate::src::src::util::sqlite3ErrorMsg(
                    
                    pParse as *mut crate::sqliteInt_h::Parse,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    aFcntl[0 as ::core::ffi::c_int as usize],
                );
                crate::src::src::malloc::sqlite3_free(aFcntl[0 as ::core::ffi::c_int as usize] as *mut ::core::ffi::c_void);
            }
            (*pParse).nErr += 1;
            (*pParse).rc = rc;
        } else {
            pPragma = pragmaLocate(zLeft);
            if !pPragma.is_null() {
                if (*pPragma).mPragFlg as ::core::ffi::c_int & crate::src::src::pragma::PragFlg_NeedSchema
                    != 0 as ::core::ffi::c_int
                {
                    if crate::src::src::prepare::sqlite3ReadSchema(pParse as *mut crate::sqliteInt_h::Parse) != 0 {
                        current_block = 9454828942646539263;
                    } else {
                        current_block = 8845338526596852646;
                    }
                } else {
                    current_block = 8845338526596852646;
                }
                match current_block {
                    9454828942646539263 => {}
                    _ => {
                        let __pPragma_ref = unsafe { &*pPragma };
                        if __pPragma_ref.mPragFlg as ::core::ffi::c_int & crate::src::src::pragma::PragFlg_NoColumns
                            == 0 as ::core::ffi::c_int
                            && (__pPragma_ref.mPragFlg as ::core::ffi::c_int & crate::src::src::pragma::PragFlg_NoColumns1
                                == 0 as ::core::ffi::c_int
                                || zRight.is_null())
                        {
                            setPragmaResultColumnNames(v, pPragma);
                        }
                        match  __pPragma_ref.ePragTyp as ::core::ffi::c_int {
    crate::src::src::pragma::PragTyp_DEFAULT_CACHE_SIZE =>  {
                                static mut iLn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                static mut getCacheSize: [crate::src::src::vdbe::VdbeOpList; 9] = [
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_Transaction as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  0 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_ReadCookie as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  1 as ::core::ffi::c_schar,
    p3:  crate::src::src::btree::BTREE_DEFAULT_CACHE_SIZE as ::core::ffi::c_schar,
},
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_IfPos as crate::src::ext::rtree::rtree::u8_0,
    p1:  1 as ::core::ffi::c_schar,
    p2:  8 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_Integer as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  2 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_Subtract as crate::src::ext::rtree::rtree::u8_0,
    p1:  1 as ::core::ffi::c_schar,
    p2:  2 as ::core::ffi::c_schar,
    p3:  1 as ::core::ffi::c_schar,
},
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_IfPos as crate::src::ext::rtree::rtree::u8_0,
    p1:  1 as ::core::ffi::c_schar,
    p2:  8 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_Integer as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  1 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_Noop as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  0 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_ResultRow as crate::src::ext::rtree::rtree::u8_0,
    p1:  1 as ::core::ffi::c_schar,
    p2:  1 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                ];
                                let mut aOp: *mut crate::src::src::vdbe::VdbeOp = ::core::ptr::null_mut::<crate::src::src::vdbe::VdbeOp>();
                                crate::src::src::vdbeaux::sqlite3VdbeUsesBtree(v, iDb);
                                if zRight.is_null() {
                                    (*pParse).nMem += 2 as ::core::ffi::c_int;
                                    aOp =  crate::src::src::vdbeaux::sqlite3VdbeAddOpList(
                                        v,
                                        (::core::mem::size_of::<[crate::src::src::vdbe::VdbeOpList; 9]>() as usize)
                                            .wrapping_div(
                                                ::core::mem::size_of::<crate::src::src::vdbe::VdbeOpList>() as usize
                                            )
                                            as ::core::ffi::c_int,
                                        
                                        &raw const getCacheSize as *const crate::src::src::vdbe::VdbeOpList as *const crate::src::src::vdbe::VdbeOpList,
                                        iLn,
                                    ) as
    *mut crate::src::src::vdbe::VdbeOp;
                                    (*aOp.offset(0 as isize)).p1 = iDb;
                                    (*aOp.offset(1 as isize)).p1 = iDb;
                                    (*aOp.offset(6 as isize)).p1 =
                                        crate::sqliteLimit_h::SQLITE_DEFAULT_CACHE_SIZE;
                                } else {
                                    let mut size: ::core::ffi::c_int =
                                        crate::src::src::util::sqlite3AbsInt32(crate::src::src::util::sqlite3Atoi(zRight));
                                    crate::src::src::build::sqlite3BeginWriteOperation(
                                        
                                        pParse as *mut crate::sqliteInt_h::Parse,
                                        0 as ::core::ffi::c_int,
                                        iDb,
                                    );
                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                        v,
                                        crate::opcodes_h::OP_SetCookie,
                                        iDb,
                                        crate::src::src::btree::BTREE_DEFAULT_CACHE_SIZE,
                                        size,
                                    );
                                    let __pDb_ref = unsafe { &mut *pDb };
                                    (*__pDb_ref.pSchema).cache_size = size;
                                    crate::src::src::btree::sqlite3BtreeSetCacheSize(
                                        __pDb_ref.pBt,
                                        (*__pDb_ref.pSchema).cache_size,
                                    );
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_PAGE_SIZE =>  {
                                let mut pBt: *mut crate::btreeInt_h::Btree = (*pDb).pBt;
                                if zRight.is_null() {
                                    let mut size_0: ::core::ffi::c_int = if !pBt.is_null() {
                                        crate::src::src::btree::sqlite3BtreeGetPageSize(pBt)
                                    } else {
                                        0 as ::core::ffi::c_int
                                    };
                                    returnSingleInt(v, size_0 as crate::src::ext::rtree::rtree::i64_0);
                                } else {
                                    (*db).nextPagesize = crate::src::src::util::sqlite3Atoi(zRight);
                                    if crate::sqlite3_h::SQLITE_NOMEM
                                        == crate::src::src::btree::sqlite3BtreeSetPageSize(
                                            pBt,
                                            (*db).nextPagesize,
                                            0 as ::core::ffi::c_int,
                                            0 as ::core::ffi::c_int,
                                        )
                                    {
                                        crate::src::src::malloc::sqlite3OomFault(db as *mut crate::sqliteInt_h::sqlite3);
                                    }
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_SECURE_DELETE =>  {
                                let mut pBt_0: *mut crate::btreeInt_h::Btree = (*pDb).pBt;
                                let mut b: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
                                if !zRight.is_null() {
                                    if crate::src::src::util::sqlite3_stricmp(
                                        zRight,
                                        b"fast\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        b = 2 as ::core::ffi::c_int;
                                    } else {
                                        b = sqlite3GetBoolean(zRight, 0 as crate::src::ext::rtree::rtree::u8_0)
                                            as ::core::ffi::c_int;
                                    }
                                }
                                if (*pId2).n == 0 as ::core::ffi::c_uint
                                    && b >= 0 as ::core::ffi::c_int
                                {
                                    let mut ii: ::core::ffi::c_int = 0;
                                    ii = 0 as ::core::ffi::c_int;
                                    while ii < (*db).nDb {
                                        crate::src::src::btree::sqlite3BtreeSecureDelete(
                                            (*(*db).aDb.offset(ii as isize)).pBt,
                                            b,
                                        );
                                        ii += 1;
                                    }
                                }
                                b = crate::src::src::btree::sqlite3BtreeSecureDelete(pBt_0, b);
                                returnSingleInt(v, b as crate::src::ext::rtree::rtree::i64_0);
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_PAGE_COUNT =>  {
                                let mut iReg: ::core::ffi::c_int = 0;
                                let mut x: crate::src::ext::rtree::rtree::i64_0 = 0 as crate::src::ext::rtree::rtree::i64_0;
                                crate::src::src::build::sqlite3CodeVerifySchema(pParse as *mut crate::sqliteInt_h::Parse, iDb);
                                (*pParse).nMem += 1;
                                iReg = (*pParse).nMem;
                                if *(&raw const crate::src::src::global::sqlite3UpperToLower as *const ::core::ffi::c_uchar)
                                    .offset(*zLeft.offset(0 as isize)
                                        as ::core::ffi::c_uchar
                                        as isize)
                                    as ::core::ffi::c_int
                                    == 'p' as i32
                                {
                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(v, crate::opcodes_h::OP_Pagecount, iDb, iReg);
                                } else {
                                    if !zRight.is_null()
                                        && crate::src::src::util::sqlite3DecOrHexToI64(zRight, &raw mut x)
                                            == 0 as ::core::ffi::c_int
                                    {
                                        if x < 0 as crate::src::ext::rtree::rtree::i64_0 {
                                            x = 0 as crate::src::ext::rtree::rtree::i64_0;
                                        } else if x > 0xfffffffe as crate::src::ext::rtree::rtree::i64_0 {
                                            x = 0xfffffffe as crate::src::ext::rtree::rtree::i64_0;
                                        }
                                    } else {
                                        x = 0 as crate::src::ext::rtree::rtree::i64_0;
                                    }
                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                        v,
                                        crate::opcodes_h::OP_MaxPgcnt,
                                        iDb,
                                        iReg,
                                        x as ::core::ffi::c_int,
                                    );
                                }
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(v, crate::opcodes_h::OP_ResultRow, iReg, 1 as ::core::ffi::c_int);
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_LOCKING_MODE =>  {
                                let mut zRet: *const ::core::ffi::c_char =
                                    b"normal\0" as *const u8 as *const ::core::ffi::c_char;
                                let mut eMode: ::core::ffi::c_int = getLockingMode(zRight);
                                if (*pId2).n == 0 as ::core::ffi::c_uint
                                    && eMode == crate::src::src::pager::PAGER_LOCKINGMODE_QUERY
                                {
                                    eMode = (*db).dfltLockMode as ::core::ffi::c_int;
                                } else {
                                    let mut pPager: *mut crate::src::src::pager::Pager = ::core::ptr::null_mut::<crate::src::src::pager::Pager>();
                                    if (*pId2).n == 0 as ::core::ffi::c_uint {
                                        let mut ii_0: ::core::ffi::c_int = 0;
                                        ii_0 = 2 as ::core::ffi::c_int;
                                        while ii_0 < (*db).nDb {
                                            pPager = crate::src::src::btree::sqlite3BtreePager(
                                                (*(*db).aDb.offset(ii_0 as isize)).pBt,
                                            )
                                                as *mut crate::src::src::pager::Pager;
                                            crate::src::src::pager::sqlite3PagerLockingMode(pPager, eMode);
                                            ii_0 += 1;
                                        }
                                        (*db).dfltLockMode = eMode as crate::src::ext::rtree::rtree::u8_0;
                                    }
                                    pPager = crate::src::src::btree::sqlite3BtreePager((*pDb).pBt) as *mut crate::src::src::pager::Pager;
                                    eMode = crate::src::src::pager::sqlite3PagerLockingMode(pPager, eMode);
                                }
                                if eMode == crate::src::src::pager::PAGER_LOCKINGMODE_EXCLUSIVE {
                                    zRet =
                                        b"exclusive\0" as *const u8 as *const ::core::ffi::c_char;
                                }
                                returnSingleText(v, zRet);
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_JOURNAL_MODE =>  {
                                let mut eMode_0: ::core::ffi::c_int = 0;
                                let mut ii_1: ::core::ffi::c_int = 0;
                                if zRight.is_null() {
                                    eMode_0 = crate::src::src::pager::PAGER_JOURNALMODE_QUERY;
                                } else {
                                    let mut zMode: *const ::core::ffi::c_char =
                                        ::core::ptr::null::<::core::ffi::c_char>();
                                    let mut n: ::core::ffi::c_int = crate::src::src::util::sqlite3Strlen30(zRight);
                                    eMode_0 = 0 as ::core::ffi::c_int;
                                    loop {
                                        zMode = sqlite3JournalModename(eMode_0);
                                        if zMode.is_null() {
                                            break;
                                        }
                                        if crate::src::src::util::sqlite3_strnicmp(zRight, zMode, n)
                                            == 0 as ::core::ffi::c_int
                                        {
                                            break;
                                        }
                                        eMode_0 += 1;
                                    }
                                    if zMode.is_null() {
                                        eMode_0 = crate::src::src::pager::PAGER_JOURNALMODE_QUERY;
                                    }
                                    if eMode_0 == crate::src::src::pager::PAGER_JOURNALMODE_OFF
                                        && (*db).flags & crate::sqliteInt_h::SQLITE_Defensive as crate::src::ext::rtree::rtree::u64_0 != 0 as crate::src::ext::rtree::rtree::u64_0
                                    {
                                        eMode_0 = crate::src::src::pager::PAGER_JOURNALMODE_QUERY;
                                    }
                                }
                                if eMode_0 == crate::src::src::pager::PAGER_JOURNALMODE_QUERY
                                    && (*pId2).n == 0 as ::core::ffi::c_uint
                                {
                                    iDb = 0 as ::core::ffi::c_int;
                                    (*pId2).n = 1 as ::core::ffi::c_uint;
                                }
                                ii_1 = (*db).nDb - 1 as ::core::ffi::c_int;
                                while ii_1 >= 0 as ::core::ffi::c_int {
                                    if !(*(*db).aDb.offset(ii_1 as isize)).pBt.is_null()
                                        && (ii_1 == iDb || (*pId2).n == 0 as ::core::ffi::c_uint)
                                    {
                                        crate::src::src::vdbeaux::sqlite3VdbeUsesBtree(v, ii_1);
                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                            v,
                                            crate::opcodes_h::OP_JournalMode,
                                            ii_1,
                                            1 as ::core::ffi::c_int,
                                            eMode_0,
                                        );
                                    }
                                    ii_1 -= 1;
                                }
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                    v,
                                    crate::opcodes_h::OP_ResultRow,
                                    1 as ::core::ffi::c_int,
                                    1 as ::core::ffi::c_int,
                                );
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_JOURNAL_SIZE_LIMIT =>  {
                                let mut pPager_0: *mut crate::src::src::pager::Pager =
                                    crate::src::src::btree::sqlite3BtreePager((*pDb).pBt) as *mut crate::src::src::pager::Pager;
                                let mut iLimit: crate::src::ext::rtree::rtree::i64_0 = -(2 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0;
                                if !zRight.is_null() {
                                    crate::src::src::util::sqlite3DecOrHexToI64(zRight, &raw mut iLimit);
                                    if iLimit < -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0 {
                                        iLimit = -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0;
                                    }
                                }
                                iLimit = crate::src::src::pager::sqlite3PagerJournalSizeLimit(pPager_0, iLimit);
                                returnSingleInt(v, iLimit);
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_AUTO_VACUUM =>  {
                                let mut pBt_1: *mut crate::btreeInt_h::Btree = (*pDb).pBt;
                                if zRight.is_null() {
                                    returnSingleInt(v, crate::src::src::btree::sqlite3BtreeGetAutoVacuum(pBt_1) as crate::src::ext::rtree::rtree::i64_0);
                                } else {
                                    let mut eAuto: ::core::ffi::c_int = getAutoVacuum(zRight);
                                    (*db).nextAutovac = eAuto as crate::src::ext::rtree::rtree::u8_0 as ::core::ffi::c_schar;
                                    rc = crate::src::src::btree::sqlite3BtreeSetAutoVacuum(pBt_1, eAuto);
                                    if rc == crate::sqlite3_h::SQLITE_OK
                                        && (eAuto == 1 as ::core::ffi::c_int
                                            || eAuto == 2 as ::core::ffi::c_int)
                                    {
                                        static mut iLn_0: ::core::ffi::c_int =
                                            0 as ::core::ffi::c_int;
                                        static mut setMeta6: [crate::src::src::vdbe::VdbeOpList; 5] = [
                                            crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_Transaction as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  1 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                            crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_ReadCookie as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  1 as ::core::ffi::c_schar,
    p3:  crate::src::src::btree::BTREE_LARGEST_ROOT_PAGE as ::core::ffi::c_schar,
},
                                            crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_If as crate::src::ext::rtree::rtree::u8_0,
    p1:  1 as ::core::ffi::c_schar,
    p2:  0 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                            crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_Halt as crate::src::ext::rtree::rtree::u8_0,
    p1:  crate::sqlite3_h::SQLITE_OK as ::core::ffi::c_schar,
    p2:  crate::sqliteInt_h::OE_Abort as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                            crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_SetCookie as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  crate::src::src::btree::BTREE_INCR_VACUUM as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                        ];
                                        let mut aOp_0: *mut crate::src::src::vdbe::VdbeOp =
                                            ::core::ptr::null_mut::<crate::src::src::vdbe::VdbeOp>();
                                        let mut iAddr: ::core::ffi::c_int =
                                            crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v);
                                        aOp_0 =  crate::src::src::vdbeaux::sqlite3VdbeAddOpList(
                                            v,
                                            (::core::mem::size_of::<[crate::src::src::vdbe::VdbeOpList; 5]>() as usize)
                                                .wrapping_div(
                                                    ::core::mem::size_of::<crate::src::src::vdbe::VdbeOpList>() as usize
                                                )
                                                as ::core::ffi::c_int,
                                            
                                            &raw const setMeta6 as *const crate::src::src::vdbe::VdbeOpList as *const crate::src::src::vdbe::VdbeOpList,
                                            iLn_0,
                                        ) as
    *mut crate::src::src::vdbe::VdbeOp;
                                        (*aOp_0.offset(0 as isize)).p1 = iDb;
                                        (*aOp_0.offset(1 as isize)).p1 = iDb;
                                        (*aOp_0.offset(2 as isize)).p2 =
                                            iAddr + 4 as ::core::ffi::c_int;
                                        (*aOp_0.offset(4 as isize)).p1 = iDb;
                                        (*aOp_0.offset(4 as isize)).p3 =
                                            eAuto - 1 as ::core::ffi::c_int;
                                        crate::src::src::vdbeaux::sqlite3VdbeUsesBtree(v, iDb);
                                    }
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_INCREMENTAL_VACUUM =>  {
                                let mut iLimit_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                let mut addr: ::core::ffi::c_int = 0;
                                if zRight.is_null()
                                    || crate::src::src::util::sqlite3GetInt32(zRight, &raw mut iLimit_0) == 0
                                    || iLimit_0 <= 0 as ::core::ffi::c_int
                                {
                                    iLimit_0 = 0x7fffffff as ::core::ffi::c_int;
                                }
                                crate::src::src::build::sqlite3BeginWriteOperation(pParse as *mut crate::sqliteInt_h::Parse, 0 as ::core::ffi::c_int, iDb);
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(v, crate::opcodes_h::OP_Integer, iLimit_0, 1 as ::core::ffi::c_int);
                                addr = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(v, crate::opcodes_h::OP_IncrVacuum, iDb);
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp1(v, crate::opcodes_h::OP_ResultRow, 1 as ::core::ffi::c_int);
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                    v,
                                    crate::opcodes_h::OP_AddImm,
                                    1 as ::core::ffi::c_int,
                                    -(1 as ::core::ffi::c_int),
                                );
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(v, crate::opcodes_h::OP_IfPos, 1 as ::core::ffi::c_int, addr);
                                crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addr);
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_CACHE_SIZE =>  {
                                if zRight.is_null() {
                                    returnSingleInt(v, (*(*pDb).pSchema).cache_size as crate::src::ext::rtree::rtree::i64_0);
                                } else {
                                    let mut size_1: ::core::ffi::c_int = crate::src::src::util::sqlite3Atoi(zRight);
                                    let __pDb_ref = unsafe { &mut *pDb };
                                    (*__pDb_ref.pSchema).cache_size = size_1;
                                    crate::src::src::btree::sqlite3BtreeSetCacheSize(
                                        __pDb_ref.pBt,
                                        (*__pDb_ref.pSchema).cache_size,
                                    );
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_CACHE_SPILL =>  {
                                if zRight.is_null() {
                                    returnSingleInt(
                                        v,
                                        (if (*db).flags & crate::sqliteInt_h::SQLITE_CacheSpill as crate::src::ext::rtree::rtree::u64_0 == 0 as crate::src::ext::rtree::rtree::u64_0 {
                                            0 as ::core::ffi::c_int
                                        } else {
                                            crate::src::src::btree::sqlite3BtreeSetSpillSize(
                                                (*pDb).pBt,
                                                0 as ::core::ffi::c_int,
                                            )
                                        }) as crate::src::ext::rtree::rtree::i64_0,
                                    );
                                } else {
                                    let mut size_2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                                    if crate::src::src::util::sqlite3GetInt32(zRight, &raw mut size_2) != 0 {
                                        crate::src::src::btree::sqlite3BtreeSetSpillSize((*pDb).pBt, size_2);
                                    }
                                    if sqlite3GetBoolean(
                                        zRight,
                                        (size_2 != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                                            as crate::src::ext::rtree::rtree::u8_0,
                                    ) != 0
                                    {
                                        (*db).flags |= crate::sqliteInt_h::SQLITE_CacheSpill as crate::src::ext::rtree::rtree::u64_0;
                                    } else {
                                        (*db).flags &= !(crate::sqliteInt_h::SQLITE_CacheSpill as crate::src::ext::rtree::rtree::u64_0);
                                    }
                                    setAllPagerFlags(db);
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_MMAP_SIZE =>  {
                                let mut sz: crate::sqlite3_h::sqlite3_int64 = 0;
                                if !zRight.is_null() {
                                    let mut ii_2: ::core::ffi::c_int = 0;
                                    crate::src::src::util::sqlite3DecOrHexToI64(zRight, &raw mut sz);
                                    if sz < 0 as crate::sqlite3_h::sqlite3_int64 {
                                        sz = crate::src::src::global::sqlite3Config.szMmap;
                                    }
                                    if (*pId2).n == 0 as ::core::ffi::c_uint {
                                        (*db).szMmap = sz as crate::src::ext::rtree::rtree::i64_0;
                                    }
                                    ii_2 = (*db).nDb - 1 as ::core::ffi::c_int;
                                    while ii_2 >= 0 as ::core::ffi::c_int {
                                        if !(*(*db).aDb.offset(ii_2 as isize)).pBt.is_null()
                                            && (ii_2 == iDb
                                                || (*pId2).n == 0 as ::core::ffi::c_uint)
                                        {
                                            crate::src::src::btree::sqlite3BtreeSetMmapLimit(
                                                (*(*db).aDb.offset(ii_2 as isize)).pBt,
                                                sz,
                                            );
                                        }
                                        ii_2 -= 1;
                                    }
                                }
                                sz = -(1 as ::core::ffi::c_int) as crate::sqlite3_h::sqlite3_int64;
                                rc = crate::src::src::main::sqlite3_file_control(
                                    
                                    db as *mut crate::sqliteInt_h::sqlite3,
                                    zDb,
                                    crate::sqlite3_h::SQLITE_FCNTL_MMAP_SIZE_1,
                                    &raw mut sz as *mut ::core::ffi::c_void,
                                );
                                if rc == crate::sqlite3_h::SQLITE_OK {
                                    returnSingleInt(v, sz as crate::src::ext::rtree::rtree::i64_0);
                                } else if rc != crate::sqlite3_h::SQLITE_NOTFOUND {
                                    (*pParse).nErr += 1;
                                    (*pParse).rc = rc;
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_TEMP_STORE =>  {
                                if zRight.is_null() {
                                    returnSingleInt(v, (*db).temp_store as crate::src::ext::rtree::rtree::i64_0);
                                } else {
                                    changeTempStorage(pParse, zRight);
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_TEMP_STORE_DIRECTORY =>  {
                                crate::src::src::mutex::sqlite3_mutex_enter(crate::src::src::mutex::sqlite3MutexAlloc(crate::sqliteInt_h::SQLITE_MUTEX_STATIC_TEMPDIR));
                                if zRight.is_null() {
                                    returnSingleText(v, crate::src::src::main::sqlite3_temp_directory);
                                    current_block = 6678684093116635837;
                                } else {
                                    if *zRight.offset(0 as isize) != 0 {
                                        let mut res: ::core::ffi::c_int = 0;
                                        rc = crate::src::src::os::sqlite3OsAccess(
                                            
                                            (*db).pVfs as *mut crate::sqlite3_h::sqlite3_vfs,
                                            zRight,
                                            crate::sqlite3_h::SQLITE_ACCESS_READWRITE,
                                            &raw mut res,
                                        );
                                        if rc != crate::sqlite3_h::SQLITE_OK || res == 0 as ::core::ffi::c_int {
                                            crate::src::src::util::sqlite3ErrorMsg(
                                                
                                                pParse as *mut crate::sqliteInt_h::Parse,
                                                b"not a writable directory\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                            crate::src::src::mutex::sqlite3_mutex_leave(crate::src::src::mutex::sqlite3MutexAlloc(
                                                crate::sqliteInt_h::SQLITE_MUTEX_STATIC_TEMPDIR,
                                            ));
                                            current_block = 9454828942646539263;
                                        } else {
                                            current_block = 4497948414247713119;
                                        }
                                    } else {
                                        current_block = 4497948414247713119;
                                    }
                                    match current_block {
                                        9454828942646539263 => {}
                                        _ => {
                                            if crate::sqliteInt_h::SQLITE_TEMP_STORE == 0 as ::core::ffi::c_int
                                                || crate::sqliteInt_h::SQLITE_TEMP_STORE == 1 as ::core::ffi::c_int
                                                    && (*db).temp_store as ::core::ffi::c_int
                                                        <= 1 as ::core::ffi::c_int
                                                || crate::sqliteInt_h::SQLITE_TEMP_STORE == 2 as ::core::ffi::c_int
                                                    && (*db).temp_store as ::core::ffi::c_int
                                                        == 1 as ::core::ffi::c_int
                                            {
                                                invalidateTempStorage(pParse);
                                            }
                                            crate::src::src::malloc::sqlite3_free(
                                                crate::src::src::main::sqlite3_temp_directory as *mut ::core::ffi::c_void,
                                            );
                                            if *zRight.offset(0 as isize) != 0
                                            {
                                                crate::src::src::main::sqlite3_temp_directory = crate::src::src::printf::sqlite3_mprintf(
                                                    b"%s\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    zRight,
                                                );
                                            } else {
                                                crate::src::src::main::sqlite3_temp_directory =
                                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                            }
                                            current_block = 6678684093116635837;
                                        }
                                    }
                                }
                                match current_block {
                                    9454828942646539263 => {}
                                    _ => {
                                        crate::src::src::mutex::sqlite3_mutex_leave(crate::src::src::mutex::sqlite3MutexAlloc(
                                            crate::sqliteInt_h::SQLITE_MUTEX_STATIC_TEMPDIR,
                                        ));
                                        current_block = 17607975748632905087;
                                    }
                                }
                            }
    crate::src::src::pragma::PragTyp_SYNCHRONOUS =>  {
                                if zRight.is_null() {
                                    returnSingleInt(
                                        v,
                                        ((*pDb).safety_level as ::core::ffi::c_int
                                            - 1 as ::core::ffi::c_int)
                                            as crate::src::ext::rtree::rtree::i64_0,
                                    );
                                } else if (*db).autoCommit == 0 {
                                    crate::src::src::util::sqlite3ErrorMsg(
                                        
                                        pParse as *mut crate::sqliteInt_h::Parse,
                                        b"Safety level may not be changed inside a transaction\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                } else if iDb != 1 as ::core::ffi::c_int {
                                    let mut iLevel: ::core::ffi::c_int =
                                        getSafetyLevel(zRight, 0 as ::core::ffi::c_int, 1 as crate::src::ext::rtree::rtree::u8_0)
                                            as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int
                                            & crate::src::src::pager::PAGER_SYNCHRONOUS_MASK;
                                    if iLevel == 0 as ::core::ffi::c_int {
                                        iLevel = 1 as ::core::ffi::c_int;
                                    }
                                    (*pDb).safety_level = iLevel as crate::src::ext::rtree::rtree::u8_0;
                                    (*pDb).bSyncSet = 1 as crate::src::ext::rtree::rtree::u8_0;
                                    setAllPagerFlags(db);
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_FLAG =>  {
                                if zRight.is_null() {
                                    setPragmaResultColumnNames(v, pPragma);
                                    returnSingleInt(
                                        v,
                                        ((*db).flags & __pPragma_ref.iArg != 0 as crate::src::ext::rtree::rtree::u64_0)
                                            as ::core::ffi::c_int
                                            as crate::src::ext::rtree::rtree::i64_0,
                                    );
                                } else {
                                    let mut mask: crate::src::ext::rtree::rtree::u64_0 = __pPragma_ref.iArg;
                                    if (*db).autoCommit as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int
                                    {
                                        mask &= !(0x4000 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::u64_0;
                                    }
                                    if sqlite3GetBoolean(zRight, 0 as crate::src::ext::rtree::rtree::u8_0) != 0 {
                                        if mask & crate::sqliteInt_h::SQLITE_WriteSchema as crate::src::ext::rtree::rtree::u64_0 == 0 as crate::src::ext::rtree::rtree::u64_0
                                            || (*db).flags & crate::sqliteInt_h::SQLITE_Defensive as crate::src::ext::rtree::rtree::u64_0 == 0 as crate::src::ext::rtree::rtree::u64_0
                                        {
                                            (*db).flags |= mask;
                                        }
                                    } else {
                                        (*db).flags &= !mask;
                                        if mask == crate::sqliteInt_h::SQLITE_DeferFKs as crate::src::ext::rtree::rtree::u64_0 {
                                            (*db).nDeferredImmCons = 0 as crate::src::ext::rtree::rtree::i64_0;
                                            (*db).nDeferredCons = 0 as crate::src::ext::rtree::rtree::i64_0;
                                        }
                                        if mask & crate::sqliteInt_h::SQLITE_WriteSchema as crate::src::ext::rtree::rtree::u64_0 != 0 as crate::src::ext::rtree::rtree::u64_0
                                            && crate::src::src::util::sqlite3_stricmp(
                                                zRight,
                                                b"reset\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            ) == 0 as ::core::ffi::c_int
                                        {
                                            crate::src::src::build::sqlite3ResetAllSchemasOfConnection(db as *mut crate::sqliteInt_h::sqlite3);
                                        }
                                    }
                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp0(v, crate::opcodes_h::OP_Expire);
                                    setAllPagerFlags(db);
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_TABLE_INFO =>  {
                                if !zRight.is_null() {
                                    let mut pTab: *mut crate::sqliteInt_h::Table = ::core::ptr::null_mut::<crate::sqliteInt_h::Table>();
                                    crate::src::src::build::sqlite3CodeVerifyNamedSchema(pParse as *mut crate::sqliteInt_h::Parse, zDb);
                                    pTab =  crate::src::src::build::sqlite3LocateTable(
                                        
                                        pParse as *mut crate::sqliteInt_h::Parse,
                                        crate::sqliteInt_h::LOCATE_NOERR as crate::src::ext::rtree::rtree::u32_0,
                                        zRight,
                                        zDb,
                                    ) as
    *mut crate::sqliteInt_h::Table;
                                    if !pTab.is_null() {
                                        let mut i: ::core::ffi::c_int = 0;
                                        let mut k: ::core::ffi::c_int = 0;
                                        let mut nHidden: ::core::ffi::c_int =
                                            0 as ::core::ffi::c_int;
                                        let mut pCol: *mut crate::sqliteInt_h::Column =
                                            ::core::ptr::null_mut::<crate::sqliteInt_h::Column>();
                                        let mut pPk: *mut crate::sqliteInt_h::Index =  crate::src::src::build::sqlite3PrimaryKeyIndex(pTab as *mut crate::sqliteInt_h::Table) as *mut crate::sqliteInt_h::Index;
                                        (*pParse).nMem = 7 as ::core::ffi::c_int;
                                        crate::src::src::build::sqlite3ViewGetColumnNames(pParse as *mut crate::sqliteInt_h::Parse,  pTab as *mut crate::sqliteInt_h::Table);
                                        let mut current_block_303: u64;
                                        i = 0 as ::core::ffi::c_int;
                                        pCol = (*pTab).aCol;
                                        while i < (*pTab).nCol as ::core::ffi::c_int {
                                            let mut isHidden: ::core::ffi::c_int =
                                                0 as ::core::ffi::c_int;
                                            let mut pColExpr: *const crate::sqliteInt_h::Expr =
                                                ::core::ptr::null::<crate::sqliteInt_h::Expr>();
                                            if (*pCol).colFlags as ::core::ffi::c_int
                                                & crate::sqliteInt_h::COLFLAG_NOINSERT
                                                != 0
                                            {
                                                if __pPragma_ref.iArg == 0 as crate::src::ext::rtree::rtree::u64_0 {
                                                    nHidden += 1;
                                                    current_block_303 = 10453323034968249808;
                                                } else {
                                                    if (*pCol).colFlags as ::core::ffi::c_int
                                                        & crate::sqliteInt_h::COLFLAG_VIRTUAL
                                                        != 0
                                                    {
                                                        isHidden = 2 as ::core::ffi::c_int;
                                                    } else if (*pCol).colFlags as ::core::ffi::c_int
                                                        & crate::sqliteInt_h::COLFLAG_STORED
                                                        != 0
                                                    {
                                                        isHidden = 3 as ::core::ffi::c_int;
                                                    } else {
                                                        isHidden = 1 as ::core::ffi::c_int;
                                                    }
                                                    current_block_303 = 4839309778395429725;
                                                }
                                            } else {
                                                current_block_303 = 4839309778395429725;
                                            }
                                            match current_block_303 {
                                                4839309778395429725 => {
                                                    let __pCol_ref = unsafe { &mut *pCol };
                                                    if __pCol_ref.colFlags as ::core::ffi::c_int
                                                        & crate::sqliteInt_h::COLFLAG_PRIMKEY
                                                        == 0 as ::core::ffi::c_int
                                                    {
                                                        k = 0 as ::core::ffi::c_int;
                                                    } else if pPk.is_null() {
                                                        k = 1 as ::core::ffi::c_int;
                                                    } else {
                                                        k = 1 as ::core::ffi::c_int;
                                                        while k
                                                            <= (*pTab).nCol as ::core::ffi::c_int
                                                            && *(*pPk).aiColumn.offset(
                                                                (k - 1 as ::core::ffi::c_int)
                                                                    as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != i
                                                        {
                                                            k += 1;
                                                        }
                                                    }
                                                    pColExpr =  crate::src::src::build::sqlite3ColumnExpr(pTab as *mut crate::sqliteInt_h::Table,  pCol as *mut crate::sqliteInt_h::Column) as
    *mut crate::sqliteInt_h::Expr;
                                                    crate::src::src::vdbeaux::sqlite3VdbeMultiLoad(
                                                        v,
                                                        1 as ::core::ffi::c_int,
                                                        if __pPragma_ref.iArg != 0 {
                                                            b"issisii\0" as *const u8
                                                                as *const ::core::ffi::c_char
                                                        } else {
                                                            b"issisi\0" as *const u8
                                                                as *const ::core::ffi::c_char
                                                        },
                                                        i - nHidden,
                                                        __pCol_ref.zCnName,
                                                        crate::src::src::util::sqlite3ColumnType(
                                                            
                                                            pCol as *mut crate::sqliteInt_h::Column,
                                                            b"\0" as *const u8
                                                                as *const ::core::ffi::c_char
                                                                as *mut ::core::ffi::c_char,
                                                        ),
                                                        if __pCol_ref.notNull() as ::core::ffi::c_int
                                                            != 0
                                                        {
                                                            1 as ::core::ffi::c_int
                                                        } else {
                                                            0 as ::core::ffi::c_int
                                                        },
                                                        if isHidden >= 2 as ::core::ffi::c_int
                                                            || pColExpr.is_null()
                                                        {
                                                            ::core::ptr::null_mut::<
                                                                ::core::ffi::c_char,
                                                            >(
                                                            )
                                                        } else {
                                                            (*pColExpr).u.zToken
                                                        },
                                                        k,
                                                        isHidden,
                                                    );
                                                }
                                                _ => {}
                                            }
                                            i += 1;
                                            pCol = pCol.offset(1);
                                        }
                                    }
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_TABLE_LIST =>  {
                                let mut ii_3: ::core::ffi::c_int = 0;
                                (*pParse).nMem = 6 as ::core::ffi::c_int;
                                crate::src::src::build::sqlite3CodeVerifyNamedSchema(pParse as *mut crate::sqliteInt_h::Parse, zDb);
                                ii_3 = 0 as ::core::ffi::c_int;
                                while ii_3 < (*db).nDb {
                                    let mut k_0: *mut crate::src::src::hash::HashElem =
                                        ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
                                    let mut pHash: *mut crate::src::src::hash::Hash = ::core::ptr::null_mut::<crate::src::src::hash::Hash>();
                                    let mut initNCol: ::core::ffi::c_int = 0;
                                    if !(!zDb.is_null()
                                        && crate::src::src::util::sqlite3_stricmp(
                                            zDb,
                                            (*(*db).aDb.offset(ii_3 as isize)).zDbSName,
                                        ) != 0 as ::core::ffi::c_int)
                                    {
                                        pHash = &raw mut (*(*(*db).aDb.offset(ii_3 as isize))
                                            .pSchema)
                                            .tblHash;
                                        initNCol = (*pHash).count as ::core::ffi::c_int;
                                        loop {
                                            let fresh0 = initNCol;
                                            initNCol = initNCol - 1;
                                            if !(fresh0 != 0) {
                                                break;
                                            }
                                            k_0 = (*pHash).first;
                                            loop {
                                                let mut pTab_0: *mut crate::sqliteInt_h::Table =
                                                    ::core::ptr::null_mut::<crate::sqliteInt_h::Table>();
                                                if k_0.is_null() {
                                                    initNCol = 0 as ::core::ffi::c_int;
                                                    break;
                                                } else {
                                                    pTab_0 = (*k_0).data as *mut crate::sqliteInt_h::Table;
                                                    if (*pTab_0).nCol as ::core::ffi::c_int
                                                        == 0 as ::core::ffi::c_int
                                                    {
                                                        let mut zSql: *mut ::core::ffi::c_char =
                                                            crate::src::src::printf::sqlite3MPrintf(
                                                                
                                                                db as *mut crate::sqliteInt_h::sqlite3,
                                                                b"SELECT*FROM\"%w\"\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                (*pTab_0).zName,
                                                            );
                                                        if !zSql.is_null() {
                                                            let mut pDummy: *mut crate::sqlite3_h::sqlite3_stmt =
                                                                ::core::ptr::null_mut::<crate::sqlite3_h::sqlite3_stmt>(
                                                                );
                                                            crate::src::src::prepare::sqlite3_prepare_v3(
                                                                
                                                                db as *mut crate::sqliteInt_h::sqlite3,
                                                                zSql,
                                                                -(1 as ::core::ffi::c_int),
                                                                crate::sqlite3_h::SQLITE_PREPARE_DONT_LOG
                                                                    as ::core::ffi::c_uint,
                                                                &raw mut pDummy,
                                                                ::core::ptr::null_mut::<
                                                                    *const ::core::ffi::c_char,
                                                                >(
                                                                ),
                                                            );
                                                            crate::src::src::vdbeapi::sqlite3_finalize(pDummy);
                                                            crate::src::src::malloc::sqlite3DbFree(
                                                                
                                                                db as *mut crate::sqliteInt_h::sqlite3,
                                                                zSql as *mut ::core::ffi::c_void,
                                                            );
                                                        }
                                                        if (*db).mallocFailed != 0 {
                                                            crate::src::src::util::sqlite3ErrorMsg(
                                                                
                                                                (*db).pParse as *mut crate::sqliteInt_h::Parse,
                                                                b"out of memory\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                            );
                                                            (*(*db).pParse).rc = crate::sqliteInt_h::SQLITE_NOMEM_BKPT;
                                                        }
                                                        pHash = &raw mut (*(*(*db)
                                                            .aDb
                                                            .offset(ii_3 as isize))
                                                        .pSchema)
                                                            .tblHash;
                                                        break;
                                                    } else {
                                                        k_0 = (*k_0).next;
                                                    }
                                                }
                                            }
                                        }
                                        k_0 = (*pHash).first;
                                        while !k_0.is_null() {
                                            let mut pTab_1: *mut crate::sqliteInt_h::Table = (*k_0).data as *mut crate::sqliteInt_h::Table;
                                            let mut zType: *const ::core::ffi::c_char =
                                                ::core::ptr::null::<::core::ffi::c_char>();
                                            if !(!zRight.is_null()
                                                && crate::src::src::util::sqlite3_stricmp(zRight, (*pTab_1).zName)
                                                    != 0 as ::core::ffi::c_int)
                                            {
                                                let __pTab_1_ref = unsafe { &*pTab_1 };
                                                if __pTab_1_ref.eTabType as ::core::ffi::c_int
                                                    == crate::sqliteInt_h::TABTYP_VIEW
                                                {
                                                    zType = b"view\0" as *const u8
                                                        as *const ::core::ffi::c_char;
                                                } else if __pTab_1_ref.eTabType as ::core::ffi::c_int
                                                    == crate::sqliteInt_h::TABTYP_VTAB
                                                {
                                                    zType = b"virtual\0" as *const u8
                                                        as *const ::core::ffi::c_char;
                                                } else if __pTab_1_ref.tabFlags & crate::sqliteInt_h::TF_Shadow as crate::src::ext::rtree::rtree::u32_0
                                                    != 0
                                                {
                                                    zType = b"shadow\0" as *const u8
                                                        as *const ::core::ffi::c_char;
                                                } else {
                                                    zType = b"table\0" as *const u8
                                                        as *const ::core::ffi::c_char;
                                                }
                                                crate::src::src::vdbeaux::sqlite3VdbeMultiLoad(
                                                    v,
                                                    1 as ::core::ffi::c_int,
                                                    b"sssiii\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    (*(*db).aDb.offset(ii_3 as isize)).zDbSName,
                                                    crate::src::src::build::sqlite3PreferredTableName(__pTab_1_ref.zName),
                                                    zType,
                                                    __pTab_1_ref.nCol as ::core::ffi::c_int,
                                                    (__pTab_1_ref.tabFlags & crate::sqliteInt_h::TF_WithoutRowid as crate::src::ext::rtree::rtree::u32_0
                                                        != 0 as crate::src::ext::rtree::rtree::u32_0)
                                                        as ::core::ffi::c_int,
                                                    (__pTab_1_ref.tabFlags & crate::sqliteInt_h::TF_Strict as crate::src::ext::rtree::rtree::u32_0
                                                        != 0 as crate::src::ext::rtree::rtree::u32_0)
                                                        as ::core::ffi::c_int,
                                                );
                                            }
                                            k_0 = (*k_0).next;
                                        }
                                    }
                                    ii_3 += 1;
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_INDEX_INFO =>  {
                                if !zRight.is_null() {
                                    let mut pIdx: *mut crate::sqliteInt_h::Index = ::core::ptr::null_mut::<crate::sqliteInt_h::Index>();
                                    let mut pTab_2: *mut crate::sqliteInt_h::Table = ::core::ptr::null_mut::<crate::sqliteInt_h::Table>();
                                    pIdx =  crate::src::src::build::sqlite3FindIndex(db as *mut crate::sqliteInt_h::sqlite3, zRight, zDb) as
    *mut crate::sqliteInt_h::Index;
                                    if pIdx.is_null() {
                                        pTab_2 =  crate::src::src::build::sqlite3LocateTable(
                                            
                                            pParse as *mut crate::sqliteInt_h::Parse,
                                            crate::sqliteInt_h::LOCATE_NOERR as crate::src::ext::rtree::rtree::u32_0,
                                            zRight,
                                            zDb,
                                        ) as
    *mut crate::sqliteInt_h::Table;
                                        if !pTab_2.is_null()
                                            && !((*pTab_2).tabFlags & crate::sqliteInt_h::TF_WithoutRowid as crate::src::ext::rtree::rtree::u32_0
                                                == 0 as crate::src::ext::rtree::rtree::u32_0)
                                        {
                                            pIdx =  crate::src::src::build::sqlite3PrimaryKeyIndex(pTab_2 as *mut crate::sqliteInt_h::Table) as *mut crate::sqliteInt_h::Index;
                                        }
                                    }
                                    if !pIdx.is_null() {
                                        let mut iIdxDb: ::core::ffi::c_int =
                                            crate::src::src::prepare::sqlite3SchemaToIndex(db as *mut crate::sqliteInt_h::sqlite3,  (*pIdx).pSchema as *mut crate::sqliteInt_h::Schema);
                                        let mut i_0: ::core::ffi::c_int = 0;
                                        let mut mx: ::core::ffi::c_int = 0;
                                        if __pPragma_ref.iArg != 0 {
                                            mx = (*pIdx).nColumn as ::core::ffi::c_int;
                                            (*pParse).nMem = 6 as ::core::ffi::c_int;
                                        } else {
                                            mx = (*pIdx).nKeyCol as ::core::ffi::c_int;
                                            (*pParse).nMem = 3 as ::core::ffi::c_int;
                                        }
                                        pTab_2 = (*pIdx).pTable;
                                        crate::src::src::build::sqlite3CodeVerifySchema(pParse as *mut crate::sqliteInt_h::Parse, iIdxDb);
                                        i_0 = 0 as ::core::ffi::c_int;
                                        while i_0 < mx {
                                            let mut cnum: crate::src::fts5::i16_0 =
                                                *(*pIdx).aiColumn.offset(i_0 as isize);
                                            crate::src::src::vdbeaux::sqlite3VdbeMultiLoad(
                                                v,
                                                1 as ::core::ffi::c_int,
                                                b"iisX\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                i_0,
                                                cnum as ::core::ffi::c_int,
                                                if (cnum as ::core::ffi::c_int)
                                                    < 0 as ::core::ffi::c_int
                                                {
                                                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                                                } else {
                                                    (*(*pTab_2).aCol.offset(cnum as isize)).zCnName
                                                },
                                            );
                                            if __pPragma_ref.iArg != 0 {
                                                let __pIdx_ref = unsafe { &mut *pIdx };
                                                crate::src::src::vdbeaux::sqlite3VdbeMultiLoad(
                                                    v,
                                                    4 as ::core::ffi::c_int,
                                                    b"isiX\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    *__pIdx_ref.aSortOrder.offset(i_0 as isize)
                                                        as ::core::ffi::c_int,
                                                    *__pIdx_ref.azColl.offset(i_0 as isize),
                                                    (i_0 < __pIdx_ref.nKeyCol as ::core::ffi::c_int)
                                                        as ::core::ffi::c_int,
                                                );
                                            }
                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                v,
                                                crate::opcodes_h::OP_ResultRow,
                                                1 as ::core::ffi::c_int,
                                                (*pParse).nMem,
                                            );
                                            i_0 += 1;
                                        }
                                    }
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_INDEX_LIST =>  {
                                if !zRight.is_null() {
                                    let mut pIdx_0: *mut crate::sqliteInt_h::Index = ::core::ptr::null_mut::<crate::sqliteInt_h::Index>();
                                    let mut pTab_3: *mut crate::sqliteInt_h::Table = ::core::ptr::null_mut::<crate::sqliteInt_h::Table>();
                                    let mut i_1: ::core::ffi::c_int = 0;
                                    pTab_3 =  crate::src::src::build::sqlite3FindTable(db as *mut crate::sqliteInt_h::sqlite3, zRight, zDb) as
    *mut crate::sqliteInt_h::Table;
                                    if !pTab_3.is_null() {
                                        let mut iTabDb: ::core::ffi::c_int =
                                            crate::src::src::prepare::sqlite3SchemaToIndex(db as *mut crate::sqliteInt_h::sqlite3,  (*pTab_3).pSchema as *mut crate::sqliteInt_h::Schema);
                                        (*pParse).nMem = 5 as ::core::ffi::c_int;
                                        crate::src::src::build::sqlite3CodeVerifySchema(pParse as *mut crate::sqliteInt_h::Parse, iTabDb);
                                        pIdx_0 = (*pTab_3).pIndex;
                                        i_1 = 0 as ::core::ffi::c_int;
                                        while !pIdx_0.is_null() {
                                            let mut azOrigin: [*const ::core::ffi::c_char; 3] = [
                                                b"c\0" as *const u8 as *const ::core::ffi::c_char,
                                                b"u\0" as *const u8 as *const ::core::ffi::c_char,
                                                b"pk\0" as *const u8 as *const ::core::ffi::c_char,
                                            ];
                                            crate::src::src::vdbeaux::sqlite3VdbeMultiLoad(
                                                v,
                                                1 as ::core::ffi::c_int,
                                                b"isisi\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                i_1,
                                                (*pIdx_0).zName,
                                                ((*pIdx_0).onError as ::core::ffi::c_int != crate::sqliteInt_h::OE_None)
                                                    as ::core::ffi::c_int,
                                                azOrigin[(*pIdx_0).idxType() as usize],
                                                ((*pIdx_0).pPartIdxWhere
                                                    != ::core::ptr::null_mut::<crate::sqliteInt_h::Expr>())
                                                    as ::core::ffi::c_int,
                                            );
                                            pIdx_0 = (*pIdx_0).pNext;
                                            i_1 += 1;
                                        }
                                    }
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_DATABASE_LIST =>  {
                                let mut i_2: ::core::ffi::c_int = 0;
                                (*pParse).nMem = 3 as ::core::ffi::c_int;
                                i_2 = 0 as ::core::ffi::c_int;
                                while i_2 < (*db).nDb {
                                    if !(*(*db).aDb.offset(i_2 as isize)).pBt.is_null() {
                                        crate::src::src::vdbeaux::sqlite3VdbeMultiLoad(
                                            v,
                                            1 as ::core::ffi::c_int,
                                            b"iss\0" as *const u8 as *const ::core::ffi::c_char,
                                            i_2,
                                            (*(*db).aDb.offset(i_2 as isize)).zDbSName,
                                            crate::src::src::btree::sqlite3BtreeGetFilename(
                                                (*(*db).aDb.offset(i_2 as isize)).pBt,
                                            ),
                                        );
                                    }
                                    i_2 += 1;
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_COLLATION_LIST =>  {
                                let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                let mut p: *mut crate::src::src::hash::HashElem = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
                                (*pParse).nMem = 2 as ::core::ffi::c_int;
                                p = (*db).aCollSeq.first;
                                while !p.is_null() {
                                    let mut pColl: *mut crate::sqliteInt_h::CollSeq = (*p).data as *mut crate::sqliteInt_h::CollSeq;
                                    let fresh1 = i_3;
                                    i_3 = i_3 + 1;
                                    crate::src::src::vdbeaux::sqlite3VdbeMultiLoad(
                                        v,
                                        1 as ::core::ffi::c_int,
                                        b"is\0" as *const u8 as *const ::core::ffi::c_char,
                                        fresh1,
                                        (*pColl).zName,
                                    );
                                    p = (*p).next;
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_FUNCTION_LIST =>  {
                                let mut i_4: ::core::ffi::c_int = 0;
                                let mut j: *mut crate::src::src::hash::HashElem = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
                                let mut p_0: *mut crate::sqliteInt_h::FuncDef = ::core::ptr::null_mut::<crate::sqliteInt_h::FuncDef>();
                                let mut showInternFunc: ::core::ffi::c_int =
                                    ((*db).mDbFlags & crate::sqliteInt_h::DBFLAG_InternalFunc as crate::src::ext::rtree::rtree::u32_0 != 0 as crate::src::ext::rtree::rtree::u32_0)
                                        as ::core::ffi::c_int;
                                (*pParse).nMem = 6 as ::core::ffi::c_int;
                                i_4 = 0 as ::core::ffi::c_int;
                                while i_4 < crate::sqliteInt_h::SQLITE_FUNC_HASH_SZ {
                                    p_0 = crate::src::src::global::sqlite3BuiltinFunctions.a[i_4 as usize];
                                    while !p_0.is_null() {
                                        pragmaFunclistLine(
                                            v,
                                            p_0,
                                            1 as ::core::ffi::c_int,
                                            showInternFunc,
                                        );
                                        p_0 = (*p_0).u.pHash;
                                    }
                                    i_4 += 1;
                                }
                                j = (*db).aFunc.first;
                                while !j.is_null() {
                                    p_0 = (*j).data as *mut crate::sqliteInt_h::FuncDef;
                                    pragmaFunclistLine(
                                        v,
                                        p_0,
                                        0 as ::core::ffi::c_int,
                                        showInternFunc,
                                    );
                                    j = (*j).next;
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_MODULE_LIST =>  {
                                let mut j_0: *mut crate::src::src::hash::HashElem = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
                                (*pParse).nMem = 1 as ::core::ffi::c_int;
                                j_0 = (*db).aModule.first;
                                while !j_0.is_null() {
                                    let mut pMod: *mut crate::sqliteInt_h::Module = (*j_0).data as *mut crate::sqliteInt_h::Module;
                                    crate::src::src::vdbeaux::sqlite3VdbeMultiLoad(
                                        v,
                                        1 as ::core::ffi::c_int,
                                        b"s\0" as *const u8 as *const ::core::ffi::c_char,
                                        (*pMod).zName,
                                    );
                                    j_0 = (*j_0).next;
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_PRAGMA_LIST =>  {
                                let mut i_5: ::core::ffi::c_int = 0;
                                i_5 = 0 as ::core::ffi::c_int;
                                while i_5
                                    < (::core::mem::size_of::<[crate::src::src::pragma::PragmaName; 67]>() as usize)
                                        .wrapping_div(::core::mem::size_of::<crate::src::src::pragma::PragmaName>() as usize)
                                        as ::core::ffi::c_int
                                {
                                    crate::src::src::vdbeaux::sqlite3VdbeMultiLoad(
                                        v,
                                        1 as ::core::ffi::c_int,
                                        b"s\0" as *const u8 as *const ::core::ffi::c_char,
                                        aPragmaName[i_5 as usize].zName,
                                    );
                                    i_5 += 1;
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_FOREIGN_KEY_LIST =>  {
                                if !zRight.is_null() {
                                    let mut pFK: *mut crate::sqliteInt_h::FKey = ::core::ptr::null_mut::<crate::sqliteInt_h::FKey>();
                                    let mut pTab_4: *mut crate::sqliteInt_h::Table = ::core::ptr::null_mut::<crate::sqliteInt_h::Table>();
                                    pTab_4 =  crate::src::src::build::sqlite3FindTable(db as *mut crate::sqliteInt_h::sqlite3, zRight, zDb) as
    *mut crate::sqliteInt_h::Table;
                                    if !pTab_4.is_null()
                                        && (*pTab_4).eTabType as ::core::ffi::c_int == crate::sqliteInt_h::TABTYP_NORM
                                    {
                                        pFK = (*pTab_4).u.tab.pFKey;
                                        if !pFK.is_null() {
                                            let mut iTabDb_0: ::core::ffi::c_int =
                                                crate::src::src::prepare::sqlite3SchemaToIndex(db as *mut crate::sqliteInt_h::sqlite3,  (*pTab_4).pSchema as *mut crate::sqliteInt_h::Schema);
                                            let mut i_6: ::core::ffi::c_int =
                                                0 as ::core::ffi::c_int;
                                            (*pParse).nMem = 8 as ::core::ffi::c_int;
                                            crate::src::src::build::sqlite3CodeVerifySchema(pParse as *mut crate::sqliteInt_h::Parse, iTabDb_0);
                                            while !pFK.is_null() {
                                                let mut j_1: ::core::ffi::c_int = 0;
                                                j_1 = 0 as ::core::ffi::c_int;
                                                while j_1 < (*pFK).nCol {
                                                    let __pFK_ref = unsafe { &mut *pFK };
                                                    crate::src::src::vdbeaux::sqlite3VdbeMultiLoad(
                                                        v,
                                                        1 as ::core::ffi::c_int,
                                                        b"iissssss\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        i_6,
                                                        j_1,
                                                        __pFK_ref.zTo,
                                                        (*(*pTab_4).aCol.offset(
                                                            (*(&raw mut __pFK_ref.aCol
                                                                as *mut crate::sqliteInt_h::sColMap)
                                                                .offset(j_1 as isize))
                                                            .iFrom
                                                                as isize,
                                                        ))
                                                        .zCnName,
                                                        (*(&raw mut __pFK_ref.aCol as *mut crate::sqliteInt_h::sColMap)
                                                            .offset(j_1 as isize))
                                                        .zCol,
                                                        actionName(
                                                            __pFK_ref.aAction
                                                                [1 as ::core::ffi::c_int as usize],
                                                        ),
                                                        actionName(
                                                            __pFK_ref.aAction
                                                                [0 as ::core::ffi::c_int as usize],
                                                        ),
                                                        b"NONE\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    j_1 += 1;
                                                }
                                                i_6 += 1;
                                                pFK = (*pFK).pNextFrom;
                                            }
                                        }
                                    }
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_FOREIGN_KEY_CHECK =>  {
                                let mut pFK_0: *mut crate::sqliteInt_h::FKey = ::core::ptr::null_mut::<crate::sqliteInt_h::FKey>();
                                let mut pTab_5: *mut crate::sqliteInt_h::Table = ::core::ptr::null_mut::<crate::sqliteInt_h::Table>();
                                let mut pParent: *mut crate::sqliteInt_h::Table = ::core::ptr::null_mut::<crate::sqliteInt_h::Table>();
                                let mut pIdx_1: *mut crate::sqliteInt_h::Index = ::core::ptr::null_mut::<crate::sqliteInt_h::Index>();
                                let mut i_7: ::core::ffi::c_int = 0;
                                let mut j_2: ::core::ffi::c_int = 0;
                                let mut k_1: *mut crate::src::src::hash::HashElem = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
                                let mut x_0: ::core::ffi::c_int = 0;
                                let mut regResult: ::core::ffi::c_int = 0;
                                let mut regRow: ::core::ffi::c_int = 0;
                                let mut addrTop: ::core::ffi::c_int = 0;
                                let mut addrOk: ::core::ffi::c_int = 0;
                                let mut aiCols: *mut ::core::ffi::c_int =
                                    ::core::ptr::null_mut::<::core::ffi::c_int>();
                                let __pParse_ref = unsafe { &mut *pParse };
                                regResult = __pParse_ref.nMem + 1 as ::core::ffi::c_int;
                                __pParse_ref.nMem += 4 as ::core::ffi::c_int;
                                __pParse_ref.nMem += 1;
                                regRow = __pParse_ref.nMem;
                                k_1 = (*(*(*db).aDb.offset(iDb as isize)).pSchema).tblHash.first;
                                while !k_1.is_null() {
                                    if !zRight.is_null() {
                                        pTab_5 =
                                            
                                            crate::src::src::build::sqlite3LocateTable(pParse as *mut crate::sqliteInt_h::Parse, 0 as crate::src::ext::rtree::rtree::u32_0, zRight, zDb) as
    *mut crate::sqliteInt_h::Table;
                                        k_1 = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
                                    } else {
                                        pTab_5 = (*k_1).data as *mut crate::sqliteInt_h::Table;
                                        k_1 = (*k_1).next;
                                    }
                                    if pTab_5.is_null()
                                        || !((*pTab_5).eTabType as ::core::ffi::c_int
                                            == crate::sqliteInt_h::TABTYP_NORM)
                                        || (*pTab_5).u.tab.pFKey.is_null()
                                    {
                                        continue;
                                    }
                                    iDb = crate::src::src::prepare::sqlite3SchemaToIndex(db as *mut crate::sqliteInt_h::sqlite3,  (*pTab_5).pSchema as *mut crate::sqliteInt_h::Schema);
                                    zDb = (*(*db).aDb.offset(iDb as isize)).zDbSName;
                                    crate::src::src::build::sqlite3CodeVerifySchema(pParse as *mut crate::sqliteInt_h::Parse, iDb);
                                    crate::src::src::build::sqlite3TableLock(
                                        
                                        pParse as *mut crate::sqliteInt_h::Parse,
                                        iDb,
                                        (*pTab_5).tnum,
                                        0 as crate::src::ext::rtree::rtree::u8_0,
                                        (*pTab_5).zName,
                                    );
                                    crate::src::src::expr::sqlite3TouchRegister(
                                        
                                        pParse as *mut crate::sqliteInt_h::Parse,
                                        (*pTab_5).nCol as ::core::ffi::c_int + regRow,
                                    );
                                    crate::src::src::insert::sqlite3OpenTable(
                                        
                                        pParse as *mut crate::sqliteInt_h::Parse,
                                        0 as ::core::ffi::c_int,
                                        iDb,
                                        
                                        pTab_5 as *mut crate::sqliteInt_h::Table,
                                        crate::opcodes_h::OP_OpenRead,
                                    );
                                    crate::src::src::vdbeaux::sqlite3VdbeLoadString(v, regResult, (*pTab_5).zName);
                                    i_7 = 1 as ::core::ffi::c_int;
                                    pFK_0 = (*pTab_5).u.tab.pFKey;
                                    while !pFK_0.is_null() {
                                        pParent =  crate::src::src::build::sqlite3FindTable(db as *mut crate::sqliteInt_h::sqlite3, (*pFK_0).zTo, zDb) as
    *mut crate::sqliteInt_h::Table;
                                        if !pParent.is_null() {
                                            pIdx_1 = ::core::ptr::null_mut::<crate::sqliteInt_h::Index>();
                                            crate::src::src::build::sqlite3TableLock(
                                                
                                                pParse as *mut crate::sqliteInt_h::Parse,
                                                iDb,
                                                (*pParent).tnum,
                                                0 as crate::src::ext::rtree::rtree::u8_0,
                                                (*pParent).zName,
                                            );
                                            x_0 = crate::src::src::fkey::sqlite3FkLocateIndex(
                                                
                                                pParse as *mut crate::sqliteInt_h::Parse,
                                                
                                                pParent as *mut crate::sqliteInt_h::Table,
                                                
                                                pFK_0 as *mut crate::sqliteInt_h::FKey,
                                                
                                                &raw mut pIdx_1 as *mut _ as *mut *mut crate::sqliteInt_h::Index,
                                                ::core::ptr::null_mut::<*mut ::core::ffi::c_int>(),
                                            );
                                            if x_0 == 0 as ::core::ffi::c_int {
                                                if pIdx_1.is_null() {
                                                    crate::src::src::insert::sqlite3OpenTable(
                                                        
                                                        pParse as *mut crate::sqliteInt_h::Parse,
                                                        i_7,
                                                        iDb,
                                                        
                                                        pParent as *mut crate::sqliteInt_h::Table,
                                                        crate::opcodes_h::OP_OpenRead,
                                                    );
                                                } else {
                                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                        v,
                                                        crate::opcodes_h::OP_OpenRead,
                                                        i_7,
                                                        (*pIdx_1).tnum as ::core::ffi::c_int,
                                                        iDb,
                                                    );
                                                    crate::src::src::vdbeaux::sqlite3VdbeSetP4KeyInfo(pParse as *mut crate::sqliteInt_h::Parse,  pIdx_1 as *mut crate::sqliteInt_h::Index);
                                                }
                                            } else {
                                                k_1 = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
                                                break;
                                            }
                                        }
                                        i_7 += 1;
                                        pFK_0 = (*pFK_0).pNextFrom;
                                    }
                                    if !pFK_0.is_null() {
                                        break;
                                    }
                                    if __pParse_ref.nTab < i_7 {
                                        __pParse_ref.nTab = i_7;
                                    }
                                    addrTop =
                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp1(v, crate::opcodes_h::OP_Rewind, 0 as ::core::ffi::c_int);
                                    i_7 = 1 as ::core::ffi::c_int;
                                    pFK_0 = (*pTab_5).u.tab.pFKey;
                                    while !pFK_0.is_null() {
                                        pParent =  crate::src::src::build::sqlite3FindTable(db as *mut crate::sqliteInt_h::sqlite3, (*pFK_0).zTo, zDb) as
    *mut crate::sqliteInt_h::Table;
                                        pIdx_1 = ::core::ptr::null_mut::<crate::sqliteInt_h::Index>();
                                        aiCols = ::core::ptr::null_mut::<::core::ffi::c_int>();
                                        if !pParent.is_null() {
                                            x_0 = crate::src::src::fkey::sqlite3FkLocateIndex(
                                                
                                                pParse as *mut crate::sqliteInt_h::Parse,
                                                
                                                pParent as *mut crate::sqliteInt_h::Table,
                                                
                                                pFK_0 as *mut crate::sqliteInt_h::FKey,
                                                
                                                &raw mut pIdx_1 as *mut _ as *mut *mut crate::sqliteInt_h::Index,
                                                &raw mut aiCols,
                                            );
                                        }
                                        addrOk = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(pParse as *mut crate::sqliteInt_h::Parse);
                                        crate::src::src::expr::sqlite3TouchRegister(pParse as *mut crate::sqliteInt_h::Parse, regRow + (*pFK_0).nCol);
                                        j_2 = 0 as ::core::ffi::c_int;
                                        while j_2 < (*pFK_0).nCol {
                                            let mut iCol: ::core::ffi::c_int = if !aiCols.is_null()
                                            {
                                                *aiCols.offset(j_2 as isize)
                                            } else {
                                                (*(&raw mut (*pFK_0).aCol as *mut crate::sqliteInt_h::sColMap)
                                                    .offset(j_2 as isize))
                                                .iFrom
                                            };
                                            crate::src::src::expr::sqlite3ExprCodeGetColumnOfTable(
                                                v,
                                                
                                                pTab_5 as *mut crate::sqliteInt_h::Table,
                                                0 as ::core::ffi::c_int,
                                                iCol,
                                                regRow + j_2,
                                            );
                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(v, crate::opcodes_h::OP_IsNull, regRow + j_2, addrOk);
                                            j_2 += 1;
                                        }
                                        if !pIdx_1.is_null() {
                                            let __pFK_0_ref = unsafe { &*pFK_0 };
                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                v,
                                                crate::opcodes_h::OP_Affinity,
                                                regRow,
                                                __pFK_0_ref.nCol,
                                                0 as ::core::ffi::c_int,
                                                crate::src::src::insert::sqlite3IndexAffinityStr(db as *mut crate::sqliteInt_h::sqlite3,  pIdx_1 as *mut crate::sqliteInt_h::Index),
                                                __pFK_0_ref.nCol,
                                            );
                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                                                v,
                                                crate::opcodes_h::OP_Found,
                                                i_7,
                                                addrOk,
                                                regRow,
                                                __pFK_0_ref.nCol,
                                            );
                                        } else if !pParent.is_null() {
                                            let mut jmp: ::core::ffi::c_int =
                                                crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int;
                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(v, crate::opcodes_h::OP_SeekRowid, i_7, jmp, regRow);
                                            crate::src::src::vdbeaux::sqlite3VdbeGoto(v, addrOk);
                                        }
                                        if (*pTab_5).tabFlags & crate::sqliteInt_h::TF_WithoutRowid as crate::src::ext::rtree::rtree::u32_0
                                            == 0 as crate::src::ext::rtree::rtree::u32_0
                                        {
                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                v,
                                                crate::opcodes_h::OP_Rowid,
                                                0 as ::core::ffi::c_int,
                                                regResult + 1 as ::core::ffi::c_int,
                                            );
                                        } else {
                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                v,
                                                crate::opcodes_h::OP_Null,
                                                0 as ::core::ffi::c_int,
                                                regResult + 1 as ::core::ffi::c_int,
                                            );
                                        }
                                        crate::src::src::vdbeaux::sqlite3VdbeMultiLoad(
                                            v,
                                            regResult + 2 as ::core::ffi::c_int,
                                            b"siX\0" as *const u8 as *const ::core::ffi::c_char,
                                            (*pFK_0).zTo,
                                            i_7 - 1 as ::core::ffi::c_int,
                                        );
                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                            v,
                                            crate::opcodes_h::OP_ResultRow,
                                            regResult,
                                            4 as ::core::ffi::c_int,
                                        );
                                        crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(v, addrOk);
                                        crate::src::src::malloc::sqlite3DbFree(db as *mut crate::sqliteInt_h::sqlite3, aiCols as *mut ::core::ffi::c_void);
                                        i_7 += 1;
                                        pFK_0 = (*pFK_0).pNextFrom;
                                    }
                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                        v,
                                        crate::opcodes_h::OP_Next,
                                        0 as ::core::ffi::c_int,
                                        addrTop + 1 as ::core::ffi::c_int,
                                    );
                                    crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addrTop);
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_CASE_SENSITIVE_LIKE =>  {
                                if !zRight.is_null() {
                                    crate::src::src::func::sqlite3RegisterLikeFunctions(
                                        
                                        db as *mut crate::sqliteInt_h::sqlite3,
                                        sqlite3GetBoolean(zRight, 0 as crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_int,
                                    );
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_INTEGRITY_CHECK =>  {
                                let mut i_8: ::core::ffi::c_int = 0;
                                let mut j_3: ::core::ffi::c_int = 0;
                                let mut addr_0: ::core::ffi::c_int = 0;
                                let mut mxErr: ::core::ffi::c_int = 0;
                                let mut pObjTab: *mut crate::sqliteInt_h::Table = ::core::ptr::null_mut::<crate::sqliteInt_h::Table>();
                                let mut isQuick: ::core::ffi::c_int =
                                    (*(&raw const crate::src::src::global::sqlite3UpperToLower
                                        as *const ::core::ffi::c_uchar)
                                        .offset(*zLeft.offset(0 as isize)
                                            as ::core::ffi::c_uchar
                                            as isize)
                                        as ::core::ffi::c_int
                                        == 'q' as i32)
                                        as ::core::ffi::c_int;
                                if (*pId2).z.is_null() {
                                    iDb = -(1 as ::core::ffi::c_int);
                                }
                                (*pParse).nMem = 6 as ::core::ffi::c_int;
                                mxErr = SQLITE_INTEGRITY_CHECK_ERROR_MAX;
                                if !zRight.is_null() {
                                    if crate::src::src::util::sqlite3GetInt32((*pValue).z, &raw mut mxErr) != 0 {
                                        if mxErr <= 0 as ::core::ffi::c_int {
                                            mxErr = SQLITE_INTEGRITY_CHECK_ERROR_MAX;
                                        }
                                    } else {
                                        pObjTab =  crate::src::src::build::sqlite3LocateTable(
                                            
                                            pParse as *mut crate::sqliteInt_h::Parse,
                                            0 as crate::src::ext::rtree::rtree::u32_0,
                                            zRight,
                                            if iDb >= 0 as ::core::ffi::c_int {
                                                (*(*db).aDb.offset(iDb as isize)).zDbSName
                                            } else {
                                                ::core::ptr::null_mut::<::core::ffi::c_char>()
                                            },
                                        ) as
    *mut crate::sqliteInt_h::Table;
                                    }
                                }
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                    v,
                                    crate::opcodes_h::OP_Integer,
                                    mxErr - 1 as ::core::ffi::c_int,
                                    1 as ::core::ffi::c_int,
                                );
                                i_8 = 0 as ::core::ffi::c_int;
                                while i_8 < (*db).nDb {
                                    let mut x_1: *mut crate::src::src::hash::HashElem =
                                        ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
                                    let mut pTbls: *mut crate::src::src::hash::Hash = ::core::ptr::null_mut::<crate::src::src::hash::Hash>();
                                    let mut aRoot: *mut ::core::ffi::c_int =
                                        ::core::ptr::null_mut::<::core::ffi::c_int>();
                                    let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                    if !(crate::sqliteInt_h::OMIT_TEMPDB != 0 && i_8 == 1 as ::core::ffi::c_int) {
                                        if !(iDb >= 0 as ::core::ffi::c_int && i_8 != iDb) {
                                            crate::src::src::build::sqlite3CodeVerifySchema(pParse as *mut crate::sqliteInt_h::Parse, i_8);
                                            (*pParse).set_okConstFactor(0 as crate::sqliteInt_h::bft as crate::sqliteInt_h::bft);
                                            pTbls = &raw mut (*(*(*db).aDb.offset(i_8 as isize))
                                                .pSchema)
                                                .tblHash;
                                            cnt = 0 as ::core::ffi::c_int;
                                            x_1 = (*pTbls).first;
                                            while !x_1.is_null() {
                                                let mut pTab_6: *mut crate::sqliteInt_h::Table =
                                                    (*x_1).data as *mut crate::sqliteInt_h::Table;
                                                let mut pIdx_2: *mut crate::sqliteInt_h::Index =
                                                    ::core::ptr::null_mut::<crate::sqliteInt_h::Index>();
                                                let mut nIdx: ::core::ffi::c_int = 0;
                                                if !(tableSkipIntegrityCheck(pTab_6, pObjTab) != 0)
                                                {
                                                    if (*pTab_6).tabFlags & crate::sqliteInt_h::TF_WithoutRowid as crate::src::ext::rtree::rtree::u32_0
                                                        == 0 as crate::src::ext::rtree::rtree::u32_0
                                                    {
                                                        cnt += 1;
                                                    }
                                                    nIdx = 0 as ::core::ffi::c_int;
                                                    pIdx_2 = (*pTab_6).pIndex;
                                                    while !pIdx_2.is_null() {
                                                        cnt += 1;
                                                        pIdx_2 = (*pIdx_2).pNext;
                                                        nIdx += 1;
                                                    }
                                                }
                                                x_1 = (*x_1).next;
                                            }
                                            if !(cnt == 0 as ::core::ffi::c_int) {
                                                if !pObjTab.is_null() {
                                                    cnt += 1;
                                                }
                                                aRoot = crate::src::src::malloc::sqlite3DbMallocRawNN(
                                                    
                                                    db as *mut crate::sqliteInt_h::sqlite3,
                                                    (::core::mem::size_of::<::core::ffi::c_int>()
                                                        as usize)
                                                        .wrapping_mul(
                                                            (cnt + 1 as ::core::ffi::c_int)
                                                                as usize,
                                                        )
                                                        as crate::src::ext::rtree::rtree::u64_0,
                                                )
                                                    as *mut ::core::ffi::c_int;
                                                if aRoot.is_null() {
                                                    break;
                                                }
                                                cnt = 0 as ::core::ffi::c_int;
                                                if !pObjTab.is_null() {
                                                    cnt += 1;
                                                    *aRoot.offset(cnt as isize) =
                                                        0 as ::core::ffi::c_int;
                                                }
                                                let __pTbls_ref = unsafe { &*pTbls };
                                                x_1 = __pTbls_ref.first;
                                                while !x_1.is_null() {
                                                    let mut pTab_7: *mut crate::sqliteInt_h::Table =
                                                        (*x_1).data as *mut crate::sqliteInt_h::Table;
                                                    let mut pIdx_3: *mut crate::sqliteInt_h::Index =
                                                        ::core::ptr::null_mut::<crate::sqliteInt_h::Index>();
                                                    if !(tableSkipIntegrityCheck(pTab_7, pObjTab)
                                                        != 0)
                                                    {
                                                        if (*pTab_7).tabFlags
                                                            & crate::sqliteInt_h::TF_WithoutRowid as crate::src::ext::rtree::rtree::u32_0
                                                            == 0 as crate::src::ext::rtree::rtree::u32_0
                                                        {
                                                            cnt += 1;
                                                            *aRoot.offset(cnt as isize) = (*pTab_7)
                                                                .tnum
                                                                as ::core::ffi::c_int;
                                                        }
                                                        pIdx_3 = (*pTab_7).pIndex;
                                                        while !pIdx_3.is_null() {
                                                            cnt += 1;
                                                            *aRoot.offset(cnt as isize) = (*pIdx_3)
                                                                .tnum
                                                                as ::core::ffi::c_int;
                                                            pIdx_3 = (*pIdx_3).pNext;
                                                        }
                                                    }
                                                    x_1 = (*x_1).next;
                                                }
                                                *aRoot.offset(0 as isize) =
                                                    cnt;
                                                crate::src::src::expr::sqlite3TouchRegister(
                                                    
                                                    pParse as *mut crate::sqliteInt_h::Parse,
                                                    8 as ::core::ffi::c_int + cnt,
                                                );
                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                    v,
                                                    crate::opcodes_h::OP_Null,
                                                    0 as ::core::ffi::c_int,
                                                    8 as ::core::ffi::c_int,
                                                    8 as ::core::ffi::c_int + cnt,
                                                );
                                                crate::src::src::expr::sqlite3ClearTempRegCache(pParse as *mut crate::sqliteInt_h::Parse);
                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                    v,
                                                    crate::opcodes_h::OP_IntegrityCk,
                                                    1 as ::core::ffi::c_int,
                                                    cnt,
                                                    8 as ::core::ffi::c_int,
                                                    aRoot as *mut ::core::ffi::c_char,
                                                    crate::src::src::vdbe::P4_INTARRAY,
                                                );
                                                crate::src::src::vdbeaux::sqlite3VdbeChangeP5(v, i_8 as crate::src::fts5::u16_0);
                                                addr_0 = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                                                    v,
                                                    crate::opcodes_h::OP_IsNull,
                                                    2 as ::core::ffi::c_int,
                                                );
                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                    v,
                                                    crate::opcodes_h::OP_String8,
                                                    0 as ::core::ffi::c_int,
                                                    3 as ::core::ffi::c_int,
                                                    0 as ::core::ffi::c_int,
                                                    crate::src::src::printf::sqlite3MPrintf(
                                                        
                                                        db as *mut crate::sqliteInt_h::sqlite3,
                                                        b"*** in database %s ***\n\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        (*(*db).aDb.offset(i_8 as isize)).zDbSName,
                                                    ),
                                                    crate::src::src::vdbe::P4_DYNAMIC,
                                                );
                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                    v,
                                                    crate::opcodes_h::OP_Concat,
                                                    2 as ::core::ffi::c_int,
                                                    3 as ::core::ffi::c_int,
                                                    3 as ::core::ffi::c_int,
                                                );
                                                integrityCheckResultRow(v);
                                                crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addr_0);
                                                cnt = if !pObjTab.is_null() {
                                                    1 as ::core::ffi::c_int
                                                } else {
                                                    0 as ::core::ffi::c_int
                                                };
                                                crate::src::src::vdbeaux::sqlite3VdbeLoadString(
                                                    v,
                                                    2 as ::core::ffi::c_int,
                                                    b"wrong # of entries in index \0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                );
                                                x_1 = __pTbls_ref.first;
                                                while !x_1.is_null() {
                                                    let mut iTab: ::core::ffi::c_int =
                                                        0 as ::core::ffi::c_int;
                                                    let mut pTab_8: *mut crate::sqliteInt_h::Table =
                                                        (*x_1).data as *mut crate::sqliteInt_h::Table;
                                                    let mut pIdx_4: *mut crate::sqliteInt_h::Index =
                                                        ::core::ptr::null_mut::<crate::sqliteInt_h::Index>();
                                                    if !(tableSkipIntegrityCheck(pTab_8, pObjTab)
                                                        != 0)
                                                    {
                                                        if (*pTab_8).tabFlags
                                                            & crate::sqliteInt_h::TF_WithoutRowid as crate::src::ext::rtree::rtree::u32_0
                                                            == 0 as crate::src::ext::rtree::rtree::u32_0
                                                        {
                                                            let fresh2 = cnt;
                                                            cnt = cnt + 1;
                                                            iTab = fresh2;
                                                        } else {
                                                            iTab = cnt;
                                                            pIdx_4 = (*pTab_8).pIndex;
                                                            while !pIdx_4.is_null() {
                                                                if (*pIdx_4).idxType()
                                                                    as ::core::ffi::c_int
                                                                    == crate::sqliteInt_h::SQLITE_IDXTYPE_PRIMARYKEY
                                                                {
                                                                    break;
                                                                }
                                                                iTab += 1;
                                                                pIdx_4 = (*pIdx_4).pNext;
                                                            }
                                                        }
                                                        pIdx_4 = (*pTab_8).pIndex;
                                                        while !pIdx_4.is_null() {
                                                            if (*pIdx_4).pPartIdxWhere.is_null() {
                                                                addr_0 = crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                    v,
                                                                    crate::opcodes_h::OP_Eq,
                                                                    8 as ::core::ffi::c_int + cnt,
                                                                    0 as ::core::ffi::c_int,
                                                                    8 as ::core::ffi::c_int + iTab,
                                                                );
                                                                crate::src::src::vdbeaux::sqlite3VdbeLoadString(
                                                                    v,
                                                                    4 as ::core::ffi::c_int,
                                                                    (*pIdx_4).zName,
                                                                );
                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                    v,
                                                                    crate::opcodes_h::OP_Concat,
                                                                    4 as ::core::ffi::c_int,
                                                                    2 as ::core::ffi::c_int,
                                                                    3 as ::core::ffi::c_int,
                                                                );
                                                                integrityCheckResultRow(v);
                                                                crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, addr_0);
                                                            }
                                                            cnt += 1;
                                                            pIdx_4 = (*pIdx_4).pNext;
                                                        }
                                                    }
                                                    x_1 = (*x_1).next;
                                                }
                                                x_1 = __pTbls_ref.first;
                                                while !x_1.is_null() {
                                                    let mut pTab_9: *mut crate::sqliteInt_h::Table =
                                                        (*x_1).data as *mut crate::sqliteInt_h::Table;
                                                    let mut pIdx_5: *mut crate::sqliteInt_h::Index =
                                                        ::core::ptr::null_mut::<crate::sqliteInt_h::Index>();
                                                    let mut pPk_0: *mut crate::sqliteInt_h::Index =
                                                        ::core::ptr::null_mut::<crate::sqliteInt_h::Index>();
                                                    let mut pPrior: *mut crate::sqliteInt_h::Index =
                                                        ::core::ptr::null_mut::<crate::sqliteInt_h::Index>();
                                                    let mut loopTop: ::core::ffi::c_int = 0;
                                                    let mut iDataCur: ::core::ffi::c_int = 0;
                                                    let mut iIdxCur: ::core::ffi::c_int = 0;
                                                    let mut r1: ::core::ffi::c_int =
                                                        -(1 as ::core::ffi::c_int);
                                                    let mut bStrict: ::core::ffi::c_int = 0;
                                                    let mut r2: ::core::ffi::c_int = 0;
                                                    let mut mxCol: ::core::ffi::c_int = 0;
                                                    if !(tableSkipIntegrityCheck(pTab_9, pObjTab)
                                                        != 0)
                                                    {
                                                        if (*pTab_9).eTabType as ::core::ffi::c_int
                                                            == crate::sqliteInt_h::TABTYP_NORM
                                                        {
                                                            let __pTab_9_ref = unsafe { &mut *pTab_9 };
                                                            if isQuick != 0
                                                                || __pTab_9_ref.tabFlags
                                                                    & crate::sqliteInt_h::TF_WithoutRowid as crate::src::ext::rtree::rtree::u32_0
                                                                    == 0 as crate::src::ext::rtree::rtree::u32_0
                                                            {
                                                                pPk_0 =
                                                                    ::core::ptr::null_mut::<crate::sqliteInt_h::Index>(
                                                                    );
                                                                r2 = 0 as ::core::ffi::c_int;
                                                            } else {
                                                                pPk_0 =
                                                                    
                                                                    crate::src::src::build::sqlite3PrimaryKeyIndex(pTab_9 as *mut crate::sqliteInt_h::Table) as *mut crate::sqliteInt_h::Index;
                                                                r2 = crate::src::src::expr::sqlite3GetTempRange(
                                                                    
                                                                    pParse as *mut crate::sqliteInt_h::Parse,
                                                                    (*pPk_0).nKeyCol
                                                                        as ::core::ffi::c_int,
                                                                );
                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                    v,
                                                                    crate::opcodes_h::OP_Null,
                                                                    1 as ::core::ffi::c_int,
                                                                    r2,
                                                                    r2 + (*pPk_0).nKeyCol
                                                                        as ::core::ffi::c_int
                                                                        - 1 as ::core::ffi::c_int,
                                                                );
                                                            }
                                                            crate::src::src::insert::sqlite3OpenTableAndIndices(
                                                                
                                                                pParse as *mut crate::sqliteInt_h::Parse,
                                                                
                                                                pTab_9 as *mut crate::sqliteInt_h::Table,
                                                                crate::opcodes_h::OP_OpenRead,
                                                                0 as crate::src::ext::rtree::rtree::u8_0,
                                                                1 as ::core::ffi::c_int,
                                                                ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u8_0>(),
                                                                &raw mut iDataCur,
                                                                &raw mut iIdxCur,
                                                            );
                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                v,
                                                                crate::opcodes_h::OP_Integer,
                                                                0 as ::core::ffi::c_int,
                                                                7 as ::core::ffi::c_int,
                                                            );
                                                            j_3 = 0 as ::core::ffi::c_int;
                                                            pIdx_5 = __pTab_9_ref.pIndex;
                                                            while !pIdx_5.is_null() {
                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                    v,
                                                                    crate::opcodes_h::OP_Integer,
                                                                    0 as ::core::ffi::c_int,
                                                                    8 as ::core::ffi::c_int + j_3,
                                                                );
                                                                pIdx_5 = (*pIdx_5).pNext;
                                                                j_3 += 1;
                                                            }
                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                v,
                                                                crate::opcodes_h::OP_Rewind,
                                                                iDataCur,
                                                                0 as ::core::ffi::c_int,
                                                            );
                                                            loopTop = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                v,
                                                                crate::opcodes_h::OP_AddImm,
                                                                7 as ::core::ffi::c_int,
                                                                1 as ::core::ffi::c_int,
                                                            );
                                                            if __pTab_9_ref.tabFlags
                                                                & crate::sqliteInt_h::TF_WithoutRowid as crate::src::ext::rtree::rtree::u32_0
                                                                == 0 as crate::src::ext::rtree::rtree::u32_0
                                                            {
                                                                mxCol = -(1 as ::core::ffi::c_int);
                                                                j_3 = 0 as ::core::ffi::c_int;
                                                                while j_3
                                                                    < __pTab_9_ref.nCol
                                                                        as ::core::ffi::c_int
                                                                {
                                                                    if (*(*pTab_9)
                                                                        .aCol
                                                                        .offset(j_3 as isize))
                                                                    .colFlags
                                                                        as ::core::ffi::c_int
                                                                        & crate::sqliteInt_h::COLFLAG_VIRTUAL
                                                                        == 0 as ::core::ffi::c_int
                                                                    {
                                                                        mxCol += 1;
                                                                    }
                                                                    j_3 += 1;
                                                                }
                                                                if mxCol
                                                                    == __pTab_9_ref.iPKey
                                                                        as ::core::ffi::c_int
                                                                {
                                                                    mxCol -= 1;
                                                                }
                                                            } else {
                                                                mxCol = (*(crate::src::src::build::sqlite3PrimaryKeyIndex(
                                                                    
                                                                    pTab_9 as *mut crate::sqliteInt_h::Table,
                                                                ) as *mut crate::sqliteInt_h::Index))
                                                                .nColumn
                                                                    as ::core::ffi::c_int
                                                                    - 1 as ::core::ffi::c_int;
                                                            }
                                                            if mxCol >= 0 as ::core::ffi::c_int {
                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                    v,
                                                                    crate::opcodes_h::OP_Column,
                                                                    iDataCur,
                                                                    mxCol,
                                                                    3 as ::core::ffi::c_int,
                                                                );
                                                                crate::src::src::vdbeaux::sqlite3VdbeTypeofColumn(
                                                                    v,
                                                                    3 as ::core::ffi::c_int,
                                                                );
                                                            }
                                                            if isQuick == 0 {
                                                                if !pPk_0.is_null() {
                                                                    let mut a1: ::core::ffi::c_int =
                                                                        0;
                                                                    let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                                                        ::core::ffi::c_char,
                                                                    >();
                                                                    a1 = crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                                                                        v,
                                                                        crate::opcodes_h::OP_IdxGT,
                                                                        iDataCur,
                                                                        0 as ::core::ffi::c_int,
                                                                        r2,
                                                                        (*pPk_0).nKeyCol
                                                                            as ::core::ffi::c_int,
                                                                    );
                                                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                                                                        v, crate::opcodes_h::OP_IsNull, r2,
                                                                    );
                                                                    zErr = crate::src::src::printf::sqlite3MPrintf(
                                                                        
                                                                        db as *mut crate::sqliteInt_h::sqlite3,
                                                                        b"row not in PRIMARY KEY order for %s\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        __pTab_9_ref.zName,
                                                                    );
                                                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                                        v,
                                                                        crate::opcodes_h::OP_String8,
                                                                        0 as ::core::ffi::c_int,
                                                                        3 as ::core::ffi::c_int,
                                                                        0 as ::core::ffi::c_int,
                                                                        zErr,
                                                                        crate::src::src::vdbe::P4_DYNAMIC,
                                                                    );
                                                                    integrityCheckResultRow(v);
                                                                    crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, a1);
                                                                    crate::src::src::vdbeaux::sqlite3VdbeJumpHere(
                                                                        v,
                                                                        a1 + 1
                                                                            as ::core::ffi::c_int,
                                                                    );
                                                                    j_3 = 0 as ::core::ffi::c_int;
                                                                    while j_3
                                                                        < (*pPk_0).nKeyCol
                                                                            as ::core::ffi::c_int
                                                                    {
                                                                        crate::src::src::expr::sqlite3ExprCodeLoadIndexColumn(
                                                                            
                                                                            pParse as *mut crate::sqliteInt_h::Parse,
                                                                            
                                                                            pPk_0 as *mut crate::sqliteInt_h::Index,
                                                                            iDataCur,
                                                                            j_3,
                                                                            r2 + j_3,
                                                                        );
                                                                        j_3 += 1;
                                                                    }
                                                                }
                                                            }
                                                            bStrict = (__pTab_9_ref.tabFlags
                                                                & crate::sqliteInt_h::TF_Strict as crate::src::ext::rtree::rtree::u32_0
                                                                != 0 as crate::src::ext::rtree::rtree::u32_0)
                                                                as ::core::ffi::c_int;
                                                            j_3 = 0 as ::core::ffi::c_int;
                                                            while j_3
                                                                < __pTab_9_ref.nCol
                                                                    as ::core::ffi::c_int
                                                            {
                                                                let mut zErr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                                                    ::core::ffi::c_char,
                                                                >();
                                                                let mut pCol_0: *mut crate::sqliteInt_h::Column =
                                                                    (*pTab_9)
                                                                        .aCol
                                                                        .offset(j_3 as isize);
                                                                let mut labelError: ::core::ffi::c_int = 0;
                                                                let mut labelOk: ::core::ffi::c_int = 0;
                                                                let mut p1: ::core::ffi::c_int = 0;
                                                                let mut p3: ::core::ffi::c_int = 0;
                                                                let mut p4: ::core::ffi::c_int = 0;
                                                                let mut doTypeCheck: ::core::ffi::c_int = 0;
                                                                if !(j_3
                                                                    == __pTab_9_ref.iPKey
                                                                        as ::core::ffi::c_int)
                                                                {
                                                                    if bStrict != 0 {
                                                                        doTypeCheck = ((*pCol_0)
                                                                            .eCType()
                                                                            as ::core::ffi::c_int
                                                                            > crate::sqliteInt_h::COLTYPE_ANY)
                                                                            as ::core::ffi::c_int;
                                                                    } else {
                                                                        doTypeCheck = ((*pCol_0)
                                                                            .affinity
                                                                            as ::core::ffi::c_int
                                                                            > crate::sqliteInt_h::SQLITE_AFF_BLOB)
                                                                            as ::core::ffi::c_int;
                                                                    }
                                                                    if !((*pCol_0).notNull()
                                                                        as ::core::ffi::c_int
                                                                        == 0 as ::core::ffi::c_int
                                                                        && doTypeCheck == 0)
                                                                    {
                                                                        p4 = crate::sqlite3_h::SQLITE_NULL;
                                                                        let __pCol_0_ref = unsafe { &mut *pCol_0 };
                                                                        if __pCol_0_ref.colFlags
                                                                            as ::core::ffi::c_int
                                                                            & crate::sqliteInt_h::COLFLAG_VIRTUAL
                                                                            != 0
                                                                        {
                                                                            crate::src::src::expr::sqlite3ExprCodeGetColumnOfTable(
                                                                                v,
                                                                                
                                                                                pTab_9 as *mut crate::sqliteInt_h::Table,
                                                                                iDataCur,
                                                                                j_3,
                                                                                3 as ::core::ffi::c_int,
                                                                            );
                                                                            p1 = -(1 as ::core::ffi::c_int);
                                                                            p3 = 3 as ::core::ffi::c_int;
                                                                        } else {
                                                                            if __pCol_0_ref.iDflt != 0
                                                                            {
                                                                                let mut pDfltValue: *mut crate::vdbeInt_h::sqlite3_value = ::core::ptr::null_mut::<
                                                                                    crate::vdbeInt_h::sqlite3_value,
                                                                                >();
                                                                                crate::src::src::vdbemem::sqlite3ValueFromExpr(
                                                                                    
                                                                                    db as *mut crate::sqliteInt_h::sqlite3,
                                                                                    
                                                                                    crate::src::src::build::sqlite3ColumnExpr(pTab_9 as *mut crate::sqliteInt_h::Table,  pCol_0 as *mut crate::sqliteInt_h::Column) as
        *mut crate::sqliteInt_h::Expr as *const crate::sqliteInt_h::Expr,
                                                                                    (*db).enc,
                                                                                    __pCol_0_ref.affinity as crate::src::ext::rtree::rtree::u8_0,
                                                                                    &raw mut pDfltValue,
                                                                                );
                                                                                if !pDfltValue
                                                                                    .is_null()
                                                                                {
                                                                                    p4 = crate::src::src::vdbeapi::sqlite3_value_type(pDfltValue);
                                                                                    crate::src::src::vdbemem::sqlite3ValueFree(pDfltValue);
                                                                                }
                                                                            }
                                                                            p1 = iDataCur;
                                                                            if !(__pTab_9_ref.tabFlags
                                                                                & crate::sqliteInt_h::TF_WithoutRowid
                                                                                    as crate::src::ext::rtree::rtree::u32_0
                                                                                == 0 as crate::src::ext::rtree::rtree::u32_0)
                                                                            {
                                                                                p3 = crate::src::src::build::sqlite3TableColumnToIndex(
                                                                                    
                                                                                    crate::src::src::build::sqlite3PrimaryKeyIndex(pTab_9 as *mut crate::sqliteInt_h::Table) as
        *mut crate::sqliteInt_h::Index as *mut crate::sqliteInt_h::Index,
                                                                                    j_3,
                                                                                );
                                                                            } else {
                                                                                p3 = crate::src::src::build::sqlite3TableColumnToStorage(pTab_9 as *mut crate::sqliteInt_h::Table, j_3 as crate::src::fts5::i16_0)
                                                                                    as ::core::ffi::c_int;
                                                                            }
                                                                        }
                                                                        labelError =
                                                                            crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                                                                                
                                                                                pParse as *mut crate::sqliteInt_h::Parse,
                                                                            );
                                                                        labelOk =
                                                                            crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                                                                                
                                                                                pParse as *mut crate::sqliteInt_h::Parse,
                                                                            );
                                                                        if __pCol_0_ref.notNull() != 0
                                                                        {
                                                                            let mut jmp3: ::core::ffi::c_int = 0;
                                                                            let mut jmp2: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                                                                                v,
                                                                                crate::opcodes_h::OP_IsType,
                                                                                p1,
                                                                                labelOk,
                                                                                p3,
                                                                                p4,
                                                                            );
                                                                            if p1 < 0 as ::core::ffi::c_int {
                                                                                crate::src::src::vdbeaux::sqlite3VdbeChangeP5(v, 0xf as crate::src::fts5::u16_0);
                                                                                jmp3 = jmp2;
                                                                            } else {
                                                                                crate::src::src::vdbeaux::sqlite3VdbeChangeP5(v, 0xd as crate::src::fts5::u16_0);
                                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                                    v,
                                                                                    crate::opcodes_h::OP_Column,
                                                                                    p1,
                                                                                    p3,
                                                                                    3 as ::core::ffi::c_int,
                                                                                );
                                                                                crate::src::src::update::sqlite3ColumnDefault(
                                                                                    v,
                                                                                    
                                                                                    pTab_9 as *mut crate::sqliteInt_h::Table,
                                                                                    j_3,
                                                                                    3 as ::core::ffi::c_int,
                                                                                );
                                                                                jmp3 = crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                                    v,
                                                                                    crate::opcodes_h::OP_NotNull,
                                                                                    3 as ::core::ffi::c_int,
                                                                                    labelOk,
                                                                                );
                                                                            }
                                                                            zErr_0 = crate::src::src::printf::sqlite3MPrintf(
                                                                                
                                                                                db as *mut crate::sqliteInt_h::sqlite3,
                                                                                b"NULL value in %s.%s\0" as *const u8
                                                                                    as *const ::core::ffi::c_char,
                                                                                __pTab_9_ref.zName,
                                                                                __pCol_0_ref.zCnName,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                                                v,
                                                                                crate::opcodes_h::OP_String8,
                                                                                0 as ::core::ffi::c_int,
                                                                                3 as ::core::ffi::c_int,
                                                                                0 as ::core::ffi::c_int,
                                                                                zErr_0,
                                                                                crate::src::src::vdbe::P4_DYNAMIC,
                                                                            );
                                                                            if doTypeCheck != 0 {
                                                                                crate::src::src::vdbeaux::sqlite3VdbeGoto(
                                                                                    v, labelError,
                                                                                );
                                                                                crate::src::src::vdbeaux::sqlite3VdbeJumpHere(
                                                                                    v, jmp2,
                                                                                );
                                                                                crate::src::src::vdbeaux::sqlite3VdbeJumpHere(
                                                                                    v, jmp3,
                                                                                );
                                                                            }
                                                                        }
                                                                        if bStrict != 0 && doTypeCheck != 0 {
                                                                            static mut aStdTypeMask: [::core::ffi::c_uchar; 6] = [
                                                                                0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                                                0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                                                0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                                                0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                                                0x13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                                                0x14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
                                                                            ];
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(v, crate::opcodes_h::OP_IsType, p1, labelOk, p3, p4);
                                                                            crate::src::src::vdbeaux::sqlite3VdbeChangeP5(
                                                                                v,
                                                                                aStdTypeMask[(__pCol_0_ref.eCType() as ::core::ffi::c_int
                                                                                    - 1 as ::core::ffi::c_int) as usize] as crate::src::fts5::u16_0,
                                                                            );
                                                                            zErr_0 = crate::src::src::printf::sqlite3MPrintf(
                                                                                
                                                                                db as *mut crate::sqliteInt_h::sqlite3,
                                                                                b"non-%s value in %s.%s\0" as *const u8
                                                                                    as *const ::core::ffi::c_char,
                                                                                *(&raw mut crate::src::src::global::sqlite3StdType
                                                                                    as *mut *const ::core::ffi::c_char)
                                                                                    .offset(
                                                                                        (__pCol_0_ref.eCType() as ::core::ffi::c_int
                                                                                            - 1 as ::core::ffi::c_int) as isize,
                                                                                    ),
                                                                                __pTab_9_ref.zName,
                                                                                (*__pTab_9_ref.aCol.offset(j_3 as isize)).zCnName,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                                                v,
                                                                                crate::opcodes_h::OP_String8,
                                                                                0 as ::core::ffi::c_int,
                                                                                3 as ::core::ffi::c_int,
                                                                                0 as ::core::ffi::c_int,
                                                                                zErr_0,
                                                                                crate::src::src::vdbe::P4_DYNAMIC,
                                                                            );
                                                                        } else if bStrict == 0
                                                                            && __pCol_0_ref.affinity as ::core::ffi::c_int
                                                                                == crate::sqliteInt_h::SQLITE_AFF_TEXT
                                                                        {
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(v, crate::opcodes_h::OP_IsType, p1, labelOk, p3, p4);
                                                                            crate::src::src::vdbeaux::sqlite3VdbeChangeP5(v, 0x1c as crate::src::fts5::u16_0);
                                                                            zErr_0 = crate::src::src::printf::sqlite3MPrintf(
                                                                                
                                                                                db as *mut crate::sqliteInt_h::sqlite3,
                                                                                b"NUMERIC value in %s.%s\0" as *const u8
                                                                                    as *const ::core::ffi::c_char,
                                                                                __pTab_9_ref.zName,
                                                                                (*__pTab_9_ref.aCol.offset(j_3 as isize)).zCnName,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                                                v,
                                                                                crate::opcodes_h::OP_String8,
                                                                                0 as ::core::ffi::c_int,
                                                                                3 as ::core::ffi::c_int,
                                                                                0 as ::core::ffi::c_int,
                                                                                zErr_0,
                                                                                crate::src::src::vdbe::P4_DYNAMIC,
                                                                            );
                                                                        } else if bStrict == 0
                                                                            && __pCol_0_ref.affinity as ::core::ffi::c_int
                                                                                >= crate::sqliteInt_h::SQLITE_AFF_NUMERIC
                                                                        {
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(v, crate::opcodes_h::OP_IsType, p1, labelOk, p3, p4);
                                                                            crate::src::src::vdbeaux::sqlite3VdbeChangeP5(v, 0x1b as crate::src::fts5::u16_0);
                                                                            if p1 >= 0 as ::core::ffi::c_int {
                                                                                crate::src::src::expr::sqlite3ExprCodeGetColumnOfTable(
                                                                                    v,
                                                                                    
                                                                                    pTab_9 as *mut crate::sqliteInt_h::Table,
                                                                                    iDataCur,
                                                                                    j_3,
                                                                                    3 as ::core::ffi::c_int,
                                                                                );
                                                                            }
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                                                v,
                                                                                crate::opcodes_h::OP_Affinity,
                                                                                3 as ::core::ffi::c_int,
                                                                                1 as ::core::ffi::c_int,
                                                                                0 as ::core::ffi::c_int,
                                                                                b"C\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                crate::src::src::vdbe::P4_STATIC,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                                                                                v,
                                                                                crate::opcodes_h::OP_IsType,
                                                                                -(1 as ::core::ffi::c_int),
                                                                                labelOk,
                                                                                3 as ::core::ffi::c_int,
                                                                                p4,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeChangeP5(v, 0x1c as crate::src::fts5::u16_0);
                                                                            zErr_0 = crate::src::src::printf::sqlite3MPrintf(
                                                                                
                                                                                db as *mut crate::sqliteInt_h::sqlite3,
                                                                                b"TEXT value in %s.%s\0" as *const u8
                                                                                    as *const ::core::ffi::c_char,
                                                                                __pTab_9_ref.zName,
                                                                                (*__pTab_9_ref.aCol.offset(j_3 as isize)).zCnName,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                                                v,
                                                                                crate::opcodes_h::OP_String8,
                                                                                0 as ::core::ffi::c_int,
                                                                                3 as ::core::ffi::c_int,
                                                                                0 as ::core::ffi::c_int,
                                                                                zErr_0,
                                                                                crate::src::src::vdbe::P4_DYNAMIC,
                                                                            );
                                                                        }
                                                                        crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(
                                                                            v, labelError,
                                                                        );
                                                                        integrityCheckResultRow(v);
                                                                        crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(
                                                                            v, labelOk,
                                                                        );
                                                                    }
                                                                }
                                                                j_3 += 1;
                                                            }
                                                            if !__pTab_9_ref.pCheck.is_null()
                                                                && (*db).flags
                                                                    & crate::sqliteInt_h::SQLITE_IgnoreChecks as crate::src::ext::rtree::rtree::u64_0
                                                                    == 0 as crate::src::ext::rtree::rtree::u64_0
                                                            {
                                                                let mut pCheck: *mut crate::sqliteInt_h::ExprList =
                                                                    
                                                                    crate::src::src::expr::sqlite3ExprListDup(
                                                                        
                                                                        db as *mut crate::sqliteInt_h::sqlite3,
                                                                        
                                                                        __pTab_9_ref.pCheck as *const crate::sqliteInt_h::ExprList,
                                                                        0 as ::core::ffi::c_int,
                                                                    ) as *mut crate::sqliteInt_h::ExprList;
                                                                if (*db).mallocFailed
                                                                    as ::core::ffi::c_int
                                                                    == 0 as ::core::ffi::c_int
                                                                {
                                                                    let mut addrCkFault: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                                                                        
                                                                        pParse as *mut crate::sqliteInt_h::Parse,
                                                                    );
                                                                    let mut addrCkOk: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                                                                        
                                                                        pParse as *mut crate::sqliteInt_h::Parse,
                                                                    );
                                                                    let mut zErr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                                                                        ::core::ffi::c_char,
                                                                    >();
                                                                    let mut k_2: ::core::ffi::c_int = 0;
                                                                    (*pParse).iSelfTab = iDataCur
                                                                        + 1 as ::core::ffi::c_int;
                                                                    k_2 = (*pCheck).nExpr
                                                                        - 1 as ::core::ffi::c_int;
                                                                    while k_2
                                                                        > 0 as ::core::ffi::c_int
                                                                    {
                                                                        crate::src::src::expr::sqlite3ExprIfFalse(
                                                                            
                                                                            pParse as *mut crate::sqliteInt_h::Parse,
                                                                            
                                                                            (*(&raw mut (*pCheck).a as *mut crate::sqliteInt_h::ExprList_item)
                                                                                .offset(k_2 as isize))
                                                                                .pExpr as *mut crate::sqliteInt_h::Expr,
                                                                            addrCkFault,
                                                                            0 as ::core::ffi::c_int,
                                                                        );
                                                                        k_2 -= 1;
                                                                    }
                                                                    crate::src::src::expr::sqlite3ExprIfTrue(
                                                                        
                                                                        pParse as *mut crate::sqliteInt_h::Parse,
                                                                        
                                                                        (*(&raw mut (*pCheck).a as *mut crate::sqliteInt_h::ExprList_item)
                                                                            .offset(0 as isize))
                                                                            .pExpr as
    *mut crate::sqliteInt_h::Expr,
                                                                        addrCkOk,
                                                                        crate::sqliteInt_h::SQLITE_JUMPIFNULL,
                                                                    );
                                                                    crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(
                                                                        v,
                                                                        addrCkFault,
                                                                    );
                                                                    (*pParse).iSelfTab =
                                                                        0 as ::core::ffi::c_int;
                                                                    zErr_1 = crate::src::src::printf::sqlite3MPrintf(
                                                                        
                                                                        db as *mut crate::sqliteInt_h::sqlite3,
                                                                        b"CHECK constraint failed in %s\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        __pTab_9_ref.zName,
                                                                    );
                                                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                                        v,
                                                                        crate::opcodes_h::OP_String8,
                                                                        0 as ::core::ffi::c_int,
                                                                        3 as ::core::ffi::c_int,
                                                                        0 as ::core::ffi::c_int,
                                                                        zErr_1,
                                                                        crate::src::src::vdbe::P4_DYNAMIC,
                                                                    );
                                                                    integrityCheckResultRow(v);
                                                                    crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(
                                                                        v, addrCkOk,
                                                                    );
                                                                }
                                                                crate::src::src::expr::sqlite3ExprListDelete(db as *mut crate::sqliteInt_h::sqlite3,  pCheck as *mut crate::sqliteInt_h::ExprList);
                                                            }
                                                            if isQuick == 0 {
                                                                j_3 = 0 as ::core::ffi::c_int;
                                                                pIdx_5 = __pTab_9_ref.pIndex;
                                                                while !pIdx_5.is_null() {
                                                                    let mut jmp2_0: ::core::ffi::c_int = 0;
                                                                    let mut jmp3_0: ::core::ffi::c_int = 0;
                                                                    let mut jmp4: ::core::ffi::c_int = 0;
                                                                    let mut jmp5: ::core::ffi::c_int = 0;
                                                                    let mut label6: ::core::ffi::c_int = 0;
                                                                    let mut kk: ::core::ffi::c_int =
                                                                        0;
                                                                    let mut ckUniq: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                                                                        
                                                                        pParse as *mut crate::sqliteInt_h::Parse,
                                                                    );
                                                                    if !(pPk_0 == pIdx_5) {
                                                                        r1 = crate::src::src::delete::sqlite3GenerateIndexKey(
                                                                            
                                                                            pParse as *mut crate::sqliteInt_h::Parse,
                                                                            
                                                                            pIdx_5 as *mut crate::sqliteInt_h::Index,
                                                                            iDataCur,
                                                                            0 as ::core::ffi::c_int,
                                                                            0 as ::core::ffi::c_int,
                                                                            &raw mut jmp3_0,
                                                                            
                                                                            pPrior as *mut crate::sqliteInt_h::Index,
                                                                            r1,
                                                                        );
                                                                        pPrior = pIdx_5;
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                            v,
                                                                            crate::opcodes_h::OP_AddImm,
                                                                            8 as ::core::ffi::c_int
                                                                                + j_3,
                                                                            1 as ::core::ffi::c_int,
                                                                        );
                                                                        let __pIdx_5_ref = unsafe { &mut *pIdx_5 };
                                                                        jmp2_0 = crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                                                                            v,
                                                                            crate::opcodes_h::OP_Found,
                                                                            iIdxCur + j_3,
                                                                            ckUniq,
                                                                            r1,
                                                                            __pIdx_5_ref.nColumn as ::core::ffi::c_int,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeLoadString(
                                                                            v,
                                                                            3 as ::core::ffi::c_int,
                                                                            b"row \0" as *const u8 as *const ::core::ffi::c_char,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                            v,
                                                                            crate::opcodes_h::OP_Concat,
                                                                            7 as ::core::ffi::c_int,
                                                                            3 as ::core::ffi::c_int,
                                                                            3 as ::core::ffi::c_int,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeLoadString(
                                                                            v,
                                                                            4 as ::core::ffi::c_int,
                                                                            b" missing from index \0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                            v,
                                                                            crate::opcodes_h::OP_Concat,
                                                                            4 as ::core::ffi::c_int,
                                                                            3 as ::core::ffi::c_int,
                                                                            3 as ::core::ffi::c_int,
                                                                        );
                                                                        jmp5 = crate::src::src::vdbeaux::sqlite3VdbeLoadString(
                                                                            v,
                                                                            4 as ::core::ffi::c_int,
                                                                            __pIdx_5_ref.zName,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                            v,
                                                                            crate::opcodes_h::OP_Concat,
                                                                            4 as ::core::ffi::c_int,
                                                                            3 as ::core::ffi::c_int,
                                                                            3 as ::core::ffi::c_int,
                                                                        );
                                                                        jmp4 =
                                                                            integrityCheckResultRow(
                                                                                v,
                                                                            );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeJumpHere(
                                                                            v, jmp2_0,
                                                                        );
                                                                        if __pTab_9_ref.tabFlags
                                                                            & crate::sqliteInt_h::TF_WithoutRowid
                                                                                as crate::src::ext::rtree::rtree::u32_0
                                                                            == 0 as crate::src::ext::rtree::rtree::u32_0
                                                                        {
                                                                            let mut jmp7: ::core::ffi::c_int = 0;
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                                v,
                                                                                crate::opcodes_h::OP_IdxRowid,
                                                                                iIdxCur + j_3,
                                                                                3 as ::core::ffi::c_int,
                                                                            );
                                                                            jmp7 = crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                                v,
                                                                                crate::opcodes_h::OP_Eq,
                                                                                3 as ::core::ffi::c_int,
                                                                                0 as ::core::ffi::c_int,
                                                                                r1 + __pIdx_5_ref.nColumn as ::core::ffi::c_int
                                                                                    - 1 as ::core::ffi::c_int,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeLoadString(
                                                                                v,
                                                                                3 as ::core::ffi::c_int,
                                                                                b"rowid not at end-of-record for row \0" as *const u8
                                                                                    as *const ::core::ffi::c_char,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                                v,
                                                                                crate::opcodes_h::OP_Concat,
                                                                                7 as ::core::ffi::c_int,
                                                                                3 as ::core::ffi::c_int,
                                                                                3 as ::core::ffi::c_int,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeLoadString(
                                                                                v,
                                                                                4 as ::core::ffi::c_int,
                                                                                b" of index \0" as *const u8 as *const ::core::ffi::c_char,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeGoto(v, jmp5 - 1 as ::core::ffi::c_int);
                                                                            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(
                                                                                v, jmp7,
                                                                            );
                                                                        }
                                                                        label6 =
                                                                            0 as ::core::ffi::c_int;
                                                                        kk =
                                                                            0 as ::core::ffi::c_int;
                                                                        while kk < __pIdx_5_ref.nKeyCol
                                                                            as ::core::ffi::c_int
                                                                        {
                                                                            if !(*__pIdx_5_ref.azColl.offset(kk as isize)
                                                                                == &raw const crate::src::src::global::sqlite3StrBINARY
                                                                                    as *const ::core::ffi::c_char)
                                                                            {
                                                                                if label6 == 0 as ::core::ffi::c_int {
                                                                                    label6 = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(pParse as *mut crate::sqliteInt_h::Parse);
                                                                                }
                                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                                    v,
                                                                                    crate::opcodes_h::OP_Column,
                                                                                    iIdxCur + j_3,
                                                                                    kk,
                                                                                    3 as ::core::ffi::c_int,
                                                                                );
                                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                                    v,
                                                                                    crate::opcodes_h::OP_Ne,
                                                                                    3 as ::core::ffi::c_int,
                                                                                    label6,
                                                                                    r1 + kk,
                                                                                );
                                                                            }
                                                                            kk += 1;
                                                                        }
                                                                        if label6 != 0 {
                                                                            let mut jmp6: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeAddOp0(
                                                                                v,
                                                                                crate::opcodes_h::OP_Goto,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(
                                                                                v, label6,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeLoadString(
                                                                                v,
                                                                                3 as ::core::ffi::c_int,
                                                                                b"row \0" as *const u8 as *const ::core::ffi::c_char,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                                v,
                                                                                crate::opcodes_h::OP_Concat,
                                                                                7 as ::core::ffi::c_int,
                                                                                3 as ::core::ffi::c_int,
                                                                                3 as ::core::ffi::c_int,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeLoadString(
                                                                                v,
                                                                                4 as ::core::ffi::c_int,
                                                                                b" values differ from index \0" as *const u8
                                                                                    as *const ::core::ffi::c_char,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeGoto(v, jmp5 - 1 as ::core::ffi::c_int);
                                                                            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(
                                                                                v, jmp6,
                                                                            );
                                                                        }
                                                                        if __pIdx_5_ref.onError
                                                                            as ::core::ffi::c_int
                                                                            != crate::sqliteInt_h::OE_None
                                                                        {
                                                                            let mut uniqOk: ::core::ffi::c_int = crate::src::src::vdbeaux::sqlite3VdbeMakeLabel(
                                                                                
                                                                                pParse as *mut crate::sqliteInt_h::Parse,
                                                                            );
                                                                            let mut jmp6_0: ::core::ffi::c_int = 0;
                                                                            kk = 0 as ::core::ffi::c_int;
                                                                            while kk < __pIdx_5_ref.nKeyCol as ::core::ffi::c_int {
                                                                                let mut iCol_0: ::core::ffi::c_int = *(*pIdx_5)
                                                                                    .aiColumn
                                                                                    .offset(kk as isize) as ::core::ffi::c_int;
                                                                                if !(iCol_0 >= 0 as ::core::ffi::c_int
                                                                                    && (*__pTab_9_ref.aCol.offset(iCol_0 as isize)).notNull()
                                                                                        as ::core::ffi::c_int != 0)
                                                                                {
                                                                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(v, crate::opcodes_h::OP_IsNull, r1 + kk, uniqOk);
                                                                                }
                                                                                kk += 1;
                                                                            }
                                                                            jmp6_0 =
                                                                                crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                                                                                    v,
                                                                                    crate::opcodes_h::OP_Next,
                                                                                    iIdxCur + j_3,
                                                                                );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeGoto(
                                                                                v, uniqOk,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(
                                                                                v, jmp6_0,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                                                                                v,
                                                                                crate::opcodes_h::OP_IdxGT,
                                                                                iIdxCur + j_3,
                                                                                uniqOk,
                                                                                r1,
                                                                                __pIdx_5_ref.nKeyCol as ::core::ffi::c_int,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeLoadString(
                                                                                v,
                                                                                3 as ::core::ffi::c_int,
                                                                                b"non-unique entry in index \0" as *const u8
                                                                                    as *const ::core::ffi::c_char,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeGoto(
                                                                                v, jmp5,
                                                                            );
                                                                            crate::src::src::vdbeaux::sqlite3VdbeResolveLabel(
                                                                                v, uniqOk,
                                                                            );
                                                                        }
                                                                        crate::src::src::vdbeaux::sqlite3VdbeJumpHere(
                                                                            v, jmp4,
                                                                        );
                                                                        crate::src::src::delete::sqlite3ResolvePartIdxLabel(
                                                                            
                                                                            pParse as *mut crate::sqliteInt_h::Parse, jmp3_0,
                                                                        );
                                                                    }
                                                                    pIdx_5 = (*pIdx_5).pNext;
                                                                    j_3 += 1;
                                                                }
                                                            }
                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                v, crate::opcodes_h::OP_Next, iDataCur, loopTop,
                                                            );
                                                            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(
                                                                v,
                                                                loopTop - 1 as ::core::ffi::c_int,
                                                            );
                                                            if !pPk_0.is_null() {
                                                                crate::src::src::expr::sqlite3ReleaseTempRange(
                                                                    
                                                                    pParse as *mut crate::sqliteInt_h::Parse,
                                                                    r2,
                                                                    (*pPk_0).nKeyCol
                                                                        as ::core::ffi::c_int,
                                                                );
                                                            }
                                                        }
                                                    }
                                                    x_1 = (*x_1).next;
                                                }
                                                let mut current_block_842: u64;
                                                x_1 = __pTbls_ref.first;
                                                while !x_1.is_null() {
                                                    let mut pTab_10: *mut crate::sqliteInt_h::Table =
                                                        (*x_1).data as *mut crate::sqliteInt_h::Table;
                                                    let mut pVTab: *mut crate::sqlite3_h::sqlite3_vtab =
                                                        ::core::ptr::null_mut::<crate::sqlite3_h::sqlite3_vtab>();
                                                    let mut a1_0: ::core::ffi::c_int = 0;
                                                    if !(tableSkipIntegrityCheck(pTab_10, pObjTab)
                                                        != 0)
                                                    {
                                                        if !((*pTab_10).eTabType
                                                            as ::core::ffi::c_int
                                                            == crate::sqliteInt_h::TABTYP_NORM)
                                                        {
                                                            if (*pTab_10).eTabType
                                                                as ::core::ffi::c_int
                                                                == crate::sqliteInt_h::TABTYP_VTAB
                                                            {
                                                                if (*pTab_10).nCol
                                                                    as ::core::ffi::c_int
                                                                    <= 0 as ::core::ffi::c_int
                                                                {
                                                                    let mut zMod: *const ::core::ffi::c_char = *(*pTab_10)
                                                                        .u
                                                                        .vtab
                                                                        .azArg
                                                                        .offset(0 as isize);
                                                                    if crate::src::src::hash::sqlite3HashFind(
                                                                        
                                                                        &raw mut (*db).aModule as *mut _ as *const crate::src::src::hash::Hash,
                                                                        zMod,
                                                                    )
                                                                    .is_null()
                                                                    {
                                                                        current_block_842 =
                                                                            17618370480342365186;
                                                                    } else {
                                                                        current_block_842 =
                                                                            10746975261342658336;
                                                                    }
                                                                } else {
                                                                    current_block_842 =
                                                                        10746975261342658336;
                                                                }
                                                                match current_block_842 {
                                                                    17618370480342365186 => {}
                                                                    _ => {
                                                                        crate::src::src::build::sqlite3ViewGetColumnNames(
                                                                            
                                                                            pParse as *mut crate::sqliteInt_h::Parse,  pTab_10 as *mut crate::sqliteInt_h::Table,
                                                                        );
                                                                        if !(*pTab_10)
                                                                            .u
                                                                            .vtab
                                                                            .p
                                                                            .is_null()
                                                                        {
                                                                            pVTab = (*(*pTab_10)
                                                                                .u
                                                                                .vtab
                                                                                .p)
                                                                                .pVtab;
                                                                            if !pVTab.is_null() {
                                                                                if !(*pVTab)
                                                                                    .pModule
                                                                                    .is_null()
                                                                                {
                                                                                    if !((*(*pVTab).pModule).iVersion < 4 as ::core::ffi::c_int)
                                                                                    {
                                                                                        if !(*(*pVTab).pModule).xIntegrity.is_none() {
                                                                                            crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                                                                                v,
                                                                                                crate::opcodes_h::OP_VCheck,
                                                                                                i_8,
                                                                                                3 as ::core::ffi::c_int,
                                                                                                isQuick,
                                                                                            );
                                                                                            (*pTab_10).nTabRef = (*pTab_10).nTabRef.wrapping_add(1);
                                                                                            crate::src::src::vdbeaux::sqlite3VdbeAppendP4(
                                                                                                v,
                                                                                                pTab_10 as *mut ::core::ffi::c_void,
                                                                                                crate::src::src::vdbe::P4_TABLEREF,
                                                                                            );
                                                                                            a1_0 = crate::src::src::vdbeaux::sqlite3VdbeAddOp1(
                                                                                                v,
                                                                                                crate::opcodes_h::OP_IsNull,
                                                                                                3 as ::core::ffi::c_int,
                                                                                            );
                                                                                            integrityCheckResultRow(v);
                                                                                            crate::src::src::vdbeaux::sqlite3VdbeJumpHere(v, a1_0);
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
                                                    x_1 = (*x_1).next;
                                                }
                                            }
                                        }
                                    }
                                    i_8 += 1;
                                }
                                static mut iLn_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                static mut endCode: [crate::src::src::vdbe::VdbeOpList; 7] = [
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_AddImm as crate::src::ext::rtree::rtree::u8_0,
    p1:  1 as ::core::ffi::c_schar,
    p2:  0 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_IfNotZero as crate::src::ext::rtree::rtree::u8_0,
    p1:  1 as ::core::ffi::c_schar,
    p2:  4 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_String8 as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  3 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_ResultRow as crate::src::ext::rtree::rtree::u8_0,
    p1:  3 as ::core::ffi::c_schar,
    p2:  1 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_Halt as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  0 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_String8 as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  3 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                    crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_Goto as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  3 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                ];
                                let mut aOp_1: *mut crate::src::src::vdbe::VdbeOp = ::core::ptr::null_mut::<crate::src::src::vdbe::VdbeOp>();
                                aOp_1 =  crate::src::src::vdbeaux::sqlite3VdbeAddOpList(
                                    v,
                                    (::core::mem::size_of::<[crate::src::src::vdbe::VdbeOpList; 7]>() as usize)
                                        .wrapping_div(::core::mem::size_of::<crate::src::src::vdbe::VdbeOpList>() as usize)
                                        as ::core::ffi::c_int,
                                    
                                    &raw const endCode as *const crate::src::src::vdbe::VdbeOpList as *const crate::src::src::vdbe::VdbeOpList,
                                    iLn_1,
                                ) as
    *mut crate::src::src::vdbe::VdbeOp;
                                if !aOp_1.is_null() {
                                    (*aOp_1.offset(0 as isize)).p2 =
                                        1 as ::core::ffi::c_int - mxErr;
                                    (*aOp_1.offset(2 as isize)).p4type =
                                        crate::src::src::vdbe::P4_STATIC as ::core::ffi::c_schar;
                                    let ref mut fresh3 =
                                        (*aOp_1.offset(2 as isize)).p4.z;
                                    *fresh3 = b"ok\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                    (*aOp_1.offset(5 as isize)).p4type =
                                        crate::src::src::vdbe::P4_STATIC as ::core::ffi::c_schar;
                                    let ref mut fresh4 =
                                        (*aOp_1.offset(5 as isize)).p4.z;
                                    *fresh4 =
                                        crate::src::src::main::sqlite3ErrStr(crate::sqlite3_h::SQLITE_CORRUPT) as *mut ::core::ffi::c_char;
                                }
                                crate::src::src::vdbeaux::sqlite3VdbeChangeP3(
                                    v,
                                    0 as ::core::ffi::c_int,
                                    crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v) - 2 as ::core::ffi::c_int,
                                );
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_ENCODING =>  {
                                static mut encnames: [EncName; 9] = [
                                    EncName {
                                        zName: b"UTF8\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        enc: crate::sqlite3_h::SQLITE_UTF8 as crate::src::ext::rtree::rtree::u8_0,
                                    },
                                    EncName {
                                        zName: b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        enc: crate::sqlite3_h::SQLITE_UTF8 as crate::src::ext::rtree::rtree::u8_0,
                                    },
                                    EncName {
                                        zName: b"UTF-16le\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        enc: crate::sqlite3_h::SQLITE_UTF16LE as crate::src::ext::rtree::rtree::u8_0,
                                    },
                                    EncName {
                                        zName: b"UTF-16be\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        enc: crate::sqlite3_h::SQLITE_UTF16BE as crate::src::ext::rtree::rtree::u8_0,
                                    },
                                    EncName {
                                        zName: b"UTF16le\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        enc: crate::sqlite3_h::SQLITE_UTF16LE as crate::src::ext::rtree::rtree::u8_0,
                                    },
                                    EncName {
                                        zName: b"UTF16be\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        enc: crate::sqlite3_h::SQLITE_UTF16BE as crate::src::ext::rtree::rtree::u8_0,
                                    },
                                    EncName {
                                        zName: b"UTF-16\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        enc: 0 as crate::src::ext::rtree::rtree::u8_0,
                                    },
                                    EncName {
                                        zName: b"UTF16\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        enc: 0 as crate::src::ext::rtree::rtree::u8_0,
                                    },
                                    EncName {
                                        zName: ::core::ptr::null::<::core::ffi::c_char>()
                                            as *mut ::core::ffi::c_char,
                                        enc: 0 as crate::src::ext::rtree::rtree::u8_0,
                                    },
                                ];
                                let mut pEnc: *const EncName = ::core::ptr::null::<EncName>();
                                if zRight.is_null() {
                                    if crate::src::src::prepare::sqlite3ReadSchema(pParse as *mut crate::sqliteInt_h::Parse) != 0 {
                                        current_block = 9454828942646539263;
                                    } else {
                                        returnSingleText(
                                            v,
                                            encnames[(*(*pParse).db).enc as usize].zName,
                                        );
                                        current_block = 17607975748632905087;
                                    }
                                } else {
                                    if (*db).mDbFlags & crate::sqliteInt_h::DBFLAG_EncodingFixed as crate::src::ext::rtree::rtree::u32_0 == 0 as crate::src::ext::rtree::rtree::u32_0
                                    {
                                        pEnc = (&raw const encnames as *const EncName)
                                            .offset(0 as isize)
                                            as *const EncName
                                            as *const EncName;
                                        while !(*pEnc).zName.is_null() {
                                            if 0 as ::core::ffi::c_int
                                                == crate::src::src::util::sqlite3StrICmp(zRight, (*pEnc).zName)
                                            {
                                                let mut enc: crate::src::ext::rtree::rtree::u8_0 =
                                                    (if (*pEnc).enc as ::core::ffi::c_int != 0 {
                                                        (*pEnc).enc as ::core::ffi::c_int
                                                    } else {
                                                        crate::sqliteInt_h::SQLITE_UTF16NATIVE
                                                    })
                                                        as crate::src::ext::rtree::rtree::u8_0;
                                                (*(*(*db)
                                                    .aDb
                                                    .offset(0 as isize))
                                                .pSchema)
                                                    .enc = enc;
                                                crate::src::src::callback::sqlite3SetTextEncoding(db as *mut crate::sqliteInt_h::sqlite3, enc);
                                                break;
                                            } else {
                                                pEnc = pEnc.offset(1);
                                            }
                                        }
                                        if (*pEnc).zName.is_null() {
                                            crate::src::src::util::sqlite3ErrorMsg(
                                                
                                                pParse as *mut crate::sqliteInt_h::Parse,
                                                b"unsupported encoding: %s\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                zRight,
                                            );
                                        }
                                    }
                                    current_block = 17607975748632905087;
                                }
                            }
    crate::src::src::pragma::PragTyp_HEADER_VALUE =>  {
                                let mut iCookie: ::core::ffi::c_int =
                                    __pPragma_ref.iArg as ::core::ffi::c_int;
                                crate::src::src::vdbeaux::sqlite3VdbeUsesBtree(v, iDb);
                                if !zRight.is_null()
                                    && __pPragma_ref.mPragFlg as ::core::ffi::c_int & crate::src::src::pragma::PragFlg_ReadOnly
                                        == 0 as ::core::ffi::c_int
                                {
                                    static mut setCookie: [crate::src::src::vdbe::VdbeOpList; 2] = [
                                        crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_Transaction as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  1 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                        crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_SetCookie as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  0 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                    ];
                                    let mut aOp_2: *mut crate::src::src::vdbe::VdbeOp = ::core::ptr::null_mut::<crate::src::src::vdbe::VdbeOp>();
                                    aOp_2 =  crate::src::src::vdbeaux::sqlite3VdbeAddOpList(
                                        v,
                                        (::core::mem::size_of::<[crate::src::src::vdbe::VdbeOpList; 2]>() as usize)
                                            .wrapping_div(
                                                ::core::mem::size_of::<crate::src::src::vdbe::VdbeOpList>() as usize
                                            )
                                            as ::core::ffi::c_int,
                                        
                                        &raw const setCookie as *const crate::src::src::vdbe::VdbeOpList as *const crate::src::src::vdbe::VdbeOpList,
                                        0 as ::core::ffi::c_int,
                                    )
    as *mut crate::src::src::vdbe::VdbeOp;
                                    (*aOp_2.offset(0 as isize)).p1 = iDb;
                                    (*aOp_2.offset(1 as isize)).p1 = iDb;
                                    (*aOp_2.offset(1 as isize)).p2 = iCookie;
                                    (*aOp_2.offset(1 as isize)).p3 =
                                        crate::src::src::util::sqlite3Atoi(zRight);
                                    (*aOp_2.offset(1 as isize)).p5 =
                                        1 as crate::src::fts5::u16_0;
                                    if iCookie == crate::src::src::btree::BTREE_SCHEMA_VERSION
                                        && (*db).flags & crate::sqliteInt_h::SQLITE_Defensive as crate::src::ext::rtree::rtree::u64_0 != 0 as crate::src::ext::rtree::rtree::u64_0
                                    {
                                        (*aOp_2.offset(1 as isize)).opcode =
                                            crate::opcodes_h::OP_Noop as crate::src::ext::rtree::rtree::u8_0;
                                    }
                                } else {
                                    static mut readCookie: [crate::src::src::vdbe::VdbeOpList; 3] = [
                                        crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_Transaction as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  0 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                        crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_ReadCookie as crate::src::ext::rtree::rtree::u8_0,
    p1:  0 as ::core::ffi::c_schar,
    p2:  1 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                        crate::src::src::vdbe::VdbeOpList {
    opcode:  crate::opcodes_h::OP_ResultRow as crate::src::ext::rtree::rtree::u8_0,
    p1:  1 as ::core::ffi::c_schar,
    p2:  1 as ::core::ffi::c_schar,
    p3:  0 as ::core::ffi::c_schar,
},
                                    ];
                                    let mut aOp_3: *mut crate::src::src::vdbe::VdbeOp = ::core::ptr::null_mut::<crate::src::src::vdbe::VdbeOp>();
                                    aOp_3 =  crate::src::src::vdbeaux::sqlite3VdbeAddOpList(
                                        v,
                                        (::core::mem::size_of::<[crate::src::src::vdbe::VdbeOpList; 3]>() as usize)
                                            .wrapping_div(
                                                ::core::mem::size_of::<crate::src::src::vdbe::VdbeOpList>() as usize
                                            )
                                            as ::core::ffi::c_int,
                                        
                                        &raw const readCookie as *const crate::src::src::vdbe::VdbeOpList as *const crate::src::src::vdbe::VdbeOpList,
                                        0 as ::core::ffi::c_int,
                                    )
    as *mut crate::src::src::vdbe::VdbeOp;
                                    (*aOp_3.offset(0 as isize)).p1 = iDb;
                                    (*aOp_3.offset(1 as isize)).p1 = iDb;
                                    (*aOp_3.offset(1 as isize)).p3 = iCookie;
                                    crate::src::src::vdbeaux::sqlite3VdbeReusable(v);
                                }
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_COMPILE_OPTIONS =>  {
                                let mut i_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                let mut zOpt: *const ::core::ffi::c_char =
                                    ::core::ptr::null::<::core::ffi::c_char>();
                                (*pParse).nMem = 1 as ::core::ffi::c_int;
                                loop {
                                    let fresh5 = i_9;
                                    i_9 = i_9 + 1;
                                    zOpt = crate::src::src::main::sqlite3_compileoption_get(fresh5);
                                    if zOpt.is_null() {
                                        break;
                                    }
                                    crate::src::src::vdbeaux::sqlite3VdbeLoadString(v, 1 as ::core::ffi::c_int, zOpt);
                                    crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                        v,
                                        crate::opcodes_h::OP_ResultRow,
                                        1 as ::core::ffi::c_int,
                                        1 as ::core::ffi::c_int,
                                    );
                                }
                                crate::src::src::vdbeaux::sqlite3VdbeReusable(v);
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_WAL_CHECKPOINT =>  {
                                let mut iBt: ::core::ffi::c_int = if !(*pId2).z.is_null() {
                                    iDb
                                } else {
                                    crate::sqliteInt_h::SQLITE_MAX_DB
                                };
                                let mut eMode_1: ::core::ffi::c_int = crate::sqlite3_h::SQLITE_CHECKPOINT_PASSIVE;
                                if !zRight.is_null() {
                                    if crate::src::src::util::sqlite3StrICmp(
                                        zRight,
                                        b"full\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        eMode_1 = crate::sqlite3_h::SQLITE_CHECKPOINT_FULL;
                                    } else if crate::src::src::util::sqlite3StrICmp(
                                        zRight,
                                        b"restart\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        eMode_1 = crate::sqlite3_h::SQLITE_CHECKPOINT_RESTART;
                                    } else if crate::src::src::util::sqlite3StrICmp(
                                        zRight,
                                        b"truncate\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        eMode_1 = crate::sqlite3_h::SQLITE_CHECKPOINT_TRUNCATE;
                                    } else if crate::src::src::util::sqlite3StrICmp(
                                        zRight,
                                        b"noop\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        eMode_1 = crate::sqlite3_h::SQLITE_CHECKPOINT_NOOP;
                                    }
                                }
                                (*pParse).nMem = 3 as ::core::ffi::c_int;
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp3(
                                    v,
                                    crate::opcodes_h::OP_Checkpoint,
                                    iBt,
                                    eMode_1,
                                    1 as ::core::ffi::c_int,
                                );
                                crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                    v,
                                    crate::opcodes_h::OP_ResultRow,
                                    1 as ::core::ffi::c_int,
                                    3 as ::core::ffi::c_int,
                                );
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_WAL_AUTOCHECKPOINT =>  {
                                if !zRight.is_null() {
                                    crate::src::src::main::sqlite3_wal_autocheckpoint(db as *mut crate::sqliteInt_h::sqlite3, crate::src::src::util::sqlite3Atoi(zRight));
                                }
                                returnSingleInt(
                                    v,
                                    (if (*db).xWalCallback
                                        == Some(
                                            crate::src::src::main::sqlite3WalDefaultHook
                                                as unsafe extern "C" fn(
                                                    *mut ::core::ffi::c_void,
                                                    *mut crate::sqliteInt_h::sqlite3,
                                                    *const ::core::ffi::c_char,
                                                    ::core::ffi::c_int,
                                                )
                                                    -> ::core::ffi::c_int,
                                        )
                                    {
                                        (*db).pWalArg as crate::stdlib::intptr_t as ::core::ffi::c_int
                                    } else {
                                        0 as ::core::ffi::c_int
                                    }) as crate::src::ext::rtree::rtree::i64_0,
                                );
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_SHRINK_MEMORY =>  {
                                crate::src::src::main::sqlite3_db_release_memory(db as *mut crate::sqliteInt_h::sqlite3);
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_OPTIMIZE =>  {
                                let mut iDbLast: ::core::ffi::c_int = 0;
                                let mut iTabCur: ::core::ffi::c_int = 0;
                                let mut k_3: *mut crate::src::src::hash::HashElem = ::core::ptr::null_mut::<crate::src::src::hash::HashElem>();
                                let mut pSchema: *mut crate::sqliteInt_h::Schema = ::core::ptr::null_mut::<crate::sqliteInt_h::Schema>();
                                let mut pTab_11: *mut crate::sqliteInt_h::Table = ::core::ptr::null_mut::<crate::sqliteInt_h::Table>();
                                let mut pIdx_6: *mut crate::sqliteInt_h::Index = ::core::ptr::null_mut::<crate::sqliteInt_h::Index>();
                                let mut szThreshold: crate::sqliteInt_h::LogEst = 0;
                                let mut zSubSql: *mut ::core::ffi::c_char =
                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                let mut opMask: crate::src::ext::rtree::rtree::u32_0 = 0;
                                let mut nLimit: ::core::ffi::c_int = 0;
                                let mut nCheck: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                let mut nBtree: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                let mut nIndex: ::core::ffi::c_int = 0;
                                if !zRight.is_null() {
                                    opMask = crate::src::src::util::sqlite3Atoi(zRight) as crate::src::ext::rtree::rtree::u32_0;
                                    if opMask & 0x2 as crate::src::ext::rtree::rtree::u32_0 == 0 as crate::src::ext::rtree::rtree::u32_0 {
                                        current_block = 17607975748632905087;
                                    } else {
                                        current_block = 12301477442606723217;
                                    }
                                } else {
                                    opMask = 0xfffe as crate::src::ext::rtree::rtree::u32_0;
                                    current_block = 12301477442606723217;
                                }
                                match current_block {
                                    17607975748632905087 => {}
                                    _ => {
                                        let __db_ref = unsafe { &mut *db };
                                        if opMask & 0x10 as crate::src::ext::rtree::rtree::u32_0 == 0 as crate::src::ext::rtree::rtree::u32_0 {
                                            nLimit = 0 as ::core::ffi::c_int;
                                        } else if __db_ref.nAnalysisLimit > 0 as ::core::ffi::c_int
                                            && __db_ref.nAnalysisLimit < SQLITE_DEFAULT_OPTIMIZE_LIMIT
                                        {
                                            nLimit = 0 as ::core::ffi::c_int;
                                        } else {
                                            nLimit = SQLITE_DEFAULT_OPTIMIZE_LIMIT;
                                        }
                                        let __pParse_ref = unsafe { &mut *pParse };
                                        let fresh6 = __pParse_ref.nTab;
                                        __pParse_ref.nTab = __pParse_ref.nTab + 1;
                                        iTabCur = fresh6;
                                        iDbLast = if !zDb.is_null() {
                                            iDb
                                        } else {
                                            __db_ref.nDb - 1 as ::core::ffi::c_int
                                        };
                                        while iDb <= iDbLast {
                                            if !(iDb == 1 as ::core::ffi::c_int) {
                                                crate::src::src::build::sqlite3CodeVerifySchema(pParse as *mut crate::sqliteInt_h::Parse, iDb);
                                                pSchema = (*__db_ref.aDb.offset(iDb as isize)).pSchema;
                                                let mut current_block_966: u64;
                                                k_3 = (*pSchema).tblHash.first;
                                                while !k_3.is_null() {
                                                    pTab_11 = (*k_3).data as *mut crate::sqliteInt_h::Table;
                                                    if (*pTab_11).eTabType as ::core::ffi::c_int
                                                        == crate::sqliteInt_h::TABTYP_NORM
                                                    {
                                                        if !(0 as ::core::ffi::c_int
                                                            == crate::src::src::util::sqlite3_strnicmp(
                                                                (*pTab_11).zName,
                                                                b"sqlite_\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                7 as ::core::ffi::c_int,
                                                            ))
                                                        {
                                                            let __pTab_11_ref = unsafe { &mut *pTab_11 };
                                                            szThreshold = __pTab_11_ref.nRowLogEst;
                                                            nIndex = 0 as ::core::ffi::c_int;
                                                            pIdx_6 = __pTab_11_ref.pIndex;
                                                            while !pIdx_6.is_null() {
                                                                nIndex += 1;
                                                                if (*pIdx_6).hasStat1() == 0 {
                                                                    szThreshold = -(1
                                                                        as ::core::ffi::c_int)
                                                                        as crate::sqliteInt_h::LogEst;
                                                                }
                                                                pIdx_6 = (*pIdx_6).pNext;
                                                            }
                                                            if __pTab_11_ref.tabFlags
                                                                & crate::sqliteInt_h::TF_MaybeReanalyze as crate::src::ext::rtree::rtree::u32_0
                                                                != 0 as crate::src::ext::rtree::rtree::u32_0
                                                            {
                                                                current_block_966 =
                                                                    3733147086002097443;
                                                            } else if opMask & 0x10000 as crate::src::ext::rtree::rtree::u32_0 != 0
                                                            {
                                                                current_block_966 =
                                                                    3733147086002097443;
                                                            } else if !__pTab_11_ref.pIndex.is_null()
                                                                && (szThreshold
                                                                    as ::core::ffi::c_int)
                                                                    < 0 as ::core::ffi::c_int
                                                            {
                                                                current_block_966 =
                                                                    3733147086002097443;
                                                            } else {
                                                                current_block_966 =
                                                                    4707016446154836770;
                                                            }
                                                            match current_block_966 {
                                                                4707016446154836770 => {}
                                                                _ => {
                                                                    nCheck += 1;
                                                                    if nCheck
                                                                        == 2 as ::core::ffi::c_int
                                                                    {
                                                                        crate::src::src::build::sqlite3BeginWriteOperation(
                                                                            
                                                                            pParse as *mut crate::sqliteInt_h::Parse,
                                                                            0 as ::core::ffi::c_int,
                                                                            iDb,
                                                                        );
                                                                    }
                                                                    nBtree += nIndex
                                                                        + 1 as ::core::ffi::c_int;
                                                                    crate::src::src::insert::sqlite3OpenTable(
                                                                        
                                                                        pParse as *mut crate::sqliteInt_h::Parse,
                                                                        iTabCur,
                                                                        iDb,
                                                                        
                                                                        pTab_11 as *mut crate::sqliteInt_h::Table,
                                                                        crate::opcodes_h::OP_OpenRead,
                                                                    );
                                                                    if szThreshold
                                                                        as ::core::ffi::c_int
                                                                        >= 0 as ::core::ffi::c_int
                                                                    {
                                                                        let iRange: crate::sqliteInt_h::LogEst =
                                                                            33 as crate::sqliteInt_h::LogEst;
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp4Int(
                                                                            v,
                                                                            crate::opcodes_h::OP_IfSizeBetween,
                                                                            iTabCur,
                                                                            ((crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int)
                                                                                as crate::src::ext::rtree::rtree::u32_0)
                                                                                .wrapping_add(opMask & 1 as crate::src::ext::rtree::rtree::u32_0) as ::core::ffi::c_int,
                                                                            if szThreshold as ::core::ffi::c_int
                                                                                >= iRange as ::core::ffi::c_int
                                                                            {
                                                                                szThreshold as ::core::ffi::c_int
                                                                                    - iRange as ::core::ffi::c_int
                                                                            } else {
                                                                                -(1 as ::core::ffi::c_int)
                                                                            },
                                                                            szThreshold as ::core::ffi::c_int
                                                                                + iRange as ::core::ffi::c_int,
                                                                        );
                                                                    } else {
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                            v,
                                                                            crate::opcodes_h::OP_Rewind,
                                                                            iTabCur,
                                                                            ((crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v) + 2 as ::core::ffi::c_int)
                                                                                as crate::src::ext::rtree::rtree::u32_0)
                                                                                .wrapping_add(opMask & 1 as crate::src::ext::rtree::rtree::u32_0) as ::core::ffi::c_int,
                                                                        );
                                                                    }
                                                                    zSubSql = crate::src::src::printf::sqlite3MPrintf(
                                                                        
                                                                        db as *mut crate::sqliteInt_h::sqlite3,
                                                                        b"ANALYZE \"%w\".\"%w\"\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        (*__db_ref.aDb.offset(iDb as isize)).zDbSName,
                                                                        __pTab_11_ref.zName,
                                                                    );
                                                                    if opMask & 0x1 as crate::src::ext::rtree::rtree::u32_0 != 0 {
                                                                        let mut r1_0: ::core::ffi::c_int = crate::src::src::expr::sqlite3GetTempReg(
                                                                            
                                                                            pParse as *mut crate::sqliteInt_h::Parse,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                                            v,
                                                                            crate::opcodes_h::OP_String8,
                                                                            0 as ::core::ffi::c_int,
                                                                            r1_0,
                                                                            0 as ::core::ffi::c_int,
                                                                            zSubSql,
                                                                            crate::src::src::vdbe::P4_DYNAMIC,
                                                                        );
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp2(
                                                                            v,
                                                                            crate::opcodes_h::OP_ResultRow,
                                                                            r1_0,
                                                                            1 as ::core::ffi::c_int,
                                                                        );
                                                                    } else {
                                                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp4(
                                                                            v,
                                                                            crate::opcodes_h::OP_SqlExec,
                                                                            if nLimit != 0 {
                                                                                0x2 as ::core::ffi::c_int
                                                                            } else {
                                                                                0 as ::core::ffi::c_int
                                                                            },
                                                                            nLimit,
                                                                            0 as ::core::ffi::c_int,
                                                                            zSubSql,
                                                                            crate::src::src::vdbe::P4_DYNAMIC,
                                                                        );
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    k_3 = (*k_3).next;
                                                }
                                            }
                                            iDb += 1;
                                        }
                                        crate::src::src::vdbeaux::sqlite3VdbeAddOp0(v, crate::opcodes_h::OP_Expire);
                                        if __db_ref.mallocFailed == 0
                                            && nLimit > 0 as ::core::ffi::c_int
                                            && nBtree > 100 as ::core::ffi::c_int
                                        {
                                            let mut iAddr_0: ::core::ffi::c_int = 0;
                                            let mut iEnd: ::core::ffi::c_int = 0;
                                            let mut aOp_4: *mut crate::src::src::vdbe::VdbeOp =
                                                ::core::ptr::null_mut::<crate::src::src::vdbe::VdbeOp>();
                                            nLimit = 100 as ::core::ffi::c_int * nLimit / nBtree;
                                            if nLimit < 100 as ::core::ffi::c_int {
                                                nLimit = 100 as ::core::ffi::c_int;
                                            }
                                            aOp_4 =  crate::src::src::vdbeaux::sqlite3VdbeGetOp(v, 0 as ::core::ffi::c_int) as
    *mut crate::src::src::vdbe::VdbeOp;
                                            iEnd = crate::src::src::vdbeaux::sqlite3VdbeCurrentAddr(v);
                                            iAddr_0 = 0 as ::core::ffi::c_int;
                                            while iAddr_0 < iEnd {
                                                if (*aOp_4.offset(iAddr_0 as isize)).opcode
                                                    as ::core::ffi::c_int
                                                    == crate::opcodes_h::OP_SqlExec
                                                {
                                                    (*aOp_4.offset(iAddr_0 as isize)).p2 = nLimit;
                                                }
                                                iAddr_0 += 1;
                                            }
                                        }
                                        current_block = 17607975748632905087;
                                    }
                                }
                            }
    crate::src::src::pragma::PragTyp_SOFT_HEAP_LIMIT =>  {
                                let mut N: crate::sqlite3_h::sqlite3_int64 = 0;
                                if !zRight.is_null()
                                    && crate::src::src::util::sqlite3DecOrHexToI64(zRight, &raw mut N) == crate::sqlite3_h::SQLITE_OK
                                {
                                    crate::src::src::malloc::sqlite3_soft_heap_limit64(N);
                                }
                                returnSingleInt(
                                    v,
                                    crate::src::src::malloc::sqlite3_soft_heap_limit64(
                                        -(1 as ::core::ffi::c_int) as crate::sqlite3_h::sqlite3_int64,
                                    ) as crate::src::ext::rtree::rtree::i64_0,
                                );
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_HARD_HEAP_LIMIT =>  {
                                let mut N_0: crate::sqlite3_h::sqlite3_int64 = 0;
                                if !zRight.is_null()
                                    && crate::src::src::util::sqlite3DecOrHexToI64(zRight, &raw mut N_0) == crate::sqlite3_h::SQLITE_OK
                                {
                                    let mut iPrior: crate::sqlite3_h::sqlite3_int64 = crate::src::src::malloc::sqlite3_hard_heap_limit64(
                                        -(1 as ::core::ffi::c_int) as crate::sqlite3_h::sqlite3_int64,
                                    );
                                    if N_0 > 0 as crate::sqlite3_h::sqlite3_int64
                                        && (iPrior == 0 as crate::sqlite3_h::sqlite3_int64 || iPrior > N_0)
                                    {
                                        crate::src::src::malloc::sqlite3_hard_heap_limit64(N_0);
                                    }
                                }
                                returnSingleInt(
                                    v,
                                    crate::src::src::malloc::sqlite3_hard_heap_limit64(
                                        -(1 as ::core::ffi::c_int) as crate::sqlite3_h::sqlite3_int64,
                                    ) as crate::src::ext::rtree::rtree::i64_0,
                                );
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_THREADS =>  {
                                let mut N_1: crate::sqlite3_h::sqlite3_int64 = 0;
                                if !zRight.is_null()
                                    && crate::src::src::util::sqlite3DecOrHexToI64(zRight, &raw mut N_1) == crate::sqlite3_h::SQLITE_OK
                                    && N_1 >= 0 as crate::sqlite3_h::sqlite3_int64
                                {
                                    crate::src::src::main::sqlite3_limit(
                                        
                                        db as *mut crate::sqliteInt_h::sqlite3,
                                        crate::sqlite3_h::SQLITE_LIMIT_WORKER_THREADS,
                                        (N_1 & 0x7fffffff as crate::sqlite3_h::sqlite3_int64) as ::core::ffi::c_int,
                                    );
                                }
                                returnSingleInt(
                                    v,
                                    crate::src::src::main::sqlite3_limit(
                                        
                                        db as *mut crate::sqliteInt_h::sqlite3,
                                        crate::sqlite3_h::SQLITE_LIMIT_WORKER_THREADS,
                                        -(1 as ::core::ffi::c_int),
                                    ) as crate::src::ext::rtree::rtree::i64_0,
                                );
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_ANALYSIS_LIMIT =>  {
                                let mut N_2: crate::sqlite3_h::sqlite3_int64 = 0;
                                if !zRight.is_null()
                                    && crate::src::src::util::sqlite3DecOrHexToI64(zRight, &raw mut N_2) == crate::sqlite3_h::SQLITE_OK
                                    && N_2 >= 0 as crate::sqlite3_h::sqlite3_int64
                                {
                                    (*db).nAnalysisLimit =
                                        (N_2 & 0x7fffffff as crate::sqlite3_h::sqlite3_int64) as ::core::ffi::c_int;
                                }
                                returnSingleInt(v, (*db).nAnalysisLimit as crate::src::ext::rtree::rtree::i64_0);
                                current_block = 17607975748632905087;
                            }
    crate::src::src::pragma::PragTyp_LOCK_STATUS =>  {
                                static mut azLockName: [*const ::core::ffi::c_char; 5] = [
                                    b"unlocked\0" as *const u8 as *const ::core::ffi::c_char,
                                    b"shared\0" as *const u8 as *const ::core::ffi::c_char,
                                    b"reserved\0" as *const u8 as *const ::core::ffi::c_char,
                                    b"pending\0" as *const u8 as *const ::core::ffi::c_char,
                                    b"exclusive\0" as *const u8 as *const ::core::ffi::c_char,
                                ];
                                let mut i_10: ::core::ffi::c_int = 0;
                                (*pParse).nMem = 2 as ::core::ffi::c_int;
                                i_10 = 0 as ::core::ffi::c_int;
                                while i_10 < (*db).nDb {
                                    let mut pBt_2: *mut crate::btreeInt_h::Btree = ::core::ptr::null_mut::<crate::btreeInt_h::Btree>();
                                    let mut zState: *const ::core::ffi::c_char =
                                        b"unknown\0" as *const u8 as *const ::core::ffi::c_char;
                                    let mut j_4: ::core::ffi::c_int = 0;
                                    if !(*(*db).aDb.offset(i_10 as isize)).zDbSName.is_null() {
                                        pBt_2 = (*(*db).aDb.offset(i_10 as isize)).pBt;
                                        if pBt_2.is_null() || crate::src::src::btree::sqlite3BtreePager(pBt_2).is_null() {
                                            zState = b"closed\0" as *const u8
                                                as *const ::core::ffi::c_char;
                                        } else if crate::src::src::main::sqlite3_file_control(
                                            
                                            db as *mut crate::sqliteInt_h::sqlite3,
                                            if i_10 != 0 {
                                                (*(*db).aDb.offset(i_10 as isize)).zDbSName
                                            } else {
                                                ::core::ptr::null_mut::<::core::ffi::c_char>()
                                            },
                                            crate::sqlite3_h::SQLITE_FCNTL_LOCKSTATE_1,
                                            &raw mut j_4 as *mut ::core::ffi::c_void,
                                        ) == crate::sqlite3_h::SQLITE_OK
                                        {
                                            zState = azLockName[j_4 as usize];
                                        }
                                        crate::src::src::vdbeaux::sqlite3VdbeMultiLoad(
                                            v,
                                            1 as ::core::ffi::c_int,
                                            b"ss\0" as *const u8 as *const ::core::ffi::c_char,
                                            (*(*db).aDb.offset(i_10 as isize)).zDbSName,
                                            zState,
                                        );
                                    }
                                    i_10 += 1;
                                }
                                current_block = 17607975748632905087;
                            }
    _ =>  {
                                if !zRight.is_null() {
                                    crate::src::src::main::sqlite3_busy_timeout(db as *mut crate::sqliteInt_h::sqlite3, crate::src::src::util::sqlite3Atoi(zRight));
                                }
                                returnSingleInt(v, (*db).busyTimeout as crate::src::ext::rtree::rtree::i64_0);
                                current_block = 17607975748632905087;
                            }
}
                        match current_block {
                            9454828942646539263 => {}
                            _ => {
                                __pPragma_ref.mPragFlg as ::core::ffi::c_int & crate::src::src::pragma::PragFlg_NoColumns1 != 0
                                    && !zRight.is_null();
                            }
                        }
                    }
                }
            }
        }
    }
    crate::src::src::malloc::sqlite3DbFree(db as *mut crate::sqliteInt_h::sqlite3, zLeft as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3DbFree(db as *mut crate::sqliteInt_h::sqlite3, zRight as *mut ::core::ffi::c_void);
}

pub const SQLITE_INTEGRITY_CHECK_ERROR_MAX: ::core::ffi::c_int = 100 as ::core::ffi::c_int;

unsafe extern "C" fn pragmaVtabConnect(
    mut db: *mut crate::sqliteInt_h::sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut _argc: ::core::ffi::c_int,
    mut _argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut crate::sqlite3_h::sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pPragma: *const crate::src::src::pragma::PragmaName = pAux as *const crate::src::src::pragma::PragmaName;
    let mut pTab: *mut PragmaVtab = ::core::ptr::null_mut::<PragmaVtab>();
    let mut rc: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut cSep: ::core::ffi::c_char = '(' as i32 as ::core::ffi::c_char;
    let mut acc: crate::sqliteInt_h::StrAccum = crate::sqliteInt_h::sqlite3_str {
    db:  ::core::ptr::null_mut::<crate::sqliteInt_h::sqlite3>(),
    zText:  ::core::ptr::null_mut::<::core::ffi::c_char>(),
    nAlloc:  0,
    mxAlloc:  0,
    nChar:  0,
    accError:  0,
    printfFlags:  0,
};
    let mut zBuf: [::core::ffi::c_char; 200] = [0; 200];
    crate::src::src::printf::sqlite3StrAccumInit(
        
        &raw mut acc as *mut _ as *mut crate::sqliteInt_h::sqlite3_str,
        
        ::core::ptr::null_mut::<crate::sqliteInt_h::sqlite3>() as
    *mut crate::sqliteInt_h::sqlite3,
        &raw mut zBuf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 200]>() as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    crate::src::src::printf::sqlite3_str_appendall(
        
        &raw mut acc as *mut _ as *mut crate::sqliteInt_h::sqlite3_str,
        b"CREATE TABLE x\0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    let __pPragma_ref = unsafe { &*pPragma };
    j = __pPragma_ref.iPragCName as ::core::ffi::c_int;
    while i < __pPragma_ref.nPragCName as ::core::ffi::c_int {
        crate::src::src::printf::sqlite3_str_appendf(
            
            &raw mut acc as *mut _ as *mut crate::sqliteInt_h::sqlite3_str,
            b"%c\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            cSep as ::core::ffi::c_int,
            pragCName[j as usize],
        );
        cSep = ',' as i32 as ::core::ffi::c_char;
        i += 1;
        j += 1;
    }
    if i == 0 as ::core::ffi::c_int {
        crate::src::src::printf::sqlite3_str_appendf(
            
            &raw mut acc as *mut _ as *mut crate::sqliteInt_h::sqlite3_str,
            b"(\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            __pPragma_ref.zName,
        );
        i += 1;
    }
    j = 0 as ::core::ffi::c_int;
    if __pPragma_ref.mPragFlg as ::core::ffi::c_int & crate::src::src::pragma::PragFlg_Result1 != 0 {
        crate::src::src::printf::sqlite3_str_appendall(
            
            &raw mut acc as *mut _ as *mut crate::sqliteInt_h::sqlite3_str,
            b",arg HIDDEN\0" as *const u8 as *const ::core::ffi::c_char,
        );
        j += 1;
    }
    if __pPragma_ref.mPragFlg as ::core::ffi::c_int & (crate::src::src::pragma::PragFlg_SchemaOpt | crate::src::src::pragma::PragFlg_SchemaReq) != 0 {
        crate::src::src::printf::sqlite3_str_appendall(
            
            &raw mut acc as *mut _ as *mut crate::sqliteInt_h::sqlite3_str,
            b",schema HIDDEN\0" as *const u8 as *const ::core::ffi::c_char,
        );
        j += 1;
    }
    crate::src::src::printf::sqlite3_str_append(
        
        &raw mut acc as *mut _ as *mut crate::sqliteInt_h::sqlite3_str,
        b")\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    crate::src::src::printf::sqlite3StrAccumFinish(&raw mut acc as *mut _ as *mut crate::sqliteInt_h::sqlite3_str);
    rc = crate::src::src::vtab::sqlite3_declare_vtab(db as *mut crate::sqliteInt_h::sqlite3, &raw mut zBuf as *mut ::core::ffi::c_char);
    if rc == crate::sqlite3_h::SQLITE_OK {
        pTab = crate::src::src::malloc::sqlite3_malloc(::core::mem::size_of::<PragmaVtab>() as ::core::ffi::c_int)
            as *mut PragmaVtab;
        if pTab.is_null() {
            rc = crate::sqlite3_h::SQLITE_NOMEM;
        } else {
            ::libc::memset(
                pTab as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<PragmaVtab>() as crate::__stddef_size_t_h::size_t,
            );
            let __pTab_ref = unsafe { &mut *pTab };
            __pTab_ref.pName = pPragma;
            __pTab_ref.db = db;
            __pTab_ref.iHidden = i as crate::src::ext::rtree::rtree::u8_0;
            __pTab_ref.nHidden = j as crate::src::ext::rtree::rtree::u8_0;
        }
    } else {
        *pzErr = crate::src::src::printf::sqlite3_mprintf(
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            crate::src::src::main::sqlite3_errmsg(db as *mut crate::sqliteInt_h::sqlite3),
        );
    }
    *ppVtab = pTab as *mut crate::sqlite3_h::sqlite3_vtab;
    return rc;
}

unsafe extern "C" fn pragmaVtabDisconnect(mut pVtab: *mut crate::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int {
    let mut pTab: *mut PragmaVtab = pVtab as *mut PragmaVtab;
    crate::src::src::malloc::sqlite3_free(pTab as *mut ::core::ffi::c_void);
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn pragmaVtabBestIndex(
    mut tab: *mut crate::sqlite3_h::sqlite3_vtab,
    mut pIdxInfo: *mut crate::sqlite3_h::sqlite3_index_info,
) -> ::core::ffi::c_int {
    let mut pTab: *mut PragmaVtab = tab as *mut PragmaVtab;
    let mut pConstraint: *const crate::sqlite3_h::sqlite3_index_constraint =
        ::core::ptr::null::<crate::sqlite3_h::sqlite3_index_constraint>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut seen: [::core::ffi::c_int; 2] = [0; 2];
    let __pIdxInfo_ref = unsafe { &mut *pIdxInfo };
    __pIdxInfo_ref.estimatedCost = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    if (*pTab).nHidden as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return crate::sqlite3_h::SQLITE_OK;
    }
    pConstraint = __pIdxInfo_ref.aConstraint;
    seen[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    seen[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < __pIdxInfo_ref.nConstraint {
        if !((*pConstraint).iColumn < (*pTab).iHidden as ::core::ffi::c_int) {
            if !((*pConstraint).op as ::core::ffi::c_int != crate::sqlite3_h::SQLITE_INDEX_CONSTRAINT_EQ) {
                if (*pConstraint).usable as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    return crate::sqlite3_h::SQLITE_CONSTRAINT;
                }
                j = (*pConstraint).iColumn - (*pTab).iHidden as ::core::ffi::c_int;
                seen[j as usize] = i + 1 as ::core::ffi::c_int;
            }
        }
        i += 1;
        pConstraint = pConstraint.offset(1);
    }
    if seen[0 as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int {
        __pIdxInfo_ref.estimatedCost = 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double;
        __pIdxInfo_ref.estimatedRows = 2147483647 as crate::sqlite3_h::sqlite3_int64;
        return crate::sqlite3_h::SQLITE_OK;
    }
    j = seen[0 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int;
    (*__pIdxInfo_ref.aConstraintUsage.offset(j as isize)).argvIndex = 1 as ::core::ffi::c_int;
    (*__pIdxInfo_ref.aConstraintUsage.offset(j as isize)).omit = 1 as ::core::ffi::c_uchar;
    __pIdxInfo_ref.estimatedCost = 20 as ::core::ffi::c_int as ::core::ffi::c_double;
    __pIdxInfo_ref.estimatedRows = 20 as crate::sqlite3_h::sqlite3_int64;
    if seen[1 as ::core::ffi::c_int as usize] != 0 {
        j = seen[1 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int;
        (*__pIdxInfo_ref.aConstraintUsage.offset(j as isize)).argvIndex = 2 as ::core::ffi::c_int;
        (*__pIdxInfo_ref.aConstraintUsage.offset(j as isize)).omit = 1 as ::core::ffi::c_uchar;
    }
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn pragmaVtabOpen(
    mut pVtab: *mut crate::sqlite3_h::sqlite3_vtab,
    mut ppCursor: *mut *mut crate::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut PragmaVtabCursor = ::core::ptr::null_mut::<PragmaVtabCursor>();
    pCsr = crate::src::src::malloc::sqlite3_malloc(::core::mem::size_of::<PragmaVtabCursor>() as ::core::ffi::c_int)
        as *mut PragmaVtabCursor;
    if pCsr.is_null() {
        return crate::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        pCsr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PragmaVtabCursor>() as crate::__stddef_size_t_h::size_t,
    );
    (*pCsr).base.pVtab = pVtab;
    *ppCursor = &raw mut (*pCsr).base;
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn pragmaVtabCursorClear(mut pCsr: *mut PragmaVtabCursor) {
    let mut i: ::core::ffi::c_int = 0;
    let __pCsr_ref = unsafe { &mut *pCsr };
    crate::src::src::vdbeapi::sqlite3_finalize(__pCsr_ref.pPragma);
    __pCsr_ref.pPragma = ::core::ptr::null_mut::<crate::sqlite3_h::sqlite3_stmt>();
    __pCsr_ref.iRowid = 0 as crate::sqlite3_h::sqlite_int64;
    i = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[*mut ::core::ffi::c_char; 2]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            as ::core::ffi::c_int
    {
        crate::src::src::malloc::sqlite3_free(__pCsr_ref.azArg[i as usize] as *mut ::core::ffi::c_void);
        __pCsr_ref.azArg[i as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
        i += 1;
    }
}

unsafe extern "C" fn pragmaVtabClose(mut cur: *mut crate::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let mut pCsr: *mut PragmaVtabCursor = cur as *mut PragmaVtabCursor;
    pragmaVtabCursorClear(pCsr);
    crate::src::src::malloc::sqlite3_free(pCsr as *mut ::core::ffi::c_void);
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn pragmaVtabNext(
    mut pVtabCursor: *mut crate::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut PragmaVtabCursor = pVtabCursor as *mut PragmaVtabCursor;
    let mut rc: ::core::ffi::c_int = crate::sqlite3_h::SQLITE_OK;
    (*pCsr).iRowid += 1;
    if crate::sqlite3_h::SQLITE_ROW != crate::src::src::vdbeapi::sqlite3_step((*pCsr).pPragma) {
        rc = crate::src::src::vdbeapi::sqlite3_finalize((*pCsr).pPragma);
        (*pCsr).pPragma = ::core::ptr::null_mut::<crate::sqlite3_h::sqlite3_stmt>();
        pragmaVtabCursorClear(pCsr);
    }
    return rc;
}

unsafe extern "C" fn pragmaVtabFilter(
    mut pVtabCursor: *mut crate::sqlite3_h::sqlite3_vtab_cursor,
    mut _idxNum: ::core::ffi::c_int,
    mut _idxStr: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut crate::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut PragmaVtabCursor = pVtabCursor as *mut PragmaVtabCursor;
    let mut pTab: *mut PragmaVtab = (*pVtabCursor).pVtab as *mut PragmaVtab;
    let mut rc: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut acc: crate::sqliteInt_h::StrAccum = crate::sqliteInt_h::sqlite3_str {
    db:  ::core::ptr::null_mut::<crate::sqliteInt_h::sqlite3>(),
    zText:  ::core::ptr::null_mut::<::core::ffi::c_char>(),
    nAlloc:  0,
    mxAlloc:  0,
    nChar:  0,
    accError:  0,
    printfFlags:  0,
};
    let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    pragmaVtabCursorClear(pCsr);
    let __pTab_ref = unsafe { &mut *pTab };
    j = if (*__pTab_ref.pName).mPragFlg as ::core::ffi::c_int & crate::src::src::pragma::PragFlg_Result1
        != 0 as ::core::ffi::c_int
    {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    i = 0 as ::core::ffi::c_int;
    let __pCsr_ref = unsafe { &mut *pCsr };
    while i < argc {
        let mut zText: *const ::core::ffi::c_char =
            crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(i as isize)) as *const ::core::ffi::c_char;
        if !zText.is_null() {
            __pCsr_ref.azArg[j as usize] =
                crate::src::src::printf::sqlite3_mprintf(b"%s\0" as *const u8 as *const ::core::ffi::c_char, zText);
            if __pCsr_ref.azArg[j as usize].is_null() {
                return crate::sqlite3_h::SQLITE_NOMEM;
            }
        }
        i += 1;
        j += 1;
    }
    crate::src::src::printf::sqlite3StrAccumInit(
        
        &raw mut acc as *mut _ as *mut crate::sqliteInt_h::sqlite3_str,
        
        ::core::ptr::null_mut::<crate::sqliteInt_h::sqlite3>() as
    *mut crate::sqliteInt_h::sqlite3,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        (*__pTab_ref.db).aLimit[crate::sqlite3_h::SQLITE_LIMIT_SQL_LENGTH as usize],
    );
    crate::src::src::printf::sqlite3_str_appendall(
        
        &raw mut acc as *mut _ as *mut crate::sqliteInt_h::sqlite3_str,
        b"PRAGMA \0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !__pCsr_ref.azArg[1 as ::core::ffi::c_int as usize].is_null() {
        crate::src::src::printf::sqlite3_str_appendf(
            
            &raw mut acc as *mut _ as *mut crate::sqliteInt_h::sqlite3_str,
            b"%Q.\0" as *const u8 as *const ::core::ffi::c_char,
            __pCsr_ref.azArg[1 as ::core::ffi::c_int as usize],
        );
    }
    crate::src::src::printf::sqlite3_str_appendall(&raw mut acc as *mut _ as *mut crate::sqliteInt_h::sqlite3_str, (*__pTab_ref.pName).zName);
    if !__pCsr_ref.azArg[0 as ::core::ffi::c_int as usize].is_null() {
        crate::src::src::printf::sqlite3_str_appendf(
            
            &raw mut acc as *mut _ as *mut crate::sqliteInt_h::sqlite3_str,
            b"=%Q\0" as *const u8 as *const ::core::ffi::c_char,
            __pCsr_ref.azArg[0 as ::core::ffi::c_int as usize],
        );
    }
    zSql = crate::src::src::printf::sqlite3StrAccumFinish(&raw mut acc as *mut _ as *mut crate::sqliteInt_h::sqlite3_str);
    if zSql.is_null() {
        return crate::sqlite3_h::SQLITE_NOMEM;
    }
    rc = crate::src::src::prepare::sqlite3_prepare_v2(
        
        __pTab_ref.db as *mut crate::sqliteInt_h::sqlite3,
        zSql,
        -(1 as ::core::ffi::c_int),
        &raw mut __pCsr_ref.pPragma,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    crate::src::src::malloc::sqlite3_free(zSql as *mut ::core::ffi::c_void);
    if rc != crate::sqlite3_h::SQLITE_OK {
        __pTab_ref.base.zErrMsg = crate::src::src::printf::sqlite3_mprintf(
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            crate::src::src::main::sqlite3_errmsg(__pTab_ref.db as *mut crate::sqliteInt_h::sqlite3),
        );
        return rc;
    }
    return pragmaVtabNext(pVtabCursor);
}

unsafe extern "C" fn pragmaVtabEof(
    mut pVtabCursor: *mut crate::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut PragmaVtabCursor = pVtabCursor as *mut PragmaVtabCursor;
    return ((*pCsr).pPragma == ::core::ptr::null_mut::<crate::sqlite3_h::sqlite3_stmt>()) as ::core::ffi::c_int;
}

unsafe extern "C" fn pragmaVtabColumn(
    mut pVtabCursor: *mut crate::sqlite3_h::sqlite3_vtab_cursor,
    mut ctx: *mut crate::vdbeInt_h::sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut PragmaVtabCursor = pVtabCursor as *mut PragmaVtabCursor;
    let mut pTab: *mut PragmaVtab = (*pVtabCursor).pVtab as *mut PragmaVtab;
    if i < (*pTab).iHidden as ::core::ffi::c_int {
        crate::src::src::vdbeapi::sqlite3_result_value(ctx, crate::src::src::vdbeapi::sqlite3_column_value((*pCsr).pPragma, i));
    } else {
        crate::src::src::vdbeapi::sqlite3_result_text(
            ctx,
            (*pCsr).azArg[(i - (*pTab).iHidden as ::core::ffi::c_int) as usize],
            -(1 as ::core::ffi::c_int),
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
        );
    }
    return crate::sqlite3_h::SQLITE_OK;
}

unsafe extern "C" fn pragmaVtabRowid(
    mut pVtabCursor: *mut crate::sqlite3_h::sqlite3_vtab_cursor,
    mut p: *mut crate::sqlite3_h::sqlite_int64,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut PragmaVtabCursor = pVtabCursor as *mut PragmaVtabCursor;
    *p = (*pCsr).iRowid;
    return crate::sqlite3_h::SQLITE_OK;
}

static mut pragmaVtabModule: crate::sqlite3_h::sqlite3_module = unsafe {
    crate::sqlite3_h::sqlite3_module {
    iVersion:  0 as ::core::ffi::c_int,
    xCreate:  None,
    xConnect:  Some(
            pragmaVtabConnect
                as unsafe extern "C" fn(
                    *mut crate::sqliteInt_h::sqlite3,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const *const ::core::ffi::c_char,
                    *mut *mut crate::sqlite3_h::sqlite3_vtab,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    xBestIndex:  Some(
            pragmaVtabBestIndex
                as unsafe extern "C" fn(
                    *mut crate::sqlite3_h::sqlite3_vtab,
                    *mut crate::sqlite3_h::sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
    xDisconnect:  Some(
            pragmaVtabDisconnect as unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int,
        ),
    xDestroy:  None,
    xOpen:  Some(
            pragmaVtabOpen
                as unsafe extern "C" fn(
                    *mut crate::sqlite3_h::sqlite3_vtab,
                    *mut *mut crate::sqlite3_h::sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
    xClose:  Some(
            pragmaVtabClose as unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
    xFilter:  Some(
            pragmaVtabFilter
                as unsafe extern "C" fn(
                    *mut crate::sqlite3_h::sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut crate::vdbeInt_h::sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
    xNext:  Some(
            pragmaVtabNext as unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
    xEof:  Some(
            pragmaVtabEof as unsafe extern "C" fn(*mut crate::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
    xColumn:  Some(
            pragmaVtabColumn
                as unsafe extern "C" fn(
                    *mut crate::sqlite3_h::sqlite3_vtab_cursor,
                    *mut crate::vdbeInt_h::sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xRowid:  Some(
            pragmaVtabRowid
                as unsafe extern "C" fn(
                    *mut crate::sqlite3_h::sqlite3_vtab_cursor,
                    *mut crate::sqlite3_h::sqlite_int64,
                ) -> ::core::ffi::c_int,
        ),
    xUpdate:  None,
    xBegin:  None,
    xSync:  None,
    xCommit:  None,
    xRollback:  None,
    xFindFunction:  None,
    xRename:  None,
    xSavepoint:  None,
    xRelease:  None,
    xRollbackTo:  None,
    xShadowName:  None,
    xIntegrity:  None,
}
};
#[no_mangle]

pub unsafe extern "C" fn sqlite3PragmaVtabRegister(
    mut db: *mut crate::sqliteInt_h::sqlite3,
    mut zName: *const ::core::ffi::c_char,
) -> *mut crate::sqliteInt_h::Module {
    let mut pName: *const crate::src::src::pragma::PragmaName = ::core::ptr::null::<crate::src::src::pragma::PragmaName>();
    pName = pragmaLocate(zName.offset(7 as isize));
    if pName.is_null() {
        return ::core::ptr::null_mut::<crate::sqliteInt_h::Module>();
    }
    if (*pName).mPragFlg as ::core::ffi::c_int & (crate::src::src::pragma::PragFlg_Result0 | crate::src::src::pragma::PragFlg_Result1)
        == 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<crate::sqliteInt_h::Module>();
    }
    return  crate::src::src::vtab::sqlite3VtabCreateModule(
        
        db as *mut crate::sqliteInt_h::sqlite3,
        zName,
        
        &raw const pragmaVtabModule as *const _ as
    *const crate::sqlite3_h::sqlite3_module,
        pName as *mut ::core::ffi::c_void,
        None,
    ) as
    *mut crate::sqliteInt_h::Module;
}
