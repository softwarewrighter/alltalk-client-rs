//! Reusable UI widgets for the TTS web application.

use crate::state::{StatusLevel, StatusMessage};
use yew::prelude::*;

/// Header component with title and navigation controls.
#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="header">
            <h1>{"TTS Control Plane"}</h1>
            <nav class="nav">
                <button class="btn">{"Load Script"}</button>
                <button class="btn">{"Save Script"}</button>
                <button class="btn btn-primary">{"Render Audio"}</button>
            </nav>
        </header>
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
    html! {
        <footer class="footer">
            <div class="footer-main">
                {"Copyright (c) 2025 Software Wrighter LLC • "}
                <a href="https://github.com/softwarewrighter/alltalk-client-rs/blob/main/LICENSE">{"License: MIT OR Apache-2.0"}</a>
                {" • "}
                <a href="https://github.com/softwarewrighter/alltalk-client-rs">{"Repository"}</a>
            </div>
            <div class="footer-build">
                {"Build Host: wasm32 • Build Commit: dev • Build Time: runtime"}
            </div>
        </footer>
    }
}
