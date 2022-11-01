use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::errors::Error;

/// A client to the edihkal API.
pub struct Client<'u> {
    base_url: &'u str,
    agent: ureq::Agent,
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

impl Client<'_> {
    /// Constructs a client to an edihkal service at a given base URL.
    pub fn new(base_url: &str) -> Client {
        Client {
            base_url,
            agent: ureq::Agent::new(),
        }
    }

    // TODO: Implement other HTTP methods.

    /// Sends a POST request to the edihkal API service.
    pub fn post<E: Endpoint>(&self, path: &str, data: Value) -> Result<Response<E::Output>, Error> {
        let response = self
            .agent
            .post(&self.url(path))
            .set("Accept", "appliation/json")
            .send_json(data);
        Self::process_response::<E>(response)
    }

    /// Process `ureq` result from API call into edihkal API result.
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
