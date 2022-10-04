use crate::helpers::test_client;

#[tokio::test]
async fn openapi_json_returns_200_with_valid_json() {
    let client = test_client().await;

    let response = client.get("/openapi.json").send().await;

    assert_eq!(response.status(), StatusCode::OK);
    assert!(serde_json::from_str(response.text).is_ok());
}
