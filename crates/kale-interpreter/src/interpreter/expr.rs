mod call;

use crate::interpreter::flow::Result;
use crate::interpreter::Interpreter;
use kale_runtime::args::Args;
use kale_runtime::object::{Bool, BoundMethod, Closure, Frozen, List, Method, Mutable, Nil, Num, Object, Str};
use kale_runtime::Error;
use kale_syntax::ast;
use kale_syntax::ast::{BinOp, Binary, Call, Expr, Ident, Index, Literal, Member, UnOp, Unary};
use std::rc::Rc;

impl Interpreter {
    pub(super) fn eval_expr(&mut self, expr: &Expr) -> Result<Object> {
        match expr {
            Expr::Literal(literal) => Ok(self.eval_literal(literal)),
            Expr::Ident(ident) => self.eval_ident(ident),
            Expr::Call(node) => self.eval_call(node),
            Expr::Binary(node) => self.eval_binary(node),
            Expr::Unary(node) => self.eval_unary(node),
            Expr::List(node) => self.eval_list(node).map(Into::into),
            Expr::Closure(node) => Ok(self.eval_closure(node).into()),
            Expr::Member(node) => self.eval_member(node),
            Expr::Index(node) => self.eval_index(node),
        }
    }

    fn eval_literal(&mut self, literal: &Literal) -> Object {
        match literal {
            Literal::Nil => Nil.into(),
            Literal::Num(n) => Num(*n).into(),
            Literal::Bool(b) => Bool(*b).into(),
            Literal::Str(s) => Str::new(s).into(),
        }
    }

    fn eval_ident(&mut self, ident: &Ident) -> Result<Object> {
        let object = self.env.lookup(ident)?;
        Ok(object)
    }

    fn eval_call(&mut self, node: &Call) -> Result<Object> {
        let callee = self.eval_expr(&node.callee)?;
        let args = node.args
            .iter()
            .map(|arg| self.eval_expr(arg))
            .collect::<Result<Vec<_>>>()?;

        match callee {
            Object::Function(function) => self.call_function(&function, args),
            Object::Closure(closure) => self.call_closure(closure, args),
            Object::Builtin(builtin) => Ok((builtin.func)(Args(&args))?),
            Object::Method(bound) => self.call_bound(&bound, args),
            _ => Err(Error::TypeError(
                format!("{} is not callable", callee.type_name())
            ).into()),
        }
    }

