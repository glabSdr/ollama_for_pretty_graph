use crate::Config;
use crate::private::client::BLOCKING_CLIENT;
use crate::private::structs::{ModelResponse, Msg, Prompt};

pub fn execute_blocking(base_url: &str, model: String, system: Option<String>, prompt: String, format: Option<String>, config: Option<Config>) -> String {
    let mut messages = Vec::new();
    if system.is_some() { messages.push(Msg::system(system.unwrap())); }
    messages.push(Msg::user(prompt.clone()));

    let body = Prompt {
        model,
        messages,
        format,
        config,
        stream: false,
    };

    let body = serde_json::to_string(&body).unwrap();

    let res = BLOCKING_CLIENT
        .post(format!("{}/api/chat", base_url))
        .body(body)
        .send()
        .expect("Failed to send request to ollama")
        .text()
        .expect("Failed to read response body");


    let map: ModelResponse = serde_json::from_str(&res).unwrap_or(ModelResponse { response: "".to_string() });
    map.response
}