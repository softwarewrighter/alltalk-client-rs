//! Application state management.

use serde::{Deserialize, Serialize};
use tts_spec_model::{EngineKind, Script, Segment, SegmentId};

/// Application state for the TTS web UI.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppState {
    /// Current script being edited
    pub script: Script,
    /// Currently selected segment ID
    pub selected_segment: Option<SegmentId>,
    /// API base URL for AllTalk
    pub api_url: String,
    /// Status message to display
    pub status: StatusMessage,
}

/// Status message with severity level.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StatusMessage {
    pub text: String,
    pub level: StatusLevel,
}

/// Severity level for status messages.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum StatusLevel {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl AppState {
    /// Create new state with default API URL.
    pub fn new() -> Self {
        Self {
            api_url: "http://localhost:7851".into(),
            ..Default::default()
        }
    }

    /// Add a new segment to the script.
    pub fn add_segment(&mut self, segment: Segment) {
        self.script.segments.push(segment);
    }

    /// Remove segment by ID.
    #[allow(dead_code)]
    pub fn remove_segment(&mut self, id: &SegmentId) {
        self.script.segments.retain(|s| &s.id != id);
        if self.selected_segment.as_ref() == Some(id) {
            self.selected_segment = None;
        }
    }
}

/// Create a default segment with given ID.
pub fn default_segment(id: &str) -> Segment {
    Segment {
        id: SegmentId(id.to_string()),
        speaker: "default".into(),
        engine: EngineKind::Parler,
        text: String::new(),
        emotion: None,
        style_tags: vec![],
        pause_before_ms: 0,
        pause_after_ms: 0,
        nonverbals: vec![],
    }
}
