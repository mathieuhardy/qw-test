use std::sync::atomic::AtomicBool;

use crate::config::Config;
use crate::units::Size;

#[derive(Debug)]
pub struct Cell {
    /// Check if a cell is occupied by a rover.
    pub occupied: AtomicBool,
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
}
