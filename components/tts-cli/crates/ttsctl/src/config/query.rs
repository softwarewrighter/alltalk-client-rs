//! Configuration query functions.

use super::types::{BackendConfig, Config, EngineInfoResponse, EnginesResponse};

/// Get the full URL for a backend.
pub fn backend_url(backend: &BackendConfig) -> String {
    format!("http://{}:{}", backend.host, backend.port)
}

/// Get URL for a specific engine.
pub fn url_for_engine(config: &Config, engine_id: &str) -> Option<String> {
    config
        .engines
        .get(engine_id)
        .and_then(|e| config.backends.get(&e.backend))
        .map(backend_url)
}

/// List all engine IDs.
pub fn all_engines(config: &Config) -> Vec<&str> {
    config.engines.keys().map(|s| s.as_str()).collect()
}

/// Convert Config to API response format.
pub fn to_engines_response(config: &Config) -> EnginesResponse {
    let mut engines: Vec<EngineInfoResponse> = config
        .engines
        .iter()
        .map(|(id, e)| {
            let available = config.backends.contains_key(&e.backend);
            EngineInfoResponse {
                id: id.clone(),
                name: e.name.clone(),
                available,
                license: e.license.clone(),
                cloning: e.cloning,
                commercial: e.commercial,
            }
        })
        .collect();
    engines.sort_by(|a, b| a.name.cmp(&b.name));
    EnginesResponse { engines }
}
