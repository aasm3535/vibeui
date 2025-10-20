//! Styling module
//!
//! This module contains the styling system for VibeUI applications,
//! including colors, styles, and styling utilities.

pub mod color;
pub mod style;
pub mod theme;

pub use color::Color;
pub use style::Style;
pub use theme::Theme;

/// Style-wide error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Color parsing error: {0}")]
    ColorParse(String),
    
    #[error("Invalid style: {0}")]
    InvalidStyle(String),
    
    #[error("Theme error: {0}")]
    Theme(String),
}

/// Result type used throughout the style system
pub type Result<T> = std::result::Result<T, Error>;