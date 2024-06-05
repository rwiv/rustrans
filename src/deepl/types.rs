use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub text: Vec<String>,
    pub target_lang: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub translations: Vec<Translation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Translation {
    pub detected_source_language: String,
    pub text: String,
}
