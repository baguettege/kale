use kale_syntax::ast::Program;

pub struct Kast {
    pub source: String,
    pub program: Program,
}

impl Kast {
    pub fn new(source: impl Into<String>, program: Program) -> Self {
        let source = source.into();
        Self { source, program }
    }
}
