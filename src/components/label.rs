//! Label component for displaying text

use crate::app::Result;
use crate::components::{BaseComponent, Component};
use crate::events::Event;
use crate::render::Renderer;
use crate::style::{Style, Color};

/// Simple text label component
#[derive(Debug, Clone)]
pub struct Label {
    base: BaseComponent,
    text: String,
    style: Style,
}

impl Label {
    /// Create a new label with the given text
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self {
            base: BaseComponent::new("label"),
            text: text.into(),
            style: Style::default(),
        }
    }

    /// Create a new label with ID and text
    pub fn with_id<S: Into<String>>(id: S, text: S) -> Self {
        let mut label = Self::new(text);
        label.base = BaseComponent::new(&id.into());
        label
    }

    /// Get the label text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set the label text
    pub fn set_text<S: Into<String>>(&mut self, text: S) {
        self.text = text.into();
    }

    /// Get the label style
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// Set the label style
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    /// Set text color
    pub fn with_color(mut self, color: Color) -> Self {
        self.style.foreground = Some(color);
        self
    }

    /// Set background color
    pub fn with_background(mut self, color: Color) -> Self {
        self.style.background = Some(color);
        self
    }

    /// Set bold text
    pub fn with_bold(mut self, bold: bool) -> Self {
        self.style.bold = bold;
        self
    }

    /// Set italic text
    pub fn with_italic(mut self, italic: bool) -> Self {
        self.style.italic = italic;
        self
    }
}

impl Component for Label {
    fn render(&self, renderer: &mut Renderer) -> Result<()> {
        if !self.base.is_visible() {
            return Ok(());
        }

        let (x, y, width, height) = self.bounds();
        renderer.draw_text(x, y, &self.text, &self.style, Some(width))?;
        
        Ok(())
    }

    fn handle_event(&mut self, _event: &Event) -> bool {
        // Labels don't handle events
        false
    }

    fn update(&mut self) -> Result<()> {
        // Labels don't need to update
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
    fn test_label_creation() {
        let label = Label::new("Test Label");
        assert_eq!(label.text(), "Test Label");
    }

    #[test]
    fn test_label_with_id() {
        let label = Label::with_id("my_label", "Test");
        assert_eq!(label.id(), "my_label");
        assert_eq!(label.text(), "Test");
    }

    #[test]
    fn test_label_style() {
        let label = Label::new("Test")
            .with_color(Color::Red)
            .with_bold(true);
        
        assert_eq!(label.style().foreground, Some(Color::Red));
        assert!(label.style().bold);
    }

    #[test]
    fn test_label_setters() {
        let mut label = Label::new("Original");
        label.set_text("Updated");
        label.set_visible(false);
        
        assert_eq!(label.text(), "Updated");
        assert!(!label.is_visible());
    }
}