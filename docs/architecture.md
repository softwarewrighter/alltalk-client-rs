# Architecture

## Overview

alltalk-client-rs is a Rust-based TTS (Text-to-Speech) control plane that orchestrates speech synthesis via an AllTalk v2 backend server. The system follows a client-server architecture where the Rust CLI acts as a "director/sequencer" for generating audio from structured scripts.

## System Context

```
+-------------------+          HTTP/REST          +-------------------+
|   Rust Client     | <-------------------------> |   AllTalk Hub     |
|   (M3 MacBook)    |                             |   (Arch + RTX)    |
|                   |                             |                   |
| - CLI (ttsctl)    |                             | - Parler TTS      |
| - Optional Yew UI |                             | - Piper TTS       |
| - Script Parser   |                             | - XTTS/Coqui      |
| - Audio Pipeline  |                             | - RVC/Voice Clone |
+-------------------+                             +-------------------+
```

## Component Architecture

The project follows a strict multi-component workspace structure with dependency leveling:

```
tts-spec (lowest level - no deps)
    |
    v
tts-engine (depends on tts-spec)
    |
    v
tts-render (depends on tts-spec, tts-engine)
    |
    v
tts-apps (depends on all above)
```

### Component 1: tts-spec (Data + Script DSL)

Location: `components/tts-spec/`

Crates:
- `tts-spec-model` - Core data types (Segment, Script, EngineKind, NonVerbal)
- `tts-spec-script` - Script DSL parsing/validation (YAML/JSON)
- `tts-spec-layout` - Logical timeline layout (events without audio)

Purpose: Define the domain model and script format. Zero external dependencies except serde.

### Component 2: tts-engine (AllTalk Client)

Location: `components/tts-engine/`

Crates:
- `tts-engine-api` - HTTP types + traits (TtsEngine, TtsHub)
- `tts-engine-alltalk` - Concrete AllTalk HTTP client implementation

Purpose: Abstract TTS engine communication. Maps Segments to HTTP requests.

### Component 3: tts-render (Audio Pipeline)

Location: `components/tts-render/`

Crates:
- `tts-render-core` - Pure audio operations (silence, concat, WAV I/O)
- `tts-render-pipeline` - Orchestrates engine calls + audio assembly

Purpose: Transform script timelines into final WAV files with precise pause control.

### Component 4: tts-apps (User-Facing Applications)

Location: `components/tts-apps/`

Crates:
- `ttsctl` - CLI binary
- `tts-orchestrator-server` - HTTP server for UI
- `tts-orchestrator-ui` - Yew/WASM SPA (optional)

Purpose: User interaction layer. Thin glue over lower components.

## Code Constraints

The architecture enforces strict limits to maintain small, testable units:

| Constraint | Limit |
|------------|-------|
| Functions per module | <= 4 |
| Modules per crate | < 5 |
| Crates per component | < 5 |
| Lines per function | < 50 (prefer 10-30) |
| Lines per file | < 500 (prefer 200-300) |

Additional rules:
- `lib.rs` and `mod.rs` contain no functions (only re-exports)
- Pure functions separated from state-changing functions
- Minimal impl methods; prefer passing structs to pure functions
- Tests in separate files where possible

## Data Flow

```
Script (YAML/JSON)
       |
       v
[tts-spec-script] Parse + Validate
       |
       v
Script struct (segments with speakers, engines, pauses, emotions)
       |
       v
[tts-spec-layout] Build Timeline
       |
       v
TimelineEvents (ordered: Silence, TtsSegment, NonVerbal)
       |
       v
[tts-engine-alltalk] HTTP calls to AllTalk
       |
       v
WAV bytes per segment
       |
       v
[tts-render-core] Insert silence, concat buffers
       |
       v
Final WAV file(s)
```

## Pause and Non-Verbal Control

Three-layer approach for precise audio control:

1. **Text-level markup** - Custom tags stripped from TTS text:
   - `[pause:500]` - Explicit silence in ms
   - `[breath:soft]` - Non-verbal sound
   - `[emotion:excited]` - Prompt hint for Parler

2. **Timeline-level silence** - Deterministic pauses:
   - `pause_before_ms` and `pause_after_ms` per segment
   - Generated as zero-sample arrays at target sample rate

3. **Non-verbal assets** - Pre-recorded sound library:
   - `mike_breath_soft_01.wav`
   - `guest_laugh_short_01.wav`
   - Inserted at specific timeline positions

## Engine Abstraction

The `TtsEngine` trait abstracts different TTS backends:

```rust
#[async_trait]
pub trait TtsEngine {
    async fn synthesize_segment(&self, segment: &Segment) -> Result<Vec<u8>, ScriptError>;
    async fn list_voices(&self) -> Result<Vec<String>, ScriptError>;
}
```

Engine-specific adapters handle:
- Parler: Description prompts for emotion/style
- Piper: Punctuation-driven prosody
- XTTS: Voice cloning via reference audio

## Deployment Topology

**GPU Server (Arch Linux + RTX 3060/5060):**
- AllTalk v2 with Multi-Engine Manager
- Parler, Piper, XTTS engines
- DeepSpeed for performance
- Exposes HTTP API on local network

**Client (M3 MacBook Pro):**
- ttsctl CLI for batch rendering
- Optional local web UI for script editing
- Connects to GPU server via HTTP

## Security Considerations

- AllTalk runs on private network (not exposed to internet)
- API keys managed via environment variables
- No credentials stored in repository
- License compliance: AllTalk (AGPL), Parler (Apache-2.0), Piper (GPL-3.0)

## Future Extensions

- CI/CD pipeline via GitHub Actions
- Multi-track export for DAW import
- Subtitle/SRT generation
- Voice cloning workflow integration
- Real-time streaming preview
