use crate::units::SizeUnit;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Position {
    x: SizeUnit,
    y: SizeUnit,
}

impl Position {
    pub fn new(x: SizeUnit, y: SizeUnit) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> SizeUnit {
        self.x
    }

    pub fn y(&self) -> SizeUnit {
        self.y
    }
}
