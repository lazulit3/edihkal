use axum::extract::{Path, State};
use axum::{http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel};

use entity::drug;
use entity::{drug::NewDrug, Drug};
use uuid::Uuid;

#[tracing::instrument(skip(db))]
pub async fn get_drug(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<drug::Model>, StatusCode> {
    let drug = select_drug(&db, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match drug {
        Some(drug) => Ok(Json(drug)),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

#[tracing::instrument(name = "Getting drugs", skip(db))]
pub async fn get_drugs(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<drug::Model>>, (StatusCode, &'static str)> {
    let drugs = Drug::find().all(&db).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to get defined drugs",
        )
    })?;
    Ok(Json(drugs))
}

/// Handles requests to define a `NewDrug`.
#[tracing::instrument(name = "Defining new drug", skip(db), fields(drug = drug.name))]
pub async fn define_drug(
    State(db): State<DatabaseConnection>,
    Json(drug): Json<NewDrug>,
) -> Result<Json<drug::Model>, (StatusCode, &'static str)> {
    let drug = insert_drug(&db, drug).await.map_err(|_| {
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

/// Select a drug by ID from the database.
#[tracing::instrument(name = "Selecting drug", skip(db))]
pub async fn select_drug(
    db: &DatabaseConnection,
    drug_id: Uuid,
) -> Result<Option<drug::Model>, DbErr> {
    Drug::find_by_id(drug_id).one(db).await
}
