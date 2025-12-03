//! Functions for parsing scripts from YAML and JSON.

use std::io::Read;
use tts_spec_model::{Script, ScriptError};

/// Parse a script from a YAML string.
pub fn from_yaml_str(src: &str) -> Result<Script, ScriptError> {
    serde_yaml::from_str::<Script>(src).map_err(|e| ScriptError::Parse(e.to_string()))
}

/// Parse a script from a JSON string.
pub fn from_json_str(src: &str) -> Result<Script, ScriptError> {
    serde_json::from_str::<Script>(src).map_err(|e| ScriptError::Parse(e.to_string()))
}

/// Parse a script from a YAML reader.
pub fn from_yaml_reader<R: Read>(mut reader: R) -> Result<Script, ScriptError> {
    let mut buf = String::new();
    reader
        .read_to_string(&mut buf)
        .map_err(|e| ScriptError::Io(e.to_string()))?;
    from_yaml_str(&buf)
}

/// Parse a script from a JSON reader.
pub fn from_json_reader<R: Read>(mut reader: R) -> Result<Script, ScriptError> {
    let mut buf = String::new();
    reader
        .read_to_string(&mut buf)
        .map_err(|e| ScriptError::Io(e.to_string()))?;
    from_json_str(&buf)
}
