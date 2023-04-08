extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

mod derives;
mod util;

/// The DeriveNewModel derive macro will derive a corresponding `NewModel` from a `Model` excluding the model's primary key.
///
/// The `NewModel` type is valuable as a type definition when requesting creation of a new `Model` that does not yet
/// exist in the database and therefor does not have a primary key ID.
#[proc_macro_derive(DeriveNewModel, attributes(sea_orm))]
pub fn derive_new_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derives::expand_derive_new_model(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
