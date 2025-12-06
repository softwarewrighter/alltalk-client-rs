//! Generate tab with model-specific UIs.

use crate::state::{EngineInfo, ModelKind, ParlerState, PiperState, StatusLevel, XttsState};
use crate::widgets::{BackendDownWarning, NonCommercialWarning};
use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Blob, BlobPropertyBag, HtmlSelectElement, HtmlTextAreaElement, Url};
use yew::prelude::*;

const PARLER_SPEAKERS: &[&str] = &[
    "Jon", "Lea", "Gary", "Jenna", "Mike", "Laura", "Karen", "Rick", "Brenda", "David", "Emily",
    "James", "Sarah", "Tom", "Anna", "Chris",
];

const PIPER_VOICES: &[&str] = &["en_US-amy-medium"];

const XTTS_LANGUAGES: &[(&str, &str)] = &[
    ("en", "English"),
    ("es", "Spanish"),
    ("fr", "French"),
    ("de", "German"),
    ("it", "Italian"),
    ("pt", "Portuguese"),
    ("pl", "Polish"),
    ("tr", "Turkish"),
    ("ru", "Russian"),
    ("nl", "Dutch"),
    ("cs", "Czech"),
    ("ar", "Arabic"),
    ("zh-cn", "Chinese"),
    ("ja", "Japanese"),
    ("ko", "Korean"),
    ("hu", "Hungarian"),
];

/// Properties for GenerateTab component.
#[derive(Properties, PartialEq)]
pub struct GenerateTabProps {
    pub model: ModelKind,
    pub engine_info: Option<EngineInfo>,
    pub parler: ParlerState,
    pub piper: PiperState,
    pub xtts: XttsState,
    pub api_url: String,
    pub on_parler_text_change: Callback<String>,
    pub on_parler_speaker_change: Callback<String>,
    pub on_parler_description_change: Callback<String>,
    pub on_piper_text_change: Callback<String>,
    pub on_piper_voice_change: Callback<String>,
    pub on_xtts_text_change: Callback<String>,
    pub on_xtts_voice_change: Callback<String>,
    pub on_xtts_language_change: Callback<String>,
    pub on_audio_received: Callback<String>,
    pub on_status_change: Callback<(String, StatusLevel)>,
}

/// Generate tab that switches between model-specific UIs.
#[function_component(GenerateTab)]
pub fn generate_tab(props: &GenerateTabProps) -> Html {
    let backend_available = props
        .engine_info
        .as_ref()
        .map(|e| e.available)
        .unwrap_or(true);

    let content = if !backend_available {
        html! {
            <p class="backend-down-notice">
                {"Generation is unavailable while the backend is down."}
            </p>
        }
    } else {
        match props.model {
            ModelKind::Parler => render_parler_form(props),
            ModelKind::Piper => render_piper_form(props),
            // All cloning engines use similar form
            ModelKind::Xtts | ModelKind::GptSovits | ModelKind::Dia | ModelKind::Toucan => {
                render_cloning_form(props)
            }
        }
    };

    html! {
        <div class="generate-tab">
            <h2>{"Generate Speech"}</h2>
            <NonCommercialWarning model={props.model.clone()} />
            <BackendDownWarning engine_info={props.engine_info.clone()} />
            {content}
        </div>
    }
}

/// Render Parler TTS form.
fn render_parler_form(props: &GenerateTabProps) -> Html {
    let state = &props.parler;

    let on_text_input = {
        let cb = props.on_parler_text_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
            {
                cb.emit(input.value());
            }
        })
    };

    let on_speaker_change = {
        let cb = props.on_parler_speaker_change.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
            {
                cb.emit(select.value());
            }
        })
    };

    let on_desc_input = {
        let cb = props.on_parler_description_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
            {
                cb.emit(input.value());
            }
        })
    };

    let on_generate = {
        let api_url = props.api_url.clone();
        let text = state.text.clone();
        let description = state.description.clone();
        let on_audio = props.on_audio_received.clone();
        let on_status = props.on_status_change.clone();

        Callback::from(move |_: MouseEvent| {
            let api_url = api_url.clone();
            let text = text.clone();
            let description = description.clone();
            let on_audio = on_audio.clone();
            let on_status = on_status.clone();

            if text.trim().is_empty() {
                on_status.emit(("Please enter text to speak".into(), StatusLevel::Warning));
                return;
            }

            on_status.emit(("Generating audio...".into(), StatusLevel::Info));

            spawn_local(async move {
                let result = generate_parler(&api_url, &text, &description).await;
                match result {
                    Ok(blob_url) => on_audio.emit(blob_url),
                    Err(e) => on_status.emit((format!("Error: {}", e), StatusLevel::Error)),
                }
            });
        })
    };

    let speaker_options = PARLER_SPEAKERS.iter().map(|s| {
        html! { <option value={*s} selected={*s == state.speaker}>{s}</option> }
    });

    html! {
        <div class="parler-form">
            <div class="form-group">
                <label>{"Text to speak:"}</label>
                <textarea
                    class="text-input"
                    rows="4"
                    value={state.text.clone()}
                    placeholder="Enter text..."
                    oninput={on_text_input}
                />
            </div>
            <div class="form-row">
                <div class="form-group">
                    <label>{"Speaker:"}</label>
                    <select class="speaker-select" onchange={on_speaker_change}>
                        {for speaker_options}
                    </select>
                </div>
            </div>
            <div class="form-group">
                <label>{"Speaker Description:"}</label>
                <textarea
                    class="description-input"
                    rows="2"
                    value={state.description.clone()}
                    oninput={on_desc_input}
                />
            </div>
            <button class="btn btn-primary generate-btn" onclick={on_generate}>
                {"Generate Audio"}
            </button>
        </div>
    }
}

