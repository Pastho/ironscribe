//! IronScribe - A flexible and extensible logging framework for Rust applications
//!
//! This crate provides a pluggable architecture for logging to multiple destinations
//! including console, MongoDB, and PostgreSQL.

pub use crate::core::*;
pub use crate::service::LogService;

pub mod core;
pub mod destinations;
pub mod service;
pub mod macros;

// Re-export commonly used types
pub use core::log_entry::{LogEntry, LogLevel, LogMessageType};
pub use core::log_unit::LogUnit;
pub use service::default::DefaultLogService;

// Re-export macros
pub use crate::macros::*;

#[cfg(feature = "mongo")]
pub use destinations::mongodb::{MongoDestination, MongoConfig};

#[cfg(feature = "postgres")]
pub use destinations::postgres::{PostgresDestination, PostgresConfig};

#[cfg(feature = "console")]
pub use destinations::console::ConsoleDestination;