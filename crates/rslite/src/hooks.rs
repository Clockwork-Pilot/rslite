//! Callback hooks for SQLite connection events.
//!
//! Provides safe wrappers around `sqlite3_commit_hook`, `sqlite3_rollback_hook`,
//! and `sqlite3_update_hook`.

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};

// SQLite authorizer action codes (sqlite.h §3.1).
// Action codes not in the stable public API surface, defined locally.
const SQLITE_DELETE: c_int = 9;
const SQLITE_INSERT: c_int = 18;
const SQLITE_UPDATE: c_int = 23;

/// The DML operation that triggered an [`update_hook`](crate::Connection::update_hook) call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Delete,
    Insert,
    Update,
    /// An action code not covered by the variants above.
    Unknown(i32),
}

impl From<c_int> for Action {
    fn from(code: c_int) -> Self {
        match code {
            SQLITE_DELETE => Action::Delete,
            SQLITE_INSERT => Action::Insert,
            SQLITE_UPDATE => Action::Update,
            n             => Action::Unknown(n),
        }
    }
}

// ── Stable heap slot for closures ─────────────────────────────────────────────
//
// We need to hand SQLite a `*mut c_void` that lives as long as the hook is
// registered.  We achieve this by boxing the closure on the heap, returning a
// raw pointer to SQLite and keeping ownership in a `HookSlot` stored on the
// `Connection`.  When the slot is replaced or the connection is dropped, the
// `HookSlot` destructor frees the closure.

pub(crate) struct HookSlot {
    ptr:  *mut (),
    drop: unsafe fn(*mut ()),
}

// SAFETY: HookSlot owns heap memory and is only used from one thread at a time
// (protected by Connection which is Send but !Sync).
unsafe impl Send for HookSlot {}

impl Drop for HookSlot {
    fn drop(&mut self) {
        unsafe { (self.drop)(self.ptr) }
    }
}

impl HookSlot {
    /// Box `f` on the heap, return a slot (takes ownership) and a raw pointer
    /// suitable for passing to SQLite as `pArg`.
    pub(crate) fn new<F: Send + 'static>(f: F) -> (Self, *mut ()) {
        let raw = Box::into_raw(Box::new(f)) as *mut ();
        let slot = HookSlot { ptr: raw, drop: drop_box::<F> };
        (slot, raw)
    }
}

unsafe fn drop_box<F>(ptr: *mut ()) {
    unsafe { drop(Box::from_raw(ptr as *mut F)) };
}

// ── Trampoline functions ───────────────────────────────────────────────────────

/// C trampoline for commit hooks.
///
/// Returns 0 to allow the commit, non-zero to convert it to a rollback.
pub(crate) unsafe extern "C" fn commit_trampoline<F>(p_arg: *mut c_void) -> c_int
where
    F: FnMut() -> bool,
{
    let f = unsafe { &mut *(p_arg as *mut F) };
    if f() { 1 } else { 0 }
}

/// C trampoline for rollback hooks.
pub(crate) unsafe extern "C" fn rollback_trampoline<F>(p_arg: *mut c_void)
where
    F: FnMut(),
{
    let f = unsafe { &mut *(p_arg as *mut F) };
    f();
}

/// C trampoline for update hooks.
pub(crate) unsafe extern "C" fn update_trampoline<F>(
    p_arg:    *mut c_void,
    action:   c_int,
    db_name:  *const c_char,
    tbl_name: *const c_char,
    rowid:    i64,
) where
    F: FnMut(Action, &str, &str, i64),
{
    let f = unsafe { &mut *(p_arg as *mut F) };
    let db  = unsafe { CStr::from_ptr(db_name).to_str().unwrap_or("") };
    let tbl = unsafe { CStr::from_ptr(tbl_name).to_str().unwrap_or("") };
    f(Action::from(action), db, tbl, rowid);
}
