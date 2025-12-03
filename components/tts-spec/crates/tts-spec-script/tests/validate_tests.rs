//! Tests for script validation functions.

use tts_spec_model::{EngineKind, Script, Segment, SegmentId};
use tts_spec_script::validate::validate;

fn make_segment(id: &str) -> Segment {
    Segment {
        id: SegmentId(id.to_string()),
        speaker: "test".to_string(),
        engine: EngineKind::Parler,
        text: "Test text".to_string(),
        emotion: None,
        style_tags: vec![],
        pause_before_ms: 0,
        pause_after_ms: 0,
        nonverbals: vec![],
    }
}

#[test]
fn test_validate_valid_script() {
    let script = Script {
        sample_rate: 24000,
        segments: vec![make_segment("seg1"), make_segment("seg2")],
    };
    assert!(validate(&script).is_ok());
}

#[test]
fn test_validate_zero_sample_rate() {
    let script = Script {
        sample_rate: 0,
        segments: vec![],
    };
    let result = validate(&script);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("sample_rate"));
}

#[test]
fn test_validate_duplicate_ids() {
    let script = Script {
        sample_rate: 24000,
        segments: vec![make_segment("dup"), make_segment("dup")],
    };
    let result = validate(&script);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("duplicate"));
}

#[test]
fn test_validate_empty_segments_ok() {
    let script = Script {
        sample_rate: 24000,
        segments: vec![],
    };
    assert!(validate(&script).is_ok());
}
