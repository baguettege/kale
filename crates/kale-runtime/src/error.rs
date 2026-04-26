use kale_syntax::ast::Ident;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("type error: {0}")]
    TypeError(String),
    #[error("undefined variable: {0}")]
    UndefinedVariable(Ident),
    #[error("index out of bounds: {0}")]
    IndexOutOfBounds(usize),
    #[error("invalid assignment target")]
    InvalidAssign,
    #[error("missing argument at index {0}")]
    MissingArg(usize),
}

pub type Result<T> = std::result::Result<T, Error>;
