use std::collections::HashMap;

use crate::ast::{BinOpKind, Expr, ExprF};

pub struct Env {
    pub bindings: HashMap<String, Expr>,
}

macro_rules! new_env {
    ($($k:expr => $v:expr),* $(,)?) => {{
        Env { bindings: std::collections::HashMap::from([$(($k.to_string(), Expr(Box::new(ExprF::Int { n: $v }))),)*]) }
    }};
}

pub(crate) use new_env;

impl Env {
    pub fn lookup(&self, x: &String) -> Expr {
        self.bindings.get(x).unwrap().clone()
    }
}

pub fn eval(expr: Expr, env: &Env) -> Expr {
    match *expr.0 {
        ExprF::Int { n } => expr,
        ExprF::Var { name } => env.lookup(&name),
        ExprF::BinOp { left, right, kind } => {
            let f = match kind {
                BinOpKind::Add => std::ops::Add::add,
                BinOpKind::Sub => std::ops::Sub::sub,
                BinOpKind::Mul => std::ops::Mul::mul,
                BinOpKind::Div => std::ops::Div::div,
            };

            let (x, y) = match (*eval(left, env).0, *eval(right, env).0) {
                (ExprF::Int { n: x }, ExprF::Int { n: y }) => (x, y),
                _ => panic!("oh god oh fuck"),
            };

            Expr(Box::new(ExprF::Int { n: f(x, y) }))
        }
        ExprF::Fun { arg, body } => Expr(Box::new(ExprF::Fun { arg, body })),
        ExprF::App { fun, arg } => {
            let (arg_name, body) = match *eval(fun, env).0 {
                ExprF::Fun { arg, body } => (arg, body),
                _ => panic!("oh god oh fuck"),
            };
            let arg = eval(arg, env);

            eval(cas(body, arg, arg_name), env)
        }
    }
}

fn cas(e1: Expr, e2: Expr, var: String) -> Expr {
    return match *e1.0 {
        ExprF::Var { name } => {
            if var == name {
                e2
            } else {
                Expr(Box::new(ExprF::Var { name }))
            }
        }
        ExprF::Fun { arg, body } => {
            if var == arg {
                Expr(Box::new(ExprF::Fun { arg, body }))
            } else {
                Expr(Box::new(ExprF::Fun {
                    arg,
                    body: cas(body, e2, var),
                }))
            }
        }
        ExprF::App { fun, arg } => Expr(Box::new(ExprF::App {
            fun: cas(fun, e2.clone(), var.clone()),
            arg: cas(arg, e2, var),
        })),
        ExprF::Int { n } => Expr(Box::new(ExprF::Int { n })),
        ExprF::BinOp { left, right, kind } => Expr(Box::new(ExprF::BinOp {
            left: cas(left, e2.clone(), var.clone()),
            right: cas(right, e2, var),
            kind,
        })),
    };
}

#[cfg(test)]
mod tests {
    use super::Env;
    use crate::{ast::Expr, ast::ExprF, eval, parser};

    fn eval_test(s: String) -> Expr {
        eval::eval(
            parser::parse(s.as_str()).unwrap(),
            &new_env!("x" => 0, "y" => 1, "x'" => 2, "foo" => 3, "a" => 4, "b" => 5, "c" => 6, "d" => 7, "e" => 8),
        )
    }

    #[test]
    fn evals() {
        insta::glob!("snapshot_inputs/**/*.panda", |path| {
            let source = eval_test(std::fs::read_to_string(&path).unwrap());
            insta::assert_debug_snapshot!(source);
        });
    }
}
