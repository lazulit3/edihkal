//! Contains [`ActiveModelBehavior`][AMB] and [`IntoCondition`][IC] implementations for `Model` types.
//!
//![`ActiveModelBehavior`][AMB] is implemented for each `Model` to ensure that an [`ActiveModel`s][AMT] are not inserted
//! into the database with a [nil `Uuid`][uuid::Uuid::nil].
//!
//! [`IntoCondition`][IC] is implemented for some `NewModel` types to create a query [`Condition`][C] for checking if an
//! equivalent entity (matching all of `NewModel` fields) already exists in the database.
//!
//! [AMT]: sea_orm::entity::ActiveModelTrait
//! [AMB]: sea_orm::entity::ActiveModelBehavior
//! [C]: sea_orm::Condition
//! [IC]: sea_orm::sea_query::IntoCondition
pub mod drug;
pub mod entry;
