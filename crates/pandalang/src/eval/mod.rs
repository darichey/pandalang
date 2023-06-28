mod builtins;
pub mod env;

use std::io::Write;

use crate::ast::expr::{App, BinOp, BinOpKind, Bool, Expr, Fun, If, Int, Let, Var};
use crate::ast::stmt::Stmt;
use crate::ast::{stmt, Program};
use crate::value::Value;

use self::builtins::Builtins;
use self::env::{BoundValue, Env};

pub fn run_program(program: Program, stdout: &mut dyn Write) -> Result<Value, String> {
    let mut evaluator = Evaluator::new(stdout);

    for stmt in program.stmts {
        match stmt {
            Stmt::Let(stmt::Let { name, value, rec }) => {
                let value = evaluator.eval_let_value(name.clone(), *value, rec)?;
                evaluator.env.push_binding(&name, value)
            }
            Stmt::Declare(stmt::Declare { name, .. }) => evaluator
                .env
                .push_binding(&name.clone(), BoundValue::Value(Value::Builtin(name))),
        }
    }

    let main = evaluator.env.lookup("main").ok_or("Couldn't find main")?;

    check_fully_evaluated(main)
}

pub fn eval(mut evaluator: Evaluator, expr: Expr) -> Result<Value, String> {
    check_fully_evaluated(evaluator.eval(expr)?)
}

fn check_fully_evaluated(v: BoundValue) -> Result<Value, String> {
    match v {
        BoundValue::Value(v) => Ok(v),
        BoundValue::Thunk(_) => Err("Final value should be fully evaluated".to_string()),
    }
}

pub struct Evaluator<'a> {
    env: Env,
    builtins: Builtins<'a>,
}

impl<'a> Evaluator<'a> {
    pub fn new(stdout: &'a mut dyn Write) -> Self {
        Self {
            env: Env::new(),
            builtins: Builtins::new(stdout),
        }
    }

    fn eval(&mut self, expr: Expr) -> Result<BoundValue, String> {
        match expr {
            Expr::Int(n) => Ok(BoundValue::Value(Value::Int(n))),
            Expr::Str(s) => Ok(BoundValue::Value(Value::Str(s))),
            Expr::Unit => Ok(BoundValue::Value(Value::Unit)),
            Expr::Bool(b) => Ok(BoundValue::Value(Value::Bool(b))),
            Expr::Var(Var { name }) => Ok(self
                .env
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
                env: self.env.clone(),
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

                    // set evaluator env to the captured env of the closure, evaluate the body, and then set the env back
                    let mut temp_env = fun_env;
                    std::mem::swap(&mut self.env, &mut temp_env);
                    let result = self.eval_with_binding(arg_name, arg, *body);
                    std::mem::swap(&mut self.env, &mut temp_env);
                    result
                }
                BoundValue::Value(Value::Builtin(builtin)) => {
                    let arg = self.eval(*arg)?;
                    self.builtins.eval(builtin, arg)
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
                self.eval_with_binding(name, value, *body)
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
        &mut self,
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

    fn eval_let_value(
        &mut self,
        name: String,
        value: Expr,
        rec: bool,
    ) -> Result<BoundValue, String> {
        if rec {
            self.eval_with_binding(name, BoundValue::Thunk(value.clone()), value)
        } else {
            self.eval(value)
        }
    }

    fn eval_with_binding(
        &mut self,
        name: String,
        value: BoundValue,
        expr: Expr,
    ) -> Result<BoundValue, String> {
        self.env.push_binding(&name, value);
        let result = self.eval(expr);
        self.env.pop_binding(&name);
        result
    }
}
