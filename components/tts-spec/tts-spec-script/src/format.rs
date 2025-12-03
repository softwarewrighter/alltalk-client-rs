//! Functions for serializing scripts to YAML and JSON.

use tts_spec_model::{Script, ScriptError};

/// Serialize a script to YAML format.
pub fn to_yaml(script: &Script) -> Result<String, ScriptError> {
    serde_yaml::to_string(script).map_err(|e| ScriptError::Parse(e.to_string()))
}

/// Serialize a script to pretty-printed JSON format.
pub fn to_json(script: &Script) -> Result<String, ScriptError> {
    serde_json::to_string_pretty(script).map_err(|e| ScriptError::Parse(e.to_string()))
}

/// Serialize a script to minified JSON format.
pub fn to_json_compact(script: &Script) -> Result<String, ScriptError> {
    serde_json::to_string(script).map_err(|e| ScriptError::Parse(e.to_string()))
}
