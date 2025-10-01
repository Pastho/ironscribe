use ironscribe::{
    DefaultLogService, LogEntry, LogLevel, LogMessageType, core::LogService, create_log_unit,
    destinations::ConsoleDestination, error, info, success, warn,
};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_console_destination_basic_functionality() {
    let console = ConsoleDestination::new();

    // Test creating a log unit (using the trait method)
    let unit = LogService::create_log_unit(&console, "integration-test".to_string())
        .await
        .unwrap();

    assert_eq!(unit.external_id, "integration-test");
    assert!(!unit.id.is_nil());

    // Test retrieving the created log unit
    let retrieved_unit = LogService::get_log_unit(&console, unit.id).await.unwrap();
    assert!(retrieved_unit.is_some());
    assert_eq!(retrieved_unit.unwrap().external_id, "integration-test");
}

#[tokio::test]
async fn test_console_logging_all_message_types() {
    let console = ConsoleDestination::new();
    let unit = LogService::create_log_unit(&console, "message-types-test".to_string())
        .await
        .unwrap();

    // Test all message types
    let error_entry = LogEntry::error(unit.id, "This is an error message".to_string());
    let warning_entry = LogEntry::warning(unit.id, "This is a warning message".to_string());
    let info_entry = LogEntry::info(unit.id, "This is an info message".to_string());
    let success_entry = LogEntry::success(unit.id, "This is a success message".to_string());

    // Log all entries
    LogService::log(&console, error_entry.clone())
        .await
        .unwrap();
    LogService::log(&console, warning_entry.clone())
        .await
        .unwrap();
    LogService::log(&console, info_entry.clone()).await.unwrap();
    LogService::log(&console, success_entry.clone())
        .await
        .unwrap();

    // Retrieve and verify all entries
    let entries = LogService::get_log_entries(&console, unit.id)
        .await
        .unwrap();
    assert_eq!(entries.len(), 4);

    // Verify message types and content
    assert_eq!(entries[0].message_type, LogMessageType::Error);
    assert_eq!(entries[0].message, "This is an error message");
    assert_eq!(entries[1].message_type, LogMessageType::Warning);
    assert_eq!(entries[1].message, "This is a warning message");
    assert_eq!(entries[2].message_type, LogMessageType::Info);
    assert_eq!(entries[2].message, "This is an info message");
    assert_eq!(entries[3].message_type, LogMessageType::Success);
    assert_eq!(entries[3].message, "This is a success message");
}

#[tokio::test]
async fn test_console_log_levels_ordering() {
    let console = ConsoleDestination::new();
    let unit = LogService::create_log_unit(&console, "log-levels-test".to_string())
        .await
        .unwrap();

    // Create entries with different levels
    let error_entry = LogEntry::error(unit.id, "Error".to_string());
    let warning_entry = LogEntry::warning(unit.id, "Warning".to_string());
    let info_entry = LogEntry::info(unit.id, "Info".to_string());
    let success_entry = LogEntry::success(unit.id, "Success".to_string());

    // Verify log level ordering
    assert!(error_entry.level < warning_entry.level);
    assert!(warning_entry.level < info_entry.level);
    assert!(info_entry.level < success_entry.level);

    assert_eq!(error_entry.level, LogLevel::Error);
    assert_eq!(warning_entry.level, LogLevel::Warning);
    assert_eq!(info_entry.level, LogLevel::Info);
    assert_eq!(success_entry.level, LogLevel::Success);
}

