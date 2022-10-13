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
    use axum::http::StatusCode;
    use axum::routing::get;
    use axum::Router;
    use axum_test_helper::TestClient;

    #[tokio::test]
    async fn test_get_request() {
        let app = Router::new().route("/", get(|| async {}));
        let client = TestClient::new(app);
        let res = client.get("/").send().await;
        assert_eq!(res.status(), StatusCode::OK);
    }
}
