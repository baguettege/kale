#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] kale_api::parser::Error),
    #[error("usage: kalec <file>")]
    NoFile,
}

pub type Result<T> = std::result::Result<T, Error>;
