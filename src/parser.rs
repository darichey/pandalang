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
    use crate::{
        ast::{BinOpKind, Expr},
        parser::parse,
    };

    #[test]
    fn parses_ints() {
        assert_eq!(parse("0").unwrap(), Box::new(Expr::Int(0)));
        assert_eq!(parse("10").unwrap(), Box::new(Expr::Int(10)));
        assert_eq!(parse("-37").unwrap(), Box::new(Expr::Int(-37)));
        assert_eq!(parse("1337").unwrap(), Box::new(Expr::Int(1337)));
        assert_eq!(parse("-0").unwrap(), Box::new(Expr::Int(-0)));
    }

    #[test]
    fn parses_vars() {
        assert_eq!(parse("x").unwrap(), Box::new(Expr::Var("x".to_string())));
        assert_eq!(
            parse("foo").unwrap(),
            Box::new(Expr::Var("foo".to_string()))
        );
        assert_eq!(parse("x'").unwrap(), Box::new(Expr::Var("x'".to_string())));
    }

    #[test]
    fn parses_bin_ops() {
        assert_eq!(
            parse("1 + 1").unwrap(),
            Box::new(Expr::BinOp {
                left: Box::new(Expr::Int(1)),
                right: Box::new(Expr::Int(1)),
                kind: BinOpKind::Add
            })
        );

        assert_eq!(
            parse("1 - 1").unwrap(),
            Box::new(Expr::BinOp {
                left: Box::new(Expr::Int(1)),
                right: Box::new(Expr::Int(1)),
                kind: BinOpKind::Sub
            })
        );

        assert_eq!(
            parse("1 * 1").unwrap(),
            Box::new(Expr::BinOp {
                left: Box::new(Expr::Int(1)),
                right: Box::new(Expr::Int(1)),
                kind: BinOpKind::Mul
            })
        );

        assert_eq!(
            parse("1 / 1").unwrap(),
            Box::new(Expr::BinOp {
                left: Box::new(Expr::Int(1)),
                right: Box::new(Expr::Int(1)),
                kind: BinOpKind::Div
            })
        );

        assert_eq!(
            parse("a + b").unwrap(),
            Box::new(Expr::BinOp {
                left: Box::new(Expr::Var("a".to_string())),
                right: Box::new(Expr::Var("b".to_string())),
                kind: BinOpKind::Add
            })
        );

        assert_eq!(
            parse("a + b + c").unwrap(),
            Box::new(Expr::BinOp {
                left: Box::new(Expr::BinOp {
                    left: Box::new(Expr::Var("a".to_string())),
                    right: Box::new(Expr::Var("b".to_string())),
                    kind: BinOpKind::Add
                }),
                right: Box::new(Expr::Var("c".to_string())),
                kind: BinOpKind::Add
            })
        );

        assert_eq!(
            parse("a + b - c").unwrap(),
            Box::new(Expr::BinOp {
                left: Box::new(Expr::BinOp {
                    left: Box::new(Expr::Var("a".to_string())),
                    right: Box::new(Expr::Var("b".to_string())),
                    kind: BinOpKind::Add
                }),
                right: Box::new(Expr::Var("c".to_string())),
                kind: BinOpKind::Sub
            })
        );

        assert_eq!(
            parse("a - b + c").unwrap(),
            Box::new(Expr::BinOp {
                left: Box::new(Expr::BinOp {
                    left: Box::new(Expr::Var("a".to_string())),
                    right: Box::new(Expr::Var("b".to_string())),
                    kind: BinOpKind::Sub
                }),
                right: Box::new(Expr::Var("c".to_string())),
                kind: BinOpKind::Add
            })
        );

        assert_eq!(
            parse("a * b / c").unwrap(),
            Box::new(Expr::BinOp {
                left: Box::new(Expr::BinOp {
                    left: Box::new(Expr::Var("a".to_string())),
                    right: Box::new(Expr::Var("b".to_string())),
                    kind: BinOpKind::Mul
                }),
                right: Box::new(Expr::Var("c".to_string())),
                kind: BinOpKind::Div
            })
        );

        assert_eq!(
            parse("a + b * c - d / e").unwrap(),
            Box::new(Expr::BinOp {
                left: Box::new(Expr::BinOp {
                    left: Box::new(Expr::Var("a".to_string())),
                    right: Box::new(Expr::BinOp {
                        left: Box::new(Expr::Var("b".to_string())),
                        right: Box::new(Expr::Var("c".to_string())),
                        kind: BinOpKind::Mul
                    }),
                    kind: BinOpKind::Add
                }),
                right: Box::new(Expr::BinOp {
                    left: Box::new(Expr::Var("d".to_string())),
                    right: Box::new(Expr::Var("e".to_string())),
                    kind: BinOpKind::Div
                }),
                kind: BinOpKind::Sub
            })
        );
    }

    #[test]
    fn parses_parens() {
        assert_eq!(parse("(0)").unwrap(), Box::new(Expr::Int(0)));
        assert_eq!(parse("(x)").unwrap(), Box::new(Expr::Var("x".to_string())));

        assert_eq!(
            parse("(a + b)").unwrap(),
            Box::new(Expr::BinOp {
                left: Box::new(Expr::Var("a".to_string())),
                right: Box::new(Expr::Var("b".to_string())),
                kind: BinOpKind::Add
            })
        );

        assert_eq!(
            parse("(a + b) + c").unwrap(),
            Box::new(Expr::BinOp {
                left: Box::new(Expr::BinOp {
                    left: Box::new(Expr::Var("a".to_string())),
                    right: Box::new(Expr::Var("b".to_string())),
                    kind: BinOpKind::Add
                }),
                right: Box::new(Expr::Var("c".to_string())),
                kind: BinOpKind::Add
            })
        );

        assert_eq!(
            parse("a + (b * c) - (d / e)").unwrap(),
            Box::new(Expr::BinOp {
                left: Box::new(Expr::BinOp {
                    left: Box::new(Expr::Var("a".to_string())),
                    right: Box::new(Expr::BinOp {
                        left: Box::new(Expr::Var("b".to_string())),
                        right: Box::new(Expr::Var("c".to_string())),
                        kind: BinOpKind::Mul
                    }),
                    kind: BinOpKind::Add
                }),
                right: Box::new(Expr::BinOp {
                    left: Box::new(Expr::Var("d".to_string())),
                    right: Box::new(Expr::Var("e".to_string())),
                    kind: BinOpKind::Div
                }),
                kind: BinOpKind::Sub
            })
        );

        assert_eq!(
            parse("(a + b * (c - d)) / e").unwrap(),
            Box::new(Expr::BinOp {
                left: Box::new(Expr::BinOp {
                    left: Box::new(Expr::Var("a".to_string())),
                    right: Box::new(Expr::BinOp {
                        left: Box::new(Expr::Var("b".to_string())),
                        right: Box::new(Expr::BinOp {
                            left: Box::new(Expr::Var("c".to_string())),
                            right: Box::new(Expr::Var("d".to_string())),
                            kind: BinOpKind::Sub
                        }),
                        kind: BinOpKind::Mul
                    }),
                    kind: BinOpKind::Add
                }),
                right: Box::new(Expr::Var("e".to_string())),
                kind: BinOpKind::Div
            })
        );
    }
}
