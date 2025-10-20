//! Platform-specific module
//!
//! This module contains platform-specific code for terminal handling,
//! including initialization, cleanup, and platform-specific optimizations.

pub mod terminal;

pub use terminal::Terminal;

/// Platform-wide error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Terminal initialization error: {0}")]
    Init(String),
    
    #[error("Platform not supported: {0}")]
    Unsupported(String),
    
    #[error("Terminal size detection error: {0}")]
    SizeDetection(String),
}

/// Result type used throughout the platform module
pub type Result<T> = std::result::Result<T, Error>;

/// Initialize the platform
pub fn init() -> Result<()> {
    Terminal::init()
}

/// Cleanup the platform
pub fn cleanup() -> Result<()> {
    Terminal::cleanup()
}

/// Get terminal size
pub fn terminal_size() -> Result<(u16, u16)> {
    Terminal::size()
}

/// Check if the terminal supports colors
pub fn supports_color() -> bool {
    Terminal::supports_color()
}

/// Check if the terminal supports mouse events
pub fn supports_mouse() -> bool {
    Terminal::supports_mouse()
}

/// Enable raw mode
pub fn enable_raw_mode() -> Result<()> {
    Terminal::enable_raw_mode()
}

/// Disable raw mode
pub fn disable_raw_mode() -> Result<()> {
    Terminal::disable_raw_mode()
}

/// Enable mouse capture
pub fn enable_mouse_capture() -> Result<()> {
    Terminal::enable_mouse_capture()
}

/// Disable mouse capture
pub fn disable_mouse_capture() -> Result<()> {
    Terminal::disable_mouse_capture()
}