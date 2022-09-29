use std::sync::Arc;

use axum::{http::StatusCode, routing::get, Router};
use sqlx::PgPool;

use crate::{
    configuration::{DatabaseSettings, Settings},
    drugs::{define_drug, get_drugs},
};

pub async fn app(configuration: &Settings) -> Router<PgPool> {
    let db_pool = db_pool(&configuration.database).await;
    router(db_pool).await
}

pub async fn router(db_pool: PgPool) -> Router<PgPool> {
    let db_pool = Arc::new(db_pool);
    Router::with_state_arc(db_pool)
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/drugs", get(get_drugs).post(define_drug))
}

async fn db_pool(db_settings: &DatabaseSettings) -> PgPool {
    PgPool::connect(&db_settings.connection_string())
        .await
        .expect("Failed to connect to database")
}
