use anyhow::Context;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::DatabaseConnection;
use sea_skipper::Resource;

use crate::{
    db::{mutation, query},
    errors::ApiError,
};

#[tracing::instrument(skip(db))]
pub async fn delete<R: Resource>(
    State(db): State<DatabaseConnection>,
    Path(id): Path<R::Id>,
) -> Result<StatusCode, ApiError> {
    let resource = query::find_by_id::<R>(&db, id)
        .await
        .context("Failed to query resource to delete by ID")?
        .ok_or_else(|| ApiError::NotFound)?;

    mutation::delete::<R>(&db, resource)
        .await
        .context("Failed to delete resource in database")?;
    Ok(StatusCode::OK)
}
