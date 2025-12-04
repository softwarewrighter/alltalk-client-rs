//! Main Yew application component.

use crate::generate::GenerateTab;
use crate::state::{AppState, ModelKind, StatusLevel, TabKind};
use crate::training::SettingsTab;
use crate::widgets::{AudioPlayer, Footer, Header, StatusBar, TabBar};
use yew::prelude::*;

/// Root application component.
#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(AppState::new);

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

    let tab_content = match state.active_tab {
        TabKind::Settings => html! {
            <SettingsTab
                model={state.active_model.clone()}
                api_url={state.api_url.clone()}
                on_api_url_change={on_api_url_change}
            />
        },
        TabKind::Generate => html! {
            <GenerateTab
                model={state.active_model.clone()}
                parler={state.parler.clone()}
                piper={state.piper.clone()}
                api_url={state.api_url.clone()}
                on_parler_text_change={on_parler_text_change}
                on_parler_speaker_change={on_parler_speaker_change}
                on_parler_description_change={on_parler_description_change}
                on_piper_text_change={on_piper_text_change}
                on_piper_voice_change={on_piper_voice_change}
                on_audio_received={on_audio_received}
                on_status_change={on_status_change}
            />
        },
    };

    html! {
        <div class="app">
            <Header active_model={state.active_model.clone()} on_model_change={on_model_change} />
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
