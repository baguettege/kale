use crate::object::Object;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("undefined variable: {0}")]
    UndefinedVariable(String),
    #[error("missing argument at index {0}")]
    MissingArg(usize),
    #[error("type error: {0}")]
    TypeError(String),
    #[error("raised: {0}")]
    Raised(Object),
    #[error("invalid assignment target")]
    InvalidAssign,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn type_mismatch(expected: impl Into<String>, got: impl Into<String>) -> Self {
        Self::TypeError(format!("expected {}, got {}", expected.into(), got.into()))
    }

    pub fn raise(object: impl Into<Object>) -> Self {
        Self::Raised(object.into())
    }
}
