//! Event handling module
//!
//! This module contains all event types and the event handling system
//! for VibeUI applications.

pub mod event;
pub mod handler;
pub mod key;

pub use event::Event;
pub use handler::EventHandler;
pub use key::Key;

/// Event-wide error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Event parsing error: {0}")]
    Parse(String),
    
    #[error("Event channel error: {0}")]
    Channel(String),
}

/// Result type used throughout the event system
pub type Result<T> = std::result::Result<T, Error>;