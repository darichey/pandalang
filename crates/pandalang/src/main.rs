#![feature(if_let_guard)]
#![feature(str_split_whitespace_remainder)]

mod ast;
mod eval;
mod parser;
mod pretty;
mod repl;
mod types;
mod value;

#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate lazy_static;

fn main() -> Result<(), String> {
    repl::run_repl()
}
