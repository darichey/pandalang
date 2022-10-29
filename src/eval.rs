use std::collections::HashMap;

use crate::ast::{BinOp, BinOpKind, Expr, Int, Var};
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
        // Expr::Let(Let { name, value, body }) => {
        //     let mut new_env = Env {
        //         bindings: env.bindings.clone(),
        //     }; // TODO: no
        //     new_env.bindings.insert(name, eval(*value, env));
        //     eval(*body, &new_env)
        // }
        Expr::Fun(fun) => Value::Fun(fun),
    }
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
