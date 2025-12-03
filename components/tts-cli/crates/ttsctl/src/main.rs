//! ttsctl - CLI for controlling TTS synthesis via AllTalk

use anyhow::Result;
use clap::Parser;

mod cli;
mod config;
mod handlers;

use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    handlers::run(cli).await
}
