use syn::{parse_quote, Item, ItemImpl, ItemStruct, Result};

use crate::utils::generics_to_ident_list;

pub fn impl_manager(st: &ItemStruct) -> Result<Item> {
    let ident = &st.ident;
    let generics_params = generics_to_ident_list(&st.generics);

    let mut res: ItemImpl = parse_quote! {
        impl baseline::prelude::Manager for #ident<#generics_params> {}
    };

    res.generics = st.generics.clone();

    Ok(Item::Impl(res))
}
