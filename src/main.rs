mod config;
mod lib;
mod cli;

use clap::{Parser, Subcommand};

use cli::{Commands, Cli};
use log::{error};
use std::process::ExitCode;



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
