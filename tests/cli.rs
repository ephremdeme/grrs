use std::io::Write;

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::{prelude::PredicateBooleanExt, str::contains};

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

#[test]
fn supports_case_insensitive_flag() {
    let mut temp = tempfile::NamedTempFile::new().expect("create temp file");
    writeln!(temp, "Hello\nhELLo there\nbye").expect("write temp file");

    let mut cmd = cargo_bin_cmd!("grrs");
    cmd.args([
        "--ignore-case",
        "HELLO",
        temp.path().to_str().expect("utf-8 path"),
    ]);
    cmd.assert()
        .success()
        .stdout(contains("1: Hello"))
        .stdout(contains("2: hELLo there"));
}

#[test]
fn supports_invert_match_flag() {
    let mut temp = tempfile::NamedTempFile::new().expect("create temp file");
    writeln!(temp, "foo\nbar\nbaz foo").expect("write temp file");

    let mut cmd = cargo_bin_cmd!("grrs");
    cmd.args([
        "--invert-match",
        "foo",
        temp.path().to_str().expect("utf-8 path"),
    ]);
    cmd.assert()
        .success()
        .stdout(contains("2: bar"))
        .stdout(contains("1: foo").not())
        .stdout(contains("3: baz foo").not());
}
