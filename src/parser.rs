use lalrpop_util::{lexer::Token, ParseError};
use recursion::Expand;

use crate::ast::{ExprBoxed, Expr};

lalrpop_mod!(pub grammar);

pub fn parse<'input>(
    s: &'input str,
) -> Result<Expr, ParseError<usize, Token<'input>, &'static str>> {
    let expr_boxed = grammar::ExprParser::new().parse(s)?;
    Ok(Expand::expand_layers(expr_boxed, |ExprBoxed(boxed)| *boxed))
}

#[cfg(test)]
mod tests {
    use crate::{ast::ExprBoxed, parser};

    fn parse(s: String) -> ExprBoxed {
        parser::parse(s.as_str()).unwrap().into()
    }

    #[test]
    fn parses() {
        insta::glob!("snapshot_inputs/**/*.panda", |path| {
            let source = parse(std::fs::read_to_string(&path).unwrap());
            insta::assert_debug_snapshot!(source);
        });
    }
}
