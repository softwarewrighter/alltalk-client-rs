//! CLI argument parsing with clap.

use clap::Parser;
use std::path::PathBuf;

const SHORT_HELP: &str = "\
ttsctl - TTS control plane CLI for AllTalk

Synthesize speech from scripts with precise pause and emotion control.

Examples:
    ttsctl -s script.yaml -o output.wav
    ttsctl -w -b 0.0.0.0:5157";

const LONG_HELP: &str = "\
ttsctl - TTS control plane CLI for AllTalk

Synthesize speech from scripts with precise pause and emotion control.
Supports multiple TTS engines (Parler, Piper, XTTS) via AllTalk v2 backend.

EXAMPLES:
    ttsctl -s podcast.yaml -o episode1.wav
    ttsctl -c ./my-config.toml -s script.yaml -o out.wav
    ttsctl -w -b 192.168.1.100:5157

AI CODING AGENT INSTRUCTIONS:
=============================
This CLI is designed for use by both humans and AI coding agents.

CONFIGURATION:
    The config file (config.toml) contains:
    - alltalk_url: Base URL of AllTalk server (e.g., \"http://192.168.1.100:7851\")
    - default_engine: Default TTS engine (parler, piper, xtts)
    - default_sample_rate: Output sample rate in Hz (e.g., 24000)
    - speakers: Map of speaker names to voice profiles

SCRIPT FORMAT:
    Scripts are YAML or JSON files with the following structure:

    sample_rate: 24000
    segments:
      - id: intro
        speaker: mike
        engine: parler
        text: \"Welcome to the show.\"
        emotion: confident
        pause_before_ms: 0
        pause_after_ms: 500

    Supported fields per segment:
    - id: Unique segment identifier (required)
    - speaker: Speaker name from config (required)
    - engine: TTS engine - parler, piper, xtts (required)
    - text: Text to synthesize (required)
    - emotion: Emotion hint for engine (optional)
    - style_tags: List of style modifiers (optional)
    - pause_before_ms: Silence before segment in ms (default: 0)
    - pause_after_ms: Silence after segment in ms (default: 0)
    - nonverbals: List of non-verbal sounds (optional)

EXIT CODES:
    0 - Success
    1 - Error (configuration, network, synthesis failure)

For more information: https://github.com/softwarewrighter/alltalk-client-rs";

/// TTS control plane CLI for AllTalk.
#[derive(Parser, Debug)]
#[command(
    name = "ttsctl",
    version = version_string(),
    about = SHORT_HELP,
    long_about = LONG_HELP,
    help_template = "{about}\n\n{usage-heading} {usage}\n\n{all-args}"
)]
pub struct Cli {
    /// Path to configuration file
    #[arg(short = 'c', long = "config-file", value_name = "PATH")]
    pub config_file: Option<PathBuf>,

    /// Path to script file (YAML or JSON)
    #[arg(short = 's', long = "script-file", value_name = "PATH")]
    pub script_file: Option<PathBuf>,

    /// Path to output WAV file
    #[arg(short = 'o', long = "output-file", value_name = "PATH")]
    pub output_file: Option<PathBuf>,

    /// Start web UI server
    #[arg(short = 'w', long = "web-ui")]
    pub web_ui: bool,

    /// Bind address for web UI (default: 0.0.0.0:5157)
    #[arg(
        short = 'b',
        long = "bind",
        value_name = "ADDR:PORT",
        default_value = "0.0.0.0:5157"
    )]
    pub bind: String,

    /// Backend AllTalk URL for web UI proxy (overrides config)
    #[arg(long = "backend-url", value_name = "URL")]
    pub backend_url: Option<String>,
}

/// Generate version string with build info.
const fn version_string() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION"),
        "\n",
        "Copyright (c) 2025 Software Wrighter LLC\n",
        "License: MIT OR Apache-2.0\n",
        "Repository: https://github.com/softwarewrighter/alltalk-client-rs\n",
        "Build Host: ",
        env!("TARGET"),
        "\n",
        "Build Commit: ",
        env!("GIT_HASH"),
        "\n",
        "Build Time: ",
        env!("BUILD_TIME"),
    )
}
