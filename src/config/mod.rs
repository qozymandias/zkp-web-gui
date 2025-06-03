use once_cell::sync::OnceCell;
use serde::Deserialize;

pub static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn set_config(config_in: Config) {
    CONFIG.set(config_in).expect("Should set static config variable");
}

pub fn get_config() -> &'static Config {
    CONFIG.get().expect("Should get static config variable")
}

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api: ApiConfig,
}

const CONFIG_TOML: &str = include_str!("../../config.toml");

pub fn load_config() -> anyhow::Result<Config> {
    let config = toml::from_str::<Config>(CONFIG_TOML)?;
    Ok(config)
}
