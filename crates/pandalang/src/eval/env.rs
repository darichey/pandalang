use std::collections::HashMap;

use crate::{ast::expr::Expr, value::Value};

#[derive(Clone)]
pub enum BoundValue {
    Value(Value),
    Thunk(Expr),
}

impl PartialEq for BoundValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct Env {
    bindings: HashMap<String, Vec<BoundValue>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn lookup(&self, name: &str) -> Option<BoundValue> {
        let bindings = self.bindings.get(name)?;
        let value = bindings.last()?;
        Some(value.clone()) // TODO: story around cloning here?
    }

    pub fn push_binding(&mut self, name: &String, value: BoundValue) {
        match self.bindings.get_mut(name) {
            Some(current_bindings) => current_bindings.push(value),
            None => {
                self.bindings.insert(name.clone(), vec![value]);
            }
        };
    }

    pub fn pop_binding(&mut self, name: &String) {
        if let Some(current_bindings) = self.bindings.get_mut(name) {
            current_bindings.pop();
        }
    }
}
