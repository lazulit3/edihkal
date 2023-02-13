use edihkal_client::Client;

use crate::config::Config;

/// Returns an `edihkal_client::Client` for the edihkal URL from edid's configuration.
pub fn client() -> Result<Client, anyhow::Error> {
    let config = Config::load()?;
    Ok(Client::new(config.edihkal_url))
}
