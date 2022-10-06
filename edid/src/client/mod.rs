//! A client to the edihkal API.

pub mod drugs;

pub struct Client {
    /// Base URL of the edihkal API service.
    pub edihkal_base_url: String,
}
