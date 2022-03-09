use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SmsParam {
    pub name: String,
    pub code: String,
}