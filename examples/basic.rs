//! Basic VibeUI example application
//!
//! This example demonstrates the basic usage of VibeUI with a simple
//! interface containing labels, buttons, and input fields.

use vibeui::{App, Component, Label, Button, TextInput, Container, Style, Color, Result};
use vibeui::components::LayoutType;

fn main() -> Result<()> {
    // Initialize the library
    vibeui::init()?;
    
    println!("Starting VibeUI Basic Example...");
    println!("Press ESC or Ctrl+C to exit");
    
    // Create the application
    let mut app = App::new()?;
    
    // Create main container with border
    let mut main_container = Container::with_id("main")
        .with_layout(LayoutType::Vertical)
        .with_border(Style::default().with_foreground(Color::Blue))
        .with_uniform_padding(2);
    
    // Add title
    main_container.add_child("title", Label::with_id("title", "🎨 VibeUI Basic Example")
        .with_color(Color::Cyan)
        .with_bold(true));
    
    // Add separator
    main_container.add_child("separator1", Label::with_id("separator1", "─".repeat(40)));
    
    // Add description
    main_container.add_child("description", Label::with_id("description", 
        "This is a basic example of VibeUI library.\n\
        Try interacting with the components below!")
        .with_color(Color::White));
    
    // Add input field
    let mut input_field = TextInput::with_id("name_input")
        .with_placeholder("Enter your name...")
        .with_max_length(20);
    input_field.set_text("VibeUI User");
    main_container.add_child("name_input", input_field);
    
    // Add buttons container
    let mut button_container = Container::with_id("buttons")
        .with_layout(LayoutType::Horizontal)
        .with_uniform_padding(1);
    
    // Add buttons
    button_container.add_child("button1", Button::with_id("button1", "Click Me!")
        .on_click(|_button| {
            println!("Button 1 clicked! 🎉");
        }));
    
    button_container.add_child("button2", Button::with_id("button2", "Exit")
        .on_click(|_button| {
            println!("Exit button clicked!");
            // In a real app, you'd set a flag to stop the app
        }));
    
    main_container.add_child("buttons", button_container);
    
    // Add another separator
    main_container.add_child("separator2", Label::with_id("separator2", "─".repeat(40)));
    
    // Add status label
    let mut status_label = Label::with_id("status", "Status: Ready")
        .with_color(Color::Green);
    main_container.add_child("status", status_label);
    
    // Add instructions
    main_container.add_child("instructions", Label::with_id("instructions", 
        "Instructions:\n\
        • Type in the input field\n\
        • Click the buttons\n\
        • Press ESC to exit")
        .with_color(Color::Gray)
        .with_dim(true));
    
    // Add the main container to the app
    app.add_component("main", main_container);
    
    // Run the application
    println!("Running application...");
    match app.run() {
        Ok(_) => println!("Application exited successfully"),
        Err(e) => eprintln!("Application error: {}", e),
    }
    
    Ok(())
}

/// Alternative simple example that just shows a hello world message
pub fn hello_world() -> Result<()> {
    // Initialize the library
    vibeui::init()?;
    
    // Create a simple app with just a label
    let mut app = App::new()?;
    
    let hello_label = Label::with_id("hello", "👋 Hello, VibeUI World!")
        .with_color(Color::Green)
        .with_bold(true);
    
    app.add_component("hello", hello_label);
    
    // Run the app
    app.run()?;
    
    Ok(())
}