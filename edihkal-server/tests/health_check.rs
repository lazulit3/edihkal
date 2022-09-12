use axum::http::StatusCode;
use axum_test_helper::TestClient;
use edihkal_server::{configuration::get_configuration, router::app};

#[tokio::test]
async fn health_check_works() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let client = TestClient::new(app(&configuration).await);

    let response = client.get("/health_check").send().await;

    assert_eq!(response.status(), StatusCode::OK);
}
