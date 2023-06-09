use lalrpop_util::{lexer::Token, ParseError};

use crate::ast::{expr::Expr, Program};

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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{
        ast::{expr::Expr, Program},
        parser,
    };

    fn test_expr(path: &Path) -> Result<Box<Expr>, String> {
        let source = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
        parser::parse_expr(&source).map_err(|err| err.to_string())
    }

    #[test]
    fn parses_exprs() {
        insta::glob!("snapshot_inputs/exprs/**/*.panda", |path| {
            insta::assert_debug_snapshot!(test_expr(path));
        });
    }

    fn test_prog(path: &Path) -> Result<Program, String> {
        let source = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
        parser::parse(&source).map_err(|err| err.to_string())
    }

    #[test]
    fn parses_progs() {
        insta::glob!("snapshot_inputs/progs/**/*.panda", |path| {
            insta::assert_debug_snapshot!(test_prog(path));
        });
    }
}
