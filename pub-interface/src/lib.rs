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
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

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
        Ok(Rows { rows })
    }

    /// Execute a statement with parameterized parameters (safe from SQL injection).
    pub fn execute_with_params(&mut self, sql: &str, params: &[Value]) -> Result<()> {
        self.db.execute_with_params(sql, params)
    }

    /// Execute a query with parameterized parameters (safe from SQL injection).
    pub fn query_with_params(&mut self, sql: &str, params: &[Value]) -> Result<Rows> {
        let rows = self.db.query_with_params(sql, params)?;
        Ok(Rows { rows })
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
        if let Some(row) = rows.iter().next() {
            if let Some(Value::Text(s)) = row.first() {
                return Ok(s.clone());
            }
        }
        Err(Error::Database("failed to set journal_mode".to_string()))
    }

    /// Get journal_mode pragma.
    pub fn journal_mode(&mut self) -> Result<String> {
        let rows = self.query("PRAGMA journal_mode")?;
        if let Some(row) = rows.iter().next() {
            if let Some(Value::Text(s)) = row.first() {
                return Ok(s.clone());
            }
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
            if let Some(Value::Integer(i)) = row.first() {
                return Ok(*i);
            }
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
            if let Some(Value::Integer(i)) = row.first() {
                return Ok(*i);
            }
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
            if let Some(Value::Integer(i)) = row.first() {
                return Ok(*i);
            }
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
            if let Some(Value::Integer(i)) = row.first() {
                return Ok(*i);
            }
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
            if let Some(Value::Integer(i)) = row.first() {
                return Ok(*i);
            }
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
            if let Some(Value::Integer(i)) = row.first() {
                return Ok(*i != 0);
            }
        }
        Err(Error::Database("failed to read pragma".to_string()))
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

/// Query result rows.
#[derive(Debug)]
pub struct Rows {
    rows: Vec<Vec<Value>>,
}

impl Rows {
    /// Iterate over rows.
    pub fn iter(&self) -> impl Iterator<Item = &Vec<Value>> {
        self.rows.iter()
    }
}
