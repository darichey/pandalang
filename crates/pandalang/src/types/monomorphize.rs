use std::collections::HashMap;

use super::{check::Checker, Polytype, TVar, TVarRef, Type};

struct Monomorphize<'a> {
    checker: &'a mut Checker,
    to_replace: HashMap<TVarRef, Type>,
}

impl<'a> Monomorphize<'a> {
    fn new(checker: &mut Checker) -> Monomorphize {
        Monomorphize {
            checker,
            to_replace: HashMap::new(),
        }
    }

    fn monomorphize(mut self, poly: Polytype) -> Type {
        let Polytype(vars, typ) = poly;
        for tvar_id in vars {
            self.to_replace.insert(tvar_id, self.checker.new_tvar());
        }
        self.replace(typ)
    }

    fn replace(&mut self, typ: Type) -> Type {
        match typ {
            Type::Int => Type::Int,
            Type::Str => Type::Str,
            Type::Unit => Type::Unit,
            Type::Bool => Type::Bool,
            Type::Var(tvar) => match self.checker.tvars.get(tvar) {
                TVar::Bound(t) => self.replace(t.clone()),
                TVar::Unbound(id, _) => match self.to_replace.get(id) {
                    Some(t) => t.clone(),
                    None => typ.clone(),
                },
            },
            Type::Fun(a, b) => Type::Fun(Box::new(self.replace(*a)), Box::new(self.replace(*b))),
        }
    }
}

pub(super) fn monomorphize(checker: &mut Checker, poly: Polytype) -> Type {
    Monomorphize::new(checker).monomorphize(poly)
}
