use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, thiserror::Error, Serialize)]
pub enum Error {
    #[error("Bad Request")]
    BadRequest,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not Found")]
    NotFound,
    #[error("The edihkal service encountered an unexpected error")]
    InternalServerError,
    #[error("Bad Gateway")]
    BadGateway,
    #[error("Service Unavailable")]
    ServiceUnavailable,
    #[error("Gateway Timeout")]
    GatewayTimeout,

    /// Error for status codes not captured by other `Error` variants (see [`parse_status_error()`])
    #[error("Error Status - {0} {1}")]
    Status(String, String),

    #[error("Transport Error - {0}({1})")]
    Transport(String, String),

    #[error("Deserialization Error: {0}")]
    Deserialization(String),
}

impl Error {
    /// Returns `Error` parsed from status code & response
    pub fn parse_status_error(code: u16, response: ureq::Response) -> Error {
        match code {
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            404 => Self::NotFound,
            500 => Self::InternalServerError,
            502 => Self::BadGateway,
            503 => Self::ServiceUnavailable,
            504 => Self::GatewayTimeout,
            _ => Self::Status(
                response.status().to_string(),
                response.status_text().to_string(),
            ),
        }
    }

    /// Returns `Error::Transport` parsed from [`ureq::Transport`]
    pub fn parse_transport_error(transport: ureq::Transport) -> Error {
        Self::Transport(transport.to_string(), transport.kind().to_string())
    }
}
