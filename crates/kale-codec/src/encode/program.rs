use kale_syntax::ast::Program;
use crate::encode::{Encode, Encoder};

impl Encode for Program {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.encode(&self.0);
    }
}
