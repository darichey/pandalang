use std::collections::HashSet;

use super::{check::Checker, Polytype, TVar, TVarRef, Type};

struct Polymorphize<'a> {
    checker: &'a mut Checker,
    vars: HashSet<TVarRef>,
}

impl<'a> Polymorphize<'a> {
    fn new(checker: &'a mut Checker) -> Polymorphize {
        Polymorphize {
            checker,
            vars: HashSet::new(),
        }
    }

    fn polymorphize(mut self, typ: Type) -> Polytype {
        self.collect_vars(typ.clone());
        let vars = self.vars.into_iter().collect();
        Polytype(vars, typ)
    }

    fn collect_vars(&mut self, typ: Type) {
        match typ {
            Type::Int => (),
            Type::Str => (),
            Type::Var(tvar) => match self.checker.tvars.get(tvar) {
                TVar::Bound(t) => self.collect_vars(t.clone()),
                TVar::Unbound(id, level) => {
                    if level > &self.checker.cur_level {
                        self.vars.insert(*id);
                    }
                }
            },
            Type::Fun(a, b) => {
                self.collect_vars(*a);
                self.collect_vars(*b);
            }
        }
    }
}

pub(super) fn polymorphize(checker: &mut Checker, typ: Type) -> Polytype {
    Polymorphize::new(checker).polymorphize(typ)
}
