use entity::{drug::ActiveModel, NewDrug};
/// Tests around different ways to construct an ActiveModel.
use sea_orm::{ActiveModelBehavior, IntoActiveModel, Set};

/// `NewDrug.into_active_model()` has expected values.
///
/// This is the preferred approach to get an `ActiveModel` from a new drug request.
#[test]
fn new_drug_into_active_model_has_expected_values() {
    let new_drug = NewDrug {
        name: "Alcohol".to_owned(),
    };
    let active_model = new_drug.into_active_model();

    assert!(active_model.id.is_set());
    assert!(!active_model.id.as_ref().is_nil());
    assert!(active_model.name.is_set());
    assert_eq!(active_model.name.as_ref(), "Alcohol");
}

/// An `ActiveModel` constructed with `ActiveModel`'s `Default::default()` has expected values.
///
/// Converting `NewDrug` may be preferred, but this is works.
#[test]
fn active_model_with_default_id_has_expected_values() {
    let active_model = ActiveModel {
        name: Set("Bufotenine".to_owned()),
        ..Default::default()
    };

    assert!(!active_model.id.as_ref().is_nil());
    assert!(active_model.id.is_set());
    assert!(active_model.name.is_set());
    assert_eq!(active_model.name.as_ref(), "Bufotenine");
}

/// An `ActiveModel` constructed with `ActiveModelBehavior::new()` has expected values.
///
/// Converting `NewDrug` may be preferred, but this is works.
#[test]
fn active_model_new_has_expected_values() {
    let mut active_model = ActiveModel::new();
    active_model.name = Set("Cannabis".to_owned());

    assert!(!active_model.id.as_ref().is_nil());
    assert!(active_model.id.is_set());
    assert!(active_model.name.is_set());
    assert_eq!(active_model.name.as_ref(), "Cannabis");
}
