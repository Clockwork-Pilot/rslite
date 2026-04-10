//! Statement cache for `Connection::prepare_cached`.
//!
//! Provides a bounded cache of prepared `sqlite3_stmt*` pointers keyed by SQL
//! text so that frequently-reused statements avoid repeated parsing overhead.

use std::collections::{HashMap, VecDeque};
use std::ops::{Deref, DerefMut};

use sqlite_noamalgam::{sqlite3_finalize, sqlite3_stmt};

use crate::{Connection, Statement};

// ── StatementCache ────────────────────────────────────────────────────────────

/// Bounded cache of raw `sqlite3_stmt*` pointers keyed by SQL text.
///
/// When a `CachedStatement` is dropped, the underlying pointer is returned here
/// rather than finalized, so subsequent calls with the same SQL reuse the
/// already-compiled statement.
pub(crate) struct StatementCache {
    cap:   usize,
    cache: HashMap<String, Vec<*mut sqlite3_stmt>>,
    order: VecDeque<String>,  // insertion order for LRU eviction
}

// SAFETY: The pointers are only accessed through the Connection that owns this
// cache, and Connection is Send.
unsafe impl Send for StatementCache {}

impl StatementCache {
    /// Create a new statement cache that will hold at most `cap` compiled statements.
    ///
    /// When `cap` is 0 the cache is effectively disabled: every statement returned via
    /// [`insert`](StatementCache::insert) is finalized immediately instead of being stored.
    /// The default capacity used by `Connection` is 8, which covers the most common set of
    /// hot queries without consuming excessive memory for prepared-statement handles.
    pub(crate) fn new(cap: usize) -> Self {
        StatementCache { cap, cache: HashMap::new(), order: VecDeque::new() }
    }

    /// Returns the current maximum number of compiled statements that the cache will retain.
    ///
    /// This is the capacity as last set by [`new`](StatementCache::new) or
    /// [`set_capacity`](StatementCache::set_capacity).  The actual number of cached entries may
    /// be lower if fewer distinct SQL strings have been prepared since the cache was created or
    /// last flushed.
    #[allow(dead_code)]
    pub(crate) fn capacity(&self) -> usize { self.cap }

    /// Change the cache capacity to `cap`, immediately evicting LRU entries if the current
    /// number of cached statements exceeds the new limit.
    ///
    /// Evicted entries are finalized via `sqlite3_finalize` as they are removed, so reducing
    /// the capacity to 0 is equivalent to calling [`flush`](StatementCache::flush) and then
    /// disabling future caching.  Increasing the capacity never triggers eviction.
    pub(crate) fn set_capacity(&mut self, cap: usize) {
        self.cap = cap;
        self.evict_to_capacity();
    }

    /// Remove and return a cached `sqlite3_stmt*` previously stored under `sql`, or `None` if
    /// no entry for that SQL string exists.
    ///
    /// After a successful retrieval the cache entry is consumed.  If the bucket for `sql` is
    /// now empty the key is also removed from the insertion-order deque so that capacity
    /// calculations remain accurate.  The caller is responsible for eventually finalizing the
    /// returned pointer if it is not returned to the cache via [`insert`](StatementCache::insert).
    pub(crate) fn get(&mut self, sql: &str) -> Option<*mut sqlite3_stmt> {
        let ptr = self.cache.get_mut(sql)?.pop()?;
        if self.cache.get(sql).map_or(false, |v| v.is_empty()) {
            self.cache.remove(sql);
            self.order.retain(|s| s != sql);
        }
        Some(ptr)
    }

