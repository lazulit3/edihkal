use axum::{http::StatusCode, routing::get, Router, Server};
use std::net::SocketAddr;

pub async fn start() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let app = Router::new().route("/health_check", get(|| async { StatusCode::OK }));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
