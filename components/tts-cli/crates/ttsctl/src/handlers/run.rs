//! Main CLI entry point.

use crate::cli::Cli;
use crate::config::load_config;
use anyhow::Result;

/// Main entry point for CLI execution.
pub async fn run(cli: Cli) -> Result<()> {
    let config = load_config(cli.config_file.as_deref())?;

    if cli.web_ui {
        return super::web::serve_web_ui(&cli.bind, &config).await;
    }

    if let Some(script_path) = &cli.script_file {
        return super::script::render_script(&config, script_path, cli.output_file.as_deref())
            .await;
    }

    println!("No action specified. Use --help for usage information.");
    Ok(())
}
