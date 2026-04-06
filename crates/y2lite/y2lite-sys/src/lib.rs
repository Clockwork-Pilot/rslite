//! Raw FFI bindings to y2lite (equivalent to libsqlite3-sys)
//!
//! This crate exposes all `sqlite3_*` functions for direct FFI usage.
//! Use this for low-level database access when you need maximum control.
//!
//! # Platform Support
//! Supported: **Linux, macOS**
//! Not supported: Windows (win32-specific functions like `sqlite3_win32_set_directory` are not usable)
//!
//! # Example
//! ```ignore
//! use pub_interface_raw::*;
//!
//! let mut db = std::ptr::null_mut();
//! let rc = unsafe { sqlite3_open(c"test.db".as_ptr(), &mut db) };
//! if rc == SQLITE_OK {
//!     // ... use db
//!     unsafe { sqlite3_close(db) };
//! }
//! ```

// Re-export all sqlite3_* functions
pub use sqlite_noamalgam::src::src::main::*;        // sqlite3_open, sqlite3_close, etc.
pub use sqlite_noamalgam::src::src::vdbeapi::*;     // sqlite3_step, sqlite3_bind_*, sqlite3_column_*, etc.
pub use sqlite_noamalgam::src::src::prepare::*;     // sqlite3_prepare_v2

// Re-export types
pub use sqlite_noamalgam::src::headers::sqlite3_h::*; // Constants, types
pub use sqlite_noamalgam::src::headers::sqliteInt_h::sqlite3; // Connection handle
pub use sqlite_noamalgam::src::headers::sqlite3_h::sqlite3_stmt; // Prepared statement


