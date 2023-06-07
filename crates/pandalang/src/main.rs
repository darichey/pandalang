#![feature(if_let_guard)]

mod ast;
mod eval;
mod parser;
mod pretty;
mod repl;
mod types;
mod value;

#[macro_use]
extern crate lalrpop_util;

fn main() -> rustyline::Result<()> {
    repl::run_repl()
}
