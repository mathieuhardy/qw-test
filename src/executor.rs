use std::sync::Arc;

use crate::error::Error;
use crate::land::Land;
use crate::rover::Rover;

pub struct Executor {
    /// The land to be mowed.
    land: Arc<Land>,

    /// The list of rovers (with their start positions and orientations, and their list of commands).
    rovers: Vec<Rover>,
}

impl Executor {
    pub fn new(land: Arc<Land>, rovers: Vec<Rover>) -> Self {
        Executor { land, rovers }
    }

    pub fn run(&self) -> Result<(), Error> {
        Ok(())
    }
}
