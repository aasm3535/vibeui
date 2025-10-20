//! Error handling for VibeUI

use thiserror::Error;

/// Main error type for VibeUI
#[derive(Debug, Error)]
pub enum Error {
    #[error("Application error: {0}")]
    App(#[from] crate::app::Error),
    
    #[error("Platform error: {0}")]
    Platform(#[from] crate::platform::Error),
    
    #[error("Render error: {0}")]
    Render(#[from] crate::render::Error),
    
    #[error("Event error: {0}")]
    Event(#[from] crate::events::Error),
    
    #[error("Style error: {0}")]
    Style(#[from] crate::style::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Component error: {0}")]
    Component(String),
    
    #[error("Initialization error: {0}")]
    Init(String),
    
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Timeout: {0}")]
    Timeout(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type used throughout VibeUI
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create a new configuration error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config(message.into())
    }

    /// Create a new component error
    pub fn component<S: Into<String>>(message: S) -> Self {
        Self::Component(message.into())
    }

    /// Create a new initialization error
    pub fn init<S: Into<String>>(message: S) -> Self {
        Self::Init(message.into())
    }

    /// Create a new not implemented error
    pub fn not_implemented<S: Into<String>>(feature: S) -> Self {
        Self::NotImplemented(feature.into())
    }

    /// Create a new invalid argument error
    pub fn invalid_argument<S: Into<String>>(message: S) -> Self {
        Self::InvalidArgument(message.into())
    }

    /// Create a new not found error
    pub fn not_found<S: Into<String>>(resource: S) -> Self {
        Self::NotFound(resource.into())
    }

    /// Create a new permission denied error
    pub fn permission_denied<S: Into<String>>(message: S) -> Self {
        Self::PermissionDenied(message.into())
    }

    /// Create a new timeout error
    pub fn timeout<S: Into<String>>(message: S) -> Self {
        Self::Timeout(message.into())
    }

    /// Create a new unknown error
    pub fn unknown<S: Into<String>>(message: S) -> Self {
        Self::Unknown(message.into())
    }

    /// Check if this is a recoverable error
    pub fn is_recoverable(&self) -> bool {
        match self {
            Error::Io(_) => true,
            Error::Event(_) => true,
            Error::Render(_) => true,
            Error::Platform(_) => false,
            Error::App(_) => false,
            Error::Style(_) => false,
            Error::Config(_) => false,
            Error::Component(_) => true,
            Error::Init(_) => false,
            Error::NotImplemented(_) => false,
            Error::InvalidArgument(_) => true,
            Error::NotFound(_) => true,
            Error::PermissionDenied(_) => false,
            Error::Timeout(_) => true,
            Error::Unknown(_) => false,
        }
    }

    /// Check if this is a critical error
    pub fn is_critical(&self) -> bool {
        !self.is_recoverable()
    }

    /// Get error category for logging
    pub fn category(&self) -> &'static str {
        match self {
            Error::App(_) => "app",
            Error::Platform(_) => "platform",
            Error::Render(_) => "render",
            Error::Event(_) => "event",
            Error::Style(_) => "style",
            Error::Io(_) => "io",
            Error::Config(_) => "config",
            Error::Component(_) => "component",
            Error::Init(_) => "init",
            Error::NotImplemented(_) => "not_implemented",
            Error::InvalidArgument(_) => "invalid_argument",
            Error::NotFound(_) => "not_found",
            Error::PermissionDenied(_) => "permission_denied",
            Error::Timeout(_) => "timeout",
            Error::Unknown(_) => "unknown",
        }
    }

    /// Get error code for programmatic handling
    pub fn code(&self) -> u32 {
        match self {
            Error::App(_) => 1000,
            Error::Platform(_) => 2000,
            Error::Render(_) => 3000,
            Error::Event(_) => 4000,
            Error::Style(_) => 5000,
            Error::Io(_) => 6000,
            Error::Config(_) => 7000,
            Error::Component(_) => 8000,
            Error::Init(_) => 9000,
            Error::NotImplemented(_) => 10000,
            Error::InvalidArgument(_) => 11000,
            Error::NotFound(_) => 12000,
            Error::PermissionDenied(_) => 13000,
            Error::Timeout(_) => 14000,
            Error::Unknown(_) => 15000,
        }
    }
}

/// Error context for additional information
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Error message
    pub message: String,
    /// File where error occurred
    pub file: Option<String>,
    /// Line number where error occurred
    pub line: Option<u32>,
    /// Function where error occurred
    pub function: Option<String>,
    /// Additional context
    pub context: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    /// Create a new error context
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            file: None,
            line: None,
            function: None,
            context: std::collections::HashMap::new(),
        }
    }

    /// Set the file location
    pub fn file<S: Into<String>>(mut self, file: S) -> Self {
        self.file = Some(file.into());
        self
    }

    /// Set the line number
    pub fn line(mut self, line: u32) -> Self {
        self.line = Some(line);
        self
    }

    /// Set the function name
    pub fn function<S: Into<String>>(mut self, function: S) -> Self {
        self.function = Some(function.into());
        self
    }

    /// Add context information
    pub fn add_context<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.context.insert(key.into(), value.into());
        self
    }

