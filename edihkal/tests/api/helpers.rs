use axum_test_helper::TestClient;
use edihkal::{
    app::{migrate, router},
    configuration::{get_configuration, DatabaseSettings},
};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Statement};
use secrecy::ExposeSecret;
use uuid::Uuid;

pub async fn test_client() -> TestClient {
    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let test_service_db = configure_database(&configuration.database).await;
    TestClient::new(router(test_service_db))
}

pub async fn test_client_and_db() -> (TestClient, DatabaseConnection) {
    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let test_service_db = configure_database(&configuration.database).await;
    let test_client = TestClient::new(router(test_service_db));

    let test_db = Database::connect(configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to database");

    (test_client, test_db)
}

/// Create a database, run migrations, and return a `DatabaseConnection` for isolated test runs.
pub async fn configure_database(config: &DatabaseSettings) -> DatabaseConnection {
    // Connect to database service without selecting a specific database name
    let db = Database::connect(config.connection_string_without_db().expose_secret())
        .await
        .expect("Failed to connect to database service");

    // Create the new database with the configured name
    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("CREATE DATABASE \"{}\";", config.database_name),
    ))
    .await
    .expect("Failed to create database");

    // Create a new database connection selecting the newly created database
    let db = Database::connect(config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to newly created database");

    // Run migrations against the newly created database
    migrate(&db).await;

    db
}
