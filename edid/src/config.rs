use std::path::Path;
use serde::Deserialize;

use crate::{cli::Opts, client::Client};


/// Configuration for edid loaded from `EDID_*` environment variables and an optional config file.
#[derive(Deserialize)]
pub struct Config {
    pub edihkal_url: String,
}

impl Config {
    /// Returns `Config` loaded from `EDID_*` environment variables only.
    fn load() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(
                config::Environment::with_prefix("EDID")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;
        config.try_deserialize::<Self>()
    }

    /// Returns `Config` loaded from config file and `EDID_*` environment variables.
    ///
    /// If a configuration is defined in both the config file and an environment variable,
    /// the environment variable will take precedence.
    fn load_with_config_file(config_path: &Path) -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::File::from(config_path))
            .add_source(
                config::Environment::with_prefix("EDID")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;
        config.try_deserialize::<Self>()
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
