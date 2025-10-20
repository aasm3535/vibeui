//! # VibeUI
//!
//! A cross-platform terminal UI library for Rust that provides a modern, reactive
//! approach to building terminal applications.
//!
//! ## Features
//!
//! - Cross-platform support (Windows, macOS, Linux)
//! - Reactive event system
//! - Flexible styling system
//! - Component-based architecture
//! - Async support (optional)
//!
//! ## Quick Start
//!
//! ```rust
//! use vibeui::{App, Component, RenderContext};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut app = App::new();
//!     app.run();
//!     Ok(())
//! }
//! ```

// Public modules
pub mod app;
pub mod components;
pub mod error;
pub mod events;
pub mod platform;
pub mod render;
pub mod style;

// Re-export commonly used types
pub use app::App;
pub use components::Component;
pub use error::{Error, Result};
pub use events::{Event, EventHandler};
pub use render::{RenderContext, Renderer};
pub use style::{Style, Color};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the VibeUI library with default configuration
pub fn init() -> Result<(), Error> {
    platform::init()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }
}