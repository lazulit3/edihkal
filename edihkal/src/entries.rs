use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, IntoActiveModel};

use entity::{entry, NewEntry};

/// Handler to create an entry.
#[tracing::instrument(skip(db))]
pub async fn create_entry(
    State(db): State<DatabaseConnection>,
    Json(entry): Json<NewEntry>,
) -> Result<Json<entry::Model>, (StatusCode, &'static str)> {
    let entry = insert_entry(&db, entry).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to insert new journal entry into database",
        )
    })?;
    Ok(Json(entry))
}

/// Inserts a new journal entry into the database.
#[tracing::instrument(name = "Inserting entry into database", skip(db))]
pub async fn insert_entry(db: &DatabaseConnection, entry: NewEntry) -> Result<entry::Model, DbErr> {
    let entry = entry.into_active_model().insert(db).await?;
    Ok(entry)
}
