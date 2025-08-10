#![allow(clippy::result_large_err)]

mod command;
mod config;
mod error;
mod executor;
mod land;
mod orientation;
mod position;
mod rover;
mod units;

use std::sync::Arc;

use crate::config::Config;
use crate::error::Error;
use crate::executor::Executor;
use crate::land::Land;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err(Error::MissingInpputFile);
    }

    // Try to parse the configuration
    let config = Config::new(&args[1])?;

    // Create the land based on this configuration
    let land = Land::from(&config);

    // Create executor and launch the simulation
    let executor = Executor::new(Arc::new(land), config.rovers().to_vec());
    let rovers = executor.run().await?;

    for rover in rovers {
        println!(
            "Rover {}: Position: ({}, {}), Orientation: {:?}",
            rover.id(),
            rover.position().x(),
            rover.position().y(),
            rover.orientation()
        );
    }

    Ok(())
}
