#[cfg(feature = "postgres")]
use async_trait::async_trait;
#[cfg(feature = "postgres")]
use std::sync::Arc;
#[cfg(feature = "postgres")]
use tokio_postgres::{Client, NoTls};
#[cfg(feature = "postgres")]
use uuid::Uuid;

#[cfg(feature = "postgres")]
use crate::core::{LogEntry, LogService, LogUnit, LogLevel, LogMessageType};
#[cfg(feature = "postgres")]
use crate::core::log_service::LogResult;
#[cfg(feature = "postgres")]
use crate::destinations::console::ConsoleDestination;

#[cfg(feature = "postgres")]
#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub connection_string: String,
    pub log_units_table: String,
    pub log_entries_table: String,
}

#[cfg(feature = "postgres")]
impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            connection_string: "postgresql://localhost/ironscribe".to_string(),
            log_units_table: "log_units".to_string(),
            log_entries_table: "log_entries".to_string(),
        }
    }
}

#[cfg(feature = "postgres")]
pub struct PostgresDestination {
    client: Arc<Client>,
    config: PostgresConfig,
    console: ConsoleDestination,
}

#[cfg(feature = "postgres")]
impl PostgresDestination {
    pub async fn new(config: PostgresConfig) -> LogResult<Self> {
        let (client, connection) = tokio_postgres::connect(&config.connection_string, NoTls).await?;

        // Spawn the connection task
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("PostgreSQL connection error: {}", e);
            }
        });

        let destination = Self {
            client: Arc::new(client),
            config: config.clone(),
            console: ConsoleDestination::new(),
        };

        // Create tables if they don't exist
        destination.create_tables().await?;

        Ok(destination)
    }

    pub async fn with_default_config() -> LogResult<Self> {
        Self::new(PostgresConfig::default()).await
    }

    async fn create_tables(&self) -> LogResult<()> {
        // Create log_units table
        let create_units_table = format!(
            r#"
            CREATE TABLE IF NOT EXISTS {} (
                id UUID PRIMARY KEY,
                external_id VARCHAR NOT NULL,
                timestamp TIMESTAMPTZ NOT NULL
            )
            "#,
            self.config.log_units_table
        );

        // Create log_entries table
        let create_entries_table = format!(
            r#"
            CREATE TABLE IF NOT EXISTS {} (
                log_unit_id UUID NOT NULL,
                message_id UUID PRIMARY KEY,
                level INTEGER NOT NULL,
                message TEXT NOT NULL,
                message_type VARCHAR NOT NULL,
                timestamp TIMESTAMPTZ NOT NULL,
                FOREIGN KEY (log_unit_id) REFERENCES {} (id)
            )
            "#,
            self.config.log_entries_table,
            self.config.log_units_table
        );

        self.client.execute(&create_units_table, &[]).await?;
        self.client.execute(&create_entries_table, &[]).await?;

        Ok(())
    }

    fn message_type_to_string(msg_type: LogMessageType) -> &'static str {
        match msg_type {
            LogMessageType::Error => "Error",
            LogMessageType::Warning => "Warning",
            LogMessageType::Info => "Info",
            LogMessageType::Success => "Success",
        }
    }

    fn string_to_message_type(s: &str) -> LogResult<LogMessageType> {
        match s {
            "Error" => Ok(LogMessageType::Error),
            "Warning" => Ok(LogMessageType::Warning),
            "Info" => Ok(LogMessageType::Info),
            "Success" => Ok(LogMessageType::Success),
            _ => Err(format!("Unknown message type: {}", s).into()),
        }
    }
}

#[cfg(feature = "postgres")]
#[async_trait]
impl LogService for PostgresDestination {
    async fn create_log_unit(&self, external_id: String) -> LogResult<LogUnit> {
        let log_unit = LogUnit::new(external_id);

        // Store in PostgreSQL
        let query = format!(
            "INSERT INTO {} (id, external_id, timestamp) VALUES ($1, $2, $3)",
            self.config.log_units_table
        );

        self.client.execute(
            &query,
            &[&log_unit.id, &log_unit.external_id, &log_unit.timestamp]
        ).await?;

        // Also log to console
        self.console.create_log_unit(log_unit.external_id.clone()).await?;

        Ok(log_unit)
    }

    async fn log(&self, entry: LogEntry) -> LogResult<()> {
        // Store in PostgreSQL
        let query = format!(
            "INSERT INTO {} (log_unit_id, message_id, level, message, message_type, timestamp) VALUES ($1, $2, $3, $4, $5, $6)",
            self.config.log_entries_table
        );

        self.client.execute(
            &query,
            &[
                &entry.log_unit_id,
                &entry.message_id,
                &(entry.level as i32),
                &entry.message,
                &Self::message_type_to_string(entry.message_type),
                &entry.timestamp
            ]
        ).await?;

        // Also log to console
        self.console.log(entry).await?;

        Ok(())
    }

    async fn get_log_entries(&self, log_unit_id: Uuid) -> LogResult<Vec<LogEntry>> {
        let query = format!(
            "SELECT log_unit_id, message_id, level, message, message_type, timestamp FROM {} WHERE log_unit_id = $1 ORDER BY timestamp",
            self.config.log_entries_table
        );

        let rows = self.client.query(&query, &[&log_unit_id]).await?;
        let mut entries = Vec::new();

        for row in rows {
            let level_int: i32 = row.get(2);
            let level = match level_int {
                0 => LogLevel::Error,
                1 => LogLevel::Warning,
                2 => LogLevel::Info,
                3 => LogLevel::Success,
                _ => LogLevel::Info, // Default fallback
            };

            let message_type_str: String = row.get(4);
            let message_type = Self::string_to_message_type(&message_type_str)?;

            entries.push(LogEntry {
                log_unit_id: row.get(0),
                message_id: row.get(1),
                level,
                message: row.get(3),
                message_type,
                timestamp: row.get(5),
            });
        }

        Ok(entries)
    }

    async fn get_log_unit(&self, log_unit_id: Uuid) -> LogResult<Option<LogUnit>> {
        let query = format!(
            "SELECT id, external_id, timestamp FROM {} WHERE id = $1",
            self.config.log_units_table
        );

        let rows = self.client.query(&query, &[&log_unit_id]).await?;

        if let Some(row) = rows.first() {
            Ok(Some(LogUnit {
                id: row.get(0),
                external_id: row.get(1),
                timestamp: row.get(2),
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_log_units_by_external_id(&self, external_id: &str) -> LogResult<Vec<LogUnit>> {
        let query = format!(
            "SELECT id, external_id, timestamp FROM {} WHERE external_id = $1 ORDER BY timestamp",
            self.config.log_units_table
        );

        let rows = self.client.query(&query, &[&external_id]).await?;
        let mut units = Vec::new();

        for row in rows {
            units.push(LogUnit {
                id: row.get(0),
                external_id: row.get(1),
                timestamp: row.get(2),
            });
        }

        Ok(units)
    }
}

// Provide stub implementation when postgres feature is not enabled
#[cfg(not(feature = "postgres"))]
pub struct PostgresDestination;

#[cfg(not(feature = "postgres"))]
impl PostgresDestination {
    pub fn new() -> Self {
        panic!("PostgreSQL destination requires 'postgres' feature to be enabled");
    }
}