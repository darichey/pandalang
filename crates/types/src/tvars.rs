use super::{TVar, TVarRef};

pub(super) struct TVars {
    vars: Vec<TVar>,
}

impl TVars {
    pub fn new() -> TVars {
        TVars { vars: Vec::new() }
    }

    pub fn add<F: Fn(TVarRef) -> TVar>(&mut self, f: F) -> TVarRef {
        let next_ref = TVarRef(self.vars.len());
        self.vars.push(f(next_ref));
        next_ref
    }

    pub fn get(&self, var_ref: TVarRef) -> &TVar {
        self.vars
            .get(var_ref.0)
            .unwrap_or_else(|| panic!("No such var: {:?}", var_ref))
    }

    pub fn set(&mut self, var_ref: TVarRef, x: TVar) {
        match self.vars.get_mut(var_ref.0) {
            Some(cur) => *cur = x,
            None => panic!("No such var: {:?}", var_ref),
        }
    }
}
