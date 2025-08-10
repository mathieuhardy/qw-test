#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing input file")]
    MissingInpputFile,
}
