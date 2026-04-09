//! User-defined functions (scalar and aggregate).

use std::ffi::{CString, c_void};
use std::marker::PhantomData;
use std::mem::size_of;
use std::os::raw::{c_char, c_int};

use rslite_raw::*;

use crate::{
    error::sqlite_error_from_code,
    types::{FromSql, ToSql, ToSqlOutput, ValueRef},
    Result,
};

// ── FunctionFlags ─────────────────────────────────────────────────────────────

bitflags::bitflags! {
    /// Flags passed to `create_scalar_function` / `create_aggregate_function`.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct FunctionFlags: i32 {
        /// The function always returns the same result given the same inputs.
        const DETERMINISTIC = SQLITE_DETERMINISTIC;
        /// The function may only be used in direct SQL, not in triggers or views.
        const DIRECTONLY    = SQLITE_DIRECTONLY;
        /// The function is safe for use in generated columns, triggers, and views.
        const INNOCUOUS     = SQLITE_INNOCUOUS;
    }
}

// ── Context ───────────────────────────────────────────────────────────────────

/// The execution context passed to scalar and aggregate functions.
///
/// Provides typed access to the call's arguments via [`get`](Context::get).
pub struct Context<'a> {
    ctx:    *mut sqlite3_context,
    args:   *mut *mut sqlite3_value,
    n_args: usize,
    _lt:    PhantomData<&'a ()>,
}

impl<'a> Context<'a> {
    pub(crate) unsafe fn from_raw(
        ctx:  *mut sqlite3_context,
        argc: c_int,
        argv: *mut *mut sqlite3_value,
    ) -> Self {
        Context { ctx, args: argv, n_args: argc as usize, _lt: PhantomData }
    }

    /// Number of arguments passed to this function call.
    pub fn len(&self) -> usize { self.n_args }

    pub fn is_empty(&self) -> bool { self.n_args == 0 }

    /// Get the argument at `idx` as type `T`.
    pub fn get<T: FromSql>(&self, idx: usize) -> Result<T> {
        let vref = self.get_value_ref(idx)?;
        let typ = vref.data_type();
        T::column_result(vref)
            .map_err(|e| crate::Error::FromSqlConversionFailure(idx, typ, Box::new(e)))
    }

    fn get_value_ref(&self, idx: usize) -> Result<ValueRef<'_>> {
        if idx >= self.n_args {
            return Err(crate::Error::InvalidColumnIndex(idx));
        }
        let val = unsafe { *self.args.add(idx) };
        Ok(unsafe { value_to_value_ref(val) })
    }
}

// ── Aggregate trait ───────────────────────────────────────────────────────────

