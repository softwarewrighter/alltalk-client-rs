//! Command handlers for CLI operations.

use crate::cli::Cli;
use crate::config::load_config;
use anyhow::{bail, Result};
use std::path::Path;

/// Main entry point for CLI execution.
pub async fn run(cli: Cli) -> Result<()> {
    let config = load_config(cli.config_file.as_deref())?;

    if let Some(script_path) = &cli.script_file {
        return render_script(&config, script_path, cli.output_file.as_deref()).await;
    }

    println!("No action specified. Use --help for usage information.");
    Ok(())
}

async fn render_script(
    config: &crate::config::Config,
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
    println!("Using AllTalk at: {}", config.alltalk_url);
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
