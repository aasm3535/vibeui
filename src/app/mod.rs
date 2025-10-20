//! Application management module
//!
//! This module contains the main application structure and logic for running
//! VibeUI applications.

pub mod app;
pub mod config;

pub use app::App;
pub use config::Config;

/// Application-wide error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Platform error: {0}")]
    Platform(#[from] crate::platform::Error),
    
    #[error("Render error: {0}")]
    Render(#[from] crate::render::Error),
    
    #[error("Event error: {0}")]
    Event(#[from] crate::events::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type used throughout the application
pub type Result<T> = std::result::Result<T, Error>;