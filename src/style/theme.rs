//! Theme system for VibeUI

use crate::style::{Style, Color, StyleRegistry};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Theme definition containing named styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Theme description
    pub description: Option<String>,
    /// Theme version
    pub version: String,
    /// Theme author
    pub author: Option<String>,
    /// Named styles
    pub styles: HashMap<String, Style>,
    /// Color palette
    pub palette: ColorPalette,
}

/// Color palette for themes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Primary color
    pub primary: Color,
    /// Secondary color
    pub secondary: Color,
    /// Accent color
    pub accent: Color,
    /// Background color
    pub background: Color,
    /// Surface color (for panels, cards, etc.)
    pub surface: Color,
    /// Text color
    pub text: Color,
    /// Text color on primary background
    pub on_primary: Color,
    /// Text color on secondary background
    pub on_secondary: Color,
    /// Text color on accent background
    pub on_accent: Color,
    /// Text color on surface
    pub on_surface: Color,
    /// Error color
    pub error: Color,
    /// Warning color
    pub warning: Color,
    /// Success color
    pub success: Color,
    /// Info color
    pub info: Color,
    /// Border color
    pub border: Color,
    /// Disabled color
    pub disabled: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary: Color::Blue,
            secondary: Color::Gray,
            accent: Color::Cyan,
            background: Color::Black,
            surface: Color::Rgb(32, 32, 32),
            text: Color::White,
            on_primary: Color::White,
            on_secondary: Color::White,
            on_accent: Color::Black,
            on_surface: Color::White,
            error: Color::Red,
            warning: Color::Yellow,
            success: Color::Green,
            info: Color::Cyan,
            border: Color::Gray,
            disabled: Color::Rgb(128, 128, 128),
        }
    }
}

impl Theme {
    /// Create a new theme
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            version: "1.0.0".to_string(),
            author: None,
            styles: HashMap::new(),
            palette: ColorPalette::default(),
        }
    }

    /// Create a new theme with a name and description
    pub fn with_description(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: Some(description.to_string()),
            version: "1.0.0".to_string(),
            author: None,
            styles: HashMap::new(),
            palette: ColorPalette::default(),
        }
    }

    /// Set the theme version
    pub fn with_version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    /// Set the theme author
    pub fn with_author(mut self, author: &str) -> Self {
        self.author = Some(author.to_string());
        self
    }

    /// Set the color palette
    pub fn with_palette(mut self, palette: ColorPalette) -> Self {
        self.palette = palette;
        self
    }

    /// Add a style to the theme
    pub fn add_style(mut self, name: &str, style: Style) -> Self {
        self.styles.insert(name.to_string(), style);
        self
    }

    /// Get a style by name
    pub fn get_style(&self, name: &str) -> Option<&Style> {
        self.styles.get(name)
    }

    /// Remove a style by name
    pub fn remove_style(&mut self, name: &str) -> Option<Style> {
        self.styles.remove(name)
    }

    /// Get all style names
    pub fn style_names(&self) -> Vec<&str> {
        self.styles.keys().map(|s| s.as_str()).collect()
    }

    /// Create a style registry from this theme
    pub fn create_registry(&self) -> StyleRegistry {
        let mut registry = StyleRegistry::new();
        for (name, style) in &self.styles {
            registry.register(name, *style);
        }
        registry
    }

    /// Generate default styles based on the color palette
    pub fn generate_default_styles(&mut self) {
        let p = &self.palette;
        
        // Basic text styles
        self.styles.insert("default".to_string(), Style::new()
            .with_foreground(p.text)
            .with_background(p.background));
        
        self.styles.insert("primary".to_string(), Style::new()
            .with_foreground(p.on_primary)
            .with_background(p.primary));
        
        self.styles.insert("secondary".to_string(), Style::new()
            .with_foreground(p.on_secondary)
            .with_background(p.secondary));
        
        self.styles.insert("accent".to_string(), Style::new()
            .with_foreground(p.on_accent)
            .with_background(p.accent));
        
        self.styles.insert("surface".to_string(), Style::new()
            .with_foreground(p.on_surface)
            .with_background(p.surface));
        
        // Status styles
        self.styles.insert("error".to_string(), Style::new()
            .with_foreground(Color::White)
            .with_background(p.error)
            .with_bold(true));
        
        self.styles.insert("warning".to_string(), Style::new()
            .with_foreground(Color::Black)
            .with_background(p.warning)
            .with_bold(true));
        
        self.styles.insert("success".to_string(), Style::new()
            .with_foreground(Color::Black)
            .with_background(p.success)
            .with_bold(true));
        
        self.styles.insert("info".to_string(), Style::new()
            .with_foreground(Color::Black)
            .with_background(p.info));
        
        // Interactive styles
        self.styles.insert("button".to_string(), Style::new()
            .with_foreground(p.on_primary)
            .with_background(p.primary));
        
        self.styles.insert("button_hover".to_string(), Style::new()
            .with_foreground(p.on_primary)
            .with_background(p.primary.blend(&p.accent, 0.3)));
        
        self.styles.insert("button_active".to_string(), Style::new()
            .with_foreground(p.on_primary)
            .with_background(p.primary.blend(&p.accent, 0.5)));
        
        self.styles.insert("input".to_string(), Style::new()
            .with_foreground(p.text)
            .with_background(p.surface)
            .with_underline(true));
        
        self.styles.insert("input_focused".to_string(), Style::new()
            .with_foreground(p.text)
            .with_background(p.surface)
            .with_underline(true)
            .with_bold(true));
        
        // Border styles
        self.styles.insert("border".to_string(), Style::new()
            .with_foreground(p.border));
        
        self.styles.insert("border_focus".to_string(), Style::new()
            .with_foreground(p.accent));
        
        // Text styles
        self.styles.insert("header".to_string(), Style::new()
            .with_foreground(p.primary)
            .with_bold(true));
        
        self.styles.insert("title".to_string(), Style::new()
            .with_foreground(p.text)
            .with_bold(true));
        
        self.styles.insert("subtitle".to_string(), Style::new()
            .with_foreground(p.text)
            .with_italic(true));
        
        self.styles.insert("body".to_string(), Style::new()
            .with_foreground(p.text));
        
        self.styles.insert("caption".to_string(), Style::new()
            .with_foreground(p.disabled)
            .with_dim(true));
        
        // Selection styles
        self.styles.insert("selected".to_string(), Style::new()
            .with_foreground(p.on_primary)
            .with_background(p.primary));
        
        self.styles.insert("highlight".to_string(), Style::new()
            .with_foreground(p.on_accent)
            .with_background(p.accent));
    }

    /// Load a theme from a JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Convert the theme to a JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Load a theme from a TOML string
    pub fn from_toml(toml: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml)
    }

    /// Convert the theme to a TOML string
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    /// Save the theme to a file
    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_json()?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load a theme from a file
    pub fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let theme = Self::from_json(&content)?;
        Ok(theme)
    }
}

