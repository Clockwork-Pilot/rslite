//! Safe SQLite database interface wrapping the pure-Rust C API.
//!
//! This crate provides a rusqlite-like API backed by the statically-linked
//! pure-Rust SQLite implementation, with no external C dependencies.

use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use sqlite_noamalgam::src::src::prepare::sqlite3_prepare_v2;
use sqlite_noamalgam::src::src::vdbeapi::{
    sqlite3_step, sqlite3_finalize, sqlite3_column_count, sqlite3_column_int64,
    sqlite3_column_text, sqlite3_column_bytes,
    sqlite3_bind_int64, sqlite3_bind_text, sqlite3_bind_blob, sqlite3_bind_null,
    sqlite3_bind_double,
};
use sqlite_noamalgam::src::src::main::{
    sqlite3, sqlite3_open, sqlite3_errmsg, sqlite3_close,
};
use sqlite_noamalgam::src::headers::sqlite3_h::{sqlite3_stmt, SQLITE_OK, SQLITE_ROW, SQLITE_DONE};

/// A safe database connection (statically linked, C API).
pub struct Connection {
    db: *mut sqlite3,
}

impl Connection {
    /// Open or create a database at the given path.
    pub fn open(path: &str) -> Result<Self> {
        let c_path = CString::new(path)
            .map_err(|_| Error::Database("invalid path".to_string()))?;

        let mut db = std::ptr::null_mut();
        unsafe {
            let rc = sqlite3_open(c_path.as_ptr(), &mut db);
            if rc != SQLITE_OK {
                return Err(Error::Database(
                    CStr::from_ptr(sqlite3_errmsg(db))
                        .to_string_lossy()
                        .to_string()
                ));
            }
        }

        Ok(Connection { db })
    }

    /// Execute a SQL statement (DDL).
    pub fn execute(&mut self, sql: &str) -> Result<()> {
        let c_sql = CString::new(sql)
            .map_err(|_| Error::Database("invalid SQL".to_string()))?;

        let mut stmt = std::ptr::null_mut();
        unsafe {
            let rc = sqlite3_prepare_v2(self.db, c_sql.as_ptr(), -1, &mut stmt, std::ptr::null_mut());
            if rc != SQLITE_OK {
                return Err(Error::Database(
                    CStr::from_ptr(sqlite3_errmsg(self.db))
                        .to_string_lossy()
                        .to_string()
                ));
            }

            let rc = sqlite3_step(stmt);
            if rc != SQLITE_DONE {
                sqlite3_finalize(stmt);
                return Err(Error::Database("step failed".to_string()));
            }

            sqlite3_finalize(stmt);
        }

        Ok(())
    }

    /// Execute a query and return rows.
    pub fn query(&mut self, sql: &str) -> Result<Rows> {
        self.query_with_params(sql, &[])
    }

    /// Execute a query with parameterized parameters (safe from SQL injection).
    pub fn query_with_params(&mut self, sql: &str, params: &[Value]) -> Result<Rows> {
        let c_sql = CString::new(sql)
            .map_err(|_| Error::Database("invalid SQL".to_string()))?;

        let mut stmt = std::ptr::null_mut();
        unsafe {
            let rc = sqlite3_prepare_v2(self.db, c_sql.as_ptr(), -1, &mut stmt, std::ptr::null_mut());
            if rc != SQLITE_OK {
                return Err(Error::Database(
                    CStr::from_ptr(sqlite3_errmsg(self.db))
                        .to_string_lossy()
                        .to_string()
                ));
            }

            // Bind parameters
            self.bind_params(stmt, params).map_err(|e| {
                sqlite3_finalize(stmt);
                e
            })?;

            let mut rows = Vec::new();
            loop {
                let rc = sqlite3_step(stmt);
                if rc == SQLITE_DONE {
                    break;
                } else if rc != SQLITE_ROW {
                    sqlite3_finalize(stmt);
                    return Err(Error::Database("step failed".to_string()));
                }

                let col_count = sqlite3_column_count(stmt) as usize;
                let mut row = Vec::with_capacity(col_count);

                for i in 0..col_count {
                    let val = Value::from_stmt(stmt, i as c_int);
                    row.push(val);
                }

                rows.push(row);
            }

            sqlite3_finalize(stmt);
            Ok(Rows { rows })
        }
    }

