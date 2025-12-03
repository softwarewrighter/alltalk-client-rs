//! Error types for script operations.

use thiserror::Error;

/// Errors that can occur during script operations.
#[derive(Debug, Error)]
pub enum ScriptError {
    /// Failed to parse script from YAML or JSON
    #[error("parse error: {0}")]
    Parse(String),

    /// Script validation failed
    #[error("validation error: {0}")]
    Validation(String),

    /// TTS engine communication failed
    #[error("engine error: {0}")]
    Engine(String),

    /// Audio processing failed
    #[error("audio error: {0}")]
    Audio(String),

    /// Network request failed
    #[error("network error: {0}")]
    Network(String),

    /// I/O operation failed
    #[error("io error: {0}")]
    Io(String),
}
