use axum::Server;
use edihkal_server::router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    Server::bind(&addr)
        .serve(router().into_make_service())
        .await
        .unwrap();
}
