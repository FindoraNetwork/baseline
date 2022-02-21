use syn::{ItemStruct, Item, Result, ItemImpl, parse_quote};

use crate::utils::generics_to_ident_list;

pub fn impl_module(st: &ItemStruct) -> Result<Item> {
    let ident = &st.ident;
    let generics_params = generics_to_ident_list(&st.generics);

    let mut res: ItemImpl = parse_quote! {
        impl baseline::prelude::Module for #ident<#generics_params> {
            // type Context
        }
    };

    res.generics = st.generics.clone();

    Ok(Item::Impl(res))
}
