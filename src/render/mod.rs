//! Rendering module
//!
//! This module contains the rendering system for VibeUI applications,
//! including the renderer interface and rendering utilities.

pub mod renderer;
pub mod context;
pub mod buffer;

pub use renderer::Renderer;
pub use context::RenderContext;
pub use buffer::RenderBuffer;

/// Render-wide error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Rendering error: {0}")]
    Render(String),
    
    #[error("Buffer error: {0}")]
    Buffer(String),
    
    #[error("Terminal error: {0}")]
    Terminal(#[from] crate::platform::Error),
}

/// Result type used throughout the render system
pub type Result<T> = std::result::Result<T, Error>;