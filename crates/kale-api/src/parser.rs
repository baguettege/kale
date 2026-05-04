pub use kale_lexer::Error as LexError;
pub use kale_parser::Error as ParseError;
use kale_report::Diagnostic;
use kale_syntax::ast::Program;
use kale_syntax::span::Span;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Lex(#[from] LexError),
    #[error(transparent)]
    Parse(#[from] ParseError),
}

impl Diagnostic for Error {
    fn message(&self) -> String {
        // thiserror's `transparent` handles delegation to `to_string()`
        self.to_string()
    }

    fn span(&self) -> Span {
        match self {
            Error::Lex(e) => e.span(),
            Error::Parse(e) => e.span(),
        }
    }
}

pub fn parse(input: &str) -> Result<Program, Error> {
    let tokens = kale_lexer::tokenize(input)?;
    let program = kale_parser::parse(&tokens)?;
    Ok(program)
}
