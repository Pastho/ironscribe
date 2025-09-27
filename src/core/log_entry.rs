use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents the type of log message
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogMessageType {
    Error,
    Warning,
    Info,
    Success,
}

/// Numeric log levels for sorting
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 0,
    Warning = 1,
    Info = 2,
    Success = 3,
}

impl From<LogMessageType> for LogLevel {
    fn from(msg_type: LogMessageType) -> Self {
        match msg_type {
            LogMessageType::Error => LogLevel::Error,
            LogMessageType::Warning => LogLevel::Warning,
            LogMessageType::Info => LogLevel::Info,
            LogMessageType::Success => LogLevel::Success,
        }
    }
}

/// Represents a single log entry/message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogEntry {
    /// ID of the parent log unit
    pub log_unit_id: Uuid,
    /// Unique identifier for this log message
    pub message_id: Uuid,
    /// Numeric log level for sorting
    pub level: LogLevel,
    /// The actual log message content
    pub message: String,
    /// Type of the log message
    pub message_type: LogMessageType,
    /// Timestamp when the log message was created
    pub timestamp: DateTime<Utc>,
}

impl LogEntry {
    /// Creates a new log entry
    pub fn new(
        log_unit_id: Uuid,
        message: String,
        message_type: LogMessageType,
    ) -> Self {
        Self {
            log_unit_id,
            message_id: Uuid::new_v4(),
            level: LogLevel::from(message_type),
            message,
            message_type,
            timestamp: Utc::now(),
        }
    }

    /// Creates an error log entry
    pub fn error(log_unit_id: Uuid, message: String) -> Self {
        Self::new(log_unit_id, message, LogMessageType::Error)
    }

    /// Creates a warning log entry
    pub fn warning(log_unit_id: Uuid, message: String) -> Self {
        Self::new(log_unit_id, message, LogMessageType::Warning)
    }

    /// Creates an info log entry
    pub fn info(log_unit_id: Uuid, message: String) -> Self {
        Self::new(log_unit_id, message, LogMessageType::Info)
    }

    /// Creates a success log entry
    pub fn success(log_unit_id: Uuid, message: String) -> Self {
        Self::new(log_unit_id, message, LogMessageType::Success)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_entry_creation() {
        let unit_id = Uuid::new_v4();
        let message = "Test message".to_string();
        let entry = LogEntry::info(unit_id, message.clone());

        assert_eq!(entry.log_unit_id, unit_id);
        assert_eq!(entry.message, message);
        assert_eq!(entry.message_type, LogMessageType::Info);
        assert_eq!(entry.level, LogLevel::Info);
    }

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Error < LogLevel::Warning);
        assert!(LogLevel::Warning < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Success);
    }

    #[test]
    fn test_message_type_to_level_conversion() {
        assert_eq!(LogLevel::from(LogMessageType::Error), LogLevel::Error);
        assert_eq!(LogLevel::from(LogMessageType::Warning), LogLevel::Warning);
        assert_eq!(LogLevel::from(LogMessageType::Info), LogLevel::Info);
        assert_eq!(LogLevel::from(LogMessageType::Success), LogLevel::Success);
    }
}