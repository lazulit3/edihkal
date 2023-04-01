//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;
use sea_skipper::{DeriveNewModel, Location, Resource};
use serde::{Deserialize, Serialize};

use crate::Uuid;

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, DeriveNewModel, Eq, Serialize, Deserialize,
)]
#[sea_orm(table_name = "entry")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub drug_id: Uuid,
    pub time: DateTime,
    pub dose: i32,
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

// TODO: Derive this.
impl Resource for Entity {
    type ActiveModel = ActiveModel;
    type Data = Model;
    type Id = Uuid;
}

// TODO: Derive this.
impl Location for Model {
    fn location(&self) -> String {
        format!("/entries/{}", self.id)
    }
}
