//! Configuration loading and types.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Backend server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    /// Hostname or IP
    pub host: String,
    /// Port number
    pub port: u16,
    /// Engines provided by this backend
    #[serde(default)]
    pub engines: Vec<String>,
}

impl BackendConfig {
    /// Get the full URL for this backend
    pub fn url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Default TTS engine
    pub default_engine: String,
    /// Default sample rate in Hz
    pub default_sample_rate: u32,
    /// Backend servers (alltalk, dia, gptsovits, toucan)
    #[serde(default)]
    pub backends: HashMap<String, BackendConfig>,
    /// Speaker voice profiles
    #[serde(default)]
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
        let mut backends = HashMap::new();
        backends.insert(
            "alltalk".into(),
            BackendConfig {
                host: "localhost".into(),
                port: 5157,
                engines: vec!["parler".into(), "piper".into(), "xtts".into()],
            },
        );
        backends.insert(
            "dia".into(),
            BackendConfig {
                host: "localhost".into(),
                port: 1110,
                engines: vec!["dia".into()],
            },
        );
        backends.insert(
            "gptsovits".into(),
            BackendConfig {
                host: "localhost".into(),
                port: 6910,
                engines: vec!["gptsovits".into()],
            },
        );
        backends.insert(
            "toucan".into(),
            BackendConfig {
                host: "localhost".into(),
                port: 1721,
                engines: vec!["toucan".into()],
            },
        );

        Self {
            default_engine: "parler".into(),
            default_sample_rate: 24000,
            backends,
            speakers: HashMap::new(),
        }
    }
}

impl Config {
    /// Find which backend provides a given engine
    pub fn backend_for_engine(&self, engine: &str) -> Option<&BackendConfig> {
        self.backends
            .values()
            .find(|b| b.engines.iter().any(|e| e == engine))
    }

    /// Get URL for a specific engine
    pub fn url_for_engine(&self, engine: &str) -> Option<String> {
        self.backend_for_engine(engine).map(|b| b.url())
    }

    /// List all available engines across all backends
    pub fn all_engines(&self) -> Vec<&str> {
        self.backends
            .values()
            .flat_map(|b| b.engines.iter().map(|s| s.as_str()))
            .collect()
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
