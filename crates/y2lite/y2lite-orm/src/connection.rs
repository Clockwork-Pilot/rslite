//! Diesel Connection implementation

use y2lite;

/// Diesel Connection wrapper for y2lite
pub struct Connection {
    inner: y2lite::Connection,
}

impl Connection {
    /// Open a database connection
    pub fn open(path: &str) -> y2lite::Result<Self> {
        let inner = y2lite::Connection::open(path)?;
        Ok(Connection { inner })
    }
}

// TODO: Implement diesel::connection::Connection trait on Connection
// This will map y2lite::Connection to Diesel's expected interface
