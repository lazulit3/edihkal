use sea_orm::entity::prelude::*;
use sea_orm_new_model::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
#[sea_orm(entity = Ent1ty)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "derive_new_model_attrs"
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel, DeriveNewModel)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(enum_name = "Fl4v0r")]
    pub flavor: String,
    pub color: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Fl4v0r,
    Color,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;

    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Fl4v0r => ColumnType::String(None).def(),
            Self::Color => ColumnType::String(None).def(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[test]
pub fn derive_new_model_with_enum_name_attr() {
    let flavor = "strawberry".to_string();
    let color = "pink".to_string();
    let _new_model = NewModel { flavor, color };
}
