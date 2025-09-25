pub mod core;

pub use core::{LogType, LogMessage, LogService};
pub use core::{init_logger, with_logger, log_message};
pub use core::basic_logger::{BasicLogService, init_default_logger};