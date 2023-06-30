use super::{expr::Expr, types::Type};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Stmt {
    Let(Let),
    Declare(Declare),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Let {
    pub name: String,
    pub value: Box<Expr>,
    pub rec: bool,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Declare {
    pub name: String,
    pub typ: Type,
}
