use kale_syntax::ast::Block;
use crate::interpreter::Interpreter;
use crate::interpreter::flow::Result;

impl Interpreter {
    pub(super) fn eval_block(&mut self, block: &Block) -> Result<()> {
        block.0.iter().try_for_each(|stmt| self.eval_stmt(stmt))
    }

    pub(super) fn with_scope<F, R>(&mut self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Self) -> Result<R>,
    {
        self.env.enter_scope();
        let result = f(self);
        self.env.exit_scope()?;
        result
    }
}
