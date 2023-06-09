use std::{cmp::min, collections::HashMap};

use crate::ast::expr::*;

use super::{
    error::Error, monomorphize::monomorphize, polymorphize::polymorphize, tvars::TVars, Level,
    Polytype, TVar, TVarRef, Type,
};

pub(super) struct Checker {
    pub cur_level: Level,
    pub tvars: TVars,
    pub bindings: HashMap<String, Polytype>,
}

impl Checker {
    pub fn new() -> Checker {
        Checker {
            cur_level: Level(0),
            tvars: TVars::new(),
            bindings: HashMap::new(),
        }
    }

    pub fn check(&mut self, expr: Expr) -> Result<Type, Error> {
        match expr {
            Expr::Int(_) => Ok(Type::Int),
            Expr::Str(_) => Ok(Type::Str),
            Expr::Unit => Ok(Type::Unit),
            Expr::Var(Var { name }) => match self.bindings.get(&name) {
                Some(t) => Ok(monomorphize(self, t.clone())),
                None => Err(Error::NotInScope { name }),
            },
            Expr::App(App { fun, arg }) => {
                let fun_t = self.check(*fun)?;
                let arg_t = self.check(*arg)?;
                let t = self.new_tvar();
                self.unify(fun_t, Type::Fun(Box::new(arg_t), Box::new(t.clone())))?;
                Ok(t)
            }
            Expr::Fun(Fun { arg, body }) => {
                let in_t = self.new_tvar();
                self.bindings
                    .insert(arg.clone(), Polytype(vec![], in_t.clone()));
                let out_t = self.check(*body)?;
                self.bindings.remove(&arg);
                Ok(Type::Fun(Box::new(in_t), Box::new(out_t)))
            }
            Expr::Let(Let { name, value, body }) => {
                self.enter_level();
                let value_t = self.check(*value)?;
                self.exit_level();
                let poly = polymorphize(self, value_t);
                self.bindings.insert(name.clone(), poly);
                let t = self.check(*body)?;
                self.bindings.remove(&name);
                Ok(t)
            }
            Expr::BinOp(BinOp { left, right, kind }) => {
                let op_t = match kind {
                    BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mul | BinOpKind::Div => Type::Fun(
                        Box::new(Type::Int),
                        Box::new(Type::Fun(Box::new(Type::Int), Box::new(Type::Int))),
                    ),
                };
                let left_t = self.check(*left)?;
                let right_t = self.check(*right)?;
                let t = self.new_tvar();
                self.unify(
                    op_t,
                    Type::Fun(
                        Box::new(left_t),
                        Box::new(Type::Fun(Box::new(right_t), Box::new(t.clone()))),
                    ),
                )?;
                Ok(t)
            }
        }
    }

    fn enter_level(&mut self) {
        let Level(level) = self.cur_level;
        self.cur_level = Level(level + 1);
    }

    fn exit_level(&mut self) {
        let Level(level) = self.cur_level;
        self.cur_level = Level(level - 1);
    }

    pub fn new_tvar(&mut self) -> Type {
        Type::Var(
            self.tvars
                .add(|var_ref| TVar::Unbound(var_ref, self.cur_level)),
        )
    }

    fn occurs(&mut self, id: TVarRef, level: Level, typ: Type) -> bool {
        match typ {
            Type::Int | Type::Str | Type::Unit => false,
            Type::Var(tvar) => match self.tvars.get(tvar) {
                TVar::Bound(t) => self.occurs(id, level, t.clone()),
                TVar::Unbound(b_id, b_level) => {
                    let ret = id == *b_id;
                    let min_level = min(level, *b_level);
                    self.tvars.set(tvar, TVar::Unbound(*b_id, min_level));
                    ret
                }
            },
            Type::Fun(a, b) => self.occurs(id, level, *a) || self.occurs(id, level, *b),
        }
    }

    fn unify(&mut self, t1: Type, t2: Type) -> Result<(), Error> {
        match (t1.clone(), t2.clone()) {
            (Type::Int, Type::Int) => Ok(()),
            (Type::Str, Type::Str) => Ok(()),
            (Type::Fun(a, b), Type::Fun(c, d)) => {
                self.unify(*a, *c)?;
                self.unify(*b, *d)
            }
            (Type::Var(tvar), b) if let TVar::Bound(a) = self.tvars.get(tvar) => self.unify(a.clone(), b),
            (a, Type::Var(tvar)) if let TVar::Bound(b) = self.tvars.get(tvar) => self.unify(a, b.clone()),
            (Type::Var(tvar), b) if let TVar::Unbound(a_id, a_level) = self.tvars.get(tvar) => {
                if t1 == t2 {
                    Ok(())
                } else if self.occurs(*a_id, *a_level, b.clone()) {
                    Err(Error::Occurs)
                } else {
                    self.tvars.set(tvar, TVar::Bound(b));
                    Ok(())
                }
            }
            (a, Type::Var(tvar)) if let TVar::Unbound(b_id, b_level) = self.tvars.get(tvar) => {
                if t1 == t2 {
                    Ok(())
                } else if self.occurs(*b_id, *b_level, a.clone()) {
                    Err(Error::Occurs)
                } else {
                    self.tvars.set(tvar, TVar::Bound(a));
                    Ok(())
                }
            }
            _ => Err(Error::NoUnify),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{parser, types::string_of_type::string_of_type};

    use super::{Checker, Polytype, Type};

    fn test(path: &Path) -> Result<String, String> {
        let source = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
        let ast = parser::parse_expr(&source).map_err(|err| err.to_string())?;
        let mut checker = Checker::new();
        let bindings = &mut checker.bindings;
        bindings.insert("x".to_string(), Polytype(vec![], Type::Int));
        bindings.insert("y".to_string(), Polytype(vec![], Type::Int));
        bindings.insert("x'".to_string(), Polytype(vec![], Type::Int));
        bindings.insert("foo".to_string(), Polytype(vec![], Type::Int));
        bindings.insert("a".to_string(), Polytype(vec![], Type::Int));
        bindings.insert("b".to_string(), Polytype(vec![], Type::Int));
        bindings.insert("c".to_string(), Polytype(vec![], Type::Int));
        bindings.insert("d".to_string(), Polytype(vec![], Type::Int));
        bindings.insert("e".to_string(), Polytype(vec![], Type::Int));
        bindings.insert("foo_bar".to_string(), Polytype(vec![], Type::Int));
        let t = checker.check(*ast).map_err(|err| err.to_string())?;
        Ok(string_of_type(&mut checker, t))
    }

    #[test]
    fn types() {
        insta::glob!("..", "snapshot_inputs/exprs/**/*.panda", |path| {
            insta::assert_debug_snapshot!(test(path));
        });
    }
}
