use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use rslite_raw::*;
use crate::{
    error::{sqlite_error, sqlite_error_from_code},
    types::{FromSql, FromSqlError, ToSql, ToSqlOutput, Type, Value, ValueRef},
    Connection, Error, Result, Rows,
};

/// Returns `true` if `tail` (the unparsed remainder after `sqlite3_prepare_v2`)
/// contains another SQL statement — i.e., more than just whitespace, semicolons,
/// or SQL comments.
fn has_more_sql(mut tail: &[u8]) -> bool {
    loop {
        // Strip leading whitespace and semicolons
        let stripped = tail.iter().position(|&b| !matches!(b, b' ' | b'\t' | b'\n' | b'\r' | b';'));
        tail = match stripped {
            None => return false,
            Some(i) => &tail[i..],
        };
        // Skip -- line comment
        if tail.starts_with(b"--") {
            tail = match tail.iter().position(|&b| b == b'\n') {
                Some(i) => &tail[i + 1..],
                None    => return false, // comment goes to end of input
            };
            continue;
        }
        // Skip /* block comment */
        if tail.starts_with(b"/*") {
            tail = match tail.windows(2).position(|w| w == b"*/") {
                Some(i) => &tail[i + 2..],
                None    => return false, // unterminated block comment
            };
            continue;
        }
        return true; // non-empty, non-comment content remains
    }
}

/// `SQLITE_TRANSIENT` — SQLite will copy the data before the call returns.
fn sqlite_transient() -> sqlite3_destructor_type {
    // SQLITE_TRANSIENT = (sqlite3_destructor_type)-1
    unsafe { std::mem::transmute::<isize, _>(-1isize) }
}

/// A prepared SQL statement bound to a connection's lifetime.
pub struct Statement<'conn> {
    pub(crate) conn: &'conn Connection,
    pub(crate) stmt: *mut sqlite3_stmt,
    column_count: usize,
    column_names: Vec<String>,
}

impl<'conn> Statement<'conn> {
    /// Prepare a SQL statement. Returns an error if the SQL contains multiple
    /// statements (use `Connection::execute_batch` for that).
    pub(crate) fn prepare(conn: &'conn Connection, sql: &str) -> Result<Self> {
        let c_sql = CString::new(sql)?;
        let mut stmt = std::ptr::null_mut();
        let mut tail = std::ptr::null();
        let db = conn.handle_ptr();

        let rc = unsafe {
            sqlite3_prepare_v2(db, c_sql.as_ptr(), -1, &mut stmt, &mut tail)
        };

        if rc != SQLITE_OK {
            return Err(unsafe { sqlite_error(db, rc) });
        }

        // Reject multi-statement SQL (allow trailing whitespace, semicolons, and comments)
        if !tail.is_null() {
            let tail_bytes = unsafe { CStr::from_ptr(tail) }.to_bytes();
            if has_more_sql(tail_bytes) {
                unsafe { sqlite3_finalize(stmt); }
                return Err(Error::MultipleStatement);
            }
        }

        let column_count = unsafe { sqlite3_column_count(stmt) } as usize;
        let column_names = (0..column_count)
            .map(|i| unsafe {
                let ptr = sqlite3_column_name(stmt, i as c_int);
                if ptr.is_null() { String::new() }
                else { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
            })
            .collect();

        Ok(Statement { conn, stmt, column_count, column_names })
    }

    /// Execute the statement (INSERT/UPDATE/DELETE).  Returns the number of
    /// rows changed.
    pub fn execute<P: crate::Params>(&mut self, params: P) -> Result<usize> {
        self.reset();
        params.bind_to(self)?;
        let rc = unsafe { sqlite3_step(self.stmt) };
        match rc {
            SQLITE_DONE => Ok(unsafe { sqlite3_changes(self.conn.handle_ptr()) as usize }),
            SQLITE_ROW  => Err(Error::ExecuteReturnedResults),
            _           => Err(unsafe { sqlite_error(self.conn.handle_ptr(), rc) }),
        }
    }

    /// Execute and return a streaming row iterator.
    pub fn query<P: crate::Params>(&mut self, params: P) -> Result<Rows<'_>> {
        self.reset();
        params.bind_to(self)?;
        Ok(Rows::new(self as &_))
    }

