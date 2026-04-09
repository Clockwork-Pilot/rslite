//! Raw bindings to the rslite SQLite implementation.
//!
//! Exposes the full public `sqlite3_*` C API as unsafe Rust functions,
//! backed by the pure-Rust `rslite-core` translation. No external C library
//! is required.
//!
//! This crate is the equivalent of `libsqlite3-sys` for the rslite project.
//! Use it when you need direct, zero-overhead access to the SQLite C API.
//! For a safe, ergonomic interface see the `rslite` crate instead.
//!
//! # Platform support
//! Supported: **Linux, macOS**
//! Not supported: Windows (`sqlite3_win32_*` functions are absent).
//!
//! # Example
//! ```ignore
//! use rslite_raw::*;
//!
//! let mut db = std::ptr::null_mut();
//! let rc = unsafe { sqlite3_open(c"test.db".as_ptr(), &mut db) };
//! if rc == SQLITE_OK {
//!     unsafe { sqlite3_close(db) };
//! }
//! ```

use sqlite_noamalgam as core;

// ── Types ────────────────────────────────────────────────────────────────────

pub use core::src::headers::sqlite3_h::sqlite3_stmt;
pub use core::src::headers::vdbeInt_h::sqlite3_value;
pub use core::src::headers::vdbeInt_h::sqlite3_context;
pub use core::src::headers::sqlite3_h::sqlite3_vfs;
pub use core::src::headers::sqlite3_h::sqlite3_file;
pub use core::src::headers::sqlite3_h::sqlite3_filename;
pub use core::src::headers::sqlite3_h::sqlite3_io_methods;
pub use core::src::headers::sqlite3_h::sqlite3_module;
pub use core::src::headers::sqlite3_h::sqlite3_vtab;
pub use core::src::headers::sqlite3_h::sqlite3_vtab_cursor;
pub use core::src::headers::sqlite3_h::sqlite3_index_info;
pub use core::src::headers::sqlite3_h::sqlite3_index_constraint;
pub use core::src::headers::sqlite3_h::sqlite3_index_constraint_usage;
pub use core::src::headers::sqlite3_h::sqlite3_index_orderby;
pub use core::src::headers::sqlite3_h::sqlite3_mem_methods;
pub use core::src::headers::sqlite3_h::sqlite3_pcache;
pub use core::src::headers::sqlite3_h::sqlite3_pcache_methods2;
pub use core::src::headers::sqlite3_h::sqlite3_pcache_page;
pub use core::src::headers::sqlite3_h::sqlite3_mutex_methods;
pub use core::src::headers::sqlite3_h::sqlite3_int64;
pub use core::src::headers::sqlite3_h::sqlite3_uint64;
pub use core::src::headers::sqlite3_h::sqlite3_destructor_type;
pub use core::src::headers::sqlite3_h::sqlite3_syscall_ptr;
pub use core::src::headers::sqliteInt_h::sqlite3;
pub use core::src::src::mutex_unix::sqlite3_mutex;
pub use core::src::src::backup::sqlite3_backup;

// ── Constants ─────────────────────────────────────────────────────────────────

