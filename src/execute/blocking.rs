use serde::{Deserialize, Serialize};
use crate::client::BLOCKING_CLIENT;
use crate::config::Config;

#[derive(Serialize)]
struct Prompt {
    model: String,
    system: Option<String>,
    prompt: String,
    stream: bool,
    format: Option<String>,
    config: Option<Config>,
}

#[derive(Deserialize)]
struct ModelResponse {
    pub response: String
}


pub fn execute_blocking(base_url: &str, model: String, system: Option<String>, prompt: String, format: Option<String>, config: Option<Config>) -> String {

    let body = Prompt {
        model,
        system,
        prompt,
        stream: false,
        format,
        config,
    };

    let body = serde_json::to_string(&body).unwrap();

    let res = BLOCKING_CLIENT
        .post(format!("{}/api/generate", base_url))
        .body(body)
        .send()
        .expect("Failed to send request to ollama")
        .text()
        .expect("Failed to read response body");


    let map: ModelResponse = serde_json::from_str(&res).unwrap_or(ModelResponse { response: "".to_string() });
    map.response
}