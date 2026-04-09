// =============== BEGIN parse_h ================
pub const TK_SEMI: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const TK_EXPLAIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const TK_QUERY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const TK_PLAN: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const TK_BEGIN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const TK_TRANSACTION: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub const TK_DEFERRED: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub const TK_IMMEDIATE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

pub const TK_EXCLUSIVE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;

pub const TK_COMMIT: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

pub const TK_END: ::core::ffi::c_int = 11 as ::core::ffi::c_int;

pub const TK_ROLLBACK: ::core::ffi::c_int = 12 as ::core::ffi::c_int;

pub const TK_SAVEPOINT: ::core::ffi::c_int = 13 as ::core::ffi::c_int;

pub const TK_RELEASE: ::core::ffi::c_int = 14 as ::core::ffi::c_int;

pub const TK_TO: ::core::ffi::c_int = 15 as ::core::ffi::c_int;

pub const TK_TABLE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;

pub const TK_CREATE: ::core::ffi::c_int = 17 as ::core::ffi::c_int;

pub const TK_IF: ::core::ffi::c_int = 18 as ::core::ffi::c_int;

pub const TK_NOT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;

pub const TK_NOT_1: ::core::ffi::c_int = 19;

pub const TK_EXISTS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;

pub const TK_EXISTS_1: ::core::ffi::c_int = 20;

pub const TK_TEMP: ::core::ffi::c_int = 21 as ::core::ffi::c_int;

pub const TK_LP: ::core::ffi::c_int = 22 as ::core::ffi::c_int;

pub const TK_RP: ::core::ffi::c_int = 23 as ::core::ffi::c_int;

pub const TK_AS: ::core::ffi::c_int = 24 as ::core::ffi::c_int;

pub const TK_COMMA: ::core::ffi::c_int = 25 as ::core::ffi::c_int;

pub const TK_WITHOUT: ::core::ffi::c_int = 26 as ::core::ffi::c_int;

pub const TK_ABORT: ::core::ffi::c_int = 27 as ::core::ffi::c_int;

pub const TK_ACTION: ::core::ffi::c_int = 28 as ::core::ffi::c_int;

pub const TK_AFTER: ::core::ffi::c_int = 29 as ::core::ffi::c_int;

pub const TK_ANALYZE: ::core::ffi::c_int = 30 as ::core::ffi::c_int;

pub const TK_ASC: ::core::ffi::c_int = 31 as ::core::ffi::c_int;

pub const TK_ATTACH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;

pub const TK_BEFORE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;

pub const TK_BY: ::core::ffi::c_int = 34 as ::core::ffi::c_int;

pub const TK_CASCADE: ::core::ffi::c_int = 35 as ::core::ffi::c_int;

pub const TK_CAST: ::core::ffi::c_int = 36 as ::core::ffi::c_int;

pub const TK_CONFLICT: ::core::ffi::c_int = 37 as ::core::ffi::c_int;

pub const TK_DATABASE: ::core::ffi::c_int = 38 as ::core::ffi::c_int;

pub const TK_DESC: ::core::ffi::c_int = 39 as ::core::ffi::c_int;

pub const TK_DETACH: ::core::ffi::c_int = 40 as ::core::ffi::c_int;

pub const TK_EACH: ::core::ffi::c_int = 41 as ::core::ffi::c_int;

pub const TK_FAIL: ::core::ffi::c_int = 42 as ::core::ffi::c_int;

pub const TK_OR: ::core::ffi::c_int = 43 as ::core::ffi::c_int;

pub const TK_AND: ::core::ffi::c_int = 44 as ::core::ffi::c_int;

pub const TK_IS: ::core::ffi::c_int = 45 as ::core::ffi::c_int;

pub const TK_IS_1: ::core::ffi::c_int = 45;

pub const TK_ISNOT: ::core::ffi::c_int = 46 as ::core::ffi::c_int;

pub const TK_ISNOT_1: ::core::ffi::c_int = 46;

pub const TK_MATCH: ::core::ffi::c_int = 47 as ::core::ffi::c_int;

pub const TK_LIKE_KW: ::core::ffi::c_int = 48 as ::core::ffi::c_int;

pub const TK_BETWEEN: ::core::ffi::c_int = 49 as ::core::ffi::c_int;

pub const TK_BETWEEN_1: ::core::ffi::c_int = 49;

pub const TK_IN: ::core::ffi::c_int = 50 as ::core::ffi::c_int;

pub const TK_IN_1: ::core::ffi::c_int = 50;

pub const TK_ISNULL: ::core::ffi::c_int = 51 as ::core::ffi::c_int;

pub const TK_ISNULL_1: ::core::ffi::c_int = 51;

pub const TK_NOTNULL: ::core::ffi::c_int = 52 as ::core::ffi::c_int;

pub const TK_NOTNULL_1: ::core::ffi::c_int = 52;

pub const TK_NE: ::core::ffi::c_int = 53 as ::core::ffi::c_int;

pub const TK_NE_1: ::core::ffi::c_int = 53;

pub const TK_EQ: ::core::ffi::c_int = 54 as ::core::ffi::c_int;

pub const TK_EQ_1: ::core::ffi::c_int = 54;

pub const TK_GT: ::core::ffi::c_int = 55;

pub const TK_GT_1: ::core::ffi::c_int = 55 as ::core::ffi::c_int;

pub const TK_LE: ::core::ffi::c_int = 56 as ::core::ffi::c_int;

pub const TK_LE_1: ::core::ffi::c_int = 56;

pub const TK_LT: ::core::ffi::c_int = 57;

pub const TK_LT_1: ::core::ffi::c_int = 57 as ::core::ffi::c_int;

pub const TK_GE: ::core::ffi::c_int = 58 as ::core::ffi::c_int;

pub const TK_GE_1: ::core::ffi::c_int = 58;

pub const TK_ESCAPE: ::core::ffi::c_int = 59 as ::core::ffi::c_int;

pub const TK_ID: ::core::ffi::c_int = 60 as ::core::ffi::c_int;

pub const TK_ID_1: ::core::ffi::c_int = 60;

pub const TK_COLUMNKW: ::core::ffi::c_int = 61 as ::core::ffi::c_int;

pub const TK_DO: ::core::ffi::c_int = 62 as ::core::ffi::c_int;

pub const TK_FOR: ::core::ffi::c_int = 63 as ::core::ffi::c_int;

pub const TK_IGNORE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;

pub const TK_INITIALLY: ::core::ffi::c_int = 65 as ::core::ffi::c_int;

pub const TK_INSTEAD: ::core::ffi::c_int = 66 as ::core::ffi::c_int;

pub const TK_NO: ::core::ffi::c_int = 67 as ::core::ffi::c_int;

pub const TK_KEY: ::core::ffi::c_int = 68 as ::core::ffi::c_int;

pub const TK_OF: ::core::ffi::c_int = 69 as ::core::ffi::c_int;

pub const TK_OFFSET: ::core::ffi::c_int = 70 as ::core::ffi::c_int;

pub const TK_PRAGMA: ::core::ffi::c_int = 71 as ::core::ffi::c_int;

pub const TK_RAISE: ::core::ffi::c_int = 72 as ::core::ffi::c_int;

pub const TK_RECURSIVE: ::core::ffi::c_int = 73 as ::core::ffi::c_int;

pub const TK_REPLACE: ::core::ffi::c_int = 74 as ::core::ffi::c_int;

pub const TK_RESTRICT: ::core::ffi::c_int = 75 as ::core::ffi::c_int;

pub const TK_ROW: ::core::ffi::c_int = 76;

pub const TK_ROW_1: ::core::ffi::c_int = 76 as ::core::ffi::c_int;

pub const TK_ROWS: ::core::ffi::c_int = 77 as ::core::ffi::c_int;

pub const TK_TRIGGER: ::core::ffi::c_int = 78 as ::core::ffi::c_int;

pub const TK_VACUUM: ::core::ffi::c_int = 79 as ::core::ffi::c_int;

pub const TK_VIEW: ::core::ffi::c_int = 80 as ::core::ffi::c_int;

pub const TK_VIRTUAL: ::core::ffi::c_int = 81 as ::core::ffi::c_int;

pub const TK_WITH: ::core::ffi::c_int = 82 as ::core::ffi::c_int;

pub const TK_NULLS: ::core::ffi::c_int = 83;

pub const TK_NULLS_1: ::core::ffi::c_int = 83 as ::core::ffi::c_int;

pub const TK_FIRST: ::core::ffi::c_int = 84 as ::core::ffi::c_int;

pub const TK_LAST: ::core::ffi::c_int = 85 as ::core::ffi::c_int;

pub const TK_CURRENT: ::core::ffi::c_int = 86 as ::core::ffi::c_int;

pub const TK_FOLLOWING: ::core::ffi::c_int = 87 as ::core::ffi::c_int;

pub const TK_PARTITION: ::core::ffi::c_int = 88 as ::core::ffi::c_int;

pub const TK_PRECEDING: ::core::ffi::c_int = 89 as ::core::ffi::c_int;

pub const TK_RANGE: ::core::ffi::c_int = 90 as ::core::ffi::c_int;

pub const TK_UNBOUNDED: ::core::ffi::c_int = 91 as ::core::ffi::c_int;

pub const TK_EXCLUDE: ::core::ffi::c_int = 92 as ::core::ffi::c_int;

pub const TK_GROUPS: ::core::ffi::c_int = 93 as ::core::ffi::c_int;

pub const TK_OTHERS: ::core::ffi::c_int = 94 as ::core::ffi::c_int;

pub const TK_TIES: ::core::ffi::c_int = 95 as ::core::ffi::c_int;

pub const TK_GENERATED: ::core::ffi::c_int = 96 as ::core::ffi::c_int;

pub const TK_ALWAYS: ::core::ffi::c_int = 97 as ::core::ffi::c_int;

pub const TK_MATERIALIZED: ::core::ffi::c_int = 98 as ::core::ffi::c_int;

pub const TK_REINDEX: ::core::ffi::c_int = 99 as ::core::ffi::c_int;

pub const TK_RENAME: ::core::ffi::c_int = 100 as ::core::ffi::c_int;

pub const TK_CTIME_KW: ::core::ffi::c_int = 101 as ::core::ffi::c_int;

pub const TK_BITAND: ::core::ffi::c_int = 103;

pub const TK_BITAND_1: ::core::ffi::c_int = 103 as ::core::ffi::c_int;

pub const TK_BITOR: ::core::ffi::c_int = 104;

pub const TK_BITOR_1: ::core::ffi::c_int = 104 as ::core::ffi::c_int;

pub const TK_LSHIFT: ::core::ffi::c_int = 105;

pub const TK_LSHIFT_1: ::core::ffi::c_int = 105 as ::core::ffi::c_int;

pub const TK_RSHIFT: ::core::ffi::c_int = 106;

pub const TK_RSHIFT_1: ::core::ffi::c_int = 106 as ::core::ffi::c_int;

pub const TK_PLUS: ::core::ffi::c_int = 107 as ::core::ffi::c_int;

pub const TK_PLUS_1: ::core::ffi::c_int = 107;

pub const TK_MINUS: ::core::ffi::c_int = 108;

pub const TK_MINUS_1: ::core::ffi::c_int = 108 as ::core::ffi::c_int;

pub const TK_STAR: ::core::ffi::c_int = 109;

pub const TK_STAR_1: ::core::ffi::c_int = 109 as ::core::ffi::c_int;

pub const TK_SLASH: ::core::ffi::c_int = 110;

pub const TK_SLASH_1: ::core::ffi::c_int = 110 as ::core::ffi::c_int;

pub const TK_REM: ::core::ffi::c_int = 111;

pub const TK_REM_1: ::core::ffi::c_int = 111 as ::core::ffi::c_int;

pub const TK_CONCAT: ::core::ffi::c_int = 112;

pub const TK_CONCAT_1: ::core::ffi::c_int = 112 as ::core::ffi::c_int;

pub const TK_PTR: ::core::ffi::c_int = 113 as ::core::ffi::c_int;

pub const TK_COLLATE: ::core::ffi::c_int = 114 as ::core::ffi::c_int;

pub const TK_BITNOT: ::core::ffi::c_int = 115;

pub const TK_BITNOT_1: ::core::ffi::c_int = 115 as ::core::ffi::c_int;

pub const TK_ON: ::core::ffi::c_int = 116 as ::core::ffi::c_int;

pub const TK_INDEXED: ::core::ffi::c_int = 117 as ::core::ffi::c_int;

pub const TK_STRING: ::core::ffi::c_int = 118 as ::core::ffi::c_int;

pub const TK_JOIN_KW: ::core::ffi::c_int = 119 as ::core::ffi::c_int;

pub const TK_CONSTRAINT: ::core::ffi::c_int = 120 as ::core::ffi::c_int;

pub const TK_DEFAULT: ::core::ffi::c_int = 121 as ::core::ffi::c_int;

pub const TK_NULL: ::core::ffi::c_int = 122 as ::core::ffi::c_int;

pub const TK_PRIMARY: ::core::ffi::c_int = 123 as ::core::ffi::c_int;

pub const TK_UNIQUE: ::core::ffi::c_int = 124 as ::core::ffi::c_int;

pub const TK_CHECK: ::core::ffi::c_int = 125 as ::core::ffi::c_int;

pub const TK_REFERENCES: ::core::ffi::c_int = 126 as ::core::ffi::c_int;

pub const TK_AUTOINCR: ::core::ffi::c_int = 127 as ::core::ffi::c_int;

pub const TK_INSERT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;

pub const TK_DELETE: ::core::ffi::c_int = 129 as ::core::ffi::c_int;

pub const TK_UPDATE: ::core::ffi::c_int = 130 as ::core::ffi::c_int;

pub const TK_SET: ::core::ffi::c_int = 131 as ::core::ffi::c_int;

pub const TK_DEFERRABLE: ::core::ffi::c_int = 132 as ::core::ffi::c_int;

pub const TK_FOREIGN: ::core::ffi::c_int = 133 as ::core::ffi::c_int;

pub const TK_DROP: ::core::ffi::c_int = 134 as ::core::ffi::c_int;

pub const TK_UNION: ::core::ffi::c_int = 135 as ::core::ffi::c_int;

pub const TK_ALL: ::core::ffi::c_int = 136 as ::core::ffi::c_int;

pub const TK_EXCEPT: ::core::ffi::c_int = 137;

pub const TK_EXCEPT_1: ::core::ffi::c_int = 137 as ::core::ffi::c_int;

pub const TK_INTERSECT: ::core::ffi::c_int = 138;

pub const TK_INTERSECT_1: ::core::ffi::c_int = 138 as ::core::ffi::c_int;

pub const TK_SELECT: ::core::ffi::c_int = 139 as ::core::ffi::c_int;

pub const TK_SELECT_1: ::core::ffi::c_int = 139;

pub const TK_VALUES: ::core::ffi::c_int = 140 as ::core::ffi::c_int;

pub const TK_DISTINCT: ::core::ffi::c_int = 141 as ::core::ffi::c_int;

pub const TK_DOT: ::core::ffi::c_int = 142 as ::core::ffi::c_int;

pub const TK_DOT_1: ::core::ffi::c_int = 142;

pub const TK_FROM: ::core::ffi::c_int = 143 as ::core::ffi::c_int;

pub const TK_JOIN: ::core::ffi::c_int = 144 as ::core::ffi::c_int;

pub const TK_USING: ::core::ffi::c_int = 145 as ::core::ffi::c_int;

pub const TK_ORDER: ::core::ffi::c_int = 146 as ::core::ffi::c_int;

pub const TK_GROUP: ::core::ffi::c_int = 147 as ::core::ffi::c_int;

pub const TK_HAVING: ::core::ffi::c_int = 148 as ::core::ffi::c_int;

pub const TK_LIMIT: ::core::ffi::c_int = 149 as ::core::ffi::c_int;

pub const TK_WHERE: ::core::ffi::c_int = 150 as ::core::ffi::c_int;

pub const TK_RETURNING: ::core::ffi::c_int = 151 as ::core::ffi::c_int;

pub const TK_INTO: ::core::ffi::c_int = 152 as ::core::ffi::c_int;

pub const TK_NOTHING: ::core::ffi::c_int = 153 as ::core::ffi::c_int;

pub const TK_FLOAT: ::core::ffi::c_int = 154;

pub const TK_FLOAT_1: ::core::ffi::c_int = 154 as ::core::ffi::c_int;

pub const TK_BLOB: ::core::ffi::c_int = 155;

pub const TK_BLOB_1: ::core::ffi::c_int = 155 as ::core::ffi::c_int;

pub const TK_INTEGER: ::core::ffi::c_int = 156 as ::core::ffi::c_int;

pub const TK_VARIABLE: ::core::ffi::c_int = 157 as ::core::ffi::c_int;

pub const TK_VARIABLE_1: ::core::ffi::c_int = 157;

pub const TK_CASE: ::core::ffi::c_int = 158 as ::core::ffi::c_int;

pub const TK_CASE_1: ::core::ffi::c_int = 158;

pub const TK_WHEN: ::core::ffi::c_int = 159 as ::core::ffi::c_int;

pub const TK_THEN: ::core::ffi::c_int = 160 as ::core::ffi::c_int;

pub const TK_ELSE: ::core::ffi::c_int = 161 as ::core::ffi::c_int;

pub const TK_INDEX: ::core::ffi::c_int = 162 as ::core::ffi::c_int;

pub const TK_ALTER: ::core::ffi::c_int = 163 as ::core::ffi::c_int;

pub const TK_ADD: ::core::ffi::c_int = 164 as ::core::ffi::c_int;

pub const TK_WINDOW: ::core::ffi::c_int = 165 as ::core::ffi::c_int;

pub const TK_OVER: ::core::ffi::c_int = 166 as ::core::ffi::c_int;

pub const TK_FILTER: ::core::ffi::c_int = 167 as ::core::ffi::c_int;

pub const TK_COLUMN: ::core::ffi::c_int = 168 as ::core::ffi::c_int;

pub const TK_COLUMN_1: ::core::ffi::c_int = 168;

pub const TK_AGG_FUNCTION: ::core::ffi::c_int = 169 as ::core::ffi::c_int;

pub const TK_AGG_COLUMN: ::core::ffi::c_int = 170 as ::core::ffi::c_int;

pub const TK_TRUEFALSE: ::core::ffi::c_int = 171 as ::core::ffi::c_int;

pub const TK_FUNCTION: ::core::ffi::c_int = 172 as ::core::ffi::c_int;

pub const TK_FUNCTION_1: ::core::ffi::c_int = 172;

pub const TK_UPLUS: ::core::ffi::c_int = 173 as ::core::ffi::c_int;

pub const TK_UMINUS: ::core::ffi::c_int = 174 as ::core::ffi::c_int;

pub const TK_TRUTH: ::core::ffi::c_int = 175 as ::core::ffi::c_int;

pub const TK_REGISTER: ::core::ffi::c_int = 176 as ::core::ffi::c_int;

pub const TK_VECTOR: ::core::ffi::c_int = 177 as ::core::ffi::c_int;

pub const TK_SELECT_COLUMN: ::core::ffi::c_int = 178 as ::core::ffi::c_int;

pub const TK_IF_NULL_ROW: ::core::ffi::c_int = 179;

pub const TK_IF_NULL_ROW_1: ::core::ffi::c_int = 179 as ::core::ffi::c_int;

pub const TK_ASTERISK: ::core::ffi::c_int = 180 as ::core::ffi::c_int;

pub const TK_SPAN: ::core::ffi::c_int = 181 as ::core::ffi::c_int;

pub const TK_SPAN_1: ::core::ffi::c_int = 181;

pub const TK_ERROR: ::core::ffi::c_int = 182 as ::core::ffi::c_int;

pub const TK_QNUMBER: ::core::ffi::c_int = 183 as ::core::ffi::c_int;

pub const TK_SPACE: ::core::ffi::c_int = 184 as ::core::ffi::c_int;

pub const TK_COMMENT: ::core::ffi::c_int = 185 as ::core::ffi::c_int;

pub const TK_ILLEGAL: ::core::ffi::c_int = 186 as ::core::ffi::c_int;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::src::hash::_ht;
pub use crate::src::src::hash::Hash;
pub use crate::src::src::hash::HashElem;
pub use crate::src::src::pager::Pgno;

