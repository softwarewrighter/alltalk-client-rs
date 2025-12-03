# Design Document

## Design Philosophy

This project follows strict constraints to ensure maintainability:

1. **Small units** - Functions, modules, and crates are deliberately limited in size
2. **Pure functions** - Separate read-only functions from state-changing ones
3. **Data-first** - Core data types have zero dependencies
4. **Minimal impl** - Prefer free functions over struct methods
5. **Explicit layering** - Clear dependency hierarchy between components

## Directory Structure

```
alltalk-client-rs/
+-- Cargo.toml              # Workspace root
+-- docs/
|   +-- ai_agent_instructions.md
|   +-- architecture.md
|   +-- design.md           # This file
|   +-- plan.md
|   +-- prd.md
|   +-- process.md
|   +-- status.md
|   +-- tools.md
+-- components/
    +-- tts-spec/           # Component 1: Data + DSL
    |   +-- Cargo.toml      # Workspace for tts-spec crates
    |   +-- tts-spec-model/
    |   |   +-- Cargo.toml
    |   |   +-- src/
    |   |       +-- lib.rs      # Re-exports only
    |   |       +-- types.rs    # Core data structs
    |   |       +-- errors.rs   # Error types
    |   |       +-- macros.rs   # Helper macros
    |   +-- tts-spec-script/
    |   |   +-- Cargo.toml
    |   |   +-- src/
    |   |       +-- lib.rs
    |   |       +-- parse.rs    # YAML/JSON parsing
    |   |       +-- validate.rs # Script validation
    |   |       +-- format.rs   # Serialization
    |   +-- tts-spec-layout/
    |       +-- Cargo.toml
    |       +-- src/
    |           +-- lib.rs
    |           +-- events.rs   # Timeline event types
    |           +-- build.rs    # Timeline construction
    +-- tts-engine/          # Component 2: Engine abstraction
    |   +-- Cargo.toml
    |   +-- tts-engine-api/
    |   |   +-- Cargo.toml
    |   |   +-- src/
    |   |       +-- lib.rs
    |   |       +-- traits.rs   # TtsEngine, TtsHub traits
    |   |       +-- requests.rs # Request types
    |   |       +-- responses.rs
    |   +-- tts-engine-alltalk/
    |       +-- Cargo.toml
    |       +-- src/
    |           +-- lib.rs
    |           +-- client.rs   # AllTalkClient struct
    |           +-- http_ops.rs # Async HTTP functions
    |           +-- mapping.rs  # Segment -> Request mapping
    +-- tts-render/          # Component 3: Audio pipeline
    |   +-- Cargo.toml
    |   +-- tts-render-core/
    |   |   +-- Cargo.toml
    |   |   +-- src/
    |   |       +-- lib.rs
    |   |       +-- silence.rs  # Silence generation
    |   |       +-- concat.rs   # Buffer concatenation
    |   |       +-- wav_io.rs   # WAV read/write
    |   +-- tts-render-pipeline/
    |       +-- Cargo.toml
    |       +-- src/
    |           +-- lib.rs
    |           +-- state.rs    # PipelineState
    |           +-- build.rs    # State construction
    |           +-- exec.rs     # Async execution
    +-- tts-apps/            # Component 4: User applications
        +-- Cargo.toml
        +-- ttsctl/
        |   +-- Cargo.toml
        |   +-- src/
        |       +-- main.rs     # Entry point only
        |       +-- cmd.rs      # CLI arg parsing
        |       +-- handlers.rs # Command handlers
        |       +-- config.rs   # Configuration
        +-- tts-orchestrator-server/
        |   +-- Cargo.toml
        |   +-- src/
        |       +-- lib.rs
        |       +-- routes.rs
        |       +-- state.rs
        +-- tts-orchestrator-ui/
            +-- Cargo.toml
            +-- src/
                +-- lib.rs
                +-- app.rs
                +-- components/
```

## Core Data Types

### Segment

