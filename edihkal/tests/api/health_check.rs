use super::helpers::{http, TestService};
use axum::http::StatusCode;

#[tokio::test]
async fn health_check_works() {
    let service = TestService::new().await;
    let client = http::Client::new(service.service_url());

    let response = client.get("/health_check").send().await;

    assert_eq!(response.status(), StatusCode::OK);
}
