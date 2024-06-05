use std::env;
use std::path::{PathBuf};

pub fn get_project_root_path() -> PathBuf {
    let str = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR is not set");
    PathBuf::from(str)
}