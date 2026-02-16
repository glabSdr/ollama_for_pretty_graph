#![doc = include_str!("../README.md")]

mod private;
mod public;

pub use public::{Config, OllamaForPrettyGraph, OLLAMA_DEFAULT_PORT, OLLAMA_DEFAULT_URL};
