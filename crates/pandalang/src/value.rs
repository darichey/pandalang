use crate::{ast, eval::Env};

#[derive(Debug, Clone)]
pub enum Value {
    Int(ast::Int),
    Str(ast::Str),
    Fun { fun: ast::Fun, env: Env },
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(ast::Int { n }) => write!(f, "{}", n),
            Value::Str(ast::Str { s }) => write!(f, "{}", s),
            Value::Fun { .. } => write!(f, "<function>"),
        }
    }
}
