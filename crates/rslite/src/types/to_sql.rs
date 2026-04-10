use super::{Value, ValueRef};
use crate::Result;

/// What `ToSql::to_sql` produces — either a borrowed or owned value.
pub enum ToSqlOutput<'a> {
    Borrowed(ValueRef<'a>),
    Owned(Value),
}

impl<'a> ToSqlOutput<'a> {
    /// Borrow the contained value as a [`ValueRef`] without copying any heap data.
    ///
    /// When the output is [`ToSqlOutput::Borrowed`], the inner `ValueRef` is returned as-is.
    /// When it is [`ToSqlOutput::Owned`], a `ValueRef` is constructed that borrows from the owned
    /// [`Value`]'s storage, so no allocation takes place.  This method is called internally by
    /// [`Statement::bind_value`] immediately before passing the value to `sqlite3_bind_*`.
    pub fn as_value_ref(&'a self) -> ValueRef<'a> {
        match self {
            ToSqlOutput::Borrowed(v) => *v,
            ToSqlOutput::Owned(v) => match v {
                Value::Null       => ValueRef::Null,
                Value::Integer(i) => ValueRef::Integer(*i),
                Value::Real(f)    => ValueRef::Real(*f),
                Value::Text(s)    => ValueRef::Text(s.as_str()),
                Value::Blob(b)    => ValueRef::Blob(b.as_slice()),
            },
        }
    }
}

/// Implemented by types that can be bound to a SQLite parameter.
pub trait ToSql {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>>;
}

// ── Primitive impls ───────────────────────────────────────────────────────────

impl ToSql for i8  { fn to_sql(&self) -> Result<ToSqlOutput<'_>> { Ok(ToSqlOutput::Owned(Value::Integer(*self as i64))) } }
impl ToSql for i16 { fn to_sql(&self) -> Result<ToSqlOutput<'_>> { Ok(ToSqlOutput::Owned(Value::Integer(*self as i64))) } }
impl ToSql for i32 { fn to_sql(&self) -> Result<ToSqlOutput<'_>> { Ok(ToSqlOutput::Owned(Value::Integer(*self as i64))) } }
impl ToSql for i64 { fn to_sql(&self) -> Result<ToSqlOutput<'_>> { Ok(ToSqlOutput::Owned(Value::Integer(*self))) } }

impl ToSql for u8  { fn to_sql(&self) -> Result<ToSqlOutput<'_>> { Ok(ToSqlOutput::Owned(Value::Integer(*self as i64))) } }
impl ToSql for u16 { fn to_sql(&self) -> Result<ToSqlOutput<'_>> { Ok(ToSqlOutput::Owned(Value::Integer(*self as i64))) } }
impl ToSql for u32 { fn to_sql(&self) -> Result<ToSqlOutput<'_>> { Ok(ToSqlOutput::Owned(Value::Integer(*self as i64))) } }
impl ToSql for u64 {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        i64::try_from(*self)
            .map(|i| ToSqlOutput::Owned(Value::Integer(i)))
            .map_err(|e| crate::Error::ToSqlConversionFailure(Box::new(e)))
    }
}
impl ToSql for usize {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        i64::try_from(*self)
            .map(|i| ToSqlOutput::Owned(Value::Integer(i)))
            .map_err(|e| crate::Error::ToSqlConversionFailure(Box::new(e)))
    }
}

impl ToSql for f32 { fn to_sql(&self) -> Result<ToSqlOutput<'_>> { Ok(ToSqlOutput::Owned(Value::Real(*self as f64))) } }
impl ToSql for f64 { fn to_sql(&self) -> Result<ToSqlOutput<'_>> { Ok(ToSqlOutput::Owned(Value::Real(*self))) } }

impl ToSql for bool {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
    }
}

impl ToSql for String {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(self.as_str())))
    }
}

impl ToSql for str {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(self)))
    }
}

impl ToSql for Vec<u8> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Blob(self.as_slice())))
    }
}

impl ToSql for [u8] {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Blob(self)))
    }
}

impl<T: ToSql> ToSql for Option<T> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        match self {
            Some(v) => v.to_sql(),
            None    => Ok(ToSqlOutput::Owned(Value::Null)),
        }
    }
}

impl ToSql for Value {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(match self {
            Value::Null       => ToSqlOutput::Borrowed(ValueRef::Null),
            Value::Integer(i) => ToSqlOutput::Borrowed(ValueRef::Integer(*i)),
            Value::Real(f)    => ToSqlOutput::Borrowed(ValueRef::Real(*f)),
            Value::Text(s)    => ToSqlOutput::Borrowed(ValueRef::Text(s.as_str())),
            Value::Blob(b)    => ToSqlOutput::Borrowed(ValueRef::Blob(b.as_slice())),
        })
    }
}

impl<'a> ToSql for ValueRef<'a> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(*self))
    }
}

impl<T: ToSql + ?Sized> ToSql for &T {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        (*self).to_sql()
    }
}

impl<T: ToSql + ?Sized> ToSql for Box<T> {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        self.as_ref().to_sql()
    }
}

/// A sentinel for explicitly binding a SQL NULL parameter.
///
/// ```ignore
/// conn.execute("INSERT INTO t VALUES (?1)", (Null,))?;
/// ```
pub struct Null;

impl ToSql for Null {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Null))
    }
}
