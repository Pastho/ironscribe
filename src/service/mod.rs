//! Default log service implementation

pub mod default;

pub use default::DefaultLogService;

/// Type alias for the default log service
pub type LogService = DefaultLogService;