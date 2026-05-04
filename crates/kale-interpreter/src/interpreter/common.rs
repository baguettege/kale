use kale_runtime::env::Env;
use kale_syntax::ast::Block;
use kale_syntax::span::Span;
use crate::error::Error;
use crate::interpreter::flow::Result;
use crate::interpreter::Interpreter;

impl Interpreter {
    pub(super) fn error(&self, error: kale_runtime::Error) -> Error {
        // convenience helper to construct a spanned error at the current
        // `self.span` location, reducing boilerplate at call sites
        Error::new(error, self.span)
    }

    pub(super) fn eval_block(&mut self, block: &Block) -> Result<()> {
        for stmt in block {
            self.eval_stmt(stmt)?;
        }
        Ok(())
    }

    pub(super) fn with_env<R, F>(&mut self, env: Env, f: F) -> Result<R>
    where
        F: FnOnce(&mut Self) -> Result<R>,
    {
        // replaces `self.env` with `env` for the duration of `f`, then restores it
        // centralizes env swapping so callers don't have to manually restore
        
        let prev = std::mem::replace(&mut self.env, env);
        let result = f(self);
        self.env = prev;
        result
    }

    pub(super) fn with_scope<R, F>(&mut self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Self) -> Result<R>,
    {
        // enters a new scope, executes `f`, and exits it once the `ScopeGuard` is dropped
        // centralizes scope handling so callers don't have to manually restore
        
        let _guard = self.env.enter_scope();
        f(self)
    }

    pub(super) fn with_span<F, R>(&mut self, span: Span, f: F) -> Result<R>
    where
        F: FnOnce(&mut Self) -> Result<R>,
    {
        // replaces `self.span` with `span` for the duration of `f`, then restores it
        // centralizes env swapping so callers don't have to manually restore
        
        let prev = self.span;
        self.span = span;
        
        let result = f(self);
        
        self.span = prev;
        result
    }
}
