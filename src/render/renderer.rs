//! Main renderer for VibeUI

use crate::platform::Terminal;
use crate::render::{RenderBuffer, RenderContext, Result};
use crate::style::Style;
use crossterm::{
    execute, queue,
    style::{Color as CrosstermColor, Print, SetForegroundColor, SetBackgroundColor, 
            SetAttribute, Attribute, ResetColor},
    cursor::MoveTo,
    terminal::ClearType,
    Command,
};
use std::io::{stdout, Write};

/// Main renderer for drawing UI components
#[derive(Debug)]
pub struct Renderer {
    terminal: Terminal,
    buffer: RenderBuffer,
    context: RenderContext,
}

impl Renderer {
    /// Create a new renderer
    pub fn new(terminal: &Terminal) -> Self {
        Self {
            terminal: Terminal::new().expect("Failed to create terminal"),
            buffer: RenderBuffer::new(terminal.width(), terminal.height()),
            context: RenderContext::new(terminal.width(), terminal.height()),
        }
    }

    /// Get the terminal width
    pub fn width(&self) -> u16 {
        self.terminal.width()
    }

    /// Get the terminal height
    pub fn height(&self) -> u16 {
        self.terminal.height()
    }

    /// Update the renderer size
    pub fn update_size(&mut self) -> Result<()> {
        self.terminal.update_size()?;
        self.buffer.resize(self.terminal.width(), self.terminal.height());
        self.context.resize(self.terminal.width(), self.terminal.height());
        Ok(())
    }

    /// Clear the screen
    pub fn clear(&mut self) -> Result<()> {
        self.buffer.clear();
        Ok(())
    }

    /// Present the buffer to the screen
    pub fn present(&mut self) -> Result<()> {
        // Move cursor to top-left corner
        execute!(stdout(), MoveTo(0, 0))?;
        
        // Render the buffer to stdout
        self.buffer.render_to_terminal()?;
        
        // Flush output
        stdout().flush()?;
        
        Ok(())
    }

    /// Draw text at the specified position
    pub fn draw_text(&mut self, x: u16, y: u16, text: &str, style: &Style, max_width: Option<u16>) -> Result<()> {
        let text = if let Some(max_width) = max_width {
            let text_width = text.chars().count() as u16;
            if text_width > max_width {
                // Truncate text with ellipsis
                let truncate_len = (max_width.saturating_sub(3)) as usize;
                let truncated: String = text.chars().take(truncate_len).collect();
                format!("{}...", truncated)
            } else {
                text.to_string()
            }
        } else {
            text.to_string()
        };

        self.buffer.draw_text(x, y, &text, style)
    }

    /// Draw a rectangle
    pub fn draw_rect(&mut self, x: u16, y: u16, width: u16, height: u16, style: &Style) -> Result<()> {
        for row in 0..height {
            for col in 0..width {
                let buffer_x = x + col;
                let buffer_y = y + row;
                if buffer_x < self.buffer.width() && buffer_y < self.buffer.height() {
                    self.buffer.draw_char(buffer_x, buffer_y, ' ', style)?;
                }
            }
        }
        Ok(())
    }

    /// Draw a border
    pub fn draw_border(&mut self, x: u16, y: u16, width: u16, height: u16, style: &Style) -> Result<()> {
        if width < 2 || height < 2 {
            return Ok(());
        }

        // Draw corners
        self.buffer.draw_char(x, y, '┌', style)?;
        self.buffer.draw_char(x + width - 1, y, '┐', style)?;
        self.buffer.draw_char(x, y + height - 1, '└', style)?;
        self.buffer.draw_char(x + width - 1, y + height - 1, '┘', style)?;

        // Draw horizontal lines
        for col in 1..width - 1 {
            self.buffer.draw_char(x + col, y, '─', style)?;
            self.buffer.draw_char(x + col, y + height - 1, '─', style)?;
        }

        // Draw vertical lines
        for row in 1..height - 1 {
            self.buffer.draw_char(x, y + row, '│', style)?;
            self.buffer.draw_char(x + width - 1, y + row, '│', style)?;
        }

        Ok(())
    }

    /// Draw a line
    pub fn draw_line(&mut self, x1: u16, y1: u16, x2: u16, y2: u16, style: &Style) -> Result<()> {
        // Simple line drawing using Bresenham's algorithm
        let dx = if x2 >= x1 { x2 - x1 } else { x1 - x2 };
        let dy = if y2 >= y1 { y2 - y1 } else { y1 - y2 };
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx as i32 - dy as i32;

        let mut x = x1 as i32;
        let mut y = y1 as i32;
        let x2 = x2 as i32;
        let y2 = y2 as i32;

        loop {
            if x >= 0 && y >= 0 && x < self.buffer.width() as i32 && y < self.buffer.height() as i32 {
                self.buffer.draw_char(x as u16, y as u16, '•', style)?;
            }

            if x == x2 && y == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy as i32 {
                err -= dy as i32;
                x += sx;
            }
            if e2 < dx as i32 {
                err += dx as i32;
                y += sy;
            }
        }

        Ok(())
    }

