#[derive(PartialEq, Eq, Debug)]
pub enum Expr {
    Int(i64),
    Var(String),
}
