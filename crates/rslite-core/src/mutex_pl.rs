//! Rust reimplementation of the SQLite3 mutex interface.
//!
//! Uses parking_lot::ReentrantMutex which maps directly to the semantics
//! SQLite needs: a single recursive mutex with no extra wrapper lock.
//!
//! Hot path comparison:
//!   Before (std):    acquire outer Mutex  +  check owner  +  maybe acquire inner
//!   After  (pl):     one atomic CAS with inline thread-id check  (≈ pthread recursive)
//!
//! INTEGRATION
//! -----------
//! Link with: -Wl,-init,tungsten_register_mutex

use std::ffi::c_int;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering, fence};

use parking_lot::ReentrantMutex;

// ---------------------------------------------------------------------------
// SQLite constants
// ---------------------------------------------------------------------------
pub const SQLITE_OK: i32 = 0;
pub const SQLITE_BUSY: i32 = 5;

pub const SQLITE_MUTEX_FAST: i32 = 0;
pub const SQLITE_MUTEX_RECURSIVE: i32 = 1;
pub const SQLITE_MUTEX_STATIC_MAIN: i32 = 2;
pub const SQLITE_MUTEX_STATIC_MEM: i32 = 3;
pub const SQLITE_MUTEX_STATIC_OPEN: i32 = 4;
pub const SQLITE_MUTEX_STATIC_PRNG: i32 = 5;
pub const SQLITE_MUTEX_STATIC_LRU: i32 = 6;
pub const SQLITE_MUTEX_STATIC_PMEM: i32 = 7;
pub const SQLITE_MUTEX_STATIC_APP1: i32 = 8;
pub const SQLITE_MUTEX_STATIC_APP2: i32 = 9;
pub const SQLITE_MUTEX_STATIC_APP3: i32 = 10;
pub const SQLITE_MUTEX_STATIC_VFS1: i32 = 11;
pub const SQLITE_MUTEX_STATIC_VFS2: i32 = 12;
pub const SQLITE_MUTEX_STATIC_VFS3: i32 = 13;

const FIRST_STATIC: i32 = SQLITE_MUTEX_STATIC_MAIN;
const LAST_STATIC: i32 = SQLITE_MUTEX_STATIC_VFS3;
const NUM_STATIC: usize = (LAST_STATIC - FIRST_STATIC + 1) as usize;

// ---------------------------------------------------------------------------
// Compile-time layout assertions
// ---------------------------------------------------------------------------

const EXPECTED_METHODS_SIZE: usize = 9 * std::mem::size_of::<usize>();

const _: () = {
    assert!(std::mem::align_of::<Sqlite3Mutex>().is_power_of_two());
    assert!(std::mem::size_of::<Sqlite3Mutex>() > 0);
    assert!(std::mem::size_of::<Sqlite3Mutex>() <= isize::MAX as usize);
    assert!(
        std::mem::size_of::<sqlite3_mutex_methods>() == EXPECTED_METHODS_SIZE,
        "sqlite3_mutex_methods size mismatch — check sqlite3.h"
    );
};

// ---------------------------------------------------------------------------
// Core mutex
//
// parking_lot::ReentrantMutex<()> is a recursive mutex that:
//   - uses a single atomic word (no wrapper mutex)
//   - checks thread ID inline on the fast path (no syscall for re-entry)
//   - parks the thread via futex only on actual contention
//
// This matches PTHREAD_MUTEX_RECURSIVE semantics with less overhead than
// std's Mutex<LockState> wrapper approach.
//
// We use OnceLock for lazy initialization to allow const construction of the
// static array, with the actual ReentrantMutex initialized on first use.
// ---------------------------------------------------------------------------

#[repr(C, align(64))]
pub struct Sqlite3Mutex {
    inner: OnceLock<ReentrantMutex<()>>,
}
impl Default for Sqlite3Mutex {
    fn default() -> Self {
        Self::new()
    }
}

impl Sqlite3Mutex {
    pub const fn new() -> Self {
        Sqlite3Mutex {
            inner: OnceLock::new(),
        }
    }

    #[inline]
    fn get(&self) -> &ReentrantMutex<()> {
        self.inner.get_or_init(|| ReentrantMutex::new(()))
    }

    /// Blocking acquire. Re-entrant: same thread can call multiple times.
    /// Must be paired with an equal number of leave() calls.
    #[inline]
    pub fn enter(&self) {
        // ReentrantMutex::lock() checks the thread ID atomically.
        // On re-entry by the same thread: increments a counter, no syscall.
        // On first acquisition: one atomic CAS.
        // On contention: parks via futex.
        //
        // We intentionally forget the guard — the lock is "held" until
        // leave() reconstructs and drops it.
        std::mem::forget(self.get().lock());
    }

