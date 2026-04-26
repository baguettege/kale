#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Codec(#[from] kale_api::codec::Error),
    #[error(transparent)]
    Runtime(#[from] kale_api::runtime::Error),
    #[error("usage: kale <file>")]
    NoFile,
}

pub type Result<T> = std::result::Result<T, Error>;
