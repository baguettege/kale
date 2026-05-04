mod error;
mod parser;

use crate::parser::Parser;
use kale_syntax::ast::Program;
use kale_syntax::token::Token;

pub use error::{Error, ErrorKind, Result};

pub fn parse(tokens: &[Token]) -> Result<Program> {
    Parser::new(tokens).parse()
}
