//! Error types, downcasting, and [`IntoResponse`][IR] conversions.
//!
//! [IR]: axum::response::IntoResponse

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::{DbErr, RuntimeErr};

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
    /// **Warning:** Downcasting to `UniqueViolation` is currently only implemented for Postgres (`sea-orm-postgres` feature).
    /// On other database backends a unique violation will result in a [`DatabaseError::Unknown`].
    #[error("Duplicate key violates unique constraint")]
    UniqueViolation(#[from] sqlx::Error),

    /// Includes errors that downcasting is not implemented for yet (almost everything).
    #[error("Database error occurred")]
    Unknown(#[from] anyhow::Error),
}

impl From<DbErr> for DatabaseError {
    fn from(error: DbErr) -> Self {
        match error {
            // TODO: Behavior for other databases
            // #[cfg(feature = "sea-orm-postgres")]
            // TODO: Check e.kind() for ErrorKind::UniqueViolation after sqlx 0.7 release:
            //       https://github.com/launchbadge/sqlx/pull/2109
            DbErr::Query(RuntimeErr::SqlxError(sqlx::Error::Database(e)))
                if e.code().unwrap().eq("23505") =>
            {
                Self::UniqueViolation(sqlx::Error::Database(e))
            }
            _ => Self::Unknown(error.into()),
        }
    }
}

/// Convert [`ApiError`] into an [`INTERNAL_SERVER_ERROR`][I] and output error info in a trace event.
///
/// [I]: axum::http::StatusCode::INTERNAL_SERVER_ERROR
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::Conflict => StatusCode::CONFLICT,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InternalServerError(err) => {
                // TODO: Check what trace output looks like (does it have context?)
                tracing::error!("{:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
        .into_response()
    }
}
