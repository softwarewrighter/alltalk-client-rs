//! Application state management.

use serde::{Deserialize, Serialize};

/// Active TTS model/engine selection.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum ModelKind {
    #[default]
    Parler,
    Piper,
    Xtts,
}

/// Active tab selection.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum TabKind {
    Settings,
    #[default]
    Generate,
}

/// Parler TTS model state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParlerState {
    pub text: String,
    pub speaker: String,
    pub description: String,
    pub audio_quality: String,
    pub temperature: f32,
    pub do_sample: bool,
}

impl Default for ParlerState {
    fn default() -> Self {
        Self {
            text: String::new(),
            speaker: "Jon".into(),
            description:
                "Jon's voice is monotone yet slightly fast in delivery, with very clear audio."
                    .into(),
            audio_quality: "clear".into(),
            temperature: 1.0,
            do_sample: false,
        }
    }
}

/// Piper TTS model state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PiperState {
    pub text: String,
    pub voice_model: String,
    pub speaker_id: Option<u32>,
    pub length_scale: f32,
    pub noise_scale: f32,
    pub noise_w: f32,
    pub sentence_silence: f32,
}

impl Default for PiperState {
    fn default() -> Self {
        Self {
            text: String::new(),
            voice_model: "en_US-amy-medium".into(),
            speaker_id: None,
            length_scale: 1.0,
            noise_scale: 0.667,
            noise_w: 0.8,
            sentence_silence: 0.2,
        }
    }
}

/// Audio player state.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerState {
    pub audio_url: Option<String>,
    pub waveform_data: Vec<f32>,
    pub is_playing: bool,
    pub current_time: f32,
    pub duration: f32,
}

/// Status message with severity level.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StatusMessage {
    pub text: String,
    pub level: StatusLevel,
}

/// Severity level for status messages.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum StatusLevel {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// Application state for the TTS web UI.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppState {
    pub active_model: ModelKind,
    pub active_tab: TabKind,
    pub parler: ParlerState,
    pub piper: PiperState,
    pub player: PlayerState,
    pub status: StatusMessage,
    pub api_url: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    /// Create new state with default values.
    pub fn new() -> Self {
        Self {
            active_model: ModelKind::default(),
            active_tab: TabKind::default(),
            parler: ParlerState::default(),
            piper: PiperState::default(),
            player: PlayerState::default(),
            status: StatusMessage::default(),
            api_url: "http://localhost:7851".into(),
        }
    }

    /// Set the active model.
    pub fn set_model(&mut self, model: ModelKind) {
        self.active_model = model;
    }

    /// Set the active tab.
    pub fn set_tab(&mut self, tab: TabKind) {
        self.active_tab = tab;
    }

    /// Set status message.
    pub fn set_status(&mut self, text: &str, level: StatusLevel) {
        self.status = StatusMessage {
            text: text.into(),
            level,
        };
    }

    /// Set API URL.
    pub fn set_api_url(&mut self, url: String) {
        self.api_url = url;
    }

    /// Set audio URL for player.
    pub fn set_audio_url(&mut self, url: Option<String>) {
        self.player.audio_url = url;
    }

    /// Set playing state.
    pub fn set_playing(&mut self, playing: bool) {
        self.player.is_playing = playing;
    }

    /// Update Parler text.
    pub fn set_parler_text(&mut self, text: String) {
        self.parler.text = text;
    }

    /// Update Parler speaker.
    pub fn set_parler_speaker(&mut self, speaker: String) {
        self.parler.description = format!(
            "{}'s voice is monotone yet slightly fast in delivery, with {} audio.",
            speaker, self.parler.audio_quality
        );
        self.parler.speaker = speaker;
    }

    /// Update Parler description.
    pub fn set_parler_description(&mut self, description: String) {
        self.parler.description = description;
    }

    /// Update Piper text.
    pub fn set_piper_text(&mut self, text: String) {
        self.piper.text = text;
    }

    /// Update Piper voice model.
    pub fn set_piper_voice(&mut self, voice: String) {
        self.piper.voice_model = voice;
    }
}