pub use crate::src::ext::rtree::rtree::i64_0;
pub use crate::src::ext::rtree::rtree::u8_0;
pub use crate::src::ext::rtree::rtree::u32_0;
pub use crate::src::ext::rtree::rtree::u64_0;
pub use crate::src::fts5::i16_0;
pub use crate::src::fts5::u16_0;
pub use crate::src::headers::sqlite3_h::SQLITE_LIMIT_COMPOUND_SELECT;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::sqlite_int64;
pub use crate::src::headers::sqlite3_h::sqlite_uint64;
pub use crate::src::headers::sqlite3_h::sqlite3_file;
pub use crate::src::headers::sqlite3_h::sqlite3_filename;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;
pub use crate::src::headers::sqlite3_h::sqlite3_index_info;
pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;
pub use crate::src::headers::sqlite3_h::sqlite3_int64;
pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;
pub use crate::src::headers::sqlite3_h::sqlite3_module;
pub use crate::src::headers::sqlite3_h::sqlite3_syscall_ptr;
pub use crate::src::headers::sqlite3_h::sqlite3_uint64;
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
pub use crate::src::headers::sqliteInt_h::CollSeq;
pub use crate::src::headers::sqliteInt_h::Column;
pub use crate::src::headers::sqliteInt_h::Cte;
pub use crate::src::headers::sqliteInt_h::CteUse;
pub use crate::src::headers::sqliteInt_h::DBFLAG_EncodingFixed;
pub use crate::src::headers::sqliteInt_h::Db;
pub use crate::src::headers::sqliteInt_h::DbClientData;
pub use crate::src::headers::sqliteInt_h::EP_Collate;
pub use crate::src::headers::sqliteInt_h::EP_HasFunc;
pub use crate::src::headers::sqliteInt_h::EP_InfixFunc;
pub use crate::src::headers::sqliteInt_h::EP_Leaf;
pub use crate::src::headers::sqliteInt_h::EP_Propagate;
pub use crate::src::headers::sqliteInt_h::EP_Subquery;
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
pub use crate::src::headers::sqliteInt_h::JT_INNER;
pub use crate::src::headers::sqliteInt_h::KeyInfo;
pub use crate::src::headers::sqliteInt_h::LogEst;
pub use crate::src::headers::sqliteInt_h::Lookaside;
pub use crate::src::headers::sqliteInt_h::LookasideSlot;
pub use crate::src::headers::sqliteInt_h::M10d_Any;
pub use crate::src::headers::sqliteInt_h::M10d_No;
pub use crate::src::headers::sqliteInt_h::M10d_Yes;
pub use crate::src::headers::sqliteInt_h::Module;
pub use crate::src::headers::sqliteInt_h::OE_Abort;
pub use crate::src::headers::sqliteInt_h::OE_Cascade;
pub use crate::src::headers::sqliteInt_h::OE_Default;
pub use crate::src::headers::sqliteInt_h::OE_Fail;
pub use crate::src::headers::sqliteInt_h::OE_Ignore;
pub use crate::src::headers::sqliteInt_h::OE_None;
pub use crate::src::headers::sqliteInt_h::OE_Replace;
pub use crate::src::headers::sqliteInt_h::OE_Restrict;
pub use crate::src::headers::sqliteInt_h::OE_Rollback;
pub use crate::src::headers::sqliteInt_h::OE_SetDflt;
pub use crate::src::headers::sqliteInt_h::OE_SetNull;
pub use crate::src::headers::sqliteInt_h::OnOrUsing;
pub use crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME;
pub use crate::src::headers::sqliteInt_h::Parse;
pub use crate::src::headers::sqliteInt_h::ParseCleanup;
pub use crate::src::headers::sqliteInt_h::RenameToken;
pub use crate::src::headers::sqliteInt_h::Returning;
pub use crate::src::headers::sqliteInt_h::SAVEPOINT_BEGIN;
pub use crate::src::headers::sqliteInt_h::SAVEPOINT_RELEASE;
pub use crate::src::headers::sqliteInt_h::SAVEPOINT_ROLLBACK;
pub use crate::src::headers::sqliteInt_h::SF_All;
pub use crate::src::headers::sqliteInt_h::SF_Compound;
pub use crate::src::headers::sqliteInt_h::SF_Distinct;
pub use crate::src::headers::sqliteInt_h::SF_MultiValue;
pub use crate::src::headers::sqliteInt_h::SF_NestedFrom;
pub use crate::src::headers::sqliteInt_h::SF_Values;
pub use crate::src::headers::sqliteInt_h::SQLITE_IDXTYPE_APPDEF;
pub use crate::src::headers::sqliteInt_h::SQLITE_IDXTYPE_UNIQUE;
pub use crate::src::headers::sqliteInt_h::SQLITE_SO_ASC;
pub use crate::src::headers::sqliteInt_h::SQLITE_SO_DESC;
pub use crate::src::headers::sqliteInt_h::SQLITE_SO_UNDEFINED;
pub use crate::src::headers::sqliteInt_h::SRT_Output;
pub use crate::src::headers::sqliteInt_h::Savepoint;
pub use crate::src::headers::sqliteInt_h::Schema;
pub use crate::src::headers::sqliteInt_h::Select;
pub use crate::src::headers::sqliteInt_h::SelectDest;
pub use crate::src::headers::sqliteInt_h::SrcItem;
pub use crate::src::headers::sqliteInt_h::SrcList;
pub use crate::src::headers::sqliteInt_h::Subquery;
pub use crate::src::headers::sqliteInt_h::TF_NoVisibleRowid;
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
pub use crate::src::headers::sqliteInt_h::Window;
pub use crate::src::headers::sqliteInt_h::With;
pub use crate::src::headers::sqliteInt_h::bft;
pub use crate::src::headers::sqliteInt_h::sColMap;
pub use crate::src::headers::sqliteInt_h::sqlite3;
pub use crate::src::headers::sqliteInt_h::sqlite3_xauth;
pub use crate::src::headers::sqliteInt_h::sqlite3InitInfo;
pub use crate::src::headers::sqliteInt_h::yDbMask;
pub use crate::src::headers::sqliteInt_h::ynVar;
pub use crate::src::headers::stdlib::int16_t;
pub use crate::src::headers::vdbeInt_h::PreUpdate;
pub use crate::src::headers::vdbeInt_h::sqlite3_context;
pub use crate::src::headers::vdbeInt_h::sqlite3_value;
pub use crate::src::src::alter::sqlite3AlterBeginAddColumn;
pub use crate::src::src::alter::sqlite3AlterDropColumn;
pub use crate::src::src::alter::sqlite3AlterFinishAddColumn;
pub use crate::src::src::alter::sqlite3AlterRenameColumn;
pub use crate::src::src::alter::sqlite3AlterRenameTable;
pub use crate::src::src::alter::sqlite3RenameTokenMap;
pub use crate::src::src::alter::sqlite3RenameTokenRemap;
pub use crate::src::src::analyze::sqlite3Analyze;
pub use crate::src::src::attach::sqlite3Attach;
pub use crate::src::src::attach::sqlite3Detach;
pub use crate::src::src::build::sqlite3AddCheckConstraint;
pub use crate::src::src::build::sqlite3AddCollateType;
pub use crate::src::src::build::sqlite3AddColumn;
pub use crate::src::src::build::sqlite3AddDefaultValue;
pub use crate::src::src::build::sqlite3AddGenerated;
pub use crate::src::src::build::sqlite3AddNotNull;
pub use crate::src::src::build::sqlite3AddPrimaryKey;
pub use crate::src::src::build::sqlite3AddReturning;
pub use crate::src::src::build::sqlite3BeginTransaction;
pub use crate::src::src::build::sqlite3CreateForeignKey;
pub use crate::src::src::build::sqlite3CreateIndex;
pub use crate::src::src::build::sqlite3CreateView;
pub use crate::src::src::build::sqlite3CteNew;
pub use crate::src::src::build::sqlite3DeferForeignKey;
pub use crate::src::src::build::sqlite3DropIndex;
pub use crate::src::src::build::sqlite3DropTable;
pub use crate::src::src::build::sqlite3EndTable;
pub use crate::src::src::build::sqlite3EndTransaction;
pub use crate::src::src::build::sqlite3FinishCoding;
pub use crate::src::src::build::sqlite3IdListAppend;
pub use crate::src::src::build::sqlite3IdListDelete;
pub use crate::src::src::build::sqlite3NameFromToken;
pub use crate::src::src::build::sqlite3Reindex;
pub use crate::src::src::build::sqlite3Savepoint;
pub use crate::src::src::build::sqlite3SrcListAppend;
pub use crate::src::src::build::sqlite3SrcListAppendFromTerm;
pub use crate::src::src::build::sqlite3SrcListAppendList;
pub use crate::src::src::build::sqlite3SrcListDelete;
pub use crate::src::src::build::sqlite3SrcListFuncArgs;
pub use crate::src::src::build::sqlite3SrcListIndexedBy;
pub use crate::src::src::build::sqlite3SrcListShiftJoinType;
pub use crate::src::src::build::sqlite3StartTable;
pub use crate::src::src::build::sqlite3WithAdd;
pub use crate::src::src::build::sqlite3WithDelete;
pub use crate::src::src::delete::sqlite3DeleteFrom;
pub use crate::src::src::expr::sqlite3Expr;
pub use crate::src::src::expr::sqlite3ExprAddCollateToken;
pub use crate::src::src::expr::sqlite3ExprAddFunctionOrderBy;
pub use crate::src::src::expr::sqlite3ExprAlloc;
pub use crate::src::src::expr::sqlite3ExprAnd;
pub use crate::src::src::expr::sqlite3ExprAssignVarNumber;
pub use crate::src::src::expr::sqlite3ExprAttachSubtrees;
pub use crate::src::src::expr::sqlite3ExprDelete;
pub use crate::src::src::expr::sqlite3ExprFunction;
pub use crate::src::src::expr::sqlite3ExprIdToTrueFalse;
pub use crate::src::src::expr::sqlite3ExprIsConstant;
pub use crate::src::src::expr::sqlite3ExprListAppend;
pub use crate::src::src::expr::sqlite3ExprListAppendVector;
pub use crate::src::src::expr::sqlite3ExprListCheckLength;
pub use crate::src::src::expr::sqlite3ExprListDelete;
pub use crate::src::src::expr::sqlite3ExprListSetName;
pub use crate::src::src::expr::sqlite3ExprListSetSortOrder;
pub use crate::src::src::expr::sqlite3ExprListSetSpan;
pub use crate::src::src::expr::sqlite3ExprListToValues;
pub use crate::src::src::expr::sqlite3ExprSetErrorOffset;
pub use crate::src::src::expr::sqlite3ExprSetHeightAndFlags;
pub use crate::src::src::expr::sqlite3ExprUnmapAndDelete;
pub use crate::src::src::expr::sqlite3PExpr;
pub use crate::src::src::expr::sqlite3PExprAddSelect;
pub use crate::src::src::global::sqlite3CtypeMap;
pub use crate::src::src::insert::sqlite3Insert;
pub use crate::src::src::insert::sqlite3MultiValues;
pub use crate::src::src::insert::sqlite3MultiValuesEnd;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3_realloc;
pub use crate::src::src::malloc::sqlite3DbMallocRawNN;
pub use crate::src::src::malloc::sqlite3DbMallocZero;
pub use crate::src::src::malloc::sqlite3DbStrNDup;
pub use crate::src::src::malloc::sqlite3OomFault;
pub use crate::src::src::mutex_unix::sqlite3_mutex;
pub use crate::src::src::pragma::sqlite3Pragma;
pub use crate::src::src::prepare::sqlite3ReadSchema;
pub use crate::src::src::select::sqlite3JoinType;
pub use crate::src::src::select::sqlite3Select;
pub use crate::src::src::select::sqlite3SelectDelete;
pub use crate::src::src::select::sqlite3SelectNew;
pub use crate::src::src::select::sqlite3SelectOpName;
pub use crate::src::src::select::sqlite3WithPush;
pub use crate::src::src::trigger::sqlite3BeginTrigger;
pub use crate::src::src::trigger::sqlite3DeleteTriggerStep;
pub use crate::src::src::trigger::sqlite3DropTrigger;
pub use crate::src::src::trigger::sqlite3FinishTrigger;
pub use crate::src::src::trigger::sqlite3TriggerDeleteStep;
pub use crate::src::src::trigger::sqlite3TriggerInsertStep;
pub use crate::src::src::trigger::sqlite3TriggerSelectStep;
pub use crate::src::src::trigger::sqlite3TriggerUpdateStep;
pub use crate::src::src::update::sqlite3Update;
pub use crate::src::src::upsert::sqlite3UpsertNew;
pub use crate::src::src::util::sqlite3_strnicmp;
pub use crate::src::src::util::sqlite3DequoteExpr;
pub use crate::src::src::util::sqlite3DequoteNumber;
pub use crate::src::src::util::sqlite3FaultSim;
pub use crate::src::src::util::sqlite3GetInt32;
pub use crate::src::src::vacuum::sqlite3Vacuum;
pub use crate::src::src::vtab::sqlite3VtabArgExtend;
pub use crate::src::src::vtab::sqlite3VtabArgInit;
pub use crate::src::src::vtab::sqlite3VtabBeginParse;
pub use crate::src::src::vtab::sqlite3VtabFinishParse;
pub use crate::src::src::window::sqlite3WindowAlloc;
pub use crate::src::src::window::sqlite3WindowAssemble;
pub use crate::src::src::window::sqlite3WindowAttach;
pub use crate::src::src::window::sqlite3WindowChain;
pub use crate::src::src::window::sqlite3WindowDelete;
pub use crate::src::src::window::sqlite3WindowListDelete;

pub use crate::src::headers::stdlib::__int16_t;
pub use crate::src::headers::stdlib::__uint8_t;
pub use crate::src::headers::stdlib::__uint16_t;
pub use crate::src::headers::stdlib::__uint32_t;
pub use crate::src::headers::stdlib::uint8_t;
pub use crate::src::headers::stdlib::uint16_t;
pub use crate::src::headers::stdlib::uint32_t;
pub use crate::src::headers::vdbeInt_h::Vdbe;
pub use crate::src::src::vdbe::Mem;
pub use crate::src::src::vdbe::SubProgram;
pub use crate::src::src::vdbe::SubrtnSig;
pub use crate::src::src::vdbe::VdbeOp;
pub use crate::src::src::vdbe::p4union;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yyParser {
    pub yytos: *mut yyStackEntry,
    pub pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    pub yystackEnd: *mut yyStackEntry,
    pub yystack: *mut yyStackEntry,
    pub yystk0: [yyStackEntry; 100],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yyStackEntry {
    pub stateno: ::core::ffi::c_ushort,
    pub major: ::core::ffi::c_ushort,
    pub minor: YYMINORTYPE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYMINORTYPE {
    pub yyinit: ::core::ffi::c_int,
    pub yy0: crate::src::headers::sqliteInt_h::Token,
    pub yy9: crate::src::ext::rtree::rtree::u32_0,
    pub yy28: TrigEvent,
    pub yy125: *mut crate::src::headers::sqliteInt_h::With,
    pub yy204: *mut crate::src::headers::sqliteInt_h::IdList,
    pub yy205: FrameBound,
    pub yy319: *mut crate::src::headers::sqliteInt_h::TriggerStep,
    pub yy342: *const ::core::ffi::c_char,
    pub yy361: *mut crate::src::headers::sqliteInt_h::Cte,
    pub yy402: *mut crate::src::headers::sqliteInt_h::ExprList,
    pub yy403: *mut crate::src::headers::sqliteInt_h::Upsert,
    pub yy421: crate::src::headers::sqliteInt_h::OnOrUsing,
    pub yy444: crate::src::ext::rtree::rtree::u8_0,
    pub yy481: C2RustUnnamed,
    pub yy483: *mut crate::src::headers::sqliteInt_h::Window,
    pub yy502: ::core::ffi::c_int,
    pub yy563: *mut crate::src::headers::sqliteInt_h::SrcList,
    pub yy590: *mut crate::src::headers::sqliteInt_h::Expr,
    pub yy637: *mut crate::src::headers::sqliteInt_h::Select,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub value: ::core::ffi::c_int,
    pub mask: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameBound {
    pub eType: ::core::ffi::c_int,
    pub pExpr: *mut crate::src::headers::sqliteInt_h::Expr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TrigEvent {
    pub a: ::core::ffi::c_int,
    pub b: *mut crate::src::headers::sqliteInt_h::IdList,
}

unsafe extern "C" fn parserSyntaxError(
    mut pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut p: *mut crate::src::headers::sqliteInt_h::Token,
) {
    crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        b"near \"%T\": syntax error\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::src::src::printf::PrintfArg::Token(
            p as *mut crate::src::headers::sqliteInt_h::Token,
        )],
    );
}

unsafe extern "C" fn disableLookaside(mut pParse: *mut crate::src::headers::sqliteInt_h::Parse) {
    let __pParse_ref = unsafe { &mut *pParse };
    let mut db: *mut crate::src::headers::sqliteInt_h::sqlite3 = __pParse_ref.db;
    __pParse_ref.disableLookaside = __pParse_ref.disableLookaside.wrapping_add(1);
    ::libc::memset(
        &raw mut __pParse_ref.u1.cr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<crate::src::headers::sqliteInt_h::__anon_struct_7>()
            as crate::__stddef_size_t_h::size_t,
    );
    let __db_ref = unsafe { &mut *db };
    __db_ref.lookaside.bDisable = __db_ref.lookaside.bDisable.wrapping_add(1);
    __db_ref.lookaside.sz = 0 as crate::src::fts5::u16_0;
}

pub const YYWILDCARD: ::core::ffi::c_int = 102 as ::core::ffi::c_int;

unsafe extern "C" fn parserDoubleLinkSelect(
    mut pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut p: *mut crate::src::headers::sqliteInt_h::Select,
) {
    if !(*p).pPrior.is_null() {
        let mut pNext: *mut crate::src::headers::sqliteInt_h::Select =
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
        let mut pLoop: *mut crate::src::headers::sqliteInt_h::Select = p;
        let mut mxSelect: ::core::ffi::c_int = 0;
        let mut cnt: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        loop {
            (*pLoop).pNext = pNext;
            (*pLoop).selFlags |= crate::src::headers::sqliteInt_h::SF_Compound
                as crate::src::ext::rtree::rtree::u32_0;
            pNext = pLoop;
            pLoop = (*pLoop).pPrior;
            if pLoop.is_null() {
                break;
            }
            cnt += 1;
            if !(!(*pLoop).pOrderBy.is_null() || !(*pLoop).pLimit.is_null()) {
                continue;
            }
            crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                b"%s clause should come after %s not before\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[
                    crate::src::src::printf::PrintfArg::Str(if !(*pLoop).pOrderBy.is_null() {
                        b"ORDER BY\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"LIMIT\0" as *const u8 as *const ::core::ffi::c_char
                    }
                        as *mut ::core::ffi::c_char),
                    crate::src::src::printf::PrintfArg::Str(
                        crate::src::src::select::sqlite3SelectOpName(
                            (*pNext).op as ::core::ffi::c_int,
                        ) as *mut ::core::ffi::c_char,
                    ),
                ],
            );
            break;
        }
        if (*p).selFlags
            & (crate::src::headers::sqliteInt_h::SF_MultiValue
                | crate::src::headers::sqliteInt_h::SF_Values)
                as crate::src::ext::rtree::rtree::u32_0
            == 0 as crate::src::ext::rtree::rtree::u32_0
            && {
                mxSelect = (*(*pParse).db).aLimit
                    [crate::src::headers::sqlite3_h::SQLITE_LIMIT_COMPOUND_SELECT as usize];
                mxSelect > 0 as ::core::ffi::c_int
            }
            && cnt > mxSelect
        {
            crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                b"too many terms in compound SELECT\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
    }
}

pub const YYSTACKDEPTH: ::core::ffi::c_int = 100 as ::core::ffi::c_int;

