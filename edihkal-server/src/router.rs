use std::sync::Arc;

use axum::{body::Body, http::StatusCode, routing::get, Extension, Router};
use sqlx::{Connection, PgConnection};

use crate::{
    configuration::Settings,
    drugs::{define_drug, get_drugs},
};

pub async fn router(configuration: &Settings) -> Router<Body> {
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");
    let connection = Arc::new(connection);

    Router::new()
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/drugs", get(get_drugs).post(define_drug))
        .layer(Extension(connection))
}