    /// Execute a statement with parameterized parameters (safe from SQL injection).
    pub fn execute_with_params(&mut self, sql: &str, params: &[Value]) -> Result<()> {
        let c_sql = CString::new(sql)
            .map_err(|_| Error::Database("invalid SQL".to_string()))?;

        let mut stmt = std::ptr::null_mut();
        unsafe {
            let rc = sqlite3_prepare_v2(self.db, c_sql.as_ptr(), -1, &mut stmt, std::ptr::null_mut());
            if rc != SQLITE_OK {
                return Err(Error::Database(
                    CStr::from_ptr(sqlite3_errmsg(self.db))
                        .to_string_lossy()
                        .to_string()
                ));
            }

            // Bind parameters
            self.bind_params(stmt, params).map_err(|e| {
                sqlite3_finalize(stmt);
                e
            })?;

            let rc = sqlite3_step(stmt);
            if rc != SQLITE_DONE {
                sqlite3_finalize(stmt);
                return Err(Error::Database("step failed".to_string()));
            }

            sqlite3_finalize(stmt);
        }

        Ok(())
    }

    /// Set foreign_keys pragma.
    pub fn set_foreign_keys(&mut self, enabled: bool) -> Result<()> {
        let value = if enabled { "ON" } else { "OFF" };
        self.execute(&format!("PRAGMA foreign_keys = {}", value))
    }

    /// Get foreign_keys pragma.
    pub fn foreign_keys(&mut self) -> Result<bool> {
        let rows = self.query("PRAGMA foreign_keys")?;
        if let Some(row) = rows.iter().next() {
            if let Some(Value::Integer(i)) = row.first() {
                return Ok(*i != 0);
            }
        }
        Err(Error::Database("failed to read pragma".to_string()))
    }

