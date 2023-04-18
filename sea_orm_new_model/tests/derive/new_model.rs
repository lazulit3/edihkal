use sea_orm::entity::prelude::*;
use sea_orm_new_model::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, DeriveNewModel)]
#[sea_orm(table_name = "derive_new_model")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub flavor: String,
    pub color: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[test]
pub fn derive_new_model() {
    let flavor = "strawberry".to_string();
    let color = "pink".to_string();
    let _new_model = NewModel { flavor, color };
}
