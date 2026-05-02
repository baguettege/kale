use std::collections::HashMap;
use std::rc::Rc;
use kale_runtime::Error;
use kale_runtime::object::{Closure, Method, Module, Object, StructDef};
use kale_syntax::ast;
use kale_syntax::ast::{Assign, Expr, FnDef, If, Let, Raise, Return, Stmt, While};
use crate::interpreter::flow::{Result, Signal};
use crate::interpreter::Interpreter;

impl Interpreter {
    pub(super) fn eval_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Expr(expr) => self.eval_expr_stmt(expr),
            Stmt::Module(node) => self.eval_module(node),
            Stmt::Struct(node) => self.eval_struct(node),
            Stmt::FnDef(node) => Ok(self.eval_fndef(node)),
            Stmt::Let(node) => self.eval_let(node),
            Stmt::Assign(node) => self.eval_assign(node),
            Stmt::If(node) => self.eval_if(node),
            Stmt::While(node) => self.eval_while(node),
            Stmt::Return(node) => self.eval_return(node),
            Stmt::Raise(node) => self.eval_raise(node),
        }
    }

    fn eval_expr_stmt(&mut self, expr: &Expr) -> Result<()> {
        self.eval_expr(expr).map(|_| ())
    }

    fn eval_module(&mut self, node: &ast::Module) -> Result<()> {
        let capture = self.with_env(self.env.fork(), |this| {
            this.eval_block(&node.body)?;
            Ok(this.env.capture())
        })?;

        let module = Module::from(capture).into();
        self.env.define(&node.ident, module);
        Ok(())
    }

    fn eval_struct(&mut self, node: &ast::Struct) -> Result<()> {
        let mut methods = HashMap::<ast::Ident, Method>::new();

        for def in &node.methods {
            // note that we don't require defining the closure in its own captured
            // `Env`, as methods are looked up via `StructDef`, not `Env` like
            // with free functions, which require this for recursion
            let closure = Closure::new(
                def.params.clone(),
                def.body.clone(),
                self.env.clone(),
            );

            let method = Method::Closure(Rc::new(closure));
            methods.insert(def.ident.clone(), method);
        }

        let def = StructDef::new(node.fields.clone(), methods);
        self.env.define(&node.ident, def.into());

        Ok(())
    }

    fn eval_fndef(&mut self, node: &FnDef) {
        let closure = Closure::new(
            node.params.clone(),
            node.body.clone(),
            // shallow clone: `closure.env.locals` share the
            // same allocation with `self.env`
            self.env.clone(),
        );

        // inject self-reference through the shared `Rc`, both
        // `self.env` and `closure.env` see this write
        self.env.define(&node.ident, closure.into());

        // detach `self.env` - gives `self.env` a new independent
        // allocation and `closure.env` still holds the original `Rc`
        self.env.detach();
    }

    fn eval_let(&mut self, node: &Let) -> Result<()> {
        let init = self.eval_expr(&node.init)?;
        self.env.define(&node.ident, init);
        Ok(())
    }

    fn eval_assign(&mut self, node: &Assign) -> Result<()> {
        let value = self.eval_expr(&node.value)?;

        match &node.target {
            Expr::Ident(ident) => self.env
                .set(ident, value)
                .map_err(Into::into),
            Expr::Member(node) => {
                let result = match self.eval_expr(&node.object)? {
                    Object::Module(module) => module
                        .borrow_mut()
                        .set(&node.property, value),
                    Object::Struct(instance) => instance
                        .borrow_mut()
                        .set(&node.property, value),
                    _ => None,
                };

                result
                    .ok_or_else(|| Error::UndefinedVariable(node.property.clone()))
                    .map_err(Into::into)
            },
            _ => Err(Error::InvalidAssign.into()),
        }
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
        Err(Signal::Return(value))
    }

    fn eval_raise(&mut self, node: &Raise) -> Result<()> {
        let value = self.eval_expr(&node.value)?;
        Err(Error::raise(value).into())
    }
}
