use crate::types::ToSql;
use crate::{Error, Result, Statement};

mod private {
    pub trait Sealed {}
}

/// A sealed trait for anything that can be bound to a statement's parameters.
///
/// Implemented for:
/// - `()` — no parameters
/// - `&[&dyn ToSql]` / `&[T: ToSql]` — homogeneous or heterogeneous slice
/// - `[T; N]` — fixed-size arrays
/// - `&[(&str, &T)]` / `&[(&str, &dyn ToSql)]` — named parameters
/// - Tuples up to 16 elements
/// - `ParamsFromIter` — iterator-based
pub trait Params: private::Sealed {
    fn bind_to(self, stmt: &mut Statement<'_>) -> Result<()>;
}

// ── () — no params ────────────────────────────────────────────────────────────

impl private::Sealed for () {}
impl Params for () {
    fn bind_to(self, _: &mut Statement<'_>) -> Result<()> { Ok(()) }
}

// ── &[(&str, &T)] — named params (covers &[(&str, &dyn ToSql)] too) ──────────

impl<T: ToSql + ?Sized> private::Sealed for &[(&str, &T)] {}
impl<T: ToSql + ?Sized> Params for &[(&str, &T)] {
    fn bind_to(self, stmt: &mut Statement<'_>) -> Result<()> {
        for &(name, value) in self {
            let idx = stmt
                .parameter_index(name)?
                .ok_or_else(|| Error::InvalidParameterName(name.to_string()))?;
            stmt.bind_value(idx, value)?;
        }
        Ok(())
    }
}

// ── &[(&str, &T); N] — named params as array reference ───────────────────────

impl<const N: usize, T: ToSql + ?Sized> private::Sealed for &[(&str, &T); N] {}
impl<const N: usize, T: ToSql + ?Sized> Params for &[(&str, &T); N] {
    fn bind_to(self, stmt: &mut Statement<'_>) -> Result<()> {
        (self as &[_]).bind_to(stmt)
    }
}

// ── Vec<(&str, &dyn ToSql)> — for `vec![(":x", &val)]` patterns ──────────────

impl<T: ToSql + ?Sized> private::Sealed for Vec<(&str, &T)> {}
impl<T: ToSql + ?Sized> Params for Vec<(&str, &T)> {
    fn bind_to(self, stmt: &mut Statement<'_>) -> Result<()> {
        self.as_slice().bind_to(stmt)
    }
}

// ── &[T] where T: ToSql ───────────────────────────────────────────────────────

impl<T: ToSql> private::Sealed for &[T] {}
impl<T: ToSql> Params for &[T] {
    fn bind_to(self, stmt: &mut Statement<'_>) -> Result<()> {
        let expected = stmt.parameter_count();
        if self.len() != expected {
            return Err(Error::InvalidParameterCount(self.len(), expected));
        }
        for (i, p) in self.iter().enumerate() {
            stmt.bind_value(i + 1, p)?;
        }
        Ok(())
    }
}

// ── [T; N] — fixed-size arrays (e.g. `[1i32, 2i32]`, `[]`) ──────────────────

impl<const N: usize, T: ToSql> private::Sealed for [T; N] {}
impl<const N: usize, T: ToSql> Params for [T; N] {
    fn bind_to(self, stmt: &mut Statement<'_>) -> Result<()> {
        let expected = stmt.parameter_count();
        if N != expected {
            return Err(Error::InvalidParameterCount(N, expected));
        }
        for (i, p) in self.iter().enumerate() {
            stmt.bind_value(i + 1, p)?;
        }
        Ok(())
    }
}

// ── &[T; N] — array references ───────────────────────────────────────────────

impl<const N: usize, T: ToSql> private::Sealed for &[T; N] {}
impl<const N: usize, T: ToSql> Params for &[T; N] {
    fn bind_to(self, stmt: &mut Statement<'_>) -> Result<()> {
        self.as_slice().bind_to(stmt)
    }
}

// ── ParamsFromIter ────────────────────────────────────────────────────────────

/// Wraps an iterator of `ToSql` items as `Params`.
pub struct ParamsFromIter<I>(I);

/// Create positional params from any iterator of `ToSql` values.
pub fn params_from_iter<I>(iter: I) -> ParamsFromIter<I::IntoIter>
where
    I: IntoIterator,
    I::Item: ToSql,
{
    ParamsFromIter(iter.into_iter())
}

impl<I> private::Sealed for ParamsFromIter<I> {}
impl<I> Params for ParamsFromIter<I>
where
    I: Iterator,
    I::Item: ToSql,
{
    fn bind_to(self, stmt: &mut Statement<'_>) -> Result<()> {
        let mut count = 0usize;
        for item in self.0 {
            count += 1;
            stmt.bind_value(count, &item)?;
        }
        let expected = stmt.parameter_count();
        if count != expected {
            return Err(Error::InvalidParameterCount(count, expected));
        }
        Ok(())
    }
}

// ── Tuple impls (up to 16) ────────────────────────────────────────────────────

macro_rules! impl_params_for_tuple {
    ($($T:ident: $i:tt),+) => {
        impl<$($T: ToSql),+> private::Sealed for ($($T,)+) {}
        impl<$($T: ToSql),+> Params for ($($T,)+) {
            fn bind_to(self, stmt: &mut Statement<'_>) -> Result<()> {
                let count = [$($i,)+].len();
                let expected = stmt.parameter_count();
                if count != expected {
                    return Err(Error::InvalidParameterCount(count, expected));
                }
                $(stmt.bind_value($i + 1, &self.$i)?;)+
                Ok(())
            }
        }
    };
}

impl_params_for_tuple!(A:0);
impl_params_for_tuple!(A:0, B:1);
impl_params_for_tuple!(A:0, B:1, C:2);
impl_params_for_tuple!(A:0, B:1, C:2, D:3);
impl_params_for_tuple!(A:0, B:1, C:2, D:3, E:4);
impl_params_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5);
impl_params_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6);
impl_params_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7);
impl_params_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8);
impl_params_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9);
impl_params_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10);
impl_params_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11);
impl_params_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12);
impl_params_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13);
impl_params_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14);
impl_params_for_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15);

/// Bind positional parameters. Accepts heterogeneous types.
///
/// ```ignore
/// conn.execute("INSERT INTO t VALUES (?1, ?2)", params![42i32, "hello"])?;
/// ```
#[macro_export]
macro_rules! params {
    () => {
        () as ()
    };
    ($($v:expr),+ $(,)?) => {
        &[$(&$v as &dyn $crate::types::ToSql),+] as &[&dyn $crate::types::ToSql]
    };
}

/// Bind named parameters. Accepts heterogeneous types.
///
/// ```ignore
/// conn.execute(
///     "INSERT INTO t VALUES (:id, :name)",
///     named_params!{ ":id": 42i32, ":name": "hello" },
/// )?;
/// ```
#[macro_export]
macro_rules! named_params {
    () => {
        &[] as &[(&str, &dyn $crate::types::ToSql)]
    };
    ($($n:literal: $v:expr),+ $(,)?) => {
        &[$(($n, &$v as &dyn $crate::types::ToSql)),+] as &[(&str, &dyn $crate::types::ToSql)]
    };
}