/// Render Piper TTS form.
fn render_piper_form(props: &GenerateTabProps) -> Html {
    let state = &props.piper;

    let on_text_input = {
        let cb = props.on_piper_text_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
            {
                cb.emit(input.value());
            }
        })
    };

    let on_voice_change = {
        let cb = props.on_piper_voice_change.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
            {
                cb.emit(select.value());
            }
        })
    };

    let on_generate = {
        let api_url = props.api_url.clone();
        let text = state.text.clone();
        let on_audio = props.on_audio_received.clone();
        let on_status = props.on_status_change.clone();

        Callback::from(move |_: MouseEvent| {
            let api_url = api_url.clone();
            let text = text.clone();
            let on_audio = on_audio.clone();
            let on_status = on_status.clone();

            if text.trim().is_empty() {
                on_status.emit(("Please enter text to speak".into(), StatusLevel::Warning));
                return;
            }

            on_status.emit(("Generating audio...".into(), StatusLevel::Info));

            spawn_local(async move {
                let result = generate_piper(&api_url, &text).await;
                match result {
                    Ok(blob_url) => on_audio.emit(blob_url),
                    Err(e) => on_status.emit((format!("Error: {}", e), StatusLevel::Error)),
                }
            });
        })
    };

    let voice_options = PIPER_VOICES.iter().map(|v| {
        html! { <option value={*v} selected={*v == state.voice_model}>{v}</option> }
    });

    html! {
        <div class="piper-form">
            <div class="form-group">
                <label>{"Text to speak:"}</label>
                <textarea
                    class="text-input"
                    rows="4"
                    value={state.text.clone()}
                    placeholder="Enter text..."
                    oninput={on_text_input}
                />
            </div>
            <div class="form-group">
                <label>{"Voice Model:"}</label>
                <select class="voice-select" onchange={on_voice_change}>
                    {for voice_options}
                </select>
            </div>
            <button class="btn btn-primary generate-btn" onclick={on_generate}>
                {"Generate Audio"}
            </button>
        </div>
    }
}

/// Generate audio using Parler TTS API.
/// Uses relative URL /api/tts-generate which is proxied by the local server.
async fn generate_parler(_api_url: &str, text: &str, description: &str) -> Result<String, String> {
    let body = format!(
        "text_input={}&engine=parler&speaker_description={}",
        urlencoding(text),
        urlencoding(description)
    );

    let response = Request::post("/api/tts-generate")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        let err_text = response.text().await.unwrap_or_default();
        return Err(format!("API error: {}", err_text));
    }

    let bytes = response.binary().await.map_err(|e| e.to_string())?;
    create_blob_url(&bytes, "audio/wav")
}