pub use core::src::headers::sqlite3_h::{
    // Primary result codes
    SQLITE_OK,
    SQLITE_ERROR,
    SQLITE_INTERNAL,
    SQLITE_PERM,
    SQLITE_ABORT,
    SQLITE_BUSY,
    SQLITE_LOCKED,
    SQLITE_NOMEM,
    SQLITE_READONLY,
    SQLITE_INTERRUPT,
    SQLITE_IOERR,
    SQLITE_CORRUPT,
    SQLITE_NOTFOUND,
    SQLITE_FULL,
    SQLITE_CANTOPEN,
    SQLITE_PROTOCOL,
    SQLITE_EMPTY,
    SQLITE_SCHEMA,
    SQLITE_TOOBIG,
    SQLITE_CONSTRAINT,
    SQLITE_MISMATCH,
    SQLITE_MISUSE,
    SQLITE_NOLFS,
    SQLITE_AUTH,
    SQLITE_FORMAT,
    SQLITE_RANGE,
    SQLITE_NOTADB,
    SQLITE_NOTICE,
    SQLITE_WARNING,
    SQLITE_ROW,
    SQLITE_DONE,
    // Extended result codes
    SQLITE_ABORT_ROLLBACK,
    SQLITE_BUSY_RECOVERY,
    SQLITE_BUSY_SNAPSHOT,
    SQLITE_CANTOPEN_CONVPATH,
    SQLITE_CANTOPEN_FULLPATH,
    SQLITE_CANTOPEN_ISDIR,
    SQLITE_CANTOPEN_NOTEMPDIR,
    SQLITE_CANTOPEN_SYMLINK,
    SQLITE_CONSTRAINT_CHECK,
    SQLITE_CONSTRAINT_COMMITHOOK,
    SQLITE_CONSTRAINT_FOREIGNKEY,
    SQLITE_CONSTRAINT_FUNCTION,
    SQLITE_CONSTRAINT_NOTNULL,
    SQLITE_CONSTRAINT_PRIMARYKEY,
    SQLITE_CONSTRAINT_ROWID,
    SQLITE_CONSTRAINT_TRIGGER,
    SQLITE_CONSTRAINT_UNIQUE,
    SQLITE_CONSTRAINT_VTAB,
    SQLITE_CORRUPT_VTAB,
    SQLITE_ERROR_MISSING_COLLSEQ,
    SQLITE_ERROR_RETRY,
    SQLITE_ERROR_SNAPSHOT,
    SQLITE_IOERR_ACCESS,
    SQLITE_IOERR_CHECKRESERVEDLOCK,
    SQLITE_IOERR_CLOSE,
    SQLITE_IOERR_CONVPATH,
    SQLITE_IOERR_DELETE,
    SQLITE_IOERR_DELETE_NOENT,
    SQLITE_IOERR_DIR_CLOSE,
    SQLITE_IOERR_DIR_FSYNC,
    SQLITE_IOERR_FSTAT,
    SQLITE_IOERR_FSYNC,
    SQLITE_IOERR_GETTEMPPATH,
    SQLITE_IOERR_LOCK,
    SQLITE_IOERR_MMAP,
    SQLITE_IOERR_NOMEM,
    SQLITE_IOERR_RDLOCK,
    SQLITE_IOERR_READ,
    SQLITE_IOERR_SEEK,
    SQLITE_IOERR_SHMLOCK,
    SQLITE_IOERR_SHMMAP,
    SQLITE_IOERR_SHMOPEN,
    SQLITE_IOERR_SHMSIZE,
    SQLITE_IOERR_SHORT_READ,
    SQLITE_IOERR_TRUNCATE,
    SQLITE_IOERR_UNLOCK,
    SQLITE_IOERR_WRITE,
    SQLITE_LOCKED_SHAREDCACHE,
    SQLITE_NOTICE_RBU,
    SQLITE_NOTICE_RECOVER_ROLLBACK,
    SQLITE_NOTICE_RECOVER_WAL,
    SQLITE_READONLY_CANTINIT,
    SQLITE_READONLY_DBMOVED,
    SQLITE_READONLY_DIRECTORY,
    SQLITE_READONLY_RECOVERY,
    SQLITE_READONLY_ROLLBACK,
    SQLITE_WARNING_AUTOINDEX,
    // Open flags
    SQLITE_OPEN_READONLY,
    SQLITE_OPEN_READWRITE,
    SQLITE_OPEN_CREATE,
    SQLITE_OPEN_DELETEONCLOSE,
    SQLITE_OPEN_EXCLUSIVE,
    SQLITE_OPEN_URI,
    SQLITE_OPEN_MEMORY,
    SQLITE_OPEN_MAIN_DB,
    SQLITE_OPEN_TEMP_DB,
    SQLITE_OPEN_TRANSIENT_DB,
    SQLITE_OPEN_MAIN_JOURNAL,
    SQLITE_OPEN_TEMP_JOURNAL,
    SQLITE_OPEN_SUBJOURNAL,
    SQLITE_OPEN_SUPER_JOURNAL,
    SQLITE_OPEN_NOMUTEX,
    SQLITE_OPEN_FULLMUTEX,
    SQLITE_OPEN_SHAREDCACHE,
    SQLITE_OPEN_PRIVATECACHE,
    SQLITE_OPEN_WAL,
    SQLITE_OPEN_NOFOLLOW,
    SQLITE_OPEN_EXRESCODE,
    // Text encodings
    SQLITE_UTF8,
    SQLITE_UTF16LE,
    SQLITE_UTF16BE,
    SQLITE_UTF16,
    SQLITE_UTF16_ALIGNED,
    // Function flags
    SQLITE_DETERMINISTIC,
    SQLITE_DIRECTONLY,
    SQLITE_SUBTYPE,
    SQLITE_INNOCUOUS,
    SQLITE_RESULT_SUBTYPE,
    SQLITE_SELFORDER1,
    // Destructor constants
    SQLITE_STATIC,
    // Checkpoint modes
    SQLITE_CHECKPOINT_PASSIVE,
    SQLITE_CHECKPOINT_FULL,
    SQLITE_CHECKPOINT_RESTART,
    SQLITE_CHECKPOINT_TRUNCATE,
    // Trace event codes
    SQLITE_TRACE_STMT,
    SQLITE_TRACE_PROFILE,
    SQLITE_TRACE_ROW,
    SQLITE_TRACE_CLOSE,
    // Transaction state
    SQLITE_TXN_NONE,
    SQLITE_TXN_WRITE,
    // Mutex types
    SQLITE_MUTEX_FAST,
    SQLITE_MUTEX_RECURSIVE,
    // FCNTL opcodes
    SQLITE_FCNTL_DATA_VERSION,
    SQLITE_FCNTL_FILE_POINTER,
    SQLITE_FCNTL_JOURNAL_POINTER,
    SQLITE_FCNTL_RESERVE_BYTES,
    SQLITE_FCNTL_RESET_CACHE,
    SQLITE_FCNTL_VFS_POINTER,
    // Column/value types
    SQLITE_INTEGER,
    SQLITE_FLOAT,
    SQLITE_TEXT,
    SQLITE_BLOB,
    SQLITE_NULL,
    // Misc
    SQLITE_VERSION,
    SQLITE_VERSION_NUMBER,
    SQLITE_SOURCE_ID,
};

