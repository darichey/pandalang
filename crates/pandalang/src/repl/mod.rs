// TODO: command aliases
// TODO: "behavioral" commands like :quit (need more than just str -> str)

use std::collections::HashMap;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::eval::Env;
use crate::{parser, types};

lazy_static! {
    static ref COMMANDS: HashMap<&'static str, ReplCommand> = {
        let mut m = HashMap::new();
        m.insert("ast", ast_command());
        m.insert("type", type_check_command());
        m.insert("eval", eval_command());
        m
    };
}

const COMMAND_PREFIX: &str = ":";

fn parse_input(input: &str) -> Result<(&str, &str), &'static str> {
    if let Some(input) = input.strip_prefix(COMMAND_PREFIX) {
        let mut split = input.split_whitespace();
        let cmd = split.next().ok_or("No command")?;
        let source = split.remainder().unwrap_or("");
        Ok((cmd, source))
    } else {
        Ok(("eval", input))
    }
}

pub fn run_repl() -> Result<(), String> {
    let mut rl = Editor::<()>::new().map_err(|err| err.to_string())?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let (cmd, source) = parse_input(&line)?;

                match COMMANDS.get(cmd) {
                    Some(ReplCommand { execute }) => match execute(source) {
                        Ok(result) => println!("{}", result),
                        Err(err) => println!("{}", err),
                    },
                    None => {
                        println!("Unknown command: {}", cmd);
                    }
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

struct ReplCommand {
    execute: fn(&str) -> Result<String, String>,
}

fn ast_command() -> ReplCommand {
    ReplCommand {
        execute: |source| Ok(format!("{:?}", parser::parse_expr(source))),
    }
}

fn eval_command() -> ReplCommand {
    ReplCommand {
        execute: |source| {
            let env = Env::new();
            let ast = *parser::parse_expr(source).map_err(|err| err.to_string())?;
            let value_string = env.eval(ast).to_string();
            Ok(value_string)
        },
    }
}

fn type_check_command() -> ReplCommand {
    ReplCommand {
        execute: |source| {
            let ast = parser::parse_expr(source).map_err(|err| err.to_string())?;
            let type_string = types::check_to_string(*ast).map_err(|err| err.to_string())?;
            Ok(type_string)
        },
    }
}
