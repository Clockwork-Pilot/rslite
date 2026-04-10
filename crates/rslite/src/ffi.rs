//! Private unsafe FFI layer. All unsafe code is confined here.
//! Users of this module must ensure safety invariants are maintained.

use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use sqlite_noamalgam::{
    sqlite3_prepare_v2, sqlite3_step, sqlite3_finalize, sqlite3_column_count, sqlite3_column_int64,
    sqlite3_column_text, sqlite3_column_bytes, sqlite3_column_double, sqlite3_column_type,
    sqlite3_column_blob,
    sqlite3_bind_int64, sqlite3_bind_text, sqlite3_bind_blob, sqlite3_bind_null,
    sqlite3_bind_double,
};
use sqlite_noamalgam::{
    sqlite3, sqlite3_open, sqlite3_errmsg, sqlite3_close,
};
use sqlite_noamalgam::{
    sqlite3_stmt, SQLITE_OK, SQLITE_ROW, SQLITE_DONE,
    SQLITE_INTEGER, SQLITE_FLOAT, SQLITE_BLOB, SQLITE_NULL, SQLITE_TEXT,
};
use std::ffi::{CStr, CString};
use std::os::raw::c_int;

use crate::{Error, Result, Value};

/// Low-level FFI database handle wrapper.
pub struct Database {
    ptr: *mut sqlite3,
}

impl Database {
    /// Open a database (unsafe - requires valid path).
    pub fn open(path: &str) -> Result<Self> {
        let c_path = CString::new(path).map_err(|_| Error::Database("invalid path".to_string()))?;

        let mut db = std::ptr::null_mut();
        unsafe {
            let rc = sqlite3_open(c_path.as_ptr(), &mut db);
            if rc != SQLITE_OK {
                let err = CStr::from_ptr(sqlite3_errmsg(db))
                    .to_string_lossy()
                    .to_string();
                return Err(Error::Database(err));
            }
        }

        Ok(Database { ptr: db })
    }

    /// Get error message (unsafe - requires valid db pointer).
    pub fn error_msg(&self) -> String {
        unsafe {
            CStr::from_ptr(sqlite3_errmsg(self.ptr))
                .to_string_lossy()
                .to_string()
        }
    }

