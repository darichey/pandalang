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
        ast::{BinOp, BinOpKind, Expr, Fun, Int, Let, Var},
        parser::parse,
    };

    #[test]
    fn parses_ints() {
        assert_eq!(parse("0").unwrap(), Box::new(Expr::Int(Int { n: 0 })));
        assert_eq!(parse("10").unwrap(), Box::new(Expr::Int(Int { n: 10 })));
        assert_eq!(parse("-37").unwrap(), Box::new(Expr::Int(Int { n: -37 })));
        assert_eq!(parse("1337").unwrap(), Box::new(Expr::Int(Int { n: 1337 })));
        assert_eq!(parse("-0").unwrap(), Box::new(Expr::Int(Int { n: -0 })));
    }

    #[test]
    fn parses_vars() {
        assert_eq!(
            parse("x").unwrap(),
            Box::new(Expr::Var(Var {
                name: "x".to_string()
            }))
        );
        assert_eq!(
            parse("foo").unwrap(),
            Box::new(Expr::Var(Var {
                name: "foo".to_string()
            }))
        );
        assert_eq!(
            parse("x'").unwrap(),
            Box::new(Expr::Var(Var {
                name: "x'".to_string()
            }))
        );
    }

    #[test]
    fn parses_bin_ops() {
        assert_eq!(
            parse("1 + 1").unwrap(),
            Box::new(Expr::BinOp(BinOp {
                left: Box::new(Expr::Int(Int { n: 1 })),
                right: Box::new(Expr::Int(Int { n: 1 })),
                kind: BinOpKind::Add
            }))
        );

        assert_eq!(
            parse("1 - 1").unwrap(),
            Box::new(Expr::BinOp(BinOp {
                left: Box::new(Expr::Int(Int { n: 1 })),
                right: Box::new(Expr::Int(Int { n: 1 })),
                kind: BinOpKind::Sub
            }))
        );

        assert_eq!(
            parse("1 * 1").unwrap(),
            Box::new(Expr::BinOp(BinOp {
                left: Box::new(Expr::Int(Int { n: 1 })),
                right: Box::new(Expr::Int(Int { n: 1 })),
                kind: BinOpKind::Mul
            }))
        );

        assert_eq!(
            parse("1 / 1").unwrap(),
            Box::new(Expr::BinOp(BinOp {
                left: Box::new(Expr::Int(Int { n: 1 })),
                right: Box::new(Expr::Int(Int { n: 1 })),
                kind: BinOpKind::Div
            }))
        );

        assert_eq!(
            parse("a + b").unwrap(),
            Box::new(Expr::BinOp(BinOp {
                left: Box::new(Expr::Var(Var {
                    name: "a".to_string()
                })),
                right: Box::new(Expr::Var(Var {
                    name: "b".to_string()
                })),
                kind: BinOpKind::Add
            }))
        );

        assert_eq!(
            parse("a + b + c").unwrap(),
            Box::new(Expr::BinOp(BinOp {
                left: Box::new(Expr::BinOp(BinOp {
                    left: Box::new(Expr::Var(Var {
                        name: "a".to_string()
                    })),
                    right: Box::new(Expr::Var(Var {
                        name: "b".to_string()
                    })),
                    kind: BinOpKind::Add
                })),
                right: Box::new(Expr::Var(Var {
                    name: "c".to_string()
                })),
                kind: BinOpKind::Add
            }))
        );

        assert_eq!(
            parse("a + b - c").unwrap(),
            Box::new(Expr::BinOp(BinOp {
                left: Box::new(Expr::BinOp(BinOp {
                    left: Box::new(Expr::Var(Var {
                        name: "a".to_string()
                    })),
                    right: Box::new(Expr::Var(Var {
                        name: "b".to_string()
                    })),
                    kind: BinOpKind::Add
                })),
                right: Box::new(Expr::Var(Var {
                    name: "c".to_string()
                })),
                kind: BinOpKind::Sub
            }))
        );

        assert_eq!(
            parse("a - b + c").unwrap(),
            Box::new(Expr::BinOp(BinOp {
                left: Box::new(Expr::BinOp(BinOp {
                    left: Box::new(Expr::Var(Var {
                        name: "a".to_string()
                    })),
                    right: Box::new(Expr::Var(Var {
                        name: "b".to_string()
                    })),
                    kind: BinOpKind::Sub
                })),
                right: Box::new(Expr::Var(Var {
                    name: "c".to_string()
                })),
                kind: BinOpKind::Add
            }))
        );

        assert_eq!(
            parse("a * b / c").unwrap(),
            Box::new(Expr::BinOp(BinOp {
                left: Box::new(Expr::BinOp(BinOp {
                    left: Box::new(Expr::Var(Var {
                        name: "a".to_string()
                    })),
                    right: Box::new(Expr::Var(Var {
                        name: "b".to_string()
                    })),
                    kind: BinOpKind::Mul
                })),
                right: Box::new(Expr::Var(Var {
                    name: "c".to_string()
                })),
                kind: BinOpKind::Div
            }))
        );

        assert_eq!(
            parse("a + b * c - d / e").unwrap(),
            Box::new(Expr::BinOp(BinOp {
                left: Box::new(Expr::BinOp(BinOp {
                    left: Box::new(Expr::Var(Var {
                        name: "a".to_string()
                    })),
                    right: Box::new(Expr::BinOp(BinOp {
                        left: Box::new(Expr::Var(Var {
                            name: "b".to_string()
                        })),
                        right: Box::new(Expr::Var(Var {
                            name: "c".to_string()
                        })),
                        kind: BinOpKind::Mul
                    })),
                    kind: BinOpKind::Add
                })),
                right: Box::new(Expr::BinOp(BinOp {
                    left: Box::new(Expr::Var(Var {
                        name: "d".to_string()
                    })),
                    right: Box::new(Expr::Var(Var {
                        name: "e".to_string()
                    })),
                    kind: BinOpKind::Div
                })),
                kind: BinOpKind::Sub
            }))
        );
    }

    #[test]
    fn parses_parens() {
        assert_eq!(parse("(0)").unwrap(), Box::new(Expr::Int(Int { n: 0 })));
        assert_eq!(
            parse("(x)").unwrap(),
            Box::new(Expr::Var(Var {
                name: "x".to_string()
            }))
        );

        assert_eq!(
            parse("(a + b)").unwrap(),
            Box::new(Expr::BinOp(BinOp {
                left: Box::new(Expr::Var(Var {
                    name: "a".to_string()
                })),
                right: Box::new(Expr::Var(Var {
                    name: "b".to_string()
                })),
                kind: BinOpKind::Add
            }))
        );

        assert_eq!(
            parse("(a + b) + c").unwrap(),
            Box::new(Expr::BinOp(BinOp {
                left: Box::new(Expr::BinOp(BinOp {
                    left: Box::new(Expr::Var(Var {
                        name: "a".to_string()
                    })),
                    right: Box::new(Expr::Var(Var {
                        name: "b".to_string()
                    })),
                    kind: BinOpKind::Add
                })),
                right: Box::new(Expr::Var(Var {
                    name: "c".to_string()
                })),
                kind: BinOpKind::Add
            }))
        );

        assert_eq!(
            parse("a + (b * c) - (d / e)").unwrap(),
            Box::new(Expr::BinOp(BinOp {
                left: Box::new(Expr::BinOp(BinOp {
                    left: Box::new(Expr::Var(Var {
                        name: "a".to_string()
                    })),
                    right: Box::new(Expr::BinOp(BinOp {
                        left: Box::new(Expr::Var(Var {
                            name: "b".to_string()
                        })),
                        right: Box::new(Expr::Var(Var {
                            name: "c".to_string()
                        })),
                        kind: BinOpKind::Mul
                    })),
                    kind: BinOpKind::Add
                })),
                right: Box::new(Expr::BinOp(BinOp {
                    left: Box::new(Expr::Var(Var {
                        name: "d".to_string()
                    })),
                    right: Box::new(Expr::Var(Var {
                        name: "e".to_string()
                    })),
                    kind: BinOpKind::Div
                })),
                kind: BinOpKind::Sub
            }))
        );

        assert_eq!(
            parse("(a + b * (c - d)) / e").unwrap(),
            Box::new(Expr::BinOp(BinOp {
                left: Box::new(Expr::BinOp(BinOp {
                    left: Box::new(Expr::Var(Var {
                        name: "a".to_string()
                    })),
                    right: Box::new(Expr::BinOp(BinOp {
                        left: Box::new(Expr::Var(Var {
                            name: "b".to_string()
                        })),
                        right: Box::new(Expr::BinOp(BinOp {
                            left: Box::new(Expr::Var(Var {
                                name: "c".to_string()
                            })),
                            right: Box::new(Expr::Var(Var {
                                name: "d".to_string()
                            })),
                            kind: BinOpKind::Sub
                        })),
                        kind: BinOpKind::Mul
                    })),
                    kind: BinOpKind::Add
                })),
                right: Box::new(Expr::Var(Var {
                    name: "e".to_string()
                })),
                kind: BinOpKind::Div
            }))
        );
    }

    #[test]
    fn parses_let() {
        assert_eq!(
            parse("let x = 3 in x").unwrap(),
            Box::new(Expr::Let(Let {
                name: "x".to_string(),
                value: Box::new(Expr::Int(Int { n: 3 })),
                body: Box::new(Expr::Var(Var {
                    name: "x".to_string()
                }))
            }))
        )
    }

    #[test]
    fn parses_let_nested() {
        assert_eq!(
            parse("let x = 3 in let y = 5 in x + y").unwrap(),
            Box::new(Expr::Let(Let {
                name: "x".to_string(),
                value: Box::new(Expr::Int(Int { n: 3 })),
                body: Box::new(Expr::Let(Let {
                    name: "y".to_string(),
                    value: Box::new(Expr::Int(Int { n: 5 })),
                    body: Box::new(Expr::BinOp(BinOp {
                        left: Box::new(Expr::Var(Var {
                            name: "x".to_string()
                        })),
                        right: Box::new(Expr::Var(Var {
                            name: "y".to_string()
                        })),
                        kind: BinOpKind::Add
                    }))
                }))
            }))
        )
    }

    #[test]
    fn parses_fun() {
        assert_eq!(
            parse("fun x -> x").unwrap(),
            Box::new(Expr::Fun(Fun {
                arg: "x".to_string(),
                body: Box::new(Expr::Var(Var {
                    name: "x".to_string()
                }))
            }))
        )
    }

    #[test]
    fn parses_fun_nested() {
        assert_eq!(
            parse("fun x -> fun y -> x + y").unwrap(),
            Box::new(Expr::Fun(Fun {
                arg: "x".to_string(),
                body: Box::new(Expr::Fun(Fun {
                    arg: "y".to_string(),
                    body: Box::new(Expr::BinOp(BinOp {
                        left: Box::new(Expr::Var(Var {
                            name: "x".to_string()
                        })),
                        right: Box::new(Expr::Var(Var {
                            name: "y".to_string()
                        })),
                        kind: BinOpKind::Add
                    }))
                }))
            }))
        )
    }
}
