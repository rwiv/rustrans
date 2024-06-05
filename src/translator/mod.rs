use std::collections::HashMap;
use std::mem;
use anyhow::{anyhow, Result};
use futures::future::join_all;
use crate::utils::list::split_vec_move;

pub trait Client {
    fn translate(&self, str: &str) -> impl std::future::Future<Output = Result<String>> + Send;
}

pub struct Translator<T: Client> {
    client: T,
}

impl <T: Client> Translator<T> {

    async fn translate(
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
    use crate::deepl::DeeplClient;

    #[tokio::test]
    async fn test_translate_parallel() {
        let client = DeeplClient {};
        let translator = Translator{ client };

        let mut map = HashMap::new();
        map.insert(3, "hello world!");
        map.insert(7, "hello world~");

        let results = translator.translate(map, 3).await;
        for result in results {
            println!("{:?}", result);
        }
    }
}