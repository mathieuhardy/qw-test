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
