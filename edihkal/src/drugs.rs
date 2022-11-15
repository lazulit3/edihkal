use axum::{http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, IntoActiveModel};

use entity::drug;
use entity::drug::NewDrug;

pub async fn get_drugs(Extension(ref _db): Extension<DatabaseConnection>) -> StatusCode {
    // TODO
    StatusCode::OK
}

/// Handles requests to define a `NewDrug`.
#[tracing::instrument(name = "Defining new drug", skip(db), fields(drug = drug.name))]
pub async fn define_drug(
    Extension(ref db): Extension<DatabaseConnection>,
    Json(drug): Json<NewDrug>,
) -> Result<Json<drug::Model>, (StatusCode, &'static str)> {
    let drug = insert_drug(db, drug).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to insert new drug into database",
        )
    })?;
    Ok(Json(drug))
}

/// Inserts a new drug into the database.
#[tracing::instrument(name = "Inserting drug into database", skip(db), fields(drug = drug.name))]
pub async fn insert_drug(db: &DatabaseConnection, drug: NewDrug) -> Result<drug::Model, DbErr> {
    let drug = drug.into_active_model().insert(db).await?;
    Ok(drug)
}
