use std::sync::atomic::{AtomicBool, Ordering};

use crate::config::Config;
use crate::error::Error;
use crate::orientation::Orientation;
use crate::position::Position;
use crate::units::{Size, SizeUnit};

#[derive(Debug)]
pub struct Cell {
    /// Check if a cell is occupied by a rover.
    pub occupied: AtomicBool,
}

impl Cell {
    /// Try to occupy a cell.
    /// # Returns
    /// A boolean that indicates whether the cell was already occupied before.
    pub fn set_occupied(&self, occupied: bool) -> bool {
        self.occupied.swap(occupied, Ordering::SeqCst)
    }
}

pub struct Land {
    /// Size of the land to be mowed.
    size: Size,

    /// Use one dimensional vector to represent the land (better than Vec<Vec<Cell>>).
    cells: Vec<Cell>,
}

impl Land {
    pub fn from(config: &Config) -> Self {
        Self {
            size: Size::default(),
            cells: vec![],
        }
    }

    /// Set the occupied state for a cell of the land.
    ///
    /// # Returns
    /// A boolean that indicates whether the cell was already occupied before.
    pub fn set_occupied(&self, position: &Position, occupied: bool) -> bool {
        if let Ok(cell) = self.cell(position) {
            cell.set_occupied(occupied)
        } else {
            false
        }
    }

    /// Gets a cell of the land giving its coordinates.
    fn cell(&self, position: &Position) -> Result<&Cell, Error> {
        let x = position.x();
        let y = position.y();
        let flipped_y = self.size.height() - 1 - y;
        let index = (flipped_y * self.size.width() + x) as usize;

        self.cells.get(index).ok_or(Error::InvalidPosition(x, y))
    }

    /// Computes the next cell position based on the current position and orientation.
    pub fn next_cell_position(
        &self,
        position: &Position,
        orientation: &Orientation,
    ) -> Result<Position, Error> {
        match orientation {
            Orientation::North => {
                if position.y() < self.size.height() - 1 {
                    Ok(Position::new(position.x(), position.y() + 1))
                } else {
                    Err(Error::OutOfBounds)
                }
            }

            Orientation::South => {
                if position.y() > 0 {
                    Ok(Position::new(position.x(), position.y() - 1))
                } else {
                    Err(Error::OutOfBounds)
                }
            }

            Orientation::West => {
                if position.x() > 0 {
                    Ok(Position::new(position.x() - 1, position.y()))
                } else {
                    Err(Error::OutOfBounds)
                }
            }

            Orientation::East => {
                if position.x() < self.size.width() - 1 {
                    Ok(Position::new(position.x() + 1, position.y()))
                } else {
                    Err(Error::OutOfBounds)
                }
            }
        }
    }
}
