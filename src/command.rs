#[derive(Clone, Debug)]
pub enum Command {
    Left,
    Front,
    Right,
}

impl std::convert::From<char> for Command {
    fn from(c: char) -> Self {
        match c {
            'L' => Command::Left,
            'F' => Command::Front,
            'R' => Command::Right,

            // Would be better to impl try_from and return a proper error!
            _ => unreachable!(),
        }
    }
}
