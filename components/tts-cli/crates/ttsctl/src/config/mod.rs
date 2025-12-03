//! Configuration loading and types.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Base URL of AllTalk server
    pub alltalk_url: String,
    /// Default TTS engine
    pub default_engine: String,
    /// Default sample rate in Hz
    pub default_sample_rate: u32,
    /// Speaker voice profiles
    pub speakers: HashMap<String, SpeakerConfig>,
}

/// Speaker voice profile configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerConfig {
    /// TTS engine for this speaker
    pub engine: String,
    /// Voice ID or name
    pub voice_id: String,
    /// Default emotion
    #[serde(default)]
    pub default_emotion: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            alltalk_url: "http://localhost:7851".into(),
            default_engine: "parler".into(),
            default_sample_rate: 24000,
            speakers: HashMap::new(),
        }
    }
}

/// Load configuration from file or use defaults.
pub fn load_config(path: Option<&Path>) -> Result<Config> {
    match path {
        Some(p) => load_from_file(p),
        None => Ok(Config::default()),
    }
}

fn load_from_file(path: &Path) -> Result<Config> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;
    toml::from_str(&contents)
        .with_context(|| format!("failed to parse config file: {}", path.display()))
}
