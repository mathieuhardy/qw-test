use crate::config::Rule;
use crate::units::SizeUnit;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid configuration file")]
    InvalidConfig,

    #[error("Size in invalid: {0}x{1}")]
    InvalidSize(SizeUnit, SizeUnit),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Missing input file")]
    MissingInpputFile,

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Pest(#[from] pest::error::Error<Rule>),
}