impl Default for Theme {
    fn default() -> Self {
        let mut theme = Self::new("default");
        theme.generate_default_styles();
        theme
    }
}

/// Theme manager for managing multiple themes
#[derive(Debug, Clone, Default)]
pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    current_theme: Option<String>,
}

impl ThemeManager {
    /// Create a new theme manager
    pub fn new() -> Self {
        let mut manager = Self {
            themes: HashMap::new(),
            current_theme: None,
        };
        
        // Add default theme
        let default_theme = Theme::default();
        manager.add_theme(default_theme);
        manager.set_current_theme("default");
        
        manager
    }

    /// Add a theme
    pub fn add_theme(&mut self, theme: Theme) {
        let name = theme.name.clone();
        self.themes.insert(name, theme);
    }

    /// Get a theme by name
    pub fn get_theme(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }

    /// Remove a theme by name
    pub fn remove_theme(&mut self, name: &str) -> Option<Theme> {
        self.themes.remove(name)
    }

    /// Get the current theme
    pub fn current_theme(&self) -> Option<&Theme> {
        self.current_theme.as_ref().and_then(|name| self.themes.get(name))
    }

    /// Set the current theme
    pub fn set_current_theme(&mut self, name: &str) -> bool {
        if self.themes.contains_key(name) {
            self.current_theme = Some(name.to_string());
            true
        } else {
            false
        }
    }

    /// Get all theme names
    pub fn theme_names(&self) -> Vec<&str> {
        self.themes.keys().map(|s| s.as_str()).collect()
    }

    /// Get the current theme name
    pub fn current_theme_name(&self) -> Option<&str> {
        self.current_theme.as_deref()
    }

    /// Get a style from the current theme
    pub fn get_style(&self, name: &str) -> Option<&Style> {
        self.current_theme().and_then(|theme| theme.get_style(name))
    }

    /// Get the color palette from the current theme
    pub fn get_palette(&self) -> Option<&ColorPalette> {
        self.current_theme().map(|theme| &theme.palette)
    }
}

/// Predefined themes
pub mod builtin {
    use super::*;
    use crate::style::palette;

    /// Dark theme
    pub fn dark() -> Theme {
        let mut theme = Theme::with_description("dark", "Dark theme for VibeUI")
            .with_author("VibeUI")
            .with_version("1.0.0");
        
        theme.palette = ColorPalette {
            primary: Color::Rgb(66, 133, 244),
            secondary: Color::Rgb(117, 117, 117),
            accent: Color::Rgb(52, 168, 83),
            background: Color::Rgb(24, 24, 24),
            surface: Color::Rgb(41, 41, 41),
            text: Color::Rgb(241, 241, 241),
            on_primary: Color::White,
            on_secondary: Color::White,
            on_accent: Color::White,
            on_surface: Color::Rgb(241, 241, 241),
            error: Color::Rgb(244, 67, 54),
            warning: Color::Rgb(251, 192, 45),
            success: Color::Rgb(52, 168, 83),
            info: Color::Rgb(66, 133, 244),
            border: Color::Rgb(117, 117, 117),
            disabled: Color::Rgb(117, 117, 117),
        };
        
        theme.generate_default_styles();
        theme
    }