#[tokio::test]
async fn test_console_multiple_log_units() {
    let console = ConsoleDestination::new();

    // Create multiple log units
    let unit1 = LogService::create_log_unit(&console, "unit-1".to_string())
        .await
        .unwrap();
    let unit2 = LogService::create_log_unit(&console, "unit-2".to_string())
        .await
        .unwrap();
    let unit3 = LogService::create_log_unit(&console, "unit-1".to_string())
        .await
        .unwrap(); // Same external_id as unit1

    // Log entries to different units
    LogService::log(
        &console,
        LogEntry::info(unit1.id, "Message for unit 1".to_string()),
    )
    .await
    .unwrap();
    LogService::log(
        &console,
        LogEntry::error(unit2.id, "Error for unit 2".to_string()),
    )
    .await
    .unwrap();
    LogService::log(
        &console,
        LogEntry::success(unit3.id, "Success for unit 3".to_string()),
    )
    .await
    .unwrap();

    // Verify entries are stored separately
    let entries1 = LogService::get_log_entries(&console, unit1.id)
        .await
        .unwrap();
    let entries2 = LogService::get_log_entries(&console, unit2.id)
        .await
        .unwrap();
    let entries3 = LogService::get_log_entries(&console, unit3.id)
        .await
        .unwrap();

    assert_eq!(entries1.len(), 1);
    assert_eq!(entries2.len(), 1);
    assert_eq!(entries3.len(), 1);

    // Test retrieving by external_id
    let units_by_external_id = LogService::get_log_units_by_external_id(&console, "unit-1")
        .await
        .unwrap();
    assert_eq!(units_by_external_id.len(), 2); // unit1 and unit3
}

#[tokio::test]
async fn test_default_log_service_with_console() {
    let service = DefaultLogService::new();

    // Create log unit using the service
    let unit = LogService::create_log_unit(&service, "default-service-test".to_string())
        .await
        .unwrap();

    // Log various messages
    LogService::log(
        &service,
        LogEntry::info(unit.id, "Service info message".to_string()),
    )
    .await
    .unwrap();
    LogService::log(
        &service,
        LogEntry::warning(unit.id, "Service warning message".to_string()),
    )
    .await
    .unwrap();
    LogService::log(
        &service,
        LogEntry::error(unit.id, "Service error message".to_string()),
    )
    .await
    .unwrap();
    LogService::log(
        &service,
        LogEntry::success(unit.id, "Service success message".to_string()),
    )
    .await
    .unwrap();

    // Retrieve and verify
    let entries = LogService::get_log_entries(&service, unit.id)
        .await
        .unwrap();
    assert_eq!(entries.len(), 4);

    // Verify the service maintains the correct order
    assert_eq!(entries[0].message, "Service info message");
    assert_eq!(entries[1].message, "Service warning message");
    assert_eq!(entries[2].message, "Service error message");
    assert_eq!(entries[3].message, "Service success message");
}

#[tokio::test]
async fn test_logging_macros_integration() {
    let service = DefaultLogService::new();
    let unit = create_log_unit!(service, "macro-test");

    // Test all logging macros
    info!(service, unit, "Info message from macro");

    warn!(service, unit, "Warning message from macro");
    error!(service, unit, "Error message from macro");
    success!(service, unit, "Success message from macro");

    // Test formatted messages
    let count = 42;
    info!(service, unit, "Formatted message with count: {}", count);

    // Verify all entries
    let entries = LogService::get_log_entries(&service, unit.id)
        .await
        .unwrap();
    assert_eq!(entries.len(), 5);

    assert_eq!(entries[0].message, "Info message from macro");
    assert_eq!(entries[1].message, "Warning message from macro");
    assert_eq!(entries[2].message, "Error message from macro");
    assert_eq!(entries[3].message, "Success message from macro");
    assert_eq!(entries[4].message, "Formatted message with count: 42");
}

