use serde::Deserialize;
use std::fs;
use toml;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub author: String,
    pub auth_url: String,
    pub token_url: String,
    pub blog_id: u32,
    pub blog_url: String,
    pub client_id: u32,
    pub client_secret: String,
    pub token: String,
}

pub fn get_config(filename: &str) -> Config {
    let config_file = fs::read_to_string(filename).unwrap();
    return toml::from_str(&config_file).unwrap();
}