// ── Connection: open / close / config ─────────────────────────────────────────

pub use core::src::src::main::sqlite3_version;

pub use core::src::src::main::{
    sqlite3_open,
    sqlite3_open_v2,
    sqlite3_open16,
    sqlite3_close,
    sqlite3_close_v2,
    sqlite3_initialize,
    sqlite3_shutdown,
    sqlite3_libversion,
    sqlite3_libversion_number,
    sqlite3_sourceid,
    sqlite3_threadsafe,
    sqlite3_db_mutex,
    sqlite3_db_release_memory,
    sqlite3_db_cacheflush,
    sqlite3_db_name,
    sqlite3_db_filename,
    sqlite3_db_readonly,
    sqlite3_txn_state,
    sqlite3_get_autocommit,
    sqlite3_extended_result_codes,
    sqlite3_limit,
    sqlite3_sleep,
    sqlite3_file_control,
    sqlite3_table_column_metadata,
    sqlite3_get_clientdata,
    sqlite3_set_clientdata,
    // Error reporting
    sqlite3_errmsg,
    sqlite3_errmsg16,
    sqlite3_errcode,
    sqlite3_extended_errcode,
    sqlite3_errstr,
    sqlite3_system_errno,
    sqlite3_error_offset,
    sqlite3_set_errmsg,
    // Changes
    sqlite3_changes,
    sqlite3_changes64,
    sqlite3_total_changes,
    sqlite3_total_changes64,
    sqlite3_last_insert_rowid,
    sqlite3_set_last_insert_rowid,
    // Busy handling
    sqlite3_busy_handler,
    sqlite3_busy_timeout,
    sqlite3_setlk_timeout,
    sqlite3_progress_handler,
    // Interrupts
    sqlite3_interrupt,
    sqlite3_is_interrupted,
    // Hooks
    sqlite3_commit_hook,
    sqlite3_rollback_hook,
    sqlite3_update_hook,
    sqlite3_preupdate_hook,
    sqlite3_autovacuum_pages,
    sqlite3_trace,
    sqlite3_trace_v2,
    sqlite3_profile,
    sqlite3_wal_hook,
    sqlite3_wal_autocheckpoint,
    sqlite3_wal_checkpoint,
    sqlite3_wal_checkpoint_v2,
    // Function registration
    sqlite3_create_function,
    sqlite3_create_function_v2,
    sqlite3_create_function16,
    sqlite3_create_window_function,
    sqlite3_overload_function,
    // Collation
    sqlite3_create_collation,
    sqlite3_create_collation_v2,
    sqlite3_create_collation16,
    sqlite3_collation_needed,
    sqlite3_collation_needed16,
    // Filename / URI utilities
    sqlite3_create_filename,
    sqlite3_free_filename,
    sqlite3_uri_parameter,
    sqlite3_uri_key,
    sqlite3_uri_boolean,
    sqlite3_uri_int64,
    sqlite3_filename_database,
    sqlite3_filename_journal,
    sqlite3_filename_wal,
    // Compile options
    sqlite3_compileoption_used,
    sqlite3_compileoption_get,
    // Deprecated / legacy
    sqlite3_global_recover,
    sqlite3_thread_cleanup,
};

