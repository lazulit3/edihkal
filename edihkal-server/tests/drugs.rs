use axum::http::StatusCode;
use axum_test_helper::TestClient;
use edihkal_server::{configuration::get_configuration, router::router};
use sqlx::{Connection, PgConnection};
use std::collections::HashMap;

#[tokio::test]
async fn define_drug_returns_200_for_valid_data() {
    let client = TestClient::new(router());
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to database");

    // TODO: Replace with model
    let mut drug_data = HashMap::new();
    drug_data.insert("name", "caffeine");

    let response = client
        .post("/drugs")
        .header("Content-Type", "application/json")
        .json(&drug_data)
        .send()
        .await;

    assert_eq!(response.status(), StatusCode::OK);

    let saved_drug = sqlx::query!("SELECT name FROM drugs",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch defined drug.");

    assert_eq!(saved_drug.name, "caffeine")
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
