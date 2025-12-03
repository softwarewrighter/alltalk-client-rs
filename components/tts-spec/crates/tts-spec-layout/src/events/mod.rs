//! Timeline event types.

use tts_spec_model::{NonVerbalKind, SegmentId};

/// The kind of event on the timeline.
#[derive(Clone, Debug, PartialEq)]
pub enum TimelineEventKind {
    /// Insert silence of the specified duration
    Silence {
        /// Duration in milliseconds
        duration_ms: u32,
    },
    /// Synthesize a TTS segment
    TtsSegment {
        /// Reference to the segment to synthesize
        segment_id: SegmentId,
    },
    /// Insert a non-verbal sound
    NonVerbal {
        /// Type of non-verbal sound
        kind: NonVerbalKind,
        /// Optional asset name for custom sounds
        asset_name: Option<String>,
    },
}

/// An event on the audio timeline.
#[derive(Clone, Debug, PartialEq)]
pub struct TimelineEvent {
    /// Index in the ordered sequence of events
    pub order_index: u32,
    /// Time offset from start in milliseconds (estimated)
    pub at_ms: u64,
    /// The kind of event
    pub kind: TimelineEventKind,
}
