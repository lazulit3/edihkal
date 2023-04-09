//! Module for representing a `sea-orm::entity` and related types.
use sea_orm::prelude::*;

/// Contains types related to a [`sea-orm::entity`].
///
/// Define `DbEntity` to capture the related types for some SeaORM entity (corresponding to a table in the database).
/// This includes types such as `Entity`, `Model`, `ActiveModel`, etc.
///
/// # Example
///
/// For example, a `DbEntity` defined for [`sea_orm::tests_cfg::cake`]:
///
/// ```
/// use sea_orm::tests_cfg::cake;
/// use seaxum::db::entity::DbEntity;
///
/// pub struct CakeEntity;
/// impl DbEntity for CakeEntity {
///     type Entity = cake::Entity;
/// }
/// ```
pub trait DbEntity {
    type Entity: EntityTrait;
}
