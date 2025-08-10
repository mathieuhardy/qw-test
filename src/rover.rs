use crate::command::Command;
use crate::orientation::Orientation;
use crate::position::Position;
use crate::units::SizeUnit;

#[derive(Clone, Debug)]
pub struct Rover {
    id: SizeUnit,
    position: Position,
    orientation: Orientation,
    commands: Vec<Command>,
    current_command: usize,
}

impl Rover {
    pub fn new(
        id: SizeUnit,
        position: Position,
        orientation: Orientation,
        commands: Vec<Command>,
    ) -> Self {
        Rover {
            id,
            position,
            orientation,
            commands,
            current_command: 0,
        }
    }

    pub fn id(&self) -> SizeUnit {
        self.id
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn orientation(&self) -> &Orientation {
        &self.orientation
    }

    pub fn commands(&self) -> &[Command] {
        &self.commands
    }
}
