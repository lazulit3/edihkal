use edihkal_core::drugs::Drug;
use reqwest::Response;

use super::EdihkalClient;

impl EdihkalClient<'_> {
    /// Define a new drug in edihkal.
    pub async fn define_drug(self, name: &str) -> Result<Response, reqwest::Error> {
        let drug = Drug {
            name: name.to_string(),
        };
        self.client.get("/drugs").json(&drug).send().await
    }
}
