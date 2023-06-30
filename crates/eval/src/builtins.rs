use std::io::Write;

use pandalang_parser::ast::expr::{Int, Str};

use crate::value::Value;

use super::BoundValue;

pub struct Builtins<'a> {
    stdout: &'a mut dyn Write,
}

impl<'a> Builtins<'a> {
    pub fn new(stdout: &'a mut dyn Write) -> Self {
        Self { stdout }
    }

    pub fn eval(&mut self, builtin_name: String, arg: BoundValue) -> Result<BoundValue, String> {
        match builtin_name.as_str() {
            "str_of_int" => self.str_of_int(arg),
            "println" => self.println_(arg),
            _ => Err("Builtin not found".to_string()),
        }
    }

    fn str_of_int(&self, x: BoundValue) -> Result<BoundValue, String> {
        match x {
            BoundValue::Value(Value::Int(Int { n })) => {
                Ok(BoundValue::Value(Value::Str(Str { s: n.to_string() })))
            }
            _ => Err("Not an Int".to_string()),
        }
    }

    fn println_(&mut self, x: BoundValue) -> Result<BoundValue, String> {
        match x {
            BoundValue::Value(Value::Str(Str { s })) => {
                writeln!(self.stdout, "{}", s).map_err(|err| err.to_string())?;
                Ok(BoundValue::Value(Value::Unit))
            }
            _ => Err("Not a Str".to_string()),
        }
    }
}
