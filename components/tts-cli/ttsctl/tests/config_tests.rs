//! Configuration loading tests.

use std::io::Write;
use tempfile::NamedTempFile;

// Note: We test config through the binary since config module is private
// These tests verify the config loading behavior

#[test]
fn test_invalid_config_file_error() {
    let mut temp = NamedTempFile::new().expect("failed to create temp file");
    writeln!(temp, "invalid = [toml").expect("failed to write");

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ttsctl"))
        .arg("-c")
        .arg(temp.path())
        .output()
        .expect("failed to run ttsctl");

    // Should fail with parse error
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("parse") || stderr.contains("config"),
        "expected config parse error"
    );
}

#[test]
fn test_missing_config_file_error() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ttsctl"))
        .arg("-c")
        .arg("/nonexistent/path/config.toml")
        .output()
        .expect("failed to run ttsctl");

    assert!(!output.status.success());
}
