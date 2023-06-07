use crate::ast::expr::{App, BinOp, Expr, Fun, Int, Let, Str, Var};

pub fn pretty(e: Expr) -> String {
    match e {
        Expr::Int(Int { n }) => n.to_string(),
        Expr::Str(Str { s }) => format!("\"{}\"", s),
        Expr::Var(Var { name }) => name,
        Expr::BinOp(BinOp { left, right, kind }) => {
            format!("{} {} {}", pretty(*left), kind.to_string(), pretty(*right))
        }
        Expr::Let(Let { name, value, body }) => {
            format!("let {} = {} in {}", name, pretty(*value), pretty(*body))
        }
        Expr::Fun(Fun { arg, body }) => {
            format!("fun {} -> {}", arg, pretty(*body))
        }
        Expr::App(App { fun, arg }) => format!("({}) ({})", pretty(*fun), pretty(*arg)),
    }
}
