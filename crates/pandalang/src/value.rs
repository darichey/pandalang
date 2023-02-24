use crate::{ast, eval::Env};

#[derive(Clone)]
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

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(n) => f.debug_tuple("Int").field(n).finish(),
            Self::Str(s) => f.debug_tuple("Str").field(s).finish(),
            Self::Fun { fun, env: _ } => f
                .debug_struct("Fun")
                .field("fun", fun)
                .field("env", &"<opaque>".to_string())
                .finish(),
        }
    }
}
