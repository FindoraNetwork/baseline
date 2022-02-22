use syn::{parse_quote, FieldValue, Item, ItemImpl, ItemStruct, Result};

use crate::utils::generics_to_ident_list;

pub fn impl_defined(st: &ItemStruct) -> Result<Item> {
    let mut field = Vec::new();

    for item in &st.fields {
        let ident = item.ident.clone();

        let f: FieldValue = parse_quote! {
            #ident: self.#ident.clone()
        };

        field.push(f);
    }

    let ident = &st.ident;
    let generics_params = generics_to_ident_list(&st.generics);

    let mut res: ItemImpl = parse_quote! {
        impl baseline::prelude::ModuleDefined for #ident<#generics_params> {}
    };

    res.generics = st.generics.clone();

    Ok(Item::Impl(res))
}
