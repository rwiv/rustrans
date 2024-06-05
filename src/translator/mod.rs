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
    pub async fn translate<'a>(&'a self, strings: &'a Vec<String>, size: usize) -> Vec<(&str, String)> {
        let mut targets: HashMap<usize, &str> = HashMap::new();
        for (idx, value) in strings.iter().enumerate() {
            if !value.trim().is_empty() {
                targets.insert(idx, value);
            }
        }
        let mut map = self.translate_map(targets, size).await;
        let mut result: Vec<(&str, String)> = Vec::new();
        for (idx, org) in strings.iter().enumerate() {
            if let Some(ret) = map.remove(&idx) {
                if let Ok(s) = ret {
                    result.push((org, s));
                } else {
                    result.push((org, String::from("")));
                }
            } else {
                result.push((org, String::from("")));
            }
        }
        result
    }

    async fn translate_map(
        &self, map: HashMap<usize, &str>, size: usize
    ) -> HashMap<usize, Result<String>> {
        let keys: Vec<usize> = map.keys().cloned().collect();
        let mut strings: Vec<&str> = map.values().cloned().collect();

        // parallel request
        let mut ts_strings: Vec<Result<String>> = Vec::new();
        for sub in split_vec_move(&mut strings, size) {
            let mut tasks = Vec::new();
            for str in sub {
                tasks.push(self.client.translate(str))
            }
            let sub_results = join_all(tasks).await;
            for seg_result in sub_results {
                ts_strings.push(seg_result);
            }
        }

        // mapping key-value
        let mut result = HashMap::new();
        for (idx, key) in keys.into_iter().enumerate() {
            if let Some(elem) = ts_strings.get_mut(idx) {
                let val = mem::replace(elem, Ok(String::from("")));
                result.insert(key, val);
            } else {
                result.insert(key, Err(anyhow!("not found value")));
            }
        }
        result
    }

    // async fn translate_vec(&self, mut strings: Vec<&str>, size: usize) -> Vec<Result<String>> {
    //     let mut result = Vec::new();
    //     for sub in split_vec_move(&mut strings, size) {
    //         let mut tasks = Vec::new();
    //         for str in sub {
    //             tasks.push(self.client.translate(str))
    //         }
    //         let mut sub_results = join_all(tasks).await;
    //         result.append(&mut sub_results);
    //     }
    //     result
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deepl::DeeplClient;

    #[tokio::test]
    async fn test_translate_parallel() {
        let client = DeeplClient {};
        let translator = Translator{ client };

        let mut vec = vec!(
            String::from("hello world!"),
            String::from("     "),
            String::from("hello world~"),
            String::from("hello world!"),
            String::from(""),
        );
        let results = translator.translate(&mut vec, 3).await;
        for result in results {
            println!("{:?}", result);
        }
    }
}
