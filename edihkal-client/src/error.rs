#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("the edihkal service reported an unexpected error")]
    Service,
}
