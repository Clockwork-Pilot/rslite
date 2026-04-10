//! Re-exports of raw SQLite constants and types.
//!
//! This module mirrors rusqlite's `ffi` module: it exposes low-level
//! SQLite C API constants useful for inspecting error codes and configuring
//! behavior.
//!
//! Most users should work with the high-level `Error`, `ErrorCode`, and
//! `Connection` types.  This module is provided for advanced use cases such
//! as inspecting extended error codes.

pub use sqlite_noamalgam::{
    // Core result codes
    SQLITE_OK,
    SQLITE_ERROR,
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
    SQLITE_NOTICE,
    SQLITE_WARNING,
    SQLITE_ROW,
    SQLITE_DONE,
    // Extended constraint codes
    SQLITE_CONSTRAINT_CHECK,
    SQLITE_CONSTRAINT_NOTNULL,
    SQLITE_CONSTRAINT_FOREIGNKEY,
    SQLITE_CONSTRAINT_PRIMARYKEY,
    SQLITE_CONSTRAINT_UNIQUE,
    // Trace event flags
    SQLITE_TRACE_STMT,
    SQLITE_TRACE_PROFILE,
    SQLITE_TRACE_ROW,
    SQLITE_TRACE_CLOSE,
};

// Limit category constants (SQLite SQLITE_LIMIT_*)
pub const SQLITE_LIMIT_LENGTH:              i32 = 0;
pub const SQLITE_LIMIT_SQL_LENGTH:          i32 = 1;
pub const SQLITE_LIMIT_COLUMN:              i32 = 2;
pub const SQLITE_LIMIT_EXPR_DEPTH:          i32 = 3;
pub const SQLITE_LIMIT_COMPOUND_SELECT:     i32 = 4;
pub const SQLITE_LIMIT_VDBE_OP:             i32 = 5;
pub const SQLITE_LIMIT_FUNCTION_ARG:        i32 = 6;
pub const SQLITE_LIMIT_ATTACHED:            i32 = 7;
pub const SQLITE_LIMIT_LIKE_PATTERN_LENGTH: i32 = 8;
pub const SQLITE_LIMIT_VARIABLE_NUMBER:     i32 = 9;
pub const SQLITE_LIMIT_TRIGGER_DEPTH:       i32 = 10;
pub const SQLITE_LIMIT_WORKER_THREADS:      i32 = 11;

// Authorizer action codes (SQLite SQLITE_*)
pub const SQLITE_CREATE_INDEX:   i32 = 1;
pub const SQLITE_CREATE_TABLE:   i32 = 2;
pub const SQLITE_CREATE_TRIGGER: i32 = 7;
pub const SQLITE_CREATE_VIEW:    i32 = 8;
pub const SQLITE_DELETE:         i32 = 9;
pub const SQLITE_DROP_INDEX:     i32 = 10;
pub const SQLITE_DROP_TABLE:     i32 = 11;
pub const SQLITE_DROP_TRIGGER:   i32 = 16;
pub const SQLITE_DROP_VIEW:      i32 = 17;
pub const SQLITE_INSERT:         i32 = 18;
pub const SQLITE_PRAGMA:         i32 = 19;
pub const SQLITE_READ:           i32 = 20;
pub const SQLITE_SELECT:         i32 = 21;
pub const SQLITE_TRANSACTION:    i32 = 22;
pub const SQLITE_UPDATE:         i32 = 23;
pub const SQLITE_ATTACH:         i32 = 24;
pub const SQLITE_DETACH:         i32 = 25;
pub const SQLITE_ALTER_TABLE:    i32 = 26;
pub const SQLITE_REINDEX:        i32 = 27;
pub const SQLITE_ANALYZE:        i32 = 28;
pub const SQLITE_CREATE_VTABLE:  i32 = 29;
pub const SQLITE_DROP_VTABLE:    i32 = 30;
pub const SQLITE_FUNCTION:       i32 = 31;
pub const SQLITE_SAVEPOINT:      i32 = 32;
pub const SQLITE_RECURSIVE:      i32 = 33;

// Authorizer return values
pub const SQLITE_DENY:   i32 = 1;
pub const SQLITE_IGNORE: i32 = 2;
