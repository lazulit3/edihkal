use axum::http::StatusCode;
use edihkal::drugs::insert_drug;
use entity::{drug, drug::NewDrug, Drug};
use sea_orm::EntityTrait;
use uuid::Uuid;

use super::helpers::{http, TestService};

#[tokio::test]
async fn define_drug_returns_201_for_valid_data() {
    let service = TestService::new().await;
    let client = http::Client::new(service.service_url());
    let db = service.database_connection();

    let drug = NewDrug::new("caffeine");

    let response = client
        .post("/drugs")
        .header("Content-Type", "application/json")
        .json(&drug)
        .send()
        .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    match Drug::find().one(db).await.unwrap() {
        Some(drug) => assert_eq!(drug.name(), "caffeine"),
        None => panic!("failed to find newly defined drug in database"),
    }
}

#[tokio::test]
async fn define_drug_returns_400_for_missing_data() {
    let service = TestService::new().await;
    let client = http::Client::new(service.service_url());

    let response = client
        .post("/drugs")
        .header("Content-Type", "application/json")
        .send()
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn get_drug_returns_200_and_drug() {
    // Arrange
    let service = TestService::new().await;
    let db = service.database_connection();
    let client = http::Client::new(service.service_url());

    let defined_drug = insert_drug(db, NewDrug::new("aporphine")).await.unwrap();

    // Act
    let path = format!("/drugs/{}", defined_drug.id());
    let drug_response = client.get(&path).header("Accept", "application/json").send().await;

    // Assert
    assert_eq!(drug_response.status(), StatusCode::OK);
    let drug: drug::Model = drug_response.json().await;
    assert_eq!(drug.name(), defined_drug.name());
    assert_eq!(drug.id(), defined_drug.id());
}

#[tokio::test]
async fn get_drug_returns_404_not_found() {
    // Arrange
    let service = TestService::new().await;
    let client = http::Client::new(service.service_url());

    // Act
    let path = format!("/drugs/{}", Uuid::new_v4());
    let drug_response = client.get(&path).header("Accept", "application/json").send().await;

    // Assert
    assert_eq!(drug_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_drugs_returns_list_of_drugs() {
    // Arrange
    let service = TestService::new().await;
    let edihkal_client = edihkal_client::Client::new(service.service_url().to_string());
    let http_client = http::Client::new(service.service_url());

    // Define drugs with these names
    let defined_drugs = vec!["gamma-hydroxybutyrate", "hydrocodone"];
    for drug_name in &defined_drugs {
        let drug = NewDrug::new(*drug_name);
        edihkal_client.define_drug(drug).await.unwrap();
    }

    // Act
    // Request list of defined drugs from API
    let response = http_client
        .get("/drugs")
        .header("Content-Type", "application/json")
        .send()
        .await;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    // Response JSON should deserialize into list of drug models.
    let drugs: Vec<drug::Model> = response.json().await;

    // edihkal should return the same quantity of drugs as what we defined.
    assert_eq!(defined_drugs.len(), drugs.len());

    // Each drug should have a name from `drug_names` and a non-nil `Uuid`
    for drug in drugs {
        assert!(defined_drugs.contains(&drug.name()));
        assert!(!drug.id().is_nil());
    }
}
