#![feature(if_let_guard)]

mod ast;
mod eval;
mod parser;
mod pretty;
mod types;
mod value;

#[macro_use]
extern crate lalrpop_util;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use eval::Env;

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                if let Some(source) = line.strip_prefix("#ast ") {
                    println!("{}", ast(source))
                } else if let Some(source) = line.strip_prefix("#type") {
                    println!("{}", type_check(source))
                } else {
                    let source = match line.strip_prefix("#eval ") {
                        Some(source) => source,
                        None => line.as_str(),
                    };

                    println!("{}", eval(source));
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

fn eval(s: &str) -> String {
    let mut env = Env::new();
    format!("{}", env.eval(*parser::parse(s).unwrap()))
}

fn ast(s: &str) -> String {
    format!("{:?}", parser::parse(s))
}

fn type_check(s: &str) -> String {
    match parser::parse(s) {
        Ok(ast) => match types::check_to_string(*ast) {
            Ok(s) => s,
            Err(e) => format!("{:?}", e),
        },
        Err(err) => format!("{:?}", err),
    }
}
