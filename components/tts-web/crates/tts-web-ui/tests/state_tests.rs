//! State management tests.

use tts_spec_model::{EngineKind, Segment, SegmentId};

/// Application state for testing (mirrors src/state.rs).
#[derive(Debug, Clone, Default)]
struct AppState {
    segments: Vec<Segment>,
    selected_segment: Option<SegmentId>,
}

impl AppState {
    fn add_segment(&mut self, segment: Segment) {
        self.segments.push(segment);
    }

    fn remove_segment(&mut self, id: &SegmentId) {
        self.segments.retain(|s| &s.id != id);
        if self.selected_segment.as_ref() == Some(id) {
            self.selected_segment = None;
        }
    }
}

fn test_segment(id: &str) -> Segment {
    Segment {
        id: SegmentId(id.to_string()),
        speaker: "test".into(),
        engine: EngineKind::Parler,
        text: "Test text".into(),
        emotion: None,
        style_tags: vec![],
        pause_before_ms: 0,
        pause_after_ms: 0,
        nonverbals: vec![],
    }
}

#[test]
fn test_add_segment() {
    let mut state = AppState::default();
    assert!(state.segments.is_empty());

    state.add_segment(test_segment("seg1"));
    assert_eq!(state.segments.len(), 1);
    assert_eq!(state.segments[0].id.0, "seg1");
}

#[test]
fn test_remove_segment() {
    let mut state = AppState::default();
    state.add_segment(test_segment("seg1"));
    state.add_segment(test_segment("seg2"));
    assert_eq!(state.segments.len(), 2);

    state.remove_segment(&SegmentId("seg1".into()));
    assert_eq!(state.segments.len(), 1);
    assert_eq!(state.segments[0].id.0, "seg2");
}

#[test]
fn test_remove_clears_selection() {
    let mut state = AppState::default();
    state.add_segment(test_segment("seg1"));
    state.selected_segment = Some(SegmentId("seg1".into()));

    state.remove_segment(&SegmentId("seg1".into()));
    assert!(state.selected_segment.is_none());
}
