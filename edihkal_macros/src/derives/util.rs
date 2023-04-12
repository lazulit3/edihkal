use syn::{punctuated::Punctuated, token::Comma, Field, Meta};

/// Returns true if field does not have attribute `#[sea_orm(primary_key)]`.
pub(crate) fn field_not_primary_key(field: &Field) -> bool {
    for attr in field.attrs.iter() {
        // Skip this attr if it's not a sea_orm attribute.
        if let Some(ident) = attr.path.get_ident() {
            if ident != "sea_orm" {
                continue;
            }
        } else {
            continue;
        }

        if let Ok(list) = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated) {
            for meta in list.iter() {
                if let Meta::Path(path) = meta {
                    if let Some(name) = path.get_ident() {
                        if name == "primary_key" {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}