    /// Store a compiled `sqlite3_stmt*` in the cache under `sql`.
    ///
    /// If the cache is at capacity the oldest entry (by insertion order) is evicted and
    /// finalized before the new pointer is inserted, keeping the total count at or below
    /// `self.cap`.  If `cap` is 0 the pointer is finalized immediately without being stored.
    /// The `sql` string is the canonical cache key; the same statement may be cached multiple
    /// times under the same key if it is returned by multiple `CachedStatement` drops.
    pub(crate) fn insert(&mut self, sql: String, ptr: *mut sqlite3_stmt) {
        if self.cap == 0 {
            unsafe { sqlite3_finalize(ptr); }
            return;
        }
        let entry_count: usize = self.cache.values().map(|v| v.len()).sum();
        // Evict oldest entries until we're under capacity
        while entry_count >= self.cap {
            if let Some(old_sql) = self.order.pop_front() {
                if let Some(v) = self.cache.get_mut(&old_sql) {
                    if let Some(old_ptr) = v.pop() {
                        unsafe { sqlite3_finalize(old_ptr); }
                    }
                    if v.is_empty() { self.cache.remove(&old_sql); }
                }
            } else {
                break;
            }
        }
        self.cache.entry(sql.clone()).or_default().push(ptr);
        if !self.order.contains(&sql) {
            self.order.push_back(sql);
        }
    }

    /// Finalize every cached `sqlite3_stmt*` and clear the cache completely.
    ///
    /// Called automatically by [`StatementCache::drop`] and by [`Connection::cache_flush`].
    /// It is also called before closing a connection to ensure all statements are finalized
    /// before `sqlite3_close` is invoked, since SQLite returns `SQLITE_BUSY` if any prepared
    /// statements remain open when the connection is closed.
    pub(crate) fn flush(&mut self) {
        for (_, stmts) in self.cache.drain() {
            for ptr in stmts {
                unsafe { sqlite3_finalize(ptr); }
            }
        }
        self.order.clear();
    }

    fn evict_to_capacity(&mut self) {
        while self.cache.values().map(|v| v.len()).sum::<usize>() > self.cap {
            if let Some(old_sql) = self.order.pop_front() {
                if let Some(v) = self.cache.get_mut(&old_sql) {
                    if let Some(ptr) = v.pop() {
                        unsafe { sqlite3_finalize(ptr); }
                    }
                    if v.is_empty() { self.cache.remove(&old_sql); }
                }
            } else {
                break;
            }
        }
    }
}

impl Drop for StatementCache {
    fn drop(&mut self) { self.flush(); }
}

// ── CachedStatement ───────────────────────────────────────────────────────────

/// A prepared statement vended by `Connection::prepare_cached`.
///
/// Behaves like a `Statement` but is returned to the connection's statement
/// cache when dropped instead of being finalized.  This avoids repeated SQL
/// parsing for frequently-used queries.
pub struct CachedStatement<'conn> {
    stmt: Option<Statement<'conn>>,
    conn: &'conn Connection,
    sql:  String,
}

impl<'conn> CachedStatement<'conn> {
    /// Wrap a prepared [`Statement`] in a `CachedStatement` that will be returned to the
    /// connection's statement cache when dropped instead of being finalized.
    ///
    /// `sql` must be the exact SQL string used to prepare `stmt` so that the cache can
    /// look it up on the next call to [`Connection::prepare_cached`] with the same string.
    /// Called exclusively by [`Connection::prepare_cached`] after obtaining or freshly
    /// preparing the underlying statement.
    pub(crate) fn new(stmt: Statement<'conn>, conn: &'conn Connection, sql: String) -> Self {
        CachedStatement { stmt: Some(stmt), conn, sql }
    }
}

impl<'conn> Deref for CachedStatement<'conn> {
    type Target = Statement<'conn>;
    fn deref(&self) -> &Statement<'conn> { self.stmt.as_ref().unwrap() }
}

impl<'conn> DerefMut for CachedStatement<'conn> {
    fn deref_mut(&mut self) -> &mut Statement<'conn> { self.stmt.as_mut().unwrap() }
}

impl Drop for CachedStatement<'_> {
    fn drop(&mut self) {
        if let Some(stmt) = self.stmt.take() {
            let ptr = stmt.into_raw();
            // Return the raw pointer to the connection's cache.
            // SAFETY: We hold a shared reference to conn, which has a RefCell
            // for interior mutability of the cache.
            self.conn.cache_return(self.sql.clone(), ptr);
        }
    }
}

impl std::fmt::Debug for CachedStatement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.stmt {
            Some(s) => write!(f, "CachedStatement({:?})", s),
            None    => write!(f, "CachedStatement(returned)"),
        }
    }
}
