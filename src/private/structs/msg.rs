use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Msg {
    pub role: String,
    pub content: String
}

impl Msg {
    pub fn system(system: String) -> Self {
        Self {
            role: "system".to_owned(),
            content: system
        }
    }

    pub fn user(prompt: String) -> Self {
        Self {
            role: "user".to_string(),
            content: prompt
        }
    }
}