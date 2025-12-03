//! Tests for script parsing functions.

use std::io::Cursor;
use tts_spec_script::parse::{from_json_str, from_yaml_reader, from_yaml_str};

#[test]
fn test_from_yaml_str_valid() {
    let yaml = r#"
sample_rate: 24000
segments:
  - id: intro
    speaker: mike
    engine: parler
    text: "Hello, world!"
"#;
    let script = from_yaml_str(yaml).unwrap();
    assert_eq!(script.sample_rate, 24000);
    assert_eq!(script.segments.len(), 1);
    assert_eq!(script.segments[0].speaker, "mike");
}

#[test]
fn test_from_json_str_valid() {
    let json = r#"{
        "sample_rate": 48000,
        "segments": [{
            "id": "seg1",
            "speaker": "guest",
            "engine": "piper",
            "text": "Hi there!"
        }]
    }"#;
    let script = from_json_str(json).unwrap();
    assert_eq!(script.sample_rate, 48000);
    assert_eq!(script.segments[0].id.0, "seg1");
}

#[test]
fn test_from_yaml_str_invalid() {
    let yaml = "not: valid: yaml: here";
    let result = from_yaml_str(yaml);
    assert!(result.is_err());
}

#[test]
fn test_from_yaml_reader() {
    let yaml = "sample_rate: 24000\nsegments: []";
    let cursor = Cursor::new(yaml);
    let script = from_yaml_reader(cursor).unwrap();
    assert_eq!(script.sample_rate, 24000);
}
