# Implementation Plan

## Phase Overview

| Phase | Focus | Components |
|-------|-------|------------|
| 1 | Foundation | tts-spec-model, tts-spec-script |
| 2 | Engine Client | tts-engine-api, tts-engine-alltalk |
| 3 | Audio Pipeline | tts-render-core, tts-render-pipeline |
| 4 | CLI | ttsctl (basic commands) |
| 5 | Integration | End-to-end testing, refinement |
| 6 | Enhancement | Multi-track, non-verbals, UI (optional) |

---

## Phase 1: Foundation (tts-spec)

### 1.1 Create Workspace Structure

- [ ] Create root Cargo.toml with workspace configuration
- [ ] Create `components/tts-spec/Cargo.toml` (nested workspace)
- [ ] Create `components/tts-spec/tts-spec-model/` crate
- [ ] Create `components/tts-spec/tts-spec-script/` crate
- [ ] Create `components/tts-spec/tts-spec-layout/` crate

### 1.2 tts-spec-model Crate

- [ ] Define `SegmentId` newtype
- [ ] Define `EngineKind` enum (Parler, Piper, Xtts, Other)
- [ ] Define `NonVerbalKind` enum
- [ ] Define `NonVerbal` struct
- [ ] Define `Segment` struct
- [ ] Define `Script` struct
- [ ] Define `ScriptError` enum
- [ ] Add serde derives for all types
- [ ] Write doc comments for all public items

### 1.3 tts-spec-script Crate

- [ ] Implement `from_yaml_str()` - parse Script from YAML
- [ ] Implement `from_json_str()` - parse Script from JSON
- [ ] Implement `from_yaml_reader()` - parse from io::Read
- [ ] Implement `from_json_reader()` - parse from io::Read
- [ ] Implement `validate()` - top-level validation
- [ ] Implement `validate_segment_ids_unique()` - check ID uniqueness
- [ ] Implement `validate_sample_rate()` - check positive sample rate
- [ ] Implement `validate_for_engine()` - engine-specific checks (stub)
- [ ] Implement `to_yaml()` - serialize to YAML
- [ ] Implement `to_json()` - serialize to JSON
- [ ] Write unit tests for parsing valid scripts
- [ ] Write unit tests for parsing invalid scripts
- [ ] Write unit tests for validation rules

### 1.4 tts-spec-layout Crate

- [ ] Define `TimelineEventKind` enum
- [ ] Define `TimelineEvent` struct
- [ ] Implement `build_timeline()` - convert Script to events
- [ ] Implement `estimate_duration_ms()` - sum silence durations
- [ ] Write unit tests for timeline building

---

## Phase 2: Engine Client (tts-engine)

### 2.1 Create Workspace Structure

- [ ] Create `components/tts-engine/Cargo.toml`
- [ ] Create `components/tts-engine/tts-engine-api/` crate
- [ ] Create `components/tts-engine/tts-engine-alltalk/` crate

### 2.2 tts-engine-api Crate

- [ ] Define `TtsEngine` trait with async methods
- [ ] Define `TtsHub` trait for multi-engine operations
- [ ] Define request/response types for synthesis
- [ ] Write doc comments

### 2.3 tts-engine-alltalk Crate

- [ ] Define `AllTalkConfig` struct
- [ ] Define `AllTalkClient` struct
- [ ] Implement `new_client()` - create client from URL
- [ ] Implement `tts_generate_url()` - build API URLs
- [ ] Implement `ready_url()` - health check URL
- [ ] Implement `check_ready()` - async health check
- [ ] Implement `synthesize_segment_wav()` - call /api/tts-generate
- [ ] Implement `list_voices()` - call /api/voices
- [ ] Implement `switch_engine()` - call /api/reload
- [ ] Define `AllTalkTtsRequest` struct
- [ ] Implement `segment_to_alltalk_request()` - mapping
- [ ] Implement `engine_to_method()` - enum to string
- [ ] Write integration tests with mock server

---

## Phase 3: Audio Pipeline (tts-render)

### 3.1 Create Workspace Structure

- [ ] Create `components/tts-render/Cargo.toml`
- [ ] Create `components/tts-render/tts-render-core/` crate
- [ ] Create `components/tts-render/tts-render-pipeline/` crate

