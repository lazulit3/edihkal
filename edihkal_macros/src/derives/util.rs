use syn::{punctuated::Punctuated, token::Comma, Attribute, Field, Meta};

/// Returns true if `attr` is not a `#[sea_orm(...)]` attribute.
pub(crate) fn not_sea_orm_attr(attr: &Attribute) -> bool {
    if let Some(ident) = attr.path.get_ident() {
        if ident == "sea_orm" {
            return false;
        }
    }
    true
}

/// Returns true if field does not have attribute `#[sea_orm(primary_key)]`.
pub(crate) fn field_not_primary_key(field: &Field) -> bool {
    for attr in field.attrs.iter() {
        if not_sea_orm_attr(attr) {
            continue;
        };
        if let Ok(list) = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated) {
            for meta in list.iter() {
                if let Meta::Path(path) = meta {
                    if let Some(field_name) = path.get_ident() {
                        if field_name == "primary_key" {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

/// Code lifted from upstream @ sea-orm-macros::derives::utils.
pub use sea_orm_macros_utils::*;
mod sea_orm_macros_utils {
    pub fn escape_rust_keyword<T>(string: T) -> String
    where
        T: ToString,
    {
        let string = string.to_string();
        if RUST_KEYWORDS.iter().any(|s| s.eq(&string)) {
            format!("r#{string}")
        } else if RUST_SPECIAL_KEYWORDS.iter().any(|s| s.eq(&string)) {
            format!("{string}_")
        } else {
            string
        }
    }

    pub fn trim_starting_raw_identifier<T>(string: T) -> String
    where
        T: ToString,
    {
        string.to_string().trim_start_matches(RAW_IDENTIFIER).to_string()
    }

    pub const RAW_IDENTIFIER: &str = "r#";

    pub(crate) const RUST_KEYWORDS: [&str; 49] = [
        "as", "async", "await", "break", "const", "continue", "dyn", "else", "enum", "extern",
        "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
        "pub", "ref", "return", "static", "struct", "super", "trait", "true", "type", "union",
        "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
        "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
    ];

    pub(crate) const RUST_SPECIAL_KEYWORDS: [&str; 3] = ["crate", "Self", "self"];
}
