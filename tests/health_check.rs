use axum::Server;
use edihkal_server::router;
use std::net::SocketAddr;

#[tokio::test]
async fn health_check_works() {
    spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to request /health_check.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let server = Server::bind(&addr).serve(router().into_make_service());
    let _ = tokio::spawn(server);
}
