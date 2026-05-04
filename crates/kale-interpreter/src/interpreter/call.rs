use std::collections::HashMap;
use kale_runtime::args::Args;
use kale_runtime::ctx::Ctx;
use kale_runtime::Error;
use kale_runtime::object::{BoundMethod, Closure, Immutable, Method, NativeFn, Object, Struct, StructDef};
use crate::interpreter::flow::{Result, Signal};
use crate::interpreter::Interpreter;

impl Interpreter {
    pub(super) fn call(&mut self, callee: Object, args: Vec<Object>) -> Result<Object> {
        match callee {
            Object::Closure(closure) => self.call_closure(closure, args),
            Object::Bound(bound) => self.call_bound(bound, args),
            Object::StructDef(def) => self.call_struct_def(def, args).map(Into::into),
            Object::NativeFn(native) => self.call_native_fn(native, args),
            _ => Err(self.error(Error::TypeError(
                format!("{} is not callable", callee.ty()),
            )).into()),
        }
    }

    fn call_closure(
        &mut self,
        closure: Immutable<Closure>,
        mut args: Vec<Object>,
    ) -> Result<Object> {
        self.with_env(closure.env().clone(), |this| {
            this.with_scope(|this| {
                args.resize(closure.params().len(), Object::Nil);

                for (param, arg) in closure.params().iter().zip(args) {
                    this.env.define(param, arg);
                }

                match this.eval_block(closure.body()) {
                    Ok(()) => Ok(Object::Nil),
                    Err(Signal::Return(value)) => Ok(value),
                    Err(Signal::Error(e)) => Err(e.into()),
                }
            })
        })
    }

    fn call_bound(
        &mut self,
        bound: Immutable<BoundMethod>,
        mut args: Vec<Object>,
    ) -> Result<Object> {
        match bound.method() {
            Method::Native(native) => {
                let ctx = Ctx::new(self);
                let object = bound.receiver();
                let args = Args::new(&args);
                (native.func)(ctx, object, args).map_err(|e| self.error(e).into())
            },
            Method::Closure(closure) => {
                args.insert(0, bound.receiver());
                self.call_closure(closure, args)
            },
            Method::Static(closure) => {
                self.call_closure(closure, args)
            },
        }
    }

    fn call_struct_def(
        &mut self,
        def: Immutable<StructDef>,
        mut args: Vec<Object>,
    ) -> Result<Struct> {
        let mut fields = HashMap::new();
        args.resize(def.fields().len(), Object::Nil);

        for (field, arg) in def.fields().iter().zip(args) {
            fields.insert(field.clone(), arg);
        }

        Ok(Struct::new(def, fields))
    }

    fn call_native_fn(
        &mut self,
        native: NativeFn,
        args: Vec<Object>,
    ) -> Result<Object> {
        let ctx = Ctx::new(self);
        let args = Args::new(&args);
        (native.func)(ctx, args).map_err(|e| self.error(e).into())
    }
}
