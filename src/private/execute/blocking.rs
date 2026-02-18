use crate::Config;
use crate::private::client::BLOCKING_CLIENT;
use crate::private::structs::{ModelResponse, Msg, Prompt};

pub fn execute_blocking(base_url: &str, model: String, system: Option<String>, prompt: String, format: Option<String>, config: Option<Config>) -> String {
    let mut messages = Vec::new();
    if system.is_some() { messages.push(Msg::system(system.unwrap())); }
    messages.push(Msg::user(prompt.clone()));

    let url = format!("{}/api/chat", base_url);
    let body = Prompt {
        model,
        messages,
        format,
        options: config,
        stream: false,
    };

    
    println!("Sending request to {url} with body: \n{:#?}", body);
    let body = serde_json::to_string(&body).unwrap();

    let res = BLOCKING_CLIENT
        .post(url)
        .body(body)
        .send()
        .expect("Failed to send request to ollama")
        .text()
        .expect("Failed to read response body");


    let map: ModelResponse = serde_json::from_str(&res).unwrap_or(ModelResponse { response: "".to_string() });
    map.response
}