//! Keyboard key definitions

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents keyboard keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Key {
    /// Printable characters
    Char(char),
    
    /// Backspace key
    Backspace,
    /// Enter/Return key
    Enter,
    /// Tab key
    Tab,
    /// Delete key
    Delete,
    /// Insert key
    Insert,
    
    /// Escape key
    Esc,
    
    /// Home key
    Home,
    /// End key
    End,
    /// Page Up key
    PageUp,
    /// Page Down key
    PageDown,
    
    /// Arrow keys
    Up,
    Down,
    Left,
    Right,
    
    /// Function keys
    F(u8),
    
    /// Modifier keys
    Shift,
    Ctrl,
    Alt,
    Meta,
    
    /// Space key
    Space,
    
    /// Control characters
    Ctrl(char),
    Alt(char),
    
    /// Unknown key
    Unknown,
}

impl Key {
    /// Check if this is a printable character
    pub fn is_printable(&self) -> bool {
        matches!(self, Key::Char(c) if c.is_ascii_graphic())
    }

    /// Check if this is a control key
    pub fn is_control(&self) -> bool {
        matches!(
            self,
            Key::Backspace
                | Key::Enter
                | Key::Tab
                | Key::Delete
                | Key::Insert
                | Key::Esc
                | Key::Home
                | Key::End
                | Key::PageUp
                | Key::PageDown
                | Key::Up
                | Key::Down
                | Key::Left
                | Key::Right
                | Key::F(_)
                | Key::Shift
                | Key::Ctrl
                | Key::Alt
                | Key::Meta
                | Key::Ctrl(_)
                | Key::Alt(_)
        )
    }

    /// Check if this is a navigation key
    pub fn is_navigation(&self) -> bool {
        matches!(
            self,
            Key::Up | Key::Down | Key::Left | Key::Right | Key::Home | Key::End | Key::PageUp | Key::PageDown
        )
    }

    /// Check if this is a function key
    pub fn is_function(&self) -> bool {
        matches!(self, Key::F(_))
    }

    /// Get the character value if this is a character key
    pub fn as_char(&self) -> Option<char> {
        match self {
            Key::Char(c) => Some(*c),
            Key::Space => Some(' '),
            Key::Tab => Some('\t'),
            Key::Enter => Some('\n'),
            _ => None,
        }
    }

    /// Get the function key number if this is a function key
    pub fn as_function(&self) -> Option<u8> {
        match self {
            Key::F(n) => Some(*n),
            _ => None,
        }
    }

    /// Convert to a string representation
    pub fn to_string(&self) -> String {
        match self {
            Key::Char(c) => c.to_string(),
            Key::Backspace => "Backspace".to_string(),
            Key::Enter => "Enter".to_string(),
            Key::Tab => "Tab".to_string(),
            Key::Delete => "Delete".to_string(),
            Key::Insert => "Insert".to_string(),
            Key::Esc => "Esc".to_string(),
            Key::Home => "Home".to_string(),
            Key::End => "End".to_string(),
            Key::PageUp => "PageUp".to_string(),
            Key::PageDown => "PageDown".to_string(),
            Key::Up => "Up".to_string(),
            Key::Down => "Down".to_string(),
            Key::Left => "Left".to_string(),
            Key::Right => "Right".to_string(),
            Key::F(n) => format!("F{}", n),
            Key::Shift => "Shift".to_string(),
            Key::Ctrl => "Ctrl".to_string(),
            Key::Alt => "Alt".to_string(),
            Key::Meta => "Meta".to_string(),
            Key::Space => "Space".to_string(),
            Key::Ctrl(c) => format!("Ctrl+{}", c),
            Key::Alt(c) => format!("Alt+{}", c),
            Key::Unknown => "Unknown".to_string(),
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Key parsing utilities
pub mod parse {
    use super::*;

    /// Parse a string into a key
    pub fn parse_key(s: &str) -> Key {
        match s.to_uppercase().as_str() {
            "BACKSPACE" => Key::Backspace,
            "ENTER" | "RETURN" => Key::Enter,
            "TAB" => Key::Tab,
            "DELETE" | "DEL" => Key::Delete,
            "INSERT" | "INS" => Key::Insert,
            "ESCAPE" | "ESC" => Key::Esc,
            "HOME" => Key::Home,
            "END" => Key::End,
            "PAGEUP" => Key::PageUp,
            "PAGEDOWN" => Key::PageDown,
            "UP" => Key::Up,
            "DOWN" => Key::Down,
            "LEFT" => Key::Left,
            "RIGHT" => Key::Right,
            "SHIFT" => Key::Shift,
            "CTRL" | "CONTROL" => Key::Ctrl,
            "ALT" => Key::Alt,
            "META" | "CMD" | "WIN" => Key::Meta,
            "SPACE" => Key::Space,
            _ => {
                // Check for function keys
                if let Some(rest) = s.strip_prefix("F").to_owned() {
                    if let Ok(n) = rest.parse::<u8>() {
                        return Key::F(n);
                    }
                }
                
                // Check for Ctrl+ combinations
                if let Some(rest) = s.strip_prefix("CTRL+").to_owned() {
                    if rest.len() == 1 {
                        return Key::Ctrl(rest.chars().next().unwrap());
                    }
                }
                
                // Check for Alt+ combinations
                if let Some(rest) = s.strip_prefix("ALT+").to_owned() {
                    if rest.len() == 1 {
                        return Key::Alt(rest.chars().next().unwrap());
                    }
                }
                
                // Single character
                if s.len() == 1 {
                    return Key::Char(s.chars().next().unwrap());
                }
                
                Key::Unknown
            }
        }
    }

    /// Parse a key sequence from a string
    pub fn parse_key_sequence(s: &str) -> Vec<Key> {
        s.split('+').map(parse_key).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_properties() {
        assert!(Key::Char('a').is_printable());
        assert!(!Key::Char('a').is_control());
        assert!(Key::Enter.is_control());
        assert!(!Key::Enter.is_printable());
        assert!(Key::Up.is_navigation());
        assert!(Key::F(5).is_function());
    }

    #[test]
    fn test_key_conversion() {
        assert_eq!(Key::Char('a').as_char(), Some('a'));
        assert_eq!(Key::Space.as_char(), Some(' '));
        assert_eq!(Key::Enter.as_char(), Some('\n'));
        assert_eq!(Key::F(5).as_function(), Some(5));
        assert_eq!(Key::Char('a').as_function(), None);
    }

    #[test]
    fn test_key_parsing() {
        assert_eq!(parse::parse_key("A"), Key::Char('A'));
        assert_eq!(parse::parse_key("enter"), Key::Enter);
        assert_eq!(parse::parse_key("F5"), Key::F(5));
        assert_eq!(parse::parse_key("Ctrl+A"), Key::Ctrl('A'));
        assert_eq!(parse::parse_key("Alt+B"), Key::Alt('B'));
    }

    #[test]
    fn test_key_display() {
        assert_eq!(Key::Char('a').to_string(), "a");
        assert_eq!(Key::Enter.to_string(), "Enter");
        assert_eq!(Key::F(12).to_string(), "F12");
        assert_eq!(Key::Ctrl('c').to_string(), "Ctrl+c");
    }
}