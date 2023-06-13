use std::{collections::HashMap, rc::Rc};

use crate::{ast::expr, value::Value};

use super::BoundValue;

type BuiltinFunction = fn(BoundValue) -> Result<BoundValue, String>;

lazy_static! {
    static ref BUILTINS: HashMap<&'static str, BuiltinFunction> = {
        let mut m: HashMap<&'static str, BuiltinFunction> = HashMap::new();
        m.insert("str_of_int", str_of_int);
        m.insert("println", println_);
        m
    };
}

pub fn eval(builtin_name: String, arg: BoundValue) -> Result<BoundValue, String> {
    let builtin = BUILTINS
        .get(builtin_name.as_str())
        .ok_or("Builtin not found")?;
    builtin(arg)
}

pub fn str_of_int(x: BoundValue) -> Result<BoundValue, String> {
    match x {
        BoundValue::Value(x) => match x.as_ref() {
            Value::Int(expr::Int { n }) => Ok(BoundValue::Value(Rc::new(Value::Str(expr::Str {
                s: n.to_string(),
            })))),
            _ => Err("Not an Int".to_string()),
        },
        _ => Err("Not an Int".to_string()),
    }
}

pub fn println_(x: BoundValue) -> Result<BoundValue, String> {
    match x {
        BoundValue::Value(x) => match x.as_ref() {
            Value::Str(expr::Str { s }) => {
                println!("{}", s);
                Ok(BoundValue::Value(Rc::new(Value::Unit)))
            }
            _ => Err("Not a Str".to_string()),
        },
        _ => Err("Not a Str".to_string()),
    }
}
