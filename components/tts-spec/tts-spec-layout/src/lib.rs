//! Timeline layout for TTS script segments.
//!
//! This crate converts a [`Script`] into a sequence of [`TimelineEvent`]s that
//! represent the ordered operations needed to produce the final audio: silence
//! insertion, TTS synthesis, and non-verbal sound placement.
//!
//! # Example
//!
//! ```
//! use tts_spec_model::{Script, Segment, SegmentId, EngineKind};
//! use tts_spec_layout::{build, events::TimelineEventKind};
//!
//! let script = Script {
//!     sample_rate: 24000,
//!     segments: vec![Segment {
//!         id: SegmentId("intro".to_string()),
//!         speaker: "mike".to_string(),
//!         engine: EngineKind::Parler,
//!         text: "Hello".to_string(),
//!         emotion: None,
//!         style_tags: vec![],
//!         pause_before_ms: 500,
//!         pause_after_ms: 250,
//!         nonverbals: vec![],
//!     }],
//! };
//!
//! let events = build::build_timeline(&script);
//! assert!(!events.is_empty());
//! ```

pub mod build;
pub mod events;
pub mod query;
