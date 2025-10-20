//! Render buffer for double buffering

use crate::render::{Result, Renderer};
use crate::style::Style;
use crossterm::{
    execute, queue,
    style::{Print, ResetColor},
    cursor::MoveTo,
    Command,
};
use std::io::{stdout, Write};

/// A cell in the render buffer
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BufferCell {
    /// Character to display
    pub ch: char,
    /// Style of the character
    pub style: Style,
    /// Whether this cell has been modified
    pub dirty: bool,
}

impl Default for BufferCell {
    fn default() -> Self {
        Self {
            ch: ' ',
            style: Style::default(),
            dirty: false,
        }
    }
}

/// Render buffer for double buffering
#[derive(Debug)]
pub struct RenderBuffer {
    /// Buffer width
    width: u16,
    /// Buffer height
    height: u16,
    /// Buffer data
    cells: Vec<BufferCell>,
    /// Previous buffer state for diffing
    prev_cells: Vec<BufferCell>,
}

impl RenderBuffer {
    /// Create a new render buffer
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        let cells = vec![BufferCell::default(); size];
        let prev_cells = vec![BufferCell::default(); size];
        
        Self {
            width,
            height,
            cells,
            prev_cells,
        }
    }

    /// Get the buffer width
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Get the buffer height
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Resize the buffer
    pub fn resize(&mut self, width: u16, height: u16) {
        if width == self.width && height == self.height {
            return;
        }

        let new_size = (width as usize) * (height as usize);
        let mut new_cells = vec![BufferCell::default(); new_size];
        let mut new_prev_cells = vec![BufferCell::default(); new_size];

        // Copy existing content
        let min_width = self.width.min(width);
        let min_height = self.height.min(height);
        
        for y in 0..min_height {
            for x in 0..min_width {
                let old_index = (y * self.width + x) as usize;
                let new_index = (y * width + x) as usize;
                new_cells[new_index] = self.cells[old_index];
                new_prev_cells[new_index] = self.prev_cells[old_index];
            }
        }

        self.width = width;
        self.height = height;
        self.cells = new_cells;
        self.prev_cells = new_prev_cells;
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = BufferCell::default();
        }
    }

    /// Get a cell at the specified position
    pub fn get_cell(&self, x: u16, y: u16) -> Option<&BufferCell> {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            Some(&self.cells[index])
        } else {
            None
        }
    }

    /// Get a mutable cell at the specified position
    pub fn get_cell_mut(&mut self, x: u16, y: u16) -> Option<&mut BufferCell> {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            Some(&mut self.cells[index])
        } else {
            None
        }
    }

    /// Set a cell at the specified position
    pub fn set_cell(&mut self, x: u16, y: u16, ch: char, style: &Style) -> Result<()> {
        if let Some(cell) = self.get_cell_mut(x, y) {
            let new_cell = BufferCell {
                ch,
                style: *style,
                dirty: cell.ch != ch || cell.style != *style,
            };
            *cell = new_cell;
        }
        Ok(())
    }

    /// Draw a character at the specified position
    pub fn draw_char(&mut self, x: u16, y: u16, ch: char, style: &Style) -> Result<()> {
        self.set_cell(x, y, ch, style)
    }

    /// Draw text at the specified position
    pub fn draw_text(&mut self, x: u16, y: u16, text: &str, style: &Style) -> Result<()> {
        let mut current_x = x;
        for ch in text.chars() {
            if current_x >= self.width {
                break;
            }
            self.draw_char(current_x, y, ch, style)?;
            current_x += 1;
        }
        Ok(())
    }

    /// Draw a rectangle
    pub fn draw_rect(&mut self, x: u16, y: u16, width: u16, height: u16, style: &Style) -> Result<()> {
        for row in 0..height {
            for col in 0..width {
                let buffer_x = x + col;
                let buffer_y = y + row;
                if buffer_x < self.width && buffer_y < self.height {
                    self.draw_char(buffer_x, buffer_y, ' ', style)?;
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
        self.draw_char(x, y, '┌', style)?;
        self.draw_char(x + width - 1, y, '┐', style)?;
        self.draw_char(x, y + height - 1, '└', style)?;
        self.draw_char(x + width - 1, y + height - 1, '┘', style)?;

        // Draw horizontal lines
        for col in 1..width - 1 {
            self.draw_char(x + col, y, '─', style)?;
            self.draw_char(x + col, y + height - 1, '─', style)?;
        }

        // Draw vertical lines
        for row in 1..height - 1 {
            self.draw_char(x, y + row, '│', style)?;
            self.draw_char(x + width - 1, y + row, '│', style)?;
        }

        Ok(())
    }

    /// Fill an area with a character
    pub fn fill_area(&mut self, x: u16, y: u16, width: u16, height: u16, ch: char, style: &Style) -> Result<()> {
        for row in 0..height {
            for col in 0..width {
                let buffer_x = x + col;
                let buffer_y = y + row;
                if buffer_x < self.width && buffer_y < self.height {
                    self.draw_char(buffer_x, buffer_y, ch, style)?;
                }
            }
        }
        Ok(())
    }

    /// Render the buffer to the terminal
    pub fn render_to_terminal(&mut self) -> Result<()> {
        let mut stdout = stdout();
        let mut current_style = None;
        let mut x = 0u16;
        let mut y = 0u16;

        // Swap buffers
        std::mem::swap(&mut self.cells, &mut self.prev_cells);

        // Render only dirty cells
        for (index, cell) in self.cells.iter().enumerate() {
            let prev_cell = &self.prev_cells[index];
            
            if cell.ch != prev_cell.ch || cell.style != prev_cell.style {
                let cell_x = (index as u16) % self.width;
                let cell_y = (index as u16) / self.width;

                // Move cursor if needed
                if cell_x != x || cell_y != y {
                    queue!(stdout, MoveTo(cell_x, cell_y))?;
                    x = cell_x;
                    y = cell_y;
                }

                // Apply style if changed
                if Some(cell.style) != current_style {
                    let commands = Renderer::style_to_commands(&cell.style);
                    for command in commands {
                        queue!(stdout, command)?;
                    }
                    current_style = Some(cell.style);
                }

                // Print character
                queue!(stdout, Print(cell.ch))?;
                x += 1;
                if x >= self.width {
                    x = 0;
                    y += 1;
                }
            }
        }

        // Reset style at the end
        if current_style.is_some() {
            queue!(stdout, ResetColor)?;
        }

        // Flush output
        stdout.flush()?;
        Ok(())
    }

    /// Get the buffer as a string (for debugging)
    pub fn as_string(&self) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = self.get_cell(x, y) {
                    result.push(cell.ch);
                }
            }
            if y < self.height - 1 {
                result.push('\n');
            }
        }
        result
    }

    /// Get the buffer as a colored string (for debugging)
    pub fn as_colored_string(&self) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = self.get_cell(x, y) {
                    // Add ANSI color codes for debugging
                    if let Some(fg) = cell.style.foreground {
                        result.push_str(&format!("\x1b[38;5;{}m", fg.as_ansi_value()));
                    }
                    result.push(cell.ch);
                    result.push_str("\x1b[0m");
                }
            }
            if y < self.height - 1 {
                result.push('\n');
            }
        }
        result
    }

    /// Count dirty cells
    pub fn count_dirty_cells(&self) -> usize {
        self.cells.iter().filter(|cell| cell.dirty).count()
    }

    /// Mark all cells as clean
    pub fn mark_all_clean(&mut self) {
        for cell in &mut self.cells {
            cell.dirty = false;
        }
    }

    /// Mark all cells as dirty
    pub fn mark_all_dirty(&mut self) {
        for cell in &mut self.cells {
            cell.dirty = true;
        }
    }

    /// Copy a region from another buffer
    pub fn copy_region(&mut self, src: &RenderBuffer, src_x: u16, src_y: u16, dest_x: u16, dest_y: u16, width: u16, height: u16) -> Result<()> {
        for row in 0..height {
            for col in 0..width {
                let src_cell_x = src_x + col;
                let src_cell_y = src_y + row;
                let dest_cell_x = dest_x + col;
                let dest_cell_y = dest_y + row;

                if let Some(src_cell) = src.get_cell(src_cell_x, src_cell_y) {
                    if dest_cell_x < self.width && dest_cell_y < self.height {
                        self.set_cell(dest_cell_x, dest_cell_y, src_cell.ch, &src_cell.style)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl Default for RenderBuffer {
    fn default() -> Self {
        Self::new(80, 24) // Default terminal size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{Style, Color};

    #[test]
    fn test_buffer_creation() {
        let buffer = RenderBuffer::new(80, 24);
        assert_eq!(buffer.width(), 80);
        assert_eq!(buffer.height(), 24);
        assert_eq!(buffer.cells.len(), 80 * 24);
    }

    #[test]
    fn test_cell_operations() {
        let mut buffer = RenderBuffer::new(10, 5);
        
        // Test getting and setting cells
        assert!(buffer.get_cell(0, 0).is_some());
        assert!(buffer.get_cell(10, 5).is_none());
        
        buffer.draw_char(5, 3, 'X', &Style::default()).unwrap();
        let cell = buffer.get_cell(5, 3).unwrap();
        assert_eq!(cell.ch, 'X');
    }

    #[test]
    fn test_text_operations() {
        let mut buffer = RenderBuffer::new(10, 5);
        let style = Style::default().with_foreground(Color::Red);
        
        buffer.draw_text(0, 0, "hello", &style).unwrap();
        
        for (i, ch) in "hello".chars().enumerate() {
            let cell = buffer.get_cell(i as u16, 0).unwrap();
            assert_eq!(cell.ch, ch);
            assert_eq!(cell.style.foreground, Some(Color::Red));
        }
    }

    #[test]
    fn test_rect_operations() {
        let mut buffer = RenderBuffer::new(10, 5);
        let style = Style::default().with_background(Color::Blue);
        
        buffer.draw_rect(2, 1, 5, 3, &style).unwrap();
        
        for y in 1..4 {
            for x in 2..7 {
                let cell = buffer.get_cell(x, y).unwrap();
                assert_eq!(cell.ch, ' ');
                assert_eq!(cell.style.background, Some(Color::Blue));
            }
        }
    }

    #[test]
    fn test_border_operations() {
        let mut buffer = RenderBuffer::new(10, 5);
        let style = Style::default();
        
        buffer.draw_border(1, 1, 8, 3, &style).unwrap();
        
        // Test corners
        assert_eq!(buffer.get_cell(1, 1).unwrap().ch, '┌');
        assert_eq!(buffer.get_cell(8, 1).unwrap().ch, '┐');
        assert_eq!(buffer.get_cell(1, 3).unwrap().ch, '└');
        assert_eq!(buffer.get_cell(8, 3).unwrap().ch, '┘');
        
        // Test edges
        assert_eq!(buffer.get_cell(5, 1).unwrap().ch, '─');
        assert_eq!(buffer.get_cell(5, 3).unwrap().ch, '─');
        assert_eq!(buffer.get_cell(1, 2).unwrap().ch, '│');
        assert_eq!(buffer.get_cell(8, 2).unwrap().ch, '│');
    }

    #[test]
    fn test_buffer_resize() {
        let mut buffer = RenderBuffer::new(5, 5);
        buffer.draw_char(2, 2, 'X', &Style::default()).unwrap();
        
        buffer.resize(10, 10);
        assert_eq!(buffer.width(), 10);
        assert_eq!(buffer.height(), 10);
        assert_eq!(buffer.get_cell(2, 2).unwrap().ch, 'X');
    }

    #[test]
    fn test_as_string() {
        let mut buffer = RenderBuffer::new(5, 2);
        buffer.draw_text(0, 0, "hello", &Style::default()).unwrap();
        buffer.draw_text(0, 1, "world", &Style::default()).unwrap();
        
        let string = buffer.as_string();
        assert_eq!(string, "hello\nworld");
    }
}