    /// Convert to a string representation
    pub fn to_string(&self) -> String {
        let mut result = self.message.clone();
        
        if let Some(file) = &self.file {
            result.push_str(&format!(" (file: {}", file));
            if let Some(line) = self.line {
                result.push_str(&format!(":{}", line));
            }
            result.push(')');
        }
        
        if let Some(function) = &self.function {
            result.push_str(&format!(" (function: {})", function));
        }
        
        if !self.context.is_empty() {
            result.push_str(" (context: ");
            let context_strings: Vec<String> = self.context
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            result.push_str(&context_strings.join(", "));
            result.push(')');
        }
        
        result
    }
}

/// Macro for creating errors with context
#[macro_export]
macro_rules! error_with_context {
    ($error:expr, $msg:expr) => {
        $crate::error::Error::context(
            $error,
            $crate::error::ErrorContext::new($msg)
        )
    };
    ($error:expr, $msg:expr, file = $file:expr, line = $line:expr, function = $function:expr) => {
        $crate::error::Error::context(
            $error,
            $crate::error::ErrorContext::new($msg)
                .file($file)
                .line($line)
                .function($function)
        )
    };
}

/// Convenience macro for creating errors
#[macro_export]
macro_rules! vibe_error {
    (config, $msg:expr) => {
        $crate::error::Error::config($msg)
    };
    (component, $msg:expr) => {
        $crate::error::Error::component($msg)
    };
    (init, $msg:expr) => {
        $crate::error::Error::init($msg)
    };
    (not_implemented, $feature:expr) => {
        $crate::error::Error::not_implemented($feature)
    };
    (invalid_argument, $msg:expr) => {
        $crate::error::Error::invalid_argument($msg)
    };
    (not_found, $resource:expr) => {
        $crate::error::Error::not_found($resource)
    };
    (permission_denied, $msg:expr) => {
        $crate::error::Error::permission_denied($msg)
    };
    (timeout, $msg:expr) => {
        $crate::error::Error::timeout($msg)
    };
    (unknown, $msg:expr) => {
        $crate::error::Error::unknown($msg)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = Error::config("Invalid configuration");
        assert_eq!(error.category(), "config");
        assert_eq!(error.code(), 7000);
        assert!(!error.is_recoverable());
        assert!(error.is_critical());
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new("Test error")
            .file("test.rs")
            .line(42)
            .function("test_function")
            .add_context("user_id", "123");
        
        let string = context.to_string();
        assert!(string.contains("Test error"));
        assert!(string.contains("file: test.rs:42"));
        assert!(string.contains("function: test_function"));
        assert!(string.contains("user_id=123"));
    }

    #[test]
    fn test_error_macros() {
        let error = vibe_error!(config, "Test config error");
        assert!(matches!(error, Error::Config(_)));
        
        let error = vibe_error!(not_implemented, "test_feature");
        assert!(matches!(error, Error::NotImplemented(_)));
    }

    #[test]
    fn test_error_categories() {
        let io_error = Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
        assert_eq!(io_error.category(), "io");
        assert!(io_error.is_recoverable());
        assert!(!io_error.is_critical());
        
        let platform_error = Error::Platform(crate::platform::Error::Init("test".to_string()));
        assert_eq!(platform_error.category(), "platform");
        assert!(!platform_error.is_recoverable());
        assert!(platform_error.is_critical());
    }
}