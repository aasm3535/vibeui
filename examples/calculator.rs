//! Simple calculator example using VibeUI
//!
//! This example demonstrates a basic calculator interface with buttons
//! for numbers and operations.

use vibeui::{App, Component, Label, Button, Container, Style, Color, Result};
use vibeui::components::LayoutType;

fn main() -> Result<()> {
    // Initialize the library
    vibeui::init()?;
    
    println!("Starting VibeUI Calculator Example...");
    println!("Press ESC or Ctrl+C to exit");
    
    // Create the application
    let mut app = App::new()?;
    
    // Create main container
    let mut main_container = Container::with_id("main")
        .with_layout(LayoutType::Vertical)
        .with_border(Style::default().with_foreground(Color::Cyan))
        .with_uniform_padding(1);
    
    // Add title
    main_container.add_child("title", Label::with_id("title", "🧮 VibeUI Calculator")
        .with_color(Color::Cyan)
        .with_bold(true));
    
    // Add display
    let mut display = Label::with_id("display", "0")
        .with_color(Color::White)
        .with_background(Color::Black)
        .with_bold(true);
    main_container.add_child("display", display);
    
    // Create button grid container
    let mut grid_container = Container::with_id("grid")
        .with_layout(LayoutType::Vertical)
        .with_uniform_padding(0);
    
    // First row: 7, 8, 9, /
    let mut row1 = Container::with_id("row1")
        .with_layout(LayoutType::Horizontal)
        .with_uniform_padding(0);
    
    row1.add_child("btn7", create_calc_button("7", Color::White));
    row1.add_child("btn8", create_calc_button("8", Color::White));
    row1.add_child("btn9", create_calc_button("9", Color::White));
    row1.add_child("btn_div", create_calc_button("/", Color::Yellow));
    
    // Second row: 4, 5, 6, *
    let mut row2 = Container::with_id("row2")
        .with_layout(LayoutType::Horizontal)
        .with_uniform_padding(0);
    
    row2.add_child("btn4", create_calc_button("4", Color::White));
    row2.add_child("btn5", create_calc_button("5", Color::White));
    row2.add_child("btn6", create_calc_button("6", Color::White));
    row2.add_child("btn_mul", create_calc_button("*", Color::Yellow));
    
    // Third row: 1, 2, 3, -
    let mut row3 = Container::with_id("row3")
        .with_layout(LayoutType::Horizontal)
        .with_uniform_padding(0);
    
    row3.add_child("btn1", create_calc_button("1", Color::White));
    row3.add_child("btn2", create_calc_button("2", Color::White));
    row3.add_child("btn3", create_calc_button("3", Color::White));
    row3.add_child("btn_sub", create_calc_button("-", Color::Yellow));
    
    // Fourth row: 0, C, =, +
    let mut row4 = Container::with_id("row4")
        .with_layout(LayoutType::Horizontal)
        .with_uniform_padding(0);
    
    row4.add_child("btn0", create_calc_button("0", Color::White));
    row4.add_child("btn_clear", create_calc_button("C", Color::Red));
    row4.add_child("btn_eq", create_calc_button("=", Color::Green));
    row4.add_child("btn_add", create_calc_button("+", Color::Yellow));
    
    // Add rows to grid
    grid_container.add_child("row1", row1);
    grid_container.add_child("row2", row2);
    grid_container.add_child("row3", row3);
    grid_container.add_child("row4", row4);
    
    main_container.add_child("grid", grid_container);
    
    // Add status
    main_container.add_child("status", Label::with_id("status", "Ready to calculate!")
        .with_color(Color::Green));
    
    // Add the main container to the app
    app.add_component("main", main_container);
    
    // Run the application
    println!("Running calculator...");
    match app.run() {
        Ok(_) => println!("Calculator exited successfully"),
        Err(e) => eprintln!("Calculator error: {}", e),
    }
    
    Ok(())
}

/// Create a calculator button with consistent styling
fn create_calc_button(text: &str, color: Color) -> Button {
    Button::with_id(format!("btn_{}", text), text)
        .with_color(color)
        .with_bold(true)
        .on_click(move |button| {
            // In a real calculator, this would update the display and perform calculations
            println!("Calculator button '{}' clicked!", text);
        })
}

/// Advanced calculator with state management
pub mod advanced {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, Clone)]
    enum CalcOperation {
        None,
        Add,
        Subtract,
        Multiply,
        Divide,
    }

    struct CalculatorState {
        display: String,
        current_value: f64,
        previous_value: f64,
        operation: CalcOperation,
        new_input: bool,
    }

    impl CalculatorState {
        fn new() -> Self {
            Self {
                display: "0".to_string(),
                current_value: 0.0,
                previous_value: 0.0,
                operation: CalcOperation::None,
                new_input: true,
            }
        }

        fn input_digit(&mut self, digit: char) {
            if self.new_input || self.display == "0" {
                self.display = digit.to_string();
                self.new_input = false;
            } else if self.display.len() < 10 {
                self.display.push(digit);
            }
            self.current_value = self.display.parse().unwrap_or(0.0);
        }

        fn input_decimal(&mut self) {
            if self.new_input {
                self.display = "0.".to_string();
                self.new_input = false;
            } else if !self.display.contains('.') {
                self.display.push('.');
            }
        }

        fn clear(&mut self) {
            self.display = "0".to_string();
            self.current_value = 0.0;
            self.previous_value = 0.0;
            self.operation = CalcOperation::None;
            self.new_input = true;
        }

        fn set_operation(&mut self, op: CalcOperation) {
            if !self.new_input {
                self.calculate();
            }
            self.previous_value = self.current_value;
            self.operation = op;
            self.new_input = true;
        }

        fn calculate(&mut self) {
            if self.operation != CalcOperation::None {
                let result = match self.operation {
                    CalcOperation::Add => self.previous_value + self.current_value,
                    CalcOperation::Subtract => self.previous_value - self.current_value,
                    CalcOperation::Multiply => self.previous_value * self.current_value,
                    CalcOperation::Divide => {
                        if self.current_value != 0.0 {
                            self.previous_value / self.current_value
                        } else {
                            0.0
                        }
                    }
                    CalcOperation::None => self.current_value,
                };
                
                self.display = format!("{:.10}", result).trim_end_matches('0').trim_end_matches('.').to_string();
                self.current_value = result;
                self.operation = CalcOperation::None;
                self.new_input = true;
            }
        }
    }

    pub fn run_advanced_calculator() -> Result<()> {
        vibeui::init()?;
        
        let state = Rc::new(RefCell::new(CalculatorState::new()));
        
        let mut app = App::new()?;
        
        // ... implementation would go here ...
        
        println!("Advanced calculator not yet fully implemented");
        println!("Use the basic calculator for now.");
        
        Ok(())
    }
}