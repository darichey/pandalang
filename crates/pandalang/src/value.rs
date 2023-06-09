use crate::{ast::expr::*, eval::Env};

#[derive(Clone)]
pub enum Value {
    Int(Int),
    Str(Str),
    Fun { fun: Fun, env: Env },
    Builtin(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(Int { n }) => write!(f, "{}", n),
            Value::Str(Str { s }) => write!(f, "{}", s),
            Value::Fun { .. } => write!(f, "<function>"),
            Value::Builtin(_) => write!(f, "<builtin>"),
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
            Self::Builtin(name) => f.debug_tuple("Builtin").field(name).finish(),
        }
    }
}
