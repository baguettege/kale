use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use kale_syntax::ast::Ident;
use crate::object::Object;
use crate::{Error, Result};

pub type Scope = HashMap<Ident, Object>;

#[derive(Debug, Clone)]
pub struct Globals(Scope);

// holds an owned clone of `Env` to allow for mutable access
// without locking the `Env` via a borrow
pub struct ScopeGuard(Env);

// cloning `Env` will provide a shallow clone, allowing
// for mutability of locals, used for persistence between closure calls
#[derive(Debug, Clone)]
pub struct Env {
    locals: Rc<RefCell<Vec<Scope>>>,
    globals: Rc<Globals>,
}

impl Globals {
    pub fn new() -> Self {
        Self(Scope::new())
    }

    pub fn define(&mut self, ident: impl Into<Ident>, object: impl Into<Object>) {
        self.0.insert(ident.into(), object.into());
    }
}

impl From<Globals> for Scope {
    fn from(globals: Globals) -> Self {
        globals.0
    }
}

impl ScopeGuard {
    fn new(mut env: Env) -> Self {
        env.push_scope();
        Self(env)
    }
}

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        self.0.pop_scope();
    }
}

impl Env {
    fn from_parts(locals: Vec<Scope>, globals: Rc<Globals>) -> Self {
        let locals = Rc::new(RefCell::new(locals));
        Self { locals, globals }
    }

    fn push_scope(&mut self) {
        self.locals.borrow_mut().push(Scope::new());
    }

    fn pop_scope(&mut self) {
        let mut locals = self.locals.borrow_mut();
        assert!(locals.len() > 1, "outermost local scope cannot be exited");
        locals.pop();
    }
}

impl Env {
    pub fn new(globals: Globals) -> Self {
        Self::from_parts(vec![Scope::new()], Rc::new(globals))
    }

    pub fn enter_scope(&mut self) -> ScopeGuard {
        // let the guard push the scope
        ScopeGuard::new(self.clone())
    }

    pub fn define(&mut self, ident: impl Into<Ident>, object: Object) {
        self.locals.borrow_mut()
            .last_mut()
            .unwrap()
            .insert(ident.into(), object);
    }

    pub fn set(&mut self, ident: impl Into<Ident>, object: Object) -> Result<()> {
        let ident = ident.into();

        if let Some(scope) = self.locals
            .borrow_mut()
            .iter_mut()
            .rev()
            .find(|sc| sc.contains_key(&ident))
        {
            scope.insert(ident.clone(), object);
            Ok(())
        } else {
            Err(Error::UndefinedVariable(ident))
        }
    }

    pub fn lookup(&self, ident: &Ident) -> Result<Object> {
        self.locals.borrow()
            .iter()
            .rev()
            .find_map(|sc| sc.get(ident))
            .or_else(|| self.globals.0.get(ident))
            .cloned()
            .ok_or_else(|| Error::UndefinedVariable(ident.clone()))
    }
}

impl Env {
    pub fn detach(&mut self) {
        // break shared locals
        let snapshot = self.locals.borrow().clone();
        self.locals = Rc::new(RefCell::new(snapshot));
    }

    pub fn capture(&self) -> Scope {
        // flatten all local scopes into one map (inner overrides outer)
        let mut capture = Scope::new();
        for scope in self.locals.borrow().iter() {
            capture.extend(scope.clone());
        }
        capture
    }

    pub fn fork(&self) -> Self {
        // shared globals, fresh locals
        Self::from_parts(vec![Scope::new()], Rc::clone(&self.globals))
    }

    pub fn isolate(&self) -> Self {
        // shared globals, independent locals
        Self::from_parts(self.locals.borrow().clone(), Rc::clone(&self.globals))
    }
}
