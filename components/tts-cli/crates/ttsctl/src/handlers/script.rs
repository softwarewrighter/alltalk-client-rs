//! Script rendering handlers.

use crate::config::{Config, all_engines, url_for_engine};
use anyhow::{Result, bail};
use std::path::Path;

/// Render a TTS script to audio output.
pub async fn render_script(
    config: &Config,
    script_path: &Path,
    output: Option<&Path>,
) -> Result<()> {
    let output_path = output.unwrap_or_else(|| Path::new("output.wav"));
    println!("Loading script: {}", script_path.display());
    let script = load_script(script_path)?;
    println!("Validating script...");
    tts_spec_script::validate::validate(&script)?;
    println!(
        "Rendering {} segments to {}",
        script.segments.len(),
        output_path.display()
    );
    let engines = all_engines(config);
    println!("Available engines: {:?}", engines);
    if let Some(url) = url_for_engine(config, &config.default_engine) {
        println!("Default engine '{}' at: {}", config.default_engine, url);
    }
    println!("Rendering not yet implemented - waiting for tts-engine component");
    Ok(())
}

fn load_script(path: &Path) -> Result<tts_spec_model::Script> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let file = std::fs::File::open(path)?;
    match ext {
        "yaml" | "yml" => Ok(tts_spec_script::parse::from_yaml_reader(file)?),
        "json" => Ok(tts_spec_script::parse::from_json_reader(file)?),
        _ => bail!("unsupported file extension: {ext} (use .yaml or .json)"),
    }
}
