//! Terminal handling for cross-platform support

use crate::platform::{Error, Result};
use crossterm::{
    event::{self, Event as CrosstermEvent, KeyCode, KeyEvent, MouseEvent, MouseEventKind},
    execute, queue,
    terminal::{self, ClearType},
    cursor::{self, MoveTo},
    style::{Color as CrosstermColor, Print, SetForegroundColor, SetBackgroundColor, 
            SetAttribute, Attribute, ResetColor},
    Command,
};
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};

/// Terminal state tracking
static TERMINAL_INITIALIZED: AtomicBool = AtomicBool::new(false);
static RAW_MODE_ENABLED: AtomicBool = AtomicBool::new(false);
static MOUSE_CAPTURE_ENABLED: AtomicBool = AtomicBool::new(false);

/// Terminal abstraction for cross-platform support
#[derive(Debug)]
pub struct Terminal {
    width: u16,
    height: u16,
    original_terminal_settings: Option<()>, // Platform-specific settings
}

impl Terminal {
    /// Initialize the terminal system
    pub fn init() -> Result<()> {
        if TERMINAL_INITIALIZED.load(Ordering::SeqCst) {
            return Ok(());
        }

        // Enable raw mode
        terminal::enable_raw_mode().map_err(|e| {
            Error::Init(format!("Failed to enable raw mode: {}", e))
        })?;
        RAW_MODE_ENABLED.store(true, Ordering::SeqCst);

        // Enable mouse events
        event::EnableMouseCapture::apply().map_err(|e| {
            Error::Init(format!("Failed to enable mouse capture: {}", e))
        })?;
        MOUSE_CAPTURE_ENABLED.store(true, Ordering::SeqCst);

        // Hide cursor
        cursor::Hide::apply().map_err(|e| {
            Error::Init(format!("Failed to hide cursor: {}", e))
        })?;

        // Clear screen
        terminal::clear(ClearType::All).map_err(|e| {
            Error::Init(format!("Failed to clear screen: {}", e))
        })?;

        TERMINAL_INITIALIZED.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Cleanup the terminal system
    pub fn cleanup() -> Result<()> {
        if !TERMINAL_INITIALIZED.load(Ordering::SeqCst) {
            return Ok(());
        }

        // Show cursor
        let _ = cursor::Show::apply();

        // Disable mouse capture
        if MOUSE_CAPTURE_ENABLED.load(Ordering::SeqCst) {
            let _ = event::DisableMouseCapture::apply();
            MOUSE_CAPTURE_ENABLED.store(false, Ordering::SeqCst);
        }

        // Disable raw mode
        if RAW_MODE_ENABLED.load(Ordering::SeqCst) {
            let _ = terminal::disable_raw_mode();
            RAW_MODE_ENABLED.store(false, Ordering::SeqCst);
        }

        // Clear screen and reset
        let _ = terminal::clear(ClearType::All);
        let _ = cursor::MoveTo(0, 0);

        TERMINAL_INITIALIZED.store(false, Ordering::SeqCst);
        Ok(())
    }

    /// Create a new terminal instance
    pub fn new() -> Result<Self> {
        let (width, height) = Self::size()?;
        
        Ok(Self {
            width,
            height,
            original_terminal_settings: None,
        })
    }

    /// Initialize this terminal instance
    pub fn initialize(&mut self) -> Result<()> {
        Self::init()?;
        self.update_size()?;
        Ok(())
    }

    /// Update terminal size
    pub fn update_size(&mut self) -> Result<()> {
        let (width, height) = Self::size()?;
        self.width = width;
        self.height = height;
        Ok(())
    }

    /// Get terminal size
    pub fn size() -> Result<(u16, u16)> {
        terminal::size().map_err(|e| {
            Error::SizeDetection(format!("Failed to get terminal size: {}", e))
        })
    }

    /// Get terminal width
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Get terminal height
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Check if terminal supports colors
    pub fn supports_color() -> bool {
        // Check if the TERM environment variable indicates color support
        std::env::var("TERM")
            .map(|term| {
                term.contains("color") 
                    || term.contains("256") 
                    || term.contains("xterm") 
                    || term.contains("screen")
            })
            .unwrap_or(false)
    }

    /// Check if terminal supports mouse events
    pub fn supports_mouse() -> bool {
        // Most modern terminals support mouse events
        true
    }

    /// Enable raw mode
    pub fn enable_raw_mode() -> Result<()> {
        terminal::enable_raw_mode().map_err(|e| {
            Error::Init(format!("Failed to enable raw mode: {}", e))
        })?;
        RAW_MODE_ENABLED.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Disable raw mode
    pub fn disable_raw_mode() -> Result<()> {
        terminal::disable_raw_mode().map_err(|e| {
            Error::Init(format!("Failed to disable raw mode: {}", e))
        })?;
        RAW_MODE_ENABLED.store(false, Ordering::SeqCst);
        Ok(())
    }

    /// Enable mouse capture
    pub fn enable_mouse_capture() -> Result<()> {
        event::EnableMouseCapture::apply().map_err(|e| {
            Error::Init(format!("Failed to enable mouse capture: {}", e))
        })?;
        MOUSE_CAPTURE_ENABLED.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Disable mouse capture
    pub fn disable_mouse_capture() -> Result<()> {
        event::DisableMouseCapture::apply().map_err(|e| {
            Error::Init(format!("Failed to disable mouse capture: {}", e))
        })?;
        MOUSE_CAPTURE_ENABLED.store(false, Ordering::SeqCst);
        Ok(())
    }

    /// Clear the terminal
    pub fn clear() -> Result<()> {
        terminal::clear(ClearType::All).map_err(|e| {
            Error::Io(e)
        })
    }

    /// Clear a line
    pub fn clear_line() -> Result<()> {
        terminal::clear(ClearType::CurrentLine).map_err(|e| {
            Error::Io(e)
        })
    }

    /// Move cursor to position
    pub fn move_cursor(x: u16, y: u16) -> Result<()> {
        execute!(stdout(), MoveTo(x, y)).map_err(|e| {
            Error::Io(e)
        })
    }

    /// Show cursor
    pub fn show_cursor() -> Result<()> {
        cursor::Show::apply().map_err(|e| {
            Error::Io(e)
        })
    }

    /// Hide cursor
    pub fn hide_cursor() -> Result<()> {
        cursor::Hide::apply().map_err(|e| {
            Error::Io(e)
        })
    }

    /// Flush output
    pub fn flush() -> Result<()> {
        stdout().flush().map_err(|e| {
            Error::Io(e)
        })
    }

    /// Read a single event
    pub fn read_event() -> Result<Option<crate::events::Event>> {
        if event::poll(std::time::Duration::from_millis(0))? {
            let crossterm_event = event::read()?;
            Ok(Some(self.convert_crossterm_event(crossterm_event)?))
        } else {
            Ok(None)
        }
    }

    /// Convert crossterm event to VibeUI event
    fn convert_crossterm_event(&self, event: CrosstermEvent) -> Result<crate::events::Event> {
        match event {
            CrosstermEvent::Key(KeyEvent { code, modifiers }) => {
                let key = self.convert_key_code(code)?;
                let vibe_modifiers = self.convert_modifiers(modifiers);
                
                Ok(crate::events::Event::KeyPress {
                    key,
                    modifiers: vibe_modifiers,
                })
            }
            CrosstermEvent::Mouse(MouseEvent { kind, column, row, modifiers }) => {
                let vibe_modifiers = self.convert_modifiers(modifiers);
                let x = column;
                let y = row;
                
                match kind {
                    MouseEventKind::Down(button) => {
                        let vibe_button = self.convert_mouse_button(button);
                        Ok(crate::events::Event::MousePress {
                            button: vibe_button,
                            x,
                            y,
                            modifiers: vibe_modifiers,
                        })
                    }
                    MouseEventKind::Up(button) => {
                        let vibe_button = self.convert_mouse_button(button);
                        Ok(crate::events::Event::MouseRelease {
                            button: vibe_button,
                            x,
                            y,
                            modifiers: vibe_modifiers,
                        })
                    }
                    MouseEventKind::Drag(button) => {
                        let vibe_button = self.convert_mouse_button(button);
                        Ok(crate::events::Event::MouseMove {
                            x,
                            y,
                            modifiers: vibe_modifiers,
                        })
                    }
                    MouseEventKind::Moved => {
                        Ok(crate::events::Event::MouseMove {
                            x,
                            y,
                            modifiers: vibe_modifiers,
                        })
                    }
                    MouseEventKind::ScrollUp => {
                        Ok(crate::events::Event::MouseScroll {
                            direction: crate::events::ScrollDirection::Up,
                            delta: 1,
                            x,
                            y,
                            modifiers: vibe_modifiers,
                        })
                    }
                    MouseEventKind::ScrollDown => {
                        Ok(crate::events::Event::MouseScroll {
                            direction: crate::events::ScrollDirection::Down,
                            delta: 1,
                            x,
                            y,
                            modifiers: vibe_modifiers,
                        })
                    }
                    MouseEventKind::ScrollLeft => {
                        Ok(crate::events::Event::MouseScroll {
                            direction: crate::events::ScrollDirection::Left,
                            delta: 1,
                            x,
                            y,
                            modifiers: vibe_modifiers,
                        })
                    }
                    MouseEventKind::ScrollRight => {
                        Ok(crate::events::Event::MouseScroll {
                            direction: crate::events::ScrollDirection::Right,
                            delta: 1,
                            x,
                            y,
                            modifiers: vibe_modifiers,
                        })
                    }
                }
            }
            CrosstermEvent::Resize(width, height) => {
                Ok(crate::events::Event::Resize { width, height })
            }
            CrosstermEvent::FocusGained => {
                Ok(crate::events::Event::FocusGained)
            }
            CrosstermEvent::FocusLost => {
                Ok(crate::events::Event::FocusLost)
            }
            CrosstermEvent::Paste(_) => {
                // TODO: Handle paste events
                Ok(crate::events::Event::Custom {
                    event_type: "paste".to_string(),
                    data: "".to_string(),
                })
            }
        }
    }

    /// Convert crossterm key code to VibeUI key
    fn convert_key_code(&self, code: KeyCode) -> Result<crate::events::Key> {
        match code {
            KeyCode::Char(c) => Ok(crate::events::Key::Char(c)),
            KeyCode::Backspace => Ok(crate::events::Key::Backspace),
            KeyCode::Enter => Ok(crate::events::Key::Enter),
            KeyCode::Tab => Ok(crate::events::Key::Tab),
            KeyCode::Delete => Ok(crate::events::Key::Delete),
            KeyCode::Insert => Ok(crate::events::Key::Insert),
            KeyCode::Esc => Ok(crate::events::Key::Esc),
            KeyCode::Home => Ok(crate::events::Key::Home),
            KeyCode::End => Ok(crate::events::Key::End),
            KeyCode::PageUp => Ok(crate::events::Key::PageUp),
            KeyCode::PageDown => Ok(crate::events::Key::PageDown),
            KeyCode::Up => Ok(crate::events::Key::Up),
            KeyCode::Down => Ok(crate::events::Key::Down),
            KeyCode::Left => Ok(crate::events::Key::Left),
            KeyCode::Right => Ok(crate::events::Key::Right),
            KeyCode::F(n) => Ok(crate::events::Key::F(n)),
            KeyCode::Null => Ok(crate::events::Key::Unknown),
            _ => Ok(crate::events::Key::Unknown),
        }
    }

    /// Convert crossterm modifiers to VibeUI modifiers
    fn convert_modifiers(&self, modifiers: event::KeyModifiers) -> crate::events::Modifiers {
        crate::events::Modifiers {
            shift: modifiers.contains(event::KeyModifiers::SHIFT),
            ctrl: modifiers.contains(event::KeyModifiers::CONTROL),
            alt: modifiers.contains(event::KeyModifiers::ALT),
            meta: modifiers.contains(event::KeyModifiers::SUPER),
        }
    }

    /// Convert crossterm mouse button to VibeUI mouse button
    fn convert_mouse_button(&self, button: event::MouseButton) -> crate::events::MouseButton {
        match button {
            event::MouseButton::Left => crate::events::MouseButton::Left,
            event::MouseButton::Right => crate::events::MouseButton::Right,
            event::MouseButton::Middle => crate::events::MouseButton::Middle,
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = Self::cleanup();
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new().expect("Failed to create Terminal")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_creation() {
        let terminal = Terminal::new();
        assert!(terminal.is_ok());
    }

    #[test]
    fn test_terminal_size() {
        let size = Terminal::size();
        assert!(size.is_ok());
        let (width, height) = size.unwrap();
        assert!(width > 0);
        assert!(height > 0);
    }

    #[test]
    fn test_supports_color() {
        let supports = Terminal::supports_color();
        // This might be true or false depending on the environment
        // We just test that it doesn't panic
    }
}