### 3.2 tts-render-core Crate

- [ ] Implement `silence_i16_mono()` - generate silence
- [ ] Implement `silence_f32_mono()` - float version
- [ ] Implement `silence_i16_stereo()` - stereo version
- [ ] Implement `concat_i16()` - concatenate buffers
- [ ] Implement `concat_f32()` - float version
- [ ] Implement `interleave_stereo_i16()` - L/R interleave
- [ ] Implement `write_i16_mono()` - write WAV file
- [ ] Implement `read_i16_mono()` - read WAV file
- [ ] Write unit tests for silence generation
- [ ] Write unit tests for concatenation
- [ ] Write tests for WAV I/O

### 3.3 tts-render-pipeline Crate

- [ ] Define `PipelineState` struct
- [ ] Implement `new_state()` - create from Script
- [ ] Implement `segments_in_order()` - get segment refs
- [ ] Implement `render_to_wav()` - full async render
- [ ] Implement `decode_i16_mono_from_bytes()` - decode WAV bytes
- [ ] Write integration tests

---

## Phase 4: CLI (ttsctl)

### 4.1 Create Workspace Structure

- [ ] Create `components/tts-apps/Cargo.toml`
- [ ] Create `components/tts-apps/ttsctl/` crate

### 4.2 CLI Implementation

- [ ] Define CLI args with clap derive
- [ ] Define `Command` enum (Ping, ListVoices, RenderLine, RenderScript)
- [ ] Define `Config` struct
- [ ] Implement `load_config()` - read from file
- [ ] Implement `handle_ping()` - check connectivity
- [ ] Implement `handle_list_voices()` - list voices
- [ ] Implement `handle_render_line()` - single line synthesis
- [ ] Implement `handle_render_script()` - full script render
- [ ] Write main.rs entry point
- [ ] Test CLI manually

---

## Phase 5: Integration

### 5.1 End-to-End Testing

- [ ] Set up AllTalk on test server
- [ ] Create sample scripts (YouTube narration, podcast)
- [ ] Test full render pipeline
- [ ] Verify audio quality and timing

### 5.2 Documentation

- [ ] Update README with usage examples
- [ ] Add script format examples
- [ ] Document configuration options
- [ ] Add troubleshooting guide

### 5.3 Quality Gates

- [ ] All tests pass (`cargo test`)
- [ ] Zero clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Markdown validated (`markdown-checker`)
- [ ] sw-checklist passes

---

## Phase 6: Enhancements (Future)

### 6.1 Multi-Track Export

- [ ] Implement per-speaker WAV export
- [ ] Add `--mix multitrack` CLI flag
- [ ] Test with DAW import

### 6.2 Non-Verbal Sound Library

- [ ] Create asset directory structure
- [ ] Implement asset loading
- [ ] Implement timeline insertion
- [ ] Generate sample non-verbal assets

### 6.3 Web UI (Optional)

- [ ] Create `tts-orchestrator-server` crate
- [ ] Create `tts-orchestrator-ui` Yew app
- [ ] Implement script editor
- [ ] Implement preview playback
- [ ] Implement export functionality

### 6.4 Advanced Features

- [ ] Streaming synthesis support
- [ ] Parler description templates
- [ ] Voice clone workflow integration
- [ ] Subtitle/SRT generation

---

## Dependencies Between Phases

```
Phase 1 (tts-spec)
    |
    v
Phase 2 (tts-engine) ----+
    |                    |
    v                    v
Phase 3 (tts-render) <---+
    |
    v
Phase 4 (ttsctl)
    |
    v
Phase 5 (Integration)
    |
    v
Phase 6 (Enhancements)
```

---

## Checkpoint Criteria

Each phase is complete when:

1. All checklist items are done
2. All tests pass
3. Zero clippy warnings
4. Code formatted
5. Documentation updated
6. Git commit with clear message
7. Pushed to remote

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| AllTalk API changes | Version-pin, add API version check |
| Audio format mismatch | Detect and resample as needed |
| Large script memory | Stream processing for long scripts |
| Network failures | Retry logic, offline mode for testing |