    /// Execute and apply `f` to each row, collecting into a `MappedRows` iterator.
    pub fn query_map<T, P, F>(&mut self, params: P, f: F) -> Result<MappedRows<'_, F>>
    where
        P: crate::Params,
        F: FnMut(&crate::Row<'_>) -> Result<T>,
    {
        let rows = self.query(params)?;
        Ok(MappedRows { rows, f })
    }

    /// Execute and apply `f` to the first row; error if no rows.
    pub fn query_row<T, P, F>(&mut self, params: P, f: F) -> Result<T>
    where
        P: crate::Params,
        F: FnOnce(&crate::Row<'_>) -> Result<T>,
    {
        let mut rows = self.query(params)?;
        match rows.next()? {
            Some(row) => f(&row),
            None      => Err(Error::QueryReturnedNoRows),
        }
    }

    /// Execute an INSERT and return the rowid of the inserted row.
    /// Returns `Error::StatementChangedRows(n)` if the number of changed rows is not 1.
    pub fn insert<P: crate::Params>(&mut self, params: P) -> Result<i64> {
        let changes = self.execute(params)?;
        match changes {
            1 => Ok(unsafe { sqlite3_last_insert_rowid(self.conn.handle_ptr()) }),
            _ => Err(crate::Error::StatementChangedRows(changes)),
        }
    }

    /// Execute and return whether any row exists.
    pub fn exists<P: crate::Params>(&mut self, params: P) -> Result<bool> {
        let mut rows = self.query(params)?;
        Ok(rows.next()?.is_some())
    }

    // ── Column metadata ───────────────────────────────────────────────────────

    pub fn column_count(&self) -> usize { self.column_count }

    pub fn column_name(&self, col: usize) -> Result<&str> {
        self.column_names.get(col)
            .map(|s| s.as_str())
            .ok_or(Error::InvalidColumnIndex(col))
    }

    pub fn column_names(&self) -> Vec<&str> {
        self.column_names.iter().map(|s| s.as_str()).collect()
    }

    pub fn column_index(&self, name: &str) -> Result<usize> {
        self.column_names.iter().position(|n| n == name)
            .or_else(|| self.column_names.iter().position(|n| n.eq_ignore_ascii_case(name)))
            .ok_or_else(|| Error::InvalidColumnName(name.to_string()))
    }

    // ── Parameter metadata ────────────────────────────────────────────────────

    pub fn parameter_count(&self) -> usize {
        unsafe { sqlite3_bind_parameter_count(self.stmt) as usize }
    }

    /// Returns the name of the parameter at 1-based `index`, or `None` if out
    /// of range or the parameter is positional (e.g. `?1`).
    pub fn parameter_name(&self, index: usize) -> Option<&str> {
        unsafe {
            let ptr = sqlite3_bind_parameter_name(self.stmt, index as c_int);
            if ptr.is_null() { None }
            else { CStr::from_ptr(ptr).to_str().ok() }
        }
    }

    /// Returns the 1-based bind index for a named parameter, or `None` if not found.
    pub fn parameter_index(&self, name: &str) -> Result<Option<usize>> {
        let c_name = CString::new(name)?;
        let idx = unsafe {
            sqlite3_bind_parameter_index(self.stmt, c_name.as_ptr())
        };
        if idx == 0 { Ok(None) } else { Ok(Some(idx as usize)) }
    }

    // ── Statement info ────────────────────────────────────────────────────────

    pub fn readonly(&self) -> bool {
        unsafe { sqlite3_stmt_readonly(self.stmt) != 0 }
    }

    pub fn expanded_sql(&self) -> Option<String> {
        unsafe {
            let ptr = sqlite3_expanded_sql(self.stmt);
            if ptr.is_null() { return None; }
            let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            sqlite3_free(ptr as *mut std::ffi::c_void);
            Some(s)
        }
    }

    pub fn sql(&self) -> Option<&str> {
        unsafe {
            let ptr = sqlite3_sql(self.stmt);
            if ptr.is_null() { None }
            else { CStr::from_ptr(ptr).to_str().ok() }
        }
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    pub(crate) fn reset(&mut self) {
        unsafe { sqlite3_reset(self.stmt); }
    }

    pub(crate) fn clear_bindings(&mut self) {
        unsafe { sqlite3_clear_bindings(self.stmt); }
    }

    /// Bind a single `ToSql` value at a 1-based parameter index.
    pub(crate) fn bind_value<V: ToSql + ?Sized>(&mut self, one_idx: usize, value: &V) -> Result<()> {
        let output = value.to_sql()
            .map_err(|e| Error::ToSqlConversionFailure(Box::new(e)))?;
        let idx = one_idx as c_int;
        let db  = self.conn.handle_ptr();

        let rc = match output.as_value_ref() {
            ValueRef::Null       => unsafe { sqlite3_bind_null(self.stmt, idx) },
            ValueRef::Integer(i) => unsafe { sqlite3_bind_int64(self.stmt, idx, i) },
            ValueRef::Real(f)    => unsafe { sqlite3_bind_double(self.stmt, idx, f) },
            ValueRef::Text(s)    => {
                let bytes = s.as_bytes();
                unsafe {
                    sqlite3_bind_text(
                        self.stmt, idx,
                        bytes.as_ptr() as *const _,
                        bytes.len() as c_int,
                        sqlite_transient(),
                    )
                }
            }
            ValueRef::Blob(b) => unsafe {
                sqlite3_bind_blob(
                    self.stmt, idx,
                    b.as_ptr() as *const _,
                    b.len() as c_int,
                    sqlite_transient(),
                )
            },
        };

        if rc != SQLITE_OK {
            Err(unsafe { sqlite_error(db, rc) })
        } else {
            Ok(())
        }
    }

    /// Read column `col` as a `ValueRef` tied to this step's lifetime.
    pub(crate) unsafe fn column_value_ref<'s>(&'s self, col: usize) -> Result<ValueRef<'s>> {
        if col >= self.column_count {
            return Err(Error::InvalidColumnIndex(col));
        }
        let ci = col as c_int;
        let col_type = unsafe { sqlite3_column_type(self.stmt, ci) };
        let vref = match col_type {
            SQLITE_INTEGER => ValueRef::Integer(unsafe { sqlite3_column_int64(self.stmt, ci) }),
            SQLITE_FLOAT   => ValueRef::Real(unsafe { sqlite3_column_double(self.stmt, ci) }),
            SQLITE_TEXT    => {
                let ptr = unsafe { sqlite3_column_text(self.stmt, ci) };
                let len = unsafe { sqlite3_column_bytes(self.stmt, ci) } as usize;
                if ptr.is_null() {
                    ValueRef::Null
                } else {
                    let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, len) };
                    let s = std::str::from_utf8(slice)?;
                    ValueRef::Text(s)
                }
            }
            SQLITE_BLOB    => {
                let len = unsafe { sqlite3_column_bytes(self.stmt, ci) } as usize;
                let ptr = unsafe { sqlite3_column_blob(self.stmt, ci) };
                if ptr.is_null() {
                    // Empty blob: sqlite3_column_blob returns NULL when length is 0
                    ValueRef::Blob(&[])
                } else {
                    let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, len) };
                    ValueRef::Blob(slice)
                }
            }
            _ => ValueRef::Null,
        };
        Ok(vref)
    }
}

impl std::fmt::Debug for Statement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Statement")
            .field("sql", &self.sql())
            .finish_non_exhaustive()
    }
}

impl Drop for Statement<'_> {
    fn drop(&mut self) {
        unsafe { sqlite3_finalize(self.stmt); }
    }
}

// ── MappedRows ────────────────────────────────────────────────────────────────

pub struct MappedRows<'stmt, F> {
    pub(crate) rows: Rows<'stmt>,
    pub(crate) f: F,
}

impl<T, F> Iterator for MappedRows<'_, F>
where
    F: FnMut(&crate::Row<'_>) -> Result<T>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.rows.next() {
            Err(e)           => Some(Err(e)),
            Ok(None)         => None,
            Ok(Some(ref row)) => Some((self.f)(row)),
        }
    }
}
