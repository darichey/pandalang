use std::collections::HashMap;

use crate::{ast::expr, value::Value};

use super::BoundValue;

type BuiltinFunction = fn(BoundValue) -> Result<BoundValue, String>;

#[derive(Clone)]
pub struct Builtins {
    builtins: HashMap<&'static str, BuiltinFunction>,
}

impl Builtins {
    pub fn new() -> Self {
        let mut builtins: HashMap<&'static str, BuiltinFunction> = HashMap::new();
        builtins.insert("str_of_int", str_of_int);
        builtins.insert("println", println_);
        Builtins { builtins }
    }

    pub fn eval(&self, builtin_name: String, arg: BoundValue) -> Result<BoundValue, String> {
        let builtin = self
            .builtins
            .get(builtin_name.as_str())
            .ok_or("Builtin not found")?;
        builtin(arg)
    }
}

pub fn str_of_int(x: BoundValue) -> Result<BoundValue, String> {
    match x {
        BoundValue::Value(Value::Int(expr::Int { n })) => {
            Ok(BoundValue::Value(Value::Str(expr::Str {
                s: n.to_string(),
            })))
        }
        _ => Err("Not an Int".to_string()),
    }
}

pub fn println_(x: BoundValue) -> Result<BoundValue, String> {
    match x {
        BoundValue::Value(Value::Str(expr::Str { s })) => {
            println!("{}", s);
            Ok(BoundValue::Value(Value::Unit))
        }
        _ => Err("Not a Str".to_string()),
    }
}
