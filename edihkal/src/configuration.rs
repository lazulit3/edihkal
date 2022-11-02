use anyhow::{Context, Result};
use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

pub fn get_configuration() -> Result<Settings> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("EDIHKAL_ENVIRONMENT")
        .unwrap_or_else(|_| "localdev".into())
        .try_into()
        .expect("Failed to parse EDIHKAL_ENVIRONMENT.");
    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(&environment_filename),
        ))
        .add_source(
            config::Environment::with_prefix("EDIHKAL")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .context("Failed to build configuration source behavior")?;
    settings
        .try_deserialize::<Settings>()
        .context("Failed to deserialize configuration")
}

pub enum Environment {
    LocalDev,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::LocalDev => "localdev",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "localdev" => Ok(Self::LocalDev),
            other => Err(format!("{} is not a supported environment. Currently upported environments are `localdev`.", other)),
        }
    }
}
