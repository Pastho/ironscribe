use async_trait::async_trait;
use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::core::{LogEntry, LogService, LogUnit, LogMessageType};
use crate::core::log_service::LogResult;

/// Console-based log destination that prints colored output
pub struct ConsoleDestination {
    log_units: Arc<RwLock<HashMap<Uuid, LogUnit>>>,
    log_entries: Arc<RwLock<HashMap<Uuid, Vec<LogEntry>>>>,
}

impl ConsoleDestination {
    /// Creates a new console destination
    pub fn new() -> Self {
        Self {
            log_units: Arc::new(RwLock::new(HashMap::new())),
            log_entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Prints a log entry to the console with appropriate colors
    fn print_entry(&self, entry: &LogEntry) {
        let formatted_time = entry.timestamp.format("%Y-%m-%d %H:%M:%S UTC");
        let level_str = match entry.message_type {
            LogMessageType::Error => "ERROR".red().bold().to_string(),
            LogMessageType::Warning => "WARN".yellow().bold().to_string(),
            LogMessageType::Info => "INFO".blue().bold().to_string(),
            LogMessageType::Success => "SUCCESS".green().bold().to_string(),
        };

        println!(
            "[{}] [{}] [{}] {}",
            formatted_time,
            level_str,
            entry.log_unit_id.to_string().dimmed(),
            entry.message
        );
    }
}

impl Default for ConsoleDestination {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LogService for ConsoleDestination {
    async fn create_log_unit(&self, external_id: String) -> LogResult<LogUnit> {
        let log_unit = LogUnit::new(external_id);

        // Store the log unit
        let mut units = self.log_units.write().await;
        units.insert(log_unit.id, log_unit.clone());

        // Initialize empty entries vector for this unit
        let mut entries = self.log_entries.write().await;
        entries.insert(log_unit.id, Vec::new());

        Ok(log_unit)
    }

    async fn log(&self, entry: LogEntry) -> LogResult<()> {
        // Print to console
        self.print_entry(&entry);

        // Store the entry
        let mut entries = self.log_entries.write().await;
        if let Some(unit_entries) = entries.get_mut(&entry.log_unit_id) {
            unit_entries.push(entry);
        } else {
            entries.insert(entry.log_unit_id, vec![entry]);
        }

        Ok(())
    }

    async fn get_log_entries(&self, log_unit_id: Uuid) -> LogResult<Vec<LogEntry>> {
        let entries = self.log_entries.read().await;
        Ok(entries.get(&log_unit_id).cloned().unwrap_or_default())
    }

    async fn get_log_unit(&self, log_unit_id: Uuid) -> LogResult<Option<LogUnit>> {
        let units = self.log_units.read().await;
        Ok(units.get(&log_unit_id).cloned())
    }

    async fn get_log_units_by_external_id(&self, external_id: &str) -> LogResult<Vec<LogUnit>> {
        let units = self.log_units.read().await;
        let matching_units: Vec<LogUnit> = units
            .values()
            .filter(|unit| unit.external_id == external_id)
            .cloned()
            .collect();
        Ok(matching_units)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_console_destination() {
        let destination = ConsoleDestination::new();
        let unit = destination.create_log_unit("test".to_string()).await.unwrap();

        let entry = LogEntry::info(unit.id, "Test message".to_string());
        destination.log(entry.clone()).await.unwrap();

        let entries = destination.get_log_entries(unit.id).await.unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].message, "Test message");
    }
}