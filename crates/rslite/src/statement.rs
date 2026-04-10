use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use sqlite_noamalgam::*;
use crate::{
    error::sqlite_error,
    types::{ToSql, ValueRef},
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

    /// Execute and apply `f` to each row, collecting into an `AndThenRows` iterator.
    ///
    /// Unlike `query_map`, the closure may return any error type `E` that implements
    /// `From<Error>`.
    pub fn query_and_then<T, E, P, F>(&mut self, params: P, f: F) -> Result<AndThenRows<'_, F>>
    where
        P: crate::Params,
        E: From<Error>,
        F: FnMut(&crate::Row<'_>) -> std::result::Result<T, E>,
    {
        let rows = self.query(params)?;
        Ok(AndThenRows { rows, f })
    }

    /// Execute and return whether any row exists.
    pub fn exists<P: crate::Params>(&mut self, params: P) -> Result<bool> {
        let mut rows = self.query(params)?;
        Ok(rows.next()?.is_some())
    }

    // ── Column metadata ───────────────────────────────────────────────────────

    /// Returns the number of columns in the result set projected by this statement.
    ///
    /// For `SELECT` statements the count equals the number of expressions in the select list.
    /// For DML statements (`INSERT`/`UPDATE`/`DELETE`) the count is typically 0 unless a
    /// `RETURNING` clause is present.  The value is cached at prepare time so calling this
    /// method is free.
    pub fn column_count(&self) -> usize { self.column_count }

    /// Returns the name of result-set column `col` (0-based), or an [`Error::InvalidColumnIndex`]
    /// if `col` is out of range.
    ///
    /// The name is the `AS` alias when one is supplied; otherwise it is the bare column name from
    /// the schema.  For computed expressions without an alias, SQLite synthesises a name such as
    /// `"col0"`.  The slice borrows from the `Statement` and is valid for its lifetime.
    pub fn column_name(&self, col: usize) -> Result<&str> {
        self.column_names.get(col)
            .map(|s| s.as_str())
            .ok_or(Error::InvalidColumnIndex(col))
    }

    /// Returns all result-set column names as a `Vec<&str>`, in left-to-right order.
    ///
    /// Each slice borrows from the `Statement`'s internal name cache and is valid for the
    /// statement's lifetime.  The vector is allocated on each call; for hot paths that need
    /// a single column, prefer [`column_name`](Statement::column_name) to avoid the allocation.
    pub fn column_names(&self) -> Vec<&str> {
        self.column_names.iter().map(|s| s.as_str()).collect()
    }

    /// Find the 0-based column index for a column with the given name.
    ///
    /// The lookup is first attempted with an exact (case-sensitive) match; if no exact match is
    /// found, a second pass tries ASCII case-insensitive comparison.  Returns
    /// [`Error::InvalidColumnName`] if no column with that name exists.  This two-pass strategy
    /// matches rusqlite's behaviour and avoids false negatives for mixed-case aliases.
    pub fn column_index(&self, name: &str) -> Result<usize> {
        self.column_names.iter().position(|n| n == name)
            .or_else(|| self.column_names.iter().position(|n| n.eq_ignore_ascii_case(name)))
            .ok_or_else(|| Error::InvalidColumnName(name.to_string()))
    }

    // ── Parameter metadata ────────────────────────────────────────────────────

    /// Returns the number of host parameters (`?`, `?N`, `:name`, `@name`, `$name`) in this
    /// statement, as reported by `sqlite3_bind_parameter_count`.
    ///
    /// Parameters are numbered starting at 1 in the SQLite C API.  This count includes all
    /// positional and named parameters; duplicate named parameters are counted only once.
    /// It can be used to validate that the correct number of values are bound before execution.
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

    /// Returns `true` if this statement makes no direct changes to the database content.
    ///
    /// The result is derived from `sqlite3_stmt_readonly`, which inspects the compiled VDBE
    /// bytecode rather than the SQL text.  A `SELECT` that calls a non-deterministic function
    /// with side effects may still return `true` here; this flag only reflects whether the
    /// statement itself issues any write operations, not whether side effects are possible.
    pub fn readonly(&self) -> bool {
        unsafe { sqlite3_stmt_readonly(self.stmt) != 0 }
    }

    /// Returns the SQL text of this statement with all bound parameters replaced by their
    /// literal values, allocated by SQLite and returned as an owned `String`.
    ///
    /// The expansion is performed by `sqlite3_expanded_sql` and the memory is freed with
    /// `sqlite3_free` before returning.  Returns `None` if SQLite could not allocate the
    /// string (out of memory) or if the statement has no SQL text.  Primarily useful for
    /// debugging and logging; do not use the result to re-prepare the statement.
    pub fn expanded_sql(&self) -> Option<String> {
        unsafe {
            let ptr = sqlite3_expanded_sql(self.stmt);
            if ptr.is_null() { return None; }
            let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            sqlite3_free(ptr as *mut std::ffi::c_void);
            Some(s)
        }
    }

    /// Returns the original SQL text that was used to prepare this statement, as a borrowed
    /// `&str` tied to the statement's lifetime.
    ///
    /// The pointer is owned by SQLite and remains valid as long as the statement is not
    /// finalized.  Returns `None` if the statement handle is null or if the text is not valid
    /// UTF-8.  Unlike [`expanded_sql`](Statement::expanded_sql), parameter placeholders are
    /// returned verbatim rather than substituted with their bound values.
    pub fn sql(&self) -> Option<&str> {
        unsafe {
            let ptr = sqlite3_sql(self.stmt);
            if ptr.is_null() { None }
            else { CStr::from_ptr(ptr).to_str().ok() }
        }
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    /// Reset the statement cursor back to the beginning so it can be executed again.
    ///
    /// Calls `sqlite3_reset`, which does not clear parameter bindings — bound values remain in
    /// place for the next execution.  Use [`clear_bindings`](Statement::clear_bindings) afterward
    /// if you need to remove them.  This is called automatically by [`execute`] and [`query`]
    /// before each execution, so manual calls are rarely necessary.
    pub(crate) fn reset(&mut self) {
        unsafe { sqlite3_reset(self.stmt); }
    }

    #[allow(dead_code)]
    pub(crate) fn clear_bindings(&mut self) {
        unsafe { sqlite3_clear_bindings(self.stmt); }
    }

    /// Extract the raw `sqlite3_stmt*` pointer, consuming the `Statement`
    /// without finalizing it.  The caller must eventually finalize the pointer.
    pub(crate) fn into_raw(self) -> *mut sqlite3_stmt {
        let ptr = self.stmt;
        std::mem::forget(self);
        ptr
    }

    /// Wrap a raw `sqlite3_stmt*` in a `Statement`.  Resets the statement and
    /// clears all bindings.  Called by the statement cache on reuse.
    ///
    /// # Safety
    /// `raw` must be a valid, non-finalized `sqlite3_stmt*` associated with
    /// `conn`'s database handle.
    pub(crate) unsafe fn from_raw<'c>(conn: &'c Connection, raw: *mut sqlite3_stmt) -> Statement<'c> {
        unsafe {
            sqlite3_reset(raw);
            sqlite3_clear_bindings(raw);
        }
        let column_count = unsafe { sqlite3_column_count(raw) } as usize;
        let column_names = (0..column_count)
            .map(|i| unsafe {
                let ptr = sqlite3_column_name(raw, i as c_int);
                if ptr.is_null() { String::new() }
                else { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
            })
            .collect();
        Statement { conn, stmt: raw, column_count, column_names }
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

    /// Explicitly finalize the statement, returning any SQLite error.
    ///
    /// Calling this is optional — the statement is finalized on drop — but this
    /// method lets you propagate any deferred error from the last step.
    pub fn finalize(self) -> Result<()> {
        let db = self.conn.handle_ptr();
        let ptr = self.into_raw();
        let rc = unsafe { sqlite3_finalize(ptr) };
        if rc == SQLITE_OK { Ok(()) } else { Err(unsafe { sqlite_error(db, rc) }) }
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

// ── AndThenRows ───────────────────────────────────────────────────────────────

/// Iterator produced by [`Statement::query_and_then`].
///
/// Like `MappedRows` but the closure returns `Result<T, E>` where `E` can be
/// any error type that implements `From<Error>`.
pub struct AndThenRows<'stmt, F> {
    pub(crate) rows: Rows<'stmt>,
    pub(crate) f: F,
}

impl<T, E, F> Iterator for AndThenRows<'_, F>
where
    E: From<Error>,
    F: FnMut(&crate::Row<'_>) -> std::result::Result<T, E>,
{
    type Item = std::result::Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.rows.next() {
            Err(e)            => Some(Err(E::from(e))),
            Ok(None)          => None,
            Ok(Some(ref row)) => Some((self.f)(row)),
        }
    }
}