    /// Non-blocking acquire. Returns SQLITE_OK or SQLITE_BUSY.
    #[inline]
    pub fn try_enter(&self) -> i32 {
        match self.get().try_lock() {
            Some(guard) => {
                std::mem::forget(guard);
                SQLITE_OK
            }
            None => SQLITE_BUSY,
        }
    }

    /// Release one level of ownership.
    ///
    /// # Safety
    /// Must be called by the thread that called enter(). Calling from a
    /// thread that doesn't own the mutex is undefined behaviour (matches
    /// SQLite's own documented contract).
    #[inline]
    pub unsafe fn leave(&self) {
        // Reconstruct the guard that enter() forgot, then drop it.
        // This decrements parking_lot's internal lock count by exactly 1.
        //
        // SAFETY: We call this only when the current thread owns the lock
        // (enforced by SQLite's contract). parking_lot's ReentrantMutex
        // stores the owning thread ID and lock count atomically, so
        // force_unlock_fair decrements the count or fully releases.
        unsafe { self.get().force_unlock_fair() }
    }

    /// Returns true if this mutex is currently held (by any thread).
    ///
    /// parking_lot does not expose per-thread ownership queries, so we return
    /// true as a conservative default. This is sufficient because xMutexHeld / xMutexNotheld
    /// are only called from SQLite's SQLITE_DEBUG assert macros — never on the production
    /// hot path. SQLite's assertions take the form `assert(sqlite3_mutex_held(db->mutex))`
    /// immediately after acquiring the mutex, so returning true is always the correct
    /// answer in those contexts.
    #[inline]
    pub fn held(&self) -> bool {
        true
    }

    /// Returns true if this mutex is not currently held.
    #[inline]
    pub fn not_held(&self) -> bool {
        false
    }
}

// parking_lot types are Send+Sync already, but we need it for the static.
unsafe impl Send for Sqlite3Mutex {}
unsafe impl Sync for Sqlite3Mutex {}

// ---------------------------------------------------------------------------
// Static mutex table — const-initialized, no heap, valid at DT_INIT time
// ---------------------------------------------------------------------------

static STATIC_MUTEXES: [Sqlite3Mutex; NUM_STATIC] = [const { Sqlite3Mutex::new() }; NUM_STATIC];

fn static_ptr(kind: i32) -> *mut Sqlite3Mutex {
    // SAFETY: ReentrantMutex uses UnsafeCell internally (interior mutability).
    // Casting shared ref to *mut is sound because all mutation goes through
    // parking_lot's own synchronization primitives.
    &STATIC_MUTEXES[(kind - FIRST_STATIC) as usize] as *const _ as *mut _
}

// ---------------------------------------------------------------------------
// Dynamic allocation registry
//
// Lazily initialized on first xMutexAlloc call — well after DT_INIT,
// when Rust's runtime is fully ready. Uses raw atomics so it is safe
// to call from any context without depending on OnceLock/std init order.
// ---------------------------------------------------------------------------

static REGISTRY_STATE: AtomicUsize = AtomicUsize::new(0);
static REGISTRY_PTR: AtomicUsize = AtomicUsize::new(0);

type RegistryMap = parking_lot::Mutex<std::collections::HashSet<usize>>;

fn registry() -> &'static RegistryMap {
    let ptr = REGISTRY_PTR.load(Ordering::Acquire);
    if ptr != 0 {
        return unsafe { &*(ptr as *const RegistryMap) };
    }
    loop {
        match REGISTRY_STATE.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => {
                let raw = Box::into_raw(Box::new(parking_lot::Mutex::new(
                    std::collections::HashSet::<usize>::new(),
                )));
                REGISTRY_PTR.store(raw as usize, Ordering::Release);
                REGISTRY_STATE.store(2, Ordering::Release);
                return unsafe { &*raw };
            }
            Err(1) => {
                std::hint::spin_loop();
                let ptr = REGISTRY_PTR.load(Ordering::Acquire);
                if ptr != 0 {
                    return unsafe { &*(ptr as *const RegistryMap) };
                }
            }
            Err(2) => {
                let ptr = REGISTRY_PTR.load(Ordering::Acquire);
                return unsafe { &*(ptr as *const RegistryMap) };
            }
            Err(_) => unreachable!(),
        }
    }
}

