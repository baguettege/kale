use crate::interpreter::flow::{Outcome, Result};
use crate::interpreter::Interpreter;
use kale_runtime::object::{Closure, Frozen, List, Module, Mutable, Num};
use kale_runtime::Error;
use kale_syntax::ast;
use kale_syntax::ast::{Assign, Expr, FnDef, If, Return, Stmt, While};

impl Interpreter {
    pub(super) fn eval_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Expr(expr) => self.eval_expr_stmt(expr),
            Stmt::Module(node) => self.eval_module(node),
            Stmt::FnDef(node) => Ok(self.eval_fndef(node)),
            Stmt::Assign(node) => self.eval_assign(node),
            Stmt::If(node) => self.eval_if(node),
            Stmt::While(node) => self.eval_while(node),
            Stmt::Return(node) => self.eval_return(node),
        }
    }

    fn eval_expr_stmt(&mut self, expr: &Expr) -> Result<()> {
        self.eval_expr(expr).map(|_| ())
    }

    fn eval_module(&mut self, node: &ast::Module) -> Result<()> {
        self.env.enter_scope();
        self.eval_block(&node.body)?;

        let bindings = self.env.exit_scope()?;
        let module = Module::from(bindings);

        self.env.define(node.ident.clone(), module.into());
        Ok(())
    }

    fn eval_fndef(&mut self, node: &FnDef) {
        let params= node.params.clone();
        let body = node.body.clone();
        let env = self.env.clone();

        let closure = Closure::new(params, body, env);
        self.env.define(node.ident.clone(), closure.into());
    }

    fn eval_assign(&mut self, node: &Assign) -> Result<()> {
        let value = self.eval_expr(&node.value)?;

        match &node.target {
            Expr::Ident(ident) => {
                self.env.define(ident, value);
            },
            Expr::Member(node) => {
                let module: Mutable<Module> = self.eval_expr(&node.object)?.try_into()?;
                module.borrow_mut().define(node.property.clone(), value);
            },
            Expr::Index(node) => {
                let list: Mutable<List> = self.eval_expr(&node.object)?.try_into()?;
                let index: Frozen<Num> = self.eval_expr(&node.index)?.try_into()?;

                let index = index.0 as usize;
                list.borrow_mut()
                    .set(index, value)
                    .ok_or_else(|| Error::IndexOutOfBounds(index))?;
            },
            _ => return Err(Error::InvalidAssign.into()),
        }

        Ok(())
    }

    fn eval_if(&mut self, node: &If) -> Result<()> {
        self.with_scope(|this| {
            if this.eval_expr(&node.cond)?.is_truthy() {
                this.eval_block(&node.then_branch)
            } else if let Some(else_branch) = &node.else_branch {
                this.eval_block(else_branch)
            } else {
                Ok(())
            }
        })
    }

    fn eval_while(&mut self, node: &While) -> Result<()> {
        while self.eval_expr(&node.cond)?.is_truthy() {
            self.with_scope(|this| {
                this.eval_block(&node.body)
            })?;
        }

        Ok(())
    }

    fn eval_return(&mut self, node: &Return) -> Result<()> {
        let value = self.eval_expr(&node.value)?;
        Err(Outcome::Return(value))
    }
}
