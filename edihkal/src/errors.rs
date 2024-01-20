//! Error types, downcasting, and [`IntoResponse`][IR] conversions.
//!
//! [IR]: axum::response::IntoResponse

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;

#[derive(thiserror::Error, Debug)]
// This is not currently a stable API.
#[non_exhaustive]
pub enum ApiError {
    // TODO: The server SHOULD generate a payload that includes enough information for a user to recognize the source of the conflict.
    /// 409 Conflict
    #[error("Conflict with existing resource")]
    Conflict,

    // TODO: Look for more correct approach to tracing errors
    /// 500 Internal Server Error
    /// Includes [`anyhow::Error`] for tracing output (it's not a response body)
    #[error(transparent)]
    InternalServerError(#[from] anyhow::Error),

    /// 404 Not Found
    #[error("Not Found")]
    NotFound,
}

/// Errors encountered during database operations.
#[derive(thiserror::Error, Debug)]
// This is not currently a stable API.
#[non_exhaustive]
pub enum DatabaseError {
    #[error("Duplicate key violates unique constraint")]
    /// Error for duplicate record in unique field or primary key field
    UniqueViolation(sea_orm::DbErr),

    /// Includes errors that downcasting is not implemented for yet (almost everything).
    #[error("Database error occurred")]
    Unknown(#[from] anyhow::Error),
}

/// Convert [`ApiError`] into an [`INTERNAL_SERVER_ERROR`][I] and output error info in a trace event.
///
/// [I]: axum::http::StatusCode::INTERNAL_SERVER_ERROR
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::Conflict => StatusCode::CONFLICT.into_response(),
            Self::NotFound => StatusCode::NOT_FOUND.into_response(),
            Self::InternalServerError(err) => {
                tracing::error!("{:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

impl From<DbErr> for DatabaseError {
    fn from(err: DbErr) -> Self {
        if let Some(sea_orm::SqlErr::UniqueConstraintViolation(_)) = err.sql_err() {
            Self::UniqueViolation(err)
        } else {
            Self::Unknown(err.into())
        }
    }
}
