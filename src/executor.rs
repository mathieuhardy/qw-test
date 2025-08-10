use std::sync::Arc;
use tokio::task;

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

    pub async fn run(&self) -> Result<Vec<Rover>, Error> {
        let mut tasks = vec![];

        // Place all rovers to their initial positions
        for rover in &self.rovers {
            self.land.set_occupied(rover.position(), true);
        }

        // Start one worker per rover
        for rover in self.rovers.clone() {
            let t = task::spawn(rover_worker(self.land.clone(), rover));

            tasks.push(t);
        }

        // Wait for all tasks to finish
        let mut rovers = vec![];

        for t in tasks {
            let rover = t.await?;
            rovers.push(rover)
        }

        Ok(rovers)
    }
}

async fn rover_worker(land: Arc<Land>, mut rover: Rover) -> Rover {
    while let Some(command) = rover.next_command() {
        // Get orientation after executing the command (in order to see if it has changed).
        let new_orientation = rover.orientation().get_next(&command);

        if *rover.orientation() != new_orientation {
            // Simply change orientation of the rover
            rover.set_orientation(new_orientation);
            continue;
        }

        // Try to step forward
        let new_position = land.next_cell_position(rover.position(), rover.orientation());

        if let Ok(new_position) = new_position {
            // The move forward is legit and the rover can move to the cell (unless it's
            // already occupied by another rover of course).
            let already_occupied = land.set_occupied(&new_position, true);

            if already_occupied {
                // Don't move, ignore the command and continue the simulation
                continue;
            }

            // Let's free the previous cell we were in
            land.set_occupied(rover.position(), false);

            // Update rover's position
            rover.set_position(new_position);
        }
    }

    rover
}
