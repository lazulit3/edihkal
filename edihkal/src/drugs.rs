use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::{prelude::*, IntoActiveModel, QueryTrait};
use uuid::Uuid;

use entity::{
    drug::{self, NewDrug},
    Drug,
};

use crate::errors::DatabaseError;

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

/// Get drugs defined in edihkal.
///
/// Responds with a JSON list of drugs matching the query.
///
/// /drugs - Get all defined Drugs
/// /drugs?name=methaqualone - Get Drugs named "methaqualone"
#[tracing::instrument(name = "Getting drugs", skip(db))]
pub async fn get_drugs(
    Query(params): Query<HashMap<String, String>>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<drug::Model>>, (StatusCode, &'static str)> {
    let drugs = Drug::find()
        .apply_if(params.get("name"), |query, name| {
            query.filter(drug::Column::Name.eq(name))
        })
        .all(&db)
        .await
        .map_err(|_| {
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
) -> Response {
    match insert_drug(&db, drug.clone()).await {
        Ok(drug) => (StatusCode::CREATED, Json(drug)).into_response(), // TODO
        Err(DatabaseError::UniqueViolation) => {
            see_other_drug_with_name(&db, &drug.name).await.into_response()
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Returns 303 See Other redirect to Drug named `drug_name`.
async fn see_other_drug_with_name(
    db: &DatabaseConnection,
    drug_name: &str,
) -> Result<impl IntoResponse, StatusCode> {
    match select_drug_with_name(db, drug_name).await {
        Ok(Some(drug)) => Ok((
            StatusCode::SEE_OTHER,
            [(header::LOCATION, format!("/drugs/{}", drug.id()))],
            format!("A drug with the same name ({}) already exists", drug.name()),
        )),
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Inserts a new drug into the database.
#[tracing::instrument(name = "Inserting drug into database", skip(db), fields(drug = drug.name))]
pub async fn insert_drug(
    db: &DatabaseConnection,
    drug: NewDrug,
) -> Result<drug::Model, DatabaseError> {
    let drug = drug
        .into_active_model()
        .insert(db)
        .await
        .map_err(Into::<DatabaseError>::into)?;
    Ok(drug)
}

/// Select a drug by ID from the database.
#[tracing::instrument(skip(db))]
pub async fn select_drug(
    db: &DatabaseConnection,
    drug_id: Uuid,
) -> Result<Option<drug::Model>, DbErr> {
    Drug::find_by_id(drug_id).one(db).await
}

#[tracing::instrument(skip(db), fields(drug = drug_name))]
async fn select_drug_with_name(
    db: &DatabaseConnection,
    drug_name: &str,
) -> Result<Option<drug::Model>, DbErr> {
    let drug = Drug::find().filter(drug::Column::Name.eq(drug_name)).one(db).await?;
    Ok(drug)
}
