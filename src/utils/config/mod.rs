const CONFIG_TOML: &str = include_str!("../../../config.toml");

pub static CONFIG: once_cell::sync::Lazy<Config> =
    once_cell::sync::Lazy::new(|| load_config().expect("Should load config"));

#[derive(serde::Deserialize)]
pub struct ApiConfig {
    pub url: String,
}

#[derive(serde::Deserialize)]
pub struct Config {
    pub api: ApiConfig,
}

pub fn load_config() -> anyhow::Result<Config> {
    let config = toml::from_str::<Config>(CONFIG_TOML)?;
    Ok(config)
}
