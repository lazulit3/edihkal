use sea_orm::{prelude::*, Set};

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
