//! Diesel ORM integration for pub-interface
//!
//! This crate provides a Diesel Connection implementation backed by pub-interface-wrapper,
//! enabling full ORM support with query builders, migrations, and type safety.
//!
//! # Example
//! ```ignore
//! use pub_interface_orm::Connection;
//! use diesel::prelude::*;
//!
//! let mut conn = Connection::open(":memory:")?;
//! diesel::sql_query("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)").execute(&mut conn)?;
//! ```

pub mod connection;
pub mod transaction;

pub use connection::Connection;
