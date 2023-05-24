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

/// Responds with a collection of some [`Resource`] filtered with a query string.
///
/// The request's query string filters on exact values. Supported query parameters for filtering
/// are defined by the [`QueryParams`] implementation.
#[tracing::instrument(skip(db))]
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
