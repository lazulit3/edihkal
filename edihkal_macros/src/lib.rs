extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

mod attributes;
mod derives;

/// The DeriveNewModel derive macro will derive a corresponding `NewModel` from a `Model` excluding the model's primary key.
///
/// The `NewModel` type is valuable as a type definition when requesting creation of a new `Model` that does not yet
/// exist in the database and therefor does not have a primary key ID.
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

/// Derives `NewModelTrait` implementation on a [`NewModel`](derive_new_model).
///
/// [`DeriveNewModel`](derive_new_model) automatically implements `NewModelTrait`, so
/// [`DeriveNewModelTrait`] should only be used with `NewModel` types that are manually defined
/// without [`DeriveNewModel`].
#[proc_macro_derive(DeriveNewModelTrait, attributes(sea_orm))]
pub fn derive_new_model_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derives::expand_derive_new_model_trait(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
