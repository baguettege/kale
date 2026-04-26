#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected eof")]
    UnexpectedEof,
    #[error("unknown tag: {0}")]
    UnknownTag(u8),
    #[error(transparent)]
    InvalidUtf8(#[from] std::str::Utf8Error),
    #[error("invalid data: {0}")]
    InvalidData(String),
}

pub type Result<T> = std::result::Result<T, Error>;
