use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Text Tool")]
#[command(about = "A CLI tool for basic text transformations", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
