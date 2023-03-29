use axum::http::StatusCode;
use edihkal::{drugs::insert_drug, entity::Resource};
use entity::{drug, Drug, NewDrug};
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::helpers::define_drugs;

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
        Some(drug) => {
            let id = drug.id();
            assert!(!id.is_nil());
            assert_eq!(drug.name(), "caffeine");
        }
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
    let path = defined_drug.location();
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

    let defined_drug_names = vec!["gamma-hydroxybutyrate", "hydrocodone"];
    define_drugs(&edihkal_client, &defined_drug_names).await;

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
    assert_eq!(defined_drug_names.len(), drugs.len());

    // Each drug should have a name from `drug_names` and a non-nil `Uuid`
    for drug in drugs {
        assert!(defined_drug_names.contains(&drug.name()));
        assert!(!drug.id().is_nil());
    }
}

#[tokio::test]
async fn get_drugs_filters_by_name() {
    // Arrange
    let service = TestService::new().await;
    let edihkal_client = edihkal_client::Client::new(service.service_url().to_string());
    let http_client = http::Client::new(service.service_url());

    let defined_drug_names = vec!["phencyclidine", "salvia"];
    define_drugs(&edihkal_client, &defined_drug_names).await;

    // Act
    // Request list of defined drugs with name "salvia"
    let response = http_client
        .get("/drugs?name=salvia")
        .header("Content-Type", "application/json")
        .send()
        .await;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    // Response JSON should deserialize into list of drug models.
    let drugs: Vec<drug::Model> = response.json().await;

    // edihkal should return only one drug named "salvia" with a non-nil `Uuid`
    assert_eq!(1, drugs.len());
    assert_eq!(drugs[0].name(), "salvia");
    assert!(!drugs[0].id().is_nil());
}
