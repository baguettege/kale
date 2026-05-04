use kale_syntax::span::Spanned;
use kale_syntax::token::Token;

pub type Error = Spanned<ErrorKind>;

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("unexpected eof")]
    UnexpectedEof,
    #[error("unexpected token `{0}`")]
    UnexpectedToken(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn unexpected_token(token: &Token) -> Error {
    Error::new(
        token.span(),
        ErrorKind::UnexpectedToken(token.to_string()),
    )
}