unsafe extern "C" fn attachWithToSelect(
    mut pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut pSelect: *mut crate::src::headers::sqliteInt_h::Select,
    mut pWith: *mut crate::src::headers::sqliteInt_h::With,
) -> *mut crate::src::headers::sqliteInt_h::Select {
    if !pSelect.is_null() {
        (*pSelect).pWith = pWith;
        parserDoubleLinkSelect(pParse, pSelect);
    } else {
        crate::src::src::build::sqlite3WithDelete(
            (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            pWith as *mut crate::src::headers::sqliteInt_h::With,
        );
    }
    pSelect
}

pub const YY_MAX_SHIFT: ::core::ffi::c_int = 586 as ::core::ffi::c_int;

pub const YY_MIN_SHIFTREDUCE: ::core::ffi::c_int = 849 as ::core::ffi::c_int;

unsafe extern "C" fn parserStackRealloc(
    mut pOld: *mut ::core::ffi::c_void,
    mut newSize: crate::src::headers::sqlite3_h::sqlite3_uint64,
) -> *mut ::core::ffi::c_void {
    if crate::src::src::util::sqlite3FaultSim(700 as ::core::ffi::c_int) != 0 {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    } else {
        crate::src::src::malloc::sqlite3_realloc(pOld, newSize as ::core::ffi::c_int)
    }
}

pub const YY_MAX_SHIFTREDUCE: ::core::ffi::c_int = 1257 as ::core::ffi::c_int;

pub const YY_ACCEPT_ACTION: ::core::ffi::c_int = 1259 as ::core::ffi::c_int;

pub const YY_MIN_REDUCE: ::core::ffi::c_int = 1261 as ::core::ffi::c_int;

pub const YY_MIN_DSTRCTR: ::core::ffi::c_int = 206 as ::core::ffi::c_int;

static mut yy_action: [::core::ffi::c_ushort; 2207] = [
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    282 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    282 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1332 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1311 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    464 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    289 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    289 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1626 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    385 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1332 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1304 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1546 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    485 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    528 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    464 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    463 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    987 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    294 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    379 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    502 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    61 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    61 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    988 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1581 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    182 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    485 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    540 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    436 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    529 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    561 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1294 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    425 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    563 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    296 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    464 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    402 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1253 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1054 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1054 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1068 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1071 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    586 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    585 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1178 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1178 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1588 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    383 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    485 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1345 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    460 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    299 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1345 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    502 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    866 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1285 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    283 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    527 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    527 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    582 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    582 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1349 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1058 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    551 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1545 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    302 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1579 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    474 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    554 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    459 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    554 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    459 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    542 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    487 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1023 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1089 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    93 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    405 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1502 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    430 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    267 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    348 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    471 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    334 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1285 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    6 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    515 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    512 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    511 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1283 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    94 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1023 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    468 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    555 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    555 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    510 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1575 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    38 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    415 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    534 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    402 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1152 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    39 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    348 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    379 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    475 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    316 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    264 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    231 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    46 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    160 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    303 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    446 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1586 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    548 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    878 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1586 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    384 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    386 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    364 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    182 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    561 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1343 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    561 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    561 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1281 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    476 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    350 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    530 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    535 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    560 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    443 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1515 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    469 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1515 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1517 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    536 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    427 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    516 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    415 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    878 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    564 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1297 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    497 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    507 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1296 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    515 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    512 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    511 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1632 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    572 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    275 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    510 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1515 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    331 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    552 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    552 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    575 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    537 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    987 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    890 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    96 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1602 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1237 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    988 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1454 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    569 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    229 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    526 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    538 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1337 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1337 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1453 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1599 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    975 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1044 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    422 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    883 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    952 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1599 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    581 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    367 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    421 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    886 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1599 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    466 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1034 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1035 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    574 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    574 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    574 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    197 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    427 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1044 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    572 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    40 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    392 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    388 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    886 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1033 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    427 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    575 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    533 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1572 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    579 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    496 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    493 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1191 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1335 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1335 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    953 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    435 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1034 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    569 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    481 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    212 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1044 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    213 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    974 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1045 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    581 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    491 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    382 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1034 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1035 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    465 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1044 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    381 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1153 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1661 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    539 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1661 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    441 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    906 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1572 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    368 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    416 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    329 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1033 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    523 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    3 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1663 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    403 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    897 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    519 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    502 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    948 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1028 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    544 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    907 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    947 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    396 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1034 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    410 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    298 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1153 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1662 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1662 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    145 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    145 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    293 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    518 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1572 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1509 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    438 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    439 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    543 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1510 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    376 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    521 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    371 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    520 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1578 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    485 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    367 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1572 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    381 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1572 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    437 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1211 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1631 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    915 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    120 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    502 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    442 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    340 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1439 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    453 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    453 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    453 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1372 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    315 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    81 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    81 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    304 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1574 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    381 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1211 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1344 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    390 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    381 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    467 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1556 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    337 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    467 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    339 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    161 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    381 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    391 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    432 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    10 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    449 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    267 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1558 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    536 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    926 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    147 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    147 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    927 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    502 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    491 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    556 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    285 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    973 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    66 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    66 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1439 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    67 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    67 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    21 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    21 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    499 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    338 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    297 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    297 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1340 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1315 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    450 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    578 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    940 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    940 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    126 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    141 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    537 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    433 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    476 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    350 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    461 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    347 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1439 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    407 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    502 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1554 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    324 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    491 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    973 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    550 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    68 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    68 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    557 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    69 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    69 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    355 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    6 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    948 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    414 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1439 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    947 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    454 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    549 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    222 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1184 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    483 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    973 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    434 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    70 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    70 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    71 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    71 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1057 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    166 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    854 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    855 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    856 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1286 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    423 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    73 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    73 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1314 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    362 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1184 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    305 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    572 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    498 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    492 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    575 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    568 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    538 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    455 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1606 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    586 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    569 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1439 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    489 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    357 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    360 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1345 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    59 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    59 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    973 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    573 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    423 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    74 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    74 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    75 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    75 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    231 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1370 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    76 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    76 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1044 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    424 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    184 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    77 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    77 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    97 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    581 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    524 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    577 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    144 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    144 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    478 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    482 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    572 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    415 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    447 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    415 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    78 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    78 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    62 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    62 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    79 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    79 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    575 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    319 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1034 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1035 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    422 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    63 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    63 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    290 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    415 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    9 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    80 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    80 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1148 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    404 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    490 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    459 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    325 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    346 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    64 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    64 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    477 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    569 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    170 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    170 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    171 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    171 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    87 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    87 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    328 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    65 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    65 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    546 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    83 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    83 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    545 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    572 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    345 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    84 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    84 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    168 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    168 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1044 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    148 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    148 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1384 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    575 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1025 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    428 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    581 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    557 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    162 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    162 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    532 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    893 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    375 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    152 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    152 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1383 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    149 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    149 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    374 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    150 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    150 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    569 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    484 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    86 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    86 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1096 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1034 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1035 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    546 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    486 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    580 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    470 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    547 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    572 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1620 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    88 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    88 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    85 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    85 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    479 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1044 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    222 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    905 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    904 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    575 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    244 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1036 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    893 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    581 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    912 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    913 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    300 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    351 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    508 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    365 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    165 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1092 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    978 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1096 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    990 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    991 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    946 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    943 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    569 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    876 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    159 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    945 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1561 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1034 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1035 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    546 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    341 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1534 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1533 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    545 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    503 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1593 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    494 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    352 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1380 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    356 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    359 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1036 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    361 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1044 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    363 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1328 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    370 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    567 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    380 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1393 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1438 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1366 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    280 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1378 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    581 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    167 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1443 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1293 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1284 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1613 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1363 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    314 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    401 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    237 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    224 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1425 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    295 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    333 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    336 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1430 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    343 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    488 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    344 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    513 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1375 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1616 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1376 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1429 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    408 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    301 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1034 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1035 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1605 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1196 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    458 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    349 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1311 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    373 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1506 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1505 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    400 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    400 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    399 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    397 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1374 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1373 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    863 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1553 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    186 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    572 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    235 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    395 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    210 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    211 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1551 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    327 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    426 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    96 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    575 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    557 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    326 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    180 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    472 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1420 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    332 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1413 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    335 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    570 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    190 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    191 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    193 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    473 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    506 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    247 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1434 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    495 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    199 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    406 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    480 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1426 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    409 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    102 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    14 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    501 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1511 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1432 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    569 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1431 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    203 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    92 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    95 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1500 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    175 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    281 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    358 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    504 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    253 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    411 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    522 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1331 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1522 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    354 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    440 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1322 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1330 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1630 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1629 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    897 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    229 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1044 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    444 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    445 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    448 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1329 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    531 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    268 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    310 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    412 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1398 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1301 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    581 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1300 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    372 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1299 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1628 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    378 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    311 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    90 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    572 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1397 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1598 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    418 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1487 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    389 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1584 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    559 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    541 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    575 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1583 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    42 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1354 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    387 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    583 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    216 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1353 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1034 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1035 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    393 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    394 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    462 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    419 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    584 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1538 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1264 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    172 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    420 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1539 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1537 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1536 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    308 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    225 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    89 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    569 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    850 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    226 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    457 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    174 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    322 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    330 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    176 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    929 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    189 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    243 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    342 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1120 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    194 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    178 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    429 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    431 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    98 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    196 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1044 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    99 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    185 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    100 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    179 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    249 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    163 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    250 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    581 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    456 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1196 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    458 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    353 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    200 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    400 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    400 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    399 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    397 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    500 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    863 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    15 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    865 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    558 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    505 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    400 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    400 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    399 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    397 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    374 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    863 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    327 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1034 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1035 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    326 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    26 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    509 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    327 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    366 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    514 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    369 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    895 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    908 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    517 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    326 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    309 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    164 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    525 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    181 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    27 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1189 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1073 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1159 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    284 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    230 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    286 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1175 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    982 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1182 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    976 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    175 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    28 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1179 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    29 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    30 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    31 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    8 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    32 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1164 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    41 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    175 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    553 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    33 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1087 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1074 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1072 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1076 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1077 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    565 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    18 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    939 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1037 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    877 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    124 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    37 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    398 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1197 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    576 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    153 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1621 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    418 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    418 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    571 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    462 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    462 as ::core::ffi::c_int as ::core::ffi::c_ushort,
];

static mut yy_lookahead: [::core::ffi::c_ushort; 2394] = [
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    225 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    235 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    225 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    298 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    213 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    31 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    39 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    264 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    189 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    190 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    191 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    190 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    87 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    89 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    197 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    199 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    197 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    199 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    299 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    21 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    102 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    70 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    77 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    59 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    88 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    90 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    93 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    81 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    301 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    301 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    74 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    124 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    68 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    162 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    120 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    124 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    74 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    310 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    311 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    311 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    73 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    289 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    294 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    73 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    319 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    102 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    319 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    264 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    264 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    264 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    213 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    29 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    33 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    66 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    120 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    124 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    77 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    26 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    90 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    93 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    31 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    152 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    39 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    166 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    167 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    145 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    237 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    315 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    316 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    144 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    315 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    316 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    200 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    315 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    316 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    212 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    213 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    253 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    284 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    237 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    243 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    144 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    309 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    35 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    304 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    305 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    77 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    230 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    67 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    88 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    75 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    90 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    141 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    203 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    93 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    295 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    243 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    100 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    102 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    96 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    286 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    120 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    124 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    126 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    309 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    102 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    77 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    90 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    243 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    159 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    93 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    161 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    243 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    212 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    213 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    309 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    160 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    309 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    78 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    267 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    80 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    309 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    281 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    27 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    42 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    64 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    74 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    162 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    267 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    160 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    137 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    295 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    144 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    67 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    141 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    86 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    165 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    95 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    45 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    48 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    50 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    51 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    52 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    53 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    55 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    56 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    57 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    58 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    27 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    8 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    9 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    210 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    211 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    147 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    42 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    295 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    294 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    64 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    145 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    103 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    104 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    105 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    109 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    189 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    190 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    191 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    197 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    199 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    78 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    80 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    144 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    210 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    211 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    15 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    47 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    302 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    303 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    150 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    61 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    49 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    301 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    86 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    153 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    62 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    150 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    86 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    130 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    8 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    153 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    98 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    84 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    85 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    86 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    91 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    322 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    290 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    244 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    193 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    300 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    216 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    247 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    296 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    296 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    222 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    61 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    231 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    247 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    10 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    14 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    300 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    300 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    247 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    30 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    38 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    32 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    152 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    149 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    146 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    40 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    43 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    18 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    283 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    18 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    150 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    275 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    159 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    63 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    286 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    71 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    275 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    275 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    297 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    297 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    79 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    293 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    65 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    229 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    226 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    226 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    99 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    166 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    229 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    83 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    92 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    308 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    285 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    268 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    222 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    285 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    268 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    316 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    280 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    159 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    148 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    141 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    282 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    147 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    36 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    250 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    196 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    249 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    196 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    163 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    6 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    194 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    194 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    194 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    224 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    224 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    216 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    72 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    216 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    3 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    15 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    164 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    152 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    145 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    62 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    37 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    152 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    303 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    54 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    131 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    108 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    162 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    69 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    69 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    76 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    10 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    14 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    41 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    116 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    20 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    141 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    19 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    10 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    14 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    132 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    30 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    32 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    126 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    154 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    156 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    158 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    40 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    30 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    68 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    32 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    68 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    28 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    97 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    40 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    68 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    150 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    37 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    98 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    71 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    89 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    117 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    94 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    144 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    79 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    76 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    87 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    71 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    44 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    76 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    79 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    24 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    82 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    34 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    99 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    143 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    99 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    136 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    22 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    15 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    25 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    23 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    142 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    134 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    140 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    163 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    163 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
];

unsafe extern "C" fn tokenExpr(
    mut pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut op: ::core::ffi::c_int,
    mut t: crate::src::headers::sqliteInt_h::Token,
) -> *mut crate::src::headers::sqliteInt_h::Expr {
    let mut p: *mut crate::src::headers::sqliteInt_h::Expr =
        crate::src::src::malloc::sqlite3DbMallocRawNN(
            (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            (::core::mem::size_of::<crate::src::headers::sqliteInt_h::Expr>() as usize)
                .wrapping_add(t.n as usize)
                .wrapping_add(1 as usize) as crate::src::ext::rtree::rtree::u64_0,
        ) as *mut crate::src::headers::sqliteInt_h::Expr;
    if !p.is_null() {
        let __p_ref = unsafe { &mut *p };
        __p_ref.op = op as crate::src::ext::rtree::rtree::u8_0;
        __p_ref.affExpr = 0 as ::core::ffi::c_char;
        __p_ref.flags =
            crate::src::headers::sqliteInt_h::EP_Leaf as crate::src::ext::rtree::rtree::u32_0;
        __p_ref.pRight = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
        __p_ref.pLeft = __p_ref.pRight;
        __p_ref.pAggInfo = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::AggInfo>();
        ::libc::memset(
            &raw mut __p_ref.x as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<crate::src::headers::sqliteInt_h::__anon_union_6>()
                as crate::__stddef_size_t_h::size_t,
        );
        ::libc::memset(
            &raw mut __p_ref.y as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<crate::src::headers::sqliteInt_h::__anon_union_8>()
                as crate::__stddef_size_t_h::size_t,
        );
        __p_ref.op2 = 0 as crate::src::ext::rtree::rtree::u8_0;
        __p_ref.iTable = 0 as ::core::ffi::c_int;
        __p_ref.iColumn = 0 as crate::src::headers::sqliteInt_h::ynVar;
        __p_ref.u.zToken = p.offset(1 as isize) as *mut crate::src::headers::sqliteInt_h::Expr
            as *mut ::core::ffi::c_char;
        ::core::ptr::copy_nonoverlapping(
            t.z as *const u8,
            __p_ref.u.zToken as *mut u8,
            t.n as usize,
        );
        *__p_ref.u.zToken.offset(t.n as isize) = 0 as ::core::ffi::c_char;
        __p_ref.w.iOfst =
            t.z.offset_from((*pParse).zTail) as ::core::ffi::c_long as ::core::ffi::c_int;
        if *(&raw const crate::src::src::global::sqlite3CtypeMap as *const ::core::ffi::c_uchar)
            .offset(*__p_ref.u.zToken.offset(0 as isize) as ::core::ffi::c_uchar as isize)
            as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            != 0
        {
            crate::src::src::util::sqlite3DequoteExpr(
                p as *mut crate::src::headers::sqliteInt_h::Expr,
            );
        }
        __p_ref.nHeight = 1 as ::core::ffi::c_int;
        if (*pParse).eParseMode as ::core::ffi::c_int
            >= crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME
        {
            return crate::src::src::alter::sqlite3RenameTokenMap(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                p as *mut ::core::ffi::c_void,
                &raw mut t as *mut _ as *const crate::src::headers::sqliteInt_h::Token,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
        }
    }
    p
}

static mut yy_shift_ofst: [::core::ffi::c_ushort; 587] = [
    2029 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1801 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2043 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1380 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1380 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1496 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1569 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1642 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    740 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    216 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    503 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    503 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    348 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    736 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    736 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    736 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    736 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    40 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    340 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    445 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    489 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    593 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    637 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    741 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    785 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    889 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    909 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1023 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1043 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1157 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1197 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1177 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1301 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    554 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1802 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1910 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    702 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    138 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    183 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    99 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    169 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    549 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    542 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1017 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1017 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1001 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    350 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    464 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    464 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    464 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    586 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    854 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    854 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    854 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    465 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    694 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    694 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    694 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    694 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1096 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1096 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    825 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    549 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    847 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    904 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    488 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    947 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    947 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    495 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    495 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1139 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    967 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    967 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1173 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    617 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    765 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    765 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    697 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    444 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    708 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    660 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    745 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    510 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    663 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    864 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    839 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    839 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    839 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1155 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1247 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1353 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    610 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1063 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    494 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1319 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    775 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1375 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    667 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1341 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1341 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1435 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1487 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    667 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    667 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1487 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    667 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1435 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    777 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1011 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1423 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    584 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    584 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    584 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1471 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1471 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    880 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1530 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1190 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1095 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1731 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1731 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1794 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1794 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1683 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1685 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1815 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1690 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1694 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1799 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1690 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1694 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1825 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1825 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1825 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1825 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1837 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1707 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1685 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1685 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1707 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1815 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1799 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1707 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1799 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1707 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1837 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1709 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1807 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1837 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1854 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1837 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1837 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1854 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1772 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1772 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1772 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1829 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1868 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1868 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1854 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1772 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1774 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1772 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1829 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1772 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1772 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1736 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1880 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1792 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1792 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1854 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1816 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1816 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1824 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1824 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1690 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1694 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1909 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1668 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1779 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1690 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1791 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1798 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1707 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1923 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1941 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1941 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1959 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1959 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1959 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    69 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1032 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    79 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    357 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    400 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1525 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    835 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    332 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1540 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1437 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1539 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1536 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1548 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1583 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1620 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1633 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1670 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1671 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1674 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1567 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1553 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1682 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1506 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1675 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1358 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1607 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1589 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1678 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1681 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1624 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1687 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1688 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1283 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1561 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1693 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1696 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1623 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1521 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1979 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1983 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1966 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1826 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1974 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1975 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1969 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1970 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1855 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1842 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1865 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1972 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1972 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1976 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1856 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1978 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1857 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1985 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2002 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1861 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1874 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1972 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1875 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1945 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1971 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1972 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1858 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1955 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1958 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1960 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1961 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1885 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1902 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1986 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1879 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2021 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2018 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2004 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1908 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1871 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1967 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2010 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1968 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1962 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2003 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1903 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1931 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2025 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2030 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2033 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1926 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1936 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2041 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2006 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2048 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2049 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2053 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2050 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2009 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2019 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2054 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1984 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2052 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2060 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2016 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2051 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2062 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2055 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1937 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2064 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2067 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2068 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2069 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2072 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2070 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1998 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1956 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2074 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2076 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1987 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2071 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2079 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1963 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2077 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2075 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2078 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2081 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2082 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2014 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2034 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2026 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2073 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2042 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2012 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2085 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2097 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2099 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2100 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2098 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2101 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2093 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1988 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1989 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2106 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2077 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2107 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2110 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2111 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2113 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2112 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2114 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2115 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2118 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2125 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2119 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2121 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2122 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2123 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2126 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2127 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2128 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2008 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2005 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2013 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2015 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2129 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2133 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2135 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2150 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2151 as ::core::ffi::c_int as ::core::ffi::c_ushort,
];

static mut yy_reduce_ofst: [::core::ffi::c_short; 417] = [
    -(67 as ::core::ffi::c_int) as ::core::ffi::c_short,
    1252 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(64 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(178 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(181 as ::core::ffi::c_int) as ::core::ffi::c_short,
    160 as ::core::ffi::c_int as ::core::ffi::c_short,
    1071 as ::core::ffi::c_int as ::core::ffi::c_short,
    143 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(184 as ::core::ffi::c_int) as ::core::ffi::c_short,
    137 as ::core::ffi::c_int as ::core::ffi::c_short,
    218 as ::core::ffi::c_int as ::core::ffi::c_short,
    220 as ::core::ffi::c_int as ::core::ffi::c_short,
    222 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(174 as ::core::ffi::c_int) as ::core::ffi::c_short,
    229 as ::core::ffi::c_int as ::core::ffi::c_short,
    268 as ::core::ffi::c_int as ::core::ffi::c_short,
    272 as ::core::ffi::c_int as ::core::ffi::c_short,
    275 as ::core::ffi::c_int as ::core::ffi::c_short,
    324 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(208 as ::core::ffi::c_int) as ::core::ffi::c_short,
    242 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(277 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(39 as ::core::ffi::c_int) as ::core::ffi::c_short,
    81 as ::core::ffi::c_int as ::core::ffi::c_short,
    537 as ::core::ffi::c_int as ::core::ffi::c_short,
    792 as ::core::ffi::c_int as ::core::ffi::c_short,
    810 as ::core::ffi::c_int as ::core::ffi::c_short,
    812 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(189 as ::core::ffi::c_int) as ::core::ffi::c_short,
    814 as ::core::ffi::c_int as ::core::ffi::c_short,
    831 as ::core::ffi::c_int as ::core::ffi::c_short,
    163 as ::core::ffi::c_int as ::core::ffi::c_short,
    865 as ::core::ffi::c_int as ::core::ffi::c_short,
    944 as ::core::ffi::c_int as ::core::ffi::c_short,
    887 as ::core::ffi::c_int as ::core::ffi::c_short,
    840 as ::core::ffi::c_int as ::core::ffi::c_short,
    964 as ::core::ffi::c_int as ::core::ffi::c_short,
    1077 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(187 as ::core::ffi::c_int) as ::core::ffi::c_short,
    292 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(133 as ::core::ffi::c_int) as ::core::ffi::c_short,
    274 as ::core::ffi::c_int as ::core::ffi::c_short,
    673 as ::core::ffi::c_int as ::core::ffi::c_short,
    558 as ::core::ffi::c_int as ::core::ffi::c_short,
    682 as ::core::ffi::c_int as ::core::ffi::c_short,
    795 as ::core::ffi::c_int as ::core::ffi::c_short,
    809 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(238 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(232 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(238 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(232 as ::core::ffi::c_int) as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    557 as ::core::ffi::c_int as ::core::ffi::c_short,
    712 as ::core::ffi::c_int as ::core::ffi::c_short,
    949 as ::core::ffi::c_int as ::core::ffi::c_short,
    966 as ::core::ffi::c_int as ::core::ffi::c_short,
    969 as ::core::ffi::c_int as ::core::ffi::c_short,
    971 as ::core::ffi::c_int as ::core::ffi::c_short,
    979 as ::core::ffi::c_int as ::core::ffi::c_short,
    1097 as ::core::ffi::c_int as ::core::ffi::c_short,
    1099 as ::core::ffi::c_int as ::core::ffi::c_short,
    1103 as ::core::ffi::c_int as ::core::ffi::c_short,
    1142 as ::core::ffi::c_int as ::core::ffi::c_short,
    1144 as ::core::ffi::c_int as ::core::ffi::c_short,
    1169 as ::core::ffi::c_int as ::core::ffi::c_short,
    1172 as ::core::ffi::c_int as ::core::ffi::c_short,
    1201 as ::core::ffi::c_int as ::core::ffi::c_short,
    1203 as ::core::ffi::c_int as ::core::ffi::c_short,
    1228 as ::core::ffi::c_int as ::core::ffi::c_short,
    1241 as ::core::ffi::c_int as ::core::ffi::c_short,
    1250 as ::core::ffi::c_int as ::core::ffi::c_short,
    1253 as ::core::ffi::c_int as ::core::ffi::c_short,
    1255 as ::core::ffi::c_int as ::core::ffi::c_short,
    1261 as ::core::ffi::c_int as ::core::ffi::c_short,
    1266 as ::core::ffi::c_int as ::core::ffi::c_short,
    1271 as ::core::ffi::c_int as ::core::ffi::c_short,
    1282 as ::core::ffi::c_int as ::core::ffi::c_short,
    1291 as ::core::ffi::c_int as ::core::ffi::c_short,
    1308 as ::core::ffi::c_int as ::core::ffi::c_short,
    1310 as ::core::ffi::c_int as ::core::ffi::c_short,
    1312 as ::core::ffi::c_int as ::core::ffi::c_short,
    1322 as ::core::ffi::c_int as ::core::ffi::c_short,
    1328 as ::core::ffi::c_int as ::core::ffi::c_short,
    1347 as ::core::ffi::c_int as ::core::ffi::c_short,
    1354 as ::core::ffi::c_int as ::core::ffi::c_short,
    1356 as ::core::ffi::c_int as ::core::ffi::c_short,
    1359 as ::core::ffi::c_int as ::core::ffi::c_short,
    1362 as ::core::ffi::c_int as ::core::ffi::c_short,
    1365 as ::core::ffi::c_int as ::core::ffi::c_short,
    1367 as ::core::ffi::c_int as ::core::ffi::c_short,
    1374 as ::core::ffi::c_int as ::core::ffi::c_short,
    1376 as ::core::ffi::c_int as ::core::ffi::c_short,
    1381 as ::core::ffi::c_int as ::core::ffi::c_short,
    1401 as ::core::ffi::c_int as ::core::ffi::c_short,
    1403 as ::core::ffi::c_int as ::core::ffi::c_short,
    1406 as ::core::ffi::c_int as ::core::ffi::c_short,
    1412 as ::core::ffi::c_int as ::core::ffi::c_short,
    1414 as ::core::ffi::c_int as ::core::ffi::c_short,
    1417 as ::core::ffi::c_int as ::core::ffi::c_short,
    1421 as ::core::ffi::c_int as ::core::ffi::c_short,
    1428 as ::core::ffi::c_int as ::core::ffi::c_short,
    1447 as ::core::ffi::c_int as ::core::ffi::c_short,
    1449 as ::core::ffi::c_int as ::core::ffi::c_short,
    1453 as ::core::ffi::c_int as ::core::ffi::c_short,
    1462 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(22 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(159 as ::core::ffi::c_int) as ::core::ffi::c_short,
    475 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(220 as ::core::ffi::c_int) as ::core::ffi::c_short,
    756 as ::core::ffi::c_int as ::core::ffi::c_short,
    38 as ::core::ffi::c_int as ::core::ffi::c_short,
    501 as ::core::ffi::c_int as ::core::ffi::c_short,
    841 as ::core::ffi::c_int as ::core::ffi::c_short,
    714 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    118 as ::core::ffi::c_int as ::core::ffi::c_short,
    337 as ::core::ffi::c_int as ::core::ffi::c_short,
    349 as ::core::ffi::c_int as ::core::ffi::c_short,
    363 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(56 as ::core::ffi::c_int) as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    329 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(205 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(205 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(205 as ::core::ffi::c_int) as ::core::ffi::c_short,
    687 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(172 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(130 as ::core::ffi::c_int) as ::core::ffi::c_short,
    -(57 as ::core::ffi::c_int) as ::core::ffi::c_short,
    790 as ::core::ffi::c_int as ::core::ffi::c_short,
    397 as ::core::ffi::c_int as ::core::ffi::c_short,
    528 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(271 as ::core::ffi::c_int) as ::core::ffi::c_short,
    136 as ::core::ffi::c_int as ::core::ffi::c_short,
    596 as ::core::ffi::c_int as ::core::ffi::c_short,
    596 as ::core::ffi::c_int as ::core::ffi::c_short,
    90 as ::core::ffi::c_int as ::core::ffi::c_short,
    316 as ::core::ffi::c_int as ::core::ffi::c_short,
    522 as ::core::ffi::c_int as ::core::ffi::c_short,
    541 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(37 as ::core::ffi::c_int) as ::core::ffi::c_short,
    715 as ::core::ffi::c_int as ::core::ffi::c_short,
    849 as ::core::ffi::c_int as ::core::ffi::c_short,
    977 as ::core::ffi::c_int as ::core::ffi::c_short,
    628 as ::core::ffi::c_int as ::core::ffi::c_short,
    856 as ::core::ffi::c_int as ::core::ffi::c_short,
    980 as ::core::ffi::c_int as ::core::ffi::c_short,
    991 as ::core::ffi::c_int as ::core::ffi::c_short,
    1081 as ::core::ffi::c_int as ::core::ffi::c_short,
    1102 as ::core::ffi::c_int as ::core::ffi::c_short,
    1135 as ::core::ffi::c_int as ::core::ffi::c_short,
    1083 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(162 as ::core::ffi::c_int) as ::core::ffi::c_short,
    208 as ::core::ffi::c_int as ::core::ffi::c_short,
    1258 as ::core::ffi::c_int as ::core::ffi::c_short,
    794 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(86 as ::core::ffi::c_int) as ::core::ffi::c_short,
    159 as ::core::ffi::c_int as ::core::ffi::c_short,
    41 as ::core::ffi::c_int as ::core::ffi::c_short,
    1109 as ::core::ffi::c_int as ::core::ffi::c_short,
    671 as ::core::ffi::c_int as ::core::ffi::c_short,
    852 as ::core::ffi::c_int as ::core::ffi::c_short,
    844 as ::core::ffi::c_int as ::core::ffi::c_short,
    932 as ::core::ffi::c_int as ::core::ffi::c_short,
    1175 as ::core::ffi::c_int as ::core::ffi::c_short,
    1254 as ::core::ffi::c_int as ::core::ffi::c_short,
    480 as ::core::ffi::c_int as ::core::ffi::c_short,
    1180 as ::core::ffi::c_int as ::core::ffi::c_short,
    100 as ::core::ffi::c_int as ::core::ffi::c_short,
    258 as ::core::ffi::c_int as ::core::ffi::c_short,
    1265 as ::core::ffi::c_int as ::core::ffi::c_short,
    1268 as ::core::ffi::c_int as ::core::ffi::c_short,
    1216 as ::core::ffi::c_int as ::core::ffi::c_short,
    1287 as ::core::ffi::c_int as ::core::ffi::c_short,
    -(139 as ::core::ffi::c_int) as ::core::ffi::c_short,
    317 as ::core::ffi::c_int as ::core::ffi::c_short,
    344 as ::core::ffi::c_int as ::core::ffi::c_short,
    63 as ::core::ffi::c_int as ::core::ffi::c_short,
    339 as ::core::ffi::c_int as ::core::ffi::c_short,
    423 as ::core::ffi::c_int as ::core::ffi::c_short,
    563 as ::core::ffi::c_int as ::core::ffi::c_short,
    636 as ::core::ffi::c_int as ::core::ffi::c_short,
    676 as ::core::ffi::c_int as ::core::ffi::c_short,
    813 as ::core::ffi::c_int as ::core::ffi::c_short,
    908 as ::core::ffi::c_int as ::core::ffi::c_short,
    914 as ::core::ffi::c_int as ::core::ffi::c_short,
    950 as ::core::ffi::c_int as ::core::ffi::c_short,
    1078 as ::core::ffi::c_int as ::core::ffi::c_short,
    1084 as ::core::ffi::c_int as ::core::ffi::c_short,
    1098 as ::core::ffi::c_int as ::core::ffi::c_short,
    1363 as ::core::ffi::c_int as ::core::ffi::c_short,
    1384 as ::core::ffi::c_int as ::core::ffi::c_short,
    1407 as ::core::ffi::c_int as ::core::ffi::c_short,
    1439 as ::core::ffi::c_int as ::core::ffi::c_short,
    1464 as ::core::ffi::c_int as ::core::ffi::c_short,
    411 as ::core::ffi::c_int as ::core::ffi::c_short,
    1527 as ::core::ffi::c_int as ::core::ffi::c_short,
    1534 as ::core::ffi::c_int as ::core::ffi::c_short,
    1535 as ::core::ffi::c_int as ::core::ffi::c_short,
    1537 as ::core::ffi::c_int as ::core::ffi::c_short,
    1541 as ::core::ffi::c_int as ::core::ffi::c_short,
    1542 as ::core::ffi::c_int as ::core::ffi::c_short,
    1543 as ::core::ffi::c_int as ::core::ffi::c_short,
    1544 as ::core::ffi::c_int as ::core::ffi::c_short,
    1545 as ::core::ffi::c_int as ::core::ffi::c_short,
    1547 as ::core::ffi::c_int as ::core::ffi::c_short,
    1549 as ::core::ffi::c_int as ::core::ffi::c_short,
    1550 as ::core::ffi::c_int as ::core::ffi::c_short,
    990 as ::core::ffi::c_int as ::core::ffi::c_short,
    1164 as ::core::ffi::c_int as ::core::ffi::c_short,
    1492 as ::core::ffi::c_int as ::core::ffi::c_short,
    1551 as ::core::ffi::c_int as ::core::ffi::c_short,
    1552 as ::core::ffi::c_int as ::core::ffi::c_short,
    1556 as ::core::ffi::c_int as ::core::ffi::c_short,
    1217 as ::core::ffi::c_int as ::core::ffi::c_short,
    1558 as ::core::ffi::c_int as ::core::ffi::c_short,
    1559 as ::core::ffi::c_int as ::core::ffi::c_short,
    1560 as ::core::ffi::c_int as ::core::ffi::c_short,
    1473 as ::core::ffi::c_int as ::core::ffi::c_short,
    1413 as ::core::ffi::c_int as ::core::ffi::c_short,
    1563 as ::core::ffi::c_int as ::core::ffi::c_short,
    1510 as ::core::ffi::c_int as ::core::ffi::c_short,
    1568 as ::core::ffi::c_int as ::core::ffi::c_short,
    563 as ::core::ffi::c_int as ::core::ffi::c_short,
    1570 as ::core::ffi::c_int as ::core::ffi::c_short,
    1571 as ::core::ffi::c_int as ::core::ffi::c_short,
    1572 as ::core::ffi::c_int as ::core::ffi::c_short,
    1573 as ::core::ffi::c_int as ::core::ffi::c_short,
    1574 as ::core::ffi::c_int as ::core::ffi::c_short,
    1575 as ::core::ffi::c_int as ::core::ffi::c_short,
    1443 as ::core::ffi::c_int as ::core::ffi::c_short,
    1466 as ::core::ffi::c_int as ::core::ffi::c_short,
    1518 as ::core::ffi::c_int as ::core::ffi::c_short,
    1513 as ::core::ffi::c_int as ::core::ffi::c_short,
    1514 as ::core::ffi::c_int as ::core::ffi::c_short,
    1515 as ::core::ffi::c_int as ::core::ffi::c_short,
    1516 as ::core::ffi::c_int as ::core::ffi::c_short,
    1217 as ::core::ffi::c_int as ::core::ffi::c_short,
    1518 as ::core::ffi::c_int as ::core::ffi::c_short,
    1518 as ::core::ffi::c_int as ::core::ffi::c_short,
    1531 as ::core::ffi::c_int as ::core::ffi::c_short,
    1562 as ::core::ffi::c_int as ::core::ffi::c_short,
    1582 as ::core::ffi::c_int as ::core::ffi::c_short,
    1477 as ::core::ffi::c_int as ::core::ffi::c_short,
    1505 as ::core::ffi::c_int as ::core::ffi::c_short,
    1511 as ::core::ffi::c_int as ::core::ffi::c_short,
    1533 as ::core::ffi::c_int as ::core::ffi::c_short,
    1512 as ::core::ffi::c_int as ::core::ffi::c_short,
    1488 as ::core::ffi::c_int as ::core::ffi::c_short,
    1538 as ::core::ffi::c_int as ::core::ffi::c_short,
    1509 as ::core::ffi::c_int as ::core::ffi::c_short,
    1517 as ::core::ffi::c_int as ::core::ffi::c_short,
    1546 as ::core::ffi::c_int as ::core::ffi::c_short,
    1519 as ::core::ffi::c_int as ::core::ffi::c_short,
    1557 as ::core::ffi::c_int as ::core::ffi::c_short,
    1489 as ::core::ffi::c_int as ::core::ffi::c_short,
    1565 as ::core::ffi::c_int as ::core::ffi::c_short,
    1564 as ::core::ffi::c_int as ::core::ffi::c_short,
    1578 as ::core::ffi::c_int as ::core::ffi::c_short,
    1586 as ::core::ffi::c_int as ::core::ffi::c_short,
    1587 as ::core::ffi::c_int as ::core::ffi::c_short,
    1588 as ::core::ffi::c_int as ::core::ffi::c_short,
    1526 as ::core::ffi::c_int as ::core::ffi::c_short,
    1528 as ::core::ffi::c_int as ::core::ffi::c_short,
    1554 as ::core::ffi::c_int as ::core::ffi::c_short,
    1555 as ::core::ffi::c_int as ::core::ffi::c_short,
    1576 as ::core::ffi::c_int as ::core::ffi::c_short,
    1577 as ::core::ffi::c_int as ::core::ffi::c_short,
    1566 as ::core::ffi::c_int as ::core::ffi::c_short,
    1579 as ::core::ffi::c_int as ::core::ffi::c_short,
    1584 as ::core::ffi::c_int as ::core::ffi::c_short,
    1591 as ::core::ffi::c_int as ::core::ffi::c_short,
    1520 as ::core::ffi::c_int as ::core::ffi::c_short,
    1523 as ::core::ffi::c_int as ::core::ffi::c_short,
    1617 as ::core::ffi::c_int as ::core::ffi::c_short,
    1628 as ::core::ffi::c_int as ::core::ffi::c_short,
    1580 as ::core::ffi::c_int as ::core::ffi::c_short,
    1581 as ::core::ffi::c_int as ::core::ffi::c_short,
    1632 as ::core::ffi::c_int as ::core::ffi::c_short,
    1585 as ::core::ffi::c_int as ::core::ffi::c_short,
    1590 as ::core::ffi::c_int as ::core::ffi::c_short,
    1593 as ::core::ffi::c_int as ::core::ffi::c_short,
    1592 as ::core::ffi::c_int as ::core::ffi::c_short,
    1594 as ::core::ffi::c_int as ::core::ffi::c_short,
    1610 as ::core::ffi::c_int as ::core::ffi::c_short,
    1595 as ::core::ffi::c_int as ::core::ffi::c_short,
    1597 as ::core::ffi::c_int as ::core::ffi::c_short,
    1611 as ::core::ffi::c_int as ::core::ffi::c_short,
    1612 as ::core::ffi::c_int as ::core::ffi::c_short,
    1613 as ::core::ffi::c_int as ::core::ffi::c_short,
    1614 as ::core::ffi::c_int as ::core::ffi::c_short,
    1652 as ::core::ffi::c_int as ::core::ffi::c_short,
    1655 as ::core::ffi::c_int as ::core::ffi::c_short,
    1615 as ::core::ffi::c_int as ::core::ffi::c_short,
    1598 as ::core::ffi::c_int as ::core::ffi::c_short,
    1600 as ::core::ffi::c_int as ::core::ffi::c_short,
    1616 as ::core::ffi::c_int as ::core::ffi::c_short,
    1596 as ::core::ffi::c_int as ::core::ffi::c_short,
    1622 as ::core::ffi::c_int as ::core::ffi::c_short,
    1619 as ::core::ffi::c_int as ::core::ffi::c_short,
    1625 as ::core::ffi::c_int as ::core::ffi::c_short,
    1631 as ::core::ffi::c_int as ::core::ffi::c_short,
    1657 as ::core::ffi::c_int as ::core::ffi::c_short,
    1659 as ::core::ffi::c_int as ::core::ffi::c_short,
    1599 as ::core::ffi::c_int as ::core::ffi::c_short,
    1601 as ::core::ffi::c_int as ::core::ffi::c_short,
    1679 as ::core::ffi::c_int as ::core::ffi::c_short,
    1684 as ::core::ffi::c_int as ::core::ffi::c_short,
    1661 as ::core::ffi::c_int as ::core::ffi::c_short,
    1680 as ::core::ffi::c_int as ::core::ffi::c_short,
    1686 as ::core::ffi::c_int as ::core::ffi::c_short,
    1689 as ::core::ffi::c_int as ::core::ffi::c_short,
    1695 as ::core::ffi::c_int as ::core::ffi::c_short,
    1663 as ::core::ffi::c_int as ::core::ffi::c_short,
    1669 as ::core::ffi::c_int as ::core::ffi::c_short,
    1677 as ::core::ffi::c_int as ::core::ffi::c_short,
    1691 as ::core::ffi::c_int as ::core::ffi::c_short,
    1666 as ::core::ffi::c_int as ::core::ffi::c_short,
    1672 as ::core::ffi::c_int as ::core::ffi::c_short,
    1673 as ::core::ffi::c_int as ::core::ffi::c_short,
    1692 as ::core::ffi::c_int as ::core::ffi::c_short,
    1698 as ::core::ffi::c_int as ::core::ffi::c_short,
    1700 as ::core::ffi::c_int as ::core::ffi::c_short,
    1703 as ::core::ffi::c_int as ::core::ffi::c_short,
    1676 as ::core::ffi::c_int as ::core::ffi::c_short,
    1705 as ::core::ffi::c_int as ::core::ffi::c_short,
    1706 as ::core::ffi::c_int as ::core::ffi::c_short,
    1618 as ::core::ffi::c_int as ::core::ffi::c_short,
    1604 as ::core::ffi::c_int as ::core::ffi::c_short,
    1629 as ::core::ffi::c_int as ::core::ffi::c_short,
    1643 as ::core::ffi::c_int as ::core::ffi::c_short,
    1704 as ::core::ffi::c_int as ::core::ffi::c_short,
    1711 as ::core::ffi::c_int as ::core::ffi::c_short,
    1621 as ::core::ffi::c_int as ::core::ffi::c_short,
    1626 as ::core::ffi::c_int as ::core::ffi::c_short,
    1648 as ::core::ffi::c_int as ::core::ffi::c_short,
    1665 as ::core::ffi::c_int as ::core::ffi::c_short,
    1697 as ::core::ffi::c_int as ::core::ffi::c_short,
    1699 as ::core::ffi::c_int as ::core::ffi::c_short,
    1656 as ::core::ffi::c_int as ::core::ffi::c_short,
    1735 as ::core::ffi::c_int as ::core::ffi::c_short,
    1662 as ::core::ffi::c_int as ::core::ffi::c_short,
    1701 as ::core::ffi::c_int as ::core::ffi::c_short,
    1702 as ::core::ffi::c_int as ::core::ffi::c_short,
    1712 as ::core::ffi::c_int as ::core::ffi::c_short,
    1714 as ::core::ffi::c_int as ::core::ffi::c_short,
    1747 as ::core::ffi::c_int as ::core::ffi::c_short,
    1759 as ::core::ffi::c_int as ::core::ffi::c_short,
    1767 as ::core::ffi::c_int as ::core::ffi::c_short,
    1773 as ::core::ffi::c_int as ::core::ffi::c_short,
    1775 as ::core::ffi::c_int as ::core::ffi::c_short,
    1777 as ::core::ffi::c_int as ::core::ffi::c_short,
    1660 as ::core::ffi::c_int as ::core::ffi::c_short,
    1667 as ::core::ffi::c_int as ::core::ffi::c_short,
    1710 as ::core::ffi::c_int as ::core::ffi::c_short,
    1763 as ::core::ffi::c_int as ::core::ffi::c_short,
    1753 as ::core::ffi::c_int as ::core::ffi::c_short,
    1760 as ::core::ffi::c_int as ::core::ffi::c_short,
    1761 as ::core::ffi::c_int as ::core::ffi::c_short,
    1762 as ::core::ffi::c_int as ::core::ffi::c_short,
    1765 as ::core::ffi::c_int as ::core::ffi::c_short,
    1754 as ::core::ffi::c_int as ::core::ffi::c_short,
    1755 as ::core::ffi::c_int as ::core::ffi::c_short,
    1764 as ::core::ffi::c_int as ::core::ffi::c_short,
    1768 as ::core::ffi::c_int as ::core::ffi::c_short,
    1766 as ::core::ffi::c_int as ::core::ffi::c_short,
    1778 as ::core::ffi::c_int as ::core::ffi::c_short,
];

static mut yy_default: [::core::ffi::c_ushort; 587] = [
    1667 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1667 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1667 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1495 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1371 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1495 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1495 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1495 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1401 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1401 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1548 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1494 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1582 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1582 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1567 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1566 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1410 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1417 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1496 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1497 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1547 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1549 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1512 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1424 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1423 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1422 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1421 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1530 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1389 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1415 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1408 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1412 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1491 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1492 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1490 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1645 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1497 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1496 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1411 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1459 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1475 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1458 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1467 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1474 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1473 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1472 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1481 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1471 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1468 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1461 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1460 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1462 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1463 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1282 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1333 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1464 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1452 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1451 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1450 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1478 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1465 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1477 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1476 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1555 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1619 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1618 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1513 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1582 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1391 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1582 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1582 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1582 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1582 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1392 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1392 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1395 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1362 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1362 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1362 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1362 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1371 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1362 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1552 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1550 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1367 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1612 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1525 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1347 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1367 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1367 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1367 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1367 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1369 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1348 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1346 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1361 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1659 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1427 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1416 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1368 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1416 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1656 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1414 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1427 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1427 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1414 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1427 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1368 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1656 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1308 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1634 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1303 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1401 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1401 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1401 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1391 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1391 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1391 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1391 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1395 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1395 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1493 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1368 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1361 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1659 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1659 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1658 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1658 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1513 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1642 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1436 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1409 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1395 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1336 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1409 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1395 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1342 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1342 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1342 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1342 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1414 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1642 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1642 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1414 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1436 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1336 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1414 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1336 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1414 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1529 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1653 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1503 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1503 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1334 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1334 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1334 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1503 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1334 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1308 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1334 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1323 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1334 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1334 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1600 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1507 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1507 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1503 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1592 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1592 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1404 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1404 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1409 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1395 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1498 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1377 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1409 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1407 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1405 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1414 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1326 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1615 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1615 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1611 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1611 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1611 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1664 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1664 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1562 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1627 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1627 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1310 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1310 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1627 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1622 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1557 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1514 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1381 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1568 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1441 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1559 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1418 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1419 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1382 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1433 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1428 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1655 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1528 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1527 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1379 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1406 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1597 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1396 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1646 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1356 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1638 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1350 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1442 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1445 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1280 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
];

static mut yyFallback: [::core::ffi::c_ushort; 187] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    60 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
];

unsafe extern "C" fn binaryToUnaryIfNull(
    mut pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut pY: *mut crate::src::headers::sqliteInt_h::Expr,
    mut pA: *mut crate::src::headers::sqliteInt_h::Expr,
    mut op: ::core::ffi::c_int,
) {
    let mut db: *mut crate::src::headers::sqliteInt_h::sqlite3 = (*pParse).db;
    if !pA.is_null()
        && !pY.is_null()
        && (*pY).op as ::core::ffi::c_int == crate::src::parse::TK_NULL
        && !((*pParse).eParseMode as ::core::ffi::c_int
            >= crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME)
    {
        let __pA_ref = unsafe { &mut *pA };
        __pA_ref.op = op as crate::src::ext::rtree::rtree::u8_0;
        crate::src::src::expr::sqlite3ExprDelete(
            db as *mut crate::src::headers::sqliteInt_h::sqlite3,
            __pA_ref.pRight as *mut crate::src::headers::sqliteInt_h::Expr,
        );
        __pA_ref.pRight = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
    }
}

unsafe extern "C" fn parserAddExprIdListTerm(
    mut pParse: *mut crate::src::headers::sqliteInt_h::Parse,
    mut pPrior: *mut crate::src::headers::sqliteInt_h::ExprList,
    mut pIdToken: *mut crate::src::headers::sqliteInt_h::Token,
    mut hasCollate: ::core::ffi::c_int,
    mut sortOrder: ::core::ffi::c_int,
) -> *mut crate::src::headers::sqliteInt_h::ExprList {
    let mut p: *mut crate::src::headers::sqliteInt_h::ExprList =
        crate::src::src::expr::sqlite3ExprListAppend(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            pPrior as *mut crate::src::headers::sqliteInt_h::ExprList,
            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                as *mut crate::src::headers::sqliteInt_h::Expr,
        ) as *mut crate::src::headers::sqliteInt_h::ExprList;
    if (hasCollate != 0 || sortOrder != crate::src::headers::sqliteInt_h::SQLITE_SO_UNDEFINED)
        && (*(*pParse).db).init.busy as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"syntax error after column name \"%.*s\"\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::src::src::printf::PrintfArg::Int(
                    (*pIdToken).n as crate::src::ext::rtree::rtree::i64_0,
                ),
                crate::src::src::printf::PrintfArg::Str((*pIdToken).z as *mut ::core::ffi::c_char),
            ],
        );
    }
    crate::src::src::expr::sqlite3ExprListSetName(
        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
        p as *mut crate::src::headers::sqliteInt_h::ExprList,
        pIdToken as *const crate::src::headers::sqliteInt_h::Token,
        1 as ::core::ffi::c_int,
    );
    p
}

unsafe extern "C" fn yyGrowStack(mut p: *mut yyParser) -> ::core::ffi::c_int {
    let __p_ref = unsafe { &mut *p };
    let mut oldSize: ::core::ffi::c_int = 1 as ::core::ffi::c_int
        + __p_ref.yystackEnd.offset_from(__p_ref.yystack) as ::core::ffi::c_long
            as ::core::ffi::c_int;
    let mut newSize: ::core::ffi::c_int = 0;
    let mut idx: ::core::ffi::c_int = 0;
    let mut pNew: *mut yyStackEntry = ::core::ptr::null_mut::<yyStackEntry>();
    newSize = oldSize * 2 as ::core::ffi::c_int + 100 as ::core::ffi::c_int;
    idx = __p_ref.yytos.offset_from(__p_ref.yystack) as ::core::ffi::c_long as ::core::ffi::c_int;
    if __p_ref.yystack == &raw mut __p_ref.yystk0 as *mut yyStackEntry {
        pNew = parserStackRealloc(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            (newSize as usize).wrapping_mul(::core::mem::size_of::<yyStackEntry>() as usize)
                as crate::src::headers::sqlite3_h::sqlite3_uint64,
        ) as *mut yyStackEntry;
        if pNew.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        ::core::ptr::copy_nonoverlapping(
            __p_ref.yystack as *const u8,
            pNew as *mut u8,
            ((oldSize as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(
                    ::core::mem::size_of::<yyStackEntry>() as crate::__stddef_size_t_h::size_t
                )) as usize,
        );
    } else {
        pNew = parserStackRealloc(
            __p_ref.yystack as *mut ::core::ffi::c_void,
            (newSize as usize).wrapping_mul(::core::mem::size_of::<yyStackEntry>() as usize)
                as crate::src::headers::sqlite3_h::sqlite3_uint64,
        ) as *mut yyStackEntry;
        if pNew.is_null() {
            return 1 as ::core::ffi::c_int;
        }
    }
    __p_ref.yystack = pNew;
    __p_ref.yytos = __p_ref.yystack.offset(idx as isize) as *mut yyStackEntry;
    __p_ref.yystackEnd = __p_ref
        .yystack
        .offset((newSize - 1 as ::core::ffi::c_int) as isize)
        as *mut yyStackEntry;
    0 as ::core::ffi::c_int
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3ParserInit(
    mut yypRawParser: *mut ::core::ffi::c_void,
    mut pParse: *mut crate::src::headers::sqliteInt_h::Parse,
) {
    let mut yypParser: *mut yyParser = yypRawParser as *mut yyParser;
    let __yypParser_ref = unsafe { &mut *yypParser };
    __yypParser_ref.pParse = pParse;
    __yypParser_ref.yystack = &raw mut __yypParser_ref.yystk0 as *mut yyStackEntry;
    __yypParser_ref.yystackEnd = (*yypParser)
        .yystack
        .offset((YYSTACKDEPTH - 1 as ::core::ffi::c_int) as isize)
        as *mut yyStackEntry;
    __yypParser_ref.yytos = __yypParser_ref.yystack;
    (*(*yypParser).yystack.offset(0 as isize)).stateno = 0 as ::core::ffi::c_ushort;
    (*(*yypParser).yystack.offset(0 as isize)).major = 0 as ::core::ffi::c_ushort;
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3ParserAlloc(
    mut mallocProc: Option<
        unsafe extern "C" fn(crate::src::ext::rtree::rtree::u64_0) -> *mut ::core::ffi::c_void,
    >,
    mut pParse: *mut crate::src::headers::sqliteInt_h::Parse,
) -> *mut ::core::ffi::c_void {
    let mut yypParser: *mut yyParser = ::core::ptr::null_mut::<yyParser>();
    yypParser = Some(mallocProc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        ::core::mem::size_of::<yyParser>() as crate::src::ext::rtree::rtree::u64_0
    ) as *mut yyParser;
    if !yypParser.is_null() {
        (*yypParser).pParse = pParse;
        sqlite3ParserInit(yypParser as *mut ::core::ffi::c_void, pParse);
    }
    yypParser as *mut ::core::ffi::c_void
}

unsafe extern "C" fn yy_destructor(
    mut yypParser: *mut yyParser,
    mut yymajor: ::core::ffi::c_ushort,
    mut yypminor: *mut YYMINORTYPE,
) {
    let mut pParse: *mut crate::src::headers::sqliteInt_h::Parse = (*yypParser).pParse;
    match yymajor as ::core::ffi::c_int {
        206 | 241 | 242 | 254 | 256 => {
            crate::src::src::select::sqlite3SelectDelete(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yypminor).yy637 as *mut crate::src::headers::sqliteInt_h::Select,
            );
        }
        218 | 219 | 248 | 250 | 270 | 281 | 283 | 286 | 293 | 298 | 315 => {
            crate::src::src::expr::sqlite3ExprDelete(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yypminor).yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
            );
        }
        223 | 233 | 234 | 246 | 249 | 251 | 255 | 257 | 264 | 271 | 280 | 282 | 314 => {
            crate::src::src::expr::sqlite3ExprListDelete(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yypminor).yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
            );
        }
        240 | 247 | 259 | 260 | 265 => {
            crate::src::src::build::sqlite3SrcListDelete(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yypminor).yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
            );
        }
        243 => {
            crate::src::src::build::sqlite3WithDelete(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yypminor).yy125 as *mut crate::src::headers::sqliteInt_h::With,
            );
        }
        253 | 310 => {
            crate::src::src::window::sqlite3WindowListDelete(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yypminor).yy483 as *mut crate::src::headers::sqliteInt_h::Window,
            );
        }
        266 | 273 => {
            crate::src::src::build::sqlite3IdListDelete(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yypminor).yy204 as *mut crate::src::headers::sqliteInt_h::IdList,
            );
        }
        276 | 311 | 312 | 313 | 316 => {
            crate::src::src::window::sqlite3WindowDelete(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yypminor).yy483 as *mut crate::src::headers::sqliteInt_h::Window,
            );
        }
        289 | 294 => {
            crate::src::src::trigger::sqlite3DeleteTriggerStep(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yypminor).yy319 as *mut crate::src::headers::sqliteInt_h::TriggerStep,
            );
        }
        291 => {
            crate::src::src::build::sqlite3IdListDelete(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yypminor).yy28.b as *mut crate::src::headers::sqliteInt_h::IdList,
            );
        }
        318 | 319 | 320 => {
            crate::src::src::expr::sqlite3ExprDelete(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yypminor).yy205.pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
            );
        }
        _ => {}
    };
}

