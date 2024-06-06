use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::de::DeserializeOwned;
use anyhow::{Result};

pub fn read_from_file<T: DeserializeOwned>(file_path: &Path) -> Result<T> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let result = serde_json::from_reader(reader)?;
    Ok(result)
}
