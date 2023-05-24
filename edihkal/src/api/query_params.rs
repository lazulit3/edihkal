//! Defines [`sea_skipper::query_filter::QueryParams`] types required for query string filters.
//!
//! These provide mappings of query string parameters to [`sea_orm::EntityTrait::Column`]s for
//! entities where URL query strings may be used to filter database queries.

use entity::drug;
use sea_skipper::query_filter::QueryParams;

/// [`query_params::Drug`] describes [`QueryParams`] for filtering on `/drugs/`.
#[derive(Debug)]
pub struct Drug;

impl QueryParams for Drug {
    type Entity = drug::Entity;
    fn column(query_param: &str) -> Option<drug::Column> {
        match query_param {
            "name" => Some(drug::Column::Name),
            _ => None,
        }
    }
}
