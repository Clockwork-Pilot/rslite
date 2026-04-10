use std::fmt;
use sqlite_noamalgam::{sqlite3, sqlite3_errmsg, sqlite3_errcode, sqlite3_extended_errcode};
use std::ffi::CStr;

/// Primary SQLite error codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorCode {
    InternalMalfunction,
    PermissionDenied,
    OperationAborted,
    DatabaseBusy,
    DatabaseLocked,
    OutOfMemory,
    ReadOnly,
    OperationInterrupted,
    SystemIoFailure,
    DatabaseCorrupt,
    NotFound,
    DiskFull,
    CannotOpen,
    FileLockingProtocolFailed,
    SchemaChanged,
    TooBig,
    ConstraintViolation,
    TypeMismatch,
    ApiMisuse,
    NoLargeFileSupport,
    AuthorizationDenied,
    ParameterOutOfRange,
    NotADatabase,
    Unknown(i32),
}

impl ErrorCode {
    /// Map a raw SQLite integer result code to the most appropriate [`ErrorCode`] variant.
    ///
    /// Only the lower 8 bits are examined, which strips extended error information and yields
    /// the primary error class (e.g. `SQLITE_CONSTRAINT_NOTNULL` → `ConstraintViolation`).
    /// Codes that do not correspond to a known primary error class are wrapped in the
    /// `Unknown(i32)` variant so that future SQLite versions do not cause a panic.
    pub(crate) fn from_i32(code: i32) -> Self {
        match code & 0xff {
            1  => ErrorCode::InternalMalfunction,
            2  => ErrorCode::PermissionDenied,
            3  => ErrorCode::OperationAborted,
            5  => ErrorCode::DatabaseBusy,
            6  => ErrorCode::DatabaseLocked,
            7  => ErrorCode::OutOfMemory,
            8  => ErrorCode::ReadOnly,
            9  => ErrorCode::OperationInterrupted,
            10 => ErrorCode::SystemIoFailure,
            11 => ErrorCode::DatabaseCorrupt,
            12 => ErrorCode::NotFound,
            13 => ErrorCode::DiskFull,
            14 => ErrorCode::CannotOpen,
            15 => ErrorCode::FileLockingProtocolFailed,
            17 => ErrorCode::SchemaChanged,
            18 => ErrorCode::TooBig,
            19 => ErrorCode::ConstraintViolation,
            20 => ErrorCode::TypeMismatch,
            21 => ErrorCode::ApiMisuse,
            22 => ErrorCode::NoLargeFileSupport,
            23 => ErrorCode::AuthorizationDenied,
            25 => ErrorCode::ParameterOutOfRange,
            26 => ErrorCode::NotADatabase,
            c  => ErrorCode::Unknown(c),
        }
    }
}

/// Low-level SQLite error (code + message).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SqliteError {
    pub code: ErrorCode,
    pub extended_code: i32,
}

impl SqliteError {
    /// Build a [`SqliteError`] by querying the current error state from a live `sqlite3*` handle.
    ///
    /// Calls `sqlite3_errcode` for the primary code and `sqlite3_extended_errcode` for the
    /// extended code, which together describe both the error class and the specific cause
    /// (e.g. `SQLITE_CONSTRAINT` vs `SQLITE_CONSTRAINT_NOTNULL`).  The caller must ensure
    /// the handle is non-null and that the most recent API call on it actually failed.
    pub(crate) unsafe fn from_db(db: *mut sqlite3) -> Self {
        let extended = unsafe { sqlite3_extended_errcode(db) };
        let primary   = unsafe { sqlite3_errcode(db) };
        SqliteError {
            code: ErrorCode::from_i32(primary),
            extended_code: extended,
        }
    }

    /// Build a [`SqliteError`] from a bare integer return code, without a live database handle.
    ///
    /// Used in situations where no `sqlite3*` is available — for example, when `sqlite3_open`
    /// fails before the handle is valid, or when an error originates from a context-free API.
    /// Both `code` and `extended_code` are set to `rc` because no extended information can be
    /// retrieved without calling `sqlite3_extended_errcode` on a valid connection handle.
    pub(crate) fn from_code(rc: i32) -> Self {
        SqliteError {
            code: ErrorCode::from_i32(rc),
            extended_code: rc,
        }
    }
}

impl fmt::Display for SqliteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SQLite error {:?} ({})", self.code, self.extended_code)
    }
}

