//! Functions for validating script structure.

use std::collections::HashSet;
use tts_spec_model::{Script, ScriptError};

/// Validate a script, checking all validation rules.
pub fn validate(script: &Script) -> Result<(), ScriptError> {
    validate_sample_rate(script)?;
    validate_segment_ids_unique(script)?;
    Ok(())
}

/// Validate that sample rate is positive.
fn validate_sample_rate(script: &Script) -> Result<(), ScriptError> {
    if script.sample_rate == 0 {
        return Err(ScriptError::Validation(
            "sample_rate must be greater than 0".into(),
        ));
    }
    Ok(())
}

/// Validate that all segment IDs are unique.
fn validate_segment_ids_unique(script: &Script) -> Result<(), ScriptError> {
    let mut seen: HashSet<&str> = HashSet::new();
    for seg in &script.segments {
        let id: &str = &seg.id.0;
        if !seen.insert(id) {
            return Err(ScriptError::Validation(format!(
                "duplicate segment id: {id}"
            )));
        }
    }
    Ok(())
}

/// Validate a script for a specific engine (extensible).
pub fn validate_for_engine(script: &Script, _engine: &str) -> Result<(), ScriptError> {
    validate(script)
}