#[tokio::test]
async fn test_console_concurrent_logging() {
    let console = Arc::new(ConsoleDestination::new());
    let unit = LogService::create_log_unit(&*console, "concurrent-test".to_string())
        .await
        .unwrap();

    // Create multiple concurrent logging tasks
    let mut handles = vec![];

    for i in 0..10 {
        let console_clone = Arc::clone(&console);
        let unit_id = unit.id;

        let handle = tokio::spawn(async move {
            let entry = LogEntry::info(unit_id, format!("Concurrent message {}", i));
            LogService::log(&*console_clone, entry).await.unwrap();
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all messages were logged
    let entries = LogService::get_log_entries(&*console, unit.id)
        .await
        .unwrap();
    assert_eq!(entries.len(), 10);

    // Verify all messages are present (order may vary due to concurrency)
    let messages: std::collections::HashSet<String> =
        entries.iter().map(|e| e.message.clone()).collect();

    for i in 0..10 {
        assert!(messages.contains(&format!("Concurrent message {}", i)));
    }
}

#[tokio::test]
async fn test_console_log_unit_timestamps() {
    let console = ConsoleDestination::new();

    let unit1 = LogService::create_log_unit(&console, "timestamp-test-1".to_string())
        .await
        .unwrap();

    // Small delay to ensure different timestamps
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    let unit2 = LogService::create_log_unit(&console, "timestamp-test-2".to_string())
        .await
        .unwrap();

    // Verify timestamps are different and in order
    assert!(unit1.timestamp < unit2.timestamp);

    // Log entries and verify their timestamps
    let entry1 = LogEntry::info(unit1.id, "First message".to_string());
    let entry2 = LogEntry::info(unit2.id, "Second message".to_string());

    LogService::log(&console, entry1.clone()).await.unwrap();

    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    LogService::log(&console, entry2.clone()).await.unwrap();

    // Retrieve entries and verify timestamps
    let entries1 = LogService::get_log_entries(&console, unit1.id)
        .await
        .unwrap();
    let entries2 = LogService::get_log_entries(&console, unit2.id)
        .await
        .unwrap();

    assert_eq!(entries1.len(), 1);
    assert_eq!(entries2.len(), 1);

    // Entry timestamps should be after their respective unit timestamps
    assert!(entries1[0].timestamp >= unit1.timestamp);
    assert!(entries2[0].timestamp >= unit2.timestamp);
}

#[tokio::test]
async fn test_console_empty_log_units() {
    let console = ConsoleDestination::new();
    let unit = LogService::create_log_unit(&console, "empty-test".to_string())
        .await
        .unwrap();

    // Test retrieving entries from empty log unit
    let entries = LogService::get_log_entries(&console, unit.id)
        .await
        .unwrap();
    assert_eq!(entries.len(), 0);

    // Test retrieving non-existent log unit
    let non_existent_id = Uuid::new_v4();
    let non_existent_unit = LogService::get_log_unit(&console, non_existent_id)
        .await
        .unwrap();
    assert!(non_existent_unit.is_none());

    // Test retrieving entries for non-existent log unit
    let non_existent_entries = LogService::get_log_entries(&console, non_existent_id)
        .await
        .unwrap();
    assert_eq!(non_existent_entries.len(), 0);
}

#[tokio::test]
async fn test_console_log_entry_structure() {
    let console = ConsoleDestination::new();
    let unit = LogService::create_log_unit(&console, "structure-test".to_string())
        .await
        .unwrap();

    let message = "Test message structure";
    let entry = LogEntry::info(unit.id, message.to_string());

    // Verify entry structure before logging
    assert_eq!(entry.log_unit_id, unit.id);
    assert!(!entry.message_id.is_nil());
    assert_eq!(entry.level, LogLevel::Info);
    assert_eq!(entry.message, message);
    assert_eq!(entry.message_type, LogMessageType::Info);

    LogService::log(&console, entry.clone()).await.unwrap();

    // Retrieve and verify structure is preserved
    let entries = LogService::get_log_entries(&console, unit.id)
        .await
        .unwrap();
    let retrieved_entry = &entries[0];

    assert_eq!(retrieved_entry.log_unit_id, entry.log_unit_id);
    assert_eq!(retrieved_entry.message_id, entry.message_id);
    assert_eq!(retrieved_entry.level, entry.level);
    assert_eq!(retrieved_entry.message, entry.message);
    assert_eq!(retrieved_entry.message_type, entry.message_type);
    assert_eq!(retrieved_entry.timestamp, entry.timestamp);
}

#[tokio::test]
async fn test_console_destination_direct_usage() {
    // Test using ConsoleDestination as a trait object
    let console: Box<dyn LogService> = Box::new(ConsoleDestination::new());

    let unit = console
        .create_log_unit("trait-object-test".to_string())
        .await
        .unwrap();
    console
        .log(LogEntry::info(
            unit.id,
            "Message via trait object".to_string(),
        ))
        .await
        .unwrap();

    let entries = console.get_log_entries(unit.id).await.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].message, "Message via trait object");
}
