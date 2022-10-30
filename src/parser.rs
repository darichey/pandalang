use lalrpop_util::{lexer::Token, ParseError};

use crate::ast::Expr;

lalrpop_mod!(pub grammar);

pub fn parse<'input>(
    s: &'input str,
) -> Result<Expr, ParseError<usize, Token<'input>, &'static str>> {
    grammar::ExprParser::new().parse(s)
}

#[cfg(test)]
mod tests {
    use crate::{ast::Expr, parser};

    fn parse(s: String) -> Expr {
        parser::parse(s.as_str()).unwrap()
    }

    #[test]
    fn parses() {
        insta::glob!("snapshot_inputs/**/*.panda", |path| {
            let source = parse(std::fs::read_to_string(&path).unwrap());
            insta::assert_debug_snapshot!(source);
        });
    }
}
