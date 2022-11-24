use serde::{de::DeserializeOwned, Serialize};

use crate::errors::Error;

/// A client to the edihkal API.
pub struct Client {
    client: reqwest::Client,
    base_url: String,
}

/// Defines output types for different endpoints.
pub trait Endpoint {
    type NewModel: Serialize;
    type Model: DeserializeOwned;
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

    /// Sends a GET request to the edihkal API service.
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get<E: Endpoint>(&self, path: &str) -> Result<Vec<E::Model>, Error> {
        let response = self
            .client
            .get(&self.url(path))
            .header("Accept", "appliation/json")
            .send()
            .await;
        Self::process_response::<Vec<E::Model>>(response).await
    }

    /// Sends a POST request to the edihkal API service.
    #[tracing::instrument(level = "debug", skip(self, data))]
    pub async fn post<E: Endpoint>(
        &self,
        path: &str,
        data: E::NewModel,
    ) -> Result<E::Model, Error> {
        let response = self
            .client
            .post(&self.url(path))
            .header("Accept", "appliation/json")
            .header("Content-Type", "application/json")
            .json(&data)
            .send()
            .await;
        Self::process_response::<E::Model>(response).await
    }

    /// Deserializes response's JSON data or propagate errors as [`edihkal_client::Error`].
    #[tracing::instrument(level = "debug")]
    async fn process_response<O: DeserializeOwned>(
        result: Result<reqwest::Response, reqwest::Error>,
    ) -> Result<O, Error> {
        // First consume response data as a string so it can be included in the Error if JSON
        // deserialization fails.
        let data: String = result?.text().await?;
        let data: O = serde_json::from_str(&data).map_err(|source| Error::InvalidJson {
            source,
            raw: data.into(),
        })?;
        Ok(data)
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

    use crate::drugs::DrugEndpoint;

    use super::Client;

    #[tokio::test]
    async fn get_json() {
        // Arrange
        lazy_tracing();

        let mock_server = MockServer::start().await;
        let mock_uri = mock_server.uri();
        let client = Client::new(mock_uri);

        let response_body = vec![
            drug::Model::new("lysergic acid diethylamide"),
            drug::Model::new("3,4-methylenedioxy-methamphetamine"),
        ];

        Mock::given(method("GET"))
            .and(path("/drugs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .expect(1) // Assert
            .mount(&mock_server)
            .await;

        // Act
        client
            .get::<DrugEndpoint>("/drugs")
            .await
            .expect("GET request failed");
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
            .expect(1) // Assert
            .mount(&mock_server)
            .await;

        // Act
        client
            .post::<DrugEndpoint>("/drugs", request_body)
            .await
            .expect("POST request failed");
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
