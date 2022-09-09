use axum::http::StatusCode;
use axum_test_helper::TestClient;
use edihkal_server::{configuration::get_configuration, router::router};
use sqlx::{Connection, PgConnection};
use std::collections::HashMap;

#[tokio::test]
async fn define_drug_returns_200_for_valid_data() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");
    let client = TestClient::new(router(connection));

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

    // TODO: This is silly
    let mut test_connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");

    let saved_drug = sqlx::query!("SELECT name FROM drugs",)
        .fetch_one(&mut test_connection)
        .await
        .expect("Failed to fetch defined drug.");

    assert_eq!(saved_drug.name, "caffeine")
}

#[tokio::test]
async fn define_drug_returns_400_for_missing_data() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");
    let client = TestClient::new(router(connection));
    let response = client.post("/drugs").send().await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn get_drugs_returns_list_of_drugs() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");
    let client = TestClient::new(router(connection));
    todo!()
}
