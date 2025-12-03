//! CLI argument parsing tests.

use std::process::Command;

#[test]
fn test_version_output_contains_required_fields() {
    let output = Command::new(env!("CARGO_BIN_EXE_ttsctl"))
        .arg("--version")
        .output()
        .expect("failed to run ttsctl");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Copyright"), "missing Copyright");
    assert!(stdout.contains("License"), "missing License");
    assert!(stdout.contains("Repository"), "missing Repository");
    assert!(stdout.contains("Build Host"), "missing Build Host");
    assert!(stdout.contains("Build Commit"), "missing Build Commit");
    assert!(stdout.contains("Build Time"), "missing Build Time");
}

#[test]
fn test_help_contains_ai_agent_instructions() {
    let output = Command::new(env!("CARGO_BIN_EXE_ttsctl"))
        .arg("--help")
        .output()
        .expect("failed to run ttsctl");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("AI CODING AGENT INSTRUCTIONS"),
        "missing AI CODING AGENT INSTRUCTIONS section"
    );
}

#[test]
fn test_short_help_differs_from_long_help() {
    let short = Command::new(env!("CARGO_BIN_EXE_ttsctl"))
        .arg("-h")
        .output()
        .expect("failed to run ttsctl -h");

    let long = Command::new(env!("CARGO_BIN_EXE_ttsctl"))
        .arg("--help")
        .output()
        .expect("failed to run ttsctl --help");

    let short_out = String::from_utf8_lossy(&short.stdout);
    let long_out = String::from_utf8_lossy(&long.stdout);

    // Long help should be longer and contain AI instructions
    assert!(long_out.len() > short_out.len());
    assert!(!short_out.contains("AI CODING AGENT INSTRUCTIONS"));
    assert!(long_out.contains("AI CODING AGENT INSTRUCTIONS"));
}
