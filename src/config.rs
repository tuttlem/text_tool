use config::Config as AppConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub default_output: Option<String>,
}

pub fn load_config() -> Config {
    let mut settings = AppConfig::default();
    settings
        .merge(config::File::with_name("config"))
        .expect("Failed to load configuration file");

    // Deserialize settings into the Config struct
    settings
        .try_deserialize::<Config>()
        .expect("Failed to deserialize configuration")
}
