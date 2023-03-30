use sea_orm::ModelTrait;
use uuid::Uuid;

/// Trait describing a [data model](ModelTrait) as an API [`Resource`].
pub trait Resource: ModelTrait {
    /// API route path that the [`Resource`] belongs to.
    const ROUTE: &'static str;

    fn id(&self) -> &Uuid;

    /// Returns a unique path identifying a [`Resource`] that may be used in the `Location` header.
    fn location(&self) -> String {
        format!("{}/{}", Self::ROUTE, self.id())
    }
}

impl Resource for entity::drug::Model {
    const ROUTE: &'static str = "/drugs";

    fn id(&self) -> &Uuid {
        self.id()
    }
}

impl Resource for entity::entry::Model {
    const ROUTE: &'static str = "/entries";

    fn id(&self) -> &Uuid {
        self.id()
    }
}
