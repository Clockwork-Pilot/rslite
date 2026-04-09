use super::{Type, ValueRef};
use std::fmt;

/// Error from a `FromSql` conversion.
#[derive(Debug)]
pub enum FromSqlError {
    /// The SQLite type cannot be converted to this Rust type.
    InvalidType,
    /// An integer value was out of range for the target type.
    OutOfRange(i64),
    /// A custom error from a user-defined `FromSql` impl.
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl fmt::Display for FromSqlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FromSqlError::InvalidType  => write!(f, "invalid type for conversion"),
            FromSqlError::OutOfRange(v) => write!(f, "integer {} out of range", v),
            FromSqlError::Other(e)     => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for FromSqlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FromSqlError::Other(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

pub type FromSqlResult<T> = std::result::Result<T, FromSqlError>;

/// Implemented by types that can be extracted from a SQLite column.
pub trait FromSql: Sized {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self>;
}

// ── Primitive impls ───────────────────────────────────────────────────────────

impl FromSql for i8 {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        i64::column_result(v).and_then(|i| i8::try_from(i).map_err(|_| FromSqlError::OutOfRange(i)))
    }
}
impl FromSql for i16 {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        i64::column_result(v).and_then(|i| i16::try_from(i).map_err(|_| FromSqlError::OutOfRange(i)))
    }
}
impl FromSql for i32 {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        i64::column_result(v).and_then(|i| i32::try_from(i).map_err(|_| FromSqlError::OutOfRange(i)))
    }
}
impl FromSql for i64 {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        match v {
            ValueRef::Integer(i) => Ok(i),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl FromSql for u8 {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        i64::column_result(v).and_then(|i| u8::try_from(i).map_err(|_| FromSqlError::OutOfRange(i)))
    }
}
impl FromSql for u16 {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        i64::column_result(v).and_then(|i| u16::try_from(i).map_err(|_| FromSqlError::OutOfRange(i)))
    }
}
impl FromSql for u32 {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        i64::column_result(v).and_then(|i| u32::try_from(i).map_err(|_| FromSqlError::OutOfRange(i)))
    }
}
impl FromSql for u64 {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        i64::column_result(v).and_then(|i| u64::try_from(i).map_err(|_| FromSqlError::OutOfRange(i)))
    }
}
impl FromSql for usize {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        i64::column_result(v).and_then(|i| usize::try_from(i).map_err(|_| FromSqlError::OutOfRange(i)))
    }
}

impl FromSql for f32 {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        f64::column_result(v).map(|f| f as f32)
    }
}
impl FromSql for f64 {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        match v {
            ValueRef::Real(f)    => Ok(f),
            ValueRef::Integer(i) => Ok(i as f64),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl FromSql for bool {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        i64::column_result(v).map(|i| i != 0)
    }
}

impl FromSql for String {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        match v {
            ValueRef::Text(s) => Ok(s.to_owned()),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl FromSql for Vec<u8> {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        match v {
            ValueRef::Blob(b) => Ok(b.to_vec()),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl<T: FromSql> FromSql for Option<T> {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        match v {
            ValueRef::Null => Ok(None),
            _              => T::column_result(v).map(Some),
        }
    }
}

impl FromSql for super::Value {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(v.into())
    }
}

impl FromSql for Type {
    fn column_result(v: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(v.data_type())
    }
}
