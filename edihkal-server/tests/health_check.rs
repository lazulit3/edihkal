use axum_test_helper::TestClient;
use edihkal_server::router;

#[tokio::test]
async fn health_check_works() {
    let client = TestClient::new(router());

    let res = client.get("/health_check").send().await;

    assert!(res.status().is_success());
}
