use axum::{http::StatusCode, routing::get, Router, Server};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health_check", get(|| async { StatusCode::OK }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
