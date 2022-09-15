use lalrpop_util::{lexer::Token, ParseError};

use crate::ast::Expr;

lalrpop_mod!(pub grammar);

pub fn parse<'input>(
    s: &'input str,
) -> Result<Box<Expr>, ParseError<usize, Token<'input>, &'static str>> {
    grammar::ExprParser::new().parse(s)
}

#[cfg(test)]
mod tests {
    use crate::{parser::parse, ast::Expr};

    #[test]
    fn parses_ints() {
        assert_eq!(*parse("0").unwrap(), Expr::Int(0));
        assert_eq!(*parse("10").unwrap(), Expr::Int(10));
        assert_eq!(*parse("-37").unwrap(), Expr::Int(-37));
        assert_eq!(*parse("1337").unwrap(), Expr::Int(1337));
        assert_eq!(*parse("-0").unwrap(), Expr::Int(-0));
    }

    #[test]
    fn parses_vars() {
        assert_eq!(*parse("x").unwrap(), Expr::Var("x".to_string()));
        assert_eq!(*parse("foo").unwrap(), Expr::Var("foo".to_string()));
        assert_eq!(*parse("x'").unwrap(), Expr::Var("x'".to_string()));
    }
}
