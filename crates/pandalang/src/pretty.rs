use crate::ast::{App, BinOp, Expr, Fun, Int, Let, Pattern, Str, Var};

pub fn pretty(e: Expr) -> String {
    match e {
        Expr::Int(Int { n }) => n.to_string(),
        Expr::Str(Str { s }) => format!("\"{}\"", s),
        Expr::Var(Var { name }) => name,
        Expr::BinOp(BinOp { left, right, kind }) => {
            format!("{} {} {}", pretty(*left), kind.to_string(), pretty(*right))
        }
        Expr::Let(Let { patt, value, body }) => {
            format!(
                "let {} = {} in {}",
                pretty_pattern(patt),
                pretty(*value),
                pretty(*body)
            )
        }
        Expr::Fun(Fun { patt, body }) => {
            format!("fun {} -> {}", pretty_pattern(patt), pretty(*body))
        }
        Expr::App(App { fun, arg }) => format!("({}) ({})", pretty(*fun), pretty(*arg)),
    }
}

fn pretty_pattern(patt: Pattern) -> String {
    match patt {
        Pattern::Id { name } => name,
    }
}
