use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};

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

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Repl => pandalang_repl::run_repl(),
        Commands::Run { program } => {
            let src = fs::read_to_string(program).map_err(|err| err.to_string())?;
            let ast = pandalang_parser::parse(&src).map_err(|err| err.to_string())?;
            pandalang_types::check_prog_to_strings(ast.clone()).map_err(|err| err.to_string())?;
            let mut stdout = std::io::stdout();
            let value = pandalang_eval::run_program(ast, &mut stdout)?;
            println!("{}", value);
            Ok(())
        }
    }
}
