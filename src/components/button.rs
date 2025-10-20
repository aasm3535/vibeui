//! Button component for clickable actions

use crate::app::Result;
use crate::components::{BaseComponent, Component};
use crate::events::Event;
use crate::render::Renderer;
use crate::style::{Style, Color};

/// Button component that can be clicked
#[derive(Debug, Clone)]
pub struct Button {
    base: BaseComponent,
    text: String,
    style: Style,
    hover_style: Style,
    active_style: Style,
    is_hovered: bool,
    is_active: bool,
    on_click: Option<Box<dyn Fn(&mut Self)>>,
}

impl Button {
    /// Create a new button with the given text
    pub fn new<S: Into<String>>(text: S) -> Self {
        let default_style = Style::default()
            .with_foreground(Color::White)
            .with_background(Color::Blue)
            .with_bold(true);
            
        let hover_style = Style::default()
            .with_foreground(Color::Black)
            .with_background(Color::Cyan)
            .with_bold(true);
            
        let active_style = Style::default()
            .with_foreground(Color::White)
            .with_background(Color::Red)
            .with_bold(true);

        Self {
            base: BaseComponent::new("button"),
            text: text.into(),
            style: default_style,
            hover_style,
            active_style,
            is_hovered: false,
            is_active: false,
            on_click: None,
        }
    }

    /// Create a new button with ID and text
    pub fn with_id<S: Into<String>>(id: S, text: S) -> Self {
        let mut button = Self::new(text);
        button.base = BaseComponent::new(&id.into());
        button
    }

    /// Get the button text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set the button text
    pub fn set_text<S: Into<String>>(&mut self, text: S) {
        self.text = text.into();
    }

    /// Set the click callback
    pub fn on_click<F: Fn(&mut Self) + 'static>(mut self, f: F) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }

    /// Check if the button is hovered
    pub fn is_hovered(&self) -> bool {
        self.is_hovered
    }

    /// Check if the button is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Set custom styles
    pub fn with_styles(mut self, normal: Style, hover: Style, active: Style) -> Self {
        self.style = normal;
        self.hover_style = hover;
        self.active_style = active;
        self
    }

    /// Get the current style based on button state
    fn current_style(&self) -> &Style {
        if self.is_active {
            &self.active_style
        } else if self.is_hovered {
            &self.hover_style
        } else {
            &self.style
        }
    }

    /// Handle mouse click
    fn handle_click(&mut self) {
        if let Some(ref callback) = self.on_click {
            callback(self);
        }
    }
}

impl Component for Button {
    fn render(&self, renderer: &mut Renderer) -> Result<()> {
        if !self.base.is_visible() {
            return Ok(());
        }

        let (x, y, width, height) = self.bounds();
        let style = self.current_style();
        
        // Draw button background
        for row in 0..height {
            renderer.draw_rect(x, y + row, width, 1, style)?;
        }
        
        // Draw button text (centered)
        let text_len = self.text.len() as u16;
        if text_len < width {
            let text_x = x + (width - text_len) / 2;
            let text_y = y + height / 2;
            renderer.draw_text(text_x, text_y, &self.text, style, Some(width))?;
        } else {
            // Truncate text if it's too long
            let truncated = &self.text[..(width - 3).min(text_len) as usize];
            let text = format!("{}...", truncated);
            renderer.draw_text(x, y + height / 2, &text, style, Some(width))?;
        }
        
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::MouseClick { x, y, .. } => {
                let (bx, by, bw, bh) = self.bounds();
                if *x >= bx && *x < bx + bw && *y >= by && *y < by + bh {
                    self.handle_click();
                    self.is_active = true;
                    return true;
                }
            }
            Event::MouseMove { x, y, .. } => {
                let (bx, by, bw, bh) = self.bounds();
                let was_hovered = self.is_hovered;
                self.is_hovered = *x >= bx && *x < bx + bw && *y >= by && *y < by + bh;
                if was_hovered != self.is_hovered {
                    return true; // State changed, need to redraw
                }
            }
            Event::MouseRelease { .. } => {
                if self.is_active {
                    self.is_active = false;
                    return true;
                }
            }
            _ => {}
        }
        false
    }

    fn update(&mut self) -> Result<()> {
        // Buttons don't need to update
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
    use crate::style::Color;

    #[test]
    fn test_button_creation() {
        let button = Button::new("Click me");
        assert_eq!(button.text(), "Click me");
        assert!(!button.is_hovered());
        assert!(!button.is_active());
    }

    #[test]
    fn test_button_with_id() {
        let button = Button::with_id("my_button", "Test");
        assert_eq!(button.id(), "my_button");
        assert_eq!(button.text(), "Test");
    }

    #[test]
    fn test_button_states() {
        let mut button = Button::new("Test");
        // Initially not hovered or active
        assert!(!button.is_hovered());
        assert!(!button.is_active());
    }
}