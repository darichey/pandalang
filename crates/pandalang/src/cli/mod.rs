use clap::{Parser, Subcommand};

use crate::repl;

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
}

pub fn run_cli() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Repl => repl::run_repl(),
    }
}
