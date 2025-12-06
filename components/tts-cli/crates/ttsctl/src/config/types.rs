//! Configuration type definitions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// API type for the engine backend.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ApiType {
    /// Gateway backend routing to multiple engines (e.g., AllTalk)
    Gateway,
    /// Standalone backend with single engine
    #[default]
    Standalone,
}

/// Engine configuration with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Display name
    pub name: String,
    /// Backend reference (key into backends map)
    pub backend: String,
    /// API type: gateway or standalone
    #[serde(default)]
    pub api_type: ApiType,
    /// SPDX license identifier
    pub license: String,
    /// Commercial use allowed
    #[serde(default = "default_true")]
    pub commercial: bool,
    /// Supports voice cloning
    #[serde(default)]
    pub cloning: bool,
}

fn default_true() -> bool {
    true
}

/// Backend server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    /// Hostname or IP
    pub host: String,
    /// Port number
    pub port: u16,
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

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Default TTS engine
    pub default_engine: String,
    /// Default sample rate in Hz
    pub default_sample_rate: u32,
    /// Engine definitions with metadata
    #[serde(default)]
    pub engines: HashMap<String, EngineConfig>,
    /// Backend servers
    #[serde(default)]
    pub backends: HashMap<String, BackendConfig>,
    /// Speaker voice profiles
    #[serde(default)]
    pub speakers: HashMap<String, SpeakerConfig>,
}

/// Engine info for API responses (matches Web UI EngineInfo).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineInfoResponse {
    pub id: String,
    pub name: String,
    pub available: bool,
    pub license: String,
    pub cloning: bool,
    pub commercial: bool,
}

/// Response for /api/engines endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnginesResponse {
    pub engines: Vec<EngineInfoResponse>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_engine: "parler".into(),
            default_sample_rate: 24000,
            engines: default_engines(),
            backends: default_backends(),
            speakers: HashMap::new(),
        }
    }
}

fn default_engines() -> HashMap<String, EngineConfig> {
    let mut engines = HashMap::new();
    engines.insert(
        "parler".into(),
        EngineConfig {
            name: "Parler".into(),
            backend: "alltalk".into(),
            api_type: ApiType::Gateway,
            license: "Apache-2.0".into(),
            commercial: true,
            cloning: false,
        },
    );
    engines.insert(
        "piper".into(),
        EngineConfig {
            name: "Piper".into(),
            backend: "alltalk".into(),
            api_type: ApiType::Gateway,
            license: "MIT".into(),
            commercial: true,
            cloning: false,
        },
    );
    engines.insert(
        "xtts".into(),
        EngineConfig {
            name: "XTTS".into(),
            backend: "alltalk".into(),
            api_type: ApiType::Gateway,
            license: "CPML".into(),
            commercial: false,
            cloning: true,
        },
    );
    engines.insert(
        "dia".into(),
        EngineConfig {
            name: "Dia".into(),
            backend: "dia".into(),
            api_type: ApiType::Standalone,
            license: "Apache-2.0".into(),
            commercial: true,
            cloning: true,
        },
    );
    engines.insert(
        "gptsovits".into(),
        EngineConfig {
            name: "GPT-SoVITS".into(),
            backend: "gptsovits".into(),
            api_type: ApiType::Standalone,
            license: "MIT".into(),
            commercial: true,
            cloning: true,
        },
    );
    engines.insert(
        "toucan".into(),
        EngineConfig {
            name: "Toucan".into(),
            backend: "toucan".into(),
            api_type: ApiType::Standalone,
            license: "Apache-2.0".into(),
            commercial: true,
            cloning: true,
        },
    );
    engines
}

fn default_backends() -> HashMap<String, BackendConfig> {
    let mut backends = HashMap::new();
    backends.insert(
        "alltalk".into(),
        BackendConfig {
            host: "localhost".into(),
            port: 5157,
        },
    );
    backends.insert(
        "dia".into(),
        BackendConfig {
            host: "localhost".into(),
            port: 1110,
        },
    );
    backends.insert(
        "gptsovits".into(),
        BackendConfig {
            host: "localhost".into(),
            port: 6910,
        },
    );
    backends.insert(
        "toucan".into(),
        BackendConfig {
            host: "localhost".into(),
            port: 1721,
        },
    );
    backends
}
