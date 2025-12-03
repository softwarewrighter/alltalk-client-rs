//! Core data types for TTS script specification.
//!
//! This crate provides the fundamental types used throughout the alltalk-client-rs
//! system. It has minimal dependencies (only serde and thiserror) and serves as
//! the foundation layer that other crates depend on.
//!
//! # Types
//!
//! - [`Script`] - A complete TTS script with segments
//! - [`Segment`] - A single unit of speech with speaker, text, and timing
//! - [`EngineKind`] - Supported TTS engines (Parler, Piper, Xtts)
//! - [`NonVerbal`] - Non-verbal sounds (breaths, laughs)
//! - [`ScriptError`] - Error types for script operations

mod errors;
mod types;

pub use errors::ScriptError;
pub use types::{EngineKind, NonVerbal, NonVerbalKind, Script, Segment, SegmentId};
