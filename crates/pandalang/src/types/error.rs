#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub enum Error {
    NotInScope { name: String },
    NoUnify,
    Occurs,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotInScope { name } => write!(f, "{} is not in scope", name),
            Error::NoUnify => write!(f, "Unification failure"),
            Error::Occurs => write!(f, "Occurs check failed"),
        }
    }
}
