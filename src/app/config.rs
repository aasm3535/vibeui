//! Application configuration

use crate::style::Color;
use std::time::Duration;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Terminal title
    pub title: String,
    
    /// Background color
    pub background_color: Color,
    
    /// Foreground color
    pub foreground_color: Color,
    
    /// Frame rate limit (frames per second)
    pub frame_rate: u32,
    
    /// Enable mouse support
    pub mouse_support: bool,
    
    /// Enable raw mode
    pub raw_mode: bool,
    
    /// Tick rate for event handling
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: "VibeUI Application".to_string(),
            background_color: Color::Black,
            foreground_color: Color::White,
            frame_rate: 60,
            mouse_support: true,
            raw_mode: true,
            tick_rate: Duration::from_millis(16),
        }
    }
}

impl Config {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the application title
    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }

    /// Set the background color
    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Set the foreground color
    pub fn with_foreground_color(mut self, color: Color) -> Self {
        self.foreground_color = color;
        self
    }

    /// Set the frame rate
    pub fn with_frame_rate(mut self, frame_rate: u32) -> Self {
        self.frame_rate = frame_rate;
        self
    }

    /// Enable or disable mouse support
    pub fn with_mouse_support(mut self, mouse_support: bool) -> Self {
        self.mouse_support = mouse_support;
        self
    }

    /// Enable or disable raw mode
    pub fn with_raw_mode(mut self, raw_mode: bool) -> Self {
        self.raw_mode = raw_mode;
        self
    }

    /// Set the tick rate
    pub fn with_tick_rate(mut self, tick_rate: Duration) -> Self {
        self.tick_rate = tick_rate;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.title, "VibeUI Application");
        assert_eq!(config.frame_rate, 60);
        assert!(config.mouse_support);
        assert!(config.raw_mode);
    }

    #[test]
    fn test_config_builder() {
        let config = Config::new()
            .with_title("Test App")
            .with_frame_rate(30)
            .with_mouse_support(false);

        assert_eq!(config.title, "Test App");
        assert_eq!(config.frame_rate, 30);
        assert!(!config.mouse_support);
        assert!(config.raw_mode); // Should remain true
    }
}