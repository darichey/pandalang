// An implementation of Algorithm J for Hindley-Milner type checking
// Based in part on this implementation: https://github.com/jfecher/algorithm-j/blob/7119150ae1822deac1dfe1dbb14f172d7c75e921/j.ml

use std::{cmp::min, collections::HashMap};

use pandalang_parser::ast::expr::*;

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
            Expr::Bool(_) => Ok(Type::Bool),
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
            Expr::Let(Let {
                name,
                value,
                body,
                rec,
            }) => {
                self.check_let_value(name.clone(), *value, rec)?;
                let t = self.check(*body)?;
                self.bindings.remove(&name);
                Ok(t)
            }
            // TODO: everything after this point can be desugared to the rules above. We should do that to make the type checker simpler
            Expr::BinOp(BinOp { left, right, kind }) => {
                let op_t = match kind {
                    BinOpKind::Add
                    | BinOpKind::Sub
                    | BinOpKind::Mul
                    | BinOpKind::Div
                    | BinOpKind::Rem => Type::Fun(
                        Box::new(Type::Int),
                        Box::new(Type::Fun(Box::new(Type::Int), Box::new(Type::Int))),
                    ),
                    BinOpKind::Eql => {
                        let t = self.new_tvar();
                        Type::Fun(
                            Box::new(t.clone()),
                            Box::new(Type::Fun(Box::new(t), Box::new(Type::Bool))),
                        )
                    }
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
            Expr::If(If { check, then, els }) => {
                let check_t = self.check(*check)?;
                self.unify(check_t, Type::Bool)?;
                let then_t = self.check(*then)?;
                let els_t = self.check(*els)?;
                self.unify(then_t.clone(), els_t)?;
                Ok(then_t)
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
            Type::Int | Type::Str | Type::Unit | Type::Bool => false,
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
            (Type::Bool, Type::Bool) => Ok(()),
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

    pub fn check_let_value(&mut self, name: String, value: Expr, rec: bool) -> Result<(), Error> {
        self.enter_level();
        let value_t = if rec {
            // In the expression `let rec v = e1 in e2`,
            // `e1` is checked with `v` bound to a fresh tvar in monotype position
            let value_t = self.new_tvar();
            self.bindings
                .insert(name.clone(), Polytype(vec![], value_t.clone()));
            self.check(value)?;
            self.bindings.remove(&name);
            value_t
        } else {
            // Otherwise, we have `let v = e1 in e2` where referring to `v` in `e1` is illegal
            self.check(value)?
        };
        self.exit_level();
        let poly = polymorphize(self, value_t);
        self.bindings.insert(name, poly);
        Ok(())
    }

    pub fn insert_declare(&mut self, name: String, typ: Type) {
        let poly = polymorphize(self, typ);
        self.bindings.insert(name, poly);
    }

    pub fn get_bindings(&mut self) -> HashMap<String, Type> {
        self.bindings
            .clone()
            .into_iter()
            .map(|(name, typ)| (name, monomorphize(self, typ)))
            .collect()
    }
}
