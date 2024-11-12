mod config;
mod lib;

use clap::{Parser, Subcommand};
use log::{error};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "Text Tool")]
#[command(about = "A CLI tool for basic text transformations", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Uppercase { input: Option<String>, output: Option<String> },
    Lowercase { input: Option<String>, output: Option<String> },
    Replace {
        #[arg(required = true)]
        input: String,    // Make `input` required to avoid clap errors
        #[arg(required = true)]
        from: String,
        #[arg(required = true)]
        to: String,
        output: Option<String>,
    },
}


fn main() -> ExitCode {
    env_logger::init();

    let args = Cli::parse();

    match run(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            error!("{}", err);
            ExitCode::FAILURE
        }
    }
}

fn run(args: Cli) -> Result<(), lib::CliError> {
    match args.command {
        Commands::Uppercase { input, output } => {
            let content = lib::read_input(input)?;
            let result = lib::uppercase(&content);
            lib::write_output(result, output)?;
        }
        Commands::Lowercase { input, output } => {
            let content = lib::read_input(input)?;
            let result = lib::lowercase(&content);
            lib::write_output(result, output)?;
        }
        Commands::Replace { input, from, to, output } => {
            let content = lib::read_input(Some(input))?;
            let result = lib::replace(&content, &from, &to);
            lib::write_output(result, output)?;
        }
    }
    Ok(())
}
