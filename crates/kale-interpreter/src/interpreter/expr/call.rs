use kale_runtime::args::Args;
use kale_runtime::object::{BoundMethod, Closure, Function, Method, Mutable, Nil, Object};
use crate::interpreter::flow::{Outcome, Result};
use crate::interpreter::Interpreter;

impl Interpreter {
    pub(super) fn call_function(
        &mut self,
        function: &Function,
        mut args: Vec<Object>,
    ) -> Result<Object> {
        args.resize(function.params.len(), Nil.into());
        for (param, arg) in function.params.iter().zip(args) {
            self.env.define(param, arg);
        }

        match self.eval_block(&function.body) {
            Ok(()) => Ok(Nil.into()),
            Err(Outcome::Return(value)) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub(super) fn call_closure(
        &mut self,
        closure: Mutable<Closure>,
        mut args: Vec<Object>,
    ) -> Result<Object> {
        let (params, body, env) = {
            let closure = closure.borrow();
            (
                closure.params().to_vec(),
                closure.body().clone(),
                closure.env.clone(),
            )
        };

        let prev_env = std::mem::replace(&mut self.env, env);
        self.env.enter_scope(); // scope is discarded when prev_env is restored below

        args.resize(params.len(), Nil.into());
        for (param, arg) in params.iter().zip(args) {
            self.env.define(param, arg);
        }

        let result = match self.eval_block(&body) {
            Ok(()) => Ok(Nil.into()),
            Err(Outcome::Return(value)) => Ok(value),
            Err(e) => Err(e),
        };

        closure.borrow_mut().env = self.env.clone();
        self.env = prev_env;
        result
    }

    pub(super) fn call_bound(
        &mut self,
        bound: &BoundMethod,
        mut args: Vec<Object>,
    ) -> Result<Object> {
        args.insert(0, bound.receiver.clone());

        match &bound.method {
            Method::Builtin(builtin) => Ok((builtin.func)(Args(&args))?),
        }
    }
}
