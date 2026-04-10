use crate::{statement::Statement, types::{FromSql, FromSqlError, ValueRef}, Error, Result};
use sqlite_noamalgam::{sqlite3_step, SQLITE_ROW, SQLITE_DONE};

/// A streaming iterator over query result rows.
///
/// Unlike a standard `Iterator`, `next()` returns `Result<Option<Row>>` to
/// propagate SQLite errors.  Use a `while let` loop:
///
/// ```ignore
/// let mut rows = stmt.query(params![])?;
/// while let Some(row) = rows.next()? {
///     let id: i64 = row.get(0)?;
/// }
/// ```
///
/// # Design note
/// `Rows` holds a shared reference (not `&mut`) so that `Statement` is
/// covariant in its connection lifetime, which is necessary to satisfy the
/// borrow checker when returning `Rows<'_>` from `Statement::query`.
/// Exclusive access to the underlying `sqlite3_stmt` pointer is still
/// enforced at the type level: the `Rows` borrow prevents any other use of
/// the statement for its lifetime.
pub struct Rows<'stmt> {
    stmt: &'stmt Statement<'stmt>,
    done: bool,
}

impl<'stmt> Rows<'stmt> {
    /// Create a new `Rows` streaming iterator bound to the given prepared statement.
    ///
    /// The statement must already have parameters bound and must not have been stepped yet;
    /// `Statement::query` is the only public entry point and ensures these preconditions.
    /// Holding a `Rows` value prevents any other use of the statement until it is dropped,
    /// which guarantees that the SQLite column data pointers remain valid for each row's lifetime.
    pub(crate) fn new(stmt: &'stmt Statement<'stmt>) -> Self {
        Rows { stmt, done: false }
    }

    /// Advance to the next row.  Returns `Ok(None)` when exhausted.
    pub fn next(&mut self) -> Result<Option<Row<'_>>> {
        if self.done {
            return Ok(None);
        }
        // SAFETY: We have an exclusive logical borrow on the statement (Rows
        // holds the only live reference for 'stmt).  sqlite3_step only writes
        // to the statement's internal cursor and result columns, which is safe
        // to do through the raw pointer while no Rust &mut exists.
        let rc = unsafe { sqlite3_step(self.stmt.stmt) };
        match rc {
            SQLITE_ROW  => Ok(Some(Row { stmt: self.stmt })),
            SQLITE_DONE => { self.done = true; Ok(None) }
            _           => {
                self.done = true;
                Err(unsafe { crate::error::sqlite_error(self.stmt.conn.handle_ptr(), rc) })
            }
        }
    }

    /// Collect all remaining rows into a `Vec` by applying `f` to each one.
    pub fn map<T, F>(mut self, mut f: F) -> Result<Vec<T>>
    where
        F: FnMut(&Row<'_>) -> Result<T>,
    {
        let mut out = Vec::new();
        while let Some(row) = self.next()? {
            out.push(f(&row)?);
        }
        Ok(out)
    }
}

// ── Row ───────────────────────────────────────────────────────────────────────

/// A reference to a single result row from a query.
///
/// The `Row` borrows the `Statement` immutably.  The column data pointers
/// returned by SQLite are valid only until the next `sqlite3_step` call —
/// which requires `&mut Statement` — so the borrow checker prevents unsafe
/// access automatically.
pub struct Row<'stmt> {
    stmt: &'stmt Statement<'stmt>,
}

impl<'stmt> Row<'stmt> {
    /// Get a column value by index, converting to `T` via `FromSql`.
    pub fn get<I: RowIndex, T: FromSql>(&self, idx: I) -> Result<T> {
        let col = idx.index(self.stmt)?;
        let vref = unsafe { self.stmt.column_value_ref(col)? };
        let col_name = self.stmt.column_name(col)
            .map(|s| s.to_string())
            .unwrap_or_default();
        T::column_result(vref).map_err(|e| match e {
            FromSqlError::InvalidType    =>
                Error::InvalidColumnType(col, col_name, vref.data_type()),
            FromSqlError::OutOfRange(v)  => Error::IntegralValueOutOfRange(col, v),
            FromSqlError::Other(inner)   =>
                Error::FromSqlConversionFailure(col, vref.data_type(), inner),
        })
    }

    /// Like `get`, but panics instead of returning an error.
    pub fn get_unwrap<I: RowIndex, T: FromSql>(&self, idx: I) -> T {
        self.get(idx).unwrap()
    }

    /// Get the raw `ValueRef` for a column — zero-copy, tied to this row's lifetime.
    pub fn get_ref<I: RowIndex>(&self, idx: I) -> Result<ValueRef<'_>> {
        let col = idx.index(self.stmt)?;
        unsafe { self.stmt.column_value_ref(col) }
    }

    /// Like [`get_ref`](Row::get_ref), but panics with a descriptive message instead of returning
    /// an error.
    ///
    /// Useful in tests or in code paths where an invalid column index or unexpected type is a
    /// programming error that should never occur in production.  In production code, prefer
    /// [`get_ref`](Row::get_ref) so that errors are surfaced gracefully.
    pub fn get_ref_unwrap<I: RowIndex>(&self, idx: I) -> ValueRef<'_> {
        self.get_ref(idx).unwrap()
    }

    /// Returns the number of columns projected by the query that produced this row.
    ///
    /// The count is fixed for the lifetime of the statement and reflects the number of
    /// expressions in the `SELECT` list (or all columns for `SELECT *`).  It can be used
    /// to iterate over all columns dynamically without knowing the schema in advance.
    pub fn column_count(&self) -> usize { self.stmt.column_count() }

    /// Returns the declared name of column `col` (0-based), or an error if out of range.
    ///
    /// The name is the alias from the `AS` clause if one was provided, otherwise the column
    /// name from the table definition.  For expressions with no alias, SQLite generates a
    /// name such as `"col0"`.  The returned slice borrows from the statement's metadata and
    /// is valid for the lifetime of the `Row` (and transitively the `Statement`).
    pub fn column_name(&self, col: usize) -> Result<&str> { self.stmt.column_name(col) }
}

// ── RowIndex trait ────────────────────────────────────────────────────────────

mod private { pub trait Sealed {} }

/// Trait for things that can index into a row's columns.
pub trait RowIndex: private::Sealed {
    fn index(&self, stmt: &Statement<'_>) -> Result<usize>;
}

impl private::Sealed for usize {}
impl RowIndex for usize {
    fn index(&self, stmt: &Statement<'_>) -> Result<usize> {
        if *self >= stmt.column_count() {
            Err(Error::InvalidColumnIndex(*self))
        } else {
            Ok(*self)
        }
    }
}

impl private::Sealed for &str {}
impl RowIndex for &str {
    fn index(&self, stmt: &Statement<'_>) -> Result<usize> {
        stmt.column_index(self)
    }
}

impl private::Sealed for String {}
impl RowIndex for String {
    fn index(&self, stmt: &Statement<'_>) -> Result<usize> {
        stmt.column_index(self.as_str())
    }
}

impl private::Sealed for i32 {}
impl RowIndex for i32 {
    fn index(&self, stmt: &Statement<'_>) -> Result<usize> {
        if *self < 0 {
            return Err(Error::InvalidColumnIndex(*self as usize));
        }
        (*self as usize).index(stmt)
    }
}
