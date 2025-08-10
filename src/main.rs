mod command;
mod config;
mod error;
mod land;
mod orientation;
mod position;
mod rover;
mod units;

use crate::config::Config;
use crate::error::Error;
use crate::land::Land;

fn main() -> Result<(), Error> {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err(Error::MissingInpputFile);
    }

    // Try to parse the configuration
    let config = Config::new(&args[1])?;
    println!("Parsed configuration: {:#?}", config);

    // Create the land based on this configuration
    let land = Land::from(&config);

    Ok(())
}
