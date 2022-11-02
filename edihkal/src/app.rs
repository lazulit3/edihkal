use std::sync::Arc;

use axum::{body::Body, http::StatusCode, routing::get, Extension, Router};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse};
use tower_http::ServiceBuilderExt;
use tower_http::{request_id::MakeRequestUuid, trace::TraceLayer};

use crate::{
    configuration::{DatabaseSettings, Settings},
    drugs::{define_drug, get_drugs},
};

pub async fn app(configuration: &Settings) -> Router<Body> {
    let db_pool = db_pool(&configuration.database).await;
    router(db_pool).await
}

pub async fn router(db_pool: PgPool) -> Router<Body> {
    let db_pool = Arc::new(db_pool);
    Router::new()
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/drugs", get(get_drugs).post(define_drug))
        .layer(Extension(db_pool))
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_response(DefaultOnResponse::new().include_headers(true)),
                )
                .propagate_x_request_id(),
        )
}

async fn db_pool(db_settings: &DatabaseSettings) -> PgPool {
    PgPool::connect(db_settings.connection_string().expose_secret())
        .await
        .expect("Failed to connect to database")
}
