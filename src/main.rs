mod ast;
mod eval;
mod parser;

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
                } else {
                    let source = match line.strip_prefix("#eval ") {
                        Some(source) => source,
                        None => line.as_str(),
                    };

                    println!("{}", eval(source)); // TODO: pretty print this
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

fn eval<'a>(s: &'a str) -> String {
    format!(
        "{:?}",
        eval::eval(parser::parse(s).unwrap(), &eval::new_env!())
    )
}

fn ast<'a>(s: &'a str) -> String {
    format!("{:?}", parser::parse(s))
}
