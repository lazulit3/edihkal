//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.2

use sea_orm::{entity::prelude::*, DeriveIntoActiveModel, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "drug")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    id: Uuid,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    /// Create a new `drug::ActiveModel` with a random id and default values.
    ///
    /// This allows the `Uuid` to be initialized before inserting in the database in a few cases:
    /// 1. This defines behavior for `ActiveModel`'s `default()` implementation.
    /// 2. This defines behavior when `NewDrug` is converted into an `ActiveModel`.
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }

    /// Ensure `id` is initialized before insert operations.
    ///
    /// This avoids mistakes where a `drug::Model` is constructed with a default `Uuid` and then converted into an `ActiveModel`.
    fn before_save(mut self, insert: bool) -> Result<Self, DbErr> {
        if insert && (self.id.is_not_set() || self.id.as_ref().is_nil()) {
            self.id = Set(Uuid::new_v4());
        }
        Ok(self)
    }
}

/// ActiveModel type for requesting a `NewDrug` where `id` is ommitted.
///
/// This allows user input to specify only the properties they are concerned with when defining a `NewDrug`.
#[derive(Clone, Debug, DeriveIntoActiveModel, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewDrug {
    // id is ommitted for NewDrug requests
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::{ActiveModel, Model, NewDrug};
    use sea_orm::{ActiveModelBehavior, ActiveModelTrait, IntoActiveModel, Set};

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

    // TODO: This is not a test.
    /// A `drug::Model` with a `default()` `Uuid` converted into an `ActiveModel` has a nil `Uuid`.
    ///
    /// This is a footgun, but `ActiveModelBehavior::before_save()` can catch this.
    #[test]
    fn footgun_drug_model_with_default_id_into_active_model_has_nil_id() {
        let drug_model = Model {
            id: Default::default(),
            name: "Dextrometorphan".to_owned(),
        };
        let active_model_via_into_trait = drug_model.into_active_model();

        assert!(active_model_via_into_trait.id.as_ref().is_nil());

        let drug_model = Model {
            id: Default::default(),
            name: "Dextrometorphan".to_owned(),
        };
        let active_model_via_from = ActiveModel::from(drug_model);

        assert!(active_model_via_from.id.as_ref().is_nil());
    }

    // TODO: This is not a test.
    /// Using `ActiveModelTrait::default()` for `id` results in a `NotSet` `id`.
    ///
    /// This is a footgun, but `ActiveModelBehavior::before_save()` can catch this.
    #[test]
    fn footgun_active_model_trait_default_for_id_is_not_set() {
        let active_model = ActiveModel {
            name: Set("Ephedrine".to_owned()),
            ..ActiveModelTrait::default()
        };

        assert!(active_model.id.is_not_set());
    }

    // TODO: Test constraints around construction of drug::Model vs NewDrug vs ActiveModel
}
