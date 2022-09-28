use crate::helpers::{test_client, test_client_and_db};
use axum::http::StatusCode;
use sqlx::query;

#[tokio::test]
async fn define_drug_returns_200_for_valid_data() {
    let (client, db_pool) = test_client_and_db().await;

    let drug_body = serde_json::json!({"name": "caffeine"});

    let response = client
        .post("/drugs")
        .header("Content-Type", "application/json")
        .json(&drug_body)
        .send()
        .await;

    assert_eq!(response.status(), StatusCode::OK);

    let saved_drug = query!("SELECT name FROM drugs",)
        .fetch_one(&db_pool)
        .await
        .expect("Failed to fetch defined drug.");

    assert_eq!(saved_drug.name, "caffeine")
}

#[tokio::test]
async fn define_drug_returns_400_for_missing_data() {
    let client = test_client().await;

    let response = client
        .post("/drugs")
        .header("Content-Type", "application/json")
        .send()
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[ignore]
#[tokio::test]
async fn get_drugs_returns_list_of_drugs() {
    let client = test_client().await;
    todo!()
}
