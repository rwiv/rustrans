mod types;

pub use types::Request;
pub use types::Response;

use anyhow::{anyhow, Result};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use crate::common::conf::read_conf;
use crate::translator::Client;

pub struct DeeplClient;

impl Client for DeeplClient {
    async fn translate(&self, str: &str) -> Result<String> {
        let req = Request::new_one_ko(str);
        let res = DeeplClient::translates(req).await?;
        let mut translations = res.translations;
        if let Some(t) = translations.pop() {
            Ok(t.text)
        } else {
            Err(anyhow!("len is not 1"))
        }
    }
}

impl DeeplClient {
    async fn translates(req: Request<'_>) -> Result<Response> {
        let api_key = read_conf()?.deepl.api_key;

        let url = "https://api-free.deepl.com/v2/translate";
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("DeepL-Auth-Key {}", &api_key))?);

        let res = client.post(url)
            .headers(headers)
            .json(&req)
            .send().await?
            .error_for_status()?
            .json::<Response>().await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_translate() {
        let client = DeeplClient{};
        let result = client.translate("hello world").await;
        match result {
            Ok(res) => println!("{:?}", res),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}