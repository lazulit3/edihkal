use sea_orm::entity::prelude::*;
use sea_orm_new_model::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "derive_new_model_trait")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub flavor: String,
    pub color: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, DeriveIntoActiveModel, DeriveNewModelTrait)]
pub struct NewModel {
    pub flavor: String,
    pub color: String,
}

#[test]
pub fn derive_new_model_trait() {
    let flavor = "strawberry".to_string();
    let color = "pink".to_string();
    let _new_model = NewModel { flavor, color };
}
