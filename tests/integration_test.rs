use assert_cmd::Command;
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_uppercase_command() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(temp_file, "hello world").expect("Failed to write to temp file");

    let mut cmd = Command::cargo_bin("text_tool").unwrap();
    cmd.arg("uppercase")
        .arg(temp_file.path())
        .assert()
        .success()
        .stdout(predicates::str::contains("HELLO WORLD"));
}

#[test]
fn test_missing_argument_error() {
    let mut cmd = Command::cargo_bin("text_tool").unwrap();

    cmd.arg("replace")
        .arg("input.txt")  // Provide `input` argument to trigger missing `from` and `to` args
        .assert()
        .failure()
        .stderr(predicates::str::contains("the following required arguments were not provided"));
}

