use syn::{parse_quote, Item, ItemImpl, ItemStruct, Result};

use crate::utils::generics_to_ident_list;

use super::attr::Attr;

pub fn impl_module(st: &ItemStruct, attr: &Attr) -> Result<Item> {
    let ident = &st.ident;
    let generics_params = generics_to_ident_list(&st.generics);

    let context_type = &attr.context;

    let mut res: ItemImpl = parse_quote! {
        impl baseline::prelude::Module for #ident<#generics_params> {
            type Context = #context_type;


            fn set_ctx(&mut self, context: Self::Context) {
                // self.
            }
        }
    };

    res.generics = st.generics.clone();

    Ok(Item::Impl(res))
}
