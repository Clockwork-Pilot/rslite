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
    pub fn data_type(&self) -> Type {
        match self {
            ValueRef::Null       => Type::Null,
            ValueRef::Integer(_) => Type::Integer,
            ValueRef::Real(_)    => Type::Real,
            ValueRef::Text(_)    => Type::Text,
            ValueRef::Blob(_)    => Type::Blob,
        }
    }

    pub fn as_i64(&self) -> crate::Result<i64> {
        match self {
            ValueRef::Integer(i) => Ok(*i),
            _ => Err(crate::Error::InvalidColumnType(0, String::new(), self.data_type())),
        }
    }

    pub fn as_f64(&self) -> crate::Result<f64> {
        match self {
            ValueRef::Real(f)    => Ok(*f),
            ValueRef::Integer(i) => Ok(*i as f64),
            _ => Err(crate::Error::InvalidColumnType(0, String::new(), self.data_type())),
        }
    }

    pub fn as_str(&self) -> crate::Result<&'a str> {
        match self {
            ValueRef::Text(s) => Ok(s),
            _ => Err(crate::Error::InvalidColumnType(0, String::new(), self.data_type())),
        }
    }

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
