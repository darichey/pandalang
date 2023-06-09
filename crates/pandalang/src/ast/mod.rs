use self::stmt::Stmt;

pub mod expr;
pub mod stmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}
