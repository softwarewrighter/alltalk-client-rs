//! Application state management.

use serde::{Deserialize, Serialize};

/// Active TTS model/engine selection.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum ModelKind {
    #[default]
    Parler,
    Piper,
    Xtts,
    GptSovits,
    Dia,
    Toucan,
}

impl ModelKind {
    /// Get the engine ID string for API calls.
    pub fn as_str(&self) -> &'static str {
        match self {
            ModelKind::Parler => "parler",
            ModelKind::Piper => "piper",
            ModelKind::Xtts => "xtts",
            ModelKind::GptSovits => "gptsovits",
            ModelKind::Dia => "dia",
            ModelKind::Toucan => "toucan",
        }
    }

    /// Get the display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            ModelKind::Parler => "Parler",
            ModelKind::Piper => "Piper",
            ModelKind::Xtts => "XTTS",
            ModelKind::GptSovits => "GPT-SoVITS",
            ModelKind::Dia => "Dia",
            ModelKind::Toucan => "Toucan",
        }
    }

    /// Check if this engine is for non-commercial use only.
    pub fn is_non_commercial(&self) -> bool {
        matches!(self, ModelKind::Xtts)
    }

    /// Check if this engine supports voice cloning.
    pub fn supports_cloning(&self) -> bool {
        matches!(
            self,
            ModelKind::Xtts | ModelKind::GptSovits | ModelKind::Dia | ModelKind::Toucan
        )
    }
}

/// Information about an available TTS engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EngineInfo {
    pub id: String,
    pub name: String,
    pub available: bool,
    pub license: String,
    pub cloning: bool,
    pub commercial: bool,
}

impl EngineInfo {
    /// Convert engine ID to ModelKind.
    pub fn to_model_kind(&self) -> Option<ModelKind> {
        match self.id.as_str() {
            "parler" => Some(ModelKind::Parler),
            "piper" => Some(ModelKind::Piper),
            "xtts" => Some(ModelKind::Xtts),
            "gptsovits" => Some(ModelKind::GptSovits),
            "dia" => Some(ModelKind::Dia),
            "toucan" => Some(ModelKind::Toucan),
            _ => None,
        }
    }
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

/// XTTS voice cloning state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XttsState {
    pub text: String,
    pub voice: String,
    pub language: String,
    pub available_voices: Vec<String>,
    pub temperature: f32,
    pub speed: f32,
}

impl Default for XttsState {
    fn default() -> Self {
        Self {
            text: String::new(),
            voice: String::new(),
            language: "en".into(),
            available_voices: Vec::new(),
            temperature: 0.7,
            speed: 1.0,
        }
    }
}

/// A voice record from the backend or local storage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceRecord {
    pub name: String,
    pub engine: ModelKind,
    pub transcript: Option<String>,
    pub is_local: bool,
}

/// Voice training state for recording/uploading reference audio.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TrainingState {
    pub voice_name: String,
    pub transcript: String,
    pub is_recording: bool,
    pub audio_blob: Option<Vec<u8>>,
    pub upload_progress: Option<f32>,
    pub backend_voices: Vec<VoiceRecord>,
    pub local_preview_url: Option<String>,
    pub target_engine: Option<ModelKind>,
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
    pub available_engines: Vec<EngineInfo>,
    pub engines_loaded: bool,
    pub parler: ParlerState,
    pub piper: PiperState,
    pub xtts: XttsState,
    pub training: TrainingState,
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
            available_engines: Vec::new(),
            engines_loaded: false,
            parler: ParlerState::default(),
            piper: PiperState::default(),
            xtts: XttsState::default(),
            training: TrainingState::default(),
            player: PlayerState::default(),
            status: StatusMessage::default(),
            api_url: "http://localhost:7851".into(),
        }
    }

    /// Set the active model.
    pub fn set_model(&mut self, model: ModelKind) {
        self.active_model = model;
    }

    /// Set available engines from API response.
    pub fn set_available_engines(&mut self, engines: Vec<EngineInfo>) {
        // If current model is not available, switch to first available
        let current_available = engines
            .iter()
            .any(|e| e.to_model_kind() == Some(self.active_model.clone()));

        if !current_available
            && !engines.is_empty()
            && let Some(first) = engines.first().and_then(|e| e.to_model_kind())
        {
            self.active_model = first;
        }

        self.available_engines = engines;
        self.engines_loaded = true;
    }

    /// Check if a model is available.
    pub fn is_model_available(&self, model: &ModelKind) -> bool {
        self.available_engines
            .iter()
            .any(|e| e.to_model_kind().as_ref() == Some(model))
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

    /// Update XTTS text.
    pub fn set_xtts_text(&mut self, text: String) {
        self.xtts.text = text;
    }

    /// Update XTTS voice.
    pub fn set_xtts_voice(&mut self, voice: String) {
        self.xtts.voice = voice;
    }

    /// Update XTTS language.
    pub fn set_xtts_language(&mut self, language: String) {
        self.xtts.language = language;
    }

    /// Set available XTTS voices.
    pub fn set_xtts_voices(&mut self, voices: Vec<String>) {
        if self.xtts.voice.is_empty() && !voices.is_empty() {
            self.xtts.voice = voices[0].clone();
        }
        self.xtts.available_voices = voices;
    }

    /// Update training voice name.
    pub fn set_training_voice_name(&mut self, name: String) {
        self.training.voice_name = name;
    }

    /// Update training transcript.
    pub fn set_training_transcript(&mut self, transcript: String) {
        self.training.transcript = transcript;
    }

    /// Set recording state.
    pub fn set_recording(&mut self, recording: bool) {
        self.training.is_recording = recording;
    }

    /// Set audio blob from recording or upload.
    pub fn set_training_audio(&mut self, audio: Option<Vec<u8>>) {
        self.training.audio_blob = audio;
    }

    /// Check if the current engine is available.
    pub fn is_current_engine_available(&self) -> bool {
        self.available_engines
            .iter()
            .find(|e| e.to_model_kind() == Some(self.active_model.clone()))
            .map(|e| e.available)
            .unwrap_or(false)
    }

    /// Get info for the current engine.
    pub fn current_engine_info(&self) -> Option<&EngineInfo> {
        self.available_engines
            .iter()
            .find(|e| e.to_model_kind() == Some(self.active_model.clone()))
    }

    /// Set backend voices.
    pub fn set_backend_voices(&mut self, voices: Vec<VoiceRecord>) {
        self.training.backend_voices = voices;
    }

    /// Set local preview URL.
    pub fn set_local_preview_url(&mut self, url: Option<String>) {
        self.training.local_preview_url = url;
    }

    /// Set target engine for cloning.
    pub fn set_target_engine(&mut self, engine: Option<ModelKind>) {
        self.training.target_engine = engine;
    }
}
