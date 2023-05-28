//! Tests different ways to construct a [`user::ActiveModel`] and ensures Uuid is never nil.
use entity::user::{ActiveModel, Model};

use sea_orm::{ActiveModelBehavior, IntoActiveModel};

/// An `ActiveModel` constructed with `ActiveModel`'s `Default::default()` has expected values.
#[test]
fn active_model_with_default_id_has_expected_values() {
    let active_model = ActiveModel {
        ..Default::default()
    };

    assert!(active_model.id.is_set());
    assert!(!active_model.id.as_ref().is_nil());
}

/// An `ActiveModel` constructed with `ActiveModelBehavior::new()` has expected values.
#[test]
fn active_model_new_has_expected_values() {
    let active_model = ActiveModel::new();

    assert!(active_model.id.is_set());
    assert!(!active_model.id.as_ref().is_nil());
}

/// A `Model` with a `default()` `Uuid` converted into an `ActiveModel` has an initialized id.
///
/// Verifies that the [`entity::Uuid`] newtype's [`Default`][entity::Uuid::default()]} implementation returns
/// an initialized value (in contrast to [`uuid::Uuid`]`}).
#[test]
fn model_with_default_id_into_active_model_has_initialized_uuid() {
    let drug_model = Model {
        id: Default::default(),
        username: "willywonka".into(),
        password_hash: "TODO".into(),
    };
    let active_model = drug_model.into_active_model();

    assert!(!active_model.id.as_ref().is_nil());
}
