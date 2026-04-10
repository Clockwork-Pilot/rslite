/// The SQLite column type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    Null,
    Integer,
    Real,
    Text,
    Blob,
}

/// An owned SQLite value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl Value {
    /// Returns the SQLite storage class of this value as a [`Type`] discriminant.
    ///
    /// The mapping follows the SQLite type affinity rules: `Integer` → [`Type::Integer`],
    /// `Real` → [`Type::Real`], `Text` → [`Type::Text`], `Blob` → [`Type::Blob`], and the
    /// `Null` variant → [`Type::Null`].  This is useful for generic code that inspects a
    /// value's type before deciding how to serialize or display it.
    pub fn data_type(&self) -> Type {
        match self {
            Value::Null       => Type::Null,
            Value::Integer(_) => Type::Integer,
            Value::Real(_)    => Type::Real,
            Value::Text(_)    => Type::Text,
            Value::Blob(_)    => Type::Blob,
        }
    }
}

/// A borrowed view into a SQLite value — zero-copy column access.
///
/// The lifetime `'a` is tied to the `Row` borrow, which in turn is tied to the
/// `Statement` step.  The pointer returned by `sqlite3_column_text` /
/// `sqlite3_column_blob` is only valid until the next `sqlite3_step` or
/// `sqlite3_finalize` call.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValueRef<'a> {
    Null,
    Integer(i64),
    Real(f64),
    Text(&'a str),
    Blob(&'a [u8]),
}

impl<'a> ValueRef<'a> {
    /// Returns the SQLite storage class of this borrowed value as a [`Type`] discriminant.
    ///
    /// Mirrors [`Value::data_type`] but works on the zero-copy [`ValueRef`] variant.  The result
    /// describes the actual in-memory representation, not the declared column affinity, so a column
    /// declared `INTEGER` that currently holds a floating-point literal will return [`Type::Real`].
    pub fn data_type(&self) -> Type {
        match self {
            ValueRef::Null       => Type::Null,
            ValueRef::Integer(_) => Type::Integer,
            ValueRef::Real(_)    => Type::Real,
            ValueRef::Text(_)    => Type::Text,
            ValueRef::Blob(_)    => Type::Blob,
        }
    }

    /// Extract the contained `i64` value, returning an [`Error::InvalidColumnType`] if the value
    /// is not [`ValueRef::Integer`].
    ///
    /// Unlike `as_f64`, this method does **not** coerce real values to integers; callers that
    /// need numeric coercion should call `as_f64` and then cast, or use the [`FromSql`] trait
    /// which handles type widening according to the column's declared affinity.
    pub fn as_i64(&self) -> crate::Result<i64> {
        match self {
            ValueRef::Integer(i) => Ok(*i),
            _ => Err(crate::Error::InvalidColumnType(0, String::new(), self.data_type())),
        }
    }

    /// Extract the contained floating-point value, widening an [`ValueRef::Integer`] to `f64`
    /// if necessary, or returning an [`Error::InvalidColumnType`] for text, blob, or null.
    ///
    /// The integer-to-float widening matches SQLite's own numeric coercion behaviour: an `i64`
    /// that does not fit exactly in an `f64` mantissa may lose precision, so callers that need
    /// lossless integer handling should prefer [`as_i64`](ValueRef::as_i64) instead.
    pub fn as_f64(&self) -> crate::Result<f64> {
        match self {
            ValueRef::Real(f)    => Ok(*f),
            ValueRef::Integer(i) => Ok(*i as f64),
            _ => Err(crate::Error::InvalidColumnType(0, String::new(), self.data_type())),
        }
    }

    /// Extract the contained UTF-8 string slice, or return an [`Error::InvalidColumnType`]
    /// if the value is not [`ValueRef::Text`].
    ///
    /// The returned slice borrows from the underlying `sqlite3_stmt` step and is valid only
    /// until the next call to `sqlite3_step` or `sqlite3_finalize`.  The borrow checker
    /// enforces this through the `'a` lifetime, which is tied to the enclosing [`Row`] borrow.
    pub fn as_str(&self) -> crate::Result<&'a str> {
        match self {
            ValueRef::Text(s) => Ok(s),
            _ => Err(crate::Error::InvalidColumnType(0, String::new(), self.data_type())),
        }
    }

    /// Extract the contained blob as a byte slice, or return an [`Error::InvalidColumnType`]
    /// if the value is not [`ValueRef::Blob`].
    ///
    /// Like [`as_str`](ValueRef::as_str), the returned slice borrows from the statement's
    /// current step and must not be used after the next `sqlite3_step`.  An empty blob
    /// (length 0) is represented as `&[]` rather than a null pointer, matching SQLite's
    /// documented behaviour for zero-length blobs.
    pub fn as_blob(&self) -> crate::Result<&'a [u8]> {
        match self {
            ValueRef::Blob(b) => Ok(b),
            _ => Err(crate::Error::InvalidColumnType(0, String::new(), self.data_type())),
        }
    }
}

impl From<ValueRef<'_>> for Value {
    fn from(r: ValueRef<'_>) -> Self {
        match r {
            ValueRef::Null       => Value::Null,
            ValueRef::Integer(i) => Value::Integer(i),
            ValueRef::Real(f)    => Value::Real(f),
            ValueRef::Text(s)    => Value::Text(s.to_owned()),
            ValueRef::Blob(b)    => Value::Blob(b.to_vec()),
        }
    }
}
