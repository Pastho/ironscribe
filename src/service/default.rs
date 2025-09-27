use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::core::{LogEntry, LogService as LogServiceTrait, LogUnit};
use crate::core::log_service::LogResult;

#[cfg(feature = "console")]
use crate::destinations::ConsoleDestination;
#[cfg(feature = "mongo")]
use crate::destinations::MongoDestination;
#[cfg(feature = "postgres")]
use crate::destinations::PostgresDestination;

/// Default log service implementation that delegates to the configured destination
pub struct DefaultLogService {
    destination: Arc<dyn LogServiceTrait>,
}

impl DefaultLogService {
    /// Creates a new default log service with console destination
    #[cfg(feature = "console")]
    pub fn new_console() -> Self {
        Self {
            destination: Arc::new(ConsoleDestination::new()),
        }
    }

    /// Creates a new default log service with MongoDB destination
    #[cfg(feature = "mongo")]
    pub async fn new_mongodb(config: crate::destinations::mongodb::MongoConfig) -> LogResult<Self> {
        let destination = MongoDestination::new(config).await?;
        Ok(Self {
            destination: Arc::new(destination),
        })
    }

    /// Creates a new default log service with PostgreSQL destination
    #[cfg(feature = "postgres")]
    pub async fn new_postgres(config: crate::destinations::postgres::PostgresConfig) -> LogResult<Self> {
        let destination = PostgresDestination::new(config).await?;
        Ok(Self {
            destination: Arc::new(destination),
        })
    }

    /// Creates a new default log service with the default destination (console)
    pub fn new() -> Self {
        #[cfg(feature = "console")]
        {
            Self::new_console()
        }
        #[cfg(not(feature = "console"))]
        {
            panic!("No default destination available. Enable at least one feature: console, mongodb, or postgres");
        }
    }

    /// Creates a service with a custom destination
    pub fn with_destination(destination: Arc<dyn LogServiceTrait>) -> Self {
        Self { destination }
    }
}

impl Default for DefaultLogService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LogServiceTrait for DefaultLogService {
    async fn create_log_unit(&self, external_id: String) -> LogResult<LogUnit> {
        self.destination.create_log_unit(external_id).await
    }

    async fn log(&self, entry: LogEntry) -> LogResult<()> {
        self.destination.log(entry).await
    }

    async fn get_log_entries(&self, log_unit_id: Uuid) -> LogResult<Vec<LogEntry>> {
        self.destination.get_log_entries(log_unit_id).await
    }

    async fn get_log_unit(&self, log_unit_id: Uuid) -> LogResult<Option<LogUnit>> {
        self.destination.get_log_unit(log_unit_id).await
    }

    async fn get_log_units_by_external_id(&self, external_id: &str) -> LogResult<Vec<LogUnit>> {
        self.destination.get_log_units_by_external_id(external_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::LogEntry;

    #[tokio::test]
    #[cfg(feature = "console")]
    async fn test_default_log_service() {
        let service = DefaultLogService::new();
        let unit = service.create_log_unit("test".to_string()).await.unwrap();

        let entry = LogEntry::info(unit.id, "Test message".to_string());
        service.log(entry).await.unwrap();

        let entries = service.get_log_entries(unit.id).await.unwrap();
        assert_eq!(entries.len(), 1);
    }
}