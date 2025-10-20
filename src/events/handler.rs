//! Event handler for processing and dispatching events

use crate::events::{Event, Result};
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

/// Event handler that processes and dispatches events
pub struct EventHandler {
    /// Event receiver
    receiver: Receiver<Event>,
    /// Event sender for internal use
    sender: Sender<Event>,
    /// Event listeners
    listeners: HashMap<String, Vec<Box<dyn EventListener>>>,
    /// Event filters
    filters: Vec<Box<dyn EventFilter>>,
}

/// Trait for event listeners
pub trait EventListener: Send {
    /// Handle an event
    fn handle_event(&mut self, event: &Event) -> bool;
}

/// Trait for event filters
pub trait EventFilter: Send {
    /// Check if an event should be processed
    fn should_process(&self, event: &Event) -> bool;
}

impl EventHandler {
    /// Create a new event handler
    pub fn new() -> Result<Self> {
        let (sender, receiver) = mpsc::channel();
        
        Ok(Self {
            receiver,
            sender,
            listeners: HashMap::new(),
            filters: Vec::new(),
        })
    }

    /// Add an event listener
    pub fn add_listener<L: EventListener + 'static>(&mut self, event_type: &str, listener: L) {
        self.listeners
            .entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(listener));
    }

    /// Remove all listeners for an event type
    pub fn remove_listeners(&mut self, event_type: &str) {
        self.listeners.remove(event_type);
    }

    /// Add an event filter
    pub fn add_filter<F: EventFilter + 'static>(&mut self, filter: F) {
        self.filters.push(Box::new(filter));
    }

    /// Get the next event
    pub fn next_event(&mut self) -> Result<Option<Event>> {
        match self.receiver.recv_timeout(Duration::from_millis(0)) {
            Ok(event) => Ok(Some(event)),
            Err(mpsc::RecvTimeoutError::Timeout) => Ok(None),
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                Err(crate::events::Error::Channel("Event channel disconnected".to_string()))
            }
        }
    }

    /// Process a single event
    pub fn process_event(&mut self, event: Event) -> Result<bool> {
        // Apply filters
        for filter in &self.filters {
            if !filter.should_process(&event) {
                return Ok(false);
            }
        }

        // Determine event type
        let event_type = match &event {
            Event::KeyPress { .. } => "key_press",
            Event::KeyRelease { .. } => "key_release",
            Event::MousePress { .. } => "mouse_press",
            Event::MouseRelease { .. } => "mouse_release",
            Event::MouseClick { .. } => "mouse_click",
            Event::MouseMove { .. } => "mouse_move",
            Event::MouseScroll { .. } => "mouse_scroll",
            Event::Resize { .. } => "resize",
            Event::FocusGained => "focus_gained",
            Event::FocusLost => "focus_lost",
            Event::Quit => "quit",
            Event::Timer { .. } => "timer",
            Event::Custom { event_type, .. } => event_type,
        };

        // Notify listeners
        let mut handled = false;
        if let Some(listeners) = self.listeners.get_mut(event_type) {
            for listener in listeners {
                if listener.handle_event(&event) {
                    handled = true;
                    break; // Only one listener should handle the event
                }
            }
        }

        Ok(handled)
    }

    /// Process all pending events
    pub fn process_events(&mut self) -> Result<usize> {
        let mut count = 0;
        while let Some(event) = self.next_event()? {
            self.process_event(event)?;
            count += 1;
        }
        Ok(count)
    }

    /// Send an event
    pub fn send_event(&self, event: Event) -> Result<()> {
        self.sender.send(event).map_err(|e| {
            crate::events::Error::Channel(format!("Failed to send event: {}", e))
        })
    }

    /// Start the event processing loop in a separate thread
    pub fn start_processing_thread(mut self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            loop {
                if let Ok(count) = self.process_events() {
                    // Sleep briefly if no events were processed
                    if count == 0 {
                        thread::sleep(Duration::from_millis(1));
                    }
                }
            }
        })
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new().expect("Failed to create EventHandler")
    }
}

/// Simple event listener that calls a closure
pub struct ClosureListener {
    closure: Box<dyn FnMut(&Event) -> bool + Send>,
}

impl ClosureListener {
    /// Create a new closure listener
    pub fn new<F: FnMut(&Event) -> bool + Send + 'static>(closure: F) -> Self {
        Self {
            closure: Box::new(closure),
        }
    }
}