/// Implement this to register an aggregate SQL function.
///
/// `A` is the accumulator type; `T` is the return type.
///
/// ```ignore
/// struct Sum;
///
/// impl Aggregate<i64, i64> for Sum {
///     fn init(&self) -> i64 { 0 }
///     fn step(&self, ctx: &Context<'_>, acc: &mut i64) -> Result<()> {
///         *acc += ctx.get::<i64>(0)?;
///         Ok(())
///     }
///     fn finalize(&self, _ctx: &Context<'_>, acc: Option<i64>) -> Result<i64> {
///         Ok(acc.unwrap_or(0))
///     }
/// }
///
/// conn.create_aggregate_function("sum2", 1, FunctionFlags::default(), Sum)?;
/// ```
pub trait Aggregate<A, T>: Send + Sync + 'static
where
    T: ToSql,
{
    /// Create the initial accumulator for a new group.
    fn init(&self) -> A;

    /// Process one row.  Mutate `acc` as needed.
    fn step(&self, ctx: &Context<'_>, acc: &mut A) -> Result<()>;

    /// Produce the final result.  `acc` is `None` when the group had no rows.
    fn finalize(&self, ctx: &Context<'_>, acc: Option<A>) -> Result<T>;
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// SQLITE_TRANSIENT: SQLite copies text/blob data before the call returns.
#[allow(clippy::transmutes_expressible_as_ptr_casts)]
unsafe fn sqlite_transient() -> Option<unsafe extern "C" fn(*mut c_void)> {
    unsafe { std::mem::transmute(-1isize) }
}

unsafe fn value_to_value_ref<'a>(val: *mut sqlite3_value) -> ValueRef<'a> {
    let typ = unsafe { sqlite3_value_type(val) };
    match typ {
        SQLITE_INTEGER => ValueRef::Integer(unsafe { sqlite3_value_int64(val) }),
        SQLITE_FLOAT   => ValueRef::Real(unsafe { sqlite3_value_double(val) }),
        SQLITE_TEXT => {
            let ptr = unsafe { sqlite3_value_text(val) };
            let len = unsafe { sqlite3_value_bytes(val) } as usize;
            if ptr.is_null() || len == 0 {
                ValueRef::Text("")
            } else {
                let s = unsafe { std::slice::from_raw_parts(ptr, len) };
                ValueRef::Text(std::str::from_utf8(s).unwrap_or(""))
            }
        }
        SQLITE_BLOB => {
            let ptr = unsafe { sqlite3_value_blob(val) } as *const u8;
            let len = unsafe { sqlite3_value_bytes(val) } as usize;
            if ptr.is_null() || len == 0 {
                ValueRef::Blob(&[])
            } else {
                ValueRef::Blob(unsafe { std::slice::from_raw_parts(ptr, len) })
            }
        }
        _ => ValueRef::Null,
    }
}

pub(crate) unsafe fn set_result(ctx: *mut sqlite3_context, out: ToSqlOutput<'_>) {
    unsafe { set_result_value_ref(ctx, out.as_value_ref()) };
}

unsafe fn set_result_value_ref(ctx: *mut sqlite3_context, val: ValueRef<'_>) {
    match val {
        ValueRef::Integer(i) => unsafe { sqlite3_result_int64(ctx, i) },
        ValueRef::Real(f)    => unsafe { sqlite3_result_double(ctx, f) },
        ValueRef::Text(s)    => unsafe {
            sqlite3_result_text(
                ctx,
                s.as_ptr() as *const c_char,
                s.len() as c_int,
                sqlite_transient(),
            )
        },
        ValueRef::Blob(b) => unsafe {
            sqlite3_result_blob(
                ctx,
                b.as_ptr() as *const c_void,
                b.len() as c_int,
                sqlite_transient(),
            )
        },
        ValueRef::Null => unsafe { sqlite3_result_null(ctx) },
    }
}

pub(crate) unsafe fn set_result_error(ctx: *mut sqlite3_context, msg: &str) {
    unsafe { sqlite3_result_error(ctx, msg.as_ptr() as *const c_char, msg.len() as c_int) };
}

// ── Scalar trampolines ────────────────────────────────────────────────────────

pub(crate) unsafe extern "C" fn scalar_trampoline<F, T>(
    ctx:  *mut sqlite3_context,
    argc: c_int,
    argv: *mut *mut sqlite3_value,
) where
    F: FnMut(&Context<'_>) -> Result<T>,
    T: ToSql,
{
    let f = unsafe { &mut *(sqlite3_user_data(ctx) as *mut F) };
    let context = unsafe { Context::from_raw(ctx, argc, argv) };
    match f(&context) {
        Ok(v) => match v.to_sql() {
            Ok(out) => unsafe { set_result(ctx, out) },
            Err(e)  => unsafe { set_result_error(ctx, &e.to_string()) },
        },
        Err(e) => unsafe { set_result_error(ctx, &e.to_string()) },
    }
}

pub(crate) unsafe extern "C" fn scalar_destroy<F>(p: *mut c_void) {
    unsafe { drop(Box::from_raw(p as *mut F)) };
}

// ── Aggregate trampolines ─────────────────────────────────────────────────────

pub(crate) unsafe extern "C" fn agg_step_trampoline<D, A, T>(
    ctx:  *mut sqlite3_context,
    argc: c_int,
    argv: *mut *mut sqlite3_value,
) where
    D: Aggregate<A, T>,
    T: ToSql,
{
    let agg = unsafe { &*(sqlite3_user_data(ctx) as *const D) };

    // The aggregate context buffer holds a single `*mut A` (heap pointer).
    // It is zero-initialised by SQLite, so a null pointer means "not yet init".
    let slot_ptr = unsafe {
        sqlite3_aggregate_context(ctx, size_of::<*mut A>() as c_int)
    };
    if slot_ptr.is_null() { return; } // OOM
    let slot = unsafe { &mut *(slot_ptr as *mut *mut A) };

    if slot.is_null() {
        *slot = Box::into_raw(Box::new(agg.init()));
    }

    let acc = unsafe { &mut **slot };
    let context = unsafe { Context::from_raw(ctx, argc, argv) };
    if let Err(e) = agg.step(&context, acc) {
        unsafe { set_result_error(ctx, &e.to_string()) };
    }
}

pub(crate) unsafe extern "C" fn agg_final_trampoline<D, A, T>(
    ctx: *mut sqlite3_context,
) where
    D: Aggregate<A, T>,
    T: ToSql,
{
    let agg = unsafe { &*(sqlite3_user_data(ctx) as *const D) };

    // size=0 returns the existing buffer without allocating; null if no rows.
    let slot_ptr = unsafe { sqlite3_aggregate_context(ctx, 0) };
    let acc: Option<A> = if slot_ptr.is_null() {
        None
    } else {
        let slot = unsafe { &mut *(slot_ptr as *mut *mut A) };
        if slot.is_null() {
            None
        } else {
            Some(*unsafe { Box::from_raw(*slot) })
        }
    };

    // In xFinal there are no argument rows; pass an empty context.
    let context = unsafe { Context::from_raw(ctx, 0, std::ptr::null_mut()) };
    match agg.finalize(&context, acc) {
        Ok(v) => match v.to_sql() {
            Ok(out) => unsafe { set_result(ctx, out) },
            Err(e)  => unsafe { set_result_error(ctx, &e.to_string()) },
        },
        Err(e) => unsafe { set_result_error(ctx, &e.to_string()) },
    }
}

pub(crate) unsafe extern "C" fn agg_destroy<D>(p: *mut c_void) {
    unsafe { drop(Box::from_raw(p as *mut D)) };
}

// ── Connection methods (free functions called from connection.rs) ─────────────

/// Register a scalar SQL function.
///
/// Called via [`Connection::create_scalar_function`].
pub(crate) fn register_scalar<F, T>(
    db:     *mut sqlite3,
    name:   &str,
    n_args: i32,
    flags:  FunctionFlags,
    f:      F,
) -> Result<()>
where
    F: FnMut(&Context<'_>) -> Result<T> + Send + 'static,
    T: ToSql,
{
    let c_name = CString::new(name)?;
    let raw = Box::into_raw(Box::new(f));
    let enc = SQLITE_UTF8 as i32 | flags.bits();
    let rc = unsafe {
        sqlite3_create_function_v2(
            db,
            c_name.as_ptr(),
            n_args,
            enc,
            raw as *mut c_void,
            Some(scalar_trampoline::<F, T>),
            None,
            None,
            Some(scalar_destroy::<F>),
        )
    };
    if rc != SQLITE_OK {
        // xDestroy won't be called on failure; free manually.
        unsafe { drop(Box::from_raw(raw)) };
        return Err(sqlite_error_from_code(rc));
    }
    Ok(())
}

/// Register an aggregate SQL function.
///
/// Called via [`Connection::create_aggregate_function`].
pub(crate) fn register_aggregate<D, A, T>(
    db:     *mut sqlite3,
    name:   &str,
    n_args: i32,
    flags:  FunctionFlags,
    agg:    D,
) -> Result<()>
where
    D: Aggregate<A, T>,
    T: ToSql,
{
    let c_name = CString::new(name)?;
    let raw = Box::into_raw(Box::new(agg));
    let enc = SQLITE_UTF8 as i32 | flags.bits();
    let rc = unsafe {
        sqlite3_create_function_v2(
            db,
            c_name.as_ptr(),
            n_args,
            enc,
            raw as *mut c_void,
            None,
            Some(agg_step_trampoline::<D, A, T>),
            Some(agg_final_trampoline::<D, A, T>),
            Some(agg_destroy::<D>),
        )
    };
    if rc != SQLITE_OK {
        unsafe { drop(Box::from_raw(raw)) };
        return Err(sqlite_error_from_code(rc));
    }
    Ok(())
}

/// Remove a previously registered function.
///
/// Called via [`Connection::remove_function`].
pub(crate) fn deregister(db: *mut sqlite3, name: &str, n_args: i32) -> Result<()> {
    let c_name = CString::new(name)?;
    let rc = unsafe {
        sqlite3_create_function_v2(
            db,
            c_name.as_ptr(),
            n_args,
            SQLITE_UTF8 as i32,
            std::ptr::null_mut(),
            None,
            None,
            None,
            None,
        )
    };
    if rc != SQLITE_OK {
        return Err(sqlite_error_from_code(rc));
    }
    Ok(())
}
