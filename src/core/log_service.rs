use async_trait::async_trait;
use std::error::Error;
use uuid::Uuid;

use crate::core::{LogEntry, LogUnit};

/// Result type for log service operations
pub type LogResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

/// Core trait for log service implementations
#[async_trait]
pub trait LogService: Send + Sync {
    /// Creates a new log unit
    async fn create_log_unit(&self, external_id: String) -> LogResult<LogUnit>;

    /// Logs an entry to the service
    async fn log(&self, entry: LogEntry) -> LogResult<()>;

    /// Retrieves log entries for a specific log unit
    async fn get_log_entries(&self, log_unit_id: Uuid) -> LogResult<Vec<LogEntry>>;

    /// Retrieves a log unit by its ID
    async fn get_log_unit(&self, log_unit_id: Uuid) -> LogResult<Option<LogUnit>>;

    /// Retrieves log units by external ID
    async fn get_log_units_by_external_id(&self, external_id: &str) -> LogResult<Vec<LogUnit>>;
}