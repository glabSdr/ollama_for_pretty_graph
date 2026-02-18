use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Msg {
    role: String,
    content: String
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