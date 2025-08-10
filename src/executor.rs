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

    pub async fn run(&self) -> Result<(), Error> {
        let mut tasks = vec![];

        // Place all rovers to their initial positions
        for rover in &self.rovers {
            self.land.set_occupied(rover.position(), true);
        }

        // Start one worker per rover
        for rover in self.rovers.clone() {
            let t = task::spawn(rover_worker(rover));

            tasks.push(t);
        }

        // Wait for all tasks to finish
        for t in tasks {
            t.await.unwrap();
        }

        Ok(())
    }
}

async fn rover_worker(_rover: Rover) {
    println!("Rover task started and finished");
}
