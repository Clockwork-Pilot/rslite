use std::ffi::{CStr, CString};
use std::path::Path;
use sqlite_noamalgam::*;
use crate::{
    error::{sqlite_error, sqlite_error_from_code},
    functions::{self, Aggregate, Context, FunctionFlags},
    hooks::{
        Action, HookSlot,
        commit_trampoline, rollback_trampoline, update_trampoline,
    },
    transaction::{Transaction, TransactionBehavior, Savepoint},
    types::ToSql,
    Params, Result, Statement,
};

bitflags::bitflags! {
    /// Flags for `Connection::open_with_flags`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct OpenFlags: i32 {
        const READONLY      = SQLITE_OPEN_READONLY;
        const READWRITE     = SQLITE_OPEN_READWRITE;
        const CREATE        = SQLITE_OPEN_CREATE;
        const URI           = SQLITE_OPEN_URI;
        const MEMORY        = SQLITE_OPEN_MEMORY;
        const NO_MUTEX      = SQLITE_OPEN_NOMUTEX;
        const FULL_MUTEX    = SQLITE_OPEN_FULLMUTEX;
        const SHARED_CACHE  = SQLITE_OPEN_SHAREDCACHE;
        const PRIVATE_CACHE = SQLITE_OPEN_PRIVATECACHE;
        const NOFOLLOW      = SQLITE_OPEN_NOFOLLOW;
        const EXRESCODE     = SQLITE_OPEN_EXRESCODE;
    }
}

impl Default for OpenFlags {
    fn default() -> Self {
        OpenFlags::READWRITE | OpenFlags::CREATE
    }
}

/// A SQLite database connection.
///
/// # Thread safety
/// `Connection` is `Send` but `!Sync`.  It can be moved between threads, but
/// must only be used from one thread at a time.  For shared access, wrap it in
/// `Mutex<Connection>`.
pub struct Connection {
    pub(crate) db: *mut sqlite3,
    // Keeps hook closures alive for the lifetime of the connection.
    commit_hook:   Option<HookSlot>,
    rollback_hook: Option<HookSlot>,
    update_hook:   Option<HookSlot>,
}

impl std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection")
            .field("path", &self.path())
            .finish_non_exhaustive()
    }
}

// A sqlite3* can be safely moved to another thread — the restriction is
// *concurrent* access (Sync), not ownership transfer (Send).
// SAFETY: SQLite connections are not thread-safe for concurrent access, but
// they can be moved between threads as long as only one thread uses them at
// a time.  We implement Send but intentionally leave Sync unimplemented.
unsafe impl Send for Connection {}

impl Connection {
    // ── Constructors ──────────────────────────────────────────────────────────

