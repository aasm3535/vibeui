//! UI Components module
//!
//! This module contains all UI components that can be used to build
//! terminal user interfaces.

pub mod button;
pub mod label;
pub mod text_input;
pub mod container;
pub mod layout;

pub use button::Button;
pub use label::Label;
pub use text_input::TextInput;
pub use container::Container;
pub use layout::Layout;

/// Component trait that all UI components must implement
pub trait Component {
    /// Render the component
    fn render(&self, renderer: &mut crate::render::Renderer) -> crate::app::Result<()>;
    
    /// Handle an event
    fn handle_event(&mut self, event: &crate::events::Event) -> bool;
    
    /// Update the component state
    fn update(&mut self) -> crate::app::Result<()>;
    
    /// Get the component's position and size
    fn bounds(&self) -> (u16, u16, u16, u16); // x, y, width, height
    
    /// Set the component's position
    fn set_position(&mut self, x: u16, y: u16);
    
    /// Set the component's size
    fn set_size(&mut self, width: u16, height: u16);
    
    /// Check if the component is visible
    fn is_visible(&self) -> bool;
    
    /// Set the component's visibility
    fn set_visible(&mut self, visible: bool);
    
    /// Get the component's unique ID
    fn id(&self) -> &str;
}

/// Base component structure that provides common functionality
#[derive(Debug, Clone)]
pub struct BaseComponent {
    id: String,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    visible: bool,
}

impl BaseComponent {
    /// Create a new base component
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            x: 0,
            y: 0,
            width: 10,
            height: 1,
            visible: true,
        }
    }

    /// Get the component ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the component position
    pub fn position(&self) -> (u16, u16) {
        (self.x, self.y)
    }

    /// Set the component position
    pub fn set_position(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    /// Get the component size
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    /// Set the component size
    pub fn set_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    /// Check if the component is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Set the component visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_component() {
        let mut component = BaseComponent::new("test");
        
        assert_eq!(component.id(), "test");
        assert_eq!(component.position(), (0, 0));
        assert_eq!(component.size(), (10, 1));
        assert!(component.is_visible());
        
        component.set_position(5, 10);
        component.set_size(20, 5);
        component.set_visible(false);
        
        assert_eq!(component.position(), (5, 10));
        assert_eq!(component.size(), (20, 5));
        assert!(!component.is_visible());
    }
}