use std::collections::HashMap;

use crate::ast::{BinOp, BinOpKind, Expr, Int, Let, Var};
use crate::value::Value;

pub struct Env {
    bindings: HashMap<String, Value>,
}

macro_rules! new_env {
    ($($k:expr => $v:expr),* $(,)?) => {{
        Env { bindings: HashMap::from([$(($k.to_string(), $v),)*]) }
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
        Expr::Let(Let { name, value, body }) => {
            let mut new_env = Env {
                bindings: env.bindings.clone(),
            }; // TODO: no
            new_env.bindings.insert(name, eval(*value, env));
            eval(*body, &new_env)
        }
        Expr::Fun(fun) => Value::Fun(fun),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::ast::BinOp;
    use crate::ast::BinOpKind;
    use crate::ast::Expr;
    use crate::ast::Fun;
    use crate::ast::Int;
    use crate::ast::Let;
    use crate::ast::Var;
    use crate::value::Value;

    use super::eval;
    use super::Env;

    #[test]
    fn int_eval_id() {
        let env = new_env!();
        assert_eq!(
            eval(Expr::Int(Int { n: 0 }), &env),
            Value::Int(Int { n: 0 })
        )
    }

    #[test]
    fn var_eval_id() {
        let env = new_env!("x" => Value::Int(Int { n: 3 }));
        assert_eq!(
            eval(
                Expr::Var(Var {
                    name: "x".to_string()
                }),
                &env
            ),
            Value::Int(Int { n: 3 })
        );
    }

    #[test]
    fn int_eval_add() {
        let env = new_env!();
        assert_eq!(
            eval(
                Expr::BinOp(BinOp {
                    left: Box::new(Expr::Int(Int { n: 1 })),
                    right: Box::new(Expr::Int(Int { n: 2 })),
                    kind: BinOpKind::Add,
                }),
                &env
            ),
            Value::Int(Int { n: 3 })
        )
    }

    #[test]
    fn int_eval_sub() {
        let env = new_env!();
        assert_eq!(
            eval(
                Expr::BinOp(BinOp {
                    left: Box::new(Expr::Int(Int { n: 1 })),
                    right: Box::new(Expr::Int(Int { n: 2 })),
                    kind: BinOpKind::Sub,
                }),
                &env
            ),
            Value::Int(Int { n: -1 })
        )
    }

    #[test]
    fn int_eval_mul() {
        let env = new_env!();
        assert_eq!(
            eval(
                Expr::BinOp(BinOp {
                    left: Box::new(Expr::Int(Int { n: 1 })),
                    right: Box::new(Expr::Int(Int { n: 2 })),
                    kind: BinOpKind::Mul,
                }),
                &env
            ),
            Value::Int(Int { n: 2 })
        )
    }

    #[test]
    fn int_eval_div() {
        let env = new_env!();
        assert_eq!(
            eval(
                Expr::BinOp(BinOp {
                    left: Box::new(Expr::Int(Int { n: 1 })),
                    right: Box::new(Expr::Int(Int { n: 2 })),
                    kind: BinOpKind::Div,
                }),
                &env
            ),
            Value::Int(Int { n: 0 })
        )
    }

    #[test]
    fn var_eval_add() {
        let env = new_env!("x" => Value::Int(Int { n: 1 }), "y" => Value::Int(Int { n: 2 }));
        assert_eq!(
            eval(
                Expr::BinOp(BinOp {
                    left: Box::new(Expr::Var(Var {
                        name: "x".to_string()
                    })),
                    right: Box::new(Expr::Var(Var {
                        name: "y".to_string()
                    })),
                    kind: BinOpKind::Add,
                }),
                &env
            ),
            Value::Int(Int { n: 3 })
        )
    }

    #[test]
    fn var_eval_sub() {
        let env = new_env!("x" => Value::Int(Int { n: 1 }), "y" => Value::Int(Int { n: 2 }));
        assert_eq!(
            eval(
                Expr::BinOp(BinOp {
                    left: Box::new(Expr::Var(Var {
                        name: "x".to_string()
                    })),
                    right: Box::new(Expr::Var(Var {
                        name: "y".to_string()
                    })),
                    kind: BinOpKind::Sub,
                }),
                &env
            ),
            Value::Int(Int { n: -1 })
        )
    }

    #[test]
    fn var_eval_mul() {
        let env = new_env!("x" => Value::Int(Int { n: 1 }), "y" => Value::Int(Int { n: 2 }));
        assert_eq!(
            eval(
                Expr::BinOp(BinOp {
                    left: Box::new(Expr::Var(Var {
                        name: "x".to_string()
                    })),
                    right: Box::new(Expr::Var(Var {
                        name: "y".to_string()
                    })),
                    kind: BinOpKind::Mul,
                }),
                &env
            ),
            Value::Int(Int { n: 2 })
        )
    }

    #[test]
    fn var_eval_div() {
        let env = new_env!("x" => Value::Int(Int { n: 1 }), "y" => Value::Int(Int { n: 2 }));
        assert_eq!(
            eval(
                Expr::BinOp(BinOp {
                    left: Box::new(Expr::Var(Var {
                        name: "x".to_string()
                    })),
                    right: Box::new(Expr::Var(Var {
                        name: "y".to_string()
                    })),
                    kind: BinOpKind::Div,
                }),
                &env
            ),
            Value::Int(Int { n: 0 })
        )
    }

    #[test]
    fn let_eval_id() {
        let env = new_env!();
        assert_eq!(
            eval(
                Expr::Let(Let {
                    name: "x".to_string(),
                    value: Box::new(Expr::Int(Int { n: 3 })),
                    body: Box::new(Expr::Var(Var {
                        name: "x".to_string()
                    }))
                }),
                &env
            ),
            Value::Int(Int { n: 3 })
        )
    }

    #[test]
    fn let_eval_shadow() {
        let env = new_env!("x" => Value::Int(Int { n: 5 }));
        assert_eq!(
            eval(
                Expr::Let(Let {
                    name: "x".to_string(),
                    value: Box::new(Expr::Int(Int { n: 3 })),
                    body: Box::new(Expr::Var(Var {
                        name: "x".to_string()
                    }))
                }),
                &env
            ),
            Value::Int(Int { n: 3 })
        )
    }

    #[test]
    fn let_eval_nested() {
        let env = new_env!();
        assert_eq!(
            eval(
                Expr::Let(Let {
                    name: "x".to_string(),
                    value: Box::new(Expr::Int(Int { n: 3 })),
                    body: Box::new(Expr::Let(Let {
                        name: "y".to_string(),
                        value: Box::new(Expr::Int(Int { n: 5 })),
                        body: Box::new(Expr::BinOp(BinOp {
                            left: Box::new(Expr::Var(Var {
                                name: "x".to_string()
                            })),
                            right: Box::new(Expr::Var(Var {
                                name: "y".to_string()
                            })),
                            kind: BinOpKind::Add
                        }))
                    }))
                }),
                &env
            ),
            Value::Int(Int { n: 8 })
        )
    }

    #[test]
    fn fun_eval_id() {
        let env = new_env!();
        assert_eq!(
            eval(
                Expr::Fun(Fun {
                    arg: "x".to_string(),
                    body: Box::new(Expr::Var(Var {
                        name: "x".to_string()
                    }))
                }),
                &env
            ),
            Value::Fun(Fun {
                arg: "x".to_string(),
                body: Box::new(Expr::Var(Var {
                    name: "x".to_string()
                }))
            })
        )
    }
}
