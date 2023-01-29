use crate::ast::{App, BinOp, Expr, Fun, Let};

pub fn desugar_let(e: Expr) -> Expr {
    match e {
        Expr::Int(x) => Expr::Int(x),
        Expr::Var(v) => Expr::Var(v),
        Expr::BinOp(BinOp { left, right, kind }) => Expr::BinOp(BinOp {
            left: Box::new(desugar_let(*left)),
            right: Box::new(desugar_let(*right)),
            kind,
        }),
        Expr::Let(Let { name, value, body }) => Expr::App(App {
            fun: Box::new(Expr::Fun(Fun {
                arg: name,
                body: Box::new(desugar_let(*body)),
            })),
            arg: Box::new(desugar_let(*value)),
        }),
        Expr::Fun(Fun { arg, body }) => Expr::Fun(Fun {
            arg,
            body: Box::new(desugar_let(*body)),
        }),
        Expr::App(App { fun, arg }) => Expr::App(App {
            fun: Box::new(desugar_let(*fun)),
            arg: Box::new(desugar_let(*arg)),
        }),
    }
}
