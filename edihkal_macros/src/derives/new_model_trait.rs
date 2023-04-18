use heck::ToUpperCamelCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{punctuated::Punctuated, token::Comma, Lit, Meta};

use super::util::{escape_rust_keyword, not_sea_orm_attr, trim_starting_raw_identifier};
use crate::attributes::derive_attr;

enum Error {
    InputNotStruct,
    Syn(syn::Error),
}

struct DeriveNewModelTrait {
    active_model_ident: syn::Ident,
    column_idents: Vec<syn::Ident>,
    entity_ident: syn::Ident,
    field_idents: Vec<syn::Ident>,
    ident: syn::Ident,
    model_ident: syn::Ident,
    primary_key_attrs: Vec<bool>,
}

impl DeriveNewModelTrait {
    fn new(input: syn::DeriveInput) -> Result<Self, Error> {
        // Extract named fields from the struct.
        let fields = match input.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
                ..
            }) => named,
            _ => return Err(Error::InputNotStruct),
        };

        // Parse model attributes.
        let sea_attr = derive_attr::SeaOrm::try_from_attributes(&input.attrs)
            .map_err(Error::Syn)?
            .unwrap_or_default();

        let ident = input.ident;
        let field_idents = fields
            .iter()
            .map(|field| field.ident.as_ref().unwrap().clone())
            .collect();
        let active_model_ident =
            sea_attr.active_model.unwrap_or_else(|| format_ident!("ActiveModel"));
        let entity_ident = sea_attr.entity.unwrap_or_else(|| format_ident!("Entity"));
        let model_ident = sea_attr.model.unwrap_or_else(|| format_ident!("Model"));

        // Examine each field's attributes to identify primary_keys and Column idents.
        let mut primary_key_attrs: Vec<bool> = Vec::new();
        let mut column_idents: Vec<Ident> = Vec::new();
        for field in fields {
            if let Some(ident) = &field.ident {
                let original_field_name = trim_starting_raw_identifier(ident);
                let mut column_ident = Ident::new(
                    &original_field_name.to_upper_camel_case(),
                    Span::call_site(),
                );
                let mut enum_name = None;
                let mut is_primary_key = false;
                // TODO: Handle ignore?
                // search for #[sea_orm(primary_key, enum_name = "Name")] to identify primary key columns.
                for attr in field.attrs.iter() {
                    if not_sea_orm_attr(attr) {
                        continue;
                    }

                    if let Ok(list) =
                        attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated)
                    {
                        for meta in list.iter() {
                            match meta {
                                Meta::NameValue(nv) => {
                                    if let Some(name) = nv.path.get_ident() {
                                        if name == "enum_name" {
                                            if let Lit::Str(litstr) = &nv.lit {
                                                enum_name =
                                                    syn::parse_str(&litstr.value()).unwrap();
                                            }
                                        }
                                    }
                                }
                                Meta::Path(p) => {
                                    if let Some(name) = p.get_ident() {
                                        if name == "primary_key" {
                                            is_primary_key = true;
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }

                // Track whether this field is a primary key.
                primary_key_attrs.push(is_primary_key);

                if let Some(enum_name) = enum_name {
                    column_ident = enum_name;
                }

                column_ident = Ident::new(&escape_rust_keyword(column_ident), Span::call_site());

                column_idents.push(column_ident);
            }
        }

        Ok(DeriveNewModelTrait {
            active_model_ident,
            column_idents,
            entity_ident,
            field_idents,
            ident,
            model_ident,
            primary_key_attrs,
        })
    }

    fn expand<'a>(&'a self) -> syn::Result<TokenStream> {
        let Self {
            active_model_ident,
            entity_ident,
            ident,
            model_ident,
            primary_key_attrs,
            ..
        } = self;

        // Ignore fields & columns for primary keys.
        let ignore = |(ident, ignore): (&'a Ident, &bool)| -> Option<&'a Ident> {
            if *ignore {
                None
            } else {
                Some(ident)
            }
        };
        let field_idents: Vec<&Ident> = self
            .field_idents
            .iter()
            .zip(primary_key_attrs)
            .filter_map(ignore)
            .collect();
        let column_idents: Vec<&Ident> = self
            .column_idents
            .iter()
            .zip(primary_key_attrs)
            .filter_map(ignore)
            .collect();

        let missing_field_msg = format!("field does not exist on {ident}");

        Ok(quote!(
            #[automatically_derived]
            impl NewModelTrait for NewModel {
                type ActiveModel = #active_model_ident;
                type Entity = #entity_ident;
                type Model = #model_ident;

                fn get(&self, c: <Self::Entity as sea_orm::entity::EntityTrait>::Column) -> sea_orm::Value {
                    match c {
                        #(<Self::Entity as sea_orm::entity::EntityTrait>::Column::#column_idents => self.#field_idents.clone().into(),)*
                        _ => panic!(#missing_field_msg),
                    }
                }

                fn set(&mut self, c: <Self::Entity as sea_orm::entity::EntityTrait>::Column, v: sea_orm::Value) {
                    match c {
                        #(<Self::Entity as sea_orm::entity::EntityTrait>::Column::#column_idents => self.#field_idents = v.unwrap(),)*
                        _ => panic!(#missing_field_msg),
                    }
                }

                fn as_filter_all(&self) -> sea_orm::Condition {
                    sea_orm::Condition::all()
                        #(.add(<Self::Entity as EntityTrait>::Column::#column_idents.eq(self.get(<Self::Entity as EntityTrait>::Column::#column_idents))))*
                }
            }
        ))
    }
}

/// Method to derive a `NewModelTrait` from a `Model` definition.
pub fn expand_derive_new_model_trait(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let ident_span = input.ident.span();

    match DeriveNewModelTrait::new(input) {
        Ok(new_model) => new_model.expand(),
        Err(Error::InputNotStruct) => Ok(quote_spanned! {
            ident_span => compile_error!("you can only derive DeriveNewModel on structs");
        }),
        Err(Error::Syn(err)) => Err(err),
    }
}
