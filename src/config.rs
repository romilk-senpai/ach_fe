use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, Deserialize, Default)]
pub struct Config {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
}

const CONFIG: &str = include_str!("../config/config.txt");

#[hook]
pub fn use_config() -> Config {
    let config = Config {
        base_url: CONFIG.split("=").nth(1).unwrap().to_string(),
    };

    config
}
