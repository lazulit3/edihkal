//! Module defining RESTful API resources.

/// A RESTful API `Resource`.
///
/// Defines an API `Resource` identified by a relative URI path.
///
/// # Example
///
/// For example, an API [`Resource`] for [`sea_orm::tests_cfg::cake`]:
///
/// ```
/// use seaxum::api::resource::Resource;
///
/// pub struct Cake;
/// impl Resource for Cake {
///     const PATH: &'static str = "/cake";
/// }
/// ```
pub trait Resource {
    // TODO: Validate PATH's syntax at build time (using a derive macro attribute?)
    /// Relative URI `PATH` that identifies [`Self`] as an API `Resource`.
    const PATH: &'static str;
}

#[cfg(test)]
mod test {
    #[test]
    pub fn define_cake_resource() {
        use seaxum::api::resource::Resource;

        pub struct Cake;
        impl Resource for Cake {
            const PATH: &'static str = "/cake";
        }
    }
}
