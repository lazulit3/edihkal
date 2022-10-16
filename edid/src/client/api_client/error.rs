#[derive(thiserror::Error, Debug)]
pub enum BaseUrlError {
    #[error("base_url {0} is expected to end in a trailing slash (/) to avoid potentially unexpected Url::join() behavior")]
    MissingTrailingSlash(String),
}

#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    #[error("failed to join request endpoint_path {endpoint_path} to base_url {base_url}")]
    RequestUrl {
        base_url: String,
        endpoint_path: String,
    },
}
