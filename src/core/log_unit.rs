use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::core::LogService;

/// Represents a log unit that groups related log messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogUnit {
    /// Unique identifier for the log unit
    pub id: Uuid,
    /// External identifier for the log unit
    pub external_id: String,
    /// Timestamp when the log unit was created
    pub timestamp: DateTime<Utc>,
}

impl LogUnit {
    /// Creates a new log unit with the given external ID
    pub fn new(external_id: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            external_id,
            timestamp: Utc::now(),
        }
    }

    /// Creates a new log unit with a generated external ID
    pub fn new_with_generated_id() -> Self {
        Self::new(Uuid::new_v4().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_unit_creation() {
        let external_id = "test-unit".to_string();
        let unit = LogUnit::new(external_id.clone());

        assert_eq!(unit.external_id, external_id);
        assert!(!unit.id.is_nil());
    }

    #[test]
    fn test_log_unit_with_generated_id() {
        let unit = LogUnit::new_with_generated_id();

        assert!(!unit.external_id.is_empty());
        assert!(!unit.id.is_nil());
    }
}