pub use core::src::src::auth::sqlite3_set_authorizer;

// ── Statement: prepare / step / finalize ──────────────────────────────────────

pub use core::src::src::prepare::{
    sqlite3_prepare,
    sqlite3_prepare_v2,
    sqlite3_prepare_v3,
    sqlite3_prepare16,
    sqlite3_prepare16_v2,
    sqlite3_prepare16_v3,
};

pub use core::src::src::vdbeapi::{
    sqlite3_step,
    sqlite3_finalize,
    sqlite3_reset,
    sqlite3_clear_bindings,
    sqlite3_expired,
    // Bind
    sqlite3_bind_blob,
    sqlite3_bind_blob64,
    sqlite3_bind_double,
    sqlite3_bind_int,
    sqlite3_bind_int64,
    sqlite3_bind_null,
    sqlite3_bind_pointer,
    sqlite3_bind_text,
    sqlite3_bind_text64,
    sqlite3_bind_text16,
    sqlite3_bind_value,
    sqlite3_bind_zeroblob,
    sqlite3_bind_zeroblob64,
    sqlite3_bind_parameter_count,
    sqlite3_bind_parameter_name,
    sqlite3_bind_parameter_index,
    sqlite3_transfer_bindings,
    // Column
    sqlite3_column_count,
    sqlite3_data_count,
    sqlite3_column_name,
    sqlite3_column_name16,
    sqlite3_column_decltype,
    sqlite3_column_decltype16,
    sqlite3_column_blob,
    sqlite3_column_bytes,
    sqlite3_column_bytes16,
    sqlite3_column_double,
    sqlite3_column_int,
    sqlite3_column_int64,
    sqlite3_column_text,
    sqlite3_column_text16,
    sqlite3_column_type,
    sqlite3_column_value,
    // Statement info
    sqlite3_stmt_readonly,
    sqlite3_stmt_isexplain,
    sqlite3_stmt_explain,
    sqlite3_stmt_busy,
    sqlite3_stmt_status,
    sqlite3_next_stmt,
    sqlite3_sql,
    sqlite3_expanded_sql,
    sqlite3_db_handle,
    // Value accessors
    sqlite3_value_blob,
    sqlite3_value_bytes,
    sqlite3_value_bytes16,
    sqlite3_value_double,
    sqlite3_value_int,
    sqlite3_value_int64,
    sqlite3_value_subtype,
    sqlite3_value_pointer,
    sqlite3_value_text,
    sqlite3_value_text16,
    sqlite3_value_text16be,
    sqlite3_value_text16le,
    sqlite3_value_type,
    sqlite3_value_encoding,
    sqlite3_value_nochange,
    sqlite3_value_frombind,
    sqlite3_value_dup,
    sqlite3_value_free,
    // Result setters
    sqlite3_result_blob,
    sqlite3_result_blob64,
    sqlite3_result_double,
    sqlite3_result_error,
    sqlite3_result_error16,
    sqlite3_result_error_code,
    sqlite3_result_error_nomem,
    sqlite3_result_error_toobig,
    sqlite3_result_int,
    sqlite3_result_int64,
    sqlite3_result_null,
    sqlite3_result_pointer,
    sqlite3_result_subtype,
    sqlite3_result_text,
    sqlite3_result_text64,
    sqlite3_result_text16,
    sqlite3_result_text16be,
    sqlite3_result_text16le,
    sqlite3_result_value,
    sqlite3_result_zeroblob,
    sqlite3_result_zeroblob64,
    // Function context
    sqlite3_user_data,
    sqlite3_context_db_handle,
    sqlite3_aggregate_context,
    sqlite3_aggregate_count,
    sqlite3_get_auxdata,
    sqlite3_set_auxdata,
    // Virtual table cursor
    sqlite3_vtab_nochange,
    sqlite3_vtab_in_first,
    sqlite3_vtab_in_next,
    // Preupdate
    sqlite3_preupdate_old,
    sqlite3_preupdate_new,
    sqlite3_preupdate_count,
    sqlite3_preupdate_depth,
    sqlite3_preupdate_blobwrite,
};

