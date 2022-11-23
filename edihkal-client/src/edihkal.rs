use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::errors::Error;

/// A client to the edihkal API.
pub struct Client {
    agent: ureq::Agent,
    base_url: String,
}

/// Defines output types for different endpoints.
pub trait Endpoint {
    type Output: DeserializeOwned;
}

/// Response from an edihkal API call.
#[derive(Debug)]
pub struct Response<T> {
    /// Object(s) returned by the API (type `T` is determined by the API endpoint).
    pub data: T,
    /// HTTP status code.
    pub status: u16,
}

impl Client {
    /// Constructs a client to an edihkal service at a given base URL.
    pub fn new<S: Into<String>>(base_url: S) -> Client {
        Client {
            agent: ureq::Agent::new(),
            base_url: base_url.into(),
        }
    }

    // TODO: Implement other HTTP methods.

    /// Sends a POST request to the edihkal API service.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn post<E: Endpoint>(&self, path: &str, data: Value) -> Result<Response<E::Output>, Error> {
        let response = self
            .agent
            .post(&self.url(path))
            .set("Accept", "appliation/json")
            .send_json(data);
        Self::process_response::<E>(response)
    }

    /// Process `ureq` result from API call into edihkal API result.
    #[tracing::instrument(level = "debug")]
    fn process_response<E: Endpoint>(
        result: Result<ureq::Response, ureq::Error>,
    ) -> Result<Response<E::Output>, Error> {
        match result {
            Ok(response) => Self::parse_response::<E>(response),
            Err(ureq::Error::Status(code, response)) => {
                Err(Error::parse_status_error(code, response))
            }
            Err(ureq::Error::Transport(transport)) => Err(Error::parse_transport_error(transport)),
        }
    }

    /// Returns a `Response` parsed from `ureq::Response`.
    #[tracing::instrument(level = "debug")]
    fn parse_response<E: Endpoint>(response: ureq::Response) -> Result<Response<E::Output>, Error> {
        let status = response.status();

        let data: E::Output = response
            .into_json()
            .map_err(|e| Error::Deserialization(e.to_string()))?;

        Ok(Response { data, status })
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
        type Output = crate::Drug;
    }

    #[tokio::test]
    async fn post_json() {
        // Arrange
        lazy_tracing();

        let mock_server = MockServer::start().await;
        let mock_uri = mock_server.uri();
        let client = Client::new(mock_uri);

        let request_body = serde_json::to_value(NewDrug::new("iboga")).unwrap();
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
