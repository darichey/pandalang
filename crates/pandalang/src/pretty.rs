use crate::ast::expr::{App, BinOp, Bool, Expr, Fun, If, Int, Let, Str, Var};

#[allow(unused)] // TODO: actually implement a pretty printer and expose it
pub fn pretty(e: Expr) -> String {
    match e {
        Expr::Int(Int { n }) => n.to_string(),
        Expr::Str(Str { s }) => format!("\"{}\"", s),
        Expr::Unit => "()".to_string(),
        Expr::Bool(Bool { b }) => b.to_string(),
        Expr::Var(Var { name }) => name,
        Expr::BinOp(BinOp { left, right, kind }) => {
            format!("{} {} {}", pretty(*left), kind.to_string(), pretty(*right))
        }
        Expr::Let(Let {
            name,
            value,
            body,
            rec,
        }) => {
            format!("let {} = {} in {}", name, pretty(*value), pretty(*body))
        }
        Expr::Fun(Fun { arg, body }) => {
            format!("fun {} -> {}", arg, pretty(*body))
        }
        Expr::App(App { fun, arg }) => format!("({}) ({})", pretty(*fun), pretty(*arg)),
        Expr::If(If { check, then, els }) => format!(
            "if {} then {} else {}",
            pretty(*check),
            pretty(*then),
            pretty(*els)
        ),
    }
}
