//! A convenience wrapper around `reqwest::Client` that allows making requests with relative paths.

// TODO: Consider moving this into its own crate.

use reqwest::RequestBuilder;

/// A wrapper around `reqwest::Client` that stores the `base_url` that endpoint paths are appended to.
pub struct ApiClient<'a> {
    client: reqwest::Client,
    /// Base URL for requests.
    base_url: &'a str,
}

impl<'a> ApiClient<'a> {
    pub fn new(base_url: &'a str) -> Self {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        ApiClient { client, base_url }
    }

    /// Make a `GET` request to a relative `endpoint_path` joined to the `ApiClient` base URL.
    pub fn get(&self, endpoint_path: &str) -> RequestBuilder {
        self.client
            .get(format!("{}{}", self.base_url, endpoint_path))
    }

    /// Make a `HEAD` request to a relative `endpoint_path` joined to the `ApiClient` base URL.
    pub fn head(&self, endpoint_path: &str) -> RequestBuilder {
        self.client
            .head(format!("{}{}", self.base_url, endpoint_path))
    }

    /// Make a `POST` request to a relative `endpoint_path` joined to the `ApiClient` base URL.
    pub fn post(&self, endpoint_path: &str) -> RequestBuilder {
        self.client
            .post(format!("{}{}", self.base_url, endpoint_path))
    }

    /// Make a `PUT` request to a relative `endpoint_path` joined to the `ApiClient` base URL.
    pub fn put(&self, endpoint_path: &str) -> RequestBuilder {
        self.client
            .put(format!("{}{}", self.base_url, endpoint_path))
    }

    /// Make a `PATCH` request to a relative `endpoint_path` joined to the `ApiClient` base URL.
    pub fn patch(&self, endpoint_path: &str) -> RequestBuilder {
        self.client
            .patch(format!("{}{}", self.base_url, endpoint_path))
    }

    /// Make a `DELETE` request to a relative `endpoint_path` joined to the `ApiClient` base URL.
    pub fn delete(&self, endpoint_path: &str) -> RequestBuilder {
        self.client
            .delete(format!("{}{}", self.base_url, endpoint_path))
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
