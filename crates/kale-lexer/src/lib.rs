mod error;
mod lexer;

use crate::lexer::Lexer;
pub use error::{Error, Result};
use kale_syntax::token::Token;

pub fn tokenize(input: &str) -> Result<Vec<Token>> {
    Lexer::new(input).tokenize()
}
