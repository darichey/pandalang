mod builtins;

use std::collections::HashMap;

use crate::ast::expr::{App, BinOp, BinOpKind, Expr, Fun, Int, Let, Var};
use crate::ast::stmt::Stmt;
use crate::ast::{stmt, Program};
use crate::value::Value;

pub fn run_program(program: Program) -> Result<Value, String> {
    let mut env = Env::new();

    for stmt in program.stmts {
        match stmt {
            Stmt::Let(stmt::Let { name, value }) => {
                let value = env.eval(*value);
                env.push_binding(&name, value);
            }
            Stmt::Declare(stmt::Declare { name, typ }) => {
                env.push_binding(&name, Value::Builtin(name.clone()))
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
        std::collections::HashMap::from([$(($k.to_string(), vec![Value::Int(Int { n: $v })]),)*])
    }};
}

#[derive(Debug, Clone)]
pub struct Env {
    pub bindings: HashMap<String, Vec<Value>>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            bindings: bindings!(),
        }
    }

    pub fn with_bindings(bindings: HashMap<String, Vec<Value>>) -> Env {
        Env { bindings }
    }

    // TODO: Result instead of panic
    pub fn eval(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Int(n) => Value::Int(n),
            Expr::Str(s) => Value::Str(s),
            Expr::Var(Var { name }) => self
                .lookup(&name)
                .unwrap_or_else(|| panic!("{} is not bound!", name)),
            Expr::BinOp(BinOp { left, right, kind }) => {
                let f = match kind {
                    BinOpKind::Add => std::ops::Add::add,
                    BinOpKind::Sub => std::ops::Sub::sub,
                    BinOpKind::Mul => std::ops::Mul::mul,
                    BinOpKind::Div => std::ops::Div::div,
                };

                let (x, y) = match (self.eval(*left), self.eval(*right)) {
                    (Value::Int(Int { n: x }), Value::Int(Int { n: y })) => (x, y),
                    _ => panic!("Cannot eval BinOp with non-Int operands"),
                };

                Value::Int(Int { n: f(x, y) })
            }
            Expr::Fun(fun) => Value::Fun {
                fun,
                env: self.clone(),
            },
            Expr::App(App { fun, arg }) => {
                match self.eval(*fun) {
                    Value::Fun {
                        fun:
                            Fun {
                                arg: arg_name,
                                body,
                            },
                        env: mut fun_env,
                    } => {
                        let arg = self.eval(*arg);

                        fun_env.push_binding(&arg_name, arg);
                        let result = fun_env.eval(*body);
                        fun_env.pop_binding(&arg_name);
                        result
                    }
                    Value::Builtin(builtin) => {
                        let arg = self.eval(*arg);
                        builtins::eval(builtin, arg).unwrap()
                    }
                    _ => panic!("Cannot apply non-functions"),
                }
                // let (arg_name, body, mut fun_env) = match self.eval(*fun) {
                //     Value::Fun {
                //         fun: Fun { arg, body },
                //         env,
                //     } => (arg, body, env),
                //     _ => panic!("Cannot apply non-functions"),
                // };
            }
            Expr::Let(Let { name, value, body }) => {
                let value = self.eval(*value);
                self.push_binding(&name, value);
                let result = self.eval(*body);
                self.pop_binding(&name);
                result
            }
        }
    }

    fn lookup(&self, name: &String) -> Option<Value> {
        let bindings = self.bindings.get(name)?;
        let value = bindings.last()?;
        Some(value.clone()) // TODO: story around cloning here?
    }

    fn push_binding(&mut self, name: &String, value: Value) {
        match self.bindings.get_mut(name) {
            Some(current_bindings) => current_bindings.push(value),
            None => {
                self.bindings.insert(name.clone(), vec![value]);
            }
        };
    }

    fn pop_binding(&mut self, name: &String) {
        if let Some(current_bindings) = self.bindings.get_mut(name) {
            current_bindings.pop();
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
        let mut env = Env {
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
