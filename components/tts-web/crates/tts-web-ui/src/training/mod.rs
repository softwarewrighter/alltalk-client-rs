//! Settings tab with model-specific configuration and voice training.

use crate::state::{EngineInfo, ModelKind, StatusLevel, TrainingState};
use crate::widgets::{BackendDownWarning, VoiceList};
use gloo_net::http::Request;
use js_sys::{Array, Uint8Array};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    Blob, BlobPropertyBag, DragEvent, File, FileReader, HtmlInputElement, MediaRecorder,
    MediaRecorderOptions, MediaStream, Url,
};
use yew::prelude::*;

/// Properties for SettingsTab component.
#[derive(Properties, PartialEq)]
pub struct SettingsTabProps {
    pub model: ModelKind,
    pub engine_info: Option<EngineInfo>,
    pub api_url: String,
    pub training: TrainingState,
    pub on_api_url_change: Callback<String>,
    pub on_voice_name_change: Callback<String>,
    pub on_transcript_change: Callback<String>,
    pub on_recording_change: Callback<bool>,
    pub on_audio_change: Callback<Option<Vec<u8>>>,
    pub on_status_change: Callback<(String, StatusLevel)>,
    pub on_voices_refresh: Callback<Vec<String>>,
    pub on_voice_select: Callback<String>,
    pub on_voice_delete: Callback<String>,
}

/// Settings tab with model-specific options and voice training.
#[function_component(SettingsTab)]
pub fn settings_tab(props: &SettingsTabProps) -> Html {
    let supports_training = props.model.supports_cloning();
    let model_name = props.model.display_name();
    let backend_available = props
        .engine_info
        .as_ref()
        .map(|e| e.available)
        .unwrap_or(true);

    let on_url_change = {
        let cb = props.on_api_url_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                cb.emit(input.value());
            }
        })
    };

    // Get selected voice from XTTS state (if any)
    let selected_voice = props
        .training
        .backend_voices
        .iter()
        .find(|v| v.name == props.training.voice_name)
        .map(|v| v.name.clone());

    html! {
        <div class="settings-tab">
            <h2>{format!("{} Settings", model_name)}</h2>
            <BackendDownWarning engine_info={props.engine_info.clone()} />

            <div class="form-group">
                <label>{"AllTalk API URL:"}</label>
                <input
                    type="text"
                    value={props.api_url.clone()}
                    placeholder="http://localhost:7851"
                    oninput={on_url_change}
                />
                <span class="field-hint">{"Enter the URL of your TTS backend"}</span>
            </div>

            <div class={classes!("training-section", (!supports_training).then_some("disabled"))}>
                <h3>{"Voice Training"}</h3>
                {render_training_hint(supports_training, model_name)}
                {if supports_training && backend_available {
                    html! {
                        <>
                            <div class="voice-management">
                                <h4>{"Imported Voices"}</h4>
                                <VoiceList
                                    voices={props.training.backend_voices.clone()}
                                    selected={selected_voice}
                                    on_select={props.on_voice_select.clone()}
                                    on_delete={props.on_voice_delete.clone()}
                                />
                            </div>
                            <TrainingForm
                                training={props.training.clone()}
                                on_voice_name_change={props.on_voice_name_change.clone()}
                                on_transcript_change={props.on_transcript_change.clone()}
                                on_recording_change={props.on_recording_change.clone()}
                                on_audio_change={props.on_audio_change.clone()}
                                on_status_change={props.on_status_change.clone()}
                                on_voices_refresh={props.on_voices_refresh.clone()}
                            />
                        </>
                    }
                } else if supports_training && !backend_available {
                    html! {
                        <p class="backend-down-notice">
                            {"Voice training is unavailable while the backend is down."}
                        </p>
                    }
                } else {
                    render_disabled_form()
                }}
            </div>
        </div>
    }
}

fn render_training_hint(supports: bool, model: &str) -> Html {
    if supports {
        html! {
            <p class="tab-description">
                {"Import reference audio (3-10 seconds) to create a cloned voice."}
            </p>
        }
    } else {
        html! {
            <p class="tab-description disabled-hint">
                {format!("{} does not support voice training. Select XTTS model.", model)}
            </p>
        }
    }
}

fn render_disabled_form() -> Html {
    html! {
        <div class="training-form disabled">
            <div class="form-group">
                <label>{"Voice Name:"}</label>
                <input type="text" disabled=true placeholder="Select XTTS model..." />
            </div>
            <div class="form-group">
                <label>{"Reference Audio:"}</label>
                <div class="drop-zone disabled">{"Drag audio here or click to upload"}</div>
            </div>
            <button class="btn btn-primary" disabled=true>{"Import Voice"}</button>
        </div>
    }
}