    /// Fill an area with a character
    pub fn fill_area(&mut self, x: u16, y: u16, width: u16, height: u16, char: char, style: &Style) -> Result<()> {
        for row in 0..height {
            for col in 0..width {
                let buffer_x = x + col;
                let buffer_y = y + row;
                if buffer_x < self.buffer.width() && buffer_y < self.buffer.height() {
                    self.buffer.draw_char(buffer_x, buffer_y, char, style)?;
                }
            }
        }
        Ok(())
    }

    /// Render a component
    pub fn render_component(&mut self, name: &str, component: &dyn crate::components::Component) -> Result<()> {
        // Set component name in context
        self.context.set_current_component(name);
        
        // Render the component
        component.render(self)?;
        
        // Clear current component
        self.context.clear_current_component();
        
        Ok(())
    }

    /// Get the render context
    pub fn context(&self) -> &RenderContext {
        &self.context
    }

    /// Get a mutable render context
    pub fn context_mut(&mut self) -> &mut RenderContext {
        &mut self.context
    }

    /// Get the render buffer
    pub fn buffer(&self) -> &RenderBuffer {
        &self.buffer
    }

    /// Get a mutable render buffer
    pub fn buffer_mut(&mut self) -> &mut RenderBuffer {
        &mut self.buffer
    }

    /// Convert VibeUI style to crossterm commands
    pub fn style_to_commands(style: &Style) -> Vec<Box<dyn Command>> {
        let mut commands = Vec::new();

        // Set foreground color
        if let Some(fg) = style.foreground {
            commands.push(Box::new(SetForegroundColor(Self::color_to_crossterm(fg))));
        }

        // Set background color
        if let Some(bg) = style.background {
            commands.push(Box::new(SetBackgroundColor(Self::color_to_crossterm(bg))));
        }

        // Set attributes
        if style.bold {
            commands.push(Box::new(SetAttribute(Attribute::Bold)));
        }
        if style.italic {
            commands.push(Box::new(SetAttribute(Attribute::Italic)));
        }
        if style.underline {
            commands.push(Box::new(SetAttribute(Attribute::Underlined)));
        }
        if style.dim {
            commands.push(Box::new(SetAttribute(Attribute::Dim)));
        }
        if style.blink {
            commands.push(Box::new(SetAttribute(Attribute::SlowBlink)));
        }
        if style.reverse {
            commands.push(Box::new(SetAttribute(Attribute::Reverse)));
        }
        if style.hidden {
            commands.push(Box::new(SetAttribute(Attribute::Hidden)));
        }
        if style.strikethrough {
            commands.push(Box::new(SetAttribute(Attribute::CrossedOut)));
        }

        commands
    }

    /// Convert VibeUI color to crossterm color
    fn color_to_crossterm(color: crate::style::Color) -> CrosstermColor {
        match color {
            crate::style::Color::Black => CrosstermColor::Black,
            crate::style::Color::Red => CrosstermColor::Red,
            crate::style::Color::Green => CrosstermColor::Green,
            crate::style::Color::Yellow => CrosstermColor::Yellow,
            crate::style::Color::Blue => CrosstermColor::Blue,
            crate::style::Color::Magenta => CrosstermColor::Magenta,
            crate::style::Color::Cyan => CrosstermColor::Cyan,
            crate::style::Color::White => CrosstermColor::White,
            crate::style::Color::Rgb(r, g, b) => CrosstermColor::Rgb { r, g, b },
            crate::style::Color::AnsiValue(v) => CrosstermColor::AnsiValue(v),
        }
    }
}

impl Default for Renderer {
    fn default() -> Self {
        let terminal = Terminal::new().expect("Failed to create terminal");
        Self::new(&terminal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{Style, Color};

    #[test]
    fn test_renderer_creation() {
        let renderer = Renderer::default();
        assert!(renderer.width() > 0);
        assert!(renderer.height() > 0);
    }

    #[test]
    fn test_style_conversion() {
        let style = Style::default()
            .with_foreground(Color::Red)
            .with_background(Color::Blue)
            .with_bold(true);
        
        let commands = Renderer::style_to_commands(&style);
        assert!(!commands.is_empty());
    }

    #[test]
    fn test_color_conversion() {
        let crossterm_red = Renderer::color_to_crossterm(Color::Red);
        assert_eq!(crossterm_red, CrosstermColor::Red);
        
        let crossterm_rgb = Renderer::color_to_crossterm(Color::Rgb(255, 0, 0));
        if let CrosstermColor::Rgb { r, g, b } = crossterm_rgb {
            assert_eq!(r, 255);
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        } else {
            panic!("Expected RGB color");
        }
    }
}