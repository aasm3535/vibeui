//! Container component for grouping other components

use crate::app::Result;
use crate::components::{BaseComponent, Component};
use crate::events::Event;
use crate::render::Renderer;
use crate::style::{Style, Color};
use std::collections::HashMap;

/// Container component that can hold other components
#[derive(Debug, Clone)]
pub struct Container {
    base: BaseComponent,
    style: Style,
    border_style: Option<Style>,
    children: HashMap<String, Box<dyn Component>>,
    layout: LayoutType,
    padding: (u16, u16, u16, u16), // top, right, bottom, left
}

/// Layout types for container
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutType {
    /// Stack children vertically
    Vertical,
    /// Stack children horizontally
    Horizontal,
    /// Place children at specific positions
    Absolute,
    /// Grid layout (not implemented yet)
    Grid,
}

impl Container {
    /// Create a new container
    pub fn new() -> Self {
        Self {
            base: BaseComponent::new("container"),
            style: Style::default(),
            border_style: None,
            children: HashMap::new(),
            layout: LayoutType::Vertical,
            padding: (1, 1, 1, 1),
        }
    }

    /// Create a new container with ID
    pub fn with_id<S: Into<String>>(id: S) -> Self {
        let mut container = Self::new();
        container.base = BaseComponent::new(&id.into());
        container
    }

    /// Set the layout type
    pub fn with_layout(mut self, layout: LayoutType) -> Self {
        self.layout = layout;
        self
    }

    /// Set padding (top, right, bottom, left)
    pub fn with_padding(mut self, top: u16, right: u16, bottom: u16, left: u16) -> Self {
        self.padding = (top, right, bottom, left);
        self
    }

    /// Set uniform padding
    pub fn with_uniform_padding(mut self, padding: u16) -> Self {
        self.padding = (padding, padding, padding, padding);
        self
    }

    /// Set border style
    pub fn with_border(mut self, style: Style) -> Self {
        self.border_style = Some(style);
        self
    }

    /// Remove border
    pub fn without_border(mut self) -> Self {
        self.border_style = None;
        self
    }

    /// Add a child component
    pub fn add_child<C: Component + 'static>(&mut self, name: &str, component: C) {
        self.children.insert(name.to_string(), Box::new(component));
    }

    /// Get a child component by name
    pub fn get_child(&self, name: &str) -> Option<&dyn Component> {
        self.children.get(name).map(|c| c.as_ref())
    }

    /// Get a mutable child component by name
    pub fn get_child_mut(&mut self, name: &str) -> Option<&mut dyn Component> {
        self.children.get_mut(name).map(|c| c.as_mut())
    }

    /// Remove a child component
    pub fn remove_child(&mut self, name: &str) -> Option<Box<dyn Component>> {
        self.children.remove(name)
    }

    /// Get all child names
    pub fn child_names(&self) -> Vec<&str> {
        self.children.keys().map(|s| s.as_str()).collect()
    }

    /// Calculate child positions based on layout
    fn arrange_children(&mut self) {
        let (x, y, width, height) = self.bounds();
        let (padding_top, padding_right, padding_bottom, padding_left) = self.padding;
        
        let content_x = x + padding_left;
        let content_y = y + padding_top;
        let content_width = width.saturating_sub(padding_left + padding_right);
        let content_height = height.saturating_sub(padding_top + padding_bottom);

        match self.layout {
            LayoutType::Vertical => {
                let child_height = content_height / self.children.len().max(1) as u16;
                let mut current_y = content_y;
                
                for (i, (_, child)) in self.children.iter_mut().enumerate() {
                    child.set_position(content_x, current_y);
                    child.set_size(content_width, child_height);
                    current_y += child_height;
                }
            }
            LayoutType::Horizontal => {
                let child_width = content_width / self.children.len().max(1) as u16;
                let mut current_x = content_x;
                
                for (_, child) in self.children.values_mut() {
                    child.set_position(current_x, content_y);
                    child.set_size(child_width, content_height);
                    current_x += child_width;
                }
            }
            LayoutType::Absolute => {
                // Children keep their positions
                for child in self.children.values_mut() {
                    let (cx, cy, _, _) = child.bounds();
                    // Ensure children are within container bounds
                    let new_x = content_x + cx.min(content_width);
                    let new_y = content_y + cy.min(content_height);
                    child.set_position(new_x, new_y);
                }
            }
            LayoutType::Grid => {
                // TODO: Implement grid layout
                // For now, use vertical layout
                let child_height = content_height / self.children.len().max(1) as u16;
                let mut current_y = content_y;
                
                for (_, child) in self.children.values_mut() {
                    child.set_position(content_x, current_y);
                    child.set_size(content_width, child_height);
                    current_y += child_height;
                }
            }
        }
    }

    /// Draw border if present
    fn draw_border(&self, renderer: &mut Renderer) -> Result<()> {
        if let Some(ref border_style) = self.border_style {
            let (x, y, width, height) = self.bounds();
            
            // Draw top and bottom borders
            for i in 0..width {
                renderer.draw_text(x + i, y, "─", border_style, Some(1))?;
                renderer.draw_text(x + i, y + height - 1, "─", border_style, Some(1))?;
            }
            
            // Draw left and right borders
            for i in 0..height {
                renderer.draw_text(x, y + i, "│", border_style, Some(1))?;
                renderer.draw_text(x + width - 1, y + i, "│", border_style, Some(1))?;
            }
            
            // Draw corners
            renderer.draw_text(x, y, "┌", border_style, Some(1))?;
            renderer.draw_text(x + width - 1, y, "┐", border_style, Some(1))?;
            renderer.draw_text(x, y + height - 1, "└", border_style, Some(1))?;
            renderer.draw_text(x + width - 1, y + height - 1, "┘", border_style, Some(1))?;
        }
        Ok(())
    }
}

