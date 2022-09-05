pub mod drugs;
use drugs::{define_drug, get_drugs};

use axum::{
    body::Body,
    http::StatusCode,
    routing::{get, post},
    Router,
};

pub fn router() -> Router<Body> {
    Router::new()
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/drugs", get(get_drugs).post(define_drug))
}
