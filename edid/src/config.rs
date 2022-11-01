use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

/// Configuration for edid loaded from `EDID_*` environment variables and an optional config file.
#[derive(Deserialize)]
pub struct Config {
    pub edihkal_url: String,
}

impl Config {
    /// Returns `Config` loaded from a config file (if defined) and `EDID_*` environment
    /// variables.
    pub fn load(config_path: Option<&Path>) -> Result<Self> {
        match config_path {
            Some(config_path) => Config::load_from_config_file_and_envs(config_path),
            None => Config::load_from_envs(),
        }
        .context("Failed to load configuration")
    }

    /// Returns `Config` loaded from `EDID_*` environment variables only.
    fn load_from_envs() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(
                config::Environment::with_prefix("EDID")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;
        config
            .try_deserialize::<Self>()
            .context("Failed to deserialize configuration from environment variables")
    }

    /// Returns `Config` loaded from config file and `EDID_*` environment variables.
    ///
    /// If a configuration is defined in both the config file and an environment variable,
    /// the environment variable will take precedence.
    fn load_from_config_file_and_envs(config_path: &Path) -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::from(config_path))
            .add_source(
                config::Environment::with_prefix("EDID")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;
        config.try_deserialize::<Self>().context(
            "Failed to deserialize configuration from config file and environment variables",
        )
    }
}
