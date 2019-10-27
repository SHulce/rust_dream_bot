extern crate serde_json;

use std::fs;

pub struct Config {
    pub bot_token: String,
    pub giphy_api_key: String,
}

pub fn initialize_config() -> Config {
    let config_filename = "botconfig.json";
    let config_content = fs::read_to_string(config_filename).expect("Failed to open config");
    parse_config(config_content)
}

fn parse_config(contents: String) -> Config {
    let data: serde_json::value::Value = serde_json::from_str(&contents).unwrap();
    //eprintln!("Data in parse_config: {:?}", data);
    Config { bot_token: String::from(data["bot_token"].as_str().unwrap()), giphy_api_key: String::from(data["giphy_api_key"].as_str().unwrap())}
}