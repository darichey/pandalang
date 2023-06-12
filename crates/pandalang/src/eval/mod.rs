mod builtins;

use rpds::HashTrieMap;

use crate::ast::expr::{App, BinOp, BinOpKind, Bool, Expr, Fun, If, Int, Let, Var};
use crate::ast::stmt::Stmt;
use crate::ast::{stmt, Program};
use crate::value::Value;

pub fn run_program(program: Program) -> Result<Value, String> {
    let mut env = Env::new();

    // TODO: fold
    for stmt in program.stmts {
        match stmt {
            Stmt::Let(stmt::Let { name, value, rec }) => {
                let value = env.eval(*value);
                env = env.with_binding(name, value);
            }
            Stmt::Declare(stmt::Declare { name, .. }) => {
                env = env.with_binding(name.clone(), Value::Builtin(name))
            }
        }
    }

    let main_value = env
        .lookup(&"main".to_string())
        .ok_or("Couldn't find main")?;

    Ok(main_value)
}

macro_rules! bindings {
    ($($k:expr => $v:expr),* $(,)?) => {{
        rpds::HashTrieMap::from_iter([$(($k.to_string(), Value::Int(Int { n: $v })),)*])
    }};
}

#[derive(Debug, Clone)]
pub struct Env {
    // pub bindings: HashMap<String, Vec<Value>>,
    pub bindings: HashTrieMap<String, Value>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            bindings: bindings!(),
        }
    }

    // TODO: Result instead of panic
    pub fn eval(&self, expr: Expr) -> Value {
        match expr {
            Expr::Int(n) => Value::Int(n),
            Expr::Str(s) => Value::Str(s),
            Expr::Unit => Value::Unit,
            Expr::Bool(b) => Value::Bool(b),
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
                    Value::Bool(Bool { b: left == right })
                }
            },
            Expr::Fun(fun) => Value::Fun {
                fun,
                env: self.clone(),
            },
            Expr::App(App { fun, arg }) => match self.eval(*fun) {
                Value::Fun {
                    fun:
                        Fun {
                            arg: arg_name,
                            body,
                        },
                    env: fun_env,
                } => {
                    let arg = self.eval(*arg);
                    fun_env.with_binding(arg_name, arg).eval(*body)
                }
                Value::Builtin(builtin) => {
                    let arg = self.eval(*arg);
                    builtins::eval(builtin, arg).unwrap()
                }
                _ => panic!("Cannot apply non-functions"),
            },
            Expr::Let(Let {
                name,
                value,
                body,
                rec,
            }) => {
                let value = self.eval(*value);
                self.with_binding(name, value).eval(*body)
            }
            Expr::If(If { check, then, els }) => {
                let check = self.eval(*check);
                match check {
                    Value::Bool(Bool { b }) => {
                        if b {
                            self.eval(*then)
                        } else {
                            self.eval(*els)
                        }
                    }
                    _ => panic!("If check must be Bool"),
                }
            }
        }
    }

    fn eval_arith(&self, left: Expr, right: Expr, f: fn(i64, i64) -> i64) -> Value {
        let (x, y) = match (self.eval(left), self.eval(right)) {
            (Value::Int(Int { n: x }), Value::Int(Int { n: y })) => (x, y),
            _ => panic!("Cannot eval BinOp with non-Int operands"),
        };

        Value::Int(Int { n: f(x, y) })
    }

    fn lookup(&self, name: &String) -> Option<Value> {
        let value = self.bindings.get(name)?;
        Some(value.clone()) // TODO: story around cloning here?
    }

    fn with_binding(&self, name: String, value: Value) -> Env {
        Env {
            bindings: self.bindings.insert(name, value),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{run_program, Env};
    use crate::ast::expr::Int;
    use crate::parser;
    use crate::value::Value;

    fn test_expr(path: &Path) -> Result<Value, String> {
        let env = Env {
            bindings: bindings!("x" => 0, "y" => 1, "x'" => 2, "foo" => 3, "a" => 4, "b" => 5, "c" => 6, "d" => 7, "e" => 8, "foo_bar" => 9),
        };
        let source = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
        let ast = parser::parse_expr(&source).map_err(|err| err.to_string())?;
        Ok(env.eval(*ast))
    }

    #[test]
    fn evals_exprs() {
        insta::glob!("..", "snapshot_inputs/exprs/**/*.panda", |path| {
            insta::assert_debug_snapshot!(test_expr(path));
        });
    }

    fn test_prog(path: &Path) -> Result<Value, String> {
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
