use crate::ast::{
    self,
    expr::Expr,
    stmt::{self, Stmt},
    Program,
};

use self::{check::Checker, error::Error};

mod check;
mod error;
mod monomorphize;
mod polymorphize;
mod string_of_type;
mod tvars;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
enum Type {
    Int,
    Str,
    Unit,
    Bool,
    Var(TVarRef),
    Fun(Box<Type>, Box<Type>),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
enum TVar {
    Bound(Type),
    Unbound(TVarRef, Level),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct TVarRef(usize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Level(usize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
struct Polytype(Vec<TVarRef>, Type);

// TODO: It shouldn't be necessary to export a special function like this.
// This module should have a public Type type that can be consumed by
// other modules. This shouldn't be the existing Type type (i.e., that should)
// remain private to this module, because that would expose all of the
// gross internals of type checking (which could get more gross over time).
// string_of_type does an interesting thing where it kind of "concretizes" a
// type, replacing tvars with actual types... it just happens to then turn them
// all to strings too. We should instead have some "Concretizer" that does this
// process and whatever it spits out can be that exported Type type. We need to
// decide how best to represent unbound tvars for that type first.
pub fn check_expr_to_string(ast: Expr) -> Result<String, Error> {
    let mut checker = Checker::new();
    let typ = checker.check(ast)?;
    Ok(string_of_type::string_of_type(&checker, typ))
}

fn checker_type_of_ast_type(ast_type: ast::types::Type) -> Result<Type, Error> {
    match ast_type {
        ast::types::Type::Simple(name) => match name.as_str() {
            "Int" => Ok(Type::Int),
            "Str" => Ok(Type::Str),
            "Unit" => Ok(Type::Unit),
            "Bool" => Ok(Type::Bool),
            _ => Err(Error::UnknownType { name }),
        },
        ast::types::Type::Fun(ast::types::Fun { from, to }) => Ok(Type::Fun(
            Box::new(checker_type_of_ast_type(*from)?),
            Box::new(checker_type_of_ast_type(*to)?),
        )),
    }
}

pub fn check_prog_to_strings(program: Program) -> Result<Vec<(String, String)>, Error> {
    let mut checker = Checker::new();

    for stmt in program.stmts {
        match stmt {
            Stmt::Let(stmt::Let { name, value, rec }) => {
                checker.check_let_value(name, *value, rec)?;
            }
            Stmt::Declare(stmt::Declare { name, typ }) => {
                let typ = checker_type_of_ast_type(typ)?;
                checker.insert_declare(name, typ);
            }
        }
    }

    // TODO: check type of main

    let mut bindings: Vec<(String, String)> = checker
        .get_bindings()
        .into_iter()
        .map(|(name, typ)| (name, string_of_type::string_of_type(&checker, typ)))
        .collect();

    bindings.sort();

    Ok(bindings)
}
