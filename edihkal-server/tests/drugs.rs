use axum::http::StatusCode;
use axum_test_helper::TestClient;
use edihkal_server::router;
use std::collections::HashMap;

#[tokio::test]
async fn define_drug_returns_200_for_valid_data() {
    // TODO: Replace with model
    let mut map = HashMap::new();
    map.insert("name", "caffeine");

    let client = TestClient::new(router());
    let response = client
        .post("/drugs")
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn defin_drug_returns_400_for_missing_data() {
    let client = TestClient::new(router());
    let response = client.post("/drugs").send().await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn get_drugs_returns_list_of_drugs() {
    let client = TestClient::new(router());
    todo!()
}
