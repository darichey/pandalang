// TODO: syntax for polymorphic types
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    Simple(String),
    Fun(Fun),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Fun {
    pub from: Box<Type>,
    pub to: Box<Type>,
}
