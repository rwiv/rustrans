use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::de::DeserializeOwned;

#[derive(thiserror::Error, Debug)]
pub enum JsonError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse Error: {0}")]
    Parse(#[from] serde_json::error::Error),
}

pub fn read_from_file<T: DeserializeOwned>(file_path: &Path) -> Result<T, JsonError> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let result = serde_json::from_reader(reader)?;
    Ok(result)
}
