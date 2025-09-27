//! Pluggable destination implementations for different log targets

#[cfg(feature = "console")]
pub mod console;

#[cfg(feature = "mongo")]
pub mod mongodb;

#[cfg(feature = "postgres")]
pub mod postgres;

// Re-export destination traits and types
#[cfg(feature = "console")]
pub use console::ConsoleDestination;

#[cfg(feature = "mongo")]
pub use mongodb::MongoDestination;

#[cfg(feature = "postgres")]
pub use postgres::PostgresDestination;