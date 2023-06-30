pub mod ast;

use ast::{expr::Expr, types::Type, Program};
use lalrpop_util::{lalrpop_mod, lexer::Token, ParseError};

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub grammar
);

pub fn parse(s: &str) -> Result<Program, ParseError<usize, Token<'_>, &'static str>> {
    grammar::ProgramParser::new().parse(s)
}

pub fn parse_expr(s: &str) -> Result<Box<Expr>, ParseError<usize, Token<'_>, &'static str>> {
    grammar::ExprParser::new().parse(s)
}

pub fn parse_type(s: &str) -> Result<Box<Type>, ParseError<usize, Token<'_>, &'static str>> {
    grammar::TypeParser::new().parse(s)
}
