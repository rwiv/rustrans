mod types;

pub use types::Request;
pub use types::Response;

use anyhow::Result;
use futures::future::join_all;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use crate::utils::list::split_vec_move;
use crate::common::conf::read_conf;

async fn translate_parallel(mut strings: Vec<&str>, size: usize) -> Vec<Result<Response>> {
    let mut result = Vec::new();
    for sub in split_vec_move(&mut strings, size) {
        let mut tasks = Vec::new();
        for str in sub {
            tasks.push(translate(Request::new_one_ko(str)))
        }
        let mut sub_results = join_all(tasks).await;
        result.append(&mut sub_results);
    }
    result
}

async fn translate(req: Request<'_>) -> Result<Response> {
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
    async fn test_translate() {
        let req = Request {
            text: vec!("hello world"),
            target_lang: "ko"
        };
        let result = translate(req).await;
        match result {
            Ok(res) => println!("{:?}", res),
            Err(e) => eprintln!("{:?}", e),
        }
    }

    #[tokio::test]
    async fn test_translate_parallel() {
        let strings = vec![
            "hello world!", "hello world~", "hello world..",
            "hello world!", "hello world~", "hello world..",
            "hello world!", "hello world~",
        ];
        let results = translate_parallel(strings, 3).await;
        for result in results {
            println!("{:?}", result);
        }
    }
}
