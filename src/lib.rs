use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug)]
pub enum CliError {
    Io(std::io::Error),
    MissingArgument(String),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CliError::Io(err) => write!(f, "I/O error: {}", err),
            CliError::MissingArgument(arg) => write!(f, "Missing required argument: {}", arg),
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> CliError {
        CliError::Io(err)
    }
}

pub fn read_input(input: Option<String>) -> Result<String, CliError> {
    let mut content = String::new();

    match input {
        Some(path) if path == "-" => {
            // If the input is "-" (indicating STDIN), read from STDIN
            std::io::stdin().read_to_string(&mut content)?;
        }
        Some(path) => {
            // Otherwise, read from the specified file
            let mut file = File::open(path)?;
            file.read_to_string(&mut content)?;
        }
        None => {
            // If no path is provided, also read from STDIN
            std::io::stdin().read_to_string(&mut content)?;
        }
    }

    Ok(content)
}



pub fn write_output(content: String, output: Option<String>) -> Result<(), CliError> {
    if let Some(path) = output {
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
    } else {
        println!("{}", content);
    }
    Ok(())
}

pub fn uppercase(input: &str) -> String {
    input.to_uppercase()
}

pub fn lowercase(input: &str) -> String {
    input.to_lowercase()
}

pub fn replace(input: &str, from: &str, to: &str) -> String {
    input.replace(from, to)
}
