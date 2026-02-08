#![doc = include_str!("../README.md")]

mod trait_impl;
mod traits;
mod execute;
mod client;
mod constants;

pub use traits::*;
pub use constants::OLLAMA_DEFAULT_URL;

