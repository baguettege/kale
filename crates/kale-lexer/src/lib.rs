mod error;
mod lexer;

use crate::lexer::Lexer;
pub use error::{Error, Result};
use kale_syntax::token::TokenStream;

pub fn tokenize(input: &str) -> Result<TokenStream> {
    Lexer::new(input).tokenize()
}
