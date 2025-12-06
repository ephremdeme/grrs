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

#[derive(Debug, Clone, Copy, Default)]
pub struct SearchOptions {
    pub case_insensitive: bool,
    pub invert_match: bool,
}

pub fn search_file(pattern: &str, path: &Path, options: SearchOptions) -> Result<Vec<String>, CustomError> {
    let file = File::open(path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", path.display(), err)))?;
    let reader = BufReader::new(file);
    let mut matches = Vec::new();
    let normalized_pattern = options
        .case_insensitive
        .then(|| pattern.to_lowercase());

    for (idx, line) in reader.lines().enumerate() {
        let content = line
            .map_err(|err| CustomError(format!("Error reading `{}`: {}", path.display(), err)))?;
        let is_match = if options.case_insensitive {
            let needle = normalized_pattern
                .as_deref()
                .expect("lowercased pattern available when option set");
            content.to_lowercase().contains(needle)
        } else {
            content.contains(pattern)
        };

        let should_include = if options.invert_match {
            !is_match
        } else {
            is_match
        };

        if should_include {
            matches.push(format!("{}: {}", idx + 1, content));
        }
    }

    Ok(matches)
}