// ── Memory management ─────────────────────────────────────────────────────────

pub use core::src::src::malloc::{
    sqlite3_malloc,
    sqlite3_malloc64,
    sqlite3_realloc,
    sqlite3_realloc64,
    sqlite3_free,
    sqlite3_msize,
    sqlite3_release_memory,
    sqlite3_memory_used,
    sqlite3_memory_highwater,
    sqlite3_soft_heap_limit64,
    sqlite3_soft_heap_limit,
    sqlite3_hard_heap_limit64,
    sqlite3_memory_alarm,
};

// ── Mutex ─────────────────────────────────────────────────────────────────────

pub use core::src::src::mutex::{
    sqlite3_mutex_alloc,
    sqlite3_mutex_free,
    sqlite3_mutex_enter,
    sqlite3_mutex_try,
    sqlite3_mutex_leave,
};

// ── Backup ────────────────────────────────────────────────────────────────────

pub use core::src::src::backup::{
    sqlite3_backup_init,
    sqlite3_backup_step,
    sqlite3_backup_finish,
    sqlite3_backup_remaining,
    sqlite3_backup_pagecount,
};

// ── Blob I/O ──────────────────────────────────────────────────────────────────

pub use core::src::src::vdbeblob::{
    sqlite3_blob_open,
    sqlite3_blob_close,
    sqlite3_blob_read,
    sqlite3_blob_write,
    sqlite3_blob_bytes,
    sqlite3_blob_reopen,
};

// ── Serialization / deserialization ───────────────────────────────────────────

pub use core::src::src::memdb::{
    sqlite3_serialize,
    sqlite3_deserialize,
};

// ── VFS ───────────────────────────────────────────────────────────────────────

pub use core::src::src::os::{
    sqlite3_vfs_find,
    sqlite3_vfs_register,
    sqlite3_vfs_unregister,
};

pub use core::src::src::os_unix::{
    sqlite3_os_init,
    sqlite3_os_end,
};

// ── Extension loading ─────────────────────────────────────────────────────────

pub use core::src::src::loadext::{
    sqlite3_load_extension,
    sqlite3_enable_load_extension,
    sqlite3_auto_extension,
    sqlite3_cancel_auto_extension,
    sqlite3_reset_auto_extension,
};

// ── Virtual tables ────────────────────────────────────────────────────────────

pub use core::src::src::vtab::{
    sqlite3_create_module,
    sqlite3_create_module_v2,
    sqlite3_drop_modules,
    sqlite3_declare_vtab,
    sqlite3_vtab_on_conflict,
};

pub use core::src::src::r#where::{
    sqlite3_vtab_collation,
    sqlite3_vtab_in,
    sqlite3_vtab_rhs_value,
    sqlite3_vtab_distinct,
};

// ── Table helpers ─────────────────────────────────────────────────────────────

pub use core::src::src::legacy::sqlite3_exec;

pub use core::src::src::table::{
    sqlite3_get_table,
    sqlite3_free_table,
};

// ── Status ────────────────────────────────────────────────────────────────────

pub use core::src::src::status::{
    sqlite3_status,
    sqlite3_status64,
    sqlite3_db_status,
    sqlite3_db_status64,
};

// ── String utilities ──────────────────────────────────────────────────────────

pub use core::src::src::util::{
    sqlite3_stricmp,
    sqlite3_strnicmp,
};

