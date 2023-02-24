use std::collections::HashMap;

use crate::ast::{self, Expr};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    Int,
    Str,
    Fun(Box<Type>, Box<Type>),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Str => write!(f, "Str"),
            Type::Fun(i, o) => write!(f, "({} -> {})", i, o),
        }
    }
}

macro_rules! types {
    ($($k:expr => $v:ident),* $(,)?) => {{
        std::collections::HashMap::from([$(($k.to_string(), vec![Type::$v]),)*])
    }};
}

pub struct Context {
    types: HashMap<String, Vec<Type>>,
}

impl Context {
    pub fn new() -> Context {
        Context { types: types!() }
    }

    pub fn check(&mut self, expr: Expr) -> Result<Type, Error> {
        match expr {
            Expr::Int(_) => Ok(Type::Int),
            Expr::Str(_) => Ok(Type::Str),
            Expr::Var(ast::Var { name }) => match self.lookup(&name) {
                Some(t) => Ok(t),
                None => Err(Error::NotInScope { name }),
            },
            Expr::BinOp(ast::BinOp { left, right, kind }) => match kind {
                ast::BinOpKind::Add
                | ast::BinOpKind::Sub
                | ast::BinOpKind::Mul
                | ast::BinOpKind::Div => {
                    let lhs_t = self.check(*left)?;
                    let rhs_t = self.check(*right)?;

                    expect_t(&Type::Int, &lhs_t)?;
                    expect_t(&Type::Int, &rhs_t)?;

                    Ok(Type::Int)
                }
            },
            Expr::Let(_) => {
                panic!("Let in type checking")
            }
            Expr::Fun(ast::Fun { patt, body }) => {
                let (name, name_t) = match patt {
                    ast::Pattern::Id { name, typ } => (name, parse_type(typ)?),
                };
                self.push_type(&name, &name_t);
                let body_t = self.check(*body)?;
                self.pop_type(&name);
                Ok(Type::Fun(Box::new(name_t), Box::new(body_t)))
            }
            Expr::App(ast::App { fun, arg }) => {
                let fun_t = self.check(*fun)?;
                match fun_t {
                    Type::Fun(in_t, out_t) => {
                        let arg_t = self.check(*arg)?;
                        expect_t(&in_t, &arg_t)?;
                        Ok(*out_t)
                    }
                    _ => Err(Error::NotFunction { actual: fun_t }),
                }
            }
        }
    }

    fn lookup(&self, name: &String) -> Option<Type> {
        let ts = self.types.get(name)?;
        let t = ts.last()?;
        Some(t.clone())
    }

    fn push_type(&mut self, name: &String, t: &Type) {
        match self.types.get_mut(name) {
            Some(types) => types.push(t.clone()),
            None => {
                self.types.insert(name.clone(), vec![t.clone()]);
            }
        }
    }

    fn pop_type(&mut self, name: &String) {
        if let Some(types) = self.types.get_mut(name) {
            types.pop();
        }
    }
}

fn expect_t(expected: &Type, actual: &Type) -> Result<(), Error> {
    if expected != actual {
        Err(Error::Incompatible {
            expected: expected.clone(),
            actual: actual.clone(),
        })
    } else {
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Error {
    Incompatible { expected: Type, actual: Type },
    NotFunction { actual: Type },
    NotInScope { name: String },
    UnknownType { name: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Incompatible { expected, actual } => {
                write!(f, "{} is incompatible with {}", expected, actual)
            }
            Error::NotFunction { actual } => write!(f, "{} is not a function", actual),
            Error::NotInScope { name } => write!(f, "The variable {} is not in scope", name),
            Error::UnknownType { name } => write!(f, "The type {} is not in scope", name),
        }
    }
}

fn parse_type(typ: ast::Type) -> Result<Type, Error> {
    match typ {
        ast::Type::Base { name } => match name.as_str() {
            "Int" => Ok(Type::Int),
            "Str" => Ok(Type::Str),
            _ => Err(Error::UnknownType { name }),
        },
        ast::Type::Fun(i, o) => Ok(Type::Fun(
            Box::new(parse_type(*i)?),
            Box::new(parse_type(*o)?),
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{Context, Type};
    use crate::parser;

    fn test(path: &Path) -> Result<Type, String> {
        let mut ctx = Context {
            types: types!("x" => Int, "y" => Int, "x'" => Int, "foo" => Int, "a" => Int, "b" => Int, "c" => Int, "d" => Int, "e" => Int),
        };
        let source = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
        let ast = parser::parse(&source).map_err(|err| err.to_string())?;
        ctx.check(*ast).map_err(|err| err.to_string())
    }

    #[test]
    fn types() {
        insta::glob!("snapshot_inputs/**/*.panda", |path| {
            insta::assert_debug_snapshot!(test(path));
        });
    }
}
