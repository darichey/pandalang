mod builtins;

use rpds::HashTrieMap;

use crate::ast::expr::{App, BinOp, BinOpKind, Bool, Expr, Fun, If, Int, Let, Var};
use crate::ast::stmt::Stmt;
use crate::ast::{stmt, Program};
use crate::value::Value;

pub fn run_program(program: Program) -> Result<Value, String> {
    let env =
        program
            .stmts
            .into_iter()
            .try_fold(Env::new(), |env, stmt| -> Result<Env, String> {
                match stmt {
                    Stmt::Let(stmt::Let { name, value, rec }) => {
                        let value = env.eval_let_value(name.clone(), *value, rec)?;
                        Ok(env.with_binding(name, value))
                    }
                    Stmt::Declare(stmt::Declare { name, .. }) => {
                        Ok(env.with_binding(name.clone(), BoundValue::Value(Value::Builtin(name))))
                    }
                }
            })?;

    check_fully_evaluated(env.lookup("main").ok_or("Couldn't find main")?)
}

pub fn eval(env: Env, expr: Expr) -> Result<Value, String> {
    check_fully_evaluated(env.eval(expr)?)
}

fn check_fully_evaluated(v: BoundValue) -> Result<Value, String> {
    match v {
        BoundValue::Value(v) => Ok(v),
        BoundValue::Thunk(_) => Err("Final value should be fully evaluated".to_string()),
    }
}

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

macro_rules! bindings {
    ($($k:expr => $v:expr),* $(,)?) => {{
        rpds::HashTrieMap::from_iter([$(($k.to_string(), BoundValue::Value(Value::Int(Int { n: $v }))),)*])
    }};
}

#[derive(Clone)]
pub struct Env {
    pub bindings: HashTrieMap<String, BoundValue>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            bindings: bindings!(),
        }
    }

    fn eval(&self, expr: Expr) -> Result<BoundValue, String> {
        match expr {
            Expr::Int(n) => Ok(BoundValue::Value(Value::Int(n))),
            Expr::Str(s) => Ok(BoundValue::Value(Value::Str(s))),
            Expr::Unit => Ok(BoundValue::Value(Value::Unit)),
            Expr::Bool(b) => Ok(BoundValue::Value(Value::Bool(b))),
            Expr::Var(Var { name }) => Ok(self
                .lookup(&name)
                .ok_or(format!("{} is not bound!", name))?),
            Expr::BinOp(BinOp { left, right, kind }) => match kind {
                BinOpKind::Add => self.eval_arith(*left, *right, std::ops::Add::add),
                BinOpKind::Sub => self.eval_arith(*left, *right, std::ops::Sub::sub),
                BinOpKind::Mul => self.eval_arith(*left, *right, std::ops::Mul::mul),
                BinOpKind::Div => self.eval_arith(*left, *right, std::ops::Div::div),
                BinOpKind::Rem => self.eval_arith(*left, *right, std::ops::Rem::rem),
                BinOpKind::Eql => {
                    let left = self.eval(*left)?;
                    let right = self.eval(*right)?;
                    Ok(BoundValue::Value(Value::Bool(Bool { b: left == right })))
                }
            },
            Expr::Fun(fun) => Ok(BoundValue::Value(Value::Fun {
                fun,
                env: self.clone(),
            })),
            Expr::App(App { fun, arg }) => match self.eval(*fun)? {
                BoundValue::Value(Value::Fun {
                    fun:
                        Fun {
                            arg: arg_name,
                            body,
                        },
                    env: fun_env,
                }) => {
                    let arg = self.eval(*arg)?;
                    fun_env.with_binding(arg_name, arg).eval(*body)
                }
                BoundValue::Value(Value::Builtin(builtin)) => {
                    let arg = self.eval(*arg)?;
                    builtins::eval(builtin, arg)
                }
                BoundValue::Thunk(expr) => self.eval(Expr::App(App {
                    fun: Box::new(expr),
                    arg,
                })),
                _ => Err("Cannot apply non-functions".to_string()),
            },
            Expr::Let(Let {
                name,
                value,
                body,
                rec,
            }) => {
                let value = self.eval_let_value(name.clone(), *value, rec)?;
                self.with_binding(name, value).eval(*body)
            }
            Expr::If(If { check, then, els }) => {
                let check = self.eval(*check)?;
                match check {
                    BoundValue::Value(Value::Bool(Bool { b })) => {
                        if b {
                            self.eval(*then)
                        } else {
                            self.eval(*els)
                        }
                    }
                    _ => Err("If check must be Bool".to_string()),
                }
            }
        }
    }

    fn eval_arith(
        &self,
        left: Expr,
        right: Expr,
        f: fn(i64, i64) -> i64,
    ) -> Result<BoundValue, String> {
        let (x, y) = match (self.eval(left)?, self.eval(right)?) {
            (
                BoundValue::Value(Value::Int(Int { n: x })),
                BoundValue::Value(Value::Int(Int { n: y })),
            ) => Ok((x, y)),
            _ => Err("Cannot eval BinOp with non-Int operands"),
        }?;

        Ok(BoundValue::Value(Value::Int(Int { n: f(x, y) })))
    }

    fn eval_let_value(&self, name: String, value: Expr, rec: bool) -> Result<BoundValue, String> {
        if rec {
            self.with_binding(name, BoundValue::Thunk(value.clone()))
                .eval(value)
        } else {
            self.eval(value)
        }
    }

    fn lookup(&self, name: &str) -> Option<BoundValue> {
        let value = self.bindings.get(name)?;
        Some(value.clone()) // TODO: story around cloning here?
    }

    fn with_binding(&self, name: String, value: BoundValue) -> Env {
        Env {
            bindings: self.bindings.insert(name, value),
        }
    }
}
