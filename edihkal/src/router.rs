use std::sync::Arc;

use axum::{body::Body, http::StatusCode, routing::get, Extension, Router};
use sqlx::PgPool;

use migration::{Migrator, MigratorTrait};

use crate::{
    configuration::{DatabaseSettings, Settings},
    drugs::{define_drug, get_drugs},
};

pub async fn app(configuration: &Settings) -> Router<Body> {
    let db_settings = configuration.database;
    let migrate(&db_settings);
    let db_pool = db_pool(&db_settings).await;
    router(db_pool).await
}

pub async fn router(db_pool: PgPool) -> Router<Body> {
    let db_pool = Arc::new(db_pool);
    Router::new()
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/drugs", get(get_drugs).post(define_drug))
        .layer(Extension(db_pool))
}

async fn db_pool(db_settings: &DatabaseSettings) -> PgPool {
    PgPool::connect(&db_settings.connection_string())
        .await
        .expect("Failed to connect to database")
}

/// Run database migrations
async fn migrate(db_settings: &DatabaseSettings) {
    let connection = sea_orm::Database::connect(&db_settings.connection_string())
        .await
        .unwrap();
    Migrator::up(&connection, None).await.unwrap();
}
