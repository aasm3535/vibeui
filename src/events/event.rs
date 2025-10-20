//! Event types for VibeUI

use crate::events::Key;
use serde::{Deserialize, Serialize};

/// Represents all possible events in the UI system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Event {
    /// Keyboard key press
    KeyPress {
        key: Key,
        modifiers: Modifiers,
    },
    
    /// Keyboard key release
    KeyRelease {
        key: Key,
        modifiers: Modifiers,
    },
    
    /// Mouse button press
    MousePress {
        button: MouseButton,
        x: u16,
        y: u16,
        modifiers: Modifiers,
    },
    
    /// Mouse button release
    MouseRelease {
        button: MouseButton,
        x: u16,
        y: u16,
        modifiers: Modifiers,
    },
    
    /// Mouse click (press + release)
    MouseClick {
        button: MouseButton,
        x: u16,
        y: u16,
        modifiers: Modifiers,
    },
    
    /// Mouse movement
    MouseMove {
        x: u16,
        y: u16,
        modifiers: Modifiers,
    },
    
    /// Mouse wheel scroll
    MouseScroll {
        direction: ScrollDirection,
        delta: i32,
        x: u16,
        y: u16,
        modifiers: Modifiers,
    },
    
    /// Window resize
    Resize {
        width: u16,
        height: u16,
    },
    
    /// Focus gained
    FocusGained,
    
    /// Focus lost
    FocusLost,
    
    /// Application quit
    Quit,
    
    /// Timer event
    Timer {
        id: String,
    },
    
    /// Custom event with data
    Custom {
        event_type: String,
        data: String,
    },
}

/// Mouse button types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button
    Middle,
    /// Additional mouse button 4
    Button4,
    /// Additional mouse button 5
    Button5,
}

/// Scroll direction
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ScrollDirection {
    /// Scroll up
    Up,
    /// Scroll down
    Down,
    /// Scroll left
    Left,
    /// Scroll right
    Right,
}

/// Keyboard modifier keys
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Modifiers {
    /// Shift key
    pub shift: bool,
    /// Control key
    pub ctrl: bool,
    /// Alt key
    pub alt: bool,
    /// Meta/Windows/Command key
    pub meta: bool,
}

impl Default for Modifiers {
    fn default() -> Self {
        Self {
            shift: false,
            ctrl: false,
            alt: false,
            meta: false,
        }
    }
}

impl Modifiers {
    /// Create a new modifier set
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any modifier is pressed
    pub fn is_empty(&self) -> bool {
        !self.shift && !self.ctrl && !self.alt && !self.meta
    }

    /// Set shift modifier
    pub fn with_shift(mut self, shift: bool) -> Self {
        self.shift = shift;
        self
    }

    /// Set control modifier
    pub fn with_ctrl(mut self, ctrl: bool) -> Self {
        self.ctrl = ctrl;
        self
    }

    /// Set alt modifier
    pub fn with_alt(mut self, alt: bool) -> Self {
        self.alt = alt;
        self
    }

    /// Set meta modifier
    pub fn with_meta(mut self, meta: bool) -> Self {
        self.meta = meta;
        self
    }
}

impl Event {
    /// Check if this is a keyboard event
    pub fn is_keyboard(&self) -> bool {
        matches!(self, Event::KeyPress { .. } | Event::KeyRelease { .. })
    }

    /// Check if this is a mouse event
    pub fn is_mouse(&self) -> bool {
        matches!(
            self,
            Event::MousePress { .. }
                | Event::MouseRelease { .. }
                | Event::MouseClick { .. }
                | Event::MouseMove { .. }
                | Event::MouseScroll { .. }
        )
    }

    /// Check if this is a window event
    pub fn is_window(&self) -> bool {
        matches!(
            self,
            Event::Resize { .. } | Event::FocusGained | Event::FocusLost | Event::Quit
        )
    }

    /// Get the position of a mouse event
    pub fn mouse_position(&self) -> Option<(u16, u16)> {
        match self {
            Event::MousePress { x, y, .. }
            | Event::MouseRelease { x, y, .. }
            | Event::MouseClick { x, y, .. }
            | Event::MouseMove { x, y, .. }
            | Event::MouseScroll { x, y, .. } => Some((*x, *y)),
            _ => None,
        }
    }

    /// Get the modifiers of an event
    pub fn modifiers(&self) -> Modifiers {
        match self {
            Event::KeyPress { modifiers, .. }
            | Event::KeyRelease { modifiers, .. }
            | Event::MousePress { modifiers, .. }
            | Event::MouseRelease { modifiers, .. }
            | Event::MouseClick { modifiers, .. }
            | Event::MouseMove { modifiers, .. }
            | Event::MouseScroll { modifiers, .. } => *modifiers,
            _ => Modifiers::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifiers() {
        let modifiers = Modifiers::new()
            .with_shift(true)
            .with_ctrl(true);
        
        assert!(modifiers.shift);
        assert!(modifiers.ctrl);
        assert!(!modifiers.alt);
        assert!(!modifiers.meta);
        assert!(!modifiers.is_empty());
    }

    #[test]
    fn test_event_types() {
        let key_event = Event::KeyPress {
            key: Key::Char('a'),
            modifiers: Modifiers::default(),
        };
        assert!(key_event.is_keyboard());
        assert!(!key_event.is_mouse());
        assert!(!key_event.is_window());

        let mouse_event = Event::MouseClick {
            button: MouseButton::Left,
            x: 10,
            y: 20,
            modifiers: Modifiers::default(),
        };
        assert!(!mouse_event.is_keyboard());
        assert!(mouse_event.is_mouse());
        assert!(!mouse_event.is_window());
        assert_eq!(mouse_event.mouse_position(), Some((10, 20)));

        let window_event = Event::Resize { width: 80, height: 24 };
        assert!(!window_event.is_keyboard());
        assert!(!window_event.is_mouse());
        assert!(window_event.is_window());
    }
}