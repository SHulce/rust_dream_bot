extern crate serde;
extern crate serde_json;

use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

const _CONFIG_FILENAME: &str = "resources/botconfig.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub bot_token: String,
    pub giphy_api_key: String,
    pub next_session: String,
    pub files: HashMap<String, String>,
}

impl Config {

    pub fn new() -> Config {
        let config_content = fs::read_to_string(_CONFIG_FILENAME).expect("Failed to open config");
        serde_json::from_str(&config_content).unwrap()
    }

    pub fn save_config(&self) -> std::io::Result<()> {
        let config_json = serde_json::to_string(&self).unwrap();
        fs::write(_CONFIG_FILENAME, config_json)?;  
        Ok(())
    }
}