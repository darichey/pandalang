use crate::{ast::expr::*, eval::Env};

#[derive(Clone)]
pub enum Value {
    Int(Int),
    Str(Str),
    Unit,
    Bool(Bool),
    Fun { fun: Fun, env: Env },
    Builtin(String),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Str(l0), Self::Str(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(Int { n }) => write!(f, "{}", n),
            Value::Str(Str { s }) => write!(f, "{}", s),
            Value::Unit => write!(f, "()"),
            Value::Bool(Bool { b }) => write!(f, "{}", b),
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
            Self::Unit => f.debug_tuple("Unit").finish(),
            Self::Bool(b) => f.debug_tuple("Bool").field(b).finish(),
            Self::Fun { fun, env: _ } => f
                .debug_struct("Fun")
                .field("fun", fun)
                .field("env", &"<opaque>".to_string())
                .finish(),
            Self::Builtin(name) => f.debug_tuple("Builtin").field(name).finish(),
        }
    }
}
