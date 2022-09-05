use axum::http::StatusCode;
use axum_test_helper::TestClient;
use edihkal_server::router;

#[tokio::test]
async fn health_check_works() {
    let client = TestClient::new(router());

    let response = client.get("/health_check").send().await;

    assert_eq!(response.status(), StatusCode::OK);
}