impl Component for Container {
    fn render(&self, renderer: &mut Renderer) -> Result<()> {
        if !self.base.is_visible() {
            return Ok(());
        }

        let (x, y, width, height) = self.bounds();
        
        // Draw container background
        for row in 0..height {
            renderer.draw_rect(x, y + row, width, 1, &self.style)?;
        }
        
        // Draw border if present
        self.draw_border(renderer)?;
        
        // Render children (note: we need a mutable reference to rearrange,
        // but render takes &self, so arrangement should happen in update)
        for child in self.children.values() {
            if child.is_visible() {
                child.render(renderer)?;
            }
        }
        
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        let mut handled = false;
        
        // Pass events to children in reverse order (top to bottom)
        for child in self.children.values_mut().rev() {
            if child.handle_event(event) {
                handled = true;
                break; // Only one component should handle the event
            }
        }
        
        handled
    }

    fn update(&mut self) -> Result<()> {
        // Rearrange children based on layout
        self.arrange_children();
        
        // Update all children
        for child in self.children.values_mut() {
            child.update()?;
        }
        
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
    use crate::components::Label;

    #[test]
    fn test_container_creation() {
        let container = Container::new();
        assert_eq!(container.id(), "container");
        assert_eq!(container.layout, LayoutType::Vertical);
        assert_eq!(container.padding, (1, 1, 1, 1));
    }

    #[test]
    fn test_container_with_id() {
        let container = Container::with_id("my_container");
        assert_eq!(container.id(), "my_container");
    }

    #[test]
    fn test_container_children() {
        let mut container = Container::new();
        let label = Label::new("Test");
        
        container.add_child("label1", label);
        assert_eq!(container.child_names().len(), 1);
        assert!(container.get_child("label1").is_some());
        assert!(container.get_child("nonexistent").is_none());
    }

    #[test]
    fn test_container_styles() {
        let container = Container::new()
            .with_layout(LayoutType::Horizontal)
            .with_uniform_padding(2);
        
        assert_eq!(container.layout, LayoutType::Horizontal);
        assert_eq!(container.padding, (2, 2, 2, 2));
    }
}