use std::collections::HashMap;
use std::mem;
use anyhow::{anyhow, Result};
use futures::future::join_all;
use crate::utils::list::split_vec_move;

pub mod deepl;

pub trait Client {
    fn translate(&self, str: &str) -> impl std::future::Future<Output = Result<String>> + Send;
}

pub struct Translator<T: Client> {
    pub client: T,
}

impl <T: Client> Translator<T> {

    pub async fn translate<'a>(
        &'a self, inputs: &'a Vec<&str>, size: usize,
    ) -> Vec<(&str, String)> {
        let mut targets = HashMap::new();
        for (idx, value) in inputs.iter().enumerate() {
            if !value.trim().is_empty() {
                targets.insert(idx, *value);
            }
        }
        let mut translated_map = self.translate_map(targets, size).await;
        let mut result = Vec::new();
        for (idx, org) in inputs.iter().enumerate() {
            if let Some(ret) = translated_map.remove(&idx) {
                if let Ok(s) = ret {
                    result.push((*org, s));
                } else {
                    result.push((*org, String::from("")));
                }
            } else {
                result.push((*org, String::from("")));
            }
        }
        result
    }

    async fn translate_map(
        &self, input_map: HashMap<usize, &str>, size: usize,
    ) -> HashMap<usize, Result<String>> {
        let input_keys: Vec<usize> = input_map.keys().cloned().collect();
        let mut input_values: Vec<&str> = input_map.values().cloned().collect();

        // parallel request
        let mut translated: Vec<Result<String>> = Vec::new();
        for sub in split_vec_move(&mut input_values, size) {
            let mut tasks = Vec::new();
            for str in sub {
                tasks.push(self.client.translate(str))
            }
            let sub_results = join_all(tasks).await;
            for seg_result in sub_results {
                translated.push(seg_result);
            }
        }

        // mapping key-value
        let mut result = HashMap::new();
        for (idx, key) in input_keys.into_iter().enumerate() {
            if let Some(elem) = translated.get_mut(idx) {
                let val = mem::replace(elem, Ok(String::from("")));
                result.insert(key, val);
            } else {
                result.insert(key, Err(anyhow!("not found value")));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deepl::DeeplClient;

    #[tokio::test]
    async fn test_translate() {
        let client = DeeplClient {};
        let translator = Translator{ client };

        let mut vec = vec!(
            "hello world!",
            "          ",
            "hello world~",
            "hello world!",
            "",
        );
        let results = translator.translate(&mut vec, 3).await;
        for result in results {
            println!("{:?}", result);
        }
    }
}
