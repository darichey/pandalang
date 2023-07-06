use crate::Type;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub enum Error {
    NotInScope { name: String },
    NoUnify(Type, Type),
    Occurs,
    UnknownType { name: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotInScope { name } => write!(f, "{} is not in scope", name),
            Error::NoUnify(t1, t2) => write!(f, "Could not unify {:?} with {:?}", t1, t2),
            Error::Occurs => write!(f, "Occurs check failed"),
            Error::UnknownType { name } => write!(f, "{} is not a known type", name),
        }
    }
}
