//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
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
