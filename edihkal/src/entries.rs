use anyhow::Context;
use axum::{extract::State, response::IntoResponse, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};

use entity::{entry, NewEntry};

use crate::{
    errors::{ApiError, DatabaseError},
    responses::created,
};

/// Handler to create an entry.
#[tracing::instrument(skip(db))]
pub async fn create_entry(
    State(db): State<DatabaseConnection>,
    Json(entry): Json<NewEntry>,
) -> Result<impl IntoResponse, ApiError> {
    let entry = insert_entry(&db, entry).await.context("Failed to create NewEntry")?;
    Ok(created(entry))
}

/// Inserts a new journal entry into the database.
#[tracing::instrument(name = "Inserting entry into database", skip(db))]
pub async fn insert_entry(
    db: &DatabaseConnection,
    entry: NewEntry,
) -> Result<entry::Model, DatabaseError> {
    let entry = entry
        .into_active_model()
        .insert(db)
        .await
        .context("Failed to insert NewEntry into the database")?;
    Ok(entry)
}
