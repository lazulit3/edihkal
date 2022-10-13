use thiserror::Error;

// TODO: client/request error?
#[derive(Error, Debug)]
pub enum Error {
    #[error("the edihkal service reported an unexpected error")]
    Service,
}
