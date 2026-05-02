use crate::parser::cursor::Cursor;
use crate::Result;
use kale_syntax::token::Token;
use kale_syntax::ast::{Block, Program};

mod cursor;
mod stmt;
mod expr;
mod common;

pub(crate) struct Parser<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: &'a [Token]) -> Self {
        Self { cursor: Cursor::new(tokens) }
    }

    pub(crate) fn parse(mut self) -> Result<Program> {
        let mut block = Block::new();

        while !self.cursor.is_at_end() {
            block.push(self.parse_stmt()?);
        }

        Ok(Program(block))
    }
}
