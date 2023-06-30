use std::collections::{hash_map::Entry, HashMap};

use super::{check::Checker, TVar, TVarRef, Type};

struct StringOfType<'a> {
    checker: &'a Checker,
    names: HashMap<TVarRef, String>,
    i: u8,
}

impl<'a> StringOfType<'a> {
    fn new(checker: &'a Checker) -> StringOfType {
        StringOfType {
            checker,
            names: HashMap::new(),
            i: 0,
        }
    }

    fn string_of_type(&mut self, typ: Type) -> String {
        match typ {
            Type::Int => "Int".to_string(),
            Type::Str => "Str".to_string(),
            Type::Unit => "Unit".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::Var(var) => match self.checker.tvars.get(var) {
                TVar::Bound(t) => self.string_of_type(t.clone()),
                TVar::Unbound(var_ref, _) => self.var_name(*var_ref),
            },
            Type::Fun(a, b) => {
                format!(
                    "({} -> {})",
                    self.string_of_type(*a),
                    self.string_of_type(*b)
                )
            }
        }
    }

    fn var_name(&mut self, var_ref: TVarRef) -> String {
        match self.names.entry(var_ref) {
            Entry::Occupied(o) => o.get().clone(),
            Entry::Vacant(v) => {
                let name = format!("'{}", std::str::from_utf8(&[b'a' + self.i]).unwrap());
                v.insert(name.clone());
                self.i += 1;
                name
            }
        }
    }
}

pub(super) fn string_of_type(checker: &Checker, typ: Type) -> String {
    StringOfType::new(checker).string_of_type(typ)
}
