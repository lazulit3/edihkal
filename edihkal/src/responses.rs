use axum::{
    http::{header::LOCATION, StatusCode},
    response::IntoResponse,
    Json,
};
use sea_skipper::Location;
use serde::Serialize;

/// Returns a response indicating that a [`Resource`] was successfully created.
pub fn created<R>(resource: R) -> impl IntoResponse
where
    R: Serialize + Location,
{
    (
        StatusCode::CREATED,
        [(LOCATION, resource.location())],
        Json(resource),
    )
}
