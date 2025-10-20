//! Style definitions and utilities

use crate::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Style definition for UI elements
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Style {
    /// Foreground color
    pub foreground: Option<Color>,
    /// Background color
    pub background: Option<Color>,
    /// Bold text
    pub bold: bool,
    /// Italic text
    pub italic: bool,
    /// Underlined text
    pub underline: bool,
    /// Dim text
    pub dim: bool,
    /// Blinking text
    pub blink: bool,
    /// Reverse video
    pub reverse: bool,
    /// Hidden text
    pub hidden: bool,
    /// Strikethrough text
    pub strikethrough: bool,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            foreground: None,
            background: None,
            bold: false,
            italic: false,
            underline: false,
            dim: false,
            blink: false,
            reverse: false,
            hidden: false,
            strikethrough: false,
        }
    }
}

impl Style {
    /// Create a new default style
    pub fn new() -> Self {
        Self::default()
    }

    /// Set foreground color
    pub fn with_foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    /// Set background color
    pub fn with_background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Set bold
    pub fn with_bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Set italic
    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    /// Set underline
    pub fn with_underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    /// Set dim
    pub fn with_dim(mut self, dim: bool) -> Self {
        self.dim = dim;
        self
    }

    /// Set blink
    pub fn with_blink(mut self, blink: bool) -> Self {
        self.blink = blink;
        self
    }

    /// Set reverse
    pub fn with_reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    /// Set hidden
    pub fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Set strikethrough
    pub fn with_strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = strikethrough;
        self
    }

    /// Set both foreground and background colors
    pub fn with_colors(mut self, foreground: Color, background: Color) -> Self {
        self.foreground = Some(foreground);
        self.background = Some(background);
        self
    }

    /// Reset all attributes
    pub fn reset(mut self) -> Self {
        self.foreground = None;
        self.background = None;
        self.bold = false;
        self.italic = false;
        self.underline = false;
        self.dim = false;
        self.blink = false;
        self.reverse = false;
        self.hidden = false;
        self.strikethrough = false;
        self
    }

    /// Check if this style has any attributes set
    pub fn is_empty(&self) -> bool {
        self.foreground.is_none()
            && self.background.is_none()
            && !self.bold
            && !self.italic
            && !self.underline
            && !self.dim
            && !self.blink
            && !self.reverse
            && !self.hidden
            && !self.strikethrough
    }

    /// Apply another style on top of this one
    pub fn apply(&mut self, other: &Style) {
        if other.foreground.is_some() {
            self.foreground = other.foreground;
        }
        if other.background.is_some() {
            self.background = other.background;
        }
        if other.bold {
            self.bold = true;
        }
        if other.italic {
            self.italic = true;
        }
        if other.underline {
            self.underline = true;
        }
        if other.dim {
            self.dim = true;
        }
        if other.blink {
            self.blink = true;
        }
        if other.reverse {
            self.reverse = true;
        }
        if other.hidden {
            self.hidden = true;
        }
        if other.strikethrough {
            self.strikethrough = true;
        }
    }

    /// Create a new style by applying another style on top of this one
    pub fn applied_with(&self, other: &Style) -> Style {
        let mut result = *self;
        result.apply(other);
        result
    }

    /// Invert the colors
    pub fn invert(&mut self) {
        let fg = self.foreground;
        let bg = self.background;
        self.foreground = bg;
        self.background = fg;
    }

    /// Create a new style with inverted colors
    pub fn inverted(&self) -> Style {
        let mut result = *self;
        result.invert();
        result
    }

    /// Make the style dim
    pub fn dim(&mut self) {
        self.dim = true;
    }

    /// Create a new dim style
    pub fn dimmed(&self) -> Style {
        let mut result = *self;
        result.dim = true;
        result
    }

    /// Make the style bright
    pub fn bright(&mut self) {
        self.dim = false;
    }

    /// Create a new bright style
    pub fn brightened(&self) -> Style {
        let mut result = *self;
        result.bright();
        result
    }

    /// Get the effective foreground color (considering reverse)
    pub fn effective_foreground(&self) -> Option<Color> {
        if self.reverse {
            self.background
        } else {
            self.foreground
        }
    }

    /// Get the effective background color (considering reverse)
    pub fn effective_background(&self) -> Option<Color> {
        if self.reverse {
            self.foreground
        } else {
            self.background
        }
    }

    /// Parse a style from a string
    pub fn parse(s: &str) -> Result<Self, crate::style::Error> {
        let mut style = Style::default();
        let parts = s.split_whitespace();

        for part in parts {
            match part.to_lowercase().as_str() {
                "bold" => style.bold = true,
                "italic" => style.italic = true,
                "underline" => style.underline = true,
                "dim" => style.dim = true,
                "blink" => style.blink = true,
                "reverse" => style.reverse = true,
                "hidden" => style.hidden = true,
                "strikethrough" => style.strikethrough = true,
                _ => {
                    // Try to parse as color
                    if let Some(color_part) = part.strip_prefix("fg=") {
                        style.foreground = Some(Color::parse(color_part)?);
                    } else if let Some(color_part) = part.strip_prefix("bg=") {
                        style.background = Some(Color::parse(color_part)?);
                    } else {
                        // Try to parse as a color (default to foreground)
                        style.foreground = Some(Color::parse(part)?);
                    }
                }
            }
        }

        Ok(style)
    }

    /// Convert to a string representation
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();

        if let Some(fg) = self.foreground {
            parts.push(format!("fg={}", fg));
        }
        if let Some(bg) = self.background {
            parts.push(format!("bg={}", bg));
        }
        if self.bold {
            parts.push("bold".to_string());
        }
        if self.italic {
            parts.push("italic".to_string());
        }
        if self.underline {
            parts.push("underline".to_string());
        }
        if self.dim {
            parts.push("dim".to_string());
        }
        if self.blink {
            parts.push("blink".to_string());
        }
        if self.reverse {
            parts.push("reverse".to_string());
        }
        if self.hidden {
            parts.push("hidden".to_string());
        }
        if self.strikethrough {
            parts.push("strikethrough".to_string());
        }

        parts.join(" ")
    }
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Style builder for constructing styles step by step
pub struct StyleBuilder {
    style: Style,
}

