use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[cfg(feature = "mongo")]
use async_trait::async_trait;
#[cfg(feature = "mongo")]
use futures::TryStreamExt;
#[cfg(feature = "mongo")]
use mongodb::{bson::doc, Client, Collection, Database};
#[cfg(feature = "mongo")]
use crate::core::{LogEntry, LogService, LogUnit};
#[cfg(feature = "mongo")]
use crate::core::log_service::LogResult;
#[cfg(feature = "mongo")]
use crate::destinations::console::ConsoleDestination;
use crate::{LogLevel, LogMessageType};

#[cfg(feature = "mongo")]
#[derive(Debug, Clone)]
pub struct MongoConfig {
    pub connection_string: String,
    pub database_name: String,
    pub log_units_collection: String,
    pub log_entries_collection: String,
}
#[cfg(feature = "mongo")]
impl Default for MongoConfig {
    fn default() -> Self {
        Self {
            connection_string: "mongodb://localhost:27017".to_string(),
            database_name: "ironscribe".to_string(),
            log_units_collection: "log_units".to_string(),
            log_entries_collection: "log_entries".to_string(),
        }
    }
}

#[cfg(feature = "mongo")]
pub struct MongoDestination {
    database: Database,
    log_units: Collection<LogUnitWrapper>,
    log_entries: Collection<LogEntryWrapper>,
    console: ConsoleDestination,
}

#[cfg(feature = "mongo")]
impl MongoDestination {
    pub async fn new(config: MongoConfig) -> LogResult<Self> {
        let client = Client::with_uri_str(&config.connection_string)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let database = client.database(&config.database_name);
        let log_units = database.collection::<LogUnitWrapper>(&config.log_units_collection);
        let log_entries = database.collection::<LogEntryWrapper>(&config.log_entries_collection);
        Ok(Self {
            database,
            log_units,
            log_entries,
            console: ConsoleDestination::new(),
        })
    }

    pub async fn with_default_config() -> LogResult<Self> {
        Self::new(MongoConfig::default()).await
    }
}

#[cfg(feature = "mongo")]
#[async_trait]
impl LogService for MongoDestination {
    async fn create_log_unit(&self, external_id: String) -> LogResult<LogUnit> {
        let log_unit = LogUnit::new(external_id);
        // Store in MongoDB using wrapper
        let wrapper = LogUnitWrapper::from(log_unit.clone());
        self.log_units
            .insert_one(&wrapper)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        // Also log to console
        self.console.create_log_unit(log_unit.external_id.clone()).await?;
        Ok(log_unit)
    }

    async fn log(&self, entry: LogEntry) -> LogResult<()> {
        let wrapper = LogEntryWrapper::from(entry);
        // Store in MongoDB
        self.log_entries
            .insert_one(&wrapper)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        // Also log to console
        self.console.log(LogEntry::from(wrapper)).await?;
        Ok(())
    }

    async fn get_log_entries(&self, log_unit_id: Uuid) -> LogResult<Vec<LogEntry>> {
        use mongodb::bson::doc;

        let log_unit_id_string = log_unit_id.to_string();

        let filter = doc! { "log_unit_id": log_unit_id_string };
        let cursor = self.log_entries
            .find(filter)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let entries: Vec<LogEntryWrapper> = cursor
            .try_collect()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        Ok(entries.into_iter().map(LogEntry::from).collect())
    }

    async fn get_log_unit(&self, log_unit_id: Uuid) -> LogResult<Option<LogUnit>> {
        use mongodb::bson::doc;

        let log_unit_id_string = log_unit_id.to_string();

        let filter = doc! { "log_unit_id": log_unit_id_string };
        let unit = self.log_units
            .find_one(filter)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        Ok(unit.map(LogUnit::from))
    }

    async fn get_log_units_by_external_id(&self, external_id: &str) -> LogResult<Vec<LogUnit>> {
        use mongodb::bson::doc;
        let filter = doc! { "external_id": external_id };
        let cursor = self.log_units
            .find(filter)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let units: Vec<LogUnitWrapper> = cursor
            .try_collect()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        Ok(units.into_iter().map(LogUnit::from).collect())
    }
}

#[cfg(feature = "mongo")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntryWrapper {
    #[serde(rename = "_id")]
    pub message_id: String,
    pub log_unit_id: String,
    pub message: String,
    message_type: LogMessageType,
    level: LogLevel,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl From<LogEntry> for LogEntryWrapper {
    fn from(entry: LogEntry) -> Self {
        Self {
            message_id: entry.message_id.to_string(),
            log_unit_id: entry.log_unit_id.to_string(),
            message: entry.message,
            message_type: entry.message_type,
            level: entry.level,
            timestamp: entry.timestamp,
        }
    }
}

impl From<LogEntryWrapper> for LogEntry {
    fn from(wrapper: LogEntryWrapper) -> Self {
        Self {
            message_id: wrapper.message_id.parse::<Uuid>().unwrap(),
            log_unit_id: wrapper.log_unit_id.parse::<Uuid>().unwrap(),
            message: wrapper.message,
            message_type: wrapper.message_type,
            level: wrapper.level,
            timestamp: wrapper.timestamp,
        }
    }
}

#[cfg(feature = "mongo")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogUnitWrapper {
    #[serde(rename = "_id")]
    pub log_unit_id: String,
    pub external_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[cfg(feature = "mongo")]
impl From<LogUnit> for LogUnitWrapper {
    fn from(unit: LogUnit) -> Self {
        Self {
            log_unit_id: unit.log_unit_id.to_string(),
            external_id: unit.external_id,
            timestamp: unit.timestamp,
        }
    }
}

#[cfg(feature = "mongo")]
impl From<LogUnitWrapper> for LogUnit {
    fn from(wrapper: LogUnitWrapper) -> Self {
        Self {
            log_unit_id: Uuid::parse_str(&wrapper.log_unit_id).unwrap(),
            external_id: wrapper.external_id,
            timestamp: wrapper.timestamp,
        }
    }
}

// Provide stub implementation when mongodb feature is not enabled
#[cfg(not(feature = "mongo"))]
pub struct MongoDestination;

#[cfg(not(feature = "mongo"))]
impl MongoDestination {
    pub fn new() -> Self {
        panic!("MongoDB destination requires 'mongodb' feature to be enabled");
    }
}