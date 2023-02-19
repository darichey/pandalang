use std::collections::HashMap;

use crate::ast::{App, BinOp, BinOpKind, Expr, Fun, Int, Var};
use crate::value::Value;

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
                let (arg_name, body, mut fun_env) = match self.eval(*fun) {
                    Value::Fun {
                        fun: Fun { arg, body },
                        env,
                    } => (arg, body, env),
                    _ => panic!("Cannot apply non-functions"),
                };
                let arg = self.eval(*arg);

                fun_env.push_binding(&arg_name, arg);
                let result = fun_env.eval(*body);
                fun_env.pop_binding(&arg_name);
                result
            }
            Expr::Let(_) => panic!("Let in eval"),
        }
    }

    fn lookup(&self, name: &String) -> Option<Value> {
        let bindings = self.bindings.get(name)?;
        println!("{} = {:?}", name, bindings);
        let value = bindings.last()?;
        Some(value.clone()) // TODO: story around cloning here?
    }

    fn push_binding(&mut self, name: &String, value: Value) {
        println!("push {} = {:?}", name, value);
        match self.bindings.get_mut(name) {
            Some(current_bindings) => current_bindings.push(value),
            None => {
                self.bindings.insert(name.clone(), vec![value]);
            }
        };
    }

    fn pop_binding(&mut self, name: &String) {
        println!("pop {}", name);
        if let Some(current_bindings) = self.bindings.get_mut(name) {
            current_bindings.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Env;
    use crate::ast::Int;
    use crate::parser;
    use crate::value::Value;

    fn eval_test(s: String) -> Value {
        let mut env = Env {
            bindings: bindings!("x" => 0, "y" => 1, "x'" => 2, "foo" => 3, "a" => 4, "b" => 5, "c" => 6, "d" => 7, "e" => 8),
        };
        env.eval(*parser::parse(s.as_str()).unwrap())
    }

    #[test]
    fn evals() {
        insta::glob!("snapshot_inputs/**/*.panda", |path| {
            let source = eval_test(std::fs::read_to_string(path).unwrap());
            insta::assert_debug_snapshot!(source);
        });
    }
}
