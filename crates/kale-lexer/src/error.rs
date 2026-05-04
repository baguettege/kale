use kale_syntax::span::Spanned;

pub type Error = Spanned<ErrorKind>;

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("unexpected eof")]
    UnexpectedEof,
    #[error("unexpected char `{0}`")]
    UnexpectedChar(char),
    #[error("invalid num `{0}`")]
    InvalidNum(String),
}

pub type Result<T> = std::result::Result<T, Error>;