impl EventListener for ClosureListener {
    fn handle_event(&mut self, event: &Event) -> bool {
        (self.closure)(event)
    }
}

/// Event filter that only allows specific event types
pub struct EventTypeFilter {
    allowed_types: Vec<String>,
}

impl EventTypeFilter {
    /// Create a new event type filter
    pub fn new<T: Into<String>>(types: Vec<T>) -> Self {
        Self {
            allowed_types: types.into_iter().map(|t| t.into()).collect(),
        }
    }
}

impl EventFilter for EventTypeFilter {
    fn should_process(&self, event: &Event) -> bool {
        let event_type = match &event {
            Event::KeyPress { .. } => "key_press",
            Event::KeyRelease { .. } => "key_release",
            Event::MousePress { .. } => "mouse_press",
            Event::MouseRelease { .. } => "mouse_release",
            Event::MouseClick { .. } => "mouse_click",
            Event::MouseMove { .. } => "mouse_move",
            Event::MouseScroll { .. } => "mouse_scroll",
            Event::Resize { .. } => "resize",
            Event::FocusGained => "focus_gained",
            Event::FocusLost => "focus_lost",
            Event::Quit => "quit",
            Event::Timer { .. } => "timer",
            Event::Custom { event_type, .. } => event_type,
        };

        self.allowed_types.contains(&event_type.to_string())
    }
}

/// Event filter that blocks specific event types
pub struct EventBlockFilter {
    blocked_types: Vec<String>,
}

impl EventBlockFilter {
    /// Create a new event block filter
    pub fn new<T: Into<String>>(types: Vec<T>) -> Self {
        Self {
            blocked_types: types.into_iter().map(|t| t.into()).collect(),
        }
    }
}

impl EventFilter for EventBlockFilter {
    fn should_process(&self, event: &Event) -> bool {
        let event_type = match &event {
            Event::KeyPress { .. } => "key_press",
            Event::KeyRelease { .. } => "key_release",
            Event::MousePress { .. } => "mouse_press",
            Event::MouseRelease { .. } => "mouse_release",
            Event::MouseClick { .. } => "mouse_click",
            Event::MouseMove { .. } => "mouse_move",
            Event::MouseScroll { .. } => "mouse_scroll",
            Event::Resize { .. } => "resize",
            Event::FocusGained => "focus_gained",
            Event::FocusLost => "focus_lost",
            Event::Quit => "quit",
            Event::Timer { .. } => "timer",
            Event::Custom { event_type, .. } => event_type,
        };

        !self.blocked_types.contains(&event_type.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::Key;

    #[test]
    fn test_event_handler_creation() {
        let handler = EventHandler::new();
        assert!(handler.is_ok());
    }

    #[test]
    fn test_closure_listener() {
        let mut listener = ClosureListener::new(|event| {
            matches!(event, Event::KeyPress { .. })
        });

        let key_event = Event::KeyPress {
            key: Key::Char('a'),
            modifiers: crate::events::Modifiers::default(),
        };
        let mouse_event = Event::MouseClick {
            button: crate::events::MouseButton::Left,
            x: 0,
            y: 0,
            modifiers: crate::events::Modifiers::default(),
        };

        assert!(listener.handle_event(&key_event));
        assert!(!listener.handle_event(&mouse_event));
    }

    #[test]
    fn test_event_filters() {
        let allow_filter = EventTypeFilter::new(vec!["key_press", "mouse_click"]);
        let block_filter = EventBlockFilter::new(vec!["mouse_move"]);

        let key_event = Event::KeyPress {
            key: Key::Char('a'),
            modifiers: crate::events::Modifiers::default(),
        };
        let mouse_event = Event::MouseClick {
            button: crate::events::MouseButton::Left,
            x: 0,
            y: 0,
            modifiers: crate::events::Modifiers::default(),
        };
        let move_event = Event::MouseMove {
            x: 0,
            y: 0,
            modifiers: crate::events::Modifiers::default(),
        };

        assert!(allow_filter.should_process(&key_event));
        assert!(allow_filter.should_process(&mouse_event));
        assert!(!allow_filter.should_process(&move_event));

        assert!(block_filter.should_process(&key_event));
        assert!(block_filter.should_process(&mouse_event));
        assert!(!block_filter.should_process(&move_event));
    }
}