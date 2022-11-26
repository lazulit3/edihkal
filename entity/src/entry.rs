//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::{entity::prelude::*, prelude::async_trait::async_trait, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "entry")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    id: Uuid,
    dose: i32,
    drug_id: Uuid,
    time: DateTime,
}

impl Model {
    /// Construct a new `entry::Model` with a random id.
    pub fn new(time: DateTime, drug_id: Uuid, dose: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            time,
            drug_id,
            dose,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn time(&self) -> &DateTime {
        &self.time
    }

    pub fn drug_id(&self) -> &Uuid {
        &self.drug_id
    }

    pub fn dose(&self) -> i32 {
        self.dose
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::drug::Entity",
        from = "Column::DrugId",
        to = "super::drug::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Drug,
}

impl Related<super::drug::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Drug.def()
    }
}

#[async_trait]
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
