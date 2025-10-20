//! Render context for tracking rendering state

use crate::render::Result;
use std::collections::HashMap;

/// Render context for tracking rendering state and providing utilities
#[derive(Debug, Clone)]
pub struct RenderContext {
    /// Terminal width
    width: u16,
    /// Terminal height
    height: u16,
    /// Current component being rendered
    current_component: Option<String>,
    /// Component hierarchy stack
    component_stack: Vec<String>,
    /// Custom properties
    properties: HashMap<String, String>,
    /// Render statistics
    stats: RenderStats,
}

/// Render statistics
#[derive(Debug, Clone, Default)]
pub struct RenderStats {
    /// Number of characters rendered
    pub chars_rendered: usize,
    /// Number of draw calls
    pub draw_calls: usize,
    /// Render time in milliseconds
    pub render_time_ms: u64,
}

impl RenderContext {
    /// Create a new render context
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            current_component: None,
            component_stack: Vec::new(),
            properties: HashMap::new(),
            stats: RenderStats::default(),
        }
    }

    /// Get the terminal width
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Get the terminal height
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Resize the context
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    /// Get the current component being rendered
    pub fn current_component(&self) -> Option<&str> {
        self.current_component.as_deref()
    }

    /// Set the current component
    pub fn set_current_component(&mut self, name: &str) {
        self.current_component = Some(name.to_string());
        self.component_stack.push(name.to_string());
    }

    /// Clear the current component
    pub fn clear_current_component(&mut self) {
        self.current_component = None;
        if !self.component_stack.is_empty() {
            self.component_stack.pop();
        }
    }

    /// Get the component hierarchy
    pub fn component_stack(&self) -> &[String] {
        &self.component_stack
    }

    /// Get the parent component
    pub fn parent_component(&self) -> Option<&str> {
        if self.component_stack.len() >= 2 {
            self.component_stack.get(self.component_stack.len() - 2).map(|s| s.as_str())
        } else {
            None
        }
    }

    /// Check if a point is within bounds
    pub fn is_in_bounds(&self, x: u16, y: u16) -> bool {
        x < self.width && y < self.height
    }

    /// Clamp a point to be within bounds
    pub fn clamp_to_bounds(&self, x: u16, y: u16) -> (u16, u16) {
        let clamped_x = x.min(self.width.saturating_sub(1));
        let clamped_y = y.min(self.height.saturating_sub(1));
        (clamped_x, clamped_y)
    }

    /// Clamp a rectangle to be within bounds
    pub fn clamp_rect_to_bounds(&self, x: u16, y: u16, width: u16, height: u16) -> (u16, u16, u16, u16) {
        let (clamped_x, clamped_y) = self.clamp_to_bounds(x, y);
        let max_width = self.width.saturating_sub(clamped_x);
        let max_height = self.height.saturating_sub(clamped_y);
        let clamped_width = width.min(max_width);
        let clamped_height = height.min(max_height);
        (clamped_x, clamped_y, clamped_width, clamped_height)
    }

    /// Set a custom property
    pub fn set_property(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
    }

    /// Get a custom property
    pub fn get_property(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }

    /// Remove a custom property
    pub fn remove_property(&mut self, key: &str) -> Option<String> {
        self.properties.remove(key)
    }

    /// Get all properties
    pub fn properties(&self) -> &HashMap<String, String> {
        &self.properties
    }

    /// Get mutable properties
    pub fn properties_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.properties
    }

    /// Get the render statistics
    pub fn stats(&self) -> &RenderStats {
        &self.stats
    }

    /// Get mutable render statistics
    pub fn stats_mut(&mut self) -> &mut RenderStats {
        &mut self.stats
    }

    /// Reset the render statistics
    pub fn reset_stats(&mut self) {
        self.stats = RenderStats::default();
    }

    /// Increment the character count
    pub fn increment_chars(&mut self, count: usize) {
        self.stats.chars_rendered += count;
    }

    /// Increment the draw call count
    pub fn increment_draw_calls(&mut self) {
        self.stats.draw_calls += 1;
    }

    /// Set the render time
    pub fn set_render_time(&mut self, time_ms: u64) {
        self.stats.render_time_ms = time_ms;
    }

    /// Check if a component is in the hierarchy
    pub fn is_component_in_hierarchy(&self, name: &str) -> bool {
        self.component_stack.contains(&name.to_string())
    }

    /// Get the depth of the current component hierarchy
    pub fn hierarchy_depth(&self) -> usize {
        self.component_stack.len()
    }

    /// Check if we're rendering inside a specific component
    pub fn is_inside_component(&self, name: &str) -> bool {
        self.component_stack.iter().any(|component| component == name)
    }

    /// Get the path to the current component
    pub fn component_path(&self) -> String {
        self.component_stack.join(" -> ")
    }

    /// Create a sub-context for a child component
    pub fn create_sub_context(&self, child_name: &str) -> Self {
        let mut sub_context = self.clone();
        sub_context.set_current_component(child_name);
        sub_context
    }

    /// Calculate text width considering Unicode characters
    pub fn calculate_text_width(&self, text: &str) -> u16 {
        unicode_width::UnicodeWidthStr::width(text) as u16
    }

    /// Truncate text to fit within a width
    pub fn truncate_text(&self, text: &str, max_width: u16) -> String {
        let text_width = self.calculate_text_width(text);
        if text_width <= max_width {
            return text.to_string();
        }

        // Try to truncate with ellipsis
        let ellipsis_width = self.calculate_text_width("...");
        if max_width <= ellipsis_width {
            return ".".repeat(max_width as usize);
        }

        let available_width = max_width - ellipsis_width;
        let mut truncated = String::new();
        let mut current_width = 0;

        for ch in text.chars() {
            let char_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0) as u16;
            if current_width + char_width > available_width {
                break;
            }
            truncated.push(ch);
            current_width += char_width;
        }

        truncated + "..."
    }

    /// Wrap text to fit within a width
    pub fn wrap_text(&self, text: &str, max_width: u16) -> Vec<String> {
        if max_width == 0 {
            return vec![];
        }

        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0;

        for word in text.split_whitespace() {
            let word_width = self.calculate_text_width(word);
            
            if current_width == 0 {
                current_line = word.to_string();
                current_width = word_width;
            } else if current_width + 1 + word_width <= max_width {
                current_line.push(' ');
                current_line.push_str(word);
                current_width += 1 + word_width;
            } else {
                lines.push(current_line);
                current_line = word.to_string();
                current_width = word_width;
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }
}

impl Default for RenderContext {
    fn default() -> Self {
        Self::new(80, 24) // Default terminal size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_context_creation() {
        let context = RenderContext::new(80, 24);
        assert_eq!(context.width(), 80);
        assert_eq!(context.height(), 24);
        assert!(context.current_component().is_none());
        assert_eq!(context.hierarchy_depth(), 0);
    }

    #[test]
    fn test_component_hierarchy() {
        let mut context = RenderContext::new(80, 24);
        
        context.set_current_component("root");
        assert_eq!(context.current_component(), Some("root"));
        assert_eq!(context.hierarchy_depth(), 1);
        
        context.set_current_component("child");
        assert_eq!(context.current_component(), Some("child"));
        assert_eq!(context.hierarchy_depth(), 2);
        assert_eq!(context.parent_component(), Some("root"));
        
        context.clear_current_component();
        assert_eq!(context.current_component(), Some("root"));
        assert_eq!(context.hierarchy_depth(), 1);
    }

    #[test]
    fn test_bounds_checking() {
        let context = RenderContext::new(80, 24);
        
        assert!(context.is_in_bounds(0, 0));
        assert!(context.is_in_bounds(79, 23));
        assert!(!context.is_in_bounds(80, 24));
        assert!(!context.is_in_bounds(100, 100));
        
        let (x, y) = context.clamp_to_bounds(100, 100);
        assert_eq!(x, 79);
        assert_eq!(y, 23);
    }

    #[test]
    fn test_properties() {
        let mut context = RenderContext::new(80, 24);
        
        context.set_property("theme", "dark");
        assert_eq!(context.get_property("theme"), Some("dark"));
        
        let removed = context.remove_property("theme");
        assert_eq!(removed, Some("dark".to_string()));
        assert_eq!(context.get_property("theme"), None);
    }

    #[test]
    fn test_text_operations() {
        let context = RenderContext::new(80, 24);
        
        assert_eq!(context.calculate_text_width("hello"), 5);
        assert_eq!(context.calculate_text_width("こんにちは"), 5); // Japanese characters
        
        let truncated = context.truncate_text("hello world", 8);
        assert_eq!(truncated, "hello...");
        
        let wrapped = context.wrap_text("hello world this is a long text", 10);
        assert!(wrapped.len() > 1);
    }

    #[test]
    fn test_stats() {
        let mut context = RenderContext::new(80, 24);
        
        context.increment_chars(10);
        context.increment_draw_calls();
        context.set_render_time(16);
        
        assert_eq!(context.stats().chars_rendered, 10);
        assert_eq!(context.stats().draw_calls, 1);
        assert_eq!(context.stats().render_time_ms, 16);
        
        context.reset_stats();
        assert_eq!(context.stats().chars_rendered, 0);
        assert_eq!(context.stats().draw_calls, 0);
        assert_eq!(context.stats().render_time_ms, 0);
    }
}