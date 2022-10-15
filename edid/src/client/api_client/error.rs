use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("base_url is expected to end in a trailing slash (/) to avoid potentially unexpected Url::join() behavior")]
    UnexpectedBaseUrl,
    #[error("failed to join request's endpoint_path to base_url")]
    UrlJoin,
}
