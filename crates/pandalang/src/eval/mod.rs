mod builtins;

use std::rc::Rc;

use rpds::HashTrieMap;

use crate::ast::expr::{App, BinOp, BinOpKind, Bool, Expr, Fun, If, Int, Let, Var};
use crate::ast::stmt::Stmt;
use crate::ast::{stmt, Program};
use crate::value::Value;

pub fn run_program<'a>(program: Program) -> Result<Rc<Value<'a>>, String> {
    let mut env = Env::new();

    // TODO: fold
    for stmt in program.stmts {
        match stmt {
            Stmt::Let(stmt::Let { name, value, rec }) => {
                let value = env.eval_let_value(name.clone(), *value, rec);
                env = env.with_binding(name, value);
            }
            Stmt::Declare(stmt::Declare { name, .. }) => {
                env = env.with_binding(
                    name.clone(),
                    BoundValue::Value(Rc::new(Value::Builtin(name))),
                )
            }
        }
    }

    let main_value = env
        .lookup(&"main".to_string())
        .ok_or("Couldn't find main")?;

    let main_value = check_fully_evaluated(main_value)?;

    Ok(main_value)
}

pub fn eval(env: Env, expr: Expr) -> Result<Rc<Value>, String> {
    check_fully_evaluated(env.eval(expr))
}

fn check_fully_evaluated(v: BoundValue) -> Result<Rc<Value>, String> {
    match v {
        BoundValue::Value(v) => Ok(v),
        BoundValue::Thunk(_) => Err("Final value should be fully evaluated".to_string()),
    }
}

#[derive(Clone)]
pub enum BoundValue<'a> {
    Value(Rc<Value<'a>>),
    Thunk(Rc<dyn Fn() -> BoundValue<'a> + 'a>),
}

impl PartialEq for BoundValue<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            _ => false,
        }
    }
}

macro_rules! bindings {
    ($($k:expr => $v:expr),* $(,)?) => {{
        rpds::HashTrieMap::from_iter([$(($k.to_string(), BoundValue::Value(std::rc::Rc::new(Value::Int(Int { n: $v })))),)*])
    }};
}

#[derive(Clone)]
pub struct Env<'a> {
    pub bindings: HashTrieMap<String, BoundValue<'a>>,
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            bindings: bindings!(),
        }
    }

    // TODO: Result instead of panic
    fn eval(&self, expr: Expr) -> BoundValue<'a> {
        match expr {
            Expr::Int(n) => BoundValue::Value(Rc::new(Value::Int(n))),
            Expr::Str(s) => BoundValue::Value(Rc::new(Value::Str(s))),
            Expr::Unit => BoundValue::Value(Rc::new(Value::Unit)),
            Expr::Bool(b) => BoundValue::Value(Rc::new(Value::Bool(b))),
            Expr::Var(Var { name }) => self
                .lookup(&name)
                .unwrap_or_else(|| panic!("{} is not bound!", name)),
            Expr::BinOp(BinOp { left, right, kind }) => match kind {
                BinOpKind::Add => self.eval_arith(*left, *right, std::ops::Add::add),
                BinOpKind::Sub => self.eval_arith(*left, *right, std::ops::Sub::sub),
                BinOpKind::Mul => self.eval_arith(*left, *right, std::ops::Mul::mul),
                BinOpKind::Div => self.eval_arith(*left, *right, std::ops::Div::div),
                BinOpKind::Rem => self.eval_arith(*left, *right, std::ops::Rem::rem),
                BinOpKind::Eql => {
                    let left = self.eval(*left);
                    let right = self.eval(*right);
                    BoundValue::Value(Rc::new(Value::Bool(Bool { b: left == right })))
                }
            },
            Expr::Fun(fun) => BoundValue::Value(Rc::new(Value::Fun {
                fun,
                env: self.clone(),
            })),
            Expr::App(App { fun, arg }) => match self.eval(*fun) {
                BoundValue::Value(fun) => match fun.as_ref() {
                    Value::Fun {
                        fun:
                            Fun {
                                arg: arg_name,
                                body,
                            },
                        env: fun_env,
                    } => {
                        let arg = self.eval(*arg);
                        fun_env
                            .with_binding(arg_name.clone(), arg)
                            .eval(*body.clone())
                    }
                    Value::Builtin(builtin) => {
                        let arg = self.eval(*arg);
                        builtins::eval(builtin.clone(), arg).unwrap()
                    }
                    _ => panic!("Cannot apply non-functions"),
                },
                _ => panic!("Cannot apply non-functions"),
            },
            Expr::Let(Let {
                name,
                value,
                body,
                rec,
            }) => {
                let value = self.eval_let_value(name.clone(), *value, rec);
                self.with_binding(name, value).eval(*body)
            }
            Expr::If(If { check, then, els }) => {
                let check = self.eval(*check);
                match check {
                    BoundValue::Value(check) => match check.as_ref() {
                        Value::Bool(Bool { b }) => {
                            if *b {
                                self.eval(*then)
                            } else {
                                self.eval(*els)
                            }
                        }
                        _ => panic!("If check must be Bool"),
                    },
                    _ => panic!("If check must be Bool"),
                }
            }
        }
    }

    fn eval_arith(&self, left: Expr, right: Expr, f: fn(i64, i64) -> i64) -> BoundValue<'a> {
        let (x, y) = match (self.eval(left), self.eval(right)) {
            (BoundValue::Value(left), BoundValue::Value(right)) => {
                match (left.as_ref(), right.as_ref()) {
                    (Value::Int(Int { n: x }), Value::Int(Int { n: y })) => (*x, *y),
                    _ => panic!("Cannot eval BinOp with non-Int operands"),
                }
            }
            _ => panic!("Cannot eval BinOp with non-Int operands"),
        };

        BoundValue::Value(Rc::new(Value::Int(Int { n: f(x, y) })))
    }

    fn eval_let_value(&self, name: String, value: Expr, rec: bool) -> BoundValue<'a> {
        if rec {
            todo!()
        } else {
            self.eval(value)
        }
    }

    fn lookup(&self, name: &String) -> Option<BoundValue<'a>> {
        let value = self.bindings.get(name)?;
        Some(value.clone())
    }

    fn with_binding(&self, name: String, value: BoundValue<'a>) -> Env<'a> {
        Env {
            bindings: self.bindings.insert(name, value),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::rc::Rc;

    use super::{eval, run_program, BoundValue, Env};
    use crate::ast::expr::Int;
    use crate::parser;
    use crate::value::Value;

    fn test_expr(path: &Path) -> Result<Rc<Value>, String> {
        let env = Env {
            bindings: bindings!("x" => 0, "y" => 1, "x'" => 2, "foo" => 3, "a" => 4, "b" => 5, "c" => 6, "d" => 7, "e" => 8, "foo_bar" => 9),
        };
        let source = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
        let ast = parser::parse_expr(&source).map_err(|err| err.to_string())?;
        eval(env, *ast)
    }

    #[test]
    fn evals_exprs() {
        insta::glob!("..", "snapshot_inputs/exprs/**/*.panda", |path| {
            insta::assert_debug_snapshot!(test_expr(path));
        });
    }

    fn test_prog(path: &Path) -> Result<Rc<Value>, String> {
        let source = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
        let ast = parser::parse(&source).map_err(|err| err.to_string())?;
        run_program(ast)
    }

    #[test]
    fn evals_progs() {
        insta::glob!("..", "snapshot_inputs/progs/**/*.panda", |path| {
            insta::assert_debug_snapshot!(test_prog(path));
        });
    }
}