impl StyleBuilder {
    /// Create a new style builder
    pub fn new() -> Self {
        Self {
            style: Style::default(),
        }
    }

    /// Set foreground color
    pub fn foreground(mut self, color: Color) -> Self {
        self.style.foreground = Some(color);
        self
    }

    /// Set background color
    pub fn background(mut self, color: Color) -> Self {
        self.style.background = Some(color);
        self
    }

    /// Set bold
    pub fn bold(mut self, bold: bool) -> Self {
        self.style.bold = bold;
        self
    }

    /// Set italic
    pub fn italic(mut self, italic: bool) -> Self {
        self.style.italic = italic;
        self
    }

    /// Set underline
    pub fn underline(mut self, underline: bool) -> Self {
        self.style.underline = underline;
        self
    }

    /// Set dim
    pub fn dim(mut self, dim: bool) -> Self {
        self.style.dim = dim;
        self
    }

    /// Set blink
    pub fn blink(mut self, blink: bool) -> Self {
        self.style.blink = blink;
        self
    }

    /// Set reverse
    pub fn reverse(mut self, reverse: bool) -> Self {
        self.style.reverse = reverse;
        self
    }

    /// Set hidden
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.style.hidden = hidden;
        self
    }

    /// Set strikethrough
    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.style.strikethrough = strikethrough;
        self
    }

    /// Build the style
    pub fn build(self) -> Style {
        self.style
    }
}

impl Default for StyleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Predefined styles
pub mod presets {
    use super::{Style, Color, palette};

    /// Default style
    pub const DEFAULT: Style = Style {
        foreground: None,
        background: None,
        bold: false,
        italic: false,
        underline: false,
        dim: false,
        blink: false,
        reverse: false,
        hidden: false,
        strikethrough: false,
    };

    /// Normal text style
    pub const NORMAL: Style = Style {
        foreground: Some(palette::WHITE),
        background: Some(palette::BLACK),
        bold: false,
        italic: false,
        underline: false,
        dim: false,
        blink: false,
        reverse: false,
        hidden: false,
        strikethrough: false,
    };

    /// Header style
    pub const HEADER: Style = Style {
        foreground: Some(palette::BRIGHT_WHITE),
        background: Some(palette::BLUE),
        bold: true,
        italic: false,
        underline: false,
        dim: false,
        blink: false,
        reverse: false,
        hidden: false,
        strikethrough: false,
    };

    /// Warning style
    pub const WARNING: Style = Style {
        foreground: Some(palette::BLACK),
        background: Some(palette::YELLOW),
        bold: true,
        italic: false,
        underline: false,
        dim: false,
        blink: false,
        reverse: false,
        hidden: false,
        strikethrough: false,
    };

    /// Error style
    pub const ERROR: Style = Style {
        foreground: Some(palette::WHITE),
        background: Some(palette::RED),
        bold: true,
        italic: false,
        underline: false,
        dim: false,
        blink: false,
        reverse: false,
        hidden: false,
        strikethrough: false,
    };

    /// Success style
    pub const SUCCESS: Style = Style {
        foreground: Some(palette::BLACK),
        background: Some(palette::GREEN),
        bold: true,
        italic: false,
        underline: false,
        dim: false,
        blink: false,
        reverse: false,
        hidden: false,
        strikethrough: false,
    };