    fn eval_binary(&mut self, node: &Binary) -> Result<Object> {
        fn ref_eq(a: &Object, b: &Object) -> bool {
            match (a, b) {
                (Object::Nil(_), Object::Nil(_)) => true,
                (Object::Num(a), Object::Num(b)) => Rc::ptr_eq(a, b),
                (Object::Bool(a), Object::Bool(b)) => Rc::ptr_eq(a, b),
                (Object::Str(a), Object::Str(b)) => Rc::ptr_eq(a, b),
                (Object::List(a), Object::List(b)) => Rc::ptr_eq(a, b),
                (Object::Closure(a), Object::Closure(b)) => Rc::ptr_eq(a, b),
                (Object::Module(a), Object::Module(b)) => Rc::ptr_eq(a, b),
                _ => false,
            }
        }

        let lhs = self.eval_expr(&node.lhs)?;
        let rhs = self.eval_expr(&node.rhs)?;

        let result = match (lhs, node.op, rhs) {
            (lhs, BinOp::Is, rhs) => Bool(ref_eq(&lhs, &rhs)).into(),

            (Object::Nil(_), BinOp::Eq, Object::Nil(_)) => Bool(true).into(),
            (Object::Nil(_), BinOp::Ne, Object::Nil(_)) => Bool(false).into(),
            (Object::Num(lhs), BinOp::Eq, Object::Num(rhs)) => Bool(lhs.0 == rhs.0).into(),
            (Object::Num(lhs), BinOp::Ne, Object::Num(rhs)) => Bool(lhs.0 != rhs.0).into(),
            (Object::Bool(lhs), BinOp::Eq, Object::Bool(rhs)) => Bool(lhs.0 == rhs.0).into(),
            (Object::Bool(lhs), BinOp::Ne, Object::Bool(rhs)) => Bool(lhs.0 != rhs.0).into(),
            (Object::Str(lhs), BinOp::Eq, Object::Str(rhs)) => Bool(lhs.as_str() == rhs.as_str()).into(),
            (Object::Str(lhs), BinOp::Ne, Object::Str(rhs)) => Bool(lhs.as_str() != rhs.as_str()).into(),

            (Object::Num(lhs), BinOp::Mul, Object::Num(rhs)) => Num(lhs.0 * rhs.0).into(),
            (Object::Num(lhs), BinOp::Div, Object::Num(rhs)) => Num(lhs.0 / rhs.0).into(),
            (Object::Num(lhs), BinOp::Add, Object::Num(rhs)) => Num(lhs.0 + rhs.0).into(),
            (Object::Num(lhs), BinOp::Sub, Object::Num(rhs)) => Num(lhs.0 - rhs.0).into(),
            (Object::Num(lhs), BinOp::Lt, Object::Num(rhs)) => Bool(lhs.0 < rhs.0).into(),
            (Object::Num(lhs), BinOp::Le, Object::Num(rhs)) => Bool(lhs.0 <= rhs.0).into(),
            (Object::Num(lhs), BinOp::Gt, Object::Num(rhs)) => Bool(lhs.0 > rhs.0).into(),
            (Object::Num(lhs), BinOp::Ge, Object::Num(rhs)) => Bool(lhs.0 >= rhs.0).into(),

            (lhs, BinOp::And, rhs) => if lhs.is_truthy() { rhs } else { lhs },
            (lhs, BinOp::Or, rhs) => if lhs.is_truthy() { lhs } else { rhs },

            (Object::Str(lhs), BinOp::Add, Object::Str(rhs)) => Str::new(format!("{lhs}{rhs}")).into(),

            (lhs, op, rhs) => return Err(Error::TypeError(
                format!("cannot apply '{op}' to {} and {}", lhs.type_name(), rhs.type_name())
            ).into()),
        };

        Ok(result)
    }

    fn eval_unary(&mut self, node: &Unary) -> Result<Object> {
        let object = self.eval_expr(&node.expr)?;

        let result = match (node.op, object) {
            (UnOp::Neg, Object::Num(n)) => Num(-n.0).into(),
            (UnOp::Not, Object::Bool(b)) => Bool(!b.0).into(),

            (op, obj) => return Err(Error::TypeError(
                format!("cannot apply '{op}' to {}", obj.type_name())
            ).into()),
        };

        Ok(result)
    }

    fn eval_list(&mut self, node: &ast::List) -> Result<List> {
        let elements = node.elements
            .iter()
            .map(|expr| self.eval_expr(expr))
            .collect::<Result<_>>()?;
        Ok(List::new(elements))
    }

    fn eval_closure(&mut self, node: &ast::Closure) -> Closure {
        let params= node.params.clone();
        let body = node.body.clone();
        let env = self.env.clone();
        Closure::new(params, body, env)
    }

    fn eval_member(&mut self, node: &Member) -> Result<Object> {
        let object = self.eval_expr(&node.object)?;
        let property = &node.property;

        let method = object.methods()
            .iter()
            .find(|b| b.ident == property.as_str());

        if let Some(method) = method {
            let bound = BoundMethod::new(object, Method::Builtin(method));
            Ok(bound.into())
        } else if let Object::Module(module) = object {
            let object = module.borrow()
                .lookup(property)
                .ok_or_else(|| Error::UndefinedVariable(property.clone()))?;
            Ok(object)
        } else {
            Err(Error::TypeError(
                format!("no member '{property}' on {}", object.type_name())
            ).into())
        }
    }

    fn eval_index(&mut self, node: &Index) -> Result<Object> {
        let list: Mutable<List> = self.eval_expr(&node.object)?.try_into()?;
        let index: Frozen<Num> = self.eval_expr(&node.index)?.try_into()?;

        let index = index.0 as usize;
        let object = list.borrow()
            .get(index)
            .ok_or_else(|| Error::IndexOutOfBounds(index))?;
        Ok(object)
    }
}
