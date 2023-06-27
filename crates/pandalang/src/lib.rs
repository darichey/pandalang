#![feature(if_let_guard)]
#![feature(str_split_whitespace_remainder)]

#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate lazy_static;

pub mod ast;
pub mod cli;
pub mod eval;
pub mod parser;
pub mod pretty;
pub mod repl;
pub mod types;
pub mod value;
