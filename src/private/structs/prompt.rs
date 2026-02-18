use serde::Serialize;
use crate::Config;
use crate::private::structs::Msg;

#[derive(Debug, Serialize)]
pub struct Prompt {
    pub model: String,
    pub messages: Vec<Msg>,
    pub format: Option<String>,
    pub options: Option<Config>,
    pub stream: bool
}
