use axum::http::StatusCode;
use axum_test_helper::TestClient;
use edihkal_server::{configuration::get_configuration, router::app};
use sqlx::{query, Connection, PgConnection};

#[tokio::test]
async fn define_drug_returns_200_for_valid_data() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let client = TestClient::new(app(&configuration).await);

    let drug_body = serde_json::json!({"name": "caffeine"});

    let response = client
        .post("/drugs")
        .header("Content-Type", "application/json")
        .json(&drug_body)
        .send()
        .await;

    assert_eq!(response.status(), StatusCode::OK);

    let mut db_connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");

    let saved_drug = query!("SELECT name FROM drugs",)
        .fetch_one(&mut db_connection)
        .await
        .expect("Failed to fetch defined drug.");

    assert_eq!(saved_drug.name, "caffeine")
}

#[tokio::test]
async fn define_drug_returns_400_for_missing_data() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let client = TestClient::new(app(&configuration).await);

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
    let configuration = get_configuration().expect("Failed to read configuration");
    let _client = TestClient::new(app(&configuration).await);
    todo!()
}
