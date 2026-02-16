#![doc = include_str!("../README.md")]

mod trait_impl;
mod traits;
mod execute;
mod client;
mod constants;
mod config;

pub use traits::OllamaForPrettyGraph;
pub use constants::{OLLAMA_DEFAULT_URL, OLLAMA_DEFAULT_PORT};
pub use config::Config;