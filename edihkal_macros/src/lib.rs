//! The `edihkal_macros` crate defines derive macros for `sea_orm_new_model`.
//!
//! This crate is not indended to be used directly; see the `sea_orm_new_model` crate for usage.

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

mod attributes;
mod derives;

/// Derives a `NewModel` type from a `Model` and excludes the model's primary key fields.
///
/// A `NewModel` is equivalent to a `Model` with fields marked by `#[sea_orm(primary_key)]` removed.
/// All other `Model` fields appear in `NewModel` with the same name and type.
///
/// [`DeriveNewModel`] derives a `sea_orm::IntoActiveModel` implementation to allow `NewModel`
/// data to be inserted into the database.
///
/// # Usage
///
/// To derive a `NewModel` from a `Model` generated by `sea-orm`, add `DeriveNewModel` to the
/// `Model`'s derives:
///
/// ```ignore
/// #[sea_orm(table_name = "posts")]
/// #[derive(
///     Clone, Debug, PartialEq, Eq, DeriveEntityModel, DeriveNewModel, Serialize, Deserialize,
/// )]
/// pub struct Model {
///     #[sea_orm(primary_key)]
///     #[serde(skip_deserializing)]
///     pub id: i32,
///     pub title: String,
///     #[sea_orm(column_type = "Text")]
///     pub text: String,
/// }
/// ```
///
/// This expands to a `NewModel` behind the scenes:
///
/// ```ignore
/// #[derive(
///     Clone, Debug, PartialEq, Eq, DeriveNewModel, Serialize, Deserialize,
/// )]
/// pub struct Model {
///     pub title: String,
///     pub text: String,
/// }
/// ````
#[proc_macro_derive(DeriveNewModel, attributes(sea_orm))]
pub fn derive_new_model(input: TokenStream) -> TokenStream {
    let input_ts = input.clone();
    let input = parse_macro_input!(input as DeriveInput);

    let mut ts: TokenStream = derives::expand_derive_new_model(input)
        .unwrap_or_else(Error::into_compile_error)
        .into();
    ts.extend([derive_new_model_trait(input_ts)]);
    ts
}

/// Derives `NewModelTrait` implementation on a `NewModel`.
///
/// [`DeriveNewModelTrait`] may be used to implement `NewModelTrait` on `NewModel` types that are
/// defined manually without using [`DeriveNewModel`].
#[proc_macro_derive(DeriveNewModelTrait, attributes(sea_orm))]
pub fn derive_new_model_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derives::expand_derive_new_model_trait(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
