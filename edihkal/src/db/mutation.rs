use anyhow::Context;
use sea_orm::{prelude::*, DatabaseConnection, DeleteResult};
use sea_skipper::{DataTrait, Resource};

use crate::errors::DatabaseError;

#[tracing::instrument(skip(db))]
pub async fn insert<R, D>(db: &DatabaseConnection, data: D) -> Result<R::Data, DatabaseError>
where
    R: Resource,
    D: DataTrait<R>,
{
    // anyhow context is not used here to support `DatabaseError::UniqueViolation` conversion from
    // `sqlx::Error`.
    Ok(data.into_active_model().insert(db).await?)
}

#[tracing::instrument(skip(db))]
pub async fn delete<R: Resource>(
    db: &DatabaseConnection,
    model: R::Data,
) -> Result<DeleteResult, DatabaseError> {
    Ok(model
        .delete(db)
        .await
        .context("Failed to delete model from database")?)
}
