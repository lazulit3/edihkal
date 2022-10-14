use reqwest::RequestBuilder;
use url::Url;

/// An opinionated wrapper around [`reqwest::Client`] to build requests to relative URL
/// paths against a configured `base_url`.
///
/// When a request method is passed a relative URL path, `ApiClient` joins the relative path to its configured `base_url`.
///
/// When a request method is passed an absolute URL, `ApiClient` passes the request to `reqwest` with no changes.
pub(super) struct ApiClient<'u> {
    /// The base URL that all relative paths joined to before sending a request.
    base_url: &'u Url,
    /// The `reqwest::Client` wrapped by `ApiClient`.
    inner: reqwest::Client,
}

impl ApiClient<'_> {
    /// Construct an `ApiClient` for `base_url`.
    ///
    /// # Panics
    /// This method panics if `reqwest::Client::new()` fails to build with defaults.
    /// See [`reqwest::Client::new()`] for details.
    pub fn new(base_url: &Url) -> ApiClient {
        let client = reqwest::Client::new();
        ApiClient {
            base_url,
            inner: client,
        }
    }

    /// Construct an `ApiClient` for `base_url` that uses an explicit `reqwest::Client`.
    pub fn with_client(base_url: &Url, client: reqwest::Client) -> ApiClient {
        ApiClient {
            base_url,
            inner: client,
        }
    }

    /// Make a `GET` request to a relative `endpoint_path` joined to the `ApiClient` base URL.
    pub fn get(&self, endpoint_path: &str) -> RequestBuilder {
        self.inner.get(self.to_endpoint_url(endpoint_path))
    }

    /// Make a `DELETE` request to a relative `endpoint_path` joined to the `ApiClient` base URL.
    pub fn delete(&self, endpoint_path: &str) -> RequestBuilder {
        self.inner.delete(self.to_endpoint_url(endpoint_path))
    }

    /// Make a `HEAD` request to a relative `endpoint_path` joined to the `ApiClient` base URL.
    pub fn head(&self, endpoint_path: &str) -> RequestBuilder {
        self.inner.head(self.to_endpoint_url(endpoint_path))
    }

    /// Make a `PATCH` request to a relative `endpoint_path` joined to the `ApiClient` base URL.
    pub fn patch(&self, endpoint_path: &str) -> RequestBuilder {
        self.inner.patch(self.to_endpoint_url(endpoint_path))
    }

    /// Make a `POST` request to a relative `endpoint_path` joined to the `ApiClient` base URL.
    pub fn post(&self, endpoint_path: &str) -> RequestBuilder {
        self.inner.post(self.to_endpoint_url(endpoint_path))
    }

    /// Make a `PUT` request to a relative `endpoint_path` joined to the `ApiClient` base URL.
    pub fn put(&self, endpoint_path: &str) -> RequestBuilder {
        self.inner.put(self.to_endpoint_url(endpoint_path))
    }

    /// Returns absolute URL for an API endpoint given the `endpoint_path` to join with the `ApiClient`'s `base_url`.
    fn to_endpoint_url(&self, endpoint_path: &str) -> Url {
        self.base_url.join(endpoint_path).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use url::Url;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    use super::ApiClient;

    // Test all supported HTTP methods using a relative path
    #[rstest]
    #[tokio::test]
    async fn test_http_methods_with_relative_paths(
        #[values("GET", "DELETE", "HEAD", "PATCH", "POST", "PUT")] http_method: &str,
        #[values("/", "/foo/bar")] test_path: &str,
    ) {
        let mock_server = MockServer::start().await;

        // Configure an ApiClient with base_url of the mock_server
        let base_url = Url::parse(&mock_server.uri()).unwrap();
        let client = ApiClient::new(&base_url).unwrap();

        // Expect 1x http_method request to test_path
        Mock::given(method(http_method))
            .and(path(test_path))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Select the appropriate ApiClient method to test
        let client_method = match http_method {
            "GET" => ApiClient::get,
            "DELETE" => ApiClient::delete,
            "HEAD" => ApiClient::head,
            "PATCH" => ApiClient::patch,
            "POST" => ApiClient::post,
            "PUT" => ApiClient::put,
            _ => panic!(),
        };

        // Call the method with a relative path; Mock expects a single call to this path.
        client_method(&client, test_path).send().await.unwrap();
    }
}
