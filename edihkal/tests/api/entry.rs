use entity::{drug, entry, prelude::Entry};

use axum::http::StatusCode;
use chrono::Utc;
use sea_orm::EntityTrait;

use super::helpers::{http, TestService};

#[tokio::test]
async fn define_entry_returns_201_for_valid_data() {
    // Arrange
    let service = TestService::new().await;
    let http_client = http::Client::new(service.service_url());
    let edihkal_client = edihkal_client::Client::new(service.service_url().to_string());
    let db = service.database_connection();

    let drug = drug::NewModel::new("THC".to_string());
    let drug = edihkal_client.define_drug(drug).await.unwrap();
    let drug_id = drug.id;

    let now = Utc::now().naive_utc();
    let entry = entry::NewModel::new(drug_id, now, 10);

    // Act
    let response = http_client
        .post("/entries")
        .header("Content-Type", "application/json")
        .json(&entry)
        .send()
        .await;

    // Asert
    assert_eq!(response.status(), StatusCode::CREATED);
    match Entry::find().one(db).await.unwrap() {
        Some(entry) => {
            let id = entry.id;
            assert!(!id.is_nil());
            assert!(entry.dose >= 0);
            assert!(entry.time <= Utc::now().naive_utc());
            assert_eq!(
                response.headers().get("Location").unwrap().to_str().unwrap(),
                format!("/entries/{}", id)
            );
        }
        None => panic!("failed to find newly recorded entry in database"),
    }
}

#[tokio::test]
async fn define_entry_returns_400_for_missing_data() {
    let service = TestService::new().await;
    let client = http::Client::new(service.service_url());

    let response = client
        .post("/entries")
        .header("Content-Type", "application/json")
        .send()
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
