use syn::{parse_quote, FieldValue, Item, ItemImpl, ItemStruct, Result};

use crate::utils::generics_to_ident_list;

pub fn impl_clone(st: &ItemStruct) -> Result<Item> {
    let ident = &st.ident;
    let generics_params = generics_to_ident_list(&st.generics);

    let mut items = Vec::new();

    for item in &st.fields {
        let ident = item.ident.clone();

        let f: FieldValue = parse_quote! {
            #ident: self.#ident.clone()
        };

        items.push(f);
    }

    let mut res: ItemImpl = parse_quote! {
        impl Clone for #ident<#generics_params> {
            fn clone(&self) -> Self {
                Self {
                    #(#items,)*
                }
            }
        }
    };

    res.generics = st.generics.clone();

    Ok(Item::Impl(res))
}
