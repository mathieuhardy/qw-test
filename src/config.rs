use pest::Parser;
use std::fs::read_to_string;

use crate::command::Command;
use crate::error::Error;
use crate::orientation::Orientation;
use crate::position::Position;
use crate::rover::Rover;
use crate::units::Size;
use crate::units::SizeUnit;

#[derive(Clone, Debug, Default)]
pub struct Config {
    land_size: Size,
    rovers: Vec<Rover>,
}

impl Config {
    pub fn new(input_file_path: &str) -> Result<Self, Error> {
        let content = read_to_string(input_file_path)?;

        parse_content(&content)
    }

    pub fn rovers(&self) -> &[Rover] {
        &self.rovers
    }
}

fn parse_content(content: &str) -> Result<Config, Error> {
    let mut pairs = ConfigParser::parse(Rule::config, &content)?;
    let config_pair = pairs.next().ok_or(Error::InvalidConfig)?;

    let mut config = Config::default();
    let mut rover_id = 0;

    for pair in config_pair.into_inner() {
        match pair.as_rule() {
            Rule::land => {
                let mut inner = pair.into_inner();
                let width = inner.next().ok_or(Error::InvalidConfig)?.as_str();
                let height = inner.next().ok_or(Error::InvalidConfig)?.as_str();

                let width = width.parse::<SizeUnit>()?;
                let height = height.parse::<SizeUnit>()?;

                config.land_size = Size::new(width, height)?;
            }

            Rule::rovers => {
                for rover_pair in pair.into_inner() {
                    let mut rover_inner = rover_pair.into_inner();

                    let position_pair = rover_inner.next().ok_or(Error::InvalidConfig)?;
                    let mut pos_inner = position_pair.into_inner();

                    let x = pos_inner.next().ok_or(Error::InvalidConfig)?.as_str();
                    let y = pos_inner.next().ok_or(Error::InvalidConfig)?.as_str();
                    let orientation = pos_inner.next().ok_or(Error::InvalidConfig)?.as_str();

                    let x = x.parse::<u32>()?;
                    let y = y.parse::<u32>()?;
                    let orientation = orientation.chars().next().ok_or(Error::InvalidConfig)?;
                    let orientation = Orientation::from(orientation);

                    let position = Position::new(x, y);

                    let commands_pair = rover_inner.next().ok_or(Error::InvalidConfig)?;
                    let commands = commands_pair
                        .as_str()
                        .chars()
                        .filter(|c| *c == 'L' || *c == 'R' || *c == 'F')
                        .map(Command::from)
                        .collect::<Vec<_>>();

                    config
                        .rovers
                        .push(Rover::new(rover_id, position, orientation, commands));

                    rover_id += 1;
                }
            }
            _ => {}
        }
    }

    Ok(config)
}

#[derive(pest_derive::Parser)]
#[grammar = "config.pest"]
struct ConfigParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nominal() -> Result<(), Error> {
        let content = "5 13\n1 2 N\nLFLFLFLFF\n3 3 E\nFFRFFRFRRF";
        let config = parse_content(content)?;

        assert_eq!(config.land_size.width(), 5);
        assert_eq!(config.land_size.height(), 13);
        assert_eq!(config.rovers.len(), 2);

        assert_eq!(config.rovers[0].id(), 0);
        assert_eq!(config.rovers[0].position(), &Position::new(1, 2));
        assert_eq!(config.rovers[0].orientation(), &Orientation::North);
        assert_eq!(config.rovers[0].commands().len(), 9);

        Ok(())
    }

    #[test]
    fn test_invalid_config() -> Result<(), Error> {
        let content = "invalid config";
        let result = parse_content(content);
        assert!(result.is_err());

        Ok(())
    }
}
