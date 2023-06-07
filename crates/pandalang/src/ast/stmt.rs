use super::expr::Expr;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Stmt {
    Let(Let),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Let {
    pub name: String,
    pub value: Box<Expr>,
}