/// Render voice cloning form (used by XTTS, Dia, GPT-SoVITS, Toucan).
fn render_cloning_form(props: &GenerateTabProps) -> Html {
    let state = &props.xtts;

    let on_text_input = {
        let cb = props.on_xtts_text_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
            {
                cb.emit(input.value());
            }
        })
    };

    let on_voice_change = {
        let cb = props.on_xtts_voice_change.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
            {
                cb.emit(select.value());
            }
        })
    };

    let on_language_change = {
        let cb = props.on_xtts_language_change.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
            {
                cb.emit(select.value());
            }
        })
    };

    let on_generate = {
        let api_url = props.api_url.clone();
        let text = state.text.clone();
        let voice = state.voice.clone();
        let language = state.language.clone();
        let on_audio = props.on_audio_received.clone();
        let on_status = props.on_status_change.clone();

        Callback::from(move |_: MouseEvent| {
            let api_url = api_url.clone();
            let text = text.clone();
            let voice = voice.clone();
            let language = language.clone();
            let on_audio = on_audio.clone();
            let on_status = on_status.clone();

            if text.trim().is_empty() {
                on_status.emit(("Please enter text to speak".into(), StatusLevel::Warning));
                return;
            }

            if voice.is_empty() {
                on_status.emit((
                    "Please select a voice (import one in Settings tab first)".into(),
                    StatusLevel::Warning,
                ));
                return;
            }

            on_status.emit(("Generating audio...".into(), StatusLevel::Info));

            spawn_local(async move {
                let result = generate_xtts(&api_url, &text, &voice, &language).await;
                match result {
                    Ok(blob_url) => on_audio.emit(blob_url),
                    Err(e) => on_status.emit((format!("Error: {}", e), StatusLevel::Error)),
                }
            });
        })
    };

    let voice_options = state.available_voices.iter().map(|v| {
        html! { <option value={v.clone()} selected={*v == state.voice}>{v}</option> }
    });

    let language_options = XTTS_LANGUAGES.iter().map(|(code, name)| {
        html! { <option value={*code} selected={*code == state.language}>{name}</option> }
    });

    let has_voices = !state.available_voices.is_empty();

    let model_name = props.model.display_name();

    html! {
        <div class="cloning-form">
            <p class="form-hint">
                {format!("{} uses voice cloning. Import reference audio in the Settings tab first.", model_name)}
            </p>
            <div class="form-group">
                <label>{"Text to speak:"}</label>
                <textarea
                    class="text-input"
                    rows="4"
                    value={state.text.clone()}
                    placeholder="Enter text..."
                    oninput={on_text_input}
                />
            </div>
            <div class="form-row">
                <div class="form-group">
                    <label>{"Voice:"}</label>
                    <select class="voice-select" onchange={on_voice_change} disabled={!has_voices}>
                        {if !has_voices {
                            html! { <option>{"(No voices - import in Settings)"}</option> }
                        } else {
                            html! { {for voice_options} }
                        }}
                    </select>
                </div>
                <div class="form-group">
                    <label>{"Language:"}</label>
                    <select class="language-select" onchange={on_language_change}>
                        {for language_options}
                    </select>
                </div>
            </div>
            <button
                class="btn btn-primary generate-btn"
                onclick={on_generate}
                disabled={!has_voices}
            >
                {"Generate Audio"}
            </button>
        </div>
    }
}

/// Generate audio using Piper TTS API.
/// Uses relative URL /api/tts-generate which is proxied by the local server.
async fn generate_piper(_api_url: &str, text: &str) -> Result<String, String> {
    let body = format!("text_input={}&engine=piper", urlencoding(text));

    let response = Request::post("/api/tts-generate")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        let err_text = response.text().await.unwrap_or_default();
        return Err(format!("API error: {}", err_text));
    }

    let bytes = response.binary().await.map_err(|e| e.to_string())?;
    create_blob_url(&bytes, "audio/wav")
}

/// Generate audio using XTTS voice cloning API.
async fn generate_xtts(
    _api_url: &str,
    text: &str,
    voice: &str,
    language: &str,
) -> Result<String, String> {
    let body = format!(
        "text_input={}&engine=xtts&voice={}&language={}",
        urlencoding(text),
        urlencoding(voice),
        urlencoding(language)
    );

    let response = Request::post("/api/tts-generate")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        let err_text = response.text().await.unwrap_or_default();
        return Err(format!("API error: {}", err_text));
    }

    let bytes = response.binary().await.map_err(|e| e.to_string())?;
    create_blob_url(&bytes, "audio/wav")
}

/// Simple URL encoding for form data.
fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

/// Create a blob URL from audio bytes.
fn create_blob_url(bytes: &[u8], mime_type: &str) -> Result<String, String> {
    let uint8_array = js_sys::Uint8Array::from(bytes);
    let array = js_sys::Array::new();
    array.push(&uint8_array);

    let options = BlobPropertyBag::new();
    options.set_type(mime_type);

    let blob = Blob::new_with_u8_array_sequence_and_options(&array, &options)
        .map_err(|_| "Failed to create blob")?;

    Url::create_object_url_with_blob(&blob).map_err(|_| "Failed to create blob URL".to_string())
}
