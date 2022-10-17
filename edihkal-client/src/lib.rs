use url::Url;

use edihkal_core::drugs::Drug;

pub mod api_client;
pub mod error;

use api_client::ApiClient;
use error::Error;

#[derive(Debug)]
pub struct Client<'c> {
    api: ApiClient<'c>,
}

impl Client<'_> {
    /// Construct a `Client` for Edihkal service at `edihkal_url`.
    ///
    /// # Panics
    /// This method panics if `reqwest::Client::new()` fails to build with defaults.
    /// See [`reqwest::Client::new()`] for details.
    pub fn new(base_url: &Url) -> Result<Client, api_client::BaseUrlError> {
        let api = ApiClient::new(base_url)?;
        Ok(Client { api })
    }

    pub async fn define_drug(&self, name: &str) -> Result<(), Error> {
        let drug = Drug {
            name: name.to_string(),
        };
        self.api.get("/drugs").json(&drug).send().await;
        // TODO: Implement proper error handling
        Ok(())
    }
}
