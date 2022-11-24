#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP request received error status code
    #[error("HTTP Error - {0}")]
    Http(#[source] reqwest::Error),

    // TODO: Probs not enough info to troubleshoot effectively?
    #[error("Error while parsing JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),

    /// An error encountered communicating with API service
    #[error("Network Error - {0}")]
    Network(#[source] reqwest::Error),
}

/// Map [`reqwest::Error`] origin to `Error` variants.
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        if error.is_status() {
            return Error::Http(error);
        }
        Error::Network(error)
    }
}
