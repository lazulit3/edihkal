use axum::Server;
use edihkal_server::{configuration::get_configuration, router::app};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));

    Server::bind(&addr)
        .serve(app(&configuration).await.into_make_service())
        .await
        .unwrap();
}
