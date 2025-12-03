//! Core data structures for TTS scripts.

use serde::{Deserialize, Serialize};

/// Unique identifier for a segment within a script.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SegmentId(pub String);

/// Supported TTS engine types.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EngineKind {
    /// Parler TTS - description-based synthesis (Apache-2.0)
    Parler,
    /// Piper TTS - fast neural synthesis (GPL-3.0)
    Piper,
    /// XTTS - voice cloning via Coqui
    Xtts,
    /// Other engine identified by name
    Other(String),
}

/// Types of non-verbal sounds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NonVerbalKind {
    /// Breathing sound
    Breath,
    /// Laughter
    Laugh,
    /// Sigh
    Sigh,
    /// Custom sound effect by asset name
    Sfx(String),
}

/// A non-verbal sound to insert in the audio.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonVerbal {
    /// Type of non-verbal sound
    pub kind: NonVerbalKind,
    /// Offset from segment start in milliseconds
    #[serde(default)]
    pub offset_ms: u32,
}

/// A single segment of speech within a script.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Segment {
    /// Unique identifier for this segment
    pub id: SegmentId,
    /// Speaker name (maps to voice profile)
    pub speaker: String,
    /// TTS engine to use for synthesis
    pub engine: EngineKind,
    /// Text to synthesize
    pub text: String,
    /// Optional emotion hint (engine-specific interpretation)
    #[serde(default)]
    pub emotion: Option<String>,
    /// Style modifiers (e.g., "fast", "whisper")
    #[serde(default)]
    pub style_tags: Vec<String>,
    /// Silence to insert before this segment in milliseconds
    #[serde(default)]
    pub pause_before_ms: u32,
    /// Silence to insert after this segment in milliseconds
    #[serde(default)]
    pub pause_after_ms: u32,
    /// Non-verbal sounds to insert
    #[serde(default)]
    pub nonverbals: Vec<NonVerbal>,
}

/// A complete TTS script containing multiple segments.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Script {
    /// Target sample rate for output audio in Hz (default: 24000)
    #[serde(default = "default_sample_rate")]
    pub sample_rate: u32,
    /// Ordered list of segments to synthesize
    #[serde(default)]
    pub segments: Vec<Segment>,
}

fn default_sample_rate() -> u32 {
    24000
}
