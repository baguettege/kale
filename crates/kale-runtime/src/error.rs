use kale_syntax::ast::Ident;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("scope stack underflow: cannot exit global scope")]
    ScopeUnderflow,
    #[error("type error: {0}")]
    TypeError(String),
    #[error("undefined variable: {0}")]
    UndefinedVariable(Ident),
    #[error("index out of bounds: {0}")]
    IndexOutOfBounds(usize),
    #[error("invalid assignment target")]
    InvalidAssign,
}

pub type Result<T> = std::result::Result<T, Error>;
