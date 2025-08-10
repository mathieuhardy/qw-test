use crate::command::Command;

#[derive(Clone, Debug, PartialEq)]
pub enum Orientation {
    North,
    South,
    East,
    West,
}

impl std::convert::From<char> for Orientation {
    fn from(c: char) -> Self {
        match c {
            'N' => Orientation::North,
            'S' => Orientation::South,
            'E' => Orientation::East,
            'W' => Orientation::West,

            // Would be better to impl try_from and return a proper error!
            _ => unreachable!(),
        }
    }
}

impl Orientation {
    pub fn get_next(&self, command: &Command) -> Self {
        match command {
            Command::Left => match self {
                Orientation::North => Orientation::West,
                Orientation::West => Orientation::South,
                Orientation::South => Orientation::East,
                Orientation::East => Orientation::North,
            },

            Command::Right => match self {
                Orientation::North => Orientation::East,
                Orientation::East => Orientation::South,
                Orientation::South => Orientation::West,
                Orientation::West => Orientation::North,
            },

            // No change in the orientation because it's a forward command.
            Command::Front => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_orientation() {
        // Turn left
        let north = Orientation::North;
        let west = Orientation::West;
        let south = Orientation::South;
        let east = Orientation::East;

        assert_eq!(north.get_next(&Command::Left), Orientation::West);
        assert_eq!(west.get_next(&Command::Left), Orientation::South);
        assert_eq!(south.get_next(&Command::Left), Orientation::East);
        assert_eq!(east.get_next(&Command::Left), Orientation::North);

        // Turn right
        assert_eq!(north.get_next(&Command::Right), Orientation::East);
        assert_eq!(east.get_next(&Command::Right), Orientation::South);
        assert_eq!(south.get_next(&Command::Right), Orientation::West);
        assert_eq!(
            Orientation::West.get_next(&Command::Right),
            Orientation::North
        );

        // Move foward
        for orientation in [
            Orientation::North,
            Orientation::South,
            Orientation::East,
            Orientation::West,
        ] {
            assert_eq!(orientation.get_next(&Command::Front), orientation);
        }
    }
}
