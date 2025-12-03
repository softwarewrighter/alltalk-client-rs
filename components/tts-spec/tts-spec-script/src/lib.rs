//! Script parsing and validation for TTS specifications.
//!
//! This crate provides functions to parse [`Script`] from YAML or JSON format,
//! validate script structure, and serialize scripts back to these formats.
//!
//! # Example
//!
//! ```
//! use tts_spec_script::{parse, validate};
//!
//! let yaml = r#"
//! sample_rate: 24000
//! segments:
//!   - id: intro
//!     speaker: mike
//!     engine: parler
//!     text: "Hello, world!"
//! "#;
//!
//! let script = parse::from_yaml_str(yaml).unwrap();
//! validate::validate(&script).unwrap();
//! ```

pub mod format;
pub mod parse;
pub mod validate;
