//! Safe SQLite database interface wrapping the pure-Rust C API.
//!
//! This crate provides a rusqlite-like API backed by the statically-linked
//! pure-Rust SQLite implementation, with no external C dependencies.
//!
//! # Safety
//! All unsafe FFI operations are confined to the private `ffi` module.
//! The public API is completely safe.

/// Column value.
#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
    Null,
}

/// Error type.
#[derive(Debug)]
pub enum Error {
    Database(String),
    ColumnIndexOutOfBounds(usize),
    ColumnNotFound(String),
    ColumnTypeMismatch { column: String, expected: &'static str },
    InvalidParameterType,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Database(msg) => write!(f, "database error: {}", msg),
            Error::ColumnIndexOutOfBounds(idx) => write!(f, "column index out of bounds: {}", idx),
            Error::ColumnNotFound(name) => write!(f, "column not found: {}", name),
            Error::ColumnTypeMismatch { column, expected } => {
                write!(f, "column '{}' type mismatch: expected {}", column, expected)
            }
            Error::InvalidParameterType => write!(f, "invalid parameter type"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

/// Trait for converting Value to Rust types.
/// Implement this trait to customize how values are extracted from query results.
pub trait FromValue: Sized {
    /// Extract this type from a Value.
    fn from_value(val: &Value) -> Result<Self>;
}

// Implementations for common types
impl FromValue for i64 {
    fn from_value(val: &Value) -> Result<Self> {
        match val {
            Value::Integer(i) => Ok(*i),
            Value::Null => Err(Error::ColumnTypeMismatch {
                column: "unknown".to_string(),
                expected: "i64",
            }),
            _ => Err(Error::ColumnTypeMismatch {
                column: "unknown".to_string(),
                expected: "i64",
            }),
        }
    }
}

impl FromValue for i32 {
    fn from_value(val: &Value) -> Result<Self> {
        match val {
            Value::Integer(i) => Ok(*i as i32),
            Value::Null => Err(Error::ColumnTypeMismatch {
                column: "unknown".to_string(),
                expected: "i32",
            }),
            _ => Err(Error::ColumnTypeMismatch {
                column: "unknown".to_string(),
                expected: "i32",
            }),
        }
    }
}

impl FromValue for f64 {
    fn from_value(val: &Value) -> Result<Self> {
        match val {
            Value::Real(f) => Ok(*f),
            Value::Integer(i) => Ok(*i as f64),
            Value::Null => Err(Error::ColumnTypeMismatch {
                column: "unknown".to_string(),
                expected: "f64",
            }),
            _ => Err(Error::ColumnTypeMismatch {
                column: "unknown".to_string(),
                expected: "f64",
            }),
        }
    }
}

impl FromValue for String {
    fn from_value(val: &Value) -> Result<Self> {
        match val {
            Value::Text(s) => Ok(s.clone()),
            Value::Null => Err(Error::ColumnTypeMismatch {
                column: "unknown".to_string(),
                expected: "String",
            }),
            _ => Err(Error::ColumnTypeMismatch {
                column: "unknown".to_string(),
                expected: "String",
            }),
        }
    }
}

impl FromValue for Vec<u8> {
    fn from_value(val: &Value) -> Result<Self> {
        match val {
            Value::Blob(b) => Ok(b.clone()),
            Value::Null => Err(Error::ColumnTypeMismatch {
                column: "unknown".to_string(),
                expected: "Vec<u8>",
            }),
            _ => Err(Error::ColumnTypeMismatch {
                column: "unknown".to_string(),
                expected: "Vec<u8>",
            }),
        }
    }
}

impl FromValue for bool {
    fn from_value(val: &Value) -> Result<Self> {
        match val {
            Value::Integer(i) => Ok(*i != 0),
            Value::Null => Err(Error::ColumnTypeMismatch {
                column: "unknown".to_string(),
                expected: "bool",
            }),
            _ => Err(Error::ColumnTypeMismatch {
                column: "unknown".to_string(),
                expected: "bool",
            }),
        }
    }
}

impl<T: FromValue> FromValue for Option<T> {
    fn from_value(val: &Value) -> Result<Self> {
        match val {
            Value::Null => Ok(None),
            _ => T::from_value(val).map(Some),
        }
    }
}

impl FromValue for Value {
    fn from_value(val: &Value) -> Result<Self> {
        Ok(val.clone())
    }
}

mod ffi;

use ffi::Database as FfiDb;

/// A safe database connection (statically linked, C API).
pub struct Connection {
    db: FfiDb,
    in_transaction: bool,
}

impl Connection {
    /// Open or create a database at the given path.
    pub fn open(path: &str) -> Result<Self> {
        let db = FfiDb::open(path)?;
        Ok(Connection {
            db,
            in_transaction: false,
        })
    }

    /// Execute a SQL statement (DDL).
    pub fn execute(&mut self, sql: &str) -> Result<()> {
        self.db.execute(sql)
    }

    /// Execute a query and return rows.
    pub fn query(&mut self, sql: &str) -> Result<Rows> {
        let rows = self.db.query(sql)?;
        Ok(Rows::new(rows, None)) // TODO: Extract column names from query
    }

    /// Execute a statement with parameterized parameters (safe from SQL injection).
    pub fn execute_with_params(&mut self, sql: &str, params: &[Value]) -> Result<()> {
        self.db.execute_with_params(sql, params)
    }

    /// Execute a query with parameterized parameters (safe from SQL injection).
    pub fn query_with_params(&mut self, sql: &str, params: &[Value]) -> Result<Rows> {
        let rows = self.db.query_with_params(sql, params)?;
        Ok(Rows::new(rows, None)) // TODO: Extract column names from query
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
            let i: i64 = row.get(0)?;
            return Ok(i != 0);
        }
        Err(Error::Database("failed to read pragma".to_string()))
    }

    /// Set journal_mode pragma.
    pub fn set_journal_mode(&mut self, mode: &str) -> Result<String> {
        let rows = self.query(&format!("PRAGMA journal_mode = {}", mode))?;
        if let Some(row) = rows.iter().next() {
            return row.get(0);
        }
        Err(Error::Database("failed to set journal_mode".to_string()))
    }

    /// Get journal_mode pragma.
    pub fn journal_mode(&mut self) -> Result<String> {
        let rows = self.query("PRAGMA journal_mode")?;
        if let Some(row) = rows.iter().next() {
            return row.get(0);
        }
        Err(Error::Database("failed to read pragma".to_string()))
    }

    /// Set synchronous pragma (0=OFF, 1=NORMAL, 2=FULL).
    pub fn set_synchronous(&mut self, level: i64) -> Result<()> {
        self.execute(&format!("PRAGMA synchronous = {}", level))
    }

    /// Get synchronous pragma.
    pub fn synchronous(&mut self) -> Result<i64> {
        let rows = self.query("PRAGMA synchronous")?;
        if let Some(row) = rows.iter().next() {
            return row.get(0);
        }
        Err(Error::Database("failed to read pragma".to_string()))
    }

    /// Set cache_size pragma (in pages).
    pub fn set_cache_size(&mut self, pages: i64) -> Result<()> {
        self.execute(&format!("PRAGMA cache_size = {}", pages))
    }

    /// Get cache_size pragma.
    pub fn cache_size(&mut self) -> Result<i64> {
        let rows = self.query("PRAGMA cache_size")?;
        if let Some(row) = rows.iter().next() {
            return row.get(0);
        }
        Err(Error::Database("failed to read pragma".to_string()))
    }

    /// Set page_size pragma (in bytes).
    pub fn set_page_size(&mut self, bytes: i64) -> Result<()> {
        self.execute(&format!("PRAGMA page_size = {}", bytes))
    }

    /// Get page_size pragma.
    pub fn page_size(&mut self) -> Result<i64> {
        let rows = self.query("PRAGMA page_size")?;
        if let Some(row) = rows.iter().next() {
            return row.get(0);
        }
        Err(Error::Database("failed to read pragma".to_string()))
    }

    /// Set user_version pragma.
    pub fn set_user_version(&mut self, version: i64) -> Result<()> {
        self.execute(&format!("PRAGMA user_version = {}", version))
    }

    /// Get user_version pragma.
    pub fn user_version(&mut self) -> Result<i64> {
        let rows = self.query("PRAGMA user_version")?;
        if let Some(row) = rows.iter().next() {
            return row.get(0);
        }
        Err(Error::Database("failed to read pragma".to_string()))
    }

    /// Set application_id pragma.
    pub fn set_application_id(&mut self, id: i64) -> Result<()> {
        self.execute(&format!("PRAGMA application_id = {}", id))
    }

    /// Get application_id pragma.
    pub fn application_id(&mut self) -> Result<i64> {
        let rows = self.query("PRAGMA application_id")?;
        if let Some(row) = rows.iter().next() {
            return row.get(0);
        }
        Err(Error::Database("failed to read pragma".to_string()))
    }

    /// Set query_only pragma.
    pub fn set_query_only(&mut self, enabled: bool) -> Result<()> {
        let value = if enabled { "ON" } else { "OFF" };
        self.execute(&format!("PRAGMA query_only = {}", value))
    }

    /// Get query_only pragma.
    pub fn query_only(&mut self) -> Result<bool> {
        let rows = self.query("PRAGMA query_only")?;
        if let Some(row) = rows.iter().next() {
            let i: i64 = row.get(0)?;
            return Ok(i != 0);
        }
        Err(Error::Database("failed to read pragma".to_string()))
    }

    /// Get table_info for a table. Returns list of column metadata.
    pub fn table_info(&mut self, table: &str) -> Result<Vec<TableColumn>> {
        let rows = self.query(&format!("PRAGMA table_info({})", table))?;
        let mut cols = Vec::new();
        for row in rows.iter() {
            if row.len() >= 6 {
                let cid: i64 = row.get(0)?;
                let name: String = row.get(1)?;
                let col_type: String = row.get(2)?;
                let notnull: i64 = row.get(3)?;
                let dflt_value: Value = row.get(4)?;
                let pk: i64 = row.get(5)?;

                cols.push(TableColumn {
                    cid,
                    name,
                    col_type,
                    notnull: notnull != 0,
                    dflt_value,
                    pk: pk != 0,
                });
            }
        }
        Ok(cols)
    }

    /// Begin a transaction.
    pub fn begin(&mut self) -> Result<()> {
        if self.in_transaction {
            return Err(Error::Database(
                "transaction already in progress".to_string(),
            ));
        }
        self.execute("BEGIN")?;
        self.in_transaction = true;
        Ok(())
    }

    /// Commit the current transaction.
    pub fn commit(&mut self) -> Result<()> {
        if !self.in_transaction {
            return Err(Error::Database("no transaction in progress".to_string()));
        }
        self.execute("COMMIT")?;
        self.in_transaction = false;
        Ok(())
    }

    /// Rollback the current transaction.
    pub fn rollback(&mut self) -> Result<()> {
        if !self.in_transaction {
            return Err(Error::Database("no transaction in progress".to_string()));
        }
        self.execute("ROLLBACK")?;
        self.in_transaction = false;
        Ok(())
    }

    /// Check if a transaction is currently active.
    pub fn in_transaction(&self) -> bool {
        self.in_transaction
    }

    /// Execute a closure within a transaction.
    /// If the closure succeeds, commits. If it fails, automatically rolls back.
    pub fn transaction<F, T>(&mut self, f: F) -> Result<T>
    where
        F: FnOnce(&mut Self) -> Result<T>,
    {
        self.begin()?;
        match f(self) {
            Ok(t) => {
                self.commit()?;
                Ok(t)
            }
            Err(e) => {
                let _ = self.rollback();
                Err(e)
            }
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        // Automatically rollback any active transaction
        if self.in_transaction {
            let _ = self.rollback();
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

/// A single row from a query result.
/// Provides ergonomic access to column values by index or name.
#[derive(Debug)]
pub struct Row {
    values: Vec<Value>,
    column_names: Option<Vec<String>>,
}

impl Row {
    /// Create a new row from values and optional column names.
    fn new(values: Vec<Value>, column_names: Option<Vec<String>>) -> Self {
        Row {
            values,
            column_names,
        }
    }

    /// Get a value by column index.
    ///
    /// # Example
    /// ```ignore
    /// let id: i64 = row.get(0)?;
    /// ```
    pub fn get<T: FromValue>(&self, index: usize) -> Result<T> {
        let val = self
            .values
            .get(index)
            .ok_or(Error::ColumnIndexOutOfBounds(index))?;
        T::from_value(val)
    }

    /// Get a value by column name.
    ///
    /// # Example
    /// ```ignore
    /// let name: String = row.get_named("username")?;
    /// ```
    pub fn get_named<T: FromValue>(&self, col_name: &str) -> Result<T> {
        let index = self
            .column_names
            .as_ref()
            .and_then(|names| names.iter().position(|n| n == col_name))
            .ok_or_else(|| Error::ColumnNotFound(col_name.to_string()))?;

        self.get(index)
    }

    /// Get the number of columns in this row.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Check if the row has no columns.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// Query result rows.
/// Provides iteration over rows with ergonomic access patterns.
#[derive(Debug)]
pub struct Rows {
    rows: Vec<Vec<Value>>,
    column_names: Option<Vec<String>>,
}

impl Rows {
    /// Create a new Rows collection.
    fn new(rows: Vec<Vec<Value>>, column_names: Option<Vec<String>>) -> Self {
        Rows { rows, column_names }
    }

    /// Iterate over rows as references.
    pub fn iter(&self) -> RowIterator {
        RowIterator {
            inner: self.rows.iter(),
            column_names: self.column_names.clone(),
        }
    }

    /// Get a single row by index.
    pub fn get(&self, index: usize) -> Option<Row> {
        self.rows.get(index).map(|values| {
            Row::new(values.clone(), self.column_names.clone())
        })
    }

    /// Get the number of rows.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Check if there are no rows.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

/// Iterator over query result rows.
pub struct RowIterator<'a> {
    inner: std::slice::Iter<'a, Vec<Value>>,
    column_names: Option<Vec<String>>,
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|values| {
            Row::new(values.clone(), self.column_names.clone())
        })
    }
}
