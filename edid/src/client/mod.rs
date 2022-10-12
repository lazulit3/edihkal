use self::api_client::ApiClient;

pub mod api_client;
pub mod drugs;

pub struct Client<'c> {
    client: ApiClient<'c>,
}

impl Client<'_> {
    pub fn new(edihkal_url: &str) -> Client {
        Client {
            client: ApiClient::new(edihkal_url),
        }
    }
}
