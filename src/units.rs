use crate::error::Error;

// This type allows to quickly change the unit for the whole application, in can we need to handle
// larger land sizes.
pub type SizeUnit = u32;

#[derive(Clone, Debug, Default)]
pub struct Size {
    pub width: SizeUnit,
    pub height: SizeUnit,
}

impl Size {
    pub fn new(width: SizeUnit, height: SizeUnit) -> Result<Self, Error> {
        if width <= 0 || height <= 0 {
            return Err(Error::InvalidSize(width, height));
        }

        Ok(Self { width, height })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_creation() -> Result<(), Error> {
        let size = Size::new(5, 13)?;
        assert_eq!(size.width, 5);
        assert_eq!(size.height, 13);

        Ok(())
    }

    #[test]
    fn test_invalid_size_creation() -> Result<(), Error> {
        let size = Size::new(0, 13);
        assert!(size.is_err());

        let size = Size::new(5, 0);
        assert!(size.is_err());

        Ok(())
    }
}
