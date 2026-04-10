use std::cell::RefCell;
use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_void};
use std::path::Path;
use sqlite_noamalgam::*;
use crate::ffi::{
    SQLITE_CREATE_INDEX, SQLITE_CREATE_TABLE, SQLITE_CREATE_TRIGGER, SQLITE_CREATE_VIEW,
    SQLITE_DELETE, SQLITE_DROP_INDEX, SQLITE_DROP_TABLE, SQLITE_DROP_TRIGGER, SQLITE_DROP_VIEW,
    SQLITE_INSERT, SQLITE_READ, SQLITE_SELECT, SQLITE_TRANSACTION, SQLITE_UPDATE,
    SQLITE_ATTACH, SQLITE_DETACH, SQLITE_ALTER_TABLE, SQLITE_REINDEX, SQLITE_ANALYZE,
    SQLITE_CREATE_VTABLE, SQLITE_DROP_VTABLE, SQLITE_FUNCTION, SQLITE_SAVEPOINT, SQLITE_RECURSIVE,
    SQLITE_DENY, SQLITE_IGNORE,
};
use crate::{
    cache::{CachedStatement, StatementCache},
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

/// SQLite runtime limit categories, used with [`Connection::limit`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LimitCategory {
    /// Maximum string or BLOB length.
    Length              = 0,
    /// Maximum length of an SQL statement.
    SqlLength           = 1,
    /// Maximum number of columns in a table definition or SELECT.
    Column              = 2,
    /// Maximum depth of the parse tree on any single expression.
    ExprDepth           = 3,
    /// Maximum number of terms in a compound SELECT statement.
    CompoundSelect      = 4,
    /// Maximum number of instructions in a compiled VDBE program.
    VdbeOp              = 5,
    /// Maximum number of arguments on a SQL function.
    FunctionArg         = 6,
    /// Maximum number of ATTACH'ed databases.
    Attached            = 7,
    /// Maximum length of the pattern argument to the LIKE or GLOB operator.
    LikePatternLength   = 8,
    /// Maximum index number of any parameter in an SQL statement.
    VariableNumber      = 9,
    /// Maximum depth of recursion for triggers.
    TriggerDepth        = 10,
    /// Maximum number of auxiliary worker threads.
    WorkerThreads       = 11,
}