/// Properties for TrainingForm component.
#[derive(Properties, PartialEq)]
pub struct TrainingFormProps {
    pub training: TrainingState,
    pub on_voice_name_change: Callback<String>,
    pub on_transcript_change: Callback<String>,
    pub on_recording_change: Callback<bool>,
    pub on_audio_change: Callback<Option<Vec<u8>>>,
    pub on_status_change: Callback<(String, StatusLevel)>,
    pub on_voices_refresh: Callback<Vec<String>>,
}

/// Training form with microphone recording and file upload.
#[function_component(TrainingForm)]
pub fn training_form(props: &TrainingFormProps) -> Html {
    let is_dragging = use_state(|| false);
    let audio_preview_url = use_state(|| Option::<String>::None);
    let recorder_ref = use_state(|| Option::<MediaRecorder>::None);

    let on_name_change = {
        let cb = props.on_voice_name_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                cb.emit(input.value());
            }
        })
    };

    let on_transcript_input = {
        let cb = props.on_transcript_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok())
            {
                cb.emit(input.value());
            }
        })
    };

    // Handle file selection from input
    let on_file_select = {
        let on_audio = props.on_audio_change.clone();
        let on_status = props.on_status_change.clone();
        let preview_url = audio_preview_url.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                && let Some(files) = input.files()
                && let Some(file) = files.get(0)
            {
                handle_file(
                    file,
                    on_audio.clone(),
                    on_status.clone(),
                    preview_url.clone(),
                );
            }
        })
    };

    // Drag and drop handlers
    let on_drag_over = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            is_dragging.set(true);
        })
    };

    let on_drag_leave = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            is_dragging.set(false);
        })
    };

    let on_drop = {
        let on_audio = props.on_audio_change.clone();
        let on_status = props.on_status_change.clone();
        let is_dragging = is_dragging.clone();
        let preview_url = audio_preview_url.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            is_dragging.set(false);
            if let Some(dt) = e.data_transfer()
                && let Some(files) = dt.files()
                && let Some(file) = files.get(0)
            {
                handle_file(
                    file,
                    on_audio.clone(),
                    on_status.clone(),
                    preview_url.clone(),
                );
            }
        })
    };

    // Microphone recording
    let on_record_toggle = {
        let is_recording = props.training.is_recording;
        let on_recording = props.on_recording_change.clone();
        let on_audio = props.on_audio_change.clone();
        let on_status = props.on_status_change.clone();
        let recorder_ref = recorder_ref.clone();
        let preview_url = audio_preview_url.clone();

        Callback::from(move |_: MouseEvent| {
            let on_recording = on_recording.clone();
            let on_audio = on_audio.clone();
            let on_status = on_status.clone();
            let recorder_ref = recorder_ref.clone();
            let preview_url = preview_url.clone();

            if is_recording {
                // Stop recording
                if let Some(recorder) = (*recorder_ref).as_ref() {
                    let _ = recorder.stop();
                }
                on_recording.emit(false);
            } else {
                // Start recording
                on_recording.emit(true);
                on_status.emit(("Recording... Click again to stop".into(), StatusLevel::Info));

                spawn_local(async move {
                    match start_recording(on_audio, on_status.clone(), preview_url).await {
                        Ok(recorder) => {
                            recorder_ref.set(Some(recorder));
                        }
                        Err(e) => {
                            on_recording.emit(false);
                            on_status.emit((format!("Mic error: {}", e), StatusLevel::Error));
                        }
                    }
                });
            }
        })
    };

    // Import voice to server
    let on_import = {
        let voice_name = props.training.voice_name.clone();
        let transcript = props.training.transcript.clone();
        let audio_blob = props.training.audio_blob.clone();
        let on_status = props.on_status_change.clone();
        let on_voices = props.on_voices_refresh.clone();

        Callback::from(move |_: MouseEvent| {
            let voice_name = voice_name.clone();
            let transcript = transcript.clone();
            let audio_blob = audio_blob.clone();
            let on_status = on_status.clone();
            let on_voices = on_voices.clone();

            if voice_name.trim().is_empty() {
                on_status.emit(("Please enter a voice name".into(), StatusLevel::Warning));
                return;
            }

            let audio = match audio_blob {
                Some(ref a) => a.clone(),
                None => {
                    on_status.emit((
                        "Please record or upload audio first".into(),
                        StatusLevel::Warning,
                    ));
                    return;
                }
            };

            on_status.emit(("Importing voice...".into(), StatusLevel::Info));

            spawn_local(async move {
                match import_voice(&voice_name, &transcript, &audio).await {
                    Ok(()) => {
                        on_status.emit((
                            format!("Voice '{}' imported successfully!", voice_name),
                            StatusLevel::Success,
                        ));
                        // Refresh voice list
                        if let Ok(voices) = fetch_voices().await {
                            on_voices.emit(voices);
                        }
                    }
                    Err(e) => {
                        on_status.emit((format!("Import failed: {}", e), StatusLevel::Error));
                    }
                }
            });
        })
    };

    let drop_zone_class = if *is_dragging {
        "drop-zone dragging"
    } else if props.training.audio_blob.is_some() {
        "drop-zone has-audio"
    } else {
        "drop-zone"
    };

    let record_btn_class = if props.training.is_recording {
        "btn btn-record recording"
    } else {
        "btn btn-record"
    };

    let has_audio = props.training.audio_blob.is_some();
    let can_import = has_audio && !props.training.voice_name.trim().is_empty();

    html! {
        <div class="training-form">
            <div class="form-group">
                <label>{"Voice Name:"}</label>
                <input
                    type="text"
                    value={props.training.voice_name.clone()}
                    placeholder="e.g., my_voice"
                    oninput={on_name_change}
                />
            </div>

            <div class="form-group">
                <label>{"Reference Audio (3-10 seconds):"}</label>
                <div class="audio-input-row">
                    <div
                        class={drop_zone_class}
                        ondragover={on_drag_over}
                        ondragleave={on_drag_leave}
                        ondrop={on_drop}
                    >
                        {if has_audio {
                            html! { <span class="drop-text">{"Audio loaded - drag new file to replace"}</span> }
                        } else {
                            html! { <span class="drop-text">{"Drag audio here or click to upload"}</span> }
                        }}
                        <input
                            type="file"
                            accept="audio/*"
                            class="file-input-hidden"
                            onchange={on_file_select}
                        />
                    </div>
                    <button class={record_btn_class} onclick={on_record_toggle}>
                        {if props.training.is_recording { "Stop" } else { "Record" }}
                    </button>
                </div>
                {if let Some(url) = (*audio_preview_url).as_ref() {
                    html! {
                        <audio controls=true src={url.clone()} class="audio-preview">
                            {"Your browser does not support audio."}
                        </audio>
                    }
                } else {
                    html! {}
                }}
            </div>

            <div class="form-group">
                <label>{"Transcript (optional, improves quality):"}</label>
                <textarea
                    class="transcript-input"
                    rows="2"
                    value={props.training.transcript.clone()}
                    placeholder="Text spoken in the audio..."
                    oninput={on_transcript_input}
                />
            </div>

            <button
                class="btn btn-primary"
                onclick={on_import}
                disabled={!can_import}
            >
                {"Import Voice"}
            </button>
        </div>
    }
}

