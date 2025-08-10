use crate::config::Rule;
use crate::units::SizeUnit;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid configuration file")]
    InvalidConfig,

    #[error("Position in invalid: {0}x{1}")]
    InvalidPosition(SizeUnit, SizeUnit),

    #[error("Size in invalid: {0}x{1}")]
    InvalidSize(SizeUnit, SizeUnit),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    TaskJoin(#[from] tokio::task::JoinError),

    #[error("Missing input file")]
    MissingInpputFile,

    #[error("Position is out of bounds")]
    OutOfBounds,

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Pest(#[from] pest::error::Error<Rule>),
}
