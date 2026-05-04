use std::collections::HashMap;
use std::rc::Rc;
use kale_runtime::Error;
use kale_runtime::object::{Closure, Method, Module, Object, StructDef};
use kale_syntax::ast;
use kale_syntax::ast::{Assign, Expr, ExprKind, FnDef, Ident, If, Let, Raise, Return, Stmt, StmtKind, While};
use crate::interpreter::flow::{Result, Signal};
use crate::interpreter::Interpreter;

impl Interpreter {
    pub(super) fn eval_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        self.with_span(
            stmt.span(),
            |this| this.eval_stmt_kind(stmt.inner()),
        )
    }
    
    fn eval_stmt_kind(&mut self, kind: &StmtKind) -> Result<()> {
        match kind {
            StmtKind::Expr(expr) => self.eval_expr_stmt(expr),
            StmtKind::Module(node) => self.eval_module(node),
            StmtKind::Struct(node) => self.eval_struct(node),
            StmtKind::FnDef(node) => Ok(self.eval_fndef(node)),
            StmtKind::Let(node) => self.eval_let(node),
            StmtKind::Assign(node) => self.eval_assign(node),
            StmtKind::If(node) => self.eval_if(node),
            StmtKind::While(node) => self.eval_while(node),
            StmtKind::Return(node) => self.eval_return(node),
            StmtKind::Raise(node) => self.eval_raise(node),
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
        let mut methods = HashMap::new();

        for def in &node.methods {
            let closure = Rc::new(Closure::new(
                def.params.clone(),
                def.body.clone(),
                self.env.clone(),
            ));

            let first_param = closure.params()
                .first()
                .map(Ident::as_str);
            let method = match first_param {
                Some("self") => Method::Closure(closure),
                _ => Method::Static(closure),
            };

            methods.insert(def.ident.clone(), method);
        }

        let def = StructDef::new(node.fields.clone(), methods);

        // same trick as `eval_fndef`: the struct isn't defined yet when we build
        // its methods, so we clone the env, define the struct, then detach
        self.env.define(&node.ident, def.into());
        self.env.detach();

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

        match node.target.inner() {
            ExprKind::Ident(ident) => self.env
                .set(ident, value)
                .map_err(|e| self.error(e).into()),
            ExprKind::Member(node) => {
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
                    .ok_or_else(|| Error::Undefined(node.property.clone()))
                    .map_err(|e| self.error(e).into())
            },
            _ => Err(self.error(Error::InvalidAssign).into()),
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
        Err(self.error(Error::raise(value)).into())
    }
}
