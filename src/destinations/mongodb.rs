#[cfg(feature = "mongo")]
use async_trait::async_trait;
#[cfg(feature = "mongo")]
use futures::TryStreamExt;
#[cfg(feature = "mongo")]
use mongodb::{Client, Collection, Database};
#[cfg(feature = "mongo")]
use uuid::Uuid;

#[cfg(feature = "mongo")]
use crate::core::{LogEntry, LogService, LogUnit};
#[cfg(feature = "mongo")]
use crate::core::log_service::LogResult;
#[cfg(feature = "mongo")]
use crate::destinations::console::ConsoleDestination;

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
    log_units: Collection<LogUnit>,
    log_entries: Collection<LogEntry>,
    console: ConsoleDestination,
}

#[cfg(feature = "mongo")]
impl MongoDestination {
    pub async fn new(config: MongoConfig) -> LogResult<Self> {
        let client = Client::with_uri_str(&config.connection_string)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        let database = client.database(&config.database_name);
        let log_units = database.collection::<LogUnit>(&config.log_units_collection);
        let log_entries = database.collection::<LogEntry>(&config.log_entries_collection);

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

        // Store in MongoDB
        self.log_units
            .insert_one(&log_unit)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        // Also log to console
        self.console.create_log_unit(log_unit.external_id.clone()).await?;

        Ok(log_unit)
    }

    async fn log(&self, entry: LogEntry) -> LogResult<()> {
        // Store in MongoDB
        self.log_entries
            .insert_one(&entry)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        // Also log to console
        self.console.log(entry).await?;

        Ok(())
    }

    async fn get_log_entries(&self, log_unit_id: Uuid) -> LogResult<Vec<LogEntry>> {
        use mongodb::bson::doc;

        let filter = doc! { "log_unit_id": log_unit_id.to_string() };
        let cursor = self.log_entries
            .find(filter)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        let entries: Vec<LogEntry> = cursor
            .try_collect()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(entries)
    }

    async fn get_log_unit(&self, log_unit_id: Uuid) -> LogResult<Option<LogUnit>> {
        use mongodb::bson::doc;

        let filter = doc! { "id": log_unit_id.to_string() };
        let unit = self.log_units
            .find_one(filter)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(unit)
    }

    async fn get_log_units_by_external_id(&self, external_id: &str) -> LogResult<Vec<LogUnit>> {
        use mongodb::bson::doc;

        let filter = doc! { "external_id": external_id };
        let cursor = self.log_units
            .find(filter)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        let units: Vec<LogUnit> = cursor
            .try_collect()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(units)
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