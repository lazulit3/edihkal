use sea_orm::{prelude::*, sea_query::IntoCondition, Condition, IntoSimpleExpr, Set};

use crate::{drug, Uuid};

#[async_trait::async_trait]
impl ActiveModelBehavior for drug::ActiveModel {
    /// Create a new `drug::ActiveModel` with a random id and default values.
    ///
    /// This allows the `Uuid` to be initialized before inserting in the database in a few cases:
    /// 1. This defines behavior for `ActiveModel`'s `default()` implementation.
    /// 2. This defines behavior when `NewDrug` is converted into an `ActiveModel`.
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}

/// Convert `NewModel` into a query [`Condition`] that matches all [`drug::Column`]s except for [`drug::Column::Id`].
///
/// When inserting a `NewModel` into the database results in a unique violation database error (i.e. a drug with the
/// same unique name already exists), this may be used for determining if the drug that would be inserted matches the
/// existing ['drug::Model'].
impl IntoCondition for drug::NewModel {
    fn into_condition(self) -> Condition {
        Condition::all().add(drug::Column::Name.into_simple_expr().eq(self.name))
    }
}
