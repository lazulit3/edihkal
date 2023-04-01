use anyhow::Context;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Json,
};
use sea_orm::DatabaseConnection;
use sea_skipper::{DataTrait, Location, ModelCondition, Resource};

use crate::{
    db::{mutation, query::find_one_with_filter},
    errors::{ApiError, DatabaseError},
    responses::created,
};

#[tracing::instrument(skip(db))]
pub async fn create<R, D>(
    State(db): State<DatabaseConnection>,
    Json(data): Json<D>,
) -> Result<impl IntoResponse, ApiError>
where
    R: Resource,
    R::Data: Location,
    D: DataTrait<R> + ModelCondition,
{
    match mutation::insert(&db, data.clone()).await {
        Ok(resource) => Ok(created(resource).into_response()),

        // Some unique field in new_resource conflicted with an existing record in the database.
        Err(DatabaseError::UniqueViolation(_)) => {
            // Look for a record in database with fields matching new_resource.
            let existing_resource: Option<R::Data> =
                find_one_with_filter::<R, _>(&db, data.to_all_condition())
                    .await
                    .context(
                        "Failed to query database for existing resource matching create request",
                    )?;
            if let Some(resource) = existing_resource {
                Ok(Redirect::to(&resource.location()).into_response())
            } else {
                Err(ApiError::Conflict)
            }
        }
        Err(DatabaseError::Unknown(err)) => Err(ApiError::InternalServerError(err)),
    }
}
