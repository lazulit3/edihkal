use anyhow::{Context, Result};
use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use secrecy::ExposeSecret;
use tower::ServiceBuilder;
use tower_http::{
    request_id::MakeRequestUuid,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};

use entity::{
    drug, entry,
    prelude::{Drug, Entry},
};
use migration::{Migrator, MigratorTrait};

use crate::{
    api::{
        handlers::{create, delete, get_by_id, get_collection_with_filter},
        query_params,
    },
    configuration::{DatabaseSettings, Settings},
};

pub async fn app(configuration: &Settings) -> Result<Router> {
    let db_settings = &configuration.database;
    let db = db_connection(db_settings).await?;
    migrate(&db).await?;
    Ok(router(db))
}

pub fn router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route(
            "/drugs",
            get(get_collection_with_filter::<Drug, query_params::Drug>)
                .post(create::<Drug, drug::NewModel>),
        )
        .route("/drugs/:id", get(get_by_id::<Drug>).delete(delete::<Drug>))
        .route("/entries", post(create::<Entry, entry::NewModel>))
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
        .with_state(db)
}

pub async fn db_connection(db_settings: &DatabaseSettings) -> Result<DatabaseConnection> {
    let mut options =
        ConnectOptions::new(db_settings.connection_string().expose_secret().to_string());

    options.sqlx_logging_level(tracing::log::LevelFilter::Debug);

    Database::connect(options)
        .await
        .context("Failed to connect to database")
}

/// Run database migrations
pub async fn migrate(db_connection: &DatabaseConnection) -> Result<()> {
    Migrator::up(db_connection, None)
        .await
        .context("Failed to run database migrations")
}
