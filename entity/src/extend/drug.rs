use sea_orm::{prelude::*, Set};
use sea_skipper::{Location, Resource};

use crate::{drug::*, Uuid};

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
}

// TODO: This could easily be derived.
impl Resource for Entity {
    type ActiveModel = ActiveModel;
    type Data = Model;
    type Id = Uuid;
}

// XXX: This may make more sense for `entity` to become a sub-module of
// `edihkal` since that's where the route is defined. (But that has other considerations for model
// definitions / dependencies for other crates e.g. `edihkal-client`.)
impl Location for Model {
    fn location(&self) -> String {
        format!("/drugs/{}", self.id)
    }
}
