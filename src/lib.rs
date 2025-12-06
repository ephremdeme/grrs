use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct CustomError(pub String);

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError {}

pub fn search_file(pattern: &str, path: &Path) -> Result<Vec<String>, CustomError> {
    let file = File::open(path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", path.display(), err)))?;
    let reader = BufReader::new(file);
    let mut matches = Vec::new();

    for (idx, line) in reader.lines().enumerate() {
        let content = line
            .map_err(|err| CustomError(format!("Error reading `{}`: {}", path.display(), err)))?;
        if content.contains(pattern) {
            matches.push(format!("{}: {}", idx + 1, content));
        }
    }

    Ok(matches)
}