pub use core::src::src::func::{
    sqlite3_strglob,
    sqlite3_strlike,
};

// ── Dynamic string builder ────────────────────────────────────────────────────

pub use core::src::src::printf::{
    sqlite3_str_new,
    sqlite3_str_finish,
    sqlite3_str_append,
    sqlite3_str_appendall,
    sqlite3_str_appendchar,
    sqlite3_str_errcode,
    sqlite3_str_length,
    sqlite3_str_value,
    sqlite3_str_reset,
};

// ── Complete statement detection ──────────────────────────────────────────────

pub use core::src::src::complete::{
    sqlite3_complete,
    sqlite3_complete16,
};

// ── Randomness ────────────────────────────────────────────────────────────────

pub use core::src::src::random::sqlite3_randomness;

// ── Keyword introspection ─────────────────────────────────────────────────────

pub use core::src::src::tokenize::{
    sqlite3_keyword_name,
    sqlite3_keyword_count,
    sqlite3_keyword_check,
};

// ── Shared cache ──────────────────────────────────────────────────────────────

pub use core::src::src::btree::sqlite3_enable_shared_cache;

// ── Pager internals ───────────────────────────────────────────────────────────

pub use core::src::src::pager::sqlite3_database_file_object;

// ── Carray extension ─────────────────────────────────────────────────────────

pub use core::src::src::carray::sqlite3_carray_bind;

// ── VDBE utilities ───────────────────────────────────────────────────────────

pub use core::src::src::vdbe::sqlite3_value_numeric_type;

// ── Variadic function equivalents ────────────────────────────────────────────
//
// SQLite's C variadic functions (sqlite3_config, sqlite3_db_config,
// sqlite3_vtab_config, sqlite3_test_control, sqlite3_mprintf, etc.) cannot be
// expressed directly in Rust's FFI. The rslite port handles this in two ways:
//
// Printf-style (sqlite3_mprintf, sqlite3_vmprintf, sqlite3_snprintf,
// sqlite3_log, sqlite3_str_appendf, sqlite3_str_vappendf):
//   Replaced by `_args` variants that take a `&[PrintfArg]` slice built with
//   the `sqlite_printf_macros` proc-macro crate.
//
// Config-style (sqlite3_config, sqlite3_db_config, sqlite3_vtab_config,
// sqlite3_test_control):
//   Thin C wrappers (in c_code/*.c) pack va_args into u64 slots; Rust
//   dispatch functions (re-exported below) receive a typed op + *const u64.
//   The corresponding Op enums let you construct calls from pure Rust.
//
// NOTE: these have not yet been fully tested — use with caution.

// Printf argument type and _args variants
pub use core::src::src::printf::{
    PrintfArg,
    sqlite3_vmprintf_args,   // sqlite3_mprintf / sqlite3_vmprintf
    sqlite3_snprintf_args,   // sqlite3_snprintf / sqlite3_vsnprintf
    sqlite3_str_vappendf_args, // sqlite3_str_appendf / sqlite3_str_vappendf
};

pub use core::src::printf_c_variadic::sqlite3_log_args; // sqlite3_log

// sqlite3_config — call rs_config_dispatch(op, args_ptr)
pub use core::src::printf_c_variadic::{
    ConfigOp,
    rs_config_dispatch as sqlite3_config_args,
};

// sqlite3_db_config — call rs_db_config_dispatch(db, op, args_ptr)
pub use core::src::printf_c_variadic::rs_db_config_dispatch as sqlite3_db_config_args;

// sqlite3_vtab_config — call rs_vtab_config_dispatch(db, op, args_ptr)
pub use core::src::printf_c_variadic::{
    VtabConfigOp,
    rs_vtab_config_dispatch as sqlite3_vtab_config_args,
};

// sqlite3_test_control — call rs_test_control_dispatch(op, args_ptr)
pub use core::src::printf_c_variadic::{
    TestControlOp,
    rs_test_control_dispatch as sqlite3_test_control_args,
};

// ── R*Tree extension ─────────────────────────────────────────────────────────

pub use core::src::ext::rtree::rtree::{
    sqlite3_geopoly_init,
    sqlite3_rtree_geometry_callback,
    sqlite3_rtree_query_callback,
};
