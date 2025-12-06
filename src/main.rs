use clap::Parser;
use std::io::{self, Write};


use grrs::search_file;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    
    let stdout = io::stdout();
    let mut handle = stdout.lock(); // acquire a lock on it

    let matches = search_file(&args.pattern, &args.path)?;
    for line in matches {
        writeln!(handle, "{}", line)?;
    }
    Ok(())
}

