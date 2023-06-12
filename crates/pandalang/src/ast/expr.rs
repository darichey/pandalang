#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    Int(Int),
    Str(Str),
    Unit,
    Bool(Bool),
    Var(Var),
    BinOp(BinOp),
    Let(Let),
    Fun(Fun),
    App(App),
    If(If),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Int {
    pub n: i64,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Str {
    pub s: String,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Bool {
    pub b: bool,
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Let {
    pub name: String,
    pub value: Box<Expr>,
    pub body: Box<Expr>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Fun {
    pub arg: String,
    pub body: Box<Expr>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct App {
    pub fun: Box<Expr>,
    pub arg: Box<Expr>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct If {
    pub check: Box<Expr>,
    pub then: Box<Expr>,
    pub els: Box<Expr>,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Eql,
}

impl ToString for BinOpKind {
    fn to_string(&self) -> String {
        match self {
            BinOpKind::Add => "+",
            BinOpKind::Sub => "-",
            BinOpKind::Mul => "*",
            BinOpKind::Div => "/",
            BinOpKind::Eql => "==",
        }
        .to_string()
    }
}
