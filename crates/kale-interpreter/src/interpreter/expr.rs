use crate::interpreter::flow::Result;
use crate::interpreter::Interpreter;
use kale_runtime::Error;
use kale_runtime::object::{BoundMethod, Closure, Method, Object};
use kale_syntax::ast;
use kale_syntax::ast::{BinOp, Binary, Call, Expr, ExprKind, Ident, Literal, Member, UnOp, Unary};

impl Interpreter {
    pub(super) fn eval_expr(&mut self, expr: &Expr) -> Result<Object> {
        self.with_span(
            expr.span(),
            |this| this.eval_expr_kind(expr.inner()),
        )
    }

    fn eval_expr_kind(&mut self, kind: &ExprKind) -> Result<Object> {
        match kind {
            ExprKind::Literal(literal) => Ok(self.eval_literal(literal)),
            ExprKind::Ident(ident) => self.eval_ident(ident),
            ExprKind::Call(node) => self.eval_call(node),
            ExprKind::Binary(node) => self.eval_binary(node),
            ExprKind::Unary(node) => self.eval_unary(node),
            ExprKind::List(node) => self.eval_list(node).map(Into::into),
            ExprKind::Closure(node) => Ok(self.eval_closure(node).into()),
            ExprKind::Member(node) => self.eval_member(node),
        }
    }

    fn eval_literal(&mut self, literal: &Literal) -> Object {
        match literal {
            Literal::Nil => Object::Nil,
            Literal::Num(n) => (*n).into(),
            Literal::Bool(b) => (*b).into(),
            Literal::Char(c) => (*c).into(),
            Literal::Str(s) => s.into(),
        }
    }

    fn eval_ident(&mut self, ident: &Ident) -> Result<Object> {
        self.env.lookup(ident).map_err(|e| self.error(e).into())
    }

    fn eval_call(&mut self, node: &Call) -> Result<Object> {
        let callee = self.eval_expr(&node.callee)?;
        let args = node.args
            .iter()
            .map(|arg| self.eval_expr(arg))
            .collect::<Result<Vec<_>>>()?;
        self.call(callee, args)
    }

    fn eval_binary(&mut self, node: &Binary) -> Result<Object> {
        let lhs = self.eval_expr(&node.lhs)?;
        let rhs = self.eval_expr(&node.rhs)?;

        let result = match (lhs, node.op, rhs) {
            (l, BinOp::Is, r) => l.is(&r).into(),

            (l, BinOp::And, r) => if l.is_truthy() { r } else { l },
            (l, BinOp::Or, r) => if l.is_truthy() { l } else { r },

            (Object::Nil, BinOp::Eq, Object::Nil) => true.into(),
            (Object::Nil, BinOp::Ne, Object::Nil) => false.into(),

            (Object::Nil, BinOp::Eq, _) | (_, BinOp::Eq, Object::Nil) => false.into(),
            (Object::Nil, BinOp::Ne, _) | (_, BinOp::Ne, Object::Nil) => true.into(),

            (Object::Num(l), BinOp::Eq, Object::Num(r)) => (l == r).into(),
            (Object::Num(l), BinOp::Ne, Object::Num(r)) => (l != r).into(),
            (Object::Num(l), BinOp::Mul, Object::Num(r)) => (l * r).into(),
            (Object::Num(l), BinOp::Div, Object::Num(r)) => (l / r).into(),
            (Object::Num(l), BinOp::Mod, Object::Num(r)) => (l % r).into(),
            (Object::Num(l), BinOp::Add, Object::Num(r)) => (l + r).into(),
            (Object::Num(l), BinOp::Sub, Object::Num(r)) => (l - r).into(),
            (Object::Num(l), BinOp::Lt, Object::Num(r)) => (l < r).into(),
            (Object::Num(l), BinOp::Le, Object::Num(r)) => (l <= r).into(),
            (Object::Num(l), BinOp::Gt, Object::Num(r)) => (l > r).into(),
            (Object::Num(l), BinOp::Ge, Object::Num(r)) => (l >= r).into(),

            (Object::Bool(l), BinOp::Eq, Object::Bool(r)) => (l == r).into(),
            (Object::Bool(l), BinOp::Ne, Object::Bool(r)) => (l != r).into(),

            (Object::Char(l), BinOp::Eq, Object::Char(r)) => (l == r).into(),
            (Object::Char(l), BinOp::Ne, Object::Char(r)) => (l != r).into(),

            (Object::Str(l), BinOp::Eq, Object::Str(r)) => (l == r).into(),
            (Object::Str(l), BinOp::Ne, Object::Str(r)) => (l != r).into(),
            (Object::Str(l), BinOp::Add, Object::Str(r)) => format!("{l}{r}").into(),

            (l, op, r) => return Err(self.error(Error::TypeError(
                format!("cannot apply `{op}` to {} and {}", l.ty(), r.ty()),
            )).into()),
        };

        Ok(result)
    }

    fn eval_unary(&mut self, node: &Unary) -> Result<Object> {
        let object = self.eval_expr(&node.expr)?;

        let result = match (node.op, object) {
            (UnOp::Neg, Object::Num(n)) => (-n).into(),
            (UnOp::Not, Object::Bool(b)) => (!b).into(),

            (op, object) => return Err(self.error(Error::TypeError(
                format!("cannot apply `{op}` to {}", object.ty()),
            )).into()),
        };

        Ok(result)
    }

    fn eval_list(&mut self, node: &ast::List) -> Result<Vec<Object>> {
        node.elements
            .iter()
            .map(|expr| self.eval_expr(expr))
            .collect()
    }

    fn eval_closure(&mut self, node: &ast::Closure) -> Closure {
        Closure::new(
            node.params.clone(),
            node.body.clone(),
            self.env.isolate(),
        )
    }

    fn eval_member(&mut self, node: &Member) -> Result<Object> {
        let object = self.eval_expr(&node.object)?;
        let property = &node.property;

        // native methods take priority
        if let Some(method) = object
            .methods()
            .iter()
            .find(|m| m.ident == property.as_str())
        {
            let bound = BoundMethod::new(object, Method::Native(*method));
            return Ok(bound.into());
        }

        let result = match object {
            Object::Module(module) => module
                .borrow()
                .lookup(property),
            Object::StructDef(def) => {
                match def.method(property) {
                    Some(Method::Static(closure)) => Some(Object::Closure(closure)),
                    Some(_) => return Err(self.error(Error::TypeError(
                        format!("cannot call instance method {property} on a type"),
                    )).into()),
                    None => None,
                }
            },
            Object::Struct(instance) => {
                let receiver: Object = instance.clone().into();
                let instance = instance.borrow();

                instance.get(property)
                    .or_else(|| instance
                        .def()
                        .method(property)
                        .map(|m| BoundMethod::new(receiver, m).into())
                    )
            },
            _ => return Err(self.error(Error::TypeError(
                format!("no member `{property}` on {}", object.ty()),
            )).into()),
        };

        result
            .ok_or_else(|| Error::Undefined(property.clone()))
            .map_err(|e| self.error(e).into())
    }
}