use lalrpop_util::{lexer::Token, ParseError};

use crate::ast::Expr;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub grammar
);

pub fn parse(s: &str) -> Result<Box<Expr>, ParseError<usize, Token<'_>, &'static str>> {
    grammar::ExprParser::new().parse(s)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{ast::Expr, parser};

    fn test(path: &Path) -> Result<Box<Expr>, String> {
        let source = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
        parser::parse(&source).map_err(|err| err.to_string())
    }

    #[test]
    fn parses() {
        insta::glob!("snapshot_inputs/**/*.panda", |path| {
            insta::assert_debug_snapshot!(test(path));
        });
    }
}
