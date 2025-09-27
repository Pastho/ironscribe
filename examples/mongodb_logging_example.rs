use ironscribe::{
    DefaultLogService,
    LogEntry,
    LogMessageType,
    MongoConfig,
    MongoDestination,
    core::LogService,
    log_error,
    log_info,
    log_success,
    log_warn,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🚀 Starting MongoDB logging example...");

    // Method 1: Using MongoDestination directly with default config
    let mongo_destination = MongoDestination::with_default_config().await?;

    // Create a log unit for this session
    let log_unit =
        LogService::create_log_unit(&mongo_destination, "mongodb-example-session".to_string())
            .await?;

    println!(
        "📝 Created log unit: {} (ID: {})",
        log_unit.external_id, log_unit.id
    );

    // Log different types of messages
    LogService::log(
        &mongo_destination,
        LogEntry::info(log_unit.id, "Application started successfully".to_string()),
    )
    .await?;

    LogService::log(
        &mongo_destination,
        LogEntry::warning(log_unit.id, "This is a warning message".to_string()),
    )
    .await?;

    LogService::log(
        &mongo_destination,
        LogEntry::error(
            log_unit.id,
            "An error occurred during processing".to_string(),
        ),
    )
    .await?;

    LogService::log(
        &mongo_destination,
        LogEntry::success(log_unit.id, "Task completed successfully".to_string()),
    )
    .await?;

    // Method 2: Using DefaultLogService with custom MongoDB config
    let custom_config = MongoConfig {
        connection_string: "mongodb://localhost:27017".to_string(),
        database_name: "my_app_logs".to_string(),
        log_units_collection: "application_log_units".to_string(),
        log_entries_collection: "application_log_entries".to_string(),
    };

    let service = DefaultLogService::new_mongodb(custom_config).await?;

    // Create another log unit for a different workflow
    let workflow_unit =
        LogService::create_log_unit(&service, "data-processing-workflow".to_string()).await?;

    // Use the convenient macros for logging
    log_info!(
        service,
        workflow_unit.id,
        "Starting data processing workflow"
    )?;

    // Simulate some processing steps
    for i in 1..=5 {
        log_info!(service, workflow_unit.id, "Processing batch {} of 5", i)?;

        // Simulate some work
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        if i == 3 {
            log_warn!(
                service,
                workflow_unit.id,
                "Batch {} took longer than expected",
                i
            )?;
        }
    }

    log_success!(
        service,
        workflow_unit.id,
        "Data processing workflow completed successfully"
    )?;

    // Method 3: Error handling scenario
    let error_unit =
        LogService::create_log_unit(&service, "error-handling-demo".to_string()).await?;

    // Simulate an error scenario
    match simulate_database_operation().await {
        Ok(result) => {
            log_success!(
                service,
                error_unit.id,
                "Database operation successful: {}",
                result
            )?;
        }
        Err(e) => {
            log_error!(service, error_unit.id, "Database operation failed: {}", e)?;
            log_info!(service, error_unit.id, "Attempting retry...")?;
            // Retry logic would go here
        }
    }

    // Retrieve and display logged entries
    println!("\n📊 Retrieving logged entries...");

    let entries = LogService::get_log_entries(&service, workflow_unit.id).await?;
    println!("Found {} entries for workflow unit:", entries.len());

    for entry in &entries {
        let level_emoji = match entry.message_type {
            LogMessageType::Info => "ℹ️",
            LogMessageType::Warning => "⚠️",
            LogMessageType::Error => "❌",
            LogMessageType::Success => "✅",
        };
        println!(
            "  {} [{}] {}",
            level_emoji,
            entry.timestamp.format("%H:%M:%S"),
            entry.message
        );
    }

    // Retrieve log units by external ID
    let units =
        LogService::get_log_units_by_external_id(&service, "data-processing-workflow").await?;
    println!(
        "\nFound {} log units with external_id 'data-processing-workflow'",
        units.len()
    );

    println!("\n✨ MongoDB logging example completed!");
    Ok(())
}

// Simulate a database operation that might fail
async fn simulate_database_operation() -> Result<String, String> {
    // Simulate some async work
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Randomly succeed or fail for demonstration
    if rand::random::<bool>() {
        Ok("Data retrieved successfully".to_string())
    } else {
        Err("Connection timeout".to_string())
    }
}
