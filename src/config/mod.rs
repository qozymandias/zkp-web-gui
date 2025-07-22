use serde::Deserialize;

pub static CONFIG: once_cell::sync::Lazy<Config> =
    once_cell::sync::Lazy::new(|| load_config().expect("Should load config"));

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api: ApiConfig,
}

impl Config {
    pub fn into_href(&self, path : Vec<&str>) -> String {
        format!("{}/{}", self.api.url, path.join("/"))
    }
}

const CONFIG_TOML: &str = include_str!("../../config.toml");

pub fn load_config() -> anyhow::Result<Config> {
    let config = toml::from_str::<Config>(CONFIG_TOML)?;
    Ok(config)
}
