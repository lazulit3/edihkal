use entity::Uuid;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Statement};
use secrecy::ExposeSecret;

use edihkal::{app::migrate, configuration::get_configuration};

/// Create a database, run migrations, and return a `DatabaseConnection` for isolated test runs.
pub async fn unique_database() -> DatabaseConnection {
    // Build datatabase connection URL with a random database name.
    let mut config = get_configuration().expect("Failed to read configuration").database;
    config.database_name = Uuid::new_v4().to_string();

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
    migrate(&db).await.unwrap();

    db
}