    /// Set journal_mode pragma.
    pub fn set_journal_mode(&mut self, mode: &str) -> Result<String> {
        let rows = self.query(&format!("PRAGMA journal_mode = {}", mode))?;
        rows.iter().next()
            .and_then(|row| row.first())
            .and_then(|val| {
                if let Value::Text(s) = val {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| Error::Database("failed to set journal_mode".to_string()))
    }

    /// Get journal_mode pragma.
    pub fn journal_mode(&mut self) -> Result<String> {
        let rows = self.query("PRAGMA journal_mode")?;
        rows.iter().next()
            .and_then(|row| row.first())
            .and_then(|val| {
                if let Value::Text(s) = val {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| Error::Database("failed to read pragma".to_string()))
    }

    /// Set synchronous pragma (0=OFF, 1=NORMAL, 2=FULL).
    pub fn set_synchronous(&mut self, level: i64) -> Result<()> {
        self.execute(&format!("PRAGMA synchronous = {}", level))
    }

    /// Get synchronous pragma.
    pub fn synchronous(&mut self) -> Result<i64> {
        let rows = self.query("PRAGMA synchronous")?;
        rows.iter().next()
            .and_then(|row| row.first())
            .and_then(|val| {
                if let Value::Integer(i) = val {
                    Some(*i)
                } else {
                    None
                }
            })
            .ok_or_else(|| Error::Database("failed to read pragma".to_string()))
    }

    /// Set cache_size pragma (in pages).
    pub fn set_cache_size(&mut self, pages: i64) -> Result<()> {
        self.execute(&format!("PRAGMA cache_size = {}", pages))
    }

    /// Get cache_size pragma.
    pub fn cache_size(&mut self) -> Result<i64> {
        let rows = self.query("PRAGMA cache_size")?;
        rows.iter().next()
            .and_then(|row| row.first())
            .and_then(|val| {
                if let Value::Integer(i) = val {
                    Some(*i)
                } else {
                    None
                }
            })
            .ok_or_else(|| Error::Database("failed to read pragma".to_string()))
    }

    /// Set page_size pragma (in bytes).
    pub fn set_page_size(&mut self, bytes: i64) -> Result<()> {
        self.execute(&format!("PRAGMA page_size = {}", bytes))
    }

    /// Get page_size pragma.
    pub fn page_size(&mut self) -> Result<i64> {
        let rows = self.query("PRAGMA page_size")?;
        rows.iter().next()
            .and_then(|row| row.first())
            .and_then(|val| {
                if let Value::Integer(i) = val {
                    Some(*i)
                } else {
                    None
                }
            })
            .ok_or_else(|| Error::Database("failed to read pragma".to_string()))
    }

    /// Set user_version pragma.
    pub fn set_user_version(&mut self, version: i64) -> Result<()> {
        self.execute(&format!("PRAGMA user_version = {}", version))
    }

    /// Get user_version pragma.
    pub fn user_version(&mut self) -> Result<i64> {
        let rows = self.query("PRAGMA user_version")?;
        rows.iter().next()
            .and_then(|row| row.first())
            .and_then(|val| {
                if let Value::Integer(i) = val {
                    Some(*i)
                } else {
                    None
                }
            })
            .ok_or_else(|| Error::Database("failed to read pragma".to_string()))
    }

    /// Set application_id pragma.
    pub fn set_application_id(&mut self, id: i64) -> Result<()> {
        self.execute(&format!("PRAGMA application_id = {}", id))
    }

    /// Get application_id pragma.
    pub fn application_id(&mut self) -> Result<i64> {
        let rows = self.query("PRAGMA application_id")?;
        rows.iter().next()
            .and_then(|row| row.first())
            .and_then(|val| {
                if let Value::Integer(i) = val {
                    Some(*i)
                } else {
                    None
                }
            })
            .ok_or_else(|| Error::Database("failed to read pragma".to_string()))
    }

    /// Set query_only pragma.
    pub fn set_query_only(&mut self, enabled: bool) -> Result<()> {
        let value = if enabled { "ON" } else { "OFF" };
        self.execute(&format!("PRAGMA query_only = {}", value))
    }

    /// Get query_only pragma.
    pub fn query_only(&mut self) -> Result<bool> {
        let rows = self.query("PRAGMA query_only")?;
        rows.iter().next()
            .and_then(|row| row.first())
            .and_then(|val| {
                match val {
                    Value::Integer(i) => Some(*i != 0),
                    _ => None,
                }
            })
            .ok_or_else(|| Error::Database("failed to read pragma".to_string()))
    }

    /// Get table_info for a table. Returns list of column metadata.
    pub fn table_info(&mut self, table: &str) -> Result<Vec<TableColumn>> {
        let rows = self.query(&format!("PRAGMA table_info({})", table))?;
        let mut cols = Vec::new();
        for row in rows.iter() {
            if row.len() >= 6 {
                if let (
                    Value::Integer(cid),
                    Value::Text(name),
                    Value::Text(col_type),
                    Value::Integer(notnull),
                    dflt_value,
                    Value::Integer(pk),
                ) = (
                    &row[0],
                    &row[1],
                    &row[2],
                    &row[3],
                    &row[4],
                    &row[5],
                ) {
                    cols.push(TableColumn {
                        cid: *cid,
                        name: name.clone(),
                        col_type: col_type.clone(),
                        notnull: *notnull != 0,
                        dflt_value: dflt_value.clone(),
                        pk: *pk != 0,
                    });
                }
            }
        }
        Ok(cols)
    }

    /// Bind parameters to a prepared statement.
    unsafe fn bind_params(&self, stmt: *mut sqlite3_stmt, params: &[Value]) -> Result<()> {
        for (idx, param) in params.iter().enumerate() {
            let param_idx = (idx + 1) as c_int;
            let rc = match param {
                Value::Integer(i) => sqlite3_bind_int64(stmt, param_idx, *i),
                Value::Real(f) => sqlite3_bind_double(stmt, param_idx, *f),
                Value::Text(s) => {
                    let c_str = CString::new(s.as_str())
                        .map_err(|_| Error::Database("invalid parameter".to_string()))?;
                    sqlite3_bind_text(
                        stmt,
                        param_idx,
                        c_str.as_ptr(),
                        -1,
                        std::mem::transmute(-1isize),
                    )
                }
                Value::Blob(b) => sqlite3_bind_blob(
                    stmt,
                    param_idx,
                    b.as_ptr() as *const std::ffi::c_void,
                    b.len() as i32,
                    std::mem::transmute(-1isize),
                ),
                Value::Null => sqlite3_bind_null(stmt, param_idx),
            };

            if rc != SQLITE_OK {
                return Err(Error::Database("bind failed".to_string()));
            }
        }
        Ok(())
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            sqlite3_close(self.db);
        }
    }
}

/// Table column metadata from PRAGMA table_info.
#[derive(Debug, Clone)]
pub struct TableColumn {
    pub cid: i64,
    pub name: String,
    pub col_type: String,
    pub notnull: bool,
    pub dflt_value: Value,
    pub pk: bool,
}

/// Column value.
#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
    Null,
}

impl Value {
    unsafe fn from_stmt(stmt: *mut sqlite3_stmt, col: c_int) -> Self {
        // Simplified - would need sqlite3_column_type for proper type detection
        if let Some(ptr) = std::ptr::NonNull::new(sqlite3_column_text(stmt, col) as *mut u8) {
            let len = sqlite3_column_bytes(stmt, col) as usize;
            let slice = std::slice::from_raw_parts(ptr.as_ptr(), len);
            Value::Text(String::from_utf8_lossy(slice).to_string())
        } else {
            let i = sqlite3_column_int64(stmt, col);
            if i != 0 {
                Value::Integer(i)
            } else {
                Value::Null
            }
        }
    }
}

/// Query result rows.
pub struct Rows {
    rows: Vec<Vec<Value>>,
}

impl Rows {
    /// Iterate over rows.
    pub fn iter(&self) -> impl Iterator<Item = &Vec<Value>> {
        self.rows.iter()
    }
}

/// Error type.
#[derive(Debug)]
pub enum Error {
    Database(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
