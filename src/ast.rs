#[derive(PartialEq, Eq, Debug)]
pub enum Expr {
    Int(i64),
    Var(String),
    BinOp {
        left: Box<Expr>,
        right: Box<Expr>,
        kind: BinOpKind,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}
