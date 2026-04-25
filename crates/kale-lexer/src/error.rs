#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected eof")]
    UnexpectedEof,
    #[error("unexpected char: {0}")]
    UnexpectedChar(char),
    #[error("expected char: {0}")]
    ExpectedChar(char),
    #[error("invalid num: {0}")]
    InvalidNum(String),
}

pub type Result<T> = std::result::Result<T, Error>;
