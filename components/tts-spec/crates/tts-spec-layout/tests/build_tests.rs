//! Tests for timeline building functions.

use tts_spec_layout::build::build_timeline;
use tts_spec_layout::events::TimelineEventKind;
use tts_spec_layout::query::estimate_silence_ms;
use tts_spec_model::{EngineKind, Script, Segment, SegmentId};

fn make_segment(id: &str, pause_before: u32, pause_after: u32) -> Segment {
    Segment {
        id: SegmentId(id.to_string()),
        speaker: "test".to_string(),
        engine: EngineKind::Parler,
        text: "Test".to_string(),
        emotion: None,
        style_tags: vec![],
        pause_before_ms: pause_before,
        pause_after_ms: pause_after,
        nonverbals: vec![],
    }
}

#[test]
fn test_build_timeline_empty() {
    let script = Script {
        sample_rate: 24000,
        segments: vec![],
    };
    let events = build_timeline(&script);
    assert!(events.is_empty());
}

#[test]
fn test_build_timeline_single_segment_no_pauses() {
    let script = Script {
        sample_rate: 24000,
        segments: vec![make_segment("seg1", 0, 0)],
    };
    let events = build_timeline(&script);
    assert_eq!(events.len(), 1);
    assert!(matches!(
        events[0].kind,
        TimelineEventKind::TtsSegment { .. }
    ));
}

#[test]
fn test_build_timeline_with_pauses() {
    let script = Script {
        sample_rate: 24000,
        segments: vec![make_segment("seg1", 500, 250)],
    };
    let events = build_timeline(&script);
    assert_eq!(events.len(), 3);
    assert!(matches!(
        events[0].kind,
        TimelineEventKind::Silence { duration_ms: 500 }
    ));
    assert!(matches!(
        events[1].kind,
        TimelineEventKind::TtsSegment { .. }
    ));
    assert!(matches!(
        events[2].kind,
        TimelineEventKind::Silence { duration_ms: 250 }
    ));
}

#[test]
fn test_estimate_silence_ms() {
    let script = Script {
        sample_rate: 24000,
        segments: vec![
            make_segment("seg1", 500, 250),
            make_segment("seg2", 100, 100),
        ],
    };
    let events = build_timeline(&script);
    assert_eq!(estimate_silence_ms(&events), 950);
}
