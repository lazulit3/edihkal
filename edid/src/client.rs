//! A client to the edihkal API.
use crate::configuration::Config;

pub struct Client {
    /// Base URL of the edihkal API service.
    pub edihkal_base_url: String,
}

impl From<&Config> for Client {
    fn from(config: &Config) -> Self {
        Client {
            edihkal_base_url: config.edihkal_url.clone(),
        }
    }
}
