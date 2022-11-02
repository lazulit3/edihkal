use axum_test_helper::TestClient;
use edihkal::{
    app::router,
    configuration::{get_configuration, DatabaseSettings},
};
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub async fn test_client() -> TestClient {
    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let db_pool = configure_database(&configuration.database).await;
    TestClient::new(router(db_pool).await)
}

pub async fn test_client_and_db() -> (TestClient, PgPool) {
    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let client_db_pool = configure_database(&configuration.database).await;
    let test_client = TestClient::new(router(client_db_pool).await);

    let db_pool = PgPool::connect(configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    (test_client, db_pool)
}

/// Create database, run migrations, and return connection pool
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection =
        PgConnection::connect(config.connection_string_without_db().expose_secret())
            .await
            .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect(config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}
