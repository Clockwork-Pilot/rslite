//! Safe SQLite database interface wrapping the pure-Rust C API.
//!
//! This crate provides a rusqlite-compatible API backed by the statically-linked
//! pure-Rust SQLite implementation, with no external C dependencies.
//!
//! # Quick start
//!
//! ```ignore
//! use rslite::{Connection, params};
//!
//! let conn = Connection::open_in_memory()?;
//! conn.execute_batch("CREATE TABLE t (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")?;
//! conn.execute("INSERT INTO t (name) VALUES (?1)", params!["Alice"])?;
//!
//! let name: String = conn.query_row(
//!     "SELECT name FROM t WHERE id = ?1",
//!     params![1i64],
//!     |row| row.get(0),
//! )?;
//! ```
//!
//! # Safety
//! All unsafe FFI operations are confined to the private module internals.
//! The public API is completely safe.

pub mod error;
pub mod functions;
pub mod hooks;
pub mod types;
pub mod params;
pub mod statement;
pub mod rows;
pub mod transaction;
pub mod connection;

pub use error::{Error, ErrorCode, Result, OptionalExtension};
pub use hooks::Action;
pub use functions::{Aggregate, Context, FunctionFlags};
pub use types::{Type, Value, ValueRef, ToSql, ToSqlOutput, FromSql, FromSqlError, FromSqlResult, Null};
pub use params::{Params, ParamsFromIter, params_from_iter};
pub use statement::{Statement, MappedRows};
pub use rows::{Row, RowIndex, Rows};
pub use transaction::{Transaction, TransactionBehavior, DropBehavior, Savepoint};
pub use connection::{Connection, OpenFlags};
