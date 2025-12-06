//! Main Yew application component.

use crate::generate::GenerateTab;
use crate::state::{AppState, EngineInfo, ModelKind, StatusLevel, TabKind};
use crate::training::SettingsTab;
use crate::widgets::{AudioPlayer, Footer, Header, StatusBar, TabBar};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

/// Fetch available engines from the API.
async fn fetch_engines() -> Result<Vec<EngineInfo>, String> {
    let response = Request::get("/api/engines")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err("Failed to fetch engines".into());
    }

    #[derive(serde::Deserialize)]
    struct EnginesResponse {
        engines: Vec<EngineInfo>,
    }

    let data: EnginesResponse = response.json().await.map_err(|e| e.to_string())?;
    Ok(data.engines)
}

/// Root application component.
#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(AppState::new);

    // Fetch available engines on startup
    {
        let state = state.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match fetch_engines().await {
                    Ok(engines) => {
                        let mut new_state = (*state).clone();
                        new_state.set_available_engines(engines);
                        new_state.set_status("Connected to TTS backend", StatusLevel::Success);
                        state.set(new_state);
                    }
                    Err(e) => {
                        let mut new_state = (*state).clone();
                        new_state.engines_loaded = true; // Mark as loaded even on error
                        new_state.set_status(
                            &format!("Failed to connect: {}. Check API URL in Settings.", e),
                            StatusLevel::Error,
                        );
                        state.set(new_state);
                    }
                }
            });
            || ()
        });
    }

    let on_model_change = {
        let state = state.clone();
        Callback::from(move |model: ModelKind| {
            let mut new_state = (*state).clone();
            new_state.set_model(model);
            state.set(new_state);
        })
    };

    let on_tab_change = {
        let state = state.clone();
        Callback::from(move |tab: TabKind| {
            let mut new_state = (*state).clone();
            new_state.set_tab(tab);
            state.set(new_state);
        })
    };

    let on_api_url_change = {
        let state = state.clone();
        Callback::from(move |url: String| {
            let mut new_state = (*state).clone();
            new_state.set_api_url(url);
            state.set(new_state);
        })
    };

    let on_parler_text_change = {
        let state = state.clone();
        Callback::from(move |text: String| {
            let mut new_state = (*state).clone();
            new_state.set_parler_text(text);
            state.set(new_state);
        })
    };

    let on_parler_speaker_change = {
        let state = state.clone();
        Callback::from(move |speaker: String| {
            let mut new_state = (*state).clone();
            new_state.set_parler_speaker(speaker);
            state.set(new_state);
        })
    };

    let on_parler_description_change = {
        let state = state.clone();
        Callback::from(move |desc: String| {
            let mut new_state = (*state).clone();
            new_state.set_parler_description(desc);
            state.set(new_state);
        })
    };

    let on_piper_text_change = {
        let state = state.clone();
        Callback::from(move |text: String| {
            let mut new_state = (*state).clone();
            new_state.set_piper_text(text);
            state.set(new_state);
        })
    };

    let on_piper_voice_change = {
        let state = state.clone();
        Callback::from(move |voice: String| {
            let mut new_state = (*state).clone();
            new_state.set_piper_voice(voice);
            state.set(new_state);
        })
    };

    let on_xtts_text_change = {
        let state = state.clone();
        Callback::from(move |text: String| {
            let mut new_state = (*state).clone();
            new_state.set_xtts_text(text);
            state.set(new_state);
        })
    };

    let on_xtts_voice_change = {
        let state = state.clone();
        Callback::from(move |voice: String| {
            let mut new_state = (*state).clone();
            new_state.set_xtts_voice(voice);
            state.set(new_state);
        })
    };

    let on_xtts_language_change = {
        let state = state.clone();
        Callback::from(move |lang: String| {
            let mut new_state = (*state).clone();
            new_state.set_xtts_language(lang);
            state.set(new_state);
        })
    };

    let on_audio_received = {
        let state = state.clone();
        Callback::from(move |url: String| {
            let mut new_state = (*state).clone();
            new_state.set_audio_url(Some(url));
            new_state.set_status("Audio generated successfully", StatusLevel::Success);
            state.set(new_state);
        })
    };

    let on_status_change = {
        let state = state.clone();
        Callback::from(move |(text, level): (String, StatusLevel)| {
            let mut new_state = (*state).clone();
            new_state.set_status(&text, level);
            state.set(new_state);
        })
    };

    let on_voice_name_change = {
        let state = state.clone();
        Callback::from(move |name: String| {
            let mut new_state = (*state).clone();
            new_state.set_training_voice_name(name);
            state.set(new_state);
        })
    };

    let on_transcript_change = {
        let state = state.clone();
        Callback::from(move |transcript: String| {
            let mut new_state = (*state).clone();
            new_state.set_training_transcript(transcript);
            state.set(new_state);
        })
    };

    let on_recording_change = {
        let state = state.clone();
        Callback::from(move |recording: bool| {
            let mut new_state = (*state).clone();
            new_state.set_recording(recording);
            state.set(new_state);
        })
    };

    let on_audio_change = {
        let state = state.clone();
        Callback::from(move |audio: Option<Vec<u8>>| {
            let mut new_state = (*state).clone();
            new_state.set_training_audio(audio);
            state.set(new_state);
        })
    };

    let on_voices_refresh = {
        let state = state.clone();
        Callback::from(move |voices: Vec<String>| {
            let mut new_state = (*state).clone();
            new_state.set_xtts_voices(voices);
            state.set(new_state);
        })
    };

    let on_voice_select = {
        let state = state.clone();
        Callback::from(move |name: String| {
            let mut new_state = (*state).clone();
            new_state.set_training_voice_name(name.clone());
            new_state.set_xtts_voice(name);
            state.set(new_state);
        })
    };

    let on_voice_delete = {
        let state = state.clone();
        Callback::from(move |name: String| {
            let mut new_state = (*state).clone();
            // Remove from backend_voices
            new_state.training.backend_voices.retain(|v| v.name != name);
            // Remove from xtts voices
            new_state.xtts.available_voices.retain(|v| *v != name);
            // Clear selection if it was the deleted voice
            if new_state.training.voice_name == name {
                new_state.training.voice_name.clear();
            }
            if new_state.xtts.voice == name {
                new_state.xtts.voice = new_state
                    .xtts
                    .available_voices
                    .first()
                    .cloned()
                    .unwrap_or_default();
            }
            state.set(new_state);
        })
    };

    let on_status_change_settings = on_status_change.clone();

    // Get current engine info
    let current_engine_info = state.current_engine_info().cloned();

    let tab_content = match state.active_tab {
        TabKind::Settings => html! {
            <SettingsTab
                model={state.active_model.clone()}
                engine_info={current_engine_info.clone()}
                api_url={state.api_url.clone()}
                training={state.training.clone()}
                on_api_url_change={on_api_url_change}
                on_voice_name_change={on_voice_name_change}
                on_transcript_change={on_transcript_change}
                on_recording_change={on_recording_change}
                on_audio_change={on_audio_change}
                on_status_change={on_status_change_settings}
                on_voices_refresh={on_voices_refresh}
                on_voice_select={on_voice_select.clone()}
                on_voice_delete={on_voice_delete.clone()}
            />
        },
        TabKind::Generate => html! {
            <GenerateTab
                model={state.active_model.clone()}
                engine_info={current_engine_info}
                parler={state.parler.clone()}
                piper={state.piper.clone()}
                xtts={state.xtts.clone()}
                api_url={state.api_url.clone()}
                on_parler_text_change={on_parler_text_change}
                on_parler_speaker_change={on_parler_speaker_change}
                on_parler_description_change={on_parler_description_change}
                on_piper_text_change={on_piper_text_change}
                on_piper_voice_change={on_piper_voice_change}
                on_xtts_text_change={on_xtts_text_change}
                on_xtts_voice_change={on_xtts_voice_change}
                on_xtts_language_change={on_xtts_language_change}
                on_audio_received={on_audio_received}
                on_status_change={on_status_change}
            />
        },
    };

    html! {
        <div class="app">
            <Header
                active_model={state.active_model.clone()}
                available_engines={state.available_engines.clone()}
                engines_loaded={state.engines_loaded}
                on_model_change={on_model_change}
            />
            <TabBar active_tab={state.active_tab.clone()} on_tab_change={on_tab_change} />
            <main class="main-content">
                {tab_content}
                <AudioPlayer player={state.player.clone()} />
                <StatusBar message={state.status.clone()} />
            </main>
            <Footer />
        </div>
    }
}
