//! Layout utilities for arranging components

use crate::components::Component;
use crate::style::{Style, Color};

/// Layout constraints for components
#[derive(Debug, Clone, Copy)]
pub struct Constraints {
    /// Minimum width
    pub min_width: Option<u16>,
    /// Maximum width
    pub max_width: Option<u16>,
    /// Minimum height
    pub min_height: Option<u16>,
    /// Maximum height
    pub max_height: Option<u16>,
}

impl Default for Constraints {
    fn default() -> Self {
        Self {
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
        }
    }
}

impl Constraints {
    /// Create new constraints
    pub fn new() -> Self {
        Self::default()
    }

    /// Set minimum width
    pub fn with_min_width(mut self, width: u16) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Set maximum width
    pub fn with_max_width(mut self, width: u16) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Set minimum height
    pub fn with_min_height(mut self, height: u16) -> Self {
        self.min_height = Some(height);
        self
    }

    /// Set maximum height
    pub fn with_max_height(mut self, height: u16) -> Self {
        self.max_height = Some(height);
        self
    }

    /// Set both minimum and maximum width
    pub fn with_width(mut self, width: u16) -> Self {
        self.min_width = Some(width);
        self.max_width = Some(width);
        self
    }

    /// Set both minimum and maximum height
    pub fn with_height(mut self, height: u16) -> Self {
        self.min_height = Some(height);
        self.max_height = Some(height);
        self
    }

    /// Apply constraints to a size
    pub fn apply(&self, width: u16, height: u16) -> (u16, u16) {
        let constrained_width = width
            .max(self.min_width.unwrap_or(0))
            .min(self.max_width.unwrap_or(u16::MAX));
        let constrained_height = height
            .max(self.min_height.unwrap_or(0))
            .min(self.max_height.unwrap_or(u16::MAX));
        
        (constrained_width, constrained_height)
    }
}

/// Alignment options for layout
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    /// Align to start (left/top)
    Start,
    /// Align to center
    Center,
    /// Align to end (right/bottom)
    End,
    /// Stretch to fill available space
    Stretch,
}

/// Layout margins
#[derive(Debug, Clone, Copy)]
pub struct Margins {
    /// Left margin
    pub left: u16,
    /// Top margin
    pub top: u16,
    /// Right margin
    pub right: u16,
    /// Bottom margin
    pub bottom: u16,
}

impl Default for Margins {
    fn default() -> Self {
        Self {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }
}

impl Margins {
    /// Create new margins
    pub fn new() -> Self {
        Self::default()
    }

    /// Set uniform margins
    pub fn all(value: u16) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }

    /// Set horizontal margins
    pub fn horizontal(value: u16) -> Self {
        Self {
            left: value,
            right: value,
            top: 0,
            bottom: 0,
        }
    }

    /// Set vertical margins
    pub fn vertical(value: u16) -> Self {
        Self {
            left: 0,
            right: 0,
            top: value,
            bottom: value,
        }
    }

    /// Set individual margins
    pub fn custom(left: u16, top: u16, right: u16, bottom: u16) -> Self {
        Self { left, top, right, bottom }
    }

    /// Get total horizontal margin
    pub fn horizontal_total(&self) -> u16 {
        self.left + self.right
    }

    /// Get total vertical margin
    pub fn vertical_total(&self) -> u16 {
        self.top + self.bottom
    }
}

/// Flex layout utilities
pub mod flex {
    use super::*;

    /// Flex direction
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Direction {
        /// Arrange items horizontally
        Row,
        /// Arrange items vertically
        Column,
    }

    /// Flex justification (main axis alignment)
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Justify {
        /// Align to start
        Start,
        /// Align to center
        Center,
        /// Align to end
        End,
        /// Distribute space between items
        SpaceBetween,
        /// Distribute space around items
        SpaceAround,
        /// Distribute space evenly
        SpaceEvenly,
    }

    /// Flex alignment (cross axis alignment)
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Align {
        /// Align to start
        Start,
        /// Align to center
        Center,
        /// Align to end
        End,
        /// Stretch to fill cross axis
        Stretch,
    }

