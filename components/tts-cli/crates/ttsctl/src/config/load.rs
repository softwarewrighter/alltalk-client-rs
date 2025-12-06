//! Configuration loading functions.

use super::types::Config;
use anyhow::{Context, Result};
use std::path::Path;

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
