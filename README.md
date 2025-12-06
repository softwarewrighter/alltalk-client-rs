# alltalk-client-rs

A Rust CLI and web UI for controlling TTS (Text-to-Speech) synthesis via an AllTalk v2 backend.

## Overview

alltalk-client-rs provides precise control over text-to-speech synthesis with support for:

- 6 TTS engines: Parler, Piper, XTTS, Dia, GPT-SoVITS, Toucan
- Deterministic pause and silence control
- Structured scripts with per-segment configuration
- Multi-speaker podcast generation
- Web UI for interactive TTS generation

![TTS Web UI Generate Screen](images/screenshot.png?ts=1733341289000)

## Architecture

```
                                            GPU Server (Docker)
                                    ┌─────────────────────────────────────┐
                                    │  AllTalk Gateway (:5157)            │
                                    │  ├── Parler  (GPU, :8001)           │
+-------------------+               │  ├── Piper   (CPU, :8002)           │
|   ttsctl CLI      | ──────────────│  └── XTTS    (GPU, :8003)           │
|   + Web UI Server |               ├─────────────────────────────────────┤
|   (Rust/Axum)     |               │  Standalone Backends                │
+-------------------+               │  ├── Dia        (:1110) Apache 2.0  │
        │                           │  ├── GPT-SoVITS (:6910) MIT         │
        ▼                           │  └── Toucan     (:1721)             │
+-------------------+               └─────────────────────────────────────┘
|   Web UI (WASM)   |
|   (Yew/Trunk)     |
+-------------------+
```

The ttsctl server proxies API requests to TTS backends running on a GPU server:

| Backend | Port | Engines | License | Use Case |
|---------|------|---------|---------|----------|
| AllTalk | 5157 | Parler, Piper, XTTS | Mixed | Primary gateway (3 engines) |
| Dia | 1110 | Dia | Apache 2.0 | Dialogue-focused, commercial OK |
| GPT-SoVITS | 6910 | GPT-SoVITS | MIT | Voice cloning, commercial OK |
| Toucan | 1721 | Toucan | - | Multilingual synthesis |

**Note:** XTTS uses CPML license (non-commercial only). For commercial use, prefer Dia or GPT-SoVITS.

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
- AllTalk backend running (see [Backend Setup](docs/backend-setup.org))

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
# With default backend (localhost:5157)
ttsctl -w

# With custom backend URL
ttsctl -w --backend-url http://gpu-server:5157

# With config file
ttsctl -c config.toml -w

# Custom bind address
ttsctl -w -b 0.0.0.0:8080
```

Open http://localhost:8080 in your browser (or the address specified with `-b`).

### Render Script (CLI)

```bash
ttsctl -s script.yaml -o output.wav
```

## Configuration

Create a `config.toml` file:

```toml
# Default TTS engine
default_engine = "parler"

# Default sample rate in Hz
default_sample_rate = 24000

# Backend configurations
[backends.alltalk]
url = "http://gpu-server:5157"

[backends.dia]
url = "http://gpu-server:1110"

[backends.gptsovits]
url = "http://gpu-server:6910"

# Speaker profiles (optional)
[speakers.mike]
engine = "parler"
voice_id = "Jon"
default_emotion = "confident"

[speakers.narrator]
engine = "piper"
length_scale = 0.9
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

### Backend Deployment

| Document | Description |
|----------|-------------|
| [docs/scripts/README.md](docs/scripts/README.md) | Quick start for Docker TTS backends |
| [docs/backend-setup.org](docs/backend-setup.org) | Detailed backend setup (Arch Linux + GPU) |

### Project Documentation

| Document | Description |
|----------|-------------|
| [docs/prd.md](docs/prd.md) | Product Requirements Document |
| [docs/architecture.md](docs/architecture.md) | System architecture and design |
| [docs/design.md](docs/design.md) | Detailed design specifications |
| [docs/plan.md](docs/plan.md) | Implementation plan and phases |
| [docs/status.md](docs/status.md) | Current project status |

### Development Guides

| Document | Description |
|----------|-------------|
| [docs/ai_agent_instructions.md](docs/ai_agent_instructions.md) | Instructions for AI coding agents |
| [docs/tools.md](docs/tools.md) | Development tools and setup |
| [docs/process.md](docs/process.md) | Development process guidelines |

## Supported TTS Engines

### Parler TTS

Neural TTS with natural language voice control:

- 16 named speakers (Jon, Lea, Gary, Jenna, etc.)
- Natural language descriptions for voice characteristics
- Temperature and sampling controls
- **License:** Apache 2.0 (commercial use allowed)

### Piper TTS

Fast, local TTS with ONNX models:

- Multiple voice models available
- Speed, noise, and silence controls
- Multi-speaker model support
- **License:** MIT (commercial use allowed)

### XTTS

Voice cloning with reference audio:

- Clone any voice from a short audio sample
- High-quality neural synthesis (GPU required)
- **License:** CPML (non-commercial use only)

### Dia

Dialogue-focused TTS:

- Optimized for conversational speech
- **License:** Apache 2.0 (commercial use allowed)

### GPT-SoVITS

Advanced voice cloning:

- Few-shot voice cloning from minimal samples
- High fidelity reproduction
- **License:** MIT (commercial use allowed)

### Toucan

Multilingual neural TTS:

- Support for 7000+ languages and dialects
- IPA-based phoneme input for precise pronunciation
- Lightweight and fast inference
- **License:** Apache 2.0 (commercial use allowed)

## Development

### Project Structure

```
alltalk-client-rs/
├── components/
│   ├── tts-spec/           # Core data types and parsing
│   │   └── crates/
│   │       ├── tts-spec-model/    # Data types
│   │       ├── tts-spec-script/   # Script parsing
│   │       └── tts-spec-layout/   # Timeline layout
│   ├── tts-cli/            # CLI application
│   │   └── crates/
│   │       └── ttsctl/            # Main CLI binary
│   └── tts-web/            # Web UI
│       └── crates/
│           └── tts-web-ui/        # Yew WASM frontend
├── docs/
│   └── scripts/            # Docker backend deployment scripts
│       ├── at-*.sh         # AllTalk stack (gateway + 3 engines)
│       ├── dia-*.sh        # Dia standalone
│       ├── gpt-*.sh        # GPT-SoVITS standalone
│       └── tou-*.sh        # Toucan standalone
└── scripts/                # Build and utility scripts
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

MIT

## Copyright

Copyright (c) 2025 Michael A. Wright / Software Wrighter LLC