/// Handle file from input or drop.
fn handle_file(
    file: File,
    on_audio: Callback<Option<Vec<u8>>>,
    on_status: Callback<(String, StatusLevel)>,
    preview_url: UseStateHandle<Option<String>>,
) {
    let file_type = file.type_();
    if !file_type.starts_with("audio/") {
        on_status.emit(("Please select an audio file".into(), StatusLevel::Warning));
        return;
    }

    on_status.emit((format!("Loading {}...", file.name()), StatusLevel::Info));

    // Create preview URL
    if let Ok(url) = Url::create_object_url_with_blob(&file) {
        preview_url.set(Some(url));
    }

    // Read file bytes
    let reader = FileReader::new().unwrap();
    let reader_clone = reader.clone();

    let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
        if let Ok(result) = reader_clone.result() {
            let array_buffer = result.dyn_into::<js_sys::ArrayBuffer>().unwrap();
            let uint8_array = Uint8Array::new(&array_buffer);
            let bytes = uint8_array.to_vec();
            on_audio.emit(Some(bytes));
            on_status.emit(("Audio loaded".into(), StatusLevel::Success));
        }
    }) as Box<dyn FnMut(_)>);

    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
    onload.forget();

    let _ = reader.read_as_array_buffer(&file);
}

/// Start microphone recording, returns MediaRecorder.
async fn start_recording(
    on_audio: Callback<Option<Vec<u8>>>,
    on_status: Callback<(String, StatusLevel)>,
    preview_url: UseStateHandle<Option<String>>,
) -> Result<MediaRecorder, String> {
    let window = web_sys::window().ok_or("No window")?;
    let navigator = window.navigator();
    let media_devices = navigator.media_devices().map_err(|_| "No media devices")?;

    let constraints = web_sys::MediaStreamConstraints::new();
    constraints.set_audio(&JsValue::TRUE);

    let promise = media_devices
        .get_user_media_with_constraints(&constraints)
        .map_err(|_| "Failed to get user media")?;

    let stream: MediaStream = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|_| "Failed to access microphone")?
        .dyn_into()
        .map_err(|_| "Invalid media stream")?;

    let options = MediaRecorderOptions::new();
    options.set_mime_type("audio/webm");

    let recorder =
        MediaRecorder::new_with_media_stream_and_media_recorder_options(&stream, &options)
            .map_err(|_| "Failed to create recorder")?;

    let chunks: std::rc::Rc<std::cell::RefCell<Vec<Blob>>> =
        std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));

    // ondataavailable
    let chunks_clone = chunks.clone();
    let ondataavailable = Closure::wrap(Box::new(move |e: web_sys::BlobEvent| {
        if let Some(blob) = e.data() {
            chunks_clone.borrow_mut().push(blob);
        }
    }) as Box<dyn FnMut(_)>);
    recorder.set_ondataavailable(Some(ondataavailable.as_ref().unchecked_ref()));
    ondataavailable.forget();

    // onstop
    let on_audio_clone = on_audio;
    let on_status_clone = on_status;
    let preview_clone = preview_url;
    let onstop = Closure::wrap(Box::new(move |_: web_sys::Event| {
        let borrowed = chunks.borrow();
        if borrowed.is_empty() {
            on_status_clone.emit(("No audio recorded".into(), StatusLevel::Warning));
            return;
        }

        let array = Array::new();
        for chunk in borrowed.iter() {
            array.push(chunk);
        }

        let options = BlobPropertyBag::new();
        options.set_type("audio/webm");

        if let Ok(blob) = Blob::new_with_blob_sequence_and_options(&array, &options) {
            // Preview URL
            if let Ok(url) = Url::create_object_url_with_blob(&blob) {
                preview_clone.set(Some(url));
            }

            // Read blob to bytes
            let reader = FileReader::new().unwrap();
            let reader_clone = reader.clone();
            let on_audio = on_audio_clone.clone();
            let on_status = on_status_clone.clone();

            let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(result) = reader_clone.result() {
                    let array_buffer = result.dyn_into::<js_sys::ArrayBuffer>().unwrap();
                    let uint8_array = Uint8Array::new(&array_buffer);
                    on_audio.emit(Some(uint8_array.to_vec()));
                    on_status.emit(("Recording saved".into(), StatusLevel::Success));
                }
            }) as Box<dyn FnMut(_)>);

            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            onload.forget();
            let _ = reader.read_as_array_buffer(&blob);
        }
    }) as Box<dyn FnMut(_)>);
    recorder.set_onstop(Some(onstop.as_ref().unchecked_ref()));
    onstop.forget();

    recorder.start().map_err(|_| "Failed to start recording")?;
    Ok(recorder)
}

