use axum::{body::Body, http::StatusCode, routing::get, Extension, Router};

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

use crate::{
    configuration::{DatabaseSettings, Settings},
    drugs::{define_drug, get_drugs},
};

pub async fn app(configuration: &Settings) -> Router<Body> {
    let db_settings = &configuration.database;
    let db = db_connection(db_settings).await;
    migrate(&db).await;
    router(db)
}

pub fn router(db: DatabaseConnection) -> Router<Body> {
    Router::new()
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/drugs", get(get_drugs).post(define_drug))
        .layer(Extension(db))
}

pub async fn db_connection(db_settings: &DatabaseSettings) -> DatabaseConnection {
    Database::connect(&db_settings.connection_string())
        .await
        .unwrap()
}

/// Run database migrations
pub async fn migrate(db_connection: &DatabaseConnection) {
    Migrator::up(db_connection, None).await.unwrap();
}
