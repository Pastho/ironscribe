//! Macros for convenient logging

/// Creates an info log entry and logs it
#[macro_export]
macro_rules! info {
    ($service:expr, $unit:expr, $($arg:tt)*) => {{
        let entry = $crate::LogEntry::info($unit.log_unit_id, format!($($arg)*));
        match $service.log(entry).await {
            Ok(_) => {}
            Err(e) => eprintln!("Failed to log info message: {}", e),
        }
    }};
}

/// Creates a warning log entry and logs it
#[macro_export]
macro_rules! warn {
    ($service:expr, $unit:expr, $($arg:tt)*) => {{
        let entry = $crate::LogEntry::warning($unit.log_unit_id, format!($($arg)*));
        match $service.log(entry).await {
            Ok(_) => {}
            Err(e) => eprintln!("Failed to log warning message: {}", e),
        }
    }};
}

/// Creates an error log entry and logs it
#[macro_export]
macro_rules! error {
    ($service:expr, $unit:expr, $($arg:tt)*) => {{
        let entry = $crate::LogEntry::error($unit.log_unit_id, format!($($arg)*));
        match $service.log(entry).await {
            Ok(_) => {}
            Err(e) => eprintln!("Failed to log error message: {}", e),
        }
    }};
}

/// Creates a success log entry and logs it
#[macro_export]
macro_rules! success {
    ($service:expr, $unit:expr, $($arg:tt)*) => {{
        let entry = $crate::LogEntry::success($unit.log_unit_id, format!($($arg)*));
        match $service.log(entry).await {
            Ok(_) => {}
            Err(e) => eprintln!("Failed to log success message: {}", e),
        }
    }};
}

/// Convenience macro to create a log unit and return its ID
#[macro_export]
macro_rules! create_log_unit {
    ($service:expr, $external_id:expr) => {{
        match $service.create_log_unit($external_id.to_string()).await {
            Ok(unit) => unit,
            Err(e) => {
                eprintln!("Failed to create log unit: {}", e);
                panic!("Log unit creation failed");
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::core::LogService;
    use crate::service::DefaultLogService;

    #[tokio::test]
    #[cfg(feature = "console")]
    async fn test_logging_macros() {
        let service = DefaultLogService::new();
        let unit = create_log_unit!(service, "test");

        info!(service, unit, "This is an info message");
        warn!(service, unit, "This is a warning message");
        error!(service, unit, "This is an error message");
        success!(service, unit, "This is a success message");

        let entries = service.get_log_entries(unit.log_unit_id).await.unwrap();
        assert_eq!(entries.len(), 4);
    }
}
