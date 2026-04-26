mod error;
mod parser;

pub use error::{Error, Result};
use crate::parser::Parser;
use kale_syntax::ast::Block;
use kale_syntax::token::TokenStream;

pub fn parse(stream: &TokenStream) -> Result<Block> {
    Parser::new(&stream.0).parse()
}