    /// Light theme
    pub fn light() -> Theme {
        let mut theme = Theme::with_description("light", "Light theme for VibeUI")
            .with_author("VibeUI")
            .with_version("1.0.0");
        
        theme.palette = ColorPalette {
            primary: Color::Rgb(66, 133, 244),
            secondary: Color::Rgb(117, 117, 117),
            accent: Color::Rgb(52, 168, 83),
            background: Color::White,
            surface: Color::Rgb(248, 248, 248),
            text: Color::Rgb(33, 33, 33),
            on_primary: Color::White,
            on_secondary: Color::White,
            on_accent: Color::White,
            on_surface: Color::Rgb(33, 33, 33),
            error: Color::Rgb(244, 67, 54),
            warning: Color::Rgb(251, 192, 45),
            success: Color::Rgb(52, 168, 83),
            info: Color::Rgb(66, 133, 244),
            border: Color::Rgb(189, 189, 189),
            disabled: Color::Rgb(189, 189, 189),
        };
        
        theme.generate_default_styles();
        theme
    }

    /// High contrast theme
    pub fn high_contrast() -> Theme {
        let mut theme = Theme::with_description("high_contrast", "High contrast theme for accessibility")
            .with_author("VibeUI")
            .with_version("1.0.0");
        
        theme.palette = ColorPalette {
            primary: Color::BrightWhite,
            secondary: Color::BrightWhite,
            accent: Color::BrightCyan,
            background: Color::Black,
            surface: Color::Black,
            text: Color::BrightWhite,
            on_primary: Color::Black,
            on_secondary: Color::Black,
            on_accent: Color::Black,
            on_surface: Color::BrightWhite,
            error: Color::BrightRed,
            warning: Color::BrightYellow,
            success: Color::BrightGreen,
            info: Color::BrightBlue,
            border: Color::BrightWhite,
            disabled: Color::Gray,
        };
        
        theme.generate_default_styles();
        theme
    }

    /// Retro terminal theme
    pub fn retro() -> Theme {
        let mut theme = Theme::with_description("retro", "Retro terminal theme")
            .with_author("VibeUI")
            .with_version("1.0.0");
        
        theme.palette = ColorPalette {
            primary: Color::Green,
            secondary: Color::BrightGreen,
            accent: Color::Yellow,
            background: Color::Black,
            surface: Color::Black,
            text: Color::Green,
            on_primary: Color::Black,
            on_secondary: Color::Black,
            on_accent: Color::Black,
            on_surface: Color::Green,
            error: Color::Red,
            warning: Color::Yellow,
            success: Color::Green,
            info: Color::Blue,
            border: Color::Green,
            disabled: Color::BrightBlack,
        };
        
        theme.generate_default_styles();
        theme
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new("test")
            .with_description("Test theme")
            .with_version("1.0.0")
            .with_author("Test Author");
        
        assert_eq!(theme.name, "test");
        assert_eq!(theme.description, Some("Test theme".to_string()));
        assert_eq!(theme.version, "1.0.0");
        assert_eq!(theme.author, Some("Test Author".to_string()));
    }

    #[test]
    fn test_theme_styles() {
        let mut theme = Theme::new("test");
        let style = Style::new().with_foreground(Color::Red);
        
        theme = theme.add_style("red_text", style);
        
        assert!(theme.get_style("red_text").is_some());
        assert_eq!(theme.get_style("red_text").unwrap().foreground, Some(Color::Red));
    }

    #[test]
    fn test_theme_manager() {
        let mut manager = ThemeManager::new();
        
        assert_eq!(manager.current_theme_name(), Some("default"));
        assert!(manager.get_theme("default").is_some());
        
        let dark_theme = builtin::dark();
        manager.add_theme(dark_theme);
        manager.set_current_theme("dark");
        
        assert_eq!(manager.current_theme_name(), Some("dark"));
        assert!(manager.get_style("primary").is_some());
    }

    #[test]
    fn test_builtin_themes() {
        let dark = builtin::dark();
        let light = builtin::light();
        let high_contrast = builtin::high_contrast();
        let retro = builtin::retro();
        
        assert_eq!(dark.name, "dark");
        assert_eq!(light.name, "light");
        assert_eq!(high_contrast.name, "high_contrast");
        assert_eq!(retro.name, "retro");
        
        // Check that themes have different palettes
        assert_ne!(dark.palette.background, light.palette.background);
        assert_eq!(high_contrast.palette.background, Color::Black);
        assert_eq!(retro.palette.primary, Color::Green);
    }

    #[test]
    fn test_theme_serialization() {
        let theme = builtin::dark();
        
        let json = theme.to_json().unwrap();
        let deserialized = Theme::from_json(&json).unwrap();
        
        assert_eq!(theme.name, deserialized.name);
        assert_eq!(theme.palette.primary, deserialized.palette.primary);
    }
}