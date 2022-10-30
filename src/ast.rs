use recursion::{
    map_layer::MapLayer,
    recursive_tree::{RecursiveTree, StackMarker},
    Collapse, Expand,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ExprF<A> {
    Int { n: i64 },
    Var { name: String },
    BinOp { left: A, right: A, kind: BinOpKind },
    Fun { arg: String, body: A },
    App { fun: A, arg: A },
}

impl<A, B> MapLayer<B> for ExprF<A> {
    type To = ExprF<B>;
    type Unwrapped = A;

    fn map_layer<F: FnMut(Self::Unwrapped) -> B>(self, mut f: F) -> Self::To {
        match self {
            ExprF::Int { n } => ExprF::Int { n },
            ExprF::Var { name } => ExprF::Var { name },
            ExprF::BinOp { left, right, kind } => ExprF::BinOp {
                left: f(left),
                right: f(right),
                kind,
            },
            ExprF::Fun { arg, body } => ExprF::Fun { arg, body: f(body) },
            ExprF::App { fun, arg } => ExprF::App {
                fun: f(fun),
                arg: f(arg),
            },
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ExprBoxed(pub Box<ExprF<ExprBoxed>>);

pub type Expr = RecursiveTree<ExprF<StackMarker>, StackMarker>;

impl From<ExprBoxed> for Expr {
    fn from(expr_boxed: ExprBoxed) -> Self {
        Expand::expand_layers(expr_boxed, |ExprBoxed(boxed)| *boxed)
    }
}

impl From<Expr> for ExprBoxed {
    fn from(expr: Expr) -> Self {
        Collapse::collapse_layers(expr, |expr| ExprBoxed(Box::new(expr)))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}
