use proc_macro2::Span;
use syn::{
    parse_quote, Attribute, Error, Generics, Ident, Item, ItemImpl, Meta, NestedMeta, Result,
};

use crate::utils::generics_to_ident_list;

pub fn impl_metadata(attr: Attribute, ident: &Ident, generics: &Generics) -> Result<Item> {
    let generics_params = generics_to_ident_list(generics);

    let args = attr.parse_meta()?;

    let mut name = None;
    let mut version = None;
    let mut impl_version = None;
    let mut target_height = None;

    if let Meta::List(v) = args {
        for meta in v.nested {
            if let NestedMeta::Meta(v) = meta {
                if let Meta::NameValue(n) = v {
                    let key = n
                        .path
                        .get_ident()
                        .ok_or(Error::new(Span::call_site(), "get name error"))?
                        .to_string();
                    match key.as_str() {
                        "name" => name = Some(n.lit),
                        "version" => version = Some(n.lit),
                        "impl_version" => impl_version = Some(n.lit),
                        "target_height" => target_height = Some(n.lit),
                        _ => return Err(Error::new(Span::call_site(), "Unexpected key")),
                    }
                }
            }
        }
    }

    let mut impl_: ItemImpl = parse_quote! {
        impl baseline::prelude::ModuleMetadata for #ident<#generics_params> {
            fn metadata() -> baseline::Metadata {
                baseline::Metadata {
                    name: String::from(#name),
                    version: baseline::types::ModuleVersion(#version),
                    impl_version: String::from(#impl_version),
                    target_height: baseline::types::BlockHeight(#target_height),
                }
            }
        }
    };

    impl_.generics = generics.clone();

    Ok(Item::Impl(impl_))
}
