use std::sync::Arc;

use axum::{body::Body, http::StatusCode, routing::get, Extension, Router};
use sqlx::PgConnection;

use crate::drugs::{define_drug, get_drugs};

pub fn router(connection: PgConnection) -> Router<Body> {
    let connection = Arc::new(connection);
    Router::new()
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/drugs", get(get_drugs).post(define_drug))
        .layer(Extension(connection))
}
