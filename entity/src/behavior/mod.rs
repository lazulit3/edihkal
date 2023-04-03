//! [`behavior`] contains [`ActiveModelBehavior`][AMB] implementation for [Models][M].
//!
//![`ActiveModelBehavior`][AMB] is implemented for each `Model` to ensure that an [`ActiveModel`s][AMT] are not inserted
//! into the database with a [nil `Uuid`][uuid::Uuid::nil]
//!
//! [AMT]: sea_orm::entity::ActiveModelTraot
//! [AMB]: sea_orm::entity::ActiveModelBehavior
pub mod drug;
pub mod entry;
