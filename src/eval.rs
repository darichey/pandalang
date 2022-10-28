use std::collections::HashMap;

use crate::ast::{BinOpKind, Expr};

pub struct Env {
    bindings: HashMap<String, i64>,
}

macro_rules! new_env {
    ($($k:expr => $v:expr),* $(,)?) => {{
        Env { bindings: HashMap::from([$(($k.to_string(), $v),)*]) }
    }};
}

impl Env {
    pub fn lookup(&self, x: &String) -> i64 {
        *self.bindings.get(x).unwrap()
    }
}

pub fn eval(expr: Expr, env: &Env) -> i64 {
    match expr {
        Expr::Int(n) => n,
        Expr::Var(x) => env.lookup(&x),
        Expr::BinOp { left, right, kind } => {
            let f: fn(i64, i64) -> i64 = match kind {
                BinOpKind::Add => std::ops::Add::add,
                BinOpKind::Sub => std::ops::Sub::sub,
                BinOpKind::Mul => std::ops::Mul::mul,
                BinOpKind::Div => std::ops::Div::div,
            };
            f(eval(*left, env), eval(*right, env))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::ast::Expr;

    use super::eval;
    use super::Env;

    #[test]
    fn int_eval_id() {
        let env = new_env!();
        assert_eq!(eval(Expr::Int(0), &env), 0)
    }

    #[test]
    fn var_eval_id() {
        let env = new_env!("x" => 3);
        assert_eq!(eval(Expr::Var("x".to_string()), &env), 3);
    }

    #[test]
    fn int_eval_add() {
        let env = new_env!();
        assert_eq!(
            eval(
                Expr::BinOp {
                    left: Box::new(Expr::Int(1)),
                    right: Box::new(Expr::Int(2)),
                    kind: crate::ast::BinOpKind::Add,
                },
                &env
            ),
            3
        )
    }

    #[test]
    fn int_eval_sub() {
        let env = new_env!();
        assert_eq!(
            eval(
                Expr::BinOp {
                    left: Box::new(Expr::Int(1)),
                    right: Box::new(Expr::Int(2)),
                    kind: crate::ast::BinOpKind::Sub,
                },
                &env
            ),
            -1
        )
    }

    #[test]
    fn int_eval_mul() {
        let env = new_env!();
        assert_eq!(
            eval(
                Expr::BinOp {
                    left: Box::new(Expr::Int(1)),
                    right: Box::new(Expr::Int(2)),
                    kind: crate::ast::BinOpKind::Mul,
                },
                &env
            ),
            2
        )
    }

    #[test]
    fn int_eval_div() {
        let env = new_env!();
        assert_eq!(
            eval(
                Expr::BinOp {
                    left: Box::new(Expr::Int(1)),
                    right: Box::new(Expr::Int(2)),
                    kind: crate::ast::BinOpKind::Div,
                },
                &env
            ),
            0
        )
    }

    #[test]
    fn var_eval_add() {
        let env = new_env!("x" => 1, "y" => 2);
        assert_eq!(
            eval(
                Expr::BinOp {
                    left: Box::new(Expr::Var("x".to_string())),
                    right: Box::new(Expr::Var("y".to_string())),
                    kind: crate::ast::BinOpKind::Add,
                },
                &env
            ),
            3
        )
    }

    #[test]
    fn var_eval_sub() {
        let env = new_env!("x" => 1, "y" => 2);
        assert_eq!(
            eval(
                Expr::BinOp {
                    left: Box::new(Expr::Var("x".to_string())),
                    right: Box::new(Expr::Var("y".to_string())),
                    kind: crate::ast::BinOpKind::Sub,
                },
                &env
            ),
            -1
        )
    }

    #[test]
    fn var_eval_mul() {
        let env = new_env!("x" => 1, "y" => 2);
        assert_eq!(
            eval(
                Expr::BinOp {
                    left: Box::new(Expr::Var("x".to_string())),
                    right: Box::new(Expr::Var("y".to_string())),
                    kind: crate::ast::BinOpKind::Mul,
                },
                &env
            ),
            2
        )
    }

    #[test]
    fn var_eval_div() {
        let env = new_env!("x" => 1, "y" => 2);
        assert_eq!(
            eval(
                Expr::BinOp {
                    left: Box::new(Expr::Var("x".to_string())),
                    right: Box::new(Expr::Var("y".to_string())),
                    kind: crate::ast::BinOpKind::Div,
                },
                &env
            ),
            0
        )
    }
}
