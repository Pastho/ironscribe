//! Core traits and types for the IronScribe logging framework

pub mod log_unit;
pub mod log_entry;
pub mod log_service;

pub use log_unit::LogUnit;
pub use log_entry::{LogEntry, LogLevel, LogMessageType};
pub use log_service::LogService;