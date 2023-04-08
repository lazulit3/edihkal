use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

use crate::util::field_not_primary_key;

enum Error {
    InputNotStruct,
}

struct NewModel {
    field_idents: Vec<syn::Ident>,
    field_types: Vec<syn::Type>,
}

impl NewModel {
    fn new(input: syn::DeriveInput) -> Result<Self, Error> {
        let fields = match input.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
                ..
            }) => named,
            _ => return Err(Error::InputNotStruct),
        };

        // Exclude primary key (ID) from NewModel's fields.
        let fields = fields.into_iter().filter(field_not_primary_key);

        let (field_idents, field_types) = fields
            .into_iter()
            .map(|field| (field.ident.as_ref().unwrap().clone(), field.ty))
            .unzip();

        Ok(NewModel {
            field_idents,
            field_types,
        })
    }

    fn expand(&self) -> syn::Result<TokenStream> {
        let Self {
            field_idents,
            field_types,
        } = self;

        Ok(quote!(
            #[automatically_derived]
            #[derive(Clone, Debug, PartialEq, Eq, DeriveIntoActiveModel, Serialize, Deserialize)]
            #[doc = " Generated by [`edihkal_macros::DeriveNewModel`]"]
            pub struct NewModel {
                #(
                    pub #field_idents: #field_types,
                )*
            }

            impl NewModel {
                pub fn new(#(#field_idents: #field_types,)*) -> Self {
                    Self {
                        #(#field_idents),*
                    }
                }
            }
        ))
    }
}

/// Method to derive a `NewModel` from a `Model` definition.
// pub fn expand_derive_new_model(ident: Ident, data: Data) -> syn::Result<TokenStream> {
pub fn expand_derive_new_model(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let ident_span = input.ident.span();

    match NewModel::new(input) {
        Ok(new_model) => new_model.expand(),
        Err(Error::InputNotStruct) => Ok(quote_spanned! {
            ident_span => compile_error!("you can only derive DeriveNewModel on structs");
        }),
    }
}
