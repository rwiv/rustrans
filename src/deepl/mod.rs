mod types;

pub use types::Request;
pub use types::Response;
use crate::common::conf::read_conf;

use anyhow::Result;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};

async fn translate(req: Request) -> Result<Response> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let req = Request {
            text: vec!(String::from("hello world")),
            target_lang: String::from("ko")
        };
        let result = translate(req).await;
        match result {
            Ok(res) => println!("{:?}", res),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