    /// Info style
    pub const INFO: Style = Style {
        foreground: Some(palette::BLACK),
        background: Some(palette::CYAN),
        bold: false,
        italic: false,
        underline: false,
        dim: false,
        blink: false,
        reverse: false,
        hidden: false,
        strikethrough: false,
    };

    /// Highlight style
    pub const HIGHLIGHT: Style = Style {
        foreground: Some(palette::BLACK),
        background: Some(palette::BRIGHT_YELLOW),
        bold: true,
        italic: false,
        underline: false,
        dim: false,
        blink: false,
        reverse: false,
        hidden: false,
        strikethrough: false,
    };

    /// Dim style
    pub const DIM: Style = Style {
        foreground: Some(palette::GRAY),
        background: None,
        bold: false,
        italic: false,
        underline: false,
        dim: true,
        blink: false,
        reverse: false,
        hidden: false,
        strikethrough: false,
    };

    /// Inverse style
    pub const INVERSE: Style = Style {
        foreground: Some(palette::BLACK),
        background: Some(palette::WHITE),
        bold: false,
        italic: false,
        underline: false,
        dim: false,
        blink: false,
        reverse: false,
        hidden: false,
        strikethrough: false,
    };
}

/// Style registry for managing named styles
#[derive(Debug, Clone, Default)]
pub struct StyleRegistry {
    styles: HashMap<String, Style>,
}

impl StyleRegistry {
    /// Create a new style registry
    pub fn new() -> Self {
        let mut registry = Self {
            styles: HashMap::new(),
        };
        
        // Register preset styles
        registry.register("default", presets::DEFAULT);
        registry.register("normal", presets::NORMAL);
        registry.register("header", presets::HEADER);
        registry.register("warning", presets::WARNING);
        registry.register("error", presets::ERROR);
        registry.register("success", presets::SUCCESS);
        registry.register("info", presets::INFO);
        registry.register("highlight", presets::HIGHLIGHT);
        registry.register("dim", presets::DIM);
        registry.register("inverse", presets::INVERSE);
        
        registry
    }

    /// Register a style with a name
    pub fn register(&mut self, name: &str, style: Style) {
        self.styles.insert(name.to_string(), style);
    }

    /// Get a style by name
    pub fn get(&self, name: &str) -> Option<&Style> {
        self.styles.get(name)
    }

    /// Remove a style by name
    pub fn remove(&mut self, name: &str) -> Option<Style> {
        self.styles.remove(name)
    }

    /// Get all style names
    pub fn names(&self) -> Vec<&str> {
        self.styles.keys().map(|s| s.as_str()).collect()
    }

    /// Clear all styles
    pub fn clear(&mut self) {
        self.styles.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_creation() {
        let style = Style::new()
            .with_foreground(Color::Red)
            .with_background(Color::Blue)
            .with_bold(true);
        
        assert_eq!(style.foreground, Some(Color::Red));
        assert_eq!(style.background, Some(Color::Blue));
        assert!(style.bold);
        assert!(!style.italic);
    }

    #[test]
    fn test_style_builder() {
        let style = StyleBuilder::new()
            .foreground(Color::Green)
            .bold(true)
            .underline(true)
            .build();
        
        assert_eq!(style.foreground, Some(Color::Green));
        assert!(style.bold);
        assert!(style.underline);
    }

    #[test]
    fn test_style_apply() {
        let mut base = Style::new().with_foreground(Color::Red);
        let overlay = Style::new().with_bold(true).with_background(Color::Blue);
        
        base.apply(&overlay);
        
        assert_eq!(base.foreground, Some(Color::Red));
        assert_eq!(base.background, Some(Color::Blue));
        assert!(base.bold);
    }

    #[test]
    fn test_style_invert() {
        let mut style = Style::new()
            .with_foreground(Color::Red)
            .with_background(Color::Blue);
        
        style.invert();
        
        assert_eq!(style.foreground, Some(Color::Blue));
        assert_eq!(style.background, Some(Color::Red));
    }

    #[test]
    fn test_style_parse() {
        let style = Style::parse("fg=red bg=blue bold underline").unwrap();
        
        assert_eq!(style.foreground, Some(Color::Red));
        assert_eq!(style.background, Some(Color::Blue));
        assert!(style.bold);
        assert!(style.underline);
    }

    #[test]
    fn test_style_registry() {
        let mut registry = StyleRegistry::new();
        
        registry.register("custom", Style::new().with_foreground(Color::Purple));
        
        assert!(registry.get("header").is_some());
        assert!(registry.get("custom").is_some());
        assert!(registry.get("nonexistent").is_none());
        
        let names = registry.names();
        assert!(names.contains(&"header"));
        assert!(names.contains(&"custom"));
    }
}