/// Import voice to server.
async fn import_voice(name: &str, transcript: &str, audio: &[u8]) -> Result<(), String> {
    // Create form data with multipart
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let mut body = Vec::new();

    // Voice name field
    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"voice_name\"\r\n\r\n");
    body.extend_from_slice(name.as_bytes());
    body.extend_from_slice(b"\r\n");

    // Transcript field
    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"transcript\"\r\n\r\n");
    body.extend_from_slice(transcript.as_bytes());
    body.extend_from_slice(b"\r\n");

    // Audio file
    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(
        b"Content-Disposition: form-data; name=\"audio\"; filename=\"reference.wav\"\r\n",
    );
    body.extend_from_slice(b"Content-Type: audio/wav\r\n\r\n");
    body.extend_from_slice(audio);
    body.extend_from_slice(b"\r\n");

    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

    let response = Request::post("/api/voice-import")
        .header(
            "Content-Type",
            &format!("multipart/form-data; boundary={}", boundary),
        )
        .body(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        let err = response.text().await.unwrap_or_default();
        return Err(format!("Server error: {}", err));
    }

    Ok(())
}

/// Fetch available voices from server.
async fn fetch_voices() -> Result<Vec<String>, String> {
    let response = Request::get("/api/voices")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err("Failed to fetch voices".into());
    }

    let json: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

    let voices = json
        .get("voices")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    Ok(voices)
}