unsafe extern "C" fn yy_pop_parser_stack(mut pParser: *mut yyParser) {
    let mut yytos: *mut yyStackEntry = ::core::ptr::null_mut::<yyStackEntry>();
    let __pParser_ref = unsafe { &mut *pParser };
    let fresh0 = __pParser_ref.yytos;
    __pParser_ref.yytos = __pParser_ref.yytos.offset(-1);
    yytos = fresh0;
    yy_destructor(pParser, (*yytos).major, &raw mut (*yytos).minor);
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3ParserFinalize(mut p: *mut ::core::ffi::c_void) {
    let mut pParser: *mut yyParser = p as *mut yyParser;
    let __pParser_ref = unsafe { &mut *pParser };
    let mut yytos: *mut yyStackEntry = __pParser_ref.yytos;
    while yytos > __pParser_ref.yystack {
        if (*yytos).major as ::core::ffi::c_int >= YY_MIN_DSTRCTR {
            yy_destructor(pParser, (*yytos).major, &raw mut (*yytos).minor);
        }
        yytos = yytos.offset(-1);
    }
    if __pParser_ref.yystack != &raw mut __pParser_ref.yystk0 as *mut yyStackEntry {
        crate::src::src::malloc::sqlite3_free(__pParser_ref.yystack as *mut ::core::ffi::c_void);
    }
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3ParserFree(
    mut p: *mut ::core::ffi::c_void,
    mut freeProc: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    sqlite3ParserFinalize(p);
    Some(freeProc.expect("non-null function pointer")).expect("non-null function pointer")(p);
}

unsafe extern "C" fn yy_find_shift_action(
    mut iLookAhead: ::core::ffi::c_ushort,
    mut stateno: ::core::ffi::c_ushort,
) -> ::core::ffi::c_ushort {
    let mut i: ::core::ffi::c_int = 0;
    if stateno as ::core::ffi::c_int > YY_MAX_SHIFT {
        return stateno;
    }
    loop {
        i = yy_shift_ofst[stateno as usize] as ::core::ffi::c_int;
        i += iLookAhead as ::core::ffi::c_int;
        if yy_lookahead[i as usize] as ::core::ffi::c_int != iLookAhead as ::core::ffi::c_int {
            let mut iFallback: ::core::ffi::c_ushort = 0;
            iFallback = yyFallback[iLookAhead as usize];
            if iFallback as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                iLookAhead = iFallback;
            } else {
                let mut j: ::core::ffi::c_int = i - iLookAhead as ::core::ffi::c_int + YYWILDCARD;
                if yy_lookahead[j as usize] as ::core::ffi::c_int == YYWILDCARD
                    && iLookAhead as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                {
                    return yy_action[j as usize];
                }
                return yy_default[stateno as usize];
            }
        } else {
            return yy_action[i as usize];
        }
    }
}

unsafe extern "C" fn yy_find_reduce_action(
    mut stateno: ::core::ffi::c_ushort,
    mut iLookAhead: ::core::ffi::c_ushort,
) -> ::core::ffi::c_ushort {
    let mut i: ::core::ffi::c_int = 0;
    i = yy_reduce_ofst[stateno as usize] as ::core::ffi::c_int;
    i += iLookAhead as ::core::ffi::c_int;
    yy_action[i as usize]
}

unsafe extern "C" fn yyStackOverflow(mut yypParser: *mut yyParser) {
    let __yypParser_ref = unsafe { &mut *yypParser };
    let mut pParse: *mut crate::src::headers::sqliteInt_h::Parse = __yypParser_ref.pParse;
    while __yypParser_ref.yytos > __yypParser_ref.yystack {
        yy_pop_parser_stack(yypParser);
    }
    crate::src::src::malloc::sqlite3OomFault(
        (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
    );
    __yypParser_ref.pParse = pParse;
}

unsafe extern "C" fn yy_shift(
    mut yypParser: *mut yyParser,
    mut yyNewState: ::core::ffi::c_ushort,
    mut yyMajor: ::core::ffi::c_ushort,
    mut yyMinor: crate::src::headers::sqliteInt_h::Token,
) {
    let mut yytos: *mut yyStackEntry = ::core::ptr::null_mut::<yyStackEntry>();
    let __yypParser_ref = unsafe { &mut *yypParser };
    __yypParser_ref.yytos = __yypParser_ref.yytos.offset(1);
    yytos = __yypParser_ref.yytos;
    if yytos > __yypParser_ref.yystackEnd {
        if yyGrowStack(yypParser) != 0 {
            __yypParser_ref.yytos = __yypParser_ref.yytos.offset(-1);
            yyStackOverflow(yypParser);
            return;
        }
        yytos = __yypParser_ref.yytos;
    }
    if yyNewState as ::core::ffi::c_int > YY_MAX_SHIFT {
        yyNewState = (yyNewState as ::core::ffi::c_int + (YY_MIN_REDUCE - YY_MIN_SHIFTREDUCE))
            as ::core::ffi::c_ushort;
    }
    (*yytos).stateno = yyNewState;
    (*yytos).major = yyMajor;
    (*yytos).minor.yy0 = yyMinor;
}

static mut yyRuleInfoLhs: [::core::ffi::c_ushort; 409] = [
    191 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    191 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    190 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    193 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    193 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    193 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    193 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    197 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    199 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    201 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    200 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    200 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    198 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    207 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    208 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    210 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    210 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    210 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    211 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    215 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    216 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    226 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    226 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    222 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    222 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    224 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    224 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    227 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    228 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    225 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    225 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    229 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    229 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    229 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    231 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    232 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    235 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    220 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    236 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    237 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    237 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    239 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    206 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    244 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    244 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    244 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    254 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    256 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    245 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    246 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    247 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    247 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    260 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    259 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    202 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    240 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    265 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    261 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    262 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    267 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    263 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    251 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    233 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    221 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    268 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    268 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    268 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    249 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    249 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    250 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    250 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    252 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    248 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    270 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    271 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    274 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    275 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    272 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    273 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    266 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    278 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    279 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    282 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    282 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    283 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    283 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    281 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    264 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    255 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    280 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    280 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    284 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    284 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    223 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    234 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    285 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    285 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    286 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    286 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    213 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    214 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    288 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    290 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    290 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    290 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    291 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    293 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    293 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    289 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    289 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    295 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    296 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    296 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    294 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    294 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    294 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    294 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    238 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    298 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    298 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    299 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    301 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    303 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    304 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    304 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    305 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    308 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    308 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    308 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    307 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    309 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    243 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    243 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    310 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    311 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    313 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    317 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    319 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    319 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    320 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    318 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    321 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    322 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    322 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    322 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    253 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    276 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    316 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    316 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    315 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    218 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    187 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    188 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    189 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    189 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    189 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    194 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    194 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    194 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    196 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    196 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    192 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    205 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    203 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    203 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    195 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    210 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    211 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    212 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    212 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    209 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    217 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    204 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    230 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    230 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    231 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    235 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    237 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    241 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    242 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    257 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    258 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    267 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    275 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    219 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    277 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    281 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    264 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    287 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    213 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    292 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    295 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    296 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    297 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    297 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    300 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    300 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    302 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    302 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    303 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    306 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    269 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    310 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    312 as ::core::ffi::c_int as ::core::ffi::c_ushort,
];

static mut yyRuleInfoNRhs: [::core::ffi::c_schar; 409] = [
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(7 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(10 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(9 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(9 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(10 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(8 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(8 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(11 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(7 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(7 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(8 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(12 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(9 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(8 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(8 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(9 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(12 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(11 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(9 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(8 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(7 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(8 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(8 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(6 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(5 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(3 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(4 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(2 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
];

unsafe extern "C" fn yy_reduce(
    mut yypParser: *mut yyParser,
    mut yyruleno: ::core::ffi::c_uint,
    mut _yyLookahead: ::core::ffi::c_int,
    mut yyLookaheadToken: crate::src::headers::sqliteInt_h::Token,
    mut pParse: *mut crate::src::headers::sqliteInt_h::Parse,
) -> ::core::ffi::c_ushort {
    let mut yygoto: ::core::ffi::c_int = 0;
    let mut yyact: ::core::ffi::c_ushort = 0;
    let mut yymsp: *mut yyStackEntry = ::core::ptr::null_mut::<yyStackEntry>();
    let mut yysize: ::core::ffi::c_int = 0;
    yymsp = (*yypParser).yytos;
    let mut yylhsminor: YYMINORTYPE = unsafe { ::core::mem::zeroed() };
    match yyruleno {
        0 => {
            if (*pParse).pReprepare.is_null() {
                (*pParse).explain = 1 as crate::src::ext::rtree::rtree::u8_0;
            }
        }
        1 => {
            if (*pParse).pReprepare.is_null() {
                (*pParse).explain = 2 as crate::src::ext::rtree::rtree::u8_0;
            }
        }
        2 => {
            crate::src::src::build::sqlite3FinishCoding(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            );
        }
        3 => {
            crate::src::src::build::sqlite3BeginTransaction(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        4 => {
            (*yymsp.offset(1 as isize)).minor.yy502 = crate::src::parse::TK_DEFERRED;
        }
        5 | 6 | 7 | 324 => {
            (*yymsp.offset(0 as isize)).minor.yy502 =
                (*yymsp.offset(0 as isize)).major as ::core::ffi::c_int;
        }
        8 | 9 => {
            crate::src::src::build::sqlite3EndTransaction(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize)).major as ::core::ffi::c_int,
            );
        }
        10 => {
            crate::src::src::build::sqlite3Savepoint(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::headers::sqliteInt_h::SAVEPOINT_BEGIN,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        11 => {
            crate::src::src::build::sqlite3Savepoint(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::headers::sqliteInt_h::SAVEPOINT_RELEASE,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        12 => {
            crate::src::src::build::sqlite3Savepoint(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::headers::sqliteInt_h::SAVEPOINT_ROLLBACK,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        13 => {
            crate::src::src::build::sqlite3StartTable(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        14 => {
            disableLookaside(pParse);
        }
        15 | 18 | 47 | 62 | 72 | 81 | 100 | 246 => {
            (*yymsp.offset(1 as isize)).minor.yy502 = 0 as ::core::ffi::c_int;
        }
        16 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = 1 as ::core::ffi::c_int;
        }
        17 => {
            (*yymsp.offset(0 as isize)).minor.yy502 =
                ((*(*pParse).db).init.busy as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
        }
        19 => {
            crate::src::src::build::sqlite3EndTable(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(0 as isize)).minor.yy9,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>()
                    as *mut crate::src::headers::sqliteInt_h::Select,
            );
        }
        20 => {
            crate::src::src::build::sqlite3EndTable(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
                0 as crate::src::ext::rtree::rtree::u32_0,
                (*yymsp.offset(0 as isize)).minor.yy637
                    as *mut crate::src::headers::sqliteInt_h::Select,
            );
            crate::src::src::select::sqlite3SelectDelete(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yymsp.offset(0 as isize)).minor.yy637
                    as *mut crate::src::headers::sqliteInt_h::Select,
            );
        }
        21 => {
            (*yymsp.offset(1 as isize)).minor.yy9 = 0 as crate::src::ext::rtree::rtree::u32_0;
        }
        22 => {
            yylhsminor.yy9 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy9
                | (*yymsp.offset(0 as isize)).minor.yy9;
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy9 = yylhsminor.yy9;
        }
        23 => {
            if (*yymsp.offset(0 as isize)).minor.yy0.n == 5 as ::core::ffi::c_uint
                && crate::src::src::util::sqlite3_strnicmp(
                    (*yymsp.offset(0 as isize)).minor.yy0.z,
                    b"rowid\0" as *const u8 as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
            {
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy9 = (crate::src::headers::sqliteInt_h::TF_WithoutRowid
                    | crate::src::headers::sqliteInt_h::TF_NoVisibleRowid)
                    as crate::src::ext::rtree::rtree::u32_0;
            } else {
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy9 = 0 as crate::src::ext::rtree::rtree::u32_0;
                crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    b"unknown table option: %.*s\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::src::src::printf::PrintfArg::Int(
                            (*yymsp.offset(0 as isize)).minor.yy0.n
                                as crate::src::ext::rtree::rtree::i64_0,
                        ),
                        crate::src::src::printf::PrintfArg::Str(
                            (*yymsp.offset(0 as isize)).minor.yy0.z as *mut ::core::ffi::c_char,
                        ),
                    ],
                );
            }
        }
        24 => {
            if (*yymsp.offset(0 as isize)).minor.yy0.n == 6 as ::core::ffi::c_uint
                && crate::src::src::util::sqlite3_strnicmp(
                    (*yymsp.offset(0 as isize)).minor.yy0.z,
                    b"strict\0" as *const u8 as *const ::core::ffi::c_char,
                    6 as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
            {
                yylhsminor.yy9 = crate::src::headers::sqliteInt_h::TF_Strict
                    as crate::src::ext::rtree::rtree::u32_0;
            } else {
                yylhsminor.yy9 = 0 as crate::src::ext::rtree::rtree::u32_0;
                crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    b"unknown table option: %.*s\0" as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::src::src::printf::PrintfArg::Int(
                            (*yymsp.offset(0 as isize)).minor.yy0.n
                                as crate::src::ext::rtree::rtree::i64_0,
                        ),
                        crate::src::src::printf::PrintfArg::Str(
                            (*yymsp.offset(0 as isize)).minor.yy0.z as *mut ::core::ffi::c_char,
                        ),
                    ],
                );
            }
            (*yymsp.offset(0 as isize)).minor.yy9 = yylhsminor.yy9;
        }
        25 => {
            crate::src::src::build::sqlite3AddColumn(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(0 as isize)).minor.yy0 as crate::src::headers::sqliteInt_h::Token,
            );
        }
        26 | 65 | 106 => {
            (*yymsp.offset(1 as isize)).minor.yy0.n = 0 as ::core::ffi::c_uint;
            let ref mut fresh1 = (*yymsp.offset(1 as isize)).minor.yy0.z;
            *fresh1 = ::core::ptr::null::<::core::ffi::c_char>();
        }
        27 => {
            (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n = ((*yymsp.offset(0 as isize))
                .minor
                .yy0
                .z
                .offset((*yymsp.offset(0 as isize)).minor.yy0.n as isize)
                as *const ::core::ffi::c_char)
                .offset_from(
                    (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0
                        .z,
                ) as ::core::ffi::c_long as ::core::ffi::c_int
                as ::core::ffi::c_uint;
        }
        28 => {
            (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n = ((*yymsp.offset(0 as isize))
                .minor
                .yy0
                .z
                .offset((*yymsp.offset(0 as isize)).minor.yy0.n as isize)
                as *const ::core::ffi::c_char)
                .offset_from(
                    (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0
                        .z,
                ) as ::core::ffi::c_long as ::core::ffi::c_int
                as ::core::ffi::c_uint;
        }
        29 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n = (*yymsp.offset(0 as isize)).minor.yy0.n.wrapping_add(
                (*yymsp.offset(0 as isize)).minor.yy0.z.offset_from(
                    (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0
                        .z,
                ) as ::core::ffi::c_long as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
            );
        }
        30 => {
            let ref mut fresh2 = (*yymsp.offset(1 as isize)).minor.yy342;
            *fresh2 = yyLookaheadToken.z;
        }
        31 => {
            (*yymsp.offset(1 as isize)).minor.yy0 = yyLookaheadToken;
        }
        32 | 67 => {
            (*pParse).u1.cr.constraintName = (*yymsp.offset(0 as isize)).minor.yy0;
        }
        33 => {
            crate::src::src::build::sqlite3AddDefaultValue(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z
                    .offset(
                        (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy0
                            .n as isize,
                    ) as *const ::core::ffi::c_char,
            );
        }
        34 => {
            crate::src::src::build::sqlite3AddDefaultValue(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z
                    .offset(1 as isize),
                (*yymsp.offset(0 as isize)).minor.yy0.z,
            );
        }
        35 => {
            crate::src::src::build::sqlite3AddDefaultValue(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z
                    .offset(
                        (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy0
                            .n as isize,
                    ) as *const ::core::ffi::c_char,
            );
        }
        36 => {
            let mut p: *mut crate::src::headers::sqliteInt_h::Expr =
                crate::src::src::expr::sqlite3PExpr(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    crate::src::parse::TK_UMINUS,
                    (*yymsp.offset(0 as isize)).minor.yy590
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::Expr;
            crate::src::src::build::sqlite3AddDefaultValue(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                p as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z
                    .offset(
                        (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy0
                            .n as isize,
                    ) as *const ::core::ffi::c_char,
            );
        }
        37 => {
            let mut p_0: *mut crate::src::headers::sqliteInt_h::Expr = tokenExpr(
                pParse,
                crate::src::parse::TK_STRING,
                (*yymsp.offset(0 as isize)).minor.yy0,
            );
            if !p_0.is_null() {
                crate::src::src::expr::sqlite3ExprIdToTrueFalse(
                    p_0 as *mut crate::src::headers::sqliteInt_h::Expr,
                );
            }
            crate::src::src::build::sqlite3AddDefaultValue(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                p_0 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy0.z,
                (*yymsp.offset(0 as isize))
                    .minor
                    .yy0
                    .z
                    .offset((*yymsp.offset(0 as isize)).minor.yy0.n as isize),
            );
        }
        38 => {
            crate::src::src::build::sqlite3AddNotNull(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy502,
            );
        }
        39 => {
            crate::src::src::build::sqlite3AddPrimaryKey(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(0 as isize)).minor.yy502,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        40 => {
            crate::src::src::build::sqlite3CreateIndex(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>()
                    as *mut crate::src::headers::sqliteInt_h::SrcList,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy502,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                crate::src::headers::sqliteInt_h::SQLITE_IDXTYPE_UNIQUE
                    as crate::src::ext::rtree::rtree::u8_0,
            );
        }
        41 => {
            crate::src::src::build::sqlite3AddCheckConstraint(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
                (*yymsp.offset(0 as isize)).minor.yy0.z,
            );
        }
        42 => {
            crate::src::src::build::sqlite3CreateForeignKey(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy502,
            );
        }
        43 => {
            crate::src::src::build::sqlite3DeferForeignKey(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy502,
            );
        }
        44 => {
            crate::src::src::build::sqlite3AddCollateType(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        45 => {
            crate::src::src::build::sqlite3AddGenerated(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        46 => {
            crate::src::src::build::sqlite3AddGenerated(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        48 => {
            (*yymsp.offset(0 as isize)).minor.yy502 = 1 as ::core::ffi::c_int;
        }
        49 => {
            (*yymsp.offset(1 as isize)).minor.yy502 =
                crate::src::headers::sqliteInt_h::OE_None * 0x101 as ::core::ffi::c_int;
        }
        50 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502
                & !(*yymsp.offset(0 as isize)).minor.yy481.mask
                | (*yymsp.offset(0 as isize)).minor.yy481.value;
        }
        51 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .value = 0 as ::core::ffi::c_int;
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .mask = 0 as ::core::ffi::c_int;
        }
        52 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .value = 0 as ::core::ffi::c_int;
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .mask = 0 as ::core::ffi::c_int;
        }
        53 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .value = (*yymsp.offset(0 as isize)).minor.yy502;
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .mask = 0xff as ::core::ffi::c_int;
        }
        54 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .value = (*yymsp.offset(0 as isize)).minor.yy502 << 8 as ::core::ffi::c_int;
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy481
                .mask = 0xff00 as ::core::ffi::c_int;
        }
        55 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = crate::src::headers::sqliteInt_h::OE_SetNull;
        }
        56 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = crate::src::headers::sqliteInt_h::OE_SetDflt;
        }
        57 => {
            (*yymsp.offset(0 as isize)).minor.yy502 = crate::src::headers::sqliteInt_h::OE_Cascade;
        }
        58 => {
            (*yymsp.offset(0 as isize)).minor.yy502 = crate::src::headers::sqliteInt_h::OE_Restrict;
        }
        59 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = crate::src::headers::sqliteInt_h::OE_None;
        }
        60 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = 0 as ::core::ffi::c_int;
        }
        61 | 76 | 173 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = (*yymsp.offset(0 as isize)).minor.yy502;
        }
        63 | 80 | 219 | 222 | 247 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = 1 as ::core::ffi::c_int;
        }
        64 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = 0 as ::core::ffi::c_int;
        }
        66 => {
            (*pParse).u1.cr.constraintName.n = 0 as ::core::ffi::c_uint;
        }
        68 => {
            crate::src::src::build::sqlite3AddPrimaryKey(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy502,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                0 as ::core::ffi::c_int,
            );
        }
        69 => {
            crate::src::src::build::sqlite3CreateIndex(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>()
                    as *mut crate::src::headers::sqliteInt_h::SrcList,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy502,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                crate::src::headers::sqliteInt_h::SQLITE_IDXTYPE_UNIQUE
                    as crate::src::ext::rtree::rtree::u8_0,
            );
        }
        70 => {
            crate::src::src::build::sqlite3AddCheckConstraint(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
            );
        }
        71 => {
            crate::src::src::build::sqlite3CreateForeignKey(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
            crate::src::src::build::sqlite3DeferForeignKey(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy502,
            );
        }
        73 | 75 => {
            (*yymsp.offset(1 as isize)).minor.yy502 = crate::src::headers::sqliteInt_h::OE_Default;
        }
        74 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = (*yymsp.offset(0 as isize)).minor.yy502;
        }
        77 => {
            (*yymsp.offset(0 as isize)).minor.yy502 = crate::src::headers::sqliteInt_h::OE_Ignore;
        }
        78 | 174 => {
            (*yymsp.offset(0 as isize)).minor.yy502 = crate::src::headers::sqliteInt_h::OE_Replace;
        }
        79 => {
            crate::src::src::build::sqlite3DropTable(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy563
                    as *mut crate::src::headers::sqliteInt_h::SrcList,
                0 as ::core::ffi::c_int,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        82 => {
            crate::src::src::build::sqlite3CreateView(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy637
                    as *mut crate::src::headers::sqliteInt_h::Select,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        83 => {
            crate::src::src::build::sqlite3DropTable(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy563
                    as *mut crate::src::headers::sqliteInt_h::SrcList,
                1 as ::core::ffi::c_int,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        84 => {
            let mut dest: crate::src::headers::sqliteInt_h::SelectDest =
                crate::src::headers::sqliteInt_h::SelectDest {
                    eDest: crate::src::headers::sqliteInt_h::SRT_Output
                        as crate::src::ext::rtree::rtree::u8_0,
                    iSDParm: 0 as ::core::ffi::c_int,
                    iSDParm2: 0 as ::core::ffi::c_int,
                    iSdst: 0 as ::core::ffi::c_int,
                    nSdst: 0 as ::core::ffi::c_int,
                    zAffSdst: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    pOrderBy: ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>(),
                };
            if (*(*pParse).db).mDbFlags
                & crate::src::headers::sqliteInt_h::DBFLAG_EncodingFixed
                    as crate::src::ext::rtree::rtree::u32_0
                != 0 as crate::src::ext::rtree::rtree::u32_0
                || crate::src::src::prepare::sqlite3ReadSchema(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ) == crate::src::headers::sqlite3_h::SQLITE_OK
            {
                crate::src::src::select::sqlite3Select(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*yymsp.offset(0 as isize)).minor.yy637
                        as *mut crate::src::headers::sqliteInt_h::Select,
                    &raw mut dest as *mut _ as *mut crate::src::headers::sqliteInt_h::SelectDest,
                );
            }
            crate::src::src::select::sqlite3SelectDelete(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yymsp.offset(0 as isize)).minor.yy637
                    as *mut crate::src::headers::sqliteInt_h::Select,
            );
        }
        85 => {
            let ref mut fresh3 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy637;
            *fresh3 = attachWithToSelect(
                pParse,
                (*yymsp.offset(0 as isize)).minor.yy637,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy125,
            );
        }
        86 => {
            let ref mut fresh4 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy637;
            *fresh4 = attachWithToSelect(
                pParse,
                (*yymsp.offset(0 as isize)).minor.yy637,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy125,
            );
        }
        87 => {
            let mut p_1: *mut crate::src::headers::sqliteInt_h::Select =
                (*yymsp.offset(0 as isize)).minor.yy637;
            if !p_1.is_null() {
                parserDoubleLinkSelect(pParse, p_1);
            }
        }
        88 => {
            let mut pRhs: *mut crate::src::headers::sqliteInt_h::Select =
                (*yymsp.offset(0 as isize)).minor.yy637;
            let mut pLhs: *mut crate::src::headers::sqliteInt_h::Select = (*yymsp
                .offset(-(2 as ::core::ffi::c_int) as isize))
            .minor
            .yy637;
            if !pRhs.is_null() && !(*pRhs).pPrior.is_null() {
                let mut pFrom: *mut crate::src::headers::sqliteInt_h::SrcList =
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>();
                let mut x: crate::src::headers::sqliteInt_h::Token =
                    unsafe { ::core::mem::zeroed() };
                x.n = 0 as ::core::ffi::c_uint;
                parserDoubleLinkSelect(pParse, pRhs);
                pFrom = crate::src::src::build::sqlite3SrcListAppendFromTerm(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>()
                        as *mut crate::src::headers::sqliteInt_h::SrcList,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                        as *mut crate::src::headers::sqliteInt_h::Token,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                        as *mut crate::src::headers::sqliteInt_h::Token,
                    &raw mut x as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                    pRhs as *mut crate::src::headers::sqliteInt_h::Select,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::OnOrUsing>()
                        as *mut crate::src::headers::sqliteInt_h::OnOrUsing,
                ) as *mut crate::src::headers::sqliteInt_h::SrcList;
                pRhs = crate::src::src::select::sqlite3SelectNew(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                        as *mut crate::src::headers::sqliteInt_h::ExprList,
                    pFrom as *mut crate::src::headers::sqliteInt_h::SrcList,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                        as *mut crate::src::headers::sqliteInt_h::ExprList,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                        as *mut crate::src::headers::sqliteInt_h::ExprList,
                    0 as crate::src::ext::rtree::rtree::u32_0,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::Select;
            }
            if !pRhs.is_null() {
                let __pRhs_ref = unsafe { &mut *pRhs };
                __pRhs_ref.op = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502 as crate::src::ext::rtree::rtree::u8_0;
                __pRhs_ref.pPrior = pLhs;
                if !pLhs.is_null() {
                    (*pLhs).selFlags &= !(crate::src::headers::sqliteInt_h::SF_MultiValue
                        as crate::src::ext::rtree::rtree::u32_0);
                }
                __pRhs_ref.selFlags &= !(crate::src::headers::sqliteInt_h::SF_MultiValue
                    as crate::src::ext::rtree::rtree::u32_0);
                if (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502
                    != crate::src::parse::TK_ALL
                {
                    (*pParse).hasCompound = 1 as crate::src::ext::rtree::rtree::u8_0;
                }
            } else {
                crate::src::src::select::sqlite3SelectDelete(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    pLhs as *mut crate::src::headers::sqliteInt_h::Select,
                );
            }
            let ref mut fresh5 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy637;
            *fresh5 = pRhs;
        }
        89 | 91 => {
            (*yymsp.offset(0 as isize)).minor.yy502 =
                (*yymsp.offset(0 as isize)).major as ::core::ffi::c_int;
        }
        90 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = crate::src::parse::TK_ALL;
        }
        92 => {
            let ref mut fresh6 = (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                .minor
                .yy637;
            *fresh6 = crate::src::src::select::sqlite3SelectNew(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502 as crate::src::ext::rtree::rtree::u32_0,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Select;
        }
        93 => {
            let ref mut fresh7 = (*yymsp.offset(-(9 as ::core::ffi::c_int) as isize))
                .minor
                .yy637;
            *fresh7 = crate::src::src::select::sqlite3SelectNew(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502 as crate::src::ext::rtree::rtree::u32_0,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Select;
            if !(*yymsp.offset(-(9 as ::core::ffi::c_int) as isize))
                .minor
                .yy637
                .is_null()
            {
                let ref mut fresh8 = (*(*yymsp.offset(-(9 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637)
                    .pWinDefn;
                *fresh8 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy483;
            } else {
                crate::src::src::window::sqlite3WindowListDelete(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy483 as *mut crate::src::headers::sqliteInt_h::Window,
                );
            }
        }
        94 => {
            let ref mut fresh9 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy637;
            *fresh9 = crate::src::src::select::sqlite3SelectNew(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
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
                crate::src::headers::sqliteInt_h::SF_Values as crate::src::ext::rtree::rtree::u32_0,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Select;
        }
        95 => {
            crate::src::src::insert::sqlite3MultiValuesEnd(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy637
                    as *mut crate::src::headers::sqliteInt_h::Select,
            );
        }
        96 | 97 => {
            let ref mut fresh10 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy637;
            *fresh10 = crate::src::src::insert::sqlite3MultiValues(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637 as *mut crate::src::headers::sqliteInt_h::Select,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
            ) as *mut crate::src::headers::sqliteInt_h::Select;
        }
        98 => {
            (*yymsp.offset(0 as isize)).minor.yy502 = crate::src::headers::sqliteInt_h::SF_Distinct;
        }
        99 => {
            (*yymsp.offset(0 as isize)).minor.yy502 = crate::src::headers::sqliteInt_h::SF_All;
        }
        101 | 134 | 144 | 234 | 237 | 242 => {
            let ref mut fresh11 = (*yymsp.offset(1 as isize)).minor.yy402;
            *fresh11 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
        }
        102 => {
            let ref mut fresh12 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh12 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            if (*yymsp.offset(0 as isize)).minor.yy0.n > 0 as ::core::ffi::c_uint {
                crate::src::src::expr::sqlite3ExprListSetName(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy402
                        as *mut crate::src::headers::sqliteInt_h::ExprList,
                    &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                        as *const crate::src::headers::sqliteInt_h::Token,
                    1 as ::core::ffi::c_int,
                );
            }
            crate::src::src::expr::sqlite3ExprListSetSpan(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy342,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy342,
            );
        }
        103 => {
            let mut p_2: *mut crate::src::headers::sqliteInt_h::Expr =
                crate::src::src::expr::sqlite3Expr(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    crate::src::parse::TK_ASTERISK,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                ) as *mut crate::src::headers::sqliteInt_h::Expr;
            crate::src::src::expr::sqlite3ExprSetErrorOffset(
                p_2 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize))
                    .minor
                    .yy0
                    .z
                    .offset_from((*pParse).zTail) as ::core::ffi::c_long
                    as ::core::ffi::c_int,
            );
            let ref mut fresh13 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh13 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                p_2 as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
        }
        104 => {
            let mut pRight: *mut crate::src::headers::sqliteInt_h::Expr =
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
            let mut pLeft: *mut crate::src::headers::sqliteInt_h::Expr =
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
            let mut pDot: *mut crate::src::headers::sqliteInt_h::Expr =
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
            pRight = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_ASTERISK,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            crate::src::src::expr::sqlite3ExprSetErrorOffset(
                pRight as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize))
                    .minor
                    .yy0
                    .z
                    .offset_from((*pParse).zTail) as ::core::ffi::c_long
                    as ::core::ffi::c_int,
            );
            pLeft = tokenExpr(
                pParse,
                crate::src::parse::TK_ID,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
            pDot = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_DOT,
                pLeft as *mut crate::src::headers::sqliteInt_h::Expr,
                pRight as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            let ref mut fresh14 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh14 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                pDot as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
        }
        105 | 117 | 258 | 259 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0 = (*yymsp.offset(0 as isize)).minor.yy0;
        }
        107 | 110 => {
            let ref mut fresh15 = (*yymsp.offset(1 as isize)).minor.yy563;
            *fresh15 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>();
        }
        108 => {
            let ref mut fresh16 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh16 = (*yymsp.offset(0 as isize)).minor.yy563;
            crate::src::src::build::sqlite3SrcListShiftJoinType(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
            );
        }
        109 => {
            if !(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy563
                .is_null()
                && (*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563)
                    .nSrc
                    > 0 as ::core::ffi::c_int
            {
                (*(&raw mut (*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563)
                    .a as *mut crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(
                        ((*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy563)
                            .nSrc
                            - 1 as ::core::ffi::c_int) as isize,
                    ))
                .fg
                .jointype =
                    (*yymsp.offset(0 as isize)).minor.yy502 as crate::src::ext::rtree::rtree::u8_0;
            }
        }
        111 => {
            let ref mut fresh17 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh17 = crate::src::src::build::sqlite3SrcListAppendFromTerm(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>()
                    as *mut crate::src::headers::sqliteInt_h::Select,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy421 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::OnOrUsing,
            ) as *mut crate::src::headers::sqliteInt_h::SrcList;
        }
        112 => {
            let ref mut fresh18 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh18 = crate::src::src::build::sqlite3SrcListAppendFromTerm(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>()
                    as *mut crate::src::headers::sqliteInt_h::Select,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy421 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::OnOrUsing,
            ) as *mut crate::src::headers::sqliteInt_h::SrcList;
            crate::src::src::build::sqlite3SrcListIndexedBy(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        113 => {
            let ref mut fresh19 = (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh19 = crate::src::src::build::sqlite3SrcListAppendFromTerm(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                &raw mut (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>()
                    as *mut crate::src::headers::sqliteInt_h::Select,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy421 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::OnOrUsing,
            ) as *mut crate::src::headers::sqliteInt_h::SrcList;
            crate::src::src::build::sqlite3SrcListFuncArgs(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
            );
        }
        114 => {
            let ref mut fresh20 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh20 = crate::src::src::build::sqlite3SrcListAppendFromTerm(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637 as *mut crate::src::headers::sqliteInt_h::Select,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy421 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::OnOrUsing,
            ) as *mut crate::src::headers::sqliteInt_h::SrcList;
        }
        115 => {
            if (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy563
                .is_null()
                && (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .n
                    == 0 as ::core::ffi::c_uint
                && (*yymsp.offset(0 as isize)).minor.yy421.pOn.is_null()
                && (*yymsp.offset(0 as isize)).minor.yy421.pUsing.is_null()
            {
                let ref mut fresh21 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563;
                *fresh21 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563;
            } else if !(*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy563
                .is_null()
                && (*(*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563)
                    .nSrc
                    == 1 as ::core::ffi::c_int
            {
                let ref mut fresh22 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563;
                *fresh22 = crate::src::src::build::sqlite3SrcListAppendFromTerm(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy563
                        as *mut crate::src::headers::sqliteInt_h::SrcList,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                        as *mut crate::src::headers::sqliteInt_h::Token,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                        as *mut crate::src::headers::sqliteInt_h::Token,
                    &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0 as *mut _
                        as *mut crate::src::headers::sqliteInt_h::Token,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>()
                        as *mut crate::src::headers::sqliteInt_h::Select,
                    &raw mut (*yymsp.offset(0 as isize)).minor.yy421 as *mut _
                        as *mut crate::src::headers::sqliteInt_h::OnOrUsing,
                ) as *mut crate::src::headers::sqliteInt_h::SrcList;
                if !(*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563
                    .is_null()
                {
                    let mut pNew: *mut crate::src::headers::sqliteInt_h::SrcItem =
                        (&raw mut (*(*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy563)
                            .a
                            as *mut crate::src::headers::sqliteInt_h::SrcItem)
                            .offset(
                                ((*(*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                                    .minor
                                    .yy563)
                                    .nSrc
                                    - 1 as ::core::ffi::c_int)
                                    as isize,
                            )
                            as *mut crate::src::headers::sqliteInt_h::SrcItem;
                    let mut pOld: *mut crate::src::headers::sqliteInt_h::SrcItem =
                        &raw mut (*(*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy563)
                            .a
                            as *mut crate::src::headers::sqliteInt_h::SrcItem;
                    let __pOld_ref = unsafe { &mut *pOld };
                    (*pNew).zName = __pOld_ref.zName;
                    if __pOld_ref.fg.isSubquery() != 0 {
                        let __pNew_ref = unsafe { &mut *pNew };
                        (*pNew)
                            .fg
                            .set_isSubquery(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        __pNew_ref.u4.pSubq = __pOld_ref.u4.pSubq;
                        __pOld_ref.u4.pSubq =
                            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Subquery>();
                        (*pOld)
                            .fg
                            .set_isSubquery(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        if (*(*__pNew_ref.u4.pSubq).pSelect).selFlags
                            & crate::src::headers::sqliteInt_h::SF_NestedFrom
                                as crate::src::ext::rtree::rtree::u32_0
                            != 0 as crate::src::ext::rtree::rtree::u32_0
                        {
                            (*pNew)
                                .fg
                                .set_isNestedFrom(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        }
                    } else {
                        (*pNew).u4.zDatabase = __pOld_ref.u4.zDatabase;
                        __pOld_ref.u4.zDatabase = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    }
                    if __pOld_ref.fg.isTabFunc() != 0 {
                        (*pNew).u1.pFuncArg = __pOld_ref.u1.pFuncArg;
                        __pOld_ref.u1.pFuncArg =
                            ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
                        (*pOld)
                            .fg
                            .set_isTabFunc(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        (*pNew)
                            .fg
                            .set_isTabFunc(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                    __pOld_ref.zName = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                crate::src::src::build::sqlite3SrcListDelete(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy563
                        as *mut crate::src::headers::sqliteInt_h::SrcList,
                );
            } else {
                let mut pSubquery: *mut crate::src::headers::sqliteInt_h::Select =
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
                crate::src::src::build::sqlite3SrcListShiftJoinType(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy563
                        as *mut crate::src::headers::sqliteInt_h::SrcList,
                );
                pSubquery = crate::src::src::select::sqlite3SelectNew(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                        as *mut crate::src::headers::sqliteInt_h::ExprList,
                    (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy563
                        as *mut crate::src::headers::sqliteInt_h::SrcList,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                        as *mut crate::src::headers::sqliteInt_h::ExprList,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                        as *mut crate::src::headers::sqliteInt_h::ExprList,
                    crate::src::headers::sqliteInt_h::SF_NestedFrom
                        as crate::src::ext::rtree::rtree::u32_0,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::Select;
                let ref mut fresh23 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563;
                *fresh23 = crate::src::src::build::sqlite3SrcListAppendFromTerm(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy563
                        as *mut crate::src::headers::sqliteInt_h::SrcList,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                        as *mut crate::src::headers::sqliteInt_h::Token,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                        as *mut crate::src::headers::sqliteInt_h::Token,
                    &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0 as *mut _
                        as *mut crate::src::headers::sqliteInt_h::Token,
                    pSubquery as *mut crate::src::headers::sqliteInt_h::Select,
                    &raw mut (*yymsp.offset(0 as isize)).minor.yy421 as *mut _
                        as *mut crate::src::headers::sqliteInt_h::OnOrUsing,
                ) as *mut crate::src::headers::sqliteInt_h::SrcList;
            }
        }
        116 | 131 => {
            let ref mut fresh24 = (*yymsp.offset(1 as isize)).minor.yy0.z;
            *fresh24 = ::core::ptr::null::<::core::ffi::c_char>();
            (*yymsp.offset(1 as isize)).minor.yy0.n = 0 as ::core::ffi::c_uint;
        }
        118 => {
            yylhsminor.yy563 = crate::src::src::build::sqlite3SrcListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>()
                    as *mut crate::src::headers::sqliteInt_h::SrcList,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
            ) as *mut crate::src::headers::sqliteInt_h::SrcList;
            if (*pParse).eParseMode as ::core::ffi::c_int
                >= crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME
                && !yylhsminor.yy563.is_null()
            {
                crate::src::src::alter::sqlite3RenameTokenMap(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*(&raw mut (*yylhsminor.yy563).a
                        as *mut crate::src::headers::sqliteInt_h::SrcItem)
                        .offset(0 as isize))
                    .zName as *const ::core::ffi::c_void,
                    &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                        as *const crate::src::headers::sqliteInt_h::Token,
                );
            }
            let ref mut fresh25 = (*yymsp.offset(0 as isize)).minor.yy563;
            *fresh25 = yylhsminor.yy563;
        }
        119 => {
            yylhsminor.yy563 = crate::src::src::build::sqlite3SrcListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>()
                    as *mut crate::src::headers::sqliteInt_h::SrcList,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
            ) as *mut crate::src::headers::sqliteInt_h::SrcList;
            if (*pParse).eParseMode as ::core::ffi::c_int
                >= crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME
                && !yylhsminor.yy563.is_null()
            {
                crate::src::src::alter::sqlite3RenameTokenMap(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*(&raw mut (*yylhsminor.yy563).a
                        as *mut crate::src::headers::sqliteInt_h::SrcItem)
                        .offset(0 as isize))
                    .zName as *const ::core::ffi::c_void,
                    &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                        as *const crate::src::headers::sqliteInt_h::Token,
                );
            }
            let ref mut fresh26 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh26 = yylhsminor.yy563;
        }
        120 => {
            let ref mut fresh27 = (*yymsp.offset(0 as isize)).minor.yy563;
            *fresh27 = crate::src::src::build::sqlite3SrcListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>()
                    as *mut crate::src::headers::sqliteInt_h::SrcList,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
            ) as *mut crate::src::headers::sqliteInt_h::SrcList;
        }
        121 => {
            let ref mut fresh28 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh28 = crate::src::src::build::sqlite3SrcListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>()
                    as *mut crate::src::headers::sqliteInt_h::SrcList,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
            ) as *mut crate::src::headers::sqliteInt_h::SrcList;
        }
        122 => {
            let ref mut fresh29 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh29 = crate::src::src::build::sqlite3SrcListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>()
                    as *mut crate::src::headers::sqliteInt_h::SrcList,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
            ) as *mut crate::src::headers::sqliteInt_h::SrcList;
            if !(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy563
                .is_null()
            {
                let ref mut fresh30 = (*(&raw mut (*(*yymsp
                    .offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy563)
                    .a
                    as *mut crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(0 as isize))
                .zAlias;
                *fresh30 = crate::src::src::build::sqlite3NameFromToken(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                        as *const crate::src::headers::sqliteInt_h::Token,
                );
            }
        }
        123 => {
            let ref mut fresh31 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
            *fresh31 = crate::src::src::build::sqlite3SrcListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>()
                    as *mut crate::src::headers::sqliteInt_h::SrcList,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
            ) as *mut crate::src::headers::sqliteInt_h::SrcList;
            if !(*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy563
                .is_null()
            {
                let ref mut fresh32 = (*(&raw mut (*(*yymsp
                    .offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy563)
                    .a
                    as *mut crate::src::headers::sqliteInt_h::SrcItem)
                    .offset(0 as isize))
                .zAlias;
                *fresh32 = crate::src::src::build::sqlite3NameFromToken(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                        as *const crate::src::headers::sqliteInt_h::Token,
                );
            }
        }
        124 => {
            (*yymsp.offset(0 as isize)).minor.yy502 = crate::src::headers::sqliteInt_h::JT_INNER;
        }
        125 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = crate::src::src::select::sqlite3JoinType(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        126 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = crate::src::src::select::sqlite3JoinType(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        127 => {
            (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = crate::src::src::select::sqlite3JoinType(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        128 => {
            let ref mut fresh33 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy421
                .pOn;
            *fresh33 = (*yymsp.offset(0 as isize)).minor.yy590;
            let ref mut fresh34 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy421
                .pUsing;
            *fresh34 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::IdList>();
        }
        129 => {
            let ref mut fresh35 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy421
                .pOn;
            *fresh35 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
            let ref mut fresh36 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy421
                .pUsing;
            *fresh36 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy204;
        }
        130 => {
            let ref mut fresh37 = (*yymsp.offset(1 as isize)).minor.yy421.pOn;
            *fresh37 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
            let ref mut fresh38 = (*yymsp.offset(1 as isize)).minor.yy421.pUsing;
            *fresh38 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::IdList>();
        }
        132 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy0 = (*yymsp.offset(0 as isize)).minor.yy0;
        }
        133 => {
            let ref mut fresh39 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .z;
            *fresh39 = ::core::ptr::null::<::core::ffi::c_char>();
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n = 1 as ::core::ffi::c_uint;
        }
        135 | 145 => {
            let ref mut fresh40 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh40 = (*yymsp.offset(0 as isize)).minor.yy402;
        }
        136 => {
            let ref mut fresh41 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh41 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            crate::src::src::expr::sqlite3ExprListSetSortOrder(
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(0 as isize)).minor.yy502,
            );
        }
        137 => {
            let ref mut fresh42 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh42 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            crate::src::src::expr::sqlite3ExprListSetSortOrder(
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(0 as isize)).minor.yy502,
            );
        }
        138 => {
            (*yymsp.offset(0 as isize)).minor.yy502 =
                crate::src::headers::sqliteInt_h::SQLITE_SO_ASC;
        }
        139 => {
            (*yymsp.offset(0 as isize)).minor.yy502 =
                crate::src::headers::sqliteInt_h::SQLITE_SO_DESC;
        }
        140 | 143 => {
            (*yymsp.offset(1 as isize)).minor.yy502 =
                crate::src::headers::sqliteInt_h::SQLITE_SO_UNDEFINED;
        }
        141 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = crate::src::headers::sqliteInt_h::SQLITE_SO_ASC;
        }
        142 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = crate::src::headers::sqliteInt_h::SQLITE_SO_DESC;
        }
        146 | 148 | 153 | 155 | 232 | 233 | 252 => {
            let ref mut fresh43 = (*yymsp.offset(1 as isize)).minor.yy590;
            *fresh43 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
        }
        147 | 154 | 156 | 231 | 251 => {
            let ref mut fresh44 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh44 = (*yymsp.offset(0 as isize)).minor.yy590;
        }
        149 => {
            let ref mut fresh45 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh45 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_LIMIT,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
        }
        150 => {
            let ref mut fresh46 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh46 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_LIMIT,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
        }
        151 => {
            let ref mut fresh47 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh47 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_LIMIT,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
        }
        152 => {
            crate::src::src::build::sqlite3SrcListIndexedBy(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
            );
            crate::src::src::delete::sqlite3DeleteFrom(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            );
        }
        157 => {
            crate::src::src::build::sqlite3AddReturning(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy402
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
            );
            let ref mut fresh48 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh48 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
        }
        158 => {
            crate::src::src::build::sqlite3AddReturning(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy402
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
            );
            let ref mut fresh49 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh49 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
        }
        159 => {
            crate::src::src::build::sqlite3SrcListIndexedBy(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                &raw mut (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
            );
            crate::src::src::expr::sqlite3ExprListCheckLength(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                b"set list\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !(*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy563
                .is_null()
            {
                let mut pFromClause: *mut crate::src::headers::sqliteInt_h::SrcList = (*yymsp
                    .offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy563;
                if (*pFromClause).nSrc > 1 as ::core::ffi::c_int {
                    let mut pSubquery_0: *mut crate::src::headers::sqliteInt_h::Select =
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
                    let mut as_0: crate::src::headers::sqliteInt_h::Token =
                        unsafe { ::core::mem::zeroed() };
                    pSubquery_0 = crate::src::src::select::sqlite3SelectNew(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                            as *mut crate::src::headers::sqliteInt_h::ExprList,
                        pFromClause as *mut crate::src::headers::sqliteInt_h::SrcList,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                            as *mut crate::src::headers::sqliteInt_h::ExprList,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                            as *mut crate::src::headers::sqliteInt_h::ExprList,
                        crate::src::headers::sqliteInt_h::SF_NestedFrom
                            as crate::src::ext::rtree::rtree::u32_0,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                    )
                        as *mut crate::src::headers::sqliteInt_h::Select;
                    as_0.n = 0 as ::core::ffi::c_uint;
                    as_0.z = ::core::ptr::null::<::core::ffi::c_char>();
                    pFromClause = crate::src::src::build::sqlite3SrcListAppendFromTerm(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>()
                            as *mut crate::src::headers::sqliteInt_h::SrcList,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                            as *mut crate::src::headers::sqliteInt_h::Token,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                            as *mut crate::src::headers::sqliteInt_h::Token,
                        &raw mut as_0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                        pSubquery_0 as *mut crate::src::headers::sqliteInt_h::Select,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::OnOrUsing>()
                            as *mut crate::src::headers::sqliteInt_h::OnOrUsing,
                    )
                        as *mut crate::src::headers::sqliteInt_h::SrcList;
                }
                let ref mut fresh50 = (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563;
                *fresh50 = crate::src::src::build::sqlite3SrcListAppendList(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy563
                        as *mut crate::src::headers::sqliteInt_h::SrcList,
                    pFromClause as *mut crate::src::headers::sqliteInt_h::SrcList,
                ) as *mut crate::src::headers::sqliteInt_h::SrcList;
            }
            crate::src::src::update::sqlite3Update(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Upsert>()
                    as *mut crate::src::headers::sqliteInt_h::Upsert,
            );
        }
        160 => {
            let ref mut fresh51 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh51 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            crate::src::src::expr::sqlite3ExprListSetName(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
                1 as ::core::ffi::c_int,
            );
        }
        161 => {
            let ref mut fresh52 = (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh52 = crate::src::src::expr::sqlite3ExprListAppendVector(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy204 as *mut crate::src::headers::sqliteInt_h::IdList,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
        }
        162 => {
            yylhsminor.yy402 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            crate::src::src::expr::sqlite3ExprListSetName(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                yylhsminor.yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
                1 as ::core::ffi::c_int,
            );
            let ref mut fresh53 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh53 = yylhsminor.yy402;
        }
        163 => {
            let ref mut fresh54 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh54 = crate::src::src::expr::sqlite3ExprListAppendVector(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy204 as *mut crate::src::headers::sqliteInt_h::IdList,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
        }
        164 => {
            crate::src::src::insert::sqlite3Insert(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637 as *mut crate::src::headers::sqliteInt_h::Select,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy204 as *mut crate::src::headers::sqliteInt_h::IdList,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(0 as isize)).minor.yy403
                    as *mut crate::src::headers::sqliteInt_h::Upsert,
            );
        }
        165 => {
            crate::src::src::insert::sqlite3Insert(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>()
                    as *mut crate::src::headers::sqliteInt_h::Select,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy204 as *mut crate::src::headers::sqliteInt_h::IdList,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Upsert>()
                    as *mut crate::src::headers::sqliteInt_h::Upsert,
            );
        }
        166 => {
            let ref mut fresh55 = (*yymsp.offset(1 as isize)).minor.yy403;
            *fresh55 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Upsert>();
        }
        167 => {
            let ref mut fresh56 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy403;
            *fresh56 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Upsert>();
            crate::src::src::build::sqlite3AddReturning(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy402
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
            );
        }
        168 => {
            let ref mut fresh57 = (*yymsp.offset(-(11 as ::core::ffi::c_int) as isize))
                .minor
                .yy403;
            *fresh57 = crate::src::src::upsert::sqlite3UpsertNew(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy403
                    as *mut crate::src::headers::sqliteInt_h::Upsert,
            ) as *mut crate::src::headers::sqliteInt_h::Upsert;
        }
        169 => {
            let ref mut fresh58 = (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                .minor
                .yy403;
            *fresh58 = crate::src::src::upsert::sqlite3UpsertNew(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy403
                    as *mut crate::src::headers::sqliteInt_h::Upsert,
            ) as *mut crate::src::headers::sqliteInt_h::Upsert;
        }
        170 => {
            let ref mut fresh59 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy403;
            *fresh59 = crate::src::src::upsert::sqlite3UpsertNew(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Upsert>()
                    as *mut crate::src::headers::sqliteInt_h::Upsert,
            ) as *mut crate::src::headers::sqliteInt_h::Upsert;
        }
        171 => {
            let ref mut fresh60 = (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                .minor
                .yy403;
            *fresh60 = crate::src::src::upsert::sqlite3UpsertNew(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Upsert>()
                    as *mut crate::src::headers::sqliteInt_h::Upsert,
            ) as *mut crate::src::headers::sqliteInt_h::Upsert;
        }
        172 => {
            crate::src::src::build::sqlite3AddReturning(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy402
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
            );
        }
        175 => {
            let ref mut fresh61 = (*yymsp.offset(1 as isize)).minor.yy204;
            *fresh61 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::IdList>();
        }
        176 => {
            let ref mut fresh62 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy204;
            *fresh62 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy204;
        }
        177 => {
            let ref mut fresh63 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy204;
            *fresh63 = crate::src::src::build::sqlite3IdListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy204 as *mut crate::src::headers::sqliteInt_h::IdList,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
            ) as *mut crate::src::headers::sqliteInt_h::IdList;
        }
        178 => {
            let ref mut fresh64 = (*yymsp.offset(0 as isize)).minor.yy204;
            *fresh64 = crate::src::src::build::sqlite3IdListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::IdList>()
                    as *mut crate::src::headers::sqliteInt_h::IdList,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
            ) as *mut crate::src::headers::sqliteInt_h::IdList;
        }
        179 => {
            let ref mut fresh65 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh65 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
        }
        180 => {
            let ref mut fresh66 = (*yymsp.offset(0 as isize)).minor.yy590;
            *fresh66 = tokenExpr(
                pParse,
                crate::src::parse::TK_ID,
                (*yymsp.offset(0 as isize)).minor.yy0,
            );
        }
        181 => {
            let mut temp1: *mut crate::src::headers::sqliteInt_h::Expr = tokenExpr(
                pParse,
                crate::src::parse::TK_ID,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
            let mut temp2: *mut crate::src::headers::sqliteInt_h::Expr = tokenExpr(
                pParse,
                crate::src::parse::TK_ID,
                (*yymsp.offset(0 as isize)).minor.yy0,
            );
            yylhsminor.yy590 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_DOT,
                temp1 as *mut crate::src::headers::sqliteInt_h::Expr,
                temp2 as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            let ref mut fresh67 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh67 = yylhsminor.yy590;
        }
        182 => {
            let mut temp1_0: *mut crate::src::headers::sqliteInt_h::Expr = tokenExpr(
                pParse,
                crate::src::parse::TK_ID,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
            let mut temp2_0: *mut crate::src::headers::sqliteInt_h::Expr = tokenExpr(
                pParse,
                crate::src::parse::TK_ID,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
            );
            let mut temp3: *mut crate::src::headers::sqliteInt_h::Expr = tokenExpr(
                pParse,
                crate::src::parse::TK_ID,
                (*yymsp.offset(0 as isize)).minor.yy0,
            );
            let mut temp4: *mut crate::src::headers::sqliteInt_h::Expr =
                crate::src::src::expr::sqlite3PExpr(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    crate::src::parse::TK_DOT,
                    temp2_0 as *mut crate::src::headers::sqliteInt_h::Expr,
                    temp3 as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::Expr;
            if (*pParse).eParseMode as ::core::ffi::c_int
                >= crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME
            {
                crate::src::src::alter::sqlite3RenameTokenRemap(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    ::core::ptr::null::<::core::ffi::c_void>(),
                    temp1_0 as *const ::core::ffi::c_void,
                );
            }
            yylhsminor.yy590 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_DOT,
                temp1_0 as *mut crate::src::headers::sqliteInt_h::Expr,
                temp4 as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            let ref mut fresh68 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh68 = yylhsminor.yy590;
        }
        183 | 184 => {
            let ref mut fresh69 = (*yymsp.offset(0 as isize)).minor.yy590;
            *fresh69 = tokenExpr(
                pParse,
                (*yymsp.offset(0 as isize)).major as ::core::ffi::c_int,
                (*yymsp.offset(0 as isize)).minor.yy0,
            );
        }
        185 => {
            yylhsminor.yy590 = crate::src::src::expr::sqlite3ExprAlloc(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                crate::src::parse::TK_INTEGER,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
                1 as ::core::ffi::c_int,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            if !yylhsminor.yy590.is_null() {
                (*yylhsminor.yy590).w.iOfst = (*yymsp.offset(0 as isize))
                    .minor
                    .yy0
                    .z
                    .offset_from((*pParse).zTail)
                    as ::core::ffi::c_long
                    as ::core::ffi::c_int;
            }
            let ref mut fresh70 = (*yymsp.offset(0 as isize)).minor.yy590;
            *fresh70 = yylhsminor.yy590;
        }
        186 => {
            if !(*(*yymsp.offset(0 as isize)).minor.yy0.z.offset(0 as isize) as ::core::ffi::c_int
                == '#' as i32
                && *(&raw const crate::src::src::global::sqlite3CtypeMap
                    as *const ::core::ffi::c_uchar)
                    .offset(*(*yymsp.offset(0 as isize)).minor.yy0.z.offset(1 as isize)
                        as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
                    & 0x4 as ::core::ffi::c_int
                    != 0)
            {
                let mut n: crate::src::ext::rtree::rtree::u32_0 =
                    (*yymsp.offset(0 as isize)).minor.yy0.n as crate::src::ext::rtree::rtree::u32_0;
                let ref mut fresh71 = (*yymsp.offset(0 as isize)).minor.yy590;
                *fresh71 = tokenExpr(
                    pParse,
                    crate::src::parse::TK_VARIABLE,
                    (*yymsp.offset(0 as isize)).minor.yy0,
                );
                crate::src::src::expr::sqlite3ExprAssignVarNumber(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*yymsp.offset(0 as isize)).minor.yy590
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                    n,
                );
            } else {
                let mut t: crate::src::headers::sqliteInt_h::Token =
                    (*yymsp.offset(0 as isize)).minor.yy0;
                if (*pParse).nested as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    parserSyntaxError(pParse, &raw mut t);
                    let ref mut fresh72 = (*yymsp.offset(0 as isize)).minor.yy590;
                    *fresh72 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
                } else {
                    let ref mut fresh73 = (*yymsp.offset(0 as isize)).minor.yy590;
                    *fresh73 = crate::src::src::expr::sqlite3PExpr(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        crate::src::parse::TK_REGISTER,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                    ) as *mut crate::src::headers::sqliteInt_h::Expr;
                    if !(*yymsp.offset(0 as isize)).minor.yy590.is_null() {
                        crate::src::src::util::sqlite3GetInt32(
                            t.z.offset(1 as isize) as *const ::core::ffi::c_char,
                            &raw mut (*(*yymsp.offset(0 as isize)).minor.yy590).iTable,
                        );
                    }
                }
            }
        }
        187 => {
            let ref mut fresh74 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh74 = crate::src::src::expr::sqlite3ExprAddCollateToken(
                pParse as *const crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
                1 as ::core::ffi::c_int,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
        }
        188 => {
            let ref mut fresh75 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh75 = crate::src::src::expr::sqlite3ExprAlloc(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                crate::src::parse::TK_CAST,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
                1 as ::core::ffi::c_int,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            crate::src::src::expr::sqlite3ExprAttachSubtrees(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            );
        }
        189 => {
            yylhsminor.yy590 = crate::src::src::expr::sqlite3ExprFunction(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            let ref mut fresh76 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh76 = yylhsminor.yy590;
        }
        190 => {
            yylhsminor.yy590 = crate::src::src::expr::sqlite3ExprFunction(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            crate::src::src::expr::sqlite3ExprAddFunctionOrderBy(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                yylhsminor.yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
            );
            let ref mut fresh77 = (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh77 = yylhsminor.yy590;
        }
        191 => {
            yylhsminor.yy590 = crate::src::src::expr::sqlite3ExprFunction(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
                0 as ::core::ffi::c_int,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            let ref mut fresh78 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh78 = yylhsminor.yy590;
        }
        192 => {
            yylhsminor.yy590 = crate::src::src::expr::sqlite3ExprFunction(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            crate::src::src::window::sqlite3WindowAttach(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                yylhsminor.yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy483
                    as *mut crate::src::headers::sqliteInt_h::Window,
            );
            let ref mut fresh79 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh79 = yylhsminor.yy590;
        }
        193 => {
            yylhsminor.yy590 = crate::src::src::expr::sqlite3ExprFunction(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            crate::src::src::window::sqlite3WindowAttach(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                yylhsminor.yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy483
                    as *mut crate::src::headers::sqliteInt_h::Window,
            );
            crate::src::src::expr::sqlite3ExprAddFunctionOrderBy(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                yylhsminor.yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
            );
            let ref mut fresh80 = (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh80 = yylhsminor.yy590;
        }
        194 => {
            yylhsminor.yy590 = crate::src::src::expr::sqlite3ExprFunction(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
                0 as ::core::ffi::c_int,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            crate::src::src::window::sqlite3WindowAttach(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                yylhsminor.yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy483
                    as *mut crate::src::headers::sqliteInt_h::Window,
            );
            let ref mut fresh81 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh81 = yylhsminor.yy590;
        }
        195 => {
            yylhsminor.yy590 = crate::src::src::expr::sqlite3ExprFunction(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
                0 as ::core::ffi::c_int,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            let ref mut fresh82 = (*yymsp.offset(0 as isize)).minor.yy590;
            *fresh82 = yylhsminor.yy590;
        }
        196 => {
            let mut pList: *mut crate::src::headers::sqliteInt_h::ExprList =
                crate::src::src::expr::sqlite3ExprListAppend(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy402
                        as *mut crate::src::headers::sqliteInt_h::ExprList,
                    (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            let ref mut fresh83 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh83 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_VECTOR,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            if !(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590
                .is_null()
            {
                let ref mut fresh84 = (*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .x
                    .pList;
                *fresh84 = pList;
                if (*pList).nExpr != 0 {
                    (*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590)
                        .flags |= (*(*(&raw mut (*pList).a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(0 as isize))
                    .pExpr)
                        .flags
                        & crate::src::headers::sqliteInt_h::EP_Propagate
                            as crate::src::ext::rtree::rtree::u32_0;
                }
            } else {
                crate::src::src::expr::sqlite3ExprListDelete(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    pList as *mut crate::src::headers::sqliteInt_h::ExprList,
                );
            }
        }
        197 => {
            let ref mut fresh85 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh85 = crate::src::src::expr::sqlite3ExprAnd(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
        }
        198 | 199 | 200 | 201 | 202 | 203 | 204 => {
            let ref mut fresh86 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh86 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize)).major as ::core::ffi::c_int,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
        }
        205 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0 = (*yymsp.offset(0 as isize)).minor.yy0;
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n |= 0x80000000 as ::core::ffi::c_uint;
        }
        206 => {
            let mut pList_0: *mut crate::src::headers::sqliteInt_h::ExprList =
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
            let mut bNot: ::core::ffi::c_int = ((*yymsp
                .offset(-(1 as ::core::ffi::c_int) as isize))
            .minor
            .yy0
            .n & 0x80000000 as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n &= 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint;
            pList_0 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            pList_0 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                pList_0 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            let ref mut fresh87 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh87 = crate::src::src::expr::sqlite3ExprFunction(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                pList_0 as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
                0 as ::core::ffi::c_int,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            if bNot != 0 {
                let ref mut fresh88 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
                *fresh88 = crate::src::src::expr::sqlite3PExpr(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    crate::src::parse::TK_NOT,
                    (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::Expr;
            }
            if !(*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590
                .is_null()
            {
                (*(*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .flags |= crate::src::headers::sqliteInt_h::EP_InfixFunc
                    as crate::src::ext::rtree::rtree::u32_0;
            }
        }
        207 => {
            let mut pList_1: *mut crate::src::headers::sqliteInt_h::ExprList =
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>();
            let mut bNot_0: ::core::ffi::c_int = ((*yymsp
                .offset(-(3 as ::core::ffi::c_int) as isize))
            .minor
            .yy0
            .n & 0x80000000 as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
            (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n &= 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint;
            pList_1 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            pList_1 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                pList_1 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            pList_1 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                pList_1 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            let ref mut fresh89 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh89 = crate::src::src::expr::sqlite3ExprFunction(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                pList_1 as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
                0 as ::core::ffi::c_int,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            if bNot_0 != 0 {
                let ref mut fresh90 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
                *fresh90 = crate::src::src::expr::sqlite3PExpr(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    crate::src::parse::TK_NOT,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::Expr;
            }
            if !(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590
                .is_null()
            {
                (*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .flags |= crate::src::headers::sqliteInt_h::EP_InfixFunc
                    as crate::src::ext::rtree::rtree::u32_0;
            }
        }
        208 => {
            let ref mut fresh91 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh91 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).major as ::core::ffi::c_int,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
        }
        209 => {
            let ref mut fresh92 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh92 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_NOTNULL,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
        }
        210 => {
            let ref mut fresh93 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh93 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_IS,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            binaryToUnaryIfNull(
                pParse,
                (*yymsp.offset(0 as isize)).minor.yy590,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                crate::src::parse::TK_ISNULL,
            );
        }
        211 => {
            let ref mut fresh94 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh94 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_ISNOT,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            binaryToUnaryIfNull(
                pParse,
                (*yymsp.offset(0 as isize)).minor.yy590,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                crate::src::parse::TK_NOTNULL,
            );
        }
        212 => {
            let ref mut fresh95 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh95 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_IS,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            binaryToUnaryIfNull(
                pParse,
                (*yymsp.offset(0 as isize)).minor.yy590,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                crate::src::parse::TK_ISNULL,
            );
        }
        213 => {
            let ref mut fresh96 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh96 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_ISNOT,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            binaryToUnaryIfNull(
                pParse,
                (*yymsp.offset(0 as isize)).minor.yy590,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590,
                crate::src::parse::TK_NOTNULL,
            );
        }
        214 | 215 => {
            let ref mut fresh97 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh97 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize)).major as ::core::ffi::c_int,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
        }
        216 => {
            let mut p_3: *mut crate::src::headers::sqliteInt_h::Expr =
                (*yymsp.offset(0 as isize)).minor.yy590;
            let mut op: crate::src::ext::rtree::rtree::u8_0 =
                ((*yymsp.offset(-(1 as ::core::ffi::c_int) as isize)).major as ::core::ffi::c_int
                    + (crate::src::parse::TK_UPLUS - crate::src::parse::TK_PLUS))
                    as crate::src::ext::rtree::rtree::u8_0;
            if !p_3.is_null() && (*p_3).op as ::core::ffi::c_int == crate::src::parse::TK_UPLUS {
                (*p_3).op = op;
                let ref mut fresh98 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
                *fresh98 = p_3;
            } else {
                let ref mut fresh99 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
                *fresh99 = crate::src::src::expr::sqlite3PExpr(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    op as ::core::ffi::c_int,
                    p_3 as *mut crate::src::headers::sqliteInt_h::Expr,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::Expr;
            }
        }
        217 => {
            let mut pList_2: *mut crate::src::headers::sqliteInt_h::ExprList =
                crate::src::src::expr::sqlite3ExprListAppend(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                        as *mut crate::src::headers::sqliteInt_h::ExprList,
                    (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            pList_2 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                pList_2 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            yylhsminor.yy590 = crate::src::src::expr::sqlite3ExprFunction(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                pList_2 as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
                0 as ::core::ffi::c_int,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            let ref mut fresh100 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh100 = yylhsminor.yy590;
        }
        218 | 221 => {
            (*yymsp.offset(0 as isize)).minor.yy502 = 0 as ::core::ffi::c_int;
        }
        220 => {
            let mut pList_3: *mut crate::src::headers::sqliteInt_h::ExprList =
                crate::src::src::expr::sqlite3ExprListAppend(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                        as *mut crate::src::headers::sqliteInt_h::ExprList,
                    (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            pList_3 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                pList_3 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            let ref mut fresh101 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh101 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_BETWEEN,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            if !(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590
                .is_null()
            {
                let ref mut fresh102 = (*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .x
                    .pList;
                *fresh102 = pList_3;
            } else {
                crate::src::src::expr::sqlite3ExprListDelete(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    pList_3 as *mut crate::src::headers::sqliteInt_h::ExprList,
                );
            }
            if (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy502
                != 0
            {
                let ref mut fresh103 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
                *fresh103 = crate::src::src::expr::sqlite3PExpr(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    crate::src::parse::TK_NOT,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::Expr;
            }
        }
        223 => {
            if (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy402
                .is_null()
            {
                let mut pB: *mut crate::src::headers::sqliteInt_h::Expr =
                    crate::src::src::expr::sqlite3Expr(
                        (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        crate::src::parse::TK_STRING,
                        if (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy502
                            != 0
                        {
                            b"true\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"false\0" as *const u8 as *const ::core::ffi::c_char
                        },
                    ) as *mut crate::src::headers::sqliteInt_h::Expr;
                if !pB.is_null() {
                    crate::src::src::expr::sqlite3ExprIdToTrueFalse(
                        pB as *mut crate::src::headers::sqliteInt_h::Expr,
                    );
                }
                if !((*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .flags
                    & 0x8 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::u32_0
                    != 0 as crate::src::ext::rtree::rtree::u32_0)
                {
                    crate::src::src::expr::sqlite3ExprUnmapAndDelete(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                    );
                    let ref mut fresh104 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590;
                    *fresh104 = pB;
                } else {
                    let ref mut fresh105 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590;
                    *fresh105 = crate::src::src::expr::sqlite3PExpr(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        if (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy502
                            != 0
                        {
                            crate::src::parse::TK_OR
                        } else {
                            crate::src::parse::TK_AND
                        },
                        pB as *mut crate::src::headers::sqliteInt_h::Expr,
                        (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                    )
                        as *mut crate::src::headers::sqliteInt_h::Expr;
                }
            } else {
                let mut pRHS: *mut crate::src::headers::sqliteInt_h::Expr = (*(&raw mut (*(*yymsp
                    .offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy402)
                    .a
                    as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                    .offset(0 as isize))
                .pExpr;
                if (*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402)
                    .nExpr
                    == 1 as ::core::ffi::c_int
                    && crate::src::src::expr::sqlite3ExprIsConstant(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        pRHS as *mut crate::src::headers::sqliteInt_h::Expr,
                    ) != 0
                    && (*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590)
                        .op as ::core::ffi::c_int
                        != crate::src::parse::TK_VECTOR
                {
                    let ref mut fresh106 = (*(&raw mut (*(*yymsp
                        .offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402)
                        .a
                        as *mut crate::src::headers::sqliteInt_h::ExprList_item)
                        .offset(0 as isize))
                    .pExpr;
                    *fresh106 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
                    crate::src::src::expr::sqlite3ExprListDelete(
                        (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy402
                            as *mut crate::src::headers::sqliteInt_h::ExprList,
                    );
                    pRHS = crate::src::src::expr::sqlite3PExpr(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        crate::src::parse::TK_UPLUS,
                        pRHS as *mut crate::src::headers::sqliteInt_h::Expr,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                    ) as *mut crate::src::headers::sqliteInt_h::Expr;
                    let ref mut fresh107 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590;
                    *fresh107 = crate::src::src::expr::sqlite3PExpr(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        crate::src::parse::TK_EQ,
                        (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                        pRHS as *mut crate::src::headers::sqliteInt_h::Expr,
                    )
                        as *mut crate::src::headers::sqliteInt_h::Expr;
                } else if (*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402)
                    .nExpr
                    == 1 as ::core::ffi::c_int
                    && (*pRHS).op as ::core::ffi::c_int == crate::src::parse::TK_SELECT
                {
                    let ref mut fresh108 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590;
                    *fresh108 = crate::src::src::expr::sqlite3PExpr(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        crate::src::parse::TK_IN,
                        (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                    )
                        as *mut crate::src::headers::sqliteInt_h::Expr;
                    crate::src::src::expr::sqlite3PExprAddSelect(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                        (*pRHS).x.pSelect as *mut crate::src::headers::sqliteInt_h::Select,
                    );
                    (*pRHS).x.pSelect =
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Select>();
                    crate::src::src::expr::sqlite3ExprListDelete(
                        (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                        (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy402
                            as *mut crate::src::headers::sqliteInt_h::ExprList,
                    );
                } else {
                    let ref mut fresh109 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590;
                    *fresh109 = crate::src::src::expr::sqlite3PExpr(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        crate::src::parse::TK_IN,
                        (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                    )
                        as *mut crate::src::headers::sqliteInt_h::Expr;
                    if (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590
                        .is_null()
                    {
                        crate::src::src::expr::sqlite3ExprListDelete(
                            (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                                .minor
                                .yy402
                                as *mut crate::src::headers::sqliteInt_h::ExprList,
                        );
                    } else if (*(*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590)
                        .pLeft)
                        .op as ::core::ffi::c_int
                        == crate::src::parse::TK_VECTOR
                    {
                        let mut nExpr: ::core::ffi::c_int = (*(*(*(*yymsp
                            .offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590)
                            .pLeft)
                            .x
                            .pList)
                            .nExpr;
                        let mut pSelectRHS: *mut crate::src::headers::sqliteInt_h::Select =
                            crate::src::src::expr::sqlite3ExprListToValues(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                nExpr,
                                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                                    .minor
                                    .yy402
                                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                            )
                                as *mut crate::src::headers::sqliteInt_h::Select;
                        if !pSelectRHS.is_null() {
                            parserDoubleLinkSelect(pParse, pSelectRHS);
                            crate::src::src::expr::sqlite3PExprAddSelect(
                                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                                    .minor
                                    .yy590
                                    as *mut crate::src::headers::sqliteInt_h::Expr,
                                pSelectRHS as *mut crate::src::headers::sqliteInt_h::Select,
                            );
                        }
                    } else {
                        let ref mut fresh110 = (*(*yymsp
                            .offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590)
                            .x
                            .pList;
                        *fresh110 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy402;
                        crate::src::src::expr::sqlite3ExprSetHeightAndFlags(
                            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                            (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                                .minor
                                .yy590
                                as *mut crate::src::headers::sqliteInt_h::Expr,
                        );
                    }
                }
                if (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502
                    != 0
                {
                    let ref mut fresh111 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590;
                    *fresh111 = crate::src::src::expr::sqlite3PExpr(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        crate::src::parse::TK_NOT,
                        (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                    )
                        as *mut crate::src::headers::sqliteInt_h::Expr;
                }
            }
        }
        224 => {
            let ref mut fresh112 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh112 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_SELECT,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            crate::src::src::expr::sqlite3PExprAddSelect(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637 as *mut crate::src::headers::sqliteInt_h::Select,
            );
        }
        225 => {
            let ref mut fresh113 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh113 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_IN,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            crate::src::src::expr::sqlite3PExprAddSelect(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637 as *mut crate::src::headers::sqliteInt_h::Select,
            );
            if (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy502
                != 0
            {
                let ref mut fresh114 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
                *fresh114 = crate::src::src::expr::sqlite3PExpr(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    crate::src::parse::TK_NOT,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::Expr;
            }
        }
        226 => {
            let mut pSrc: *mut crate::src::headers::sqliteInt_h::SrcList =
                crate::src::src::build::sqlite3SrcListAppend(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>()
                        as *mut crate::src::headers::sqliteInt_h::SrcList,
                    &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0 as *mut _
                        as *mut crate::src::headers::sqliteInt_h::Token,
                    &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0 as *mut _
                        as *mut crate::src::headers::sqliteInt_h::Token,
                ) as *mut crate::src::headers::sqliteInt_h::SrcList;
            let mut pSelect: *mut crate::src::headers::sqliteInt_h::Select =
                crate::src::src::select::sqlite3SelectNew(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                        as *mut crate::src::headers::sqliteInt_h::ExprList,
                    pSrc as *mut crate::src::headers::sqliteInt_h::SrcList,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                        as *mut crate::src::headers::sqliteInt_h::ExprList,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                        as *mut crate::src::headers::sqliteInt_h::ExprList,
                    0 as crate::src::ext::rtree::rtree::u32_0,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::Select;
            if !(*yymsp.offset(0 as isize)).minor.yy402.is_null() {
                crate::src::src::build::sqlite3SrcListFuncArgs(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    if !pSelect.is_null() {
                        pSrc
                    } else {
                        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>()
                    } as *mut crate::src::headers::sqliteInt_h::SrcList,
                    (*yymsp.offset(0 as isize)).minor.yy402
                        as *mut crate::src::headers::sqliteInt_h::ExprList,
                );
            }
            let ref mut fresh115 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh115 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_IN,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            crate::src::src::expr::sqlite3PExprAddSelect(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                pSelect as *mut crate::src::headers::sqliteInt_h::Select,
            );
            if (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy502
                != 0
            {
                let ref mut fresh116 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
                *fresh116 = crate::src::src::expr::sqlite3PExpr(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    crate::src::parse::TK_NOT,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                ) as *mut crate::src::headers::sqliteInt_h::Expr;
            }
        }
        227 => {
            let mut p_4: *mut crate::src::headers::sqliteInt_h::Expr =
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
            let ref mut fresh117 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh117 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_EXISTS,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            p_4 = *fresh117;
            crate::src::src::expr::sqlite3PExprAddSelect(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                p_4 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637 as *mut crate::src::headers::sqliteInt_h::Select,
            );
        }
        228 => {
            let ref mut fresh118 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh118 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_CASE,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            if !(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590
                .is_null()
            {
                let ref mut fresh119 = (*(*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .x
                    .pList;
                *fresh119 = if !(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590
                    .is_null()
                {
                    crate::src::src::expr::sqlite3ExprListAppend(
                        pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                        (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy402
                            as *mut crate::src::headers::sqliteInt_h::ExprList,
                        (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                            .minor
                            .yy590
                            as *mut crate::src::headers::sqliteInt_h::Expr,
                    ) as *mut crate::src::headers::sqliteInt_h::ExprList
                } else {
                    (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy402
                };
                crate::src::src::expr::sqlite3ExprSetHeightAndFlags(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                );
            } else {
                crate::src::src::expr::sqlite3ExprListDelete(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy402
                        as *mut crate::src::headers::sqliteInt_h::ExprList,
                );
                crate::src::src::expr::sqlite3ExprDelete(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                );
            }
        }
        229 => {
            let ref mut fresh120 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh120 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            let ref mut fresh121 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh121 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
        }
        230 => {
            let ref mut fresh122 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh122 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
            let ref mut fresh123 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh123 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
        }
        235 => {
            let ref mut fresh124 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh124 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
        }
        236 => {
            let ref mut fresh125 = (*yymsp.offset(0 as isize)).minor.yy402;
            *fresh125 = crate::src::src::expr::sqlite3ExprListAppend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::ExprList;
        }
        238 | 243 => {
            let ref mut fresh126 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh126 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
        }
        239 => {
            crate::src::src::build::sqlite3CreateIndex(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                crate::src::src::build::sqlite3SrcListAppend(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::SrcList>()
                        as *mut crate::src::headers::sqliteInt_h::SrcList,
                    &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0 as *mut _
                        as *mut crate::src::headers::sqliteInt_h::Token,
                    ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                        as *mut crate::src::headers::sqliteInt_h::Token,
                ) as *mut crate::src::headers::sqliteInt_h::SrcList
                    as *mut crate::src::headers::sqliteInt_h::SrcList,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(10 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                &raw mut (*yymsp.offset(-(11 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                crate::src::headers::sqliteInt_h::SQLITE_SO_ASC,
                (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                crate::src::headers::sqliteInt_h::SQLITE_IDXTYPE_APPDEF
                    as crate::src::ext::rtree::rtree::u8_0,
            );
            if (*pParse).eParseMode as ::core::ffi::c_int
                >= crate::src::headers::sqliteInt_h::PARSE_MODE_RENAME
                && !(*pParse).pNewIndex.is_null()
            {
                crate::src::src::alter::sqlite3RenameTokenMap(
                    pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                    (*(*pParse).pNewIndex).zName as *const ::core::ffi::c_void,
                    &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0 as *mut _
                        as *const crate::src::headers::sqliteInt_h::Token,
                );
            }
        }
        240 | 282 => {
            (*yymsp.offset(0 as isize)).minor.yy502 = crate::src::headers::sqliteInt_h::OE_Abort;
        }
        241 => {
            (*yymsp.offset(1 as isize)).minor.yy502 = crate::src::headers::sqliteInt_h::OE_None;
        }
        244 => {
            let ref mut fresh127 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh127 = parserAddExprIdListTerm(
                pParse,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(0 as isize)).minor.yy502,
            );
        }
        245 => {
            let ref mut fresh128 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy402;
            *fresh128 = parserAddExprIdListTerm(
                pParse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>(),
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(0 as isize)).minor.yy502,
            );
        }
        248 => {
            crate::src::src::build::sqlite3DropIndex(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy563
                    as *mut crate::src::headers::sqliteInt_h::SrcList,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        249 => {
            crate::src::src::vacuum::sqlite3Vacuum(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            );
        }
        250 => {
            crate::src::src::vacuum::sqlite3Vacuum(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            );
        }
        253 => {
            crate::src::src::pragma::sqlite3Pragma(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
                0 as ::core::ffi::c_int,
            );
        }
        254 => {
            crate::src::src::pragma::sqlite3Pragma(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
                0 as ::core::ffi::c_int,
            );
        }
        255 => {
            crate::src::src::pragma::sqlite3Pragma(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                0 as ::core::ffi::c_int,
            );
        }
        256 => {
            crate::src::src::pragma::sqlite3Pragma(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
                1 as ::core::ffi::c_int,
            );
        }
        257 => {
            crate::src::src::pragma::sqlite3Pragma(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                1 as ::core::ffi::c_int,
            );
        }
        260 => {
            let mut all: crate::src::headers::sqliteInt_h::Token = unsafe { ::core::mem::zeroed() };
            all.z = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .z;
            all.n = ((*yymsp.offset(0 as isize)).minor.yy0.z.offset_from(
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
            ) as ::core::ffi::c_long as ::core::ffi::c_int
                as ::core::ffi::c_uint)
                .wrapping_add((*yymsp.offset(0 as isize)).minor.yy0.n);
            crate::src::src::trigger::sqlite3FinishTrigger(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy319 as *mut crate::src::headers::sqliteInt_h::TriggerStep,
                &raw mut all as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        261 => {
            crate::src::src::trigger::sqlite3BeginTrigger(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy28
                    .a,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy28
                    .b as *mut crate::src::headers::sqliteInt_h::IdList,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(10 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
            (*yymsp.offset(-(10 as ::core::ffi::c_int) as isize))
                .minor
                .yy0 = if (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n
                == 0 as ::core::ffi::c_uint
            {
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
            } else {
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
            };
        }
        262 => {
            (*yymsp.offset(0 as isize)).minor.yy502 =
                (*yymsp.offset(0 as isize)).major as ::core::ffi::c_int;
        }
        263 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy502 = crate::src::parse::TK_INSTEAD;
        }
        264 => {
            (*yymsp.offset(1 as isize)).minor.yy502 = crate::src::parse::TK_BEFORE;
        }
        265 | 266 => {
            (*yymsp.offset(0 as isize)).minor.yy28.a =
                (*yymsp.offset(0 as isize)).major as ::core::ffi::c_int;
            let ref mut fresh129 = (*yymsp.offset(0 as isize)).minor.yy28.b;
            *fresh129 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::IdList>();
        }
        267 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy28
                .a = crate::src::parse::TK_UPDATE;
            let ref mut fresh130 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy28
                .b;
            *fresh130 = (*yymsp.offset(0 as isize)).minor.yy204;
        }
        268 | 287 => {
            let ref mut fresh131 = (*yymsp.offset(1 as isize)).minor.yy590;
            *fresh131 = ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
        }
        269 | 288 => {
            let ref mut fresh132 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh132 = (*yymsp.offset(0 as isize)).minor.yy590;
        }
        270 => {
            let ref mut fresh133 = (*(*(*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy319)
                .pLast)
                .pNext;
            *fresh133 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy319;
            let ref mut fresh134 = (*(*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy319)
                .pLast;
            *fresh134 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy319;
        }
        271 => {
            let ref mut fresh135 = (*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy319)
                .pLast;
            *fresh135 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy319;
        }
        272 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy0 = (*yymsp.offset(0 as isize)).minor.yy0;
            crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                b"qualified table names are not allowed on INSERT, UPDATE, and DELETE statements within triggers\0"
                    as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        273 => {
            crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                b"the INDEXED BY clause is not allowed on UPDATE or DELETE statements within triggers\0"
                    as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        274 => {
            crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                b"the NOT INDEXED clause is not allowed on UPDATE or DELETE statements within triggers\0"
                    as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        275 => {
            yylhsminor.yy319 = crate::src::src::trigger::sqlite3TriggerUpdateStep(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502 as crate::src::ext::rtree::rtree::u8_0,
                (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
                (*yymsp.offset(0 as isize)).minor.yy342,
            ) as *mut crate::src::headers::sqliteInt_h::TriggerStep;
            let ref mut fresh136 = (*yymsp.offset(-(8 as ::core::ffi::c_int) as isize))
                .minor
                .yy319;
            *fresh136 = yylhsminor.yy319;
        }
        276 => {
            yylhsminor.yy319 = crate::src::src::trigger::sqlite3TriggerInsertStep(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy204 as *mut crate::src::headers::sqliteInt_h::IdList,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637 as *mut crate::src::headers::sqliteInt_h::Select,
                (*yymsp.offset(-(6 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502 as crate::src::ext::rtree::rtree::u8_0,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy403 as *mut crate::src::headers::sqliteInt_h::Upsert,
                (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy342,
                (*yymsp.offset(0 as isize)).minor.yy342,
            ) as *mut crate::src::headers::sqliteInt_h::TriggerStep;
            let ref mut fresh137 = (*yymsp.offset(-(7 as ::core::ffi::c_int) as isize))
                .minor
                .yy319;
            *fresh137 = yylhsminor.yy319;
        }
        277 => {
            yylhsminor.yy319 = crate::src::src::trigger::sqlite3TriggerDeleteStep(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
                (*yymsp.offset(0 as isize)).minor.yy342,
            ) as *mut crate::src::headers::sqliteInt_h::TriggerStep;
            let ref mut fresh138 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy319;
            *fresh138 = yylhsminor.yy319;
        }
        278 => {
            yylhsminor.yy319 = crate::src::src::trigger::sqlite3TriggerSelectStep(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637 as *mut crate::src::headers::sqliteInt_h::Select,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy342,
                (*yymsp.offset(0 as isize)).minor.yy342,
            ) as *mut crate::src::headers::sqliteInt_h::TriggerStep;
            let ref mut fresh139 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy319;
            *fresh139 = yylhsminor.yy319;
        }
        279 => {
            let ref mut fresh140 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh140 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_RAISE,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            if !(*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy590
                .is_null()
            {
                (*(*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .affExpr = crate::src::headers::sqliteInt_h::OE_Ignore as ::core::ffi::c_char;
            }
        }
        280 => {
            let ref mut fresh141 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh141 = crate::src::src::expr::sqlite3PExpr(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                crate::src::parse::TK_RAISE,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            ) as *mut crate::src::headers::sqliteInt_h::Expr;
            if !(*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy590
                .is_null()
            {
                (*(*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590)
                    .affExpr = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502 as ::core::ffi::c_char;
            }
        }
        281 => {
            (*yymsp.offset(0 as isize)).minor.yy502 = crate::src::headers::sqliteInt_h::OE_Rollback;
        }
        283 => {
            (*yymsp.offset(0 as isize)).minor.yy502 = crate::src::headers::sqliteInt_h::OE_Fail;
        }
        284 => {
            crate::src::src::trigger::sqlite3DropTrigger(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy563
                    as *mut crate::src::headers::sqliteInt_h::SrcList,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        285 => {
            crate::src::src::attach::sqlite3Attach(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            );
        }
        286 => {
            crate::src::src::attach::sqlite3Detach(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy590
                    as *mut crate::src::headers::sqliteInt_h::Expr,
            );
        }
        289 => {
            crate::src::src::build::sqlite3Reindex(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        290 => {
            crate::src::src::build::sqlite3Reindex(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        291 => {
            crate::src::src::analyze::sqlite3Analyze(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        292 => {
            crate::src::src::analyze::sqlite3Analyze(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        293 => {
            crate::src::src::alter::sqlite3AlterRenameTable(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        294 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy0
                .n = ((*pParse).sLastToken.z.offset_from(
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0
                    .z,
            ) as ::core::ffi::c_long as ::core::ffi::c_int
                as ::core::ffi::c_uint)
                .wrapping_add((*pParse).sLastToken.n);
            crate::src::src::alter::sqlite3AlterFinishAddColumn(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        295 => {
            crate::src::src::alter::sqlite3AlterDropColumn(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *const crate::src::headers::sqliteInt_h::Token,
            );
        }
        296 => {
            disableLookaside(pParse);
            crate::src::src::alter::sqlite3AlterBeginAddColumn(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy563
                    as *mut crate::src::headers::sqliteInt_h::SrcList,
            );
        }
        297 => {
            crate::src::src::alter::sqlite3AlterRenameColumn(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy563 as *mut crate::src::headers::sqliteInt_h::SrcList,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        298 => {
            crate::src::src::vtab::sqlite3VtabFinishParse(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        299 => {
            crate::src::src::vtab::sqlite3VtabFinishParse(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        300 => {
            crate::src::src::vtab::sqlite3VtabBeginParse(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
            );
        }
        301 => {
            crate::src::src::vtab::sqlite3VtabArgInit(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            );
        }
        302 | 303 | 304 => {
            crate::src::src::vtab::sqlite3VtabArgExtend(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(0 as isize)).minor.yy0 as *mut _
                    as *mut crate::src::headers::sqliteInt_h::Token,
            );
        }
        305 | 306 => {
            crate::src::src::select::sqlite3WithPush(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy125
                    as *mut crate::src::headers::sqliteInt_h::With,
                1 as crate::src::ext::rtree::rtree::u8_0,
            ) as *mut crate::src::headers::sqliteInt_h::With;
        }
        307 => {
            (*yymsp.offset(0 as isize)).minor.yy444 =
                crate::src::headers::sqliteInt_h::M10d_Any as crate::src::ext::rtree::rtree::u8_0;
        }
        308 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy444 =
                crate::src::headers::sqliteInt_h::M10d_Yes as crate::src::ext::rtree::rtree::u8_0;
        }
        309 => {
            (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy444 =
                crate::src::headers::sqliteInt_h::M10d_No as crate::src::ext::rtree::rtree::u8_0;
        }
        310 => {
            let ref mut fresh142 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy361;
            *fresh142 = crate::src::src::build::sqlite3CteNew(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                &raw mut (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
                (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy637 as *mut crate::src::headers::sqliteInt_h::Select,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy444,
            ) as *mut crate::src::headers::sqliteInt_h::Cte;
        }
        311 => {
            (*pParse).set_bHasWith(
                1 as crate::src::headers::sqliteInt_h::bft as crate::src::headers::sqliteInt_h::bft,
            );
        }
        312 => {
            let ref mut fresh143 = (*yymsp.offset(0 as isize)).minor.yy125;
            *fresh143 = crate::src::src::build::sqlite3WithAdd(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::With>()
                    as *mut crate::src::headers::sqliteInt_h::With,
                (*yymsp.offset(0 as isize)).minor.yy361
                    as *mut crate::src::headers::sqliteInt_h::Cte,
            ) as *mut crate::src::headers::sqliteInt_h::With;
        }
        313 => {
            let ref mut fresh144 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy125;
            *fresh144 = crate::src::src::build::sqlite3WithAdd(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy125 as *mut crate::src::headers::sqliteInt_h::With,
                (*yymsp.offset(0 as isize)).minor.yy361
                    as *mut crate::src::headers::sqliteInt_h::Cte,
            ) as *mut crate::src::headers::sqliteInt_h::With;
        }
        314 => {
            crate::src::src::window::sqlite3WindowChain(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy483
                    as *mut crate::src::headers::sqliteInt_h::Window,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy483 as *mut crate::src::headers::sqliteInt_h::Window,
            );
            let ref mut fresh145 = (*(*yymsp.offset(0 as isize)).minor.yy483).pNextWin;
            *fresh145 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            yylhsminor.yy483 = (*yymsp.offset(0 as isize)).minor.yy483;
            let ref mut fresh146 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh146 = yylhsminor.yy483;
        }
        315 => {
            if !(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483
                .is_null()
            {
                let ref mut fresh147 = (*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy483)
                    .zName;
                *fresh147 = crate::src::src::malloc::sqlite3DbStrNDup(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0
                        .z,
                    (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy0
                        .n as crate::src::ext::rtree::rtree::u64_0,
                );
            }
            yylhsminor.yy483 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            let ref mut fresh148 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh148 = yylhsminor.yy483;
        }
        316 => {
            let ref mut fresh149 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh149 = crate::src::src::window::sqlite3WindowAssemble(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy483
                    as *mut crate::src::headers::sqliteInt_h::Window,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
            ) as *mut crate::src::headers::sqliteInt_h::Window;
        }
        317 => {
            yylhsminor.yy483 = crate::src::src::window::sqlite3WindowAssemble(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy483
                    as *mut crate::src::headers::sqliteInt_h::Window,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
            ) as *mut crate::src::headers::sqliteInt_h::Window;
            let ref mut fresh150 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh150 = yylhsminor.yy483;
        }
        318 => {
            let ref mut fresh151 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh151 = crate::src::src::window::sqlite3WindowAssemble(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy483
                    as *mut crate::src::headers::sqliteInt_h::Window,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Token>()
                    as *mut crate::src::headers::sqliteInt_h::Token,
            ) as *mut crate::src::headers::sqliteInt_h::Window;
        }
        319 => {
            yylhsminor.yy483 = crate::src::src::window::sqlite3WindowAssemble(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy483
                    as *mut crate::src::headers::sqliteInt_h::Window,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy402 as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
            ) as *mut crate::src::headers::sqliteInt_h::Window;
            let ref mut fresh152 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh152 = yylhsminor.yy483;
        }
        320 => {
            yylhsminor.yy483 = crate::src::src::window::sqlite3WindowAssemble(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(0 as isize)).minor.yy483
                    as *mut crate::src::headers::sqliteInt_h::Window,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::ExprList>()
                    as *mut crate::src::headers::sqliteInt_h::ExprList,
                &raw mut (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy0 as *mut _ as *mut crate::src::headers::sqliteInt_h::Token,
            ) as *mut crate::src::headers::sqliteInt_h::Window;
            let ref mut fresh153 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh153 = yylhsminor.yy483;
        }
        321 => {
            let ref mut fresh154 = (*yymsp.offset(1 as isize)).minor.yy483;
            *fresh154 = crate::src::src::window::sqlite3WindowAlloc(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                0 as ::core::ffi::c_int,
                crate::src::parse::TK_UNBOUNDED,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                crate::src::parse::TK_CURRENT,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                0 as crate::src::ext::rtree::rtree::u8_0,
            ) as *mut crate::src::headers::sqliteInt_h::Window;
        }
        322 => {
            yylhsminor.yy483 = crate::src::src::window::sqlite3WindowAlloc(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy205
                    .eType,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy205
                    .pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
                crate::src::parse::TK_CURRENT,
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>()
                    as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy444,
            ) as *mut crate::src::headers::sqliteInt_h::Window;
            let ref mut fresh155 = (*yymsp.offset(-(2 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh155 = yylhsminor.yy483;
        }
        323 => {
            yylhsminor.yy483 = crate::src::src::window::sqlite3WindowAlloc(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy502,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy205
                    .eType,
                (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy205
                    .pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy205
                    .eType,
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy205
                    .pExpr as *mut crate::src::headers::sqliteInt_h::Expr,
                (*yymsp.offset(0 as isize)).minor.yy444,
            ) as *mut crate::src::headers::sqliteInt_h::Window;
            let ref mut fresh156 = (*yymsp.offset(-(5 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh156 = yylhsminor.yy483;
        }
        325 | 327 => {
            yylhsminor.yy205 = (*yymsp.offset(0 as isize)).minor.yy205;
            (*yymsp.offset(0 as isize)).minor.yy205 = yylhsminor.yy205;
        }
        326 | 328 | 330 => {
            yylhsminor.yy205.eType =
                (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize)).major as ::core::ffi::c_int;
            yylhsminor.yy205.pExpr =
                ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::Expr>();
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy205 = yylhsminor.yy205;
        }
        329 => {
            yylhsminor.yy205.eType = (*yymsp.offset(0 as isize)).major as ::core::ffi::c_int;
            yylhsminor.yy205.pExpr = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy205 = yylhsminor.yy205;
        }
        331 => {
            (*yymsp.offset(1 as isize)).minor.yy444 = 0 as crate::src::ext::rtree::rtree::u8_0;
        }
        332 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy444 = (*yymsp.offset(0 as isize)).minor.yy444;
        }
        333 | 334 => {
            (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy444 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize)).major
                as crate::src::ext::rtree::rtree::u8_0;
        }
        335 => {
            (*yymsp.offset(0 as isize)).minor.yy444 =
                (*yymsp.offset(0 as isize)).major as crate::src::ext::rtree::rtree::u8_0;
        }
        336 => {
            let ref mut fresh157 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh157 = (*yymsp.offset(0 as isize)).minor.yy483;
        }
        337 => {
            if !(*yymsp.offset(0 as isize)).minor.yy483.is_null() {
                let ref mut fresh158 = (*(*yymsp.offset(0 as isize)).minor.yy483).pFilter;
                *fresh158 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy590;
            } else {
                crate::src::src::expr::sqlite3ExprDelete(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                        .minor
                        .yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
                );
            }
            yylhsminor.yy483 = (*yymsp.offset(0 as isize)).minor.yy483;
            let ref mut fresh159 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh159 = yylhsminor.yy483;
        }
        338 => {
            yylhsminor.yy483 = (*yymsp.offset(0 as isize)).minor.yy483;
            let ref mut fresh160 = (*yymsp.offset(0 as isize)).minor.yy483;
            *fresh160 = yylhsminor.yy483;
        }
        339 => {
            yylhsminor.yy483 = crate::src::src::malloc::sqlite3DbMallocZero(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Window>()
                    as crate::src::ext::rtree::rtree::u64_0,
            ) as *mut crate::src::headers::sqliteInt_h::Window;
            if !yylhsminor.yy483.is_null() {
                (*yylhsminor.yy483).eFrmType =
                    crate::src::parse::TK_FILTER as crate::src::ext::rtree::rtree::u8_0;
                (*yylhsminor.yy483).pFilter = (*yymsp.offset(0 as isize)).minor.yy590;
            } else {
                crate::src::src::expr::sqlite3ExprDelete(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    (*yymsp.offset(0 as isize)).minor.yy590
                        as *mut crate::src::headers::sqliteInt_h::Expr,
                );
            }
            let ref mut fresh161 = (*yymsp.offset(0 as isize)).minor.yy483;
            *fresh161 = yylhsminor.yy483;
        }
        340 => {
            let ref mut fresh162 = (*yymsp.offset(-(3 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh162 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
        }
        341 => {
            let ref mut fresh163 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483;
            *fresh163 = crate::src::src::malloc::sqlite3DbMallocZero(
                (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                ::core::mem::size_of::<crate::src::headers::sqliteInt_h::Window>()
                    as crate::src::ext::rtree::rtree::u64_0,
            ) as *mut crate::src::headers::sqliteInt_h::Window;
            if !(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy483
                .is_null()
            {
                let ref mut fresh164 = (*(*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                    .minor
                    .yy483)
                    .zName;
                *fresh164 = crate::src::src::malloc::sqlite3DbStrNDup(
                    (*pParse).db as *mut crate::src::headers::sqliteInt_h::sqlite3,
                    (*yymsp.offset(0 as isize)).minor.yy0.z,
                    (*yymsp.offset(0 as isize)).minor.yy0.n as crate::src::ext::rtree::rtree::u64_0,
                );
            }
        }
        342 => {
            let ref mut fresh165 = (*yymsp.offset(-(4 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
            *fresh165 = (*yymsp.offset(-(1 as ::core::ffi::c_int) as isize))
                .minor
                .yy590;
        }
        343 => {
            yylhsminor.yy590 = tokenExpr(
                pParse,
                (*yymsp.offset(0 as isize)).major as ::core::ffi::c_int,
                (*yymsp.offset(0 as isize)).minor.yy0,
            );
            crate::src::src::util::sqlite3DequoteNumber(
                pParse as *mut crate::src::headers::sqliteInt_h::Parse,
                yylhsminor.yy590 as *mut crate::src::headers::sqliteInt_h::Expr,
            );
            let ref mut fresh166 = (*yymsp.offset(0 as isize)).minor.yy590;
            *fresh166 = yylhsminor.yy590;
        }
        _ => {}
    }
    yygoto = yyRuleInfoLhs[yyruleno as usize] as ::core::ffi::c_int;
    yysize = yyRuleInfoNRhs[yyruleno as usize] as ::core::ffi::c_int;
    yyact = yy_find_reduce_action(
        (*yymsp.offset(yysize as isize)).stateno,
        yygoto as ::core::ffi::c_ushort,
    );
    yymsp = yymsp.offset((yysize + 1 as ::core::ffi::c_int) as isize);
    (*yypParser).yytos = yymsp;
    (*yymsp).stateno = yyact;
    (*yymsp).major = yygoto as ::core::ffi::c_ushort;
    yyact
}

unsafe extern "C" fn yy_syntax_error(
    mut yypParser: *mut yyParser,
    mut _yymajor: ::core::ffi::c_int,
    mut yyminor: crate::src::headers::sqliteInt_h::Token,
) {
    let mut pParse: *mut crate::src::headers::sqliteInt_h::Parse = (*yypParser).pParse;
    if *yyminor.z.offset(0 as isize) != 0 {
        parserSyntaxError(pParse, &raw mut yyminor);
    } else {
        crate::src::printf_c_variadic::sqlite3ErrorMsg_args(
            pParse as *mut crate::src::headers::sqliteInt_h::Parse,
            b"incomplete input\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    }
    (*yypParser).pParse = pParse;
}

unsafe extern "C" fn yy_accept(mut yypParser: *mut yyParser) {
    let mut pParse: *mut crate::src::headers::sqliteInt_h::Parse = (*yypParser).pParse;
    (*yypParser).pParse = pParse;
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3Parser(
    mut yyp: *mut ::core::ffi::c_void,
    mut yymajor: ::core::ffi::c_int,
    mut yyminor: crate::src::headers::sqliteInt_h::Token,
) {
    let mut yyminorunion: YYMINORTYPE = unsafe { ::core::mem::zeroed() };
    let mut yyact: ::core::ffi::c_ushort = 0;
    let mut yypParser: *mut yyParser = yyp as *mut yyParser;
    let mut pParse: *mut crate::src::headers::sqliteInt_h::Parse = (*yypParser).pParse;
    yyact = (*(*yypParser).yytos).stateno;
    loop {
        yyact = yy_find_shift_action(yymajor as ::core::ffi::c_ushort, yyact);
        if yyact as ::core::ffi::c_int >= YY_MIN_REDUCE {
            let mut yyruleno: ::core::ffi::c_uint =
                (yyact as ::core::ffi::c_int - YY_MIN_REDUCE) as ::core::ffi::c_uint;
            if yyRuleInfoNRhs[yyruleno as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if (*yypParser).yytos >= (*yypParser).yystackEnd {
                    if yyGrowStack(yypParser) != 0 {
                        yyStackOverflow(yypParser);
                        break;
                    }
                }
            }
            yyact = yy_reduce(yypParser, yyruleno, yymajor, yyminor, pParse);
        } else if yyact as ::core::ffi::c_int <= YY_MAX_SHIFTREDUCE {
            yy_shift(yypParser, yyact, yymajor as ::core::ffi::c_ushort, yyminor);
            break;
        } else if yyact as ::core::ffi::c_int == YY_ACCEPT_ACTION {
            (*yypParser).yytos = (*yypParser).yytos.offset(-1);
            yy_accept(yypParser);
            return;
        } else {
            yyminorunion.yy0 = yyminor;
            yy_syntax_error(yypParser, yymajor, yyminor);
            yy_destructor(
                yypParser,
                yymajor as ::core::ffi::c_ushort,
                &raw mut yyminorunion,
            );
            break;
        }
    }
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3ParserFallback(
    mut iToken: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    yyFallback[iToken as usize] as ::core::ffi::c_int
}
