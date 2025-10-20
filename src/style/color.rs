//! Color definitions and utilities

use serde::{Deserialize, Serialize};
use std::fmt;

/// Color representation for terminal UI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    /// Basic ANSI colors
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    
    /// Bright variants of basic colors
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    
    /// RGB color (24-bit true color)
    Rgb(u8, u8, u8),
    
    /// 256-color palette
    AnsiValue(u8),
}

impl Color {
    /// Create a color from RGB values
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb(r, g, b)
    }

    /// Create a color from hex string (e.g., "#FF0000" or "FF0000")
    pub fn from_hex(hex: &str) -> Result<Self, crate::style::Error> {
        let hex = hex.trim_start_matches('#');
        
        if hex.len() != 6 {
            return Err(crate::style::Error::ColorParse(
                "Hex color must be 6 characters long".to_string()
            ));
        }

        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|e| crate::style::Error::ColorParse(format!("Invalid hex: {}", e)))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|e| crate::style::Error::ColorParse(format!("Invalid hex: {}", e)))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|e| crate::style::Error::ColorParse(format!("Invalid hex: {}", e)))?;

        Ok(Color::Rgb(r, g, b))
    }

    /// Convert to ANSI value (0-255)
    pub fn as_ansi_value(&self) -> u8 {
        match self {
            Color::Black => 0,
            Color::Red => 1,
            Color::Green => 2,
            Color::Yellow => 3,
            Color::Blue => 4,
            Color::Magenta => 5,
            Color::Cyan => 6,
            Color::White => 7,
            Color::BrightBlack => 8,
            Color::BrightRed => 9,
            Color::BrightGreen => 10,
            Color::BrightYellow => 11,
            Color::BrightBlue => 12,
            Color::BrightMagenta => 13,
            Color::BrightCyan => 14,
            Color::BrightWhite => 15,
            Color::Rgb(r, g, b) => {
                // Convert RGB to 256-color palette
                if r == g && g == b {
                    // Grayscale
                    if r < 8 {
                        16
                    } else if r > 248 {
                        231
                    } else {
                        232 + (r - 8) / 10
                    }
                } else {
                    // 6x6x6 color cube
                    let r = (r as f32 / 255.0 * 5.0).round() as u8;
                    let g = (g as f32 / 255.0 * 5.0).round() as u8;
                    let b = (b as f32 / 255.0 * 5.0).round() as u8;
                    16 + 36 * r + 6 * g + b
                }
            }
            Color::AnsiValue(v) => *v,
        }
    }

    /// Convert to RGB values
    pub fn as_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Black => (0, 0, 0),
            Color::Red => (128, 0, 0),
            Color::Green => (0, 128, 0),
            Color::Yellow => (128, 128, 0),
            Color::Blue => (0, 0, 128),
            Color::Magenta => (128, 0, 128),
            Color::Cyan => (0, 128, 128),
            Color::White => (192, 192, 192),
            Color::BrightBlack => (128, 128, 128),
            Color::BrightRed => (255, 0, 0),
            Color::BrightGreen => (0, 255, 0),
            Color::BrightYellow => (255, 255, 0),
            Color::BrightBlue => (0, 0, 255),
            Color::BrightMagenta => (255, 0, 255),
            Color::BrightCyan => (0, 255, 255),
            Color::BrightWhite => (255, 255, 255),
            Color::Rgb(r, g, b) => (*r, *g, *b),
            Color::AnsiValue(v) => {
                // Convert ANSI value to RGB
                if *v < 16 {
                    // System colors
                    match *v {
                        0 => (0, 0, 0),
                        1 => (128, 0, 0),
                        2 => (0, 128, 0),
                        3 => (128, 128, 0),
                        4 => (0, 0, 128),
                        5 => (128, 0, 128),
                        6 => (0, 128, 128),
                        7 => (192, 192, 192),
                        8 => (128, 128, 128),
                        9 => (255, 0, 0),
                        10 => (0, 255, 0),
                        11 => (255, 255, 0),
                        12 => (0, 0, 255),
                        13 => (255, 0, 255),
                        14 => (0, 255, 255),
                        15 => (255, 255, 255),
                        _ => (0, 0, 0),
                    }
                } else if *v < 232 {
                    // 6x6x6 color cube
                    let v = *v - 16;
                    let r = v / 36;
                    let g = (v % 36) / 6;
                    let b = v % 6;
                    let r = if r == 0 { 0 } else { r * 40 + 55 };
                    let g = if g == 0 { 0 } else { g * 40 + 55 };
                    let b = if b == 0 { 0 } else { b * 40 + 55 };
                    (r, g, b)
                } else {
                    // Grayscale
                    let v = *v - 232;
                    let gray = v * 10 + 8;
                    (gray, gray, gray)
                }
            }
        }
    }

    /// Get the brightness of the color (0.0 to 1.0)
    pub fn brightness(&self) -> f32 {
        let (r, g, b) = self.as_rgb();
        (r as f32 * 0.299 + g as f32 * 0.587 + b as f32 * 0.114) / 255.0
    }

    /// Check if the color is light (brightness > 0.5)
    pub fn is_light(&self) -> bool {
        self.brightness() > 0.5
    }

    /// Check if the color is dark (brightness <= 0.5)
    pub fn is_dark(&self) -> bool {
        !self.is_light()
    }

    /// Blend this color with another color
    pub fn blend(&self, other: &Color, factor: f32) -> Color {
        let (r1, g1, b1) = self.as_rgb();
        let (r2, g2, b2) = other.as_rgb();
        
        let factor = factor.clamp(0.0, 1.0);
        let r = (r1 as f32 * (1.0 - factor) + r2 as f32 * factor) as u8;
        let g = (g1 as f32 * (1.0 - factor) + g2 as f32 * factor) as u8;
        let b = (b1 as f32 * (1.0 - factor) + b2 as f32 * factor) as u8;
        
        Color::Rgb(r, g, b)
    }

    /// Get a contrasting color (black or white) based on this color's brightness
    pub fn contrasting_color(&self) -> Color {
        if self.is_light() {
            Color::Black
        } else {
            Color::White
        }
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        let (r, g, b) = self.as_rgb();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    /// Parse a color from a string
    pub fn parse(s: &str) -> Result<Self, crate::style::Error> {
        let s = s.to_lowercase();
        
        match s.as_str() {
            "black" => Ok(Color::Black),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            "magenta" => Ok(Color::Magenta),
            "cyan" => Ok(Color::Cyan),
            "white" => Ok(Color::White),
            "brightblack" | "gray" => Ok(Color::BrightBlack),
            "brightred" => Ok(Color::BrightRed),
            "brightgreen" => Ok(Color::BrightGreen),
            "brightyellow" => Ok(Color::BrightYellow),
            "brightblue" => Ok(Color::BrightBlue),
            "brightmagenta" => Ok(Color::BrightMagenta),
            "brightcyan" => Ok(Color::BrightCyan),
            "brightwhite" => Ok(Color::BrightWhite),
            _ => {
                if s.starts_with('#') {
                    Self::from_hex(&s)
                } else if let Ok(ansi_value) = s.parse::<u8>() {
                    Ok(Color::AnsiValue(ansi_value))
                } else {
                    Err(crate::style::Error::ColorParse(
                        format!("Unknown color: {}", s)
                    ))
                }
            }
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Black => write!(f, "black"),
            Color::Red => write!(f, "red"),
            Color::Green => write!(f, "green"),
            Color::Yellow => write!(f, "yellow"),
            Color::Blue => write!(f, "blue"),
            Color::Magenta => write!(f, "magenta"),
            Color::Cyan => write!(f, "cyan"),
            Color::White => write!(f, "white"),
            Color::BrightBlack => write!(f, "brightblack"),
            Color::BrightRed => write!(f, "brightred"),
            Color::BrightGreen => write!(f, "brightgreen"),
            Color::BrightYellow => write!(f, "brightyellow"),
            Color::BrightBlue => write!(f, "brightblue"),
            Color::BrightMagenta => write!(f, "brightmagenta"),
            Color::BrightCyan => write!(f, "brightcyan"),
            Color::BrightWhite => write!(f, "brightwhite"),
            Color::Rgb(r, g, b) => write!(f, "#{:02X}{:02X}{:02X}", r, g, b),
            Color::AnsiValue(v) => write!(f, "ansi({})", v),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

/// Predefined color palette
pub mod palette {
    use super::Color;

    /// Basic terminal colors
    pub const BLACK: Color = Color::Black;
    pub const RED: Color = Color::Red;
    pub const GREEN: Color = Color::Green;
    pub const YELLOW: Color = Color::Yellow;
    pub const BLUE: Color = Color::Blue;
    pub const MAGENTA: Color = Color::Magenta;
    pub const CYAN: Color = Color::Cyan;
    pub const WHITE: Color = Color::White;

    /// Bright terminal colors
    pub const GRAY: Color = Color::BrightBlack;
    pub const BRIGHT_RED: Color = Color::BrightRed;
    pub const BRIGHT_GREEN: Color = Color::BrightGreen;
    pub const BRIGHT_YELLOW: Color = Color::BrightYellow;
    pub const BRIGHT_BLUE: Color = Color::BrightBlue;
    pub const BRIGHT_MAGENTA: Color = Color::BrightMagenta;
    pub const BRIGHT_CYAN: Color = Color::BrightCyan;
    pub const BRIGHT_WHITE: Color = Color::BrightWhite;

    /// Common RGB colors
    pub const ORANGE: Color = Color::Rgb(255, 165, 0);
    pub const PURPLE: Color = Color::Rgb(128, 0, 128);
    pub const PINK: Color = Color::Rgb(255, 192, 203);
    pub const BROWN: Color = Color::Rgb(165, 42, 42);
    pub const LIME: Color = Color::Rgb(0, 255, 0);
    pub const NAVY: Color = Color::Rgb(0, 0, 128);
    pub const TEAL: Color = Color::Rgb(0, 128, 128);
    pub const SILVER: Color = Color::Rgb(192, 192, 192);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let rgb = Color::rgb(255, 0, 0);
        assert_eq!(rgb, Color::Rgb(255, 0, 0));

        let hex = Color::from_hex("#FF0000").unwrap();
        assert_eq!(hex, Color::Rgb(255, 0, 0));

        let parsed = Color::parse("red").unwrap();
        assert_eq!(parsed, Color::Red);
    }

    #[test]
    fn test_color_conversion() {
        let rgb = Color::Rgb(255, 128, 0);
        let ansi = rgb.as_ansi_value();
        assert!(ansi < 256);

        let (r, g, b) = rgb.as_rgb();
        assert_eq!(r, 255);
        assert_eq!(g, 128);
        assert_eq!(b, 0);

        let hex = rgb.to_hex();
        assert_eq!(hex, "#FF8000");
    }

    #[test]
    fn test_color_brightness() {
        let black = Color::Black;
        let white = Color::White;
        let gray = Color::Rgb(128, 128, 128);

        assert!(black.is_dark());
        assert!(white.is_light());
        assert!(!gray.is_light() && !gray.is_dark()); // Around middle
    }

    #[test]
    fn test_color_blend() {
        let red = Color::Red;
        let blue = Color::Blue;
        let purple = red.blend(&blue, 0.5);
        
        let (r, g, b) = purple.as_rgb();
        assert_eq!(r, 64); // (128 + 0) / 2
        assert_eq!(g, 0);
        assert_eq!(b, 64); // (0 + 128) / 2
    }

    #[test]
    fn test_contrasting_color() {
        let black = Color::Black;
        let white = Color::White;
        
        assert_eq!(black.contrasting_color(), Color::White);
        assert_eq!(white.contrasting_color(), Color::Black);
    }

    #[test]
    fn test_color_display() {
        assert_eq!(Color::Red.to_string(), "red");
        assert_eq!(Color::Rgb(255, 0, 0).to_string(), "#FF0000");
        assert_eq!(Color::AnsiValue(42).to_string(), "ansi(42)");
    }
}