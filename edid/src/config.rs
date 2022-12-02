use anyhow::{Context, Result};
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub edihkal_url: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            edihkal_url: "http://localhost:8080".to_string(),
        }
    }
}

impl Config {
    /// Returns `Config` loaded from `edid.toml` and environment variables.
    ///
    /// If a setting is defined in both, environment variables take precedence.
    ///
    /// Environment variables are prefixed with `EDID_`; for example, to set the edihkal service URL,
    /// the `EDID_EDIHKAL_URL` environment variable would be used.
    pub fn load() -> Result<Self> {
        Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file("edid.toml"))
            .merge(Env::prefixed("EDID_"))
            .extract()
            .context("Failed to load configuration")
    }
}
