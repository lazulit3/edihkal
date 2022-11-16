use axum::http::StatusCode;
use edihkal_core::drugs::NewDrug;
use entity::prelude::Drug;
use sea_orm::EntityTrait;

use crate::helpers::{test_client, test_client_and_db};

#[tokio::test]
async fn define_drug_returns_200_for_valid_data() {
    let (client, db) = test_client_and_db().await;

    let drug = NewDrug::new("caffeine");

    let response = client
        .post("/drugs")
        .header("Content-Type", "application/json")
        .json(&drug)
        .send()
        .await;

    assert_eq!(response.status(), StatusCode::OK);

    match Drug::find().one(&db).await.unwrap() {
        Some(drug) => assert_eq!(drug.name, "caffeine"),
        None => panic!("failed to find newly defined drug in database"),
    }
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
    let _client = test_client().await;
    todo!()
}
