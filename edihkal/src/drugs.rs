use std::collections::HashMap;

use anyhow::Context;
use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect, Response},
    Json,
};
use sea_orm::{prelude::*, sea_query::IntoCondition, IntoActiveModel};
use uuid::Uuid;

use entity::{drug, Drug, NewDrug};

use crate::{
    errors::{ApiError, DatabaseError},
    query::{Filters, QueryParams},
    resource::Resource,
    responses::created,
};

impl QueryParams for Drug {
    fn column(query_param: &str) -> Option<Self::Column> {
        match query_param {
            "name" => Some(drug::Column::Name),
            _ => None,
        }
    }
}

#[tracing::instrument(skip(db))]
pub async fn get_drug(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<drug::Model>, ApiError> {
    let drug = select_drug(&db, id).await.context("Failed to get drug")?;

    match drug {
        Some(drug) => Ok(Json(drug)),
        _ => Err(ApiError::NotFound),
    }
}

/// Get drugs defined in edihkal.
///
/// Responds with a JSON list of drugs matching the query.
///
/// # Query Filters
/// Query filters may be used to limit results to matching values.
///
/// * `/drugs` - Get all drugs (no filters)
/// * `/drugs?name=methaqualone` - Get Drugs named "methaqualone"
#[tracing::instrument(name = "Getting drugs", skip(db))]
pub async fn get_drugs(
    Query(query): Query<HashMap<String, String>>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<drug::Model>>, ApiError> {
    let filter = Filters::<Drug>::from_query_string(query);

    let drugs = select_drugs(&db, filter).await.context("Failed to get drugs")?;

    Ok(Json(drugs))
}

/// Request handler that creates a [`Drug`] from a [`NewDrug`] request.
///
/// # Responses
///
/// * [`201 Created`](created) - [`NewDrug`] was created.
/// * [`303 See Other`](Redirect::to) - An equivalent [`Drug`] already exists.
///
/// # Errors
///
/// * [`409 Conflict`](ApiError::Conflict) - [`NewDrug`]'s name conflicts with a different [`Drug`] that already exists.
/// other fields differ.
/// * [`500 Internal Server Error`](ApiError::InternalServerError) - A database error occurred.
/// other fields differ.
#[tracing::instrument(skip(db), fields(drug = new_drug.name))]
pub async fn create_drug(
    State(db): State<DatabaseConnection>,
    Json(new_drug): Json<NewDrug>,
) -> Result<Response, ApiError> {
    match insert_drug(&db, new_drug.clone()).await {
        Ok(drug) => Ok(created(drug).into_response()),

        // Some unique field in NewDrug (e.g name) conflicted with a Drug already in the database.
        Err(DatabaseError::UniqueViolation(_)) => {
            // Determine whether the create request would have been idempotent (i.e. NewDrug and Drug are the same).
            match select_new_drug(&db, new_drug)
                .await
                .context("Failed to determine whether NewDrug request was idempotent")?
            {
                // Result would have been equivalent, so response may redirect to existing resouce.
                Some(existing_drug) => Ok(Redirect::to(&existing_drug.location()).into_response()),
                _ => Err(ApiError::Conflict),
            }
        }
        Err(DatabaseError::Unknown(err)) => Err(ApiError::InternalServerError(err)),
    }
}

/// Inserts a new drug into the database.
#[tracing::instrument(name = "Inserting drug into database", skip(db), fields(drug = drug.name))]
pub async fn insert_drug(
    db: &DatabaseConnection,
    drug: NewDrug,
) -> Result<drug::Model, DatabaseError> {
    let drug = drug.into_active_model().insert(db).await?;
    Ok(drug)
}

/// Select a drug by ID from the database.
#[tracing::instrument(skip(db))]
pub async fn select_drug(
    db: &DatabaseConnection,
    drug_id: Uuid,
) -> Result<Option<drug::Model>, DatabaseError> {
    Ok(Drug::find_by_id(drug_id)
        .one(db)
        .await
        .context("Failed to select Drug from database")?)
}

/// Selects drugs matching `condition` from the database.
#[tracing::instrument(skip(db))]
pub async fn select_drugs<C>(
    db: &DatabaseConnection,
    condition: C,
) -> Result<Vec<drug::Model>, DatabaseError>
where
    C: IntoCondition + std::fmt::Debug,
{
    Ok(Drug::find().filter(condition).all(db).await?)
}

/// Returns some drug matching a [`NewDrug`] from the database.
#[tracing::instrument(skip(db))]
async fn select_new_drug(
    db: &DatabaseConnection,
    new_drug: NewDrug,
) -> Result<Option<drug::Model>, DatabaseError> {
    let drug = Drug::find()
        .filter(new_drug)
        .one(db)
        .await
        .context("Failed to select Drug with NewDrug filter from database")?;
    Ok(drug)
}
