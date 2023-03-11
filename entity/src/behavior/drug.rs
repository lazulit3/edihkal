use crate::drug::{ActiveModel, Model};
use sea_orm::{prelude::*, Set};
use serde::{Deserialize, Serialize};

#[async_trait::async_trait]
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
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
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

impl NewDrug {
    pub fn new<S: Into<String>>(name: S) -> NewDrug {
        NewDrug { name: name.into() }
    }
}

impl From<NewDrug> for Model {
    fn from(drug: NewDrug) -> Self {
        Self::new(drug.name)
    }
}

impl PartialEq<NewDrug> for Model {
    fn eq(&self, other: &NewDrug) -> bool {
        self.name() == other.name
    }
}

impl PartialEq<Model> for NewDrug {
    fn eq(&self, other: &Model) -> bool {
        self.name == other.name()
    }
}

#[cfg(test)]
mod tests {
    use crate::{drug::Model, NewDrug};

    #[test]
    fn new_drug_model_partial_eq() {
        let drug_name = "Amyl Nitrite";
        let drug = Model::new(drug_name);
        let new_drug = NewDrug::new(drug_name);

        assert_eq!(drug, new_drug);
        assert_eq!(new_drug, drug);
    }
}
