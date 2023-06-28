use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::{eval, parser, repl};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts the repl
    Repl,
    /// Runs a program
    Run { program: PathBuf },
}

pub fn run_cli() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Repl => repl::run_repl(),
        Commands::Run { program } => {
            let src = fs::read_to_string(program).map_err(|err| err.to_string())?;
            let ast = parser::parse(&src).map_err(|err| err.to_string())?;
            let mut stdout = std::io::stdout();
            let value = eval::run_program(ast, &mut stdout)?;
            println!("{}", value);
            Ok(())
        }
    }
}