    /// Open or create a database file.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref()
            .to_str()
            .ok_or_else(|| crate::Error::InvalidPath(path.as_ref().to_owned()))?;
        Self::open_with_flags(path_str, OpenFlags::default())
    }

    /// Open an in-memory database.
    pub fn open_in_memory() -> Result<Self> {
        Self::open_with_flags(":memory:", OpenFlags::default())
    }

    /// Open with explicit flags.
    pub fn open_with_flags<P: AsRef<Path>>(path: P, flags: OpenFlags) -> Result<Self> {
        let path_str = path.as_ref()
            .to_str()
            .ok_or_else(|| crate::Error::InvalidPath(path.as_ref().to_owned()))?;
        let c_path = CString::new(path_str)?;
        let mut db = std::ptr::null_mut();
        let rc = unsafe {
            sqlite3_open_v2(c_path.as_ptr(), &mut db, flags.bits(), std::ptr::null())
        };
        if rc != SQLITE_OK {
            let err = unsafe { sqlite_error(db, rc) };
            unsafe { sqlite3_close(db); }
            return Err(err);
        }
        Ok(Connection { db, commit_hook: None, rollback_hook: None, update_hook: None })
    }

    /// Close the connection explicitly, returning the error if closing fails.
    pub fn close(self) -> std::result::Result<(), (Self, crate::Error)> {
        let rc = unsafe { sqlite3_close(self.db) };
        if rc == SQLITE_OK {
            std::mem::forget(self);
            Ok(())
        } else {
            Err((self, sqlite_error_from_code(rc)))
        }
    }

    // ── Core query API ────────────────────────────────────────────────────────

    /// Prepare a SQL statement for repeated execution.
    pub fn prepare(&self, sql: &str) -> Result<Statement<'_>> {
        Statement::prepare(self, sql)
    }

    /// Execute a single SQL statement (no rows returned).  Returns the number
    /// of rows changed.
    pub fn execute<P: Params>(&self, sql: &str, params: P) -> Result<usize> {
        self.prepare(sql)?.execute(params)
    }

    /// Execute one or more semicolon-separated SQL statements with no parameters.
    pub fn execute_batch(&self, sql: &str) -> Result<()> {
        let c_sql = CString::new(sql)?;
        let mut errmsg: *mut std::ffi::c_char = std::ptr::null_mut();
        let rc = unsafe {
            sqlite3_exec(self.db, c_sql.as_ptr(), None, std::ptr::null_mut(), &mut errmsg)
        };
        if rc != SQLITE_OK {
            let msg = if !errmsg.is_null() {
                let s = unsafe { CStr::from_ptr(errmsg).to_string_lossy().into_owned() };
                unsafe { sqlite3_free(errmsg as *mut std::ffi::c_void); }
                Some(s)
            } else {
                None
            };
            return Err(crate::Error::SqliteFailure(
                crate::error::SqliteError::from_code(rc), msg,
            ));
        }
        Ok(())
    }

    /// Prepare and execute a query, applying `f` to the first row.
    /// Returns `Error::QueryReturnedNoRows` if no row was produced.
    pub fn query_row<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<T>
    where
        P: Params,
        F: FnOnce(&crate::Row<'_>) -> Result<T>,
    {
        self.prepare(sql)?.query_row(params, f)
    }

    /// Execute a query that returns exactly one row and one column.
    /// Shorthand for `query_row(sql, (), |r| r.get(0))`.
    pub fn one_column<T: crate::types::FromSql>(&self, sql: &str) -> Result<T> {
        self.query_row(sql, (), |r| r.get(0))
    }

    /// Prepare and execute a query, returning `Ok(None)` if no rows.
    pub fn query_row_optional<T, P, F>(&self, sql: &str, params: P, f: F) -> Result<Option<T>>
    where
        P: Params,
        F: FnOnce(&crate::Row<'_>) -> Result<T>,
    {
        match self.prepare(sql)?.query_row(params, f) {
            Ok(v)                           => Ok(Some(v)),
            Err(crate::Error::QueryReturnedNoRows) => Ok(None),
            Err(e)                          => Err(e),
        }
    }

    // ── Transactions ──────────────────────────────────────────────────────────

    /// Begin a deferred transaction.
    pub fn transaction(&mut self) -> Result<Transaction<'_>> {
        Transaction::new(self, TransactionBehavior::Deferred)
    }

    /// Begin a transaction with explicit behavior.
    pub fn transaction_with_behavior(&mut self, behavior: TransactionBehavior) -> Result<Transaction<'_>> {
        Transaction::new(self, behavior)
    }

    /// Create a savepoint (can be used outside of an explicit transaction).
    pub fn savepoint(&mut self) -> Result<Savepoint<'_>> {
        Savepoint::new(self)
    }

    /// Create a savepoint with an explicit name.
    pub fn savepoint_with_name<N: Into<String>>(&mut self, name: N) -> Result<Savepoint<'_>> {
        Savepoint::with_name(self, name.into())
    }

    // ── Connection info ───────────────────────────────────────────────────────

    pub fn last_insert_rowid(&self) -> i64 {
        unsafe { sqlite3_last_insert_rowid(self.db) }
    }

    pub fn changes(&self) -> u64 {
        unsafe { sqlite3_changes64(self.db) as u64 }
    }

    pub fn total_changes(&self) -> u64 {
        unsafe { sqlite3_total_changes64(self.db) as u64 }
    }

    pub fn is_autocommit(&self) -> bool {
        unsafe { sqlite3_get_autocommit(self.db) != 0 }
    }

    pub fn path(&self) -> Option<&str> {
        unsafe {
            let ptr = sqlite3_db_filename(self.db, b"main\0".as_ptr() as *const _);
            if ptr.is_null() { None }
            else { CStr::from_ptr(ptr).to_str().ok() }
        }
    }

    // ── Pragmas ───────────────────────────────────────────────────────────────

    /// Set a pragma value.
    ///
    /// ```ignore
    /// conn.pragma_update(None, "journal_mode", "WAL")?;
    /// ```
    pub fn pragma_update<V: ToSql>(&self, schema: Option<&str>, name: &str, value: V) -> Result<()> {
        // SQLite PRAGMA statements don't support bound parameters, so we inline
        // the value as a literal.
        let val = value.to_sql()
            .map_err(|e| crate::Error::ToSqlConversionFailure(Box::new(e)))?;
        let literal = match val.as_value_ref() {
            crate::types::ValueRef::Integer(i) => i.to_string(),
            crate::types::ValueRef::Real(f)    => f.to_string(),
            crate::types::ValueRef::Text(s)    => format!("'{}'", s.replace('\'', "''")),
            crate::types::ValueRef::Blob(_)    => return Err(crate::Error::InvalidParameterName(
                "blob values cannot be used as pragma values".to_string()
            )),
            crate::types::ValueRef::Null       => "NULL".to_string(),
        };
        let sql = match schema {
            Some(s) => format!("PRAGMA \"{}\".\"{}\" = {}", s, name, literal),
            None    => format!("PRAGMA \"{}\" = {}", name, literal),
        };
        self.execute_batch(&sql)
    }

    /// Read a single pragma value.
    pub fn pragma_query_value<T, F>(&self, schema: Option<&str>, name: &str, f: F) -> Result<T>
    where
        F: FnOnce(&crate::Row<'_>) -> Result<T>,
    {
        let sql = match schema {
            Some(s) => format!("PRAGMA \"{}\".\"{}\"", s, name),
            None    => format!("PRAGMA \"{}\"", name),
        };
        self.query_row(&sql, (), f)
    }

    // ── Timeouts & interrupts ─────────────────────────────────────────────────

    pub fn busy_timeout(&self, timeout: std::time::Duration) -> Result<()> {
        let ms = timeout.as_millis() as i32;
        let rc = unsafe { sqlite3_busy_timeout(self.db, ms) };
        if rc == SQLITE_OK { Ok(()) } else { Err(unsafe { sqlite_error(self.db, rc) }) }
    }

    pub fn interrupt(&self) {
        unsafe { sqlite3_interrupt(self.db); }
    }

    pub fn is_busy(&self) -> bool {
        unsafe { sqlite3_is_interrupted(self.db) != 0 }
    }

    // ── User-defined functions ────────────────────────────────────────────────

    /// Register a scalar SQL function.
    ///
    /// `n_args` is the number of arguments the function accepts, or `-1` for
    /// variadic.  Pass [`FunctionFlags::DETERMINISTIC`] when the function
    /// always returns the same result for the same inputs.
    ///
    /// ```ignore
    /// conn.create_scalar_function("double", 1, FunctionFlags::DETERMINISTIC, |ctx| {
    ///     Ok(ctx.get::<i64>(0)? * 2)
    /// })?;
    /// let n: i64 = conn.one_column("SELECT double(21)")?;
    /// assert_eq!(n, 42);
    /// ```
    pub fn create_scalar_function<F, T>(
        &self,
        name:   &str,
        n_args: i32,
        flags:  FunctionFlags,
        f:      F,
    ) -> Result<()>
    where
        F: FnMut(&Context<'_>) -> Result<T> + Send + 'static,
        T: ToSql,
    {
        functions::register_scalar(self.db, name, n_args, flags, f)
    }

    /// Register an aggregate SQL function.
    ///
    /// `agg` must implement [`Aggregate<A, T>`].
    pub fn create_aggregate_function<D, A, T>(
        &self,
        name:   &str,
        n_args: i32,
        flags:  FunctionFlags,
        agg:    D,
    ) -> Result<()>
    where
        D: Aggregate<A, T>,
        T: ToSql,
    {
        functions::register_aggregate(self.db, name, n_args, flags, agg)
    }

    /// Remove a previously registered scalar or aggregate function.
    pub fn remove_function(&self, name: &str, n_args: i32) -> Result<()> {
        functions::deregister(self.db, name, n_args)
    }

    // ── Hooks ─────────────────────────────────────────────────────────────────

    /// Register a commit hook.
    ///
    /// `f` is called just before each transaction commits.  Return `true` to
    /// convert the commit into a rollback, `false` to allow it.
    ///
    /// Pass `None` to remove the current hook.
    pub fn commit_hook<F>(&mut self, hook: Option<F>)
    where
        F: FnMut() -> bool + Send + 'static,
    {
        match hook {
            None => {
                self.commit_hook = None;
                unsafe { sqlite3_commit_hook(self.db, None, std::ptr::null_mut()); }
            }
            Some(f) => {
                let (slot, raw) = HookSlot::new(f);
                self.commit_hook = Some(slot);
                unsafe {
                    sqlite3_commit_hook(
                        self.db,
                        Some(commit_trampoline::<F>),
                        raw as *mut std::ffi::c_void,
                    );
                }
            }
        }
    }

    /// Register a rollback hook.
    ///
    /// `f` is called whenever a transaction is rolled back.
    ///
    /// Pass `None` to remove the current hook.
    pub fn rollback_hook<F>(&mut self, hook: Option<F>)
    where
        F: FnMut() + Send + 'static,
    {
        match hook {
            None => {
                self.rollback_hook = None;
                unsafe { sqlite3_rollback_hook(self.db, None, std::ptr::null_mut()); }
            }
            Some(f) => {
                let (slot, raw) = HookSlot::new(f);
                self.rollback_hook = Some(slot);
                unsafe {
                    sqlite3_rollback_hook(
                        self.db,
                        Some(rollback_trampoline::<F>),
                        raw as *mut std::ffi::c_void,
                    );
                }
            }
        }
    }

    /// Register an update hook.
    ///
    /// `f` is called after every INSERT, UPDATE, or DELETE.  Arguments are:
    /// the [`Action`], the database name, the table name, and the affected
    /// row's `rowid`.
    ///
    /// Pass `None` to remove the current hook.
    pub fn update_hook<F>(&mut self, hook: Option<F>)
    where
        F: FnMut(Action, &str, &str, i64) + Send + 'static,
    {
        match hook {
            None => {
                self.update_hook = None;
                unsafe { sqlite3_update_hook(self.db, None, std::ptr::null_mut()); }
            }
            Some(f) => {
                let (slot, raw) = HookSlot::new(f);
                self.update_hook = Some(slot);
                unsafe {
                    sqlite3_update_hook(
                        self.db,
                        Some(update_trampoline::<F>),
                        raw as *mut std::ffi::c_void,
                    );
                }
            }
        }
    }

    // ── Raw handle ────────────────────────────────────────────────────────────

    /// Access the raw `sqlite3*`.  Use with care — calling unsafe SQLite
    /// functions directly can violate rslite's safety invariants.
    pub unsafe fn handle(&self) -> *mut sqlite3 { self.db }

    pub(crate) fn handle_ptr(&self) -> *mut sqlite3 { self.db }
}

impl Drop for Connection {
    fn drop(&mut self) {
        // sqlite3_close_v2 defers close until all prepared statements are finalized.
        unsafe { sqlite3_close_v2(self.db); }
    }
}
