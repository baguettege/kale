#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected eof")]
    UnexpectedEof,
    #[error("unexpected token: {0}")]
    UnexpectedToken(String),
}

pub type Result<T> = std::result::Result<T, Error>;
