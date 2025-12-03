//! Main Yew application component.

use crate::editor::ScriptEditor;
use crate::state::{default_segment, AppState, StatusLevel, StatusMessage};
use crate::widgets::{Footer, Header, StatusBar};
use yew::prelude::*;

/// Root application component.
#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(AppState::new);

    let on_add_segment = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut new_state = (*state).clone();
            let id = format!("segment_{}", new_state.script.segments.len() + 1);
            new_state.add_segment(default_segment(&id));
            new_state.status = StatusMessage {
                text: format!("Added segment: {id}"),
                level: StatusLevel::Success,
            };
            state.set(new_state);
        })
    };

    html! {
        <div class="app">
            <Header />
            <main class="main-content">
                <ScriptEditor
                    script={state.script.clone()}
                    selected={state.selected_segment.clone()}
                    on_add={on_add_segment}
                />
                <StatusBar message={state.status.clone()} />
            </main>
            <Footer />
        </div>
    }
}
