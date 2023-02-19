use crate::{ast, eval::Env};

#[derive(Debug, Clone)]
pub enum Value {
    Int(ast::Int),
    Str(ast::Str),
    Fun { fun: ast::Fun, env: Env },
}
