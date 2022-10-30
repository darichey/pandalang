use std::collections::HashMap;

use crate::ast::{App, BinOp, BinOpKind, Expr, Fun, Int, Var};
use crate::value::Value;

pub struct Env {
    bindings: HashMap<String, Value>,
}

macro_rules! new_env {
    ($($k:expr => $v:expr),* $(,)?) => {{
        Env { bindings: HashMap::from([$(($k.to_string(), Value::Int(Int { n: $v })),)*]) }
    }};
}

impl Env {
    pub fn lookup(&self, x: &String) -> Value {
        self.bindings.get(x).unwrap().clone()
    }
}

pub fn eval(expr: Expr, env: &Env) -> Value {
    match expr {
        Expr::Int(n) => Value::Int(n),
        Expr::Var(Var { name }) => env.lookup(&name),
        Expr::BinOp(BinOp { left, right, kind }) => {
            let f = match kind {
                BinOpKind::Add => std::ops::Add::add,
                BinOpKind::Sub => std::ops::Sub::sub,
                BinOpKind::Mul => std::ops::Mul::mul,
                BinOpKind::Div => std::ops::Div::div,
            };

            let (x, y) = match (eval(*left, env), eval(*right, env)) {
                (Value::Int(Int { n: x }), Value::Int(Int { n: y })) => (x, y),
                _ => panic!("oh god oh fuck"),
            };

            Value::Int(Int { n: f(x, y) })
        }
        Expr::Fun(fun) => Value::Fun(fun),
        Expr::App(App { fun, arg }) => {
            let Fun {
                arg: arg_name,
                body,
            } = match eval(*fun, env) {
                Value::Fun(fun) => fun,
                _ => panic!("oh god oh fuck"),
            };
            let arg = eval(*arg, env).as_expr();

            eval(cas(*body, arg, arg_name), env)
        }
    }
}

fn cas(e1: Expr, e2: Expr, var: String) -> Expr {
    return match e1 {
        Expr::Var(Var { name }) => {
            if var == name {
                e2
            } else {
                Expr::Var(Var { name })
            }
        }
        Expr::Fun(Fun { arg, body }) => {
            if var == arg {
                Expr::Fun(Fun { arg, body })
            } else {
                Expr::Fun(Fun {
                    arg,
                    body: Box::new(cas(*body, e2, var)),
                })
            }
        }
        Expr::App(App { fun, arg }) => Expr::App(App {
            fun: Box::new(cas(*fun, e2.clone(), var.clone())),
            arg: Box::new(cas(*arg, e2, var)),
        }),
        Expr::Int(n) => Expr::Int(n),
        Expr::BinOp(BinOp { left, right, kind }) => Expr::BinOp(BinOp {
            left: Box::new(cas(*left, e2.clone(), var.clone())),
            right: Box::new(cas(*right, e2, var)),
            kind,
        }),
    };
}

#[cfg(test)]
mod tests {
    use super::Env;
    use crate::ast::Int;
    use crate::value::Value;
    use crate::{eval, parser};
    use std::collections::HashMap;

    fn eval(s: String) -> Value {
        eval::eval(
            *parser::parse(s.as_str()).unwrap(),
            &new_env!("x" => 0, "y" => 1, "x'" => 2, "foo" => 3, "a" => 4, "b" => 5, "c" => 6, "d" => 7, "e" => 8),
        )
    }

    #[test]
    fn evals() {
        insta::glob!("snapshot_inputs/**/*.panda", |path| {
            let source = eval(std::fs::read_to_string(&path).unwrap());
            insta::assert_debug_snapshot!(source);
        });
    }
}