The fundamental unit of speech synthesis:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Segment {
    pub id: SegmentId,
    pub speaker: String,
    pub engine: EngineKind,
    pub text: String,
    pub emotion: Option<String>,
    pub style_tags: Vec<String>,
    pub pause_before_ms: u32,
    pub pause_after_ms: u32,
    pub nonverbals: Vec<NonVerbal>,
}
```

### EngineKind

Supported TTS engines:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EngineKind {
    Parler,  // Description-based TTS (Apache-2.0)
    Piper,   // Fast neural TTS (GPL-3.0)
    Xtts,    // Voice cloning (Coqui)
    Other(String),
}
```

### NonVerbal

Sound effects and breaths:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NonVerbalKind {
    Breath,
    Laugh,
    Sfx(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NonVerbal {
    pub kind: NonVerbalKind,
    pub offset_ms: u32,
}
```

### TimelineEvent

Events on the audio timeline:

```rust
#[derive(Clone, Debug)]
pub enum TimelineEventKind {
    Silence { duration_ms: u32 },
    TtsSegment { segment_id: SegmentId },
    NonVerbal { kind: NonVerbalKind, asset_name: Option<String> },
}

#[derive(Clone, Debug)]
pub struct TimelineEvent {
    pub order_index: u32,
    pub at_ms: u64,
    pub kind: TimelineEventKind,
}
```

## Engine Adapter Design

Each TTS engine has different capabilities and prompt formats. Adapters translate generic Segment data to engine-specific requests.

### Parler Adapter

Parler uses natural language descriptions for voice control:

```rust
// Input segment
Segment {
    speaker: "mike",
    emotion: Some("excited"),
    style_tags: vec!["fast", "clear"],
    text: "This is amazing!",
    ...
}

// Generated Parler description
"Mike's voice, excited and energetic, speaking quickly, very clear audio, close microphone"
```

Parler prompt construction:
1. Base voice description from speaker profile
2. Append emotion modifiers
3. Append style tags
4. Add audio quality hints ("clear", "no background noise")

### Piper Adapter

Piper is more straightforward - uses punctuation for prosody:

```rust
// Piper relies on:
// - Commas for short pauses
// - Periods for longer pauses
// - Question marks for rising intonation
// No special description needed - just map to voice ID
```

### XTTS Adapter

XTTS requires reference audio for voice cloning:

```rust
// Configuration includes:
// - Reference audio path
// - Language code
// - Speaker embedding (if pre-computed)
```

## Audio Pipeline Design

### Silence Generation

Pure function, no I/O:

```rust
pub fn silence_i16_mono(duration_ms: u32, sample_rate: u32) -> Vec<i16> {
    let total_samples = duration_ms as u64 * sample_rate as u64 / 1000;
    vec![0i16; total_samples as usize]
}
```

### Buffer Concatenation

Pure function combining audio buffers:

```rust
pub fn concat_i16(buffers: &[Vec<i16>]) -> Vec<i16> {
    let total: usize = buffers.iter().map(|b| b.len()).sum();
    let mut out = Vec::with_capacity(total);
    for b in buffers {
        out.extend_from_slice(b);
    }
    out
}
```

### Pipeline Execution

Async function with I/O:

```rust
pub async fn render_to_wav<P: AsRef<Path>>(
    client: &AllTalkClient,
    state: &PipelineState<'_>,
    out_path: P,
) -> Result<(), ScriptError> {
    let mut buffers: Vec<Vec<i16>> = Vec::new();

    for seg in &state.script.segments {
        // Insert pre-silence
        if seg.pause_before_ms > 0 {
            buffers.push(silence_i16_mono(seg.pause_before_ms, state.sample_rate));
        }

        // Synthesize segment
        let wav_bytes = synthesize_segment_wav(client, seg).await?;
        let samples = decode_wav(&wav_bytes)?;
        buffers.push(samples);

        // Insert post-silence
        if seg.pause_after_ms > 0 {
            buffers.push(silence_i16_mono(seg.pause_after_ms, state.sample_rate));
        }
    }

    let final_buf = concat_i16(&buffers);
    write_i16_mono(out_path, &final_buf, state.sample_rate)
}
```

## Error Handling

Single error enum for the domain:

```rust
#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Engine error: {0}")]
    Engine(String),

    #[error("Audio error: {0}")]
    Audio(String),

    #[error("Network error: {0}")]
    Network(String),
}
```

## Configuration Design

### Config File Location

```
~/.config/alltalk-client-rs/config.toml
```

### Config Format

```toml
[alltalk]
base_url = "http://192.168.1.100:7851"

[speakers.mike]
engine = "parler"
voice_id = "mike_clone"
default_emotion = "calm"
description_template = "Mike's voice, {emotion}, clear audio"

[speakers.guest]
engine = "piper"
voice_id = "en_US-lessac-medium"
```

### Environment Overrides

```bash
ALLTALK_URL=http://localhost:7851
ALLTALK_SPEAKER_MIKE_VOICE_ID=mike_v2
```

## CLI Design

### Command Structure

```
ttsctl <COMMAND>

Commands:
  ping           Check AllTalk connectivity
  list-voices    List available voices
  list-engines   List available TTS engines
  render-line    Synthesize a single line
  render-script  Render a full script
  serve          Start the web UI server

Options:
  -c, --config <FILE>  Config file path
  -v, --verbose        Verbose output
  -h, --help           Print help
  -V, --version        Print version
```

### Example Usage

```bash
# Check server
ttsctl ping

# Quick synthesis
ttsctl render-line --speaker mike --emotion excited "Hello world!" -o hello.wav

# Full script
ttsctl render-script podcast.yaml -o output/

# Multi-track export
ttsctl render-script podcast.yaml -o output/ --mix multitrack
```

## Testing Strategy

### Unit Tests

Location: Same file or `tests/` directory in each crate

Test pure functions:
- Silence generation (correct sample count)
- Buffer concatenation (correct ordering)
- Script parsing (valid/invalid YAML)
- Validation rules (unique IDs, positive sample rate)

### Integration Tests

Location: Separate `tts-tests` component

Test end-to-end flows:
- Mock AllTalk server (wiremock)
- Full script render pipeline
- Multi-speaker orchestration

### Manual Testing

Use Playwright via MCP for any future web UI testing.

## Dependencies

### Core Dependencies

| Crate | Purpose | License |
|-------|---------|---------|
| serde | Serialization | MIT/Apache-2.0 |
| serde_yaml | YAML parsing | MIT/Apache-2.0 |
| serde_json | JSON parsing | MIT/Apache-2.0 |
| thiserror | Error types | MIT/Apache-2.0 |
| reqwest | HTTP client | MIT/Apache-2.0 |
| tokio | Async runtime | MIT |
| hound | WAV I/O | Apache-2.0 |
| url | URL parsing | MIT/Apache-2.0 |
| async-trait | Async traits | MIT/Apache-2.0 |

### CLI Dependencies

| Crate | Purpose | License |
|-------|---------|---------|
| clap | Arg parsing | MIT/Apache-2.0 |
| anyhow | Error handling | MIT/Apache-2.0 |

### Optional UI Dependencies

| Crate | Purpose | License |
|-------|---------|---------|
| yew | WASM UI framework | MIT/Apache-2.0 |
| wasm-bindgen | JS interop | MIT/Apache-2.0 |

## Performance Considerations

1. **Parallel segment synthesis** - Use tokio for concurrent engine calls
2. **Memory efficiency** - Process segments streaming if possible
3. **Minimize engine switches** - Group segments by engine
4. **Pre-allocate buffers** - Use `with_capacity` for known sizes

## Security Considerations

1. **No credentials in code** - Use config file or environment
2. **Validate script input** - Prevent injection via text field
3. **Local network only** - AllTalk not exposed to internet
4. **License compliance** - Track engine licenses