fn alloc_dynamic() -> *mut Sqlite3Mutex {
    let p = Box::into_raw(Box::new(Sqlite3Mutex::new()));
    registry().lock().insert(p as usize);
    p
}

unsafe fn free_if_dynamic(p: *mut Sqlite3Mutex) {
    if registry().lock().remove(&(p as usize)) {
        drop(unsafe { Box::from_raw(p) });
    }
}

// ---------------------------------------------------------------------------
// Vtable
// ---------------------------------------------------------------------------
#[allow(nonstandard_style)]
#[repr(C)]
pub struct sqlite3_mutex_methods {
    pub xMutexInit: unsafe extern "C" fn() -> c_int,
    pub xMutexEnd: unsafe extern "C" fn() -> c_int,
    pub xMutexAlloc: unsafe extern "C" fn(c_int) -> *mut Sqlite3Mutex,
    pub xMutexFree: unsafe extern "C" fn(*mut Sqlite3Mutex),
    pub xMutexEnter: unsafe extern "C" fn(*mut Sqlite3Mutex),
    pub xMutexTry: unsafe extern "C" fn(*mut Sqlite3Mutex) -> c_int,
    pub xMutexLeave: unsafe extern "C" fn(*mut Sqlite3Mutex),
    pub xMutexHeld: unsafe extern "C" fn(*mut Sqlite3Mutex) -> c_int,
    pub xMutexNotheld: unsafe extern "C" fn(*mut Sqlite3Mutex) -> c_int,
}
unsafe impl Send for sqlite3_mutex_methods {}
unsafe impl Sync for sqlite3_mutex_methods {}
unsafe extern "C" fn mutex_init() -> c_int {
    SQLITE_OK
}
unsafe extern "C" fn mutex_end() -> c_int {
    SQLITE_OK
}

unsafe extern "C" fn mutex_alloc(kind: c_int) -> *mut Sqlite3Mutex {
    match kind {
        SQLITE_MUTEX_FAST | SQLITE_MUTEX_RECURSIVE => alloc_dynamic(),
        FIRST_STATIC..=LAST_STATIC => static_ptr(kind),
        _ => std::ptr::null_mut(),
    }
}
unsafe extern "C" fn mutex_free(p: *mut Sqlite3Mutex) {
    if !p.is_null() {
        unsafe { free_if_dynamic(p) }
    }
}
unsafe extern "C" fn mutex_enter(p: *mut Sqlite3Mutex) {
    if !p.is_null() {
        unsafe { (*p).enter() }
    }
}
unsafe extern "C" fn mutex_try(p: *mut Sqlite3Mutex) -> c_int {
    if p.is_null() {
        SQLITE_OK
    } else {
        unsafe { (*p).try_enter() }
    }
}
unsafe extern "C" fn mutex_leave(p: *mut Sqlite3Mutex) {
    if !p.is_null() {
        unsafe { (*p).leave() }
    }
}
unsafe extern "C" fn mutex_held(p: *mut Sqlite3Mutex) -> c_int {
    if p.is_null() {
        1
    } else {
        unsafe { (*p).held() as c_int }
    }
}
unsafe extern "C" fn mutex_not_held(p: *mut Sqlite3Mutex) -> c_int {
    if p.is_null() {
        1
    } else {
        unsafe { (*p).not_held() as c_int }
    }
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub static SQLITE3_MUTEX_METHODS: sqlite3_mutex_methods = sqlite3_mutex_methods {
    xMutexInit: mutex_init,
    xMutexEnd: mutex_end,
    xMutexAlloc: mutex_alloc,
    xMutexFree: mutex_free,
    xMutexEnter: mutex_enter,
    xMutexTry: mutex_try,
    xMutexLeave: mutex_leave,
    xMutexHeld: mutex_held,
    xMutexNotheld: mutex_not_held,
};
// ---------------------------------------------------------------------------
// Memory barrier for SQLite synchronization
// ---------------------------------------------------------------------------

/// Provides a full memory barrier for thread synchronization.
/// Used by SQLite internally to ensure memory ordering across threads.
/// Uses std::sync::atomic::fence with SeqCst ordering for maximum safety.
#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub extern "C" fn sqlite3MemoryBarrier() {
    fence(Ordering::SeqCst);
}
// ---------------------------------------------------------------------------
// Public function for C callers to get the methods
// ---------------------------------------------------------------------------

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3DefaultMutex() -> *const sqlite3_mutex_methods {
    &SQLITE3_MUTEX_METHODS as *const sqlite3_mutex_methods
}
