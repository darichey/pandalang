use std::{collections::HashMap, rc::Rc};

use crate::{ast::expr, value::Value};

type BuiltinFunction = fn(Rc<Value>) -> Result<Rc<Value>, String>;

lazy_static! {
    static ref BUILTINS: HashMap<&'static str, BuiltinFunction> = {
        let mut m: HashMap<&'static str, BuiltinFunction> = HashMap::new();
        m.insert("str_of_int", str_of_int);
        m.insert("println", println_);
        m
    };
}

pub fn eval(builtin_name: String, arg: Rc<Value>) -> Result<Rc<Value>, String> {
    let builtin = BUILTINS
        .get(builtin_name.as_str())
        .ok_or("Builtin not found")?;
    builtin(arg)
}

pub fn str_of_int(x: Rc<Value>) -> Result<Rc<Value>, String> {
    match x.as_ref() {
        Value::Int(expr::Int { n }) => Ok(Rc::new(Value::Str(expr::Str { s: n.to_string() }))),
        _ => Err("Not an Int".to_string()),
    }
}

pub fn println_(x: Rc<Value>) -> Result<Rc<Value>, String> {
    match x.as_ref() {
        Value::Str(expr::Str { s }) => {
            println!("{}", s);
            Ok(Rc::new(Value::Unit))
        }
        _ => Err("Not a Str".to_string()),
    }
}
