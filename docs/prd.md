# Product Requirements Document (PRD)

## Product Overview

**Product Name:** alltalk-client-rs

**Purpose:** A Rust CLI and optional web UI for controlling TTS (Text-to-Speech) synthesis via an AllTalk v2 backend, enabling precise control over pauses, emotions, non-verbal sounds, and multi-speaker orchestration.

**Target Users:**
- Content creators producing YouTube narrations
- Podcast producers generating multi-speaker discussions
- Developers integrating TTS into workflows

## Problem Statement

Current TTS solutions have limitations:
1. **Inconsistent pause control** - Engines interpret pauses differently
2. **Limited emotion control** - Markup like `(pause)` often fails
3. **No multi-speaker orchestration** - Manual splicing required
4. **Voice cloning complexity** - Difficult to integrate with existing workflows
5. **Engine lock-in** - Hard to switch between TTS engines

## Solution

A Rust-based TTS control plane that:
- Orchestrates multiple TTS engines via AllTalk v2
- Provides deterministic pause and silence control
- Supports structured scripts with per-segment configuration
- Enables multi-speaker podcast generation
- Abstracts engine differences behind a unified interface

## Use Cases

### UC1: YouTube Narration (Voice Clone)

**Actor:** Content creator with existing voice samples

**Flow:**
1. Write narration script in YAML with emotion/pause tags
2. Configure voice clone profile via AllTalk
3. Run `ttsctl render-script narration.yaml --out video_audio.wav`
4. Receive single-track WAV with precise timing

**Acceptance Criteria:**
- Voice sounds natural and consistent with source samples
- Pauses match specified millisecond values (+/- 10ms)
- Emotions (calm, excited) noticeably affect delivery
- Output sample rate matches target (24kHz or 48kHz)

### UC2: Multi-Speaker Podcast

**Actor:** Podcast producer

**Flow:**
1. Write dialogue script with multiple speakers
2. Assign engines/voices per speaker (Parler for hosts, Piper for system)
3. Run `ttsctl render-script podcast.yaml --out . --mix multitrack`
4. Receive per-speaker WAV files for DAW import

**Acceptance Criteria:**
- Each speaker has distinct, consistent voice
- Turn-taking has natural pacing (configurable pauses)
- Non-verbal sounds (laughs, breaths) inserted at correct positions
- Multi-track output enables post-production mixing

### UC3: Quick TTS Generation

**Actor:** Developer testing audio output

**Flow:**
1. Run `ttsctl render-line --speaker mike "Hello world" --out test.wav`
2. Receive single-line audio file

**Acceptance Criteria:**
- Fast turnaround (< 5 seconds for short text)
- Supports engine/speaker/emotion flags
- Works without full script file

## Functional Requirements

### FR1: Script Parsing

| ID | Requirement | Priority |
|----|-------------|----------|
| FR1.1 | Parse YAML script files | Must |
| FR1.2 | Parse JSON script files | Should |
| FR1.3 | Validate segment IDs are unique | Must |
| FR1.4 | Validate sample rate is positive | Must |
| FR1.5 | Validate engine names match available engines | Should |

### FR2: Engine Communication

| ID | Requirement | Priority |
|----|-------------|----------|
| FR2.1 | Connect to AllTalk v2 HTTP API | Must |
| FR2.2 | Check AllTalk readiness via /api/ready | Must |
| FR2.3 | List available voices via /api/voices | Should |
| FR2.4 | Synthesize segments via /api/tts-generate | Must |
| FR2.5 | Switch engines via /api/reload | Should |
| FR2.6 | Support streaming synthesis (future) | Could |

### FR3: Audio Pipeline

| ID | Requirement | Priority |
|----|-------------|----------|
| FR3.1 | Insert precise silence (millisecond accuracy) | Must |
| FR3.2 | Concatenate audio segments | Must |
| FR3.3 | Write mono WAV files | Must |
| FR3.4 | Write stereo WAV files | Should |
| FR3.5 | Insert non-verbal sound assets | Should |
| FR3.6 | Export multi-track (per-speaker) | Should |

### FR4: CLI Interface

| ID | Requirement | Priority |
|----|-------------|----------|
| FR4.1 | `ttsctl ping` - Check AllTalk connectivity | Must |
| FR4.2 | `ttsctl list-voices` - List available voices | Should |
| FR4.3 | `ttsctl render-line` - Synthesize single line | Must |
| FR4.4 | `ttsctl render-script` - Render full script | Must |
| FR4.5 | `ttsctl serve` - Start local web UI server | Could |

### FR5: Configuration

| ID | Requirement | Priority |
|----|-------------|----------|
| FR5.1 | Read AllTalk URL from config file | Must |
| FR5.2 | Support environment variable overrides | Should |
| FR5.3 | Per-speaker voice profiles | Should |
| FR5.4 | Engine-specific prompt templates | Could |

## Non-Functional Requirements

### NFR1: Performance

- Single segment synthesis: < 5 seconds (for 1-2 sentences)
- Full script render: Linear with segment count
- Memory: < 500MB for typical podcast scripts

### NFR2: Reliability

- Graceful handling of AllTalk unavailability
- Retry logic for transient network failures
- Clear error messages with actionable guidance

### NFR3: Maintainability

- Maximum 4 functions per Rust module
- Maximum 5 modules per crate
- Maximum 5 crates per component
- All public items documented
- Zero clippy warnings

### NFR4: Compatibility

- Rust 2024 edition
- macOS (M3) primary development platform
- Linux (Arch) for AllTalk server
- AllTalk v2 API compatibility

## Script Format Specification

```yaml
sample_rate: 24000
segments:
  - id: intro
    speaker: mike
    engine: parler
    text: "Welcome back to the channel."
    emotion: "confident"
    style_tags: ["medium_speed"]
    pause_before_ms: 0
    pause_after_ms: 750
    nonverbals: []

  - id: cohost_response
    speaker: guest
    engine: parler
    text: "Thanks for having me!"
    emotion: "excited"
    style_tags: ["smiling"]
    pause_before_ms: 250
    pause_after_ms: 500
    nonverbals:
      - kind: Laugh
        offset_ms: 0
```

### Segment Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | string | Yes | Unique identifier |
| speaker | string | Yes | Speaker name (maps to voice profile) |
| engine | enum | Yes | TTS engine (parler, piper, xtts) |
| text | string | Yes | Text to synthesize |
| emotion | string | No | Emotion hint (engine-specific) |
| style_tags | array | No | Style modifiers |
| pause_before_ms | u32 | No | Silence before segment |
| pause_after_ms | u32 | No | Silence after segment |
| nonverbals | array | No | Non-verbal sounds to insert |

## Success Metrics

1. **Functional:** All must-have requirements implemented and tested
2. **Quality:** Zero clippy warnings, all tests passing
3. **Usability:** Complete documentation with examples
4. **Performance:** Render 10-segment script in < 60 seconds

## Out of Scope (v1)

- Real-time streaming preview
- Voice training/fine-tuning
- Subtitle/SRT generation
- GUI installer
- Cloud deployment
