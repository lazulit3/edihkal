use std::collections::HashMap;

use anyhow::Context;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use sea_orm::prelude::*;

use sea_skipper::{
    query_filter::{QueryFilter, QueryParams},
    Resource,
};

use crate::{db::query, errors::ApiError};

#[tracing::instrument(skip(db))]
pub async fn get_by_id<R: Resource>(
    State(db): State<DatabaseConnection>,
    Path(id): Path<R::Id>,
) -> Result<Json<R::Data>, ApiError> {
    let resource = query::find_by_id::<R>(&db, id)
        .await
        .context("Failed to get resource")?;

    match resource {
        Some(resource) => Ok(Json(resource)),
        _ => Err(ApiError::NotFound),
    }
}

// TODO: Update outdated docs.
// Responds with a JSON list of drugs matching the query.
//
// # Query Filters
// Query filters may be used to limit results to matching values.
//
// * `/drugs` - Get all drugs (no filters)
// * `/drugs?name=methaqualone` - Get Drugs named "methaqualone"
#[tracing::instrument(skip(db))]
// pub async fn get_collection_with_filter<R: Resourcl, P: QueryParams<Entity = R>>(
pub async fn get_collection_with_filter<R: Resource, P: QueryParams>(
    Query(query): Query<HashMap<String, String>>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<R::Data>>, ApiError> {
    let filter = QueryFilter::<P>::from_query_string(query);

    let collection = query::find_with_filter::<R, _>(&db, filter)
        .await
        .context("Failed to query filtered records from database")?;

    Ok(Json(collection))
}
