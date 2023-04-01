use std::fmt::Debug;

use migration::IntoCondition;
use sea_orm::{prelude::*, DatabaseConnection};
use sea_skipper::Resource;

use crate::errors::DatabaseError;

/// Returns all of records for [`Resource`] `R` from the database.
#[tracing::instrument(skip(db))]
pub async fn find_all<R: Resource>(db: &DatabaseConnection) -> Result<Vec<R::Data>, DatabaseError> {
    Ok(R::find().all(db).await?)
}

#[tracing::instrument(skip(db))]
pub async fn find_by_id<R: Resource>(
    db: &DatabaseConnection,
    id: R::Id,
) -> Result<Option<R::Data>, DatabaseError> {
    Ok(R::find_by_id(id).one(db).await?)
}

/// Selects many entity records matching `Condition` from the database.
#[tracing::instrument(skip(db))]
pub async fn find_with_filter<E: EntityTrait, C>(
    db: &DatabaseConnection,
    condition: C,
) -> Result<Vec<E::Model>, DatabaseError>
where
    C: IntoCondition + Debug,
{
    Ok(E::find().filter(condition).all(db).await?)
}

/// Selects one or zero entity records matching `Condition` from the database.
#[tracing::instrument(skip(db))]
pub async fn find_one_with_filter<E: EntityTrait, C>(
    db: &DatabaseConnection,
    condition: C,
) -> Result<Option<E::Model>, DatabaseError>
where
    C: IntoCondition + Debug,
{
    Ok(E::find().filter(condition).one(db).await?)
}