/// The top-level error type for rslite.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// SQLite returned an error code (with optional message from the db).
    SqliteFailure(SqliteError, Option<String>),
    /// A conversion from a SQL value to a Rust type failed.
    FromSqlConversionFailure(usize, crate::types::Type, Box<dyn std::error::Error + Send + Sync + 'static>),
    /// An integer value was out of range for the target type.
    IntegralValueOutOfRange(usize, i64),
    /// A text value was not valid UTF-8.
    Utf8Error(std::str::Utf8Error),
    /// A parameter string contained an interior NUL byte.
    NulError(std::ffi::NulError),
    /// A named parameter did not exist in the statement.
    InvalidParameterName(String),
    /// A file path could not be converted.
    InvalidPath(std::path::PathBuf),
    /// `execute` was called on a query that returns rows.
    ExecuteReturnedResults,
    /// `query_row` found no rows.
    QueryReturnedNoRows,
    /// A column index was out of range.
    InvalidColumnIndex(usize),
    /// A column name was not found.
    InvalidColumnName(String),
    /// A column's SQLite type differed from the requested Rust type.
    InvalidColumnType(usize, String, crate::types::Type),
    /// The number of parameters provided did not match the statement.
    InvalidParameterCount(usize, usize),
    /// A `ToSql` implementation returned an error.
    ToSqlConversionFailure(Box<dyn std::error::Error + Send + Sync + 'static>),
    /// The SQL string contained multiple statements (where one was expected).
    MultipleStatement,
    /// `Statement::insert` changed a number of rows other than one.
    StatementChangedRows(usize),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Error::SqliteFailure(a, am), Error::SqliteFailure(b, bm)) => a == b && am == bm,
            (Error::IntegralValueOutOfRange(ac, av), Error::IntegralValueOutOfRange(bc, bv)) => ac == bc && av == bv,
            (Error::Utf8Error(a), Error::Utf8Error(b)) => a == b,
            (Error::NulError(a), Error::NulError(b)) => a.nul_position() == b.nul_position(),
            (Error::InvalidParameterName(a), Error::InvalidParameterName(b)) => a == b,
            (Error::InvalidPath(a), Error::InvalidPath(b)) => a == b,
            (Error::ExecuteReturnedResults, Error::ExecuteReturnedResults) => true,
            (Error::QueryReturnedNoRows, Error::QueryReturnedNoRows) => true,
            (Error::InvalidColumnIndex(a), Error::InvalidColumnIndex(b)) => a == b,
            (Error::InvalidColumnName(a), Error::InvalidColumnName(b)) => a == b,
            (Error::InvalidColumnType(ai, an, at), Error::InvalidColumnType(bi, bn, bt)) => ai == bi && an == bn && at == bt,
            (Error::InvalidParameterCount(ag, ae), Error::InvalidParameterCount(bg, be)) => ag == bg && ae == be,
            (Error::MultipleStatement, Error::MultipleStatement) => true,
            (Error::StatementChangedRows(a), Error::StatementChangedRows(b)) => a == b,
            // Box<dyn Error> variants: compare by Display string (best-effort)
            (Error::FromSqlConversionFailure(ai, at, ae), Error::FromSqlConversionFailure(bi, bt, be)) =>
                ai == bi && at == bt && ae.to_string() == be.to_string(),
            (Error::ToSqlConversionFailure(a), Error::ToSqlConversionFailure(b)) =>
                a.to_string() == b.to_string(),
            _ => false,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::SqliteFailure(e, Some(msg)) => write!(f, "{}: {}", e, msg),
            Error::SqliteFailure(e, None)      => write!(f, "{}", e),
            Error::FromSqlConversionFailure(col, ty, e) =>
                write!(f, "column {} has type {:?} but conversion failed: {}", col, ty, e),
            Error::IntegralValueOutOfRange(col, val) =>
                write!(f, "column {}: integer {} out of range", col, val),
            Error::Utf8Error(e)         => write!(f, "UTF-8 error: {}", e),
            Error::NulError(e)          => write!(f, "NUL in string: {}", e),
            Error::InvalidParameterName(n) => write!(f, "no parameter named {:?}", n),
            Error::InvalidPath(p)       => write!(f, "invalid path: {}", p.display()),
            Error::ExecuteReturnedResults  => write!(f, "execute called on a query returning rows"),
            Error::QueryReturnedNoRows     => write!(f, "query returned no rows"),
            Error::InvalidColumnIndex(i)   => write!(f, "invalid column index {}", i),
            Error::InvalidColumnName(n)    => write!(f, "no column named {:?}", n),
            Error::InvalidColumnType(i, n, t) =>
                write!(f, "column {} ({:?}) has unexpected type {:?}", i, n, t),
            Error::InvalidParameterCount(given, expected) =>
                write!(f, "wrong number of parameters: got {}, expected {}", given, expected),
            Error::ToSqlConversionFailure(e) => write!(f, "ToSql conversion failed: {}", e),
            Error::MultipleStatement       => write!(f, "SQL contained multiple statements"),
            Error::StatementChangedRows(n) => write!(f, "query changed {} rows, expected 1", n),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Utf8Error(e) => Some(e),
            Error::NulError(e)  => Some(e),
            Error::FromSqlConversionFailure(_, _, e) => Some(e.as_ref()),
            Error::ToSqlConversionFailure(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self { Error::Utf8Error(e) }
}

impl From<std::ffi::NulError> for Error {
    fn from(e: std::ffi::NulError) -> Self { Error::NulError(e) }
}

/// Convenience result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Extension trait that adds `.optional()` to `Result<T>`, converting
/// `Err(Error::QueryReturnedNoRows)` into `Ok(None)`.
pub trait OptionalExtension<T> {
    fn optional(self) -> Result<Option<T>>;
}

impl<T> OptionalExtension<T> for Result<T> {
    /// Convert `Err(Error::QueryReturnedNoRows)` into `Ok(None)`, leaving all other variants
    /// unchanged.
    ///
    /// This makes it ergonomic to run a `query_row` call and treat "not found" as an optional
    /// value rather than a hard error, similar to how database ORMs expose nullable lookups.
    /// Any other error — I/O failures, type mismatches, SQL errors — is propagated as-is so
    /// that only the "no rows" condition is silently converted.
    fn optional(self) -> Result<Option<T>> {
        match self {
            Ok(v)                           => Ok(Some(v)),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(e)                          => Err(e),
        }
    }
}

/// Build an `Error::SqliteFailure` from a raw db pointer and return code.
pub(crate) unsafe fn sqlite_error(db: *mut sqlite3, _rc: i32) -> Error {
    let sqlite_err = unsafe { SqliteError::from_db(db) };
    let msg = unsafe {
        let p = sqlite3_errmsg(db);
        if p.is_null() { None }
        else { Some(CStr::from_ptr(p).to_string_lossy().into_owned()) }
    };
    Error::SqliteFailure(sqlite_err, msg)
}

/// Build an `Error::SqliteFailure` from just a return code (no db handle).
pub(crate) fn sqlite_error_from_code(rc: i32) -> Error {
    Error::SqliteFailure(SqliteError::from_code(rc), None)
}