/// Authorization action, passed to the callback registered with
/// [`Connection::authorizer`].
///
/// Mirrors SQLite's `SQLITE_*` action codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuthAction<'c> {
    CreateIndex       { index_name: Option<&'c str>, table_name: Option<&'c str> },
    CreateTable       { table_name: Option<&'c str> },
    CreateTempIndex   { index_name: Option<&'c str>, table_name: Option<&'c str> },
    CreateTempTable   { table_name: Option<&'c str> },
    CreateTempTrigger { trigger_name: Option<&'c str>, table_name: Option<&'c str> },
    CreateTempView    { view_name: Option<&'c str> },
    CreateTrigger     { trigger_name: Option<&'c str>, table_name: Option<&'c str> },
    CreateView        { view_name: Option<&'c str> },
    Delete            { table_name: Option<&'c str> },
    DropIndex         { index_name: Option<&'c str>, table_name: Option<&'c str> },
    DropTable         { table_name: Option<&'c str> },
    DropTempIndex     { index_name: Option<&'c str>, table_name: Option<&'c str> },
    DropTempTable     { table_name: Option<&'c str> },
    DropTempTrigger   { trigger_name: Option<&'c str> },
    DropTempView      { view_name: Option<&'c str> },
    DropTrigger       { trigger_name: Option<&'c str>, table_name: Option<&'c str> },
    DropView          { view_name: Option<&'c str> },
    Insert            { table_name: Option<&'c str> },
    Pragma            { pragma_name: Option<&'c str>, pragma_value: Option<&'c str> },
    Read              { table_name: Option<&'c str>, column_name: Option<&'c str> },
    Select,
    Transaction       { operation: Option<&'c str> },
    Update            { table_name: Option<&'c str>, column_name: Option<&'c str> },
    Attach            { filename: Option<&'c str> },
    Detach            { database_name: Option<&'c str> },
    AlterTable        { database_name: Option<&'c str>, table_name: Option<&'c str> },
    Reindex           { index_name: Option<&'c str> },
    Analyze           { table_name: Option<&'c str> },
    CreateVtable      { table_name: Option<&'c str>, module_name: Option<&'c str> },
    DropVtable        { table_name: Option<&'c str>, module_name: Option<&'c str> },
    Function          { function_name: Option<&'c str> },
    Savepoint         { operation: Option<&'c str>, savepoint_name: Option<&'c str> },
    Recursive,
    Unknown(c_int),
}

/// The return value from an authorizer callback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Authorization {
    /// Allow the action.
    Allow,
    /// Deny the action (returns `SQLITE_AUTH`).
    Deny,
    /// Ignore (treat the object as if it did not exist).
    Ignore,
}

impl Authorization {
    fn as_i32(self) -> c_int {
        match self {
            Authorization::Allow  => 0,  // SQLITE_OK
            Authorization::Deny   => SQLITE_DENY,
            Authorization::Ignore => SQLITE_IGNORE,
        }
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
    // Statement cache for prepare_cached.
    cache: RefCell<StatementCache>,
    // Keeps hook closures alive for the lifetime of the connection.
    commit_hook:    Option<HookSlot>,
    rollback_hook:  Option<HookSlot>,
    update_hook:    Option<HookSlot>,
    // Keeps the busy handler closure alive.
    busy_handler:   Option<Box<dyn std::any::Any + Send>>,
    // Keeps the authorizer closure alive.
    authorizer_hook: Option<Box<dyn std::any::Any + Send>>,
    // Keeps trace closures alive.
    trace_hook:     Option<Box<dyn std::any::Any + Send>>,
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
        Ok(Connection {
            db,
            cache: RefCell::new(StatementCache::new(8)),
            commit_hook: None,
            rollback_hook: None,
            update_hook: None,
            busy_handler: None,
            authorizer_hook: None,
            trace_hook: None,
        })
    }

    /// Close the connection explicitly, returning the error if closing fails.
    pub fn close(self) -> std::result::Result<(), (Self, crate::Error)> {
        // Flush the cache first so all statements are finalized.
        self.cache.borrow_mut().flush();
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

    /// Prepare a SQL statement, returning a cached version if one is available.
    ///
    /// On drop, the statement is returned to the cache instead of being
    /// finalized, so the next call with the same SQL string reuses the
    /// already-compiled statement.
    pub fn prepare_cached(&self, sql: &str) -> Result<CachedStatement<'_>> {
        let raw = self.cache.borrow_mut().get(sql);
        let stmt = if let Some(ptr) = raw {
            unsafe { Statement::from_raw(self, ptr) }
        } else {
            self.prepare(sql)?
        };
        Ok(CachedStatement::new(stmt, self, sql.to_string()))
    }

    /// Set the capacity of the prepared-statement cache (default 8).
    ///
    /// Setting capacity to 0 disables the cache; previously-cached statements
    /// are finalized immediately.
    pub fn set_prepared_statement_cache_capacity(&self, capacity: usize) {
        self.cache.borrow_mut().set_capacity(capacity);
    }

    /// Flush all cached prepared statements, finalizing them.
    pub fn cache_flush(&self) {
        self.cache.borrow_mut().flush();
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

    /// Like `query_row` but the closure may return any error type `E` that
    /// implements `From<Error>`.
    pub fn query_row_and_then<T, E, P, F>(&self, sql: &str, params: P, f: F)
        -> std::result::Result<T, E>
    where
        P: Params,
        E: From<crate::Error>,
        F: FnOnce(&crate::Row<'_>) -> std::result::Result<T, E>,
    {
        let mut stmt = self.prepare(sql).map_err(E::from)?;
        let mut rows = stmt.query(params).map_err(E::from)?;
        match rows.next().map_err(E::from)? {
            Some(row) => f(&row),
            None      => Err(E::from(crate::Error::QueryReturnedNoRows)),
        }
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

    /// Returns the rowid assigned to the most recent successful `INSERT` statement on this
    /// connection, as reported by `sqlite3_last_insert_rowid`.
    ///
    /// If the inserted table has no `INTEGER PRIMARY KEY` column, SQLite assigns an implicit
    /// rowid.  The value is connection-local and unaffected by inserts on other connections.
    /// Returns 0 if no `INSERT` has been performed on this connection since it was opened.
    pub fn last_insert_rowid(&self) -> i64 {
        unsafe { sqlite3_last_insert_rowid(self.db) }
    }

    /// Returns the number of rows inserted, updated, or deleted by the most recent DML statement
    /// executed on this connection, via `sqlite3_changes64`.
    ///
    /// Auxiliary changes caused by triggers or foreign-key cascade actions are not counted here;
    /// use [`total_changes`](Connection::total_changes) for a cumulative count that includes
    /// those.  The counter is reset to 0 for each new DML statement, so the value reflects only
    /// the immediately preceding operation.
    pub fn changes(&self) -> u64 {
        unsafe { sqlite3_changes64(self.db) as u64 }
    }

    /// Returns the cumulative number of rows inserted, updated, or deleted since this connection
    /// was opened, as reported by `sqlite3_total_changes64`.
    ///
    /// Unlike [`changes`](Connection::changes), this counter accumulates across all statements
    /// and is not reset between individual DML operations.  It includes changes caused by
    /// triggers but excludes changes performed through other database connections, making it
    /// suitable for progress tracking within a single session.
    pub fn total_changes(&self) -> u64 {
        unsafe { sqlite3_total_changes64(self.db) as u64 }
    }

    /// Returns `true` if the connection is operating in autocommit mode, i.e. not currently
    /// inside an explicit `BEGIN`…`COMMIT` or `BEGIN`…`ROLLBACK` block.
    ///
    /// SQLite starts every connection in autocommit mode.  Calling `BEGIN` switches autocommit
    /// off; it is restored when the transaction is committed or rolled back.  This method wraps
    /// `sqlite3_get_autocommit` and can be used to detect whether a transaction is already open
    /// before calling [`transaction`](Connection::transaction).
    pub fn is_autocommit(&self) -> bool {
        unsafe { sqlite3_get_autocommit(self.db) != 0 }
    }

    /// Returns the file path of the `main` attached database, or `None` for in-memory databases
    /// and databases opened with an empty string path.
    ///
    /// Calls `sqlite3_db_filename` with the schema name `"main"`.  The returned slice borrows
    /// from SQLite's internal state and is valid for the lifetime of the connection.  For URI
    /// filenames (opened with `SQLITE_OPEN_URI`) the raw URI string is returned rather than the
    /// resolved path.
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

    /// Execute a pragma query, calling `f` for each result row.
    ///
    /// Unlike `pragma_query_value` this works for pragmas that return multiple
    /// rows (e.g. `PRAGMA table_info`).
    pub fn pragma_query<F>(&self, schema: Option<&str>, name: &str, mut f: F) -> Result<()>
    where
        F: FnMut(&crate::Row<'_>) -> Result<()>,
    {
        let sql = match schema {
            Some(s) => format!("PRAGMA \"{}\".\"{}\"", s, name),
            None    => format!("PRAGMA \"{}\"", name),
        };
        let mut stmt = self.prepare(&sql)?;
        let mut rows = stmt.query(())?;
        while let Some(row) = rows.next()? {
            f(&row)?;
        }
        Ok(())
    }

    // ── Timeouts & interrupts ─────────────────────────────────────────────────

    /// Set a busy-wait timeout: SQLite will automatically retry operations blocked by a lock
    /// for up to `timeout` before returning `SQLITE_BUSY`.
    ///
    /// Delegates to `sqlite3_busy_timeout` with the duration converted to milliseconds.
    /// Setting a timeout overrides any previously registered busy handler (set via
    /// [`busy_handler`](Connection::busy_handler)) and vice versa; only one mechanism is
    /// active at a time.  Pass `Duration::ZERO` to disable the built-in retry behaviour.
    pub fn busy_timeout(&self, timeout: std::time::Duration) -> Result<()> {
        let ms = timeout.as_millis() as i32;
        let rc = unsafe { sqlite3_busy_timeout(self.db, ms) };
        if rc == SQLITE_OK { Ok(()) } else { Err(unsafe { sqlite_error(self.db, rc) }) }
    }

    /// Set a custom busy handler that is invoked when a table is locked.
    ///
    /// `handler(n)` is called with the number of prior invocations.  Return
    /// `true` to retry the operation, `false` to return `SQLITE_BUSY`.
    ///
    /// Pass `None` to remove the current handler.
    ///
    /// Note: setting a busy handler with this method overrides any timeout
    /// set by `busy_timeout`, and vice versa.
    pub fn busy_handler<F>(&mut self, handler: Option<F>) -> Result<()>
    where
        F: FnMut(i32) -> bool + Send + 'static,
    {
        match handler {
            None => {
                self.busy_handler = None;
                let rc = unsafe { sqlite3_busy_handler(self.db, None, std::ptr::null_mut()) };
                if rc == SQLITE_OK { Ok(()) } else { Err(unsafe { sqlite_error(self.db, rc) }) }
            }
            Some(f) => {
                unsafe extern "C" fn trampoline<F: FnMut(i32) -> bool>(
                    p: *mut c_void,
                    n: c_int,
                ) -> c_int {
                    let f = unsafe { &mut *(p as *mut F) };
                    if f(n) { 1 } else { 0 }
                }

                // Heap-allocate f and get a stable raw pointer.
                let raw = Box::into_raw(Box::new(f));
                let rc = unsafe {
                    sqlite3_busy_handler(self.db, Some(trampoline::<F>), raw as *mut c_void)
                };
                if rc != SQLITE_OK {
                    unsafe { drop(Box::from_raw(raw)); }
                    return Err(unsafe { sqlite_error(self.db, rc) });
                }
                // Re-box to keep alive; the heap address is unchanged.
                self.busy_handler = Some(unsafe { Box::from_raw(raw) as Box<dyn std::any::Any + Send> });
                Ok(())
            }
        }
    }

    /// Request that any pending SQLite operation on this connection be interrupted as soon as
    /// possible, causing it to return `SQLITE_INTERRUPT`.
    ///
    /// This method is safe to call from a different thread than the one running the query.
    /// It sets an internal flag that is checked by the SQLite VDBE between opcodes; the
    /// actual interrupt may not take effect immediately but will occur at the next safe
    /// point.  The flag is automatically cleared once SQLite processes the interrupt.
    pub fn interrupt(&self) {
        unsafe { sqlite3_interrupt(self.db); }
    }

    /// Returns `true` if [`interrupt`](Connection::interrupt) has been called and the interrupt
    /// flag has not yet been consumed by the SQLite engine.
    ///
    /// Wraps `sqlite3_is_interrupted`, which was added in SQLite 3.41.0.  The flag is set by
    /// [`interrupt`](Connection::interrupt) and cleared once SQLite acts on it.  This can be
    /// used to poll whether a cancellation request is still pending, though in practice it is
    /// rarely needed outside of debugging and testing scenarios.
    pub fn is_busy(&self) -> bool {
        unsafe { sqlite3_is_interrupted(self.db) != 0 }
    }

    // ── Limits ────────────────────────────────────────────────────────────────

    /// Get or set a SQLite runtime limit.
    ///
    /// If `new_val` is negative, the limit is unchanged and the current value
    /// is returned.  Otherwise the limit is set to `new_val` and the previous
    /// value is returned.
    pub fn limit(&self, category: LimitCategory, new_val: i32) -> i32 {
        unsafe { sqlite3_limit(self.db, category as c_int, new_val) }
    }

    // ── Collations ────────────────────────────────────────────────────────────

    /// Register a custom text collation.
    ///
    /// `compare(a, b)` must implement a consistent ordering that matches
    /// SQLite's collation contract.
    ///
    /// ```ignore
    /// conn.create_collation("NOCASE_UTF8", |a, b| {
    ///     a.to_lowercase().cmp(&b.to_lowercase())
    /// })?;
    /// ```
    pub fn create_collation<F>(&mut self, name: &str, compare: F) -> Result<()>
    where
        F: Fn(&str, &str) -> Ordering + Send + 'static,
    {
        let c_name = CString::new(name)?;
        let boxed: Box<F> = Box::new(compare);
        let raw = Box::into_raw(boxed) as *mut c_void;

        unsafe extern "C" fn collation_trampoline<F: Fn(&str, &str) -> Ordering>(
            p:    *mut c_void,
            la:   c_int,
            pa:   *const c_void,
            lb:   c_int,
            pb:   *const c_void,
        ) -> c_int {
            let f = unsafe { &*(p as *const F) };
            let a = unsafe {
                std::str::from_utf8(std::slice::from_raw_parts(pa as *const u8, la as usize))
                    .unwrap_or("")
            };
            let b = unsafe {
                std::str::from_utf8(std::slice::from_raw_parts(pb as *const u8, lb as usize))
                    .unwrap_or("")
            };
            match f(a, b) {
                Ordering::Less    => -1,
                Ordering::Equal   =>  0,
                Ordering::Greater =>  1,
            }
        }

        unsafe extern "C" fn collation_destroy<F>(p: *mut c_void) {
            unsafe { drop(Box::from_raw(p as *mut F)); }
        }

        let rc = unsafe {
            sqlite3_create_collation_v2(
                self.db,
                c_name.as_ptr(),
                SQLITE_UTF8 as c_int,
                raw,
                Some(collation_trampoline::<F>),
                Some(collation_destroy::<F>),
            )
        };
        if rc != SQLITE_OK {
            unsafe { drop(Box::from_raw(raw as *mut F)); }
            return Err(unsafe { sqlite_error(self.db, rc) });
        }
        // The destroy callback will free the Box, so we don't store it here.
        // (We *could* store it but then we'd double-free.)
        Ok(())
    }

    // ── Authorizer ───────────────────────────────────────────────────────────

    /// Register an authorizer callback.
    ///
    /// The callback is invoked for every SQL action during statement
    /// preparation.  It returns [`Authorization::Allow`], [`Authorization::Deny`],
    /// or [`Authorization::Ignore`].
    ///
    /// Pass `None` to remove the current authorizer.
    pub fn authorizer<F>(&mut self, authorizer: Option<F>)
    where
        F: Fn(AuthAction<'_>) -> Authorization + Send + 'static,
    {
        match authorizer {
            None => {
                self.authorizer_hook = None;
                unsafe { sqlite3_set_authorizer(self.db, None, std::ptr::null_mut()); }
            }
            Some(f) => {
                let boxed: Box<F> = Box::new(f);
                let raw = Box::into_raw(boxed) as *mut c_void;

                unsafe extern "C" fn auth_trampoline<F>(
                    p:  *mut c_void,
                    action_code: c_int,
                    s1: *const std::ffi::c_char,
                    s2: *const std::ffi::c_char,
                    _s3: *const std::ffi::c_char,
                    _s4: *const std::ffi::c_char,
                ) -> c_int
                where
                    F: Fn(AuthAction<'_>) -> Authorization,
                {
                    let f = unsafe { &*(p as *const F) };

                    fn opt_str<'a>(p: *const std::ffi::c_char) -> Option<&'a str> {
                        if p.is_null() { return None; }
                        unsafe { CStr::from_ptr(p).to_str().ok() }
                    }

                    let s1 = opt_str(s1);
                    let s2 = opt_str(s2);

                    let action = match action_code {
                        SQLITE_CREATE_INDEX   => AuthAction::CreateIndex   { index_name: s1, table_name: s2 },
                        SQLITE_CREATE_TABLE   => AuthAction::CreateTable   { table_name: s1 },
                        SQLITE_CREATE_TRIGGER => AuthAction::CreateTrigger { trigger_name: s1, table_name: s2 },
                        SQLITE_CREATE_VIEW    => AuthAction::CreateView    { view_name: s1 },
                        SQLITE_DELETE         => AuthAction::Delete        { table_name: s1 },
                        SQLITE_DROP_INDEX     => AuthAction::DropIndex     { index_name: s1, table_name: s2 },
                        SQLITE_DROP_TABLE     => AuthAction::DropTable     { table_name: s1 },
                        SQLITE_DROP_TRIGGER   => AuthAction::DropTrigger   { trigger_name: s1, table_name: s2 },
                        SQLITE_DROP_VIEW      => AuthAction::DropView      { view_name: s1 },
                        SQLITE_INSERT         => AuthAction::Insert        { table_name: s1 },
                        SQLITE_READ           => AuthAction::Read          { table_name: s1, column_name: s2 },
                        SQLITE_SELECT         => AuthAction::Select,
                        SQLITE_TRANSACTION    => AuthAction::Transaction   { operation: s1 },
                        SQLITE_UPDATE         => AuthAction::Update        { table_name: s1, column_name: s2 },
                        SQLITE_ATTACH         => AuthAction::Attach        { filename: s1 },
                        SQLITE_DETACH         => AuthAction::Detach        { database_name: s1 },
                        SQLITE_ALTER_TABLE    => AuthAction::AlterTable    { database_name: s1, table_name: s2 },
                        SQLITE_REINDEX        => AuthAction::Reindex       { index_name: s1 },
                        SQLITE_ANALYZE        => AuthAction::Analyze       { table_name: s1 },
                        SQLITE_CREATE_VTABLE  => AuthAction::CreateVtable  { table_name: s1, module_name: s2 },
                        SQLITE_DROP_VTABLE    => AuthAction::DropVtable    { table_name: s1, module_name: s2 },
                        SQLITE_FUNCTION       => AuthAction::Function      { function_name: s2 },
                        SQLITE_SAVEPOINT      => AuthAction::Savepoint     { operation: s1, savepoint_name: s2 },
                        SQLITE_RECURSIVE      => AuthAction::Recursive,
                        c                     => AuthAction::Unknown(c),
                    };
                    f(action).as_i32()
                }

                // raw points to a stable heap allocation (the Box).
                // Register with SQLite first.
                unsafe {
                    sqlite3_set_authorizer(self.db, Some(auth_trampoline::<F>), raw);
                }
                // Re-box to keep alive; the heap address is unchanged.
                self.authorizer_hook = Some(unsafe {
                    Box::from_raw(raw as *mut F) as Box<dyn std::any::Any + Send>
                });
            }
        }
    }

    // ── Trace ─────────────────────────────────────────────────────────────────

    /// Register a trace callback using `sqlite3_trace_v2`.
    ///
    /// `mask` is a bitwise OR of `SQLITE_TRACE_STMT`, `SQLITE_TRACE_PROFILE`,
    /// `SQLITE_TRACE_ROW`, and/or `SQLITE_TRACE_CLOSE`.  Pass `0` to disable.
    ///
    /// The callback receives the trace event type (as a `u32`) and, for
    /// `SQLITE_TRACE_STMT`, the SQL text of the statement being executed.
    ///
    /// ```ignore
    /// conn.trace(Some(|_stmt: &str| println!("SQL: {_stmt}")));
    /// ```
    pub fn trace(&mut self, callback: Option<fn(&str)>) {
        match callback {
            None => {
                self.trace_hook = None;
                unsafe { sqlite3_trace_v2(self.db, 0, None, std::ptr::null_mut()); }
            }
            Some(f) => {
                unsafe extern "C" fn trace_trampoline(
                    trace_type: std::os::raw::c_uint,
                    ctx:   *mut c_void,
                    p:     *mut c_void,
                    _x:    *mut c_void,
                ) -> c_int {
                    if trace_type == SQLITE_TRACE_STMT as std::os::raw::c_uint {
                        let f: fn(&str) = unsafe { std::mem::transmute(ctx) };
                        if !p.is_null() {
                            if let Ok(s) = unsafe { CStr::from_ptr(p as *const _) }.to_str() {
                                f(s);
                            }
                        }
                    }
                    0
                }
                let fn_ptr = f as *mut c_void;
                unsafe {
                    sqlite3_trace_v2(
                        self.db,
                        SQLITE_TRACE_STMT as std::os::raw::c_uint,
                        Some(trace_trampoline),
                        fn_ptr,
                    );
                }
                self.trace_hook = Some(Box::new(f) as Box<dyn std::any::Any + Send>);
            }
        }
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

    /// Returns the raw `sqlite3*` handle for internal use within the crate, without requiring
    /// an `unsafe` block at the call site.
    ///
    /// This is a `pub(crate)` escape hatch used by `Statement`, `Rows`, and error helpers that
    /// need to call raw SQLite APIs.  External callers should use the `pub unsafe fn handle`
    /// method instead, which makes the unsafety explicit.
    pub(crate) fn handle_ptr(&self) -> *mut sqlite3 { self.db }

    /// Return a compiled `sqlite3_stmt*` to the connection's LRU statement cache, keyed by
    /// `sql`, after a [`CachedStatement`] is dropped.
    ///
    /// This is called exclusively from [`CachedStatement::drop`].  The raw pointer must be a
    /// valid, non-finalized statement that was previously extracted with
    /// [`Statement::into_raw`].  After this call the cache owns the pointer and is responsible
    /// for finalizing it when it is evicted or the connection is closed.
    pub(crate) fn cache_return(&self, sql: String, ptr: *mut sqlite3_stmt) {
        self.cache.borrow_mut().insert(sql, ptr);
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        // Flush the statement cache first to finalize all cached statements.
        self.cache.borrow_mut().flush();
        // sqlite3_close_v2 defers close until all prepared statements are finalized.
        unsafe { sqlite3_close_v2(self.db); }
    }
}
