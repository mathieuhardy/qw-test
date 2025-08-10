use std::sync::atomic::{AtomicBool, Ordering};

use crate::config::Config;
use crate::error::Error;
use crate::position::Position;
use crate::units::{Size, SizeUnit};

#[derive(Debug)]
pub struct Cell {
    /// Check if a cell is occupied by a rover.
    pub occupied: AtomicBool,
}

impl Cell {
    pub fn set_occupied(&self, occupied: bool) {
        self.occupied.store(occupied, Ordering::SeqCst);
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
    pub fn set_occupied(&self, position: &Position, occupied: bool) {
        if let Ok(cell) = self.cell(position) {
            cell.set_occupied(occupied);
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
}
