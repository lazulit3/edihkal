use axum::http::StatusCode;
use axum_test_helper::TestClient;
use edihkal_server::{configuration::get_configuration, router::router};
use sqlx::{Connection, PgConnection};

#[tokio::test]
async fn health_check_works() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");
    let client = TestClient::new(router(connection));

    let response = client.get("/health_check").send().await;

    assert_eq!(response.status(), StatusCode::OK);
}
