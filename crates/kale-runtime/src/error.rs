use kale_syntax::ast::Ident;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("scope stack underflow: cannot exit global scope")]
    ScopeUnderflow,
    #[error("expected type {expected}, got {got}")]
    TypeError { expected: &'static str, got: &'static str },
    #[error("undefined variable: {0}")]
    UndefinedVariable(Ident),
}

pub type Result<T> = std::result::Result<T, Error>;
