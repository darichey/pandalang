use crate::ast;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Value {
    Int(ast::Int),
    Fun(ast::Fun),
}

impl Value {
    pub fn as_expr(self) -> ast::Expr {
        match self {
            Value::Int(int) => ast::Expr::Int(int),
            Value::Fun(fun) => ast::Expr::Fun(fun),
        }
    }
}
