use clap::Parser;
use std::io::{self, Write};

use grrs::{search_file, SearchOptions};


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
    /// Match case-insensitively
    #[arg(short = 'i', long = "ignore-case")]
    ignore_case: bool,
    /// Only print lines that do not contain the pattern
    #[arg(short = 'v', long = "invert-match")]
    invert_match: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    
    let stdout = io::stdout();
    let mut handle = stdout.lock(); // acquire a lock on it

    let matches = search_file(
        &args.pattern,
        &args.path,
        SearchOptions {
            case_insensitive: args.ignore_case,
            invert_match: args.invert_match,
        },
    )?;
    for line in matches {
        writeln!(handle, "{}", line)?;
    }
    Ok(())
}

