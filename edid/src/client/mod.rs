use crate::config::Config;

use self::api_client::ApiClient;

pub mod api_client;
pub mod drugs;

pub struct EdihkalClient<'a> {
    client: ApiClient<'a>,
}

impl<'a> EdihkalClient<'a> {
    pub fn new(edihkal_url: &'a str) -> Self {
        EdihkalClient {
            client: ApiClient::new(edihkal_url),
        }
    }
}

impl<'a> From<&'a Config> for EdihkalClient<'a> {
    fn from(config: &'a Config) -> Self {
        EdihkalClient::new(&config.edihkal_url)
    }
}
