//! State management tests.

use tts_web_ui::state::{AppState, ModelKind, ParlerState, PiperState, StatusLevel, TabKind};

#[test]
fn test_default_state() {
    let state = AppState::new();
    assert_eq!(state.active_model, ModelKind::Parler);
    assert_eq!(state.active_tab, TabKind::Generate);
    assert_eq!(state.api_url, "http://localhost:7851");
}

#[test]
fn test_set_model() {
    let mut state = AppState::new();
    state.set_model(ModelKind::Piper);
    assert_eq!(state.active_model, ModelKind::Piper);
}

#[test]
fn test_set_tab() {
    let mut state = AppState::new();
    state.set_tab(TabKind::Settings);
    assert_eq!(state.active_tab, TabKind::Settings);
}

#[test]
fn test_set_status() {
    let mut state = AppState::new();
    state.set_status("Test message", StatusLevel::Success);
    assert_eq!(state.status.text, "Test message");
    assert_eq!(state.status.level, StatusLevel::Success);
}

#[test]
fn test_parler_defaults() {
    let parler = ParlerState::default();
    assert_eq!(parler.speaker, "Jon");
    assert_eq!(parler.audio_quality, "clear");
    assert_eq!(parler.temperature, 1.0);
}

#[test]
fn test_piper_defaults() {
    let piper = PiperState::default();
    assert_eq!(piper.voice_model, "en_US-amy-medium");
    assert_eq!(piper.length_scale, 1.0);
    assert_eq!(piper.noise_scale, 0.667);
}
