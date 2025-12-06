# grrs

A tiny `grep -n` clone written in Rust. It scans a file for a substring pattern and prints every matching line prefixed with its 1-based line number.

## Features
- Command-line interface powered by `clap`
- Shared search logic exposed through a `search_file` function in the library crate
- Case-insensitive (`-i`/`--ignore-case`) and inverted (`-v`/`--invert-match`) filtering
- Helpful error messages when files cannot be read
- Integration tests using `assert_cmd`, `predicates`, and `tempfile`

## Prerequisites
- Rust toolchain (cargo + rustc). Install via [rustup](https://rustup.rs/).

## Usage
```bash
cargo run -- [OPTIONS] <pattern> <path>
```

**Options**
- `-i`, `--ignore-case` – match without case sensitivity
- `-v`, `--invert-match` – print only the lines that *do not* contain the pattern

**Try it with this repository's README**
```bash
cargo run -- --ignore-case readme README.md
```

**Invert the match** (prints non-matching lines):
```bash
cargo run -- --invert-match readme README.md
```

## Testing
```bash
cargo test
```
This runs unit tests (when present) and the CLI integration test in `tests/cli.rs`.

## Next Steps
Ideas for future improvements:
- Regex-based searches (e.g., using the `regex` crate)
- Reading from `stdin` and searching multiple files recursively
- Additional flags such as `--context`, `--count`, or recursive directory walking
