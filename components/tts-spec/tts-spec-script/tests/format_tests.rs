//! Tests for script formatting functions.

use tts_spec_model::{EngineKind, Script, Segment, SegmentId};
use tts_spec_script::format::{to_json, to_json_compact, to_yaml};

fn make_script() -> Script {
    Script {
        sample_rate: 24000,
        segments: vec![Segment {
            id: SegmentId("test".to_string()),
            speaker: "mike".to_string(),
            engine: EngineKind::Parler,
            text: "Hello".to_string(),
            emotion: Some("calm".to_string()),
            style_tags: vec!["slow".to_string()],
            pause_before_ms: 100,
            pause_after_ms: 200,
            nonverbals: vec![],
        }],
    }
}

#[test]
fn test_to_yaml() {
    let script = make_script();
    let yaml = to_yaml(&script).unwrap();
    assert!(yaml.contains("sample_rate: 24000"));
    assert!(yaml.contains("speaker: mike"));
}

#[test]
fn test_to_json() {
    let script = make_script();
    let json = to_json(&script).unwrap();
    assert!(json.contains("\"sample_rate\": 24000"));
    assert!(json.contains("\"speaker\": \"mike\""));
}

#[test]
fn test_to_json_compact() {
    let script = make_script();
    let json = to_json_compact(&script).unwrap();
    assert!(!json.contains('\n'));
    assert!(json.contains("\"sample_rate\":24000"));
}
