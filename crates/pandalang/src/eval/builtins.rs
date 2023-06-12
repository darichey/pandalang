use std::collections::HashMap;

use crate::{ast::expr, value::Value};

type BuiltinFunction = fn(Value) -> Result<Value, String>;

lazy_static! {
    static ref BUILTINS: HashMap<&'static str, BuiltinFunction> = {
        let mut m: HashMap<&'static str, BuiltinFunction> = HashMap::new();
        m.insert("str_of_int", str_of_int);
        m.insert("println", println_);
        m
    };
}

pub fn eval(builtin_name: String, arg: Value) -> Result<Value, String> {
    let builtin = BUILTINS
        .get(builtin_name.as_str())
        .ok_or("Builtin not found")?;
    builtin(arg)
}

pub fn str_of_int(x: Value) -> Result<Value, String> {
    match x {
        Value::Int(expr::Int { n }) => Ok(Value::Str(expr::Str { s: n.to_string() })),
        _ => Err("Not an Int".to_string()),
    }
}

pub fn println_(x: Value) -> Result<Value, String> {
    match x {
        Value::Str(expr::Str { s }) => {
            println!("{}", s);
            Ok(Value::Unit)
        }
        _ => Err("Not a Str".to_string()),
    }
}
