//! Settings tab with model-specific configuration and training.

use crate::state::ModelKind;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Properties for SettingsTab component.
#[derive(Properties, PartialEq)]
pub struct SettingsTabProps {
    pub model: ModelKind,
    pub api_url: String,
    pub on_api_url_change: Callback<String>,
}

/// Settings tab with model-specific options and voice training.
#[function_component(SettingsTab)]
pub fn settings_tab(props: &SettingsTabProps) -> Html {
    let supports_training = matches!(props.model, ModelKind::Xtts);
    let model_name = match props.model {
        ModelKind::Parler => "Parler TTS",
        ModelKind::Piper => "Piper TTS",
        ModelKind::Xtts => "XTTS",
    };

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

    html! {
        <div class="settings-tab">
            <h2>{format!("{} Settings", model_name)}</h2>
            <div class="form-group">
                <label>{"AllTalk API URL:"}</label>
                <input
                    type="text"
                    value={props.api_url.clone()}
                    placeholder="http://localhost:7851"
                    oninput={on_url_change}
                />
                <span class="field-hint">{"Enter the URL of your TTS backend (e.g., http://curiosity:7851)"}</span>
            </div>
            <div class={classes!("training-section", (!supports_training).then_some("disabled"))}>
                <h3>{"Voice Training"}</h3>
                {render_training_hint(supports_training, model_name)}
                {render_training_form(supports_training)}
            </div>
        </div>
    }
}

fn render_training_hint(supports: bool, model: &str) -> Html {
    if supports {
        html! { <p class="tab-description">{"Import reference audio to create custom voices."}</p> }
    } else {
        html! { <p class="tab-description disabled-hint">{format!("{} does not support voice training.", model)}</p> }
    }
}

fn render_training_form(enabled: bool) -> Html {
    html! {
        <div class="training-form">
            <div class="form-group">
                <label>{"Voice Name:"}</label>
                <input type="text" disabled={!enabled} placeholder="Enter voice name..." />
            </div>
            <div class="form-group">
                <label>{"Reference Audio:"}</label>
                <input type="file" accept="audio/*" disabled={!enabled} />
                <span class="file-hint">{"WAV, MP3, or OGG (3-10s)"}</span>
            </div>
            <div class="form-group">
                <label>{"Transcript:"}</label>
                <textarea rows="2" disabled={!enabled} placeholder="Text spoken in audio..." />
            </div>
            <button class="btn btn-primary" disabled={!enabled}>{"Import Voice"}</button>
        </div>
    }
}
