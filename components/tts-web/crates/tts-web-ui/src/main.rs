//! WASM entry point for TTS web UI.

use tts_web_ui::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
