use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::utils::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Conf {
    pub deepl: DeeplConf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeeplConf {
    pub api_key: String,
}

pub fn read_conf() -> Result<Conf> {
    let project_root = path::get_project_root_path();
    let file_path = project_root.join("configs").join("conf.json");
    json::read_json_from_file::<Conf>(&file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_conf() {
        let conf = read_conf();
        println!("{:?}", conf);
    }
}
