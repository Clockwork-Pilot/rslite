use std::ops::{Deref, DerefMut};
use crate::{Connection, Error, Result};

/// How to start a transaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionBehavior {
    /// `BEGIN DEFERRED` (default)
    Deferred,
    /// `BEGIN IMMEDIATE`
    Immediate,
    /// `BEGIN EXCLUSIVE`
    Exclusive,
}

impl TransactionBehavior {
    fn begin_sql(self) -> &'static str {
        match self {
            TransactionBehavior::Deferred  => "BEGIN DEFERRED",
            TransactionBehavior::Immediate => "BEGIN IMMEDIATE",
            TransactionBehavior::Exclusive => "BEGIN EXCLUSIVE",
        }
    }
}

/// What to do when a `Transaction` is dropped without an explicit
/// `commit()` or `rollback()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropBehavior {
    /// Silently roll back (default).
    Rollback,
    /// Commit on drop.
    Commit,
    /// Do nothing — leave the transaction open (dangerous).
    Ignore,
    /// Panic.
    Panic,
}

/// An ACID transaction on a `Connection`.
///
/// The transaction is automatically rolled back on `Drop` (unless committed
/// or the `DropBehavior` is changed).  Use the `Deref` impl to call any
/// `Connection` method directly on the transaction.
pub struct Transaction<'conn> {
    conn: &'conn mut Connection,
    drop_behavior: DropBehavior,
    committed: bool,
}

impl<'conn> Transaction<'conn> {
    pub(crate) fn new(conn: &'conn mut Connection, behavior: TransactionBehavior) -> Result<Self> {
        conn.execute_batch(behavior.begin_sql())?;
        Ok(Transaction { conn, drop_behavior: DropBehavior::Rollback, committed: false })
    }

    /// Commit the transaction.
    pub fn commit(mut self) -> Result<()> {
        let result = self.conn.execute_batch("COMMIT");
        if result.is_ok() {
            self.committed = true; // prevent Drop from rolling back
        }
        // On failure, Drop will issue ROLLBACK (unless drop_behavior overrides)
        result
    }

    /// Roll back the transaction explicitly.
    pub fn rollback(mut self) -> Result<()> {
        self.committed = true; // prevent double-rollback on drop
        self.conn.execute_batch("ROLLBACK")
    }

    /// Control what happens on `Drop` if neither `commit` nor `rollback` was called.
    pub fn set_drop_behavior(&mut self, behavior: DropBehavior) {
        self.drop_behavior = behavior;
    }

    pub fn drop_behavior(&self) -> DropBehavior { self.drop_behavior }

    /// Create a `Savepoint` nested within this transaction.
    pub fn savepoint(&mut self) -> Result<Savepoint<'_>> {
        Savepoint::new(self.conn)
    }

    /// Create a named `Savepoint` nested within this transaction.
    pub fn savepoint_with_name<N: Into<String>>(&mut self, name: N) -> Result<Savepoint<'_>> {
        Savepoint::with_name(self.conn, name.into())
    }
}

impl std::fmt::Debug for Transaction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transaction")
            .field("drop_behavior", &self.drop_behavior)
            .finish_non_exhaustive()
    }
}

impl Deref for Transaction<'_> {
    type Target = Connection;
    fn deref(&self) -> &Connection { self.conn }
}

impl DerefMut for Transaction<'_> {
    fn deref_mut(&mut self) -> &mut Connection { self.conn }
}

impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        if self.committed { return; }
        match self.drop_behavior {
            DropBehavior::Rollback => { let _ = self.conn.execute_batch("ROLLBACK"); }
            DropBehavior::Commit   => { let _ = self.conn.execute_batch("COMMIT"); }
            DropBehavior::Ignore   => {}
            DropBehavior::Panic    => panic!("transaction dropped without commit or rollback"),
        }
    }
}

// ── Savepoint ─────────────────────────────────────────────────────────────────

static SAVEPOINT_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// A named savepoint nested within a transaction or another savepoint.
pub struct Savepoint<'conn> {
    conn: &'conn mut Connection,
    name: String,
    released: bool,
    drop_behavior: DropBehavior,
}

impl<'conn> Savepoint<'conn> {
    pub(crate) fn new(conn: &'conn mut Connection) -> Result<Self> {
        let n = SAVEPOINT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let name = format!("rslite_sp_{}", n);
        Self::with_name(conn, name)
    }

    pub(crate) fn with_name(conn: &'conn mut Connection, name: String) -> Result<Self> {
        conn.execute_batch(&format!("SAVEPOINT {}", name))?;
        Ok(Savepoint { conn, name, released: false, drop_behavior: DropBehavior::Rollback })
    }

    /// Create a nested savepoint with an explicit name.
    pub fn savepoint_with_name<N: Into<String>>(&mut self, name: N) -> Result<Savepoint<'_>> {
        Savepoint::with_name(self.conn, name.into())
    }

    /// Release (commit) the savepoint.
    pub fn commit(&mut self) -> Result<()> {
        let result = self.conn.execute_batch(&format!("RELEASE SAVEPOINT {}", self.name));
        if result.is_ok() {
            self.released = true;
        }
        result
    }

    /// Roll back to this savepoint.  The savepoint remains active and can be
    /// reused (unlike `Transaction::rollback`, which consumes the transaction).
    pub fn rollback(&mut self) -> Result<()> {
        self.conn.execute_batch(&format!("ROLLBACK TO SAVEPOINT {}", self.name))
    }

    pub fn set_drop_behavior(&mut self, behavior: DropBehavior) {
        self.drop_behavior = behavior;
    }
}

impl std::fmt::Debug for Savepoint<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Savepoint")
            .field("name", &self.name)
            .field("drop_behavior", &self.drop_behavior)
            .finish_non_exhaustive()
    }
}

impl Deref for Savepoint<'_> {
    type Target = Connection;
    fn deref(&self) -> &Connection { self.conn }
}

impl DerefMut for Savepoint<'_> {
    fn deref_mut(&mut self) -> &mut Connection { self.conn }
}

impl Drop for Savepoint<'_> {
    fn drop(&mut self) {
        if self.released { return; }
        match self.drop_behavior {
            DropBehavior::Rollback => {
                let _ = self.conn.execute_batch(&format!("ROLLBACK TO SAVEPOINT {}", self.name));
                let _ = self.conn.execute_batch(&format!("RELEASE SAVEPOINT {}", self.name));
            }
            DropBehavior::Commit => {
                if self.conn.execute_batch(&format!("RELEASE SAVEPOINT {}", self.name)).is_err() {
                    // RELEASE failed (e.g. deferred FK constraint violation).
                    // Fall back: roll back to the savepoint then release it.
                    let _ = self.conn.execute_batch(&format!("ROLLBACK TO SAVEPOINT {}", self.name));
                    let _ = self.conn.execute_batch(&format!("RELEASE SAVEPOINT {}", self.name));
                }
            }
            DropBehavior::Ignore => {}
            DropBehavior::Panic => panic!("savepoint '{}' dropped without commit or rollback", self.name),
        }
    }
}
