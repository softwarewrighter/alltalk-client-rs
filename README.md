# alltalk-client-rs

A Rust CLI and web UI for controlling TTS (Text-to-Speech) synthesis via an AllTalk v2 backend.

## Overview

alltalk-client-rs provides precise control over text-to-speech synthesis with support for:

- Multiple TTS engines (Parler, Piper, XTTS) via AllTalk v2
- Deterministic pause and silence control
- Structured scripts with per-segment configuration
- Multi-speaker podcast generation
- Web UI for interactive TTS generation

## Architecture

```
+-------------------+     +-------------------+     +-------------------+
|   ttsctl CLI      | --> |   AllTalk v2      | --> |   TTS Engines     |
|   (Rust/Axum)     |     |   (Gateway)       |     |   Parler/Piper    |
+-------------------+     +-------------------+     +-------------------+
        |
        v
+-------------------+
|   Web UI (WASM)   |
|   (Yew/Trunk)     |
+-------------------+
```

The web UI runs in the browser (WASM) and communicates with the local ttsctl server,
which proxies requests to the AllTalk backend.

## Components

| Component | Description |
|-----------|-------------|
| tts-spec | Core data types, script parsing, and timeline layout |
| tts-cli | CLI tool (`ttsctl`) with web UI server and API proxy |
| tts-web | Yew-based web UI for interactive TTS generation |

## Quick Start

### Prerequisites

- Rust 1.82+ (2024 edition)
- Trunk (for WASM builds): `cargo install trunk`
- AllTalk v2 backend running (default: http://localhost:7851)

### Build

```bash
# Build all components
./scripts/build-all.sh

# Or build individually
cd components/tts-cli && cargo build --release
cd components/tts-web/crates/tts-web-ui && trunk build --release
```

### Run Web UI

```bash
# With default backend (localhost:7851)
ttsctl -w

# With custom backend URL
ttsctl -w --backend-url http://your-backend:7851

# With config file
ttsctl -c config.toml -w

# Custom bind address
ttsctl -w -b 0.0.0.0:8080
```

Open http://localhost:5157 in your browser.

### Render Script (CLI)

```bash
ttsctl -s script.yaml -o output.wav
```

## Configuration

Create a `config.toml` file:

```toml
# Backend AllTalk server URL
alltalk_url = "http://localhost:7851"

# Default TTS engine (parler, piper, xtts)
default_engine = "parler"

# Default sample rate in Hz
default_sample_rate = 24000

# Speaker profiles (optional)
[speakers.mike]
engine = "parler"
voice_id = "Jon"
default_emotion = "confident"
```

## Script Format

Scripts are YAML or JSON files defining TTS segments:

```yaml
sample_rate: 24000
segments:
  - id: intro
    speaker: mike
    engine: parler
    text: "Welcome to the show."
    emotion: confident
    pause_before_ms: 0
    pause_after_ms: 500

  - id: question
    speaker: guest
    engine: piper
    text: "Thanks for having me."
    pause_after_ms: 300
```

## Documentation

| Document | Description |
|----------|-------------|
| [docs/prd.md](docs/prd.md) | Product Requirements Document |
| [docs/architecture.md](docs/architecture.md) | System architecture and design |
| [docs/design.md](docs/design.md) | Detailed design specifications |
| [docs/plan.md](docs/plan.md) | Implementation plan and phases |
| [docs/status.md](docs/status.md) | Current project status |
| [docs/ai_agent_instructions.md](docs/ai_agent_instructions.md) | Instructions for AI coding agents |
| [docs/tools.md](docs/tools.md) | Development tools and setup |
| [docs/process.md](docs/process.md) | Development process guidelines |

## Supported TTS Engines

### Parler TTS

Neural TTS with natural language voice control:

- 16 named speakers (Jon, Lea, Gary, Jenna, etc.)
- Natural language descriptions for voice characteristics
- Temperature and sampling controls

### Piper TTS

Fast, local TTS with ONNX models:

- Multiple voice models available
- Speed, noise, and silence controls
- Multi-speaker model support

### XTTS (Coming Soon)

Voice cloning with reference audio support.

## Development

### Project Structure

```
alltalk-client-rs/
+-- components/
|   +-- tts-spec/        # Core data types and parsing
|   |   +-- crates/
|   |       +-- tts-spec-model/   # Data types
|   |       +-- tts-spec-script/  # Script parsing
|   |       +-- tts-spec-layout/  # Timeline layout
|   +-- tts-cli/         # CLI application
|   |   +-- crates/
|   |       +-- ttsctl/           # Main CLI binary
|   +-- tts-web/         # Web UI
|       +-- crates/
|           +-- tts-web-ui/       # Yew WASM frontend
|           +-- tts-web-server/   # Static file server
+-- docs/                # Documentation
+-- scripts/             # Build and utility scripts
```

### Quality Gates

All code must pass:

```bash
cargo fmt -- --check    # Formatting
cargo clippy            # Linting
cargo test              # Tests
sw-checklist            # Project standards
```

## License

MIT OR Apache-2.0

## Copyright

Copyright (c) 2025 Michael A. Wright / Software Wrighter LLC
