use crate::{init_logger, LogMessage, LogService};

pub struct BasicLogService {
    pub logs: Vec<LogMessage>,
}

impl BasicLogService {
    pub fn new() -> Self {
        Self { logs: Vec::new() }
    }
}

impl LogService for BasicLogService {
    fn add_message(&mut self, log: LogMessage) {
        self.logs.push(log);
    }

    fn flush_log(&mut self) {
        self.logs.clear();
    }

    fn save_log(&mut self) {
        println!("Saving logs...");
    }

    fn print_logs(&mut self) {
        for log in &self.logs {
            println!("Log ID: {}, Type: {:?}, Message: {}, Timestamp: {}",
                     log.id, log.log_type, log.message, log.timestamp);
        }
    }
}

/// Initialize the global logger with the default BasicLogService
pub fn init_default_logger() -> Result<(), &'static str> {
    init_logger(BasicLogService::new())
}