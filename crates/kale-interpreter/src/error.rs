use std::fmt;
use kale_syntax::span::Span;

#[derive(Debug)]
pub struct Error {
    inner: kale_runtime::Error,
    span: Span,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub(crate) fn new(inner: kale_runtime::Error, span: Span) -> Self {
        Self { inner, span }
    }

    pub fn inner(&self) -> &kale_runtime::Error {
        &self.inner
    }

    pub fn into_inner(self) -> kale_runtime::Error {
        self.inner
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::error::Error for Error {}
