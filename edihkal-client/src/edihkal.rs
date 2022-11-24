use serde::{de::DeserializeOwned, Serialize};

use crate::errors::Error;

/// A client to the edihkal API.
pub struct Client {
    client: reqwest::Client,
    base_url: String,
}

/// Defines output types for different endpoints.
pub trait Endpoint {
    type Input: Serialize;
    type Output: DeserializeOwned;
}

impl Client {
    /// Constructs a client to an edihkal service at a given base URL.
    pub fn new<S: Into<String>>(base_url: S) -> Client {
        Client {
            client: reqwest::Client::new(),
            base_url: base_url.into(),
        }
    }

    // TODO: Implement other HTTP methods.

    /// Sends a POST request to the edihkal API service.
    #[tracing::instrument(level = "debug", skip(self, data))]
    pub async fn post<E: Endpoint>(&self, path: &str, data: E::Input) -> Result<E::Output, Error> {
        let response = self
            .client
            .post(&self.url(path))
            .header("Accept", "appliation/json")
            .header("Content-Type", "application/json")
            .json(&data)
            .send()
            .await;
        Self::process_response::<E>(response).await
    }

    /// Deserializes response's JSON data or propagate errors as [`edihkal_client::Error`].
    #[tracing::instrument(level = "debug")]
    async fn process_response<E: Endpoint>(
        result: Result<reqwest::Response, reqwest::Error>,
    ) -> Result<E::Output, Error> {
        // TODO: Handle connect error separately from HttpError (e.g. is_status vs is_connect)
        Ok(result?.json().await?)
    }

    /// Returns URL with `path` appended to the `Client`'s base URL.
    fn url(&self, path: &str) -> String {
        let mut url = self.base_url.to_string();
        url.push_str(path);
        url
    }
}

#[cfg(test)]
mod tests {

    use edihkal_tracing::test_helpers::lazy_tracing;

    use entity::{drug, drug::NewDrug};
    use wiremock::{
        matchers::{body_json, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    use super::{Client, Endpoint};

    struct TestEndpoint;
    impl Endpoint for TestEndpoint {
        type Input = NewDrug;
        type Output = drug::Model;
    }

    #[tokio::test]
    async fn post_json() {
        // Arrange
        lazy_tracing();

        let mock_server = MockServer::start().await;
        let mock_uri = mock_server.uri();
        let client = Client::new(mock_uri);

        let request_body = NewDrug::new("iboga");
        let response_body = drug::Model::new("iboga");

        Mock::given(method("POST"))
            .and(path("/drugs"))
            .and(body_json(&request_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        client
            .post::<TestEndpoint>("/drugs", request_body)
            .await
            .expect("POST request failed");

        // Assert
    }

    #[test]
    fn url_path_appended_to_client_base_url() {
        // Arrange
        lazy_tracing();
        let base_url = "http://127.0.0.1:8080";
        let client = Client::new(base_url);
        let relative_url_path = "/foo/bar";

        // Act
        let url = client.url(relative_url_path);

        // Assert
        assert_eq!(url, "http://127.0.0.1:8080/foo/bar");
    }
}
