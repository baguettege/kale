use kale_runtime::env::Env;
use kale_syntax::ast::Block;
use crate::interpreter::flow::Result;
use crate::interpreter::Interpreter;

impl Interpreter {
    pub(super) fn eval_block(&mut self, block: &Block) -> Result<()> {
        for stmt in block {
            self.eval_stmt(stmt)?;
        }
        Ok(())
    }

    pub(super) fn with_env<T, F>(&mut self, env: Env, f: F) -> Result<T>
    where
        F: FnOnce(&mut Self) -> Result<T>,
    {
        let prev = std::mem::replace(&mut self.env, env);
        let result = f(self);
        self.env = prev;
        result
    }

    pub(super) fn with_scope<T, F>(&mut self, f: F) -> Result<T>
    where
        F: FnOnce(&mut Self) -> Result<T>,
    {
        let _guard = self.env.enter_scope();
        f(self)
    }
}
