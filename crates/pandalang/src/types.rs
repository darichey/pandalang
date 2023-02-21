use std::{
    cmp::min,
    collections::{hash_map::Entry, HashMap, HashSet},
};

use crate::{
    ast::{self, Expr},
    managed_vec::{Idx, ManagedVec},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Level(usize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub enum Type {
    Int,
    Str,
    Var(Idx),
    Fun(Box<Type>, Box<Type>),
}

enum TVar {
    Bound(Type),
    Unbound(Idx, Level),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
struct Polytype(Vec<Idx>, Type);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub enum Error {
    NotInScope { name: String },
    NoUnify,
    Occurs,
}

pub struct Checker {
    cur_level: Level,
    tvars: ManagedVec<TVar>,
    bindings: HashMap<String, Polytype>,
}

impl Checker {
    pub fn new() -> Checker {
        Checker {
            cur_level: Level(0),
            tvars: ManagedVec::new(),
            bindings: HashMap::new(),
        }
    }

    pub fn check(&mut self, expr: Expr) -> Result<Type, Error> {
        match expr {
            Expr::Int(_) => Ok(Type::Int),
            Expr::Str(_) => Ok(Type::Str),
            Expr::Var(ast::Var { name }) => match self.bindings.get(&name) {
                Some(t) => Ok(self.monomorphize(t.clone())),
                None => Err(Error::NotInScope { name }),
            },
            Expr::App(ast::App { fun, arg }) => {
                let fun_t = self.check(*fun)?;
                let arg_t = self.check(*arg)?;
                let t = self.new_tvar();
                self.unify(fun_t, Type::Fun(Box::new(arg_t), Box::new(t.clone())))?;
                Ok(t)
            }
            Expr::Fun(ast::Fun { patt, body }) => {
                let ast::Pattern::Id { name } = patt;
                let in_t = self.new_tvar();
                self.bindings
                    .insert(name.clone(), Polytype(vec![], in_t.clone()));
                let out_t = self.check(*body)?;
                self.bindings.remove(&name);
                Ok(Type::Fun(Box::new(in_t), Box::new(out_t)))
            }
            Expr::Let(ast::Let { patt, value, body }) => {
                let ast::Pattern::Id { name } = patt;
                self.enter_level();
                let value_t = self.check(*value)?;
                self.exit_level();
                let poly = self.polymorphize(value_t);
                self.bindings.insert(name.clone(), poly);
                let t = self.check(*body)?;
                self.bindings.remove(&name);
                Ok(t)
            }
            Expr::BinOp(ast::BinOp { left, right, kind }) => {
                let op_t = match kind {
                    ast::BinOpKind::Add
                    | ast::BinOpKind::Sub
                    | ast::BinOpKind::Mul
                    | ast::BinOpKind::Div => Type::Fun(
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

    pub fn string_of_type(&mut self, typ: Type) -> String {
        StringOfType::new(self).string_of_type(typ)
    }

    fn enter_level(&mut self) {
        let Level(level) = self.cur_level;
        self.cur_level = Level(level + 1);
    }

    fn exit_level(&mut self) {
        let Level(level) = self.cur_level;
        self.cur_level = Level(level - 1);
    }

    fn new_tvar(&mut self) -> Type {
        Type::Var(self.tvars.add(|idx| TVar::Unbound(idx, self.cur_level)))
    }

    fn monomorphize(&mut self, poly: Polytype) -> Type {
        Monomorphize::new(self).monomorphize(poly)
    }

    fn polymorphize(&mut self, typ: Type) -> Polytype {
        Polymorphize::new(self).polymorphize(typ)
    }

    fn occurs(&mut self, id: Idx, level: Level, typ: Type) -> bool {
        match typ {
            Type::Int => false,
            Type::Str => false,
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

struct Monomorphize<'a> {
    checker: &'a mut Checker,
    to_replace: HashMap<Idx, Type>,
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
            Type::Var(tvar) => match self.checker.tvars.get(tvar) {
                TVar::Bound(t) => self.replace(t.clone()),
                TVar::Unbound(id, _) => match self.to_replace.get(&id) {
                    Some(t) => t.clone(),
                    None => typ.clone(),
                },
            },
            Type::Fun(a, b) => Type::Fun(Box::new(self.replace(*a)), Box::new(self.replace(*b))),
        }
    }
}

struct Polymorphize<'a> {
    checker: &'a mut Checker,
    vars: HashSet<Idx>,
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

struct StringOfType<'a> {
    checker: &'a mut Checker,
    names: HashMap<Idx, String>,
    i: u8,
}

impl<'a> StringOfType<'a> {
    fn new(checker: &'a mut Checker) -> StringOfType {
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
            Type::Var(var) => match self.checker.tvars.get(var) {
                TVar::Bound(t) => self.string_of_type(t.clone()),
                TVar::Unbound(idx, _) => self.var_name(*idx),
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

    fn var_name(&mut self, idx: Idx) -> String {
        match self.names.entry(idx) {
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
