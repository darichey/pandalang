#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expr {
    Int(Int),
    Str(Str),
    Var(Var),
    BinOp(BinOp),
    Let(Let),
    Fun(Fun),
    App(App),
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
    pub patt: Pattern,
    pub value: Box<Expr>,
    pub body: Box<Expr>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Fun {
    pub patt: Pattern,
    pub body: Box<Expr>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct App {
    pub fun: Box<Expr>,
    pub arg: Box<Expr>,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

impl ToString for BinOpKind {
    fn to_string(&self) -> String {
        match self {
            BinOpKind::Add => "+",
            BinOpKind::Sub => "-",
            BinOpKind::Mul => "*",
            BinOpKind::Div => "/",
        }
        .to_string()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Pattern {
    Id {
        name: String,
        // typ: Type
    },
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Type {
    Base { name: String },
    Fun(Box<Type>, Box<Type>),
}
