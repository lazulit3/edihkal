use sea_orm::{prelude::*, Condition, IntoActiveModel};
use std::fmt::Debug;

#[cfg(feature = "derive")]
pub use edihkal_macros::{DeriveNewModel, DeriveNewModelTrait};

#[async_trait::async_trait]
pub trait NewModelTrait: Clone + Send + Debug + IntoActiveModel<Self::ActiveModel> {
    type ActiveModel: ActiveModelTrait<Entity = Self::Entity> + ActiveModelBehavior + Send;
    type Entity: EntityTrait<Model = Self::Model>;
    type Model: ModelTrait<Entity = Self::Entity> + IntoActiveModel<Self::ActiveModel>;

    async fn insert<C>(self, db: &C) -> Result<<Self as NewModelTrait>::Model, DbErr>
    where
        C: ConnectionTrait,
    {
        self.into_active_model().insert(db).await
    }

    fn get(&self, c: <Self::Entity as EntityTrait>::Column) -> sea_orm::Value;
    fn set(&mut self, c: <Self::Entity as EntityTrait>::Column, v: sea_orm::Value);

    /// Returns a [`sea_orm::Condition::all()`] filtering on `self`'s values.
    fn as_filter_all(&self) -> Condition;
}
