use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request<'a> {
    pub text: Vec<&'a str>,
    pub target_lang: &'a str,
}

impl<'a> Request<'a> {
    pub fn new_one_ko(str: &'a str) -> Self {
        Request {
            text: vec!(str),
            target_lang: "ko",
        }
    }
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
