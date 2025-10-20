//! Main application structure

use crate::app::{Config, Result};
use crate::events::{Event, EventHandler};
use crate::platform::Terminal;
use crate::render::Renderer;
use crate::components::Component;
use std::collections::HashMap;

/// Main application structure
pub struct App {
    config: Config,
    terminal: Terminal,
    renderer: Renderer,
    event_handler: EventHandler,
    components: HashMap<String, Box<dyn Component>>,
    running: bool,
}

impl App {
    /// Create a new application with default configuration
    pub fn new() -> Result<Self> {
        let config = Config::default();
        Self::with_config(config)
    }

    /// Create a new application with custom configuration
    pub fn with_config(config: Config) -> Result<Self> {
        let terminal = Terminal::new()?;
        let renderer = Renderer::new(&terminal);
        let event_handler = EventHandler::new()?;

        Ok(Self {
            config,
            terminal,
            renderer,
            event_handler,
            components: HashMap::new(),
            running: false,
        })
    }

    /// Add a component to the application
    pub fn add_component<C: Component + 'static>(&mut self, name: &str, component: C) {
        self.components.insert(name.to_string(), Box::new(component));
    }

    /// Get a component by name
    pub fn get_component(&self, name: &str) -> Option<&dyn Component> {
        self.components.get(name).map(|c| c.as_ref())
    }

    /// Get a mutable component by name
    pub fn get_component_mut(&mut self, name: &str) -> Option<&mut dyn Component> {
        self.components.get_mut(name).map(|c| c.as_mut())
    }

    /// Run the application
    pub fn run(&mut self) -> Result<()> {
        self.running = true;
        self.terminal.initialize()?;
        
        while self.running {
            // Handle events
            while let Some(event) = self.event_handler.next_event()? {
                self.handle_event(event)?;
            }

            // Render components
            self.render()?;

            // Update components
            self.update()?;

            // Sleep for a bit to prevent high CPU usage
            std::thread::sleep(std::time::Duration::from_millis(16));
        }

        self.terminal.cleanup()?;
        Ok(())
    }

    /// Stop the application
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Handle an event
    fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Quit => self.stop(),
            _ => {
                // Pass event to all components
                for component in self.components.values_mut() {
                    component.handle_event(&event);
                }
            }
        }
        Ok(())
    }

    /// Render all components
    fn render(&mut self) -> Result<()> {
        self.renderer.clear()?;
        
        for (name, component) in &self.components {
            self.renderer.render_component(name, component.as_ref())?;
        }
        
        self.renderer.present()?;
        Ok(())
    }

    /// Update all components
    fn update(&mut self) -> Result<()> {
        for component in self.components.values_mut() {
            component.update()?;
        }
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new().expect("Failed to create App")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert!(app.is_ok());
    }
}