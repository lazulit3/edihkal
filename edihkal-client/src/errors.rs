#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP request received error status code
    #[error("HTTP Error - {0}")]
    Http(#[source] reqwest::Error),

    /// An Error encountered parsing respons from edihkal service.
    ///
    /// If get this error, please [open an issue] with the error output attached.
    /// (Consider changing sensitive data in the output e.g. drug names.)
    ///
    /// [open an issue]: https://0xacab.org/lazulite/edihkal/-/issues/new
    #[error("Error parsing JSON: {source} (raw: {raw:?})")]
    InvalidJson {
        #[source]
        source: serde_json::Error,
        /// Raw JSON string that couldn't be parsed
        raw: Box<str>,
    },

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
