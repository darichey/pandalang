#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ExprF<A> {
    Int {
        n: i64,
    },
    Var {
        name: String,
    },
    BinOp {
        left: A,
        right: A,
        kind: BinOpKind,
    },
    Fun{
        arg: String,
        body: A,
    },
    App{
        fun: A,
        arg: A,
    },
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Expr(pub Box<ExprF<Expr>>);

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}