    /// Calculate flex layout for a container
    pub fn calculate_layout(
        container_width: u16,
        container_height: u16,
        direction: Direction,
        justify: Justify,
        align: Align,
        items: &[(u16, u16)], // (width, height) for each item
        gaps: u16,
    ) -> Vec<(u16, u16, u16, u16)> {
        // (x, y, width, height) for each item
        let mut positions = Vec::new();
        
        if items.is_empty() {
            return positions;
        }

        match direction {
            Direction::Row => {
                let total_item_width: u16 = items.iter().map(|(w, _)| *w).sum();
                let total_gap_width = gaps * (items.len() - 1) as u16;
                let available_width = container_width.saturating_sub(total_item_width + total_gap_width);
                
                let mut current_x = match justify {
                    Justify::Start => 0,
                    Justify::Center => available_width / 2,
                    Justify::End => available_width,
                    Justify::SpaceBetween => 0,
                    Justify::SpaceAround => gaps / 2,
                    Justify::SpaceEvenly => gaps,
                };
                
                for (i, &(item_width, item_height)) in items.iter().enumerate() {
                    let x = current_x;
                    let y = match align {
                        Align::Start => 0,
                        Align::Center => (container_height - item_height) / 2,
                        Align::End => container_height - item_height,
                        Align::Stretch => 0,
                    };
                    let height = if align == Align::Stretch { container_height } else { item_height };
                    
                    positions.push((x, y, item_width, height));
                    
                    current_x += item_width + match justify {
                        Justify::SpaceBetween => if i < items.len() - 1 { 
                            available_width / (items.len() - 1) as u16 
                        } else { 
                            0 
                        },
                        Justify::SpaceAround => gaps,
                        Justify::SpaceEvenly => gaps,
                        _ => gaps,
                    };
                }
            }
            Direction::Column => {
                let total_item_height: u16 = items.iter().map(|(_, h)| *h).sum();
                let total_gap_height = gaps * (items.len() - 1) as u16;
                let available_height = container_height.saturating_sub(total_item_height + total_gap_height);
                
                let mut current_y = match justify {
                    Justify::Start => 0,
                    Justify::Center => available_height / 2,
                    Justify::End => available_height,
                    Justify::SpaceBetween => 0,
                    Justify::SpaceAround => gaps / 2,
                    Justify::SpaceEvenly => gaps,
                };
                
                for (i, &(item_width, item_height)) in items.iter().enumerate() {
                    let x = match align {
                        Align::Start => 0,
                        Align::Center => (container_width - item_width) / 2,
                        Align::End => container_width - item_width,
                        Align::Stretch => 0,
                    };
                    let y = current_y;
                    let width = if align == Align::Stretch { container_width } else { item_width };
                    
                    positions.push((x, y, width, item_height));
                    
                    current_y += item_height + match justify {
                        Justify::SpaceBetween => if i < items.len() - 1 { 
                            available_height / (items.len() - 1) as u16 
                        } else { 
                            0 
                        },
                        Justify::SpaceAround => gaps,
                        Justify::SpaceEvenly => gaps,
                        _ => gaps,
                    };
                }
            }
        }
        
        positions
    }
}

/// Grid layout utilities
pub mod grid {
    use super::*;

    /// Grid layout configuration
    #[derive(Debug, Clone)]
    pub struct GridConfig {
        /// Number of columns
        pub columns: u16,
        /// Number of rows
        pub rows: u16,
        /// Gap between columns
        pub column_gap: u16,
        /// Gap between rows
        pub row_gap: u16,
    }

    impl Default for GridConfig {
        fn default() -> Self {
            Self {
                columns: 1,
                rows: 1,
                column_gap: 0,
                row_gap: 0,
            }
        }
    }

    impl GridConfig {
        /// Create a new grid configuration
        pub fn new(columns: u16, rows: u16) -> Self {
            Self {
                columns,
                rows,
                column_gap: 0,
                row_gap: 0,
            }
        }

        /// Set column gap
        pub fn with_column_gap(mut self, gap: u16) -> Self {
            self.column_gap = gap;
            self
        }

        /// Set row gap
        pub fn with_row_gap(mut self, gap: u16) -> Self {
            self.row_gap = gap;
            self
        }

        /// Set both gaps
        pub fn with_gap(mut self, gap: u16) -> Self {
            self.column_gap = gap;
            self.row_gap = gap;
            self
        }

        /// Calculate grid cell positions
        pub fn calculate_cells(&self, container_width: u16, container_height: u16) -> Vec<(u16, u16, u16, u16)> {
            let mut cells = Vec::new();
            
            let total_gap_width = self.column_gap * (self.columns - 1);
            let total_gap_height = self.row_gap * (self.rows - 1);
            
            let cell_width = (container_width - total_gap_width) / self.columns;
            let cell_height = (container_height - total_gap_height) / self.rows;
            
            for row in 0..self.rows {
                for col in 0..self.columns {
                    let x = col * (cell_width + self.column_gap);
                    let y = row * (cell_height + self.row_gap);
                    cells.push((x, y, cell_width, cell_height));
                }
            }
            
            cells
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraints() {
        let constraints = Constraints::new()
            .with_min_width(10)
            .with_max_width(20)
            .with_height(15);
        
        let (width, height) = constraints.apply(5, 25);
        assert_eq!(width, 10); // Clamped to min width
        assert_eq!(height, 15); // Fixed height
        
        let (width, height) = constraints.apply(25, 10);
        assert_eq!(width, 20); // Clamped to max width
        assert_eq!(height, 15); // Fixed height
    }

    #[test]
    fn test_margins() {
        let margins = Margins::all(5);
        assert_eq!(margins.horizontal_total(), 10);
        assert_eq!(margins.vertical_total(), 10);
        
        let margins = Margins::horizontal(3).vertical(2);
        assert_eq!(margins.left, 3);
        assert_eq!(margins.right, 3);
        assert_eq!(margins.top, 2);
        assert_eq!(margins.bottom, 2);
    }

    #[test]
    fn test_flex_layout() {
        let items = [(10, 5), (15, 5), (10, 5)];
        let positions = flex::calculate_layout(
            50, 10,
            flex::Direction::Row,
            flex::Justify::SpaceBetween,
            flex::Align::Center,
            &items,
            2,
        );
        
        assert_eq!(positions.len(), 3);
        assert_eq!(positions[0], (0, 2, 10, 5)); // First item at start
        assert_eq!(positions[1], (17, 2, 15, 5)); // Middle item centered
        assert_eq!(positions[2], (40, 2, 10, 5)); // Last item at end
    }

    #[test]
    fn test_grid_config() {
        let grid = GridConfig::new(2, 3).with_gap(1);
        let cells = grid.calculate_cells(10, 7);
        
        assert_eq!(cells.len(), 6); // 2 columns * 3 rows
        assert_eq!(cells[0], (0, 0, 4, 2)); // First cell
        assert_eq!(cells[1], (5, 0, 4, 2)); // Second cell
        assert_eq!(cells[2], (0, 3, 4, 2)); // Third cell
    }
}