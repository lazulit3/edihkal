use std::sync::Arc;

use aide::axum::{
    routing::get,
    routing::get_with,
    ApiRouter,
};
use axum::{body::Body, http::StatusCode, Json, Router};
use sqlx::PgPool;

use crate::{
    configuration::{DatabaseSettings, Settings},
    drugs::{define_drug, get_drugs, define_drug_docs},
    openapi::api_docs,
};

pub async fn app(configuration: &Settings) -> Router<PgPool, Body> {
    let db_pool = db_pool(&configuration.database).await;
    router(db_pool).await
}

pub async fn router(db_pool: PgPool) -> Router<PgPool, Body> {
    let db_pool = Arc::new(db_pool);
    let mut api = api_docs();

    ApiRouter::with_state_arc(db_pool)
        .api_route(
            "/health_check",
            get_with(
                || async { StatusCode::OK },
                |op| op.response::<200, ()>(),
            ),
        )
        .api_route("/drugs", get(get_drugs).post_with(define_drug, define_drug_docs))
        .finish_api(&mut api)
        .route(
            "/openapi.json",
            axum::routing::get(|| async { Json(api) }),
        )
}

async fn db_pool(db_settings: &DatabaseSettings) -> PgPool {
    PgPool::connect(&db_settings.connection_string())
        .await
        .expect("Failed to connect to database")
}
