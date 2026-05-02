pub mod ast {
    pub use kale_syntax::ast::*;
}

pub mod parser {
    pub use kale_lexer::Error as LexError;
    pub use kale_parser::Error as ParseError;
    use crate::ast::Program;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Lex(#[from] LexError),
        #[error(transparent)]
        Parse(#[from] ParseError),
    }

    pub fn parse(input: &str) -> Result<Program, Error> {
        let tokens = kale_lexer::tokenize(input)?;
        let program = kale_parser::parse(&tokens)?;
        Ok(program)
    }
}

pub mod codec {
    pub use kale_codec::*;
}

pub mod runtime {
    pub use kale_runtime::*;
}

pub mod interpreter {
    pub use kale_interpreter::*;
}
