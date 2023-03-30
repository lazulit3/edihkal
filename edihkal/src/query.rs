use std::collections::HashMap;

use migration::IntoCondition;
use sea_orm::ColumnTrait;
use sea_orm::{sea_query::Condition, EntityTrait};

/// A trait describing how query string parameters map to a database entity's [`EntityTrait::Column`s](EntityTrait::Column).
pub trait QueryParams: EntityTrait {
    /// Returns `Some(Self::`[`Column`](EntityTrait::Column)`)` filtered by a supported `query_param`, otherwise `None`.
    fn column(query_param: &str) -> Option<Self::Column>;
}

/// A set of valid [`Filters`] on an entity's [`Column`s](EntityTrait::Column) constructed from a request URL's query string.
///
/// `Filters` represents valid filters for some entity implementing [`QueryParams`] and may be
/// constructed [`from_query_string`](Self::from_query_string) when handling a request.
///
/// [`Self::into_condition()`] may be used to convert `Self` into a database [`Condition`] to perform database operations
/// with constraints determined by the query string from a request.
#[derive(Debug)]
pub struct Filters<E: QueryParams + EntityTrait>(Vec<(E::Column, String)>);

impl<E: QueryParams + EntityTrait> Filters<E> {
    /// Returns [`Filters`] on [`EntityTrait::Column`]s from a request's `query_string`.
    ///
    /// This will drop any `query_string` parameters that are not defined in `Self`'s implementation of
    /// [`QueryParams::column`].
    pub fn from_query_string(query_string: HashMap<String, String>) -> Self {
        Self(
            query_string
                .into_iter()
                .filter_map(|(param, value)| E::column(&param).map(|column| (column, value)))
                .collect(),
        )
    }
}

/// Converts [`Filters`] into a database [`Condition`] that may be used when querying some [`EntityTrait`] from the database.
impl<E: QueryParams> IntoCondition for Filters<E> {
    fn into_condition(self) -> Condition {
        self.0.into_iter().fold(Condition::all(), |all, (column, value)| {
            all.add(column.eq(value))
        })
    }
}
