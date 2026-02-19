use serde::Deserialize;
use crate::private::structs::Msg;

#[derive(Debug, Deserialize)]
pub struct ModelResponse {
    pub message: Msg
}