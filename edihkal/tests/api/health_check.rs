use crate::helpers::test_client;
use axum::http::StatusCode;

#[tokio::test]
async fn health_check_works() {
    let client = test_client().await;

    let response = client.get("/health_check").send().await;

    assert_eq!(response.status(), StatusCode::OK);
}
