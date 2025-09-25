use ironscribe::{init_default_logger, with_logger, info, warn, error};

#[test]
fn test_global_logger_initialization() {
    // Initialize the global logger
    let result = init_default_logger();
    assert!(result.is_ok());
}

#[test]
fn test_global_logging_macros() {
    // Initialize logger (this will fail if already initialized in another test)
    let _ = init_default_logger();

    // Use the simplified macros
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");

    // Verify logs were added
    with_logger(|logger| {
        // We can't directly access logs count in the trait,
        // so we'll just verify the logger is accessible
        logger.print_logs();
    });
}

#[test]
fn test_formatted_global_logging() {
    let _ = init_default_logger();

    let user_id = 42;
    let username = "alice";

    info!("User {} ({}) logged in", user_id, username);
    warn!("User {} has {} failed login attempts", username, 3);
    error!("Failed to authenticate user {}", user_id);

    with_logger(|logger| {
        logger.print_logs();
    });
}

#[test]
fn test_with_logger_function() {
    let _ = init_default_logger();

    // Add some logs using the global macros
    info!("Test message 1");
    warn!("Test message 2");

    // Access the logger directly for custom operations
    with_logger(|logger| {
        logger.save_log();
        logger.flush_log();
    });

    // Verify logs were flushed
    info!("Message after flush");

    with_logger(|logger| {
        logger.print_logs();
    });
}