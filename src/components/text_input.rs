//! Text input component for user text entry

use crate::app::Result;
use crate::components::{BaseComponent, Component};
use crate::events::Event;
use crate::render::Renderer;
use crate::style::{Style, Color};

/// Text input component for entering text
#[derive(Debug, Clone)]
pub struct TextInput {
    base: BaseComponent,
    text: String,
    placeholder: String,
    style: Style,
    cursor_style: Style,
    cursor_position: usize,
    is_focused: bool,
    is_password: bool,
    max_length: Option<usize>,
}

impl TextInput {
    /// Create a new text input
    pub fn new() -> Self {
        let default_style = Style::default()
            .with_foreground(Color::White)
            .with_background(Color::Black);
            
        let cursor_style = Style::default()
            .with_foreground(Color::Black)
            .with_background(Color::White);

        Self {
            base: BaseComponent::new("text_input"),
            text: String::new(),
            placeholder: "Enter text...".to_string(),
            style: default_style,
            cursor_style,
            cursor_position: 0,
            is_focused: false,
            is_password: false,
            max_length: None,
        }
    }

    /// Create a new text input with ID
    pub fn with_id<S: Into<String>>(id: S) -> Self {
        let mut input = Self::new();
        input.base = BaseComponent::new(&id.into());
        input
    }

    /// Get the input text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set the input text
    pub fn set_text<S: Into<String>>(&mut self, text: S) {
        let text = text.into();
        if let Some(max_len) = self.max_length {
            if text.len() <= max_len {
                self.text = text;
                self.cursor_position = self.text.len().min(self.cursor_position);
            }
        } else {
            self.text = text;
            self.cursor_position = self.text.len().min(self.cursor_position);
        }
    }

    /// Get the placeholder text
    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    /// Set the placeholder text
    pub fn set_placeholder<S: Into<String>>(&mut self, placeholder: S) {
        self.placeholder = placeholder.into();
    }

    /// Set the maximum length of the input
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Set whether this is a password field
    pub fn with_password(mut self, is_password: bool) -> Self {
        self.is_password = is_password;
        self
    }

    /// Check if the input is focused
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    /// Set focus state
    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    /// Get the display text (with password masking if needed)
    fn display_text(&self) -> String {
        if self.is_password {
            "•".repeat(self.text.len())
        } else {
            self.text.clone()
        }
    }

    /// Insert a character at the cursor position
    fn insert_char(&mut self, c: char) {
        if let Some(max_len) = self.max_length {
            if self.text.len() >= max_len {
                return;
            }
        }
        
        self.text.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    /// Delete character at cursor position
    fn delete_char(&mut self) {
        if self.cursor_position < self.text.len() {
            self.text.remove(self.cursor_position);
        }
    }

    /// Backspace character before cursor
    fn backspace(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.text.remove(self.cursor_position);
        }
    }

    /// Move cursor left
    fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    /// Move cursor right
    fn move_cursor_right(&mut self) {
        if self.cursor_position < self.text.len() {
            self.cursor_position += 1;
        }
    }
}

impl Component for TextInput {
    fn render(&self, renderer: &mut Renderer) -> Result<()> {
        if !self.base.is_visible() {
            return Ok(());
        }

        let (x, y, width, height) = self.bounds();
        
        // Draw background
        for row in 0..height {
            renderer.draw_rect(x, y + row, width, 1, &self.style)?;
        }
        
        // Draw text or placeholder
        let display_text = if self.text.is_empty() {
            self.placeholder.clone()
        } else {
            self.display_text()
        };
        
        // Truncate text if it's too long
        let max_text_len = width.saturating_sub(2) as usize; // Leave room for borders
        let truncated_text = if display_text.len() > max_text_len {
            let start = if self.cursor_position > max_text_len {
                self.cursor_position - max_text_len
            } else {
                0
            };
            display_text.chars().skip(start).take(max_text_len).collect()
        } else {
            display_text
        };
        
        renderer.draw_text(x + 1, y, &truncated_text, &self.style, Some(width - 2))?;
        
        // Draw cursor if focused
        if self.is_focused && y < height {
            let cursor_x = x + 1 + (self.cursor_position.min(truncated_text.len()) as u16);
            renderer.draw_text(cursor_x, y, " ", &self.cursor_style, Some(1))?;
        }
        
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        if !self.is_focused {
            match event {
                Event::MouseClick { x, y, .. } => {
                    let (bx, by, bw, bh) = self.bounds();
                    if *x >= bx && *x < bx + bw && *y >= by && *y < by + bh {
                        self.is_focused = true;
                        return true;
                    }
                }
                _ => {}
            }
            return false;
        }

        match event {
            Event::KeyPress { key, .. } => {
                use crate::events::Key;
                match key {
                    Key::Char(c) => {
                        self.insert_char(*c);
                        return true;
                    }
                    Key::Backspace => {
                        self.backspace();
                        return true;
                    }
                    Key::Delete => {
                        self.delete_char();
                        return true;
                    }
                    Key::Left => {
                        self.move_cursor_left();
                        return true;
                    }
                    Key::Right => {
                        self.move_cursor_right();
                        return true;
                    }
                    Key::Home => {
                        self.cursor_position = 0;
                        return true;
                    }
                    Key::End => {
                        self.cursor_position = self.text.len();
                        return true;
                    }
                    Key::Esc => {
                        self.is_focused = false;
                        return true;
                    }
                    _ => {}
                }
            }
            Event::MouseClick { x, y, .. } => {
                let (bx, by, bw, bh) = self.bounds();
                if !(*x >= bx && *x < bx + bw && *y >= by && *y < by + bh) {
                    self.is_focused = false;
                    return true;
                }
            }
            _ => {}
        }
        false
    }

    fn update(&mut self) -> Result<()> {
        // Text inputs don't need to update
        Ok(())
    }

    fn bounds(&self) -> (u16, u16, u16, u16) {
        (
            self.base.position().0,
            self.base.position().1,
            self.base.size().0,
            self.base.size().1,
        )
    }

    fn set_position(&mut self, x: u16, y: u16) {
        self.base.set_position(x, y);
    }

    fn set_size(&mut self, width: u16, height: u16) {
        self.base.set_size(width, height);
    }

    fn is_visible(&self) -> bool {
        self.base.is_visible()
    }

    fn set_visible(&mut self, visible: bool) {
        self.base.set_visible(visible);
    }

    fn id(&self) -> &str {
        self.base.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_input_creation() {
        let input = TextInput::new();
        assert_eq!(input.text(), "");
        assert!(input.is_focused() == false);
        assert!(input.is_password == false);
    }

    #[test]
    fn test_text_input_with_id() {
        let input = TextInput::with_id("my_input");
        assert_eq!(input.id(), "my_input");
    }

    #[test]
    fn test_text_input_setters() {
        let mut input = TextInput::new();
        input.set_text("Hello");
        input.set_placeholder("Enter name");
        input.set_focused(true);
        
        assert_eq!(input.text(), "Hello");
        assert_eq!(input.placeholder(), "Enter name");
        assert!(input.is_focused());
    }

    #[test]
    fn test_password_field() {
        let input = TextInput::new().with_password(true);
        assert!(input.is_password);
    }
}