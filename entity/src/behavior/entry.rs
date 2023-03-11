use sea_orm::{prelude::*, Set};
use serde::{Deserialize, Serialize};

use crate::{entry::ActiveModel, entry::Model};

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// Create a new `entry::ActiveModel` with a random id and default values.
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }

    /// Ensure `id` is initialized before insert operations.
    ///
    /// This avoids mistakes where a `Model` is constructed with a default `Uuid` and then converted into an `ActiveModel`.
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert && (self.id.is_not_set() || self.id.as_ref().is_nil()) {
            self.id = Set(Uuid::new_v4());
        }
        Ok(self)
    }
}

/// ActiveModel type for requesting a `NewEntry` where `id` is ommitted.
#[derive(Clone, Debug, DeriveIntoActiveModel, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewEntry {
    // id is ommitted
    pub dose: i32,
    pub drug_id: Uuid,
    pub time: DateTime,
}

impl NewEntry {
    pub fn new(dose: i32, drug_id: Uuid, time: DateTime) -> NewEntry {
        NewEntry {
            dose,
            drug_id,
            time,
        }
    }
}

impl From<NewEntry> for Model {
    fn from(entry: NewEntry) -> Self {
        Model::new(entry.time, entry.drug_id, entry.dose)
    }
}
