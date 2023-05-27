//! Contains code extending the generated code for entities.
//!
//! This includes [`ActiveModelBehavior`][AMB] implementations for each entity to ensure that
//! [`ActiveModel`s][AMT] are not inserted into the database with a [nil `Uuid`][uuid::Uuid::nil].
//!
//! [AMT]: sea_orm::entity::ActiveModelTrait
//! [AMB]: sea_orm::entity::ActiveModelBehavior
pub mod drug;
pub mod entry;
pub mod user;
