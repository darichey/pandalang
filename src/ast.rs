#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    Int(Int),
    Var(Var),
    BinOp(BinOp),
    // Let(Let),
    Fun(Fun),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Int {
    pub n: i64,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Var {
    pub name: String,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BinOp {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub kind: BinOpKind,
}

// #[derive(PartialEq, Eq, Debug, Clone)]
// pub struct Let {
//     pub name: String,
//     pub value: Box<Expr>,
//     pub body: Box<Expr>,
// }

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Fun {
    pub arg: String,
    pub body: Box<Expr>,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}
