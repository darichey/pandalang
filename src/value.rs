use crate::ast;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Value {
    Int(ast::Int),
    Fun(ast::Fun),
}
