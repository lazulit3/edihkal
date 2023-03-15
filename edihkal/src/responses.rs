use axum::{
    http::{header::LOCATION, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use crate::resource::Resource;

/// Returns a response indicating that a [`Resource`] was successfully created.
pub fn created<R>(resource: R) -> impl IntoResponse
where
    R: Resource + Serialize,
{
    (
        StatusCode::CREATED,
        [(LOCATION, resource.location())],
        Json(resource),
    )
}
