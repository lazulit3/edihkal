use axum::{body::Body, http::StatusCode, routing::get, Router};

use crate::drugs::{define_drug, get_drugs};

pub fn router() -> Router<Body> {
    Router::new()
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/drugs", get(get_drugs).post(define_drug))
}
