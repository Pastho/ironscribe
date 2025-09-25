pub mod basic_logger;

use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone)]
pub enum LogType {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct LogMessage {
    pub id: Uuid,
    pub log_type: LogType,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

pub trait LogService {
    fn add_message(&mut self, log: LogMessage);
    fn flush_log(&mut self);
    fn save_log(&mut self);
    fn print_logs(&mut self);
}



// Global logger instance
static GLOBAL_LOGGER: OnceLock<Mutex<Box<dyn LogService + Send>>> = OnceLock::new();

/// Initialize the global logger with a specific implementation
pub fn init_logger<T: LogService + Send + 'static>(logger: T) -> Result<(), &'static str> {
    let boxed_logger = Box::new(logger);
    GLOBAL_LOGGER
        .set(Mutex::new(boxed_logger))
        .map_err(|_| "Logger has already been initialized")
}

/// Get access to the global logger for custom operations
pub fn with_logger<F, R>(f: F) -> R
where
    F: FnOnce(&mut dyn LogService) -> R,
{
    let logger = GLOBAL_LOGGER.get().expect("Logger not initialized. Call init_logger() first.");
    let mut guard = logger.lock().unwrap();
    f(guard.as_mut())
}

/// Internal function used by macros to add messages to the global logger
pub fn log_message(log_type: LogType, message: String) {
    with_logger(|logger| {
        let log = LogMessage {
            id: Uuid::new_v4(),
            log_type,
            message,
            timestamp: Utc::now(),
        };
        logger.add_message(log);
    });
}

// Simplified Global Logging Macros (no logger parameter needed)
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            $crate::core::log_message($crate::LogType::Info, message);
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            $crate::core::log_message($crate::LogType::Warning, message);
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            $crate::core::log_message($crate::LogType::Error, message);
        }
    };
}