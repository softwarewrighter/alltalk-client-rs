//! Reusable UI widgets for the TTS web application.

use crate::state::{EngineInfo, ModelKind, PlayerState, StatusLevel, StatusMessage, TabKind};
use yew::prelude::*;

/// Properties for Header component.
#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub active_model: ModelKind,
    pub available_engines: Vec<EngineInfo>,
    pub engines_loaded: bool,
    pub on_model_change: Callback<ModelKind>,
}

/// Header component with title and model selector.
#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let on_change = {
        let cb = props.on_model_change.clone();
        let engines = props.available_engines.clone();
        Callback::from(move |e: Event| {
            let target: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value = target.value();
            // Find the engine and convert to ModelKind
            if let Some(engine) = engines.iter().find(|e| e.id == value)
                && let Some(model) = engine.to_model_kind()
            {
                cb.emit(model);
            }
        })
    };

    let selected = props.active_model.as_str();

    // Build options from available engines
    let options = if props.engines_loaded && !props.available_engines.is_empty() {
        props
            .available_engines
            .iter()
            .map(|engine| {
                let is_selected = engine.id == selected;
                let label = if !engine.commercial {
                    format!("{} (non-commercial)", engine.name)
                } else {
                    engine.name.clone()
                };
                html! {
                    <option value={engine.id.clone()} selected={is_selected}>{label}</option>
                }
            })
            .collect::<Vec<_>>()
    } else {
        // Loading state or no engines available
        vec![html! { <option disabled=true>{"Loading..."}</option> }]
    };

    html! {
        <header class="header">
            <h1>{"TTS Control Plane"}</h1>
            <div class="model-selector">
                <label>{"Model:"}</label>
                <select onchange={on_change} disabled={!props.engines_loaded}>
                    {for options}
                </select>
            </div>
        </header>
    }
}

/// Properties for TabBar component.
#[derive(Properties, PartialEq)]
pub struct TabBarProps {
    pub active_tab: TabKind,
    pub on_tab_change: Callback<TabKind>,
}

/// Tab navigation bar.
#[function_component(TabBar)]
pub fn tab_bar(props: &TabBarProps) -> Html {
    let make_tab = |tab: TabKind, label: &'static str| {
        let active = props.active_tab == tab;
        let class = if active { "tab active" } else { "tab" };
        let cb = props.on_tab_change.clone();
        let onclick = Callback::from(move |_| cb.emit(tab.clone()));
        html! { <button class={class} onclick={onclick}>{label}</button> }
    };
    html! {
        <nav class="tab-bar">
            {make_tab(TabKind::Settings, "Settings")}
            {make_tab(TabKind::Generate, "Generate")}
        </nav>
    }
}

/// Properties for StatusBar component.
#[derive(Properties, PartialEq)]
pub struct StatusBarProps {
    pub message: StatusMessage,
}

/// Status bar for displaying messages.
#[function_component(StatusBar)]
pub fn status_bar(props: &StatusBarProps) -> Html {
    let class = match props.message.level {
        StatusLevel::Info => "status-bar info",
        StatusLevel::Success => "status-bar success",
        StatusLevel::Warning => "status-bar warning",
        StatusLevel::Error => "status-bar error",
    };
    html! { <div class={class}>{&props.message.text}</div> }
}

/// Footer component with copyright and build info.
#[function_component(Footer)]
pub fn footer() -> Html {
    let build_info = format!(
        "Built on {} at {} for {}",
        env!("BUILD_HOST"),
        env!("BUILD_TIME"),
        env!("GIT_SHA")
    );
    html! {
        <footer class="footer">
            <div class="footer-main">
                {"Copyright (c) 2025 Michael A. Wright • "}
                <a href="https://github.com/softwarewrighter/alltalk-client-rs/blob/main/LICENSE">{"License: MIT"}</a>
                {" • "}
                <a href="https://github.com/softwarewrighter/alltalk-client-rs">{"Repository"}</a>
            </div>
            <div class="footer-build">{build_info}</div>
        </footer>
    }
}

/// Properties for AudioPlayer component.
#[derive(Properties, PartialEq)]
pub struct AudioPlayerProps {
    pub player: PlayerState,
}

/// Audio player with HTML5 audio element.
#[function_component(AudioPlayer)]
pub fn audio_player(props: &AudioPlayerProps) -> Html {
    let has_audio = props.player.audio_url.is_some();

    html! {
        <div class={classes!("audio-player", (!has_audio).then_some("disabled"))}>
            {if let Some(url) = &props.player.audio_url {
                html! {
                    <div class="audio-container">
                        <audio controls=true autoplay=true src={url.clone()}>
                            {"Your browser does not support the audio element."}
                        </audio>
                        <a class="btn btn-secondary download-btn" href={url.clone()} download="tts-output.wav">
                            {"Download WAV"}
                        </a>
                    </div>
                }
            } else {
                html! {
                    <div class="audio-placeholder">
                        {"Generate audio to play it here"}
                    </div>
                }
            }}
        </div>
    }
}

/// Properties for NonCommercialWarning component.
#[derive(Properties, PartialEq)]
pub struct NonCommercialWarningProps {
    pub model: ModelKind,
}

/// Warning banner for non-commercial use models.
#[function_component(NonCommercialWarning)]
pub fn non_commercial_warning(props: &NonCommercialWarningProps) -> Html {
    if !props.model.is_non_commercial() {
        return html! {};
    }

    html! {
        <div class="warning-banner non-commercial">
            <span class="warning-icon">{"⚠️"}</span>
            <span class="warning-text">
                <strong>{"NON-COMMERCIAL USE ONLY"}</strong>
                {" — XTTS is licensed under CPML which prohibits commercial use. "}
                {"For commercial projects, use GPT-SoVITS (MIT) or Dia (Apache 2.0)."}
            </span>
        </div>
    }
}
