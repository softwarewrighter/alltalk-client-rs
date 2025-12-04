//! Generate tab with model-specific UIs.

use crate::state::{ModelKind, ParlerState, PiperState, StatusLevel};
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

/// Properties for GenerateTab component.
#[derive(Properties, PartialEq)]
pub struct GenerateTabProps {
    pub model: ModelKind,
    pub parler: ParlerState,
    pub piper: PiperState,
    pub api_url: String,
    pub on_parler_text_change: Callback<String>,
    pub on_parler_speaker_change: Callback<String>,
    pub on_parler_description_change: Callback<String>,
    pub on_piper_text_change: Callback<String>,
    pub on_piper_voice_change: Callback<String>,
    pub on_audio_received: Callback<String>,
    pub on_status_change: Callback<(String, StatusLevel)>,
}

/// Generate tab that switches between model-specific UIs.
#[function_component(GenerateTab)]
pub fn generate_tab(props: &GenerateTabProps) -> Html {
    let content = match props.model {
        ModelKind::Parler => render_parler_form(props),
        ModelKind::Piper => render_piper_form(props),
        ModelKind::Xtts => html! { <div class="xtts-placeholder">{"XTTS coming soon..."}</div> },
    };
    html! { <div class="generate-tab"><h2>{"Generate Speech"}</h2>{content}</div> }
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
