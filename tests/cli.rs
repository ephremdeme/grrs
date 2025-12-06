use std::io::Write;

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;

#[test]
fn prints_matching_lines_with_line_numbers() {
    let mut temp = tempfile::NamedTempFile::new().expect("create temp file");
    writeln!(temp, "nope\nhello world\nanother hello").expect("write temp file");

    let mut cmd = cargo_bin_cmd!("grrs");
    cmd.args(["hello", temp.path().to_str().expect("utf-8 path")]);
    cmd.assert()
        .success()
        .stdout(contains("2: hello world"))
        .stdout(contains("3: another hello"));
}
