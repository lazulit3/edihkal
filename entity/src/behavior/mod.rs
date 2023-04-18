//! Contains [`ActiveModelBehavior`][AMB] implementations for `Model` types.
//!
//![`ActiveModelBehavior`][AMB] is implemented for each `Model` to ensure that [`ActiveModel`s][AMT] are not inserted
//! into the database with a [nil `Uuid`][uuid::Uuid::nil].
//!
//! [AMT]: sea_orm::entity::ActiveModelTrait
//! [AMB]: sea_orm::entity::ActiveModelBehavior
pub mod drug;
pub mod entry;
