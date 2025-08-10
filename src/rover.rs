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

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn orientation(&self) -> &Orientation {
        &self.orientation
    }

    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    #[cfg(test)]
    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    /// Gets the next command to be executed by the rover and advances one step.
    pub fn next_command(&mut self) -> Option<Command> {
        if let Some(command) = self.commands.get(self.current_command) {
            self.current_command += 1;
            Some(command.clone())
        } else {
            None
        }
    }
}
