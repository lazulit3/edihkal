use axum::{body::Body, http::StatusCode, routing::get, Router};

pub fn router() -> Router<Body> {
    Router::new().route("/health_check", get(|| async { StatusCode::OK }))
}
