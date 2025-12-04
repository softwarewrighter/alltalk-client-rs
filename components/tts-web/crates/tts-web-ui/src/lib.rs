//! Yew-based web UI for TTS control plane.
//!
//! All domain and presentation logic is in Rust. This crate compiles to WASM
//! and runs in the browser. No TypeScript or Python - only minimal JS glue.

mod app;
mod generate;
pub mod state;
mod training;
mod widgets;

pub use app::App;
