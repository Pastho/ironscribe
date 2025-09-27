//! Macros for convenient logging

/// Creates an info log entry and logs it
#[macro_export]
macro_rules! log_info {
    ($service:expr, $unit_id:expr, $($arg:tt)*) => {
        {
            let entry = $crate::LogEntry::info($unit_id, format!($($arg)*));
            $service.log(entry).await
        }
    };
}

/// Creates a warning log entry and logs it
#[macro_export]
macro_rules! log_warn {
    ($service:expr, $unit_id:expr, $($arg:tt)*) => {
        {
            let entry = $crate::LogEntry::warning($unit_id, format!($($arg)*));
            $service.log(entry).await
        }
    };
}

/// Creates an error log entry and logs it
#[macro_export]
macro_rules! log_error {
    ($service:expr, $unit_id:expr, $($arg:tt)*) => {
        {
            let entry = $crate::LogEntry::error($unit_id, format!($($arg)*));
            $service.log(entry).await
        }
    };
}

/// Creates a success log entry and logs it
#[macro_export]
macro_rules! log_success {
    ($service:expr, $unit_id:expr, $($arg:tt)*) => {
        {
            let entry = $crate::LogEntry::success($unit_id, format!($($arg)*));
            $service.log(entry).await
        }
    };
}

/// Convenience macro to create a log unit and return its ID
#[macro_export]
macro_rules! create_log_unit {
    ($service:expr, $external_id:expr) => {
        $service.create_log_unit($external_id.to_string()).await
    };
}

#[cfg(test)]
mod tests {
    use crate::service::DefaultLogService;
    use crate::core::LogService;

    #[tokio::test]
    #[cfg(feature = "console")]
    async fn test_logging_macros() {
        let service = DefaultLogService::new();
        let unit = create_log_unit!(service, "test").unwrap();

        log_info!(service, unit.id, "This is an info message").unwrap();
        log_warn!(service, unit.id, "This is a warning message").unwrap();
        log_error!(service, unit.id, "This is an error message").unwrap();
        log_success!(service, unit.id, "This is a success message").unwrap();

        let entries = service.get_log_entries(unit.id).await.unwrap();
        assert_eq!(entries.len(), 4);
    }
}