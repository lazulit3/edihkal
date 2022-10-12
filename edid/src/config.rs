use std::path::Path;

use crate::cli::Opts;

#[derive(serde::Deserialize)]
pub struct Config {
    pub edihkal_url: String,
}

impl Config {
    /// Returns config loaded from environment variables.
    pub fn load() -> Result<Config, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(
                config::Environment::with_prefix("EDID")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;
        config.try_deserialize::<Config>()
    }

    /// Returns config loaded from config file and environment variables.
    pub fn load_with_config_file(config_path: &Path) -> Result<Config, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::File::from(config_path))
            .add_source(
                config::Environment::with_prefix("EDID")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;
        config.try_deserialize::<Config>()
    }
}

/// Load `Config` appropriately depending on whether a config file is included in CLI `Opts`.
impl From<&Opts> for Config {
    fn from(opts: &Opts) -> Self {
        if let Some(config_path) = opts.config_path() {
            Config::load_with_config_file(config_path)
        } else {
            Config::load()
        }
        .expect("Failed to load configuration")
    }
}