    /// Execute a statement without parameters (unsafe - caller owns safety).
    pub fn execute(&self, sql: &str) -> Result<()> {
        let c_sql = CString::new(sql).map_err(|_| Error::Database("invalid SQL".to_string()))?;

        unsafe {
            let mut stmt = std::ptr::null_mut();
            let rc = sqlite3_prepare_v2(
                self.ptr,
                c_sql.as_ptr(),
                -1,
                &mut stmt,
                std::ptr::null_mut(),
            );
            if rc != SQLITE_OK {
                return Err(Error::Database(self.error_msg()));
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

    /// Execute a query without parameters (unsafe - caller owns safety).
    pub fn query(&self, sql: &str) -> Result<Vec<Vec<Value>>> {
        let c_sql = CString::new(sql).map_err(|_| Error::Database("invalid SQL".to_string()))?;

        unsafe {
            let mut stmt = std::ptr::null_mut();
            let rc = sqlite3_prepare_v2(
                self.ptr,
                c_sql.as_ptr(),
                -1,
                &mut stmt,
                std::ptr::null_mut(),
            );
            if rc != SQLITE_OK {
                return Err(Error::Database(self.error_msg()));
            }

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
                    let val = extract_column(stmt, i as c_int);
                    row.push(val);
                }

                rows.push(row);
            }

            sqlite3_finalize(stmt);
            Ok(rows)
        }
    }

    /// Execute a statement with parameters (unsafe - caller owns safety).
    pub fn execute_with_params(&self, sql: &str, params: &[Value]) -> Result<()> {
        let c_sql = CString::new(sql).map_err(|_| Error::Database("invalid SQL".to_string()))?;

        unsafe {
            let mut stmt = std::ptr::null_mut();
            let rc = sqlite3_prepare_v2(
                self.ptr,
                c_sql.as_ptr(),
                -1,
                &mut stmt,
                std::ptr::null_mut(),
            );
            if rc != SQLITE_OK {
                return Err(Error::Database(self.error_msg()));
            }

            // Bind parameters
            bind_params(stmt, params).map_err(|e| {
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

    /// Execute a query with parameters (unsafe - caller owns safety).
    pub fn query_with_params(&self, sql: &str, params: &[Value]) -> Result<Vec<Vec<Value>>> {
        let c_sql = CString::new(sql).map_err(|_| Error::Database("invalid SQL".to_string()))?;

        unsafe {
            let mut stmt = std::ptr::null_mut();
            let rc = sqlite3_prepare_v2(
                self.ptr,
                c_sql.as_ptr(),
                -1,
                &mut stmt,
                std::ptr::null_mut(),
            );
            if rc != SQLITE_OK {
                return Err(Error::Database(self.error_msg()));
            }

            // Bind parameters
            bind_params(stmt, params).map_err(|e| {
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
                    let val = extract_column(stmt, i as c_int);
                    row.push(val);
                }

                rows.push(row);
            }

            sqlite3_finalize(stmt);
            Ok(rows)
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        unsafe {
            sqlite3_close(self.ptr);
        }
    }
}

/// Bind parameters to a prepared statement (unsafe - caller owns safety).
unsafe fn bind_params(stmt: *mut sqlite3_stmt, params: &[Value]) -> Result<()> {
    for (idx, param) in params.iter().enumerate() {
        let param_idx = (idx + 1) as c_int;
        let rc = match param {
            Value::Integer(i) => unsafe { sqlite3_bind_int64(stmt, param_idx, *i) },
            Value::Real(f) => unsafe { sqlite3_bind_double(stmt, param_idx, *f) },
            Value::Text(s) => {
                let c_str = CString::new(s.as_str())
                    .map_err(|_| Error::Database("invalid parameter".to_string()))?;
                unsafe {
                    sqlite3_bind_text(
                        stmt,
                        param_idx,
                        c_str.as_ptr(),
                        -1,
                        std::mem::transmute(-1isize),
                    )
                }
            }
            Value::Blob(b) => unsafe {
                sqlite3_bind_blob(
                    stmt,
                    param_idx,
                    b.as_ptr() as *const std::ffi::c_void,
                    b.len() as i32,
                    std::mem::transmute(-1isize),
                )
            },
            Value::Null => unsafe { sqlite3_bind_null(stmt, param_idx) },
        };

        if rc != SQLITE_OK {
            return Err(Error::Database("bind failed".to_string()));
        }
    }
    Ok(())
}

/// Extract a column value from a statement (unsafe - caller owns safety).
unsafe fn extract_column(stmt: *mut sqlite3_stmt, col: c_int) -> Value {
    let col_type = unsafe { sqlite3_column_type(stmt, col) };

    match col_type {
        SQLITE_INTEGER => Value::Integer(unsafe { sqlite3_column_int64(stmt, col) }),
        SQLITE_FLOAT => Value::Real(unsafe { sqlite3_column_double(stmt, col) }),
        SQLITE_TEXT => {
            let ptr = unsafe { sqlite3_column_text(stmt, col) };
            if ptr.is_null() {
                Value::Null
            } else {
                let len = unsafe { sqlite3_column_bytes(stmt, col) } as usize;
                let slice = std::slice::from_raw_parts(ptr as *const u8, len);
                Value::Text(String::from_utf8_lossy(slice).to_string())
            }
        }
        SQLITE_BLOB => {
            let ptr = unsafe { sqlite3_column_blob(stmt, col) };
            if ptr.is_null() {
                Value::Null
            } else {
                let len = unsafe { sqlite3_column_bytes(stmt, col) } as usize;
                let slice = std::slice::from_raw_parts(ptr as *const u8, len);
                Value::Blob(slice.to_vec())
            }
        }
        SQLITE_NULL => Value::Null,
        _ => Value::Null,
    }
}
